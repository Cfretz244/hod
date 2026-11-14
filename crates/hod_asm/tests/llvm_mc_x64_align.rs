//! `spec/asm.md` §18: alignment directives in x86-64 inline bodies. Each
//! body, assembled by hod at a start offset of its section, is the bytes
//! `llvm-mc` 22 and 19 write for it at that offset, for every `nop`
//! table LLVM has (15, 11, 10 and 7 bytes, chosen with `-mcpu`); the
//! body reserved at its largest and cut to the padding of its offset (as
//! code generation settles it) is the same bytes; malformed operands are
//! refused by name.

use std::fmt::Write as _;
use std::path::Path;

use hod_asm::Refusal;
use hod_asm::align::fill_x64;
use hod_asm::x64::{AlignRow, Placement, assemble_aligned};
use hod_testutil::{Scratch, run, tool};

/// The bodies. `CROSSED` marks those with a branch across a directive
/// (taken at a known offset, refused when reserved).
const BODIES: &[(&str, bool)] = &[
    (".p2align 5", false),
    ("nop\n.p2align 5\nnop", false),
    // zstd's templates (`zstd_decompress_block.c`, `zstd_lazy.c`) in a row.
    (".p2align 6\nnop\n.p2align 5\nnop\n.p2align 4,,10\nnop\n.p2align 3", false),
    (".p2align 4\nnop\nnop\n.balign 64\nnop", false),
    (".balign 16\nint3", false),
    (".align 32\nint3\n.align 8, 0xcc\nint3", false),
    ("int3\n.p2align 4,,7\nint3", false),
    ("int3\n.p2align 6,,20\nint3", false),
    ("int3\n.p2align 4, 0xcc\nint3", false),
    ("int3\n.P2ALIGN 3, -1\nint3\n.BALIGN 4\nint3", false),
    (".p2align 0\n.p2align 1\n.balign 1\n.balign 2\n.balign 0\nnop", false),
    ("int3\n.p2align 4,0xcc,8\nint3", false),
    ("int3\n.p2align 4,,15\nint3", false),
    ("int3\n.p2align 4,,16\nint3", false),
    ("nop\n.p2align 4, 0\nnop", false),
    ("nop\n.p2align 4, -112\nnop", false),
    ("jmp 1f\n1:\n.p2align 5\nmovq %rax, %rbx\n.p2align 3\n2: jne 2b", false),
    ("movl $1, %eax\n.p2align 4\n1: decl %eax\njne 1b\n.p2align 2\nret", false),
    ("jmp 1f\n.p2align 6\n1: nop\njne 1b", true),
    ("1:\n.p2align 4\njmp 1b", true),
    // A branch near the end of `rel8`'s reach: short or near by the padding.
    ("jmp 1f\n.quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0\n.p2align 4\n1: nop", true),
];

/// Fill `0x90` (written positive): `llvm-mc` 22 writes the byte, 19 reads
/// it as code alignment and writes `nop`s. hod writes 22's.
const FILL_90: &str = "nop\n.p2align 4, 0x90\nnop";

/// The `-mcpu` of each `nop` table, and the table's longest `nop`.
const TUNINGS: &[(&str, u8)] = &[("generic", 15), ("bdver1", 11), ("x86-64", 10), ("slm", 7)];

/// The start offsets: every offset modulo 64.
const STARTS: u64 = 64;

/// The body at `start`, as hod assembles it (`Placement::At`).
fn at(body: &str, start: u64, longest: u8) -> Result<Vec<u8>, Refusal> {
    let (mut code, mut refs, mut cfi, mut aligns) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let placement = Placement::At(start);
    let s = &mut String::new();
    assemble_aligned(body, placement, longest, &mut code, &mut refs, &mut cfi, &mut aligns, s)?;
    assert!(refs.is_empty(), "{body:?}");
    Ok(code)
}

/// The body reserved, then cut at each directive to the padding its
/// offset needs, as `hod_cg`'s shrinking settles it (`spec/codegen.md`
/// §5.6.15).
fn settled(body: &str, start: u64, longest: u8) -> Result<Vec<u8>, Refusal> {
    let (mut code, mut refs, mut cfi) = (Vec::new(), Vec::new(), Vec::new());
    let mut aligns: Vec<AlignRow> = Vec::new();
    let s = &mut String::new();
    let p = Placement::Reserve;
    assemble_aligned(body, p, longest, &mut code, &mut refs, &mut cfi, &mut aligns, s)?;
    for a in &aligns {
        assert_eq!(u64::from(a.len), a.align.most(), "{body:?}: reserved");
    }
    let mut out = Vec::new();
    let mut from = 0;
    for a in &aligns {
        out.extend_from_slice(&code[from..a.offset as usize]);
        let pad = a.align.pad(start + out.len() as u64) as usize;
        let here = out.len();
        out.resize(here + pad, 0);
        fill_x64(&mut out[here..], a.align.fill, longest);
        from = (a.offset + a.len) as usize;
    }
    out.extend_from_slice(&code[from..]);
    Ok(out)
}

/// The bytes of section `name` of ELF object `object`.
fn section(file: &[u8], name: &str) -> Option<Vec<u8>> {
    let u16_at = |o: usize| u16::from_le_bytes(file[o..o + 2].try_into().unwrap()) as usize;
    let u32_at = |o: usize| u32::from_le_bytes(file[o..o + 4].try_into().unwrap()) as usize;
    let u64_at = |o: usize| u64::from_le_bytes(file[o..o + 8].try_into().unwrap()) as usize;
    let shoff = u64_at(0x28);
    let (shnum, shstrndx) = (u16_at(0x3c), u16_at(0x3e));
    let header = |i: usize| shoff + 64 * i;
    let strtab = u64_at(header(shstrndx) + 0x18);
    (0..shnum).find_map(|i| {
        let h = header(i);
        let at = strtab + u32_at(h);
        let stop = at + file[at..].iter().position(|&b| b == 0).unwrap();
        (&file[at..stop] == name.as_bytes()).then(|| {
            let (offset, size) = (u64_at(h + 0x18), u64_at(h + 0x20));
            file[offset..offset + size].to_vec()
        })
    })
}

/// `llvm-mc-<major> -mcpu=<cpu>`'s bytes of each body at each start:
/// `(body index, start) -> bytes`, each in a section of its own whose
/// first `start` bytes are `int3`s.
fn oracle(dir: &Scratch, major: u32, cpu: &str, bodies: &[&str]) -> Option<Vec<Vec<Vec<u8>>>> {
    let mc = tool(&format!("llvm-mc-{major}"))?;
    let mut source = String::new();
    for (b, body) in bodies.iter().enumerate() {
        for start in 0..STARTS {
            writeln!(source, ".section .text.b{b}s{start},\"ax\",@progbits\n.p2align 7").unwrap();
            if start != 0 {
                writeln!(source, ".fill {start}, 1, 0xcc").unwrap();
            }
            writeln!(source, "{body}").unwrap();
        }
    }
    let src = dir.write(&format!("align-{major}-{cpu}.s"), &source);
    let obj = dir.join(format!("align-{major}-{cpu}.o"));
    let out = run(
        &mc,
        [
            "--triple=x86_64-unknown-linux-gnu".as_ref(),
            format!("-mcpu={cpu}").as_ref(),
            "-filetype=obj".as_ref(),
            "-o".as_ref(),
            obj.as_os_str(),
            src.as_os_str(),
        ],
    );
    assert!(out.success(), "llvm-mc-{major}: {}", out.stderr_text());
    let file = std::fs::read(Path::new(&obj)).unwrap();
    Some(
        (0..bodies.len())
            .map(|b| {
                (0..STARTS)
                    .map(|start| {
                        let s = section(&file, &format!(".text.b{b}s{start}")).expect("a section");
                        s[start as usize..].to_vec()
                    })
                    .collect()
            })
            .collect(),
    )
}

// §18: every body at every offset modulo 64, for every `nop` table, is
// `llvm-mc` 22's and 19's bytes; reserved and settled it is the same.
#[test]
fn aligned_bodies_match_llvm_mc() {
    let dir = Scratch::new("hod-asm-mc-x64-align");
    let mut bodies: Vec<&str> = BODIES.iter().map(|b| b.0).collect();
    bodies.push(FILL_90);
    let mut compared = 0;
    let mut report = String::new();
    for &(cpu, longest) in TUNINGS {
        for &(body, crossed) in BODIES {
            for start in 0..STARTS {
                let exact = at(body, start, longest).unwrap_or_else(|e| panic!("{body:?}: {e}"));
                match settled(body, start, longest) {
                    Ok(s) => {
                        assert!(!crossed, "{body:?}: reserved, not refused");
                        assert_eq!(s, exact, "{body:?} at {start}: settled");
                    }
                    Err(e) => {
                        assert!(crossed, "{body:?}: {e}");
                        assert_eq!(e.0, "asm:align-crossed");
                    }
                }
            }
        }
        for major in [22, 19] {
            let Some(want) = oracle(&dir, major, cpu, &bodies) else { continue };
            for (b, body) in bodies.iter().enumerate() {
                if *body == FILL_90 && major == 19 {
                    continue;
                }
                for start in 0..STARTS {
                    let got = at(body, start, longest).unwrap();
                    compared += 1;
                    if got != want[b][start as usize] {
                        writeln!(
                            report,
                            "  {body:?} at {start} ({cpu}): hod {got:02x?}, llvm-mc-{major} {:02x?}",
                            want[b][start as usize]
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    assert!(report.is_empty(), "the padding differs:\n{report}");
    println!("hod_asm x64 align: {compared} bodies at their offsets equal llvm-mc's");
}

// §18: the fill `0x90` is LLVM 22's byte, not 19's `nop`s.
#[test]
fn fill_0x90_is_llvm_22_s() {
    let got = at(FILL_90, 0, 15).unwrap();
    assert_eq!(got, [[0x90; 16].as_slice(), &[0x90]].concat());
    let got = at("int3\n.p2align 4, 0x90\nint3", 0, 15).unwrap();
    assert_eq!(got, [&[0xcc][..], &[0x90; 15], &[0xcc]].concat());
}

/// Bodies refused, with the refusal.
const REFUSED: &[(&str, &str)] = &[
    (".p2align 17", "asm:operand/range"),
    (".p2align -1", "asm:operand/range"),
    (".balign 3", "asm:operand/range"),
    (".align 3", "asm:operand/range"),
    (".align 131072", "asm:operand/range"),
    (".p2align 4,,0", "asm:operand/range"),
    (".p2align 4,0x100", "asm:operand/range"),
    (".p2align 4,-129", "asm:operand/range"),
    (".p2align 4,1,2,3", "asm:operand/form"),
    (".p2align 4,", "asm:operand/syntax"),
    (".p2align x", "asm:operand/symbol"),
    (".p2align", "asm:operand/syntax"),
    (".p2alignw 2", "asm:directive/p2alignw"),
    (".p2alignl 2", "asm:directive/p2alignl"),
    (".balignw 4", "asm:directive/balignw"),
    (".balignl 4", "asm:directive/balignl"),
    ("lock\n.p2align 4\nnop", "asm:operand/form"),
];

/// Bodies refused only when reserved: a branch across a directive.
const CROSSED: &[&str] = &[
    "jmp 1f\n.p2align 4\n1:",
    "1:\n.p2align 4\njmp 1b",
    "jne 1f\nnop\n.balign 8, 0xcc\nnop\n1: nop",
];

#[test]
fn refused_bodies_are_named() {
    for &(body, name) in REFUSED {
        for placement in [Placement::At(0), Placement::Reserve] {
            let (mut code, mut refs, mut cfi, mut aligns) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            let s = &mut String::new();
            let got = assemble_aligned(
                body,
                placement,
                15,
                &mut code,
                &mut refs,
                &mut cfi,
                &mut aligns,
                s,
            );
            assert_eq!(got.err().map(|r| r.0), Some(name), "{body:?}");
        }
    }
    for body in CROSSED {
        assert!(at(body, 0, 15).is_ok(), "{body:?}");
        assert_eq!(settled(body, 0, 15).err().map(|r| r.0), Some("asm:align-crossed"), "{body:?}");
    }
    // A branch over a directive that never pads is not refused, nor one
    // beside a directive.
    for body in ["jmp 1f\n.p2align 0\n1:", "jmp 1f\n1:\n.p2align 4\n2: jmp 2b"] {
        assert!(settled(body, 0, 15).is_ok(), "{body:?}");
    }
}

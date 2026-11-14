//! Alignment directives in AArch64 templates (`spec/asm.md` §17,
//! WP-217): the padding a template writes is the padding `llvm-mc` writes
//! for the same body at the same offset of its section, in hod's objects
//! and in `llc`'s (which checks the rule: the boundary is counted from
//! the section), and the sections are aligned as `llc` aligns them.
//!
//! Every template is fenced by two marker words (`hint #64..#127`, which
//! no compiler writes), so its bytes can be found in either object
//! whatever code the compilers put around it. The functions differ in
//! the number of `nop` templates before the first marked one, so each
//! body is met at several offsets.

mod common;

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use common::{emit, ll_header, parse, write_object};
use hod_cg::Options;
use hod_testutil::{LLVM_MAJORS, Scratch, run_with, tool};

/// The bodies, each placed in every function between its markers.
const BODIES: &[&str] = &[
    "nop\n.p2align 5\nnop",
    ".p2align 4\nnop\nnop\n.balign 64\nnop",
    "nop\n.p2align 6,,8\nnop",
    "b 1f\n.balign 16, 0xff\n1: nop",
    ".align 3\n.p2align 1\n.balign 2\nnop",
    "nop\n.p2align 4,,12\nnop",
    "nop\n.P2ALIGN 3, -1\nnop\n.p2align 2\nnop",
];

/// The functions: `f<n>` has `n` `nop` templates first.
const FUNCTIONS: u32 = 4;

/// The marker word `hint #imm`.
const fn hint(imm: u32) -> u32 {
    0xd503_201f | (imm << 5)
}

fn begin(case: usize) -> u32 {
    hint(64 + 2 * case as u32)
}

fn end(case: usize) -> u32 {
    hint(65 + 2 * case as u32)
}

/// The IR text of the module: each body as a template in each function,
/// with some code of the function's own between them.
fn module_text() -> String {
    let mut ll = String::new();
    for f in 0..FUNCTIONS {
        writeln!(ll, "define i64 @f{f}(i64 %x) {{").unwrap();
        for _ in 0..f {
            ll.push_str("  call void asm sideeffect \"nop\", \"\"()\n");
        }
        let mut value = "%x".to_owned();
        for (case, body) in BODIES.iter().enumerate() {
            let text = format!(
                "hint #{}\\0A{}\\0Ahint #{}",
                64 + 2 * case,
                body.replace('\n', "\\0A"),
                65 + 2 * case
            );
            writeln!(ll, "  call void asm sideeffect \"{text}\", \"\"()").unwrap();
            writeln!(ll, "  %v{case} = add i64 {value}, {}", case + 1).unwrap();
            value = format!("%v{case}");
        }
        writeln!(ll, "  ret i64 {value}\n}}").unwrap();
    }
    ll
}

/// One section of an ELF object: its bytes and alignment.
struct Section {
    bytes: Vec<u8>,
    align: u64,
}

/// The section named `name` of `object`.
fn section(object: &Path, name: &str) -> Option<Section> {
    let file = std::fs::read(object).unwrap();
    let u16_at = |o: usize| u16::from_le_bytes(file[o..o + 2].try_into().unwrap()) as usize;
    let u32_at = |o: usize| u32::from_le_bytes(file[o..o + 4].try_into().unwrap()) as usize;
    let u64_at = |o: usize| u64::from_le_bytes(file[o..o + 8].try_into().unwrap());
    let shoff = u64_at(0x28) as usize;
    let (shnum, shstrndx) = (u16_at(0x3c), u16_at(0x3e));
    let header = |i: usize| shoff + 64 * i;
    let strtab = u64_at(header(shstrndx) + 0x18) as usize;
    (0..shnum).find_map(|i| {
        let h = header(i);
        let at = strtab + u32_at(h);
        let stop = at + file[at..].iter().position(|&b| b == 0).unwrap();
        (&file[at..stop] == name.as_bytes()).then(|| {
            let (offset, size) = (u64_at(h + 0x18) as usize, u64_at(h + 0x20) as usize);
            Section { bytes: file[offset..offset + size].to_vec(), align: u64_at(h + 0x30) }
        })
    })
}

fn words(bytes: &[u8]) -> Vec<u32> {
    bytes.chunks_exact(4).map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]])).collect()
}

/// Where the template of `case` is in a function's section: the byte
/// offset of its first word, and its words.
fn template(code: &[u32], case: usize) -> (u32, Vec<u32>) {
    let b = code.iter().position(|&w| w == begin(case)).expect("the begin marker");
    let e = code[b..].iter().position(|&w| w == end(case)).expect("the end marker") + b;
    (4 * (b as u32 + 1), code[b + 1..e].to_vec())
}

/// The largest alignment a body asks for: the origin the oracle places
/// it after.
const ORIGIN: u32 = 64;

/// `llvm-mc-<major>`'s words for each `(offset, body)`: the body placed
/// at `offset` modulo [`ORIGIN`] of a section of its own.
fn oracle(dir: &Scratch, major: u32, cases: &[(u32, &str)]) -> Option<Vec<Vec<u32>>> {
    let mc = tool(&format!("llvm-mc-{major}"))?;
    let mut source = String::new();
    for (i, (offset, body)) in cases.iter().enumerate() {
        writeln!(source, ".section .text.c{i},\"ax\",@progbits\n.p2align 6").unwrap();
        for _ in 0..(offset % ORIGIN) / 4 {
            source.push_str("nop\n");
        }
        writeln!(source, "{body}").unwrap();
    }
    let src = dir.write(&format!("oracle-{major}.s"), &source);
    let obj = dir.join(format!("oracle-{major}.o"));
    let mut command = Command::new(mc);
    command.args(["--triple=aarch64-unknown-linux-gnu", "-filetype=obj", "-o"]).arg(&obj).arg(&src);
    let out = run_with(&mut command, &[], Duration::from_mins(1));
    assert!(out.success(), "llvm-mc-{major}: {}", out.stderr_text());
    Some(
        cases
            .iter()
            .enumerate()
            .map(|(i, (offset, _))| {
                let s = section(&obj, &format!(".text.c{i}")).expect("an oracle section");
                words(&s.bytes[((offset % ORIGIN) as usize)..])
            })
            .collect(),
    )
}

fn llc(dir: &Scratch, major: u32, text: &str, sections: bool) -> Option<PathBuf> {
    let llc = PathBuf::from(format!("/usr/lib/llvm-{major}/bin/llc"));
    if !llc.is_file() {
        return None;
    }
    let tag = if sections { "fs" } else { "text" };
    let input = dir.write(&format!("llc-{major}-{tag}.ll"), format!("{}{text}", ll_header(major)));
    let output = dir.join(format!("llc-{major}-{tag}.o"));
    let mut command = Command::new(llc);
    command.args(["-O0", "-global-isel=0", "-fast-isel=0", "-filetype=obj"]);
    if sections {
        command.arg("-function-sections");
    }
    command.arg(&input).arg("-o").arg(&output);
    let outcome = run_with(&mut command, &[], Duration::from_mins(2));
    assert!(outcome.success(), "llc-{major}: {}", outcome.stderr_text());
    Some(output)
}

// §17: with one section per function, each template's bytes are
// `llvm-mc`'s for its section offset, in hod's object and in `llc`'s,
// and each function's section is aligned as `llc` aligns it.
#[test]
fn padding_and_section_alignment_are_llcs() {
    let dir = Scratch::new("a64-inline-asm-align");
    let text = module_text();
    let (m, bodies) = parse(&format!("{}{text}", ll_header(22)));
    let options = Options { function_sections: true, ..Options::default() };
    let mut objects =
        vec![("hod".to_owned(), write_object(&dir, "hod.o", &emit(&m, &bodies, &options)))];
    for major in LLVM_MAJORS {
        if let Some(o) = llc(&dir, major, &text, true) {
            objects.push((format!("llc-{major}"), o));
        }
    }
    let mut report = String::new();
    let mut aligns: Vec<Vec<u64>> = Vec::new();
    for (who, object) in &objects {
        let mut found = Vec::new();
        let mut placed = Vec::new();
        let mut align = Vec::new();
        for f in 0..FUNCTIONS {
            let s = section(object, &format!(".text.f{f}")).expect("a function section");
            align.push(s.align);
            let code = words(&s.bytes);
            for (case, body) in BODIES.iter().enumerate() {
                let (offset, got) = template(&code, case);
                found.push((f, case, offset, got));
                placed.push((offset, *body));
            }
        }
        aligns.push(align);
        // Each body met at more than one offset modulo 16 (hod's own
        // functions; `llc`'s differ in their code around the templates).
        if who == "hod" {
            let first: BTreeSet<u32> =
                found.iter().filter(|c| c.1 == 0).map(|c| c.2 % 16).collect();
            assert!(first.len() > 1, "the first body is at one offset modulo 16: {first:?}");
        }
        for major in LLVM_MAJORS {
            let Some(want) = oracle(&dir, major, &placed) else { continue };
            for ((f, case, offset, got), want) in found.iter().zip(&want) {
                if got != want {
                    writeln!(
                        report,
                        "  {who} f{f} {:?} at {offset:#x}: {got:08x?}, llvm-mc-{major} {want:08x?}",
                        BODIES[*case]
                    )
                    .unwrap();
                }
            }
        }
    }
    assert!(report.is_empty(), "padding differs:\n{report}");
    for (i, (who, _)) in objects.iter().enumerate().skip(1) {
        assert_eq!(aligns[0], aligns[i], "section alignments: hod and {who}");
    }
    assert_eq!(aligns[0], vec![64; FUNCTIONS as usize]);
    println!(
        "inline_asm_align: {} objects, {} templates each",
        objects.len(),
        FUNCTIONS as usize * BODIES.len()
    );
}

// §17: in one `.text`, `llc` counts from the section offset of the
// template and hod from the function's start, which it aligns: the
// instruction after each directive is on its boundary in both, and the
// section's alignment is the same.
#[test]
fn one_text_section_is_aligned_as_llcs() {
    let dir = Scratch::new("a64-inline-asm-align-text");
    let text = module_text();
    let (m, bodies) = parse(&format!("{}{text}", ll_header(22)));
    let mut objects = vec![(
        "hod".to_owned(),
        write_object(&dir, "hod.o", &emit(&m, &bodies, &Options::default())),
    )];
    for major in LLVM_MAJORS {
        if let Some(o) = llc(&dir, major, &text, false) {
            objects.push((format!("llc-{major}"), o));
        }
    }
    let mut aligns = Vec::new();
    for (who, object) in &objects {
        let s = section(object, ".text").expect(".text");
        aligns.push(s.align);
        let code = words(&s.bytes);
        let mut from = 0;
        for _ in 0..FUNCTIONS {
            // `.p2align 5` then `nop`: the second `nop` of body 0 is on
            // a 32-byte boundary; body 1's last `nop` on a 64-byte one.
            let (offset, got) = template(&code[from..], 0);
            let at = 4 * from as u32 + offset + 4 * (got.len() as u32 - 1);
            assert_eq!(at % 32, 0, "{who}: body 0 ends at {at:#x}");
            let (offset, got) = template(&code[from..], 1);
            let at = 4 * from as u32 + offset + 4 * (got.len() as u32 - 1);
            assert_eq!(at % 64, 0, "{who}: body 1 ends at {at:#x}");
            from =
                code[from..].iter().position(|&w| w == end(BODIES.len() - 1)).unwrap() + from + 1;
        }
    }
    assert!(aligns.iter().all(|&a| a == 64), "section alignments {aligns:?}");
}

// §17: a probed frame's prologue has room that is cut once the code is
// complete (`spec/codegen.md` §5.6.15), which would move an aligned
// template off the boundary its padding was counted for: the function
// is refused. Without the probe it compiles.
#[test]
fn aligned_code_after_a_cut_is_refused() {
    let function = |attrs: &str| {
        format!(
            "define void @f() #0 {{\n  %a = alloca [64 x i8]\n  \
             call void asm sideeffect \"nop\\0A.p2align 5\\0Anop\", \"r\"(ptr %a)\n  \
             ret void\n}}\nattributes #0 = {{ nounwind {attrs} }}\n"
        )
    };
    common::assert_refused(&[(
        &function("\"probe-stack\"=\"inline-asm\""),
        hod_cg::LIMIT_ALIGNED_CODE_MOVED,
    )]);
    assert_eq!(common::compile_ll(&function("")), Ok(()));
}

//! Alignment directives in x86-64 templates (`spec/asm.md` §18, WP-218):
//! the padding a template writes is the padding `llvm-mc` writes for the
//! same body at the same section offset with the function's tuning, in
//! hod's objects (plain, stressed, far) and in `llc`'s; the sections are
//! aligned as `llc` aligns them; the `nop`s follow the tuning rule of
//! `hod_x64::tuning` for every CPU `llc` knows; and programs whose loops
//! are aligned as zstd aligns its loops run, throw through the padding
//! and are debugged as `llc`'s are.
//!
//! Every template is fenced by two marker instructions
//! (`nopl 0x7e7e7eNN(%rax)`, which no compiler writes), so its bytes can
//! be found in either object whatever code the compilers put around it.

#[path = "../../hod_obj/tests/common/linking.rs"]
#[allow(dead_code, reason = "a shared helper: not every function is used here")]
mod linking;

#[path = "common/int_gen.rs"]
mod int_gen;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use hod_cg::{CgError, Options, Stress};
use hod_testutil::{LLVM_MAJORS, Scratch, run, run_with, tool};
use int_gen::{ll_header, llc_of, parse, pic};
use linking::{Toolchain, exit_code};

/// The bodies, each placed in every function between its markers.
const BODIES: &[&str] = &[
    "nop\n.p2align 5\nnop",
    // zstd's run before a loop.
    ".p2align 6\nnop\n.p2align 5\nnop\n.p2align 4,,10\n.p2align 3\nnop",
    "nop\n.p2align 6,,8\nnop",
    "nop\n.balign 32, 0xcc\nnop",
    ".align 16\nnop\nnop\n.balign 64\nnop",
    "jmp 1f\nnop\n1: nop\n.p2align 4\nnop",
    "nop\n.P2ALIGN 3, 0\nnop\n.p2align 2\nnop",
    "nop\n.p2align 4,,15\nnopl (%rax)",
    // `0x90` written positive: LLVM 22 writes the byte, 19 `nop`s.
    "nop\n.balign 16, 0x90\nnop",
];

/// The body whose padding is LLVM 22's only.
const FILL_90: usize = 8;

/// The tunings of the functions: attributes, and the `llvm-mc` CPU that
/// writes the same `nop`s.
const TUNINGS: &[(&str, &str)] = &[
    // kiln's functions.
    ("\"target-cpu\"=\"x86-64\" \"tune-cpu\"=\"generic\"", "generic"),
    ("", "x86-64"),
    ("\"tune-cpu\"=\"slm\"", "slm"),
    ("\"tune-cpu\"=\"bdver1\"", "bdver1"),
    ("\"target-cpu\"=\"x86-64\"", "generic"),
    ("\"target-cpu\"=\"core2\"", "core2"),
];

/// The functions: each tuning with 0, 1 and 2 `nop` templates first.
const LEADS: usize = 3;

fn mark_text(n: usize) -> String {
    format!("nopl 0x7e7e7e{n:02x}(%rax)")
}

fn mark(n: usize) -> [u8; 7] {
    [0x0f, 0x1f, 0x80, n as u8, 0x7e, 0x7e, 0x7e]
}

fn find(code: &[u8], from: usize, what: &[u8]) -> Option<usize> {
    code.get(from..)?.windows(what.len()).position(|w| w == what).map(|i| i + from)
}

/// The IR text of the module (target lines not included): each body as
/// a template in each function, behind a branch over all of them, with
/// some code of the function's own between them.
fn module_text() -> String {
    let mut ll = String::new();
    for (t, (attrs, _)) in TUNINGS.iter().enumerate() {
        for lead in 0..LEADS {
            let f = t * LEADS + lead;
            writeln!(ll, "define i64 @f{f}(i64 %x) #{t} {{\nentry:").unwrap();
            for _ in 0..lead {
                ll.push_str("  call void asm sideeffect \"nop\", \"\"()\n");
            }
            ll.push_str("  %c = icmp eq i64 %x, 7\n  br i1 %c, label %out, label %body\nbody:\n");
            let mut value = "%x".to_owned();
            for (case, body) in BODIES.iter().enumerate() {
                let text = format!(
                    "{}\\0A{}\\0A{}",
                    mark_text(2 * case),
                    body.replace('\n', "\\0A"),
                    mark_text(2 * case + 1)
                );
                writeln!(ll, "  call void asm sideeffect \"{text}\", \"\"()").unwrap();
                writeln!(ll, "  %v{case} = add i64 {value}, {}", case + 1).unwrap();
                value = format!("%v{case}");
            }
            writeln!(
                ll,
                "  br label %out\nout:\n  %r = phi i64 [ 1, %entry ], [ {value}, %body ]\n  \
                 ret i64 %r\n}}"
            )
            .unwrap();
        }
        writeln!(ll, "attributes #{t} = {{ nounwind {attrs} }}").unwrap();
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

/// The largest alignment a body asks for, at most: the origin the
/// oracle places it after.
const ORIGIN: u32 = 128;

/// `llvm-mc-<major> -mcpu=<cpu>`'s bytes for each `(offset, body)`: the
/// body placed at `offset` modulo [`ORIGIN`] of a section of its own.
fn oracle(dir: &Scratch, major: u32, cpu: &str, cases: &[(u32, &str)]) -> Option<Vec<Vec<u8>>> {
    let mc = tool(&format!("llvm-mc-{major}"))?;
    let mut source = String::new();
    for (i, (offset, body)) in cases.iter().enumerate() {
        writeln!(
            source,
            ".section .text.c{i},\"ax\",@progbits\n.p2align 7\n.fill {},1,0xcc\n{body}",
            offset % ORIGIN
        )
        .unwrap();
    }
    let tag = format!("oracle-{major}-{cpu}-{}", cases.first().map_or(0, |c| c.0));
    let src = dir.write(&format!("{tag}.s"), &source);
    let obj = dir.join(format!("{tag}.o"));
    let mut command = Command::new(mc);
    command
        .args(["--triple=x86_64-unknown-linux-gnu", &format!("-mcpu={cpu}"), "-filetype=obj", "-o"])
        .arg(&obj)
        .arg(&src);
    let out = run_with(&mut command, &[], Duration::from_mins(1));
    assert!(out.success(), "llvm-mc-{major}: {}", out.stderr_text());
    Some(
        cases
            .iter()
            .enumerate()
            .map(|(i, (offset, _))| {
                let s = section(&obj, &format!(".text.c{i}")).expect("an oracle section");
                s.bytes[((offset % ORIGIN) as usize)..].to_vec()
            })
            .collect(),
    )
}

/// `llc -O0 -fast-isel=0 -relocation-model=pic` of `text`, with
/// `-function-sections` when `sections`, and `extra` flags.
fn llc(
    dir: &Scratch,
    major: u32,
    name: &str,
    text: &str,
    sections: bool,
    extra: &[&str],
) -> Option<PathBuf> {
    let llc = llc_of(major)?;
    let input = dir.write(&format!("{name}.ll"), format!("{}{text}", ll_header()));
    let output = dir.join(format!("{name}.o"));
    let mut command = Command::new(llc);
    command.args([
        "-O0",
        "-fast-isel=0",
        "-mtriple=x86_64-unknown-linux-gnu",
        "-relocation-model=pic",
        "-filetype=obj",
    ]);
    if sections {
        command.arg("-function-sections");
    }
    command.args(extra).arg(&input).arg("-o").arg(&output);
    let outcome = run_with(&mut command, &[], Duration::from_mins(5));
    assert!(outcome.success(), "llc {major} failed:\n{}", outcome.stderr_text());
    Some(output)
}

fn hod_object(dir: &Scratch, name: &str, text: &str, options: &Options) -> PathBuf {
    let (m, bodies) = parse(&format!("{}{text}", ll_header()));
    let object =
        hod_x64::compile_to_object(&m, &bodies, options).unwrap_or_else(|e| panic!("hod: {e}"));
    dir.write(&format!("{name}.o"), object)
}

/// hod's three builds: plain, every stress option, far branches only.
const BUILDS: [(&str, Stress); 3] = [
    ("hod", Stress { min_pool: false, far: false, displace: false, coro_frame: false }),
    ("stress", Stress::ALL),
    ("far", Stress { min_pool: false, far: true, displace: false, coro_frame: false }),
];

/// The objects of the module: hod's builds and `llc`'s.
fn objects(dir: &Scratch, text: &str, sections: bool) -> Vec<(String, Option<u32>, PathBuf)> {
    let tag = if sections { "fs" } else { "text" };
    let mut objects = Vec::new();
    for (who, stress) in BUILDS {
        let options = Options { function_sections: sections, ..pic(stress) };
        objects.push((
            who.to_owned(),
            None,
            hod_object(dir, &format!("{who}-{tag}"), text, &options),
        ));
    }
    for major in LLVM_MAJORS {
        if let Some(o) = llc(dir, major, &format!("llc{major}-{tag}"), text, sections, &[]) {
            objects.push((format!("llc-{major}"), Some(major), o));
        }
    }
    objects
}

/// Each template found: function, case, section offset, bytes.
type Found = Vec<(usize, usize, u32, Vec<u8>)>;

/// The templates of `object`: with one section per function, in each
/// function's; else in `.text`, function after function.
fn templates(object: &Path, sections: bool) -> (Found, Vec<u64>) {
    let functions = TUNINGS.len() * LEADS;
    let (mut found, mut aligns) = (Vec::new(), Vec::new());
    let text = (!sections).then(|| section(object, ".text").expect(".text"));
    let mut from = 0;
    for f in 0..functions {
        let own;
        let code = if let Some(text) = &text {
            &text.bytes
        } else {
            own = section(object, &format!(".text.f{f}")).expect("a function section");
            aligns.push(own.align);
            from = 0;
            &own.bytes
        };
        for case in 0..BODIES.len() {
            let b = find(code, from, &mark(2 * case)).expect("the begin marker") + 7;
            let e = find(code, b, &mark(2 * case + 1)).expect("the end marker");
            found.push((f, case, b as u32, code[b..e].to_vec()));
            from = e + 7;
        }
    }
    if let Some(text) = text {
        aligns.push(text.align);
    }
    (found, aligns)
}

/// Each template of each object against `llvm-mc` at its section offset
/// with its function's tuning (`llc`'s against its own major, hod's
/// against both but for [`FILL_90`] with 19), and the section alignments
/// against hod's.
fn compare(label: &str, sections: bool) {
    let dir = Scratch::new(label);
    let text = module_text();
    let mut report = String::new();
    let (mut compared, mut aligns) = (0, Vec::new());
    for (who, major, object) in objects(&dir, &text, sections) {
        let (found, align) = templates(&object, sections);
        aligns.push((who.clone(), align));
        if who == "hod" {
            // Each body met at more than one offset modulo 16.
            let first: BTreeSet<u32> =
                found.iter().filter(|c| c.1 == 0).map(|c| c.2 % 16).collect();
            assert!(first.len() > 1, "{label}: body 0 at one offset modulo 16: {first:?}");
        }
        let majors: Vec<u32> = major.map_or(LLVM_MAJORS.to_vec(), |m| vec![m]);
        for m in majors {
            let mut by_cpu: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
            for (i, (f, case, ..)) in found.iter().enumerate() {
                if major.is_none() && m == 19 && *case == FILL_90 {
                    continue;
                }
                by_cpu.entry(TUNINGS[f / LEADS].1).or_default().push(i);
            }
            for (cpu, indices) in by_cpu {
                let cases: Vec<(u32, &str)> =
                    indices.iter().map(|&i| (found[i].2, BODIES[found[i].1])).collect();
                let Some(want) = oracle(&dir, m, cpu, &cases) else { continue };
                for (&i, want) in indices.iter().zip(&want) {
                    let (f, case, offset, got) = &found[i];
                    compared += 1;
                    if got != want {
                        writeln!(
                            report,
                            "  {who} f{f} {:?} at {offset:#x}: {got:02x?}, llvm-mc-{m} -mcpu={cpu} \
                             {want:02x?}",
                            BODIES[*case]
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    assert!(report.is_empty(), "{label}: padding differs:\n{report}");
    for (who, align) in &aligns[1..] {
        assert_eq!(&aligns[0].1, align, "{label}: section alignments of hod and {who}");
    }
    assert!(aligns[0].1.iter().all(|&a| a == 64), "{label}: {:?}", aligns[0].1);
    eprintln!("{label}: {} objects, {compared} templates equal llvm-mc's", aligns.len());
}

// §18: with one section per function, each template's bytes are
// `llvm-mc`'s for its section offset and tuning, in hod's objects and in
// `llc`'s, and each function's section is aligned as `llc` aligns it.
#[test]
fn padding_and_section_alignment_are_llcs() {
    compare("x64-inline-asm-align-fs", true);
}

// §18: in one `.text`, `llc` counts from the section offset of the
// template and hod from the function's start, which it aligns to the
// largest alignment: the bytes are still `llvm-mc`'s at the section
// offset, and the section's alignment is `llc`'s.
#[test]
fn one_text_section_is_padded_as_llcs() {
    compare("x64-inline-asm-align-text", false);
}

/// The template of the tuning comparison: its second padding is 63
/// bytes whatever the offset.
const SIXTY_THREE: &str = "nopl 0x7e7e7e00(%rax)\\0A.p2align 6\\0Aint3\\0A.p2align 6\\0A\
                           nopl 0x7e7e7e01(%rax)";

/// The CPUs LLVM 19 does not know.
const LLVM_22_ONLY: &[&str] = &["diamondrapids", "novalake", "wildcatlake"];

// §18: for every CPU `llc` knows as `tune-cpu`, and for `target-cpu`,
// an empty `tune-cpu` and tuning features, `llc` 19 and 22 write the 63
// bytes as `fill_x64` with `hod_x64::tuning`'s length, and so does hod.
#[test]
fn nops_follow_the_tuning_for_every_cpu() {
    let dir = Scratch::new("x64-inline-asm-align-cpus");
    let mut cases: Vec<(String, u8)> = Vec::new();
    let tuning = [
        hod_x64::tuning::NOP7,
        hod_x64::tuning::NOP11,
        hod_x64::tuning::NOP15,
        hod_x64::tuning::NOP10,
    ];
    for cpu in tuning.iter().flat_map(|l| l.iter()).filter(|c| !c.is_empty()) {
        let longest = hod_x64::tuning::longest_nop(Some(cpu), None, []).expect("a measured CPU");
        cases.push((format!("\"tune-cpu\"=\"{cpu}\""), longest));
    }
    // The rest of the rule, its lengths written out.
    let mut extras: Vec<(String, u8)> = Vec::new();
    for (attrs, longest) in [
        ("", 10),
        ("\"target-cpu\"=\"x86-64\"", 15),
        ("\"target-cpu\"=\"slm\"", 7),
        ("\"target-cpu\"=\"skylake\" \"tune-cpu\"=\"\"", 10),
        ("\"target-cpu\"=\"x86-64\" \"tune-cpu\"=\"x86-64\"", 10),
        ("\"tune-cpu\"=\"slm\" \"target-features\"=\"+fast-15bytenop\"", 7),
        ("\"tune-cpu\"=\"slm\" \"target-features\"=\"-fast-7bytenop\"", 10),
        ("\"tune-cpu\"=\"generic\" \"target-features\"=\"-fast-15bytenop\"", 10),
        ("\"tune-cpu\"=\"bdver1\" \"target-features\"=\"+fast-15bytenop\"", 15),
        ("\"target-cpu\"=\"x86-64\" \"target-features\"=\"+fast-11bytenop,-fast-15bytenop\"", 11),
    ] {
        extras.push((attrs.to_owned(), longest));
    }
    let mut report = String::new();
    let mut compared = 0;
    // Two modules: `llc` keeps one subtarget per `target-cpu`,
    // `tune-cpu` and features written one after the other, so `skylake`
    // with an empty `tune-cpu` would be given the subtarget of `tune-cpu`
    // `skylake` alone if both were in one module.
    for (tag, cases) in [("cpus", &cases), ("rule", &extras)] {
        let mut text = String::new();
        for (i, (attrs, _)) in cases.iter().enumerate() {
            writeln!(
                text,
                "define void @t{i}() #{i} {{\n  call void asm sideeffect \"{SIXTY_THREE}\", \
                 \"\"()\n  ret void\n}}\nattributes #{i} = {{ nounwind {attrs} }}"
            )
            .unwrap();
        }
        let options = Options { function_sections: true, ..pic(Stress::default()) };
        let hod = hod_object(&dir, &format!("hod-{tag}"), &text, &options);
        let mut objects = vec![("hod".to_owned(), 22, hod)];
        for major in LLVM_MAJORS {
            if let Some(o) = llc(&dir, major, &format!("llc{major}-{tag}"), &text, true, &[]) {
                objects.push((format!("llc-{major}"), major, o));
            }
        }
        compared += paddings(&objects, cases, &mut report);
    }
    assert!(report.is_empty(), "nops differ from the rule's:\n{report}");
    eprintln!("tunings compared: {} cases, {compared} paddings", cases.len() + extras.len());
}

/// Each object's 63 bytes of padding against `fill_x64` with each
/// case's length; the number compared.
fn paddings(
    objects: &[(String, u32, PathBuf)],
    cases: &[(String, u8)],
    report: &mut String,
) -> usize {
    let mut compared = 0;
    for (who, major, object) in objects {
        for (i, (attrs, longest)) in cases.iter().enumerate() {
            if *major == 19 && LLVM_22_ONLY.iter().any(|c| attrs.contains(&format!("\"{c}\""))) {
                continue;
            }
            let s = section(object, &format!(".text.t{i}")).expect("a function section");
            let b = find(&s.bytes, 0, &mark(0)).expect("the begin marker") + 7;
            let int3 = b + s.bytes[b..].iter().position(|&x| x == 0xcc).expect("the int3");
            let e = find(&s.bytes, int3, &mark(1)).expect("the end marker");
            let mut want = vec![0; 63];
            hod_asm::align::fill_x64(&mut want, None, *longest);
            compared += 1;
            if s.bytes[int3 + 1..e] != want[..] {
                writeln!(report, "  {who} {attrs}: {:02x?}", &s.bytes[int3 + 1..e]).unwrap();
            }
        }
    }
    compared
}

// §18: `-mcpu` and `-mattr` reach `llc`'s padding only for a function
// without its own `target-cpu`: `hod_x64::tuning::longest_nop` takes
// them in that order (`crates/hod/src/back.rs` passes them), and here
// `llc` shows it.
#[test]
fn mcpu_and_mattr_are_llcs() {
    let dir = Scratch::new("x64-inline-asm-align-mcpu");
    let text = format!(
        "define void @t0() #0 {{\n  call void asm sideeffect \"{SIXTY_THREE}\", \"\"()\n  \
         ret void\n}}\nattributes #0 = {{ nounwind }}\n\
         define void @t1() #1 {{\n  call void asm sideeffect \"{SIXTY_THREE}\", \"\"()\n  \
         ret void\n}}\nattributes #1 = {{ nounwind \"target-cpu\"=\"x86-64\" }}\n"
    );
    let mut compared = 0;
    for (flags, cpu, mattr) in [
        (&["-mcpu=slm"][..], Some("slm"), ""),
        (&["-mcpu=bdver1", "-mattr=+fast-15bytenop"][..], Some("bdver1"), "+fast-15bytenop"),
        (&["-mattr=-fast-15bytenop"][..], None, "-fast-15bytenop"),
    ] {
        for major in LLVM_MAJORS {
            let name = format!("llc{major}-{}", flags.join("").replace(['=', '+'], "_"));
            let Some(object) = llc(&dir, major, &name, &text, true, flags) else { continue };
            for (i, own) in [None, Some("x86-64")].into_iter().enumerate() {
                let s = section(&object, &format!(".text.t{i}")).expect("a function section");
                let int3 = s.bytes.iter().position(|&x| x == 0xcc).expect("the int3");
                let e = find(&s.bytes, int3, &mark(1)).expect("the end marker");
                let longest = hod_x64::tuning::longest_nop(
                    None,
                    own.or(cpu),
                    mattr.split(',').filter(|m| !m.is_empty()),
                )
                .unwrap();
                let mut want = vec![0; 63];
                hod_asm::align::fill_x64(&mut want, None, longest);
                assert_eq!(s.bytes[int3 + 1..e], want[..], "llc {major} {flags:?} t{i}");
                compared += 1;
            }
        }
    }
    eprintln!("-mcpu/-mattr paddings compared with llc: {compared}");
}

// §18: a template that pads with `nop`s in a function tuned for a CPU
// hod has not measured is refused by name; with a fill byte, or no
// padding, it compiles.
#[test]
fn an_unmeasured_tuning_is_refused() {
    let function = |body: &str| {
        format!(
            "{}define void @f() #0 {{\n  call void asm sideeffect \"{body}\", \"\"()\n  \
             ret void\n}}\nattributes #0 = {{ nounwind \"tune-cpu\"=\"a-future-cpu\" }}\n",
            ll_header()
        )
    };
    let (m, bodies) = parse(&function("nop\\0A.p2align 4\\0Anop"));
    match hod_x64::compile_to_object(&m, &bodies, &pic(Stress::default())) {
        Err(hod_x64::ModuleError::Function { error, .. }) => {
            assert_eq!(error, CgError::unsupported(hod_x64::tuning::UNMEASURED));
        }
        other => panic!("not refused: {:?}", other.map(|o| o.len())),
    }
    for body in ["nop\\0A.p2align 4, 0xcc\\0Anop", ".p2align 0\\0Anop", "nop"] {
        let (m, bodies) = parse(&function(body));
        assert!(hod_x64::compile_to_object(&m, &bodies, &pic(Stress::default())).is_ok(), "{body}");
    }
}

/// The programs' functions: `zloop`, a loop aligned as zstd aligns its
/// loops, in a probed frame with a stack protector, a VLA, branches
/// before and after the padding and a call that may throw after it, with
/// line information; `addresses`, the address of an instruction after
/// each kind of directive modulo its alignment (0 when all are on their
/// boundaries).
const SEEDS: &str = "@_ZTIi = external constant ptr
declare i32 @__gxx_personality_v0(...)
declare ptr @__cxa_begin_catch(ptr)
declare void @__cxa_end_catch()
declare i64 @may_throw(i64)

define i64 @zloop(i64 %n) #0 personality ptr @__gxx_personality_v0 !dbg !5 {
entry:
  %big = alloca [16384 x i8], align 16, !dbg !10
  %vla = alloca i8, i64 %n, align 16, !dbg !10
  store i8 2, ptr %big, align 1, !dbg !10
  %c = icmp eq i64 %n, 0, !dbg !10
  br i1 %c, label %zero, label %pre, !dbg !10
zero:
  ret i64 100, !dbg !10
pre:
  call void asm sideeffect \".p2align 6\\0Anop\\0A.p2align 5\\0Anop\\0A.p2align 4,,10\\0A.p2align 3\", \"\"(), !dbg !11
  br label %loop, !dbg !11
loop:
  %i = phi i64 [ 0, %pre ], [ %i1, %loop ], !dbg !12
  %s = phi i64 [ 0, %pre ], [ %s1, %loop ], !dbg !12
  %p = getelementptr i8, ptr %vla, i64 %i, !dbg !12
  %t = trunc i64 %i to i8, !dbg !12
  store i8 %t, ptr %p, align 1, !dbg !12
  call void asm sideeffect \".p2align 4\\0Anop\", \"\"(), !dbg !12
  %s1 = add i64 %s, %i, !dbg !12
  %i1 = add i64 %i, 1, !dbg !12
  %more = icmp ult i64 %i1, %n, !dbg !12
  br i1 %more, label %loop, label %done, !dbg !12
done:
  %nm1 = sub i64 %n, 1, !dbg !13
  %lp = getelementptr i8, ptr %vla, i64 %nm1, !dbg !13
  %lv = load i8, ptr %lp, align 1, !dbg !13
  %lz = zext i8 %lv to i64, !dbg !13
  %bv = load i8, ptr %big, align 1, !dbg !13
  %bz = zext i8 %bv to i64, !dbg !13
  %r = invoke i64 @may_throw(i64 %s1) to label %ok unwind label %lpad, !dbg !14
ok:
  %r1 = add i64 %r, %lz, !dbg !15
  %r2 = add i64 %r1, %bz, !dbg !15
  %odd = and i64 %r2, 1, !dbg !15
  %isodd = icmp ne i64 %odd, 0, !dbg !15
  br i1 %isodd, label %oddb, label %evenb, !dbg !15
oddb:
  ret i64 %r2, !dbg !15
evenb:
  %r3 = add i64 %r2, 1000000, !dbg !15
  ret i64 %r3, !dbg !15
lpad:
  %lp0 = landingpad { ptr, i32 } catch ptr @_ZTIi, !dbg !16
  %ex = extractvalue { ptr, i32 } %lp0, 0, !dbg !16
  %pi = call ptr @__cxa_begin_catch(ptr %ex), !dbg !16
  %v = load i32, ptr %pi, align 4, !dbg !16
  call void @__cxa_end_catch(), !dbg !16
  %vz = sext i32 %v to i64, !dbg !16
  %res = add i64 %vz, 1000, !dbg !16
  ret i64 %res, !dbg !16
}

define i64 @addresses() #1 {
  %a = call i64 asm sideeffect \"nop\\0A.p2align 6\\0Aleaq -7(%rip), $0\", \"=r\"()
  %b = call i64 asm sideeffect \"nop\\0Anop\\0A.p2align 5\\0Aleaq -7(%rip), $0\", \"=r\"()
  %c = call i64 asm sideeffect \"nop\\0A.balign 8, 0x90\\0Aleaq -7(%rip), $0\", \"=r\"()
  %d = call i64 asm sideeffect \"nop\\0A.p2align 4,,15\\0Aleaq -7(%rip), $0\", \"=r\"()
  %e = call i64 asm sideeffect \".p2align 6\\0Anop\\0A.p2align 5\\0Anop\\0A.p2align 4,,10\\0A.p2align 3\\0Aleaq -7(%rip), $0\", \"=r\"()
  %ma = and i64 %a, 63
  %mb = and i64 %b, 31
  %mc = and i64 %c, 7
  %md = and i64 %d, 15
  %me = and i64 %e, 7
  %s1 = or i64 %ma, %mb
  %s2 = or i64 %s1, %mc
  %s3 = or i64 %s2, %md
  %s4 = or i64 %s3, %me
  ret i64 %s4
}

attributes #0 = { sspstrong uwtable \"probe-stack\"=\"inline-asm\" \"frame-pointer\"=\"all\" \"target-cpu\"=\"x86-64\" \"tune-cpu\"=\"generic\" }
attributes #1 = { nounwind \"target-cpu\"=\"x86-64\" \"tune-cpu\"=\"generic\" }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!3, !4}
!0 = distinct !DICompileUnit(language: DW_LANG_C11, file: !1, producer: \"t\", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!1 = !DIFile(filename: \"z.c\", directory: \"/\")
!3 = !{i32 7, !\"Dwarf Version\", i32 5}
!4 = !{i32 2, !\"Debug Info Version\", i32 3}
!5 = distinct !DISubprogram(name: \"zloop\", scope: !1, file: !1, line: 10, type: !6, scopeLine: 10, spFlags: DISPFlagDefinition, unit: !0)
!6 = !DISubroutineType(types: !7)
!7 = !{null}
!10 = !DILocation(line: 10, column: 1, scope: !5)
!11 = !DILocation(line: 20, column: 1, scope: !5)
!12 = !DILocation(line: 30, column: 1, scope: !5)
!13 = !DILocation(line: 35, column: 1, scope: !5)
!14 = !DILocation(line: 40, column: 1, scope: !5)
!15 = !DILocation(line: 45, column: 1, scope: !5)
!16 = !DILocation(line: 50, column: 1, scope: !5)
";

const SEEDS_DRIVER: &str = "extern \"C\" long may_throw(long x) {
  if (x == 45) throw 9;
  return x + 1;
}
extern \"C\" long zloop(long);
extern \"C\" long addresses();
int main() {
  if (addresses() != 0) return 10;
  if (zloop(0) != 100) return 11;
  // 0+1+2+3+4 = 10, may_throw 11, the last VLA byte 4, the big one 2.
  if (zloop(5) != 17) return 12;
  if (zloop(10) != 1009) return 13;
  return 0;
}
";

/// The objects of [`SEEDS`]: hod's builds (one `.text`), hod plain with
/// one section per function, and `llc`'s.
fn seed_objects(dir: &Scratch) -> Vec<(String, PathBuf)> {
    let mut objects = Vec::new();
    for (who, stress) in BUILDS {
        objects.push((who.to_owned(), hod_object(dir, who, SEEDS, &pic(stress))));
    }
    let options = Options { function_sections: true, ..pic(Stress::default()) };
    objects.push(("hod-fs".to_owned(), hod_object(dir, "hod-fs", SEEDS, &options)));
    for major in LLVM_MAJORS {
        if let Some(o) = llc(dir, major, &format!("llc{major}"), SEEDS, false, &[]) {
            objects.push((format!("llc-{major}"), o));
        }
    }
    objects
}

// §18: the seeds run, with every linker, dynamic and static, built by
// hod (plain, stressed, far, one section per function) and by `llc`: the
// aligned instructions are on their boundaries in the program, and an
// exception thrown after the padding is caught by its landing pad.
#[test]
fn aligned_seeds_run() {
    let dir = Scratch::new("x64-inline-asm-align-run");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) = toolchain.compile_cxx(&dir, "driver", SEEDS_DRIVER, &["-fPIC"]) else {
        return;
    };
    let stdcxx = Path::new("-lstdc++");
    let mut runs = 0;
    for (who, object) in seed_objects(&dir) {
        for linker in &toolchain.linkers {
            for (how, flags) in [("dynamic", &[][..]), ("static", &["-static"][..])] {
                let program = dir.join(format!("{who}-{how}-{}", linker.name));
                let ran =
                    toolchain.link_and_run(linker, flags, &[&driver, &object, stdcxx], &program);
                assert_eq!(
                    exit_code(&ran),
                    Some(0),
                    "{who} {how} {}: {}",
                    linker.name,
                    ran.stderr_text()
                );
                runs += 1;
            }
        }
    }
    eprintln!("aligned seed programs that ran: {runs}");
}

/// The instruction starts of `object`'s `.text` (and its size).
fn instruction_starts(object: &Path) -> BTreeSet<u64> {
    let objdump = tool("llvm-objdump").expect("llvm-objdump");
    let out = run(
        &objdump,
        [
            "-d".as_ref(),
            "--no-show-raw-insn".as_ref(),
            "--section=.text".as_ref(),
            object.as_os_str(),
        ],
    );
    assert!(out.success(), "llvm-objdump: {}", out.stderr_text());
    let mut starts: BTreeSet<u64> = out
        .stdout_text()
        .lines()
        .filter_map(|l| {
            let (at, rest) = l.split_once(':')?;
            rest.trim_start_matches(' ')
                .starts_with('\t')
                .then(|| u64::from_str_radix(at.trim(), 16).ok())?
        })
        .collect();
    assert!(starts.len() > 20, "{}: {starts:?}", object.display());
    starts.insert(section(object, ".text").expect(".text").bytes.len() as u64);
    starts
}

/// The addresses of `.debug_line`'s rows and of `.eh_frame`'s
/// `advance_loc` targets.
fn row_addresses(object: &Path) -> (Vec<u64>, Vec<u64>) {
    let dump = tool("llvm-dwarfdump").expect("llvm-dwarfdump");
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok();
    let out = run(&dump, ["--debug-line".as_ref(), object.as_os_str()]);
    assert!(out.success(), "llvm-dwarfdump: {}", out.stderr_text());
    let lines = out
        .stdout_text()
        .lines()
        .filter(|l| l.starts_with("0x") && l.split_whitespace().count() > 4)
        .filter_map(|l| hex(l.split_whitespace().next()?))
        .collect();
    let out = run(&dump, ["--eh-frame".as_ref(), object.as_os_str()]);
    assert!(out.success(), "llvm-dwarfdump: {}", out.stderr_text());
    let frames = out
        .stdout_text()
        .lines()
        .filter(|l| l.trim().starts_with("DW_CFA_advance_loc"))
        .filter_map(|l| hex(l.rsplit("to ").next()?))
        .collect();
    (lines, frames)
}

// §18: hod's `-g` objects of the seeds verify, and every row of their
// line table and unwind table is on an instruction boundary after the
// padding was cut (as `llc`'s are).
#[test]
fn seed_rows_are_on_instructions() {
    let dir = Scratch::new("x64-inline-asm-align-rows");
    let Some(dump) = tool("llvm-dwarfdump") else { return };
    if tool("llvm-objdump").is_none() {
        return;
    }
    let mut checked = 0;
    for (who, object) in seed_objects(&dir) {
        if who == "hod-fs" {
            continue;
        }
        let out = run(&dump, ["--verify".as_ref(), object.as_os_str()]);
        assert!(out.success(), "{who}: llvm-dwarfdump --verify:\n{}", out.stdout_text());
        let starts = instruction_starts(&object);
        let (lines, frames) = row_addresses(&object);
        assert!(lines.len() > 8 && !frames.is_empty(), "{who}: {lines:?} {frames:?}");
        for at in lines.iter().chain(&frames) {
            assert!(starts.contains(at), "{who}: a row at {at:#x} inside an instruction");
            checked += 1;
        }
    }
    eprintln!("seed rows on instruction boundaries: {checked}");
}

/// gdb's output for `commands` on `program`.
fn gdb(
    toolchain: &Toolchain,
    dir: &Scratch,
    program: &Path,
    port: u16,
    commands: &str,
) -> Option<String> {
    let setup = "-ex 'set pagination off' -ex 'set confirm off' \
                 -ex 'set debuginfod enabled off'";
    let script = if toolchain.emulated() {
        let gdb = tool("gdb-multiarch")?;
        let runner = toolchain.runner_command(program);
        let mut line = runner.get_program().to_string_lossy().into_owned();
        line.push_str(" -g ");
        line.push_str(&port.to_string());
        for arg in runner.get_args() {
            line.push(' ');
            line.push_str(&arg.to_string_lossy());
        }
        format!(
            "{line} &\nq=$!\n{} -nx -batch {setup} -ex 'set architecture i386:x86-64' \
             -ex 'file {}' -ex 'target remote localhost:{port}' {commands} -ex kill 2>&1\n\
             kill $q 2>/dev/null\nwait $q 2>/dev/null\nexit 0\n",
            gdb.display(),
            program.display()
        )
    } else {
        let gdb = tool("gdb")?;
        let commands = commands.replacen("-ex continue", "-ex run", 1);
        format!(
            "{} -nx -batch {setup} {commands} -ex kill {} 2>&1\n",
            gdb.display(),
            program.display()
        )
    };
    let script = dir.write_script(&format!("gdb-{port}.sh"), &script);
    let out = run_with(&mut Command::new(script), &[], Duration::from_mins(3));
    assert!(!out.timed_out(), "gdb on {}", program.display());
    Some(out.stdout_text())
}

/// The frames of a gdb backtrace, by name, each with its line.
fn frames(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter(|l| l.starts_with('#'))
        .filter_map(|l| {
            let f = l.split(" in ").nth(1).or_else(|| l.split_whitespace().nth(1))?;
            let line = l.rsplit(':').next().unwrap_or("").trim().to_owned();
            Some((f.split_whitespace().next().unwrap_or("").to_owned(), line))
        })
        .collect()
}

/// The source lines gdb stopped at, in order.
fn stops(text: &str) -> Vec<u32> {
    text.lines()
        .filter_map(|l| l.trim_start_matches("warning: ").split_once('\t')?.0.parse().ok())
        .collect()
}

// §18: gdb, on hod's and `llc`'s programs alike, walks from `may_throw`
// to `main` through the padded `zloop` with the invoke's line, and steps
// from the padding's line into the loop and on to the call.
#[test]
fn gdb_steps_through_the_padding() {
    let dir = Scratch::new("x64-inline-asm-align-gdb");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) = toolchain.compile_cxx(&dir, "driver", SEEDS_DRIVER, &["-fPIC"]) else {
        return;
    };
    let stdcxx = Path::new("-lstdc++");
    let linker = &toolchain.linkers[0];
    let first_port = 20000 + (std::process::id() % 20000) as u16;
    let mut checked = 0;
    for (n, (who, object)) in seed_objects(&dir).into_iter().enumerate() {
        let program = dir.join(format!("gdb-{who}"));
        toolchain.link_ok(linker, &["-static"], &[&driver, &object, stdcxx], &program);
        let walk = "-ex 'break may_throw' -ex continue -ex bt";
        let step = "-ex 'break z.c:20' -ex continue -ex next -ex next -ex next -ex next \
                    -ex next -ex next -ex next -ex next -ex next -ex next -ex next -ex next \
                    -ex next -ex next -ex next -ex next";
        for (k, commands) in [walk, step].into_iter().enumerate() {
            let mut text = String::new();
            let mut done = false;
            for attempt in 0..3u16 {
                let port = first_port + 100 * attempt + 2 * n as u16 + k as u16;
                let Some(out) = gdb(&toolchain, &dir, &program, port, commands) else { return };
                text = out;
                if k == 0 && !frames(&text).is_empty() || k == 1 && !stops(&text).is_empty() {
                    done = true;
                    break;
                }
            }
            assert!(done, "{who}: gdb gave nothing:\n{text}");
            if k == 0 {
                let got = frames(&text);
                let names: Vec<&str> = got.iter().take(3).map(|f| f.0.as_str()).collect();
                assert_eq!(names, ["may_throw", "zloop", "main"], "{who}: {text}");
                assert_eq!(got[1].1, "40", "{who}: zloop's line: {text}");
            } else {
                let got = stops(&text);
                let at = |line| got.iter().position(|&l| l == line);
                assert_eq!(got.first(), Some(&20), "{who}: {got:?}\n{text}");
                assert!(at(30).is_some() && at(30) < at(40), "{who}: {got:?}\n{text}");
            }
            checked += 1;
        }
    }
    eprintln!("gdb sessions through the padding: {checked}");
}

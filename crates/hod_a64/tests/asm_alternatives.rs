//! Multi-alternative inline-assembly constraints on AArch64 (WP-219,
//! `spec/asm.md` §19): which alternative and which code of it `llc` 19
//! and 22 take (`-O0 -global-isel=0 -fast-isel=0`), for a grid of shapes
//! and operand types, compared on the objects (the class of each template
//! operand: memory, a general or a vector register); what an empty
//! template does to an indirect output's memory
//! (`tests/data/asm_alternatives/semantics.ll`, run); the barrier `"+g,w"`
//! and templates with alternatives as kiln compiles C++
//! (`tests/data/asm_alternatives/barrier.cpp`, at `-O0` and `-O0 -g`),
//! built by hod (plain, stress, far) and by `llc`, linked by every linker,
//! dynamically and statically, and run; and what is not measured, refused
//! by name.

mod common;

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use common::linking::{Toolchain, exit_code};
use common::{compile_ll, emit, ll_header, parse, write_object};
use hod_a64::barrier;
use hod_cg::{CgError, Options, Stress};
use hod_testutil::{LLVM_MAJORS, Scratch, run_with, tool};

const SEMANTICS: &str = include_str!("data/asm_alternatives/semantics.ll");
const SEMANTICS_DRIVER: &str = include_str!("data/asm_alternatives/semantics.c");
/// kiln's `-O0` and `-O0 -g` modules of `barrier.cpp`.
const BARRIER: &str = include_str!("data/asm_alternatives/barrier.ll");
const BARRIER_G: &str = include_str!("data/asm_alternatives/barrier-g.ll");

/// `llc` of LLVM `major`, if installed, on `text`. LLVM 19 does not know
/// kiln's (LLVM 22's) `"frame-pointer"="non-leaf-no-reserve"`; it is
/// given `"non-leaf"`, its nearest.
fn llc(dir: &Scratch, major: u32, name: &str, text: &str) -> Option<PathBuf> {
    let llc = PathBuf::from(format!("/usr/lib/llvm-{major}/bin/llc"));
    if !llc.is_file() {
        return None;
    }
    let text = if major < 22 {
        text.replace("\"non-leaf-no-reserve\"", "\"non-leaf\"")
    } else {
        text.to_string()
    };
    let input = dir.write(&format!("{name}.ll"), text);
    let output = dir.join(format!("{name}.o"));
    let mut command = Command::new(llc);
    command
        .args(["-O0", "-global-isel=0", "-fast-isel=0", "-filetype=obj", "-relocation-model=pic"])
        .arg(&input)
        .arg("-o")
        .arg(&output);
    let outcome = run_with(&mut command, &[], Duration::from_mins(2));
    assert!(outcome.success(), "llc-{major}: {}", outcome.stderr_text());
    Some(output)
}

fn options() -> [(&'static str, Options); 3] {
    [
        ("plain", Options::default()),
        ("stress", Options { stress: Stress::ALL, ..Options::default() }),
        (
            "far",
            Options {
                stress: Stress { min_pool: false, far: true, displace: false, coro_frame: false },
                ..Options::default()
            },
        ),
    ]
}

/// hod (plain, stress, far), then `llc` of each major present: the
/// objects of `text`.
fn objects(dir: &Scratch, name: &str, text: &str) -> Vec<(String, PathBuf)> {
    let (m, bodies) = parse(text);
    let mut out = Vec::new();
    for (who, options) in options() {
        let emitted = emit(&m, &bodies, &options);
        out.push((who.to_string(), write_object(dir, &format!("{name}-{who}.o"), &emitted)));
    }
    for major in LLVM_MAJORS {
        if let Some(theirs) = llc(dir, major, &format!("{name}-llc-{major}"), text) {
            out.push((format!("llc-{major}"), theirs));
        }
    }
    out
}

/// Links each object (with `driver`, if any) by every linker found,
/// dynamically and statically, runs it, and requires "checked, 0
/// failures".
fn run_everywhere(dir: &Scratch, objects: &[(String, PathBuf)], driver: Option<&str>) {
    let Some(toolchain) = Toolchain::aarch64(dir) else { return };
    let driver = driver.map(|source| toolchain.compile_c(dir, "driver", source, &[]));
    assert!(!toolchain.linkers.is_empty());
    let mut runs = 0;
    for linker in &toolchain.linkers {
        for (who, object) in objects {
            for (how, flags) in [("dynamic", &[][..]), ("static", &["-static"][..])] {
                let program = dir.join(format!("{who}-{how}-{}", linker.name));
                let mut inputs: Vec<&Path> = vec![object];
                if let Some(d) = &driver {
                    inputs.insert(0, d);
                }
                let ran = toolchain.link_and_run(linker, flags, &inputs, &program);
                let what = format!("{who} {how} {}", linker.name);
                assert_eq!(
                    exit_code(&ran),
                    Some(0),
                    "{what}: {}{}",
                    ran.stdout_text(),
                    ran.stderr_text()
                );
                assert_eq!(ran.stdout_text(), "checked, 0 failures\n", "{what}");
                runs += 1;
            }
        }
    }
    println!("{runs} programs ran");
}

/// The memory of an indirect output: an output with alternatives and an
/// input without them is untied (memory, the input not stored); with
/// alternatives in the input it is tied (stored), as `llc` builds it.
#[test]
fn indirect_outputs_run_as_llc_builds_them() {
    let dir = Scratch::new("a64-asm-alt-semantics");
    let text = format!("{}{SEMANTICS}", ll_header(22));
    let objects = objects(&dir, "semantics", &text);
    run_everywhere(&dir, &objects, Some(SEMANTICS_DRIVER));
}

/// kiln's barrier `"+g,w"` on `int`, `long`, `short`, `char`, `float`,
/// `double`, `__int128`, a pointer, `float32x4_t`, `float64x2_t`,
/// `int32x2_t`, and templates with alternatives, at `-O0` and with `-g`.
#[test]
fn kiln_barriers_run_as_llc_builds_them() {
    let dir = Scratch::new("a64-asm-alt-kiln");
    let mut all = objects(&dir, "barrier", BARRIER);
    all.extend(
        objects(&dir, "barrier-g", BARRIER_G).into_iter().map(|(w, o)| (format!("{w}-g"), o)),
    );
    run_everywhere(&dir, &all, None);
}

/// The grid (`spec/asm.md` §19): constraints (without clobbers), operand
/// type, the template's instruction (`brk #0x77` before it marks it).
const GRID: &[(&str, &str, &str)] = &[
    // The barrier's shape: untied, memory, whatever the type.
    ("=*imr|w,0", "i32", "ldr x17, $0"),
    ("=*imr|w,0", "i64", "ldr x17, $0"),
    ("=*imr|w,0", "float", "ldr x17, $0"),
    ("=*imr|w,0", "double", "ldr x17, $0"),
    ("=*imr|w,0", "ptr", "ldr x17, $0"),
    ("=*imr|w,0", "<4 x float>", "ldr x17, $0"),
    ("=*imr|w,0", "<2 x double>", "ldr x17, $0"),
    ("=*imr|w,0", "i128", "ldr x17, $0"),
    ("=*w|imr,0", "<4 x float>", "ldr x17, $0"),
    // Tied by an input with alternatives: a general register.
    ("=*imr|w,0|0", "i32", "mov x17, ${0:x}"),
    ("=*imr|w,0|0", "i64", "mov x17, ${0:x}"),
    ("=*imr|w,0|0", "ptr", "mov x17, ${0:x}"),
    // Every alternative invalid for the pointer: the first.
    ("=*w|r,0", "<4 x float>", "fmov d17, ${0:d}"),
    ("=*w|r,0", "double", "fmov d17, ${0:d}"),
    ("=*r|w,0", "i64", "mov x17, ${0:x}"),
    // Direct outputs weigh nothing: the first alternative.
    ("=r|w,0", "i32", "mov x17, ${0:x}"),
    ("=r|w,0", "i64", "mov x17, ${0:x}"),
    ("=w|r,0", "<2 x double>", "fmov d17, ${0:d}"),
    ("=w|r,0", "double", "fmov d17, ${0:d}"),
    // Inputs, weighed by their values.
    ("imr|w", "i64", "ldr x17, $0"),
    ("imr|w", "double", "ldr x17, $0"),
    ("imr|w", "<4 x float>", "ldr x17, $0"),
    ("r|w", "i64", "mov x17, ${0:x}"),
    ("r|w", "ptr", "mov x17, ${0:x}"),
    ("r|w", "double", "fmov d17, ${0:d}"),
    ("r|w", "<4 x float>", "fmov d17, ${0:d}"),
    ("w|r", "i64", "mov x17, ${0:x}"),
    ("w|r", "double", "fmov d17, ${0:d}"),
    ("r|m", "i64", "ldr x17, $0"),
    ("m|r", "i64", "ldr x17, $0"),
    // kiln's templates of `barrier.cpp`.
    ("=*r|m,r|r", "i64", "str $1, $0"),
];

/// The `.ll` function of grid case `n`.
fn grid_function(n: usize, (constraints, t, insn): (&str, &str, &str)) -> String {
    let clobbers = "~{x17},~{d17}";
    let operands = constraints.split(',').count();
    let template = format!("brk #0x77\\0A\\09{insn}");
    let mut f = String::new();
    if constraints.starts_with("=*") {
        let inputs: Vec<String> = (1..operands).map(|i| format!("{t} %v{i}")).collect();
        let params = std::iter::once("ptr %p".to_string()).chain(inputs.iter().cloned());
        let _ = write!(
            f,
            "define void @g{n}({}) {{\n  call void asm sideeffect \"{template}\", \
             \"{constraints},{clobbers}\"(ptr elementtype({t}) %p{}{})\n  ret void\n}}\n",
            params.collect::<Vec<_>>().join(", "),
            if inputs.is_empty() { "" } else { ", " },
            inputs.join(", ")
        );
    } else if constraints.starts_with('=') {
        let _ = write!(
            f,
            "define {t} @g{n}({t} %v) {{\n  %r = call {t} asm sideeffect \"{template}\", \
             \"{constraints},{clobbers}\"({t} %v)\n  ret {t} %r\n}}\n"
        );
    } else {
        let _ = write!(
            f,
            "define void @g{n}({t} %v) {{\n  call void asm sideeffect \"{template}\", \
             \"{constraints},{clobbers}\"({t} %v)\n  ret void\n}}\n"
        );
    }
    f
}

/// The class of one disassembled operand: `mem`, `gpr` or `fp`.
fn class(operand: &str) -> &'static str {
    if operand.starts_with('[') {
        "mem"
    } else if operand.starts_with(['x', 'w']) {
        "gpr"
    } else {
        "fp"
    }
}

/// Per function `g<n>`, the classes of the operands of the instruction
/// after the `brk`, as `llvm-objdump -d` prints them (the template's own
/// `x17`/`d17` left out).
fn template_classes(objdump: &Path, object: &Path) -> Vec<(String, String)> {
    let mut command = Command::new(objdump);
    command.args(["-d", "--no-show-raw-insn"]).arg(object);
    let ran = run_with(&mut command, &[], Duration::from_mins(1));
    assert!(ran.success(), "llvm-objdump: {}", ran.stderr_text());
    let mut out = Vec::new();
    let mut function = String::new();
    let mut after_brk = false;
    for line in ran.stdout_text().lines() {
        if let Some(name) = line.split_once(" <").and_then(|(_, r)| r.strip_suffix(">:")) {
            function = name.to_string();
            continue;
        }
        // `   1c:      \tbrk\t#0x77`: an address, then the instruction.
        let Some((address, insn)) = line.split_once(':') else { continue };
        if address.trim().is_empty() || !address.trim().bytes().all(|b| b.is_ascii_hexdigit()) {
            continue;
        }
        let insn = insn.split("//").next().unwrap_or(insn).trim();
        if insn.starts_with("brk") {
            after_brk = true;
            continue;
        }
        if after_brk {
            after_brk = false;
            let operands = insn.split_once(char::is_whitespace).map_or("", |(_, o)| o.trim());
            let mut classes = Vec::new();
            let mut rest = operands;
            while !rest.is_empty() {
                // A bracketed address is one operand.
                let end = if rest.starts_with('[') {
                    rest.find(']').map_or(rest.len(), |e| e + 1)
                } else {
                    rest.find(',').unwrap_or(rest.len())
                };
                let operand = rest[..end].trim();
                if !matches!(operand, "x17" | "d17") {
                    classes.push(class(operand));
                }
                rest = rest[end..].trim_start_matches([',', ' ']);
            }
            out.push((function.clone(), classes.join(",")));
        }
    }
    out
}

/// Every grid case: the classes of the template's operands in hod's
/// objects are those in `llc` 19's and 22's.
#[test]
fn alternatives_are_llcs_choice() {
    let dir = Scratch::new("a64-asm-alt-grid");
    let mut text = ll_header(22);
    for (n, c) in GRID.iter().enumerate() {
        text.push_str(&grid_function(n, *c));
    }
    let (m, bodies) = parse(&text);
    let ours: Vec<PathBuf> = options()
        .iter()
        .map(|(who, options)| {
            write_object(&dir, &format!("grid-{who}.o"), &emit(&m, &bodies, options))
        })
        .collect();
    let Some(objdump) = tool("llvm-objdump") else { return };
    let mut compared = 0;
    for major in LLVM_MAJORS {
        let Some(theirs) = llc(&dir, major, &format!("grid-llc-{major}"), &text) else { continue };
        let expected = template_classes(&objdump, &theirs);
        assert_eq!(expected.len(), GRID.len(), "llc {major}: {expected:?}");
        for object in &ours {
            let got = template_classes(&objdump, object);
            assert_eq!(got.len(), expected.len());
            for ((f, want), (g, have)) in expected.iter().zip(&got) {
                let (constraints, ty, _) = GRID[f[1..].parse::<usize>().unwrap()];
                assert_eq!((f, want), (g, have), "llc {major}: {constraints} {ty}");
            }
        }
        compared += 1;
        let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
        for (_, c) in &expected {
            *by_class.entry(c.as_str()).or_default() += 1;
        }
        println!("llc {major}: {} cases, classes: {by_class:?}", expected.len());
    }
    assert!(compared > 0 || LLVM_MAJORS.iter().all(|m| tool(&format!("llc-{m}")).is_none()));
}

/// What the choice does not cover is refused by name; kiln's barrier on
/// every type compiles.
#[test]
fn what_is_not_measured_is_refused_by_name() {
    let void_asm = |cons: &str, ty: &str, input: &str| {
        format!(
            "define void @f(ptr %p, {input} %v) {{\n  call void asm \"\", \"{cons}\"(ptr \
             elementtype({ty}) %p, {input} %v)\n  ret void\n}}\n"
        )
    };
    let refused = |r: Result<(), CgError>| match r {
        Err(CgError::Unsupported { construct }) => construct,
        other => panic!("{other:?}"),
    };
    // An operand with more alternatives than one but fewer than another.
    assert_eq!(refused(compile_ll(&void_asm("=*r|m|w,0|0", "i32", "i32"))), barrier::ALTERNATIVE);
    // A tie in effect between operands of different types; without the
    // input's alternatives the tie is dropped (memory): that compiles.
    assert_eq!(refused(compile_ll(&void_asm("=*r|m,0|0", "i32", "i64"))), barrier::ALTERNATIVE);
    compile_ll(&void_asm("=*r|m,0", "i32", "i64")).unwrap();
    compile_ll(&void_asm("=*m|r,0", "i32", "i64")).unwrap();
    // An `llc` parse error: a digit with alternatives naming an output
    // without them, with an empty template and with one.
    assert_eq!(refused(compile_ll(&void_asm("=*imr,0|0", "i32", "i32"))), "asm:constraint/tied");
    let text = void_asm("=*r,0|0", "i64", "i64").replace("\"\"", "\"str x17, $0\"");
    assert_eq!(refused(compile_ll(&text)), "asm:constraint/tied");
    // kiln's barrier on every type it writes it for.
    for ty in ["i8", "i32", "i64", "i128", "float", "double", "ptr", "<4 x float>", "<2 x double>"]
    {
        compile_ll(&void_asm("=*imr|w,0", ty, ty)).unwrap_or_else(|e| panic!("{ty}: {e}"));
    }
}

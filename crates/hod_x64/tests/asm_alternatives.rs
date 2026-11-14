//! Multi-alternative inline-assembly constraints on x86-64 (WP-219,
//! `spec/asm.md` §19): which alternative and which code of it `llc` 19
//! and 22 take, for a grid of shapes and operand types, compared on the
//! objects (the class of each template operand: memory, a general or an
//! SSE register); what an empty template does to an indirect output's
//! memory (`tests/data/asm_alternatives/semantics.ll`, run); the barrier
//! `"+g,x"` and templates with alternatives as kiln compiles C++
//! (`tests/data/asm_alternatives/barrier.cpp`, at `-O0` and `-O0 -g`),
//! built by hod (plain, stressed, far, large code model) and by `llc`, linked
//! by every linker, dynamically and statically, and run; and what is not
//! measured, refused by name.

#[path = "../../hod_obj/tests/common/linking.rs"]
#[allow(dead_code, reason = "a shared helper: not every function is used here")]
mod linking;

#[path = "common/int_gen.rs"]
mod int_gen;

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use hod_cg::{CgError, Options, Stress};
use hod_ir::CodeModel;
use hod_testutil::{LLVM_MAJORS, Scratch, run, tool};
use hod_x64::barrier;
use int_gen::{ll_header, llc, parse, pic};
use linking::{Toolchain, exit_code};

const SEMANTICS: &str = include_str!("data/asm_alternatives/semantics.ll");
const SEMANTICS_DRIVER: &str = include_str!("data/asm_alternatives/semantics.c");
/// kiln's `-O0` and `-O0 -g` modules of `barrier.cpp`.
const BARRIER: &str = include_str!("data/asm_alternatives/barrier.ll");
const BARRIER_G: &str = include_str!("data/asm_alternatives/barrier-g.ll");

fn hod_object(text: &str, options: &Options) -> Vec<u8> {
    let (m, bodies) = parse(text);
    hod_x64::compile_to_object(&m, &bodies, options).unwrap_or_else(|e| panic!("hod: {e}"))
}

/// hod plainly, stressed and with the large code model, then `llc` of
/// each major present: the objects of `text`.
fn objects(dir: &Scratch, name: &str, text: &str) -> Vec<(String, PathBuf)> {
    let far = pic(Stress { min_pool: false, far: true, displace: false, coro_frame: false });
    let large = Options { code_model: Some(CodeModel::Large), ..pic(Stress::default()) };
    let mut out = Vec::new();
    for (who, options) in [
        ("hod", pic(Stress::default())),
        ("stress", pic(Stress::ALL)),
        ("far", far),
        ("large", large),
    ] {
        out.push((
            who.to_string(),
            dir.write(&format!("{name}-{who}.o"), hod_object(text, &options)),
        ));
    }
    for major in LLVM_MAJORS {
        if let Some(theirs) = llc(dir, major, &format!("{name}-llc-{major}"), text) {
            out.push((format!("llc-{major}"), theirs));
        }
    }
    out
}

/// Links each object (with `driver`, if any) by every linker found,
/// dynamically and statically, runs it on an emulated CPU with AVX, and
/// requires "checked, 0 failures".
fn run_everywhere(dir: &Scratch, objects: &[(String, PathBuf)], driver: Option<&str>) {
    let Some(toolchain) = Toolchain::x86_64(dir) else { return };
    let toolchain = toolchain.with_emulated_cpu("max");
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
    let dir = Scratch::new("x64-asm-alt-semantics");
    let text = format!("{}{SEMANTICS}", ll_header());
    let objects = objects(&dir, "semantics", &text);
    run_everywhere(&dir, &objects, Some(SEMANTICS_DRIVER));
}

/// kiln's barrier `"+g,x"` on `int`, `long`, `short`, `char`, `float`,
/// `double`, `__int128`, a pointer, `__m128`, `__m128d`, `__m128i`,
/// `__m256` (in an AVX function), and templates with alternatives, at
/// `-O0` and with `-g`.
#[test]
fn kiln_barriers_run_as_llc_builds_them() {
    let dir = Scratch::new("x64-asm-alt-kiln");
    let mut all = objects(&dir, "barrier", BARRIER);
    all.extend(
        objects(&dir, "barrier-g", BARRIER_G).into_iter().map(|(w, o)| (format!("{w}-g"), o)),
    );
    run_everywhere(&dir, &all, None);
}

/// One function of the grid: the constraint string (without clobbers),
/// the operand type, the template's instruction (`int3` before it marks
/// it), and whether the function needs AVX.
struct Case {
    constraints: &'static str,
    ty: &'static str,
    template: &'static str,
    avx: bool,
}

const fn case(constraints: &'static str, ty: &'static str, template: &'static str) -> Case {
    Case { constraints, ty, template, avx: false }
}

/// The grid (`spec/asm.md` §19 has the table it measures).
const GRID: &[Case] = &[
    // The barrier's shape: untied, memory, whatever the type.
    case("=*imr|x,0", "i8", "notb $0"),
    case("=*imr|x,0", "i16", "notw $0"),
    case("=*imr|x,0", "i32", "notl $0"),
    case("=*imr|x,0", "i64", "notq $0"),
    case("=*imr|x,0", "float", "notl $0"),
    case("=*imr|x,0", "double", "notq $0"),
    case("=*imr|x,0", "ptr", "notq $0"),
    case("=*imr|x,0", "<4 x float>", "movups $0, %xmm13"),
    case("=*imr|x,0", "<2 x double>", "movups $0, %xmm13"),
    case("=*imr|x,0", "<4 x i32>", "movups $0, %xmm13"),
    case("=*imr|x,0", "i128", "movups $0, %xmm13"),
    case("=*x|imr,0", "<4 x float>", "movups $0, %xmm13"),
    case("=*x|imr,0", "i32", "notl $0"),
    case("=*m|x,0", "double", "notq $0"),
    // Tied by an input with alternatives: a general register.
    case("=*imr|x,0|0", "i32", "notl $0"),
    case("=*imr|x,0|0", "i64", "notq $0"),
    case("=*imr|x,0|0", "float", "notl $0"),
    case("=*imr|x,0|0", "double", "notq $0"),
    case("=*imr|x,0|0", "ptr", "notq $0"),
    // Every alternative invalid for the pointer: the first.
    case("=*x|r,0", "<4 x float>", "movups $0, %xmm13"),
    case("=*x|r,0", "<2 x double>", "movups $0, %xmm13"),
    case("=*x|r,0", "float", "movups $0, %xmm13"),
    case("=*x|r,0", "double", "movups $0, %xmm13"),
    case("=*x|r,0", "i64", "movups $0, %xmm13"),
    case("=*r|x,0", "i32", "notl $0"),
    case("=*r|x,0", "i64", "notq $0"),
    case("=*r|x,0", "float", "notl $0"),
    case("=*r|x,0", "ptr", "notq $0"),
    // Direct outputs weigh nothing: the first alternative.
    case("=r|x,0", "i32", "notl $0"),
    case("=r|x,0", "i64", "notq $0"),
    case("=r|x,0", "float", "notl $0"),
    case("=r|x,0", "double", "notq $0"),
    case("=x|r,0", "float", "movups $0, %xmm13"),
    case("=x|r,0", "<4 x float>", "movups $0, %xmm13"),
    case("=x|r,0", "<2 x double>", "movups $0, %xmm13"),
    case("=x|r,0", "i64", "movups $0, %xmm13"),
    case("=x|r,0|0", "i32", "movups $0, %xmm13"),
    // Inputs, weighed by their values.
    case("imr|x", "i32", "notl $0"),
    case("imr|x", "i64", "notq $0"),
    case("imr|x", "float", "notl $0"),
    case("imr|x", "double", "notq $0"),
    case("imr|x", "ptr", "notq $0"),
    case("imr|x", "<4 x float>", "movups $0, %xmm13"),
    case("imr|x", "<2 x double>", "movups $0, %xmm13"),
    // (`i128` in an SSE register, `llc`'s choice for `imr|x`, `r|x` and
    // `rm|x`, stays `x64:inline-asm-operand-type`: `what_is_not_measured`.)
    case("r|x", "i32", "notl $0"),
    case("r|x", "float", "notl $0"),
    case("r|x", "double", "notq $0"),
    case("r|x", "ptr", "notq $0"),
    case("r|x", "<4 x float>", "movups $0, %xmm13"),
    case("r|x", "<2 x double>", "movups $0, %xmm13"),
    case("x|r", "i32", "notl $0"),
    case("x|r", "i64", "notq $0"),
    case("x|r", "float", "movups $0, %xmm13"),
    case("x|r", "double", "movups $0, %xmm13"),
    case("x|r", "<4 x float>", "movups $0, %xmm13"),
    case("rm|x", "i32", "notl $0"),
    case("rm|x", "<4 x float>", "movups $0, %xmm13"),
    case("x|m", "double", "notq $0"),
    case("x|m", "<2 x double>", "movups $0, %xmm13"),
    case("m|x", "<2 x double>", "movups $0, %xmm13"),
    case("r|m", "i32", "notl $0"),
    case("m|r", "i32", "notl $0"),
    case("r|m", "double", "notq $0"),
    // kiln's templates of `barrier.cpp`.
    case("=*r|m,ri|r", "i32", "movl $1, $0"),
    case("=*x|m,x|x", "float", "movss $1, $0"),
    case("=*r|m,r|r,0", "i64", "addq $1, $0"),
    // An AVX function: the same choice for 128 bits. (A 256-bit operand
    // in a template is refused, `x64:inline-asm-operand-type`; the empty
    // barrier on one is `kiln_barriers_run_as_llc_builds_them`'s.)
    Case { constraints: "imr|x", ty: "<4 x float>", template: "movups $0, %xmm13", avx: true },
    Case { constraints: "x|imr", ty: "<2 x double>", template: "movups $0, %xmm13", avx: true },
];

/// The `.ll` function of grid case `n`.
fn grid_function(n: usize, c: &Case) -> String {
    let clobbers = "~{xmm13},~{dirflag},~{fpsr},~{flags}";
    let operands = c.constraints.split(',').count();
    let template = format!("int3\\0A\\09{}", c.template);
    let attrs = if c.avx { " #0" } else { "" };
    let t = c.ty;
    let mut f = String::new();
    if c.constraints.starts_with("=*") {
        // An indirect output and as many inputs of the type as follow.
        let inputs: Vec<String> = (1..operands).map(|i| format!("{t} %v{i}")).collect();
        let params = std::iter::once("ptr %p".to_string()).chain(inputs.iter().cloned());
        let _ = write!(
            f,
            "define void @g{n}({}){attrs} {{\n  call void asm sideeffect \"{template}\", \
             \"{},{clobbers}\"(ptr elementtype({t}) %p{}{})\n  ret void\n}}\n",
            params.collect::<Vec<_>>().join(", "),
            c.constraints,
            if inputs.is_empty() { "" } else { ", " },
            inputs.join(", ")
        );
    } else if c.constraints.starts_with('=') {
        let _ = write!(
            f,
            "define {t} @g{n}({t} %v){attrs} {{\n  %r = call {t} asm sideeffect \"{template}\", \
             \"{},{clobbers}\"({t} %v)\n  ret {t} %r\n}}\n",
            c.constraints
        );
    } else {
        let _ = write!(
            f,
            "define void @g{n}({t} %v){attrs} {{\n  call void asm sideeffect \"{template}\", \
             \"{},{clobbers}\"({t} %v)\n  ret void\n}}\n",
            c.constraints
        );
    }
    f
}

/// The class of one disassembled operand: `mem`, `xmm`, `ymm` or `gpr`.
fn class(operand: &str) -> &'static str {
    if operand.contains('(') {
        "mem"
    } else if operand.starts_with("%xmm") {
        "xmm"
    } else if operand.starts_with("%ymm") {
        "ymm"
    } else {
        "gpr"
    }
}

/// Per function `g<n>`, the classes of the operands of the instruction
/// after the `int3`, as `llvm-objdump -d` prints them (the template's
/// own `%xmm13`/`%ymm13` left out).
fn template_classes(objdump: &Path, object: &Path) -> Vec<(String, String)> {
    let ran = run(objdump, ["-d".as_ref(), "--no-show-raw-insn".as_ref(), object.as_os_str()]);
    assert!(ran.success(), "llvm-objdump: {}", ran.stderr_text());
    let mut out = Vec::new();
    let mut function = String::new();
    let mut after_int3 = false;
    for line in ran.stdout_text().lines() {
        if let Some(name) = line.split_once(" <").and_then(|(_, r)| r.strip_suffix(">:")) {
            function = name.to_string();
            continue;
        }
        // `   1c:      \tint3`: an address, then the instruction.
        let Some((address, insn)) = line.split_once(':') else { continue };
        if address.trim().is_empty() || !address.trim().bytes().all(|b| b.is_ascii_hexdigit()) {
            continue;
        }
        let insn = insn.trim();
        if insn == "int3" {
            after_int3 = true;
            continue;
        }
        if after_int3 {
            after_int3 = false;
            let operands = insn.split_once(char::is_whitespace).map_or("", |(_, o)| o.trim());
            let classes: Vec<&str> = split_operands(operands)
                .into_iter()
                .filter(|o| !matches!(*o, "%xmm13" | "%ymm13"))
                .map(class)
                .collect();
            out.push((function.clone(), classes.join(",")));
        }
    }
    out
}

/// AT&T operands, split at the commas outside parentheses.
fn split_operands(text: &str) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0, 0);
    for (i, b) in text.bytes().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b',' if depth == 0 => {
                out.push(text[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if !text[start..].trim().is_empty() {
        out.push(text[start..].trim());
    }
    out
}

/// Every grid case: the classes of the template's operands in hod's
/// object are those in `llc` 19's and 22's.
#[test]
fn alternatives_are_llcs_choice() {
    let dir = Scratch::new("x64-asm-alt-grid");
    let mut text = ll_header();
    for (n, c) in GRID.iter().enumerate() {
        text.push_str(&grid_function(n, c));
    }
    text.push_str("attributes #0 = { \"target-features\"=\"+avx\" }\n");
    let ours = dir.write("grid-hod.o", hod_object(&text, &pic(Stress::default())));
    let stressed = dir.write("grid-stress.o", hod_object(&text, &pic(Stress::ALL)));
    let mut compared = 0;
    for major in LLVM_MAJORS {
        let Some(objdump) = tool(&format!("llvm-objdump-{major}")) else { continue };
        let Some(theirs) = llc(&dir, major, &format!("grid-llc-{major}"), &text) else { continue };
        let expected = template_classes(&objdump, &theirs);
        assert_eq!(expected.len(), GRID.len(), "llc {major}: {expected:?}");
        for object in [&ours, &stressed] {
            let got = template_classes(&objdump, object);
            for ((f, want), (g, have)) in expected.iter().zip(&got) {
                let n: usize = f[1..].parse().unwrap();
                let c = &GRID[n];
                assert_eq!((f, want), (g, have), "llc {major}: {} {}", c.constraints, c.ty);
            }
            assert_eq!(got.len(), expected.len());
        }
        compared += 1;
        let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
        for (_, c) in &expected {
            *by_class.entry(c.as_str()).or_default() += 1;
        }
        println!("llc {major}: {} cases, hod's classes: {by_class:?}", expected.len());
    }
    assert!(compared > 0 || LLVM_MAJORS.iter().all(|m| tool(&format!("llc-{m}")).is_none()));
}

/// What the choice does not cover is refused by name; kiln's barrier on a
/// vector compiles.
#[test]
fn what_is_not_measured_is_refused_by_name() {
    let compile = |body: &str, attrs: &str| {
        let text = format!("{}{body}\nattributes #0 = {{ {attrs} }}\n", ll_header());
        let (m, bodies) = parse(&text);
        hod_x64::compile_to_object(&m, &bodies, &pic(Stress::default())).map(|_| ())
    };
    let refused = |r: Result<(), hod_x64::ModuleError>| match r {
        Err(hod_x64::ModuleError::Function {
            error: CgError::Unsupported { construct }, ..
        }) => construct,
        other => panic!("not refused by name: {other:?}"),
    };
    let void_asm = |cons: &str, ty: &str| {
        format!(
            "define void @f(ptr %p, {ty} %v) #0 {{\n  call void asm \"\", \"{cons}\"(ptr \
             elementtype({ty}) %p, {ty} %v)\n  ret void\n}}"
        )
    };
    // An operand with more alternatives than one but fewer than another.
    assert_eq!(refused(compile(&void_asm("=*r|m|x,0|0", "i32"), "nounwind")), barrier::ALTERNATIVE);
    assert_eq!(
        refused(compile(
            &void_asm("=*r|m|x,0|0", "i32").replace("\"\"", "\"notl $0\""),
            "nounwind"
        )),
        barrier::ALTERNATIVE
    );
    // A tie in effect between operands of different types. Without the
    // input's alternatives the tie is dropped and the output is memory
    // (`llc` 19 and 22 write nothing; Benchmark's `DoNotOptimize` of a
    // struct in kiln's `dcl.asm` entry): that compiles.
    let mixed = |cons: &str| {
        format!(
            "define void @f(ptr %p, i64 %v) #0 {{\n  call void asm \"\", \"{cons}\"(ptr \
             elementtype(i32) %p, i64 %v)\n  ret void\n}}"
        )
    };
    assert_eq!(refused(compile(&mixed("=*r|m,0|0"), "nounwind")), barrier::ALTERNATIVE);
    compile(&mixed("=*r|m,0"), "nounwind").unwrap();
    compile(&mixed("=*m|r,0"), "nounwind").unwrap();
    // A 256-bit operand `x` weighs while AVX is only assumed.
    let wide = "define void @f(ptr %q) #0 {\n  %v = load <8 x float>, ptr %q\n  call void asm \"\", \
                \"imr|x\"(<8 x float> %v)\n  ret void\n}";
    assert_eq!(
        refused(compile(wide, "\"target-features\"=\"+avx10.1-256\"")),
        barrier::ALTERNATIVE
    );
    assert!(compile(wide, "\"target-features\"=\"+avx\"").is_ok());
    // An `llc` parse error: a digit with alternatives naming an output
    // without them.
    assert_eq!(refused(compile(&void_asm("=*imr,0|0", "i32"), "nounwind")), barrier::CONSTRAINT);
    // `i128` in an SSE register in a template (`llc`'s choice for
    // `imr|x`) is not taken, as before.
    let i128_x = "define void @f(i128 %v) #0 {\n  call void asm \"movups $0, %xmm13\", \
                  \"imr|x,~{xmm13}\"(i128 %v)\n  ret void\n}";
    assert_eq!(refused(compile(i128_x, "nounwind")), barrier::template::OPERAND_TYPE);
    // kiln's barrier on every vector type it writes it for.
    for ty in ["<4 x float>", "<2 x double>", "<4 x i32>", "<2 x i64>", "i128", "<8 x float>"] {
        compile(&void_asm("=*imr|x,0", ty), "nounwind").unwrap_or_else(|e| panic!("{ty}: {e}"));
    }
}

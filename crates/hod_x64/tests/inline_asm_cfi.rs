//! Call-frame directives in x86-64 inline assembly (WP-212,
//! `spec/asm.md` §15): the rows a template writes join the function's
//! unwind program where they stand.
//!
//! - Each template of `CASES`, built by hod (plain and stressed) and by
//!   `llc -O0 -fast-isel=0` of LLVM 19 and 22, has the same call-frame
//!   instructions inside the template, at the same distances from its
//!   start, in `.eh_frame` and (a `nounwind` function of a `-g` module)
//!   `.debug_frame`, as `llvm-dwarfdump` prints them. Each template
//!   starts and ends with a marker instruction, found in `.text`.
//! - A template that moves the CFA as valgrind's `CALL_FN_*` macros do
//!   and calls a C++ function that throws: the exception reaches the
//!   driver's `catch` through the template (dynamic and static, bfd, lld
//!   and mold; without the template's rows it terminates), and gdb's
//!   backtrace from the callee names the caller's caller.

#[path = "../../hod_obj/tests/common/linking.rs"]
#[allow(dead_code, reason = "a shared helper: not every function is used here")]
mod linking;

#[path = "common/int_gen.rs"]
mod int_gen;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use hod_cg::Stress;
use hod_testutil::{LLVM_MAJORS, Scratch, run, run_with, tool};
use int_gen::{ll_header, llc, parse, pic};
use linking::{Toolchain, exit_code};

/// `nopl 0x7e7e7e7e(%rax)`: the marker at each end of a template.
const MARK: &[u8] = &[0x0f, 0x1f, 0x80, 0x7e, 0x7e, 0x7e, 0x7e];

/// A template between markers, as IR string text.
fn marked(body: &str) -> String {
    format!("nopl 0x7e7e7e7e(%rax)\\0A{body}\\0Anopl 0x7e7e7e7e(%rax)")
}

/// The functions compared with `llc`: a name and the body of `@f`'s
/// template (IR string text), its constraints and arguments.
const CASES: &[(&str, &str, &str)] = &[
    // valgrind.h `CALL_FN_W_W` on amd64, its CFA from `rbp` + 16 (what
    // `__builtin_dwarf_cfa()` is in a function with a frame pointer).
    (
        "valgrind",
        "movq %rbp, %r15\\0A\\09movq $2, %rbp\\0A\\09.cfi_remember_state\\0A\\09\
         .cfi_def_cfa rbp, 0\\0A\\09movq %rsp,%r14\\0A\\09andq $$0xfffffffffffffff0,%rsp\\0A\\09\
         subq $$128,%rsp\\0A\\09movq 8(%rax), %rdi\\0A\\09movq (%rax), %rax\\0A\\09\
         rolq $$3,  %rdi ; rolq $$13, %rdi\\0A\\09rolq $$61, %rdi ; rolq $$51, %rdi\\0A\\09\
         xchgq %rbx,%rbx\\0A\\09movq %r14,%rsp\\0A\\09movq %r15, %rbp\\0A\\09.cfi_restore_state",
        "={ax},{ax},r,~{memory},~{rcx},~{rdx},~{rsi},~{rdi},~{r8},~{r9},~{r10},~{r11},~{r14},\
         ~{r15},~{dirflag},~{fpsr},~{flags}",
    ),
    // Every directive of §15.
    (
        "every",
        ".cfi_remember_state\\0A.cfi_def_cfa_offset 32\\0Anop\\0A.cfi_def_cfa_register %rsp\\0A\
         nop\\0A.cfi_offset %rbx, -48\\0A.cfi_offset 16, -16\\0Anop\\0A.cfi_offset %xmm2, -64\\0A\
         .cfi_restore %rbx\\0A.cfi_def_cfa %rbp, 16\\0Anop\\0A.cfi_restore_state",
        "",
    ),
    // Nested states, across a relaxed branch.
    (
        "nested",
        ".cfi_remember_state\\0Ajmp 1f\\0A.cfi_def_cfa rbp, 0\\0Anop\\0A1: .cfi_remember_state\\0A\
         .cfi_offset %r12, -40\\0Anop\\0A.cfi_restore_state\\0Anop\\0A.cfi_restore_state",
        "",
    ),
];

/// The function of case `(body, constraints)`, with `attrs` and, when
/// `debug`, a subprogram.
fn case_ir(body: &str, constraints: &str, debug: bool) -> String {
    let (attrs, dbg, loc) =
        if debug { (" nounwind", " !dbg !5", ", !dbg !8") } else { ("", "", "") };
    let call = if constraints.is_empty() {
        format!("call void asm sideeffect \"{}\", \"\"(){loc}", marked(body))
    } else {
        format!(
            "%v = alloca [2 x i64], align 16\n  \
             %fa = call ptr @llvm.frameaddress.p0(i32 0){loc}\n  \
             %cfa = getelementptr i8, ptr %fa, i64 16\n  \
             %r = call i64 asm sideeffect \"{}\", \"{constraints}\"(ptr %v, ptr %cfa){loc}",
            marked(body)
        )
    };
    let mut ir = format!(
        "{}declare ptr @llvm.frameaddress.p0(i32)\n\
         define void @f(){attrs}{dbg} {{\n  {call}\n  ret void{loc}\n}}\n",
        ll_header()
    );
    if debug {
        ir.push_str(
            "!llvm.dbg.cu = !{!0}\n!llvm.module.flags = !{!3, !4}\n\
             !0 = distinct !DICompileUnit(language: DW_LANG_C11, file: !1, producer: \"t\", \
             isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)\n\
             !1 = !DIFile(filename: \"t.c\", directory: \"/\")\n\
             !3 = !{i32 7, !\"Dwarf Version\", i32 5}\n\
             !4 = !{i32 2, !\"Debug Info Version\", i32 3}\n\
             !5 = distinct !DISubprogram(name: \"f\", scope: !1, file: !1, line: 1, type: !6, \
             scopeLine: 1, spFlags: DISPFlagDefinition, unit: !0)\n\
             !6 = !DISubroutineType(types: !7)\n!7 = !{null}\n\
             !8 = !DILocation(line: 1, column: 1, scope: !5)\n",
        );
    }
    ir
}

fn hod_object(text: &str, stress: Stress) -> Vec<u8> {
    let (m, bodies) = parse(text);
    hod_x64::compile_to_object(&m, &bodies, &pic(stress)).unwrap_or_else(|e| panic!("hod: {e}"))
}

/// The offsets of the two markers in `object`'s `.text`.
fn markers(object: &Path) -> (u64, u64) {
    let objcopy = tool("llvm-objcopy").expect("llvm-objcopy");
    let bin = object.with_extension("text");
    let out = run(
        &objcopy,
        [
            "-O".as_ref(),
            "binary".as_ref(),
            "--only-section=.text".as_ref(),
            object.as_os_str(),
            bin.as_os_str(),
        ],
    );
    assert!(out.success(), "llvm-objcopy: {}", out.stderr_text());
    let text = std::fs::read(&bin).expect("the .text bytes");
    let found: Vec<usize> =
        text.windows(MARK.len()).enumerate().filter(|(_, w)| *w == MARK).map(|(i, _)| i).collect();
    assert_eq!(found.len(), 2, "{}: the template's markers", object.display());
    (found[0] as u64, found[1] as u64)
}

/// The call-frame instructions of `section` (`eh-frame` or
/// `debug-frame`) at code offsets from `start` to `end`, each with its
/// distance from `start`; `None` when the section has no FDE.
fn rows_between(object: &Path, section: &str, start: u64, end: u64) -> Option<Vec<String>> {
    let dump = tool("llvm-dwarfdump").expect("llvm-dwarfdump");
    let out = run(&dump, [format!("--{section}").as_ref(), object.as_os_str()]);
    assert!(out.success(), "llvm-dwarfdump: {}", out.stderr_text());
    let text = out.stdout_text();
    let mut rows = Vec::new();
    let (mut in_fde, mut seen, mut at) = (false, false, 0u64);
    for line in text.lines() {
        if line.contains(" FDE ") {
            (in_fde, seen, at) = (true, true, 0);
            continue;
        }
        if line.contains(" CIE ") {
            in_fde = false;
            continue;
        }
        let Some(op) = line.trim().strip_prefix("DW_CFA_") else { continue };
        if !in_fde || op.starts_with("nop") {
            continue;
        }
        if op.starts_with("advance_loc") {
            let to = op.rsplit("to 0x").next().expect("an address");
            at = u64::from_str_radix(to, 16).expect("a hexadecimal address");
            continue;
        }
        // After the first marker's start: a row there is the prologue's.
        if at > start && at <= end {
            rows.push(format!("+{}: {op}", at - start));
        }
    }
    seen.then_some(rows)
}

/// §15: inside each template, the instructions of `.eh_frame` and of
/// `.debug_frame` are `llc`'s, at the same distances from its start.
#[test]
fn template_rows_are_llc_rows() {
    let dir = Scratch::new("x64-asm-cfi-llc");
    if tool("llvm-dwarfdump").is_none() || tool("llvm-objcopy").is_none() {
        return;
    }
    let mut compared = 0;
    for &(name, body, constraints) in CASES {
        for (debug, section) in [(false, "eh-frame"), (true, "debug-frame")] {
            let text = case_ir(body, constraints, debug);
            let tag = format!("{name}-{section}");
            let mut theirs = Vec::new();
            for major in LLVM_MAJORS {
                let Some(object) = llc(&dir, major, &format!("{tag}-llc{major}"), &text) else {
                    continue;
                };
                let (start, end) = markers(&object);
                let rows = rows_between(&object, section, start, end);
                assert!(rows.as_ref().is_some_and(|r| !r.is_empty()), "{tag}: llc {major}");
                theirs.push((major, rows));
            }
            for (who, stress) in [("hod", Stress::default()), ("stress", Stress::ALL)] {
                let object = dir.write(&format!("{tag}-{who}.o"), hod_object(&text, stress));
                let (start, end) = markers(&object);
                let ours = rows_between(&object, section, start, end);
                for (major, rows) in &theirs {
                    assert_eq!(&ours, rows, "{tag}: {who} against llc {major}");
                    compared += 1;
                }
            }
        }
    }
    eprintln!("template rows compared with llc: {compared}");
}

/// The function whose template calls `leaf` with the CFA moved, as
/// valgrind's macros do around their call.
const THROUGH: &str = "declare i64 @leaf(i64)
declare ptr @llvm.frameaddress.p0(i32)
define i64 @through(i64 %x) {
entry:
  %fa = call ptr @llvm.frameaddress.p0(i32 0)
  %cfa = getelementptr i8, ptr %fa, i64 16
  %r = call i64 asm sideeffect \"movq %rbp, %r15\\0Amovq $2, %rbp\\0A.cfi_remember_state\\0A\
.cfi_def_cfa rbp, 0\\0Acall leaf\\0Amovq %r15, %rbp\\0A.cfi_restore_state\", \
\"={ax},{di},r,~{rcx},~{rdx},~{rsi},~{rdi},~{r8},~{r9},~{r10},~{r11},~{r15},~{xmm0},~{xmm1},\
~{xmm2},~{xmm3},~{xmm4},~{xmm5},~{xmm6},~{xmm7},~{xmm8},~{xmm9},~{xmm10},~{xmm11},~{xmm12},\
~{xmm13},~{xmm14},~{xmm15},~{memory},~{dirflag},~{fpsr},~{flags}\"(i64 %x, ptr %cfa)
  %s = add i64 %r, 1
  ret i64 %s
}
";

const THROUGH_DRIVER: &str = "extern \"C\" long leaf(long x) {
  if (x == 42) throw 7;
  return x + 1;
}
extern \"C\" long through(long);
int main() {
  if (through(1) != 3) return 1;
  try {
    through(42);
    return 2;
  } catch (int v) {
    if (v != 7) return 3;
  }
  return 0;
}
";

/// The objects of `THROUGH` (hod plain and stressed), and `THROUGH`
/// without the template's rows. `llc` keeps no 16-byte alignment of
/// `rsp` at a template (it pushes `r15` here), so its build of the call
/// is not a program: the rows are compared above.
fn through_objects(dir: &Scratch) -> (Vec<(String, PathBuf)>, PathBuf) {
    let text = format!("{}{THROUGH}", ll_header());
    let mut objects = Vec::new();
    for (who, stress) in [("hod", Stress::default()), ("stress", Stress::ALL)] {
        objects.push((who.to_owned(), dir.write(&format!("{who}.o"), hod_object(&text, stress))));
    }
    let bare = text
        .replace("\\0A.cfi_remember_state\\0A.cfi_def_cfa rbp, 0", "")
        .replace("\\0A.cfi_restore_state", "");
    let bare = dir.write("bare.o", hod_object(&bare, Stress::default()));
    (objects, bare)
}

/// §15: an exception thrown by a function the template calls unwinds
/// through it to the driver's `catch`, with every linker, dynamic and
/// static; the same template without its rows terminates the program.
#[test]
fn an_exception_unwinds_through_the_template() {
    let dir = Scratch::new("x64-asm-cfi-throw");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) = toolchain.compile_cxx(&dir, "driver", THROUGH_DRIVER, &["-fPIC"]) else {
        return;
    };
    let (objects, bare) = through_objects(&dir);
    let stdcxx = Path::new("-lstdc++");
    let mut runs = 0;
    for linker in &toolchain.linkers {
        for (who, object) in &objects {
            for (how, flags) in [("dynamic", &[][..]), ("static", &["-static"][..])] {
                let program = dir.join(format!("{who}-{how}-{}", linker.name));
                let ran =
                    toolchain.link_and_run(linker, flags, &[&driver, object, stdcxx], &program);
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
    let linker = &toolchain.linkers[0];
    let program = dir.join("bare");
    let ran = toolchain.link_and_run(linker, &["-static"], &[&driver, &bare, stdcxx], &program);
    assert_ne!(exit_code(&ran), Some(0), "without the template's rows the exception is caught");
    eprintln!("programs that caught the exception through the template: {runs}");
}

/// gdb's backtrace from `leaf`, called inside the template: `leaf`,
/// `through`, `main`.
fn backtrace(toolchain: &Toolchain, dir: &Scratch, program: &Path, port: u16) -> Option<String> {
    let commands = "-ex 'set pagination off' -ex 'set confirm off' \
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
            "{line} &\nq=$!\n{} -nx -batch {commands} -ex 'set architecture i386:x86-64' \
             -ex 'file {}' -ex 'target remote localhost:{port}' -ex 'break leaf' -ex continue -ex continue \
             -ex bt -ex kill 2>&1\nkill $q 2>/dev/null\nwait $q 2>/dev/null\nexit 0\n",
            gdb.display(),
            program.display()
        )
    } else {
        let gdb = tool("gdb")?;
        format!(
            "{} -nx -batch {commands} -ex 'break leaf' -ex run -ex continue -ex bt -ex kill {} 2>&1\n",
            gdb.display(),
            program.display()
        )
    };
    let script = dir.write_script(&format!("gdb-{port}.sh"), &script);
    let out = run_with(&mut Command::new(script), &[], Duration::from_mins(3));
    assert!(!out.timed_out(), "gdb on {}", program.display());
    Some(out.stdout_text())
}

/// The frames of a gdb backtrace, by name.
fn frames(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| l.starts_with('#'))
        .filter_map(|l| l.split(" in ").nth(1).or_else(|| l.split_whitespace().nth(1)))
        .map(|f| f.split_whitespace().next().unwrap_or("").to_owned())
        .collect()
}

/// §15: gdb walks the stack through the template from its callee: the
/// second stop in `leaf` (the call that throws) has `through` and `main`
/// above it. Without the template's rows `main` is lost.
#[test]
fn gdb_walks_through_the_template() {
    let dir = Scratch::new("x64-asm-cfi-gdb");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) = toolchain.compile_cxx(&dir, "driver", THROUGH_DRIVER, &["-fPIC"]) else {
        return;
    };
    let (objects, bare) = through_objects(&dir);
    let stdcxx = Path::new("-lstdc++");
    let linker = &toolchain.linkers[0];
    let first_port = 20000 + (std::process::id() % 20000) as u16;
    let mut walked = 0;
    for (n, (who, object)) in
        objects.iter().map(|(w, o)| (w.as_str(), o)).chain([("bare", &bare)]).enumerate()
    {
        let program = dir.join(format!("gdb-{who}"));
        toolchain.link_ok(linker, &["-static"], &[&driver, object, stdcxx], &program);
        let (mut got, mut text) = (Vec::new(), String::new());
        for attempt in 0..3u16 {
            let port = first_port + 50 * attempt + n as u16;
            let Some(out) = backtrace(&toolchain, &dir, &program, port) else { return };
            text = out;
            got = frames(&text);
            if !got.is_empty() {
                break;
            }
        }
        let want = ["leaf", "through", "main"];
        if who == "bare" {
            assert_ne!(got.get(..3), Some(&want.map(String::from)[..]), "bare: {got:?}");
        } else {
            assert_eq!(got.get(..3), Some(&want.map(String::from)[..]), "{who}: {text}");
            walked += 1;
        }
    }
    eprintln!("backtraces through the template: {walked}");
}

//! The frame as an unwinder sees it on x86-64 (WP-215,
//! `spec/codegen.md` §5.6.38): `llvm.eh.dwarf.cfa`,
//! `llvm.addressofreturnaddress`, `llvm.eh.unwind.init`,
//! `llvm.eh.return.i64`, and `llvm.frameaddress`/`llvm.returnaddress`
//! above depth 0 beside them.
//!
//! - One program, built by hod (plain and stressed) and by `llc -O0` of
//!   LLVM 19 and 22, linked with one C++ driver by bfd, lld and mold and
//!   run: a function walks its own frame chain to depth 3 and names the
//!   functions the return addresses are in; `__builtin_dwarf_cfa()` is
//!   the caller's stack pointer before the call (plain, in a realigned
//!   frame, after a VLA, in a variadic function, with offsets); the
//!   saved return address is where `llvm.addressofreturnaddress` says; a
//!   function returns into a label of its caller with `__builtin_eh_return`,
//!   and an unwinder step skips a frame with it; `__builtin_unwind_init`
//!   puts the five callee-saved registers in the frame; valgrind's call
//!   macro (a template that points `rbp` at the CFA taken with
//!   `llvm.eh.dwarf.cfa`, realigns `rsp` and calls) lets an exception
//!   through. Every build prints the same.
//! - The unwind rows of a function with both `eh.unwind.init` and
//!   `eh.return` save the registers `llc` saves (`rax` and `rdx`
//!   included).
//! - A function with `eh.return` that also returns a value keeps it
//!   (hod; `llc` 19 and 22 pop `rax` over it).
//! - gdb walks from the callee of the template to the driver.

#[path = "../../hod_obj/tests/common/linking.rs"]
#[allow(dead_code, reason = "a shared helper: not every function is used here")]
mod linking;

#[path = "common/int_gen.rs"]
mod int_gen;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use hod_cg::Stress;
use hod_testutil::{LLVM_MAJORS, Scratch, run, run_with, tool};
use int_gen::{ll_header, llc, parse, pic};
use linking::{Toolchain, exit_code};

/// The functions under test. `#0` keeps a frame pointer in `llc`'s
/// builds, which its frame walks assume of every caller.
const FUNCTIONS: &str = r#"
@target_pc = global ptr null
@target_sp = global ptr null
@target_fp = global ptr null
@never = global i32 0

declare ptr @llvm.frameaddress.p0(i32)
declare ptr @llvm.returnaddress(i32)
declare ptr @llvm.eh.dwarf.cfa(i32)
declare ptr @llvm.addressofreturnaddress.p0()
declare ptr @llvm.stacksave.p0()
declare void @llvm.eh.unwind.init()
declare void @llvm.eh.return.i64(i64, ptr)
declare void @llvm.va_start.p0(ptr)
declare void @llvm.va_end.p0(ptr)
declare void @sink(ptr)
declare i64 @count_magic(ptr, ptr)
declare i64 @leaf(i64)

define void @walk(ptr %out) #0 {
  %r0 = call ptr @llvm.returnaddress(i32 0)
  %r1 = call ptr @llvm.returnaddress(i32 1)
  %r2 = call ptr @llvm.returnaddress(i32 2)
  %r3 = call ptr @llvm.returnaddress(i32 3)
  %f1 = call ptr @llvm.frameaddress.p0(i32 1)
  %f2 = call ptr @llvm.frameaddress.p0(i32 2)
  %f3 = call ptr @llvm.frameaddress.p0(i32 3)
  store ptr %r0, ptr %out
  %o1 = getelementptr ptr, ptr %out, i64 1
  store ptr %r1, ptr %o1
  %o2 = getelementptr ptr, ptr %out, i64 2
  store ptr %r2, ptr %o2
  %o3 = getelementptr ptr, ptr %out, i64 3
  store ptr %r3, ptr %o3
  %o4 = getelementptr ptr, ptr %out, i64 4
  store ptr %f1, ptr %o4
  %o5 = getelementptr ptr, ptr %out, i64 5
  store ptr %f2, ptr %o5
  %o6 = getelementptr ptr, ptr %out, i64 6
  store ptr %f3, ptr %o6
  ret void
}

define void @level3(ptr %out) #0 {
  %f = call ptr @llvm.frameaddress.p0(i32 0)
  %o = getelementptr ptr, ptr %out, i64 7
  store ptr %f, ptr %o
  call void @walk(ptr %out)
  ret void
}

define void @level2(ptr %out) #0 {
  %pad = alloca [40 x i8], align 16
  call void @sink(ptr %pad)
  %f = call ptr @llvm.frameaddress.p0(i32 0)
  %o = getelementptr ptr, ptr %out, i64 8
  store ptr %f, ptr %o
  call void @level3(ptr %out)
  ret void
}

define void @level1(ptr %out) #0 {
  %f = call ptr @llvm.frameaddress.p0(i32 0)
  %o = getelementptr ptr, ptr %out, i64 9
  store ptr %f, ptr %o
  call void @level2(ptr %out)
  ret void
}

define ptr @cfa_plain() #0 {
  %c = call ptr @llvm.eh.dwarf.cfa(i32 0)
  ret ptr %c
}

define ptr @cfa_aligned() #0 {
  %a = alloca [64 x i8], align 64
  call void @sink(ptr %a)
  %c = call ptr @llvm.eh.dwarf.cfa(i32 0)
  ret ptr %c
}

define ptr @cfa_vla(i64 %n) #0 {
  %a = alloca i8, i64 %n, align 16
  call void @sink(ptr %a)
  %c = call ptr @llvm.eh.dwarf.cfa(i32 0)
  ret ptr %c
}

define ptr @cfa_va(i32 %n, ...) #0 {
  %ap = alloca [24 x i8], align 8
  call void @llvm.va_start.p0(ptr %ap)
  %c = call ptr @llvm.eh.dwarf.cfa(i32 0)
  call void @llvm.va_end.p0(ptr %ap)
  ret ptr %c
}

define ptr @cfa_at(i32 %k) #0 {
  %c = call ptr @llvm.eh.dwarf.cfa(i32 %k)
  ret ptr %c
}

define ptr @cfa_minus8() #0 {
  %c = call ptr @llvm.eh.dwarf.cfa(i32 -8)
  ret ptr %c
}

define void @distance(ptr %out, i64 %i, ptr %sp, ptr %cfa) #0 {
  %s = ptrtoint ptr %sp to i64
  %c = ptrtoint ptr %cfa to i64
  %d = sub i64 %c, %s
  %o = getelementptr i64, ptr %out, i64 %i
  store i64 %d, ptr %o
  ret void
}

; Each CFA less the stack pointer before the call, from a realigned frame.
define void @cfa_distances(ptr %out) #0 {
  %big = alloca [64 x i8], align 64
  call void @sink(ptr %big)
  %s0 = call ptr @llvm.stacksave.p0()
  %c0 = call ptr @cfa_plain()
  call void @distance(ptr %out, i64 0, ptr %s0, ptr %c0)
  %s1 = call ptr @llvm.stacksave.p0()
  %c1 = call ptr @cfa_aligned()
  call void @distance(ptr %out, i64 1, ptr %s1, ptr %c1)
  %s2 = call ptr @llvm.stacksave.p0()
  %c2 = call ptr @cfa_vla(i64 100)
  call void @distance(ptr %out, i64 2, ptr %s2, ptr %c2)
  %s3 = call ptr @llvm.stacksave.p0()
  %c3 = call ptr (i32, ...) @cfa_va(i32 2, i64 7, double 1.5)
  call void @distance(ptr %out, i64 3, ptr %s3, ptr %c3)
  %s4 = call ptr @llvm.stacksave.p0()
  %c4 = call ptr @cfa_at(i32 24)
  call void @distance(ptr %out, i64 4, ptr %s4, ptr %c4)
  %s5 = call ptr @llvm.stacksave.p0()
  %c5 = call ptr @cfa_at(i32 -16)
  call void @distance(ptr %out, i64 5, ptr %s5, ptr %c5)
  %s6 = call ptr @llvm.stacksave.p0()
  %c6 = call ptr @cfa_minus8()
  call void @distance(ptr %out, i64 6, ptr %s6, ptr %c6)
  ret void
}

define i32 @aora_check() #0 {
  %a = call ptr @llvm.addressofreturnaddress.p0()
  %r = call ptr @llvm.returnaddress(i32 0)
  %v = load ptr, ptr %a
  %ok1 = icmp eq ptr %v, %r
  %c = call ptr @llvm.eh.dwarf.cfa(i32 0)
  %e = getelementptr i8, ptr %c, i64 -8
  %ok2 = icmp eq ptr %a, %e
  %f = call ptr @llvm.frameaddress.p0(i32 0)
  %g = getelementptr i8, ptr %f, i64 8
  %ok3 = icmp eq ptr %a, %g
  %ok12 = and i1 %ok1, %ok2
  %ok = and i1 %ok12, %ok3
  %z = zext i1 %ok to i32
  ret i32 %z
}

define void @jumper(i64 %off, ptr %h) #0 {
  call void @llvm.eh.unwind.init()
  call void @llvm.eh.return.i64(i64 %off, ptr %h)
  unreachable
}

define i32 @land_here() #0 {
entry:
  store ptr blockaddress(@land_here, %lab), ptr @target_pc
  %v = load volatile i32, ptr @never
  %c = icmp ne i32 %v, 0
  br i1 %c, label %ib, label %go
go:
  %pc = load ptr, ptr @target_pc
  call void @jumper(i64 0, ptr %pc)
  ret i32 1
ib:
  %pc2 = load ptr, ptr @target_pc
  indirectbr ptr %pc2, [label %lab]
lab:
  ret i32 42
}

; One step of an unwinder: the target frame's rbp installed where this
; frame saved rbp (CFA - 16), then eh.return to the target's stack.
define void @unwinder() #0 {
  call void @llvm.eh.unwind.init()
  %cfa = call ptr @llvm.eh.dwarf.cfa(i32 0)
  %sp = load ptr, ptr @target_sp
  %fp = load ptr, ptr @target_fp
  %pc = load ptr, ptr @target_pc
  %slot = getelementptr i8, ptr %cfa, i64 -16
  store ptr %fp, ptr %slot
  %spi = ptrtoint ptr %sp to i64
  %cfai = ptrtoint ptr %cfa to i64
  %off = sub i64 %spi, %cfai
  call void @llvm.eh.return.i64(i64 %off, ptr %pc)
  unreachable
}

define void @middle() #0 {
  %pad = alloca [200 x i8], align 16
  call void @sink(ptr %pad)
  call void @unwinder()
  call void @sink(ptr %pad)
  ret void
}

define i32 @catcher() #0 {
entry:
  call void @llvm.eh.unwind.init()
  %fp = call ptr @llvm.frameaddress.p0(i32 0)
  store ptr %fp, ptr @target_fp
  %sp = call ptr @llvm.stacksave.p0()
  store ptr %sp, ptr @target_sp
  store ptr blockaddress(@catcher, %lab), ptr @target_pc
  %v = load volatile i32, ptr @never
  %c = icmp ne i32 %v, 0
  br i1 %c, label %ib, label %go
go:
  call void @middle()
  ret i32 1
ib:
  %pc = load ptr, ptr @target_pc
  indirectbr ptr %pc, [label %lab]
lab:
  ret i32 7
}

define i64 @scan_saved() #0 {
  call void @llvm.eh.unwind.init()
  %lo = call ptr @llvm.stacksave.p0()
  %hi = call ptr @llvm.eh.dwarf.cfa(i32 0)
  %n = call i64 @count_magic(ptr %lo, ptr %hi)
  ret i64 %n
}

; valgrind.h's CALL_FN_W_W on amd64, reduced: rbp points at the CFA for
; the template's rows, rsp is realigned, the callee is called.
define i64 @through(i64 %x) #0 {
  %cfa = call ptr @llvm.eh.dwarf.cfa(i32 0)
  %r = call i64 asm sideeffect "movq %rbp, %r15\0Amovq $2, %rbp\0A.cfi_remember_state\0A.cfi_def_cfa rbp, 0\0Amovq %rsp, %r14\0Aandq $$-16, %rsp\0Acall leaf\0Amovq %r14, %rsp\0Amovq %r15, %rbp\0A.cfi_restore_state", "={ax},{di},r,~{rcx},~{rdx},~{rsi},~{rdi},~{r8},~{r9},~{r10},~{r11},~{r14},~{r15},~{xmm0},~{xmm1},~{xmm2},~{xmm3},~{xmm4},~{xmm5},~{xmm6},~{xmm7},~{xmm8},~{xmm9},~{xmm10},~{xmm11},~{xmm12},~{xmm13},~{xmm14},~{xmm15},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %x, ptr %cfa)
  %s = add i64 %r, 1
  ret i64 %s
}

attributes #0 = { noinline "frame-pointer"="all" }
"#;

/// A function with `llvm.eh.return.i64` that returns a value on its
/// other path: built by hod only (`llc` 19 and 22 pop `rax` over 42).
const RETURNS: &str = r#"
declare void @llvm.eh.return.i64(i64, ptr)
define i64 @maybe_return(i1 %c, i64 %off, ptr %h) #0 {
  br i1 %c, label %a, label %b
a:
  call void @llvm.eh.return.i64(i64 %off, ptr %h)
  unreachable
b:
  ret i64 42
}
attributes #0 = { noinline "frame-pointer"="all" }
"#;

const DRIVER: &str = r#"
#include <stdio.h>
#include <string.h>
extern "C" {
void walk(void **);
void level1(void **);
void level2(void **);
void level3(void **);
void cfa_distances(long *);
int aora_check(void);
int land_here(void);
int catcher(void);
long scan_saved(void);
long through(long);
long maybe_return(bool, long, void *);
__attribute__((noinline)) void sink(void *p) { asm volatile("" :: "r"(p) : "memory"); }
long count_magic(void **lo, void **hi) {
  long n = 0;
  for (void **p = lo; p < hi; p++)
    if (((unsigned long)*p >> 8) == 0xfeedfacecafe00UL) n++;
  return n;
}
long leaf(long x) {
  if (x == 42) throw 7;
  return x + 1;
}
__attribute__((noinline)) void run_walk(void **out) { level1(out); asm volatile(""); }
}
static const char *name_of(void *a) {
  struct { const char *name; void *at; } known[] = {
    {"walk", (void *)walk}, {"level3", (void *)level3}, {"level2", (void *)level2},
    {"level1", (void *)level1}, {"run_walk", (void *)run_walk},
  };
  const char *best = "?";
  unsigned long at = 0;
  for (auto &k : known)
    if ((unsigned long)k.at <= (unsigned long)a && (unsigned long)k.at >= at) {
      at = (unsigned long)k.at;
      best = k.name;
    }
  return best;
}
static long run_scan(void) {
  register long b asm("rbx") = 0xfeedfacecafe0001L;
  register long c asm("r12") = 0xfeedfacecafe0002L;
  register long d asm("r13") = 0xfeedfacecafe0003L;
  register long e asm("r14") = 0xfeedfacecafe0004L;
  register long f asm("r15") = 0xfeedfacecafe0005L;
  asm volatile("" : "+r"(b), "+r"(c), "+r"(d), "+r"(e), "+r"(f));
  long n = scan_saved();
  asm volatile("" :: "r"(b), "r"(c), "r"(d), "r"(e), "r"(f));
  return n;
}
int main(int argc, char **argv) {
  void *out[10];
  memset(out, 0, sizeof out);
  run_walk(out);
  printf("walk: %s %s %s %s chain %d\n", name_of(out[0]), name_of(out[1]), name_of(out[2]),
         name_of(out[3]), out[4] == out[7] && out[5] == out[8] && out[6] == out[9]);
  long d[7];
  cfa_distances(d);
  printf("cfa: %ld %ld %ld %ld %ld %ld %ld\n", d[0], d[1], d[2], d[3], d[4], d[5], d[6]);
  printf("aora: %d\n", aora_check());
  printf("eh.return to the caller: %d\n", land_here());
  printf("eh.return past a frame: %d\n", catcher());
  printf("saved by unwind.init: %ld\n", scan_saved() == 0 ? run_scan() : -1);
  long t = through(1);
  int caught = 0;
  try {
    through(42);
  } catch (int v) {
    caught = v;
  }
  printf("through: %ld caught %d\n", t, caught);
  if (argc > 1) printf("returned: %ld\n", maybe_return(false, 0, 0));
  return 0;
}
"#;

const EXPECTED: &str = "walk: level3 level2 level1 run_walk chain 1
cfa: 0 0 0 0 24 -16 -8
aora: 1
eh.return to the caller: 42
eh.return past a frame: 7
saved by unwind.init: 5
through: 3 caught 7
";

fn hod_object(dir: &Scratch, name: &str, text: &str, stress: Stress) -> PathBuf {
    let (m, bodies) = parse(text);
    let object = hod_x64::compile_to_object(&m, &bodies, &pic(stress))
        .unwrap_or_else(|e| panic!("hod: {e}"));
    dir.write(&format!("{name}.o"), object)
}

/// The objects of [`FUNCTIONS`]: hod plain and stressed, `llc` of each
/// LLVM present.
fn objects(dir: &Scratch) -> Vec<(String, PathBuf)> {
    let text = format!("{}{FUNCTIONS}", ll_header());
    let mut objects = vec![
        ("hod".to_owned(), hod_object(dir, "hod", &text, Stress::default())),
        ("stress".to_owned(), hod_object(dir, "stress", &text, Stress::ALL)),
    ];
    for major in LLVM_MAJORS {
        if let Some(object) = llc(dir, major, &format!("llc-{major}"), &text) {
            objects.push((format!("llc-{major}"), object));
        }
    }
    objects
}

/// §5.6.38: the program prints the same built by hod and by `llc`,
/// under every linker, dynamic and static.
#[test]
fn frames_walked_and_unwound_as_llc() {
    let dir = Scratch::new("x64-frame-intrinsics");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) =
        toolchain.compile_cxx(&dir, "driver", DRIVER, &["-fPIC", "-fno-omit-frame-pointer"])
    else {
        return;
    };
    let returns = hod_object(
        &dir,
        "returns",
        &format!("{}{RETURNS}", ll_header()),
        Stress::default(),
    );
    let stdcxx = Path::new("-lstdc++");
    let mut runs = 0;
    for (who, object) in objects(&dir) {
        for linker in &toolchain.linkers {
            for (how, flags) in [("dynamic", &[][..]), ("static", &["-static"][..])] {
                let program = dir.join(format!("{who}-{how}-{}", linker.name));
                toolchain.link_ok(linker, flags, &[&driver, &object, &returns, stdcxx], &program);
                let mut command = toolchain.runner_command(&program);
                if who.starts_with("llc") {
                    // `llc`'s `maybe_return` is not asked: see `RETURNS`.
                } else {
                    command.arg("returned");
                }
                let ran = run_with(&mut command, &[], Duration::from_mins(5));
                let mut want = EXPECTED.to_owned();
                if !who.starts_with("llc") {
                    want.push_str("returned: 42\n");
                }
                assert_eq!(
                    (exit_code(&ran), ran.stdout_text()),
                    (Some(0), want),
                    "{who} {how} {}: {}",
                    linker.name,
                    ran.stderr_text()
                );
                runs += 1;
            }
        }
    }
    eprintln!("x86-64 frame-intrinsic programs that printed the expected text: {runs}");
}

/// The registers `llvm-dwarfdump --eh-frame` shows saved (`REG=[CFA...`)
/// anywhere in the one FDE of `object`.
fn saved_registers(object: &Path) -> Option<BTreeSet<String>> {
    let dwarfdump = tool("llvm-dwarfdump")?;
    let out = run(&dwarfdump, ["--eh-frame".as_ref(), object.as_os_str()]);
    assert!(out.success(), "llvm-dwarfdump: {}", out.stderr_text());
    let text = out.stdout_text();
    let mut saved = BTreeSet::new();
    for row in text.lines().filter(|l| l.contains("CFA=")) {
        for rule in row.split(", ").chain(row.split(": ")) {
            if let Some((reg, rest)) = rule.trim().split_once('=') {
                if rest.starts_with("[CFA") {
                    saved.insert(reg.trim().to_owned());
                }
            }
        }
    }
    Some(saved)
}

/// The unwinder step alone: the rows say where every register an
/// unwinder installs is, the same set as `llc`'s.
const UNWINDER_ALONE: &str = r#"
declare ptr @llvm.eh.dwarf.cfa(i32)
declare void @llvm.eh.unwind.init()
declare void @llvm.eh.return.i64(i64, ptr)
declare ptr @next(ptr)
define void @unwinder(i64 %off) #0 {
  call void @llvm.eh.unwind.init()
  %cfa = call ptr @llvm.eh.dwarf.cfa(i32 0)
  %h = call ptr @next(ptr %cfa)
  call void @llvm.eh.return.i64(i64 %off, ptr %h)
  unreachable
}
attributes #0 = { noinline "frame-pointer"="all" }
"#;

/// §5.6.38: `rax` and `rdx` are saved besides the five callee-saved
/// registers and `rbp`, as `llc` saves them.
#[test]
fn the_unwinder_finds_the_registers_llc_saves() {
    let dir = Scratch::new("x64-frame-intrinsics-rows");
    let text = format!("{}{UNWINDER_ALONE}", ll_header());
    let mut compared = 0;
    for (who, stress) in [("hod", Stress::default()), ("stress", Stress::ALL)] {
        let ours = hod_object(&dir, who, &text, stress);
        let Some(ours) = saved_registers(&ours) else { return };
        let want: BTreeSet<String> =
            ["RAX", "RDX", "RBX", "RBP", "R12", "R13", "R14", "R15", "RIP"]
                .map(String::from)
                .into();
        assert_eq!(ours, want, "{who}");
        for major in LLVM_MAJORS {
            let Some(theirs) = llc(&dir, major, &format!("llc-{major}"), &text) else { continue };
            assert_eq!(saved_registers(&theirs), Some(ours.clone()), "{who} against llc {major}");
            compared += 1;
        }
    }
    eprintln!("register sets compared with llc: {compared}");
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
             -ex 'file {}' -ex 'target remote localhost:{port}' -ex 'break leaf' -ex continue \
             -ex bt -ex kill 2>&1\nkill $q 2>/dev/null\nwait $q 2>/dev/null\nexit 0\n",
            gdb.display(),
            program.display()
        )
    } else {
        let gdb = tool("gdb")?;
        format!(
            "{} -nx -batch {commands} -ex 'break leaf' -ex run -ex bt -ex kill {} 2>&1\n",
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

/// §5.6.38: gdb stopped in `leaf`, which valgrind's template calls with
/// `rbp` at the CFA `llvm.eh.dwarf.cfa` gave, walks back to `main`.
#[test]
fn gdb_walks_through_the_template() {
    let dir = Scratch::new("x64-frame-intrinsics-gdb");
    let Some(toolchain) = Toolchain::x86_64(&dir) else { return };
    let Some(driver) =
        toolchain.compile_cxx(&dir, "driver", DRIVER, &["-fPIC", "-fno-omit-frame-pointer"])
    else {
        return;
    };
    let returns =
        hod_object(&dir, "returns", &format!("{}{RETURNS}", ll_header()), Stress::default());
    let text = format!("{}{FUNCTIONS}", ll_header());
    let stdcxx = Path::new("-lstdc++");
    let linker = &toolchain.linkers[0];
    let first_port = 20000 + (std::process::id() % 20000) as u16;
    let mut walked = 0;
    for (n, (who, stress)) in [("hod", Stress::default()), ("stress", Stress::ALL)].iter().enumerate()
    {
        let object = hod_object(&dir, who, &text, *stress);
        let program = dir.join(format!("gdb-{who}"));
        toolchain.link_ok(linker, &["-static"], &[&driver, &object, &returns, stdcxx], &program);
        let (mut got, mut out) = (Vec::new(), String::new());
        for attempt in 0..3u16 {
            let port = first_port + 50 * attempt + n as u16;
            let Some(text) = backtrace(&toolchain, &dir, &program, port) else { return };
            out = text;
            got = frames(&out);
            if !got.is_empty() {
                break;
            }
        }
        let want = ["leaf", "through", "main"];
        assert_eq!(got.get(..3), Some(&want.map(String::from)[..]), "{who}: {out}");
        walked += 1;
    }
    eprintln!("backtraces through the template: {walked}");
}

/// §5.6.38 and `spec/ir.md` §8: `llvm.sponentry` is refused on x86-64
/// by its row (`llc` 19 and 22 abort on it), `llvm.eh.return.i32` by
/// its name.
#[test]
fn what_llc_cannot_build_is_refused_by_name() {
    let classify = |functions: &str| {
        let (module, bodies) = parse(&format!("{}{functions}", ll_header()));
        let module_pass = hod_ir::classify(&module).err();
        let body_pass = bodies.iter().find_map(|b| hod_ir::classify_body(&module, b).err());
        module_pass.or(body_pass).map(|r| r.construct.into_owned())
    };
    let cases = [
        (
            "declare ptr @llvm.sponentry.p0()\ndefine ptr @f() {\n  %r = call ptr @llvm.sponentry.p0()\n  ret ptr %r\n}\n",
            "intrinsic:llvm.sponentry",
        ),
        (
            "declare void @llvm.eh.return.i32(i32, ptr)\ndefine void @f(i32 %o, ptr %h) {\n  call void @llvm.eh.return.i32(i32 %o, ptr %h)\n  unreachable\n}\n",
            "intrinsic:llvm.eh.return.i32",
        ),
    ];
    for (functions, want) in cases {
        assert_eq!(classify(functions).as_deref(), Some(want), "{functions}");
    }
    for ok in [FUNCTIONS, RETURNS, UNWINDER_ALONE] {
        assert_eq!(classify(ok), None);
    }
}

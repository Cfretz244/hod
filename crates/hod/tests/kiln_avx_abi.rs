//! WP-214: by-value 256-bit vectors across compilers on x86-64 under AVX
//! (`spec/abi-x64.md` §1.3, §3.2, §8.5, §8.7). kiln (from `21b9110a4`)
//! takes the AVX ABI level from the translation unit's features, as clang
//! does, so a 256-bit vector, and a class or union holding exactly one,
//! is passed and returned in `ymm`. A generated grid of signatures is
//! built as two translation units, the callees and their caller, each by
//! four builders: kiln++ with hod, kiln++ with `llc` 22, clang 19 and
//! clang 22 (their own back ends). Every pairing of a callee builder with
//! a caller builder is linked with every linker and run under
//! `qemu-x86_64` (a CPU with AVX2), for `-mavx`, `-mavx2` and
//! `-march=haswell`, and must print what the all-clang-22 program prints:
//! for each callee, the hash of the arguments it received and the hash of
//! the value its caller received back.
//!
//! The grid: 256-bit vectors of every element kind, 128-bit vectors,
//! integers and floating point, classes holding one vector (directly,
//! nested, as a one-element array), unions of a vector and an array, and
//! classes of two members (in memory), as arguments (up to 14, past the
//! eight vector registers: on the stack) and returns; variadic callees
//! reading each kind with `va_arg` (from the overflow area); calls
//! through a pointer. A class whose base is narrower than itself is left
//! out: clang 21 and later pass it in memory, clang 19 in `ymm`, so
//! clang 19 and clang 22 do not agree on it.
//!
//! The kiln builders need kiln++ (`HOD_KILN`, or the workspace's
//! `target-kiln`); without it they are skipped with a note, as the other
//! kiln-dependent tests do.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

#[path = "../../hod_obj/tests/common/linking.rs"]
#[allow(dead_code, reason = "a shared helper: not every function is used here")]
mod linking;

use hod::{OutputKind, RelocModel};
use hod_testutil::{Scratch, run_with, tools_required};
use linking::Toolchain;

/// The number of generated callees.
const CALLEES: usize = 48;

/// The machines, as both front ends spell them.
const MACHINES: [&str; 3] = ["-mavx", "-mavx2", "-march=haswell"];

const TRIPLE: &str = "--target=x86_64-unknown-linux-gnu";

/// A type of the grid: its name, its definition (empty for a built-in
/// type or a vector defined in the prelude), how its value is made from
/// a seed `s` into `r`, how it is hashed from `x` (a pointer), and
/// whether `va_arg` may read it (not `float`, which is promoted).
struct Shape {
    name: &'static str,
    def: &'static str,
    make: &'static str,
    hash: &'static str,
    variadic: bool,
}

const fn shape(
    name: &'static str,
    def: &'static str,
    make: &'static str,
    hash: &'static str,
    variadic: bool,
) -> Shape {
    Shape { name, def, make, hash, variadic }
}

/// Lane `i` of a vector made from seed `s`: small, so exact in every
/// element type.
const LANES: &str = "for (int i = 0; i < (int)(sizeof r / sizeof r[0]); i++) \
                     r[i] = (__typeof__(r[0]))((s + i * 7) % 113);";

const SHAPES: &[Shape] = &[
    shape("v4d", "", LANES, "h = H(h, x, 32);", true),
    shape("v8f", "", LANES, "h = H(h, x, 32);", true),
    shape("v4q", "", LANES, "h = H(h, x, 32);", true),
    shape("v8i", "", LANES, "h = H(h, x, 32);", true),
    shape("v16s", "", LANES, "h = H(h, x, 32);", true),
    shape("v32c", "", LANES, "h = H(h, x, 32);", true),
    shape("v2d", "", LANES, "h = H(h, x, 16);", true),
    shape("v4i", "", LANES, "h = H(h, x, 16);", true),
    shape("S4d", "struct S4d { v4d v; };", "r.v = mk_v4d(s);", "h = H(h, &x->v, 32);", true),
    shape("S8f", "struct S8f { v8f v; };", "r.v = mk_v8f(s);", "h = H(h, &x->v, 32);", true),
    shape(
        "U4q",
        "union U4q { v4q v; long long a[4]; };",
        "r.v = mk_v4q(s);",
        "h = H(h, &x->v, 32);",
        true,
    ),
    shape(
        "U8f",
        "union U8f { v8f v; double d[4]; };",
        "r.v = mk_v8f(s);",
        "h = H(h, &x->v, 32);",
        true,
    ),
    shape(
        "N8i",
        "struct I8i { v8i v; }; struct N8i { I8i in; };",
        "r.in.v = mk_v8i(s);",
        "h = H(h, &x->in.v, 32);",
        true,
    ),
    shape(
        "A4d",
        "struct A4d { v4d v[1]; };",
        "r.v[0] = mk_v4d(s);",
        "h = H(h, &x->v[0], 32);",
        true,
    ),
    shape(
        "M2",
        "struct M2 { v4d a; v4q b; };",
        "r.a = mk_v4d(s); r.b = mk_v4q(s + 1);",
        "h = H(h, &x->a, 32); h = H(h, &x->b, 32);",
        true,
    ),
    shape(
        "Mi",
        "struct Mi { v8f v; int i; };",
        "r.v = mk_v8f(s); r.i = (int)s;",
        "h = H(h, &x->v, 32); h = H(h, &x->i, 4);",
        true,
    ),
    shape("S2d", "struct S2d { v2d v; };", "r.v = mk_v2d(s);", "h = H(h, &x->v, 16);", true),
    shape("int", "", "r = (int)s - 50;", "h = H(h, x, 4);", true),
    shape("i64", "typedef long long i64;", "r = s * 1000003LL;", "h = H(h, x, 8);", true),
    shape("f64", "typedef double f64;", "r = s + 0.5;", "h = H(h, x, 8);", true),
    shape("f32", "typedef float f32;", "r = (float)s + 0.25f;", "h = H(h, x, 4);", false),
];

/// A small deterministic generator (xorshift64*).
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let x = self.0.wrapping_mul(0x2545_f491_4f6c_dd1d);
        usize::try_from(x >> 33).unwrap() % n
    }
}

/// One callee: its return shape, its named parameters, and (variadic)
/// the shapes its caller passes through `...` after the count.
struct Callee {
    ret: usize,
    params: Vec<usize>,
    varargs: Option<Vec<usize>>,
    indirect: bool,
}

fn grid() -> Vec<Callee> {
    let mut rng = Rng(0x5eed_0214);
    let variadic: Vec<usize> = (0..SHAPES.len()).filter(|&i| SHAPES[i].variadic).collect();
    (0..CALLEES)
        .map(|k| {
            let ret = rng.below(SHAPES.len());
            let is_variadic = k % 4 == 3;
            let count = if is_variadic {
                rng.below(3)
            } else if k % 5 == 1 {
                // Past the eight vector registers.
                9 + rng.below(6)
            } else {
                rng.below(9)
            };
            let params = (0..count).map(|_| rng.below(SHAPES.len())).collect();
            let varargs = is_variadic.then(|| {
                (0..1 + rng.below(6)).map(|_| variadic[rng.below(variadic.len())]).collect()
            });
            Callee { ret, params, varargs, indirect: k % 3 == 2 }
        })
        .collect()
}

/// What both translation units share: the types, their makers and
/// hashers, and the callees' declarations.
fn prelude(callees: &[Callee]) -> String {
    let mut s = String::from(
        "typedef double v4d __attribute__((vector_size(32)));\n\
         typedef float v8f __attribute__((vector_size(32)));\n\
         typedef long long v4q __attribute__((vector_size(32)));\n\
         typedef int v8i __attribute__((vector_size(32)));\n\
         typedef short v16s __attribute__((vector_size(32)));\n\
         typedef signed char v32c __attribute__((vector_size(32)));\n\
         typedef double v2d __attribute__((vector_size(16)));\n\
         typedef int v4i __attribute__((vector_size(16)));\n\
         extern \"C\" unsigned long long seen[], got[];\n\
         static inline unsigned long long H(unsigned long long h, const void *p, unsigned n) {\n\
         \x20 const unsigned char *b = (const unsigned char *)p;\n\
         \x20 for (unsigned i = 0; i < n; i++) h = (h ^ b[i]) * 1099511628211ULL;\n\
         \x20 return h;\n\
         }\n",
    );
    for shape in SHAPES {
        let n = shape.name;
        writeln!(
            s,
            "{def}\nstatic inline {n} mk_{n}(long long s) {{ {n} r; {make} return r; }}\n\
             static inline unsigned long long h_{n}(unsigned long long h, const {n} *x) {{ {hash} return h; }}",
            def = shape.def,
            make = shape.make,
            hash = shape.hash,
        )
        .unwrap();
    }
    for (k, callee) in callees.iter().enumerate() {
        writeln!(
            s,
            "extern \"C\" {} callee_{k}({});",
            SHAPES[callee.ret].name,
            params_decl(callee)
        )
        .unwrap();
    }
    s
}

fn params_decl(callee: &Callee) -> String {
    let mut list: Vec<String> = callee
        .params
        .iter()
        .enumerate()
        .map(|(i, &p)| format!("{} a{i}", SHAPES[p].name))
        .collect();
    if callee.varargs.is_some() {
        list.push("int n".into());
        list.push("...".into());
    }
    list.join(", ")
}

fn callee_unit(callees: &[Callee]) -> String {
    let mut s = prelude(callees);
    for (k, callee) in callees.iter().enumerate() {
        let ret = SHAPES[callee.ret].name;
        writeln!(s, "extern \"C\" {ret} callee_{k}({}) {{", params_decl(callee)).unwrap();
        writeln!(s, "  unsigned long long h = 14695981039346656037ULL ^ {k}ULL;").unwrap();
        for (i, &p) in callee.params.iter().enumerate() {
            writeln!(s, "  h = h_{}(h, &a{i});", SHAPES[p].name).unwrap();
        }
        if let Some(varargs) = &callee.varargs {
            s.push_str(
                "  h = H(h, &n, 4);\n  __builtin_va_list ap;\n  __builtin_va_start(ap, n);\n",
            );
            for &v in varargs {
                let n = SHAPES[v].name;
                writeln!(s, "  {{ {n} t = __builtin_va_arg(ap, {n}); h = h_{n}(h, &t); }}")
                    .unwrap();
            }
            s.push_str("  __builtin_va_end(ap);\n");
        }
        writeln!(s, "  seen[{k}] = h;\n  return mk_{ret}((long long)(h % 1000));\n}}").unwrap();
    }
    s
}

fn caller_unit(callees: &[Callee]) -> String {
    let mut s = prelude(callees);
    s.push_str("extern \"C\" void run(void) {\n");
    for (k, callee) in callees.iter().enumerate() {
        let ret = SHAPES[callee.ret].name;
        s.push_str("  {\n");
        let mut args = Vec::new();
        for (i, &p) in callee.params.iter().enumerate() {
            let n = SHAPES[p].name;
            writeln!(s, "    {n} a{i} = mk_{n}({});", k * 17 + i).unwrap();
            args.push(format!("a{i}"));
        }
        if let Some(varargs) = &callee.varargs {
            args.push(varargs.len().to_string());
            for (j, &v) in varargs.iter().enumerate() {
                let n = SHAPES[v].name;
                writeln!(s, "    {n} v{j} = mk_{n}({});", k * 31 + j + 5).unwrap();
                args.push(format!("v{j}"));
            }
        }
        let target = if callee.indirect {
            writeln!(s, "    {ret} (*volatile p)({}) = callee_{k};", params_decl(callee)).unwrap();
            "p".to_owned()
        } else {
            format!("callee_{k}")
        };
        writeln!(s, "    {ret} r = {target}({});", args.join(", ")).unwrap();
        writeln!(s, "    got[{k}] = h_{ret}(14695981039346656037ULL, &r);\n  }}").unwrap();
    }
    s.push_str("}\n");
    s
}

const MAIN_C: &str = r#"
#include <stdio.h>
unsigned long long seen[64], got[64];
void run(void);
int main(void) {
  run();
  for (int i = 0; i < 64; i++) printf("%d %016llx %016llx\n", i, seen[i], got[i]);
  return 0;
}
"#;

/// kiln++, as the other kiln-dependent tests find it.
fn kiln() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("HOD_KILN").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(path));
    }
    let beside = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .ancestors()
        .nth(2)
        .map(|root| root.join("target-kiln").join("release").join("kiln++"));
    match beside {
        Some(path) if path.is_file() => Some(path),
        _ => {
            assert!(!tools_required(), "no kiln++ found; set HOD_KILN (HOD_REQUIRE_TOOLS=1)");
            eprintln!("note: no kiln++ (set HOD_KILN); skipping its builders");
            None
        }
    }
}

fn installed(major: u32, tool: &str) -> Option<PathBuf> {
    let path = PathBuf::from(format!("/usr/lib/llvm-{major}/bin/{tool}"));
    if path.is_file() {
        Some(path)
    } else {
        assert!(!tools_required(), "{} is required (HOD_REQUIRE_TOOLS=1)", path.display());
        eprintln!("note: {} not found; skipping its builder", path.display());
        None
    }
}

fn run_ok(command: &mut Command, what: &str) {
    let ran = run_with(command, &[], Duration::from_mins(3));
    assert!(ran.success(), "{what}: {}", ran.stderr_text());
}

/// Who builds a translation unit.
enum Builder {
    KilnHod(PathBuf),
    KilnLlc(PathBuf, PathBuf),
    Clang(PathBuf),
}

impl Builder {
    /// `source` (C++) to an object for `machine`.
    fn build(&self, scratch: &Scratch, source: &Path, machine: &str, tag: &str) -> PathBuf {
        let object = scratch.join(format!("{tag}.o"));
        let common = [TRIPLE, "-O0", "-fPIE", "-std=c++20", "-fno-exceptions", machine];
        match self {
            Self::Clang(clang) => {
                let mut command = Command::new(clang);
                command.args(common).arg("-c").arg(source).arg("-o").arg(&object);
                run_ok(&mut command, tag);
            }
            Self::KilnHod(kiln) | Self::KilnLlc(kiln, _) => {
                let ir = scratch.join(format!("{tag}.ll"));
                let mut command = Command::new(kiln);
                command.args(common).args(["-S", "-emit-llvm"]).arg(source).arg("-o").arg(&ir);
                run_ok(&mut command, tag);
                if let Self::KilnLlc(_, llc) = self {
                    let mut command = Command::new(llc);
                    command
                        .args(["-O0", "-relocation-model=pic", "-filetype=obj"])
                        .arg(&ir)
                        .arg("-o")
                        .arg(&object);
                    run_ok(&mut command, tag);
                } else {
                    let text = std::fs::read(&ir).expect("kiln's IR");
                    let options = hod::Options {
                        output_kind: OutputKind::Object,
                        relocation_model: Some(RelocModel::Pic),
                        ..hod::Options::default()
                    };
                    let bytes =
                        hod::compile(&text, &options).unwrap_or_else(|e| panic!("hod, {tag}: {e}"));
                    std::fs::write(&object, bytes).expect("the object");
                }
            }
        }
        object
    }
}

/// The first line two outputs differ on, for the message.
fn first_difference(got: &str, want: &str) -> String {
    got.lines()
        .zip(want.lines())
        .find(|(a, b)| a != b)
        .map_or_else(|| "(lengths differ)".to_owned(), |(a, b)| format!("got `{a}`, want `{b}`"))
}

#[test]
fn kiln_hod_clang_avx_vectors_mix() {
    let scratch = Scratch::new("hod-wp214-avx");
    let Some(toolchain) = Toolchain::x86_64(&scratch) else { return };
    let toolchain = toolchain.with_emulated_cpu("max");
    let (Some(clang22), Some(llc22)) = (installed(22, "clang++"), installed(22, "llc")) else {
        return;
    };
    // The reference builder first: clang 22's program is what every
    // pairing must print.
    let mut builders: Vec<(&str, Builder)> = vec![("clang22", Builder::Clang(clang22))];
    if let Some(clang19) = installed(19, "clang++") {
        builders.push(("clang19", Builder::Clang(clang19)));
    }
    if let Some(kiln) = kiln() {
        builders.push(("kiln-hod", Builder::KilnHod(kiln.clone())));
        builders.push(("kiln-llc", Builder::KilnLlc(kiln, llc22)));
    }
    let callees = grid();
    let callee_src = scratch.write("callee.cpp", callee_unit(&callees));
    let caller_src = scratch.write("caller.cpp", caller_unit(&callees));
    let main = toolchain.compile_c(&scratch, "main", MAIN_C, &["-fPIE"]);
    let mut runs = 0;
    for (index, machine) in MACHINES.iter().enumerate() {
        let objects: Vec<(&str, PathBuf, PathBuf)> = builders
            .iter()
            .map(|(name, builder)| {
                let tag = format!("m{index}-{name}");
                let callee =
                    builder.build(&scratch, &callee_src, machine, &format!("{tag}-callee"));
                let caller =
                    builder.build(&scratch, &caller_src, machine, &format!("{tag}-caller"));
                (*name, callee, caller)
            })
            .collect();
        let mut expected: Option<String> = None;
        let mut failures = Vec::new();
        for (callee_by, callee, _) in &objects {
            for (caller_by, _, caller) in &objects {
                let tag = format!("{machine}: callees by {callee_by}, caller by {caller_by}");
                let mut seen: Option<String> = None;
                for linker in &toolchain.linkers {
                    let exe =
                        scratch.join(format!("m{index}-{callee_by}-{caller_by}-{}", linker.name));
                    let ran = toolchain.link_and_run(
                        linker,
                        &["-pie"],
                        &[callee.as_path(), caller.as_path(), main.as_path()],
                        &exe,
                    );
                    assert!(ran.success(), "{tag} ({}): {}", linker.name, ran.stderr_text());
                    let out = ran.stdout_text();
                    if let Some(seen) = &seen {
                        assert_eq!(&out, seen, "{tag}: {} differs", linker.name);
                    }
                    seen = Some(out);
                    let _ = std::fs::remove_file(&exe);
                    runs += 1;
                }
                let out = seen.unwrap_or_default();
                match &expected {
                    None => expected = Some(out),
                    Some(want) if &out != want => {
                        failures.push(format!("{tag}: {}", first_difference(&out, want)));
                    }
                    Some(_) => {}
                }
            }
        }
        assert!(failures.is_empty(), "programs differ from clang 22's:\n{}", failures.join("\n"));
        // The grid sees the convention: callees built without AVX (256-bit
        // vectors in memory) called by the AVX caller print otherwise.
        let baseline = Builder::Clang(installed(22, "clang++").expect("clang 22"));
        let callee = baseline.build(&scratch, &callee_src, "-msse2", &format!("m{index}-sse2"));
        let caller = &objects[0].2;
        let linker = &toolchain.linkers[0];
        let exe = scratch.join(format!("m{index}-sse2"));
        let ran = toolchain.link_and_run(
            linker,
            &["-pie"],
            &[callee.as_path(), caller.as_path(), main.as_path()],
            &exe,
        );
        assert_ne!(
            Some(ran.stdout_text()),
            expected,
            "{machine}: a callee without AVX agrees with an AVX caller"
        );
    }
    eprintln!("wp214: {} builders, {runs} runs agree with clang 22", builders.len());
}

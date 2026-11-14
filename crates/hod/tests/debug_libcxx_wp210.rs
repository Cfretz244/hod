//! WP-210 (`spec/dwarf.md` §6.27): what libc++'s test suite built with
//! `-g` met in the DIE writer.
//!
//! - a static data member whose value is wider than 64 bits
//!   (`numeric_limits<__int128>::__min`, a `static constexpr long double`)
//!   is written as `llc` writes it, `DW_AT_const_value` a `block1` of the
//!   value's bytes (families 114 and 115); it was `debug:die-unsupported`.
//! - an inlined subprogram's retained variables and labels (kiln `-O2
//!   -g`) are abstract entities of its abstract instance, and a retained
//!   variable of the function's own out-of-line code names its abstract
//!   one (§6.28); they were `debug:retained-nodes`.
//!
//! Each module's DIE tree is compared with `llc` 22's (`-O0
//! -global-isel=0 -fast-isel=0`) line by line, offsets and indices masked,
//! when `llc` is installed, for DWARF 5 and 4 and both targets; hod's
//! object verifies.

use std::path::Path;
use std::time::Duration;

use hod::{CompileError, OutputKind, Report, Settings, compile_with};
use hod_testutil::{Scratch, run_with, tool};

const TRIPLES: [&str; 2] = ["aarch64-unknown-linux-gnu", "x86_64-unknown-linux-gnu"];

/// `struct L { static constexpr __int128 min = ...; static constexpr
/// unsigned __int128 max = ...; static constexpr long double one = 1.5L;
/// alignas(16) static constexpr __int128 big = ...; } x;`: MEMBERTAG is the
/// static members' tag (`DW_TAG_variable` in DWARF 5, `DW_TAG_member`
/// before), LDBL the target's `long double` constant.
const WIDE: &str = r#"target triple = "TRIPLE"

%struct.L = type { i8 }

@x = dso_local global %struct.L zeroinitializer, align 1, !dbg !5

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "kiln", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !4, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "wide.cpp", directory: "/hod/tests")
!2 = !{i32 7, !"Dwarf Version", i32 VERSION}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{!5}
!5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
!6 = distinct !DIGlobalVariable(name: "x", scope: !1, file: !1, line: 9, type: !7, isLocal: false, isDefinition: true)
!7 = distinct !DICompositeType(tag: DW_TAG_structure_type, name: "L", file: !1, line: 1, size: 8, flags: DIFlagTypePassByValue, elements: !8, identifier: "_ZTS1L")
!8 = !{!9, !11, !13, !15}
!9 = !DIDerivedType(tag: MEMBERTAG, name: "min", scope: !7, file: !1, line: 2, baseType: !10, flags: DIFlagStaticMember, extraData: i128 -170141183460469231731687303715884105728)
!10 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !20)
!11 = !DIDerivedType(tag: MEMBERTAG, name: "max", scope: !7, file: !1, line: 3, baseType: !12, flags: DIFlagProtected | DIFlagStaticMember, extraData: i128 -2)
!12 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !21)
!13 = !DIDerivedType(tag: MEMBERTAG, name: "one", scope: !7, file: !1, line: 4, baseType: !14, flags: DIFlagStaticMember, extraData: LDBL)
!14 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !22)
!15 = !DIDerivedType(tag: MEMBERTAG, name: "big", scope: !7, file: !1, line: 5, baseType: !10, flags: DIFlagStaticMember, extraData: i128 1311768467463790320, align: 128)
!20 = !DIBasicType(name: "__int128", size: 128, encoding: DW_ATE_signed)
!21 = !DIBasicType(name: "unsigned __int128", size: 128, encoding: DW_ATE_unsigned)
!22 = !DIBasicType(name: "long double", size: 128, encoding: DW_ATE_float)
"#;

/// The static members' tag, as kiln and clang write it for `version`.
fn wide(triple: &str, version: u32, tag: &str) -> String {
    // 1.5L: `fp128` on AArch64, `x86_fp80` on x86-64.
    let ldbl = if triple.starts_with("aarch64") {
        "fp128 0xL00000000000000003FFF800000000000"
    } else {
        "x86_fp80 0xK3FFFC000000000000000"
    };
    WIDE.replace("TRIPLE", triple)
        .replace("VERSION", &version.to_string())
        .replace("MEMBERTAG", tag)
        .replace("LDBL", ldbl)
}

fn compile(text: &str) -> Result<Vec<u8>, CompileError> {
    let options = hod::Options { output_kind: OutputKind::Object, ..hod::Options::default() };
    compile_with(text.as_bytes(), &options, &Settings::default(), &mut Report::default())
}

fn run_tool(tool: &Path, args: &[&std::ffi::OsStr]) -> String {
    let mut command = std::process::Command::new(tool);
    command.args(args);
    let out = run_with(&mut command, &[], Duration::from_mins(2));
    assert!(out.success(), "{command:?}: {}", out.stderr_text());
    out.stdout_text()
}

/// The DIE tree of `object`, one attribute per line, offsets, addresses,
/// sizes and indices masked; without the code ranges.
fn tree(dwarfdump: &Path, object: &Path) -> Vec<String> {
    let text = run_tool(dwarfdump, &["--debug-info".as_ref(), object.as_os_str()]);
    let mut lines = Vec::new();
    for line in text.lines().skip_while(|l| !l.contains("DW_TAG_compile_unit")) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("DW_AT_high_pc") || line.starts_with("DW_AT_low_pc")
        {
            continue;
        }
        let line = line
            .split_once(": ")
            .map_or(line, |(at, rest)| if at.starts_with("0x") { rest } else { line });
        // A slot's offset is hod's frame layout, not the tree's.
        let line = if line.starts_with("DW_AT_location\t(DW_OP_fbreg") {
            "DW_AT_location\t(DW_OP_fbreg _)".to_owned()
        } else {
            mask_offsets(line)
        };
        lines.push(line);
    }
    lines
}

/// A line with its DIE references and section offsets masked: a
/// reference `(0x0000002a "int")` keeps its name. Block contents
/// (`<0x10> 00 ...`) are kept.
fn mask_offsets(line: &str) -> String {
    // The radix of a constant (§6.7): `(0x01)` is `(1)`.
    if let Some((at, value)) = line.split_once("\t(0x")
        && let Some(digits) = value.strip_suffix(')')
        && let Ok(n) = u64::from_str_radix(digits, 16)
    {
        return format!("{at}\t({n})");
    }
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(at) = rest.find("(0x") {
        out.push_str(&rest[..at]);
        out.push_str("(0x_");
        rest = rest[at + 3..].trim_start_matches(|c: char| c.is_ascii_hexdigit());
    }
    out.push_str(rest);
    out
}

/// hod's object of `text`, compared with `llc` 22's when it is installed:
/// the same DIE tree, hod's verified. Returns hod's tree (empty without
/// `llvm-dwarfdump`).
fn compare(scratch: &Scratch, name: &str, text: &str) -> Vec<String> {
    let object = compile(text).unwrap_or_else(|e| panic!("{name}: {e}"));
    let hod = scratch.write(&format!("{name}.hod.o"), &object);
    let Some(dwarfdump) = tool("llvm-dwarfdump") else { return Vec::new() };
    let verified = run_tool(&dwarfdump, &["--verify".as_ref(), hod.as_os_str()]);
    assert!(verified.contains("No errors."), "{name}: {verified}");
    let ours = tree(&dwarfdump, &hod);
    let Some(llc) = tool("llc-22") else { return ours };
    let ll = scratch.write(&format!("{name}.ll"), text);
    let reference = scratch.join(format!("{name}.llc.o"));
    let mut command = std::process::Command::new(&llc);
    command
        .args(["-O0", "-global-isel=0", "-fast-isel=0", "-filetype=obj", "-o"])
        .arg(&reference)
        .arg(&ll);
    let out = run_with(&mut command, &[], Duration::from_mins(2));
    assert!(out.success(), "{name}: llc: {}", out.stderr_text());
    let theirs = tree(&dwarfdump, &reference);
    let producer = |l: &&String| !l.starts_with("DW_AT_producer");
    let a: Vec<&String> = ours.iter().filter(producer).collect();
    let b: Vec<&String> = theirs.iter().filter(producer).collect();
    if let Some(i) = (0..a.len().max(b.len())).find(|&i| a.get(i) != b.get(i)) {
        panic!(
            "{name}: line {i}: hod {:?}, llc {:?}\nhod:\n{}\nllc:\n{}",
            a.get(i),
            b.get(i),
            ours.join("\n"),
            theirs.join("\n")
        );
    }
    ours
}

fn count_prefix(tree: &[String], prefix: &str) -> usize {
    tree.iter().filter(|l| l.starts_with(prefix)).count()
}

/// §6.27: `i128` and `long double` static members' values are `block1`s
/// of their bytes, as `llc` writes them, in DWARF 5 (`DW_TAG_variable`,
/// family 114) and DWARF 4 (`DW_TAG_member`, family 115, written as
/// version 4).
#[test]
fn a_static_members_value_wider_than_64_bits_is_a_block() {
    let scratch = Scratch::new("wp210-wide");
    for triple in TRIPLES {
        for (version, tag) in [(5, "DW_TAG_variable"), (5, "DW_TAG_member"), (4, "DW_TAG_member")] {
            let name = format!("wide-{}-{version}-{}", &triple[..6], &tag[7..]);
            let ours = compare(&scratch, &name, &wide(triple, version, tag));
            if ours.is_empty() {
                continue;
            }
            assert_eq!(count_prefix(&ours, "DW_AT_const_value\t(<"), 4, "{name}: four blocks");
            // 1.5L: `x86_fp80`'s 10 bytes, `fp128`'s 16.
            let one = if triple.starts_with("x86_64") {
                "DW_AT_const_value\t(<0x0a> 00 00 00 00 00 00 00 c0 ff 3f )"
            } else {
                "DW_AT_const_value\t(<0x10> 00 00 00 00 00 00 00 00 00 00 00 00 00 80 ff 3f )"
            };
            assert_eq!(count_prefix(&ours, one), 1, "{name}: one");
            assert_eq!(
                count_prefix(
                    &ours,
                    "DW_AT_const_value\t(<0x10> 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 )"
                ),
                1,
                "{name}: min\n{ours:#?}"
            );
            assert_eq!(
                count_prefix(
                    &ours,
                    "DW_AT_const_value\t(<0x10> fe ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff )"
                ),
                1,
                "{name}: max"
            );
            assert_eq!(count_prefix(&ours, "DW_AT_alignment\t(16)"), 1, "{name}: big's alignment");
        }
    }
}

/// `int f(int x)` inlined into `g`; `f`'s `retainedNodes` are `x`, `y`
/// (in a block), `z` (a slot the inlined code declares), `w`, and in
/// the inlined-only module the label `out`. FFIRST and GFIRST place
/// `f`'s own code before or after `g`'s.
const RETAINED: &str = r#"target triple = "TRIPLE"

FFIRST
define dso_local i32 @g(i32 %a) #0 !dbg !10 {
  %s = alloca i32, align 4
    #dbg_declare(ptr %s, !16, !DIExpression(), !20)
  store i32 %a, ptr %s, align 4, !dbg !20
  %r = add i32 %a, 1, !dbg !20
  ret i32 %r, !dbg !21
}
GFIRST
attributes #0 = { noinline nounwind "frame-pointer"="all" }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "kiln", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, nameTableKind: None)
!1 = !DIFile(filename: "r.cpp", directory: "/hod/tests")
!2 = !{i32 7, !"Dwarf Version", i32 VERSION}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!5 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!6 = !DISubroutineType(types: !{!5, !5})
!10 = distinct !DISubprogram(name: "g", scope: !1, file: !1, line: 9, type: !6, scopeLine: 9, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0)
!11 = distinct !DISubprogram(name: "f", scope: !1, file: !1, line: 1, type: !6, scopeLine: 1, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !12)
!12 = !{!13, !14, !16, !17LABEL}
!13 = !DILocalVariable(name: "x", arg: 1, scope: !11, file: !1, line: 1, type: !5)
!14 = !DILocalVariable(name: "y", scope: !15, file: !1, line: 3, type: !5)
!15 = distinct !DILexicalBlock(scope: !11, file: !1, line: 2, column: 1)
!16 = !DILocalVariable(name: "z", scope: !11, file: !1, line: 4, type: !5)
!17 = !DILocalVariable(name: "w", scope: !11, file: !1, line: 5, type: !5)
!18 = !DILabel(scope: !11, name: "out", file: !1, line: 6)
!20 = !DILocation(line: 2, column: 3, scope: !11, inlinedAt: !22)
!21 = !DILocation(line: 10, column: 3, scope: !10)
!22 = distinct !DILocation(line: 10, column: 10, scope: !10)
!30 = !DILocation(line: 2, column: 3, scope: !11)
"#;

const F_BODY: &str = "define dso_local i32 @f(i32 %x) #0 !dbg !11 {
  %r = add i32 %x, 1, !dbg !30
  ret i32 %r, !dbg !30
}
";

/// §6.28: the abstract instance of `f` has `x`, `z`, `w`, `out` and the
/// block with `y`, in `llc`'s order; `f`'s own code, before or after `g`,
/// names its retained `x` by its abstract origin. DWARF 5 and 4, both
/// targets.
#[test]
fn an_inlined_subprograms_retained_nodes_are_abstract_entities() {
    let scratch = Scratch::new("wp210-retained");
    for triple in TRIPLES {
        for version in [5, 4] {
            for (order, ffirst, gfirst, label) in [
                ("inlined", "", "", ", !18"),
                ("f-first", F_BODY, "", ""),
                ("g-first", "", F_BODY, ""),
            ] {
                let name = format!("retained-{}-{version}-{order}", &triple[..6]);
                let text = RETAINED
                    .replace("TRIPLE", triple)
                    .replace("VERSION", &version.to_string())
                    .replace("FFIRST", ffirst)
                    .replace("GFIRST", gfirst)
                    .replace("LABEL", label);
                let ours = compare(&scratch, &name, &text);
                if ours.is_empty() {
                    continue;
                }
                let names: Vec<&str> = ours
                    .iter()
                    .filter_map(|l| l.strip_prefix("DW_AT_name\t(\""))
                    .map(|l| l.trim_end_matches("\")"))
                    .collect();
                let want: &[&str] = if label.is_empty() {
                    &["r.cpp", "f", "x", "z", "w", "y", "int", "g"]
                } else {
                    &["r.cpp", "f", "x", "z", "w", "out", "y", "int", "g"]
                };
                // `f`'s concrete DIE has no name: it is its origin's.
                assert_eq!(names, want, "{name}");
                let origins = count_prefix(&ours, "DW_AT_abstract_origin");
                // Inlined `f` and its `z`; with `f`'s code, also its DIE
                // and its retained `x`, `z` and `w` (`y`'s block has none).
                assert_eq!(origins, if label.is_empty() { 6 } else { 2 }, "{name}");
            }
        }
    }
}

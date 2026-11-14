//! WP-203: vectors wider than 128 bits under AVX (`spec/abi-x64.md` §1.3,
//! §3.2, §4.3, §5.1, §9.1): the grid of §11 for functions compiled for
//! the `-march` strings of WP-201's census and for `+avx` and `+avx2`
//! alone, hod's assignment under each against `llc` 19 and 22 in both
//! directions, run under `qemu-x86_64 -cpu max`.

use std::collections::{BTreeMap, HashSet};

use hod_x64::abi::Features;

use crate::grid::{pressures, random_case, run_cases, scalars, seq, vectors};
use crate::run::{Machine, Tools};
use crate::sigs::{Attr, Case, byval, p, pa, table};
use crate::ty::{I1, I8, I16, I32, I64, Rng, Ty, arr, st, vec};

/// The census (`tests/features/x64_census.tsv`).
const CENSUS: &str = include_str!("../features/x64_census.tsv");

/// The `target-cpu` and `target-features` clang 22 (else 19) writes for
/// `-march=<cpu>`.
pub fn census(cpu: &str) -> Machine {
    let flag = format!("-march={cpu}");
    let mut found = None;
    for line in CENSUS.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == 5 && fields[1] == flag {
            found = Some(Machine::new(fields[2], fields[4]));
            if fields[0].contains("clang22") {
                break;
            }
        }
    }
    found.unwrap_or_else(|| panic!("{flag} is not in the census"))
}

/// The machines of the grid: `+avx` and `+avx2` on the baseline CPU, and
/// four `-march` strings of the census (AVX alone, AVX2, AVX-512, AMD).
pub fn machines() -> Vec<Machine> {
    let plain = |features: &str| Machine::new("x86-64", features);
    vec![
        plain("+avx"),
        plain("+avx2"),
        census("sandybridge"),
        census("haswell"),
        census("skylake-avx512"),
        census("znver3"),
    ]
}

/// Vectors of every element type whose shape AVX changes or keeps
/// (`spec/abi-x64.md` §4.3): 256 bits, two and four times that, those
/// widened to 256 bits in one step, those LLVM passes element by element
/// with AVX too, and 128-bit neighbours.
pub fn wide_vectors() -> Vec<Ty> {
    let (h, b, f, d) = (Ty::Half, Ty::BFloat, Ty::Float, Ty::Double);
    let shapes: [(Ty, &[u32]); 10] = [
        (I1, &[8, 16, 32, 64]),
        (I8, &[17, 24, 32, 48, 64, 128]),
        (I16, &[9, 12, 16, 32, 64]),
        (I32, &[5, 6, 7, 8, 12, 16, 32]),
        (I64, &[3, 4, 5, 8, 16]),
        (Ty::Ptr, &[3, 4, 8]),
        (h, &[9, 12, 16, 32]),
        (b, &[9, 16, 32]),
        (f, &[5, 6, 7, 8, 9, 12, 16, 32]),
        (d, &[3, 4, 5, 8, 16]),
    ];
    let mut all = Vec::new();
    for (elem, lens) in shapes {
        all.extend(lens.iter().map(|&len| vec(elem.clone(), len)));
    }
    all
}

/// The aggregates clang and kiln pass `__m256` in, and others.
fn wide_aggregates() -> Vec<Ty> {
    let (v4i64, v8f32, v4f32) = (vec(I64, 4), vec(Ty::Float, 8), vec(Ty::Float, 4));
    let v4f64 = vec(Ty::Double, 4);
    vec![
        st(std::slice::from_ref(&v4i64)),
        st(&[v4i64.clone(), v4i64.clone()]),
        arr(v8f32.clone(), 2),
        st(&[arr(v8f32.clone(), 2)]),
        st(&[v4f32.clone(), v8f32.clone()]),
        st(&[v8f32.clone(), I32]),
        st(&[I8, v4f64.clone(), Ty::Double]),
        st(&[v4f64.clone(), v4f64.clone()]),
        st(&[v8f32.clone(), vec(Ty::Float, 16)]),
        Ty::Packed(vec![I8, v8f32.clone()]),
        arr(v4i64, 5),
    ]
}

/// The cases: every wide type under every pressure (as argument, return
/// and variadic), the aggregates, `byval` and `sret` around them, nine to
/// eleven `ymm` arguments, returns that share `xmm0`..`xmm3`.
pub fn wide_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut types = wide_vectors();
    types.extend(wide_aggregates());
    for ty in &types {
        cases.extend(pressures(ty));
        cases.push(Case::of(&seq(&[(&vec(Ty::Float, 4), 1), (ty, 2), (&I64, 1)])).ret(ty.clone()));
    }
    let (v4i64, v8f32, v4f32) = (vec(I64, 4), vec(Ty::Float, 8), vec(Ty::Float, 4));
    let d = Ty::Double;
    // Eight, nine and eleven `ymm` arguments, mixed with `xmm` ones.
    cases.push(Case::of(&seq(&[(&v4i64, 8), (&I32, 1), (&v4i64, 1)])).ret(v4i64.clone()));
    cases.push(Case::of(&seq(&[(&v8f32, 9), (&I32, 1), (&v4i64, 2), (&d, 1)])));
    cases.push(Case::of(&seq(&[(&v4f32, 3), (&v8f32, 5), (&d, 2), (&v8f32, 1)])));
    cases.push(Case::of(&seq(&[(&d, 7), (&v8f32, 2), (&v4f32, 1), (&I64, 7)])));
    // `byval` and `sret` holding wide vectors, as clang passes a
    // structure of one in memory.
    for pointee in [v4i64.clone(), st(&[v4i64.clone(), v4i64.clone()]), arr(v8f32.clone(), 2)] {
        cases.push(Case::new(vec![byval(pointee.clone(), Some(32), None), p(v8f32.clone())]));
        cases.push(Case::new(vec![
            pa(Ty::Ptr, Attr::Sret),
            p(v4i64.clone()),
            byval(pointee.clone(), Some(32), None),
            p(I32),
        ]));
        cases.push(Case::new(vec![p(I32), byval(pointee, Some(32), None), p(I8)]).variadic(1));
    }
    // Returns that share `xmm0`..`xmm3` with 128-bit pieces.
    for ret in [
        st(&[v4f32.clone(), v8f32.clone()]),
        st(&[v8f32.clone(), d.clone()]),
        st(&[v8f32.clone(), v4f32.clone(), v8f32.clone(), Ty::Float]),
        st(&[v8f32.clone(), v8f32.clone(), v8f32.clone(), v8f32.clone()]),
        st(&[v8f32.clone(), v8f32.clone(), v8f32.clone(), v8f32.clone(), v8f32.clone()]),
        vec(Ty::Float, 64),
    ] {
        cases.push(Case::of(&[v8f32.clone(), I16]).ret(ret));
    }
    // Variadic: named and unnamed `ymm` pieces.
    cases
        .push(Case::of(&[v8f32.clone(), I32, v8f32.clone(), d.clone(), v4f32.clone()]).variadic(1));
    cases.push(Case::of(&[I32, v8f32.clone(), v4i64.clone(), I16]).variadic(3).ret(v8f32));
    cases
}

/// The cases `assign` accepts under `features`, and the names of those
/// it refuses.
fn accepted_with(
    cases: Vec<Case>,
    features: Features,
) -> (Vec<Case>, BTreeMap<&'static str, usize>) {
    let mut types = table();
    let mut refused = BTreeMap::new();
    let mut seen = HashSet::new();
    let mut kept = Vec::new();
    for case in cases {
        match case.assign_with(&mut types, features) {
            Ok(_) => {
                if seen.insert(case.clone()) {
                    kept.push(case);
                }
            }
            Err(name) => *refused.entry(name).or_insert(0) += 1,
        }
    }
    (kept, refused)
}

/// `count` random signatures over every scalar, the 128-bit grid's
/// vectors and the wide ones.
fn random_wide(seed: u64, count: usize) -> Vec<Case> {
    let mut rng = Rng(seed);
    let mut pool = scalars();
    pool.extend(vectors());
    pool.extend(wide_vectors());
    pool.extend(wide_vectors());
    (0..count).map(|_| random_case(&mut rng, &pool)).collect()
}

/// `spec/abi-x64.md` §1.3, §3.2, §4.3, §5.1, §9.1 (WP-203): the wide grid
/// on every machine of [`machines`], against the six `llc`
/// configurations, both directions. The names refused are those §1.3
/// keeps: `x64:wide-vector-abi` (beyond 256 bits, or a vector of `i1`,
/// with AVX-512), `x64:mask-vector`, and the conventions §1.2 refuses.
#[test]
fn wide_vector_grid() {
    let Some(base) = Tools::find() else { return };
    for machine in machines() {
        let label = format!("abi-wide-{}{}", machine.cpu, machine.features.len());
        let features = machine.abi();
        assert!(features.avx, "{machine:?} has AVX");
        let Some(tools) = base.with_machine(machine.clone()) else { return };
        let (mut cases, refused) = accepted_with(wide_cases(), features);
        // The random signatures `assign` accepts; what it refuses of them
        // (invalid ones among them) is only counted.
        let (random, random_refused) = accepted_with(random_wide(0x203, 300), features);
        cases.extend(random.into_iter().filter(|case| !cases.contains(case)).collect::<Vec<_>>());
        eprintln!("random signatures refused while drawing: {random_refused:?}");
        for name in refused.keys() {
            assert!(
                ["x64:wide-vector-abi", "x64:mask-vector", "abi:cc", "x64:nest"].contains(name),
                "{} refused as {name}",
                machine.cpu
            );
        }
        if features.avx512 {
            // `qemu`'s TCG has no AVX-512: code compiled for this machine
            // stops on its first EVEX instruction. The evidence is that
            // hod places every accepted signature as each `llc` does for
            // this machine (WP-208, `zmm::same_as_llc`).
            let checked = crate::zmm::same_as_llc(&label, &cases, &machine, &tools);
            eprintln!(
                "wide grid, {}: {} signatures, {checked} llc conventions equal; \
                 refused: {refused:?}",
                machine.cpu,
                cases.len()
            );
            continue;
        }
        let report = run_cases(&label, &cases, 0x203, &tools);
        eprintln!(
            "wide grid, {} {}: {} signatures; {}; refused: {refused:?}; llc crashed on: {:#?}",
            machine.cpu,
            &machine.features[..machine.features.len().min(24)],
            cases.len(),
            report.summary(),
            report.llc_crashes
        );
        report.check(&format!("wide grid {}", machine.cpu));
    }
}

//! The `nop`s of code padding by tuning (`spec/asm.md` §18, WP-218): the
//! longest `nop` `llc` 19 and 22 write when an alignment directive of an
//! inline-assembly template pads, which follows the function's tuning
//! CPU and the tuning features of its `target-features`.
//!
//! Measured with `llc` 19 and 22 (`-O0`, a template `int3; .p2align 6;
//! int3`, 63 bytes of padding) for every CPU either major lists, as
//! `tune-cpu`, as `target-cpu` alone, with `-mcpu`, and with each tuning
//! feature turned on and off: LLVM's `X86AsmBackend::getMaximumNopSize`.
//! The rule:
//!
//! - The tuning CPU is `tune-cpu` when the attribute is there (even
//!   empty); else `target-cpu` (or `-mcpu` without it), `x86-64` read as
//!   `generic`; else none. No CPU, or an empty one, is LLVM's default
//!   tuning (`i586`): 10 bytes.
//! - Each CPU of [`NOP7`], [`NOP11`], [`NOP15`] has the one tuning
//!   feature `fast-7bytenop`, `fast-11bytenop` or `fast-15bytenop`; the
//!   CPUs of [`NOP10`] none. `+name`/`-name` items of `target-features`
//!   (then of `-mattr`) turn one on or off.
//! - The longest `nop` is 7 with `fast-7bytenop`, else 15 with
//!   `fast-15bytenop`, else 11 with `fast-11bytenop`, else 10.
//!
//! `diamondrapids`, `novalake` and `wildcatlake` are LLVM 22's (19 does
//! not know them and tunes as for no CPU): hod follows 22, kiln's LLVM.
//! Another CPU name is one hod has not measured: a template that pads
//! with `nop`s is refused ([`UNMEASURED`]).

/// A template pads with `nop`s in a function whose tuning CPU hod has
/// not measured.
pub const UNMEASURED: &str = "x64:align-tune-cpu";

/// The tuning CPUs with `fast-7bytenop`.
pub const NOP7: &[&str] = &["atom_sse4_2", "atom_sse4_2_movbe", "silvermont", "slm"];

/// The tuning CPUs with `fast-11bytenop`.
pub const NOP11: &[&str] = &["bdver1", "bdver2", "bdver3", "bdver4"];

/// The tuning CPUs with `fast-15bytenop`.
pub const NOP15: &[&str] = &[
    "alderlake",
    "arrowlake",
    "arrowlake-s",
    "arrowlake_s",
    "broadwell",
    "btver1",
    "btver2",
    "cannonlake",
    "cascadelake",
    "clearwaterforest",
    "cooperlake",
    "core-avx-i",
    "core-avx2",
    "core_2nd_gen_avx",
    "core_3rd_gen_avx",
    "core_4th_gen_avx",
    "core_4th_gen_avx_tsx",
    "core_5th_gen_avx",
    "core_5th_gen_avx_tsx",
    "corei7-avx",
    "diamondrapids",
    "emeraldrapids",
    "generic",
    "gracemont",
    "grandridge",
    "graniterapids",
    "graniterapids-d",
    "graniterapids_d",
    "haswell",
    "icelake-client",
    "icelake-server",
    "icelake_client",
    "icelake_server",
    "ivybridge",
    "lunarlake",
    "meteorlake",
    "novalake",
    "pantherlake",
    "raptorlake",
    "rocketlake",
    "sandybridge",
    "sapphirerapids",
    "sierraforest",
    "skx",
    "skylake",
    "skylake-avx512",
    "skylake_avx512",
    "tigerlake",
    "wildcatlake",
    "x86-64-v2",
    "x86-64-v3",
    "x86-64-v4",
    "znver1",
    "znver2",
    "znver3",
    "znver4",
    "znver5",
];

/// The tuning CPUs with none of the three (and the empty name).
pub const NOP10: &[&str] = &[
    "",
    "amdfam10",
    "athlon",
    "athlon-4",
    "athlon-fx",
    "athlon-mp",
    "athlon-tbird",
    "athlon-xp",
    "athlon64",
    "athlon64-sse3",
    "atom",
    "barcelona",
    "bonnell",
    "c3",
    "c3-2",
    "core2",
    "core_2_duo_sse4_1",
    "core_2_duo_ssse3",
    "core_aes_pclmulqdq",
    "core_i7_sse4_2",
    "corei7",
    "geode",
    "goldmont",
    "goldmont-plus",
    "goldmont_plus",
    "i386",
    "i486",
    "i586",
    "i686",
    "k6",
    "k6-2",
    "k6-3",
    "k8",
    "k8-sse3",
    "knl",
    "knm",
    "lakemont",
    "mic_avx512",
    "nehalem",
    "nocona",
    "opteron",
    "opteron-sse3",
    "penryn",
    "pentium",
    "pentium-m",
    "pentium-mmx",
    "pentium2",
    "pentium3",
    "pentium3m",
    "pentium4",
    "pentium4m",
    "pentium_4",
    "pentium_4_sse3",
    "pentium_ii",
    "pentium_iii",
    "pentium_iii_no_xmm_regs",
    "pentium_m",
    "pentium_mmx",
    "pentium_pro",
    "pentiumpro",
    "prescott",
    "tremont",
    "westmere",
    "winchip-c6",
    "winchip2",
    "x86-64",
    "yonah",
];

/// The longest padding `nop` of a function (module documentation):
/// `tune` and `cpu` are its `tune-cpu` and `target-cpu` (or `-mcpu`),
/// `features` the items of its `target-features` then of `-mattr`, in
/// order. `None` for a tuning CPU hod has not measured.
pub fn longest_nop<'a>(
    tune: Option<&str>,
    cpu: Option<&str>,
    features: impl IntoIterator<Item = &'a str>,
) -> Option<u8> {
    let tune = match (tune, cpu) {
        (Some(tune), _) => tune,
        (None, Some("x86-64")) => "generic",
        (None, Some(cpu)) => cpu,
        (None, None) => "",
    };
    let (mut f7, mut f11, mut f15) =
        (NOP7.contains(&tune), NOP11.contains(&tune), NOP15.contains(&tune));
    if !(f7 || f11 || f15 || NOP10.contains(&tune)) {
        return None;
    }
    for item in features {
        let (on, name) = match item.trim().split_at_checked(1) {
            Some(("+", name)) => (true, name),
            Some(("-", name)) => (false, name),
            _ => continue,
        };
        match name {
            "fast-7bytenop" => f7 = on,
            "fast-11bytenop" => f11 = on,
            "fast-15bytenop" => f15 = on,
            _ => {}
        }
    }
    Some(if f7 {
        7
    } else if f15 {
        15
    } else if f11 {
        11
    } else {
        10
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule's cases, as `llc` 19 and 22 write them (module
    /// documentation; `tests/inline_asm_align.rs` compares the rule with
    /// `llc` for every CPU).
    #[test]
    fn the_rule() {
        let none: [&str; 0] = [];
        assert_eq!(longest_nop(None, None, none), Some(10));
        assert_eq!(longest_nop(None, Some("x86-64"), none), Some(15));
        assert_eq!(longest_nop(Some("x86-64"), Some("x86-64"), none), Some(10));
        assert_eq!(longest_nop(Some("generic"), Some("x86-64"), none), Some(15));
        assert_eq!(longest_nop(Some(""), Some("skylake"), none), Some(10));
        assert_eq!(longest_nop(None, Some("slm"), ["+fast-15bytenop"]), Some(7));
        assert_eq!(longest_nop(None, Some("bdver1"), ["-fast-11bytenop"]), Some(10));
        assert_eq!(
            longest_nop(None, Some("x86-64"), ["+fast-11bytenop", "-fast-15bytenop"]),
            Some(11)
        );
        assert_eq!(
            longest_nop(None, Some("x86-64"), ["-fast-15bytenop", "+fast-15bytenop"]),
            Some(15)
        );
        assert_eq!(longest_nop(None, Some("a-future-cpu"), none), None);
        for list in [NOP7, NOP11, NOP15, NOP10] {
            for cpu in list {
                let others = [NOP7, NOP11, NOP15, NOP10].iter().filter(|l| l.contains(cpu)).count();
                assert_eq!(others, 1, "{cpu} in one list");
            }
        }
    }
}

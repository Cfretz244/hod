//! The prefetch instructions `prfm` and `prfum` (`spec/asm.md` §16,
//! WP-216): the operation by name or `#imm5`, and the words of each
//! addressing form.
//!
//! `hod_mc_a64` has no prefetch encoder (code generation never writes
//! one), so the words are built here, as `sysreg.rs` builds `mrs`/`msr`:
//! the prefetch forms are the load/store-register classes with `size` 3
//! and `opc` 2 (the 64-bit `ldrsw` slot), the operation in the `Rt`
//! field. `tests/llvm_mc.rs` holds every form to `llvm-mc` 22's and 19's
//! words.

/// The named prefetch operations and their `Rt` values, as `llvm-mc` 19
/// and 22 accept them (both the same, no feature needed). `llvm-mc`
/// prints the `slc` names back as `#6`, `#7`, ...; the values 24..31 have
/// no name.
pub const NAMES: [(&str, u32); 24] = [
    ("pldl1keep", 0),
    ("pldl1strm", 1),
    ("pldl2keep", 2),
    ("pldl2strm", 3),
    ("pldl3keep", 4),
    ("pldl3strm", 5),
    ("pldslckeep", 6),
    ("pldslcstrm", 7),
    ("plil1keep", 8),
    ("plil1strm", 9),
    ("plil2keep", 10),
    ("plil2strm", 11),
    ("plil3keep", 12),
    ("plil3strm", 13),
    ("plislckeep", 14),
    ("plislcstrm", 15),
    ("pstl1keep", 16),
    ("pstl1strm", 17),
    ("pstl2keep", 18),
    ("pstl2strm", 19),
    ("pstl3keep", 20),
    ("pstl3strm", 21),
    ("pstslckeep", 22),
    ("pstslcstrm", 23),
];

/// The value of a named operation, in any case.
pub fn named(name: &str) -> Option<u32> {
    NAMES.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|&(_, v)| v)
}

/// `PRFM <op>, [<Xn|SP>, #offset]`: PRFM (immediate), the unsigned
/// offset scaled by 8. Precondition: `op < 32`, `rn < 32`, `offset` a
/// multiple of 8 in `0..=32760`.
pub const fn prfm_off(op: u32, rn: u32, offset: u32) -> u32 {
    0xf980_0000 | (offset >> 3) << 10 | rn << 5 | op
}

/// `PRFUM <op>, [<Xn|SP>, #offset]`: the signed 9-bit byte offset.
/// Precondition: `op < 32`, `rn < 32`, `-256 <= offset <= 255`.
pub const fn prfum(op: u32, rn: u32, offset: i64) -> u32 {
    0xf880_0000 | ((offset as u32) & 0x1ff) << 12 | rn << 5 | op
}

/// `PRFM <op>, [<Xn|SP>, <Rm>, <option> {#amount}]`: PRFM (register),
/// `option` the index extension field (2 `uxtw`, 3 `lsl`, 6 `sxtw`, 7
/// `sxtx`), `scaled` the `S` bit (an amount of 3).
pub const fn prfm_reg(op: u32, rn: u32, rm: u32, option: u32, scaled: bool) -> u32 {
    0xf8a0_0800 | rm << 16 | option << 13 | (scaled as u32) << 12 | rn << 5 | op
}

/// `PRFM <op>, <label>`: PRFM (literal), the displacement field zero
/// (`R_AARCH64_LD_PREL_LO19`'s).
pub const fn prfm_lit(op: u32) -> u32 {
    0xd800_0000 | op
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_distinct_and_cover_0_to_23() {
        for (i, &(n, v)) in NAMES.iter().enumerate() {
            assert_eq!(v as usize, i, "{n}");
            assert_eq!(named(&n.to_ascii_uppercase()), Some(v));
        }
        assert_eq!(named("pldl4keep"), None);
    }

    #[test]
    fn words() {
        // `llvm-mc-22 -show-encoding`.
        assert_eq!(prfm_off(0, 8, 1024), 0xf982_0100);
        assert_eq!(prfm_off(19, 31, 8), 0xf980_07f3);
        assert_eq!(prfum(7, 31, -1), 0xf89f_f3e7);
        assert_eq!(prfm_reg(0, 0, 1, 3, true), 0xf8a1_7800);
        assert_eq!(prfm_lit(0), 0xd800_0000);
    }
}

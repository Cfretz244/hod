//! The operands of the alignment directives `.p2align`, `.balign` and
//! `.align` (`spec/asm.md` §9 "Alignment", §17), as LLVM's assembler
//! reads them, for module text and inline bodies alike.

use crate::module::{constant, split_commas};
use crate::reason::{Refusal, operand as op_reason};

/// The largest alignment hod takes: 2^16 bytes.
pub const MAX_ALIGN: u64 = 1 << 16;

/// An alignment directive's operands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Align {
    /// The alignment in bytes, a power of two (1 for `.balign 0`).
    pub align: u64,
    /// The fill byte, when one is written.
    pub fill: Option<u8>,
    /// The most bytes the directive may write, when a maximum is written
    /// that is less than the alignment (one at or above it has no
    /// effect, as `llvm-mc` warns).
    pub max: Option<u64>,
}

impl Align {
    /// The padding from offset `at` to the boundary: none when it is
    /// more than the maximum.
    pub fn pad(&self, at: u64) -> u64 {
        let pad = at.wrapping_neg() & (self.align - 1);
        if self.max.is_some_and(|m| pad > m) { 0 } else { pad }
    }
}

/// Reads `n{, fill{, max}}`: `n` is the power of two (`log2`) or the
/// byte count. An empty fill before a maximum (`.p2align 4,,8`) is no
/// fill.
///
/// # Errors
/// `asm:operand/range` for an alignment out of 0..=16 (`log2`) or not a
/// power of two up to 2^16 (bytes), a fill out of -128..=255, a maximum
/// below 1 (an `llvm-mc` error); `asm:operand/form` for more than three
/// operands; the expression refusals of an operand.
pub fn parse(rest: &str, log2: bool) -> Result<Align, Refusal> {
    let parts = split_commas(rest);
    let (n, fill, max) = match parts.as_slice() {
        [n] => (*n, None, None),
        [n, f] => (*n, Some(*f), None),
        [n, f, m] => (*n, (!f.is_empty()).then_some(*f), Some(*m)),
        _ => return Err(Refusal(op_reason::FORM)),
    };
    let n = constant(n)?;
    let align = if log2 {
        if !(0..=16).contains(&n) {
            return Err(Refusal(op_reason::RANGE));
        }
        1u64 << n
    } else {
        match u64::try_from(n) {
            Ok(0) => 1,
            Ok(a) if a.is_power_of_two() && a <= MAX_ALIGN => a,
            _ => return Err(Refusal(op_reason::RANGE)),
        }
    };
    let fill = match fill {
        None => None,
        Some(f) => {
            let f = constant(f)?;
            if !(-128..256).contains(&f) {
                return Err(Refusal(op_reason::RANGE));
            }
            Some(f as u8)
        }
    };
    let max = match max {
        None => None,
        Some(m) => {
            let m = u64::try_from(constant(m)?)
                .ok()
                .filter(|&m| m >= 1)
                .ok_or(Refusal(op_reason::RANGE))?;
            (m < align).then_some(m)
        }
    };
    Ok(Align { align, fill, max })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operands() {
        let a = |t, l| parse(t, l);
        assert_eq!(a("5", true), Ok(Align { align: 32, fill: None, max: None }));
        assert_eq!(a("16, 0", false), Ok(Align { align: 16, fill: Some(0), max: None }));
        assert_eq!(a("6,,8", true), Ok(Align { align: 64, fill: None, max: Some(8) }));
        assert_eq!(a("4,0xff,16", true), Ok(Align { align: 16, fill: Some(0xff), max: None }));
        assert_eq!(a("16,-1", false), Ok(Align { align: 16, fill: Some(0xff), max: None }));
        assert_eq!(a("0", false), Ok(Align { align: 1, fill: None, max: None }));
        assert_eq!(a("2+1", true), Ok(Align { align: 8, fill: None, max: None }));
        assert_eq!(a("17", true), Err(Refusal(op_reason::RANGE)));
        assert_eq!(a("-1", true), Err(Refusal(op_reason::RANGE)));
        assert_eq!(a("3", false), Err(Refusal(op_reason::RANGE)));
        assert_eq!(a("4,,0", true), Err(Refusal(op_reason::RANGE)));
        assert_eq!(a("4,0x100", true), Err(Refusal(op_reason::RANGE)));
        assert_eq!(a("4,1,2,3", true), Err(Refusal(op_reason::FORM)));
        assert!(a("4,", true).is_err());
        let m = Align { align: 64, fill: None, max: Some(8) };
        assert_eq!((m.pad(0), m.pad(56), m.pad(52), m.pad(4)), (0, 8, 0, 0));
    }
}

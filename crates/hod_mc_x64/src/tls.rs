//! The fixed-form instructions of the general-dynamic TLS access
//! (`spec/mc-x64.md` "Thread-local storage").
//!
//! A linker relaxes a TLS access by recognizing its exact bytes and
//! overwriting them, so the general-dynamic pair has a fixed length of 16
//! bytes, padded with prefixes that change nothing: `data16 lea rdi,
//! [rip + x@tlsgd]` then `data16 data16 rex64 call __tls_get_addr@PLT`.
//! The TLS descriptor call (`-mtls-dialect=gnu2`) is `call [rax]`,
//! whose `R_X86_64_TLSDESC_CALL` marks the instruction itself rather than
//! a field: [`tlsdesc_call`]. The other models are sequences of ordinary
//! constructors.

use crate::branch::call_m;
use crate::fixup::Fixup;
use crate::inst::{Inst, Op, plain, rel, rmem};
use crate::reg::{Mem, Reg, Size};

/// `data16 lea rdi, [rip + disp32]` (`66 48 8D 3D cd`): the first half of
/// a general-dynamic access, with a zero field at offset 4 for
/// `R_X86_64_TLSGD`.
#[inline]
pub const fn tls_gd_lea() -> Inst {
    let op = Op { p66: true, ..Op::one(0x8d).sized(Size::Q, 0) };
    rmem(op, Size::Q, Reg::Rdi, Mem::rip(0))
}

/// `data16 data16 rex64 call rel32` (`66 66 48 E8 cd`): the second half,
/// with a zero field at offset 4 for `R_X86_64_PLT32` against
/// `__tls_get_addr`.
#[inline]
pub const fn tls_gd_call() -> Inst {
    // The second `66` goes where a mandatory prefix would: after the first
    // and before REX.
    let op = Op { p66: true, w: true, ..Op::one(0xe8).rep(0x66) };
    rel(plain(op), Fixup::Rel32, 0)
}

/// `call [rax]` (`FF 10`): the call of a TLS descriptor, after `lea rax,
/// [rip + x@tlsdesc]` (`lea(Size::Q, Reg::Rax, Mem::rip(0))`,
/// `R_X86_64_GOTPC32_TLSDESC`). It has no field: its
/// `R_X86_64_TLSDESC_CALL` is at the instruction's first byte, with
/// addend 0. The linkers rewrite these two exact bytes (into `xchg ax,
/// ax` when they relax the descriptor).
#[inline]
pub const fn tlsdesc_call() -> Inst {
    call_m(Mem::base(Reg::Rax, 0))
}

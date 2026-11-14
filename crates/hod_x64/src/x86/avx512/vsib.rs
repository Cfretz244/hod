//! The AVX-512 gathers and scatters (`spec/codegen.md` §5.6.35 (10),
//! WP-224, design 0007): the 48 `llvm.x86.avx512.mask.gather*` and
//! `mask.scatter*` names of LLVM 19 and 22, all with a `<N x i1>` mask.
//!
//! A gather `(pass-through, ptr, index, mask, scale)` and a scatter
//! `(ptr, mask, index, data, scale)` are each one EVEX instruction, the
//! one `llc -O0` selects ([`EGather`], [`EScatter`]), at the length of the
//! wider of the index and the data. Accepted only in a function whose
//! features enable `avx512f`, with `avx512vl` too for a 128- or 256-bit
//! form and without `-evex512` for a 512-bit one; refused by the
//! intrinsic's own name otherwise. A scale other than 1, 2, 4 or 8 is
//! not one `llc` selects ([`SIGNATURE`]).
//!
//! - The mask goes to `k1` (`kmovw`, from its register, or from `r11`
//!   for a constant), a fresh copy for every call: the instruction clears
//!   its opmask as it completes, and no hod value ever lives in an opmask
//!   register, so the value of the mask is never the register cleared.
//!   The opmask is mandatory, also for a mask with every lane set.
//! - The index goes to `zmm17`, the data of a scatter to `zmm18`, the
//!   destination of a gather is `zmm16`: the destination is never the
//!   index (which faults). The pass-through is copied into `zmm16` first,
//!   or `zmm16` is cleared (`vpxord`) for a `zeroinitializer`, `undef` or
//!   `poison` one (a gather has no zeroing-masking).
//! - The base is the pointer's register, the displacement 0.
//! - A gather's result goes to its `xmm` or its slot (the lanes past the
//!   ones the instruction loads are 0, as the instruction leaves them).

use hod_cg::{CgResult, Class, FnCx};
use hod_ir::{Inst, Intrinsic, TypeKind, Value, ValueKind};
use hod_mc_x64 as mc;
use hod_mc_x64::{E3, EGather, EScatter, EVsib, Masking, Scale, Size, Vl};

use super::{K1, SIGNATURE, ZMM16, features, load, store, vector, zero_like, zmm};
use crate::common::{R11, deferred_first, gp, put, unsupported, value_ty};
use crate::features::{Avx512, evex512_off};
use crate::intrinsic::const_int;
use crate::target::X64;

/// The instruction of a row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Gather(EGather),
    Scatter(EScatter),
}

impl Op {
    /// `(index length, data length, element bytes, float data)`.
    fn shape(self, vl: Vl) -> (Vl, Vl, u8, bool) {
        match self {
            Op::Gather(g) => {
                (g.index_vl(vl), g.data_vl(vl), g.elem_bytes(), g.mnemonic().starts_with("vg"))
            }
            Op::Scatter(s) => {
                (s.index_vl(vl), s.data_vl(vl), s.elem_bytes(), s.mnemonic().starts_with("vs"))
            }
        }
    }

    /// Whether the index elements are 64 bits (`q`).
    fn q_index(self) -> bool {
        let mn = match self {
            Op::Gather(g) => g.mnemonic(),
            Op::Scatter(s) => s.mnemonic(),
        };
        mn.contains("gatherq") || mn.contains("scatterq")
    }
}

/// A row: the refusal, the instruction, its length.
type Row = (&'static str, Op, Vl);

#[allow(clippy::too_many_lines, reason = "one arm per name")]
fn row(which: Intrinsic<'_>) -> Option<Row> {
    use EGather as G;
    use EScatter as S;
    use Intrinsic as I;
    use Op::{Gather, Scatter};
    use Vl::{X, Y, Z};
    Some(match which {
        I::X86Avx512MaskGatherDpd512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.dpd.512", Gather(G::Gatherdpd), Z)
        }
        I::X86Avx512MaskGatherDps512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.dps.512", Gather(G::Gatherdps), Z)
        }
        I::X86Avx512MaskGatherQpd512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.qpd.512", Gather(G::Gatherqpd), Z)
        }
        I::X86Avx512MaskGatherQps512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.qps.512", Gather(G::Gatherqps), Z)
        }
        I::X86Avx512MaskGatherDpq512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.dpq.512", Gather(G::Pgatherdq), Z)
        }
        I::X86Avx512MaskGatherDpi512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.dpi.512", Gather(G::Pgatherdd), Z)
        }
        I::X86Avx512MaskGatherQpq512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.qpq.512", Gather(G::Pgatherqq), Z)
        }
        I::X86Avx512MaskGatherQpi512 => {
            ("intrinsic:llvm.x86.avx512.mask.gather.qpi.512", Gather(G::Pgatherqd), Z)
        }
        I::X86Avx512MaskGather3div2Df => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div2.df", Gather(G::Gatherqpd), X)
        }
        I::X86Avx512MaskGather3div2Di => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div2.di", Gather(G::Pgatherqq), X)
        }
        I::X86Avx512MaskGather3div4Df => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div4.df", Gather(G::Gatherqpd), Y)
        }
        I::X86Avx512MaskGather3div4Di => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div4.di", Gather(G::Pgatherqq), Y)
        }
        I::X86Avx512MaskGather3div4Sf => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div4.sf", Gather(G::Gatherqps), X)
        }
        I::X86Avx512MaskGather3div4Si => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div4.si", Gather(G::Pgatherqd), X)
        }
        I::X86Avx512MaskGather3div8Sf => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div8.sf", Gather(G::Gatherqps), Y)
        }
        I::X86Avx512MaskGather3div8Si => {
            ("intrinsic:llvm.x86.avx512.mask.gather3div8.si", Gather(G::Pgatherqd), Y)
        }
        I::X86Avx512MaskGather3siv2Df => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv2.df", Gather(G::Gatherdpd), X)
        }
        I::X86Avx512MaskGather3siv2Di => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv2.di", Gather(G::Pgatherdq), X)
        }
        I::X86Avx512MaskGather3siv4Df => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv4.df", Gather(G::Gatherdpd), Y)
        }
        I::X86Avx512MaskGather3siv4Di => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv4.di", Gather(G::Pgatherdq), Y)
        }
        I::X86Avx512MaskGather3siv4Sf => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv4.sf", Gather(G::Gatherdps), X)
        }
        I::X86Avx512MaskGather3siv4Si => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv4.si", Gather(G::Pgatherdd), X)
        }
        I::X86Avx512MaskGather3siv8Sf => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv8.sf", Gather(G::Gatherdps), Y)
        }
        I::X86Avx512MaskGather3siv8Si => {
            ("intrinsic:llvm.x86.avx512.mask.gather3siv8.si", Gather(G::Pgatherdd), Y)
        }
        I::X86Avx512MaskScatterDpd512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.dpd.512", Scatter(S::Scatterdpd), Z)
        }
        I::X86Avx512MaskScatterDps512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.dps.512", Scatter(S::Scatterdps), Z)
        }
        I::X86Avx512MaskScatterQpd512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.qpd.512", Scatter(S::Scatterqpd), Z)
        }
        I::X86Avx512MaskScatterQps512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.qps.512", Scatter(S::Scatterqps), Z)
        }
        I::X86Avx512MaskScatterDpq512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.dpq.512", Scatter(S::Pscatterdq), Z)
        }
        I::X86Avx512MaskScatterDpi512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.dpi.512", Scatter(S::Pscatterdd), Z)
        }
        I::X86Avx512MaskScatterQpq512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.qpq.512", Scatter(S::Pscatterqq), Z)
        }
        I::X86Avx512MaskScatterQpi512 => {
            ("intrinsic:llvm.x86.avx512.mask.scatter.qpi.512", Scatter(S::Pscatterqd), Z)
        }
        I::X86Avx512MaskScatterdiv2Df => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv2.df", Scatter(S::Scatterqpd), X)
        }
        I::X86Avx512MaskScatterdiv2Di => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv2.di", Scatter(S::Pscatterqq), X)
        }
        I::X86Avx512MaskScatterdiv4Df => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv4.df", Scatter(S::Scatterqpd), Y)
        }
        I::X86Avx512MaskScatterdiv4Di => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv4.di", Scatter(S::Pscatterqq), Y)
        }
        I::X86Avx512MaskScatterdiv4Sf => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv4.sf", Scatter(S::Scatterqps), X)
        }
        I::X86Avx512MaskScatterdiv4Si => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv4.si", Scatter(S::Pscatterqd), X)
        }
        I::X86Avx512MaskScatterdiv8Sf => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv8.sf", Scatter(S::Scatterqps), Y)
        }
        I::X86Avx512MaskScatterdiv8Si => {
            ("intrinsic:llvm.x86.avx512.mask.scatterdiv8.si", Scatter(S::Pscatterqd), Y)
        }
        I::X86Avx512MaskScattersiv2Df => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv2.df", Scatter(S::Scatterdpd), X)
        }
        I::X86Avx512MaskScattersiv2Di => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv2.di", Scatter(S::Pscatterdq), X)
        }
        I::X86Avx512MaskScattersiv4Df => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv4.df", Scatter(S::Scatterdpd), Y)
        }
        I::X86Avx512MaskScattersiv4Di => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv4.di", Scatter(S::Pscatterdq), Y)
        }
        I::X86Avx512MaskScattersiv4Sf => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv4.sf", Scatter(S::Scatterdps), X)
        }
        I::X86Avx512MaskScattersiv4Si => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv4.si", Scatter(S::Pscatterdd), X)
        }
        I::X86Avx512MaskScattersiv8Sf => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv8.sf", Scatter(S::Scatterdps), Y)
        }
        I::X86Avx512MaskScattersiv8Si => {
            ("intrinsic:llvm.x86.avx512.mask.scattersiv8.si", Scatter(S::Pscatterdd), Y)
        }
        _ => return None,
    })
}

/// The stable name `which` is refused with, if it is one of this
/// module's.
pub fn refusal(which: Intrinsic<'_>) -> Option<&'static str> {
    row(which).map(|row| row.0)
}

/// A call of `which` with `args`, if `which` is one of this module's.
pub fn intrinsic<'a>(
    cx: &mut FnCx<'a, X64>,
    inst: Inst<'a>,
    which: Intrinsic<'a>,
    args: &'a [Value],
) -> Option<CgResult<()>> {
    let row = row(which)?;
    Some(lower(cx, inst, args, row))
}

/// Whether `value` is a vector of `bytes` bytes of `bits`-bit elements,
/// floating-point or not as `float` says, in the class of its size.
fn is_vector(cx: &FnCx<'_, X64>, value: Value, bytes: u8, bits: u32, float: bool) -> bool {
    let ty = value_ty(cx, value);
    let Some((_, b, n)) = vector(cx, ty) else { return false };
    let TypeKind::Vector { elem, .. } = cx.types().kind(ty) else { return false };
    let is_float = matches!(cx.types().kind(elem), TypeKind::Float | TypeKind::Double);
    let class = if n == 16 { Class::Fpr } else { Class::Mem };
    b == bits && n == u32::from(bytes) && is_float == float && cx.class_of(value) == class
}

fn lower<'a>(
    cx: &mut FnCx<'a, X64>,
    inst: Inst<'a>,
    args: &'a [Value],
    (refused, op, vl): Row,
) -> CgResult<()> {
    if args.len() != 5 {
        return unsupported(SIGNATURE);
    }
    let (index_vl, data_vl, elem, float) = op.shape(vl);
    let index_bits = if op.q_index() { 64 } else { 32 };
    let lanes = (u32::from(data_vl.bytes()) / u32::from(elem))
        .min(u32::from(index_vl.bytes()) * 8 / index_bits);
    let (data, ptr, index, mask, scale) = match op {
        Op::Gather(_) => (args[0], args[1], args[2], args[3], args[4]),
        Op::Scatter(_) => (args[3], args[0], args[2], args[1], args[4]),
    };
    // The shapes: the data and the index of their lengths, the mask of
    // the lanes the instruction reads, a gather's pass-through of its
    // result's type, the pointer a pointer, the scale a constant.
    let data_ok = is_vector(cx, data, data_vl.bytes(), u32::from(elem) * 8, float)
        && match op {
            Op::Gather(_) => value_ty(cx, data) == inst.ty(),
            Op::Scatter(_) => true,
        };
    let index_ok = is_vector(cx, index, index_vl.bytes(), index_bits, false);
    let mask_ty = value_ty(cx, mask);
    let mask_ok = crate::mask::mask_lanes(cx.types(), mask_ty) == Some(lanes)
        && cx.class_of(mask) == Class::Gpr;
    let ptr_ok = matches!(cx.types().kind(value_ty(cx, ptr)), TypeKind::Ptr { .. })
        && cx.class_of(ptr) == Class::Gpr;
    let scale = match const_int(cx, scale).map(|s| s & 0xffff_ffff) {
        Some(1) => Some(Scale::S1),
        Some(2) => Some(Scale::S2),
        Some(4) => Some(Scale::S4),
        Some(8) => Some(Scale::S8),
        _ => None,
    };
    let scale_ok = matches!(cx.types().kind(value_ty(cx, args[4])), TypeKind::Int(32));
    let (true, true, true, true, true, Some(scale)) =
        (data_ok, index_ok, mask_ok, ptr_ok, scale_ok, scale)
    else {
        return unsupported(SIGNATURE);
    };
    let features = features(cx);
    if !Avx512::F.enabled(features)
        || (vl != Vl::Z && !Avx512::Vl.enabled(features))
        || (vl == Vl::Z && evex512_off(features))
    {
        return unsupported(refused);
    }
    deferred_first(cx, inst, None)?;

    // The mask: a copy in `k1`, every time.
    let constant = match mask.kind() {
        ValueKind::Const(id) => crate::mask::const_bits(cx, id, lanes),
        _ => None,
    };
    if let Some(c) = constant {
        put(cx, mc::mov_ri(Size::D, R11, c as i64));
        put(cx, mc::kmov_kr(Size::W, K1, R11));
    } else {
        let r = cx.use_reg(mask)?;
        put(cx, mc::kmov_kr(Size::W, K1, gp(r)));
        cx.release(mask);
        cx.free_temp(r);
    }
    let zmm17 = zmm(17);
    let zmm18 = zmm(18);
    match op {
        Op::Gather(g) => {
            if zero_like(cx, data) {
                put(cx, mc::e3_rrr(E3::Pxord, Vl::X, Masking::NONE, ZMM16, ZMM16, ZMM16));
            } else {
                load(cx, data, ZMM16, u32::from(data_vl.bytes()))?;
            }
            load(cx, index, zmm17, u32::from(index_vl.bytes()))?;
            let base = cx.use_reg(ptr)?;
            let at = EVsib { base: gp(base), index: zmm17, scale, disp: 0 };
            put(cx, mc::egather(g, vl, K1, ZMM16, at));
            cx.release(ptr);
            cx.free_temp(base);
            store(cx, inst, ZMM16, u32::from(data_vl.bytes()))
        }
        Op::Scatter(s) => {
            load(cx, index, zmm17, u32::from(index_vl.bytes()))?;
            load(cx, data, zmm18, u32::from(data_vl.bytes()))?;
            let base = cx.use_reg(ptr)?;
            let at = EVsib { base: gp(base), index: zmm17, scale, disp: 0 };
            put(cx, mc::escatter(s, vl, K1, at, zmm18));
            cx.release(ptr);
            cx.free_temp(base);
            Ok(())
        }
    }
}

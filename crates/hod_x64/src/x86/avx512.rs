//! The AVX-512 intrinsics (`spec/codegen.md` §5.6.35, WP-208, design
//! `docs/design/0007-avx512.md`): the `llvm.x86.avx512.*` names clang 19
//! and 22 emit at `-O0` for the F, VL, BW and DQ functions of the
//! AVX-512 headers that are not plain IR (the census of design 0007
//! §5.1), and the VBMI2 compresses that share F's overloaded name; since
//! WP-220 also the VBMI, BF16 and FP16 names and the EVEX (512-bit) forms
//! of GFNI, VAES and VPCLMULQDQ.
//!
//! **Owner: WP-208.** Tests: `tests/x86_avx512.rs`. Spec: §5.6.35.
//!
//! Each name is accepted only in a function whose `target-features`
//! enable its family ([`Avx512`]), with `avx512vl` too for a 128- or
//! 256-bit form, and without `-evex512` for a 512-bit one; it is refused
//! by its own name (`intrinsic:llvm.x86.avx512...`) otherwise. With the
//! features, hod writes the EVEX instruction `llc -O0` selects:
//!
//! - every vector operand is copied into a register of its own from
//!   `zmm17` up (`vmovdqu64`, from the operand's `xmm` or its frame
//!   slot), so no register of the pool stays pinned; the result is
//!   computed in `zmm16` (or, for a shift by 0, is the source) and goes
//!   to its `xmm` or its slot. hod's pool never holds a value in
//!   `zmm16`..`zmm31` or in an opmask register, and every one of them is
//!   caller-saved;
//! - a mask is moved into `k1` (`kmovw`, or `kmovd`/`kmovq` for 32 and 64
//!   lanes); a constant mask with every lane the instruction reads set is
//!   no mask. The destination merges into the pass-through (copied into
//!   `zmm16` first), or is zeroed (`{z}`) when the pass-through is
//!   `zeroinitializer`, `undef` or `poison`;
//! - the rounding operand gives `{sae}` (8 or 12) or `{rn-sae}`..
//!   `{rz-sae}` (8 to 11), none for 4; another value is not one `llc`
//!   selects and is refused with [`SIGNATURE`];
//! - the operand order is the instruction's: the index first for
//!   `permvar` (`vpermps dst, idx, a`), `a` the tied destination of
//!   `vpermi2var` (`vpermt2ps`) and `pternlog`, `b` that of `vfmaddsub`
//!   (`vfmaddsub213ps`);
//! - a shift by a constant count folds as `llc` does: 0 is the source, a
//!   count at or above the element width is 0 for a logical shift and the
//!   width less one for an arithmetic one; a variable count goes through
//!   `xmm15` (`movd`) to the shift by a vector count;
//! - `fpclass` writes `k0` and its result (`<N x i1>`, class `Gpr`) is
//!   `kmovw`'s zero-extension.
//!
//! The upper halves of `zmm0`..`zmm15` are not cleared afterwards
//! (`vzeroupper`): they hold no value of hod's, and a later legacy SSE
//! instruction is only slower.

use hod_cg::{CgResult, Class, FnCx};
use hod_ir::{FnStr, Inst, Intrinsic, SymbolKind, TypeId, TypeKind, Value, ValueKind};
use hod_mc_x64 as mc;
use hod_mc_x64::{
    E2, E2Imm, E3, E3Imm, ECmp, EMov, EShift, EStore, Masking, Round, Size, Vl, Xmm, Zmm,
};

use super::SIGNATURE;
use crate::common::{R11, XMM15, deferred_first, frame_inst, gp, put, unsupported, value_ty, xm};
use crate::features::{Avx512, evex512_off};
use crate::intrinsic::const_int;
use crate::target::X64;

/// Where an EVEX operand order differs from the intrinsic's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Form {
    /// `op dst, v0, v1`.
    Plain,
    /// `op dst, v1, v0` (`permvar`: the index first).
    Swap,
    /// `dst` (read) is `v[d]`: `op v[d], v[a], v[b]`.
    Tied([usize; 3]),
}

/// `vpermt2*`: `a` is the destination, the index `vvvv`.
const PERMT2: Form = Form::Tied([0, 1, 2]);
/// `vfmaddsub213*`: `b·a ± c` into `b`.
const FMA213: Form = Form::Tied([1, 0, 2]);
/// `vpternlog*`: `a` is the destination.
const TERNLOG: Form = Form::Tied([0, 1, 2]);

/// What a row writes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    E3(E3, Form),
    E3Imm(E3Imm, Form),
    E2(E2),
    E2Imm(E2Imm),
    /// A shift by an immediate count: the immediate form, the form by a
    /// vector count, whether it is logical.
    Shift(EShift, E3, bool),
    /// A narrowing move into a register.
    Store(EStore),
    /// `vfpclass*`.
    Cmp(ECmp),
    /// `vcompress*`/`vpcompress*`, by the element type.
    Compress,
    /// `vcvtps2ph`.
    Ph,
}

/// A row: the refusal, the family, the role of each operand (`v` a
/// vector, `p` the pass-through, `m` the mask, `i` an immediate, `e` a
/// rounding mode, `s` exception suppression, `c` a shift count), the
/// instruction.
type Row = (&'static str, Avx512, &'static str, Op);

#[allow(clippy::too_many_lines, reason = "one arm per name")]
fn row(which: Intrinsic<'_>) -> Option<Row> {
    use Avx512::{Bf16, Bw, Dq, F, Gfni, Vaes, Vbmi, Vpclmulqdq};
    use Intrinsic as I;
    Some(match which {
        I::X86Avx512MaskGetexpPs512 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.ps.512", F, "vpms", Op::E2(E2::Getexpps))
        }
        I::X86Avx512MaskRndscalePs512 => (
            "intrinsic:llvm.x86.avx512.mask.rndscale.ps.512",
            F,
            "vipms",
            Op::E2Imm(E2Imm::Rndscaleps),
        ),
        I::X86Avx512MaxPs512 => {
            ("intrinsic:llvm.x86.avx512.max.ps.512", F, "vvs", Op::E3(E3::Maxps, Form::Plain))
        }
        I::X86Avx512MinPs512 => {
            ("intrinsic:llvm.x86.avx512.min.ps.512", F, "vvs", Op::E3(E3::Minps, Form::Plain))
        }
        I::X86Avx512PermvarSf512 => {
            ("intrinsic:llvm.x86.avx512.permvar.sf.512", F, "vv", Op::E3(E3::Permps, Form::Swap))
        }
        I::X86Avx512Rcp14Ps512 => {
            ("intrinsic:llvm.x86.avx512.rcp14.ps.512", F, "vpm", Op::E2(E2::Rcp14ps))
        }
        I::X86Avx512Rsqrt14Ps512 => {
            ("intrinsic:llvm.x86.avx512.rsqrt14.ps.512", F, "vpm", Op::E2(E2::Rsqrt14ps))
        }
        I::X86Avx512VfmaddsubPs512 => (
            "intrinsic:llvm.x86.avx512.vfmaddsub.ps.512",
            F,
            "vvve",
            Op::E3(E3::Vfmaddsub213ps, FMA213),
        ),
        I::X86Avx512Vpermi2varPs512 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.ps.512", F, "vvv", Op::E3(E3::Permt2ps, PERMT2))
        }
        I::X86Avx512MaskVcvtps2ph512 => {
            ("intrinsic:llvm.x86.avx512.mask.vcvtps2ph.512", F, "vipm", Op::Ph)
        }
        I::X86Avx512MaskCompress => {
            ("intrinsic:llvm.x86.avx512.mask.compress", F, "vpm", Op::Compress)
        }
        I::X86Avx512MaskCvtps2dq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtps2dq.512", F, "vpme", Op::E2(E2::Cvtps2dq))
        }
        I::X86Avx512MaskCvttps2dq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2dq.512", F, "vpms", Op::E2(E2::Cvttps2dq))
        }
        I::X86Avx512MaskCvttps2udq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2udq.512", F, "vpms", Op::E2(E2::Cvttps2udq))
        }
        I::X86Avx512PermvarSi512 => {
            ("intrinsic:llvm.x86.avx512.permvar.si.512", F, "vv", Op::E3(E3::Permd, Form::Swap))
        }
        I::X86Avx512PsllD512 => {
            ("intrinsic:llvm.x86.avx512.psll.d.512", F, "vv", Op::E3(E3::Pslld, Form::Plain))
        }
        I::X86Avx512PslliD512 => (
            "intrinsic:llvm.x86.avx512.pslli.d.512",
            F,
            "vc",
            Op::Shift(EShift::Pslld, E3::Pslld, true),
        ),
        I::X86Avx512PsllvD512 => {
            ("intrinsic:llvm.x86.avx512.psllv.d.512", F, "vv", Op::E3(E3::Psllvd, Form::Plain))
        }
        I::X86Avx512PsraD512 => {
            ("intrinsic:llvm.x86.avx512.psra.d.512", F, "vv", Op::E3(E3::Psrad, Form::Plain))
        }
        I::X86Avx512PsraiD512 => (
            "intrinsic:llvm.x86.avx512.psrai.d.512",
            F,
            "vc",
            Op::Shift(EShift::Psrad, E3::Psrad, false),
        ),
        I::X86Avx512PsravD512 => {
            ("intrinsic:llvm.x86.avx512.psrav.d.512", F, "vv", Op::E3(E3::Psravd, Form::Plain))
        }
        I::X86Avx512PsrlD512 => {
            ("intrinsic:llvm.x86.avx512.psrl.d.512", F, "vv", Op::E3(E3::Psrld, Form::Plain))
        }
        I::X86Avx512PsrliD512 => (
            "intrinsic:llvm.x86.avx512.psrli.d.512",
            F,
            "vc",
            Op::Shift(EShift::Psrld, E3::Psrld, true),
        ),
        I::X86Avx512PsrlvD512 => {
            ("intrinsic:llvm.x86.avx512.psrlv.d.512", F, "vv", Op::E3(E3::Psrlvd, Form::Plain))
        }
        I::X86Avx512PternlogD512 => (
            "intrinsic:llvm.x86.avx512.pternlog.d.512",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogd, TERNLOG),
        ),
        I::X86Avx512Vpermi2varD512 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.d.512", F, "vvv", Op::E3(E3::Permt2d, PERMT2))
        }
        I::X86Avx512MaskPmovsQb128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qb.128", F, "vpm", Op::Store(EStore::Pmovsqb))
        }
        I::X86Avx512MaskPmovsQb256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qb.256", F, "vpm", Op::Store(EStore::Pmovsqb))
        }
        I::X86Avx512MaskPmovsQb512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qb.512", F, "vpm", Op::Store(EStore::Pmovsqb))
        }
        I::X86Avx512MaskPmovusDb128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.db.128", F, "vpm", Op::Store(EStore::Pmovusdb))
        }
        I::X86Avx512MaskPmovusDb256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.db.256", F, "vpm", Op::Store(EStore::Pmovusdb))
        }
        I::X86Avx512MaskPmovusDb512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.db.512", F, "vpm", Op::Store(EStore::Pmovusdb))
        }
        I::X86Avx512MaskPmovusQb128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qb.128", F, "vpm", Op::Store(EStore::Pmovusqb))
        }
        I::X86Avx512MaskPmovusQb256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qb.256", F, "vpm", Op::Store(EStore::Pmovusqb))
        }
        I::X86Avx512MaskPmovusQb512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qb.512", F, "vpm", Op::Store(EStore::Pmovusqb))
        }
        I::X86Avx512MaskGetexpPd128 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.pd.128", F, "vpm", Op::E2(E2::Getexppd))
        }
        I::X86Avx512Rcp14Pd128 => {
            ("intrinsic:llvm.x86.avx512.rcp14.pd.128", F, "vpm", Op::E2(E2::Rcp14pd))
        }
        I::X86Avx512Rcp14Sd => {
            ("intrinsic:llvm.x86.avx512.rcp14.sd", F, "vvpm", Op::E3(E3::Rcp14sd, Form::Plain))
        }
        I::X86Avx512Rsqrt14Pd128 => {
            ("intrinsic:llvm.x86.avx512.rsqrt14.pd.128", F, "vpm", Op::E2(E2::Rsqrt14pd))
        }
        I::X86Avx512Rsqrt14Sd => {
            ("intrinsic:llvm.x86.avx512.rsqrt14.sd", F, "vvpm", Op::E3(E3::Rsqrt14sd, Form::Plain))
        }
        I::X86Avx512Vpermi2varPd128 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.pd.128", F, "vvv", Op::E3(E3::Permt2pd, PERMT2))
        }
        I::X86Avx512PsraQ128 => {
            ("intrinsic:llvm.x86.avx512.psra.q.128", F, "vv", Op::E3(E3::Psraq, Form::Plain))
        }
        I::X86Avx512PsraiQ128 => (
            "intrinsic:llvm.x86.avx512.psrai.q.128",
            F,
            "vc",
            Op::Shift(EShift::Psraq, E3::Psraq, false),
        ),
        I::X86Avx512PsravQ128 => {
            ("intrinsic:llvm.x86.avx512.psrav.q.128", F, "vv", Op::E3(E3::Psravq, Form::Plain))
        }
        I::X86Avx512PternlogQ128 => (
            "intrinsic:llvm.x86.avx512.pternlog.q.128",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogq, TERNLOG),
        ),
        I::X86Avx512Vpermi2varQ128 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.q.128", F, "vvv", Op::E3(E3::Permt2q, PERMT2))
        }
        I::X86Avx512MaskGetexpPd256 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.pd.256", F, "vpm", Op::E2(E2::Getexppd))
        }
        I::X86Avx512PermvarDf256 => {
            ("intrinsic:llvm.x86.avx512.permvar.df.256", F, "vv", Op::E3(E3::Permpd, Form::Swap))
        }
        I::X86Avx512Rcp14Pd256 => {
            ("intrinsic:llvm.x86.avx512.rcp14.pd.256", F, "vpm", Op::E2(E2::Rcp14pd))
        }
        I::X86Avx512Rsqrt14Pd256 => {
            ("intrinsic:llvm.x86.avx512.rsqrt14.pd.256", F, "vpm", Op::E2(E2::Rsqrt14pd))
        }
        I::X86Avx512Vpermi2varPd256 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.pd.256", F, "vvv", Op::E3(E3::Permt2pd, PERMT2))
        }
        I::X86Avx512MaskGetexpPs128 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.ps.128", F, "vpm", Op::E2(E2::Getexpps))
        }
        I::X86Avx512Vpermi2varPs128 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.ps.128", F, "vvv", Op::E3(E3::Permt2ps, PERMT2))
        }
        I::X86Avx512MaskCvttpd2udq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2udq.128", F, "vpm", Op::E2(E2::Cvttpd2udq))
        }
        I::X86Avx512MaskCvttpd2udq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2udq.256", F, "vpm", Op::E2(E2::Cvttpd2udq))
        }
        I::X86Avx512MaskCvttps2udq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2udq.128", F, "vpm", Op::E2(E2::Cvttps2udq))
        }
        I::X86Avx512MaskPmovsQd128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qd.128", F, "vpm", Op::Store(EStore::Pmovsqd))
        }
        I::X86Avx512MaskPmovsQd256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qd.256", F, "vpm", Op::Store(EStore::Pmovsqd))
        }
        I::X86Avx512MaskPmovusQd128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qd.128", F, "vpm", Op::Store(EStore::Pmovusqd))
        }
        I::X86Avx512MaskPmovusQd256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qd.256", F, "vpm", Op::Store(EStore::Pmovusqd))
        }
        I::X86Avx512PternlogD128 => (
            "intrinsic:llvm.x86.avx512.pternlog.d.128",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogd, TERNLOG),
        ),
        I::X86Avx512Vpermi2varD128 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.d.128", F, "vvv", Op::E3(E3::Permt2d, PERMT2))
        }
        I::X86Avx512PermvarDi256 => {
            ("intrinsic:llvm.x86.avx512.permvar.di.256", F, "vv", Op::E3(E3::Permq, Form::Swap))
        }
        I::X86Avx512PsraQ256 => {
            ("intrinsic:llvm.x86.avx512.psra.q.256", F, "vv", Op::E3(E3::Psraq, Form::Plain))
        }
        I::X86Avx512PsraiQ256 => (
            "intrinsic:llvm.x86.avx512.psrai.q.256",
            F,
            "vc",
            Op::Shift(EShift::Psraq, E3::Psraq, false),
        ),
        I::X86Avx512PsravQ256 => {
            ("intrinsic:llvm.x86.avx512.psrav.q.256", F, "vv", Op::E3(E3::Psravq, Form::Plain))
        }
        I::X86Avx512PternlogQ256 => (
            "intrinsic:llvm.x86.avx512.pternlog.q.256",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogq, TERNLOG),
        ),
        I::X86Avx512Vpermi2varQ256 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.q.256", F, "vvv", Op::E3(E3::Permt2q, PERMT2))
        }
        I::X86Avx512MaskGetexpPd512 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.pd.512", F, "vpms", Op::E2(E2::Getexppd))
        }
        I::X86Avx512MaskRndscalePd512 => (
            "intrinsic:llvm.x86.avx512.mask.rndscale.pd.512",
            F,
            "vipms",
            Op::E2Imm(E2Imm::Rndscalepd),
        ),
        I::X86Avx512MaxPd512 => {
            ("intrinsic:llvm.x86.avx512.max.pd.512", F, "vvs", Op::E3(E3::Maxpd, Form::Plain))
        }
        I::X86Avx512MinPd512 => {
            ("intrinsic:llvm.x86.avx512.min.pd.512", F, "vvs", Op::E3(E3::Minpd, Form::Plain))
        }
        I::X86Avx512PermvarDf512 => {
            ("intrinsic:llvm.x86.avx512.permvar.df.512", F, "vv", Op::E3(E3::Permpd, Form::Swap))
        }
        I::X86Avx512Rcp14Pd512 => {
            ("intrinsic:llvm.x86.avx512.rcp14.pd.512", F, "vpm", Op::E2(E2::Rcp14pd))
        }
        I::X86Avx512Rsqrt14Pd512 => {
            ("intrinsic:llvm.x86.avx512.rsqrt14.pd.512", F, "vpm", Op::E2(E2::Rsqrt14pd))
        }
        I::X86Avx512VfmaddsubPd512 => (
            "intrinsic:llvm.x86.avx512.vfmaddsub.pd.512",
            F,
            "vvve",
            Op::E3(E3::Vfmaddsub213pd, FMA213),
        ),
        I::X86Avx512Vpermi2varPd512 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.pd.512", F, "vvv", Op::E3(E3::Permt2pd, PERMT2))
        }
        I::X86Avx512MaskCvtpd2ps512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtpd2ps.512", F, "vpme", Op::E2(E2::Cvtpd2ps))
        }
        I::X86Avx512MaskGetexpPs256 => {
            ("intrinsic:llvm.x86.avx512.mask.getexp.ps.256", F, "vpm", Op::E2(E2::Getexpps))
        }
        I::X86Avx512Vpermi2varPs256 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.ps.256", F, "vvv", Op::E3(E3::Permt2ps, PERMT2))
        }
        I::X86Avx512MaskPmovsQw128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qw.128", F, "vpm", Op::Store(EStore::Pmovsqw))
        }
        I::X86Avx512MaskPmovsQw256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qw.256", F, "vpm", Op::Store(EStore::Pmovsqw))
        }
        I::X86Avx512MaskPmovsQw512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qw.512", F, "vpm", Op::Store(EStore::Pmovsqw))
        }
        I::X86Avx512MaskPmovusQw128 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qw.128", F, "vpm", Op::Store(EStore::Pmovusqw))
        }
        I::X86Avx512MaskPmovusQw256 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qw.256", F, "vpm", Op::Store(EStore::Pmovusqw))
        }
        I::X86Avx512MaskPmovusQw512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qw.512", F, "vpm", Op::Store(EStore::Pmovusqw))
        }
        I::X86Avx512MaskCvtpd2dq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtpd2dq.512", F, "vpme", Op::E2(E2::Cvtpd2dq))
        }
        I::X86Avx512MaskCvttpd2dq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2dq.512", F, "vpms", Op::E2(E2::Cvttpd2dq))
        }
        I::X86Avx512MaskCvttpd2udq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2udq.512", F, "vpms", Op::E2(E2::Cvttpd2udq))
        }
        I::X86Avx512MaskCvttps2udq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2udq.256", F, "vpm", Op::E2(E2::Cvttps2udq))
        }
        I::X86Avx512MaskPmovsQd512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovs.qd.512", F, "vpm", Op::Store(EStore::Pmovsqd))
        }
        I::X86Avx512MaskPmovusQd512 => {
            ("intrinsic:llvm.x86.avx512.mask.pmovus.qd.512", F, "vpm", Op::Store(EStore::Pmovusqd))
        }
        I::X86Avx512PternlogD256 => (
            "intrinsic:llvm.x86.avx512.pternlog.d.256",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogd, TERNLOG),
        ),
        I::X86Avx512Vpermi2varD256 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.d.256", F, "vvv", Op::E3(E3::Permt2d, PERMT2))
        }
        I::X86Avx512PermvarDi512 => {
            ("intrinsic:llvm.x86.avx512.permvar.di.512", F, "vv", Op::E3(E3::Permq, Form::Swap))
        }
        I::X86Avx512PsllQ512 => {
            ("intrinsic:llvm.x86.avx512.psll.q.512", F, "vv", Op::E3(E3::Psllq, Form::Plain))
        }
        I::X86Avx512PslliQ512 => (
            "intrinsic:llvm.x86.avx512.pslli.q.512",
            F,
            "vc",
            Op::Shift(EShift::Psllq, E3::Psllq, true),
        ),
        I::X86Avx512PsllvQ512 => {
            ("intrinsic:llvm.x86.avx512.psllv.q.512", F, "vv", Op::E3(E3::Psllvq, Form::Plain))
        }
        I::X86Avx512PsraQ512 => {
            ("intrinsic:llvm.x86.avx512.psra.q.512", F, "vv", Op::E3(E3::Psraq, Form::Plain))
        }
        I::X86Avx512PsraiQ512 => (
            "intrinsic:llvm.x86.avx512.psrai.q.512",
            F,
            "vc",
            Op::Shift(EShift::Psraq, E3::Psraq, false),
        ),
        I::X86Avx512PsravQ512 => {
            ("intrinsic:llvm.x86.avx512.psrav.q.512", F, "vv", Op::E3(E3::Psravq, Form::Plain))
        }
        I::X86Avx512PsrlQ512 => {
            ("intrinsic:llvm.x86.avx512.psrl.q.512", F, "vv", Op::E3(E3::Psrlq, Form::Plain))
        }
        I::X86Avx512PsrliQ512 => (
            "intrinsic:llvm.x86.avx512.psrli.q.512",
            F,
            "vc",
            Op::Shift(EShift::Psrlq, E3::Psrlq, true),
        ),
        I::X86Avx512PsrlvQ512 => {
            ("intrinsic:llvm.x86.avx512.psrlv.q.512", F, "vv", Op::E3(E3::Psrlvq, Form::Plain))
        }
        I::X86Avx512PternlogQ512 => (
            "intrinsic:llvm.x86.avx512.pternlog.q.512",
            F,
            "vvvi",
            Op::E3Imm(E3Imm::Pternlogq, TERNLOG),
        ),
        I::X86Avx512Vpermi2varQ512 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.q.512", F, "vvv", Op::E3(E3::Permt2q, PERMT2))
        }
        I::X86Avx512Dbpsadbw256 => (
            "intrinsic:llvm.x86.avx512.dbpsadbw.256",
            Bw,
            "vvi",
            Op::E3Imm(E3Imm::Dbpsadbw, Form::Plain),
        ),
        I::X86Avx512PermvarHi256 => {
            ("intrinsic:llvm.x86.avx512.permvar.hi.256", Bw, "vv", Op::E3(E3::Permw, Form::Swap))
        }
        I::X86Avx512PsllvW256 => {
            ("intrinsic:llvm.x86.avx512.psllv.w.256", Bw, "vv", Op::E3(E3::Psllvw, Form::Plain))
        }
        I::X86Avx512PsravW256 => {
            ("intrinsic:llvm.x86.avx512.psrav.w.256", Bw, "vv", Op::E3(E3::Psravw, Form::Plain))
        }
        I::X86Avx512PsrlvW256 => {
            ("intrinsic:llvm.x86.avx512.psrlv.w.256", Bw, "vv", Op::E3(E3::Psrlvw, Form::Plain))
        }
        I::X86Avx512Vpermi2varHi256 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.hi.256", Bw, "vvv", Op::E3(E3::Permt2w, PERMT2))
        }
        I::X86Avx512Dbpsadbw512 => (
            "intrinsic:llvm.x86.avx512.dbpsadbw.512",
            Bw,
            "vvi",
            Op::E3Imm(E3Imm::Dbpsadbw, Form::Plain),
        ),
        I::X86Avx512Packssdw512 => {
            ("intrinsic:llvm.x86.avx512.packssdw.512", Bw, "vv", Op::E3(E3::Packssdw, Form::Plain))
        }
        I::X86Avx512Packusdw512 => {
            ("intrinsic:llvm.x86.avx512.packusdw.512", Bw, "vv", Op::E3(E3::Packusdw, Form::Plain))
        }
        I::X86Avx512PavgW512 => {
            ("intrinsic:llvm.x86.avx512.pavg.w.512", Bw, "vv", Op::E3(E3::Pavgw, Form::Plain))
        }
        I::X86Avx512PermvarHi512 => {
            ("intrinsic:llvm.x86.avx512.permvar.hi.512", Bw, "vv", Op::E3(E3::Permw, Form::Swap))
        }
        I::X86Avx512PmulHrSw512 => (
            "intrinsic:llvm.x86.avx512.pmul.hr.sw.512",
            Bw,
            "vv",
            Op::E3(E3::Pmulhrsw, Form::Plain),
        ),
        I::X86Avx512PmulhW512 => {
            ("intrinsic:llvm.x86.avx512.pmulh.w.512", Bw, "vv", Op::E3(E3::Pmulhw, Form::Plain))
        }
        I::X86Avx512PmulhuW512 => {
            ("intrinsic:llvm.x86.avx512.pmulhu.w.512", Bw, "vv", Op::E3(E3::Pmulhuw, Form::Plain))
        }
        I::X86Avx512PsllW512 => {
            ("intrinsic:llvm.x86.avx512.psll.w.512", Bw, "vv", Op::E3(E3::Psllw, Form::Plain))
        }
        I::X86Avx512PslliW512 => (
            "intrinsic:llvm.x86.avx512.pslli.w.512",
            Bw,
            "vc",
            Op::Shift(EShift::Psllw, E3::Psllw, true),
        ),
        I::X86Avx512PsllvW512 => {
            ("intrinsic:llvm.x86.avx512.psllv.w.512", Bw, "vv", Op::E3(E3::Psllvw, Form::Plain))
        }
        I::X86Avx512PsraW512 => {
            ("intrinsic:llvm.x86.avx512.psra.w.512", Bw, "vv", Op::E3(E3::Psraw, Form::Plain))
        }
        I::X86Avx512PsraiW512 => (
            "intrinsic:llvm.x86.avx512.psrai.w.512",
            Bw,
            "vc",
            Op::Shift(EShift::Psraw, E3::Psraw, false),
        ),
        I::X86Avx512PsravW512 => {
            ("intrinsic:llvm.x86.avx512.psrav.w.512", Bw, "vv", Op::E3(E3::Psravw, Form::Plain))
        }
        I::X86Avx512PsrlW512 => {
            ("intrinsic:llvm.x86.avx512.psrl.w.512", Bw, "vv", Op::E3(E3::Psrlw, Form::Plain))
        }
        I::X86Avx512PsrliW512 => (
            "intrinsic:llvm.x86.avx512.psrli.w.512",
            Bw,
            "vc",
            Op::Shift(EShift::Psrlw, E3::Psrlw, true),
        ),
        I::X86Avx512PsrlvW512 => {
            ("intrinsic:llvm.x86.avx512.psrlv.w.512", Bw, "vv", Op::E3(E3::Psrlvw, Form::Plain))
        }
        I::X86Avx512Vpermi2varHi512 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.hi.512", Bw, "vvv", Op::E3(E3::Permt2w, PERMT2))
        }
        I::X86Avx512Packsswb512 => {
            ("intrinsic:llvm.x86.avx512.packsswb.512", Bw, "vv", Op::E3(E3::Packsswb, Form::Plain))
        }
        I::X86Avx512Packuswb512 => {
            ("intrinsic:llvm.x86.avx512.packuswb.512", Bw, "vv", Op::E3(E3::Packuswb, Form::Plain))
        }
        I::X86Avx512PavgB512 => {
            ("intrinsic:llvm.x86.avx512.pavg.b.512", Bw, "vv", Op::E3(E3::Pavgb, Form::Plain))
        }
        I::X86Avx512PshufB512 => {
            ("intrinsic:llvm.x86.avx512.pshuf.b.512", Bw, "vv", Op::E3(E3::Pshufb, Form::Plain))
        }
        I::X86Avx512Dbpsadbw128 => (
            "intrinsic:llvm.x86.avx512.dbpsadbw.128",
            Bw,
            "vvi",
            Op::E3Imm(E3Imm::Dbpsadbw, Form::Plain),
        ),
        I::X86Avx512PermvarHi128 => {
            ("intrinsic:llvm.x86.avx512.permvar.hi.128", Bw, "vv", Op::E3(E3::Permw, Form::Swap))
        }
        I::X86Avx512PsllvW128 => {
            ("intrinsic:llvm.x86.avx512.psllv.w.128", Bw, "vv", Op::E3(E3::Psllvw, Form::Plain))
        }
        I::X86Avx512PsravW128 => {
            ("intrinsic:llvm.x86.avx512.psrav.w.128", Bw, "vv", Op::E3(E3::Psravw, Form::Plain))
        }
        I::X86Avx512PsrlvW128 => {
            ("intrinsic:llvm.x86.avx512.psrlv.w.128", Bw, "vv", Op::E3(E3::Psrlvw, Form::Plain))
        }
        I::X86Avx512Vpermi2varHi128 => {
            ("intrinsic:llvm.x86.avx512.vpermi2var.hi.128", Bw, "vvv", Op::E3(E3::Permt2w, PERMT2))
        }
        I::X86Avx512PsadBw512 => {
            ("intrinsic:llvm.x86.avx512.psad.bw.512", Bw, "vv", Op::E3(E3::Psadbw, Form::Plain))
        }
        I::X86Avx512FpclassPs512 => {
            ("intrinsic:llvm.x86.avx512.fpclass.ps.512", Dq, "vi", Op::Cmp(ECmp::Fpclassps))
        }
        I::X86Avx512FpclassPd128 => {
            ("intrinsic:llvm.x86.avx512.fpclass.pd.128", Dq, "vi", Op::Cmp(ECmp::Fpclasspd))
        }
        I::X86Avx512MaskCvtpd2qq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtpd2qq.128", Dq, "vpm", Op::E2(E2::Cvtpd2qq))
        }
        I::X86Avx512MaskCvttpd2qq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2qq.128", Dq, "vpm", Op::E2(E2::Cvttpd2qq))
        }
        I::X86Avx512MaskCvttpd2uqq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2uqq.128", Dq, "vpm", Op::E2(E2::Cvttpd2uqq))
        }
        I::X86Avx512MaskCvttps2qq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2qq.128", Dq, "vpm", Op::E2(E2::Cvttps2qq))
        }
        I::X86Avx512MaskCvttps2uqq128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2uqq.128", Dq, "vpm", Op::E2(E2::Cvttps2uqq))
        }
        I::X86Avx512MaskCvtqq2ps128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtqq2ps.128", Dq, "vpm", Op::E2(E2::Cvtqq2ps))
        }
        I::X86Avx512MaskCvtuqq2ps128 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtuqq2ps.128", Dq, "vpm", Op::E2(E2::Cvtuqq2ps))
        }
        I::X86Avx512FpclassPd256 => {
            ("intrinsic:llvm.x86.avx512.fpclass.pd.256", Dq, "vi", Op::Cmp(ECmp::Fpclasspd))
        }
        I::X86Avx512FpclassPs128 => {
            ("intrinsic:llvm.x86.avx512.fpclass.ps.128", Dq, "vi", Op::Cmp(ECmp::Fpclassps))
        }
        I::X86Avx512MaskCvtpd2qq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtpd2qq.256", Dq, "vpm", Op::E2(E2::Cvtpd2qq))
        }
        I::X86Avx512MaskCvttpd2qq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2qq.256", Dq, "vpm", Op::E2(E2::Cvttpd2qq))
        }
        I::X86Avx512MaskCvttpd2uqq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2uqq.256", Dq, "vpm", Op::E2(E2::Cvttpd2uqq))
        }
        I::X86Avx512MaskCvttps2qq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2qq.256", Dq, "vpm", Op::E2(E2::Cvttps2qq))
        }
        I::X86Avx512MaskCvttps2uqq256 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2uqq.256", Dq, "vpm", Op::E2(E2::Cvttps2uqq))
        }
        I::X86Avx512FpclassPd512 => {
            ("intrinsic:llvm.x86.avx512.fpclass.pd.512", Dq, "vi", Op::Cmp(ECmp::Fpclasspd))
        }
        I::X86Avx512FpclassPs256 => {
            ("intrinsic:llvm.x86.avx512.fpclass.ps.256", Dq, "vi", Op::Cmp(ECmp::Fpclassps))
        }
        I::X86Avx512MaskCvtpd2qq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvtpd2qq.512", Dq, "vpme", Op::E2(E2::Cvtpd2qq))
        }
        I::X86Avx512MaskCvttpd2qq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2qq.512", Dq, "vpms", Op::E2(E2::Cvttpd2qq))
        }
        I::X86Avx512MaskCvttpd2uqq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttpd2uqq.512", Dq, "vpms", Op::E2(E2::Cvttpd2uqq))
        }
        I::X86Avx512MaskCvttps2qq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2qq.512", Dq, "vpms", Op::E2(E2::Cvttps2qq))
        }
        I::X86Avx512MaskCvttps2uqq512 => {
            ("intrinsic:llvm.x86.avx512.mask.cvttps2uqq.512", Dq, "vpms", Op::E2(E2::Cvttps2uqq))
        }
        // VBMI (WP-220).
        I::X86Avx512PermvarQi128 => {
            ("intrinsic:llvm.x86.avx512.permvar.qi.128", Vbmi, "vv", Op::E3(E3::Permb, Form::Swap))
        }
        I::X86Avx512PermvarQi256 => {
            ("intrinsic:llvm.x86.avx512.permvar.qi.256", Vbmi, "vv", Op::E3(E3::Permb, Form::Swap))
        }
        I::X86Avx512PermvarQi512 => {
            ("intrinsic:llvm.x86.avx512.permvar.qi.512", Vbmi, "vv", Op::E3(E3::Permb, Form::Swap))
        }
        I::X86Avx512Vpermi2varQi128 => (
            "intrinsic:llvm.x86.avx512.vpermi2var.qi.128",
            Vbmi,
            "vvv",
            Op::E3(E3::Permt2b, PERMT2),
        ),
        I::X86Avx512Vpermi2varQi256 => (
            "intrinsic:llvm.x86.avx512.vpermi2var.qi.256",
            Vbmi,
            "vvv",
            Op::E3(E3::Permt2b, PERMT2),
        ),
        I::X86Avx512Vpermi2varQi512 => (
            "intrinsic:llvm.x86.avx512.vpermi2var.qi.512",
            Vbmi,
            "vvv",
            Op::E3(E3::Permt2b, PERMT2),
        ),
        I::X86Avx512PmultishiftQb128 => (
            "intrinsic:llvm.x86.avx512.pmultishift.qb.128",
            Vbmi,
            "vv",
            Op::E3(E3::Pmultishiftqb, Form::Plain),
        ),
        I::X86Avx512PmultishiftQb256 => (
            "intrinsic:llvm.x86.avx512.pmultishift.qb.256",
            Vbmi,
            "vv",
            Op::E3(E3::Pmultishiftqb, Form::Plain),
        ),
        I::X86Avx512PmultishiftQb512 => (
            "intrinsic:llvm.x86.avx512.pmultishift.qb.512",
            Vbmi,
            "vv",
            Op::E3(E3::Pmultishiftqb, Form::Plain),
        ),
        // GFNI, VAES and VPCLMULQDQ at 512 bits (WP-220).
        I::X86Vgf2p8affineqb512 => (
            "intrinsic:llvm.x86.vgf2p8affineqb.512",
            Gfni,
            "vvi",
            Op::E3Imm(E3Imm::Gf2p8affineqb, Form::Plain),
        ),
        I::X86Vgf2p8mulb512 => {
            ("intrinsic:llvm.x86.vgf2p8mulb.512", Gfni, "vv", Op::E3(E3::Gf2p8mulb, Form::Plain))
        }
        I::X86AesniAesenc512 => {
            ("intrinsic:llvm.x86.aesni.aesenc.512", Vaes, "vv", Op::E3(E3::Aesenc, Form::Plain))
        }
        I::X86AesniAesenclast512 => (
            "intrinsic:llvm.x86.aesni.aesenclast.512",
            Vaes,
            "vv",
            Op::E3(E3::Aesenclast, Form::Plain),
        ),
        I::X86AesniAesdec512 => {
            ("intrinsic:llvm.x86.aesni.aesdec.512", Vaes, "vv", Op::E3(E3::Aesdec, Form::Plain))
        }
        I::X86AesniAesdeclast512 => (
            "intrinsic:llvm.x86.aesni.aesdeclast.512",
            Vaes,
            "vv",
            Op::E3(E3::Aesdeclast, Form::Plain),
        ),
        I::X86Pclmulqdq512 => (
            "intrinsic:llvm.x86.pclmulqdq.512",
            Vpclmulqdq,
            "vvi",
            Op::E3Imm(E3Imm::Pclmulqdq, Form::Plain),
        ),
        // BF16 (WP-220). `vcvtneps2bf16` at 128 and 256 bits is also
        // AVX-NE-CONVERT's (VEX): that feature alone is not taken.
        I::X86Avx512bf16Cvtne2ps2bf16128 => (
            "intrinsic:llvm.x86.avx512bf16.cvtne2ps2bf16.128",
            Bf16,
            "vv",
            Op::E3(E3::Cvtne2ps2bf16, Form::Plain),
        ),
        I::X86Avx512bf16Cvtne2ps2bf16256 => (
            "intrinsic:llvm.x86.avx512bf16.cvtne2ps2bf16.256",
            Bf16,
            "vv",
            Op::E3(E3::Cvtne2ps2bf16, Form::Plain),
        ),
        I::X86Avx512bf16Cvtne2ps2bf16512 => (
            "intrinsic:llvm.x86.avx512bf16.cvtne2ps2bf16.512",
            Bf16,
            "vv",
            Op::E3(E3::Cvtne2ps2bf16, Form::Plain),
        ),
        I::X86Avx512bf16Cvtneps2bf16512 => {
            ("intrinsic:llvm.x86.avx512bf16.cvtneps2bf16.512", Bf16, "v", Op::E2(E2::Cvtneps2bf16))
        }
        I::X86Vcvtneps2bf16128 => {
            ("intrinsic:llvm.x86.vcvtneps2bf16128", Bf16, "v", Op::E2(E2::Cvtneps2bf16))
        }
        I::X86Vcvtneps2bf16256 => {
            ("intrinsic:llvm.x86.vcvtneps2bf16256", Bf16, "v", Op::E2(E2::Cvtneps2bf16))
        }
        _ => return None,
    })
}

/// The stable name `which` is refused with in a function whose features
/// do not enable it, if it is one of this module's.
pub fn refusal(which: Intrinsic<'_>) -> Option<&'static str> {
    row(which).map(|row| row.0)
}

/// A call of `which` with `args`, if `which` is an intrinsic of this
/// module.
pub fn intrinsic<'a>(
    cx: &mut FnCx<'a, X64>,
    inst: Inst<'a>,
    which: Intrinsic<'a>,
    args: &'a [Value],
) -> Option<CgResult<()>> {
    let row = row(which)?;
    Some(lower(cx, inst, args, row))
}

const fn zmm(n: u8) -> Zmm {
    match Zmm::new(n) {
        Some(z) => z,
        None => panic!("a register above zmm31"),
    }
}

/// The destination.
const ZMM16: Zmm = zmm(16);
/// The mask register.
const K1: mc::K = mc::K::K1;

/// A vector type's lanes, element bits and bytes; `None` for anything
/// else, a vector of `i1` included.
fn vector(cx: &FnCx<'_, X64>, ty: TypeId) -> Option<(u32, u32, u32)> {
    let types = cx.types();
    let TypeKind::Vector { elem, len } = types.kind(ty) else { return None };
    let bits = match types.kind(elem) {
        TypeKind::Int(bits @ (8 | 16 | 32 | 64)) => bits,
        TypeKind::Float => 32,
        TypeKind::Double => 64,
        // The BF16 and FP16 rows' (WP-220).
        TypeKind::Half | TypeKind::BFloat => 16,
        _ => return None,
    };
    let bytes = bits * len / 8;
    matches!(bytes, 16 | 32 | 64).then_some((len, bits, bytes))
}

/// The length of a vector of `bytes`.
fn vl_of(bytes: u32) -> Vl {
    match bytes {
        64 => Vl::Z,
        32 => Vl::Y,
        _ => Vl::X,
    }
}

/// The function's `target-features`, empty without them.
fn features<'a>(cx: &FnCx<'a, X64>) -> &'a [u8] {
    let SymbolKind::Function { attrs, .. } = cx.module.symbol(cx.body.symbol()).kind() else {
        return &[];
    };
    cx.module.fn_attrs(attrs).string(FnStr::TargetFeatures).unwrap_or(&[])
}

/// Whether `value` is `zeroinitializer`, `undef` or `poison`.
fn zero_like(cx: &FnCx<'_, X64>, value: Value) -> bool {
    let Some(id) = value.as_const() else { return false };
    matches!(
        cx.module.consts.get(id),
        hod_ir::Const::Zero(_) | hod_ir::Const::Undef(_) | hod_ir::Const::Poison(_)
    )
}

/// The rounding of operand `value` for role `role` (`e` or `s`): `None`
/// for 4 (`_MM_FROUND_CUR_DIRECTION`), the refusal for a value `llc`
/// does not select.
fn rounding(cx: &FnCx<'_, X64>, value: Value, role: u8) -> CgResult<Option<Round>> {
    Ok(match (role, const_int(cx, value).map(|v| v & 0xffff_ffff)) {
        (_, Some(4)) => None,
        (b's', Some(8 | 12)) => Some(Round::Sae),
        (b'e', Some(8)) => Some(Round::Rn),
        (b'e', Some(9)) => Some(Round::Rd),
        (b'e', Some(10)) => Some(Round::Ru),
        (b'e', Some(11)) => Some(Round::Rz),
        _ => return unsupported(SIGNATURE),
    })
}

/// Emits `inst`, which the row's checks made encodable.
fn emit(cx: &mut FnCx<'_, X64>, inst: Option<mc::Inst>) -> CgResult<()> {
    match inst {
        Some(inst) => {
            put(cx, inst);
            Ok(())
        }
        None => cx.bug("an AVX-512 row without an encoding"),
    }
}

/// Copies vector `value` into `z`, pinning nothing: from its register
/// (given back at once) or its frame slot.
fn load(cx: &mut FnCx<'_, X64>, value: Value, z: Zmm, bytes: u32) -> CgResult<()> {
    if cx.class_of(value) == Class::Fpr {
        let r = cx.use_reg(value)?;
        put(cx, mc::emov_rr(EMov::Movdqu64, Vl::X, Masking::NONE, z, Zmm::of(xm(r))));
        cx.release(value);
        cx.free_temp(r);
    } else {
        let slot = cx.use_mem(value)?;
        let vl = vl_of(bytes);
        frame_inst(cx, slot, |m| mc::emov_rm(EMov::Movdqu64, vl, Masking::NONE, z, m));
    }
    Ok(())
}

#[allow(clippy::too_many_lines, reason = "one arm per kind of row")]
fn lower<'a>(
    cx: &mut FnCx<'a, X64>,
    inst: Inst<'a>,
    args: &'a [Value],
    (refused, fam, roles, op): Row,
) -> CgResult<()> {
    let roles = roles.as_bytes();
    let role = |r: u8| roles.iter().position(|&x| x == r);
    if args.len() != roles.len() {
        return unsupported(SIGNATURE);
    }
    // The shapes: every vector a vector of 16, 32 or 64 bytes in the
    // class of its size, the pass-through of the result's type, the mask
    // and the immediates integers.
    let ret_ty = inst.ty();
    let ret = vector(cx, ret_ty);
    let mut vectors: Vec<(Value, u32)> = Vec::new();
    let mut first = None;
    for (&r, &a) in roles.iter().zip(args) {
        let ty = value_ty(cx, a);
        let ok = match r {
            b'v' | b'p' => match vector(cx, ty) {
                Some((lanes, bits, bytes)) => {
                    let class = if bytes == 16 { Class::Fpr } else { Class::Mem };
                    if r == b'v' {
                        first.get_or_insert((lanes, bits, bytes));
                        vectors.push((a, bytes));
                    }
                    cx.class_of(a) == class && (r == b'v' || ty == ret_ty)
                }
                None => false,
            },
            b'm' => {
                cx.class_of(a) == Class::Gpr
                    && match cx.types().kind(ty) {
                        TypeKind::Int(bits) => matches!(bits, 8 | 16 | 32 | 64),
                        _ => crate::mask::mask_lanes(cx.types(), ty).is_some(),
                    }
            }
            // The immediates of `vgf2p8affineqb` and `vpclmulqdq` are `i8`.
            b'i' if matches!(op, Op::E3Imm(E3Imm::Gf2p8affineqb | E3Imm::Pclmulqdq, _)) => {
                matches!(cx.types().kind(ty), TypeKind::Int(8))
            }
            _ => matches!(cx.types().kind(ty), TypeKind::Int(32)),
        };
        if !ok {
            return unsupported(SIGNATURE);
        }
    }
    let Some((lanes, bits, src_bytes)) = first else { return unsupported(SIGNATURE) };
    // A compress of `half` or `bfloat` lanes is not one the census or the
    // tests have: not taken.
    if op == Op::Compress
        && matches!(
            cx.types().kind(ret_ty),
            TypeKind::Vector { elem, .. }
                if matches!(cx.types().kind(elem), TypeKind::Half | TypeKind::BFloat)
        )
    {
        return unsupported(SIGNATURE);
    }
    let ret_bytes = match (op, ret) {
        (Op::Cmp(_), None) if crate::mask::mask_lanes(cx.types(), ret_ty) == Some(lanes) => 0,
        (Op::Cmp(_), _) | (_, None) => return unsupported(SIGNATURE),
        (_, Some((_, _, bytes))) => bytes,
    };
    for &r in b"ies" {
        if let Some(j) = role(r)
            && const_int(cx, args[j]).is_none()
        {
            return unsupported(SIGNATURE);
        }
    }
    // The family: the compresses of bytes and words are VBMI2's.
    let scalar = matches!(op, Op::E3(E3::Rcp14sd | E3::Rsqrt14sd, _));
    let fam = match op {
        Op::Compress if bits <= 16 => Avx512::Vbmi2,
        _ => fam,
    };
    let bytes = src_bytes.max(ret_bytes);
    // A name with a length is of vectors of that length.
    let named = [(".512", 64), (".256", 32), (".128", 16)]
        .into_iter()
        .find_map(|(suffix, n)| refused.ends_with(suffix).then_some(n));
    if named.is_some_and(|n| n != bytes) {
        return unsupported(SIGNATURE);
    }
    let vl = vl_of(bytes);
    let features = features(cx);
    if !fam.enabled(features)
        || (bytes < 64 && !scalar && !Avx512::Vl.enabled(features))
        || (bytes == 64 && evex512_off(features))
    {
        return unsupported(refused);
    }
    let round = match role(b'e').or_else(|| role(b's')) {
        Some(j) => rounding(cx, args[j], roles[j])?,
        None => None,
    };
    deferred_first(cx, inst, None)?;

    // The mask: the lanes the instruction reads.
    let mask_lanes = match ret {
        _ if scalar => 1,
        Some((n, _, _)) => n.min(lanes),
        None => lanes,
    };
    let mut masking = Masking::NONE;
    if let Some(j) = role(b'm') {
        let m = args[j];
        let ty = value_ty(cx, m);
        let width = match cx.types().kind(ty) {
            TypeKind::Int(bits) => bits,
            _ => crate::mask::mask_lanes(cx.types(), ty).unwrap_or(64),
        };
        let all = if mask_lanes >= 64 { u64::MAX } else { (1u64 << mask_lanes) - 1 };
        let constant = match m.kind() {
            ValueKind::Const(id) if crate::mask::mask_lanes(cx.types(), ty).is_some() => {
                crate::mask::const_bits(cx, id, width)
            }
            ValueKind::Const(_) => const_int(cx, m),
            _ => None,
        };
        let size = match width {
            ..=16 => Size::W,
            17..=32 => Size::D,
            _ => Size::Q,
        };
        match constant {
            Some(c) if c & all == all => {}
            Some(c) => {
                put(cx, mc::movabs_ri(R11, c as i64));
                put(cx, mc::kmov_kr(size, K1, R11));
                masking = Masking::merge(K1);
            }
            None => {
                let r = cx.use_reg(m)?;
                put(cx, mc::kmov_kr(size, K1, gp(r)));
                cx.release(m);
                cx.free_temp(r);
                masking = Masking::merge(K1);
            }
        }
    }
    let pass = role(b'p').map(|j| args[j]);
    if masking != Masking::NONE && pass.is_some_and(|p| zero_like(cx, p)) {
        masking = Masking::zero(K1);
    }

    // The vectors, each value once, from `zmm17` up.
    let mut regs: Vec<(Value, Zmm)> = Vec::new();
    let mut at = |cx: &mut FnCx<'a, X64>, value: Value, bytes: u32| -> CgResult<Zmm> {
        if let Some(&(_, z)) = regs.iter().find(|(v, _)| *v == value) {
            return Ok(z);
        }
        let z = zmm(17 + regs.len() as u8);
        load(cx, value, z, bytes)?;
        regs.push((value, z));
        Ok(z)
    };
    let mut v = Vec::with_capacity(vectors.len());
    for &(value, bytes) in &vectors {
        v.push(at(cx, value, bytes)?);
    }
    if masking == Masking::merge(K1)
        && let Some(p) = pass
    {
        let z = at(cx, p, ret_bytes)?;
        put(cx, mc::emov_rr(EMov::Movdqu64, vl, Masking::NONE, ZMM16, z));
    }
    let imm = match role(b'i') {
        Some(j) => const_int(cx, args[j]).map_or(0, |v| (v & 0xff) as u8),
        None => 0,
    };

    let mut out = ZMM16;
    match op {
        Op::E3(e3, form) => {
            let (a, b) = operands(cx, &v, form);
            emit(
                cx,
                match round {
                    Some(round) => mc::try_e3_rrr_round(e3, masking, ZMM16, a, b, round),
                    None => mc::try_e3_rrr(e3, vl, masking, ZMM16, a, b),
                },
            )?;
        }
        Op::E3Imm(e3, form) => {
            let (a, b) = operands(cx, &v, form);
            emit(cx, mc::try_e3_rrri(e3, vl, masking, ZMM16, a, b, imm))?;
        }
        Op::E2(e2) => emit(
            cx,
            match round {
                Some(round) => mc::try_e2_rr_round(e2, masking, ZMM16, v[0], round),
                None => mc::try_e2_rr(e2, vl, masking, ZMM16, v[0]),
            },
        )?,
        Op::E2Imm(e2) => emit(
            cx,
            match round {
                Some(round) => mc::try_e2_rri_round(e2, masking, ZMM16, v[0], round, imm),
                None => mc::try_e2_rri(e2, vl, masking, ZMM16, v[0], imm),
            },
        )?,
        Op::Shift(by_imm, by_reg, logical) => {
            let Some(j) = role(b'c') else { return cx.bug("a shift row without a count") };
            let count = args[j];
            match const_int(cx, count).map(|c| c & 0xffff_ffff) {
                Some(0) => out = v[0],
                Some(c) if c >= u64::from(bits) && logical => return zero(cx, inst, ret_bytes),
                Some(c) => {
                    let c = c.min(u64::from(bits) - 1) as u8;
                    emit(cx, mc::try_eshift_rri(by_imm, vl, Masking::NONE, ZMM16, v[0], c))?;
                }
                None => {
                    // `movd` zero-extends the count into the 64 bits the
                    // shift reads, as `llc`'s.
                    let n = cx.use_reg(count)?;
                    put(cx, mc::movd_xr(Size::D, XMM15, gp(n)));
                    cx.release(count);
                    cx.free_temp(n);
                    let x15 = Zmm::of(XMM15);
                    emit(cx, mc::try_e3_rrr(by_reg, vl, Masking::NONE, ZMM16, v[0], x15))?;
                }
            }
        }
        Op::Store(store) => emit(cx, mc::try_estore_rr(store, vl, masking, ZMM16, v[0]))?,
        // Every lane set: the source.
        Op::Compress if masking == Masking::NONE => out = v[0],
        Op::Compress => {
            let float = matches!(
                cx.types().kind(ret_ty),
                TypeKind::Vector { elem, .. }
                    if matches!(cx.types().kind(elem), TypeKind::Float | TypeKind::Double)
            );
            let store = match (bits, float) {
                (32, true) => EStore::Compressps,
                (64, true) => EStore::Compresspd,
                (32, false) => EStore::Pcompressd,
                (64, false) => EStore::Pcompressq,
                (16, _) => EStore::Pcompressw,
                _ => EStore::Pcompressb,
            };
            emit(cx, mc::try_estore_rr(store, vl, masking, ZMM16, v[0]))?;
        }
        Op::Ph => emit(cx, mc::try_evcvtps2ph_rri(vl, masking, ZMM16, v[0], imm))?,
        Op::Cmp(cmp) => {
            put(cx, mc::ecmp_kri(cmp, vl, mc::K::K0, mc::K::K0, v[0], imm));
            let rd = cx.def_reg(inst)?;
            put(cx, mc::kmov_rk(Size::W, gp(rd), mc::K::K0));
            return cx.finish(inst, rd);
        }
    }
    store(cx, inst, out, ret_bytes)
}

/// The register operands `a` (`vvvv`) and `b` of a row of `form`; a tied
/// destination is copied into `zmm16`.
fn operands(cx: &mut FnCx<'_, X64>, v: &[Zmm], form: Form) -> (Zmm, Zmm) {
    match form {
        Form::Plain => (v[0], v[1]),
        Form::Swap => (v[1], v[0]),
        Form::Tied([d, a, b]) => {
            put(cx, mc::emov_rr(EMov::Movdqu64, Vl::Z, Masking::NONE, ZMM16, v[d]));
            (v[a], v[b])
        }
    }
}

/// The result in `out` to its place: its `xmm`, or its slot.
fn store<'a>(cx: &mut FnCx<'a, X64>, inst: Inst<'a>, out: Zmm, bytes: u32) -> CgResult<()> {
    if bytes == 16 {
        let rd = cx.def_reg(inst)?;
        put(cx, mc::emov_rr(EMov::Movdqu64, Vl::X, Masking::NONE, Zmm::of(xm(rd)), out));
        return cx.finish(inst, rd);
    }
    let slot = cx.def_mem(inst)?;
    let vl = vl_of(bytes);
    frame_inst(cx, slot, |m| mc::emov_mr(EMov::Movdqu64, vl, mc::K::K0, m, out));
    Ok(())
}

/// A result of zeros (a logical shift by the element width or more, as
/// `llc` folds it).
fn zero<'a>(cx: &mut FnCx<'a, X64>, inst: Inst<'a>, bytes: u32) -> CgResult<()> {
    if bytes == 16 {
        let rd = cx.def_reg(inst)?;
        let x: Xmm = xm(rd);
        put(cx, mc::pxor_rr(x, x));
        return cx.finish(inst, rd);
    }
    let slot = cx.def_mem(inst)?;
    put(cx, mc::xor_rr(Size::D, R11, R11));
    for off in (0..bytes).step_by(8) {
        frame_inst(cx, slot.offset_by(off), |m| mc::mov_mr(Size::Q, m, R11));
    }
    Ok(())
}

//! Vectors across calls (`spec/codegen.md` §5.6.13 "Vectors (WP-132)",
//! `spec/abi-x64.md` §4.3): the moves of the parts of kind
//! `PartKind::Lanes` (`Target::lanes_out`, `Target::lanes_in`), which
//! `abi::assign` makes for the masks LLVM passes promoted to the lanes of
//! one `xmm`.
//!
//! **Owner: WP-132.** Tests: `tests/vcall.rs`.
//!
//! # The parts
//!
//! A mask `<N x i1>` of 2, 4, 8 or 16 elements travels as the lanes of
//! one `xmm` (`v2i64`, `v4i32`, `v8i16`, `v16i8`: lanes of `16 / N`
//! bytes, element `i` in lane `i`), or that register's 16-byte image in
//! a stack slot. Its value is a general register whose bit `i` is
//! element `i` (`mask.rs`), or the image of an aggregate's member in
//! memory (element 0 in bit 0 of byte 0, the bits after the last element
//! zero). Each lane hod produces is 0 or 1; hod reads only bit 0 of a
//! lane it consumes (`llc`'s producers write all ones, its consumers
//! shift bit 0 up or mask it).
//!
//! The lanes are built and read in a 16-byte frame slot: in the stack
//! slot of a part on the stack, else in a slot of the function's own,
//! loaded into or stored from the register (`movups`).
//!
//! WP-208, with AVX-512 (`spec/abi-x64.md` §4.3): 32 elements are the
//! byte lanes of one `ymm` and 64 those of one `zmm` or of two `ymm`
//! (the second's `first` element 32), built and read in a slot of the
//! register's size (`vector::load_wide`, `vector::store_wide`); a mask
//! LLVM passes byte by byte has one part per element, of one byte, its
//! place a general register or an 8-byte stack slot, which holds 0 or 1.
//! The pieces of one mask are all at the mask's offset, so a side in
//! memory is always the mask's image; a part of some of the elements
//! updates their bits in it.
//!
//! WP-174: a lone `<1 x i64>` or `<1 x ptr>` argument or result is one
//! `Lanes { bits: 64, len: 1 }` part (`abi::LONE_LANE`): its value is an
//! `xmm`, its place a general register or an 8-byte stack slot; it moves
//! with one `movq` (or an 8-byte load or store), `lone_out`/`lone_in`.
//!
//! # Registers
//!
//! Every move writes `r10` or `r11` (the one that is not the general
//! register holding the mask's bits, which is left unchanged as a
//! source), the flags and its destination only.

use hod_cg::abi::{LaneLoc, Part, PartKind};
use hod_cg::error::CALL_PART_LANES;
use hod_cg::{CgError, CgResult, FnCx, FrameRef, PReg};
use hod_mc_x64 as mc;
use hod_mc_x64::{Reg, Size};

use crate::common::{
    R10, gp, load_fpr, load_gpr, mov_imm, other_scratch, put, store_fpr, store_gpr, xm,
};
use crate::target::X64;
use crate::{abi, reg};

/// A part of kind `Lanes`, decoded.
#[derive(Clone, Copy, Debug)]
struct Lanes {
    /// Bytes of a lane: 1, 2, 4 or 8.
    lane: u32,
    /// The element of the first lane.
    first: u32,
    /// Elements of the vector.
    len: u32,
    /// Lanes the part carries.
    count: u32,
    /// The bytes of the place: 16 (`xmm`), 32 (`ymm`) or 64 (`zmm`); 1
    /// for one element in a general register or an 8-byte stack slot.
    size: u32,
}

impl Lanes {
    fn of(cx: &FnCx<'_, X64>, part: &Part) -> CgResult<Lanes> {
        let PartKind::Lanes { bits, lane, first, len } = part.kind else {
            return cx.bug("a lane move of a part that is no Lanes part");
        };
        let (bits, lane, first, len) =
            (u32::from(bits), u32::from(lane), u32::from(first), u32::from(len));
        let count = part.size / lane.max(1);
        let ok = bits == 1
            && matches!(lane, 1 | 2 | 4 | 8)
            && (matches!(part.size, 16 | 32 | 64) || (part.size == 1 && lane == 1))
            && count * lane == part.size
            && first + count <= len
            && len <= 64;
        if !ok {
            return cx.bug("a Lanes part whose shape abi::assign does not make");
        }
        Ok(Lanes { lane, first, len, count, size: part.size })
    }

    /// The bytes of the mask's image: `⌈len/8⌉`.
    fn image_bytes(self) -> u32 {
        self.len.div_ceil(8)
    }

    /// WP-208: whether the part is one element in a general register or
    /// an 8-byte stack slot (a mask LLVM passes byte by byte, §4.3).
    fn in_gpr(self) -> bool {
        self.size == 1
    }
}

/// A slot of the part's place: `to`'s own when it is a frame place, else
/// a new one.
fn lanes_slot(cx: &mut FnCx<'_, X64>, l: Lanes, at: LaneLoc) -> CgResult<FrameRef> {
    match at {
        LaneLoc::Frame(slot) => Ok(slot),
        LaneLoc::Reg(_) => cx.alloc_slot(u64::from(l.size), 16),
    }
}

/// `xmm<x>`, `ymm<x>` or `zmm<x>` loaded with the part's `size` bytes at
/// `slot`.
fn load_lanes(cx: &mut FnCx<'_, X64>, x: PReg, slot: FrameRef, l: Lanes) {
    if l.size == 16 {
        load_fpr(cx, xm(x), slot, 16);
    } else {
        crate::vector::load_wide(cx, xm(x), slot, l.size);
    }
}

/// The part's `size` bytes of `xmm<x>`, `ymm<x>` or `zmm<x>` stored to
/// `slot`.
fn store_lanes(cx: &mut FnCx<'_, X64>, x: PReg, slot: FrameRef, l: Lanes) {
    if l.size == 16 {
        store_fpr(cx, xm(x), slot, 16);
    } else {
        crate::vector::store_wide(cx, xm(x), slot, l.size);
    }
}

/// WP-167: whether the two sides of a lane move are registers of the
/// classes this module moves: the value (a mask's bits) a general
/// register, the place an `xmm` (WP-208: a general register for a part
/// of one element); a frame place on either side. `hod_cg` hands the
/// value in a scratch register of the value's class, and only masks
/// (class `Gpr`, `abi.rs` `mask_leaf`) have `Lanes` parts here, so
/// anything else (an `xmm` or x87 value) is refused, never treated as a
/// general register.
fn classes_ok(value: LaneLoc, place: LaneLoc, in_gpr: bool) -> bool {
    let value_ok = match value {
        LaneLoc::Reg(r) => reg::is_gpr(r),
        LaneLoc::Frame(_) => true,
    };
    let place_ok = match place {
        LaneLoc::Reg(r) if in_gpr => reg::is_gpr(r),
        LaneLoc::Reg(r) => reg::is_fpr(r),
        LaneLoc::Frame(_) => true,
    };
    value_ok && place_ok
}

/// The class of a lone part's place, before decoding the part: one
/// element of a byte in a general register.
fn part_in_gpr(part: &Part) -> bool {
    matches!(part.kind, PartKind::Lanes { lane: 1, .. }) && part.size == 1
}

/// `Target::lanes_out`: the producer's side of a `Lanes` part.
///
/// # Errors
/// `call:part-lanes` when a side is a register of a class this module
/// does not move ([`classes_ok`]); `Internal` for a part `abi::assign`
/// does not make; `limit:frame-size`.
pub fn lanes_out(cx: &mut FnCx<'_, X64>, part: &Part, from: LaneLoc, to: LaneLoc) -> CgResult<()> {
    if is_lone(part) {
        return lone_out(cx, from, to);
    }
    if !classes_ok(from, to, part_in_gpr(part)) {
        return Err(CgError::unsupported(CALL_PART_LANES));
    }
    let l = Lanes::of(cx, part)?;
    // The mask's bits in a general register.
    let g = match from {
        LaneLoc::Reg(g) => gp(g),
        LaneLoc::Frame(image) => {
            load_gpr(cx, R10, image, l.image_bytes());
            R10
        }
    };
    let t = other_scratch(g);
    if l.in_gpr() {
        // WP-208: element `first` as the byte 0 or 1, in the register
        // itself or stored to the stack slot.
        let dst = match to {
            LaneLoc::Reg(r) => gp(r),
            LaneLoc::Frame(_) => t,
        };
        bit_of(cx, dst, g, l.first);
        if let LaneLoc::Frame(slot) = to {
            store_gpr(cx, t, slot, 1);
        }
        return Ok(());
    }
    let slot = lanes_slot(cx, l, to)?;
    for i in 0..l.count {
        bit_of(cx, t, g, l.first + i);
        store_gpr(cx, t, slot.offset_by(i * l.lane), l.lane);
    }
    if let LaneLoc::Reg(x) = to {
        load_lanes(cx, x, slot, l);
    }
    Ok(())
}

/// `d` set to bit `bit` of `g` (0 or 1).
fn bit_of(cx: &mut FnCx<'_, X64>, d: Reg, g: Reg, bit: u32) {
    put(cx, mc::mov_rr(Size::Q, d, g));
    if bit != 0 {
        put(cx, mc::shr_ri(Size::Q, d, bit as u8));
    }
    put(cx, mc::and_ri(Size::D, d, 1));
}

/// `Target::lanes_in`: the consumer's side of a `Lanes` part.
///
/// # Errors
/// As [`lanes_out`].
pub fn lanes_in(cx: &mut FnCx<'_, X64>, part: &Part, from: LaneLoc, to: LaneLoc) -> CgResult<()> {
    if is_lone(part) {
        return lone_in(cx, from, to);
    }
    if !classes_ok(to, from, part_in_gpr(part)) {
        return Err(CgError::unsupported(CALL_PART_LANES));
    }
    let l = Lanes::of(cx, part)?;
    let src = match from {
        LaneLoc::Frame(slot) => Src::Frame(slot),
        LaneLoc::Reg(r) if l.in_gpr() => Src::Reg(gp(r)),
        LaneLoc::Reg(x) => {
            let slot = cx.alloc_slot(u64::from(l.size), 16)?;
            store_lanes(cx, x, slot, l);
            Src::Frame(slot)
        }
    };
    match to {
        LaneLoc::Reg(g) => {
            gather(cx, l, src, gp(g));
            Ok(())
        }
        LaneLoc::Frame(image) => {
            // WP-208: a part of some of the elements updates their bits
            // in the image (the parts come in order, the first clears it).
            if l.first != 0 {
                load_gpr(cx, R10, image, l.image_bytes());
            }
            gather(cx, l, src, R10);
            store_gpr(cx, R10, image, l.image_bytes());
            Ok(())
        }
    }
}

/// WP-174: whether `part` is the one part of a lone `<1 x i64>` or
/// `<1 x ptr>` ([`abi::LONE_LANE`]), 8 bytes.
fn is_lone(part: &Part) -> bool {
    part.kind == abi::LONE_LANE && part.size == 8
}

/// WP-174: whether the sides of a lone `<1 x i64>`/`<1 x ptr>` move are
/// of the classes it moves: the value an `xmm` (its low 8 bytes), the
/// place a general register; a frame place on either side.
fn lone_classes_ok(value: LaneLoc, place: LaneLoc) -> bool {
    let value_ok = match value {
        LaneLoc::Reg(r) => reg::is_fpr(r),
        LaneLoc::Frame(_) => true,
    };
    let place_ok = match place {
        LaneLoc::Reg(r) => reg::is_gpr(r),
        LaneLoc::Frame(_) => true,
    };
    value_ok && place_ok
}

/// WP-174: the producer's side of a lone `<1 x i64>`/`<1 x ptr>`: the
/// `xmm`'s low 8 bytes to the general register (`movq r64, xmm`) or to
/// the stack slot; a value in memory through `r10`.
fn lone_out(cx: &mut FnCx<'_, X64>, from: LaneLoc, to: LaneLoc) -> CgResult<()> {
    if !lone_classes_ok(from, to) {
        return Err(CgError::unsupported(CALL_PART_LANES));
    }
    match (from, to) {
        (LaneLoc::Reg(x), LaneLoc::Reg(g)) => put(cx, mc::movd_rx(Size::Q, gp(g), xm(x))),
        (LaneLoc::Reg(x), LaneLoc::Frame(slot)) => store_fpr(cx, xm(x), slot, 8),
        (LaneLoc::Frame(image), LaneLoc::Reg(g)) => load_gpr(cx, gp(g), image, 8),
        (LaneLoc::Frame(image), LaneLoc::Frame(slot)) => {
            load_gpr(cx, R10, image, 8);
            store_gpr(cx, R10, slot, 8);
        }
    }
    Ok(())
}

/// WP-174: the consumer's side of a lone `<1 x i64>`/`<1 x ptr>`: the
/// general register (`movq xmm, r64`) or the stack slot to the `xmm`; to
/// a value in memory through `r10`.
fn lone_in(cx: &mut FnCx<'_, X64>, from: LaneLoc, to: LaneLoc) -> CgResult<()> {
    if !lone_classes_ok(to, from) {
        return Err(CgError::unsupported(CALL_PART_LANES));
    }
    match (from, to) {
        (LaneLoc::Reg(g), LaneLoc::Reg(x)) => put(cx, mc::movd_xr(Size::Q, xm(x), gp(g))),
        (LaneLoc::Frame(slot), LaneLoc::Reg(x)) => load_fpr(cx, xm(x), slot, 8),
        (LaneLoc::Reg(g), LaneLoc::Frame(image)) => store_gpr(cx, gp(g), image, 8),
        (LaneLoc::Frame(slot), LaneLoc::Frame(image)) => {
            load_gpr(cx, R10, slot, 8);
            store_gpr(cx, R10, image, 8);
        }
    }
    Ok(())
}

/// Where a lane move reads lanes from: a frame slot of lanes, or (one
/// element) a general register.
#[derive(Clone, Copy)]
enum Src {
    Frame(FrameRef),
    Reg(Reg),
}

/// Bit `first + i` of `g` set to bit 0 of lane `i` at `src`, through the
/// other scratch register; the part of element 0 clears `g` first.
fn gather(cx: &mut FnCx<'_, X64>, l: Lanes, src: Src, g: Reg) {
    let t = other_scratch(g);
    if l.first == 0 {
        mov_imm(cx, g, 0, 64);
    }
    for i in 0..l.count {
        match src {
            Src::Frame(slot) => load_gpr(cx, t, slot.offset_by(i * l.lane), 1),
            Src::Reg(r) => put(cx, mc::mov_rr(Size::D, t, r)),
        }
        put(cx, mc::and_ri(Size::D, t, 1));
        let bit = l.first + i;
        if bit != 0 {
            put(cx, mc::shl_ri(Size::Q, t, bit as u8));
        }
        if l.first != 0 {
            put(cx, mc::btr_ri(Size::Q, g, bit as u8));
        }
        put(cx, mc::or_rr(Size::Q, g, t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WP-167: a lane move refuses a value side that is not a general
    /// register and a place side that is not an `xmm`.
    #[test]
    fn lane_moves_accept_only_their_register_classes() {
        let (g, x, st) = (reg::gpr(Reg::Rax), reg::xmm(mc::Xmm::Xmm0), reg::st(0));
        let frame = LaneLoc::Frame(FrameRef::Local(0));
        let r = LaneLoc::Reg;
        assert!(classes_ok(r(g), r(x), false));
        assert!(classes_ok(frame, r(x), false));
        assert!(classes_ok(r(g), frame, false));
        assert!(classes_ok(frame, frame, false));
        for (value, place) in [(x, x), (st, x), (g, g), (g, st), (x, g)] {
            assert!(!classes_ok(r(value), r(place), false), "{value:?} {place:?}");
        }
        assert!(!classes_ok(r(x), frame, false));
        assert!(!classes_ok(r(st), frame, false));
        assert!(!classes_ok(frame, r(g), false));
        // WP-208: a part of one element (a byte) has a general place.
        assert!(classes_ok(r(g), r(g), true));
        assert!(classes_ok(frame, r(g), true));
        for (value, place) in [(g, x), (x, g), (st, g), (g, st)] {
            assert!(!classes_ok(r(value), r(place), true), "{value:?} {place:?}");
        }
    }

    /// WP-174: a lone `<1 x i64>` move takes an `xmm` value and a
    /// general place, nothing else.
    #[test]
    fn lone_moves_accept_only_their_register_classes() {
        let (g, x, st) = (reg::gpr(Reg::Rax), reg::xmm(mc::Xmm::Xmm0), reg::st(0));
        let frame = LaneLoc::Frame(FrameRef::Local(0));
        let r = LaneLoc::Reg;
        assert!(lone_classes_ok(r(x), r(g)));
        assert!(lone_classes_ok(frame, r(g)));
        assert!(lone_classes_ok(r(x), frame));
        assert!(lone_classes_ok(frame, frame));
        for (value, place) in [(g, g), (x, x), (st, g), (x, st), (g, x)] {
            assert!(!lone_classes_ok(r(value), r(place)), "{value:?} {place:?}");
        }
    }
}

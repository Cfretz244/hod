; spec/asm.md section 19 on AArch64: what an empty template with
; alternatives does to the memory of an indirect output when the input
; differs from it. With an input without alternatives the output with
; alternatives is not tied and takes memory: the input is not stored.
; With an input with alternatives it is tied and in a register: the
; input is stored.

; *p stays what it was.
define void @untied_i32(ptr %p, i32 %v) {
  call void asm sideeffect "", "=*imr|w,0"(ptr elementtype(i32) %p, i32 %v)
  ret void
}

define void @untied_f64(ptr %p, double %v) {
  call void asm sideeffect "", "=*imr|w,0"(ptr elementtype(double) %p, double %v)
  ret void
}

define void @untied_v4f32(ptr %p, <4 x float> %v) {
  call void asm sideeffect "", "=*imr|w,0"(ptr elementtype(<4 x float>) %p, <4 x float> %v)
  ret void
}

define void @untied_w_first(ptr %p, <2 x double> %v) {
  call void asm sideeffect "", "=*w|imr,0"(ptr elementtype(<2 x double>) %p, <2 x double> %v)
  ret void
}

; *p becomes v.
define void @tied_i32(ptr %p, i32 %v) {
  call void asm sideeffect "", "=*imr|w,0|0"(ptr elementtype(i32) %p, i32 %v)
  ret void
}

define void @tied_i64(ptr %p, i64 %v) {
  call void asm sideeffect "", "=*imr|w,0|0"(ptr elementtype(i64) %p, i64 %v)
  ret void
}

define void @r_i64(ptr %p, i64 %v) {
  call void asm sideeffect "", "=*r|w,0"(ptr elementtype(i64) %p, i64 %v)
  ret void
}

; A template on the untied output's memory: *p + 1, not v + 1.
define void @inc_untied(ptr %p, i32 %v) {
  call void asm sideeffect "ldr w16, $0\0A\09add w16, w16, #1\0A\09str w16, $0", "=*imr|w,0,~{x16}"(ptr elementtype(i32) %p, i32 %v)
  ret void
}

; Direct outputs: the first alternative.
define i32 @direct_r(i32 %v) {
  %r = call i32 asm sideeffect "", "=r|w,0"(i32 %v)
  ret i32 %r
}

define <2 x double> @direct_w(<2 x double> %v) {
  %r = call <2 x double> asm sideeffect "", "=w|r,0"(<2 x double> %v)
  ret <2 x double> %r
}

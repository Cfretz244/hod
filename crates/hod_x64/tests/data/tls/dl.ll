; The dlopen module of hod_x64's tests/tls.rs (WP-213): thread-local
; variables of a shared object loaded by dlopen, reached through TLS
; descriptors (-enable-tlsdesc): general dynamic (preemptible), local
; dynamic (internal and dso_local), a variable of the other module of the
; same shared object (peer.ll, built in the other dialect), a block of
; @@SIZE@@ bytes (large enough, or not, for the loader to give the module
; dynamic rather than static TLS), values live in general and vector
; registers across the accesses, and a select under a fused compare.

@dl_gd = thread_local global i64 5, align 8
@dl_local = dso_local thread_local global i32 6, align 4
@dl_ld = internal thread_local global i32 7, align 4
@dl_big = internal thread_local global [@@SIZE@@ x i8] zeroinitializer, align 16
@peer_gd = external thread_local global i64, align 8

declare ptr @llvm.threadlocal.address.p0(ptr)

define ptr @dl_addr(i32 %k) {
entry:
  switch i32 %k, label %none [
    i32 0, label %gd
    i32 1, label %local
    i32 2, label %ld
    i32 3, label %big
    i32 4, label %peer
  ]
gd:
  %a = call ptr @llvm.threadlocal.address.p0(ptr @dl_gd)
  ret ptr %a
local:
  ret ptr @dl_local
ld:
  %c = call ptr @llvm.threadlocal.address.p0(ptr @dl_ld)
  ret ptr %c
big:
  ret ptr getelementptr inbounds ([@@SIZE@@ x i8], ptr @dl_big, i64 0, i64 @@LAST@@)
peer:
  ret ptr @peer_gd
none:
  ret ptr null
}

define i64 @dl_sum() {
  %a = load i64, ptr @dl_gd, align 8
  %b = load i32, ptr @dl_local, align 4
  %p = call ptr @llvm.threadlocal.address.p0(ptr @dl_ld)
  %c = load i32, ptr %p, align 4
  %d = load i8, ptr getelementptr inbounds ([@@SIZE@@ x i8], ptr @dl_big, i64 0, i64 @@LAST@@), align 1
  %e = load i64, ptr @peer_gd, align 8
  %b1 = sext i32 %b to i64
  %c1 = sext i32 %c to i64
  %d1 = zext i8 %d to i64
  %s1 = add i64 %a, %b1
  %s2 = add i64 %s1, %c1
  %s3 = add i64 %s2, %d1
  %s4 = add i64 %s3, %e
  ret i64 %s4
}

define void @dl_bump(i64 %k) {
  %a = load i64, ptr @dl_gd, align 8
  %a1 = add i64 %a, %k
  store i64 %a1, ptr @dl_gd, align 8
  %k32 = trunc i64 %k to i32
  %b = load i32, ptr @dl_local, align 4
  %b1 = add i32 %b, %k32
  store i32 %b1, ptr @dl_local, align 4
  %c = load i32, ptr @dl_ld, align 4
  %c1 = add i32 %c, %k32
  store i32 %c1, ptr @dl_ld, align 4
  %k8 = trunc i64 %k to i8
  %pd = getelementptr inbounds [@@SIZE@@ x i8], ptr @dl_big, i64 0, i64 @@LAST@@
  %d = load i8, ptr %pd, align 1
  %d1 = add i8 %d, %k8
  store i8 %d1, ptr %pd, align 1
  %e = load i64, ptr @peer_gd, align 8
  %e1 = add i64 %e, %k
  store i64 %e1, ptr @peer_gd, align 8
  ret void
}

; Arguments and intermediate values live across every access, in general
; and vector registers: a descriptor call keeps all of them but rax and
; the flags.
define i64 @dl_live(i64 %a, i64 %b, i64 %c, i64 %d, double %x, double %y) {
  %ab = mul i64 %a, %b
  %xy = fmul double %x, %y
  %p = call ptr @llvm.threadlocal.address.p0(ptr @dl_big)
  %v0 = load i8, ptr %p, align 1
  %q = getelementptr inbounds i8, ptr @dl_big, i64 1
  %v1 = load i8, ptr %q, align 1
  %g = load i64, ptr @dl_gd, align 8
  %l = load i32, ptr @dl_ld, align 4
  %cd = mul i64 %c, %d
  %xx = fadd double %xy, %x
  %w0 = zext i8 %v0 to i64
  %w1 = zext i8 %v1 to i64
  %l1 = sext i32 %l to i64
  %s1 = add i64 %ab, %cd
  %s2 = add i64 %s1, %w0
  %s3 = add i64 %s2, %w1
  %s4 = add i64 %s3, %g
  %s5 = add i64 %s4, %l1
  %f = fptosi double %xx to i64
  %s6 = add i64 %s5, %f
  ret i64 %s6
}

; The flags of a fused compare survive a descriptor access between the
; compare and the cmov.
define ptr @dl_select(i64 %a, i64 %b) {
  %c = icmp slt i64 %a, %b
  %p = select i1 %c, ptr @dl_gd, ptr @dl_local
  ret ptr %p
}

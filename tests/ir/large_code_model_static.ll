; The large code model (-code-model=large) in a position-dependent
; program: every symbol's address is 64-bit (x86-64 movabs, AArch64
; movz/movk; a preemptible symbol read from the GOT on AArch64, as llc
; does), in the same program as large_code_model.ll (spec/codegen.md
; §5.6.36), its landing pads not reached (see large_code_model.ll).
; Needs the C++ runtime.
;@ covers: spec/ir.md §2.1 module flags (Code Model)
;@ llc-args: -code-model=large
;@ link: c++ no-pie
;@ stdout: large code
;@ stdout: catch=7 pick=13,-1 table=17 ops=5,8 same=1 tls=6 tail=42 div=100000000000000000 copy=101 big=77 ctor=1
;@ stdout: counter=5

@_ZTIi = external constant ptr

; Through the GOT under PIC (not dso_local): GOT64.
@counter = global i32 5
; dso_local: GOTOFF64 under PIC.
@big = dso_local global [4096 x i32] zeroinitializer
@table = dso_local constant [6 x i32] [i32 3, i32 1, i32 4, i32 1, i32 5, i32 9]
@ops = dso_local global [2 x ptr] [ptr @add1, ptr @twice]
@ctor_ran = internal global i32 0
@tl = dso_local thread_local global i32 3
@tz = internal thread_local global i32 0
@.msg = private unnamed_addr constant [11 x i8] c"large code\00"
@.fmt = private unnamed_addr constant [95 x i8] c"catch=%d pick=%d,%d table=%d ops=%d,%d same=%d tls=%d tail=%d div=%lld copy=%d big=%d ctor=%d\0A\00"
@.counter = private unnamed_addr constant [12 x i8] c"counter=%d\0A\00"

@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @init, ptr null }]

declare i32 @printf(ptr, ...)
declare i32 @puts(ptr)
declare ptr @__cxa_allocate_exception(i64)
declare void @__cxa_throw(ptr, ptr, ptr)
declare ptr @__cxa_begin_catch(ptr)
declare void @__cxa_end_catch()
declare i32 @__gxx_personality_v0(...)
declare i32 @llvm.eh.typeid.for.p0(ptr)
declare void @llvm.memcpy.p0.p0.i64(ptr, ptr, i64, i1)

define internal void @init() {
  store i32 1, ptr @ctor_ran
  ret void
}

define internal i32 @add1(i32 %x) {
  %r = add i32 %x, 1
  ret i32 %r
}

define dso_local i32 @twice(i32 %x) {
  %r = shl i32 %x, 1
  ret i32 %r
}

; A tail call, and a call that is not dso_local's.
define dso_local i32 @tail(i32 %x) {
  %r = musttail call i32 @twice(i32 %x)
  ret i32 %r
}

define dso_local i32 @pick(i32 %x) {
  switch i32 %x, label %other [
    i32 0, label %c0
    i32 1, label %c1
    i32 2, label %c2
    i32 3, label %c3
    i32 4, label %c4
    i32 5, label %c5
    i32 6, label %c6
    i32 7, label %c7
  ]
c0:
  ret i32 10
c1:
  ret i32 11
c2:
  ret i32 12
c3:
  ret i32 13
c4:
  ret i32 14
c5:
  ret i32 15
c6:
  ret i32 16
c7:
  ret i32 17
other:
  ret i32 -1
}

; Loads at constant offsets of a read-only table: the addends.
define dso_local i32 @sum_table() {
  %a = load i32, ptr @table
  %b = load i32, ptr getelementptr inbounds ([6 x i32], ptr @table, i64 0, i64 1)
  %c = load i32, ptr getelementptr inbounds ([6 x i32], ptr @table, i64 0, i64 2)
  %d = load i32, ptr getelementptr inbounds ([6 x i32], ptr @table, i64 0, i64 5)
  %ab = add i32 %a, %b
  %cd = add i32 %c, %d
  %r = add i32 %ab, %cd
  ret i32 %r
}

define dso_local i32 @tls_bump() {
  %a = load i32, ptr @tl
  %b = add i32 %a, 1
  store i32 %b, ptr @tl
  %z = load i32, ptr @tz
  %z1 = add i32 %z, 2
  store i32 %z1, ptr @tz
  %r = add i32 %b, %z1
  ret i32 %r
}

define dso_local i64 @div(i128 %a, i128 %b) {
  %q = udiv i128 %a, %b
  %r = trunc i128 %q to i64
  ret i64 %r
}

; memcpy of a size known only at run time: a call of the runtime's.
define dso_local i32 @copy(i64 %n) {
  %buf = alloca [16 x i8], align 1
  call void @llvm.memcpy.p0.p0.i64(ptr %buf, ptr @.msg, i64 %n, i1 false)
  %p = getelementptr inbounds i8, ptr %buf, i64 4
  %c = load i8, ptr %p
  %r = sext i8 %c to i32
  ret i32 %r
}

define dso_local void @thrower(i32 %v) {
  %e = call ptr @__cxa_allocate_exception(i64 4)
  store i32 %v, ptr %e
  call void @__cxa_throw(ptr %e, ptr @_ZTIi, ptr null) noreturn
  unreachable
}

; A cleanup that resumes.
define dso_local i32 @middle(i32 %v) personality ptr @__gxx_personality_v0 {
  invoke void @thrower(i32 %v) to label %ok unwind label %pad
ok:
  ret i32 0
pad:
  %lp = landingpad { ptr, i32 } cleanup
  %c = load i32, ptr @counter
  %c1 = add i32 %c, 100
  store i32 %c1, ptr @counter
  resume { ptr, i32 } %lp
}

define dso_local i32 @catcher(i32 %v) personality ptr @__gxx_personality_v0 {
  %r = invoke i32 @middle(i32 %v) to label %ok unwind label %pad
ok:
  ret i32 %r
pad:
  %lp = landingpad { ptr, i32 } catch ptr @_ZTIi
  %sel = extractvalue { ptr, i32 } %lp, 1
  %id = call i32 @llvm.eh.typeid.for.p0(ptr @_ZTIi)
  %match = icmp eq i32 %sel, %id
  br i1 %match, label %caught, label %again
caught:
  %ex = extractvalue { ptr, i32 } %lp, 0
  %obj = call ptr @__cxa_begin_catch(ptr %ex)
  %val = load i32, ptr %obj
  call void @__cxa_end_catch()
  ret i32 %val
again:
  resume { ptr, i32 } %lp
}

define dso_local i32 @main() {
  %catch = add i32 7, 0
  %p3 = call i32 @pick(i32 3)
  %p42 = call i32 @pick(i32 42)
  %table = call i32 @sum_table()
  %f0 = load ptr, ptr @ops
  %f1 = load ptr, ptr getelementptr inbounds ([2 x ptr], ptr @ops, i64 0, i64 1)
  %a = call i32 %f0(i32 4)
  %b = call i32 %f1(i32 4)
  %same = icmp eq ptr %f1, @twice
  %same32 = zext i1 %same to i32
  %tls = call i32 @tls_bump()
  %tail = call i32 @tail(i32 21)
  %div = call i64 @div(i128 100000000000000000000, i128 1000)
  %copy = call i32 @copy(i64 11)
  store i32 77, ptr getelementptr inbounds ([4096 x i32], ptr @big, i64 0, i64 4000)
  %bigv = load i32, ptr getelementptr inbounds ([4096 x i32], ptr @big, i64 0, i64 4000)
  %ctor = load i32, ptr @ctor_ran
  %said = call i32 @puts(ptr @.msg)
  %printed = call i32 (ptr, ...) @printf(ptr @.fmt, i32 %catch, i32 %p3, i32 %p42, i32 %table, i32 %a, i32 %b, i32 %same32, i32 %tls, i32 %tail, i64 %div, i32 %copy, i32 %bigv, i32 %ctor)
  %counter = load i32, ptr @counter
  %printed2 = call i32 (ptr, ...) @printf(ptr @.counter, i32 %counter)
  ret i32 0
}

; WP-209: the start of a program linked without the C runtime's start
; files (tests/large_code_model.rs), whose small-model code would need the
; GOT and the data within 2 GiB. A stub the test assembles enters at
; `_start` and calls `start_c` with `argc`, `argv`, the dynamic linker's
; finalizer and the stack's end; `start_c` hands them to glibc as `crt1.o`
; does. Built for the large code model like the program.

declare i32 @__libc_start_main(ptr, i32, ptr, ptr, ptr, ptr, ptr)
declare i32 @main()

define dso_local void @start_c(i32 %argc, ptr %argv, ptr %fini, ptr %end) {
  %r = call i32 @__libc_start_main(ptr @main, i32 %argc, ptr %argv, ptr null, ptr null, ptr %fini, ptr %end)
  unreachable
}

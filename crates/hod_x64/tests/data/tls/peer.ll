; The second module of the dlopen shared object of hod_x64's tests/tls.rs
; (WP-213), built in the other TLS dialect than dl.ll: a variable each
; module reaches in the other, so the shared object holds both a
; descriptor and a __tls_get_addr GOT entry for the same symbol.

@peer_gd = thread_local global i64 40, align 8
@peer_ld = internal thread_local global i32 41, align 4
@dl_gd = external thread_local global i64, align 8

define ptr @peer_addr(i32 %k) {
entry:
  switch i32 %k, label %none [
    i32 0, label %own
    i32 1, label %other
    i32 2, label %ld
  ]
own:
  ret ptr @peer_gd
other:
  ret ptr @dl_gd
ld:
  ret ptr @peer_ld
none:
  ret ptr null
}

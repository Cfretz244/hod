// Multi-alternative inline-assembly constraints on AArch64 (spec/asm.md
// section 19): the optimization barrier "+g,w" as C++ sources write it,
// and templates whose constraints have alternatives. Compiled by kiln to
// barrier.ll (-O0) and barrier-g.ll (-O0 -g); the program checks what each
// function computes and prints "checked, N failures".
#include <arm_neon.h>

extern "C" int printf(const char *, ...);

#define BARRIER(X) __asm__("" : "+g,w"(X))

static int failures;

static void check(bool ok, const char *what) {
  if (!ok) {
    ++failures;
    printf("FAILED %s\n", what);
  }
}

template <class T> __attribute__((noinline)) T pass(T x) {
  BARRIER(x);
  return x;
}

__attribute__((noinline)) float32x4_t add_ps(float32x4_t a, float32x4_t b) {
  float32x4_t c = a + b;
  BARRIER(c);
  return c * a;
}

__attribute__((noinline)) float64x2_t add_pd(float64x2_t a, float64x2_t b) {
  float64x2_t c = a + b;
  BARRIER(c);
  return c - b;
}

// "=r,m" with "r,r": the output's pointer makes `r` invalid, so the
// output is memory and the input a register.
__attribute__((noinline)) long copy_rm(long in) {
  long out = -1;
  __asm__("str %1, %0" : "=r,m"(out) : "r,r"(in));
  return out;
}

// "+r,m": the output is memory (its tied input has no alternatives).
__attribute__((noinline)) long add_m(long a, long b) {
  __asm__("ldr x16, %0\n\tadd x16, x16, %1\n\tstr x16, %0" : "+r,m"(a) : "r,r"(b) : "x16");
  return a;
}

__attribute__((noinline)) double keep(double d) {
  __asm__ __volatile__("" : : "r,m"(d) : "memory");
  __asm__ __volatile__("" : : "w,m"(d) : "memory");
  return d * 2;
}

int main() {
  check(pass(41) == 41, "int");
  check(pass(-5L) == -5L, "long");
  check(pass((short)-3) == -3, "short");
  check(pass('a') == 'a', "char");
  check(pass(2.5f) == 2.5f, "float");
  check(pass(-0.125) == -0.125, "double");
  check(pass((__int128)1 << 100) == (__int128)1 << 100, "int128");
  static int cell = 9;
  check(pass(&cell) == &cell && *pass(&cell) == 9, "pointer");
  float32x4_t a = {1, 2, 3, 4}, b = {10, 20, 30, 40};
  float32x4_t c = add_ps(a, b);
  check(c[0] == 11 && c[1] == 44 && c[2] == 99 && c[3] == 176, "float32x4_t");
  float32x4_t p = pass(a);
  check(p[0] == 1 && p[3] == 4, "pass float32x4_t");
  float64x2_t d = add_pd(float64x2_t{1.5, -2}, float64x2_t{4, 8});
  check(d[0] == 1.5 && d[1] == -2, "float64x2_t");
  int32x2_t s = pass(int32x2_t{7, -9});
  check(s[0] == 7 && s[1] == -9, "int32x2_t");
  check(copy_rm(1234) == 1234, "copy_rm");
  check(add_m(40, 2) == 42, "add_m");
  check(keep(1.25) == 2.5, "keep");
  printf("checked, %d failures\n", failures);
  return failures != 0;
}

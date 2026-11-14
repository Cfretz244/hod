// Multi-alternative inline-assembly constraints (spec/asm.md section 19),
// as C++ sources write them: the optimization barrier "+g,x" and a few
// templates whose constraints have alternatives. Compiled by kiln to
// barrier.ll (-O0) and barrier-g.ll (-O0 -g); the program checks what each
// function computes and prints "checked, N failures".
#include <immintrin.h>

extern "C" int printf(const char *, ...);

#define BARRIER(X) __asm__("" : "+g,x"(X))

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

__attribute__((noinline)) float twice_f(float x) {
  float y = x + x;
  BARRIER(y);
  return y + 1.0f;
}

__attribute__((noinline)) __m128 add_ps(__m128 a, __m128 b) {
  __m128 c = a + b;
  BARRIER(c);
  return c * a;
}

__attribute__((noinline)) __m128d add_pd(__m128d a, __m128d b) {
  __m128d c = a + b;
  BARRIER(c);
  return c - b;
}

__attribute__((noinline, target("avx"))) __m256 add_ps8(__m256 a, __m256 b) {
  __m256 c = a + b;
  BARRIER(c);
  return c;
}

__attribute__((noinline, target("avx"))) float sum8(float base) {
  __m256 v = {base, base + 1, base + 2, base + 3, base + 4, base + 5, base + 6, base + 7};
  __m256 w = add_ps8(v, v);
  float s = 0;
  for (int i = 0; i < 8; ++i)
    s += w[i];
  return s;
}

// A template on the memory the "g" alternative gives: `incl` of the
// variable itself.
__attribute__((noinline)) int inc_g(int x) {
  __asm__("incl %0" : "+g,x"(x));
  return x;
}

// "=r,m" with "ri,r": the first alternative is invalid for the output's
// pointer, so the output is memory and the input a register.
__attribute__((noinline)) int copy_rm(int in) {
  int out = -1;
  __asm__("movl %1, %0" : "=r,m"(out) : "ri,r"(in));
  return out;
}

// "=x,m": the output in memory, the input in an SSE register.
__attribute__((noinline)) float copy_ss(float in) {
  float out = -1;
  __asm__("movss %1, %0" : "=x,m"(out) : "x,x"(in));
  return out;
}

// "+r,m": the read-write output is indirect, weighed by its pointer (`r`
// invalid), and its tied input has no alternatives: the output is
// memory, `addq` adds into the variable.
__attribute__((noinline)) long add_r(long a, long b) {
  __asm__("addq %1, %0" : "+r,m"(a) : "r,r"(b));
  return a;
}

// An input the barrier only reads.
__attribute__((noinline)) double keep(double d) {
  __asm__ __volatile__("" : : "r,m"(d) : "memory");
  __asm__ __volatile__("" : : "x,m"(d) : "memory");
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
  check(twice_f(3.0f) == 7.0f, "twice_f");
  __m128 a = {1, 2, 3, 4}, b = {10, 20, 30, 40};
  __m128 c = add_ps(a, b);
  check(c[0] == 11 && c[1] == 44 && c[2] == 99 && c[3] == 176, "__m128");
  __m128 p = pass(a);
  check(p[0] == 1 && p[3] == 4, "pass __m128");
  __m128d d = add_pd(__m128d{1.5, -2}, __m128d{4, 8});
  check(d[0] == 1.5 && d[1] == -2, "__m128d");
  __m128i i = pass(__m128i{7, -9});
  check(i[0] == 7 && i[1] == -9, "__m128i");
  check(sum8(1) == 72, "__m256");
  check(inc_g(41) == 42, "inc_g");
  check(copy_rm(1234) == 1234, "copy_rm");
  check(copy_ss(6.5f) == 6.5f, "copy_ss");
  check(add_r(40, 2) == 42, "add_r");
  check(keep(1.25) == 2.5, "keep");
  printf("checked, %d failures\n", failures);
  return failures != 0;
}

/* The driver of semantics.ll (spec/asm.md section 19). */
#include <stdio.h>

typedef float v4f __attribute__((vector_size(16)));
typedef double v2d __attribute__((vector_size(16)));

void untied_i32(int *, int);
void untied_f64(double *, double);
void untied_v4f32(v4f *, v4f);
void untied_w_first(v2d *, v2d);
void tied_i32(int *, int);
void tied_i64(long *, long);
void r_i64(long *, long);
void inc_untied(int *, int);
int direct_r(int);
v2d direct_w(v2d);

static int failures;

static void check(int ok, const char *what) {
  if (!ok) {
    ++failures;
    printf("FAILED %s\n", what);
  }
}

int main(void) {
  int i = 5;
  untied_i32(&i, 77);
  check(i == 5, "untied_i32");
  double d = 1.5;
  untied_f64(&d, -8);
  check(d == 1.5, "untied_f64");
  v4f f = {1, 2, 3, 4};
  untied_v4f32(&f, (v4f){9, 9, 9, 9});
  check(f[0] == 1 && f[3] == 4, "untied_v4f32");
  v2d e = {0.5, 0.25};
  untied_w_first(&e, (v2d){7, 7});
  check(e[0] == 0.5 && e[1] == 0.25, "untied_w_first");
  i = 5;
  tied_i32(&i, 77);
  check(i == 77, "tied_i32");
  long l = 3;
  tied_i64(&l, -12345678901L);
  check(l == -12345678901L, "tied_i64");
  l = 3;
  r_i64(&l, 424242424242L);
  check(l == 424242424242L, "r_i64");
  i = 10;
  inc_untied(&i, 100);
  check(i == 11, "inc_untied");
  check(direct_r(-42) == -42, "direct_r");
  v2d x = direct_w((v2d){3.5, -1});
  check(x[0] == 3.5 && x[1] == -1, "direct_w");
  printf("checked, %d failures\n", failures);
  return failures != 0;
}

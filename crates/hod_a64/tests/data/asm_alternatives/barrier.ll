; ModuleID = 'barrier.cpp'
source_filename = "barrier.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "aarch64-unknown-linux-gnu"

$_Z4passIiET_S0_ = comdat any

$_Z4passIlET_S0_ = comdat any

$_Z4passIsET_S0_ = comdat any

$_Z4passIcET_S0_ = comdat any

$_Z4passIfET_S0_ = comdat any

$_Z4passIdET_S0_ = comdat any

$_Z4passInET_S0_ = comdat any

$_Z4passIPiET_S1_ = comdat any

$_Z4passI13__Float32x4_tET_S1_ = comdat any

$_Z4passI11__Int32x2_tET_S1_ = comdat any

@.str = private unnamed_addr constant [4 x i8] c"int\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"long\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"short\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"char\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"float\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"double\00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"int128\00", align 1
@_ZZ4mainE4cell = internal global i32 9, align 4
@.str.7 = private unnamed_addr constant [8 x i8] c"pointer\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"float32x4_t\00", align 1
@.str.9 = private unnamed_addr constant [17 x i8] c"pass float32x4_t\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"float64x2_t\00", align 1
@.str.11 = private unnamed_addr constant [10 x i8] c"int32x2_t\00", align 1
@.str.12 = private unnamed_addr constant [8 x i8] c"copy_rm\00", align 1
@.str.13 = private unnamed_addr constant [6 x i8] c"add_m\00", align 1
@.str.14 = private unnamed_addr constant [5 x i8] c"keep\00", align 1
@.str.15 = private unnamed_addr constant [22 x i8] c"checked, %d failures\0A\00", align 1
@_ZL8failures = internal global i32 0, align 4
@.str.16 = private unnamed_addr constant [11 x i8] c"FAILED %s\0A\00", align 1

define dso_local noundef <4 x float> @_Z6add_ps13__Float32x4_tS_(<4 x float> noundef %0, <4 x float> noundef %1) #0 {
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %3, align 16
  store <4 x float> %1, ptr %4, align 16
  %6 = load <4 x float>, ptr %3, align 16
  %7 = load <4 x float>, ptr %4, align 16
  %8 = fadd <4 x float> %6, %7
  store <4 x float> %8, ptr %5, align 16
  %9 = load <4 x float>, ptr %5, align 16
  call void asm "", "=*imr|w,0"(ptr elementtype(<4 x float>) %5, <4 x float> %9) #4, !srcloc !6
  %10 = load <4 x float>, ptr %5, align 16
  %11 = load <4 x float>, ptr %3, align 16
  %12 = fmul <4 x float> %10, %11
  ret <4 x float> %12
}

define dso_local noundef <2 x double> @_Z6add_pd13__Float64x2_tS_(<2 x double> noundef %0, <2 x double> noundef %1) #0 {
  %3 = alloca <2 x double>, align 16
  %4 = alloca <2 x double>, align 16
  %5 = alloca <2 x double>, align 16
  store <2 x double> %0, ptr %3, align 16
  store <2 x double> %1, ptr %4, align 16
  %6 = load <2 x double>, ptr %3, align 16
  %7 = load <2 x double>, ptr %4, align 16
  %8 = fadd <2 x double> %6, %7
  store <2 x double> %8, ptr %5, align 16
  %9 = load <2 x double>, ptr %5, align 16
  call void asm "", "=*imr|w,0"(ptr elementtype(<2 x double>) %5, <2 x double> %9) #4, !srcloc !7
  %10 = load <2 x double>, ptr %5, align 16
  %11 = load <2 x double>, ptr %4, align 16
  %12 = fsub <2 x double> %10, %11
  ret <2 x double> %12
}

define dso_local noundef i64 @_Z7copy_rml(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  store i64 -1, ptr %3, align 8
  %4 = load i64, ptr %2, align 8
  call void asm "str $1, $0", "=*r|m,r|r"(ptr elementtype(i64) %3, i64 %4) #4, !srcloc !8
  %5 = load i64, ptr %3, align 8
  ret i64 %5
}

define dso_local noundef i64 @_Z5add_mll(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = load i64, ptr %4, align 8
  call void asm "ldr x16, $0\0A\09add x16, x16, $1\0A\09str x16, $0", "=*r|m,r|r,0,~{x16}"(ptr elementtype(i64) %3, i64 %6, i64 %5) #4, !srcloc !9
  %7 = load i64, ptr %3, align 8
  ret i64 %7
}

define dso_local noundef double @_Z4keepd(double noundef %0) #0 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
  %3 = load double, ptr %2, align 8
  call void asm sideeffect "", "r|m,~{memory}"(double %3) #4, !srcloc !10
  %4 = load double, ptr %2, align 8
  call void asm sideeffect "", "w|m,~{memory}"(double %4) #4, !srcloc !11
  %5 = load double, ptr %2, align 8
  %6 = fmul double %5, 2.000000e+00
  ret double %6
}

define dso_local noundef i32 @main() #1 {
  %1 = alloca i32, align 4
  %2 = alloca <4 x float>, align 16
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  %6 = alloca <2 x double>, align 16
  %7 = alloca <2 x i32>, align 8
  store i32 0, ptr %1, align 4
  %8 = call noundef i32 @_Z4passIiET_S0_(i32 noundef 41)
  %9 = icmp eq i32 %8, 41
  call void @_ZL5checkbPKc(i1 noundef %9, ptr noundef @.str)
  %10 = call noundef i64 @_Z4passIlET_S0_(i64 noundef -5)
  %11 = icmp eq i64 %10, -5
  call void @_ZL5checkbPKc(i1 noundef %11, ptr noundef @.str.1)
  %12 = call noundef i16 @_Z4passIsET_S0_(i16 noundef -3)
  %13 = sext i16 %12 to i32
  %14 = icmp eq i32 %13, -3
  call void @_ZL5checkbPKc(i1 noundef %14, ptr noundef @.str.2)
  %15 = call noundef i8 @_Z4passIcET_S0_(i8 noundef 97)
  %16 = zext i8 %15 to i32
  %17 = icmp eq i32 %16, 97
  call void @_ZL5checkbPKc(i1 noundef %17, ptr noundef @.str.3)
  %18 = call noundef float @_Z4passIfET_S0_(float noundef 2.500000e+00)
  %19 = fcmp oeq float %18, 2.500000e+00
  call void @_ZL5checkbPKc(i1 noundef %19, ptr noundef @.str.4)
  %20 = call noundef double @_Z4passIdET_S0_(double noundef -1.250000e-01)
  %21 = fcmp oeq double %20, -1.250000e-01
  call void @_ZL5checkbPKc(i1 noundef %21, ptr noundef @.str.5)
  %22 = call noundef i128 @_Z4passInET_S0_(i128 noundef 1267650600228229401496703205376)
  %23 = icmp eq i128 %22, 1267650600228229401496703205376
  call void @_ZL5checkbPKc(i1 noundef %23, ptr noundef @.str.6)
  %24 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell)
  %25 = icmp eq ptr %24, @_ZZ4mainE4cell
  br i1 %25, label %26, label %30

26:
  %27 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell)
  %28 = load i32, ptr %27, align 4
  %29 = icmp eq i32 %28, 9
  br label %30

30:
  %31 = phi i1 [ false, %0 ], [ %29, %26 ]
  call void @_ZL5checkbPKc(i1 noundef %31, ptr noundef @.str.7)
  store <4 x float> <float 1.000000e+00, float 2.000000e+00, float 3.000000e+00, float 4.000000e+00>, ptr %2, align 16
  store <4 x float> <float 1.000000e+01, float 2.000000e+01, float 3.000000e+01, float 4.000000e+01>, ptr %3, align 16
  %32 = load <4 x float>, ptr %2, align 16
  %33 = load <4 x float>, ptr %3, align 16
  %34 = call noundef <4 x float> @_Z6add_ps13__Float32x4_tS_(<4 x float> noundef %32, <4 x float> noundef %33)
  store <4 x float> %34, ptr %4, align 16
  %35 = load <4 x float>, ptr %4, align 16
  %36 = extractelement <4 x float> %35, i32 0
  %37 = fcmp oeq float %36, 1.100000e+01
  br i1 %37, label %38, label %50

38:
  %39 = load <4 x float>, ptr %4, align 16
  %40 = extractelement <4 x float> %39, i32 1
  %41 = fcmp oeq float %40, 4.400000e+01
  br i1 %41, label %42, label %50

42:
  %43 = load <4 x float>, ptr %4, align 16
  %44 = extractelement <4 x float> %43, i32 2
  %45 = fcmp oeq float %44, 9.900000e+01
  br i1 %45, label %46, label %50

46:
  %47 = load <4 x float>, ptr %4, align 16
  %48 = extractelement <4 x float> %47, i32 3
  %49 = fcmp oeq float %48, 1.760000e+02
  br label %50

50:
  %51 = phi i1 [ false, %42 ], [ false, %38 ], [ false, %30 ], [ %49, %46 ]
  call void @_ZL5checkbPKc(i1 noundef %51, ptr noundef @.str.8)
  %52 = load <4 x float>, ptr %2, align 16
  %53 = call noundef <4 x float> @_Z4passI13__Float32x4_tET_S1_(<4 x float> noundef %52)
  store <4 x float> %53, ptr %5, align 16
  %54 = load <4 x float>, ptr %5, align 16
  %55 = extractelement <4 x float> %54, i32 0
  %56 = fcmp oeq float %55, 1.000000e+00
  br i1 %56, label %57, label %61

57:
  %58 = load <4 x float>, ptr %5, align 16
  %59 = extractelement <4 x float> %58, i32 3
  %60 = fcmp oeq float %59, 4.000000e+00
  br label %61

61:
  %62 = phi i1 [ false, %50 ], [ %60, %57 ]
  call void @_ZL5checkbPKc(i1 noundef %62, ptr noundef @.str.9)
  %63 = call noundef <2 x double> @_Z6add_pd13__Float64x2_tS_(<2 x double> noundef <double 1.500000e+00, double -2.000000e+00>, <2 x double> noundef <double 4.000000e+00, double 8.000000e+00>)
  store <2 x double> %63, ptr %6, align 16
  %64 = load <2 x double>, ptr %6, align 16
  %65 = extractelement <2 x double> %64, i32 0
  %66 = fcmp oeq double %65, 1.500000e+00
  br i1 %66, label %67, label %71

67:
  %68 = load <2 x double>, ptr %6, align 16
  %69 = extractelement <2 x double> %68, i32 1
  %70 = fcmp oeq double %69, -2.000000e+00
  br label %71

71:
  %72 = phi i1 [ false, %61 ], [ %70, %67 ]
  call void @_ZL5checkbPKc(i1 noundef %72, ptr noundef @.str.10)
  %73 = call noundef <2 x i32> @_Z4passI11__Int32x2_tET_S1_(<2 x i32> noundef <i32 7, i32 -9>)
  store <2 x i32> %73, ptr %7, align 8
  %74 = load <2 x i32>, ptr %7, align 8
  %75 = extractelement <2 x i32> %74, i32 0
  %76 = icmp eq i32 %75, 7
  br i1 %76, label %77, label %81

77:
  %78 = load <2 x i32>, ptr %7, align 8
  %79 = extractelement <2 x i32> %78, i32 1
  %80 = icmp eq i32 %79, -9
  br label %81

81:
  %82 = phi i1 [ false, %71 ], [ %80, %77 ]
  call void @_ZL5checkbPKc(i1 noundef %82, ptr noundef @.str.11)
  %83 = call noundef i64 @_Z7copy_rml(i64 noundef 1234)
  %84 = icmp eq i64 %83, 1234
  call void @_ZL5checkbPKc(i1 noundef %84, ptr noundef @.str.12)
  %85 = call noundef i64 @_Z5add_mll(i64 noundef 40, i64 noundef 2)
  %86 = icmp eq i64 %85, 42
  call void @_ZL5checkbPKc(i1 noundef %86, ptr noundef @.str.13)
  %87 = call noundef double @_Z4keepd(double noundef 1.250000e+00)
  %88 = fcmp oeq double %87, 2.500000e+00
  call void @_ZL5checkbPKc(i1 noundef %88, ptr noundef @.str.14)
  %89 = load i32, ptr @_ZL8failures, align 4
  %90 = call i32 (ptr, ...) @printf(ptr noundef @.str.15, i32 noundef %89)
  %91 = load i32, ptr @_ZL8failures, align 4
  %92 = icmp ne i32 %91, 0
  %93 = zext i1 %92 to i32
  ret i32 %93
}

define internal void @_ZL5checkbPKc(i1 noundef %0, ptr noundef %1) #2 {
  %3 = alloca i8, align 1
  %4 = alloca ptr, align 8
  %5 = zext i1 %0 to i8
  store i8 %5, ptr %3, align 1
  store ptr %1, ptr %4, align 8
  %6 = load i8, ptr %3, align 1
  %7 = trunc i8 %6 to i1
  br i1 %7, label %13, label %8

8:
  %9 = load i32, ptr @_ZL8failures, align 4
  %10 = add nsw i32 %9, 1
  store i32 %10, ptr @_ZL8failures, align 4
  %11 = load ptr, ptr %4, align 8
  %12 = call i32 (ptr, ...) @printf(ptr noundef @.str.16, ptr noundef %11)
  br label %13

13:
  ret void
}

define linkonce_odr dso_local noundef i32 @_Z4passIiET_S0_(i32 noundef %0) #0 comdat {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  call void asm "", "=*imr|w,0"(ptr elementtype(i32) %2, i32 %3) #4, !srcloc !12
  %4 = load i32, ptr %2, align 4
  ret i32 %4
}

define linkonce_odr dso_local noundef i64 @_Z4passIlET_S0_(i64 noundef %0) #0 comdat {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  call void asm "", "=*imr|w,0"(ptr elementtype(i64) %2, i64 %3) #4, !srcloc !12
  %4 = load i64, ptr %2, align 8
  ret i64 %4
}

define linkonce_odr dso_local noundef i16 @_Z4passIsET_S0_(i16 noundef %0) #0 comdat {
  %2 = alloca i16, align 2
  store i16 %0, ptr %2, align 2
  %3 = load i16, ptr %2, align 2
  call void asm "", "=*imr|w,0"(ptr elementtype(i16) %2, i16 %3) #4, !srcloc !12
  %4 = load i16, ptr %2, align 2
  ret i16 %4
}

define linkonce_odr dso_local noundef i8 @_Z4passIcET_S0_(i8 noundef %0) #0 comdat {
  %2 = alloca i8, align 1
  store i8 %0, ptr %2, align 1
  %3 = load i8, ptr %2, align 1
  call void asm "", "=*imr|w,0"(ptr elementtype(i8) %2, i8 %3) #4, !srcloc !12
  %4 = load i8, ptr %2, align 1
  ret i8 %4
}

define linkonce_odr dso_local noundef float @_Z4passIfET_S0_(float noundef %0) #0 comdat {
  %2 = alloca float, align 4
  store float %0, ptr %2, align 4
  %3 = load float, ptr %2, align 4
  call void asm "", "=*imr|w,0"(ptr elementtype(float) %2, float %3) #4, !srcloc !12
  %4 = load float, ptr %2, align 4
  ret float %4
}

define linkonce_odr dso_local noundef double @_Z4passIdET_S0_(double noundef %0) #0 comdat {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
  %3 = load double, ptr %2, align 8
  call void asm "", "=*imr|w,0"(ptr elementtype(double) %2, double %3) #4, !srcloc !12
  %4 = load double, ptr %2, align 8
  ret double %4
}

define linkonce_odr dso_local noundef i128 @_Z4passInET_S0_(i128 noundef %0) #0 comdat {
  %2 = alloca i128, align 16
  store i128 %0, ptr %2, align 16
  %3 = load i128, ptr %2, align 16
  call void asm "", "=*imr|w,0"(ptr elementtype(i128) %2, i128 %3) #4, !srcloc !12
  %4 = load i128, ptr %2, align 16
  ret i128 %4
}

define linkonce_odr dso_local noundef ptr @_Z4passIPiET_S1_(ptr noundef %0) #0 comdat {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  call void asm "", "=*imr|w,0"(ptr elementtype(ptr) %2, ptr %3) #4, !srcloc !12
  %4 = load ptr, ptr %2, align 8
  ret ptr %4
}

define linkonce_odr dso_local noundef <4 x float> @_Z4passI13__Float32x4_tET_S1_(<4 x float> noundef %0) #0 comdat {
  %2 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %2, align 16
  %3 = load <4 x float>, ptr %2, align 16
  call void asm "", "=*imr|w,0"(ptr elementtype(<4 x float>) %2, <4 x float> %3) #4, !srcloc !12
  %4 = load <4 x float>, ptr %2, align 16
  ret <4 x float> %4
}

define linkonce_odr dso_local noundef <2 x i32> @_Z4passI11__Int32x2_tET_S1_(<2 x i32> noundef %0) #0 comdat {
  %2 = alloca <2 x i32>, align 8
  store <2 x i32> %0, ptr %2, align 8
  %3 = load <2 x i32>, ptr %2, align 8
  call void asm "", "=*imr|w,0"(ptr elementtype(<2 x i32>) %2, <2 x i32> %3) #4, !srcloc !12
  %4 = load <2 x i32>, ptr %2, align 8
  ret <2 x i32> %4
}

declare i32 @printf(ptr noundef, ...) #3

attributes #0 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #1 = { mustprogress noinline norecurse optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #2 = { mustprogress noinline optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #3 = { "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #4 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 4}
!5 = !{!"kiln version 1.1.0"}
!6 = !{i64 793}
!7 = !{i64 928}
!8 = !{i64 1151}
!9 = !{i64 1344, i64 1358, i64 1378}
!10 = !{i64 1514}
!11 = !{i64 1566}
!12 = !{i64 662}

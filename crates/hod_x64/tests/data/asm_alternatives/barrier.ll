; ModuleID = 'barrier.cpp'
source_filename = "barrier.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

$_Z4passIiET_S0_ = comdat any

$_Z4passIlET_S0_ = comdat any

$_Z4passIsET_S0_ = comdat any

$_Z4passIcET_S0_ = comdat any

$_Z4passIfET_S0_ = comdat any

$_Z4passIdET_S0_ = comdat any

$_Z4passInET_S0_ = comdat any

$_Z4passIPiET_S1_ = comdat any

$_Z4passIDv4_fET_S1_ = comdat any

$_Z4passIDv2_xET_S1_ = comdat any

@.str = private unnamed_addr constant [4 x i8] c"int\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"long\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"short\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"char\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"float\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"double\00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"int128\00", align 1
@_ZZ4mainE4cell = internal global i32 9, align 4
@.str.7 = private unnamed_addr constant [8 x i8] c"pointer\00", align 1
@.str.8 = private unnamed_addr constant [8 x i8] c"twice_f\00", align 1
@.str.9 = private unnamed_addr constant [7 x i8] c"__m128\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"pass __m128\00", align 1
@.str.11 = private unnamed_addr constant [8 x i8] c"__m128d\00", align 1
@.str.12 = private unnamed_addr constant [8 x i8] c"__m128i\00", align 1
@.str.13 = private unnamed_addr constant [7 x i8] c"__m256\00", align 1
@.str.14 = private unnamed_addr constant [6 x i8] c"inc_g\00", align 1
@.str.15 = private unnamed_addr constant [8 x i8] c"copy_rm\00", align 1
@.str.16 = private unnamed_addr constant [8 x i8] c"copy_ss\00", align 1
@.str.17 = private unnamed_addr constant [6 x i8] c"add_r\00", align 1
@.str.18 = private unnamed_addr constant [5 x i8] c"keep\00", align 1
@.str.19 = private unnamed_addr constant [22 x i8] c"checked, %d failures\0A\00", align 1
@_ZL8failures = internal global i32 0, align 4
@.str.20 = private unnamed_addr constant [11 x i8] c"FAILED %s\0A\00", align 1

define dso_local noundef float @_Z7twice_ff(float noundef %0) #0 {
  %2 = alloca float, align 4
  %3 = alloca float, align 4
  store float %0, ptr %2, align 4
  %4 = load float, ptr %2, align 4
  %5 = load float, ptr %2, align 4
  %6 = fadd float %4, %5
  store float %6, ptr %3, align 4
  %7 = load float, ptr %3, align 4
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %3, float %7) #6, !srcloc !6
  %8 = load float, ptr %3, align 4
  %9 = fadd float %8, 1.000000e+00
  ret float %9
}

define dso_local noundef <4 x float> @_Z6add_psDv4_fS_(<4 x float> noundef %0, <4 x float> noundef %1) #1 {
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
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<4 x float>) %5, <4 x float> %9) #6, !srcloc !7
  %10 = load <4 x float>, ptr %5, align 16
  %11 = load <4 x float>, ptr %3, align 16
  %12 = fmul <4 x float> %10, %11
  ret <4 x float> %12
}

define dso_local noundef <2 x double> @_Z6add_pdDv2_dS_(<2 x double> noundef %0, <2 x double> noundef %1) #1 {
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
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<2 x double>) %5, <2 x double> %9) #6, !srcloc !8
  %10 = load <2 x double>, ptr %5, align 16
  %11 = load <2 x double>, ptr %4, align 16
  %12 = fsub <2 x double> %10, %11
  ret <2 x double> %12
}

define dso_local noundef <8 x float> @_Z7add_ps8Dv8_fS_(ptr noundef byval(<8 x float>) align 32 %0, ptr noundef byval(<8 x float>) align 32 %1) #2 {
  %3 = alloca <8 x float>, align 32
  %4 = alloca <8 x float>, align 32
  %5 = alloca <8 x float>, align 32
  %6 = load <8 x float>, ptr %0, align 32
  %7 = load <8 x float>, ptr %1, align 32
  store <8 x float> %6, ptr %3, align 32
  store <8 x float> %7, ptr %4, align 32
  %8 = load <8 x float>, ptr %3, align 32
  %9 = load <8 x float>, ptr %4, align 32
  %10 = fadd <8 x float> %8, %9
  store <8 x float> %10, ptr %5, align 32
  %11 = load <8 x float>, ptr %5, align 32
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<8 x float>) %5, <8 x float> %11) #6, !srcloc !9
  %12 = load <8 x float>, ptr %5, align 32
  ret <8 x float> %12
}

define dso_local noundef float @_Z4sum8f(float noundef %0) #2 {
  %2 = alloca float, align 4
  %3 = alloca <8 x float>, align 32
  %4 = alloca <8 x float>, align 32
  %5 = alloca <8 x float>, align 32
  %6 = alloca <8 x float>, align 32
  %7 = alloca float, align 4
  %8 = alloca i32, align 4
  store float %0, ptr %2, align 4
  %9 = load float, ptr %2, align 4
  %10 = insertelement <8 x float> poison, float %9, i32 0
  %11 = load float, ptr %2, align 4
  %12 = fadd float %11, 1.000000e+00
  %13 = insertelement <8 x float> %10, float %12, i32 1
  %14 = load float, ptr %2, align 4
  %15 = fadd float %14, 2.000000e+00
  %16 = insertelement <8 x float> %13, float %15, i32 2
  %17 = load float, ptr %2, align 4
  %18 = fadd float %17, 3.000000e+00
  %19 = insertelement <8 x float> %16, float %18, i32 3
  %20 = load float, ptr %2, align 4
  %21 = fadd float %20, 4.000000e+00
  %22 = insertelement <8 x float> %19, float %21, i32 4
  %23 = load float, ptr %2, align 4
  %24 = fadd float %23, 5.000000e+00
  %25 = insertelement <8 x float> %22, float %24, i32 5
  %26 = load float, ptr %2, align 4
  %27 = fadd float %26, 6.000000e+00
  %28 = insertelement <8 x float> %25, float %27, i32 6
  %29 = load float, ptr %2, align 4
  %30 = fadd float %29, 7.000000e+00
  %31 = insertelement <8 x float> %28, float %30, i32 7
  store <8 x float> %31, ptr %3, align 32
  %32 = load <8 x float>, ptr %3, align 32
  %33 = load <8 x float>, ptr %3, align 32
  store <8 x float> %32, ptr %5, align 32
  store <8 x float> %33, ptr %6, align 32
  %34 = call noundef <8 x float> @_Z7add_ps8Dv8_fS_(ptr noundef byval(<8 x float>) align 32 %5, ptr noundef byval(<8 x float>) align 32 %6)
  store <8 x float> %34, ptr %4, align 32
  store float 0.000000e+00, ptr %7, align 4
  store i32 0, ptr %8, align 4
  br label %35

35:
  %36 = load i32, ptr %8, align 4
  %37 = icmp slt i32 %36, 8
  br i1 %37, label %38, label %47

38:
  %39 = load <8 x float>, ptr %4, align 32
  %40 = load i32, ptr %8, align 4
  %41 = extractelement <8 x float> %39, i32 %40
  %42 = load float, ptr %7, align 4
  %43 = fadd float %42, %41
  store float %43, ptr %7, align 4
  br label %44

44:
  %45 = load i32, ptr %8, align 4
  %46 = add nsw i32 %45, 1
  store i32 %46, ptr %8, align 4
  br label %35, !llvm.loop !10

47:
  %48 = load float, ptr %7, align 4
  ret float %48
}

define dso_local noundef i32 @_Z5inc_gi(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  call void asm "incl $0", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %2, i32 %3) #6, !srcloc !12
  %4 = load i32, ptr %2, align 4
  ret i32 %4
}

define dso_local noundef i32 @_Z7copy_rmi(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  store i32 -1, ptr %3, align 4
  %4 = load i32, ptr %2, align 4
  call void asm "movl $1, $0", "=*r|m,ri|r,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %3, i32 %4) #6, !srcloc !13
  %5 = load i32, ptr %3, align 4
  ret i32 %5
}

define dso_local noundef float @_Z7copy_ssf(float noundef %0) #0 {
  %2 = alloca float, align 4
  %3 = alloca float, align 4
  store float %0, ptr %2, align 4
  store float -1.000000e+00, ptr %3, align 4
  %4 = load float, ptr %2, align 4
  call void asm "movss $1, $0", "=*x|m,x|x,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %3, float %4) #6, !srcloc !14
  %5 = load float, ptr %3, align 4
  ret float %5
}

define dso_local noundef i64 @_Z5add_rll(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = load i64, ptr %4, align 8
  call void asm "addq $1, $0", "=*r|m,r|r,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i64) %3, i64 %6, i64 %5) #6, !srcloc !15
  %7 = load i64, ptr %3, align 8
  ret i64 %7
}

define dso_local noundef double @_Z4keepd(double noundef %0) #0 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
  %3 = load double, ptr %2, align 8
  call void asm sideeffect "", "r|m,~{memory},~{dirflag},~{fpsr},~{flags}"(double %3) #6, !srcloc !16
  %4 = load double, ptr %2, align 8
  call void asm sideeffect "", "x|m,~{memory},~{dirflag},~{fpsr},~{flags}"(double %4) #6, !srcloc !17
  %5 = load double, ptr %2, align 8
  %6 = fmul double %5, 2.000000e+00
  ret double %6
}

define dso_local noundef i32 @main() #3 {
  %1 = alloca i32, align 4
  %2 = alloca <4 x float>, align 16
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  %6 = alloca <2 x double>, align 16
  %7 = alloca <2 x i64>, align 16
  store i32 0, ptr %1, align 4
  %8 = call noundef i32 @_Z4passIiET_S0_(i32 noundef 41)
  %9 = icmp eq i32 %8, 41
  call void @_ZL5checkbPKc(i1 noundef zeroext %9, ptr noundef @.str)
  %10 = call noundef i64 @_Z4passIlET_S0_(i64 noundef -5)
  %11 = icmp eq i64 %10, -5
  call void @_ZL5checkbPKc(i1 noundef zeroext %11, ptr noundef @.str.1)
  %12 = call noundef signext i16 @_Z4passIsET_S0_(i16 noundef signext -3)
  %13 = sext i16 %12 to i32
  %14 = icmp eq i32 %13, -3
  call void @_ZL5checkbPKc(i1 noundef zeroext %14, ptr noundef @.str.2)
  %15 = call noundef signext i8 @_Z4passIcET_S0_(i8 noundef signext 97)
  %16 = sext i8 %15 to i32
  %17 = icmp eq i32 %16, 97
  call void @_ZL5checkbPKc(i1 noundef zeroext %17, ptr noundef @.str.3)
  %18 = call noundef float @_Z4passIfET_S0_(float noundef 2.500000e+00)
  %19 = fcmp oeq float %18, 2.500000e+00
  call void @_ZL5checkbPKc(i1 noundef zeroext %19, ptr noundef @.str.4)
  %20 = call noundef double @_Z4passIdET_S0_(double noundef -1.250000e-01)
  %21 = fcmp oeq double %20, -1.250000e-01
  call void @_ZL5checkbPKc(i1 noundef zeroext %21, ptr noundef @.str.5)
  %22 = call noundef i128 @_Z4passInET_S0_(i128 noundef 1267650600228229401496703205376)
  %23 = icmp eq i128 %22, 1267650600228229401496703205376
  call void @_ZL5checkbPKc(i1 noundef zeroext %23, ptr noundef @.str.6)
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
  call void @_ZL5checkbPKc(i1 noundef zeroext %31, ptr noundef @.str.7)
  %32 = call noundef float @_Z7twice_ff(float noundef 3.000000e+00)
  %33 = fcmp oeq float %32, 7.000000e+00
  call void @_ZL5checkbPKc(i1 noundef zeroext %33, ptr noundef @.str.8)
  store <4 x float> <float 1.000000e+00, float 2.000000e+00, float 3.000000e+00, float 4.000000e+00>, ptr %2, align 16
  store <4 x float> <float 1.000000e+01, float 2.000000e+01, float 3.000000e+01, float 4.000000e+01>, ptr %3, align 16
  %34 = load <4 x float>, ptr %2, align 16
  %35 = load <4 x float>, ptr %3, align 16
  %36 = call noundef <4 x float> @_Z6add_psDv4_fS_(<4 x float> noundef %34, <4 x float> noundef %35)
  store <4 x float> %36, ptr %4, align 16
  %37 = load <4 x float>, ptr %4, align 16
  %38 = extractelement <4 x float> %37, i32 0
  %39 = fcmp oeq float %38, 1.100000e+01
  br i1 %39, label %40, label %52

40:
  %41 = load <4 x float>, ptr %4, align 16
  %42 = extractelement <4 x float> %41, i32 1
  %43 = fcmp oeq float %42, 4.400000e+01
  br i1 %43, label %44, label %52

44:
  %45 = load <4 x float>, ptr %4, align 16
  %46 = extractelement <4 x float> %45, i32 2
  %47 = fcmp oeq float %46, 9.900000e+01
  br i1 %47, label %48, label %52

48:
  %49 = load <4 x float>, ptr %4, align 16
  %50 = extractelement <4 x float> %49, i32 3
  %51 = fcmp oeq float %50, 1.760000e+02
  br label %52

52:
  %53 = phi i1 [ false, %44 ], [ false, %40 ], [ false, %30 ], [ %51, %48 ]
  call void @_ZL5checkbPKc(i1 noundef zeroext %53, ptr noundef @.str.9)
  %54 = load <4 x float>, ptr %2, align 16
  %55 = call noundef <4 x float> @_Z4passIDv4_fET_S1_(<4 x float> noundef %54)
  store <4 x float> %55, ptr %5, align 16
  %56 = load <4 x float>, ptr %5, align 16
  %57 = extractelement <4 x float> %56, i32 0
  %58 = fcmp oeq float %57, 1.000000e+00
  br i1 %58, label %59, label %63

59:
  %60 = load <4 x float>, ptr %5, align 16
  %61 = extractelement <4 x float> %60, i32 3
  %62 = fcmp oeq float %61, 4.000000e+00
  br label %63

63:
  %64 = phi i1 [ false, %52 ], [ %62, %59 ]
  call void @_ZL5checkbPKc(i1 noundef zeroext %64, ptr noundef @.str.10)
  %65 = call noundef <2 x double> @_Z6add_pdDv2_dS_(<2 x double> noundef <double 1.500000e+00, double -2.000000e+00>, <2 x double> noundef <double 4.000000e+00, double 8.000000e+00>)
  store <2 x double> %65, ptr %6, align 16
  %66 = load <2 x double>, ptr %6, align 16
  %67 = extractelement <2 x double> %66, i32 0
  %68 = fcmp oeq double %67, 1.500000e+00
  br i1 %68, label %69, label %73

69:
  %70 = load <2 x double>, ptr %6, align 16
  %71 = extractelement <2 x double> %70, i32 1
  %72 = fcmp oeq double %71, -2.000000e+00
  br label %73

73:
  %74 = phi i1 [ false, %63 ], [ %72, %69 ]
  call void @_ZL5checkbPKc(i1 noundef zeroext %74, ptr noundef @.str.11)
  %75 = call noundef <2 x i64> @_Z4passIDv2_xET_S1_(<2 x i64> noundef <i64 7, i64 -9>)
  store <2 x i64> %75, ptr %7, align 16
  %76 = load <2 x i64>, ptr %7, align 16
  %77 = extractelement <2 x i64> %76, i32 0
  %78 = icmp eq i64 %77, 7
  br i1 %78, label %79, label %83

79:
  %80 = load <2 x i64>, ptr %7, align 16
  %81 = extractelement <2 x i64> %80, i32 1
  %82 = icmp eq i64 %81, -9
  br label %83

83:
  %84 = phi i1 [ false, %73 ], [ %82, %79 ]
  call void @_ZL5checkbPKc(i1 noundef zeroext %84, ptr noundef @.str.12)
  %85 = call noundef float @_Z4sum8f(float noundef 1.000000e+00)
  %86 = fcmp oeq float %85, 7.200000e+01
  call void @_ZL5checkbPKc(i1 noundef zeroext %86, ptr noundef @.str.13)
  %87 = call noundef i32 @_Z5inc_gi(i32 noundef 41)
  %88 = icmp eq i32 %87, 42
  call void @_ZL5checkbPKc(i1 noundef zeroext %88, ptr noundef @.str.14)
  %89 = call noundef i32 @_Z7copy_rmi(i32 noundef 1234)
  %90 = icmp eq i32 %89, 1234
  call void @_ZL5checkbPKc(i1 noundef zeroext %90, ptr noundef @.str.15)
  %91 = call noundef float @_Z7copy_ssf(float noundef 6.500000e+00)
  %92 = fcmp oeq float %91, 6.500000e+00
  call void @_ZL5checkbPKc(i1 noundef zeroext %92, ptr noundef @.str.16)
  %93 = call noundef i64 @_Z5add_rll(i64 noundef 40, i64 noundef 2)
  %94 = icmp eq i64 %93, 42
  call void @_ZL5checkbPKc(i1 noundef zeroext %94, ptr noundef @.str.17)
  %95 = call noundef double @_Z4keepd(double noundef 1.250000e+00)
  %96 = fcmp oeq double %95, 2.500000e+00
  call void @_ZL5checkbPKc(i1 noundef zeroext %96, ptr noundef @.str.18)
  %97 = load i32, ptr @_ZL8failures, align 4
  %98 = call i32 (ptr, ...) @printf(ptr noundef @.str.19, i32 noundef %97)
  %99 = load i32, ptr @_ZL8failures, align 4
  %100 = icmp ne i32 %99, 0
  %101 = zext i1 %100 to i32
  ret i32 %101
}

define internal void @_ZL5checkbPKc(i1 noundef zeroext %0, ptr noundef %1) #4 {
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
  %12 = call i32 (ptr, ...) @printf(ptr noundef @.str.20, ptr noundef %11)
  br label %13

13:
  ret void
}

define linkonce_odr dso_local noundef i32 @_Z4passIiET_S0_(i32 noundef %0) #0 comdat {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %2, i32 %3) #6, !srcloc !18
  %4 = load i32, ptr %2, align 4
  ret i32 %4
}

define linkonce_odr dso_local noundef i64 @_Z4passIlET_S0_(i64 noundef %0) #0 comdat {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i64) %2, i64 %3) #6, !srcloc !18
  %4 = load i64, ptr %2, align 8
  ret i64 %4
}

define linkonce_odr dso_local noundef signext i16 @_Z4passIsET_S0_(i16 noundef signext %0) #0 comdat {
  %2 = alloca i16, align 2
  store i16 %0, ptr %2, align 2
  %3 = load i16, ptr %2, align 2
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i16) %2, i16 %3) #6, !srcloc !18
  %4 = load i16, ptr %2, align 2
  ret i16 %4
}

define linkonce_odr dso_local noundef signext i8 @_Z4passIcET_S0_(i8 noundef signext %0) #0 comdat {
  %2 = alloca i8, align 1
  store i8 %0, ptr %2, align 1
  %3 = load i8, ptr %2, align 1
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i8) %2, i8 %3) #6, !srcloc !18
  %4 = load i8, ptr %2, align 1
  ret i8 %4
}

define linkonce_odr dso_local noundef float @_Z4passIfET_S0_(float noundef %0) #0 comdat {
  %2 = alloca float, align 4
  store float %0, ptr %2, align 4
  %3 = load float, ptr %2, align 4
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %2, float %3) #6, !srcloc !18
  %4 = load float, ptr %2, align 4
  ret float %4
}

define linkonce_odr dso_local noundef double @_Z4passIdET_S0_(double noundef %0) #0 comdat {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
  %3 = load double, ptr %2, align 8
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(double) %2, double %3) #6, !srcloc !18
  %4 = load double, ptr %2, align 8
  ret double %4
}

define linkonce_odr dso_local noundef i128 @_Z4passInET_S0_(i128 noundef %0) #0 comdat {
  %2 = alloca i128, align 16
  store i128 %0, ptr %2, align 16
  %3 = load i128, ptr %2, align 16
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i128) %2, i128 %3) #6, !srcloc !18
  %4 = load i128, ptr %2, align 16
  ret i128 %4
}

define linkonce_odr dso_local noundef ptr @_Z4passIPiET_S1_(ptr noundef %0) #0 comdat {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(ptr) %2, ptr %3) #6, !srcloc !18
  %4 = load ptr, ptr %2, align 8
  ret ptr %4
}

define linkonce_odr dso_local noundef <4 x float> @_Z4passIDv4_fET_S1_(<4 x float> noundef %0) #1 comdat {
  %2 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %2, align 16
  %3 = load <4 x float>, ptr %2, align 16
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<4 x float>) %2, <4 x float> %3) #6, !srcloc !18
  %4 = load <4 x float>, ptr %2, align 16
  ret <4 x float> %4
}

define linkonce_odr dso_local noundef <2 x i64> @_Z4passIDv2_xET_S1_(<2 x i64> noundef %0) #1 comdat {
  %2 = alloca <2 x i64>, align 16
  store <2 x i64> %0, ptr %2, align 16
  %3 = load <2 x i64>, ptr %2, align 16
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<2 x i64>) %2, <2 x i64> %3) #6, !srcloc !18
  %4 = load <2 x i64>, ptr %2, align 16
  ret <2 x i64> %4
}

declare i32 @printf(ptr noundef, ...) #5

attributes #0 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="128" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="256" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+avx,+cmov,+crc32,+cx8,+fxsr,+mmx,+popcnt,+sse,+sse2,+sse3,+sse4.1,+sse4.2,+ssse3,+x87,+xsave" "tune-cpu"="generic" }
attributes #3 = { mustprogress noinline norecurse optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="128" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress noinline optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"kiln version 1.1.0"}
!6 = !{i64 759}
!7 = !{i64 877}
!8 = !{i64 996}
!9 = !{i64 1127}
!10 = distinct !{!10, !11}
!11 = !{!"llvm.loop.mustprogress"}
!12 = !{i64 1558}
!13 = !{i64 1806}
!14 = !{i64 2009}
!15 = !{i64 2317}
!16 = !{i64 2480}
!17 = !{i64 2532}
!18 = !{i64 660}

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
@_ZZ4mainE4cell = internal global i32 9, align 4, !dbg !0
@.str.7 = private unnamed_addr constant [8 x i8] c"pointer\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"float32x4_t\00", align 1
@.str.9 = private unnamed_addr constant [17 x i8] c"pass float32x4_t\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"float64x2_t\00", align 1
@.str.11 = private unnamed_addr constant [10 x i8] c"int32x2_t\00", align 1
@.str.12 = private unnamed_addr constant [8 x i8] c"copy_rm\00", align 1
@.str.13 = private unnamed_addr constant [6 x i8] c"add_m\00", align 1
@.str.14 = private unnamed_addr constant [5 x i8] c"keep\00", align 1
@.str.15 = private unnamed_addr constant [22 x i8] c"checked, %d failures\0A\00", align 1
@_ZL8failures = internal global i32 0, align 4, !dbg !25
@.str.16 = private unnamed_addr constant [11 x i8] c"FAILED %s\0A\00", align 1

define dso_local noundef <4 x float> @_Z6add_ps13__Float32x4_tS_(<4 x float> noundef %0, <4 x float> noundef %1) #0 !dbg !36 {
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %3, align 16
    #dbg_declare(ptr %3, !45, !DIExpression(), !46)
  store <4 x float> %1, ptr %4, align 16
    #dbg_declare(ptr %4, !47, !DIExpression(), !48)
    #dbg_declare(ptr %5, !49, !DIExpression(), !50)
  %6 = load <4 x float>, ptr %3, align 16, !dbg !51
  %7 = load <4 x float>, ptr %4, align 16, !dbg !52
  %8 = fadd <4 x float> %6, %7, !dbg !53
  store <4 x float> %8, ptr %5, align 16, !dbg !50
  %9 = load <4 x float>, ptr %5, align 16, !dbg !54
  call void asm "", "=*imr|w,0"(ptr elementtype(<4 x float>) %5, <4 x float> %9) #4, !dbg !54, !srcloc !55
  %10 = load <4 x float>, ptr %5, align 16, !dbg !56
  %11 = load <4 x float>, ptr %3, align 16, !dbg !57
  %12 = fmul <4 x float> %10, %11, !dbg !58
  ret <4 x float> %12, !dbg !59
}

define dso_local noundef <2 x double> @_Z6add_pd13__Float64x2_tS_(<2 x double> noundef %0, <2 x double> noundef %1) #0 !dbg !60 {
  %3 = alloca <2 x double>, align 16
  %4 = alloca <2 x double>, align 16
  %5 = alloca <2 x double>, align 16
  store <2 x double> %0, ptr %3, align 16
    #dbg_declare(ptr %3, !63, !DIExpression(), !64)
  store <2 x double> %1, ptr %4, align 16
    #dbg_declare(ptr %4, !65, !DIExpression(), !66)
    #dbg_declare(ptr %5, !67, !DIExpression(), !68)
  %6 = load <2 x double>, ptr %3, align 16, !dbg !69
  %7 = load <2 x double>, ptr %4, align 16, !dbg !70
  %8 = fadd <2 x double> %6, %7, !dbg !71
  store <2 x double> %8, ptr %5, align 16, !dbg !68
  %9 = load <2 x double>, ptr %5, align 16, !dbg !72
  call void asm "", "=*imr|w,0"(ptr elementtype(<2 x double>) %5, <2 x double> %9) #4, !dbg !72, !srcloc !73
  %10 = load <2 x double>, ptr %5, align 16, !dbg !74
  %11 = load <2 x double>, ptr %4, align 16, !dbg !75
  %12 = fsub <2 x double> %10, %11, !dbg !76
  ret <2 x double> %12, !dbg !77
}

define dso_local noundef i64 @_Z7copy_rml(i64 noundef %0) #0 !dbg !78 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
    #dbg_declare(ptr %2, !82, !DIExpression(), !83)
    #dbg_declare(ptr %3, !84, !DIExpression(), !85)
  store i64 -1, ptr %3, align 8, !dbg !85
  %4 = load i64, ptr %2, align 8, !dbg !86
  call void asm "str $1, $0", "=*r|m,r|r"(ptr elementtype(i64) %3, i64 %4) #4, !dbg !87, !srcloc !88
  %5 = load i64, ptr %3, align 8, !dbg !89
  ret i64 %5, !dbg !90
}

define dso_local noundef i64 @_Z5add_mll(i64 noundef %0, i64 noundef %1) #0 !dbg !91 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
    #dbg_declare(ptr %3, !94, !DIExpression(), !95)
  store i64 %1, ptr %4, align 8
    #dbg_declare(ptr %4, !96, !DIExpression(), !97)
  %5 = load i64, ptr %3, align 8, !dbg !98
  %6 = load i64, ptr %4, align 8, !dbg !99
  call void asm "ldr x16, $0\0A\09add x16, x16, $1\0A\09str x16, $0", "=*r|m,r|r,0,~{x16}"(ptr elementtype(i64) %3, i64 %6, i64 %5) #4, !dbg !98, !srcloc !100
  %7 = load i64, ptr %3, align 8, !dbg !101
  ret i64 %7, !dbg !102
}

define dso_local noundef double @_Z4keepd(double noundef %0) #0 !dbg !103 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
    #dbg_declare(ptr %2, !106, !DIExpression(), !107)
  %3 = load double, ptr %2, align 8, !dbg !108
  call void asm sideeffect "", "r|m,~{memory}"(double %3) #4, !dbg !109, !srcloc !110
  %4 = load double, ptr %2, align 8, !dbg !111
  call void asm sideeffect "", "w|m,~{memory}"(double %4) #4, !dbg !112, !srcloc !113
  %5 = load double, ptr %2, align 8, !dbg !114
  %6 = fmul double %5, 2.000000e+00, !dbg !115
  ret double %6, !dbg !116
}

define dso_local noundef i32 @main() #1 !dbg !2 {
  %1 = alloca i32, align 4
  %2 = alloca <4 x float>, align 16
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  %6 = alloca <2 x double>, align 16
  %7 = alloca <2 x i32>, align 8
  store i32 0, ptr %1, align 4
  %8 = call noundef i32 @_Z4passIiET_S0_(i32 noundef 41), !dbg !117
  %9 = icmp eq i32 %8, 41, !dbg !118
  call void @_ZL5checkbPKc(i1 noundef %9, ptr noundef @.str), !dbg !119
  %10 = call noundef i64 @_Z4passIlET_S0_(i64 noundef -5), !dbg !120
  %11 = icmp eq i64 %10, -5, !dbg !121
  call void @_ZL5checkbPKc(i1 noundef %11, ptr noundef @.str.1), !dbg !122
  %12 = call noundef i16 @_Z4passIsET_S0_(i16 noundef -3), !dbg !123
  %13 = sext i16 %12 to i32, !dbg !123
  %14 = icmp eq i32 %13, -3, !dbg !124
  call void @_ZL5checkbPKc(i1 noundef %14, ptr noundef @.str.2), !dbg !125
  %15 = call noundef i8 @_Z4passIcET_S0_(i8 noundef 97), !dbg !126
  %16 = zext i8 %15 to i32, !dbg !126
  %17 = icmp eq i32 %16, 97, !dbg !127
  call void @_ZL5checkbPKc(i1 noundef %17, ptr noundef @.str.3), !dbg !128
  %18 = call noundef float @_Z4passIfET_S0_(float noundef 2.500000e+00), !dbg !129
  %19 = fcmp oeq float %18, 2.500000e+00, !dbg !130
  call void @_ZL5checkbPKc(i1 noundef %19, ptr noundef @.str.4), !dbg !131
  %20 = call noundef double @_Z4passIdET_S0_(double noundef -1.250000e-01), !dbg !132
  %21 = fcmp oeq double %20, -1.250000e-01, !dbg !133
  call void @_ZL5checkbPKc(i1 noundef %21, ptr noundef @.str.5), !dbg !134
  %22 = call noundef i128 @_Z4passInET_S0_(i128 noundef 1267650600228229401496703205376), !dbg !135
  %23 = icmp eq i128 %22, 1267650600228229401496703205376, !dbg !136
  call void @_ZL5checkbPKc(i1 noundef %23, ptr noundef @.str.6), !dbg !137
  %24 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell), !dbg !138
  %25 = icmp eq ptr %24, @_ZZ4mainE4cell, !dbg !139
  br i1 %25, label %26, label %30, !dbg !140

26:
  %27 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell), !dbg !141
  %28 = load i32, ptr %27, align 4, !dbg !142
  %29 = icmp eq i32 %28, 9, !dbg !143
  br label %30

30:
  %31 = phi i1 [ false, %0 ], [ %29, %26 ], !dbg !144
  call void @_ZL5checkbPKc(i1 noundef %31, ptr noundef @.str.7), !dbg !145
    #dbg_declare(ptr %2, !146, !DIExpression(), !147)
  store <4 x float> <float 1.000000e+00, float 2.000000e+00, float 3.000000e+00, float 4.000000e+00>, ptr %2, align 16, !dbg !147
    #dbg_declare(ptr %3, !148, !DIExpression(), !149)
  store <4 x float> <float 1.000000e+01, float 2.000000e+01, float 3.000000e+01, float 4.000000e+01>, ptr %3, align 16, !dbg !149
    #dbg_declare(ptr %4, !150, !DIExpression(), !151)
  %32 = load <4 x float>, ptr %2, align 16, !dbg !152
  %33 = load <4 x float>, ptr %3, align 16, !dbg !153
  %34 = call noundef <4 x float> @_Z6add_ps13__Float32x4_tS_(<4 x float> noundef %32, <4 x float> noundef %33), !dbg !154
  store <4 x float> %34, ptr %4, align 16, !dbg !151
  %35 = load <4 x float>, ptr %4, align 16, !dbg !155
  %36 = extractelement <4 x float> %35, i32 0, !dbg !155
  %37 = fcmp oeq float %36, 1.100000e+01, !dbg !156
  br i1 %37, label %38, label %50, !dbg !157

38:
  %39 = load <4 x float>, ptr %4, align 16, !dbg !158
  %40 = extractelement <4 x float> %39, i32 1, !dbg !158
  %41 = fcmp oeq float %40, 4.400000e+01, !dbg !159
  br i1 %41, label %42, label %50, !dbg !157

42:
  %43 = load <4 x float>, ptr %4, align 16, !dbg !160
  %44 = extractelement <4 x float> %43, i32 2, !dbg !160
  %45 = fcmp oeq float %44, 9.900000e+01, !dbg !161
  br i1 %45, label %46, label %50, !dbg !157

46:
  %47 = load <4 x float>, ptr %4, align 16, !dbg !162
  %48 = extractelement <4 x float> %47, i32 3, !dbg !162
  %49 = fcmp oeq float %48, 1.760000e+02, !dbg !163
  br label %50

50:
  %51 = phi i1 [ false, %42 ], [ false, %38 ], [ false, %30 ], [ %49, %46 ], !dbg !144
  call void @_ZL5checkbPKc(i1 noundef %51, ptr noundef @.str.8), !dbg !164
    #dbg_declare(ptr %5, !165, !DIExpression(), !166)
  %52 = load <4 x float>, ptr %2, align 16, !dbg !167
  %53 = call noundef <4 x float> @_Z4passI13__Float32x4_tET_S1_(<4 x float> noundef %52), !dbg !168
  store <4 x float> %53, ptr %5, align 16, !dbg !166
  %54 = load <4 x float>, ptr %5, align 16, !dbg !169
  %55 = extractelement <4 x float> %54, i32 0, !dbg !169
  %56 = fcmp oeq float %55, 1.000000e+00, !dbg !170
  br i1 %56, label %57, label %61, !dbg !171

57:
  %58 = load <4 x float>, ptr %5, align 16, !dbg !172
  %59 = extractelement <4 x float> %58, i32 3, !dbg !172
  %60 = fcmp oeq float %59, 4.000000e+00, !dbg !173
  br label %61

61:
  %62 = phi i1 [ false, %50 ], [ %60, %57 ], !dbg !144
  call void @_ZL5checkbPKc(i1 noundef %62, ptr noundef @.str.9), !dbg !174
    #dbg_declare(ptr %6, !175, !DIExpression(), !176)
  %63 = call noundef <2 x double> @_Z6add_pd13__Float64x2_tS_(<2 x double> noundef <double 1.500000e+00, double -2.000000e+00>, <2 x double> noundef <double 4.000000e+00, double 8.000000e+00>), !dbg !177
  store <2 x double> %63, ptr %6, align 16, !dbg !176
  %64 = load <2 x double>, ptr %6, align 16, !dbg !178
  %65 = extractelement <2 x double> %64, i32 0, !dbg !178
  %66 = fcmp oeq double %65, 1.500000e+00, !dbg !179
  br i1 %66, label %67, label %71, !dbg !180

67:
  %68 = load <2 x double>, ptr %6, align 16, !dbg !181
  %69 = extractelement <2 x double> %68, i32 1, !dbg !181
  %70 = fcmp oeq double %69, -2.000000e+00, !dbg !182
  br label %71

71:
  %72 = phi i1 [ false, %61 ], [ %70, %67 ], !dbg !144
  call void @_ZL5checkbPKc(i1 noundef %72, ptr noundef @.str.10), !dbg !183
    #dbg_declare(ptr %7, !184, !DIExpression(), !185)
  %73 = call noundef <2 x i32> @_Z4passI11__Int32x2_tET_S1_(<2 x i32> noundef <i32 7, i32 -9>), !dbg !186
  store <2 x i32> %73, ptr %7, align 8, !dbg !185
  %74 = load <2 x i32>, ptr %7, align 8, !dbg !187
  %75 = extractelement <2 x i32> %74, i32 0, !dbg !187
  %76 = icmp eq i32 %75, 7, !dbg !188
  br i1 %76, label %77, label %81, !dbg !189

77:
  %78 = load <2 x i32>, ptr %7, align 8, !dbg !190
  %79 = extractelement <2 x i32> %78, i32 1, !dbg !190
  %80 = icmp eq i32 %79, -9, !dbg !191
  br label %81

81:
  %82 = phi i1 [ false, %71 ], [ %80, %77 ], !dbg !144
  call void @_ZL5checkbPKc(i1 noundef %82, ptr noundef @.str.11), !dbg !192
  %83 = call noundef i64 @_Z7copy_rml(i64 noundef 1234), !dbg !193
  %84 = icmp eq i64 %83, 1234, !dbg !194
  call void @_ZL5checkbPKc(i1 noundef %84, ptr noundef @.str.12), !dbg !195
  %85 = call noundef i64 @_Z5add_mll(i64 noundef 40, i64 noundef 2), !dbg !196
  %86 = icmp eq i64 %85, 42, !dbg !197
  call void @_ZL5checkbPKc(i1 noundef %86, ptr noundef @.str.13), !dbg !198
  %87 = call noundef double @_Z4keepd(double noundef 1.250000e+00), !dbg !199
  %88 = fcmp oeq double %87, 2.500000e+00, !dbg !200
  call void @_ZL5checkbPKc(i1 noundef %88, ptr noundef @.str.14), !dbg !201
  %89 = load i32, ptr @_ZL8failures, align 4, !dbg !202
  %90 = call i32 (ptr, ...) @printf(ptr noundef @.str.15, i32 noundef %89), !dbg !203
  %91 = load i32, ptr @_ZL8failures, align 4, !dbg !204
  %92 = icmp ne i32 %91, 0, !dbg !205
  %93 = zext i1 %92 to i32, !dbg !204
  ret i32 %93, !dbg !206
}

define internal void @_ZL5checkbPKc(i1 noundef %0, ptr noundef %1) #2 !dbg !207 {
  %3 = alloca i8, align 1
  %4 = alloca ptr, align 8
  %5 = zext i1 %0 to i8
  store i8 %5, ptr %3, align 1
    #dbg_declare(ptr %3, !214, !DIExpression(), !215)
  store ptr %1, ptr %4, align 8
    #dbg_declare(ptr %4, !216, !DIExpression(), !217)
  %6 = load i8, ptr %3, align 1, !dbg !218
  %7 = trunc i8 %6 to i1, !dbg !218
  br i1 %7, label %13, label %8, !dbg !220

8:
  %9 = load i32, ptr @_ZL8failures, align 4, !dbg !221
  %10 = add nsw i32 %9, 1, !dbg !221
  store i32 %10, ptr @_ZL8failures, align 4, !dbg !221
  %11 = load ptr, ptr %4, align 8, !dbg !223
  %12 = call i32 (ptr, ...) @printf(ptr noundef @.str.16, ptr noundef %11), !dbg !224
  br label %13, !dbg !225

13:
  ret void, !dbg !226
}

define linkonce_odr dso_local noundef i32 @_Z4passIiET_S0_(i32 noundef %0) #0 comdat !dbg !227 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
    #dbg_declare(ptr %2, !232, !DIExpression(), !233)
  %3 = load i32, ptr %2, align 4, !dbg !234
  call void asm "", "=*imr|w,0"(ptr elementtype(i32) %2, i32 %3) #4, !dbg !234, !srcloc !235
  %4 = load i32, ptr %2, align 4, !dbg !236
  ret i32 %4, !dbg !237
}

define linkonce_odr dso_local noundef i64 @_Z4passIlET_S0_(i64 noundef %0) #0 comdat !dbg !238 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
    #dbg_declare(ptr %2, !241, !DIExpression(), !242)
  %3 = load i64, ptr %2, align 8, !dbg !243
  call void asm "", "=*imr|w,0"(ptr elementtype(i64) %2, i64 %3) #4, !dbg !243, !srcloc !235
  %4 = load i64, ptr %2, align 8, !dbg !244
  ret i64 %4, !dbg !245
}

define linkonce_odr dso_local noundef i16 @_Z4passIsET_S0_(i16 noundef %0) #0 comdat !dbg !246 {
  %2 = alloca i16, align 2
  store i16 %0, ptr %2, align 2
    #dbg_declare(ptr %2, !251, !DIExpression(), !252)
  %3 = load i16, ptr %2, align 2, !dbg !253
  call void asm "", "=*imr|w,0"(ptr elementtype(i16) %2, i16 %3) #4, !dbg !253, !srcloc !235
  %4 = load i16, ptr %2, align 2, !dbg !254
  ret i16 %4, !dbg !255
}

define linkonce_odr dso_local noundef i8 @_Z4passIcET_S0_(i8 noundef %0) #0 comdat !dbg !256 {
  %2 = alloca i8, align 1
  store i8 %0, ptr %2, align 1
    #dbg_declare(ptr %2, !261, !DIExpression(), !262)
  %3 = load i8, ptr %2, align 1, !dbg !263
  call void asm "", "=*imr|w,0"(ptr elementtype(i8) %2, i8 %3) #4, !dbg !263, !srcloc !235
  %4 = load i8, ptr %2, align 1, !dbg !264
  ret i8 %4, !dbg !265
}

define linkonce_odr dso_local noundef float @_Z4passIfET_S0_(float noundef %0) #0 comdat !dbg !266 {
  %2 = alloca float, align 4
  store float %0, ptr %2, align 4
    #dbg_declare(ptr %2, !271, !DIExpression(), !272)
  %3 = load float, ptr %2, align 4, !dbg !273
  call void asm "", "=*imr|w,0"(ptr elementtype(float) %2, float %3) #4, !dbg !273, !srcloc !235
  %4 = load float, ptr %2, align 4, !dbg !274
  ret float %4, !dbg !275
}

define linkonce_odr dso_local noundef double @_Z4passIdET_S0_(double noundef %0) #0 comdat !dbg !276 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
    #dbg_declare(ptr %2, !279, !DIExpression(), !280)
  %3 = load double, ptr %2, align 8, !dbg !281
  call void asm "", "=*imr|w,0"(ptr elementtype(double) %2, double %3) #4, !dbg !281, !srcloc !235
  %4 = load double, ptr %2, align 8, !dbg !282
  ret double %4, !dbg !283
}

define linkonce_odr dso_local noundef i128 @_Z4passInET_S0_(i128 noundef %0) #0 comdat !dbg !284 {
  %2 = alloca i128, align 16
  store i128 %0, ptr %2, align 16
    #dbg_declare(ptr %2, !289, !DIExpression(), !290)
  %3 = load i128, ptr %2, align 16, !dbg !291
  call void asm "", "=*imr|w,0"(ptr elementtype(i128) %2, i128 %3) #4, !dbg !291, !srcloc !235
  %4 = load i128, ptr %2, align 16, !dbg !292
  ret i128 %4, !dbg !293
}

define linkonce_odr dso_local noundef ptr @_Z4passIPiET_S1_(ptr noundef %0) #0 comdat !dbg !294 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
    #dbg_declare(ptr %2, !300, !DIExpression(), !301)
  %3 = load ptr, ptr %2, align 8, !dbg !302
  call void asm "", "=*imr|w,0"(ptr elementtype(ptr) %2, ptr %3) #4, !dbg !302, !srcloc !235
  %4 = load ptr, ptr %2, align 8, !dbg !303
  ret ptr %4, !dbg !304
}

define linkonce_odr dso_local noundef <4 x float> @_Z4passI13__Float32x4_tET_S1_(<4 x float> noundef %0) #0 comdat !dbg !305 {
  %2 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %2, align 16
    #dbg_declare(ptr %2, !311, !DIExpression(), !312)
  %3 = load <4 x float>, ptr %2, align 16, !dbg !313
  call void asm "", "=*imr|w,0"(ptr elementtype(<4 x float>) %2, <4 x float> %3) #4, !dbg !313, !srcloc !235
  %4 = load <4 x float>, ptr %2, align 16, !dbg !314
  ret <4 x float> %4, !dbg !315
}

define linkonce_odr dso_local noundef <2 x i32> @_Z4passI11__Int32x2_tET_S1_(<2 x i32> noundef %0) #0 comdat !dbg !316 {
  %2 = alloca <2 x i32>, align 8
  store <2 x i32> %0, ptr %2, align 8
    #dbg_declare(ptr %2, !322, !DIExpression(), !323)
  %3 = load <2 x i32>, ptr %2, align 8, !dbg !324
  call void asm "", "=*imr|w,0"(ptr elementtype(<2 x i32>) %2, <2 x i32> %3) #4, !dbg !324, !srcloc !235
  %4 = load <2 x i32>, ptr %2, align 8, !dbg !325
  ret <2 x i32> %4, !dbg !326
}

declare i32 @printf(ptr noundef, ...) #3

attributes #0 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #1 = { mustprogress noinline norecurse optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #2 = { mustprogress noinline optnone uwtable "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #3 = { "frame-pointer"="non-leaf-no-reserve" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="generic" "target-features"="+fp-armv8,+neon,+outline-atomics,+v8a,-fmv" }
attributes #4 = { nounwind }

!llvm.dbg.cu = !{!7}
!llvm.module.flags = !{!28, !29, !30, !31, !32, !33, !34}
!llvm.ident = !{!35}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "cell", scope: !2, file: !3, line: 66, type: !6, isLocal: true, isDefinition: true)
!2 = distinct !DISubprogram(name: "main", scope: !3, file: !3, line: 58, type: !4, scopeLine: 58, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!3 = !DIFile(filename: "barrier.cpp", directory: "/src/.build/target-wp-219/probe/a64", checksumkind: CSK_MD5, checksum: "833863015dcf4ae18c40b27692cf957e")
!4 = !DISubroutineType(types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!7 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !3, producer: "kiln version 1.1.0", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None, retainedTypes: !8, globals: !24)
!8 = !{!9, !10, !11, !18}
!9 = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
!10 = !DIBasicType(name: "__int128", size: 128, encoding: DW_ATE_signed)
!11 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !12, name: "float64x2_t", file: !15, line: 45)
!12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 128, flags: DIFlagVector, elements: !16)
!13 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !14, name: "float64_t", file: !15, line: 20)
!14 = !DIBasicType(name: "double", size: 64, encoding: DW_ATE_float)
!15 = !DIFile(filename: "kiln/resource/include/arm_vector_types.h", directory: "/src", checksumkind: CSK_MD5, checksum: "ac4605be3d103eca65154e6a5d03f0f2")
!16 = !{!17}
!17 = !DISubrange(count: 2)
!18 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !19, name: "int32x2_t", file: !15, line: 27)
!19 = !DICompositeType(tag: DW_TAG_array_type, baseType: !20, size: 64, flags: DIFlagVector, elements: !16)
!20 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !21, name: "int32_t", file: !23, line: 26)
!21 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !6, name: "__int32_t", file: !22, line: 41)
!22 = !DIFile(filename: "/usr/include/aarch64-linux-gnu/bits/types.h", directory: "", checksumkind: CSK_MD5, checksum: "0737a53e1b85eab0e0ba9675962d13f4")
!23 = !DIFile(filename: "/usr/include/aarch64-linux-gnu/bits/stdint-intn.h", directory: "", checksumkind: CSK_MD5, checksum: "d8f6972fff852003b8782e6edd3802e7")
!24 = !{!0, !25}
!25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
!26 = distinct !DIGlobalVariable(name: "failures", linkageName: "_ZL8failures", scope: !7, file: !3, line: 12, type: !6, isLocal: true, isDefinition: true)
!27 = !{}
!28 = !{i32 7, !"Dwarf Version", i32 5}
!29 = !{i32 2, !"Debug Info Version", i32 3}
!30 = !{i32 1, !"wchar_size", i32 4}
!31 = !{i32 8, !"PIC Level", i32 2}
!32 = !{i32 7, !"PIE Level", i32 2}
!33 = !{i32 7, !"uwtable", i32 2}
!34 = !{i32 7, !"frame-pointer", i32 4}
!35 = !{!"kiln version 1.1.0"}
!36 = distinct !DISubprogram(name: "add_ps", linkageName: "_Z6add_ps13__Float32x4_tS_", scope: !3, file: !3, line: 26, type: !37, scopeLine: 26, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!37 = !DISubroutineType(types: !38)
!38 = !{!39, !39, !39}
!39 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !40, name: "float32x4_t", file: !15, line: 42)
!40 = !DICompositeType(tag: DW_TAG_array_type, baseType: !41, size: 128, flags: DIFlagVector, elements: !43)
!41 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !42, name: "float32_t", file: !15, line: 17)
!42 = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
!43 = !{!44}
!44 = !DISubrange(count: 4)
!45 = !DILocalVariable(name: "a", arg: 1, scope: !36, file: !3, line: 26, type: !39)
!46 = !DILocation(line: 26, column: 58, scope: !36)
!47 = !DILocalVariable(name: "b", arg: 2, scope: !36, file: !3, line: 26, type: !39)
!48 = !DILocation(line: 26, column: 73, scope: !36)
!49 = !DILocalVariable(name: "c", scope: !36, file: !3, line: 27, type: !39)
!50 = !DILocation(line: 27, column: 15, scope: !36)
!51 = !DILocation(line: 27, column: 19, scope: !36)
!52 = !DILocation(line: 27, column: 23, scope: !36)
!53 = !DILocation(line: 27, column: 21, scope: !36)
!54 = !DILocation(line: 28, column: 3, scope: !36)
!55 = !{i64 793}
!56 = !DILocation(line: 29, column: 10, scope: !36)
!57 = !DILocation(line: 29, column: 14, scope: !36)
!58 = !DILocation(line: 29, column: 12, scope: !36)
!59 = !DILocation(line: 29, column: 3, scope: !36)
!60 = distinct !DISubprogram(name: "add_pd", linkageName: "_Z6add_pd13__Float64x2_tS_", scope: !3, file: !3, line: 32, type: !61, scopeLine: 32, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!61 = !DISubroutineType(types: !62)
!62 = !{!11, !11, !11}
!63 = !DILocalVariable(name: "a", arg: 1, scope: !60, file: !3, line: 32, type: !11)
!64 = !DILocation(line: 32, column: 58, scope: !60)
!65 = !DILocalVariable(name: "b", arg: 2, scope: !60, file: !3, line: 32, type: !11)
!66 = !DILocation(line: 32, column: 73, scope: !60)
!67 = !DILocalVariable(name: "c", scope: !60, file: !3, line: 33, type: !11)
!68 = !DILocation(line: 33, column: 15, scope: !60)
!69 = !DILocation(line: 33, column: 19, scope: !60)
!70 = !DILocation(line: 33, column: 23, scope: !60)
!71 = !DILocation(line: 33, column: 21, scope: !60)
!72 = !DILocation(line: 34, column: 3, scope: !60)
!73 = !{i64 928}
!74 = !DILocation(line: 35, column: 10, scope: !60)
!75 = !DILocation(line: 35, column: 14, scope: !60)
!76 = !DILocation(line: 35, column: 12, scope: !60)
!77 = !DILocation(line: 35, column: 3, scope: !60)
!78 = distinct !DISubprogram(name: "copy_rm", linkageName: "_Z7copy_rml", scope: !3, file: !3, line: 40, type: !79, scopeLine: 40, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!79 = !DISubroutineType(types: !80)
!80 = !{!81, !81}
!81 = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
!82 = !DILocalVariable(name: "in", arg: 1, scope: !78, file: !3, line: 40, type: !81)
!83 = !DILocation(line: 40, column: 45, scope: !78)
!84 = !DILocalVariable(name: "out", scope: !78, file: !3, line: 41, type: !81)
!85 = !DILocation(line: 41, column: 8, scope: !78)
!86 = !DILocation(line: 42, column: 46, scope: !78)
!87 = !DILocation(line: 42, column: 3, scope: !78)
!88 = !{i64 1151}
!89 = !DILocation(line: 43, column: 10, scope: !78)
!90 = !DILocation(line: 43, column: 3, scope: !78)
!91 = distinct !DISubprogram(name: "add_m", linkageName: "_Z5add_mll", scope: !3, file: !3, line: 47, type: !92, scopeLine: 47, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!92 = !DISubroutineType(types: !93)
!93 = !{!81, !81, !81}
!94 = !DILocalVariable(name: "a", arg: 1, scope: !91, file: !3, line: 47, type: !81)
!95 = !DILocation(line: 47, column: 43, scope: !91)
!96 = !DILocalVariable(name: "b", arg: 2, scope: !91, file: !3, line: 47, type: !81)
!97 = !DILocation(line: 47, column: 51, scope: !91)
!98 = !DILocation(line: 48, column: 3, scope: !91)
!99 = !DILocation(line: 48, column: 80, scope: !91)
!100 = !{i64 1344, i64 1358, i64 1378}
!101 = !DILocation(line: 49, column: 10, scope: !91)
!102 = !DILocation(line: 49, column: 3, scope: !91)
!103 = distinct !DISubprogram(name: "keep", linkageName: "_Z4keepd", scope: !3, file: !3, line: 52, type: !104, scopeLine: 52, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !27)
!104 = !DISubroutineType(types: !105)
!105 = !{!14, !14}
!106 = !DILocalVariable(name: "d", arg: 1, scope: !103, file: !3, line: 52, type: !14)
!107 = !DILocation(line: 52, column: 46, scope: !103)
!108 = !DILocation(line: 53, column: 37, scope: !103)
!109 = !DILocation(line: 53, column: 3, scope: !103)
!110 = !{i64 1514}
!111 = !DILocation(line: 54, column: 37, scope: !103)
!112 = !DILocation(line: 54, column: 3, scope: !103)
!113 = !{i64 1566}
!114 = !DILocation(line: 55, column: 10, scope: !103)
!115 = !DILocation(line: 55, column: 12, scope: !103)
!116 = !DILocation(line: 55, column: 3, scope: !103)
!117 = !DILocation(line: 59, column: 9, scope: !2)
!118 = !DILocation(line: 59, column: 18, scope: !2)
!119 = !DILocation(line: 59, column: 3, scope: !2)
!120 = !DILocation(line: 60, column: 9, scope: !2)
!121 = !DILocation(line: 60, column: 19, scope: !2)
!122 = !DILocation(line: 60, column: 3, scope: !2)
!123 = !DILocation(line: 61, column: 9, scope: !2)
!124 = !DILocation(line: 61, column: 25, scope: !2)
!125 = !DILocation(line: 61, column: 3, scope: !2)
!126 = !DILocation(line: 62, column: 9, scope: !2)
!127 = !DILocation(line: 62, column: 19, scope: !2)
!128 = !DILocation(line: 62, column: 3, scope: !2)
!129 = !DILocation(line: 63, column: 9, scope: !2)
!130 = !DILocation(line: 63, column: 20, scope: !2)
!131 = !DILocation(line: 63, column: 3, scope: !2)
!132 = !DILocation(line: 64, column: 9, scope: !2)
!133 = !DILocation(line: 64, column: 22, scope: !2)
!134 = !DILocation(line: 64, column: 3, scope: !2)
!135 = !DILocation(line: 65, column: 9, scope: !2)
!136 = !DILocation(line: 65, column: 34, scope: !2)
!137 = !DILocation(line: 65, column: 3, scope: !2)
!138 = !DILocation(line: 67, column: 9, scope: !2)
!139 = !DILocation(line: 67, column: 21, scope: !2)
!140 = !DILocation(line: 67, column: 30, scope: !2)
!141 = !DILocation(line: 67, column: 34, scope: !2)
!142 = !DILocation(line: 67, column: 33, scope: !2)
!143 = !DILocation(line: 67, column: 46, scope: !2)
!144 = !DILocation(line: 0, scope: !2)
!145 = !DILocation(line: 67, column: 3, scope: !2)
!146 = !DILocalVariable(name: "a", scope: !2, file: !3, line: 68, type: !39)
!147 = !DILocation(line: 68, column: 15, scope: !2)
!148 = !DILocalVariable(name: "b", scope: !2, file: !3, line: 68, type: !39)
!149 = !DILocation(line: 68, column: 33, scope: !2)
!150 = !DILocalVariable(name: "c", scope: !2, file: !3, line: 69, type: !39)
!151 = !DILocation(line: 69, column: 15, scope: !2)
!152 = !DILocation(line: 69, column: 26, scope: !2)
!153 = !DILocation(line: 69, column: 29, scope: !2)
!154 = !DILocation(line: 69, column: 19, scope: !2)
!155 = !DILocation(line: 70, column: 9, scope: !2)
!156 = !DILocation(line: 70, column: 14, scope: !2)
!157 = !DILocation(line: 70, column: 48, scope: !2)
!158 = !DILocation(line: 70, column: 23, scope: !2)
!159 = !DILocation(line: 70, column: 28, scope: !2)
!160 = !DILocation(line: 70, column: 37, scope: !2)
!161 = !DILocation(line: 70, column: 42, scope: !2)
!162 = !DILocation(line: 70, column: 51, scope: !2)
!163 = !DILocation(line: 70, column: 56, scope: !2)
!164 = !DILocation(line: 70, column: 3, scope: !2)
!165 = !DILocalVariable(name: "p", scope: !2, file: !3, line: 71, type: !39)
!166 = !DILocation(line: 71, column: 15, scope: !2)
!167 = !DILocation(line: 71, column: 24, scope: !2)
!168 = !DILocation(line: 71, column: 19, scope: !2)
!169 = !DILocation(line: 72, column: 9, scope: !2)
!170 = !DILocation(line: 72, column: 14, scope: !2)
!171 = !DILocation(line: 72, column: 19, scope: !2)
!172 = !DILocation(line: 72, column: 22, scope: !2)
!173 = !DILocation(line: 72, column: 27, scope: !2)
!174 = !DILocation(line: 72, column: 3, scope: !2)
!175 = !DILocalVariable(name: "d", scope: !2, file: !3, line: 73, type: !11)
!176 = !DILocation(line: 73, column: 15, scope: !2)
!177 = !DILocation(line: 73, column: 19, scope: !2)
!178 = !DILocation(line: 74, column: 9, scope: !2)
!179 = !DILocation(line: 74, column: 14, scope: !2)
!180 = !DILocation(line: 74, column: 21, scope: !2)
!181 = !DILocation(line: 74, column: 24, scope: !2)
!182 = !DILocation(line: 74, column: 29, scope: !2)
!183 = !DILocation(line: 74, column: 3, scope: !2)
!184 = !DILocalVariable(name: "s", scope: !2, file: !3, line: 75, type: !18)
!185 = !DILocation(line: 75, column: 13, scope: !2)
!186 = !DILocation(line: 75, column: 17, scope: !2)
!187 = !DILocation(line: 76, column: 9, scope: !2)
!188 = !DILocation(line: 76, column: 14, scope: !2)
!189 = !DILocation(line: 76, column: 19, scope: !2)
!190 = !DILocation(line: 76, column: 22, scope: !2)
!191 = !DILocation(line: 76, column: 27, scope: !2)
!192 = !DILocation(line: 76, column: 3, scope: !2)
!193 = !DILocation(line: 77, column: 9, scope: !2)
!194 = !DILocation(line: 77, column: 23, scope: !2)
!195 = !DILocation(line: 77, column: 3, scope: !2)
!196 = !DILocation(line: 78, column: 9, scope: !2)
!197 = !DILocation(line: 78, column: 22, scope: !2)
!198 = !DILocation(line: 78, column: 3, scope: !2)
!199 = !DILocation(line: 79, column: 9, scope: !2)
!200 = !DILocation(line: 79, column: 20, scope: !2)
!201 = !DILocation(line: 79, column: 3, scope: !2)
!202 = !DILocation(line: 80, column: 36, scope: !2)
!203 = !DILocation(line: 80, column: 3, scope: !2)
!204 = !DILocation(line: 81, column: 10, scope: !2)
!205 = !DILocation(line: 81, column: 19, scope: !2)
!206 = !DILocation(line: 81, column: 3, scope: !2)
!207 = distinct !DISubprogram(name: "check", linkageName: "_ZL5checkbPKc", scope: !3, file: !3, line: 14, type: !208, scopeLine: 14, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, retainedNodes: !27)
!208 = !DISubroutineType(types: !209)
!209 = !{null, !210, !211}
!210 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!211 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !212, size: 64)
!212 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !213)
!213 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_unsigned_char)
!214 = !DILocalVariable(name: "ok", arg: 1, scope: !207, file: !3, line: 14, type: !210)
!215 = !DILocation(line: 14, column: 24, scope: !207)
!216 = !DILocalVariable(name: "what", arg: 2, scope: !207, file: !3, line: 14, type: !211)
!217 = !DILocation(line: 14, column: 40, scope: !207)
!218 = !DILocation(line: 15, column: 8, scope: !219)
!219 = distinct !DILexicalBlock(scope: !207, file: !3, line: 15, column: 7)
!220 = !DILocation(line: 15, column: 7, scope: !207)
!221 = !DILocation(line: 16, column: 5, scope: !222)
!222 = distinct !DILexicalBlock(scope: !219, file: !3, line: 15, column: 12)
!223 = !DILocation(line: 17, column: 27, scope: !222)
!224 = !DILocation(line: 17, column: 5, scope: !222)
!225 = !DILocation(line: 18, column: 3, scope: !222)
!226 = !DILocation(line: 19, column: 1, scope: !207)
!227 = distinct !DISubprogram(name: "pass<int>", linkageName: "_Z4passIiET_S0_", scope: !3, file: !3, line: 21, type: !228, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !230, retainedNodes: !27)
!228 = !DISubroutineType(types: !229)
!229 = !{!6, !6}
!230 = !{!231}
!231 = !DITemplateTypeParameter(name: "T", type: !6)
!232 = !DILocalVariable(name: "x", arg: 1, scope: !227, file: !3, line: 21, type: !6)
!233 = !DILocation(line: 21, column: 55, scope: !227)
!234 = !DILocation(line: 22, column: 3, scope: !227)
!235 = !{i64 662}
!236 = !DILocation(line: 23, column: 10, scope: !227)
!237 = !DILocation(line: 23, column: 3, scope: !227)
!238 = distinct !DISubprogram(name: "pass<long>", linkageName: "_Z4passIlET_S0_", scope: !3, file: !3, line: 21, type: !79, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !239, retainedNodes: !27)
!239 = !{!240}
!240 = !DITemplateTypeParameter(name: "T", type: !81)
!241 = !DILocalVariable(name: "x", arg: 1, scope: !238, file: !3, line: 21, type: !81)
!242 = !DILocation(line: 21, column: 55, scope: !238)
!243 = !DILocation(line: 22, column: 3, scope: !238)
!244 = !DILocation(line: 23, column: 10, scope: !238)
!245 = !DILocation(line: 23, column: 3, scope: !238)
!246 = distinct !DISubprogram(name: "pass<short>", linkageName: "_Z4passIsET_S0_", scope: !3, file: !3, line: 21, type: !247, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !249, retainedNodes: !27)
!247 = !DISubroutineType(types: !248)
!248 = !{!9, !9}
!249 = !{!250}
!250 = !DITemplateTypeParameter(name: "T", type: !9)
!251 = !DILocalVariable(name: "x", arg: 1, scope: !246, file: !3, line: 21, type: !9)
!252 = !DILocation(line: 21, column: 55, scope: !246)
!253 = !DILocation(line: 22, column: 3, scope: !246)
!254 = !DILocation(line: 23, column: 10, scope: !246)
!255 = !DILocation(line: 23, column: 3, scope: !246)
!256 = distinct !DISubprogram(name: "pass<char>", linkageName: "_Z4passIcET_S0_", scope: !3, file: !3, line: 21, type: !257, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !259, retainedNodes: !27)
!257 = !DISubroutineType(types: !258)
!258 = !{!213, !213}
!259 = !{!260}
!260 = !DITemplateTypeParameter(name: "T", type: !213)
!261 = !DILocalVariable(name: "x", arg: 1, scope: !256, file: !3, line: 21, type: !213)
!262 = !DILocation(line: 21, column: 55, scope: !256)
!263 = !DILocation(line: 22, column: 3, scope: !256)
!264 = !DILocation(line: 23, column: 10, scope: !256)
!265 = !DILocation(line: 23, column: 3, scope: !256)
!266 = distinct !DISubprogram(name: "pass<float>", linkageName: "_Z4passIfET_S0_", scope: !3, file: !3, line: 21, type: !267, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !269, retainedNodes: !27)
!267 = !DISubroutineType(types: !268)
!268 = !{!42, !42}
!269 = !{!270}
!270 = !DITemplateTypeParameter(name: "T", type: !42)
!271 = !DILocalVariable(name: "x", arg: 1, scope: !266, file: !3, line: 21, type: !42)
!272 = !DILocation(line: 21, column: 55, scope: !266)
!273 = !DILocation(line: 22, column: 3, scope: !266)
!274 = !DILocation(line: 23, column: 10, scope: !266)
!275 = !DILocation(line: 23, column: 3, scope: !266)
!276 = distinct !DISubprogram(name: "pass<double>", linkageName: "_Z4passIdET_S0_", scope: !3, file: !3, line: 21, type: !104, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !277, retainedNodes: !27)
!277 = !{!278}
!278 = !DITemplateTypeParameter(name: "T", type: !14)
!279 = !DILocalVariable(name: "x", arg: 1, scope: !276, file: !3, line: 21, type: !14)
!280 = !DILocation(line: 21, column: 55, scope: !276)
!281 = !DILocation(line: 22, column: 3, scope: !276)
!282 = !DILocation(line: 23, column: 10, scope: !276)
!283 = !DILocation(line: 23, column: 3, scope: !276)
!284 = distinct !DISubprogram(name: "pass<__int128>", linkageName: "_Z4passInET_S0_", scope: !3, file: !3, line: 21, type: !285, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !287, retainedNodes: !27)
!285 = !DISubroutineType(types: !286)
!286 = !{!10, !10}
!287 = !{!288}
!288 = !DITemplateTypeParameter(name: "T", type: !10)
!289 = !DILocalVariable(name: "x", arg: 1, scope: !284, file: !3, line: 21, type: !10)
!290 = !DILocation(line: 21, column: 55, scope: !284)
!291 = !DILocation(line: 22, column: 3, scope: !284)
!292 = !DILocation(line: 23, column: 10, scope: !284)
!293 = !DILocation(line: 23, column: 3, scope: !284)
!294 = distinct !DISubprogram(name: "pass<int *>", linkageName: "_Z4passIPiET_S1_", scope: !3, file: !3, line: 21, type: !295, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !27)
!295 = !DISubroutineType(types: !296)
!296 = !{!297, !297}
!297 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !6, size: 64)
!298 = !{!299}
!299 = !DITemplateTypeParameter(name: "T", type: !297)
!300 = !DILocalVariable(name: "x", arg: 1, scope: !294, file: !3, line: 21, type: !297)
!301 = !DILocation(line: 21, column: 55, scope: !294)
!302 = !DILocation(line: 22, column: 3, scope: !294)
!303 = !DILocation(line: 23, column: 10, scope: !294)
!304 = !DILocation(line: 23, column: 3, scope: !294)
!305 = distinct !DISubprogram(name: "pass<__attribute__((neon_vector_type(4))) float>", linkageName: "_Z4passI13__Float32x4_tET_S1_", scope: !3, file: !3, line: 21, type: !306, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !309, retainedNodes: !27)
!306 = !DISubroutineType(types: !307)
!307 = !{!308, !308}
!308 = !DICompositeType(tag: DW_TAG_array_type, baseType: !42, size: 128, flags: DIFlagVector, elements: !43)
!309 = !{!310}
!310 = !DITemplateTypeParameter(name: "T", type: !308)
!311 = !DILocalVariable(name: "x", arg: 1, scope: !305, file: !3, line: 21, type: !308)
!312 = !DILocation(line: 21, column: 55, scope: !305)
!313 = !DILocation(line: 22, column: 3, scope: !305)
!314 = !DILocation(line: 23, column: 10, scope: !305)
!315 = !DILocation(line: 23, column: 3, scope: !305)
!316 = distinct !DISubprogram(name: "pass<__attribute__((neon_vector_type(2))) int>", linkageName: "_Z4passI11__Int32x2_tET_S1_", scope: !3, file: !3, line: 21, type: !317, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !320, retainedNodes: !27)
!317 = !DISubroutineType(types: !318)
!318 = !{!319, !319}
!319 = !DICompositeType(tag: DW_TAG_array_type, baseType: !6, size: 64, flags: DIFlagVector, elements: !16)
!320 = !{!321}
!321 = !DITemplateTypeParameter(name: "T", type: !319)
!322 = !DILocalVariable(name: "x", arg: 1, scope: !316, file: !3, line: 21, type: !319)
!323 = !DILocation(line: 21, column: 55, scope: !316)
!324 = !DILocation(line: 22, column: 3, scope: !316)
!325 = !DILocation(line: 23, column: 10, scope: !316)
!326 = !DILocation(line: 23, column: 3, scope: !316)

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
@_ZZ4mainE4cell = internal global i32 9, align 4, !dbg !0
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
@_ZL8failures = internal global i32 0, align 4, !dbg !21
@.str.20 = private unnamed_addr constant [11 x i8] c"FAILED %s\0A\00", align 1

define dso_local noundef float @_Z7twice_ff(float noundef %0) #0 !dbg !276 {
  %2 = alloca float, align 4
  %3 = alloca float, align 4
  store float %0, ptr %2, align 4
    #dbg_declare(ptr %2, !279, !DIExpression(), !280)
    #dbg_declare(ptr %3, !281, !DIExpression(), !282)
  %4 = load float, ptr %2, align 4, !dbg !283
  %5 = load float, ptr %2, align 4, !dbg !284
  %6 = fadd float %4, %5, !dbg !285
  store float %6, ptr %3, align 4, !dbg !282
  %7 = load float, ptr %3, align 4, !dbg !286
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %3, float %7) #6, !dbg !286, !srcloc !287
  %8 = load float, ptr %3, align 4, !dbg !288
  %9 = fadd float %8, 1.000000e+00, !dbg !289
  ret float %9, !dbg !290
}

define dso_local noundef <4 x float> @_Z6add_psDv4_fS_(<4 x float> noundef %0, <4 x float> noundef %1) #1 !dbg !291 {
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %3, align 16
    #dbg_declare(ptr %3, !299, !DIExpression(), !300)
  store <4 x float> %1, ptr %4, align 16
    #dbg_declare(ptr %4, !301, !DIExpression(), !302)
    #dbg_declare(ptr %5, !303, !DIExpression(), !304)
  %6 = load <4 x float>, ptr %3, align 16, !dbg !305
  %7 = load <4 x float>, ptr %4, align 16, !dbg !306
  %8 = fadd <4 x float> %6, %7, !dbg !307
  store <4 x float> %8, ptr %5, align 16, !dbg !304
  %9 = load <4 x float>, ptr %5, align 16, !dbg !308
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<4 x float>) %5, <4 x float> %9) #6, !dbg !308, !srcloc !309
  %10 = load <4 x float>, ptr %5, align 16, !dbg !310
  %11 = load <4 x float>, ptr %3, align 16, !dbg !311
  %12 = fmul <4 x float> %10, %11, !dbg !312
  ret <4 x float> %12, !dbg !313
}

define dso_local noundef <2 x double> @_Z6add_pdDv2_dS_(<2 x double> noundef %0, <2 x double> noundef %1) #1 !dbg !314 {
  %3 = alloca <2 x double>, align 16
  %4 = alloca <2 x double>, align 16
  %5 = alloca <2 x double>, align 16
  store <2 x double> %0, ptr %3, align 16
    #dbg_declare(ptr %3, !317, !DIExpression(), !318)
  store <2 x double> %1, ptr %4, align 16
    #dbg_declare(ptr %4, !319, !DIExpression(), !320)
    #dbg_declare(ptr %5, !321, !DIExpression(), !322)
  %6 = load <2 x double>, ptr %3, align 16, !dbg !323
  %7 = load <2 x double>, ptr %4, align 16, !dbg !324
  %8 = fadd <2 x double> %6, %7, !dbg !325
  store <2 x double> %8, ptr %5, align 16, !dbg !322
  %9 = load <2 x double>, ptr %5, align 16, !dbg !326
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<2 x double>) %5, <2 x double> %9) #6, !dbg !326, !srcloc !327
  %10 = load <2 x double>, ptr %5, align 16, !dbg !328
  %11 = load <2 x double>, ptr %4, align 16, !dbg !329
  %12 = fsub <2 x double> %10, %11, !dbg !330
  ret <2 x double> %12, !dbg !331
}

define dso_local noundef <8 x float> @_Z7add_ps8Dv8_fS_(ptr noundef byval(<8 x float>) align 32 %0, ptr noundef byval(<8 x float>) align 32 %1) #2 !dbg !332 {
  %3 = alloca <8 x float>, align 32
  %4 = alloca <8 x float>, align 32
  %5 = alloca <8 x float>, align 32
  %6 = load <8 x float>, ptr %0, align 32
  %7 = load <8 x float>, ptr %1, align 32
  store <8 x float> %6, ptr %3, align 32
    #dbg_declare(ptr %3, !340, !DIExpression(), !341)
  store <8 x float> %7, ptr %4, align 32
    #dbg_declare(ptr %4, !342, !DIExpression(), !343)
    #dbg_declare(ptr %5, !344, !DIExpression(), !345)
  %8 = load <8 x float>, ptr %3, align 32, !dbg !346
  %9 = load <8 x float>, ptr %4, align 32, !dbg !347
  %10 = fadd <8 x float> %8, %9, !dbg !348
  store <8 x float> %10, ptr %5, align 32, !dbg !345
  %11 = load <8 x float>, ptr %5, align 32, !dbg !349
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<8 x float>) %5, <8 x float> %11) #6, !dbg !349, !srcloc !350
  %12 = load <8 x float>, ptr %5, align 32, !dbg !351
  ret <8 x float> %12, !dbg !352
}

define dso_local noundef float @_Z4sum8f(float noundef %0) #2 !dbg !353 {
  %2 = alloca float, align 4
  %3 = alloca <8 x float>, align 32
  %4 = alloca <8 x float>, align 32
  %5 = alloca <8 x float>, align 32
  %6 = alloca <8 x float>, align 32
  %7 = alloca float, align 4
  %8 = alloca i32, align 4
  store float %0, ptr %2, align 4
    #dbg_declare(ptr %2, !354, !DIExpression(), !355)
    #dbg_declare(ptr %3, !356, !DIExpression(), !357)
  %9 = load float, ptr %2, align 4, !dbg !358
  %10 = insertelement <8 x float> poison, float %9, i32 0, !dbg !359
  %11 = load float, ptr %2, align 4, !dbg !360
  %12 = fadd float %11, 1.000000e+00, !dbg !361
  %13 = insertelement <8 x float> %10, float %12, i32 1, !dbg !359
  %14 = load float, ptr %2, align 4, !dbg !362
  %15 = fadd float %14, 2.000000e+00, !dbg !363
  %16 = insertelement <8 x float> %13, float %15, i32 2, !dbg !359
  %17 = load float, ptr %2, align 4, !dbg !364
  %18 = fadd float %17, 3.000000e+00, !dbg !365
  %19 = insertelement <8 x float> %16, float %18, i32 3, !dbg !359
  %20 = load float, ptr %2, align 4, !dbg !366
  %21 = fadd float %20, 4.000000e+00, !dbg !367
  %22 = insertelement <8 x float> %19, float %21, i32 4, !dbg !359
  %23 = load float, ptr %2, align 4, !dbg !368
  %24 = fadd float %23, 5.000000e+00, !dbg !369
  %25 = insertelement <8 x float> %22, float %24, i32 5, !dbg !359
  %26 = load float, ptr %2, align 4, !dbg !370
  %27 = fadd float %26, 6.000000e+00, !dbg !371
  %28 = insertelement <8 x float> %25, float %27, i32 6, !dbg !359
  %29 = load float, ptr %2, align 4, !dbg !372
  %30 = fadd float %29, 7.000000e+00, !dbg !373
  %31 = insertelement <8 x float> %28, float %30, i32 7, !dbg !359
  store <8 x float> %31, ptr %3, align 32, !dbg !357
    #dbg_declare(ptr %4, !374, !DIExpression(), !375)
  %32 = load <8 x float>, ptr %3, align 32, !dbg !376
  %33 = load <8 x float>, ptr %3, align 32, !dbg !377
  store <8 x float> %32, ptr %5, align 32, !dbg !378
  store <8 x float> %33, ptr %6, align 32, !dbg !378
  %34 = call noundef <8 x float> @_Z7add_ps8Dv8_fS_(ptr noundef byval(<8 x float>) align 32 %5, ptr noundef byval(<8 x float>) align 32 %6), !dbg !378
  store <8 x float> %34, ptr %4, align 32, !dbg !375
    #dbg_declare(ptr %7, !379, !DIExpression(), !380)
  store float 0.000000e+00, ptr %7, align 4, !dbg !380
    #dbg_declare(ptr %8, !381, !DIExpression(), !383)
  store i32 0, ptr %8, align 4, !dbg !383
  br label %35, !dbg !384

35:
  %36 = load i32, ptr %8, align 4, !dbg !385
  %37 = icmp slt i32 %36, 8, !dbg !387
  br i1 %37, label %38, label %47, !dbg !388

38:
  %39 = load <8 x float>, ptr %4, align 32, !dbg !389
  %40 = load i32, ptr %8, align 4, !dbg !390
  %41 = extractelement <8 x float> %39, i32 %40, !dbg !389
  %42 = load float, ptr %7, align 4, !dbg !391
  %43 = fadd float %42, %41, !dbg !391
  store float %43, ptr %7, align 4, !dbg !391
  br label %44, !dbg !392

44:
  %45 = load i32, ptr %8, align 4, !dbg !393
  %46 = add nsw i32 %45, 1, !dbg !393
  store i32 %46, ptr %8, align 4, !dbg !393
  br label %35, !dbg !394, !llvm.loop !395

47:
  %48 = load float, ptr %7, align 4, !dbg !398
  ret float %48, !dbg !399
}

define dso_local noundef i32 @_Z5inc_gi(i32 noundef %0) #0 !dbg !400 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
    #dbg_declare(ptr %2, !401, !DIExpression(), !402)
  %3 = load i32, ptr %2, align 4, !dbg !403
  call void asm "incl $0", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %2, i32 %3) #6, !dbg !403, !srcloc !404
  %4 = load i32, ptr %2, align 4, !dbg !405
  ret i32 %4, !dbg !406
}

define dso_local noundef i32 @_Z7copy_rmi(i32 noundef %0) #0 !dbg !407 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
    #dbg_declare(ptr %2, !408, !DIExpression(), !409)
    #dbg_declare(ptr %3, !410, !DIExpression(), !411)
  store i32 -1, ptr %3, align 4, !dbg !411
  %4 = load i32, ptr %2, align 4, !dbg !412
  call void asm "movl $1, $0", "=*r|m,ri|r,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %3, i32 %4) #6, !dbg !413, !srcloc !414
  %5 = load i32, ptr %3, align 4, !dbg !415
  ret i32 %5, !dbg !416
}

define dso_local noundef float @_Z7copy_ssf(float noundef %0) #0 !dbg !417 {
  %2 = alloca float, align 4
  %3 = alloca float, align 4
  store float %0, ptr %2, align 4
    #dbg_declare(ptr %2, !418, !DIExpression(), !419)
    #dbg_declare(ptr %3, !420, !DIExpression(), !421)
  store float -1.000000e+00, ptr %3, align 4, !dbg !421
  %4 = load float, ptr %2, align 4, !dbg !422
  call void asm "movss $1, $0", "=*x|m,x|x,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %3, float %4) #6, !dbg !423, !srcloc !424
  %5 = load float, ptr %3, align 4, !dbg !425
  ret float %5, !dbg !426
}

define dso_local noundef i64 @_Z5add_rll(i64 noundef %0, i64 noundef %1) #0 !dbg !427 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
    #dbg_declare(ptr %3, !430, !DIExpression(), !431)
  store i64 %1, ptr %4, align 8
    #dbg_declare(ptr %4, !432, !DIExpression(), !433)
  %5 = load i64, ptr %3, align 8, !dbg !434
  %6 = load i64, ptr %4, align 8, !dbg !435
  call void asm "addq $1, $0", "=*r|m,r|r,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i64) %3, i64 %6, i64 %5) #6, !dbg !434, !srcloc !436
  %7 = load i64, ptr %3, align 8, !dbg !437
  ret i64 %7, !dbg !438
}

define dso_local noundef double @_Z4keepd(double noundef %0) #0 !dbg !439 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
    #dbg_declare(ptr %2, !442, !DIExpression(), !443)
  %3 = load double, ptr %2, align 8, !dbg !444
  call void asm sideeffect "", "r|m,~{memory},~{dirflag},~{fpsr},~{flags}"(double %3) #6, !dbg !445, !srcloc !446
  %4 = load double, ptr %2, align 8, !dbg !447
  call void asm sideeffect "", "x|m,~{memory},~{dirflag},~{fpsr},~{flags}"(double %4) #6, !dbg !448, !srcloc !449
  %5 = load double, ptr %2, align 8, !dbg !450
  %6 = fmul double %5, 2.000000e+00, !dbg !451
  ret double %6, !dbg !452
}

define dso_local noundef i32 @main() #3 !dbg !2 {
  %1 = alloca i32, align 4
  %2 = alloca <4 x float>, align 16
  %3 = alloca <4 x float>, align 16
  %4 = alloca <4 x float>, align 16
  %5 = alloca <4 x float>, align 16
  %6 = alloca <2 x double>, align 16
  %7 = alloca <2 x i64>, align 16
  store i32 0, ptr %1, align 4
  %8 = call noundef i32 @_Z4passIiET_S0_(i32 noundef 41), !dbg !453
  %9 = icmp eq i32 %8, 41, !dbg !454
  call void @_ZL5checkbPKc(i1 noundef zeroext %9, ptr noundef @.str), !dbg !455
  %10 = call noundef i64 @_Z4passIlET_S0_(i64 noundef -5), !dbg !456
  %11 = icmp eq i64 %10, -5, !dbg !457
  call void @_ZL5checkbPKc(i1 noundef zeroext %11, ptr noundef @.str.1), !dbg !458
  %12 = call noundef signext i16 @_Z4passIsET_S0_(i16 noundef signext -3), !dbg !459
  %13 = sext i16 %12 to i32, !dbg !459
  %14 = icmp eq i32 %13, -3, !dbg !460
  call void @_ZL5checkbPKc(i1 noundef zeroext %14, ptr noundef @.str.2), !dbg !461
  %15 = call noundef signext i8 @_Z4passIcET_S0_(i8 noundef signext 97), !dbg !462
  %16 = sext i8 %15 to i32, !dbg !462
  %17 = icmp eq i32 %16, 97, !dbg !463
  call void @_ZL5checkbPKc(i1 noundef zeroext %17, ptr noundef @.str.3), !dbg !464
  %18 = call noundef float @_Z4passIfET_S0_(float noundef 2.500000e+00), !dbg !465
  %19 = fcmp oeq float %18, 2.500000e+00, !dbg !466
  call void @_ZL5checkbPKc(i1 noundef zeroext %19, ptr noundef @.str.4), !dbg !467
  %20 = call noundef double @_Z4passIdET_S0_(double noundef -1.250000e-01), !dbg !468
  %21 = fcmp oeq double %20, -1.250000e-01, !dbg !469
  call void @_ZL5checkbPKc(i1 noundef zeroext %21, ptr noundef @.str.5), !dbg !470
  %22 = call noundef i128 @_Z4passInET_S0_(i128 noundef 1267650600228229401496703205376), !dbg !471
  %23 = icmp eq i128 %22, 1267650600228229401496703205376, !dbg !472
  call void @_ZL5checkbPKc(i1 noundef zeroext %23, ptr noundef @.str.6), !dbg !473
  %24 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell), !dbg !474
  %25 = icmp eq ptr %24, @_ZZ4mainE4cell, !dbg !475
  br i1 %25, label %26, label %30, !dbg !476

26:
  %27 = call noundef ptr @_Z4passIPiET_S1_(ptr noundef @_ZZ4mainE4cell), !dbg !477
  %28 = load i32, ptr %27, align 4, !dbg !478
  %29 = icmp eq i32 %28, 9, !dbg !479
  br label %30

30:
  %31 = phi i1 [ false, %0 ], [ %29, %26 ], !dbg !480
  call void @_ZL5checkbPKc(i1 noundef zeroext %31, ptr noundef @.str.7), !dbg !481
  %32 = call noundef float @_Z7twice_ff(float noundef 3.000000e+00), !dbg !482
  %33 = fcmp oeq float %32, 7.000000e+00, !dbg !483
  call void @_ZL5checkbPKc(i1 noundef zeroext %33, ptr noundef @.str.8), !dbg !484
    #dbg_declare(ptr %2, !485, !DIExpression(), !486)
  store <4 x float> <float 1.000000e+00, float 2.000000e+00, float 3.000000e+00, float 4.000000e+00>, ptr %2, align 16, !dbg !486
    #dbg_declare(ptr %3, !487, !DIExpression(), !488)
  store <4 x float> <float 1.000000e+01, float 2.000000e+01, float 3.000000e+01, float 4.000000e+01>, ptr %3, align 16, !dbg !488
    #dbg_declare(ptr %4, !489, !DIExpression(), !490)
  %34 = load <4 x float>, ptr %2, align 16, !dbg !491
  %35 = load <4 x float>, ptr %3, align 16, !dbg !492
  %36 = call noundef <4 x float> @_Z6add_psDv4_fS_(<4 x float> noundef %34, <4 x float> noundef %35), !dbg !493
  store <4 x float> %36, ptr %4, align 16, !dbg !490
  %37 = load <4 x float>, ptr %4, align 16, !dbg !494
  %38 = extractelement <4 x float> %37, i32 0, !dbg !494
  %39 = fcmp oeq float %38, 1.100000e+01, !dbg !495
  br i1 %39, label %40, label %52, !dbg !496

40:
  %41 = load <4 x float>, ptr %4, align 16, !dbg !497
  %42 = extractelement <4 x float> %41, i32 1, !dbg !497
  %43 = fcmp oeq float %42, 4.400000e+01, !dbg !498
  br i1 %43, label %44, label %52, !dbg !496

44:
  %45 = load <4 x float>, ptr %4, align 16, !dbg !499
  %46 = extractelement <4 x float> %45, i32 2, !dbg !499
  %47 = fcmp oeq float %46, 9.900000e+01, !dbg !500
  br i1 %47, label %48, label %52, !dbg !496

48:
  %49 = load <4 x float>, ptr %4, align 16, !dbg !501
  %50 = extractelement <4 x float> %49, i32 3, !dbg !501
  %51 = fcmp oeq float %50, 1.760000e+02, !dbg !502
  br label %52

52:
  %53 = phi i1 [ false, %44 ], [ false, %40 ], [ false, %30 ], [ %51, %48 ], !dbg !480
  call void @_ZL5checkbPKc(i1 noundef zeroext %53, ptr noundef @.str.9), !dbg !503
    #dbg_declare(ptr %5, !504, !DIExpression(), !505)
  %54 = load <4 x float>, ptr %2, align 16, !dbg !506
  %55 = call noundef <4 x float> @_Z4passIDv4_fET_S1_(<4 x float> noundef %54), !dbg !507
  store <4 x float> %55, ptr %5, align 16, !dbg !505
  %56 = load <4 x float>, ptr %5, align 16, !dbg !508
  %57 = extractelement <4 x float> %56, i32 0, !dbg !508
  %58 = fcmp oeq float %57, 1.000000e+00, !dbg !509
  br i1 %58, label %59, label %63, !dbg !510

59:
  %60 = load <4 x float>, ptr %5, align 16, !dbg !511
  %61 = extractelement <4 x float> %60, i32 3, !dbg !511
  %62 = fcmp oeq float %61, 4.000000e+00, !dbg !512
  br label %63

63:
  %64 = phi i1 [ false, %52 ], [ %62, %59 ], !dbg !480
  call void @_ZL5checkbPKc(i1 noundef zeroext %64, ptr noundef @.str.10), !dbg !513
    #dbg_declare(ptr %6, !514, !DIExpression(), !515)
  %65 = call noundef <2 x double> @_Z6add_pdDv2_dS_(<2 x double> noundef <double 1.500000e+00, double -2.000000e+00>, <2 x double> noundef <double 4.000000e+00, double 8.000000e+00>), !dbg !516
  store <2 x double> %65, ptr %6, align 16, !dbg !515
  %66 = load <2 x double>, ptr %6, align 16, !dbg !517
  %67 = extractelement <2 x double> %66, i32 0, !dbg !517
  %68 = fcmp oeq double %67, 1.500000e+00, !dbg !518
  br i1 %68, label %69, label %73, !dbg !519

69:
  %70 = load <2 x double>, ptr %6, align 16, !dbg !520
  %71 = extractelement <2 x double> %70, i32 1, !dbg !520
  %72 = fcmp oeq double %71, -2.000000e+00, !dbg !521
  br label %73

73:
  %74 = phi i1 [ false, %63 ], [ %72, %69 ], !dbg !480
  call void @_ZL5checkbPKc(i1 noundef zeroext %74, ptr noundef @.str.11), !dbg !522
    #dbg_declare(ptr %7, !523, !DIExpression(), !524)
  %75 = call noundef <2 x i64> @_Z4passIDv2_xET_S1_(<2 x i64> noundef <i64 7, i64 -9>), !dbg !525
  store <2 x i64> %75, ptr %7, align 16, !dbg !524
  %76 = load <2 x i64>, ptr %7, align 16, !dbg !526
  %77 = extractelement <2 x i64> %76, i32 0, !dbg !526
  %78 = icmp eq i64 %77, 7, !dbg !527
  br i1 %78, label %79, label %83, !dbg !528

79:
  %80 = load <2 x i64>, ptr %7, align 16, !dbg !529
  %81 = extractelement <2 x i64> %80, i32 1, !dbg !529
  %82 = icmp eq i64 %81, -9, !dbg !530
  br label %83

83:
  %84 = phi i1 [ false, %73 ], [ %82, %79 ], !dbg !480
  call void @_ZL5checkbPKc(i1 noundef zeroext %84, ptr noundef @.str.12), !dbg !531
  %85 = call noundef float @_Z4sum8f(float noundef 1.000000e+00), !dbg !532
  %86 = fcmp oeq float %85, 7.200000e+01, !dbg !533
  call void @_ZL5checkbPKc(i1 noundef zeroext %86, ptr noundef @.str.13), !dbg !534
  %87 = call noundef i32 @_Z5inc_gi(i32 noundef 41), !dbg !535
  %88 = icmp eq i32 %87, 42, !dbg !536
  call void @_ZL5checkbPKc(i1 noundef zeroext %88, ptr noundef @.str.14), !dbg !537
  %89 = call noundef i32 @_Z7copy_rmi(i32 noundef 1234), !dbg !538
  %90 = icmp eq i32 %89, 1234, !dbg !539
  call void @_ZL5checkbPKc(i1 noundef zeroext %90, ptr noundef @.str.15), !dbg !540
  %91 = call noundef float @_Z7copy_ssf(float noundef 6.500000e+00), !dbg !541
  %92 = fcmp oeq float %91, 6.500000e+00, !dbg !542
  call void @_ZL5checkbPKc(i1 noundef zeroext %92, ptr noundef @.str.16), !dbg !543
  %93 = call noundef i64 @_Z5add_rll(i64 noundef 40, i64 noundef 2), !dbg !544
  %94 = icmp eq i64 %93, 42, !dbg !545
  call void @_ZL5checkbPKc(i1 noundef zeroext %94, ptr noundef @.str.17), !dbg !546
  %95 = call noundef double @_Z4keepd(double noundef 1.250000e+00), !dbg !547
  %96 = fcmp oeq double %95, 2.500000e+00, !dbg !548
  call void @_ZL5checkbPKc(i1 noundef zeroext %96, ptr noundef @.str.18), !dbg !549
  %97 = load i32, ptr @_ZL8failures, align 4, !dbg !550
  %98 = call i32 (ptr, ...) @printf(ptr noundef @.str.19, i32 noundef %97), !dbg !551
  %99 = load i32, ptr @_ZL8failures, align 4, !dbg !552
  %100 = icmp ne i32 %99, 0, !dbg !553
  %101 = zext i1 %100 to i32, !dbg !552
  ret i32 %101, !dbg !554
}

define internal void @_ZL5checkbPKc(i1 noundef zeroext %0, ptr noundef %1) #4 !dbg !555 {
  %3 = alloca i8, align 1
  %4 = alloca ptr, align 8
  %5 = zext i1 %0 to i8
  store i8 %5, ptr %3, align 1
    #dbg_declare(ptr %3, !559, !DIExpression(), !560)
  store ptr %1, ptr %4, align 8
    #dbg_declare(ptr %4, !561, !DIExpression(), !562)
  %6 = load i8, ptr %3, align 1, !dbg !563
  %7 = trunc i8 %6 to i1, !dbg !563
  br i1 %7, label %13, label %8, !dbg !565

8:
  %9 = load i32, ptr @_ZL8failures, align 4, !dbg !566
  %10 = add nsw i32 %9, 1, !dbg !566
  store i32 %10, ptr @_ZL8failures, align 4, !dbg !566
  %11 = load ptr, ptr %4, align 8, !dbg !568
  %12 = call i32 (ptr, ...) @printf(ptr noundef @.str.20, ptr noundef %11), !dbg !569
  br label %13, !dbg !570

13:
  ret void, !dbg !571
}

define linkonce_odr dso_local noundef i32 @_Z4passIiET_S0_(i32 noundef %0) #0 comdat !dbg !572 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
    #dbg_declare(ptr %2, !575, !DIExpression(), !576)
  %3 = load i32, ptr %2, align 4, !dbg !577
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i32) %2, i32 %3) #6, !dbg !577, !srcloc !578
  %4 = load i32, ptr %2, align 4, !dbg !579
  ret i32 %4, !dbg !580
}

define linkonce_odr dso_local noundef i64 @_Z4passIlET_S0_(i64 noundef %0) #0 comdat !dbg !581 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
    #dbg_declare(ptr %2, !584, !DIExpression(), !585)
  %3 = load i64, ptr %2, align 8, !dbg !586
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i64) %2, i64 %3) #6, !dbg !586, !srcloc !578
  %4 = load i64, ptr %2, align 8, !dbg !587
  ret i64 %4, !dbg !588
}

define linkonce_odr dso_local noundef signext i16 @_Z4passIsET_S0_(i16 noundef signext %0) #0 comdat !dbg !589 {
  %2 = alloca i16, align 2
  store i16 %0, ptr %2, align 2
    #dbg_declare(ptr %2, !594, !DIExpression(), !595)
  %3 = load i16, ptr %2, align 2, !dbg !596
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i16) %2, i16 %3) #6, !dbg !596, !srcloc !578
  %4 = load i16, ptr %2, align 2, !dbg !597
  ret i16 %4, !dbg !598
}

define linkonce_odr dso_local noundef signext i8 @_Z4passIcET_S0_(i8 noundef signext %0) #0 comdat !dbg !599 {
  %2 = alloca i8, align 1
  store i8 %0, ptr %2, align 1
    #dbg_declare(ptr %2, !604, !DIExpression(), !605)
  %3 = load i8, ptr %2, align 1, !dbg !606
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i8) %2, i8 %3) #6, !dbg !606, !srcloc !578
  %4 = load i8, ptr %2, align 1, !dbg !607
  ret i8 %4, !dbg !608
}

define linkonce_odr dso_local noundef float @_Z4passIfET_S0_(float noundef %0) #0 comdat !dbg !609 {
  %2 = alloca float, align 4
  store float %0, ptr %2, align 4
    #dbg_declare(ptr %2, !612, !DIExpression(), !613)
  %3 = load float, ptr %2, align 4, !dbg !614
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(float) %2, float %3) #6, !dbg !614, !srcloc !578
  %4 = load float, ptr %2, align 4, !dbg !615
  ret float %4, !dbg !616
}

define linkonce_odr dso_local noundef double @_Z4passIdET_S0_(double noundef %0) #0 comdat !dbg !617 {
  %2 = alloca double, align 8
  store double %0, ptr %2, align 8
    #dbg_declare(ptr %2, !620, !DIExpression(), !621)
  %3 = load double, ptr %2, align 8, !dbg !622
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(double) %2, double %3) #6, !dbg !622, !srcloc !578
  %4 = load double, ptr %2, align 8, !dbg !623
  ret double %4, !dbg !624
}

define linkonce_odr dso_local noundef i128 @_Z4passInET_S0_(i128 noundef %0) #0 comdat !dbg !625 {
  %2 = alloca i128, align 16
  store i128 %0, ptr %2, align 16
    #dbg_declare(ptr %2, !630, !DIExpression(), !631)
  %3 = load i128, ptr %2, align 16, !dbg !632
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(i128) %2, i128 %3) #6, !dbg !632, !srcloc !578
  %4 = load i128, ptr %2, align 16, !dbg !633
  ret i128 %4, !dbg !634
}

define linkonce_odr dso_local noundef ptr @_Z4passIPiET_S1_(ptr noundef %0) #0 comdat !dbg !635 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
    #dbg_declare(ptr %2, !641, !DIExpression(), !642)
  %3 = load ptr, ptr %2, align 8, !dbg !643
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(ptr) %2, ptr %3) #6, !dbg !643, !srcloc !578
  %4 = load ptr, ptr %2, align 8, !dbg !644
  ret ptr %4, !dbg !645
}

define linkonce_odr dso_local noundef <4 x float> @_Z4passIDv4_fET_S1_(<4 x float> noundef %0) #1 comdat !dbg !646 {
  %2 = alloca <4 x float>, align 16
  store <4 x float> %0, ptr %2, align 16
    #dbg_declare(ptr %2, !651, !DIExpression(), !652)
  %3 = load <4 x float>, ptr %2, align 16, !dbg !653
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<4 x float>) %2, <4 x float> %3) #6, !dbg !653, !srcloc !578
  %4 = load <4 x float>, ptr %2, align 16, !dbg !654
  ret <4 x float> %4, !dbg !655
}

define linkonce_odr dso_local noundef <2 x i64> @_Z4passIDv2_xET_S1_(<2 x i64> noundef %0) #1 comdat !dbg !656 {
  %2 = alloca <2 x i64>, align 16
  store <2 x i64> %0, ptr %2, align 16
    #dbg_declare(ptr %2, !661, !DIExpression(), !662)
  %3 = load <2 x i64>, ptr %2, align 16, !dbg !663
  call void asm "", "=*imr|x,0,~{dirflag},~{fpsr},~{flags}"(ptr elementtype(<2 x i64>) %2, <2 x i64> %3) #6, !dbg !663, !srcloc !578
  %4 = load <2 x i64>, ptr %2, align 16, !dbg !664
  ret <2 x i64> %4, !dbg !665
}

declare i32 @printf(ptr noundef, ...) #5

attributes #0 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="128" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { mustprogress noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="256" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+avx,+cmov,+crc32,+cx8,+fxsr,+mmx,+popcnt,+sse,+sse2,+sse3,+sse4.1,+sse4.2,+ssse3,+x87,+xsave" "tune-cpu"="generic" }
attributes #3 = { mustprogress noinline norecurse optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="128" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress noinline optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { nounwind }

!llvm.dbg.cu = !{!7}
!llvm.module.flags = !{!268, !269, !270, !271, !272, !273, !274}
!llvm.ident = !{!275}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "cell", scope: !2, file: !3, line: 104, type: !6, isLocal: true, isDefinition: true)
!2 = distinct !DISubprogram(name: "main", scope: !3, file: !3, line: 96, type: !4, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!3 = !DIFile(filename: "barrier.cpp", directory: "/src/hod/.worktrees/wp-219/crates/hod_x64/tests/data/asm_alternatives", checksumkind: CSK_MD5, checksum: "4832d90765996b682d3592e8aa052415")
!4 = !DISubroutineType(types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!7 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !3, producer: "kiln version 1.1.0", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None, retainedTypes: !8, globals: !20, imports: !23)
!8 = !{!9, !10, !11, !17}
!9 = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
!10 = !DIBasicType(name: "__int128", size: 128, encoding: DW_ATE_signed)
!11 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !12, name: "__m128d", file: !16, line: 19)
!12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 128, flags: DIFlagVector, elements: !14)
!13 = !DIBasicType(name: "double", size: 64, encoding: DW_ATE_float)
!14 = !{!15}
!15 = !DISubrange(count: 2)
!16 = !DIFile(filename: "kiln/resource/include/emmintrin.h", directory: "/src", checksumkind: CSK_MD5, checksum: "3b8cc71610900a890b80a5728ae6f65c")
!17 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !18, name: "__m128i", file: !16, line: 20)
!18 = !DICompositeType(tag: DW_TAG_array_type, baseType: !19, size: 128, flags: DIFlagVector, elements: !14)
!19 = !DIBasicType(name: "long long", size: 64, encoding: DW_ATE_signed)
!20 = !{!0, !21}
!21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
!22 = distinct !DIGlobalVariable(name: "failures", linkageName: "_ZL8failures", scope: !7, file: !3, line: 12, type: !6, isLocal: true, isDefinition: true)
!23 = !{!24, !31, !35, !42, !46, !54, !59, !61, !68, !72, !76, !86, !88, !92, !96, !100, !105, !109, !113, !117, !121, !129, !133, !137, !139, !141, !145, !150, !156, !160, !164, !166, !174, !178, !185, !187, !191, !195, !199, !203, !208, !213, !218, !219, !220, !221, !223, !224, !225, !226, !227, !228, !229, !231, !232, !233, !234, !235, !236, !237, !238, !243, !244, !245, !246, !247, !248, !249, !250, !251, !252, !253, !254, !255, !256, !257, !258, !259, !260, !261, !262, !263, !264, !265, !266}
!24 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !26, file: !30, line: 52)
!25 = !DINamespace(scope: null, name: "std")
!26 = !DISubprogram(name: "abs", scope: !27, file: !27, line: 980, type: !28, flags: DIFlagPrototyped, spFlags: 0)
!27 = !DIFile(filename: "/usr/lib/gcc-cross/x86_64-linux-gnu/14/../../../../x86_64-linux-gnu/include/stdlib.h", directory: "", checksumkind: CSK_MD5, checksum: "775455349f6dd75df6b36d4b094321a4")
!28 = !DISubroutineType(types: !29)
!29 = !{!6, !6}
!30 = !DIFile(filename: "/usr/lib/gcc-cross/x86_64-linux-gnu/14/../../../../x86_64-linux-gnu/include/c++/14/bits/std_abs.h", directory: "", checksumkind: CSK_MD5, checksum: "340d352fec23f6faf0d088b48c6112c6")
!31 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !32, file: !34, line: 131)
!32 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !33, name: "div_t", file: !27, line: 63)
!33 = !DICompositeType(tag: DW_TAG_structure_type, file: !27, line: 59, size: 64, flags: DIFlagFwdDecl, identifier: "_ZTS5div_t")
!34 = !DIFile(filename: "/usr/lib/gcc-cross/x86_64-linux-gnu/14/../../../../x86_64-linux-gnu/include/c++/14/cstdlib", directory: "", checksumkind: CSK_MD5, checksum: "7f5ddbe60caa47fab42e42ad0e6e281b")
!35 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !36, file: !34, line: 132)
!36 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !37, name: "ldiv_t", file: !27, line: 71)
!37 = distinct !DICompositeType(tag: DW_TAG_structure_type, file: !27, line: 67, size: 128, flags: DIFlagTypePassByValue, elements: !38, identifier: "_ZTS6ldiv_t")
!38 = !{!39, !41}
!39 = !DIDerivedType(tag: DW_TAG_member, baseType: !40, name: "quot", scope: !37, file: !27, line: 69, size: 64, offset: 0)
!40 = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
!41 = !DIDerivedType(tag: DW_TAG_member, baseType: !40, name: "rem", scope: !37, file: !27, line: 70, size: 64, offset: 64)
!42 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !43, file: !34, line: 134)
!43 = !DISubprogram(name: "abort", scope: !27, file: !27, line: 730, type: !44, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: 0)
!44 = !DISubroutineType(types: !45)
!45 = !{null}
!46 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !47, file: !34, line: 136)
!47 = !DISubprogram(name: "aligned_alloc", scope: !27, file: !27, line: 724, type: !48, flags: DIFlagPrototyped, spFlags: 0)
!48 = !DISubroutineType(types: !49)
!49 = !{!50, !51, !51}
!50 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: null, size: 64)
!51 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !52, name: "size_t", file: !53, line: 18)
!52 = !DIBasicType(name: "unsigned long", size: 64, encoding: DW_ATE_unsigned)
!53 = !DIFile(filename: "kiln/resource/include/__stddef_size_t.h", directory: "/src", checksumkind: CSK_MD5, checksum: "2c44e821a2b1951cde2eb0fb2e656867")
!54 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !55, file: !34, line: 138)
!55 = !DISubprogram(name: "atexit", scope: !27, file: !27, line: 734, type: !56, flags: DIFlagPrototyped, spFlags: 0)
!56 = !DISubroutineType(types: !57)
!57 = !{!6, !58}
!58 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !44, size: 64)
!59 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !60, file: !34, line: 141)
!60 = !DISubprogram(name: "at_quick_exit", scope: !27, file: !27, line: 739, type: !56, flags: DIFlagPrototyped, spFlags: 0)
!61 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !62, file: !34, line: 144)
!62 = !DISubprogram(name: "atof", scope: !27, file: !27, line: 102, type: !63, flags: DIFlagPrototyped, spFlags: 0)
!63 = !DISubroutineType(types: !64)
!64 = !{!13, !65}
!65 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !66, size: 64)
!66 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !67)
!67 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!68 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !69, file: !34, line: 145)
!69 = !DISubprogram(name: "atoi", scope: !27, file: !27, line: 105, type: !70, flags: DIFlagPrototyped, spFlags: 0)
!70 = !DISubroutineType(types: !71)
!71 = !{!6, !65}
!72 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !73, file: !34, line: 146)
!73 = !DISubprogram(name: "atol", scope: !27, file: !27, line: 108, type: !74, flags: DIFlagPrototyped, spFlags: 0)
!74 = !DISubroutineType(types: !75)
!75 = !{!40, !65}
!76 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !77, file: !34, line: 147)
!77 = !DISubprogram(name: "bsearch", scope: !27, file: !27, line: 960, type: !78, flags: DIFlagPrototyped, spFlags: 0)
!78 = !DISubroutineType(types: !79)
!79 = !{!50, !80, !80, !51, !51, !82}
!80 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !81, size: 64)
!81 = !DIDerivedType(tag: DW_TAG_const_type, baseType: null)
!82 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !83, name: "__compar_fn_t", file: !27, line: 948)
!83 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !84, size: 64)
!84 = !DISubroutineType(types: !85)
!85 = !{!6, !80, !80}
!86 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !87, file: !34, line: 148)
!87 = !DISubprogram(name: "calloc", scope: !27, file: !27, line: 675, type: !48, flags: DIFlagPrototyped, spFlags: 0)
!88 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !89, file: !34, line: 149)
!89 = !DISubprogram(name: "div", scope: !27, file: !27, line: 992, type: !90, flags: DIFlagPrototyped, spFlags: 0)
!90 = !DISubroutineType(types: !91)
!91 = !{!32, !6, !6}
!92 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !93, file: !34, line: 150)
!93 = !DISubprogram(name: "exit", scope: !27, file: !27, line: 756, type: !94, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: 0)
!94 = !DISubroutineType(types: !95)
!95 = !{null, !6}
!96 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !97, file: !34, line: 151)
!97 = !DISubprogram(name: "free", scope: !27, file: !27, line: 687, type: !98, flags: DIFlagPrototyped, spFlags: 0)
!98 = !DISubroutineType(types: !99)
!99 = !{null, !50}
!100 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !101, file: !34, line: 152)
!101 = !DISubprogram(name: "getenv", scope: !27, file: !27, line: 773, type: !102, flags: DIFlagPrototyped, spFlags: 0)
!102 = !DISubroutineType(types: !103)
!103 = !{!104, !65}
!104 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !67, size: 64)
!105 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !106, file: !34, line: 153)
!106 = !DISubprogram(name: "labs", scope: !27, file: !27, line: 981, type: !107, flags: DIFlagPrototyped, spFlags: 0)
!107 = !DISubroutineType(types: !108)
!108 = !{!40, !40}
!109 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !110, file: !34, line: 154)
!110 = !DISubprogram(name: "ldiv", scope: !27, file: !27, line: 994, type: !111, flags: DIFlagPrototyped, spFlags: 0)
!111 = !DISubroutineType(types: !112)
!112 = !{!36, !40, !40}
!113 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !114, file: !34, line: 155)
!114 = !DISubprogram(name: "malloc", scope: !27, file: !27, line: 672, type: !115, flags: DIFlagPrototyped, spFlags: 0)
!115 = !DISubroutineType(types: !116)
!116 = !{!50, !51}
!117 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !118, file: !34, line: 157)
!118 = !DISubprogram(name: "mblen", scope: !27, file: !27, line: 1062, type: !119, flags: DIFlagPrototyped, spFlags: 0)
!119 = !DISubroutineType(types: !120)
!120 = !{!6, !65, !51}
!121 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !122, file: !34, line: 158)
!122 = !DISubprogram(name: "mbstowcs", scope: !27, file: !27, line: 1073, type: !123, flags: DIFlagPrototyped, spFlags: 0)
!123 = !DISubroutineType(types: !124)
!124 = !{!51, !125, !128, !51}
!125 = !DIDerivedType(tag: DW_TAG_restrict_type, baseType: !126)
!126 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !127, size: 64)
!127 = !DIBasicType(name: "wchar_t", size: 32, encoding: DW_ATE_signed)
!128 = !DIDerivedType(tag: DW_TAG_restrict_type, baseType: !65)
!129 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !130, file: !34, line: 159)
!130 = !DISubprogram(name: "mbtowc", scope: !27, file: !27, line: 1065, type: !131, flags: DIFlagPrototyped, spFlags: 0)
!131 = !DISubroutineType(types: !132)
!132 = !{!6, !125, !128, !51}
!133 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !134, file: !34, line: 161)
!134 = !DISubprogram(name: "qsort", scope: !27, file: !27, line: 970, type: !135, flags: DIFlagPrototyped, spFlags: 0)
!135 = !DISubroutineType(types: !136)
!136 = !{null, !50, !51, !51, !82}
!137 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !138, file: !34, line: 164)
!138 = !DISubprogram(name: "quick_exit", scope: !27, file: !27, line: 762, type: !94, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: 0)
!139 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !140, file: !34, line: 167)
!140 = !DISubprogram(name: "rand", scope: !27, file: !27, line: 573, type: !4, flags: DIFlagPrototyped, spFlags: 0)
!141 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !142, file: !34, line: 168)
!142 = !DISubprogram(name: "realloc", scope: !27, file: !27, line: 683, type: !143, flags: DIFlagPrototyped, spFlags: 0)
!143 = !DISubroutineType(types: !144)
!144 = !{!50, !50, !51}
!145 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !146, file: !34, line: 169)
!146 = !DISubprogram(name: "srand", scope: !27, file: !27, line: 575, type: !147, flags: DIFlagPrototyped, spFlags: 0)
!147 = !DISubroutineType(types: !148)
!148 = !{null, !149}
!149 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!150 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !151, file: !34, line: 170)
!151 = !DISubprogram(name: "strtod", scope: !27, file: !27, line: 118, type: !152, flags: DIFlagPrototyped, spFlags: 0)
!152 = !DISubroutineType(types: !153)
!153 = !{!13, !128, !154}
!154 = !DIDerivedType(tag: DW_TAG_restrict_type, baseType: !155)
!155 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !104, size: 64)
!156 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !157, file: !34, line: 171)
!157 = !DISubprogram(name: "strtol", linkageName: "__isoc23_strtol", scope: !27, file: !27, line: 215, type: !158, flags: DIFlagPrototyped, spFlags: 0)
!158 = !DISubroutineType(types: !159)
!159 = !{!40, !128, !154, !6}
!160 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !161, file: !34, line: 172)
!161 = !DISubprogram(name: "strtoul", linkageName: "__isoc23_strtoul", scope: !27, file: !27, line: 219, type: !162, flags: DIFlagPrototyped, spFlags: 0)
!162 = !DISubroutineType(types: !163)
!163 = !{!52, !128, !154, !6}
!164 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !165, file: !34, line: 173)
!165 = !DISubprogram(name: "system", scope: !27, file: !27, line: 923, type: !70, flags: DIFlagPrototyped, spFlags: 0)
!166 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !167, file: !34, line: 175)
!167 = !DISubprogram(name: "wcstombs", scope: !27, file: !27, line: 1077, type: !168, flags: DIFlagPrototyped, spFlags: 0)
!168 = !DISubroutineType(types: !169)
!169 = !{!51, !170, !171, !51}
!170 = !DIDerivedType(tag: DW_TAG_restrict_type, baseType: !104)
!171 = !DIDerivedType(tag: DW_TAG_restrict_type, baseType: !172)
!172 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !173, size: 64)
!173 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !127)
!174 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !175, file: !34, line: 176)
!175 = !DISubprogram(name: "wctomb", scope: !27, file: !27, line: 1069, type: !176, flags: DIFlagPrototyped, spFlags: 0)
!176 = !DISubroutineType(types: !177)
!177 = !{!6, !104, !127}
!178 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !180, file: !34, line: 204)
!179 = !DINamespace(scope: null, name: "__gnu_cxx")
!180 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !181, name: "lldiv_t", file: !27, line: 81)
!181 = distinct !DICompositeType(tag: DW_TAG_structure_type, file: !27, line: 77, size: 128, flags: DIFlagTypePassByValue, elements: !182, identifier: "_ZTS7lldiv_t")
!182 = !{!183, !184}
!183 = !DIDerivedType(tag: DW_TAG_member, baseType: !19, name: "quot", scope: !181, file: !27, line: 79, size: 64, offset: 0)
!184 = !DIDerivedType(tag: DW_TAG_member, baseType: !19, name: "rem", scope: !181, file: !27, line: 80, size: 64, offset: 64)
!185 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !186, file: !34, line: 210)
!186 = !DISubprogram(name: "_Exit", scope: !27, file: !27, line: 768, type: !94, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: 0)
!187 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !188, file: !34, line: 216)
!188 = !DISubprogram(name: "llabs", scope: !27, file: !27, line: 984, type: !189, flags: DIFlagPrototyped, spFlags: 0)
!189 = !DISubroutineType(types: !190)
!190 = !{!19, !19}
!191 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !192, file: !34, line: 222)
!192 = !DISubprogram(name: "lldiv", scope: !27, file: !27, line: 998, type: !193, flags: DIFlagPrototyped, spFlags: 0)
!193 = !DISubroutineType(types: !194)
!194 = !{!180, !19, !19}
!195 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !196, file: !34, line: 234)
!196 = !DISubprogram(name: "atoll", scope: !27, file: !27, line: 113, type: !197, flags: DIFlagPrototyped, spFlags: 0)
!197 = !DISubroutineType(types: !198)
!198 = !{!19, !65}
!199 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !200, file: !34, line: 235)
!200 = !DISubprogram(name: "strtoll", linkageName: "__isoc23_strtoll", scope: !27, file: !27, line: 238, type: !201, flags: DIFlagPrototyped, spFlags: 0)
!201 = !DISubroutineType(types: !202)
!202 = !{!19, !128, !154, !6}
!203 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !204, file: !34, line: 236)
!204 = !DISubprogram(name: "strtoull", linkageName: "__isoc23_strtoull", scope: !27, file: !27, line: 243, type: !205, flags: DIFlagPrototyped, spFlags: 0)
!205 = !DISubroutineType(types: !206)
!206 = !{!207, !128, !154, !6}
!207 = !DIBasicType(name: "unsigned long long", size: 64, encoding: DW_ATE_unsigned)
!208 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !209, file: !34, line: 238)
!209 = !DISubprogram(name: "strtof", scope: !27, file: !27, line: 124, type: !210, flags: DIFlagPrototyped, spFlags: 0)
!210 = !DISubroutineType(types: !211)
!211 = !{!212, !128, !154}
!212 = !DIBasicType(name: "float", size: 32, encoding: DW_ATE_float)
!213 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !179, entity: !214, file: !34, line: 239)
!214 = !DISubprogram(name: "strtold", scope: !27, file: !27, line: 127, type: !215, flags: DIFlagPrototyped, spFlags: 0)
!215 = !DISubroutineType(types: !216)
!216 = !{!217, !128, !154}
!217 = !DIBasicType(name: "long double", size: 128, encoding: DW_ATE_float)
!218 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !180, file: !34, line: 247)
!219 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !186, file: !34, line: 249)
!220 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !188, file: !34, line: 251)
!221 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !222, file: !34, line: 252)
!222 = !DISubprogram(name: "div", linkageName: "_ZN9__gnu_cxx3divExx", scope: !179, file: !34, line: 219, type: !193, flags: DIFlagPrototyped, spFlags: 0)
!223 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !192, file: !34, line: 253)
!224 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !196, file: !34, line: 255)
!225 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !209, file: !34, line: 256)
!226 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !200, file: !34, line: 257)
!227 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !204, file: !34, line: 258)
!228 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !25, entity: !214, file: !34, line: 259)
!229 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !43, file: !230, line: 38)
!230 = !DIFile(filename: "/usr/lib/gcc-cross/x86_64-linux-gnu/14/../../../../x86_64-linux-gnu/include/c++/14/stdlib.h", directory: "", checksumkind: CSK_MD5, checksum: "1f96f7dcc55c9928a1405539766e004f")
!231 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !55, file: !230, line: 39)
!232 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !93, file: !230, line: 40)
!233 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !60, file: !230, line: 43)
!234 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !138, file: !230, line: 46)
!235 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !186, file: !230, line: 49)
!236 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !32, file: !230, line: 54)
!237 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !36, file: !230, line: 55)
!238 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !239, file: !230, line: 57)
!239 = !DISubprogram(name: "abs", linkageName: "_ZSt3absg", scope: !25, file: !30, line: 137, type: !240, flags: DIFlagPrototyped, spFlags: 0)
!240 = !DISubroutineType(types: !241)
!241 = !{!242, !242}
!242 = !DIBasicType(name: "__float128", size: 128, encoding: DW_ATE_float)
!243 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !62, file: !230, line: 58)
!244 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !69, file: !230, line: 59)
!245 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !73, file: !230, line: 60)
!246 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !77, file: !230, line: 61)
!247 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !87, file: !230, line: 62)
!248 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !222, file: !230, line: 63)
!249 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !97, file: !230, line: 64)
!250 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !101, file: !230, line: 65)
!251 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !106, file: !230, line: 66)
!252 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !110, file: !230, line: 67)
!253 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !114, file: !230, line: 68)
!254 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !118, file: !230, line: 70)
!255 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !122, file: !230, line: 71)
!256 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !130, file: !230, line: 72)
!257 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !134, file: !230, line: 74)
!258 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !140, file: !230, line: 75)
!259 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !142, file: !230, line: 76)
!260 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !146, file: !230, line: 77)
!261 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !151, file: !230, line: 78)
!262 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !157, file: !230, line: 79)
!263 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !161, file: !230, line: 80)
!264 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !165, file: !230, line: 81)
!265 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !167, file: !230, line: 83)
!266 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !7, entity: !175, file: !230, line: 84)
!267 = !{}
!268 = !{i32 7, !"Dwarf Version", i32 5}
!269 = !{i32 2, !"Debug Info Version", i32 3}
!270 = !{i32 1, !"wchar_size", i32 4}
!271 = !{i32 8, !"PIC Level", i32 2}
!272 = !{i32 7, !"PIE Level", i32 2}
!273 = !{i32 7, !"uwtable", i32 2}
!274 = !{i32 7, !"frame-pointer", i32 2}
!275 = !{!"kiln version 1.1.0"}
!276 = distinct !DISubprogram(name: "twice_f", linkageName: "_Z7twice_ff", scope: !3, file: !3, line: 26, type: !277, scopeLine: 26, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!277 = !DISubroutineType(types: !278)
!278 = !{!212, !212}
!279 = !DILocalVariable(name: "x", arg: 1, scope: !276, file: !3, line: 26, type: !212)
!280 = !DILocation(line: 26, column: 47, scope: !276)
!281 = !DILocalVariable(name: "y", scope: !276, file: !3, line: 27, type: !212)
!282 = !DILocation(line: 27, column: 9, scope: !276)
!283 = !DILocation(line: 27, column: 13, scope: !276)
!284 = !DILocation(line: 27, column: 17, scope: !276)
!285 = !DILocation(line: 27, column: 15, scope: !276)
!286 = !DILocation(line: 28, column: 3, scope: !276)
!287 = !{i64 759}
!288 = !DILocation(line: 29, column: 10, scope: !276)
!289 = !DILocation(line: 29, column: 12, scope: !276)
!290 = !DILocation(line: 29, column: 3, scope: !276)
!291 = distinct !DISubprogram(name: "add_ps", linkageName: "_Z6add_psDv4_fS_", scope: !3, file: !3, line: 32, type: !292, scopeLine: 32, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!292 = !DISubroutineType(types: !293)
!293 = !{!294, !294, !294}
!294 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !295, name: "__m128", file: !298, line: 21)
!295 = !DICompositeType(tag: DW_TAG_array_type, baseType: !212, size: 128, flags: DIFlagVector, elements: !296)
!296 = !{!297}
!297 = !DISubrange(count: 4)
!298 = !DIFile(filename: "kiln/resource/include/xmmintrin.h", directory: "/src", checksumkind: CSK_MD5, checksum: "6d1f3318befe69a245cc0ddc3ca336d4")
!299 = !DILocalVariable(name: "a", arg: 1, scope: !291, file: !3, line: 32, type: !294)
!300 = !DILocation(line: 32, column: 48, scope: !291)
!301 = !DILocalVariable(name: "b", arg: 2, scope: !291, file: !3, line: 32, type: !294)
!302 = !DILocation(line: 32, column: 58, scope: !291)
!303 = !DILocalVariable(name: "c", scope: !291, file: !3, line: 33, type: !294)
!304 = !DILocation(line: 33, column: 10, scope: !291)
!305 = !DILocation(line: 33, column: 14, scope: !291)
!306 = !DILocation(line: 33, column: 18, scope: !291)
!307 = !DILocation(line: 33, column: 16, scope: !291)
!308 = !DILocation(line: 34, column: 3, scope: !291)
!309 = !{i64 877}
!310 = !DILocation(line: 35, column: 10, scope: !291)
!311 = !DILocation(line: 35, column: 14, scope: !291)
!312 = !DILocation(line: 35, column: 12, scope: !291)
!313 = !DILocation(line: 35, column: 3, scope: !291)
!314 = distinct !DISubprogram(name: "add_pd", linkageName: "_Z6add_pdDv2_dS_", scope: !3, file: !3, line: 38, type: !315, scopeLine: 38, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!315 = !DISubroutineType(types: !316)
!316 = !{!11, !11, !11}
!317 = !DILocalVariable(name: "a", arg: 1, scope: !314, file: !3, line: 38, type: !11)
!318 = !DILocation(line: 38, column: 50, scope: !314)
!319 = !DILocalVariable(name: "b", arg: 2, scope: !314, file: !3, line: 38, type: !11)
!320 = !DILocation(line: 38, column: 61, scope: !314)
!321 = !DILocalVariable(name: "c", scope: !314, file: !3, line: 39, type: !11)
!322 = !DILocation(line: 39, column: 11, scope: !314)
!323 = !DILocation(line: 39, column: 15, scope: !314)
!324 = !DILocation(line: 39, column: 19, scope: !314)
!325 = !DILocation(line: 39, column: 17, scope: !314)
!326 = !DILocation(line: 40, column: 3, scope: !314)
!327 = !{i64 996}
!328 = !DILocation(line: 41, column: 10, scope: !314)
!329 = !DILocation(line: 41, column: 14, scope: !314)
!330 = !DILocation(line: 41, column: 12, scope: !314)
!331 = !DILocation(line: 41, column: 3, scope: !314)
!332 = distinct !DISubprogram(name: "add_ps8", linkageName: "_Z7add_ps8Dv8_fS_", scope: !3, file: !3, line: 44, type: !333, scopeLine: 44, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!333 = !DISubroutineType(types: !334)
!334 = !{!335, !335, !335}
!335 = !DIDerivedType(tag: DW_TAG_typedef, baseType: !336, name: "__m256", file: !339, line: 34)
!336 = !DICompositeType(tag: DW_TAG_array_type, baseType: !212, size: 256, flags: DIFlagVector, elements: !337)
!337 = !{!338}
!338 = !DISubrange(count: 8)
!339 = !DIFile(filename: "kiln/resource/include/avxintrin.h", directory: "/src", checksumkind: CSK_MD5, checksum: "a79eac168c6d1e8feccd7c4cbd96d839")
!340 = !DILocalVariable(name: "a", arg: 1, scope: !332, file: !3, line: 44, type: !335)
!341 = !DILocation(line: 44, column: 64, scope: !332)
!342 = !DILocalVariable(name: "b", arg: 2, scope: !332, file: !3, line: 44, type: !335)
!343 = !DILocation(line: 44, column: 74, scope: !332)
!344 = !DILocalVariable(name: "c", scope: !332, file: !3, line: 45, type: !335)
!345 = !DILocation(line: 45, column: 10, scope: !332)
!346 = !DILocation(line: 45, column: 14, scope: !332)
!347 = !DILocation(line: 45, column: 18, scope: !332)
!348 = !DILocation(line: 45, column: 16, scope: !332)
!349 = !DILocation(line: 46, column: 3, scope: !332)
!350 = !{i64 1127}
!351 = !DILocation(line: 47, column: 10, scope: !332)
!352 = !DILocation(line: 47, column: 3, scope: !332)
!353 = distinct !DISubprogram(name: "sum8", linkageName: "_Z4sum8f", scope: !3, file: !3, line: 50, type: !277, scopeLine: 50, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!354 = !DILocalVariable(name: "base", arg: 1, scope: !353, file: !3, line: 50, type: !212)
!355 = !DILocation(line: 50, column: 59, scope: !353)
!356 = !DILocalVariable(name: "v", scope: !353, file: !3, line: 51, type: !335)
!357 = !DILocation(line: 51, column: 10, scope: !353)
!358 = !DILocation(line: 51, column: 15, scope: !353)
!359 = !DILocation(line: 51, column: 14, scope: !353)
!360 = !DILocation(line: 51, column: 21, scope: !353)
!361 = !DILocation(line: 51, column: 26, scope: !353)
!362 = !DILocation(line: 51, column: 31, scope: !353)
!363 = !DILocation(line: 51, column: 36, scope: !353)
!364 = !DILocation(line: 51, column: 41, scope: !353)
!365 = !DILocation(line: 51, column: 46, scope: !353)
!366 = !DILocation(line: 51, column: 51, scope: !353)
!367 = !DILocation(line: 51, column: 56, scope: !353)
!368 = !DILocation(line: 51, column: 61, scope: !353)
!369 = !DILocation(line: 51, column: 66, scope: !353)
!370 = !DILocation(line: 51, column: 71, scope: !353)
!371 = !DILocation(line: 51, column: 76, scope: !353)
!372 = !DILocation(line: 51, column: 81, scope: !353)
!373 = !DILocation(line: 51, column: 86, scope: !353)
!374 = !DILocalVariable(name: "w", scope: !353, file: !3, line: 52, type: !335)
!375 = !DILocation(line: 52, column: 10, scope: !353)
!376 = !DILocation(line: 52, column: 22, scope: !353)
!377 = !DILocation(line: 52, column: 25, scope: !353)
!378 = !DILocation(line: 52, column: 14, scope: !353)
!379 = !DILocalVariable(name: "s", scope: !353, file: !3, line: 53, type: !212)
!380 = !DILocation(line: 53, column: 9, scope: !353)
!381 = !DILocalVariable(name: "i", scope: !382, file: !3, line: 54, type: !6)
!382 = distinct !DILexicalBlock(scope: !353, file: !3, line: 54, column: 3)
!383 = !DILocation(line: 54, column: 12, scope: !382)
!384 = !DILocation(line: 54, column: 8, scope: !382)
!385 = !DILocation(line: 54, column: 19, scope: !386)
!386 = distinct !DILexicalBlock(scope: !382, file: !3, line: 54, column: 3)
!387 = !DILocation(line: 54, column: 21, scope: !386)
!388 = !DILocation(line: 54, column: 3, scope: !382)
!389 = !DILocation(line: 55, column: 10, scope: !386)
!390 = !DILocation(line: 55, column: 12, scope: !386)
!391 = !DILocation(line: 55, column: 7, scope: !386)
!392 = !DILocation(line: 55, column: 5, scope: !386)
!393 = !DILocation(line: 54, column: 26, scope: !386)
!394 = !DILocation(line: 54, column: 3, scope: !386)
!395 = distinct !{!395, !388, !396, !397}
!396 = !DILocation(line: 55, column: 13, scope: !382)
!397 = !{!"llvm.loop.mustprogress"}
!398 = !DILocation(line: 56, column: 10, scope: !353)
!399 = !DILocation(line: 56, column: 3, scope: !353)
!400 = distinct !DISubprogram(name: "inc_g", linkageName: "_Z5inc_gi", scope: !3, file: !3, line: 61, type: !28, scopeLine: 61, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!401 = !DILocalVariable(name: "x", arg: 1, scope: !400, file: !3, line: 61, type: !6)
!402 = !DILocation(line: 61, column: 41, scope: !400)
!403 = !DILocation(line: 62, column: 3, scope: !400)
!404 = !{i64 1558}
!405 = !DILocation(line: 63, column: 10, scope: !400)
!406 = !DILocation(line: 63, column: 3, scope: !400)
!407 = distinct !DISubprogram(name: "copy_rm", linkageName: "_Z7copy_rmi", scope: !3, file: !3, line: 68, type: !28, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!408 = !DILocalVariable(name: "in", arg: 1, scope: !407, file: !3, line: 68, type: !6)
!409 = !DILocation(line: 68, column: 43, scope: !407)
!410 = !DILocalVariable(name: "out", scope: !407, file: !3, line: 69, type: !6)
!411 = !DILocation(line: 69, column: 7, scope: !407)
!412 = !DILocation(line: 70, column: 48, scope: !407)
!413 = !DILocation(line: 70, column: 3, scope: !407)
!414 = !{i64 1806}
!415 = !DILocation(line: 71, column: 10, scope: !407)
!416 = !DILocation(line: 71, column: 3, scope: !407)
!417 = distinct !DISubprogram(name: "copy_ss", linkageName: "_Z7copy_ssf", scope: !3, file: !3, line: 75, type: !277, scopeLine: 75, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!418 = !DILocalVariable(name: "in", arg: 1, scope: !417, file: !3, line: 75, type: !212)
!419 = !DILocation(line: 75, column: 47, scope: !417)
!420 = !DILocalVariable(name: "out", scope: !417, file: !3, line: 76, type: !212)
!421 = !DILocation(line: 76, column: 9, scope: !417)
!422 = !DILocation(line: 77, column: 48, scope: !417)
!423 = !DILocation(line: 77, column: 3, scope: !417)
!424 = !{i64 2009}
!425 = !DILocation(line: 78, column: 10, scope: !417)
!426 = !DILocation(line: 78, column: 3, scope: !417)
!427 = distinct !DISubprogram(name: "add_r", linkageName: "_Z5add_rll", scope: !3, file: !3, line: 84, type: !428, scopeLine: 84, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!428 = !DISubroutineType(types: !429)
!429 = !{!40, !40, !40}
!430 = !DILocalVariable(name: "a", arg: 1, scope: !427, file: !3, line: 84, type: !40)
!431 = !DILocation(line: 84, column: 43, scope: !427)
!432 = !DILocalVariable(name: "b", arg: 2, scope: !427, file: !3, line: 84, type: !40)
!433 = !DILocation(line: 84, column: 51, scope: !427)
!434 = !DILocation(line: 85, column: 3, scope: !427)
!435 = !DILocation(line: 85, column: 45, scope: !427)
!436 = !{i64 2317}
!437 = !DILocation(line: 86, column: 10, scope: !427)
!438 = !DILocation(line: 86, column: 3, scope: !427)
!439 = distinct !DISubprogram(name: "keep", linkageName: "_Z4keepd", scope: !3, file: !3, line: 90, type: !440, scopeLine: 90, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !267)
!440 = !DISubroutineType(types: !441)
!441 = !{!13, !13}
!442 = !DILocalVariable(name: "d", arg: 1, scope: !439, file: !3, line: 90, type: !13)
!443 = !DILocation(line: 90, column: 46, scope: !439)
!444 = !DILocation(line: 91, column: 37, scope: !439)
!445 = !DILocation(line: 91, column: 3, scope: !439)
!446 = !{i64 2480}
!447 = !DILocation(line: 92, column: 37, scope: !439)
!448 = !DILocation(line: 92, column: 3, scope: !439)
!449 = !{i64 2532}
!450 = !DILocation(line: 93, column: 10, scope: !439)
!451 = !DILocation(line: 93, column: 12, scope: !439)
!452 = !DILocation(line: 93, column: 3, scope: !439)
!453 = !DILocation(line: 97, column: 9, scope: !2)
!454 = !DILocation(line: 97, column: 18, scope: !2)
!455 = !DILocation(line: 97, column: 3, scope: !2)
!456 = !DILocation(line: 98, column: 9, scope: !2)
!457 = !DILocation(line: 98, column: 19, scope: !2)
!458 = !DILocation(line: 98, column: 3, scope: !2)
!459 = !DILocation(line: 99, column: 9, scope: !2)
!460 = !DILocation(line: 99, column: 25, scope: !2)
!461 = !DILocation(line: 99, column: 3, scope: !2)
!462 = !DILocation(line: 100, column: 9, scope: !2)
!463 = !DILocation(line: 100, column: 19, scope: !2)
!464 = !DILocation(line: 100, column: 3, scope: !2)
!465 = !DILocation(line: 101, column: 9, scope: !2)
!466 = !DILocation(line: 101, column: 20, scope: !2)
!467 = !DILocation(line: 101, column: 3, scope: !2)
!468 = !DILocation(line: 102, column: 9, scope: !2)
!469 = !DILocation(line: 102, column: 22, scope: !2)
!470 = !DILocation(line: 102, column: 3, scope: !2)
!471 = !DILocation(line: 103, column: 9, scope: !2)
!472 = !DILocation(line: 103, column: 34, scope: !2)
!473 = !DILocation(line: 103, column: 3, scope: !2)
!474 = !DILocation(line: 105, column: 9, scope: !2)
!475 = !DILocation(line: 105, column: 21, scope: !2)
!476 = !DILocation(line: 105, column: 30, scope: !2)
!477 = !DILocation(line: 105, column: 34, scope: !2)
!478 = !DILocation(line: 105, column: 33, scope: !2)
!479 = !DILocation(line: 105, column: 46, scope: !2)
!480 = !DILocation(line: 0, scope: !2)
!481 = !DILocation(line: 105, column: 3, scope: !2)
!482 = !DILocation(line: 106, column: 9, scope: !2)
!483 = !DILocation(line: 106, column: 23, scope: !2)
!484 = !DILocation(line: 106, column: 3, scope: !2)
!485 = !DILocalVariable(name: "a", scope: !2, file: !3, line: 107, type: !294)
!486 = !DILocation(line: 107, column: 10, scope: !2)
!487 = !DILocalVariable(name: "b", scope: !2, file: !3, line: 107, type: !294)
!488 = !DILocation(line: 107, column: 28, scope: !2)
!489 = !DILocalVariable(name: "c", scope: !2, file: !3, line: 108, type: !294)
!490 = !DILocation(line: 108, column: 10, scope: !2)
!491 = !DILocation(line: 108, column: 21, scope: !2)
!492 = !DILocation(line: 108, column: 24, scope: !2)
!493 = !DILocation(line: 108, column: 14, scope: !2)
!494 = !DILocation(line: 109, column: 9, scope: !2)
!495 = !DILocation(line: 109, column: 14, scope: !2)
!496 = !DILocation(line: 109, column: 48, scope: !2)
!497 = !DILocation(line: 109, column: 23, scope: !2)
!498 = !DILocation(line: 109, column: 28, scope: !2)
!499 = !DILocation(line: 109, column: 37, scope: !2)
!500 = !DILocation(line: 109, column: 42, scope: !2)
!501 = !DILocation(line: 109, column: 51, scope: !2)
!502 = !DILocation(line: 109, column: 56, scope: !2)
!503 = !DILocation(line: 109, column: 3, scope: !2)
!504 = !DILocalVariable(name: "p", scope: !2, file: !3, line: 110, type: !294)
!505 = !DILocation(line: 110, column: 10, scope: !2)
!506 = !DILocation(line: 110, column: 19, scope: !2)
!507 = !DILocation(line: 110, column: 14, scope: !2)
!508 = !DILocation(line: 111, column: 9, scope: !2)
!509 = !DILocation(line: 111, column: 14, scope: !2)
!510 = !DILocation(line: 111, column: 19, scope: !2)
!511 = !DILocation(line: 111, column: 22, scope: !2)
!512 = !DILocation(line: 111, column: 27, scope: !2)
!513 = !DILocation(line: 111, column: 3, scope: !2)
!514 = !DILocalVariable(name: "d", scope: !2, file: !3, line: 112, type: !11)
!515 = !DILocation(line: 112, column: 11, scope: !2)
!516 = !DILocation(line: 112, column: 15, scope: !2)
!517 = !DILocation(line: 113, column: 9, scope: !2)
!518 = !DILocation(line: 113, column: 14, scope: !2)
!519 = !DILocation(line: 113, column: 21, scope: !2)
!520 = !DILocation(line: 113, column: 24, scope: !2)
!521 = !DILocation(line: 113, column: 29, scope: !2)
!522 = !DILocation(line: 113, column: 3, scope: !2)
!523 = !DILocalVariable(name: "i", scope: !2, file: !3, line: 114, type: !17)
!524 = !DILocation(line: 114, column: 11, scope: !2)
!525 = !DILocation(line: 114, column: 15, scope: !2)
!526 = !DILocation(line: 115, column: 9, scope: !2)
!527 = !DILocation(line: 115, column: 14, scope: !2)
!528 = !DILocation(line: 115, column: 19, scope: !2)
!529 = !DILocation(line: 115, column: 22, scope: !2)
!530 = !DILocation(line: 115, column: 27, scope: !2)
!531 = !DILocation(line: 115, column: 3, scope: !2)
!532 = !DILocation(line: 116, column: 9, scope: !2)
!533 = !DILocation(line: 116, column: 17, scope: !2)
!534 = !DILocation(line: 116, column: 3, scope: !2)
!535 = !DILocation(line: 117, column: 9, scope: !2)
!536 = !DILocation(line: 117, column: 19, scope: !2)
!537 = !DILocation(line: 117, column: 3, scope: !2)
!538 = !DILocation(line: 118, column: 9, scope: !2)
!539 = !DILocation(line: 118, column: 23, scope: !2)
!540 = !DILocation(line: 118, column: 3, scope: !2)
!541 = !DILocation(line: 119, column: 9, scope: !2)
!542 = !DILocation(line: 119, column: 23, scope: !2)
!543 = !DILocation(line: 119, column: 3, scope: !2)
!544 = !DILocation(line: 120, column: 9, scope: !2)
!545 = !DILocation(line: 120, column: 22, scope: !2)
!546 = !DILocation(line: 120, column: 3, scope: !2)
!547 = !DILocation(line: 121, column: 9, scope: !2)
!548 = !DILocation(line: 121, column: 20, scope: !2)
!549 = !DILocation(line: 121, column: 3, scope: !2)
!550 = !DILocation(line: 122, column: 36, scope: !2)
!551 = !DILocation(line: 122, column: 3, scope: !2)
!552 = !DILocation(line: 123, column: 10, scope: !2)
!553 = !DILocation(line: 123, column: 19, scope: !2)
!554 = !DILocation(line: 123, column: 3, scope: !2)
!555 = distinct !DISubprogram(name: "check", linkageName: "_ZL5checkbPKc", scope: !3, file: !3, line: 14, type: !556, scopeLine: 14, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, retainedNodes: !267)
!556 = !DISubroutineType(types: !557)
!557 = !{null, !558, !65}
!558 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!559 = !DILocalVariable(name: "ok", arg: 1, scope: !555, file: !3, line: 14, type: !558)
!560 = !DILocation(line: 14, column: 24, scope: !555)
!561 = !DILocalVariable(name: "what", arg: 2, scope: !555, file: !3, line: 14, type: !65)
!562 = !DILocation(line: 14, column: 40, scope: !555)
!563 = !DILocation(line: 15, column: 8, scope: !564)
!564 = distinct !DILexicalBlock(scope: !555, file: !3, line: 15, column: 7)
!565 = !DILocation(line: 15, column: 7, scope: !555)
!566 = !DILocation(line: 16, column: 5, scope: !567)
!567 = distinct !DILexicalBlock(scope: !564, file: !3, line: 15, column: 12)
!568 = !DILocation(line: 17, column: 27, scope: !567)
!569 = !DILocation(line: 17, column: 5, scope: !567)
!570 = !DILocation(line: 18, column: 3, scope: !567)
!571 = !DILocation(line: 19, column: 1, scope: !555)
!572 = distinct !DISubprogram(name: "pass<int>", linkageName: "_Z4passIiET_S0_", scope: !3, file: !3, line: 21, type: !28, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !573, retainedNodes: !267)
!573 = !{!574}
!574 = !DITemplateTypeParameter(name: "T", type: !6)
!575 = !DILocalVariable(name: "x", arg: 1, scope: !572, file: !3, line: 21, type: !6)
!576 = !DILocation(line: 21, column: 55, scope: !572)
!577 = !DILocation(line: 22, column: 3, scope: !572)
!578 = !{i64 660}
!579 = !DILocation(line: 23, column: 10, scope: !572)
!580 = !DILocation(line: 23, column: 3, scope: !572)
!581 = distinct !DISubprogram(name: "pass<long>", linkageName: "_Z4passIlET_S0_", scope: !3, file: !3, line: 21, type: !107, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !582, retainedNodes: !267)
!582 = !{!583}
!583 = !DITemplateTypeParameter(name: "T", type: !40)
!584 = !DILocalVariable(name: "x", arg: 1, scope: !581, file: !3, line: 21, type: !40)
!585 = !DILocation(line: 21, column: 55, scope: !581)
!586 = !DILocation(line: 22, column: 3, scope: !581)
!587 = !DILocation(line: 23, column: 10, scope: !581)
!588 = !DILocation(line: 23, column: 3, scope: !581)
!589 = distinct !DISubprogram(name: "pass<short>", linkageName: "_Z4passIsET_S0_", scope: !3, file: !3, line: 21, type: !590, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !592, retainedNodes: !267)
!590 = !DISubroutineType(types: !591)
!591 = !{!9, !9}
!592 = !{!593}
!593 = !DITemplateTypeParameter(name: "T", type: !9)
!594 = !DILocalVariable(name: "x", arg: 1, scope: !589, file: !3, line: 21, type: !9)
!595 = !DILocation(line: 21, column: 55, scope: !589)
!596 = !DILocation(line: 22, column: 3, scope: !589)
!597 = !DILocation(line: 23, column: 10, scope: !589)
!598 = !DILocation(line: 23, column: 3, scope: !589)
!599 = distinct !DISubprogram(name: "pass<char>", linkageName: "_Z4passIcET_S0_", scope: !3, file: !3, line: 21, type: !600, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !602, retainedNodes: !267)
!600 = !DISubroutineType(types: !601)
!601 = !{!67, !67}
!602 = !{!603}
!603 = !DITemplateTypeParameter(name: "T", type: !67)
!604 = !DILocalVariable(name: "x", arg: 1, scope: !599, file: !3, line: 21, type: !67)
!605 = !DILocation(line: 21, column: 55, scope: !599)
!606 = !DILocation(line: 22, column: 3, scope: !599)
!607 = !DILocation(line: 23, column: 10, scope: !599)
!608 = !DILocation(line: 23, column: 3, scope: !599)
!609 = distinct !DISubprogram(name: "pass<float>", linkageName: "_Z4passIfET_S0_", scope: !3, file: !3, line: 21, type: !277, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !610, retainedNodes: !267)
!610 = !{!611}
!611 = !DITemplateTypeParameter(name: "T", type: !212)
!612 = !DILocalVariable(name: "x", arg: 1, scope: !609, file: !3, line: 21, type: !212)
!613 = !DILocation(line: 21, column: 55, scope: !609)
!614 = !DILocation(line: 22, column: 3, scope: !609)
!615 = !DILocation(line: 23, column: 10, scope: !609)
!616 = !DILocation(line: 23, column: 3, scope: !609)
!617 = distinct !DISubprogram(name: "pass<double>", linkageName: "_Z4passIdET_S0_", scope: !3, file: !3, line: 21, type: !440, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !618, retainedNodes: !267)
!618 = !{!619}
!619 = !DITemplateTypeParameter(name: "T", type: !13)
!620 = !DILocalVariable(name: "x", arg: 1, scope: !617, file: !3, line: 21, type: !13)
!621 = !DILocation(line: 21, column: 55, scope: !617)
!622 = !DILocation(line: 22, column: 3, scope: !617)
!623 = !DILocation(line: 23, column: 10, scope: !617)
!624 = !DILocation(line: 23, column: 3, scope: !617)
!625 = distinct !DISubprogram(name: "pass<__int128>", linkageName: "_Z4passInET_S0_", scope: !3, file: !3, line: 21, type: !626, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !628, retainedNodes: !267)
!626 = !DISubroutineType(types: !627)
!627 = !{!10, !10}
!628 = !{!629}
!629 = !DITemplateTypeParameter(name: "T", type: !10)
!630 = !DILocalVariable(name: "x", arg: 1, scope: !625, file: !3, line: 21, type: !10)
!631 = !DILocation(line: 21, column: 55, scope: !625)
!632 = !DILocation(line: 22, column: 3, scope: !625)
!633 = !DILocation(line: 23, column: 10, scope: !625)
!634 = !DILocation(line: 23, column: 3, scope: !625)
!635 = distinct !DISubprogram(name: "pass<int *>", linkageName: "_Z4passIPiET_S1_", scope: !3, file: !3, line: 21, type: !636, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !639, retainedNodes: !267)
!636 = !DISubroutineType(types: !637)
!637 = !{!638, !638}
!638 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !6, size: 64)
!639 = !{!640}
!640 = !DITemplateTypeParameter(name: "T", type: !638)
!641 = !DILocalVariable(name: "x", arg: 1, scope: !635, file: !3, line: 21, type: !638)
!642 = !DILocation(line: 21, column: 55, scope: !635)
!643 = !DILocation(line: 22, column: 3, scope: !635)
!644 = !DILocation(line: 23, column: 10, scope: !635)
!645 = !DILocation(line: 23, column: 3, scope: !635)
!646 = distinct !DISubprogram(name: "pass<__attribute__((__vector_size__(4 * sizeof(float)))) float>", linkageName: "_Z4passIDv4_fET_S1_", scope: !3, file: !3, line: 21, type: !647, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !649, retainedNodes: !267)
!647 = !DISubroutineType(types: !648)
!648 = !{!295, !295}
!649 = !{!650}
!650 = !DITemplateTypeParameter(name: "T", type: !295)
!651 = !DILocalVariable(name: "x", arg: 1, scope: !646, file: !3, line: 21, type: !295)
!652 = !DILocation(line: 21, column: 55, scope: !646)
!653 = !DILocation(line: 22, column: 3, scope: !646)
!654 = !DILocation(line: 23, column: 10, scope: !646)
!655 = !DILocation(line: 23, column: 3, scope: !646)
!656 = distinct !DISubprogram(name: "pass<__attribute__((__vector_size__(2 * sizeof(long long)))) long long>", linkageName: "_Z4passIDv2_xET_S1_", scope: !3, file: !3, line: 21, type: !657, scopeLine: 21, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !659, retainedNodes: !267)
!657 = !DISubroutineType(types: !658)
!658 = !{!18, !18}
!659 = !{!660}
!660 = !DITemplateTypeParameter(name: "T", type: !18)
!661 = !DILocalVariable(name: "x", arg: 1, scope: !656, file: !3, line: 21, type: !18)
!662 = !DILocation(line: 21, column: 55, scope: !656)
!663 = !DILocation(line: 22, column: 3, scope: !656)
!664 = !DILocation(line: 23, column: 10, scope: !656)
!665 = !DILocation(line: 23, column: 3, scope: !656)

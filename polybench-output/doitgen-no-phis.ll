; ModuleID = './polybench-output/doitgen-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/doitgen/doitgen.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"A\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local void @kernel_doitgen(i32 noundef %0, i32 noundef %1, i32 noundef %2, ptr nocapture noundef %3, ptr nocapture noundef readonly %4, ptr nocapture noundef %5) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem16 = alloca i64, align 8
  %.reg2mem19 = alloca i64, align 8
  %.reg2mem22 = alloca i64, align 8
  %.reg2mem25 = alloca i64, align 8
  %.reg2mem28 = alloca double, align 8
  %.reg2mem31 = alloca ptr, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem51 = alloca i64, align 8
  %.reg2mem53 = alloca i64, align 8
  %.reg2mem55 = alloca i1, align 1
  %.reg2mem58 = alloca i1, align 1
  %.reg2mem60 = alloca i64, align 8
  %.reg2mem62 = alloca double, align 8
  %.reg2mem64 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem68 = alloca i64, align 8
  %.reg2mem70 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %7 = icmp sgt i32 %0, 0
  br i1 %7, label %8, label %._crit_edge

._crit_edge:                                      ; preds = %6
  br label %43

8:                                                ; preds = %6
  %9 = icmp sgt i32 %1, 0
  store i1 %9, ptr %.reg2mem58, align 1
  %10 = icmp sgt i32 %2, 0
  store i1 %10, ptr %.reg2mem55, align 1
  %11 = zext nneg i32 %0 to i64
  store i64 %11, ptr %.reg2mem53, align 8
  %12 = zext nneg i32 %1 to i64
  store i64 %12, ptr %.reg2mem51, align 8
  %13 = zext nneg i32 %2 to i64
  store i64 %13, ptr %.reg2mem48, align 8
  %14 = zext nneg i32 %2 to i64
  store i64 %14, ptr %.reg2mem46, align 8
  store i64 0, ptr %.reg2mem70, align 8
  br label %15

15:                                               ; preds = %._crit_edge14, %8
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  store i64 %.reload71, ptr %.reg2mem42, align 8
  %.reload59 = load i1, ptr %.reg2mem58, align 1
  br i1 %.reload59, label %._crit_edge1, label %._crit_edge2

._crit_edge2:                                     ; preds = %15
  br label %40

._crit_edge1:                                     ; preds = %15
  store i64 0, ptr %.reg2mem68, align 8
  br label %16

16:                                               ; preds = %._crit_edge12, %._crit_edge1
  %.reload69 = load i64, ptr %.reg2mem68, align 8
  store i64 %.reload69, ptr %.reg2mem38, align 8
  %.reload57 = load i1, ptr %.reg2mem55, align 1
  br i1 %.reload57, label %._crit_edge3, label %._crit_edge4

._crit_edge4:                                     ; preds = %16
  br label %37

._crit_edge3:                                     ; preds = %16
  store i64 0, ptr %.reg2mem66, align 8
  br label %18

17:                                               ; preds = %28
  %.reload56 = load i1, ptr %.reg2mem55, align 1
  br i1 %.reload56, label %._crit_edge5, label %._crit_edge6

._crit_edge6:                                     ; preds = %17
  br label %37

._crit_edge5:                                     ; preds = %17
  store i64 0, ptr %.reg2mem60, align 8
  br label %31

18:                                               ; preds = %._crit_edge8, %._crit_edge3
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem34, align 8
  %.reload36 = load i64, ptr %.reg2mem34, align 8
  %19 = getelementptr inbounds double, ptr %5, i64 %.reload36
  store ptr %19, ptr %.reg2mem31, align 8
  %.reload33 = load ptr, ptr %.reg2mem31, align 8
  store double 0.000000e+00, ptr %.reload33, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem62, align 8
  store i64 0, ptr %.reg2mem64, align 8
  br label %20

20:                                               ; preds = %._crit_edge7, %18
  %.reload65 = load i64, ptr %.reg2mem64, align 8
  %.reload63 = load double, ptr %.reg2mem62, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %21 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %.reload44, i64 %.reload40, i64 %.reload65
  %22 = load double, ptr %21, align 8, !tbaa !5
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %23 = getelementptr inbounds [60 x double], ptr %4, i64 %.reload65, i64 %.reload35
  %24 = load double, ptr %23, align 8, !tbaa !5
  %25 = tail call double @llvm.fmuladd.f64(double %22, double %24, double %.reload63)
  store double %25, ptr %.reg2mem28, align 8
  %.reload30 = load double, ptr %.reg2mem28, align 8
  %.reload32 = load ptr, ptr %.reg2mem31, align 8
  store double %.reload30, ptr %.reload32, align 8, !tbaa !5
  %26 = add nuw nsw i64 %.reload65, 1
  store i64 %26, ptr %.reg2mem25, align 8
  %.reload26 = load i64, ptr %.reg2mem25, align 8
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %27 = icmp eq i64 %.reload26, %.reload49
  br i1 %27, label %28, label %._crit_edge7, !llvm.loop !9

._crit_edge7:                                     ; preds = %20
  %.reload27 = load i64, ptr %.reg2mem25, align 8
  %.reload29 = load double, ptr %.reg2mem28, align 8
  store double %.reload29, ptr %.reg2mem62, align 8
  store i64 %.reload27, ptr %.reg2mem64, align 8
  br label %20

28:                                               ; preds = %20
  %.reload37 = load i64, ptr %.reg2mem34, align 8
  %29 = add nuw nsw i64 %.reload37, 1
  store i64 %29, ptr %.reg2mem22, align 8
  %.reload24 = load i64, ptr %.reg2mem22, align 8
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %30 = icmp eq i64 %.reload24, %.reload50
  br i1 %30, label %17, label %._crit_edge8, !llvm.loop !12

._crit_edge8:                                     ; preds = %28
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  store i64 %.reload23, ptr %.reg2mem66, align 8
  br label %18

31:                                               ; preds = %._crit_edge10, %._crit_edge5
  %.reload61 = load i64, ptr %.reg2mem60, align 8
  %32 = getelementptr inbounds double, ptr %5, i64 %.reload61
  %33 = load double, ptr %32, align 8, !tbaa !5
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  %34 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %.reload43, i64 %.reload39, i64 %.reload61
  store double %33, ptr %34, align 8, !tbaa !5
  %35 = add nuw nsw i64 %.reload61, 1
  store i64 %35, ptr %.reg2mem19, align 8
  %.reload21 = load i64, ptr %.reg2mem19, align 8
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %36 = icmp eq i64 %.reload21, %.reload47
  br i1 %36, label %._crit_edge9, label %._crit_edge10, !llvm.loop !13

._crit_edge10:                                    ; preds = %31
  %.reload20 = load i64, ptr %.reg2mem19, align 8
  store i64 %.reload20, ptr %.reg2mem60, align 8
  br label %31

._crit_edge9:                                     ; preds = %31
  br label %37

37:                                               ; preds = %._crit_edge9, %._crit_edge6, %._crit_edge4
  %.reload41 = load i64, ptr %.reg2mem38, align 8
  %38 = add nuw nsw i64 %.reload41, 1
  store i64 %38, ptr %.reg2mem16, align 8
  %.reload18 = load i64, ptr %.reg2mem16, align 8
  %.reload52 = load i64, ptr %.reg2mem51, align 8
  %39 = icmp eq i64 %.reload18, %.reload52
  br i1 %39, label %._crit_edge11, label %._crit_edge12, !llvm.loop !14

._crit_edge12:                                    ; preds = %37
  %.reload17 = load i64, ptr %.reg2mem16, align 8
  store i64 %.reload17, ptr %.reg2mem68, align 8
  br label %16

._crit_edge11:                                    ; preds = %37
  br label %40

40:                                               ; preds = %._crit_edge11, %._crit_edge2
  %.reload45 = load i64, ptr %.reg2mem42, align 8
  %41 = add nuw nsw i64 %.reload45, 1
  store i64 %41, ptr %.reg2mem, align 8
  %.reload = load i64, ptr %.reg2mem, align 8
  %.reload54 = load i64, ptr %.reg2mem53, align 8
  %42 = icmp eq i64 %.reload, %.reload54
  br i1 %42, label %._crit_edge13, label %._crit_edge14, !llvm.loop !15

._crit_edge14:                                    ; preds = %40
  %.reload15 = load i64, ptr %.reg2mem, align 8
  store i64 %.reload15, ptr %.reg2mem70, align 8
  br label %15

._crit_edge13:                                    ; preds = %40
  br label %43

43:                                               ; preds = %._crit_edge13, %._crit_edge
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #2 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem21 = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem28 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem40 = alloca i64, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem49 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem55 = alloca double, align 8
  %.reg2mem58 = alloca ptr, align 8
  %.reg2mem61 = alloca i64, align 8
  %.reg2mem65 = alloca i64, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem73 = alloca i64, align 8
  %.reg2mem76 = alloca i64, align 8
  %.reg2mem79 = alloca i64, align 8
  %.reg2mem83 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem89 = alloca i64, align 8
  %.reg2mem92 = alloca i64, align 8
  %.reg2mem94 = alloca i64, align 8
  %.reg2mem98 = alloca i64, align 8
  %.reg2mem102 = alloca ptr, align 8
  %.reg2mem106 = alloca ptr, align 8
  %.reg2mem110 = alloca ptr, align 8
  %.reg2mem116 = alloca i64, align 8
  %.reg2mem118 = alloca i64, align 8
  %.reg2mem120 = alloca i64, align 8
  %.reg2mem122 = alloca i64, align 8
  %.reg2mem124 = alloca double, align 8
  %.reg2mem126 = alloca i64, align 8
  %.reg2mem128 = alloca i64, align 8
  %.reg2mem130 = alloca i64, align 8
  %.reg2mem132 = alloca i64, align 8
  %.reg2mem134 = alloca i64, align 8
  %.reg2mem136 = alloca i64, align 8
  %.reg2mem138 = alloca i64, align 8
  %.reg2mem140 = alloca i64, align 8
  %.reg2mem142 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 120000, i32 noundef 8) #5
  store ptr %3, ptr %.reg2mem110, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 60, i32 noundef 8) #5
  store ptr %4, ptr %.reg2mem106, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 3600, i32 noundef 8) #5
  store ptr %5, ptr %.reg2mem102, align 8
  store i64 0, ptr %.reg2mem142, align 8
  br label %6

6:                                                ; preds = %._crit_edge3, %2
  %.reload143 = load i64, ptr %.reg2mem142, align 8
  store i64 %.reload143, ptr %.reg2mem98, align 8
  store i64 0, ptr %.reg2mem140, align 8
  br label %7

7:                                                ; preds = %._crit_edge1, %6
  %.reload141 = load i64, ptr %.reg2mem140, align 8
  store i64 %.reload141, ptr %.reg2mem94, align 8
  %.reload97 = load i64, ptr %.reg2mem94, align 8
  %.reload101 = load i64, ptr %.reg2mem98, align 8
  %8 = mul nuw nsw i64 %.reload97, %.reload101
  store i64 %8, ptr %.reg2mem92, align 8
  store i64 0, ptr %.reg2mem138, align 8
  br label %9

9:                                                ; preds = %._crit_edge, %7
  %.reload139 = load i64, ptr %.reg2mem138, align 8
  %.reload93 = load i64, ptr %.reg2mem92, align 8
  %10 = add nuw nsw i64 %.reload139, %.reload93
  %11 = trunc nuw nsw i64 %10 to i32
  %12 = urem i32 %11, 60
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 6.000000e+01
  %.reload96 = load i64, ptr %.reg2mem94, align 8
  %.reload100 = load i64, ptr %.reg2mem98, align 8
  %.reload114 = load ptr, ptr %.reg2mem110, align 8
  %15 = getelementptr inbounds [40 x [60 x double]], ptr %.reload114, i64 %.reload100, i64 %.reload96, i64 %.reload139
  store double %14, ptr %15, align 8, !tbaa !5
  %16 = add nuw nsw i64 %.reload139, 1
  store i64 %16, ptr %.reg2mem89, align 8
  %.reload91 = load i64, ptr %.reg2mem89, align 8
  %17 = icmp eq i64 %.reload91, 60
  br i1 %17, label %18, label %._crit_edge, !llvm.loop !16

._crit_edge:                                      ; preds = %9
  %.reload90 = load i64, ptr %.reg2mem89, align 8
  store i64 %.reload90, ptr %.reg2mem138, align 8
  br label %9

18:                                               ; preds = %9
  %.reload95 = load i64, ptr %.reg2mem94, align 8
  %19 = add nuw nsw i64 %.reload95, 1
  store i64 %19, ptr %.reg2mem86, align 8
  %.reload88 = load i64, ptr %.reg2mem86, align 8
  %20 = icmp eq i64 %.reload88, 40
  br i1 %20, label %21, label %._crit_edge1, !llvm.loop !17

._crit_edge1:                                     ; preds = %18
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  store i64 %.reload87, ptr %.reg2mem140, align 8
  br label %7

21:                                               ; preds = %18
  %.reload99 = load i64, ptr %.reg2mem98, align 8
  %22 = add nuw nsw i64 %.reload99, 1
  store i64 %22, ptr %.reg2mem83, align 8
  %.reload85 = load i64, ptr %.reg2mem83, align 8
  %23 = icmp eq i64 %.reload85, 50
  br i1 %23, label %._crit_edge2, label %._crit_edge3, !llvm.loop !18

._crit_edge3:                                     ; preds = %21
  %.reload84 = load i64, ptr %.reg2mem83, align 8
  store i64 %.reload84, ptr %.reg2mem142, align 8
  br label %6

._crit_edge2:                                     ; preds = %21
  store i64 0, ptr %.reg2mem136, align 8
  br label %24

24:                                               ; preds = %._crit_edge6, %._crit_edge2
  %.reload137 = load i64, ptr %.reg2mem136, align 8
  store i64 %.reload137, ptr %.reg2mem79, align 8
  store i64 0, ptr %.reg2mem134, align 8
  br label %25

25:                                               ; preds = %._crit_edge4, %24
  %.reload135 = load i64, ptr %.reg2mem134, align 8
  %.reload82 = load i64, ptr %.reg2mem79, align 8
  %26 = mul nuw nsw i64 %.reload135, %.reload82
  %27 = trunc nuw nsw i64 %26 to i32
  %28 = urem i32 %27, 60
  %29 = uitofp nneg i32 %28 to double
  %30 = fdiv double %29, 6.000000e+01
  %.reload81 = load i64, ptr %.reg2mem79, align 8
  %.reload104 = load ptr, ptr %.reg2mem102, align 8
  %31 = getelementptr inbounds [60 x double], ptr %.reload104, i64 %.reload81, i64 %.reload135
  store double %30, ptr %31, align 8, !tbaa !5
  %32 = add nuw nsw i64 %.reload135, 1
  store i64 %32, ptr %.reg2mem76, align 8
  %.reload78 = load i64, ptr %.reg2mem76, align 8
  %33 = icmp eq i64 %.reload78, 60
  br i1 %33, label %34, label %._crit_edge4, !llvm.loop !19

._crit_edge4:                                     ; preds = %25
  %.reload77 = load i64, ptr %.reg2mem76, align 8
  store i64 %.reload77, ptr %.reg2mem134, align 8
  br label %25

34:                                               ; preds = %25
  %.reload80 = load i64, ptr %.reg2mem79, align 8
  %35 = add nuw nsw i64 %.reload80, 1
  store i64 %35, ptr %.reg2mem73, align 8
  %.reload75 = load i64, ptr %.reg2mem73, align 8
  %36 = icmp eq i64 %.reload75, 60
  br i1 %36, label %._crit_edge5, label %._crit_edge6, !llvm.loop !20

._crit_edge6:                                     ; preds = %34
  %.reload74 = load i64, ptr %.reg2mem73, align 8
  store i64 %.reload74, ptr %.reg2mem136, align 8
  br label %24

._crit_edge5:                                     ; preds = %34
  store i64 0, ptr %.reg2mem132, align 8
  br label %37

37:                                               ; preds = %._crit_edge12, %._crit_edge5
  %.reload133 = load i64, ptr %.reg2mem132, align 8
  store i64 %.reload133, ptr %.reg2mem69, align 8
  store i64 0, ptr %.reg2mem130, align 8
  br label %38

38:                                               ; preds = %._crit_edge11, %37
  %.reload131 = load i64, ptr %.reg2mem130, align 8
  store i64 %.reload131, ptr %.reg2mem65, align 8
  store i64 0, ptr %.reg2mem128, align 8
  br label %39

39:                                               ; preds = %._crit_edge9, %38
  %.reload129 = load i64, ptr %.reg2mem128, align 8
  store i64 %.reload129, ptr %.reg2mem61, align 8
  %.reload64 = load i64, ptr %.reg2mem61, align 8
  %.reload108 = load ptr, ptr %.reg2mem106, align 8
  %40 = getelementptr inbounds double, ptr %.reload108, i64 %.reload64
  store ptr %40, ptr %.reg2mem58, align 8
  %.reload60 = load ptr, ptr %.reg2mem58, align 8
  store double 0.000000e+00, ptr %.reload60, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem124, align 8
  store i64 0, ptr %.reg2mem126, align 8
  br label %41

41:                                               ; preds = %._crit_edge7, %39
  %.reload127 = load i64, ptr %.reg2mem126, align 8
  %.reload125 = load double, ptr %.reg2mem124, align 8
  %.reload68 = load i64, ptr %.reg2mem65, align 8
  %.reload72 = load i64, ptr %.reg2mem69, align 8
  %.reload113 = load ptr, ptr %.reg2mem110, align 8
  %42 = getelementptr inbounds [40 x [60 x double]], ptr %.reload113, i64 %.reload72, i64 %.reload68, i64 %.reload127
  %43 = load double, ptr %42, align 8, !tbaa !5
  %.reload63 = load i64, ptr %.reg2mem61, align 8
  %.reload103 = load ptr, ptr %.reg2mem102, align 8
  %44 = getelementptr inbounds [60 x double], ptr %.reload103, i64 %.reload127, i64 %.reload63
  %45 = load double, ptr %44, align 8, !tbaa !5
  %46 = tail call double @llvm.fmuladd.f64(double %43, double %45, double %.reload125)
  store double %46, ptr %.reg2mem55, align 8
  %.reload57 = load double, ptr %.reg2mem55, align 8
  %.reload59 = load ptr, ptr %.reg2mem58, align 8
  store double %.reload57, ptr %.reload59, align 8, !tbaa !5
  %47 = add nuw nsw i64 %.reload127, 1
  store i64 %47, ptr %.reg2mem52, align 8
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %48 = icmp eq i64 %.reload54, 60
  br i1 %48, label %49, label %._crit_edge7, !llvm.loop !9

._crit_edge7:                                     ; preds = %41
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  %.reload56 = load double, ptr %.reg2mem55, align 8
  store double %.reload56, ptr %.reg2mem124, align 8
  store i64 %.reload53, ptr %.reg2mem126, align 8
  br label %41

49:                                               ; preds = %41
  %.reload62 = load i64, ptr %.reg2mem61, align 8
  %50 = add nuw nsw i64 %.reload62, 1
  store i64 %50, ptr %.reg2mem49, align 8
  %.reload51 = load i64, ptr %.reg2mem49, align 8
  %51 = icmp eq i64 %.reload51, 60
  br i1 %51, label %._crit_edge8, label %._crit_edge9, !llvm.loop !12

._crit_edge9:                                     ; preds = %49
  %.reload50 = load i64, ptr %.reg2mem49, align 8
  store i64 %.reload50, ptr %.reg2mem128, align 8
  br label %39

._crit_edge8:                                     ; preds = %49
  store i64 0, ptr %.reg2mem122, align 8
  br label %52

52:                                               ; preds = %._crit_edge10, %._crit_edge8
  %.reload123 = load i64, ptr %.reg2mem122, align 8
  %.reload107 = load ptr, ptr %.reg2mem106, align 8
  %53 = getelementptr inbounds double, ptr %.reload107, i64 %.reload123
  %54 = load double, ptr %53, align 8, !tbaa !5
  %.reload67 = load i64, ptr %.reg2mem65, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %.reload112 = load ptr, ptr %.reg2mem110, align 8
  %55 = getelementptr inbounds [40 x [60 x double]], ptr %.reload112, i64 %.reload71, i64 %.reload67, i64 %.reload123
  store double %54, ptr %55, align 8, !tbaa !5
  %56 = add nuw nsw i64 %.reload123, 1
  store i64 %56, ptr %.reg2mem46, align 8
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %57 = icmp eq i64 %.reload48, 60
  br i1 %57, label %58, label %._crit_edge10, !llvm.loop !13

._crit_edge10:                                    ; preds = %52
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  store i64 %.reload47, ptr %.reg2mem122, align 8
  br label %52

58:                                               ; preds = %52
  %.reload66 = load i64, ptr %.reg2mem65, align 8
  %59 = add nuw nsw i64 %.reload66, 1
  store i64 %59, ptr %.reg2mem43, align 8
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %60 = icmp eq i64 %.reload45, 40
  br i1 %60, label %61, label %._crit_edge11, !llvm.loop !14

._crit_edge11:                                    ; preds = %58
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  store i64 %.reload44, ptr %.reg2mem130, align 8
  br label %38

61:                                               ; preds = %58
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %62 = add nuw nsw i64 %.reload70, 1
  store i64 %62, ptr %.reg2mem40, align 8
  %.reload42 = load i64, ptr %.reg2mem40, align 8
  %63 = icmp eq i64 %.reload42, 50
  br i1 %63, label %64, label %._crit_edge12, !llvm.loop !15

._crit_edge12:                                    ; preds = %61
  %.reload41 = load i64, ptr %.reg2mem40, align 8
  store i64 %.reload41, ptr %.reg2mem132, align 8
  br label %37

64:                                               ; preds = %61
  %65 = load ptr, ptr @stderr, align 8, !tbaa !21
  %66 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %65, ptr noundef nonnull @.str) #5
  %67 = load ptr, ptr @stderr, align 8, !tbaa !21
  %68 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #5
  store i64 0, ptr %.reg2mem120, align 8
  br label %69

69:                                               ; preds = %._crit_edge16, %64
  %.reload121 = load i64, ptr %.reg2mem120, align 8
  store i64 %.reload121, ptr %.reg2mem36, align 8
  %.reload39 = load i64, ptr %.reg2mem36, align 8
  %70 = mul nuw nsw i64 %.reload39, 2400
  store i64 %70, ptr %.reg2mem34, align 8
  store i64 0, ptr %.reg2mem118, align 8
  br label %71

71:                                               ; preds = %._crit_edge15, %69
  %.reload119 = load i64, ptr %.reg2mem118, align 8
  store i64 %.reload119, ptr %.reg2mem30, align 8
  %.reload33 = load i64, ptr %.reg2mem30, align 8
  %72 = mul nuw nsw i64 %.reload33, 60
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %73 = add nuw nsw i64 %72, %.reload35
  store i64 %73, ptr %.reg2mem28, align 8
  store i64 0, ptr %.reg2mem116, align 8
  br label %74

74:                                               ; preds = %._crit_edge14, %71
  %.reload117 = load i64, ptr %.reg2mem116, align 8
  store i64 %.reload117, ptr %.reg2mem24, align 8
  %.reload27 = load i64, ptr %.reg2mem24, align 8
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  %75 = add nuw nsw i64 %.reload29, %.reload27
  %76 = trunc nuw nsw i64 %75 to i32
  %77 = urem i32 %76, 20
  %78 = icmp eq i32 %77, 0
  br i1 %78, label %79, label %._crit_edge13

._crit_edge13:                                    ; preds = %74
  br label %82

79:                                               ; preds = %74
  %80 = load ptr, ptr @stderr, align 8, !tbaa !21
  %81 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %80, ptr noundef nonnull @.str.3) #5
  br label %82

82:                                               ; preds = %._crit_edge13, %79
  %83 = load ptr, ptr @stderr, align 8, !tbaa !21
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %.reload111 = load ptr, ptr %.reg2mem110, align 8
  %84 = getelementptr inbounds [40 x [60 x double]], ptr %.reload111, i64 %.reload38, i64 %.reload32, i64 %.reload26
  %85 = load double, ptr %84, align 8, !tbaa !5
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str.4, double noundef %85) #5
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  %87 = add nuw nsw i64 %.reload25, 1
  store i64 %87, ptr %.reg2mem21, align 8
  %.reload23 = load i64, ptr %.reg2mem21, align 8
  %88 = icmp eq i64 %.reload23, 60
  br i1 %88, label %89, label %._crit_edge14, !llvm.loop !23

._crit_edge14:                                    ; preds = %82
  %.reload22 = load i64, ptr %.reg2mem21, align 8
  store i64 %.reload22, ptr %.reg2mem116, align 8
  br label %74

89:                                               ; preds = %82
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  %90 = add nuw nsw i64 %.reload31, 1
  store i64 %90, ptr %.reg2mem18, align 8
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %91 = icmp eq i64 %.reload20, 40
  br i1 %91, label %92, label %._crit_edge15, !llvm.loop !24

._crit_edge15:                                    ; preds = %89
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  store i64 %.reload19, ptr %.reg2mem118, align 8
  br label %71

92:                                               ; preds = %89
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  %93 = add nuw nsw i64 %.reload37, 1
  store i64 %93, ptr %.reg2mem, align 8
  %.reload17 = load i64, ptr %.reg2mem, align 8
  %94 = icmp eq i64 %.reload17, 50
  br i1 %94, label %95, label %._crit_edge16, !llvm.loop !25

._crit_edge16:                                    ; preds = %92
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem120, align 8
  br label %69

95:                                               ; preds = %92
  %96 = load ptr, ptr @stderr, align 8, !tbaa !21
  %97 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %96, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #5
  %98 = load ptr, ptr @stderr, align 8, !tbaa !21
  %99 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %98, ptr noundef nonnull @.str.6) #5
  %.reload115 = load ptr, ptr %.reg2mem110, align 8
  tail call void @free(ptr noundef nonnull %.reload115) #5
  %.reload109 = load ptr, ptr %.reg2mem106, align 8
  tail call void @free(ptr noundef %.reload109) #5
  %.reload105 = load ptr, ptr %.reg2mem102, align 8
  tail call void @free(ptr noundef %.reload105) #5
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #4

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #3

attributes #0 = { nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nobuiltin nounwind "no-builtins" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"Ubuntu clang version 19.1.7 (20ubuntu4)"}
!5 = !{!6, !6, i64 0}
!6 = !{!"double", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
!9 = distinct !{!9, !10, !11}
!10 = !{!"llvm.loop.mustprogress"}
!11 = !{!"llvm.loop.unroll.disable"}
!12 = distinct !{!12, !10, !11}
!13 = distinct !{!13, !10, !11}
!14 = distinct !{!14, !10, !11}
!15 = distinct !{!15, !10, !11}
!16 = distinct !{!16, !10, !11}
!17 = distinct !{!17, !10, !11}
!18 = distinct !{!18, !10, !11}
!19 = distinct !{!19, !10, !11}
!20 = distinct !{!20, !10, !11}
!21 = !{!22, !22, i64 0}
!22 = !{!"any pointer", !7, i64 0}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}

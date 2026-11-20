; ModuleID = './polybench-output/gemver-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gemver/gemver.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"w\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem16 = alloca i64, align 8
  %.reg2mem20 = alloca i64, align 8
  %.reg2mem23 = alloca i64, align 8
  %.reg2mem26 = alloca double, align 8
  %.reg2mem29 = alloca double, align 8
  %.reg2mem31 = alloca ptr, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem41 = alloca i64, align 8
  %.reg2mem44 = alloca i64, align 8
  %.reg2mem47 = alloca double, align 8
  %.reg2mem50 = alloca double, align 8
  %.reg2mem52 = alloca ptr, align 8
  %.reg2mem55 = alloca i64, align 8
  %.reg2mem59 = alloca i64, align 8
  %.reg2mem62 = alloca i64, align 8
  %.reg2mem65 = alloca ptr, align 8
  %.reg2mem67 = alloca ptr, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem74 = alloca i64, align 8
  %.reg2mem77 = alloca i64, align 8
  %.reg2mem81 = alloca i64, align 8
  %.reg2mem94 = alloca ptr, align 8
  %.reg2mem98 = alloca ptr, align 8
  %.reg2mem102 = alloca ptr, align 8
  %.reg2mem108 = alloca ptr, align 8
  %.reg2mem113 = alloca ptr, align 8
  %.reg2mem117 = alloca ptr, align 8
  %.reg2mem121 = alloca ptr, align 8
  %.reg2mem125 = alloca ptr, align 8
  %.reg2mem129 = alloca ptr, align 8
  %.reg2mem135 = alloca i64, align 8
  %.reg2mem137 = alloca double, align 8
  %.reg2mem139 = alloca i64, align 8
  %.reg2mem141 = alloca i64, align 8
  %.reg2mem143 = alloca i64, align 8
  %.reg2mem145 = alloca double, align 8
  %.reg2mem147 = alloca i64, align 8
  %.reg2mem149 = alloca i64, align 8
  %.reg2mem151 = alloca i64, align 8
  %.reg2mem153 = alloca i64, align 8
  %.reg2mem155 = alloca i64, align 8
  %.reg2mem157 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem129, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem125, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem121, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem117, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem113, align 8
  %8 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %8, ptr %.reg2mem108, align 8
  %9 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %9, ptr %.reg2mem102, align 8
  %10 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %10, ptr %.reg2mem98, align 8
  %11 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %11, ptr %.reg2mem94, align 8
  store i64 0, ptr %.reg2mem157, align 8
  br label %14

12:                                               ; preds = %34
  %.reload79 = load i64, ptr %.reg2mem77, align 8
  %13 = icmp eq i64 %.reload79, 400
  br i1 %13, label %._crit_edge, label %._crit_edge1, !llvm.loop !5

._crit_edge1:                                     ; preds = %12
  %.reload78 = load i64, ptr %.reg2mem77, align 8
  store i64 %.reload78, ptr %.reg2mem157, align 8
  br label %14

._crit_edge:                                      ; preds = %12
  store i64 0, ptr %.reg2mem153, align 8
  br label %43

14:                                               ; preds = %._crit_edge1, %2
  %.reload158 = load i64, ptr %.reg2mem157, align 8
  store i64 %.reload158, ptr %.reg2mem81, align 8
  %.reload93 = load i64, ptr %.reg2mem81, align 8
  %15 = trunc nuw nsw i64 %.reload93 to i32
  %16 = uitofp nneg i32 %15 to double
  %.reload92 = load i64, ptr %.reg2mem81, align 8
  %.reload127 = load ptr, ptr %.reg2mem125, align 8
  %17 = getelementptr inbounds double, ptr %.reload127, i64 %.reload92
  store double %16, ptr %17, align 8, !tbaa !8
  %.reload91 = load i64, ptr %.reg2mem81, align 8
  %18 = add nuw nsw i64 %.reload91, 1
  store i64 %18, ptr %.reg2mem77, align 8
  %.reload80 = load i64, ptr %.reg2mem77, align 8
  %19 = trunc nuw nsw i64 %.reload80 to i32
  %20 = uitofp nneg i32 %19 to double
  %21 = fdiv double %20, 4.000000e+02
  %22 = fmul double %21, 5.000000e-01
  %.reload90 = load i64, ptr %.reg2mem81, align 8
  %.reload119 = load ptr, ptr %.reg2mem117, align 8
  %23 = getelementptr inbounds double, ptr %.reload119, i64 %.reload90
  store double %22, ptr %23, align 8, !tbaa !8
  %24 = fmul double %21, 2.500000e-01
  %.reload89 = load i64, ptr %.reg2mem81, align 8
  %.reload123 = load ptr, ptr %.reg2mem121, align 8
  %25 = getelementptr inbounds double, ptr %.reload123, i64 %.reload89
  store double %24, ptr %25, align 8, !tbaa !8
  %26 = fdiv double %21, 6.000000e+00
  %.reload88 = load i64, ptr %.reg2mem81, align 8
  %.reload115 = load ptr, ptr %.reg2mem113, align 8
  %27 = getelementptr inbounds double, ptr %.reload115, i64 %.reload88
  store double %26, ptr %27, align 8, !tbaa !8
  %28 = fmul double %21, 1.250000e-01
  %.reload87 = load i64, ptr %.reg2mem81, align 8
  %.reload100 = load ptr, ptr %.reg2mem98, align 8
  %29 = getelementptr inbounds double, ptr %.reload100, i64 %.reload87
  store double %28, ptr %29, align 8, !tbaa !8
  %30 = fdiv double %21, 9.000000e+00
  %.reload86 = load i64, ptr %.reg2mem81, align 8
  %.reload96 = load ptr, ptr %.reg2mem94, align 8
  %31 = getelementptr inbounds double, ptr %.reload96, i64 %.reload86
  store double %30, ptr %31, align 8, !tbaa !8
  %.reload85 = load i64, ptr %.reg2mem81, align 8
  %.reload106 = load ptr, ptr %.reg2mem102, align 8
  %32 = getelementptr inbounds double, ptr %.reload106, i64 %.reload85
  store double 0.000000e+00, ptr %32, align 8, !tbaa !8
  %.reload84 = load i64, ptr %.reg2mem81, align 8
  %.reload111 = load ptr, ptr %.reg2mem108, align 8
  %33 = getelementptr inbounds double, ptr %.reload111, i64 %.reload84
  store double 0.000000e+00, ptr %33, align 8, !tbaa !8
  store i64 0, ptr %.reg2mem155, align 8
  br label %34

34:                                               ; preds = %._crit_edge2, %14
  %.reload156 = load i64, ptr %.reg2mem155, align 8
  %.reload83 = load i64, ptr %.reg2mem81, align 8
  %35 = mul nuw nsw i64 %.reload156, %.reload83
  %36 = trunc nuw nsw i64 %35 to i32
  %37 = urem i32 %36, 400
  %38 = uitofp nneg i32 %37 to double
  %39 = fdiv double %38, 4.000000e+02
  %.reload82 = load i64, ptr %.reg2mem81, align 8
  %.reload133 = load ptr, ptr %.reg2mem129, align 8
  %40 = getelementptr inbounds [400 x double], ptr %.reload133, i64 %.reload82, i64 %.reload156
  store double %39, ptr %40, align 8, !tbaa !8
  %41 = add nuw nsw i64 %.reload156, 1
  store i64 %41, ptr %.reg2mem74, align 8
  %.reload76 = load i64, ptr %.reg2mem74, align 8
  %42 = icmp eq i64 %.reload76, 400
  br i1 %42, label %12, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %34
  %.reload75 = load i64, ptr %.reg2mem74, align 8
  store i64 %.reload75, ptr %.reg2mem155, align 8
  br label %34

43:                                               ; preds = %._crit_edge5, %._crit_edge
  %.reload154 = load i64, ptr %.reg2mem153, align 8
  store i64 %.reload154, ptr %.reg2mem69, align 8
  %.reload73 = load i64, ptr %.reg2mem69, align 8
  %.reload126 = load ptr, ptr %.reg2mem125, align 8
  %44 = getelementptr inbounds double, ptr %.reload126, i64 %.reload73
  store ptr %44, ptr %.reg2mem67, align 8
  %.reload72 = load i64, ptr %.reg2mem69, align 8
  %.reload118 = load ptr, ptr %.reg2mem117, align 8
  %45 = getelementptr inbounds double, ptr %.reload118, i64 %.reload72
  store ptr %45, ptr %.reg2mem65, align 8
  store i64 0, ptr %.reg2mem151, align 8
  br label %46

46:                                               ; preds = %._crit_edge3, %43
  %.reload152 = load i64, ptr %.reg2mem151, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %.reload132 = load ptr, ptr %.reg2mem129, align 8
  %47 = getelementptr inbounds [400 x double], ptr %.reload132, i64 %.reload71, i64 %.reload152
  %48 = load double, ptr %47, align 8, !tbaa !8
  %.reload68 = load ptr, ptr %.reg2mem67, align 8
  %49 = load double, ptr %.reload68, align 8, !tbaa !8
  %.reload122 = load ptr, ptr %.reg2mem121, align 8
  %50 = getelementptr inbounds double, ptr %.reload122, i64 %.reload152
  %51 = load double, ptr %50, align 8, !tbaa !8
  %52 = tail call double @llvm.fmuladd.f64(double %49, double %51, double %48)
  %.reload66 = load ptr, ptr %.reg2mem65, align 8
  %53 = load double, ptr %.reload66, align 8, !tbaa !8
  %.reload114 = load ptr, ptr %.reg2mem113, align 8
  %54 = getelementptr inbounds double, ptr %.reload114, i64 %.reload152
  %55 = load double, ptr %54, align 8, !tbaa !8
  %56 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %52)
  store double %56, ptr %47, align 8, !tbaa !8
  %57 = add nuw nsw i64 %.reload152, 1
  store i64 %57, ptr %.reg2mem62, align 8
  %.reload64 = load i64, ptr %.reg2mem62, align 8
  %58 = icmp eq i64 %.reload64, 400
  br i1 %58, label %59, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %46
  %.reload63 = load i64, ptr %.reg2mem62, align 8
  store i64 %.reload63, ptr %.reg2mem151, align 8
  br label %46

59:                                               ; preds = %46
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %60 = add nuw nsw i64 %.reload70, 1
  store i64 %60, ptr %.reg2mem59, align 8
  %.reload61 = load i64, ptr %.reg2mem59, align 8
  %61 = icmp eq i64 %.reload61, 400
  br i1 %61, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %59
  %.reload60 = load i64, ptr %.reg2mem59, align 8
  store i64 %.reload60, ptr %.reg2mem153, align 8
  br label %43

._crit_edge4:                                     ; preds = %59
  store i64 0, ptr %.reg2mem149, align 8
  br label %62

62:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload150 = load i64, ptr %.reg2mem149, align 8
  store i64 %.reload150, ptr %.reg2mem55, align 8
  %.reload58 = load i64, ptr %.reg2mem55, align 8
  %.reload105 = load ptr, ptr %.reg2mem102, align 8
  %63 = getelementptr inbounds double, ptr %.reload105, i64 %.reload58
  store ptr %63, ptr %.reg2mem52, align 8
  %.reload54 = load ptr, ptr %.reg2mem52, align 8
  %64 = load double, ptr %.reload54, align 8, !tbaa !8
  store double %64, ptr %.reg2mem50, align 8
  %.reload51 = load double, ptr %.reg2mem50, align 8
  store double %.reload51, ptr %.reg2mem145, align 8
  store i64 0, ptr %.reg2mem147, align 8
  br label %65

65:                                               ; preds = %._crit_edge6, %62
  %.reload148 = load i64, ptr %.reg2mem147, align 8
  %.reload146 = load double, ptr %.reg2mem145, align 8
  %.reload57 = load i64, ptr %.reg2mem55, align 8
  %.reload131 = load ptr, ptr %.reg2mem129, align 8
  %66 = getelementptr inbounds [400 x double], ptr %.reload131, i64 %.reload148, i64 %.reload57
  %67 = load double, ptr %66, align 8, !tbaa !8
  %68 = fmul double %67, 1.200000e+00
  %.reload99 = load ptr, ptr %.reg2mem98, align 8
  %69 = getelementptr inbounds double, ptr %.reload99, i64 %.reload148
  %70 = load double, ptr %69, align 8, !tbaa !8
  %71 = tail call double @llvm.fmuladd.f64(double %68, double %70, double %.reload146)
  store double %71, ptr %.reg2mem47, align 8
  %.reload49 = load double, ptr %.reg2mem47, align 8
  %.reload53 = load ptr, ptr %.reg2mem52, align 8
  store double %.reload49, ptr %.reload53, align 8, !tbaa !8
  %72 = add nuw nsw i64 %.reload148, 1
  store i64 %72, ptr %.reg2mem44, align 8
  %.reload46 = load i64, ptr %.reg2mem44, align 8
  %73 = icmp eq i64 %.reload46, 400
  br i1 %73, label %74, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %65
  %.reload45 = load i64, ptr %.reg2mem44, align 8
  %.reload48 = load double, ptr %.reg2mem47, align 8
  store double %.reload48, ptr %.reg2mem145, align 8
  store i64 %.reload45, ptr %.reg2mem147, align 8
  br label %65

74:                                               ; preds = %65
  %.reload56 = load i64, ptr %.reg2mem55, align 8
  %75 = add nuw nsw i64 %.reload56, 1
  store i64 %75, ptr %.reg2mem41, align 8
  %.reload43 = load i64, ptr %.reg2mem41, align 8
  %76 = icmp eq i64 %.reload43, 400
  br i1 %76, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %74
  %.reload42 = load i64, ptr %.reg2mem41, align 8
  store i64 %.reload42, ptr %.reg2mem149, align 8
  br label %62

._crit_edge7:                                     ; preds = %74
  store i64 0, ptr %.reg2mem143, align 8
  br label %77

77:                                               ; preds = %._crit_edge10, %._crit_edge7
  %.reload144 = load i64, ptr %.reg2mem143, align 8
  %.reload104 = load ptr, ptr %.reg2mem102, align 8
  %78 = getelementptr inbounds double, ptr %.reload104, i64 %.reload144
  %79 = load double, ptr %78, align 8, !tbaa !8
  %.reload95 = load ptr, ptr %.reg2mem94, align 8
  %80 = getelementptr inbounds double, ptr %.reload95, i64 %.reload144
  %81 = load double, ptr %80, align 8, !tbaa !8
  %82 = fadd double %79, %81
  store double %82, ptr %78, align 8, !tbaa !8
  %83 = add nuw nsw i64 %.reload144, 1
  store i64 %83, ptr %.reg2mem38, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %84 = icmp eq i64 %.reload40, 400
  br i1 %84, label %._crit_edge9, label %._crit_edge10, !llvm.loop !17

._crit_edge10:                                    ; preds = %77
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  store i64 %.reload39, ptr %.reg2mem143, align 8
  br label %77

._crit_edge9:                                     ; preds = %77
  store i64 0, ptr %.reg2mem141, align 8
  br label %85

85:                                               ; preds = %._crit_edge12, %._crit_edge9
  %.reload142 = load i64, ptr %.reg2mem141, align 8
  store i64 %.reload142, ptr %.reg2mem34, align 8
  %.reload37 = load i64, ptr %.reg2mem34, align 8
  %.reload110 = load ptr, ptr %.reg2mem108, align 8
  %86 = getelementptr inbounds double, ptr %.reload110, i64 %.reload37
  store ptr %86, ptr %.reg2mem31, align 8
  %.reload33 = load ptr, ptr %.reg2mem31, align 8
  %87 = load double, ptr %.reload33, align 8, !tbaa !8
  store double %87, ptr %.reg2mem29, align 8
  %.reload30 = load double, ptr %.reg2mem29, align 8
  store double %.reload30, ptr %.reg2mem137, align 8
  store i64 0, ptr %.reg2mem139, align 8
  br label %88

88:                                               ; preds = %._crit_edge11, %85
  %.reload140 = load i64, ptr %.reg2mem139, align 8
  %.reload138 = load double, ptr %.reg2mem137, align 8
  %.reload36 = load i64, ptr %.reg2mem34, align 8
  %.reload130 = load ptr, ptr %.reg2mem129, align 8
  %89 = getelementptr inbounds [400 x double], ptr %.reload130, i64 %.reload36, i64 %.reload140
  %90 = load double, ptr %89, align 8, !tbaa !8
  %91 = fmul double %90, 1.500000e+00
  %.reload103 = load ptr, ptr %.reg2mem102, align 8
  %92 = getelementptr inbounds double, ptr %.reload103, i64 %.reload140
  %93 = load double, ptr %92, align 8, !tbaa !8
  %94 = tail call double @llvm.fmuladd.f64(double %91, double %93, double %.reload138)
  store double %94, ptr %.reg2mem26, align 8
  %.reload28 = load double, ptr %.reg2mem26, align 8
  %.reload32 = load ptr, ptr %.reg2mem31, align 8
  store double %.reload28, ptr %.reload32, align 8, !tbaa !8
  %95 = add nuw nsw i64 %.reload140, 1
  store i64 %95, ptr %.reg2mem23, align 8
  %.reload25 = load i64, ptr %.reg2mem23, align 8
  %96 = icmp eq i64 %.reload25, 400
  br i1 %96, label %97, label %._crit_edge11, !llvm.loop !18

._crit_edge11:                                    ; preds = %88
  %.reload24 = load i64, ptr %.reg2mem23, align 8
  %.reload27 = load double, ptr %.reg2mem26, align 8
  store double %.reload27, ptr %.reg2mem137, align 8
  store i64 %.reload24, ptr %.reg2mem139, align 8
  br label %88

97:                                               ; preds = %88
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %98 = add nuw nsw i64 %.reload35, 1
  store i64 %98, ptr %.reg2mem20, align 8
  %.reload22 = load i64, ptr %.reg2mem20, align 8
  %99 = icmp eq i64 %.reload22, 400
  br i1 %99, label %100, label %._crit_edge12, !llvm.loop !19

._crit_edge12:                                    ; preds = %97
  %.reload21 = load i64, ptr %.reg2mem20, align 8
  store i64 %.reload21, ptr %.reg2mem141, align 8
  br label %85

100:                                              ; preds = %97
  %101 = load ptr, ptr @stderr, align 8, !tbaa !20
  %102 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %101, ptr noundef nonnull @.str) #4
  %103 = load ptr, ptr @stderr, align 8, !tbaa !20
  %104 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %103, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem135, align 8
  br label %105

105:                                              ; preds = %._crit_edge14, %100
  %.reload136 = load i64, ptr %.reg2mem135, align 8
  store i64 %.reload136, ptr %.reg2mem16, align 8
  %.reload19 = load i64, ptr %.reg2mem16, align 8
  %106 = trunc i64 %.reload19 to i16
  %107 = urem i16 %106, 20
  %108 = icmp eq i16 %107, 0
  br i1 %108, label %109, label %._crit_edge13

._crit_edge13:                                    ; preds = %105
  br label %112

109:                                              ; preds = %105
  %110 = load ptr, ptr @stderr, align 8, !tbaa !20
  %111 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %110, ptr noundef nonnull @.str.3) #4
  br label %112

112:                                              ; preds = %._crit_edge13, %109
  %113 = load ptr, ptr @stderr, align 8, !tbaa !20
  %.reload18 = load i64, ptr %.reg2mem16, align 8
  %.reload109 = load ptr, ptr %.reg2mem108, align 8
  %114 = getelementptr inbounds double, ptr %.reload109, i64 %.reload18
  %115 = load double, ptr %114, align 8, !tbaa !8
  %116 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %113, ptr noundef nonnull @.str.4, double noundef %115) #4
  %.reload17 = load i64, ptr %.reg2mem16, align 8
  %117 = add nuw nsw i64 %.reload17, 1
  store i64 %117, ptr %.reg2mem, align 8
  %.reload15 = load i64, ptr %.reg2mem, align 8
  %118 = icmp eq i64 %.reload15, 400
  br i1 %118, label %119, label %._crit_edge14, !llvm.loop !22

._crit_edge14:                                    ; preds = %112
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem135, align 8
  br label %105

119:                                              ; preds = %112
  %120 = load ptr, ptr @stderr, align 8, !tbaa !20
  %121 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %120, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %122 = load ptr, ptr @stderr, align 8, !tbaa !20
  %123 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %122, ptr noundef nonnull @.str.6) #4
  %.reload134 = load ptr, ptr %.reg2mem129, align 8
  tail call void @free(ptr noundef %.reload134) #4
  %.reload128 = load ptr, ptr %.reg2mem125, align 8
  tail call void @free(ptr noundef %.reload128) #4
  %.reload124 = load ptr, ptr %.reg2mem121, align 8
  tail call void @free(ptr noundef %.reload124) #4
  %.reload120 = load ptr, ptr %.reg2mem117, align 8
  tail call void @free(ptr noundef %.reload120) #4
  %.reload116 = load ptr, ptr %.reg2mem113, align 8
  tail call void @free(ptr noundef %.reload116) #4
  %.reload112 = load ptr, ptr %.reg2mem108, align 8
  tail call void @free(ptr noundef nonnull %.reload112) #4
  %.reload107 = load ptr, ptr %.reg2mem102, align 8
  tail call void @free(ptr noundef %.reload107) #4
  %.reload101 = load ptr, ptr %.reg2mem98, align 8
  tail call void @free(ptr noundef %.reload101) #4
  %.reload97 = load ptr, ptr %.reg2mem94, align 8
  tail call void @free(ptr noundef %.reload97) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #1

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nobuiltin nounwind "no-builtins" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"Ubuntu clang version 19.1.7 (20ubuntu4)"}
!5 = distinct !{!5, !6, !7}
!6 = !{!"llvm.loop.mustprogress"}
!7 = !{!"llvm.loop.unroll.disable"}
!8 = !{!9, !9, i64 0}
!9 = !{!"double", !10, i64 0}
!10 = !{!"omnipotent char", !11, i64 0}
!11 = !{!"Simple C/C++ TBAA"}
!12 = distinct !{!12, !6, !7}
!13 = distinct !{!13, !6, !7}
!14 = distinct !{!14, !6, !7}
!15 = distinct !{!15, !6, !7}
!16 = distinct !{!16, !6, !7}
!17 = distinct !{!17, !6, !7}
!18 = distinct !{!18, !6, !7}
!19 = distinct !{!19, !6, !7}
!20 = !{!21, !21, i64 0}
!21 = !{!"any pointer", !10, i64 0}
!22 = distinct !{!22, !6, !7}

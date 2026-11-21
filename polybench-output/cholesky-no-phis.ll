; ModuleID = './polybench-output/cholesky-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/cholesky/cholesky.c"
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

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem27 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem39 = alloca i64, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem49 = alloca double, align 8
  %.reg2mem53 = alloca i64, align 8
  %.reg2mem56 = alloca i64, align 8
  %.reg2mem59 = alloca double, align 8
  %.reg2mem63 = alloca double, align 8
  %.reg2mem65 = alloca ptr, align 8
  %.reg2mem68 = alloca double, align 8
  %.reg2mem70 = alloca i64, align 8
  %.reg2mem78 = alloca double, align 8
  %.reg2mem80 = alloca ptr, align 8
  %.reg2mem83 = alloca double, align 8
  %.reg2mem85 = alloca ptr, align 8
  %.reg2mem87 = alloca i64, align 8
  %.reg2mem99 = alloca i64, align 8
  %.reg2mem102 = alloca i64, align 8
  %.reg2mem105 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem112 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem118 = alloca ptr, align 8
  %.reg2mem120 = alloca i64, align 8
  %.reg2mem124 = alloca i64, align 8
  %.reg2mem128 = alloca i64, align 8
  %.reg2mem131 = alloca i64, align 8
  %.reg2mem134 = alloca i64, align 8
  %.reg2mem137 = alloca ptr, align 8
  %.reg2mem142 = alloca i64, align 8
  %.reg2mem144 = alloca i64, align 8
  %.reg2mem147 = alloca i64, align 8
  %.reg2mem150 = alloca i64, align 8
  %.reg2mem153 = alloca i64, align 8
  %.reg2mem157 = alloca i64, align 8
  %.reg2mem163 = alloca ptr, align 8
  %.reg2mem182 = alloca i64, align 8
  %.reg2mem184 = alloca i64, align 8
  %.reg2mem186 = alloca i64, align 8
  %.reg2mem188 = alloca double, align 8
  %.reg2mem190 = alloca double, align 8
  %.reg2mem192 = alloca i64, align 8
  %.reg2mem194 = alloca double, align 8
  %.reg2mem196 = alloca double, align 8
  %.reg2mem198 = alloca i64, align 8
  %.reg2mem200 = alloca i64, align 8
  %.reg2mem202 = alloca i64, align 8
  %.reg2mem204 = alloca i64, align 8
  %.reg2mem206 = alloca i64, align 8
  %.reg2mem208 = alloca i64, align 8
  %.reg2mem210 = alloca i64, align 8
  %.reg2mem212 = alloca i64, align 8
  %.reg2mem214 = alloca i64, align 8
  %.reg2mem216 = alloca i64, align 8
  %.reg2mem218 = alloca i64, align 8
  %.reg2mem220 = alloca i64, align 8
  %.reg2mem222 = alloca i64, align 8
  %.reg2mem224 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem163, align 8
  store i64 1, ptr %.reg2mem222, align 8
  store i64 0, ptr %.reg2mem224, align 8
  br label %4

4:                                                ; preds = %._crit_edge5, %2
  %.reload225 = load i64, ptr %.reg2mem224, align 8
  %.reload223 = load i64, ptr %.reg2mem222, align 8
  store i64 %.reload225, ptr %.reg2mem157, align 8
  store i64 %.reload223, ptr %.reg2mem153, align 8
  store i64 0, ptr %.reg2mem220, align 8
  br label %5

5:                                                ; preds = %._crit_edge, %4
  %.reload221 = load i64, ptr %.reg2mem220, align 8
  %6 = trunc i64 %.reload221 to i32
  %7 = sub i32 0, %6
  %8 = sitofp i32 %7 to double
  %9 = fdiv double %8, 4.000000e+02
  %10 = fadd double %9, 1.000000e+00
  %.reload162 = load i64, ptr %.reg2mem157, align 8
  %.reload180 = load ptr, ptr %.reg2mem163, align 8
  %11 = getelementptr inbounds [400 x double], ptr %.reload180, i64 %.reload162, i64 %.reload221
  store double %10, ptr %11, align 8, !tbaa !5
  %12 = add nuw nsw i64 %.reload221, 1
  store i64 %12, ptr %.reg2mem150, align 8
  %.reload152 = load i64, ptr %.reg2mem150, align 8
  %.reload156 = load i64, ptr %.reg2mem153, align 8
  %13 = icmp eq i64 %.reload152, %.reload156
  br i1 %13, label %14, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %5
  %.reload151 = load i64, ptr %.reg2mem150, align 8
  store i64 %.reload151, ptr %.reg2mem220, align 8
  br label %5

14:                                               ; preds = %5
  %.reload161 = load i64, ptr %.reg2mem157, align 8
  %15 = add nuw nsw i64 %.reload161, 1
  store i64 %15, ptr %.reg2mem147, align 8
  %.reload160 = load i64, ptr %.reg2mem157, align 8
  %16 = icmp ult i64 %.reload160, 399
  br i1 %16, label %._crit_edge1, label %._crit_edge2

._crit_edge2:                                     ; preds = %14
  br label %21

._crit_edge1:                                     ; preds = %14
  %.reload154 = load i64, ptr %.reg2mem153, align 8
  store i64 %.reload154, ptr %.reg2mem218, align 8
  br label %17

17:                                               ; preds = %._crit_edge4, %._crit_edge1
  %.reload219 = load i64, ptr %.reg2mem218, align 8
  %.reload158 = load i64, ptr %.reg2mem157, align 8
  %.reload175 = load ptr, ptr %.reg2mem163, align 8
  %18 = getelementptr inbounds [400 x double], ptr %.reload175, i64 %.reload158, i64 %.reload219
  store double 0.000000e+00, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %.reload219, 1
  store i64 %19, ptr %.reg2mem144, align 8
  %.reload146 = load i64, ptr %.reg2mem144, align 8
  %20 = icmp eq i64 %.reload146, 400
  br i1 %20, label %._crit_edge3, label %._crit_edge4, !llvm.loop !12

._crit_edge4:                                     ; preds = %17
  %.reload145 = load i64, ptr %.reg2mem144, align 8
  store i64 %.reload145, ptr %.reg2mem218, align 8
  br label %17

._crit_edge3:                                     ; preds = %17
  br label %21

21:                                               ; preds = %._crit_edge3, %._crit_edge2
  %.reload159 = load i64, ptr %.reg2mem157, align 8
  %.reload179 = load ptr, ptr %.reg2mem163, align 8
  %22 = getelementptr inbounds [400 x double], ptr %.reload179, i64 %.reload159, i64 %.reload159
  store double 1.000000e+00, ptr %22, align 8, !tbaa !5
  %.reload155 = load i64, ptr %.reg2mem153, align 8
  %23 = add nuw nsw i64 %.reload155, 1
  store i64 %23, ptr %.reg2mem142, align 8
  %.reload149 = load i64, ptr %.reg2mem147, align 8
  %24 = icmp eq i64 %.reload149, 400
  br i1 %24, label %25, label %._crit_edge5, !llvm.loop !13

._crit_edge5:                                     ; preds = %21
  %.reload143 = load i64, ptr %.reg2mem142, align 8
  %.reload148 = load i64, ptr %.reg2mem147, align 8
  store i64 %.reload143, ptr %.reg2mem222, align 8
  store i64 %.reload148, ptr %.reg2mem224, align 8
  br label %4

25:                                               ; preds = %21
  %26 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %26, ptr %.reg2mem137, align 8
  store i64 0, ptr %.reg2mem216, align 8
  br label %27

27:                                               ; preds = %._crit_edge8, %25
  %.reload217 = load i64, ptr %.reg2mem216, align 8
  store i64 %.reload217, ptr %.reg2mem134, align 8
  store i64 0, ptr %.reg2mem214, align 8
  br label %28

28:                                               ; preds = %._crit_edge6, %27
  %.reload215 = load i64, ptr %.reg2mem214, align 8
  %.reload136 = load i64, ptr %.reg2mem134, align 8
  %.reload141 = load ptr, ptr %.reg2mem137, align 8
  %29 = getelementptr inbounds [400 x [400 x double]], ptr %.reload141, i64 0, i64 %.reload136, i64 %.reload215
  store double 0.000000e+00, ptr %29, align 8, !tbaa !5
  %30 = add nuw nsw i64 %.reload215, 1
  store i64 %30, ptr %.reg2mem131, align 8
  %.reload133 = load i64, ptr %.reg2mem131, align 8
  %31 = icmp eq i64 %.reload133, 400
  br i1 %31, label %32, label %._crit_edge6, !llvm.loop !14

._crit_edge6:                                     ; preds = %28
  %.reload132 = load i64, ptr %.reg2mem131, align 8
  store i64 %.reload132, ptr %.reg2mem214, align 8
  br label %28

32:                                               ; preds = %28
  %.reload135 = load i64, ptr %.reg2mem134, align 8
  %33 = add nuw nsw i64 %.reload135, 1
  store i64 %33, ptr %.reg2mem128, align 8
  %.reload130 = load i64, ptr %.reg2mem128, align 8
  %34 = icmp eq i64 %.reload130, 400
  br i1 %34, label %._crit_edge7, label %._crit_edge8, !llvm.loop !15

._crit_edge8:                                     ; preds = %32
  %.reload129 = load i64, ptr %.reg2mem128, align 8
  store i64 %.reload129, ptr %.reg2mem216, align 8
  br label %27

._crit_edge7:                                     ; preds = %32
  store i64 0, ptr %.reg2mem212, align 8
  br label %35

35:                                               ; preds = %._crit_edge12, %._crit_edge7
  %.reload213 = load i64, ptr %.reg2mem212, align 8
  store i64 %.reload213, ptr %.reg2mem124, align 8
  store i64 0, ptr %.reg2mem210, align 8
  br label %36

36:                                               ; preds = %._crit_edge10, %35
  %.reload211 = load i64, ptr %.reg2mem210, align 8
  store i64 %.reload211, ptr %.reg2mem120, align 8
  %.reload123 = load i64, ptr %.reg2mem120, align 8
  %.reload127 = load i64, ptr %.reg2mem124, align 8
  %.reload178 = load ptr, ptr %.reg2mem163, align 8
  %37 = getelementptr inbounds [400 x double], ptr %.reload178, i64 %.reload123, i64 %.reload127
  store ptr %37, ptr %.reg2mem118, align 8
  store i64 0, ptr %.reg2mem208, align 8
  br label %38

38:                                               ; preds = %._crit_edge9, %36
  %.reload209 = load i64, ptr %.reg2mem208, align 8
  %.reload119 = load ptr, ptr %.reg2mem118, align 8
  %39 = load double, ptr %.reload119, align 8, !tbaa !5
  %.reload126 = load i64, ptr %.reg2mem124, align 8
  %.reload177 = load ptr, ptr %.reg2mem163, align 8
  %40 = getelementptr inbounds [400 x double], ptr %.reload177, i64 %.reload209, i64 %.reload126
  %41 = load double, ptr %40, align 8, !tbaa !5
  %.reload122 = load i64, ptr %.reg2mem120, align 8
  %.reload140 = load ptr, ptr %.reg2mem137, align 8
  %42 = getelementptr inbounds [400 x [400 x double]], ptr %.reload140, i64 0, i64 %.reload122, i64 %.reload209
  %43 = load double, ptr %42, align 8, !tbaa !5
  %44 = tail call double @llvm.fmuladd.f64(double %39, double %41, double %43)
  store double %44, ptr %42, align 8, !tbaa !5
  %45 = add nuw nsw i64 %.reload209, 1
  store i64 %45, ptr %.reg2mem115, align 8
  %.reload117 = load i64, ptr %.reg2mem115, align 8
  %46 = icmp eq i64 %.reload117, 400
  br i1 %46, label %47, label %._crit_edge9, !llvm.loop !16

._crit_edge9:                                     ; preds = %38
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  store i64 %.reload116, ptr %.reg2mem208, align 8
  br label %38

47:                                               ; preds = %38
  %.reload121 = load i64, ptr %.reg2mem120, align 8
  %48 = add nuw nsw i64 %.reload121, 1
  store i64 %48, ptr %.reg2mem112, align 8
  %.reload114 = load i64, ptr %.reg2mem112, align 8
  %49 = icmp eq i64 %.reload114, 400
  br i1 %49, label %50, label %._crit_edge10, !llvm.loop !17

._crit_edge10:                                    ; preds = %47
  %.reload113 = load i64, ptr %.reg2mem112, align 8
  store i64 %.reload113, ptr %.reg2mem210, align 8
  br label %36

50:                                               ; preds = %47
  %.reload125 = load i64, ptr %.reg2mem124, align 8
  %51 = add nuw nsw i64 %.reload125, 1
  store i64 %51, ptr %.reg2mem109, align 8
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %52 = icmp eq i64 %.reload111, 400
  br i1 %52, label %._crit_edge11, label %._crit_edge12, !llvm.loop !18

._crit_edge12:                                    ; preds = %50
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  store i64 %.reload110, ptr %.reg2mem212, align 8
  br label %35

._crit_edge11:                                    ; preds = %50
  store i64 0, ptr %.reg2mem206, align 8
  br label %53

53:                                               ; preds = %._crit_edge14, %._crit_edge11
  %.reload207 = load i64, ptr %.reg2mem206, align 8
  store i64 %.reload207, ptr %.reg2mem105, align 8
  store i64 0, ptr %.reg2mem204, align 8
  br label %54

54:                                               ; preds = %._crit_edge13, %53
  %.reload205 = load i64, ptr %.reg2mem204, align 8
  %.reload108 = load i64, ptr %.reg2mem105, align 8
  %.reload139 = load ptr, ptr %.reg2mem137, align 8
  %55 = getelementptr inbounds [400 x [400 x double]], ptr %.reload139, i64 0, i64 %.reload108, i64 %.reload205
  %56 = load double, ptr %55, align 8, !tbaa !5
  %.reload107 = load i64, ptr %.reg2mem105, align 8
  %.reload176 = load ptr, ptr %.reg2mem163, align 8
  %57 = getelementptr inbounds [400 x double], ptr %.reload176, i64 %.reload107, i64 %.reload205
  store double %56, ptr %57, align 8, !tbaa !5
  %58 = add nuw nsw i64 %.reload205, 1
  store i64 %58, ptr %.reg2mem102, align 8
  %.reload104 = load i64, ptr %.reg2mem102, align 8
  %59 = icmp eq i64 %.reload104, 400
  br i1 %59, label %60, label %._crit_edge13, !llvm.loop !19

._crit_edge13:                                    ; preds = %54
  %.reload103 = load i64, ptr %.reg2mem102, align 8
  store i64 %.reload103, ptr %.reg2mem204, align 8
  br label %54

60:                                               ; preds = %54
  %.reload106 = load i64, ptr %.reg2mem105, align 8
  %61 = add nuw nsw i64 %.reload106, 1
  store i64 %61, ptr %.reg2mem99, align 8
  %.reload101 = load i64, ptr %.reg2mem99, align 8
  %62 = icmp eq i64 %.reload101, 400
  br i1 %62, label %63, label %._crit_edge14, !llvm.loop !20

._crit_edge14:                                    ; preds = %60
  %.reload100 = load i64, ptr %.reg2mem99, align 8
  store i64 %.reload100, ptr %.reg2mem206, align 8
  br label %53

63:                                               ; preds = %60
  %.reload138 = load ptr, ptr %.reg2mem137, align 8
  tail call void @free(ptr noundef nonnull %.reload138) #4
  store i64 0, ptr %.reg2mem202, align 8
  br label %64

64:                                               ; preds = %._crit_edge20, %63
  %.reload203 = load i64, ptr %.reg2mem202, align 8
  store i64 %.reload203, ptr %.reg2mem87, align 8
  %.reload98 = load i64, ptr %.reg2mem87, align 8
  %65 = icmp eq i64 %.reload98, 0
  br i1 %65, label %68, label %66

66:                                               ; preds = %64
  %.reload97 = load i64, ptr %.reg2mem87, align 8
  %.reload174 = load ptr, ptr %.reg2mem163, align 8
  %67 = getelementptr inbounds [400 x double], ptr %.reload174, i64 %.reload97, i64 0
  store ptr %67, ptr %.reg2mem85, align 8
  store i64 0, ptr %.reg2mem200, align 8
  br label %73

68:                                               ; preds = %64
  %.reload165 = load ptr, ptr %.reg2mem163, align 8
  %69 = load double, ptr %.reload165, align 8, !tbaa !5
  store double %69, ptr %.reg2mem83, align 8
  %.reload84 = load double, ptr %.reg2mem83, align 8
  store double %.reload84, ptr %.reg2mem188, align 8
  br label %103

70:                                               ; preds = %89
  %.reload92 = load i64, ptr %.reg2mem87, align 8
  %.reload168 = load ptr, ptr %.reg2mem163, align 8
  %71 = getelementptr inbounds [400 x double], ptr %.reload168, i64 %.reload92, i64 %.reload92
  store ptr %71, ptr %.reg2mem80, align 8
  %.reload82 = load ptr, ptr %.reg2mem80, align 8
  %72 = load double, ptr %.reload82, align 8, !tbaa !5
  store double %72, ptr %.reg2mem78, align 8
  %.reload79 = load double, ptr %.reg2mem78, align 8
  store double %.reload79, ptr %.reg2mem190, align 8
  store i64 0, ptr %.reg2mem192, align 8
  br label %96

73:                                               ; preds = %._crit_edge17, %66
  %.reload201 = load i64, ptr %.reg2mem200, align 8
  store i64 %.reload201, ptr %.reg2mem70, align 8
  %.reload77 = load i64, ptr %.reg2mem70, align 8
  %74 = icmp eq i64 %.reload77, 0
  br i1 %74, label %75, label %77

75:                                               ; preds = %73
  %.reload86 = load ptr, ptr %.reg2mem85, align 8
  %76 = load double, ptr %.reload86, align 8, !tbaa !5
  store double %76, ptr %.reg2mem68, align 8
  %.reload69 = load double, ptr %.reg2mem68, align 8
  store double %.reload69, ptr %.reg2mem194, align 8
  br label %89

77:                                               ; preds = %73
  %.reload76 = load i64, ptr %.reg2mem70, align 8
  %.reload96 = load i64, ptr %.reg2mem87, align 8
  %.reload173 = load ptr, ptr %.reg2mem163, align 8
  %78 = getelementptr inbounds [400 x double], ptr %.reload173, i64 %.reload96, i64 %.reload76
  store ptr %78, ptr %.reg2mem65, align 8
  %.reload67 = load ptr, ptr %.reg2mem65, align 8
  %79 = load double, ptr %.reload67, align 8, !tbaa !5
  store double %79, ptr %.reg2mem63, align 8
  %.reload64 = load double, ptr %.reg2mem63, align 8
  store double %.reload64, ptr %.reg2mem196, align 8
  store i64 0, ptr %.reg2mem198, align 8
  br label %80

80:                                               ; preds = %._crit_edge16, %77
  %.reload199 = load i64, ptr %.reg2mem198, align 8
  %.reload197 = load double, ptr %.reg2mem196, align 8
  %.reload95 = load i64, ptr %.reg2mem87, align 8
  %.reload172 = load ptr, ptr %.reg2mem163, align 8
  %81 = getelementptr inbounds [400 x double], ptr %.reload172, i64 %.reload95, i64 %.reload199
  %82 = load double, ptr %81, align 8, !tbaa !5
  %.reload75 = load i64, ptr %.reg2mem70, align 8
  %.reload171 = load ptr, ptr %.reg2mem163, align 8
  %83 = getelementptr inbounds [400 x double], ptr %.reload171, i64 %.reload75, i64 %.reload199
  %84 = load double, ptr %83, align 8, !tbaa !5
  %85 = fneg double %82
  %86 = tail call double @llvm.fmuladd.f64(double %85, double %84, double %.reload197)
  store double %86, ptr %.reg2mem59, align 8
  %.reload62 = load double, ptr %.reg2mem59, align 8
  %.reload66 = load ptr, ptr %.reg2mem65, align 8
  store double %.reload62, ptr %.reload66, align 8, !tbaa !5
  %87 = add nuw nsw i64 %.reload199, 1
  store i64 %87, ptr %.reg2mem56, align 8
  %.reload58 = load i64, ptr %.reg2mem56, align 8
  %.reload74 = load i64, ptr %.reg2mem70, align 8
  %88 = icmp eq i64 %.reload58, %.reload74
  br i1 %88, label %._crit_edge15, label %._crit_edge16, !llvm.loop !21

._crit_edge16:                                    ; preds = %80
  %.reload57 = load i64, ptr %.reg2mem56, align 8
  %.reload61 = load double, ptr %.reg2mem59, align 8
  store double %.reload61, ptr %.reg2mem196, align 8
  store i64 %.reload57, ptr %.reg2mem198, align 8
  br label %80

._crit_edge15:                                    ; preds = %80
  %.reload60 = load double, ptr %.reg2mem59, align 8
  store double %.reload60, ptr %.reg2mem194, align 8
  br label %89

89:                                               ; preds = %._crit_edge15, %75
  %.reload195 = load double, ptr %.reg2mem194, align 8
  %.reload73 = load i64, ptr %.reg2mem70, align 8
  %.reload170 = load ptr, ptr %.reg2mem163, align 8
  %90 = getelementptr inbounds [400 x double], ptr %.reload170, i64 %.reload73, i64 %.reload73
  %91 = load double, ptr %90, align 8, !tbaa !5
  %.reload72 = load i64, ptr %.reg2mem70, align 8
  %.reload94 = load i64, ptr %.reg2mem87, align 8
  %.reload169 = load ptr, ptr %.reg2mem163, align 8
  %92 = getelementptr inbounds [400 x double], ptr %.reload169, i64 %.reload94, i64 %.reload72
  %93 = fdiv double %.reload195, %91
  store double %93, ptr %92, align 8, !tbaa !5
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  %94 = add nuw nsw i64 %.reload71, 1
  store i64 %94, ptr %.reg2mem53, align 8
  %.reload55 = load i64, ptr %.reg2mem53, align 8
  %.reload93 = load i64, ptr %.reg2mem87, align 8
  %95 = icmp eq i64 %.reload55, %.reload93
  br i1 %95, label %70, label %._crit_edge17, !llvm.loop !22

._crit_edge17:                                    ; preds = %89
  %.reload54 = load i64, ptr %.reg2mem53, align 8
  store i64 %.reload54, ptr %.reg2mem200, align 8
  br label %73

96:                                               ; preds = %._crit_edge19, %70
  %.reload193 = load i64, ptr %.reg2mem192, align 8
  %.reload191 = load double, ptr %.reg2mem190, align 8
  %.reload91 = load i64, ptr %.reg2mem87, align 8
  %.reload167 = load ptr, ptr %.reg2mem163, align 8
  %97 = getelementptr inbounds [400 x double], ptr %.reload167, i64 %.reload91, i64 %.reload193
  %98 = load double, ptr %97, align 8, !tbaa !5
  %99 = fneg double %98
  %100 = tail call double @llvm.fmuladd.f64(double %99, double %98, double %.reload191)
  store double %100, ptr %.reg2mem49, align 8
  %.reload52 = load double, ptr %.reg2mem49, align 8
  %.reload81 = load ptr, ptr %.reg2mem80, align 8
  store double %.reload52, ptr %.reload81, align 8, !tbaa !5
  %101 = add nuw nsw i64 %.reload193, 1
  store i64 %101, ptr %.reg2mem46, align 8
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %.reload90 = load i64, ptr %.reg2mem87, align 8
  %102 = icmp eq i64 %.reload48, %.reload90
  br i1 %102, label %._crit_edge18, label %._crit_edge19, !llvm.loop !23

._crit_edge19:                                    ; preds = %96
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %.reload51 = load double, ptr %.reg2mem49, align 8
  store double %.reload51, ptr %.reg2mem190, align 8
  store i64 %.reload47, ptr %.reg2mem192, align 8
  br label %96

._crit_edge18:                                    ; preds = %96
  %.reload50 = load double, ptr %.reg2mem49, align 8
  store double %.reload50, ptr %.reg2mem188, align 8
  br label %103

103:                                              ; preds = %._crit_edge18, %68
  %.reload189 = load double, ptr %.reg2mem188, align 8
  %.reload89 = load i64, ptr %.reg2mem87, align 8
  %.reload166 = load ptr, ptr %.reg2mem163, align 8
  %104 = getelementptr inbounds [400 x double], ptr %.reload166, i64 %.reload89, i64 %.reload89
  %105 = tail call double @sqrt(double noundef %.reload189) #4
  store double %105, ptr %104, align 8, !tbaa !5
  %.reload88 = load i64, ptr %.reg2mem87, align 8
  %106 = add nuw nsw i64 %.reload88, 1
  store i64 %106, ptr %.reg2mem43, align 8
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %107 = icmp eq i64 %.reload45, 400
  br i1 %107, label %108, label %._crit_edge20, !llvm.loop !24

._crit_edge20:                                    ; preds = %103
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  store i64 %.reload44, ptr %.reg2mem202, align 8
  br label %64

108:                                              ; preds = %103
  %109 = load ptr, ptr @stderr, align 8, !tbaa !25
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str) #4
  %111 = load ptr, ptr @stderr, align 8, !tbaa !25
  %112 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %111, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 1, ptr %.reg2mem184, align 8
  store i64 0, ptr %.reg2mem186, align 8
  br label %113

113:                                              ; preds = %._crit_edge23, %108
  %.reload187 = load i64, ptr %.reg2mem186, align 8
  %.reload185 = load i64, ptr %.reg2mem184, align 8
  store i64 %.reload187, ptr %.reg2mem39, align 8
  store i64 %.reload185, ptr %.reg2mem36, align 8
  %.reload42 = load i64, ptr %.reg2mem39, align 8
  %114 = mul nuw nsw i64 %.reload42, 400
  store i64 %114, ptr %.reg2mem34, align 8
  store i64 0, ptr %.reg2mem182, align 8
  br label %115

115:                                              ; preds = %._crit_edge22, %113
  %.reload183 = load i64, ptr %.reg2mem182, align 8
  store i64 %.reload183, ptr %.reg2mem30, align 8
  %.reload33 = load i64, ptr %.reg2mem30, align 8
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %116 = add nuw nsw i64 %.reload33, %.reload35
  %117 = trunc nuw nsw i64 %116 to i32
  %118 = urem i32 %117, 20
  %119 = icmp eq i32 %118, 0
  br i1 %119, label %120, label %._crit_edge21

._crit_edge21:                                    ; preds = %115
  br label %123

120:                                              ; preds = %115
  %121 = load ptr, ptr @stderr, align 8, !tbaa !25
  %122 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %121, ptr noundef nonnull @.str.3) #4
  br label %123

123:                                              ; preds = %._crit_edge21, %120
  %124 = load ptr, ptr @stderr, align 8, !tbaa !25
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %.reload41 = load i64, ptr %.reg2mem39, align 8
  %.reload164 = load ptr, ptr %.reg2mem163, align 8
  %125 = getelementptr inbounds [400 x double], ptr %.reload164, i64 %.reload41, i64 %.reload32
  %126 = load double, ptr %125, align 8, !tbaa !5
  %127 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %124, ptr noundef nonnull @.str.4, double noundef %126) #4
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  %128 = add nuw nsw i64 %.reload31, 1
  store i64 %128, ptr %.reg2mem27, align 8
  %.reload29 = load i64, ptr %.reg2mem27, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %129 = icmp eq i64 %.reload29, %.reload38
  br i1 %129, label %130, label %._crit_edge22, !llvm.loop !27

._crit_edge22:                                    ; preds = %123
  %.reload28 = load i64, ptr %.reg2mem27, align 8
  store i64 %.reload28, ptr %.reg2mem182, align 8
  br label %115

130:                                              ; preds = %123
  %.reload40 = load i64, ptr %.reg2mem39, align 8
  %131 = add nuw nsw i64 %.reload40, 1
  store i64 %131, ptr %.reg2mem24, align 8
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  %132 = add nuw nsw i64 %.reload37, 1
  store i64 %132, ptr %.reg2mem, align 8
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %133 = icmp eq i64 %.reload26, 400
  br i1 %133, label %134, label %._crit_edge23, !llvm.loop !28

._crit_edge23:                                    ; preds = %130
  %.reload = load i64, ptr %.reg2mem, align 8
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  store i64 %.reload, ptr %.reg2mem184, align 8
  store i64 %.reload25, ptr %.reg2mem186, align 8
  br label %113

134:                                              ; preds = %130
  %135 = load ptr, ptr @stderr, align 8, !tbaa !25
  %136 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %135, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %137 = load ptr, ptr @stderr, align 8, !tbaa !25
  %138 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %137, ptr noundef nonnull @.str.6) #4
  %.reload181 = load ptr, ptr %.reg2mem163, align 8
  tail call void @free(ptr noundef nonnull %.reload181) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

; Function Attrs: nounwind
declare double @sqrt(double noundef) local_unnamed_addr #2

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
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = !{!26, !26, i64 0}
!26 = !{!"any pointer", !7, i64 0}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}

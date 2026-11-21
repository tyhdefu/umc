; ModuleID = './polybench-output/lu-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/lu/lu.c"
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
  %.reg2mem29 = alloca i64, align 8
  %.reg2mem32 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem51 = alloca double, align 8
  %.reg2mem54 = alloca double, align 8
  %.reg2mem56 = alloca ptr, align 8
  %.reg2mem59 = alloca i64, align 8
  %.reg2mem63 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem69 = alloca double, align 8
  %.reg2mem73 = alloca double, align 8
  %.reg2mem75 = alloca ptr, align 8
  %.reg2mem78 = alloca double, align 8
  %.reg2mem80 = alloca i64, align 8
  %.reg2mem88 = alloca ptr, align 8
  %.reg2mem90 = alloca i1, align 1
  %.reg2mem93 = alloca i64, align 8
  %.reg2mem105 = alloca i64, align 8
  %.reg2mem108 = alloca i64, align 8
  %.reg2mem111 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem118 = alloca i64, align 8
  %.reg2mem121 = alloca i64, align 8
  %.reg2mem124 = alloca ptr, align 8
  %.reg2mem126 = alloca i64, align 8
  %.reg2mem130 = alloca i64, align 8
  %.reg2mem134 = alloca i64, align 8
  %.reg2mem137 = alloca i64, align 8
  %.reg2mem140 = alloca i64, align 8
  %.reg2mem143 = alloca ptr, align 8
  %.reg2mem148 = alloca i64, align 8
  %.reg2mem150 = alloca i64, align 8
  %.reg2mem153 = alloca i64, align 8
  %.reg2mem156 = alloca i64, align 8
  %.reg2mem159 = alloca i64, align 8
  %.reg2mem163 = alloca i64, align 8
  %.reg2mem169 = alloca ptr, align 8
  %.reg2mem187 = alloca i64, align 8
  %.reg2mem189 = alloca i64, align 8
  %.reg2mem191 = alloca double, align 8
  %.reg2mem193 = alloca i64, align 8
  %.reg2mem195 = alloca i64, align 8
  %.reg2mem197 = alloca double, align 8
  %.reg2mem199 = alloca double, align 8
  %.reg2mem201 = alloca i64, align 8
  %.reg2mem203 = alloca i64, align 8
  %.reg2mem205 = alloca i64, align 8
  %.reg2mem207 = alloca i64, align 8
  %.reg2mem209 = alloca i64, align 8
  %.reg2mem211 = alloca i64, align 8
  %.reg2mem213 = alloca i64, align 8
  %.reg2mem215 = alloca i64, align 8
  %.reg2mem217 = alloca i64, align 8
  %.reg2mem219 = alloca i64, align 8
  %.reg2mem221 = alloca i64, align 8
  %.reg2mem223 = alloca i64, align 8
  %.reg2mem225 = alloca i64, align 8
  %.reg2mem227 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem169, align 8
  store i64 1, ptr %.reg2mem225, align 8
  store i64 0, ptr %.reg2mem227, align 8
  br label %4

4:                                                ; preds = %._crit_edge5, %2
  %.reload228 = load i64, ptr %.reg2mem227, align 8
  %.reload226 = load i64, ptr %.reg2mem225, align 8
  store i64 %.reload228, ptr %.reg2mem163, align 8
  store i64 %.reload226, ptr %.reg2mem159, align 8
  store i64 0, ptr %.reg2mem223, align 8
  br label %5

5:                                                ; preds = %._crit_edge, %4
  %.reload224 = load i64, ptr %.reg2mem223, align 8
  %6 = trunc i64 %.reload224 to i32
  %7 = sub i32 0, %6
  %8 = sitofp i32 %7 to double
  %9 = fdiv double %8, 4.000000e+02
  %10 = fadd double %9, 1.000000e+00
  %.reload168 = load i64, ptr %.reg2mem163, align 8
  %.reload185 = load ptr, ptr %.reg2mem169, align 8
  %11 = getelementptr inbounds [400 x double], ptr %.reload185, i64 %.reload168, i64 %.reload224
  store double %10, ptr %11, align 8, !tbaa !5
  %12 = add nuw nsw i64 %.reload224, 1
  store i64 %12, ptr %.reg2mem156, align 8
  %.reload158 = load i64, ptr %.reg2mem156, align 8
  %.reload162 = load i64, ptr %.reg2mem159, align 8
  %13 = icmp eq i64 %.reload158, %.reload162
  br i1 %13, label %14, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %5
  %.reload157 = load i64, ptr %.reg2mem156, align 8
  store i64 %.reload157, ptr %.reg2mem223, align 8
  br label %5

14:                                               ; preds = %5
  %.reload167 = load i64, ptr %.reg2mem163, align 8
  %15 = add nuw nsw i64 %.reload167, 1
  store i64 %15, ptr %.reg2mem153, align 8
  %.reload166 = load i64, ptr %.reg2mem163, align 8
  %16 = icmp ult i64 %.reload166, 399
  br i1 %16, label %._crit_edge1, label %._crit_edge2

._crit_edge2:                                     ; preds = %14
  br label %21

._crit_edge1:                                     ; preds = %14
  %.reload160 = load i64, ptr %.reg2mem159, align 8
  store i64 %.reload160, ptr %.reg2mem221, align 8
  br label %17

17:                                               ; preds = %._crit_edge4, %._crit_edge1
  %.reload222 = load i64, ptr %.reg2mem221, align 8
  %.reload164 = load i64, ptr %.reg2mem163, align 8
  %.reload180 = load ptr, ptr %.reg2mem169, align 8
  %18 = getelementptr inbounds [400 x double], ptr %.reload180, i64 %.reload164, i64 %.reload222
  store double 0.000000e+00, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %.reload222, 1
  store i64 %19, ptr %.reg2mem150, align 8
  %.reload152 = load i64, ptr %.reg2mem150, align 8
  %20 = icmp eq i64 %.reload152, 400
  br i1 %20, label %._crit_edge3, label %._crit_edge4, !llvm.loop !12

._crit_edge4:                                     ; preds = %17
  %.reload151 = load i64, ptr %.reg2mem150, align 8
  store i64 %.reload151, ptr %.reg2mem221, align 8
  br label %17

._crit_edge3:                                     ; preds = %17
  br label %21

21:                                               ; preds = %._crit_edge3, %._crit_edge2
  %.reload165 = load i64, ptr %.reg2mem163, align 8
  %.reload184 = load ptr, ptr %.reg2mem169, align 8
  %22 = getelementptr inbounds [400 x double], ptr %.reload184, i64 %.reload165, i64 %.reload165
  store double 1.000000e+00, ptr %22, align 8, !tbaa !5
  %.reload161 = load i64, ptr %.reg2mem159, align 8
  %23 = add nuw nsw i64 %.reload161, 1
  store i64 %23, ptr %.reg2mem148, align 8
  %.reload155 = load i64, ptr %.reg2mem153, align 8
  %24 = icmp eq i64 %.reload155, 400
  br i1 %24, label %25, label %._crit_edge5, !llvm.loop !13

._crit_edge5:                                     ; preds = %21
  %.reload149 = load i64, ptr %.reg2mem148, align 8
  %.reload154 = load i64, ptr %.reg2mem153, align 8
  store i64 %.reload149, ptr %.reg2mem225, align 8
  store i64 %.reload154, ptr %.reg2mem227, align 8
  br label %4

25:                                               ; preds = %21
  %26 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %26, ptr %.reg2mem143, align 8
  store i64 0, ptr %.reg2mem219, align 8
  br label %27

27:                                               ; preds = %._crit_edge8, %25
  %.reload220 = load i64, ptr %.reg2mem219, align 8
  store i64 %.reload220, ptr %.reg2mem140, align 8
  store i64 0, ptr %.reg2mem217, align 8
  br label %28

28:                                               ; preds = %._crit_edge6, %27
  %.reload218 = load i64, ptr %.reg2mem217, align 8
  %.reload142 = load i64, ptr %.reg2mem140, align 8
  %.reload147 = load ptr, ptr %.reg2mem143, align 8
  %29 = getelementptr inbounds [400 x [400 x double]], ptr %.reload147, i64 0, i64 %.reload142, i64 %.reload218
  store double 0.000000e+00, ptr %29, align 8, !tbaa !5
  %30 = add nuw nsw i64 %.reload218, 1
  store i64 %30, ptr %.reg2mem137, align 8
  %.reload139 = load i64, ptr %.reg2mem137, align 8
  %31 = icmp eq i64 %.reload139, 400
  br i1 %31, label %32, label %._crit_edge6, !llvm.loop !14

._crit_edge6:                                     ; preds = %28
  %.reload138 = load i64, ptr %.reg2mem137, align 8
  store i64 %.reload138, ptr %.reg2mem217, align 8
  br label %28

32:                                               ; preds = %28
  %.reload141 = load i64, ptr %.reg2mem140, align 8
  %33 = add nuw nsw i64 %.reload141, 1
  store i64 %33, ptr %.reg2mem134, align 8
  %.reload136 = load i64, ptr %.reg2mem134, align 8
  %34 = icmp eq i64 %.reload136, 400
  br i1 %34, label %._crit_edge7, label %._crit_edge8, !llvm.loop !15

._crit_edge8:                                     ; preds = %32
  %.reload135 = load i64, ptr %.reg2mem134, align 8
  store i64 %.reload135, ptr %.reg2mem219, align 8
  br label %27

._crit_edge7:                                     ; preds = %32
  store i64 0, ptr %.reg2mem215, align 8
  br label %35

35:                                               ; preds = %._crit_edge12, %._crit_edge7
  %.reload216 = load i64, ptr %.reg2mem215, align 8
  store i64 %.reload216, ptr %.reg2mem130, align 8
  store i64 0, ptr %.reg2mem213, align 8
  br label %36

36:                                               ; preds = %._crit_edge10, %35
  %.reload214 = load i64, ptr %.reg2mem213, align 8
  store i64 %.reload214, ptr %.reg2mem126, align 8
  %.reload129 = load i64, ptr %.reg2mem126, align 8
  %.reload133 = load i64, ptr %.reg2mem130, align 8
  %.reload183 = load ptr, ptr %.reg2mem169, align 8
  %37 = getelementptr inbounds [400 x double], ptr %.reload183, i64 %.reload129, i64 %.reload133
  store ptr %37, ptr %.reg2mem124, align 8
  store i64 0, ptr %.reg2mem211, align 8
  br label %38

38:                                               ; preds = %._crit_edge9, %36
  %.reload212 = load i64, ptr %.reg2mem211, align 8
  %.reload125 = load ptr, ptr %.reg2mem124, align 8
  %39 = load double, ptr %.reload125, align 8, !tbaa !5
  %.reload132 = load i64, ptr %.reg2mem130, align 8
  %.reload182 = load ptr, ptr %.reg2mem169, align 8
  %40 = getelementptr inbounds [400 x double], ptr %.reload182, i64 %.reload212, i64 %.reload132
  %41 = load double, ptr %40, align 8, !tbaa !5
  %.reload128 = load i64, ptr %.reg2mem126, align 8
  %.reload146 = load ptr, ptr %.reg2mem143, align 8
  %42 = getelementptr inbounds [400 x [400 x double]], ptr %.reload146, i64 0, i64 %.reload128, i64 %.reload212
  %43 = load double, ptr %42, align 8, !tbaa !5
  %44 = tail call double @llvm.fmuladd.f64(double %39, double %41, double %43)
  store double %44, ptr %42, align 8, !tbaa !5
  %45 = add nuw nsw i64 %.reload212, 1
  store i64 %45, ptr %.reg2mem121, align 8
  %.reload123 = load i64, ptr %.reg2mem121, align 8
  %46 = icmp eq i64 %.reload123, 400
  br i1 %46, label %47, label %._crit_edge9, !llvm.loop !16

._crit_edge9:                                     ; preds = %38
  %.reload122 = load i64, ptr %.reg2mem121, align 8
  store i64 %.reload122, ptr %.reg2mem211, align 8
  br label %38

47:                                               ; preds = %38
  %.reload127 = load i64, ptr %.reg2mem126, align 8
  %48 = add nuw nsw i64 %.reload127, 1
  store i64 %48, ptr %.reg2mem118, align 8
  %.reload120 = load i64, ptr %.reg2mem118, align 8
  %49 = icmp eq i64 %.reload120, 400
  br i1 %49, label %50, label %._crit_edge10, !llvm.loop !17

._crit_edge10:                                    ; preds = %47
  %.reload119 = load i64, ptr %.reg2mem118, align 8
  store i64 %.reload119, ptr %.reg2mem213, align 8
  br label %36

50:                                               ; preds = %47
  %.reload131 = load i64, ptr %.reg2mem130, align 8
  %51 = add nuw nsw i64 %.reload131, 1
  store i64 %51, ptr %.reg2mem115, align 8
  %.reload117 = load i64, ptr %.reg2mem115, align 8
  %52 = icmp eq i64 %.reload117, 400
  br i1 %52, label %._crit_edge11, label %._crit_edge12, !llvm.loop !18

._crit_edge12:                                    ; preds = %50
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  store i64 %.reload116, ptr %.reg2mem215, align 8
  br label %35

._crit_edge11:                                    ; preds = %50
  store i64 0, ptr %.reg2mem209, align 8
  br label %53

53:                                               ; preds = %._crit_edge14, %._crit_edge11
  %.reload210 = load i64, ptr %.reg2mem209, align 8
  store i64 %.reload210, ptr %.reg2mem111, align 8
  store i64 0, ptr %.reg2mem207, align 8
  br label %54

54:                                               ; preds = %._crit_edge13, %53
  %.reload208 = load i64, ptr %.reg2mem207, align 8
  %.reload114 = load i64, ptr %.reg2mem111, align 8
  %.reload145 = load ptr, ptr %.reg2mem143, align 8
  %55 = getelementptr inbounds [400 x [400 x double]], ptr %.reload145, i64 0, i64 %.reload114, i64 %.reload208
  %56 = load double, ptr %55, align 8, !tbaa !5
  %.reload113 = load i64, ptr %.reg2mem111, align 8
  %.reload181 = load ptr, ptr %.reg2mem169, align 8
  %57 = getelementptr inbounds [400 x double], ptr %.reload181, i64 %.reload113, i64 %.reload208
  store double %56, ptr %57, align 8, !tbaa !5
  %58 = add nuw nsw i64 %.reload208, 1
  store i64 %58, ptr %.reg2mem108, align 8
  %.reload110 = load i64, ptr %.reg2mem108, align 8
  %59 = icmp eq i64 %.reload110, 400
  br i1 %59, label %60, label %._crit_edge13, !llvm.loop !19

._crit_edge13:                                    ; preds = %54
  %.reload109 = load i64, ptr %.reg2mem108, align 8
  store i64 %.reload109, ptr %.reg2mem207, align 8
  br label %54

60:                                               ; preds = %54
  %.reload112 = load i64, ptr %.reg2mem111, align 8
  %61 = add nuw nsw i64 %.reload112, 1
  store i64 %61, ptr %.reg2mem105, align 8
  %.reload107 = load i64, ptr %.reg2mem105, align 8
  %62 = icmp eq i64 %.reload107, 400
  br i1 %62, label %63, label %._crit_edge14, !llvm.loop !20

._crit_edge14:                                    ; preds = %60
  %.reload106 = load i64, ptr %.reg2mem105, align 8
  store i64 %.reload106, ptr %.reg2mem209, align 8
  br label %53

63:                                               ; preds = %60
  %.reload144 = load ptr, ptr %.reg2mem143, align 8
  tail call void @free(ptr noundef nonnull %.reload144) #4
  store i64 0, ptr %.reg2mem205, align 8
  br label %64

64:                                               ; preds = %._crit_edge24, %63
  %.reload206 = load i64, ptr %.reg2mem205, align 8
  store i64 %.reload206, ptr %.reg2mem93, align 8
  %.reload104 = load i64, ptr %.reg2mem93, align 8
  %65 = icmp eq i64 %.reload104, 0
  store i1 %65, ptr %.reg2mem90, align 1
  %.reload92 = load i1, ptr %.reg2mem90, align 1
  br i1 %.reload92, label %._crit_edge15, label %66

._crit_edge15:                                    ; preds = %64
  br label %91

66:                                               ; preds = %64
  %.reload103 = load i64, ptr %.reg2mem93, align 8
  %.reload179 = load ptr, ptr %.reg2mem169, align 8
  %67 = getelementptr inbounds [400 x double], ptr %.reload179, i64 %.reload103, i64 0
  store ptr %67, ptr %.reg2mem88, align 8
  store i64 0, ptr %.reg2mem203, align 8
  br label %68

68:                                               ; preds = %._crit_edge19, %66
  %.reload204 = load i64, ptr %.reg2mem203, align 8
  store i64 %.reload204, ptr %.reg2mem80, align 8
  %.reload87 = load i64, ptr %.reg2mem80, align 8
  %69 = icmp eq i64 %.reload87, 0
  br i1 %69, label %70, label %72

70:                                               ; preds = %68
  %.reload89 = load ptr, ptr %.reg2mem88, align 8
  %71 = load double, ptr %.reload89, align 8, !tbaa !5
  store double %71, ptr %.reg2mem78, align 8
  %.reload79 = load double, ptr %.reg2mem78, align 8
  store double %.reload79, ptr %.reg2mem197, align 8
  br label %84

72:                                               ; preds = %68
  %.reload86 = load i64, ptr %.reg2mem80, align 8
  %.reload102 = load i64, ptr %.reg2mem93, align 8
  %.reload178 = load ptr, ptr %.reg2mem169, align 8
  %73 = getelementptr inbounds [400 x double], ptr %.reload178, i64 %.reload102, i64 %.reload86
  store ptr %73, ptr %.reg2mem75, align 8
  %.reload77 = load ptr, ptr %.reg2mem75, align 8
  %74 = load double, ptr %.reload77, align 8, !tbaa !5
  store double %74, ptr %.reg2mem73, align 8
  %.reload74 = load double, ptr %.reg2mem73, align 8
  store double %.reload74, ptr %.reg2mem199, align 8
  store i64 0, ptr %.reg2mem201, align 8
  br label %75

75:                                               ; preds = %._crit_edge17, %72
  %.reload202 = load i64, ptr %.reg2mem201, align 8
  %.reload200 = load double, ptr %.reg2mem199, align 8
  %.reload101 = load i64, ptr %.reg2mem93, align 8
  %.reload177 = load ptr, ptr %.reg2mem169, align 8
  %76 = getelementptr inbounds [400 x double], ptr %.reload177, i64 %.reload101, i64 %.reload202
  %77 = load double, ptr %76, align 8, !tbaa !5
  %.reload85 = load i64, ptr %.reg2mem80, align 8
  %.reload176 = load ptr, ptr %.reg2mem169, align 8
  %78 = getelementptr inbounds [400 x double], ptr %.reload176, i64 %.reload202, i64 %.reload85
  %79 = load double, ptr %78, align 8, !tbaa !5
  %80 = fneg double %77
  %81 = tail call double @llvm.fmuladd.f64(double %80, double %79, double %.reload200)
  store double %81, ptr %.reg2mem69, align 8
  %.reload72 = load double, ptr %.reg2mem69, align 8
  %.reload76 = load ptr, ptr %.reg2mem75, align 8
  store double %.reload72, ptr %.reload76, align 8, !tbaa !5
  %82 = add nuw nsw i64 %.reload202, 1
  store i64 %82, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %.reload84 = load i64, ptr %.reg2mem80, align 8
  %83 = icmp eq i64 %.reload68, %.reload84
  br i1 %83, label %._crit_edge16, label %._crit_edge17, !llvm.loop !21

._crit_edge17:                                    ; preds = %75
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  %.reload71 = load double, ptr %.reg2mem69, align 8
  store double %.reload71, ptr %.reg2mem199, align 8
  store i64 %.reload67, ptr %.reg2mem201, align 8
  br label %75

._crit_edge16:                                    ; preds = %75
  %.reload70 = load double, ptr %.reg2mem69, align 8
  store double %.reload70, ptr %.reg2mem197, align 8
  br label %84

84:                                               ; preds = %._crit_edge16, %70
  %.reload198 = load double, ptr %.reg2mem197, align 8
  %.reload83 = load i64, ptr %.reg2mem80, align 8
  %.reload175 = load ptr, ptr %.reg2mem169, align 8
  %85 = getelementptr inbounds [400 x double], ptr %.reload175, i64 %.reload83, i64 %.reload83
  %86 = load double, ptr %85, align 8, !tbaa !5
  %.reload82 = load i64, ptr %.reg2mem80, align 8
  %.reload100 = load i64, ptr %.reg2mem93, align 8
  %.reload174 = load ptr, ptr %.reg2mem169, align 8
  %87 = getelementptr inbounds [400 x double], ptr %.reload174, i64 %.reload100, i64 %.reload82
  %88 = fdiv double %.reload198, %86
  store double %88, ptr %87, align 8, !tbaa !5
  %.reload81 = load i64, ptr %.reg2mem80, align 8
  %89 = add nuw nsw i64 %.reload81, 1
  store i64 %89, ptr %.reg2mem63, align 8
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %.reload99 = load i64, ptr %.reg2mem93, align 8
  %90 = icmp eq i64 %.reload65, %.reload99
  br i1 %90, label %._crit_edge18, label %._crit_edge19, !llvm.loop !22

._crit_edge19:                                    ; preds = %84
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  store i64 %.reload64, ptr %.reg2mem203, align 8
  br label %68

._crit_edge18:                                    ; preds = %84
  br label %91

91:                                               ; preds = %._crit_edge18, %._crit_edge15
  %.reload94 = load i64, ptr %.reg2mem93, align 8
  store i64 %.reload94, ptr %.reg2mem195, align 8
  br label %92

92:                                               ; preds = %._crit_edge23, %91
  %.reload196 = load i64, ptr %.reg2mem195, align 8
  store i64 %.reload196, ptr %.reg2mem59, align 8
  %.reload91 = load i1, ptr %.reg2mem90, align 1
  br i1 %.reload91, label %._crit_edge20, label %93

._crit_edge20:                                    ; preds = %92
  br label %105

93:                                               ; preds = %92
  %.reload62 = load i64, ptr %.reg2mem59, align 8
  %.reload98 = load i64, ptr %.reg2mem93, align 8
  %.reload173 = load ptr, ptr %.reg2mem169, align 8
  %94 = getelementptr inbounds [400 x double], ptr %.reload173, i64 %.reload98, i64 %.reload62
  store ptr %94, ptr %.reg2mem56, align 8
  %.reload58 = load ptr, ptr %.reg2mem56, align 8
  %95 = load double, ptr %.reload58, align 8, !tbaa !5
  store double %95, ptr %.reg2mem54, align 8
  %.reload55 = load double, ptr %.reg2mem54, align 8
  store double %.reload55, ptr %.reg2mem191, align 8
  store i64 0, ptr %.reg2mem193, align 8
  br label %96

96:                                               ; preds = %._crit_edge22, %93
  %.reload194 = load i64, ptr %.reg2mem193, align 8
  %.reload192 = load double, ptr %.reg2mem191, align 8
  %.reload97 = load i64, ptr %.reg2mem93, align 8
  %.reload172 = load ptr, ptr %.reg2mem169, align 8
  %97 = getelementptr inbounds [400 x double], ptr %.reload172, i64 %.reload97, i64 %.reload194
  %98 = load double, ptr %97, align 8, !tbaa !5
  %.reload61 = load i64, ptr %.reg2mem59, align 8
  %.reload171 = load ptr, ptr %.reg2mem169, align 8
  %99 = getelementptr inbounds [400 x double], ptr %.reload171, i64 %.reload194, i64 %.reload61
  %100 = load double, ptr %99, align 8, !tbaa !5
  %101 = fneg double %98
  %102 = tail call double @llvm.fmuladd.f64(double %101, double %100, double %.reload192)
  store double %102, ptr %.reg2mem51, align 8
  %.reload53 = load double, ptr %.reg2mem51, align 8
  %.reload57 = load ptr, ptr %.reg2mem56, align 8
  store double %.reload53, ptr %.reload57, align 8, !tbaa !5
  %103 = add nuw nsw i64 %.reload194, 1
  store i64 %103, ptr %.reg2mem48, align 8
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %.reload96 = load i64, ptr %.reg2mem93, align 8
  %104 = icmp eq i64 %.reload50, %.reload96
  br i1 %104, label %._crit_edge21, label %._crit_edge22, !llvm.loop !23

._crit_edge22:                                    ; preds = %96
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %.reload52 = load double, ptr %.reg2mem51, align 8
  store double %.reload52, ptr %.reg2mem191, align 8
  store i64 %.reload49, ptr %.reg2mem193, align 8
  br label %96

._crit_edge21:                                    ; preds = %96
  br label %105

105:                                              ; preds = %._crit_edge21, %._crit_edge20
  %.reload60 = load i64, ptr %.reg2mem59, align 8
  %106 = add nuw nsw i64 %.reload60, 1
  store i64 %106, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %107 = icmp eq i64 %.reload47, 400
  br i1 %107, label %108, label %._crit_edge23, !llvm.loop !24

._crit_edge23:                                    ; preds = %105
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  store i64 %.reload46, ptr %.reg2mem195, align 8
  br label %92

108:                                              ; preds = %105
  %.reload95 = load i64, ptr %.reg2mem93, align 8
  %109 = add nuw nsw i64 %.reload95, 1
  store i64 %109, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %110 = icmp eq i64 %.reload44, 400
  br i1 %110, label %111, label %._crit_edge24, !llvm.loop !25

._crit_edge24:                                    ; preds = %108
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  store i64 %.reload43, ptr %.reg2mem205, align 8
  br label %64

111:                                              ; preds = %108
  %112 = load ptr, ptr @stderr, align 8, !tbaa !26
  %113 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %112, ptr noundef nonnull @.str) #4
  %114 = load ptr, ptr @stderr, align 8, !tbaa !26
  %115 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %114, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem189, align 8
  br label %116

116:                                              ; preds = %._crit_edge27, %111
  %.reload190 = load i64, ptr %.reg2mem189, align 8
  store i64 %.reload190, ptr %.reg2mem38, align 8
  %.reload41 = load i64, ptr %.reg2mem38, align 8
  %117 = mul nuw nsw i64 %.reload41, 400
  store i64 %117, ptr %.reg2mem36, align 8
  store i64 0, ptr %.reg2mem187, align 8
  br label %118

118:                                              ; preds = %._crit_edge26, %116
  %.reload188 = load i64, ptr %.reg2mem187, align 8
  store i64 %.reload188, ptr %.reg2mem32, align 8
  %.reload35 = load i64, ptr %.reg2mem32, align 8
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  %119 = add nuw nsw i64 %.reload35, %.reload37
  %120 = trunc nuw nsw i64 %119 to i32
  %121 = urem i32 %120, 20
  %122 = icmp eq i32 %121, 0
  br i1 %122, label %123, label %._crit_edge25

._crit_edge25:                                    ; preds = %118
  br label %126

123:                                              ; preds = %118
  %124 = load ptr, ptr @stderr, align 8, !tbaa !26
  %125 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %124, ptr noundef nonnull @.str.3) #4
  br label %126

126:                                              ; preds = %._crit_edge25, %123
  %127 = load ptr, ptr @stderr, align 8, !tbaa !26
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %.reload170 = load ptr, ptr %.reg2mem169, align 8
  %128 = getelementptr inbounds [400 x double], ptr %.reload170, i64 %.reload40, i64 %.reload34
  %129 = load double, ptr %128, align 8, !tbaa !5
  %130 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %127, ptr noundef nonnull @.str.4, double noundef %129) #4
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  %131 = add nuw nsw i64 %.reload33, 1
  store i64 %131, ptr %.reg2mem29, align 8
  %.reload31 = load i64, ptr %.reg2mem29, align 8
  %132 = icmp eq i64 %.reload31, 400
  br i1 %132, label %133, label %._crit_edge26, !llvm.loop !28

._crit_edge26:                                    ; preds = %126
  %.reload30 = load i64, ptr %.reg2mem29, align 8
  store i64 %.reload30, ptr %.reg2mem187, align 8
  br label %118

133:                                              ; preds = %126
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  %134 = add nuw nsw i64 %.reload39, 1
  store i64 %134, ptr %.reg2mem, align 8
  %.reload28 = load i64, ptr %.reg2mem, align 8
  %135 = icmp eq i64 %.reload28, 400
  br i1 %135, label %136, label %._crit_edge27, !llvm.loop !29

._crit_edge27:                                    ; preds = %133
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem189, align 8
  br label %116

136:                                              ; preds = %133
  %137 = load ptr, ptr @stderr, align 8, !tbaa !26
  %138 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %137, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %139 = load ptr, ptr @stderr, align 8, !tbaa !26
  %140 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %139, ptr noundef nonnull @.str.6) #4
  %.reload186 = load ptr, ptr %.reg2mem169, align 8
  tail call void @free(ptr noundef nonnull %.reload186) #4
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
!25 = distinct !{!25, !10, !11}
!26 = !{!27, !27, i64 0}
!27 = !{!"any pointer", !7, i64 0}
!28 = distinct !{!28, !10, !11}
!29 = distinct !{!29, !10, !11}

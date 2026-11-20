; ModuleID = './polybench-output/deriche-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/medley/deriche/deriche.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [7 x i8] c"imgOut\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"%0.2f \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem22 = alloca i64, align 8
  %.reg2mem25 = alloca i64, align 8
  %.reg2mem29 = alloca i64, align 8
  %.reg2mem31 = alloca i64, align 8
  %.reg2mem35 = alloca i64, align 8
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem41 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem49 = alloca i64, align 8
  %.reg2mem51 = alloca float, align 4
  %.reg2mem53 = alloca float, align 4
  %.reg2mem56 = alloca float, align 4
  %.reg2mem59 = alloca float, align 4
  %.reg2mem62 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem72 = alloca float, align 4
  %.reg2mem74 = alloca float, align 4
  %.reg2mem77 = alloca float, align 4
  %.reg2mem80 = alloca i64, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem87 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem95 = alloca i64, align 8
  %.reg2mem98 = alloca i64, align 8
  %.reg2mem100 = alloca float, align 4
  %.reg2mem102 = alloca float, align 4
  %.reg2mem105 = alloca float, align 4
  %.reg2mem108 = alloca float, align 4
  %.reg2mem111 = alloca i64, align 8
  %.reg2mem115 = alloca float, align 4
  %.reg2mem118 = alloca float, align 4
  %.reg2mem121 = alloca i64, align 8
  %.reg2mem124 = alloca i64, align 8
  %.reg2mem127 = alloca float, align 4
  %.reg2mem129 = alloca float, align 4
  %.reg2mem132 = alloca float, align 4
  %.reg2mem135 = alloca i64, align 8
  %.reg2mem139 = alloca float, align 4
  %.reg2mem144 = alloca float, align 4
  %.reg2mem149 = alloca float, align 4
  %.reg2mem151 = alloca float, align 4
  %.reg2mem153 = alloca float, align 4
  %.reg2mem156 = alloca float, align 4
  %.reg2mem162 = alloca i64, align 8
  %.reg2mem165 = alloca i64, align 8
  %.reg2mem168 = alloca i64, align 8
  %.reg2mem170 = alloca i64, align 8
  %.reg2mem174 = alloca ptr, align 8
  %.reg2mem180 = alloca ptr, align 8
  %.reg2mem186 = alloca ptr, align 8
  %.reg2mem193 = alloca ptr, align 8
  %.reg2mem198 = alloca i64, align 8
  %.reg2mem200 = alloca i64, align 8
  %.reg2mem202 = alloca i64, align 8
  %.reg2mem204 = alloca i64, align 8
  %.reg2mem206 = alloca float, align 4
  %.reg2mem208 = alloca float, align 4
  %.reg2mem210 = alloca float, align 4
  %.reg2mem212 = alloca float, align 4
  %.reg2mem214 = alloca i64, align 8
  %.reg2mem216 = alloca i64, align 8
  %.reg2mem218 = alloca float, align 4
  %.reg2mem220 = alloca float, align 4
  %.reg2mem222 = alloca float, align 4
  %.reg2mem224 = alloca i64, align 8
  %.reg2mem226 = alloca i64, align 8
  %.reg2mem228 = alloca i64, align 8
  %.reg2mem230 = alloca i64, align 8
  %.reg2mem232 = alloca float, align 4
  %.reg2mem234 = alloca float, align 4
  %.reg2mem236 = alloca float, align 4
  %.reg2mem238 = alloca float, align 4
  %.reg2mem240 = alloca i64, align 8
  %.reg2mem242 = alloca i64, align 8
  %.reg2mem244 = alloca float, align 4
  %.reg2mem246 = alloca float, align 4
  %.reg2mem248 = alloca float, align 4
  %.reg2mem250 = alloca i64, align 8
  %.reg2mem252 = alloca i64, align 8
  %.reg2mem254 = alloca i64, align 8
  %.reg2mem256 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  store ptr %3, ptr %.reg2mem193, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  store ptr %4, ptr %.reg2mem186, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  store ptr %5, ptr %.reg2mem180, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  store ptr %6, ptr %.reg2mem174, align 8
  store i64 0, ptr %.reg2mem256, align 8
  br label %7

7:                                                ; preds = %._crit_edge1, %2
  %.reload257 = load i64, ptr %.reg2mem256, align 8
  store i64 %.reload257, ptr %.reg2mem170, align 8
  %.reload173 = load i64, ptr %.reg2mem170, align 8
  %8 = mul nuw nsw i64 %.reload173, 313
  store i64 %8, ptr %.reg2mem168, align 8
  store i64 0, ptr %.reg2mem254, align 8
  br label %9

9:                                                ; preds = %._crit_edge, %7
  %.reload255 = load i64, ptr %.reg2mem254, align 8
  %10 = mul nuw nsw i64 %.reload255, 991
  %.reload169 = load i64, ptr %.reg2mem168, align 8
  %11 = add nuw nsw i64 %10, %.reload169
  %12 = trunc nuw nsw i64 %11 to i32
  %13 = and i32 %12, 65535
  %14 = uitofp nneg i32 %13 to float
  %15 = fdiv float %14, 6.553500e+04
  %.reload172 = load i64, ptr %.reg2mem170, align 8
  %.reload196 = load ptr, ptr %.reg2mem193, align 8
  %16 = getelementptr inbounds [480 x float], ptr %.reload196, i64 %.reload172, i64 %.reload255
  store float %15, ptr %16, align 4, !tbaa !5
  %17 = add nuw nsw i64 %.reload255, 1
  store i64 %17, ptr %.reg2mem165, align 8
  %.reload167 = load i64, ptr %.reg2mem165, align 8
  %18 = icmp eq i64 %.reload167, 480
  br i1 %18, label %19, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %9
  %.reload166 = load i64, ptr %.reg2mem165, align 8
  store i64 %.reload166, ptr %.reg2mem254, align 8
  br label %9

19:                                               ; preds = %9
  %.reload171 = load i64, ptr %.reg2mem170, align 8
  %20 = add nuw nsw i64 %.reload171, 1
  store i64 %20, ptr %.reg2mem162, align 8
  %.reload164 = load i64, ptr %.reg2mem162, align 8
  %21 = icmp eq i64 %.reload164, 720
  br i1 %21, label %22, label %._crit_edge1, !llvm.loop !12

._crit_edge1:                                     ; preds = %19
  %.reload163 = load i64, ptr %.reg2mem162, align 8
  store i64 %.reload163, ptr %.reg2mem256, align 8
  br label %7

22:                                               ; preds = %19
  %23 = tail call float @expf(float noundef -2.500000e-01) #4
  %24 = fsub float 1.000000e+00, %23
  %25 = tail call float @expf(float noundef -2.500000e-01) #4
  %26 = fsub float 1.000000e+00, %25
  %27 = fmul float %24, %26
  %28 = tail call float @expf(float noundef -2.500000e-01) #4
  %29 = tail call float @llvm.fmuladd.f32(float %28, float 5.000000e-01, float 1.000000e+00)
  %30 = tail call float @expf(float noundef 5.000000e-01) #4
  %31 = fsub float %29, %30
  %32 = fdiv float %27, %31
  store float %32, ptr %.reg2mem156, align 4
  %33 = tail call float @expf(float noundef -2.500000e-01) #4
  %.reload161 = load float, ptr %.reg2mem156, align 4
  %34 = fmul float %33, %.reload161
  %35 = fmul float %34, -7.500000e-01
  store float %35, ptr %.reg2mem153, align 4
  %36 = tail call float @expf(float noundef -2.500000e-01) #4
  %.reload160 = load float, ptr %.reg2mem156, align 4
  %37 = fmul float %.reload160, %36
  store float %37, ptr %.reg2mem151, align 4
  %38 = tail call float @expf(float noundef -5.000000e-01) #4
  store float %38, ptr %.reg2mem149, align 4
  %39 = tail call float @powf(float noundef 2.000000e+00, float noundef -2.500000e-01) #4
  store float %39, ptr %.reg2mem144, align 4
  %40 = tail call float @expf(float noundef -5.000000e-01) #4
  %41 = fneg float %40
  store float %41, ptr %.reg2mem139, align 4
  store i64 0, ptr %.reg2mem252, align 8
  br label %42

42:                                               ; preds = %._crit_edge3, %22
  %.reload253 = load i64, ptr %.reg2mem252, align 8
  store i64 %.reload253, ptr %.reg2mem135, align 8
  store float 0.000000e+00, ptr %.reg2mem244, align 4
  store float 0.000000e+00, ptr %.reg2mem246, align 4
  store float 0.000000e+00, ptr %.reg2mem248, align 4
  store i64 0, ptr %.reg2mem250, align 8
  br label %43

43:                                               ; preds = %._crit_edge2, %42
  %.reload251 = load i64, ptr %.reg2mem250, align 8
  %.reload249 = load float, ptr %.reg2mem248, align 4
  %.reload247 = load float, ptr %.reg2mem246, align 4
  %.reload245 = load float, ptr %.reg2mem244, align 4
  store float %.reload247, ptr %.reg2mem132, align 4
  %.reload138 = load i64, ptr %.reg2mem135, align 8
  %.reload195 = load ptr, ptr %.reg2mem193, align 8
  %44 = getelementptr inbounds [480 x float], ptr %.reload195, i64 %.reload138, i64 %.reload251
  %45 = load float, ptr %44, align 4, !tbaa !5
  %.reload155 = load float, ptr %.reg2mem153, align 4
  %46 = fmul float %.reload155, %.reload249
  %.reload159 = load float, ptr %.reg2mem156, align 4
  %47 = tail call float @llvm.fmuladd.f32(float %.reload159, float %45, float %46)
  %.reload134 = load float, ptr %.reg2mem132, align 4
  %.reload148 = load float, ptr %.reg2mem144, align 4
  %48 = tail call float @llvm.fmuladd.f32(float %.reload148, float %.reload134, float %47)
  %.reload143 = load float, ptr %.reg2mem139, align 4
  %49 = tail call float @llvm.fmuladd.f32(float %.reload143, float %.reload245, float %48)
  store float %49, ptr %.reg2mem129, align 4
  %.reload137 = load i64, ptr %.reg2mem135, align 8
  %.reload184 = load ptr, ptr %.reg2mem180, align 8
  %50 = getelementptr inbounds [480 x float], ptr %.reload184, i64 %.reload137, i64 %.reload251
  %.reload131 = load float, ptr %.reg2mem129, align 4
  store float %.reload131, ptr %50, align 4, !tbaa !5
  %51 = load float, ptr %44, align 4, !tbaa !5
  store float %51, ptr %.reg2mem127, align 4
  %52 = add nuw nsw i64 %.reload251, 1
  store i64 %52, ptr %.reg2mem124, align 8
  %.reload126 = load i64, ptr %.reg2mem124, align 8
  %53 = icmp eq i64 %.reload126, 480
  br i1 %53, label %54, label %._crit_edge2, !llvm.loop !13

._crit_edge2:                                     ; preds = %43
  %.reload125 = load i64, ptr %.reg2mem124, align 8
  %.reload128 = load float, ptr %.reg2mem127, align 4
  %.reload130 = load float, ptr %.reg2mem129, align 4
  %.reload133 = load float, ptr %.reg2mem132, align 4
  store float %.reload133, ptr %.reg2mem244, align 4
  store float %.reload130, ptr %.reg2mem246, align 4
  store float %.reload128, ptr %.reg2mem248, align 4
  store i64 %.reload125, ptr %.reg2mem250, align 8
  br label %43

54:                                               ; preds = %43
  %.reload136 = load i64, ptr %.reg2mem135, align 8
  %55 = add nuw nsw i64 %.reload136, 1
  store i64 %55, ptr %.reg2mem121, align 8
  %.reload123 = load i64, ptr %.reg2mem121, align 8
  %56 = icmp eq i64 %.reload123, 720
  br i1 %56, label %57, label %._crit_edge3, !llvm.loop !14

._crit_edge3:                                     ; preds = %54
  %.reload122 = load i64, ptr %.reg2mem121, align 8
  store i64 %.reload122, ptr %.reg2mem252, align 8
  br label %42

57:                                               ; preds = %54
  %.reload152 = load float, ptr %.reg2mem151, align 4
  %58 = fmul float %.reload152, 1.250000e+00
  store float %58, ptr %.reg2mem118, align 4
  %.reload158 = load float, ptr %.reg2mem156, align 4
  %59 = fneg float %.reload158
  %.reload150 = load float, ptr %.reg2mem149, align 4
  %60 = fmul float %.reload150, %59
  store float %60, ptr %.reg2mem115, align 4
  store i64 0, ptr %.reg2mem242, align 8
  br label %61

61:                                               ; preds = %._crit_edge6, %57
  %.reload243 = load i64, ptr %.reg2mem242, align 8
  store i64 %.reload243, ptr %.reg2mem111, align 8
  store float 0.000000e+00, ptr %.reg2mem232, align 4
  store float 0.000000e+00, ptr %.reg2mem234, align 4
  store float 0.000000e+00, ptr %.reg2mem236, align 4
  store float 0.000000e+00, ptr %.reg2mem238, align 4
  store i64 479, ptr %.reg2mem240, align 8
  br label %62

62:                                               ; preds = %._crit_edge4, %61
  %.reload241 = load i64, ptr %.reg2mem240, align 8
  %.reload239 = load float, ptr %.reg2mem238, align 4
  %.reload237 = load float, ptr %.reg2mem236, align 4
  %.reload235 = load float, ptr %.reg2mem234, align 4
  %.reload233 = load float, ptr %.reg2mem232, align 4
  store float %.reload239, ptr %.reg2mem108, align 4
  store float %.reload235, ptr %.reg2mem105, align 4
  %.reload117 = load float, ptr %.reg2mem115, align 4
  %63 = fmul float %.reload117, %.reload237
  %.reload110 = load float, ptr %.reg2mem108, align 4
  %.reload120 = load float, ptr %.reg2mem118, align 4
  %64 = tail call float @llvm.fmuladd.f32(float %.reload120, float %.reload110, float %63)
  %.reload107 = load float, ptr %.reg2mem105, align 4
  %.reload147 = load float, ptr %.reg2mem144, align 4
  %65 = tail call float @llvm.fmuladd.f32(float %.reload147, float %.reload107, float %64)
  %.reload142 = load float, ptr %.reg2mem139, align 4
  %66 = tail call float @llvm.fmuladd.f32(float %.reload142, float %.reload233, float %65)
  store float %66, ptr %.reg2mem102, align 4
  %.reload114 = load i64, ptr %.reg2mem111, align 8
  %.reload178 = load ptr, ptr %.reg2mem174, align 8
  %67 = getelementptr inbounds [480 x float], ptr %.reload178, i64 %.reload114, i64 %.reload241
  %.reload104 = load float, ptr %.reg2mem102, align 4
  store float %.reload104, ptr %67, align 4, !tbaa !5
  %.reload113 = load i64, ptr %.reg2mem111, align 8
  %.reload194 = load ptr, ptr %.reg2mem193, align 8
  %68 = getelementptr inbounds [480 x float], ptr %.reload194, i64 %.reload113, i64 %.reload241
  %69 = load float, ptr %68, align 4, !tbaa !5
  store float %69, ptr %.reg2mem100, align 4
  %70 = add nsw i64 %.reload241, -1
  store i64 %70, ptr %.reg2mem98, align 8
  %71 = icmp eq i64 %.reload241, 0
  br i1 %71, label %72, label %._crit_edge4, !llvm.loop !15

._crit_edge4:                                     ; preds = %62
  %.reload99 = load i64, ptr %.reg2mem98, align 8
  %.reload101 = load float, ptr %.reg2mem100, align 4
  %.reload103 = load float, ptr %.reg2mem102, align 4
  %.reload106 = load float, ptr %.reg2mem105, align 4
  %.reload109 = load float, ptr %.reg2mem108, align 4
  store float %.reload106, ptr %.reg2mem232, align 4
  store float %.reload103, ptr %.reg2mem234, align 4
  store float %.reload109, ptr %.reg2mem236, align 4
  store float %.reload101, ptr %.reg2mem238, align 4
  store i64 %.reload99, ptr %.reg2mem240, align 8
  br label %62

72:                                               ; preds = %62
  %.reload112 = load i64, ptr %.reg2mem111, align 8
  %73 = add nuw nsw i64 %.reload112, 1
  store i64 %73, ptr %.reg2mem95, align 8
  %.reload97 = load i64, ptr %.reg2mem95, align 8
  %74 = icmp eq i64 %.reload97, 720
  br i1 %74, label %._crit_edge5, label %._crit_edge6, !llvm.loop !16

._crit_edge6:                                     ; preds = %72
  %.reload96 = load i64, ptr %.reg2mem95, align 8
  store i64 %.reload96, ptr %.reg2mem242, align 8
  br label %61

._crit_edge5:                                     ; preds = %72
  store i64 0, ptr %.reg2mem230, align 8
  br label %75

75:                                               ; preds = %._crit_edge9, %._crit_edge5
  %.reload231 = load i64, ptr %.reg2mem230, align 8
  store i64 %.reload231, ptr %.reg2mem90, align 8
  store i64 0, ptr %.reg2mem228, align 8
  br label %76

76:                                               ; preds = %._crit_edge7, %75
  %.reload229 = load i64, ptr %.reg2mem228, align 8
  %.reload94 = load i64, ptr %.reg2mem90, align 8
  %.reload183 = load ptr, ptr %.reg2mem180, align 8
  %77 = getelementptr inbounds [480 x float], ptr %.reload183, i64 %.reload94, i64 %.reload229
  %78 = load float, ptr %77, align 4, !tbaa !5
  %.reload93 = load i64, ptr %.reg2mem90, align 8
  %.reload177 = load ptr, ptr %.reg2mem174, align 8
  %79 = getelementptr inbounds [480 x float], ptr %.reload177, i64 %.reload93, i64 %.reload229
  %80 = load float, ptr %79, align 4, !tbaa !5
  %81 = fadd float %78, %80
  %.reload92 = load i64, ptr %.reg2mem90, align 8
  %.reload191 = load ptr, ptr %.reg2mem186, align 8
  %82 = getelementptr inbounds [480 x float], ptr %.reload191, i64 %.reload92, i64 %.reload229
  store float %81, ptr %82, align 4, !tbaa !5
  %83 = add nuw nsw i64 %.reload229, 1
  store i64 %83, ptr %.reg2mem87, align 8
  %.reload89 = load i64, ptr %.reg2mem87, align 8
  %84 = icmp eq i64 %.reload89, 480
  br i1 %84, label %85, label %._crit_edge7, !llvm.loop !17

._crit_edge7:                                     ; preds = %76
  %.reload88 = load i64, ptr %.reg2mem87, align 8
  store i64 %.reload88, ptr %.reg2mem228, align 8
  br label %76

85:                                               ; preds = %76
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %86 = add nuw nsw i64 %.reload91, 1
  store i64 %86, ptr %.reg2mem84, align 8
  %.reload86 = load i64, ptr %.reg2mem84, align 8
  %87 = icmp eq i64 %.reload86, 720
  br i1 %87, label %._crit_edge8, label %._crit_edge9, !llvm.loop !18

._crit_edge9:                                     ; preds = %85
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  store i64 %.reload85, ptr %.reg2mem230, align 8
  br label %75

._crit_edge8:                                     ; preds = %85
  store i64 0, ptr %.reg2mem226, align 8
  br label %88

88:                                               ; preds = %._crit_edge12, %._crit_edge8
  %.reload227 = load i64, ptr %.reg2mem226, align 8
  store i64 %.reload227, ptr %.reg2mem80, align 8
  store float 0.000000e+00, ptr %.reg2mem218, align 4
  store float 0.000000e+00, ptr %.reg2mem220, align 4
  store float 0.000000e+00, ptr %.reg2mem222, align 4
  store i64 0, ptr %.reg2mem224, align 8
  br label %89

89:                                               ; preds = %._crit_edge10, %88
  %.reload225 = load i64, ptr %.reg2mem224, align 8
  %.reload223 = load float, ptr %.reg2mem222, align 4
  %.reload221 = load float, ptr %.reg2mem220, align 4
  %.reload219 = load float, ptr %.reg2mem218, align 4
  store float %.reload221, ptr %.reg2mem77, align 4
  %.reload83 = load i64, ptr %.reg2mem80, align 8
  %.reload190 = load ptr, ptr %.reg2mem186, align 8
  %90 = getelementptr inbounds [480 x float], ptr %.reload190, i64 %.reload225, i64 %.reload83
  %91 = load float, ptr %90, align 4, !tbaa !5
  %.reload154 = load float, ptr %.reg2mem153, align 4
  %92 = fmul float %.reload154, %.reload223
  %.reload157 = load float, ptr %.reg2mem156, align 4
  %93 = tail call float @llvm.fmuladd.f32(float %.reload157, float %91, float %92)
  %.reload79 = load float, ptr %.reg2mem77, align 4
  %.reload146 = load float, ptr %.reg2mem144, align 4
  %94 = tail call float @llvm.fmuladd.f32(float %.reload146, float %.reload79, float %93)
  %.reload141 = load float, ptr %.reg2mem139, align 4
  %95 = tail call float @llvm.fmuladd.f32(float %.reload141, float %.reload219, float %94)
  store float %95, ptr %.reg2mem74, align 4
  %.reload82 = load i64, ptr %.reg2mem80, align 8
  %.reload182 = load ptr, ptr %.reg2mem180, align 8
  %96 = getelementptr inbounds [480 x float], ptr %.reload182, i64 %.reload225, i64 %.reload82
  %.reload76 = load float, ptr %.reg2mem74, align 4
  store float %.reload76, ptr %96, align 4, !tbaa !5
  %97 = load float, ptr %90, align 4, !tbaa !5
  store float %97, ptr %.reg2mem72, align 4
  %98 = add nuw nsw i64 %.reload225, 1
  store i64 %98, ptr %.reg2mem69, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %99 = icmp eq i64 %.reload71, 720
  br i1 %99, label %100, label %._crit_edge10, !llvm.loop !19

._crit_edge10:                                    ; preds = %89
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %.reload73 = load float, ptr %.reg2mem72, align 4
  %.reload75 = load float, ptr %.reg2mem74, align 4
  %.reload78 = load float, ptr %.reg2mem77, align 4
  store float %.reload78, ptr %.reg2mem218, align 4
  store float %.reload75, ptr %.reg2mem220, align 4
  store float %.reload73, ptr %.reg2mem222, align 4
  store i64 %.reload70, ptr %.reg2mem224, align 8
  br label %89

100:                                              ; preds = %89
  %.reload81 = load i64, ptr %.reg2mem80, align 8
  %101 = add nuw nsw i64 %.reload81, 1
  store i64 %101, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %102 = icmp eq i64 %.reload68, 480
  br i1 %102, label %._crit_edge11, label %._crit_edge12, !llvm.loop !20

._crit_edge12:                                    ; preds = %100
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem226, align 8
  br label %88

._crit_edge11:                                    ; preds = %100
  store i64 0, ptr %.reg2mem216, align 8
  br label %103

103:                                              ; preds = %._crit_edge15, %._crit_edge11
  %.reload217 = load i64, ptr %.reg2mem216, align 8
  store i64 %.reload217, ptr %.reg2mem62, align 8
  store float 0.000000e+00, ptr %.reg2mem206, align 4
  store float 0.000000e+00, ptr %.reg2mem208, align 4
  store float 0.000000e+00, ptr %.reg2mem210, align 4
  store float 0.000000e+00, ptr %.reg2mem212, align 4
  store i64 719, ptr %.reg2mem214, align 8
  br label %104

104:                                              ; preds = %._crit_edge13, %103
  %.reload215 = load i64, ptr %.reg2mem214, align 8
  %.reload213 = load float, ptr %.reg2mem212, align 4
  %.reload211 = load float, ptr %.reg2mem210, align 4
  %.reload209 = load float, ptr %.reg2mem208, align 4
  %.reload207 = load float, ptr %.reg2mem206, align 4
  store float %.reload213, ptr %.reg2mem59, align 4
  store float %.reload209, ptr %.reg2mem56, align 4
  %.reload116 = load float, ptr %.reg2mem115, align 4
  %105 = fmul float %.reload116, %.reload211
  %.reload61 = load float, ptr %.reg2mem59, align 4
  %.reload119 = load float, ptr %.reg2mem118, align 4
  %106 = tail call float @llvm.fmuladd.f32(float %.reload119, float %.reload61, float %105)
  %.reload58 = load float, ptr %.reg2mem56, align 4
  %.reload145 = load float, ptr %.reg2mem144, align 4
  %107 = tail call float @llvm.fmuladd.f32(float %.reload145, float %.reload58, float %106)
  %.reload140 = load float, ptr %.reg2mem139, align 4
  %108 = tail call float @llvm.fmuladd.f32(float %.reload140, float %.reload207, float %107)
  store float %108, ptr %.reg2mem53, align 4
  %.reload65 = load i64, ptr %.reg2mem62, align 8
  %.reload176 = load ptr, ptr %.reg2mem174, align 8
  %109 = getelementptr inbounds [480 x float], ptr %.reload176, i64 %.reload215, i64 %.reload65
  %.reload55 = load float, ptr %.reg2mem53, align 4
  store float %.reload55, ptr %109, align 4, !tbaa !5
  %.reload64 = load i64, ptr %.reg2mem62, align 8
  %.reload189 = load ptr, ptr %.reg2mem186, align 8
  %110 = getelementptr inbounds [480 x float], ptr %.reload189, i64 %.reload215, i64 %.reload64
  %111 = load float, ptr %110, align 4, !tbaa !5
  store float %111, ptr %.reg2mem51, align 4
  %112 = add nsw i64 %.reload215, -1
  store i64 %112, ptr %.reg2mem49, align 8
  %113 = icmp eq i64 %.reload215, 0
  br i1 %113, label %114, label %._crit_edge13, !llvm.loop !21

._crit_edge13:                                    ; preds = %104
  %.reload50 = load i64, ptr %.reg2mem49, align 8
  %.reload52 = load float, ptr %.reg2mem51, align 4
  %.reload54 = load float, ptr %.reg2mem53, align 4
  %.reload57 = load float, ptr %.reg2mem56, align 4
  %.reload60 = load float, ptr %.reg2mem59, align 4
  store float %.reload57, ptr %.reg2mem206, align 4
  store float %.reload54, ptr %.reg2mem208, align 4
  store float %.reload60, ptr %.reg2mem210, align 4
  store float %.reload52, ptr %.reg2mem212, align 4
  store i64 %.reload50, ptr %.reg2mem214, align 8
  br label %104

114:                                              ; preds = %104
  %.reload63 = load i64, ptr %.reg2mem62, align 8
  %115 = add nuw nsw i64 %.reload63, 1
  store i64 %115, ptr %.reg2mem46, align 8
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %116 = icmp eq i64 %.reload48, 480
  br i1 %116, label %._crit_edge14, label %._crit_edge15, !llvm.loop !22

._crit_edge15:                                    ; preds = %114
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  store i64 %.reload47, ptr %.reg2mem216, align 8
  br label %103

._crit_edge14:                                    ; preds = %114
  store i64 0, ptr %.reg2mem204, align 8
  br label %117

117:                                              ; preds = %._crit_edge17, %._crit_edge14
  %.reload205 = load i64, ptr %.reg2mem204, align 8
  store i64 %.reload205, ptr %.reg2mem41, align 8
  store i64 0, ptr %.reg2mem202, align 8
  br label %118

118:                                              ; preds = %._crit_edge16, %117
  %.reload203 = load i64, ptr %.reg2mem202, align 8
  %.reload45 = load i64, ptr %.reg2mem41, align 8
  %.reload181 = load ptr, ptr %.reg2mem180, align 8
  %119 = getelementptr inbounds [480 x float], ptr %.reload181, i64 %.reload45, i64 %.reload203
  %120 = load float, ptr %119, align 4, !tbaa !5
  %.reload44 = load i64, ptr %.reg2mem41, align 8
  %.reload175 = load ptr, ptr %.reg2mem174, align 8
  %121 = getelementptr inbounds [480 x float], ptr %.reload175, i64 %.reload44, i64 %.reload203
  %122 = load float, ptr %121, align 4, !tbaa !5
  %123 = fadd float %120, %122
  %.reload43 = load i64, ptr %.reg2mem41, align 8
  %.reload188 = load ptr, ptr %.reg2mem186, align 8
  %124 = getelementptr inbounds [480 x float], ptr %.reload188, i64 %.reload43, i64 %.reload203
  store float %123, ptr %124, align 4, !tbaa !5
  %125 = add nuw nsw i64 %.reload203, 1
  store i64 %125, ptr %.reg2mem38, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %126 = icmp eq i64 %.reload40, 480
  br i1 %126, label %127, label %._crit_edge16, !llvm.loop !23

._crit_edge16:                                    ; preds = %118
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  store i64 %.reload39, ptr %.reg2mem202, align 8
  br label %118

127:                                              ; preds = %118
  %.reload42 = load i64, ptr %.reg2mem41, align 8
  %128 = add nuw nsw i64 %.reload42, 1
  store i64 %128, ptr %.reg2mem35, align 8
  %.reload37 = load i64, ptr %.reg2mem35, align 8
  %129 = icmp eq i64 %.reload37, 720
  br i1 %129, label %130, label %._crit_edge17, !llvm.loop !24

._crit_edge17:                                    ; preds = %127
  %.reload36 = load i64, ptr %.reg2mem35, align 8
  store i64 %.reload36, ptr %.reg2mem204, align 8
  br label %117

130:                                              ; preds = %127
  %131 = load ptr, ptr @stderr, align 8, !tbaa !25
  %132 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %131, ptr noundef nonnull @.str) #4
  %133 = load ptr, ptr @stderr, align 8, !tbaa !25
  %134 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %133, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem200, align 8
  br label %135

135:                                              ; preds = %._crit_edge20, %130
  %.reload201 = load i64, ptr %.reg2mem200, align 8
  store i64 %.reload201, ptr %.reg2mem31, align 8
  %.reload34 = load i64, ptr %.reg2mem31, align 8
  %136 = mul nuw nsw i64 %.reload34, 480
  store i64 %136, ptr %.reg2mem29, align 8
  store i64 0, ptr %.reg2mem198, align 8
  br label %137

137:                                              ; preds = %._crit_edge19, %135
  %.reload199 = load i64, ptr %.reg2mem198, align 8
  store i64 %.reload199, ptr %.reg2mem25, align 8
  %.reload28 = load i64, ptr %.reg2mem25, align 8
  %.reload30 = load i64, ptr %.reg2mem29, align 8
  %138 = add nuw nsw i64 %.reload28, %.reload30
  %139 = trunc nuw nsw i64 %138 to i32
  %140 = urem i32 %139, 20
  %141 = icmp eq i32 %140, 0
  br i1 %141, label %142, label %._crit_edge18

._crit_edge18:                                    ; preds = %137
  br label %145

142:                                              ; preds = %137
  %143 = load ptr, ptr @stderr, align 8, !tbaa !25
  %144 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %143, ptr noundef nonnull @.str.3) #4
  br label %145

145:                                              ; preds = %._crit_edge18, %142
  %146 = load ptr, ptr @stderr, align 8, !tbaa !25
  %.reload27 = load i64, ptr %.reg2mem25, align 8
  %.reload33 = load i64, ptr %.reg2mem31, align 8
  %.reload187 = load ptr, ptr %.reg2mem186, align 8
  %147 = getelementptr inbounds [480 x float], ptr %.reload187, i64 %.reload33, i64 %.reload27
  %148 = load float, ptr %147, align 4, !tbaa !5
  %149 = fpext float %148 to double
  %150 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %146, ptr noundef nonnull @.str.4, double noundef %149) #4
  %.reload26 = load i64, ptr %.reg2mem25, align 8
  %151 = add nuw nsw i64 %.reload26, 1
  store i64 %151, ptr %.reg2mem22, align 8
  %.reload24 = load i64, ptr %.reg2mem22, align 8
  %152 = icmp eq i64 %.reload24, 480
  br i1 %152, label %153, label %._crit_edge19, !llvm.loop !27

._crit_edge19:                                    ; preds = %145
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  store i64 %.reload23, ptr %.reg2mem198, align 8
  br label %137

153:                                              ; preds = %145
  %.reload32 = load i64, ptr %.reg2mem31, align 8
  %154 = add nuw nsw i64 %.reload32, 1
  store i64 %154, ptr %.reg2mem, align 8
  %.reload21 = load i64, ptr %.reg2mem, align 8
  %155 = icmp eq i64 %.reload21, 720
  br i1 %155, label %156, label %._crit_edge20, !llvm.loop !28

._crit_edge20:                                    ; preds = %153
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem200, align 8
  br label %135

156:                                              ; preds = %153
  %157 = load ptr, ptr @stderr, align 8, !tbaa !25
  %158 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %157, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %159 = load ptr, ptr @stderr, align 8, !tbaa !25
  %160 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %159, ptr noundef nonnull @.str.6) #4
  %.reload197 = load ptr, ptr %.reg2mem193, align 8
  tail call void @free(ptr noundef %.reload197) #4
  %.reload192 = load ptr, ptr %.reg2mem186, align 8
  tail call void @free(ptr noundef nonnull %.reload192) #4
  %.reload185 = load ptr, ptr %.reg2mem180, align 8
  tail call void @free(ptr noundef %.reload185) #4
  %.reload179 = load ptr, ptr %.reg2mem174, align 8
  tail call void @free(ptr noundef %.reload179) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: nounwind
declare float @expf(float noundef) local_unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fmuladd.f32(float, float, float) #3

; Function Attrs: nounwind
declare float @powf(float noundef, float noundef) local_unnamed_addr #2

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
!6 = !{!"float", !7, i64 0}
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

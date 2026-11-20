; ModuleID = './polybench-output/3mm-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/3mm/3mm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"G\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem27 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem40 = alloca i64, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem49 = alloca double, align 8
  %.reg2mem52 = alloca ptr, align 8
  %.reg2mem55 = alloca i64, align 8
  %.reg2mem59 = alloca i64, align 8
  %.reg2mem63 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem72 = alloca double, align 8
  %.reg2mem75 = alloca ptr, align 8
  %.reg2mem78 = alloca i64, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem89 = alloca i64, align 8
  %.reg2mem92 = alloca i64, align 8
  %.reg2mem95 = alloca double, align 8
  %.reg2mem98 = alloca ptr, align 8
  %.reg2mem101 = alloca i64, align 8
  %.reg2mem105 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem112 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem119 = alloca i64, align 8
  %.reg2mem122 = alloca i64, align 8
  %.reg2mem125 = alloca i64, align 8
  %.reg2mem129 = alloca i64, align 8
  %.reg2mem132 = alloca i64, align 8
  %.reg2mem136 = alloca i64, align 8
  %.reg2mem140 = alloca i64, align 8
  %.reg2mem143 = alloca i64, align 8
  %.reg2mem146 = alloca i64, align 8
  %.reg2mem150 = alloca ptr, align 8
  %.reg2mem154 = alloca ptr, align 8
  %.reg2mem158 = alloca ptr, align 8
  %.reg2mem162 = alloca ptr, align 8
  %.reg2mem166 = alloca ptr, align 8
  %.reg2mem170 = alloca ptr, align 8
  %.reg2mem174 = alloca ptr, align 8
  %.reg2mem178 = alloca i64, align 8
  %.reg2mem180 = alloca i64, align 8
  %.reg2mem182 = alloca double, align 8
  %.reg2mem184 = alloca i64, align 8
  %.reg2mem186 = alloca i64, align 8
  %.reg2mem188 = alloca i64, align 8
  %.reg2mem190 = alloca double, align 8
  %.reg2mem192 = alloca i64, align 8
  %.reg2mem194 = alloca i64, align 8
  %.reg2mem196 = alloca i64, align 8
  %.reg2mem198 = alloca double, align 8
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
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 34200, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem174, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 36000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem170, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 38000, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem166, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 39900, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem162, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 41800, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem158, align 8
  %8 = tail call ptr @polybench_alloc_data(i64 noundef 46200, i32 noundef 8) #4
  store ptr %8, ptr %.reg2mem154, align 8
  %9 = tail call ptr @polybench_alloc_data(i64 noundef 37800, i32 noundef 8) #4
  store ptr %9, ptr %.reg2mem150, align 8
  store i64 0, ptr %.reg2mem220, align 8
  br label %10

10:                                               ; preds = %._crit_edge2, %2
  %.reload221 = load i64, ptr %.reg2mem220, align 8
  store i64 %.reload221, ptr %.reg2mem146, align 8
  store i64 0, ptr %.reg2mem218, align 8
  br label %11

11:                                               ; preds = %._crit_edge, %10
  %.reload219 = load i64, ptr %.reg2mem218, align 8
  %.reload149 = load i64, ptr %.reg2mem146, align 8
  %12 = mul nuw nsw i64 %.reload219, %.reload149
  %13 = trunc i64 %12 to i32
  %14 = add i32 %13, 1
  %15 = urem i32 %14, 180
  %16 = uitofp nneg i32 %15 to double
  %17 = fdiv double %16, 9.000000e+02
  %.reload148 = load i64, ptr %.reg2mem146, align 8
  %.reload172 = load ptr, ptr %.reg2mem170, align 8
  %18 = getelementptr inbounds [200 x double], ptr %.reload172, i64 %.reload148, i64 %.reload219
  store double %17, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %.reload219, 1
  store i64 %19, ptr %.reg2mem143, align 8
  %.reload145 = load i64, ptr %.reg2mem143, align 8
  %20 = icmp eq i64 %.reload145, 200
  br i1 %20, label %21, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %11
  %.reload144 = load i64, ptr %.reg2mem143, align 8
  store i64 %.reload144, ptr %.reg2mem218, align 8
  br label %11

21:                                               ; preds = %11
  %.reload147 = load i64, ptr %.reg2mem146, align 8
  %22 = add nuw nsw i64 %.reload147, 1
  store i64 %22, ptr %.reg2mem140, align 8
  %.reload142 = load i64, ptr %.reg2mem140, align 8
  %23 = icmp eq i64 %.reload142, 180
  br i1 %23, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %21
  %.reload141 = load i64, ptr %.reg2mem140, align 8
  store i64 %.reload141, ptr %.reg2mem220, align 8
  br label %10

._crit_edge1:                                     ; preds = %21
  store i64 0, ptr %.reg2mem216, align 8
  br label %24

24:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload217 = load i64, ptr %.reg2mem216, align 8
  store i64 %.reload217, ptr %.reg2mem136, align 8
  store i64 0, ptr %.reg2mem214, align 8
  br label %25

25:                                               ; preds = %._crit_edge3, %24
  %.reload215 = load i64, ptr %.reg2mem214, align 8
  %26 = add nuw nsw i64 %.reload215, 1
  store i64 %26, ptr %.reg2mem132, align 8
  %.reload135 = load i64, ptr %.reg2mem132, align 8
  %.reload139 = load i64, ptr %.reg2mem136, align 8
  %27 = mul nuw nsw i64 %.reload135, %.reload139
  %28 = trunc i64 %27 to i32
  %29 = add i32 %28, 2
  %30 = urem i32 %29, 190
  %31 = uitofp nneg i32 %30 to double
  %32 = fdiv double %31, 9.500000e+02
  %.reload138 = load i64, ptr %.reg2mem136, align 8
  %.reload168 = load ptr, ptr %.reg2mem166, align 8
  %33 = getelementptr inbounds [190 x double], ptr %.reload168, i64 %.reload138, i64 %.reload215
  store double %32, ptr %33, align 8, !tbaa !5
  %.reload134 = load i64, ptr %.reg2mem132, align 8
  %34 = icmp eq i64 %.reload134, 190
  br i1 %34, label %35, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %25
  %.reload133 = load i64, ptr %.reg2mem132, align 8
  store i64 %.reload133, ptr %.reg2mem214, align 8
  br label %25

35:                                               ; preds = %25
  %.reload137 = load i64, ptr %.reg2mem136, align 8
  %36 = add nuw nsw i64 %.reload137, 1
  store i64 %36, ptr %.reg2mem129, align 8
  %.reload131 = load i64, ptr %.reg2mem129, align 8
  %37 = icmp eq i64 %.reload131, 200
  br i1 %37, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %35
  %.reload130 = load i64, ptr %.reg2mem129, align 8
  store i64 %.reload130, ptr %.reg2mem216, align 8
  br label %24

._crit_edge4:                                     ; preds = %35
  store i64 0, ptr %.reg2mem212, align 8
  br label %38

38:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload213 = load i64, ptr %.reg2mem212, align 8
  store i64 %.reload213, ptr %.reg2mem125, align 8
  store i64 0, ptr %.reg2mem210, align 8
  br label %39

39:                                               ; preds = %._crit_edge6, %38
  %.reload211 = load i64, ptr %.reg2mem210, align 8
  %40 = add nuw nsw i64 %.reload211, 3
  %.reload128 = load i64, ptr %.reg2mem125, align 8
  %41 = mul nuw nsw i64 %40, %.reload128
  %42 = trunc nuw nsw i64 %41 to i32
  %43 = urem i32 %42, 210
  %44 = uitofp nneg i32 %43 to double
  %45 = fdiv double %44, 1.050000e+03
  %.reload127 = load i64, ptr %.reg2mem125, align 8
  %.reload160 = load ptr, ptr %.reg2mem158, align 8
  %46 = getelementptr inbounds [220 x double], ptr %.reload160, i64 %.reload127, i64 %.reload211
  store double %45, ptr %46, align 8, !tbaa !5
  %47 = add nuw nsw i64 %.reload211, 1
  store i64 %47, ptr %.reg2mem122, align 8
  %.reload124 = load i64, ptr %.reg2mem122, align 8
  %48 = icmp eq i64 %.reload124, 220
  br i1 %48, label %49, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %39
  %.reload123 = load i64, ptr %.reg2mem122, align 8
  store i64 %.reload123, ptr %.reg2mem210, align 8
  br label %39

49:                                               ; preds = %39
  %.reload126 = load i64, ptr %.reg2mem125, align 8
  %50 = add nuw nsw i64 %.reload126, 1
  store i64 %50, ptr %.reg2mem119, align 8
  %.reload121 = load i64, ptr %.reg2mem119, align 8
  %51 = icmp eq i64 %.reload121, 190
  br i1 %51, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %49
  %.reload120 = load i64, ptr %.reg2mem119, align 8
  store i64 %.reload120, ptr %.reg2mem212, align 8
  br label %38

._crit_edge7:                                     ; preds = %49
  store i64 0, ptr %.reg2mem208, align 8
  br label %52

52:                                               ; preds = %._crit_edge11, %._crit_edge7
  %.reload209 = load i64, ptr %.reg2mem208, align 8
  store i64 %.reload209, ptr %.reg2mem115, align 8
  store i64 0, ptr %.reg2mem206, align 8
  br label %53

53:                                               ; preds = %._crit_edge9, %52
  %.reload207 = load i64, ptr %.reg2mem206, align 8
  %54 = add nuw nsw i64 %.reload207, 2
  %.reload118 = load i64, ptr %.reg2mem115, align 8
  %55 = mul nuw nsw i64 %54, %.reload118
  %56 = trunc i64 %55 to i32
  %57 = add i32 %56, 2
  %58 = urem i32 %57, 200
  %59 = uitofp nneg i32 %58 to double
  %60 = fdiv double %59, 1.000000e+03
  %.reload117 = load i64, ptr %.reg2mem115, align 8
  %.reload156 = load ptr, ptr %.reg2mem154, align 8
  %61 = getelementptr inbounds [210 x double], ptr %.reload156, i64 %.reload117, i64 %.reload207
  store double %60, ptr %61, align 8, !tbaa !5
  %62 = add nuw nsw i64 %.reload207, 1
  store i64 %62, ptr %.reg2mem112, align 8
  %.reload114 = load i64, ptr %.reg2mem112, align 8
  %63 = icmp eq i64 %.reload114, 210
  br i1 %63, label %64, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %53
  %.reload113 = load i64, ptr %.reg2mem112, align 8
  store i64 %.reload113, ptr %.reg2mem206, align 8
  br label %53

64:                                               ; preds = %53
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  %65 = add nuw nsw i64 %.reload116, 1
  store i64 %65, ptr %.reg2mem109, align 8
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %66 = icmp eq i64 %.reload111, 220
  br i1 %66, label %._crit_edge10, label %._crit_edge11, !llvm.loop !18

._crit_edge11:                                    ; preds = %64
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  store i64 %.reload110, ptr %.reg2mem208, align 8
  br label %52

._crit_edge10:                                    ; preds = %64
  store i64 0, ptr %.reg2mem204, align 8
  br label %67

67:                                               ; preds = %._crit_edge15, %._crit_edge10
  %.reload205 = load i64, ptr %.reg2mem204, align 8
  store i64 %.reload205, ptr %.reg2mem105, align 8
  store i64 0, ptr %.reg2mem202, align 8
  br label %68

68:                                               ; preds = %._crit_edge13, %67
  %.reload203 = load i64, ptr %.reg2mem202, align 8
  store i64 %.reload203, ptr %.reg2mem101, align 8
  %.reload104 = load i64, ptr %.reg2mem101, align 8
  %.reload108 = load i64, ptr %.reg2mem105, align 8
  %.reload176 = load ptr, ptr %.reg2mem174, align 8
  %69 = getelementptr inbounds [190 x double], ptr %.reload176, i64 %.reload108, i64 %.reload104
  store ptr %69, ptr %.reg2mem98, align 8
  %.reload100 = load ptr, ptr %.reg2mem98, align 8
  store double 0.000000e+00, ptr %.reload100, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem198, align 8
  store i64 0, ptr %.reg2mem200, align 8
  br label %70

70:                                               ; preds = %._crit_edge12, %68
  %.reload201 = load i64, ptr %.reg2mem200, align 8
  %.reload199 = load double, ptr %.reg2mem198, align 8
  %.reload107 = load i64, ptr %.reg2mem105, align 8
  %.reload171 = load ptr, ptr %.reg2mem170, align 8
  %71 = getelementptr inbounds [200 x double], ptr %.reload171, i64 %.reload107, i64 %.reload201
  %72 = load double, ptr %71, align 8, !tbaa !5
  %.reload103 = load i64, ptr %.reg2mem101, align 8
  %.reload167 = load ptr, ptr %.reg2mem166, align 8
  %73 = getelementptr inbounds [190 x double], ptr %.reload167, i64 %.reload201, i64 %.reload103
  %74 = load double, ptr %73, align 8, !tbaa !5
  %75 = tail call double @llvm.fmuladd.f64(double %72, double %74, double %.reload199)
  store double %75, ptr %.reg2mem95, align 8
  %.reload97 = load double, ptr %.reg2mem95, align 8
  %.reload99 = load ptr, ptr %.reg2mem98, align 8
  store double %.reload97, ptr %.reload99, align 8, !tbaa !5
  %76 = add nuw nsw i64 %.reload201, 1
  store i64 %76, ptr %.reg2mem92, align 8
  %.reload94 = load i64, ptr %.reg2mem92, align 8
  %77 = icmp eq i64 %.reload94, 200
  br i1 %77, label %78, label %._crit_edge12, !llvm.loop !19

._crit_edge12:                                    ; preds = %70
  %.reload93 = load i64, ptr %.reg2mem92, align 8
  %.reload96 = load double, ptr %.reg2mem95, align 8
  store double %.reload96, ptr %.reg2mem198, align 8
  store i64 %.reload93, ptr %.reg2mem200, align 8
  br label %70

78:                                               ; preds = %70
  %.reload102 = load i64, ptr %.reg2mem101, align 8
  %79 = add nuw nsw i64 %.reload102, 1
  store i64 %79, ptr %.reg2mem89, align 8
  %.reload91 = load i64, ptr %.reg2mem89, align 8
  %80 = icmp eq i64 %.reload91, 190
  br i1 %80, label %81, label %._crit_edge13, !llvm.loop !20

._crit_edge13:                                    ; preds = %78
  %.reload90 = load i64, ptr %.reg2mem89, align 8
  store i64 %.reload90, ptr %.reg2mem202, align 8
  br label %68

81:                                               ; preds = %78
  %.reload106 = load i64, ptr %.reg2mem105, align 8
  %82 = add nuw nsw i64 %.reload106, 1
  store i64 %82, ptr %.reg2mem86, align 8
  %.reload88 = load i64, ptr %.reg2mem86, align 8
  %83 = icmp eq i64 %.reload88, 180
  br i1 %83, label %._crit_edge14, label %._crit_edge15, !llvm.loop !21

._crit_edge15:                                    ; preds = %81
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  store i64 %.reload87, ptr %.reg2mem204, align 8
  br label %67

._crit_edge14:                                    ; preds = %81
  store i64 0, ptr %.reg2mem196, align 8
  br label %84

84:                                               ; preds = %._crit_edge19, %._crit_edge14
  %.reload197 = load i64, ptr %.reg2mem196, align 8
  store i64 %.reload197, ptr %.reg2mem82, align 8
  store i64 0, ptr %.reg2mem194, align 8
  br label %85

85:                                               ; preds = %._crit_edge17, %84
  %.reload195 = load i64, ptr %.reg2mem194, align 8
  store i64 %.reload195, ptr %.reg2mem78, align 8
  %.reload81 = load i64, ptr %.reg2mem78, align 8
  %.reload85 = load i64, ptr %.reg2mem82, align 8
  %.reload164 = load ptr, ptr %.reg2mem162, align 8
  %86 = getelementptr inbounds [210 x double], ptr %.reload164, i64 %.reload85, i64 %.reload81
  store ptr %86, ptr %.reg2mem75, align 8
  %.reload77 = load ptr, ptr %.reg2mem75, align 8
  store double 0.000000e+00, ptr %.reload77, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem190, align 8
  store i64 0, ptr %.reg2mem192, align 8
  br label %87

87:                                               ; preds = %._crit_edge16, %85
  %.reload193 = load i64, ptr %.reg2mem192, align 8
  %.reload191 = load double, ptr %.reg2mem190, align 8
  %.reload84 = load i64, ptr %.reg2mem82, align 8
  %.reload159 = load ptr, ptr %.reg2mem158, align 8
  %88 = getelementptr inbounds [220 x double], ptr %.reload159, i64 %.reload84, i64 %.reload193
  %89 = load double, ptr %88, align 8, !tbaa !5
  %.reload80 = load i64, ptr %.reg2mem78, align 8
  %.reload155 = load ptr, ptr %.reg2mem154, align 8
  %90 = getelementptr inbounds [210 x double], ptr %.reload155, i64 %.reload193, i64 %.reload80
  %91 = load double, ptr %90, align 8, !tbaa !5
  %92 = tail call double @llvm.fmuladd.f64(double %89, double %91, double %.reload191)
  store double %92, ptr %.reg2mem72, align 8
  %.reload74 = load double, ptr %.reg2mem72, align 8
  %.reload76 = load ptr, ptr %.reg2mem75, align 8
  store double %.reload74, ptr %.reload76, align 8, !tbaa !5
  %93 = add nuw nsw i64 %.reload193, 1
  store i64 %93, ptr %.reg2mem69, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %94 = icmp eq i64 %.reload71, 220
  br i1 %94, label %95, label %._crit_edge16, !llvm.loop !22

._crit_edge16:                                    ; preds = %87
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %.reload73 = load double, ptr %.reg2mem72, align 8
  store double %.reload73, ptr %.reg2mem190, align 8
  store i64 %.reload70, ptr %.reg2mem192, align 8
  br label %87

95:                                               ; preds = %87
  %.reload79 = load i64, ptr %.reg2mem78, align 8
  %96 = add nuw nsw i64 %.reload79, 1
  store i64 %96, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %97 = icmp eq i64 %.reload68, 210
  br i1 %97, label %98, label %._crit_edge17, !llvm.loop !23

._crit_edge17:                                    ; preds = %95
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem194, align 8
  br label %85

98:                                               ; preds = %95
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %99 = add nuw nsw i64 %.reload83, 1
  store i64 %99, ptr %.reg2mem63, align 8
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %100 = icmp eq i64 %.reload65, 190
  br i1 %100, label %._crit_edge18, label %._crit_edge19, !llvm.loop !24

._crit_edge19:                                    ; preds = %98
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  store i64 %.reload64, ptr %.reg2mem196, align 8
  br label %84

._crit_edge18:                                    ; preds = %98
  store i64 0, ptr %.reg2mem188, align 8
  br label %101

101:                                              ; preds = %._crit_edge22, %._crit_edge18
  %.reload189 = load i64, ptr %.reg2mem188, align 8
  store i64 %.reload189, ptr %.reg2mem59, align 8
  store i64 0, ptr %.reg2mem186, align 8
  br label %102

102:                                              ; preds = %._crit_edge21, %101
  %.reload187 = load i64, ptr %.reg2mem186, align 8
  store i64 %.reload187, ptr %.reg2mem55, align 8
  %.reload58 = load i64, ptr %.reg2mem55, align 8
  %.reload62 = load i64, ptr %.reg2mem59, align 8
  %.reload152 = load ptr, ptr %.reg2mem150, align 8
  %103 = getelementptr inbounds [210 x double], ptr %.reload152, i64 %.reload62, i64 %.reload58
  store ptr %103, ptr %.reg2mem52, align 8
  %.reload54 = load ptr, ptr %.reg2mem52, align 8
  store double 0.000000e+00, ptr %.reload54, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem182, align 8
  store i64 0, ptr %.reg2mem184, align 8
  br label %104

104:                                              ; preds = %._crit_edge20, %102
  %.reload185 = load i64, ptr %.reg2mem184, align 8
  %.reload183 = load double, ptr %.reg2mem182, align 8
  %.reload61 = load i64, ptr %.reg2mem59, align 8
  %.reload175 = load ptr, ptr %.reg2mem174, align 8
  %105 = getelementptr inbounds [190 x double], ptr %.reload175, i64 %.reload61, i64 %.reload185
  %106 = load double, ptr %105, align 8, !tbaa !5
  %.reload57 = load i64, ptr %.reg2mem55, align 8
  %.reload163 = load ptr, ptr %.reg2mem162, align 8
  %107 = getelementptr inbounds [210 x double], ptr %.reload163, i64 %.reload185, i64 %.reload57
  %108 = load double, ptr %107, align 8, !tbaa !5
  %109 = tail call double @llvm.fmuladd.f64(double %106, double %108, double %.reload183)
  store double %109, ptr %.reg2mem49, align 8
  %.reload51 = load double, ptr %.reg2mem49, align 8
  %.reload53 = load ptr, ptr %.reg2mem52, align 8
  store double %.reload51, ptr %.reload53, align 8, !tbaa !5
  %110 = add nuw nsw i64 %.reload185, 1
  store i64 %110, ptr %.reg2mem46, align 8
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %111 = icmp eq i64 %.reload48, 190
  br i1 %111, label %112, label %._crit_edge20, !llvm.loop !25

._crit_edge20:                                    ; preds = %104
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %.reload50 = load double, ptr %.reg2mem49, align 8
  store double %.reload50, ptr %.reg2mem182, align 8
  store i64 %.reload47, ptr %.reg2mem184, align 8
  br label %104

112:                                              ; preds = %104
  %.reload56 = load i64, ptr %.reg2mem55, align 8
  %113 = add nuw nsw i64 %.reload56, 1
  store i64 %113, ptr %.reg2mem43, align 8
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %114 = icmp eq i64 %.reload45, 210
  br i1 %114, label %115, label %._crit_edge21, !llvm.loop !26

._crit_edge21:                                    ; preds = %112
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  store i64 %.reload44, ptr %.reg2mem186, align 8
  br label %102

115:                                              ; preds = %112
  %.reload60 = load i64, ptr %.reg2mem59, align 8
  %116 = add nuw nsw i64 %.reload60, 1
  store i64 %116, ptr %.reg2mem40, align 8
  %.reload42 = load i64, ptr %.reg2mem40, align 8
  %117 = icmp eq i64 %.reload42, 180
  br i1 %117, label %118, label %._crit_edge22, !llvm.loop !27

._crit_edge22:                                    ; preds = %115
  %.reload41 = load i64, ptr %.reg2mem40, align 8
  store i64 %.reload41, ptr %.reg2mem188, align 8
  br label %101

118:                                              ; preds = %115
  %119 = load ptr, ptr @stderr, align 8, !tbaa !28
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %119, ptr noundef nonnull @.str) #4
  %121 = load ptr, ptr @stderr, align 8, !tbaa !28
  %122 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %121, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem180, align 8
  br label %123

123:                                              ; preds = %._crit_edge25, %118
  %.reload181 = load i64, ptr %.reg2mem180, align 8
  store i64 %.reload181, ptr %.reg2mem36, align 8
  %.reload39 = load i64, ptr %.reg2mem36, align 8
  %124 = mul nuw nsw i64 %.reload39, 180
  store i64 %124, ptr %.reg2mem34, align 8
  store i64 0, ptr %.reg2mem178, align 8
  br label %125

125:                                              ; preds = %._crit_edge24, %123
  %.reload179 = load i64, ptr %.reg2mem178, align 8
  store i64 %.reload179, ptr %.reg2mem30, align 8
  %.reload33 = load i64, ptr %.reg2mem30, align 8
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %126 = add nuw nsw i64 %.reload33, %.reload35
  %127 = trunc nuw nsw i64 %126 to i32
  %128 = urem i32 %127, 20
  %129 = icmp eq i32 %128, 0
  br i1 %129, label %130, label %._crit_edge23

._crit_edge23:                                    ; preds = %125
  br label %133

130:                                              ; preds = %125
  %131 = load ptr, ptr @stderr, align 8, !tbaa !28
  %132 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %131, ptr noundef nonnull @.str.3) #4
  br label %133

133:                                              ; preds = %._crit_edge23, %130
  %134 = load ptr, ptr @stderr, align 8, !tbaa !28
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %.reload151 = load ptr, ptr %.reg2mem150, align 8
  %135 = getelementptr inbounds [210 x double], ptr %.reload151, i64 %.reload38, i64 %.reload32
  %136 = load double, ptr %135, align 8, !tbaa !5
  %137 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %134, ptr noundef nonnull @.str.4, double noundef %136) #4
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  %138 = add nuw nsw i64 %.reload31, 1
  store i64 %138, ptr %.reg2mem27, align 8
  %.reload29 = load i64, ptr %.reg2mem27, align 8
  %139 = icmp eq i64 %.reload29, 210
  br i1 %139, label %140, label %._crit_edge24, !llvm.loop !30

._crit_edge24:                                    ; preds = %133
  %.reload28 = load i64, ptr %.reg2mem27, align 8
  store i64 %.reload28, ptr %.reg2mem178, align 8
  br label %125

140:                                              ; preds = %133
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  %141 = add nuw nsw i64 %.reload37, 1
  store i64 %141, ptr %.reg2mem, align 8
  %.reload26 = load i64, ptr %.reg2mem, align 8
  %142 = icmp eq i64 %.reload26, 180
  br i1 %142, label %143, label %._crit_edge25, !llvm.loop !31

._crit_edge25:                                    ; preds = %140
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem180, align 8
  br label %123

143:                                              ; preds = %140
  %144 = load ptr, ptr @stderr, align 8, !tbaa !28
  %145 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %144, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %146 = load ptr, ptr @stderr, align 8, !tbaa !28
  %147 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %146, ptr noundef nonnull @.str.6) #4
  %.reload177 = load ptr, ptr %.reg2mem174, align 8
  tail call void @free(ptr noundef %.reload177) #4
  %.reload173 = load ptr, ptr %.reg2mem170, align 8
  tail call void @free(ptr noundef %.reload173) #4
  %.reload169 = load ptr, ptr %.reg2mem166, align 8
  tail call void @free(ptr noundef %.reload169) #4
  %.reload165 = load ptr, ptr %.reg2mem162, align 8
  tail call void @free(ptr noundef %.reload165) #4
  %.reload161 = load ptr, ptr %.reg2mem158, align 8
  tail call void @free(ptr noundef %.reload161) #4
  %.reload157 = load ptr, ptr %.reg2mem154, align 8
  tail call void @free(ptr noundef %.reload157) #4
  %.reload153 = load ptr, ptr %.reg2mem150, align 8
  tail call void @free(ptr noundef nonnull %.reload153) #4
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
!26 = distinct !{!26, !10, !11}
!27 = distinct !{!27, !10, !11}
!28 = !{!29, !29, i64 0}
!29 = !{!"any pointer", !7, i64 0}
!30 = distinct !{!30, !10, !11}
!31 = distinct !{!31, !10, !11}

; ModuleID = './polybench-output/correlation-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/datamining/correlation/correlation.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"corr\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem19 = alloca i64, align 8
  %.reg2mem22 = alloca i64, align 8
  %.reg2mem26 = alloca i64, align 8
  %.reg2mem28 = alloca i64, align 8
  %.reg2mem32 = alloca i64, align 8
  %.reg2mem35 = alloca i64, align 8
  %.reg2mem38 = alloca double, align 8
  %.reg2mem42 = alloca ptr, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem50 = alloca i64, align 8
  %.reg2mem53 = alloca i64, align 8
  %.reg2mem59 = alloca i64, align 8
  %.reg2mem61 = alloca i64, align 8
  %.reg2mem64 = alloca i64, align 8
  %.reg2mem67 = alloca i64, align 8
  %.reg2mem70 = alloca i64, align 8
  %.reg2mem73 = alloca i64, align 8
  %.reg2mem76 = alloca i64, align 8
  %.reg2mem79 = alloca double, align 8
  %.reg2mem83 = alloca ptr, align 8
  %.reg2mem85 = alloca ptr, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem95 = alloca i64, align 8
  %.reg2mem98 = alloca i64, align 8
  %.reg2mem101 = alloca double, align 8
  %.reg2mem105 = alloca ptr, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem113 = alloca i64, align 8
  %.reg2mem116 = alloca i64, align 8
  %.reg2mem119 = alloca double, align 8
  %.reg2mem121 = alloca i64, align 8
  %.reg2mem126 = alloca ptr, align 8
  %.reg2mem130 = alloca ptr, align 8
  %.reg2mem135 = alloca ptr, align 8
  %.reg2mem142 = alloca ptr, align 8
  %.reg2mem150 = alloca i64, align 8
  %.reg2mem152 = alloca i64, align 8
  %.reg2mem154 = alloca double, align 8
  %.reg2mem156 = alloca i64, align 8
  %.reg2mem158 = alloca i64, align 8
  %.reg2mem160 = alloca i64, align 8
  %.reg2mem162 = alloca i64, align 8
  %.reg2mem164 = alloca i64, align 8
  %.reg2mem166 = alloca i64, align 8
  %.reg2mem168 = alloca i64, align 8
  %.reg2mem170 = alloca double, align 8
  %.reg2mem172 = alloca i64, align 8
  %.reg2mem174 = alloca double, align 8
  %.reg2mem176 = alloca i64, align 8
  %.reg2mem178 = alloca i64, align 8
  %.reg2mem180 = alloca i64, align 8
  %.reg2mem182 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62400, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem142, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem135, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem130, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem126, align 8
  store i64 0, ptr %.reg2mem182, align 8
  br label %7

7:                                                ; preds = %._crit_edge2, %2
  %.reload183 = load i64, ptr %.reg2mem182, align 8
  store i64 %.reload183, ptr %.reg2mem121, align 8
  %.reload125 = load i64, ptr %.reg2mem121, align 8
  %8 = trunc nuw nsw i64 %.reload125 to i32
  %9 = uitofp nneg i32 %8 to double
  store double %9, ptr %.reg2mem119, align 8
  store i64 0, ptr %.reg2mem180, align 8
  br label %10

10:                                               ; preds = %._crit_edge, %7
  %.reload181 = load i64, ptr %.reg2mem180, align 8
  %.reload124 = load i64, ptr %.reg2mem121, align 8
  %11 = mul nuw nsw i64 %.reload181, %.reload124
  %12 = trunc nuw nsw i64 %11 to i32
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 2.400000e+02
  %.reload120 = load double, ptr %.reg2mem119, align 8
  %15 = fadd double %14, %.reload120
  %.reload123 = load i64, ptr %.reg2mem121, align 8
  %.reload148 = load ptr, ptr %.reg2mem142, align 8
  %16 = getelementptr inbounds [240 x double], ptr %.reload148, i64 %.reload123, i64 %.reload181
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = add nuw nsw i64 %.reload181, 1
  store i64 %17, ptr %.reg2mem116, align 8
  %.reload118 = load i64, ptr %.reg2mem116, align 8
  %18 = icmp eq i64 %.reload118, 240
  br i1 %18, label %19, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %10
  %.reload117 = load i64, ptr %.reg2mem116, align 8
  store i64 %.reload117, ptr %.reg2mem180, align 8
  br label %10

19:                                               ; preds = %10
  %.reload122 = load i64, ptr %.reg2mem121, align 8
  %20 = add nuw nsw i64 %.reload122, 1
  store i64 %20, ptr %.reg2mem113, align 8
  %.reload115 = load i64, ptr %.reg2mem113, align 8
  %21 = icmp eq i64 %.reload115, 260
  br i1 %21, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %19
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  store i64 %.reload114, ptr %.reg2mem182, align 8
  br label %7

._crit_edge1:                                     ; preds = %19
  store i64 0, ptr %.reg2mem178, align 8
  br label %22

22:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload179 = load i64, ptr %.reg2mem178, align 8
  store i64 %.reload179, ptr %.reg2mem109, align 8
  %.reload112 = load i64, ptr %.reg2mem109, align 8
  %.reload133 = load ptr, ptr %.reg2mem130, align 8
  %23 = getelementptr inbounds double, ptr %.reload133, i64 %.reload112
  store ptr %23, ptr %.reg2mem105, align 8
  %.reload108 = load ptr, ptr %.reg2mem105, align 8
  store double 0.000000e+00, ptr %.reload108, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem174, align 8
  store i64 0, ptr %.reg2mem176, align 8
  br label %24

24:                                               ; preds = %._crit_edge3, %22
  %.reload177 = load i64, ptr %.reg2mem176, align 8
  %.reload175 = load double, ptr %.reg2mem174, align 8
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %.reload147 = load ptr, ptr %.reg2mem142, align 8
  %25 = getelementptr inbounds [240 x double], ptr %.reload147, i64 %.reload177, i64 %.reload111
  %26 = load double, ptr %25, align 8, !tbaa !5
  %27 = fadd double %.reload175, %26
  store double %27, ptr %.reg2mem101, align 8
  %.reload104 = load double, ptr %.reg2mem101, align 8
  %.reload107 = load ptr, ptr %.reg2mem105, align 8
  store double %.reload104, ptr %.reload107, align 8, !tbaa !5
  %28 = add nuw nsw i64 %.reload177, 1
  store i64 %28, ptr %.reg2mem98, align 8
  %.reload100 = load i64, ptr %.reg2mem98, align 8
  %29 = icmp eq i64 %.reload100, 260
  br i1 %29, label %30, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %24
  %.reload99 = load i64, ptr %.reg2mem98, align 8
  %.reload103 = load double, ptr %.reg2mem101, align 8
  store double %.reload103, ptr %.reg2mem174, align 8
  store i64 %.reload99, ptr %.reg2mem176, align 8
  br label %24

30:                                               ; preds = %24
  %.reload102 = load double, ptr %.reg2mem101, align 8
  %31 = fdiv double %.reload102, 2.600000e+02
  %.reload106 = load ptr, ptr %.reg2mem105, align 8
  store double %31, ptr %.reload106, align 8, !tbaa !5
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  %32 = add nuw nsw i64 %.reload110, 1
  store i64 %32, ptr %.reg2mem95, align 8
  %.reload97 = load i64, ptr %.reg2mem95, align 8
  %33 = icmp eq i64 %.reload97, 240
  br i1 %33, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %30
  %.reload96 = load i64, ptr %.reg2mem95, align 8
  store i64 %.reload96, ptr %.reg2mem178, align 8
  br label %22

._crit_edge4:                                     ; preds = %30
  store i64 0, ptr %.reg2mem172, align 8
  br label %34

34:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload173 = load i64, ptr %.reg2mem172, align 8
  store i64 %.reload173, ptr %.reg2mem90, align 8
  %.reload94 = load i64, ptr %.reg2mem90, align 8
  %.reload128 = load ptr, ptr %.reg2mem126, align 8
  %35 = getelementptr inbounds double, ptr %.reload128, i64 %.reload94
  store ptr %35, ptr %.reg2mem85, align 8
  %.reload89 = load ptr, ptr %.reg2mem85, align 8
  store double 0.000000e+00, ptr %.reload89, align 8, !tbaa !5
  %.reload93 = load i64, ptr %.reg2mem90, align 8
  %.reload132 = load ptr, ptr %.reg2mem130, align 8
  %36 = getelementptr inbounds double, ptr %.reload132, i64 %.reload93
  store ptr %36, ptr %.reg2mem83, align 8
  store i64 0, ptr %.reg2mem168, align 8
  store double 0.000000e+00, ptr %.reg2mem170, align 8
  br label %37

37:                                               ; preds = %._crit_edge6, %34
  %.reload171 = load double, ptr %.reg2mem170, align 8
  %.reload169 = load i64, ptr %.reg2mem168, align 8
  %.reload92 = load i64, ptr %.reg2mem90, align 8
  %.reload146 = load ptr, ptr %.reg2mem142, align 8
  %38 = getelementptr inbounds [240 x double], ptr %.reload146, i64 %.reload169, i64 %.reload92
  %39 = load double, ptr %38, align 8, !tbaa !5
  %.reload84 = load ptr, ptr %.reg2mem83, align 8
  %40 = load double, ptr %.reload84, align 8, !tbaa !5
  %41 = fsub double %39, %40
  %42 = tail call double @llvm.fmuladd.f64(double %41, double %41, double %.reload171)
  store double %42, ptr %.reg2mem79, align 8
  %.reload82 = load double, ptr %.reg2mem79, align 8
  %.reload88 = load ptr, ptr %.reg2mem85, align 8
  store double %.reload82, ptr %.reload88, align 8, !tbaa !5
  %43 = add nuw nsw i64 %.reload169, 1
  store i64 %43, ptr %.reg2mem76, align 8
  %.reload78 = load i64, ptr %.reg2mem76, align 8
  %44 = icmp eq i64 %.reload78, 260
  br i1 %44, label %45, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %37
  %.reload77 = load i64, ptr %.reg2mem76, align 8
  %.reload81 = load double, ptr %.reg2mem79, align 8
  store i64 %.reload77, ptr %.reg2mem168, align 8
  store double %.reload81, ptr %.reg2mem170, align 8
  br label %37

45:                                               ; preds = %37
  %.reload80 = load double, ptr %.reg2mem79, align 8
  %46 = fdiv double %.reload80, 2.600000e+02
  %.reload87 = load ptr, ptr %.reg2mem85, align 8
  store double %46, ptr %.reload87, align 8, !tbaa !5
  %47 = tail call double @sqrt(double noundef %46) #4
  %48 = fcmp ugt double %47, 1.000000e-01
  %49 = select i1 %48, double %47, double 1.000000e+00
  %.reload86 = load ptr, ptr %.reg2mem85, align 8
  store double %49, ptr %.reload86, align 8, !tbaa !5
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %50 = add nuw nsw i64 %.reload91, 1
  store i64 %50, ptr %.reg2mem73, align 8
  %.reload75 = load i64, ptr %.reg2mem73, align 8
  %51 = icmp eq i64 %.reload75, 240
  br i1 %51, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %45
  %.reload74 = load i64, ptr %.reg2mem73, align 8
  store i64 %.reload74, ptr %.reg2mem172, align 8
  br label %34

._crit_edge7:                                     ; preds = %45
  store i64 0, ptr %.reg2mem166, align 8
  br label %52

52:                                               ; preds = %._crit_edge11, %._crit_edge7
  %.reload167 = load i64, ptr %.reg2mem166, align 8
  store i64 %.reload167, ptr %.reg2mem70, align 8
  store i64 0, ptr %.reg2mem164, align 8
  br label %53

53:                                               ; preds = %._crit_edge9, %52
  %.reload165 = load i64, ptr %.reg2mem164, align 8
  %.reload131 = load ptr, ptr %.reg2mem130, align 8
  %54 = getelementptr inbounds double, ptr %.reload131, i64 %.reload165
  %55 = load double, ptr %54, align 8, !tbaa !5
  %.reload72 = load i64, ptr %.reg2mem70, align 8
  %.reload145 = load ptr, ptr %.reg2mem142, align 8
  %56 = getelementptr inbounds [240 x double], ptr %.reload145, i64 %.reload72, i64 %.reload165
  %57 = load double, ptr %56, align 8, !tbaa !5
  %58 = fsub double %57, %55
  store double %58, ptr %56, align 8, !tbaa !5
  %59 = tail call double @sqrt(double noundef 2.600000e+02) #4
  %.reload127 = load ptr, ptr %.reg2mem126, align 8
  %60 = getelementptr inbounds double, ptr %.reload127, i64 %.reload165
  %61 = load double, ptr %60, align 8, !tbaa !5
  %62 = fmul double %59, %61
  %63 = load double, ptr %56, align 8, !tbaa !5
  %64 = fdiv double %63, %62
  store double %64, ptr %56, align 8, !tbaa !5
  %65 = add nuw nsw i64 %.reload165, 1
  store i64 %65, ptr %.reg2mem67, align 8
  %.reload69 = load i64, ptr %.reg2mem67, align 8
  %66 = icmp eq i64 %.reload69, 240
  br i1 %66, label %67, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %53
  %.reload68 = load i64, ptr %.reg2mem67, align 8
  store i64 %.reload68, ptr %.reg2mem164, align 8
  br label %53

67:                                               ; preds = %53
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  %68 = add nuw nsw i64 %.reload71, 1
  store i64 %68, ptr %.reg2mem64, align 8
  %.reload66 = load i64, ptr %.reg2mem64, align 8
  %69 = icmp eq i64 %.reload66, 260
  br i1 %69, label %._crit_edge10, label %._crit_edge11, !llvm.loop !18

._crit_edge11:                                    ; preds = %67
  %.reload65 = load i64, ptr %.reg2mem64, align 8
  store i64 %.reload65, ptr %.reg2mem166, align 8
  br label %52

._crit_edge10:                                    ; preds = %67
  store i64 1, ptr %.reg2mem160, align 8
  store i64 0, ptr %.reg2mem162, align 8
  br label %74

70:                                               ; preds = %86
  %.reload57 = load i64, ptr %.reg2mem53, align 8
  %71 = add nuw nsw i64 %.reload57, 1
  store i64 %71, ptr %.reg2mem61, align 8
  %.reload52 = load i64, ptr %.reg2mem50, align 8
  %72 = add nuw nsw i64 %.reload52, 1
  store i64 %72, ptr %.reg2mem59, align 8
  %.reload63 = load i64, ptr %.reg2mem61, align 8
  %73 = icmp eq i64 %.reload63, 239
  br i1 %73, label %90, label %._crit_edge12, !llvm.loop !19

._crit_edge12:                                    ; preds = %70
  %.reload60 = load i64, ptr %.reg2mem59, align 8
  %.reload62 = load i64, ptr %.reg2mem61, align 8
  store i64 %.reload60, ptr %.reg2mem160, align 8
  store i64 %.reload62, ptr %.reg2mem162, align 8
  br label %74

74:                                               ; preds = %._crit_edge12, %._crit_edge10
  %.reload163 = load i64, ptr %.reg2mem162, align 8
  %.reload161 = load i64, ptr %.reg2mem160, align 8
  store i64 %.reload163, ptr %.reg2mem53, align 8
  store i64 %.reload161, ptr %.reg2mem50, align 8
  %.reload58 = load i64, ptr %.reg2mem53, align 8
  %.reload140 = load ptr, ptr %.reg2mem135, align 8
  %75 = getelementptr inbounds [240 x double], ptr %.reload140, i64 %.reload58, i64 %.reload58
  store double 1.000000e+00, ptr %75, align 8, !tbaa !5
  %.reload51 = load i64, ptr %.reg2mem50, align 8
  store i64 %.reload51, ptr %.reg2mem158, align 8
  br label %76

76:                                               ; preds = %._crit_edge14, %74
  %.reload159 = load i64, ptr %.reg2mem158, align 8
  store i64 %.reload159, ptr %.reg2mem45, align 8
  %.reload49 = load i64, ptr %.reg2mem45, align 8
  %.reload56 = load i64, ptr %.reg2mem53, align 8
  %.reload139 = load ptr, ptr %.reg2mem135, align 8
  %77 = getelementptr inbounds [240 x double], ptr %.reload139, i64 %.reload56, i64 %.reload49
  store ptr %77, ptr %.reg2mem42, align 8
  %.reload44 = load ptr, ptr %.reg2mem42, align 8
  store double 0.000000e+00, ptr %.reload44, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem154, align 8
  store i64 0, ptr %.reg2mem156, align 8
  br label %78

78:                                               ; preds = %._crit_edge13, %76
  %.reload157 = load i64, ptr %.reg2mem156, align 8
  %.reload155 = load double, ptr %.reg2mem154, align 8
  %.reload55 = load i64, ptr %.reg2mem53, align 8
  %.reload144 = load ptr, ptr %.reg2mem142, align 8
  %79 = getelementptr inbounds [240 x double], ptr %.reload144, i64 %.reload157, i64 %.reload55
  %80 = load double, ptr %79, align 8, !tbaa !5
  %.reload48 = load i64, ptr %.reg2mem45, align 8
  %.reload143 = load ptr, ptr %.reg2mem142, align 8
  %81 = getelementptr inbounds [240 x double], ptr %.reload143, i64 %.reload157, i64 %.reload48
  %82 = load double, ptr %81, align 8, !tbaa !5
  %83 = tail call double @llvm.fmuladd.f64(double %80, double %82, double %.reload155)
  store double %83, ptr %.reg2mem38, align 8
  %.reload41 = load double, ptr %.reg2mem38, align 8
  %.reload43 = load ptr, ptr %.reg2mem42, align 8
  store double %.reload41, ptr %.reload43, align 8, !tbaa !5
  %84 = add nuw nsw i64 %.reload157, 1
  store i64 %84, ptr %.reg2mem35, align 8
  %.reload37 = load i64, ptr %.reg2mem35, align 8
  %85 = icmp eq i64 %.reload37, 260
  br i1 %85, label %86, label %._crit_edge13, !llvm.loop !20

._crit_edge13:                                    ; preds = %78
  %.reload36 = load i64, ptr %.reg2mem35, align 8
  %.reload40 = load double, ptr %.reg2mem38, align 8
  store double %.reload40, ptr %.reg2mem154, align 8
  store i64 %.reload36, ptr %.reg2mem156, align 8
  br label %78

86:                                               ; preds = %78
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %.reload54 = load i64, ptr %.reg2mem53, align 8
  %.reload138 = load ptr, ptr %.reg2mem135, align 8
  %87 = getelementptr inbounds [240 x double], ptr %.reload138, i64 %.reload47, i64 %.reload54
  %.reload39 = load double, ptr %.reg2mem38, align 8
  store double %.reload39, ptr %87, align 8, !tbaa !5
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  %88 = add nuw nsw i64 %.reload46, 1
  store i64 %88, ptr %.reg2mem32, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %89 = icmp eq i64 %.reload34, 240
  br i1 %89, label %70, label %._crit_edge14, !llvm.loop !21

._crit_edge14:                                    ; preds = %86
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  store i64 %.reload33, ptr %.reg2mem158, align 8
  br label %76

90:                                               ; preds = %70
  %.reload137 = load ptr, ptr %.reg2mem135, align 8
  %91 = getelementptr inbounds i8, ptr %.reload137, i64 460792
  store double 1.000000e+00, ptr %91, align 8, !tbaa !5
  %92 = load ptr, ptr @stderr, align 8, !tbaa !22
  %93 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %92, ptr noundef nonnull @.str) #4
  %94 = load ptr, ptr @stderr, align 8, !tbaa !22
  %95 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %94, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem152, align 8
  br label %96

96:                                               ; preds = %._crit_edge17, %90
  %.reload153 = load i64, ptr %.reg2mem152, align 8
  store i64 %.reload153, ptr %.reg2mem28, align 8
  %.reload31 = load i64, ptr %.reg2mem28, align 8
  %97 = mul nuw nsw i64 %.reload31, 240
  store i64 %97, ptr %.reg2mem26, align 8
  store i64 0, ptr %.reg2mem150, align 8
  br label %98

98:                                               ; preds = %._crit_edge16, %96
  %.reload151 = load i64, ptr %.reg2mem150, align 8
  store i64 %.reload151, ptr %.reg2mem22, align 8
  %.reload25 = load i64, ptr %.reg2mem22, align 8
  %.reload27 = load i64, ptr %.reg2mem26, align 8
  %99 = add nuw nsw i64 %.reload25, %.reload27
  %100 = trunc nuw nsw i64 %99 to i32
  %101 = urem i32 %100, 20
  %102 = icmp eq i32 %101, 0
  br i1 %102, label %103, label %._crit_edge15

._crit_edge15:                                    ; preds = %98
  br label %106

103:                                              ; preds = %98
  %104 = load ptr, ptr @stderr, align 8, !tbaa !22
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %104, ptr noundef nonnull @.str.3) #4
  br label %106

106:                                              ; preds = %._crit_edge15, %103
  %107 = load ptr, ptr @stderr, align 8, !tbaa !22
  %.reload24 = load i64, ptr %.reg2mem22, align 8
  %.reload30 = load i64, ptr %.reg2mem28, align 8
  %.reload136 = load ptr, ptr %.reg2mem135, align 8
  %108 = getelementptr inbounds [240 x double], ptr %.reload136, i64 %.reload30, i64 %.reload24
  %109 = load double, ptr %108, align 8, !tbaa !5
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %107, ptr noundef nonnull @.str.4, double noundef %109) #4
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  %111 = add nuw nsw i64 %.reload23, 1
  store i64 %111, ptr %.reg2mem19, align 8
  %.reload21 = load i64, ptr %.reg2mem19, align 8
  %112 = icmp eq i64 %.reload21, 240
  br i1 %112, label %113, label %._crit_edge16, !llvm.loop !24

._crit_edge16:                                    ; preds = %106
  %.reload20 = load i64, ptr %.reg2mem19, align 8
  store i64 %.reload20, ptr %.reg2mem150, align 8
  br label %98

113:                                              ; preds = %106
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  %114 = add nuw nsw i64 %.reload29, 1
  store i64 %114, ptr %.reg2mem, align 8
  %.reload18 = load i64, ptr %.reg2mem, align 8
  %115 = icmp eq i64 %.reload18, 240
  br i1 %115, label %116, label %._crit_edge17, !llvm.loop !25

._crit_edge17:                                    ; preds = %113
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem152, align 8
  br label %96

116:                                              ; preds = %113
  %117 = load ptr, ptr @stderr, align 8, !tbaa !22
  %118 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %117, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %119 = load ptr, ptr @stderr, align 8, !tbaa !22
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %119, ptr noundef nonnull @.str.6) #4
  %.reload149 = load ptr, ptr %.reg2mem142, align 8
  tail call void @free(ptr noundef %.reload149) #4
  %.reload141 = load ptr, ptr %.reg2mem135, align 8
  tail call void @free(ptr noundef nonnull %.reload141) #4
  %.reload134 = load ptr, ptr %.reg2mem130, align 8
  tail call void @free(ptr noundef %.reload134) #4
  %.reload129 = load ptr, ptr %.reg2mem126, align 8
  tail call void @free(ptr noundef %.reload129) #4
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
!22 = !{!23, !23, i64 0}
!23 = !{!"any pointer", !7, i64 0}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}

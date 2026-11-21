; ModuleID = './polybench-output/gramschmidt-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/gramschmidt/gramschmidt.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"R\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"Q\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem23 = alloca i64, align 8
  %.reg2mem26 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem32 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem39 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem55 = alloca i64, align 8
  %.reg2mem58 = alloca i64, align 8
  %.reg2mem61 = alloca double, align 8
  %.reg2mem64 = alloca ptr, align 8
  %.reg2mem68 = alloca i64, align 8
  %.reg2mem73 = alloca i64, align 8
  %.reg2mem76 = alloca i64, align 8
  %.reg2mem79 = alloca ptr, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem85 = alloca double, align 8
  %.reg2mem88 = alloca i64, align 8
  %.reg2mem91 = alloca i64, align 8
  %.reg2mem101 = alloca i64, align 8
  %.reg2mem103 = alloca i64, align 8
  %.reg2mem106 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem112 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem118 = alloca i64, align 8
  %.reg2mem123 = alloca ptr, align 8
  %.reg2mem130 = alloca ptr, align 8
  %.reg2mem136 = alloca ptr, align 8
  %.reg2mem143 = alloca i64, align 8
  %.reg2mem145 = alloca i64, align 8
  %.reg2mem147 = alloca i64, align 8
  %.reg2mem149 = alloca i64, align 8
  %.reg2mem151 = alloca i64, align 8
  %.reg2mem153 = alloca double, align 8
  %.reg2mem155 = alloca i64, align 8
  %.reg2mem157 = alloca i64, align 8
  %.reg2mem159 = alloca i64, align 8
  %.reg2mem161 = alloca double, align 8
  %.reg2mem163 = alloca i64, align 8
  %.reg2mem165 = alloca i64, align 8
  %.reg2mem167 = alloca i64, align 8
  %.reg2mem169 = alloca i64, align 8
  %.reg2mem171 = alloca i64, align 8
  %.reg2mem173 = alloca i64, align 8
  %.reg2mem175 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem136, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem130, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem123, align 8
  store i64 0, ptr %.reg2mem175, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload176 = load i64, ptr %.reg2mem175, align 8
  store i64 %.reload176, ptr %.reg2mem118, align 8
  store i64 0, ptr %.reg2mem173, align 8
  br label %7

7:                                                ; preds = %._crit_edge, %6
  %.reload174 = load i64, ptr %.reg2mem173, align 8
  %.reload122 = load i64, ptr %.reg2mem118, align 8
  %8 = mul nuw nsw i64 %.reload174, %.reload122
  %9 = trunc nuw nsw i64 %8 to i32
  %10 = urem i32 %9, 200
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 2.000000e+02
  %13 = tail call double @llvm.fmuladd.f64(double %12, double 1.000000e+02, double 1.000000e+01)
  %.reload121 = load i64, ptr %.reg2mem118, align 8
  %.reload141 = load ptr, ptr %.reg2mem136, align 8
  %14 = getelementptr inbounds [240 x double], ptr %.reload141, i64 %.reload121, i64 %.reload174
  store double %13, ptr %14, align 8, !tbaa !5
  %.reload120 = load i64, ptr %.reg2mem118, align 8
  %.reload128 = load ptr, ptr %.reg2mem123, align 8
  %15 = getelementptr inbounds [240 x double], ptr %.reload128, i64 %.reload120, i64 %.reload174
  store double 0.000000e+00, ptr %15, align 8, !tbaa !5
  %16 = add nuw nsw i64 %.reload174, 1
  store i64 %16, ptr %.reg2mem115, align 8
  %.reload117 = load i64, ptr %.reg2mem115, align 8
  %17 = icmp eq i64 %.reload117, 240
  br i1 %17, label %18, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %7
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  store i64 %.reload116, ptr %.reg2mem173, align 8
  br label %7

18:                                               ; preds = %7
  %.reload119 = load i64, ptr %.reg2mem118, align 8
  %19 = add nuw nsw i64 %.reload119, 1
  store i64 %19, ptr %.reg2mem112, align 8
  %.reload114 = load i64, ptr %.reg2mem112, align 8
  %20 = icmp eq i64 %.reload114, 200
  br i1 %20, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %18
  %.reload113 = load i64, ptr %.reg2mem112, align 8
  store i64 %.reload113, ptr %.reg2mem175, align 8
  br label %6

._crit_edge1:                                     ; preds = %18
  store i64 0, ptr %.reg2mem171, align 8
  br label %21

21:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload172 = load i64, ptr %.reg2mem171, align 8
  store i64 %.reload172, ptr %.reg2mem109, align 8
  store i64 0, ptr %.reg2mem169, align 8
  br label %22

22:                                               ; preds = %._crit_edge3, %21
  %.reload170 = load i64, ptr %.reg2mem169, align 8
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %.reload134 = load ptr, ptr %.reg2mem130, align 8
  %23 = getelementptr inbounds [240 x double], ptr %.reload134, i64 %.reload111, i64 %.reload170
  store double 0.000000e+00, ptr %23, align 8, !tbaa !5
  %24 = add nuw nsw i64 %.reload170, 1
  store i64 %24, ptr %.reg2mem106, align 8
  %.reload108 = load i64, ptr %.reg2mem106, align 8
  %25 = icmp eq i64 %.reload108, 240
  br i1 %25, label %26, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %22
  %.reload107 = load i64, ptr %.reg2mem106, align 8
  store i64 %.reload107, ptr %.reg2mem169, align 8
  br label %22

26:                                               ; preds = %22
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  %27 = add nuw nsw i64 %.reload110, 1
  store i64 %27, ptr %.reg2mem103, align 8
  %.reload105 = load i64, ptr %.reg2mem103, align 8
  %28 = icmp eq i64 %.reload105, 240
  br i1 %28, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %26
  %.reload104 = load i64, ptr %.reg2mem103, align 8
  store i64 %.reload104, ptr %.reg2mem171, align 8
  br label %21

._crit_edge4:                                     ; preds = %26
  store i64 1, ptr %.reg2mem165, align 8
  store i64 0, ptr %.reg2mem167, align 8
  br label %32

29:                                               ; preds = %._crit_edge14, %._crit_edge10
  %.reload90 = load i64, ptr %.reg2mem88, align 8
  %30 = add nuw nsw i64 %.reload90, 1
  store i64 %30, ptr %.reg2mem101, align 8
  %.reload75 = load i64, ptr %.reg2mem73, align 8
  %31 = icmp eq i64 %.reload75, 240
  br i1 %31, label %76, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %29
  %.reload74 = load i64, ptr %.reg2mem73, align 8
  %.reload102 = load i64, ptr %.reg2mem101, align 8
  store i64 %.reload102, ptr %.reg2mem165, align 8
  store i64 %.reload74, ptr %.reg2mem167, align 8
  br label %32

32:                                               ; preds = %._crit_edge6, %._crit_edge4
  %.reload168 = load i64, ptr %.reg2mem167, align 8
  %.reload166 = load i64, ptr %.reg2mem165, align 8
  store i64 %.reload168, ptr %.reg2mem91, align 8
  store i64 %.reload166, ptr %.reg2mem88, align 8
  store double 0.000000e+00, ptr %.reg2mem161, align 8
  store i64 0, ptr %.reg2mem163, align 8
  br label %33

33:                                               ; preds = %._crit_edge7, %32
  %.reload164 = load i64, ptr %.reg2mem163, align 8
  %.reload162 = load double, ptr %.reg2mem161, align 8
  %.reload100 = load i64, ptr %.reg2mem91, align 8
  %.reload140 = load ptr, ptr %.reg2mem136, align 8
  %34 = getelementptr inbounds [240 x double], ptr %.reload140, i64 %.reload164, i64 %.reload100
  %35 = load double, ptr %34, align 8, !tbaa !5
  %36 = tail call double @llvm.fmuladd.f64(double %35, double %35, double %.reload162)
  store double %36, ptr %.reg2mem85, align 8
  %37 = add nuw nsw i64 %.reload164, 1
  store i64 %37, ptr %.reg2mem82, align 8
  %.reload84 = load i64, ptr %.reg2mem82, align 8
  %38 = icmp eq i64 %.reload84, 200
  br i1 %38, label %39, label %._crit_edge7, !llvm.loop !16

._crit_edge7:                                     ; preds = %33
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %.reload87 = load double, ptr %.reg2mem85, align 8
  store double %.reload87, ptr %.reg2mem161, align 8
  store i64 %.reload83, ptr %.reg2mem163, align 8
  br label %33

39:                                               ; preds = %33
  %.reload86 = load double, ptr %.reg2mem85, align 8
  %40 = tail call double @sqrt(double noundef %.reload86) #4
  %.reload99 = load i64, ptr %.reg2mem91, align 8
  %.reload133 = load ptr, ptr %.reg2mem130, align 8
  %41 = getelementptr inbounds [240 x double], ptr %.reload133, i64 %.reload99, i64 %.reload99
  store ptr %41, ptr %.reg2mem79, align 8
  %.reload81 = load ptr, ptr %.reg2mem79, align 8
  store double %40, ptr %.reload81, align 8, !tbaa !5
  store i64 0, ptr %.reg2mem159, align 8
  br label %42

42:                                               ; preds = %._crit_edge8, %39
  %.reload160 = load i64, ptr %.reg2mem159, align 8
  %.reload98 = load i64, ptr %.reg2mem91, align 8
  %.reload139 = load ptr, ptr %.reg2mem136, align 8
  %43 = getelementptr inbounds [240 x double], ptr %.reload139, i64 %.reload160, i64 %.reload98
  %44 = load double, ptr %43, align 8, !tbaa !5
  %.reload80 = load ptr, ptr %.reg2mem79, align 8
  %45 = load double, ptr %.reload80, align 8, !tbaa !5
  %46 = fdiv double %44, %45
  %.reload97 = load i64, ptr %.reg2mem91, align 8
  %.reload127 = load ptr, ptr %.reg2mem123, align 8
  %47 = getelementptr inbounds [240 x double], ptr %.reload127, i64 %.reload160, i64 %.reload97
  store double %46, ptr %47, align 8, !tbaa !5
  %48 = add nuw nsw i64 %.reload160, 1
  store i64 %48, ptr %.reg2mem76, align 8
  %.reload78 = load i64, ptr %.reg2mem76, align 8
  %49 = icmp eq i64 %.reload78, 200
  br i1 %49, label %50, label %._crit_edge8, !llvm.loop !17

._crit_edge8:                                     ; preds = %42
  %.reload77 = load i64, ptr %.reg2mem76, align 8
  store i64 %.reload77, ptr %.reg2mem159, align 8
  br label %42

50:                                               ; preds = %42
  %.reload96 = load i64, ptr %.reg2mem91, align 8
  %51 = add nuw nsw i64 %.reload96, 1
  store i64 %51, ptr %.reg2mem73, align 8
  %.reload95 = load i64, ptr %.reg2mem91, align 8
  %52 = icmp ult i64 %.reload95, 239
  br i1 %52, label %._crit_edge9, label %._crit_edge10

._crit_edge10:                                    ; preds = %50
  br label %29

._crit_edge9:                                     ; preds = %50
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  store i64 %.reload89, ptr %.reg2mem157, align 8
  br label %53

53:                                               ; preds = %._crit_edge15, %._crit_edge9
  %.reload158 = load i64, ptr %.reg2mem157, align 8
  store i64 %.reload158, ptr %.reg2mem68, align 8
  %.reload72 = load i64, ptr %.reg2mem68, align 8
  %.reload94 = load i64, ptr %.reg2mem91, align 8
  %.reload132 = load ptr, ptr %.reg2mem130, align 8
  %54 = getelementptr inbounds [240 x double], ptr %.reload132, i64 %.reload94, i64 %.reload72
  store ptr %54, ptr %.reg2mem64, align 8
  %.reload67 = load ptr, ptr %.reg2mem64, align 8
  store double 0.000000e+00, ptr %.reload67, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem153, align 8
  store i64 0, ptr %.reg2mem155, align 8
  br label %55

55:                                               ; preds = %._crit_edge12, %53
  %.reload156 = load i64, ptr %.reg2mem155, align 8
  %.reload154 = load double, ptr %.reg2mem153, align 8
  %.reload93 = load i64, ptr %.reg2mem91, align 8
  %.reload126 = load ptr, ptr %.reg2mem123, align 8
  %56 = getelementptr inbounds [240 x double], ptr %.reload126, i64 %.reload156, i64 %.reload93
  %57 = load double, ptr %56, align 8, !tbaa !5
  %.reload71 = load i64, ptr %.reg2mem68, align 8
  %.reload138 = load ptr, ptr %.reg2mem136, align 8
  %58 = getelementptr inbounds [240 x double], ptr %.reload138, i64 %.reload156, i64 %.reload71
  %59 = load double, ptr %58, align 8, !tbaa !5
  %60 = tail call double @llvm.fmuladd.f64(double %57, double %59, double %.reload154)
  store double %60, ptr %.reg2mem61, align 8
  %.reload63 = load double, ptr %.reg2mem61, align 8
  %.reload66 = load ptr, ptr %.reg2mem64, align 8
  store double %.reload63, ptr %.reload66, align 8, !tbaa !5
  %61 = add nuw nsw i64 %.reload156, 1
  store i64 %61, ptr %.reg2mem58, align 8
  %.reload60 = load i64, ptr %.reg2mem58, align 8
  %62 = icmp eq i64 %.reload60, 200
  br i1 %62, label %._crit_edge11, label %._crit_edge12, !llvm.loop !18

._crit_edge12:                                    ; preds = %55
  %.reload59 = load i64, ptr %.reg2mem58, align 8
  %.reload62 = load double, ptr %.reg2mem61, align 8
  store double %.reload62, ptr %.reg2mem153, align 8
  store i64 %.reload59, ptr %.reg2mem155, align 8
  br label %55

._crit_edge11:                                    ; preds = %55
  store i64 0, ptr %.reg2mem151, align 8
  br label %63

63:                                               ; preds = %._crit_edge13, %._crit_edge11
  %.reload152 = load i64, ptr %.reg2mem151, align 8
  %.reload70 = load i64, ptr %.reg2mem68, align 8
  %.reload137 = load ptr, ptr %.reg2mem136, align 8
  %64 = getelementptr inbounds [240 x double], ptr %.reload137, i64 %.reload152, i64 %.reload70
  %65 = load double, ptr %64, align 8, !tbaa !5
  %.reload92 = load i64, ptr %.reg2mem91, align 8
  %.reload125 = load ptr, ptr %.reg2mem123, align 8
  %66 = getelementptr inbounds [240 x double], ptr %.reload125, i64 %.reload152, i64 %.reload92
  %67 = load double, ptr %66, align 8, !tbaa !5
  %.reload65 = load ptr, ptr %.reg2mem64, align 8
  %68 = load double, ptr %.reload65, align 8, !tbaa !5
  %69 = fneg double %67
  %70 = tail call double @llvm.fmuladd.f64(double %69, double %68, double %65)
  store double %70, ptr %64, align 8, !tbaa !5
  %71 = add nuw nsw i64 %.reload152, 1
  store i64 %71, ptr %.reg2mem55, align 8
  %.reload57 = load i64, ptr %.reg2mem55, align 8
  %72 = icmp eq i64 %.reload57, 200
  br i1 %72, label %73, label %._crit_edge13, !llvm.loop !19

._crit_edge13:                                    ; preds = %63
  %.reload56 = load i64, ptr %.reg2mem55, align 8
  store i64 %.reload56, ptr %.reg2mem151, align 8
  br label %63

73:                                               ; preds = %63
  %.reload69 = load i64, ptr %.reg2mem68, align 8
  %74 = add nuw nsw i64 %.reload69, 1
  store i64 %74, ptr %.reg2mem52, align 8
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %75 = icmp eq i64 %.reload54, 240
  br i1 %75, label %._crit_edge14, label %._crit_edge15, !llvm.loop !20

._crit_edge15:                                    ; preds = %73
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  store i64 %.reload53, ptr %.reg2mem157, align 8
  br label %53

._crit_edge14:                                    ; preds = %73
  br label %29

76:                                               ; preds = %29
  %77 = load ptr, ptr @stderr, align 8, !tbaa !21
  %78 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %77, ptr noundef nonnull @.str) #4
  %79 = load ptr, ptr @stderr, align 8, !tbaa !21
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem149, align 8
  br label %81

81:                                               ; preds = %._crit_edge18, %76
  %.reload150 = load i64, ptr %.reg2mem149, align 8
  store i64 %.reload150, ptr %.reg2mem48, align 8
  %.reload51 = load i64, ptr %.reg2mem48, align 8
  %82 = mul nuw nsw i64 %.reload51, 240
  store i64 %82, ptr %.reg2mem46, align 8
  store i64 0, ptr %.reg2mem147, align 8
  br label %83

83:                                               ; preds = %._crit_edge17, %81
  %.reload148 = load i64, ptr %.reg2mem147, align 8
  store i64 %.reload148, ptr %.reg2mem42, align 8
  %.reload45 = load i64, ptr %.reg2mem42, align 8
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %84 = add nuw nsw i64 %.reload45, %.reload47
  %85 = trunc nuw nsw i64 %84 to i32
  %86 = urem i32 %85, 20
  %87 = icmp eq i32 %86, 0
  br i1 %87, label %88, label %._crit_edge16

._crit_edge16:                                    ; preds = %83
  br label %91

88:                                               ; preds = %83
  %89 = load ptr, ptr @stderr, align 8, !tbaa !21
  %90 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %89, ptr noundef nonnull @.str.3) #4
  br label %91

91:                                               ; preds = %._crit_edge16, %88
  %92 = load ptr, ptr @stderr, align 8, !tbaa !21
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %.reload131 = load ptr, ptr %.reg2mem130, align 8
  %93 = getelementptr inbounds [240 x double], ptr %.reload131, i64 %.reload50, i64 %.reload44
  %94 = load double, ptr %93, align 8, !tbaa !5
  %95 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %92, ptr noundef nonnull @.str.4, double noundef %94) #4
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  %96 = add nuw nsw i64 %.reload43, 1
  store i64 %96, ptr %.reg2mem39, align 8
  %.reload41 = load i64, ptr %.reg2mem39, align 8
  %97 = icmp eq i64 %.reload41, 240
  br i1 %97, label %98, label %._crit_edge17, !llvm.loop !23

._crit_edge17:                                    ; preds = %91
  %.reload40 = load i64, ptr %.reg2mem39, align 8
  store i64 %.reload40, ptr %.reg2mem147, align 8
  br label %83

98:                                               ; preds = %91
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %99 = add nuw nsw i64 %.reload49, 1
  store i64 %99, ptr %.reg2mem36, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %100 = icmp eq i64 %.reload38, 240
  br i1 %100, label %101, label %._crit_edge18, !llvm.loop !24

._crit_edge18:                                    ; preds = %98
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  store i64 %.reload37, ptr %.reg2mem149, align 8
  br label %81

101:                                              ; preds = %98
  %102 = load ptr, ptr @stderr, align 8, !tbaa !21
  %103 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %102, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %104 = load ptr, ptr @stderr, align 8, !tbaa !21
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %104, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  store i64 0, ptr %.reg2mem145, align 8
  br label %106

106:                                              ; preds = %._crit_edge21, %101
  %.reload146 = load i64, ptr %.reg2mem145, align 8
  store i64 %.reload146, ptr %.reg2mem32, align 8
  %.reload35 = load i64, ptr %.reg2mem32, align 8
  %107 = mul nuw nsw i64 %.reload35, 240
  store i64 %107, ptr %.reg2mem30, align 8
  store i64 0, ptr %.reg2mem143, align 8
  br label %108

108:                                              ; preds = %._crit_edge20, %106
  %.reload144 = load i64, ptr %.reg2mem143, align 8
  store i64 %.reload144, ptr %.reg2mem26, align 8
  %.reload29 = load i64, ptr %.reg2mem26, align 8
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  %109 = add nuw nsw i64 %.reload29, %.reload31
  %110 = trunc nuw nsw i64 %109 to i32
  %111 = urem i32 %110, 20
  %112 = icmp eq i32 %111, 0
  br i1 %112, label %113, label %._crit_edge19

._crit_edge19:                                    ; preds = %108
  br label %116

113:                                              ; preds = %108
  %114 = load ptr, ptr @stderr, align 8, !tbaa !21
  %115 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %114, ptr noundef nonnull @.str.3) #4
  br label %116

116:                                              ; preds = %._crit_edge19, %113
  %117 = load ptr, ptr @stderr, align 8, !tbaa !21
  %.reload28 = load i64, ptr %.reg2mem26, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %.reload124 = load ptr, ptr %.reg2mem123, align 8
  %118 = getelementptr inbounds [240 x double], ptr %.reload124, i64 %.reload34, i64 %.reload28
  %119 = load double, ptr %118, align 8, !tbaa !5
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %117, ptr noundef nonnull @.str.4, double noundef %119) #4
  %.reload27 = load i64, ptr %.reg2mem26, align 8
  %121 = add nuw nsw i64 %.reload27, 1
  store i64 %121, ptr %.reg2mem23, align 8
  %.reload25 = load i64, ptr %.reg2mem23, align 8
  %122 = icmp eq i64 %.reload25, 240
  br i1 %122, label %123, label %._crit_edge20, !llvm.loop !25

._crit_edge20:                                    ; preds = %116
  %.reload24 = load i64, ptr %.reg2mem23, align 8
  store i64 %.reload24, ptr %.reg2mem143, align 8
  br label %108

123:                                              ; preds = %116
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  %124 = add nuw nsw i64 %.reload33, 1
  store i64 %124, ptr %.reg2mem, align 8
  %.reload22 = load i64, ptr %.reg2mem, align 8
  %125 = icmp eq i64 %.reload22, 200
  br i1 %125, label %126, label %._crit_edge21, !llvm.loop !26

._crit_edge21:                                    ; preds = %123
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem145, align 8
  br label %106

126:                                              ; preds = %123
  %127 = load ptr, ptr @stderr, align 8, !tbaa !21
  %128 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %127, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %129 = load ptr, ptr @stderr, align 8, !tbaa !21
  %130 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %129, ptr noundef nonnull @.str.7) #4
  %.reload142 = load ptr, ptr %.reg2mem136, align 8
  tail call void @free(ptr noundef %.reload142) #4
  %.reload135 = load ptr, ptr %.reg2mem130, align 8
  tail call void @free(ptr noundef %.reload135) #4
  %.reload129 = load ptr, ptr %.reg2mem123, align 8
  tail call void @free(ptr noundef nonnull %.reload129) #4
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
!21 = !{!22, !22, i64 0}
!22 = !{!"any pointer", !7, i64 0}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}
!26 = distinct !{!26, !10, !11}

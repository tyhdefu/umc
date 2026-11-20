; ModuleID = './polybench-output/2mm-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/2mm/2mm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"D\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

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
  %.reg2mem45 = alloca double, align 8
  %.reg2mem48 = alloca double, align 8
  %.reg2mem51 = alloca ptr, align 8
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
  %.reg2mem96 = alloca i64, align 8
  %.reg2mem99 = alloca i64, align 8
  %.reg2mem102 = alloca i64, align 8
  %.reg2mem106 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem113 = alloca i64, align 8
  %.reg2mem117 = alloca i64, align 8
  %.reg2mem120 = alloca i64, align 8
  %.reg2mem123 = alloca i64, align 8
  %.reg2mem127 = alloca ptr, align 8
  %.reg2mem132 = alloca ptr, align 8
  %.reg2mem136 = alloca ptr, align 8
  %.reg2mem140 = alloca ptr, align 8
  %.reg2mem144 = alloca ptr, align 8
  %.reg2mem148 = alloca i64, align 8
  %.reg2mem150 = alloca i64, align 8
  %.reg2mem152 = alloca double, align 8
  %.reg2mem154 = alloca i64, align 8
  %.reg2mem156 = alloca i64, align 8
  %.reg2mem158 = alloca i64, align 8
  %.reg2mem160 = alloca double, align 8
  %.reg2mem162 = alloca i64, align 8
  %.reg2mem164 = alloca i64, align 8
  %.reg2mem166 = alloca i64, align 8
  %.reg2mem168 = alloca i64, align 8
  %.reg2mem170 = alloca i64, align 8
  %.reg2mem172 = alloca i64, align 8
  %.reg2mem174 = alloca i64, align 8
  %.reg2mem176 = alloca i64, align 8
  %.reg2mem178 = alloca i64, align 8
  %.reg2mem180 = alloca i64, align 8
  %.reg2mem182 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 34200, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem144, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 37800, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem140, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 39900, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem136, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 41800, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem132, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 39600, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem127, align 8
  store i64 0, ptr %.reg2mem182, align 8
  br label %8

8:                                                ; preds = %._crit_edge2, %2
  %.reload183 = load i64, ptr %.reg2mem182, align 8
  store i64 %.reload183, ptr %.reg2mem123, align 8
  store i64 0, ptr %.reg2mem180, align 8
  br label %9

9:                                                ; preds = %._crit_edge, %8
  %.reload181 = load i64, ptr %.reg2mem180, align 8
  %.reload126 = load i64, ptr %.reg2mem123, align 8
  %10 = mul nuw nsw i64 %.reload181, %.reload126
  %11 = trunc i64 %10 to i32
  %12 = add i32 %11, 1
  %13 = urem i32 %12, 180
  %14 = uitofp nneg i32 %13 to double
  %15 = fdiv double %14, 1.800000e+02
  %.reload125 = load i64, ptr %.reg2mem123, align 8
  %.reload142 = load ptr, ptr %.reg2mem140, align 8
  %16 = getelementptr inbounds [210 x double], ptr %.reload142, i64 %.reload125, i64 %.reload181
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = add nuw nsw i64 %.reload181, 1
  store i64 %17, ptr %.reg2mem120, align 8
  %.reload122 = load i64, ptr %.reg2mem120, align 8
  %18 = icmp eq i64 %.reload122, 210
  br i1 %18, label %19, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %9
  %.reload121 = load i64, ptr %.reg2mem120, align 8
  store i64 %.reload121, ptr %.reg2mem180, align 8
  br label %9

19:                                               ; preds = %9
  %.reload124 = load i64, ptr %.reg2mem123, align 8
  %20 = add nuw nsw i64 %.reload124, 1
  store i64 %20, ptr %.reg2mem117, align 8
  %.reload119 = load i64, ptr %.reg2mem117, align 8
  %21 = icmp eq i64 %.reload119, 180
  br i1 %21, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %19
  %.reload118 = load i64, ptr %.reg2mem117, align 8
  store i64 %.reload118, ptr %.reg2mem182, align 8
  br label %8

._crit_edge1:                                     ; preds = %19
  store i64 0, ptr %.reg2mem178, align 8
  br label %22

22:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload179 = load i64, ptr %.reg2mem178, align 8
  store i64 %.reload179, ptr %.reg2mem113, align 8
  store i64 0, ptr %.reg2mem176, align 8
  br label %23

23:                                               ; preds = %._crit_edge3, %22
  %.reload177 = load i64, ptr %.reg2mem176, align 8
  %24 = add nuw nsw i64 %.reload177, 1
  store i64 %24, ptr %.reg2mem109, align 8
  %.reload112 = load i64, ptr %.reg2mem109, align 8
  %.reload116 = load i64, ptr %.reg2mem113, align 8
  %25 = mul nuw nsw i64 %.reload112, %.reload116
  %26 = trunc nuw nsw i64 %25 to i32
  %27 = urem i32 %26, 190
  %28 = uitofp nneg i32 %27 to double
  %29 = fdiv double %28, 1.900000e+02
  %.reload115 = load i64, ptr %.reg2mem113, align 8
  %.reload138 = load ptr, ptr %.reg2mem136, align 8
  %30 = getelementptr inbounds [190 x double], ptr %.reload138, i64 %.reload115, i64 %.reload177
  store double %29, ptr %30, align 8, !tbaa !5
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %31 = icmp eq i64 %.reload111, 190
  br i1 %31, label %32, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %23
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  store i64 %.reload110, ptr %.reg2mem176, align 8
  br label %23

32:                                               ; preds = %23
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  %33 = add nuw nsw i64 %.reload114, 1
  store i64 %33, ptr %.reg2mem106, align 8
  %.reload108 = load i64, ptr %.reg2mem106, align 8
  %34 = icmp eq i64 %.reload108, 210
  br i1 %34, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %32
  %.reload107 = load i64, ptr %.reg2mem106, align 8
  store i64 %.reload107, ptr %.reg2mem178, align 8
  br label %22

._crit_edge4:                                     ; preds = %32
  store i64 0, ptr %.reg2mem174, align 8
  br label %35

35:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload175 = load i64, ptr %.reg2mem174, align 8
  store i64 %.reload175, ptr %.reg2mem102, align 8
  store i64 0, ptr %.reg2mem172, align 8
  br label %36

36:                                               ; preds = %._crit_edge6, %35
  %.reload173 = load i64, ptr %.reg2mem172, align 8
  %37 = add nuw nsw i64 %.reload173, 3
  %.reload105 = load i64, ptr %.reg2mem102, align 8
  %38 = mul nuw nsw i64 %37, %.reload105
  %39 = trunc i64 %38 to i32
  %40 = add i32 %39, 1
  %41 = urem i32 %40, 220
  %42 = uitofp nneg i32 %41 to double
  %43 = fdiv double %42, 2.200000e+02
  %.reload104 = load i64, ptr %.reg2mem102, align 8
  %.reload134 = load ptr, ptr %.reg2mem132, align 8
  %44 = getelementptr inbounds [220 x double], ptr %.reload134, i64 %.reload104, i64 %.reload173
  store double %43, ptr %44, align 8, !tbaa !5
  %45 = add nuw nsw i64 %.reload173, 1
  store i64 %45, ptr %.reg2mem99, align 8
  %.reload101 = load i64, ptr %.reg2mem99, align 8
  %46 = icmp eq i64 %.reload101, 220
  br i1 %46, label %47, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %36
  %.reload100 = load i64, ptr %.reg2mem99, align 8
  store i64 %.reload100, ptr %.reg2mem172, align 8
  br label %36

47:                                               ; preds = %36
  %.reload103 = load i64, ptr %.reg2mem102, align 8
  %48 = add nuw nsw i64 %.reload103, 1
  store i64 %48, ptr %.reg2mem96, align 8
  %.reload98 = load i64, ptr %.reg2mem96, align 8
  %49 = icmp eq i64 %.reload98, 190
  br i1 %49, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %47
  %.reload97 = load i64, ptr %.reg2mem96, align 8
  store i64 %.reload97, ptr %.reg2mem174, align 8
  br label %35

._crit_edge7:                                     ; preds = %47
  store i64 0, ptr %.reg2mem170, align 8
  br label %50

50:                                               ; preds = %._crit_edge11, %._crit_edge7
  %.reload171 = load i64, ptr %.reg2mem170, align 8
  store i64 %.reload171, ptr %.reg2mem92, align 8
  store i64 0, ptr %.reg2mem168, align 8
  br label %51

51:                                               ; preds = %._crit_edge9, %50
  %.reload169 = load i64, ptr %.reg2mem168, align 8
  %52 = add nuw nsw i64 %.reload169, 2
  %.reload95 = load i64, ptr %.reg2mem92, align 8
  %53 = mul nuw nsw i64 %52, %.reload95
  %54 = trunc nuw nsw i64 %53 to i32
  %55 = urem i32 %54, 210
  %56 = uitofp nneg i32 %55 to double
  %57 = fdiv double %56, 2.100000e+02
  %.reload94 = load i64, ptr %.reg2mem92, align 8
  %.reload130 = load ptr, ptr %.reg2mem127, align 8
  %58 = getelementptr inbounds [220 x double], ptr %.reload130, i64 %.reload94, i64 %.reload169
  store double %57, ptr %58, align 8, !tbaa !5
  %59 = add nuw nsw i64 %.reload169, 1
  store i64 %59, ptr %.reg2mem89, align 8
  %.reload91 = load i64, ptr %.reg2mem89, align 8
  %60 = icmp eq i64 %.reload91, 220
  br i1 %60, label %61, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %51
  %.reload90 = load i64, ptr %.reg2mem89, align 8
  store i64 %.reload90, ptr %.reg2mem168, align 8
  br label %51

61:                                               ; preds = %51
  %.reload93 = load i64, ptr %.reg2mem92, align 8
  %62 = add nuw nsw i64 %.reload93, 1
  store i64 %62, ptr %.reg2mem86, align 8
  %.reload88 = load i64, ptr %.reg2mem86, align 8
  %63 = icmp eq i64 %.reload88, 180
  br i1 %63, label %._crit_edge10, label %._crit_edge11, !llvm.loop !18

._crit_edge11:                                    ; preds = %61
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  store i64 %.reload87, ptr %.reg2mem170, align 8
  br label %50

._crit_edge10:                                    ; preds = %61
  store i64 0, ptr %.reg2mem166, align 8
  br label %64

64:                                               ; preds = %._crit_edge15, %._crit_edge10
  %.reload167 = load i64, ptr %.reg2mem166, align 8
  store i64 %.reload167, ptr %.reg2mem82, align 8
  store i64 0, ptr %.reg2mem164, align 8
  br label %65

65:                                               ; preds = %._crit_edge13, %64
  %.reload165 = load i64, ptr %.reg2mem164, align 8
  store i64 %.reload165, ptr %.reg2mem78, align 8
  %.reload81 = load i64, ptr %.reg2mem78, align 8
  %.reload85 = load i64, ptr %.reg2mem82, align 8
  %.reload146 = load ptr, ptr %.reg2mem144, align 8
  %66 = getelementptr inbounds [190 x double], ptr %.reload146, i64 %.reload85, i64 %.reload81
  store ptr %66, ptr %.reg2mem75, align 8
  %.reload77 = load ptr, ptr %.reg2mem75, align 8
  store double 0.000000e+00, ptr %.reload77, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem160, align 8
  store i64 0, ptr %.reg2mem162, align 8
  br label %67

67:                                               ; preds = %._crit_edge12, %65
  %.reload163 = load i64, ptr %.reg2mem162, align 8
  %.reload161 = load double, ptr %.reg2mem160, align 8
  %.reload84 = load i64, ptr %.reg2mem82, align 8
  %.reload141 = load ptr, ptr %.reg2mem140, align 8
  %68 = getelementptr inbounds [210 x double], ptr %.reload141, i64 %.reload84, i64 %.reload163
  %69 = load double, ptr %68, align 8, !tbaa !5
  %70 = fmul double %69, 1.500000e+00
  %.reload80 = load i64, ptr %.reg2mem78, align 8
  %.reload137 = load ptr, ptr %.reg2mem136, align 8
  %71 = getelementptr inbounds [190 x double], ptr %.reload137, i64 %.reload163, i64 %.reload80
  %72 = load double, ptr %71, align 8, !tbaa !5
  %73 = tail call double @llvm.fmuladd.f64(double %70, double %72, double %.reload161)
  store double %73, ptr %.reg2mem72, align 8
  %.reload74 = load double, ptr %.reg2mem72, align 8
  %.reload76 = load ptr, ptr %.reg2mem75, align 8
  store double %.reload74, ptr %.reload76, align 8, !tbaa !5
  %74 = add nuw nsw i64 %.reload163, 1
  store i64 %74, ptr %.reg2mem69, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %75 = icmp eq i64 %.reload71, 210
  br i1 %75, label %76, label %._crit_edge12, !llvm.loop !19

._crit_edge12:                                    ; preds = %67
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %.reload73 = load double, ptr %.reg2mem72, align 8
  store double %.reload73, ptr %.reg2mem160, align 8
  store i64 %.reload70, ptr %.reg2mem162, align 8
  br label %67

76:                                               ; preds = %67
  %.reload79 = load i64, ptr %.reg2mem78, align 8
  %77 = add nuw nsw i64 %.reload79, 1
  store i64 %77, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %78 = icmp eq i64 %.reload68, 190
  br i1 %78, label %79, label %._crit_edge13, !llvm.loop !20

._crit_edge13:                                    ; preds = %76
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem164, align 8
  br label %65

79:                                               ; preds = %76
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %80 = add nuw nsw i64 %.reload83, 1
  store i64 %80, ptr %.reg2mem63, align 8
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %81 = icmp eq i64 %.reload65, 180
  br i1 %81, label %._crit_edge14, label %._crit_edge15, !llvm.loop !21

._crit_edge15:                                    ; preds = %79
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  store i64 %.reload64, ptr %.reg2mem166, align 8
  br label %64

._crit_edge14:                                    ; preds = %79
  store i64 0, ptr %.reg2mem158, align 8
  br label %82

82:                                               ; preds = %._crit_edge18, %._crit_edge14
  %.reload159 = load i64, ptr %.reg2mem158, align 8
  store i64 %.reload159, ptr %.reg2mem59, align 8
  store i64 0, ptr %.reg2mem156, align 8
  br label %83

83:                                               ; preds = %._crit_edge17, %82
  %.reload157 = load i64, ptr %.reg2mem156, align 8
  store i64 %.reload157, ptr %.reg2mem55, align 8
  %.reload58 = load i64, ptr %.reg2mem55, align 8
  %.reload62 = load i64, ptr %.reg2mem59, align 8
  %.reload129 = load ptr, ptr %.reg2mem127, align 8
  %84 = getelementptr inbounds [220 x double], ptr %.reload129, i64 %.reload62, i64 %.reload58
  store ptr %84, ptr %.reg2mem51, align 8
  %.reload54 = load ptr, ptr %.reg2mem51, align 8
  %85 = load double, ptr %.reload54, align 8, !tbaa !5
  %86 = fmul double %85, 1.200000e+00
  store double %86, ptr %.reg2mem48, align 8
  %.reload50 = load double, ptr %.reg2mem48, align 8
  %.reload53 = load ptr, ptr %.reg2mem51, align 8
  store double %.reload50, ptr %.reload53, align 8, !tbaa !5
  %.reload49 = load double, ptr %.reg2mem48, align 8
  store double %.reload49, ptr %.reg2mem152, align 8
  store i64 0, ptr %.reg2mem154, align 8
  br label %87

87:                                               ; preds = %._crit_edge16, %83
  %.reload155 = load i64, ptr %.reg2mem154, align 8
  %.reload153 = load double, ptr %.reg2mem152, align 8
  %.reload61 = load i64, ptr %.reg2mem59, align 8
  %.reload145 = load ptr, ptr %.reg2mem144, align 8
  %88 = getelementptr inbounds [190 x double], ptr %.reload145, i64 %.reload61, i64 %.reload155
  %89 = load double, ptr %88, align 8, !tbaa !5
  %.reload57 = load i64, ptr %.reg2mem55, align 8
  %.reload133 = load ptr, ptr %.reg2mem132, align 8
  %90 = getelementptr inbounds [220 x double], ptr %.reload133, i64 %.reload155, i64 %.reload57
  %91 = load double, ptr %90, align 8, !tbaa !5
  %92 = tail call double @llvm.fmuladd.f64(double %89, double %91, double %.reload153)
  store double %92, ptr %.reg2mem45, align 8
  %.reload47 = load double, ptr %.reg2mem45, align 8
  %.reload52 = load ptr, ptr %.reg2mem51, align 8
  store double %.reload47, ptr %.reload52, align 8, !tbaa !5
  %93 = add nuw nsw i64 %.reload155, 1
  store i64 %93, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %94 = icmp eq i64 %.reload44, 190
  br i1 %94, label %95, label %._crit_edge16, !llvm.loop !22

._crit_edge16:                                    ; preds = %87
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  %.reload46 = load double, ptr %.reg2mem45, align 8
  store double %.reload46, ptr %.reg2mem152, align 8
  store i64 %.reload43, ptr %.reg2mem154, align 8
  br label %87

95:                                               ; preds = %87
  %.reload56 = load i64, ptr %.reg2mem55, align 8
  %96 = add nuw nsw i64 %.reload56, 1
  store i64 %96, ptr %.reg2mem39, align 8
  %.reload41 = load i64, ptr %.reg2mem39, align 8
  %97 = icmp eq i64 %.reload41, 220
  br i1 %97, label %98, label %._crit_edge17, !llvm.loop !23

._crit_edge17:                                    ; preds = %95
  %.reload40 = load i64, ptr %.reg2mem39, align 8
  store i64 %.reload40, ptr %.reg2mem156, align 8
  br label %83

98:                                               ; preds = %95
  %.reload60 = load i64, ptr %.reg2mem59, align 8
  %99 = add nuw nsw i64 %.reload60, 1
  store i64 %99, ptr %.reg2mem36, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %100 = icmp eq i64 %.reload38, 180
  br i1 %100, label %101, label %._crit_edge18, !llvm.loop !24

._crit_edge18:                                    ; preds = %98
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  store i64 %.reload37, ptr %.reg2mem158, align 8
  br label %82

101:                                              ; preds = %98
  %102 = load ptr, ptr @stderr, align 8, !tbaa !25
  %103 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %102, ptr noundef nonnull @.str) #4
  %104 = load ptr, ptr @stderr, align 8, !tbaa !25
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %104, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem150, align 8
  br label %106

106:                                              ; preds = %._crit_edge21, %101
  %.reload151 = load i64, ptr %.reg2mem150, align 8
  store i64 %.reload151, ptr %.reg2mem32, align 8
  %.reload35 = load i64, ptr %.reg2mem32, align 8
  %107 = mul nuw nsw i64 %.reload35, 180
  store i64 %107, ptr %.reg2mem30, align 8
  store i64 0, ptr %.reg2mem148, align 8
  br label %108

108:                                              ; preds = %._crit_edge20, %106
  %.reload149 = load i64, ptr %.reg2mem148, align 8
  store i64 %.reload149, ptr %.reg2mem26, align 8
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
  %114 = load ptr, ptr @stderr, align 8, !tbaa !25
  %115 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %114, ptr noundef nonnull @.str.3) #4
  br label %116

116:                                              ; preds = %._crit_edge19, %113
  %117 = load ptr, ptr @stderr, align 8, !tbaa !25
  %.reload28 = load i64, ptr %.reg2mem26, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %.reload128 = load ptr, ptr %.reg2mem127, align 8
  %118 = getelementptr inbounds [220 x double], ptr %.reload128, i64 %.reload34, i64 %.reload28
  %119 = load double, ptr %118, align 8, !tbaa !5
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %117, ptr noundef nonnull @.str.4, double noundef %119) #4
  %.reload27 = load i64, ptr %.reg2mem26, align 8
  %121 = add nuw nsw i64 %.reload27, 1
  store i64 %121, ptr %.reg2mem23, align 8
  %.reload25 = load i64, ptr %.reg2mem23, align 8
  %122 = icmp eq i64 %.reload25, 220
  br i1 %122, label %123, label %._crit_edge20, !llvm.loop !27

._crit_edge20:                                    ; preds = %116
  %.reload24 = load i64, ptr %.reg2mem23, align 8
  store i64 %.reload24, ptr %.reg2mem148, align 8
  br label %108

123:                                              ; preds = %116
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  %124 = add nuw nsw i64 %.reload33, 1
  store i64 %124, ptr %.reg2mem, align 8
  %.reload22 = load i64, ptr %.reg2mem, align 8
  %125 = icmp eq i64 %.reload22, 180
  br i1 %125, label %126, label %._crit_edge21, !llvm.loop !28

._crit_edge21:                                    ; preds = %123
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem150, align 8
  br label %106

126:                                              ; preds = %123
  %127 = load ptr, ptr @stderr, align 8, !tbaa !25
  %128 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %127, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %129 = load ptr, ptr @stderr, align 8, !tbaa !25
  %130 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %129, ptr noundef nonnull @.str.6) #4
  %.reload147 = load ptr, ptr %.reg2mem144, align 8
  tail call void @free(ptr noundef %.reload147) #4
  %.reload143 = load ptr, ptr %.reg2mem140, align 8
  tail call void @free(ptr noundef %.reload143) #4
  %.reload139 = load ptr, ptr %.reg2mem136, align 8
  tail call void @free(ptr noundef %.reload139) #4
  %.reload135 = load ptr, ptr %.reg2mem132, align 8
  tail call void @free(ptr noundef %.reload135) #4
  %.reload131 = load ptr, ptr %.reg2mem127, align 8
  tail call void @free(ptr noundef nonnull %.reload131) #4
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
!25 = !{!26, !26, i64 0}
!26 = !{!"any pointer", !7, i64 0}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}

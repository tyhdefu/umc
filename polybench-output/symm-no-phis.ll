; ModuleID = './polybench-output/symm-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/symm/symm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"C\00", align 1
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
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem41 = alloca double, align 8
  %.reg2mem44 = alloca ptr, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem53 = alloca ptr, align 8
  %.reg2mem55 = alloca i1, align 1
  %.reg2mem57 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem79 = alloca i64, align 8
  %.reg2mem85 = alloca i64, align 8
  %.reg2mem87 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem93 = alloca i64, align 8
  %.reg2mem95 = alloca i64, align 8
  %.reg2mem101 = alloca ptr, align 8
  %.reg2mem107 = alloca ptr, align 8
  %.reg2mem113 = alloca ptr, align 8
  %.reg2mem119 = alloca i64, align 8
  %.reg2mem121 = alloca i64, align 8
  %.reg2mem123 = alloca double, align 8
  %.reg2mem125 = alloca double, align 8
  %.reg2mem127 = alloca i64, align 8
  %.reg2mem129 = alloca i64, align 8
  %.reg2mem131 = alloca i64, align 8
  %.reg2mem133 = alloca i64, align 8
  %.reg2mem135 = alloca i64, align 8
  %.reg2mem137 = alloca i64, align 8
  %.reg2mem139 = alloca i64, align 8
  %.reg2mem141 = alloca i64, align 8
  %.reg2mem143 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem113, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 40000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem107, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem101, align 8
  store i64 0, ptr %.reg2mem143, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload144 = load i64, ptr %.reg2mem143, align 8
  store i64 %.reload144, ptr %.reg2mem95, align 8
  %.reload100 = load i64, ptr %.reg2mem95, align 8
  %7 = add nuw nsw i64 %.reload100, 240
  store i64 %7, ptr %.reg2mem93, align 8
  store i64 0, ptr %.reg2mem141, align 8
  br label %8

8:                                                ; preds = %._crit_edge, %6
  %.reload142 = load i64, ptr %.reg2mem141, align 8
  %.reload99 = load i64, ptr %.reg2mem95, align 8
  %9 = add nuw nsw i64 %.reload142, %.reload99
  %10 = trunc nuw nsw i64 %9 to i32
  %11 = urem i32 %10, 100
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 2.000000e+02
  %.reload98 = load i64, ptr %.reg2mem95, align 8
  %.reload117 = load ptr, ptr %.reg2mem113, align 8
  %14 = getelementptr inbounds [240 x double], ptr %.reload117, i64 %.reload98, i64 %.reload142
  store double %13, ptr %14, align 8, !tbaa !5
  %.reload94 = load i64, ptr %.reg2mem93, align 8
  %15 = sub nuw nsw i64 %.reload94, %.reload142
  %16 = trunc nuw nsw i64 %15 to i32
  %17 = urem i32 %16, 100
  %18 = uitofp nneg i32 %17 to double
  %19 = fdiv double %18, 2.000000e+02
  %.reload97 = load i64, ptr %.reg2mem95, align 8
  %.reload105 = load ptr, ptr %.reg2mem101, align 8
  %20 = getelementptr inbounds [240 x double], ptr %.reload105, i64 %.reload97, i64 %.reload142
  store double %19, ptr %20, align 8, !tbaa !5
  %21 = add nuw nsw i64 %.reload142, 1
  store i64 %21, ptr %.reg2mem90, align 8
  %.reload92 = load i64, ptr %.reg2mem90, align 8
  %22 = icmp eq i64 %.reload92, 240
  br i1 %22, label %23, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %8
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  store i64 %.reload91, ptr %.reg2mem141, align 8
  br label %8

23:                                               ; preds = %8
  %.reload96 = load i64, ptr %.reg2mem95, align 8
  %24 = add nuw nsw i64 %.reload96, 1
  store i64 %24, ptr %.reg2mem87, align 8
  %.reload89 = load i64, ptr %.reg2mem87, align 8
  %25 = icmp eq i64 %.reload89, 200
  br i1 %25, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %23
  %.reload88 = load i64, ptr %.reg2mem87, align 8
  store i64 %.reload88, ptr %.reg2mem143, align 8
  br label %6

._crit_edge1:                                     ; preds = %23
  store i64 1, ptr %.reg2mem137, align 8
  store i64 0, ptr %.reg2mem139, align 8
  br label %29

26:                                               ; preds = %._crit_edge8, %._crit_edge7
  %.reload77 = load i64, ptr %.reg2mem75, align 8
  %27 = add nuw nsw i64 %.reload77, 1
  store i64 %27, ptr %.reg2mem85, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %28 = icmp eq i64 %.reload71, 200
  br i1 %28, label %._crit_edge3, label %._crit_edge4, !llvm.loop !13

._crit_edge4:                                     ; preds = %26
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %.reload86 = load i64, ptr %.reg2mem85, align 8
  store i64 %.reload86, ptr %.reg2mem137, align 8
  store i64 %.reload70, ptr %.reg2mem139, align 8
  br label %29

._crit_edge3:                                     ; preds = %26
  store i64 0, ptr %.reg2mem131, align 8
  br label %46

29:                                               ; preds = %._crit_edge4, %._crit_edge1
  %.reload140 = load i64, ptr %.reg2mem139, align 8
  %.reload138 = load i64, ptr %.reg2mem137, align 8
  store i64 %.reload140, ptr %.reg2mem79, align 8
  store i64 %.reload138, ptr %.reg2mem75, align 8
  store i64 0, ptr %.reg2mem135, align 8
  br label %30

30:                                               ; preds = %._crit_edge5, %29
  %.reload136 = load i64, ptr %.reg2mem135, align 8
  %.reload84 = load i64, ptr %.reg2mem79, align 8
  %31 = add nuw nsw i64 %.reload136, %.reload84
  %32 = trunc nuw nsw i64 %31 to i32
  %33 = urem i32 %32, 100
  %34 = uitofp nneg i32 %33 to double
  %35 = fdiv double %34, 2.000000e+02
  %.reload83 = load i64, ptr %.reg2mem79, align 8
  %.reload111 = load ptr, ptr %.reg2mem107, align 8
  %36 = getelementptr inbounds [200 x double], ptr %.reload111, i64 %.reload83, i64 %.reload136
  store double %35, ptr %36, align 8, !tbaa !5
  %37 = add nuw nsw i64 %.reload136, 1
  store i64 %37, ptr %.reg2mem72, align 8
  %.reload74 = load i64, ptr %.reg2mem72, align 8
  %.reload78 = load i64, ptr %.reg2mem75, align 8
  %38 = icmp eq i64 %.reload74, %.reload78
  br i1 %38, label %39, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %30
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem135, align 8
  br label %30

39:                                               ; preds = %30
  %.reload82 = load i64, ptr %.reg2mem79, align 8
  %40 = add nuw nsw i64 %.reload82, 1
  store i64 %40, ptr %.reg2mem69, align 8
  %.reload81 = load i64, ptr %.reg2mem79, align 8
  %41 = icmp ult i64 %.reload81, 199
  br i1 %41, label %._crit_edge6, label %._crit_edge7

._crit_edge7:                                     ; preds = %39
  br label %26

._crit_edge6:                                     ; preds = %39
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  store i64 %.reload76, ptr %.reg2mem133, align 8
  br label %42

42:                                               ; preds = %._crit_edge9, %._crit_edge6
  %.reload134 = load i64, ptr %.reg2mem133, align 8
  %.reload80 = load i64, ptr %.reg2mem79, align 8
  %.reload110 = load ptr, ptr %.reg2mem107, align 8
  %43 = getelementptr inbounds [200 x double], ptr %.reload110, i64 %.reload80, i64 %.reload134
  store double -9.990000e+02, ptr %43, align 8, !tbaa !5
  %44 = add nuw nsw i64 %.reload134, 1
  store i64 %44, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %45 = icmp eq i64 %.reload68, 200
  br i1 %45, label %._crit_edge8, label %._crit_edge9, !llvm.loop !15

._crit_edge9:                                     ; preds = %42
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem133, align 8
  br label %42

._crit_edge8:                                     ; preds = %42
  br label %26

46:                                               ; preds = %._crit_edge14, %._crit_edge3
  %.reload132 = load i64, ptr %.reg2mem131, align 8
  store i64 %.reload132, ptr %.reg2mem57, align 8
  %.reload65 = load i64, ptr %.reg2mem57, align 8
  %47 = icmp eq i64 %.reload65, 0
  store i1 %47, ptr %.reg2mem55, align 1
  %.reload64 = load i64, ptr %.reg2mem57, align 8
  %.reload109 = load ptr, ptr %.reg2mem107, align 8
  %48 = getelementptr inbounds [200 x double], ptr %.reload109, i64 %.reload64, i64 %.reload64
  store ptr %48, ptr %.reg2mem53, align 8
  store i64 0, ptr %.reg2mem129, align 8
  br label %49

49:                                               ; preds = %._crit_edge13, %46
  %.reload130 = load i64, ptr %.reg2mem129, align 8
  store i64 %.reload130, ptr %.reg2mem46, align 8
  %.reload56 = load i1, ptr %.reg2mem55, align 1
  br i1 %.reload56, label %._crit_edge10, label %50

._crit_edge10:                                    ; preds = %49
  store double 0.000000e+00, ptr %.reg2mem123, align 8
  br label %66

50:                                               ; preds = %49
  %.reload52 = load i64, ptr %.reg2mem46, align 8
  %.reload63 = load i64, ptr %.reg2mem57, align 8
  %.reload104 = load ptr, ptr %.reg2mem101, align 8
  %51 = getelementptr inbounds [240 x double], ptr %.reload104, i64 %.reload63, i64 %.reload52
  store ptr %51, ptr %.reg2mem44, align 8
  store double 0.000000e+00, ptr %.reg2mem125, align 8
  store i64 0, ptr %.reg2mem127, align 8
  br label %52

52:                                               ; preds = %._crit_edge12, %50
  %.reload128 = load i64, ptr %.reg2mem127, align 8
  %.reload126 = load double, ptr %.reg2mem125, align 8
  %.reload45 = load ptr, ptr %.reg2mem44, align 8
  %53 = load double, ptr %.reload45, align 8, !tbaa !5
  %54 = fmul double %53, 1.500000e+00
  %.reload62 = load i64, ptr %.reg2mem57, align 8
  %.reload108 = load ptr, ptr %.reg2mem107, align 8
  %55 = getelementptr inbounds [200 x double], ptr %.reload108, i64 %.reload62, i64 %.reload128
  %56 = load double, ptr %55, align 8, !tbaa !5
  %.reload51 = load i64, ptr %.reg2mem46, align 8
  %.reload116 = load ptr, ptr %.reg2mem113, align 8
  %57 = getelementptr inbounds [240 x double], ptr %.reload116, i64 %.reload128, i64 %.reload51
  %58 = load double, ptr %57, align 8, !tbaa !5
  %59 = tail call double @llvm.fmuladd.f64(double %54, double %56, double %58)
  store double %59, ptr %57, align 8, !tbaa !5
  %.reload50 = load i64, ptr %.reg2mem46, align 8
  %.reload103 = load ptr, ptr %.reg2mem101, align 8
  %60 = getelementptr inbounds [240 x double], ptr %.reload103, i64 %.reload128, i64 %.reload50
  %61 = load double, ptr %60, align 8, !tbaa !5
  %62 = load double, ptr %55, align 8, !tbaa !5
  %63 = tail call double @llvm.fmuladd.f64(double %61, double %62, double %.reload126)
  store double %63, ptr %.reg2mem41, align 8
  %64 = add nuw nsw i64 %.reload128, 1
  store i64 %64, ptr %.reg2mem38, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %.reload61 = load i64, ptr %.reg2mem57, align 8
  %65 = icmp eq i64 %.reload40, %.reload61
  br i1 %65, label %._crit_edge11, label %._crit_edge12, !llvm.loop !16

._crit_edge12:                                    ; preds = %52
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  %.reload43 = load double, ptr %.reg2mem41, align 8
  store double %.reload43, ptr %.reg2mem125, align 8
  store i64 %.reload39, ptr %.reg2mem127, align 8
  br label %52

._crit_edge11:                                    ; preds = %52
  %.reload42 = load double, ptr %.reg2mem41, align 8
  store double %.reload42, ptr %.reg2mem123, align 8
  br label %66

66:                                               ; preds = %._crit_edge11, %._crit_edge10
  %.reload124 = load double, ptr %.reg2mem123, align 8
  %.reload49 = load i64, ptr %.reg2mem46, align 8
  %.reload60 = load i64, ptr %.reg2mem57, align 8
  %.reload115 = load ptr, ptr %.reg2mem113, align 8
  %67 = getelementptr inbounds [240 x double], ptr %.reload115, i64 %.reload60, i64 %.reload49
  %68 = load double, ptr %67, align 8, !tbaa !5
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %.reload59 = load i64, ptr %.reg2mem57, align 8
  %.reload102 = load ptr, ptr %.reg2mem101, align 8
  %69 = getelementptr inbounds [240 x double], ptr %.reload102, i64 %.reload59, i64 %.reload48
  %70 = load double, ptr %69, align 8, !tbaa !5
  %71 = fmul double %70, 1.500000e+00
  %.reload54 = load ptr, ptr %.reg2mem53, align 8
  %72 = load double, ptr %.reload54, align 8, !tbaa !5
  %73 = fmul double %71, %72
  %74 = tail call double @llvm.fmuladd.f64(double %68, double 1.200000e+00, double %73)
  %75 = tail call double @llvm.fmuladd.f64(double %.reload124, double 1.500000e+00, double %74)
  store double %75, ptr %67, align 8, !tbaa !5
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %76 = add nuw nsw i64 %.reload47, 1
  store i64 %76, ptr %.reg2mem35, align 8
  %.reload37 = load i64, ptr %.reg2mem35, align 8
  %77 = icmp eq i64 %.reload37, 240
  br i1 %77, label %78, label %._crit_edge13, !llvm.loop !17

._crit_edge13:                                    ; preds = %66
  %.reload36 = load i64, ptr %.reg2mem35, align 8
  store i64 %.reload36, ptr %.reg2mem129, align 8
  br label %49

78:                                               ; preds = %66
  %.reload58 = load i64, ptr %.reg2mem57, align 8
  %79 = add nuw nsw i64 %.reload58, 1
  store i64 %79, ptr %.reg2mem32, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %80 = icmp eq i64 %.reload34, 200
  br i1 %80, label %81, label %._crit_edge14, !llvm.loop !18

._crit_edge14:                                    ; preds = %78
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  store i64 %.reload33, ptr %.reg2mem131, align 8
  br label %46

81:                                               ; preds = %78
  %82 = load ptr, ptr @stderr, align 8, !tbaa !19
  %83 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %82, ptr noundef nonnull @.str) #4
  %84 = load ptr, ptr @stderr, align 8, !tbaa !19
  %85 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %84, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem121, align 8
  br label %86

86:                                               ; preds = %._crit_edge17, %81
  %.reload122 = load i64, ptr %.reg2mem121, align 8
  store i64 %.reload122, ptr %.reg2mem28, align 8
  %.reload31 = load i64, ptr %.reg2mem28, align 8
  %87 = mul nuw nsw i64 %.reload31, 200
  store i64 %87, ptr %.reg2mem26, align 8
  store i64 0, ptr %.reg2mem119, align 8
  br label %88

88:                                               ; preds = %._crit_edge16, %86
  %.reload120 = load i64, ptr %.reg2mem119, align 8
  store i64 %.reload120, ptr %.reg2mem22, align 8
  %.reload25 = load i64, ptr %.reg2mem22, align 8
  %.reload27 = load i64, ptr %.reg2mem26, align 8
  %89 = add nuw nsw i64 %.reload25, %.reload27
  %90 = trunc nuw nsw i64 %89 to i32
  %91 = urem i32 %90, 20
  %92 = icmp eq i32 %91, 0
  br i1 %92, label %93, label %._crit_edge15

._crit_edge15:                                    ; preds = %88
  br label %96

93:                                               ; preds = %88
  %94 = load ptr, ptr @stderr, align 8, !tbaa !19
  %95 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %94, ptr noundef nonnull @.str.3) #4
  br label %96

96:                                               ; preds = %._crit_edge15, %93
  %97 = load ptr, ptr @stderr, align 8, !tbaa !19
  %.reload24 = load i64, ptr %.reg2mem22, align 8
  %.reload30 = load i64, ptr %.reg2mem28, align 8
  %.reload114 = load ptr, ptr %.reg2mem113, align 8
  %98 = getelementptr inbounds [240 x double], ptr %.reload114, i64 %.reload30, i64 %.reload24
  %99 = load double, ptr %98, align 8, !tbaa !5
  %100 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %97, ptr noundef nonnull @.str.4, double noundef %99) #4
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  %101 = add nuw nsw i64 %.reload23, 1
  store i64 %101, ptr %.reg2mem19, align 8
  %.reload21 = load i64, ptr %.reg2mem19, align 8
  %102 = icmp eq i64 %.reload21, 240
  br i1 %102, label %103, label %._crit_edge16, !llvm.loop !21

._crit_edge16:                                    ; preds = %96
  %.reload20 = load i64, ptr %.reg2mem19, align 8
  store i64 %.reload20, ptr %.reg2mem119, align 8
  br label %88

103:                                              ; preds = %96
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  %104 = add nuw nsw i64 %.reload29, 1
  store i64 %104, ptr %.reg2mem, align 8
  %.reload18 = load i64, ptr %.reg2mem, align 8
  %105 = icmp eq i64 %.reload18, 200
  br i1 %105, label %106, label %._crit_edge17, !llvm.loop !22

._crit_edge17:                                    ; preds = %103
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem121, align 8
  br label %86

106:                                              ; preds = %103
  %107 = load ptr, ptr @stderr, align 8, !tbaa !19
  %108 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %107, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %109 = load ptr, ptr @stderr, align 8, !tbaa !19
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str.6) #4
  %.reload118 = load ptr, ptr %.reg2mem113, align 8
  tail call void @free(ptr noundef nonnull %.reload118) #4
  %.reload112 = load ptr, ptr %.reg2mem107, align 8
  tail call void @free(ptr noundef %.reload112) #4
  %.reload106 = load ptr, ptr %.reg2mem101, align 8
  tail call void @free(ptr noundef %.reload106) #4
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
!19 = !{!20, !20, i64 0}
!20 = !{!"any pointer", !7, i64 0}
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}

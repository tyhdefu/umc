; ModuleID = '/home/lepton/PolyBenchC-4.2.1/datamining/covariance/covariance.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/datamining/covariance/covariance.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"cov\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62400, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %17, %2
  %7 = phi i64 [ 0, %2 ], [ %18, %17 ]
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi i64 [ 0, %6 ], [ %15, %8 ]
  %10 = mul nuw nsw i64 %9, %7
  %11 = trunc nuw nsw i64 %10 to i32
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 2.400000e+02
  %14 = getelementptr inbounds [240 x double], ptr %3, i64 %7, i64 %9
  store double %13, ptr %14, align 8, !tbaa !5
  %15 = add nuw nsw i64 %9, 1
  %16 = icmp eq i64 %15, 240
  br i1 %16, label %17, label %8, !llvm.loop !9

17:                                               ; preds = %8
  %18 = add nuw nsw i64 %7, 1
  %19 = icmp eq i64 %18, 260
  br i1 %19, label %20, label %6, !llvm.loop !12

20:                                               ; preds = %17, %31
  %21 = phi i64 [ %33, %31 ], [ 0, %17 ]
  %22 = getelementptr inbounds double, ptr %5, i64 %21
  store double 0.000000e+00, ptr %22, align 8, !tbaa !5
  br label %23

23:                                               ; preds = %23, %20
  %24 = phi i64 [ 0, %20 ], [ %29, %23 ]
  %25 = phi double [ 0.000000e+00, %20 ], [ %28, %23 ]
  %26 = getelementptr inbounds [240 x double], ptr %3, i64 %24, i64 %21
  %27 = load double, ptr %26, align 8, !tbaa !5
  %28 = fadd double %25, %27
  store double %28, ptr %22, align 8, !tbaa !5
  %29 = add nuw nsw i64 %24, 1
  %30 = icmp eq i64 %29, 260
  br i1 %30, label %31, label %23, !llvm.loop !13

31:                                               ; preds = %23
  %32 = fdiv double %28, 2.600000e+02
  store double %32, ptr %22, align 8, !tbaa !5
  %33 = add nuw nsw i64 %21, 1
  %34 = icmp eq i64 %33, 240
  br i1 %34, label %35, label %20, !llvm.loop !14

35:                                               ; preds = %31, %46
  %36 = phi i64 [ %47, %46 ], [ 0, %31 ]
  br label %37

37:                                               ; preds = %37, %35
  %38 = phi i64 [ 0, %35 ], [ %44, %37 ]
  %39 = getelementptr inbounds double, ptr %5, i64 %38
  %40 = load double, ptr %39, align 8, !tbaa !5
  %41 = getelementptr inbounds [240 x double], ptr %3, i64 %36, i64 %38
  %42 = load double, ptr %41, align 8, !tbaa !5
  %43 = fsub double %42, %40
  store double %43, ptr %41, align 8, !tbaa !5
  %44 = add nuw nsw i64 %38, 1
  %45 = icmp eq i64 %44, 240
  br i1 %45, label %46, label %37, !llvm.loop !15

46:                                               ; preds = %37
  %47 = add nuw nsw i64 %36, 1
  %48 = icmp eq i64 %47, 260
  br i1 %48, label %49, label %35, !llvm.loop !16

49:                                               ; preds = %46, %69
  %50 = phi i64 [ %70, %69 ], [ 0, %46 ]
  br label %51

51:                                               ; preds = %64, %49
  %52 = phi i64 [ %50, %49 ], [ %67, %64 ]
  %53 = getelementptr inbounds [240 x double], ptr %4, i64 %50, i64 %52
  store double 0.000000e+00, ptr %53, align 8, !tbaa !5
  br label %54

54:                                               ; preds = %54, %51
  %55 = phi i64 [ 0, %51 ], [ %62, %54 ]
  %56 = phi double [ 0.000000e+00, %51 ], [ %61, %54 ]
  %57 = getelementptr inbounds [240 x double], ptr %3, i64 %55, i64 %50
  %58 = load double, ptr %57, align 8, !tbaa !5
  %59 = getelementptr inbounds [240 x double], ptr %3, i64 %55, i64 %52
  %60 = load double, ptr %59, align 8, !tbaa !5
  %61 = tail call double @llvm.fmuladd.f64(double %58, double %60, double %56)
  store double %61, ptr %53, align 8, !tbaa !5
  %62 = add nuw nsw i64 %55, 1
  %63 = icmp eq i64 %62, 260
  br i1 %63, label %64, label %54, !llvm.loop !17

64:                                               ; preds = %54
  %65 = fdiv double %61, 2.590000e+02
  store double %65, ptr %53, align 8, !tbaa !5
  %66 = getelementptr inbounds [240 x double], ptr %4, i64 %52, i64 %50
  store double %65, ptr %66, align 8, !tbaa !5
  %67 = add nuw nsw i64 %52, 1
  %68 = icmp eq i64 %67, 240
  br i1 %68, label %69, label %51, !llvm.loop !18

69:                                               ; preds = %64
  %70 = add nuw nsw i64 %50, 1
  %71 = icmp eq i64 %70, 240
  br i1 %71, label %72, label %49, !llvm.loop !19

72:                                               ; preds = %69
  %73 = load ptr, ptr @stderr, align 8, !tbaa !20
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str) #4
  %75 = load ptr, ptr @stderr, align 8, !tbaa !20
  %76 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %75, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %77

77:                                               ; preds = %96, %72
  %78 = phi i64 [ 0, %72 ], [ %97, %96 ]
  %79 = mul nuw nsw i64 %78, 240
  br label %80

80:                                               ; preds = %89, %77
  %81 = phi i64 [ 0, %77 ], [ %94, %89 ]
  %82 = add nuw nsw i64 %81, %79
  %83 = trunc nuw nsw i64 %82 to i32
  %84 = urem i32 %83, 20
  %85 = icmp eq i32 %84, 0
  br i1 %85, label %86, label %89

86:                                               ; preds = %80
  %87 = load ptr, ptr @stderr, align 8, !tbaa !20
  %88 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %87, ptr noundef nonnull @.str.3) #4
  br label %89

89:                                               ; preds = %86, %80
  %90 = load ptr, ptr @stderr, align 8, !tbaa !20
  %91 = getelementptr inbounds [240 x double], ptr %4, i64 %78, i64 %81
  %92 = load double, ptr %91, align 8, !tbaa !5
  %93 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %90, ptr noundef nonnull @.str.4, double noundef %92) #4
  %94 = add nuw nsw i64 %81, 1
  %95 = icmp eq i64 %94, 240
  br i1 %95, label %96, label %80, !llvm.loop !22

96:                                               ; preds = %89
  %97 = add nuw nsw i64 %78, 1
  %98 = icmp eq i64 %97, 240
  br i1 %98, label %99, label %77, !llvm.loop !23

99:                                               ; preds = %96
  %100 = load ptr, ptr @stderr, align 8, !tbaa !20
  %101 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %100, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %102 = load ptr, ptr @stderr, align 8, !tbaa !20
  %103 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %102, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef nonnull %4) #4
  tail call void @free(ptr noundef %5) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #1

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
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
!20 = !{!21, !21, i64 0}
!21 = !{!"any pointer", !7, i64 0}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}

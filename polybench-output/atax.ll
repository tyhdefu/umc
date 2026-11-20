; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/atax/atax.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/atax/atax.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"y\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 159900, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  br label %7

7:                                                ; preds = %7, %2
  %8 = phi i64 [ 0, %2 ], [ %14, %7 ]
  %9 = trunc nuw nsw i64 %8 to i32
  %10 = uitofp nneg i32 %9 to double
  %11 = fdiv double %10, 4.100000e+02
  %12 = fadd double %11, 1.000000e+00
  %13 = getelementptr inbounds double, ptr %4, i64 %8
  store double %12, ptr %13, align 8, !tbaa !5
  %14 = add nuw nsw i64 %8, 1
  %15 = icmp eq i64 %14, 410
  br i1 %15, label %16, label %7, !llvm.loop !9

16:                                               ; preds = %7, %28
  %17 = phi i64 [ %29, %28 ], [ 0, %7 ]
  br label %18

18:                                               ; preds = %18, %16
  %19 = phi i64 [ 0, %16 ], [ %26, %18 ]
  %20 = add nuw nsw i64 %19, %17
  %21 = trunc nuw nsw i64 %20 to i32
  %22 = urem i32 %21, 410
  %23 = uitofp nneg i32 %22 to double
  %24 = fdiv double %23, 1.950000e+03
  %25 = getelementptr inbounds [410 x double], ptr %3, i64 %17, i64 %19
  store double %24, ptr %25, align 8, !tbaa !5
  %26 = add nuw nsw i64 %19, 1
  %27 = icmp eq i64 %26, 410
  br i1 %27, label %28, label %18, !llvm.loop !12

28:                                               ; preds = %18
  %29 = add nuw nsw i64 %17, 1
  %30 = icmp eq i64 %29, 390
  br i1 %30, label %31, label %16, !llvm.loop !13

31:                                               ; preds = %28, %31
  %32 = phi i64 [ %34, %31 ], [ 0, %28 ]
  %33 = getelementptr inbounds double, ptr %5, i64 %32
  store double 0.000000e+00, ptr %33, align 8, !tbaa !5
  %34 = add nuw nsw i64 %32, 1
  %35 = icmp eq i64 %34, 410
  br i1 %35, label %36, label %31, !llvm.loop !14

36:                                               ; preds = %31, %59
  %37 = phi i64 [ %60, %59 ], [ 0, %31 ]
  %38 = getelementptr inbounds double, ptr %6, i64 %37
  store double 0.000000e+00, ptr %38, align 8, !tbaa !5
  br label %39

39:                                               ; preds = %39, %36
  %40 = phi i64 [ 0, %36 ], [ %47, %39 ]
  %41 = phi double [ 0.000000e+00, %36 ], [ %46, %39 ]
  %42 = getelementptr inbounds [410 x double], ptr %3, i64 %37, i64 %40
  %43 = load double, ptr %42, align 8, !tbaa !5
  %44 = getelementptr inbounds double, ptr %4, i64 %40
  %45 = load double, ptr %44, align 8, !tbaa !5
  %46 = tail call double @llvm.fmuladd.f64(double %43, double %45, double %41)
  store double %46, ptr %38, align 8, !tbaa !5
  %47 = add nuw nsw i64 %40, 1
  %48 = icmp eq i64 %47, 410
  br i1 %48, label %49, label %39, !llvm.loop !15

49:                                               ; preds = %39, %49
  %50 = phi i64 [ %57, %49 ], [ 0, %39 ]
  %51 = getelementptr inbounds double, ptr %5, i64 %50
  %52 = load double, ptr %51, align 8, !tbaa !5
  %53 = getelementptr inbounds [410 x double], ptr %3, i64 %37, i64 %50
  %54 = load double, ptr %53, align 8, !tbaa !5
  %55 = load double, ptr %38, align 8, !tbaa !5
  %56 = tail call double @llvm.fmuladd.f64(double %54, double %55, double %52)
  store double %56, ptr %51, align 8, !tbaa !5
  %57 = add nuw nsw i64 %50, 1
  %58 = icmp eq i64 %57, 410
  br i1 %58, label %59, label %49, !llvm.loop !16

59:                                               ; preds = %49
  %60 = add nuw nsw i64 %37, 1
  %61 = icmp eq i64 %60, 390
  br i1 %61, label %62, label %36, !llvm.loop !17

62:                                               ; preds = %59
  %63 = load ptr, ptr @stderr, align 8, !tbaa !18
  %64 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %63, ptr noundef nonnull @.str) #4
  %65 = load ptr, ptr @stderr, align 8, !tbaa !18
  %66 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %65, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %67

67:                                               ; preds = %75, %62
  %68 = phi i64 [ 0, %62 ], [ %80, %75 ]
  %69 = trunc i64 %68 to i16
  %70 = urem i16 %69, 20
  %71 = icmp eq i16 %70, 0
  br i1 %71, label %72, label %75

72:                                               ; preds = %67
  %73 = load ptr, ptr @stderr, align 8, !tbaa !18
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str.3) #4
  br label %75

75:                                               ; preds = %72, %67
  %76 = load ptr, ptr @stderr, align 8, !tbaa !18
  %77 = getelementptr inbounds double, ptr %5, i64 %68
  %78 = load double, ptr %77, align 8, !tbaa !5
  %79 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.4, double noundef %78) #4
  %80 = add nuw nsw i64 %68, 1
  %81 = icmp eq i64 %80, 410
  br i1 %81, label %82, label %67, !llvm.loop !20

82:                                               ; preds = %75
  %83 = load ptr, ptr @stderr, align 8, !tbaa !18
  %84 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %85 = load ptr, ptr @stderr, align 8, !tbaa !18
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef nonnull %5) #4
  tail call void @free(ptr noundef %6) #4
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
!18 = !{!19, !19, i64 0}
!19 = !{!"any pointer", !7, i64 0}
!20 = distinct !{!20, !10, !11}

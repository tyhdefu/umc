; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gesummv/gesummv.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gesummv/gesummv.c"
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
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62500, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 62500, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  br label %8

8:                                                ; preds = %30, %2
  %9 = phi i64 [ 0, %2 ], [ %31, %30 ]
  %10 = trunc nuw nsw i64 %9 to i32
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 2.500000e+02
  %13 = getelementptr inbounds double, ptr %6, i64 %9
  store double %12, ptr %13, align 8, !tbaa !5
  br label %14

14:                                               ; preds = %14, %8
  %15 = phi i64 [ 0, %8 ], [ %28, %14 ]
  %16 = mul nuw nsw i64 %15, %9
  %17 = trunc i64 %16 to i32
  %18 = add i32 %17, 1
  %19 = urem i32 %18, 250
  %20 = uitofp nneg i32 %19 to double
  %21 = fdiv double %20, 2.500000e+02
  %22 = getelementptr inbounds [250 x double], ptr %3, i64 %9, i64 %15
  store double %21, ptr %22, align 8, !tbaa !5
  %23 = add i32 %17, 2
  %24 = urem i32 %23, 250
  %25 = uitofp nneg i32 %24 to double
  %26 = fdiv double %25, 2.500000e+02
  %27 = getelementptr inbounds [250 x double], ptr %4, i64 %9, i64 %15
  store double %26, ptr %27, align 8, !tbaa !5
  %28 = add nuw nsw i64 %15, 1
  %29 = icmp eq i64 %28, 250
  br i1 %29, label %30, label %14, !llvm.loop !9

30:                                               ; preds = %14
  %31 = add nuw nsw i64 %9, 1
  %32 = icmp eq i64 %31, 250
  br i1 %32, label %33, label %8, !llvm.loop !12

33:                                               ; preds = %30, %52
  %34 = phi i64 [ %56, %52 ], [ 0, %30 ]
  %35 = getelementptr inbounds double, ptr %5, i64 %34
  store double 0.000000e+00, ptr %35, align 8, !tbaa !5
  %36 = getelementptr inbounds double, ptr %7, i64 %34
  store double 0.000000e+00, ptr %36, align 8, !tbaa !5
  br label %37

37:                                               ; preds = %37, %33
  %38 = phi i64 [ 0, %33 ], [ %50, %37 ]
  %39 = getelementptr inbounds [250 x double], ptr %3, i64 %34, i64 %38
  %40 = load double, ptr %39, align 8, !tbaa !5
  %41 = getelementptr inbounds double, ptr %6, i64 %38
  %42 = load double, ptr %41, align 8, !tbaa !5
  %43 = load double, ptr %35, align 8, !tbaa !5
  %44 = tail call double @llvm.fmuladd.f64(double %40, double %42, double %43)
  store double %44, ptr %35, align 8, !tbaa !5
  %45 = getelementptr inbounds [250 x double], ptr %4, i64 %34, i64 %38
  %46 = load double, ptr %45, align 8, !tbaa !5
  %47 = load double, ptr %41, align 8, !tbaa !5
  %48 = load double, ptr %36, align 8, !tbaa !5
  %49 = tail call double @llvm.fmuladd.f64(double %46, double %47, double %48)
  store double %49, ptr %36, align 8, !tbaa !5
  %50 = add nuw nsw i64 %38, 1
  %51 = icmp eq i64 %50, 250
  br i1 %51, label %52, label %37, !llvm.loop !13

52:                                               ; preds = %37
  %53 = load double, ptr %35, align 8, !tbaa !5
  %54 = fmul double %49, 1.200000e+00
  %55 = tail call double @llvm.fmuladd.f64(double %53, double 1.500000e+00, double %54)
  store double %55, ptr %36, align 8, !tbaa !5
  %56 = add nuw nsw i64 %34, 1
  %57 = icmp eq i64 %56, 250
  br i1 %57, label %58, label %33, !llvm.loop !14

58:                                               ; preds = %52
  %59 = load ptr, ptr @stderr, align 8, !tbaa !15
  %60 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %59, ptr noundef nonnull @.str) #4
  %61 = load ptr, ptr @stderr, align 8, !tbaa !15
  %62 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %61, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %63

63:                                               ; preds = %71, %58
  %64 = phi i64 [ 0, %58 ], [ %76, %71 ]
  %65 = trunc i64 %64 to i8
  %66 = urem i8 %65, 20
  %67 = icmp eq i8 %66, 0
  br i1 %67, label %68, label %71

68:                                               ; preds = %63
  %69 = load ptr, ptr @stderr, align 8, !tbaa !15
  %70 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %69, ptr noundef nonnull @.str.3) #4
  br label %71

71:                                               ; preds = %68, %63
  %72 = load ptr, ptr @stderr, align 8, !tbaa !15
  %73 = getelementptr inbounds double, ptr %7, i64 %64
  %74 = load double, ptr %73, align 8, !tbaa !5
  %75 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %72, ptr noundef nonnull @.str.4, double noundef %74) #4
  %76 = add nuw nsw i64 %64, 1
  %77 = icmp eq i64 %76, 250
  br i1 %77, label %78, label %63, !llvm.loop !17

78:                                               ; preds = %71
  %79 = load ptr, ptr @stderr, align 8, !tbaa !15
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %81 = load ptr, ptr @stderr, align 8, !tbaa !15
  %82 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %81, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  tail call void @free(ptr noundef nonnull %7) #4
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
!15 = !{!16, !16, i64 0}
!16 = !{!"any pointer", !7, i64 0}
!17 = distinct !{!17, !10, !11}

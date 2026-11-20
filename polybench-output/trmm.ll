; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/trmm/trmm.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/trmm/trmm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"B\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 40000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  br label %5

5:                                                ; preds = %31, %2
  %6 = phi i64 [ 0, %2 ], [ %32, %31 ]
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %18, label %8

8:                                                ; preds = %5, %8
  %9 = phi i64 [ %16, %8 ], [ 0, %5 ]
  %10 = add nuw nsw i64 %9, %6
  %11 = trunc nuw nsw i64 %10 to i32
  %12 = urem i32 %11, 200
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 2.000000e+02
  %15 = getelementptr inbounds [200 x double], ptr %3, i64 %6, i64 %9
  store double %14, ptr %15, align 8, !tbaa !5
  %16 = add nuw nsw i64 %9, 1
  %17 = icmp eq i64 %16, %6
  br i1 %17, label %18, label %8, !llvm.loop !9

18:                                               ; preds = %8, %5
  %19 = getelementptr inbounds [200 x double], ptr %3, i64 %6, i64 %6
  store double 1.000000e+00, ptr %19, align 8, !tbaa !5
  %20 = add nuw nsw i64 %6, 240
  br label %21

21:                                               ; preds = %21, %18
  %22 = phi i64 [ 0, %18 ], [ %29, %21 ]
  %23 = sub nuw nsw i64 %20, %22
  %24 = trunc nuw nsw i64 %23 to i32
  %25 = urem i32 %24, 240
  %26 = uitofp nneg i32 %25 to double
  %27 = fdiv double %26, 2.400000e+02
  %28 = getelementptr inbounds [240 x double], ptr %4, i64 %6, i64 %22
  store double %27, ptr %28, align 8, !tbaa !5
  %29 = add nuw nsw i64 %22, 1
  %30 = icmp eq i64 %29, 240
  br i1 %30, label %31, label %21, !llvm.loop !12

31:                                               ; preds = %21
  %32 = add nuw nsw i64 %6, 1
  %33 = icmp eq i64 %32, 200
  br i1 %33, label %34, label %5, !llvm.loop !13

34:                                               ; preds = %31, %56
  %35 = phi i64 [ %57, %56 ], [ 0, %31 ]
  %36 = icmp ult i64 %35, 199
  br label %37

37:                                               ; preds = %51, %34
  %38 = phi i64 [ 0, %34 ], [ %54, %51 ]
  %39 = getelementptr inbounds [240 x double], ptr %4, i64 %35, i64 %38
  %40 = load double, ptr %39, align 8, !tbaa !5
  br i1 %36, label %41, label %51

41:                                               ; preds = %37, %41
  %42 = phi i64 [ %44, %41 ], [ %35, %37 ]
  %43 = phi double [ %49, %41 ], [ %40, %37 ]
  %44 = add nuw nsw i64 %42, 1
  %45 = getelementptr inbounds [200 x double], ptr %3, i64 %44, i64 %35
  %46 = load double, ptr %45, align 8, !tbaa !5
  %47 = getelementptr inbounds [240 x double], ptr %4, i64 %44, i64 %38
  %48 = load double, ptr %47, align 8, !tbaa !5
  %49 = tail call double @llvm.fmuladd.f64(double %46, double %48, double %43)
  store double %49, ptr %39, align 8, !tbaa !5
  %50 = icmp eq i64 %44, 199
  br i1 %50, label %51, label %41, !llvm.loop !14

51:                                               ; preds = %41, %37
  %52 = phi double [ %40, %37 ], [ %49, %41 ]
  %53 = fmul double %52, 1.500000e+00
  store double %53, ptr %39, align 8, !tbaa !5
  %54 = add nuw nsw i64 %38, 1
  %55 = icmp eq i64 %54, 240
  br i1 %55, label %56, label %37, !llvm.loop !15

56:                                               ; preds = %51
  %57 = add nuw nsw i64 %35, 1
  %58 = icmp eq i64 %57, 200
  br i1 %58, label %59, label %34, !llvm.loop !16

59:                                               ; preds = %56
  %60 = load ptr, ptr @stderr, align 8, !tbaa !17
  %61 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %60, ptr noundef nonnull @.str) #4
  %62 = load ptr, ptr @stderr, align 8, !tbaa !17
  %63 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %62, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %64

64:                                               ; preds = %83, %59
  %65 = phi i64 [ 0, %59 ], [ %84, %83 ]
  %66 = mul nuw nsw i64 %65, 200
  br label %67

67:                                               ; preds = %76, %64
  %68 = phi i64 [ 0, %64 ], [ %81, %76 ]
  %69 = add nuw nsw i64 %68, %66
  %70 = trunc nuw nsw i64 %69 to i32
  %71 = urem i32 %70, 20
  %72 = icmp eq i32 %71, 0
  br i1 %72, label %73, label %76

73:                                               ; preds = %67
  %74 = load ptr, ptr @stderr, align 8, !tbaa !17
  %75 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %74, ptr noundef nonnull @.str.3) #4
  br label %76

76:                                               ; preds = %73, %67
  %77 = load ptr, ptr @stderr, align 8, !tbaa !17
  %78 = getelementptr inbounds [240 x double], ptr %4, i64 %65, i64 %68
  %79 = load double, ptr %78, align 8, !tbaa !5
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %77, ptr noundef nonnull @.str.4, double noundef %79) #4
  %81 = add nuw nsw i64 %68, 1
  %82 = icmp eq i64 %81, 240
  br i1 %82, label %83, label %67, !llvm.loop !19

83:                                               ; preds = %76
  %84 = add nuw nsw i64 %65, 1
  %85 = icmp eq i64 %84, 200
  br i1 %85, label %86, label %64, !llvm.loop !20

86:                                               ; preds = %83
  %87 = load ptr, ptr @stderr, align 8, !tbaa !17
  %88 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %87, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %89 = load ptr, ptr @stderr, align 8, !tbaa !17
  %90 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %89, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef nonnull %4) #4
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
!17 = !{!18, !18, i64 0}
!18 = !{!"any pointer", !7, i64 0}
!19 = distinct !{!19, !10, !11}
!20 = distinct !{!20, !10, !11}

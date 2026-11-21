; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/trisolv/trisolv.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/trisolv/trisolv.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"x\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.4 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %24, %2
  %7 = phi i64 [ 0, %2 ], [ %25, %24 ]
  %8 = phi i64 [ 1, %2 ], [ %26, %24 ]
  %9 = getelementptr inbounds double, ptr %4, i64 %7
  store double -9.990000e+02, ptr %9, align 8, !tbaa !5
  %10 = trunc nuw nsw i64 %7 to i32
  %11 = uitofp nneg i32 %10 to double
  %12 = getelementptr inbounds double, ptr %5, i64 %7
  store double %11, ptr %12, align 8, !tbaa !5
  %13 = add nuw nsw i64 %7, 401
  br label %14

14:                                               ; preds = %14, %6
  %15 = phi i64 [ 0, %6 ], [ %22, %14 ]
  %16 = sub nuw nsw i64 %13, %15
  %17 = trunc nuw nsw i64 %16 to i32
  %18 = sitofp i32 %17 to double
  %19 = fmul double %18, 2.000000e+00
  %20 = fdiv double %19, 4.000000e+02
  %21 = getelementptr inbounds [400 x double], ptr %3, i64 %7, i64 %15
  store double %20, ptr %21, align 8, !tbaa !5
  %22 = add nuw nsw i64 %15, 1
  %23 = icmp eq i64 %22, %8
  br i1 %23, label %24, label %14, !llvm.loop !9

24:                                               ; preds = %14
  %25 = add nuw nsw i64 %7, 1
  %26 = add nuw nsw i64 %8, 1
  %27 = icmp eq i64 %25, 400
  br i1 %27, label %28, label %6, !llvm.loop !12

28:                                               ; preds = %24, %45
  %29 = phi i64 [ %50, %45 ], [ 0, %24 ]
  %30 = getelementptr inbounds double, ptr %5, i64 %29
  %31 = load double, ptr %30, align 8, !tbaa !5
  %32 = getelementptr inbounds double, ptr %4, i64 %29
  store double %31, ptr %32, align 8, !tbaa !5
  %33 = icmp eq i64 %29, 0
  br i1 %33, label %45, label %34

34:                                               ; preds = %28, %34
  %35 = phi i64 [ %43, %34 ], [ 0, %28 ]
  %36 = phi double [ %42, %34 ], [ %31, %28 ]
  %37 = getelementptr inbounds [400 x double], ptr %3, i64 %29, i64 %35
  %38 = load double, ptr %37, align 8, !tbaa !5
  %39 = getelementptr inbounds double, ptr %4, i64 %35
  %40 = load double, ptr %39, align 8, !tbaa !5
  %41 = fneg double %38
  %42 = tail call double @llvm.fmuladd.f64(double %41, double %40, double %36)
  store double %42, ptr %32, align 8, !tbaa !5
  %43 = add nuw nsw i64 %35, 1
  %44 = icmp eq i64 %43, %29
  br i1 %44, label %45, label %34, !llvm.loop !13

45:                                               ; preds = %34, %28
  %46 = phi double [ %31, %28 ], [ %42, %34 ]
  %47 = getelementptr inbounds [400 x double], ptr %3, i64 %29, i64 %29
  %48 = load double, ptr %47, align 8, !tbaa !5
  %49 = fdiv double %46, %48
  store double %49, ptr %32, align 8, !tbaa !5
  %50 = add nuw nsw i64 %29, 1
  %51 = icmp eq i64 %50, 400
  br i1 %51, label %52, label %28, !llvm.loop !14

52:                                               ; preds = %45
  %53 = load ptr, ptr @stderr, align 8, !tbaa !15
  %54 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %53, ptr noundef nonnull @.str) #4
  %55 = load ptr, ptr @stderr, align 8, !tbaa !15
  %56 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %55, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %57

57:                                               ; preds = %69, %52
  %58 = phi i64 [ 0, %52 ], [ %70, %69 ]
  %59 = load ptr, ptr @stderr, align 8, !tbaa !15
  %60 = getelementptr inbounds double, ptr %4, i64 %58
  %61 = load double, ptr %60, align 8, !tbaa !5
  %62 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %59, ptr noundef nonnull @.str.3, double noundef %61) #4
  %63 = trunc i64 %58 to i16
  %64 = urem i16 %63, 20
  %65 = icmp eq i16 %64, 0
  br i1 %65, label %66, label %69

66:                                               ; preds = %57
  %67 = load ptr, ptr @stderr, align 8, !tbaa !15
  %68 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.4) #4
  br label %69

69:                                               ; preds = %66, %57
  %70 = add nuw nsw i64 %58, 1
  %71 = icmp eq i64 %70, 400
  br i1 %71, label %72, label %57, !llvm.loop !17

72:                                               ; preds = %69
  %73 = load ptr, ptr @stderr, align 8, !tbaa !15
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %75 = load ptr, ptr @stderr, align 8, !tbaa !15
  %76 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %75, ptr noundef nonnull @.str.6) #4
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
!15 = !{!16, !16, i64 0}
!16 = !{!"any pointer", !7, i64 0}
!17 = distinct !{!17, !10, !11}

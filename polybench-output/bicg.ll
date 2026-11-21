; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/bicg/bicg.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/bicg/bicg.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"s\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"q\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 159900, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  br label %8

8:                                                ; preds = %8, %2
  %9 = phi i64 [ 0, %2 ], [ %14, %8 ]
  %10 = trunc nuw nsw i64 %9 to i32
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 3.900000e+02
  %13 = getelementptr inbounds double, ptr %6, i64 %9
  store double %12, ptr %13, align 8, !tbaa !5
  %14 = add nuw nsw i64 %9, 1
  %15 = icmp eq i64 %14, 390
  br i1 %15, label %16, label %8, !llvm.loop !9

16:                                               ; preds = %8, %32
  %17 = phi i64 [ %33, %32 ], [ 0, %8 ]
  %18 = trunc nuw nsw i64 %17 to i32
  %19 = uitofp nneg i32 %18 to double
  %20 = fdiv double %19, 4.100000e+02
  %21 = getelementptr inbounds double, ptr %7, i64 %17
  store double %20, ptr %21, align 8, !tbaa !5
  br label %22

22:                                               ; preds = %22, %16
  %23 = phi i64 [ 0, %16 ], [ %24, %22 ]
  %24 = add nuw nsw i64 %23, 1
  %25 = mul nuw nsw i64 %24, %17
  %26 = trunc nuw nsw i64 %25 to i32
  %27 = urem i32 %26, 410
  %28 = uitofp nneg i32 %27 to double
  %29 = fdiv double %28, 4.100000e+02
  %30 = getelementptr inbounds [390 x double], ptr %3, i64 %17, i64 %23
  store double %29, ptr %30, align 8, !tbaa !5
  %31 = icmp eq i64 %24, 390
  br i1 %31, label %32, label %22, !llvm.loop !12

32:                                               ; preds = %22
  %33 = add nuw nsw i64 %17, 1
  %34 = icmp eq i64 %33, 410
  br i1 %34, label %35, label %16, !llvm.loop !13

35:                                               ; preds = %32, %35
  %36 = phi i64 [ %38, %35 ], [ 0, %32 ]
  %37 = getelementptr inbounds double, ptr %4, i64 %36
  store double 0.000000e+00, ptr %37, align 8, !tbaa !5
  %38 = add nuw nsw i64 %36, 1
  %39 = icmp eq i64 %38, 390
  br i1 %39, label %40, label %35, !llvm.loop !14

40:                                               ; preds = %35, %59
  %41 = phi i64 [ %60, %59 ], [ 0, %35 ]
  %42 = getelementptr inbounds double, ptr %5, i64 %41
  store double 0.000000e+00, ptr %42, align 8, !tbaa !5
  %43 = getelementptr inbounds double, ptr %7, i64 %41
  br label %44

44:                                               ; preds = %44, %40
  %45 = phi i64 [ 0, %40 ], [ %57, %44 ]
  %46 = getelementptr inbounds double, ptr %4, i64 %45
  %47 = load double, ptr %46, align 8, !tbaa !5
  %48 = load double, ptr %43, align 8, !tbaa !5
  %49 = getelementptr inbounds [390 x double], ptr %3, i64 %41, i64 %45
  %50 = load double, ptr %49, align 8, !tbaa !5
  %51 = tail call double @llvm.fmuladd.f64(double %48, double %50, double %47)
  store double %51, ptr %46, align 8, !tbaa !5
  %52 = load double, ptr %42, align 8, !tbaa !5
  %53 = load double, ptr %49, align 8, !tbaa !5
  %54 = getelementptr inbounds double, ptr %6, i64 %45
  %55 = load double, ptr %54, align 8, !tbaa !5
  %56 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %52)
  store double %56, ptr %42, align 8, !tbaa !5
  %57 = add nuw nsw i64 %45, 1
  %58 = icmp eq i64 %57, 390
  br i1 %58, label %59, label %44, !llvm.loop !15

59:                                               ; preds = %44
  %60 = add nuw nsw i64 %41, 1
  %61 = icmp eq i64 %60, 410
  br i1 %61, label %62, label %40, !llvm.loop !16

62:                                               ; preds = %59
  %63 = load ptr, ptr @stderr, align 8, !tbaa !17
  %64 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %63, ptr noundef nonnull @.str) #4
  %65 = load ptr, ptr @stderr, align 8, !tbaa !17
  %66 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %65, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %67

67:                                               ; preds = %75, %62
  %68 = phi i64 [ 0, %62 ], [ %80, %75 ]
  %69 = trunc i64 %68 to i16
  %70 = urem i16 %69, 20
  %71 = icmp eq i16 %70, 0
  br i1 %71, label %72, label %75

72:                                               ; preds = %67
  %73 = load ptr, ptr @stderr, align 8, !tbaa !17
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str.3) #4
  br label %75

75:                                               ; preds = %72, %67
  %76 = load ptr, ptr @stderr, align 8, !tbaa !17
  %77 = getelementptr inbounds double, ptr %4, i64 %68
  %78 = load double, ptr %77, align 8, !tbaa !5
  %79 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.4, double noundef %78) #4
  %80 = add nuw nsw i64 %68, 1
  %81 = icmp eq i64 %80, 390
  br i1 %81, label %82, label %67, !llvm.loop !19

82:                                               ; preds = %75
  %83 = load ptr, ptr @stderr, align 8, !tbaa !17
  %84 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %85 = load ptr, ptr @stderr, align 8, !tbaa !17
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  br label %87

87:                                               ; preds = %95, %82
  %88 = phi i64 [ 0, %82 ], [ %100, %95 ]
  %89 = trunc i64 %88 to i16
  %90 = urem i16 %89, 20
  %91 = icmp eq i16 %90, 0
  br i1 %91, label %92, label %95

92:                                               ; preds = %87
  %93 = load ptr, ptr @stderr, align 8, !tbaa !17
  %94 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %93, ptr noundef nonnull @.str.3) #4
  br label %95

95:                                               ; preds = %92, %87
  %96 = load ptr, ptr @stderr, align 8, !tbaa !17
  %97 = getelementptr inbounds double, ptr %5, i64 %88
  %98 = load double, ptr %97, align 8, !tbaa !5
  %99 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %96, ptr noundef nonnull @.str.4, double noundef %98) #4
  %100 = add nuw nsw i64 %88, 1
  %101 = icmp eq i64 %100, 410
  br i1 %101, label %102, label %87, !llvm.loop !20

102:                                              ; preds = %95
  %103 = load ptr, ptr @stderr, align 8, !tbaa !17
  %104 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %103, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %105 = load ptr, ptr @stderr, align 8, !tbaa !17
  %106 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %105, ptr noundef nonnull @.str.7) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef nonnull %5) #4
  tail call void @free(ptr noundef %6) #4
  tail call void @free(ptr noundef %7) #4
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

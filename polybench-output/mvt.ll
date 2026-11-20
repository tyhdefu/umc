; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/mvt/mvt.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/mvt/mvt.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [3 x i8] c"x1\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [3 x i8] c"x2\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  br label %9

8:                                                ; preds = %34
  br i1 %16, label %44, label %9, !llvm.loop !5

9:                                                ; preds = %8, %2
  %10 = phi i64 [ 0, %2 ], [ %15, %8 ]
  %11 = trunc nuw nsw i64 %10 to i32
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 4.000000e+02
  %14 = getelementptr inbounds double, ptr %4, i64 %10
  store double %13, ptr %14, align 8, !tbaa !8
  %15 = add nuw nsw i64 %10, 1
  %16 = icmp eq i64 %15, 400
  %17 = trunc nuw nsw i64 %15 to i32
  %18 = uitofp nneg i32 %17 to double
  %19 = fdiv double %18, 4.000000e+02
  %20 = select i1 %16, double 0.000000e+00, double %19
  %21 = getelementptr inbounds double, ptr %5, i64 %10
  store double %20, ptr %21, align 8, !tbaa !8
  %22 = icmp ult i64 %10, 397
  %23 = select i1 %22, i32 3, i32 -397
  %24 = add nsw i32 %23, %11
  %25 = uitofp nneg i32 %24 to double
  %26 = fdiv double %25, 4.000000e+02
  %27 = getelementptr inbounds double, ptr %6, i64 %10
  store double %26, ptr %27, align 8, !tbaa !8
  %28 = icmp ult i64 %10, 396
  %29 = select i1 %28, i32 4, i32 -396
  %30 = add nsw i32 %29, %11
  %31 = uitofp nneg i32 %30 to double
  %32 = fdiv double %31, 4.000000e+02
  %33 = getelementptr inbounds double, ptr %7, i64 %10
  store double %32, ptr %33, align 8, !tbaa !8
  br label %34

34:                                               ; preds = %34, %9
  %35 = phi i64 [ 0, %9 ], [ %42, %34 ]
  %36 = mul nuw nsw i64 %35, %10
  %37 = trunc nuw nsw i64 %36 to i32
  %38 = urem i32 %37, 400
  %39 = uitofp nneg i32 %38 to double
  %40 = fdiv double %39, 4.000000e+02
  %41 = getelementptr inbounds [400 x double], ptr %3, i64 %10, i64 %35
  store double %40, ptr %41, align 8, !tbaa !8
  %42 = add nuw nsw i64 %35, 1
  %43 = icmp eq i64 %42, 400
  br i1 %43, label %8, label %34, !llvm.loop !12

44:                                               ; preds = %8, %58
  %45 = phi i64 [ %59, %58 ], [ 0, %8 ]
  %46 = getelementptr inbounds double, ptr %4, i64 %45
  %47 = load double, ptr %46, align 8, !tbaa !8
  br label %48

48:                                               ; preds = %48, %44
  %49 = phi i64 [ 0, %44 ], [ %56, %48 ]
  %50 = phi double [ %47, %44 ], [ %55, %48 ]
  %51 = getelementptr inbounds [400 x double], ptr %3, i64 %45, i64 %49
  %52 = load double, ptr %51, align 8, !tbaa !8
  %53 = getelementptr inbounds double, ptr %6, i64 %49
  %54 = load double, ptr %53, align 8, !tbaa !8
  %55 = tail call double @llvm.fmuladd.f64(double %52, double %54, double %50)
  store double %55, ptr %46, align 8, !tbaa !8
  %56 = add nuw nsw i64 %49, 1
  %57 = icmp eq i64 %56, 400
  br i1 %57, label %58, label %48, !llvm.loop !13

58:                                               ; preds = %48
  %59 = add nuw nsw i64 %45, 1
  %60 = icmp eq i64 %59, 400
  br i1 %60, label %61, label %44, !llvm.loop !14

61:                                               ; preds = %58, %75
  %62 = phi i64 [ %76, %75 ], [ 0, %58 ]
  %63 = getelementptr inbounds double, ptr %5, i64 %62
  %64 = load double, ptr %63, align 8, !tbaa !8
  br label %65

65:                                               ; preds = %65, %61
  %66 = phi i64 [ 0, %61 ], [ %73, %65 ]
  %67 = phi double [ %64, %61 ], [ %72, %65 ]
  %68 = getelementptr inbounds [400 x double], ptr %3, i64 %66, i64 %62
  %69 = load double, ptr %68, align 8, !tbaa !8
  %70 = getelementptr inbounds double, ptr %7, i64 %66
  %71 = load double, ptr %70, align 8, !tbaa !8
  %72 = tail call double @llvm.fmuladd.f64(double %69, double %71, double %67)
  store double %72, ptr %63, align 8, !tbaa !8
  %73 = add nuw nsw i64 %66, 1
  %74 = icmp eq i64 %73, 400
  br i1 %74, label %75, label %65, !llvm.loop !15

75:                                               ; preds = %65
  %76 = add nuw nsw i64 %62, 1
  %77 = icmp eq i64 %76, 400
  br i1 %77, label %78, label %61, !llvm.loop !16

78:                                               ; preds = %75
  %79 = load ptr, ptr @stderr, align 8, !tbaa !17
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str) #4
  %81 = load ptr, ptr @stderr, align 8, !tbaa !17
  %82 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %81, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %83

83:                                               ; preds = %91, %78
  %84 = phi i64 [ 0, %78 ], [ %96, %91 ]
  %85 = trunc i64 %84 to i16
  %86 = urem i16 %85, 20
  %87 = icmp eq i16 %86, 0
  br i1 %87, label %88, label %91

88:                                               ; preds = %83
  %89 = load ptr, ptr @stderr, align 8, !tbaa !17
  %90 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %89, ptr noundef nonnull @.str.3) #4
  br label %91

91:                                               ; preds = %88, %83
  %92 = load ptr, ptr @stderr, align 8, !tbaa !17
  %93 = getelementptr inbounds double, ptr %4, i64 %84
  %94 = load double, ptr %93, align 8, !tbaa !8
  %95 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %92, ptr noundef nonnull @.str.4, double noundef %94) #4
  %96 = add nuw nsw i64 %84, 1
  %97 = icmp eq i64 %96, 400
  br i1 %97, label %98, label %83, !llvm.loop !19

98:                                               ; preds = %91
  %99 = load ptr, ptr @stderr, align 8, !tbaa !17
  %100 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %99, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %101 = load ptr, ptr @stderr, align 8, !tbaa !17
  %102 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %101, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  br label %103

103:                                              ; preds = %111, %98
  %104 = phi i64 [ 0, %98 ], [ %116, %111 ]
  %105 = trunc i64 %104 to i16
  %106 = urem i16 %105, 20
  %107 = icmp eq i16 %106, 0
  br i1 %107, label %108, label %111

108:                                              ; preds = %103
  %109 = load ptr, ptr @stderr, align 8, !tbaa !17
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str.3) #4
  br label %111

111:                                              ; preds = %108, %103
  %112 = load ptr, ptr @stderr, align 8, !tbaa !17
  %113 = getelementptr inbounds double, ptr %5, i64 %104
  %114 = load double, ptr %113, align 8, !tbaa !8
  %115 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %112, ptr noundef nonnull @.str.4, double noundef %114) #4
  %116 = add nuw nsw i64 %104, 1
  %117 = icmp eq i64 %116, 400
  br i1 %117, label %118, label %103, !llvm.loop !20

118:                                              ; preds = %111
  %119 = load ptr, ptr @stderr, align 8, !tbaa !17
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %119, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %121 = load ptr, ptr @stderr, align 8, !tbaa !17
  %122 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %121, ptr noundef nonnull @.str.7) #4
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
!5 = distinct !{!5, !6, !7}
!6 = !{!"llvm.loop.mustprogress"}
!7 = !{!"llvm.loop.unroll.disable"}
!8 = !{!9, !9, i64 0}
!9 = !{!"double", !10, i64 0}
!10 = !{!"omnipotent char", !11, i64 0}
!11 = !{!"Simple C/C++ TBAA"}
!12 = distinct !{!12, !6, !7}
!13 = distinct !{!13, !6, !7}
!14 = distinct !{!14, !6, !7}
!15 = distinct !{!15, !6, !7}
!16 = distinct !{!16, !6, !7}
!17 = !{!18, !18, i64 0}
!18 = !{!"any pointer", !10, i64 0}
!19 = distinct !{!19, !6, !7}
!20 = distinct !{!20, !6, !7}

; ModuleID = 'linear-algebra/blas/gemm/gemm.c'
source_filename = "linear-algebra/blas/gemm/gemm.c"
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
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 44000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 52800, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %19, %2
  %7 = phi i64 [ 0, %2 ], [ %20, %19 ]
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi i64 [ 0, %6 ], [ %17, %8 ]
  %10 = mul nuw nsw i64 %9, %7
  %11 = trunc i64 %10 to i32
  %12 = add i32 %11, 1
  %13 = urem i32 %12, 200
  %14 = uitofp nneg i32 %13 to double
  %15 = fdiv double %14, 2.000000e+02
  %16 = getelementptr inbounds [220 x double], ptr %3, i64 %7, i64 %9
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = add nuw nsw i64 %9, 1
  %18 = icmp eq i64 %17, 220
  br i1 %18, label %19, label %8, !llvm.loop !9

19:                                               ; preds = %8
  %20 = add nuw nsw i64 %7, 1
  %21 = icmp eq i64 %20, 200
  br i1 %21, label %22, label %6, !llvm.loop !12

22:                                               ; preds = %19, %34
  %23 = phi i64 [ %35, %34 ], [ 0, %19 ]
  br label %24

24:                                               ; preds = %24, %22
  %25 = phi i64 [ 0, %22 ], [ %26, %24 ]
  %26 = add nuw nsw i64 %25, 1
  %27 = mul nuw nsw i64 %26, %23
  %28 = trunc nuw nsw i64 %27 to i32
  %29 = urem i32 %28, 240
  %30 = uitofp nneg i32 %29 to double
  %31 = fdiv double %30, 2.400000e+02
  %32 = getelementptr inbounds [240 x double], ptr %4, i64 %23, i64 %25
  store double %31, ptr %32, align 8, !tbaa !5
  %33 = icmp eq i64 %26, 240
  br i1 %33, label %34, label %24, !llvm.loop !13

34:                                               ; preds = %24
  %35 = add nuw nsw i64 %23, 1
  %36 = icmp eq i64 %35, 200
  br i1 %36, label %37, label %22, !llvm.loop !14

37:                                               ; preds = %34, %50
  %38 = phi i64 [ %51, %50 ], [ 0, %34 ]
  br label %39

39:                                               ; preds = %39, %37
  %40 = phi i64 [ 0, %37 ], [ %48, %39 ]
  %41 = add nuw nsw i64 %40, 2
  %42 = mul nuw nsw i64 %41, %38
  %43 = trunc nuw nsw i64 %42 to i32
  %44 = urem i32 %43, 220
  %45 = uitofp nneg i32 %44 to double
  %46 = fdiv double %45, 2.200000e+02
  %47 = getelementptr inbounds [220 x double], ptr %5, i64 %38, i64 %40
  store double %46, ptr %47, align 8, !tbaa !5
  %48 = add nuw nsw i64 %40, 1
  %49 = icmp eq i64 %48, 220
  br i1 %49, label %50, label %39, !llvm.loop !15

50:                                               ; preds = %39
  %51 = add nuw nsw i64 %38, 1
  %52 = icmp eq i64 %51, 240
  br i1 %52, label %53, label %37, !llvm.loop !16

53:                                               ; preds = %50, %79
  %54 = phi i64 [ %80, %79 ], [ 0, %50 ]
  br label %55

55:                                               ; preds = %55, %53
  %56 = phi i64 [ 0, %53 ], [ %60, %55 ]
  %57 = getelementptr inbounds [220 x double], ptr %3, i64 %54, i64 %56
  %58 = load double, ptr %57, align 8, !tbaa !5
  %59 = fmul double %58, 1.200000e+00
  store double %59, ptr %57, align 8, !tbaa !5
  %60 = add nuw nsw i64 %56, 1
  %61 = icmp eq i64 %60, 220
  br i1 %61, label %62, label %55, !llvm.loop !17

62:                                               ; preds = %55, %76
  %63 = phi i64 [ %77, %76 ], [ 0, %55 ]
  %64 = getelementptr inbounds [240 x double], ptr %4, i64 %54, i64 %63
  br label %65

65:                                               ; preds = %65, %62
  %66 = phi i64 [ 0, %62 ], [ %74, %65 ]
  %67 = load double, ptr %64, align 8, !tbaa !5
  %68 = fmul double %67, 1.500000e+00
  %69 = getelementptr inbounds [220 x double], ptr %5, i64 %63, i64 %66
  %70 = load double, ptr %69, align 8, !tbaa !5
  %71 = getelementptr inbounds [220 x double], ptr %3, i64 %54, i64 %66
  %72 = load double, ptr %71, align 8, !tbaa !5
  %73 = tail call double @llvm.fmuladd.f64(double %68, double %70, double %72)
  store double %73, ptr %71, align 8, !tbaa !5
  %74 = add nuw nsw i64 %66, 1
  %75 = icmp eq i64 %74, 220
  br i1 %75, label %76, label %65, !llvm.loop !18

76:                                               ; preds = %65
  %77 = add nuw nsw i64 %63, 1
  %78 = icmp eq i64 %77, 240
  br i1 %78, label %79, label %62, !llvm.loop !19

79:                                               ; preds = %76
  %80 = add nuw nsw i64 %54, 1
  %81 = icmp eq i64 %80, 200
  br i1 %81, label %82, label %53, !llvm.loop !20

82:                                               ; preds = %79
  %83 = load ptr, ptr @stderr, align 8, !tbaa !21
  %84 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str) #4
  %85 = load ptr, ptr @stderr, align 8, !tbaa !21
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %87

87:                                               ; preds = %106, %82
  %88 = phi i64 [ 0, %82 ], [ %107, %106 ]
  %89 = mul nuw nsw i64 %88, 200
  br label %90

90:                                               ; preds = %99, %87
  %91 = phi i64 [ 0, %87 ], [ %104, %99 ]
  %92 = add nuw nsw i64 %91, %89
  %93 = trunc nuw nsw i64 %92 to i32
  %94 = urem i32 %93, 20
  %95 = icmp eq i32 %94, 0
  br i1 %95, label %96, label %99

96:                                               ; preds = %90
  %97 = load ptr, ptr @stderr, align 8, !tbaa !21
  %98 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %97, ptr noundef nonnull @.str.3) #4
  br label %99

99:                                               ; preds = %96, %90
  %100 = load ptr, ptr @stderr, align 8, !tbaa !21
  %101 = getelementptr inbounds [220 x double], ptr %3, i64 %88, i64 %91
  %102 = load double, ptr %101, align 8, !tbaa !5
  %103 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %100, ptr noundef nonnull @.str.4, double noundef %102) #4
  %104 = add nuw nsw i64 %91, 1
  %105 = icmp eq i64 %104, 220
  br i1 %105, label %106, label %90, !llvm.loop !23

106:                                              ; preds = %99
  %107 = add nuw nsw i64 %88, 1
  %108 = icmp eq i64 %107, 200
  br i1 %108, label %109, label %87, !llvm.loop !24

109:                                              ; preds = %106
  %110 = load ptr, ptr @stderr, align 8, !tbaa !21
  %111 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %110, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %112 = load ptr, ptr @stderr, align 8, !tbaa !21
  %113 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %112, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef nonnull %3) #4
  tail call void @free(ptr noundef %4) #4
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
!20 = distinct !{!20, !10, !11}
!21 = !{!22, !22, i64 0}
!22 = !{!"any pointer", !7, i64 0}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}

; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/symm/symm.c'
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
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 40000, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %25, %2
  %7 = phi i64 [ 0, %2 ], [ %26, %25 ]
  %8 = add nuw nsw i64 %7, 240
  br label %9

9:                                                ; preds = %9, %6
  %10 = phi i64 [ 0, %6 ], [ %23, %9 ]
  %11 = add nuw nsw i64 %10, %7
  %12 = trunc nuw nsw i64 %11 to i32
  %13 = urem i32 %12, 100
  %14 = uitofp nneg i32 %13 to double
  %15 = fdiv double %14, 2.000000e+02
  %16 = getelementptr inbounds [240 x double], ptr %3, i64 %7, i64 %10
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = sub nuw nsw i64 %8, %10
  %18 = trunc nuw nsw i64 %17 to i32
  %19 = urem i32 %18, 100
  %20 = uitofp nneg i32 %19 to double
  %21 = fdiv double %20, 2.000000e+02
  %22 = getelementptr inbounds [240 x double], ptr %5, i64 %7, i64 %10
  store double %21, ptr %22, align 8, !tbaa !5
  %23 = add nuw nsw i64 %10, 1
  %24 = icmp eq i64 %23, 240
  br i1 %24, label %25, label %9, !llvm.loop !9

25:                                               ; preds = %9
  %26 = add nuw nsw i64 %7, 1
  %27 = icmp eq i64 %26, 200
  br i1 %27, label %31, label %6, !llvm.loop !12

28:                                               ; preds = %47, %44
  %29 = add nuw nsw i64 %33, 1
  %30 = icmp eq i64 %45, 200
  br i1 %30, label %52, label %31, !llvm.loop !13

31:                                               ; preds = %25, %28
  %32 = phi i64 [ %45, %28 ], [ 0, %25 ]
  %33 = phi i64 [ %29, %28 ], [ 1, %25 ]
  br label %34

34:                                               ; preds = %34, %31
  %35 = phi i64 [ 0, %31 ], [ %42, %34 ]
  %36 = add nuw nsw i64 %35, %32
  %37 = trunc nuw nsw i64 %36 to i32
  %38 = urem i32 %37, 100
  %39 = uitofp nneg i32 %38 to double
  %40 = fdiv double %39, 2.000000e+02
  %41 = getelementptr inbounds [200 x double], ptr %4, i64 %32, i64 %35
  store double %40, ptr %41, align 8, !tbaa !5
  %42 = add nuw nsw i64 %35, 1
  %43 = icmp eq i64 %42, %33
  br i1 %43, label %44, label %34, !llvm.loop !14

44:                                               ; preds = %34
  %45 = add nuw nsw i64 %32, 1
  %46 = icmp ult i64 %32, 199
  br i1 %46, label %47, label %28

47:                                               ; preds = %44, %47
  %48 = phi i64 [ %50, %47 ], [ %33, %44 ]
  %49 = getelementptr inbounds [200 x double], ptr %4, i64 %32, i64 %48
  store double -9.990000e+02, ptr %49, align 8, !tbaa !5
  %50 = add nuw nsw i64 %48, 1
  %51 = icmp eq i64 %50, 200
  br i1 %51, label %28, label %47, !llvm.loop !15

52:                                               ; preds = %28, %89
  %53 = phi i64 [ %90, %89 ], [ 0, %28 ]
  %54 = icmp eq i64 %53, 0
  %55 = getelementptr inbounds [200 x double], ptr %4, i64 %53, i64 %53
  br label %56

56:                                               ; preds = %76, %52
  %57 = phi i64 [ 0, %52 ], [ %87, %76 ]
  br i1 %54, label %76, label %58

58:                                               ; preds = %56
  %59 = getelementptr inbounds [240 x double], ptr %5, i64 %53, i64 %57
  br label %60

60:                                               ; preds = %60, %58
  %61 = phi i64 [ 0, %58 ], [ %74, %60 ]
  %62 = phi double [ 0.000000e+00, %58 ], [ %73, %60 ]
  %63 = load double, ptr %59, align 8, !tbaa !5
  %64 = fmul double %63, 1.500000e+00
  %65 = getelementptr inbounds [200 x double], ptr %4, i64 %53, i64 %61
  %66 = load double, ptr %65, align 8, !tbaa !5
  %67 = getelementptr inbounds [240 x double], ptr %3, i64 %61, i64 %57
  %68 = load double, ptr %67, align 8, !tbaa !5
  %69 = tail call double @llvm.fmuladd.f64(double %64, double %66, double %68)
  store double %69, ptr %67, align 8, !tbaa !5
  %70 = getelementptr inbounds [240 x double], ptr %5, i64 %61, i64 %57
  %71 = load double, ptr %70, align 8, !tbaa !5
  %72 = load double, ptr %65, align 8, !tbaa !5
  %73 = tail call double @llvm.fmuladd.f64(double %71, double %72, double %62)
  %74 = add nuw nsw i64 %61, 1
  %75 = icmp eq i64 %74, %53
  br i1 %75, label %76, label %60, !llvm.loop !16

76:                                               ; preds = %60, %56
  %77 = phi double [ 0.000000e+00, %56 ], [ %73, %60 ]
  %78 = getelementptr inbounds [240 x double], ptr %3, i64 %53, i64 %57
  %79 = load double, ptr %78, align 8, !tbaa !5
  %80 = getelementptr inbounds [240 x double], ptr %5, i64 %53, i64 %57
  %81 = load double, ptr %80, align 8, !tbaa !5
  %82 = fmul double %81, 1.500000e+00
  %83 = load double, ptr %55, align 8, !tbaa !5
  %84 = fmul double %82, %83
  %85 = tail call double @llvm.fmuladd.f64(double %79, double 1.200000e+00, double %84)
  %86 = tail call double @llvm.fmuladd.f64(double %77, double 1.500000e+00, double %85)
  store double %86, ptr %78, align 8, !tbaa !5
  %87 = add nuw nsw i64 %57, 1
  %88 = icmp eq i64 %87, 240
  br i1 %88, label %89, label %56, !llvm.loop !17

89:                                               ; preds = %76
  %90 = add nuw nsw i64 %53, 1
  %91 = icmp eq i64 %90, 200
  br i1 %91, label %92, label %52, !llvm.loop !18

92:                                               ; preds = %89
  %93 = load ptr, ptr @stderr, align 8, !tbaa !19
  %94 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %93, ptr noundef nonnull @.str) #4
  %95 = load ptr, ptr @stderr, align 8, !tbaa !19
  %96 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %95, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %97

97:                                               ; preds = %116, %92
  %98 = phi i64 [ 0, %92 ], [ %117, %116 ]
  %99 = mul nuw nsw i64 %98, 200
  br label %100

100:                                              ; preds = %109, %97
  %101 = phi i64 [ 0, %97 ], [ %114, %109 ]
  %102 = add nuw nsw i64 %101, %99
  %103 = trunc nuw nsw i64 %102 to i32
  %104 = urem i32 %103, 20
  %105 = icmp eq i32 %104, 0
  br i1 %105, label %106, label %109

106:                                              ; preds = %100
  %107 = load ptr, ptr @stderr, align 8, !tbaa !19
  %108 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %107, ptr noundef nonnull @.str.3) #4
  br label %109

109:                                              ; preds = %106, %100
  %110 = load ptr, ptr @stderr, align 8, !tbaa !19
  %111 = getelementptr inbounds [240 x double], ptr %3, i64 %98, i64 %101
  %112 = load double, ptr %111, align 8, !tbaa !5
  %113 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %110, ptr noundef nonnull @.str.4, double noundef %112) #4
  %114 = add nuw nsw i64 %101, 1
  %115 = icmp eq i64 %114, 240
  br i1 %115, label %116, label %100, !llvm.loop !21

116:                                              ; preds = %109
  %117 = add nuw nsw i64 %98, 1
  %118 = icmp eq i64 %117, 200
  br i1 %118, label %119, label %97, !llvm.loop !22

119:                                              ; preds = %116
  %120 = load ptr, ptr @stderr, align 8, !tbaa !19
  %121 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %120, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %122 = load ptr, ptr @stderr, align 8, !tbaa !19
  %123 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %122, ptr noundef nonnull @.str.6) #4
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
!19 = !{!20, !20, i64 0}
!20 = !{!"any pointer", !7, i64 0}
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}

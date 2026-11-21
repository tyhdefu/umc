; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gemver/gemver.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gemver/gemver.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"w\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %8 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %9 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %10 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %11 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  br label %14

12:                                               ; preds = %35
  %13 = icmp eq i64 %19, 400
  br i1 %13, label %45, label %14, !llvm.loop !5

14:                                               ; preds = %12, %2
  %15 = phi i64 [ 0, %2 ], [ %19, %12 ]
  %16 = trunc nuw nsw i64 %15 to i32
  %17 = uitofp nneg i32 %16 to double
  %18 = getelementptr inbounds double, ptr %4, i64 %15
  store double %17, ptr %18, align 8, !tbaa !8
  %19 = add nuw nsw i64 %15, 1
  %20 = trunc nuw nsw i64 %19 to i32
  %21 = uitofp nneg i32 %20 to double
  %22 = fdiv double %21, 4.000000e+02
  %23 = fmul double %22, 5.000000e-01
  %24 = getelementptr inbounds double, ptr %6, i64 %15
  store double %23, ptr %24, align 8, !tbaa !8
  %25 = fmul double %22, 2.500000e-01
  %26 = getelementptr inbounds double, ptr %5, i64 %15
  store double %25, ptr %26, align 8, !tbaa !8
  %27 = fdiv double %22, 6.000000e+00
  %28 = getelementptr inbounds double, ptr %7, i64 %15
  store double %27, ptr %28, align 8, !tbaa !8
  %29 = fmul double %22, 1.250000e-01
  %30 = getelementptr inbounds double, ptr %10, i64 %15
  store double %29, ptr %30, align 8, !tbaa !8
  %31 = fdiv double %22, 9.000000e+00
  %32 = getelementptr inbounds double, ptr %11, i64 %15
  store double %31, ptr %32, align 8, !tbaa !8
  %33 = getelementptr inbounds double, ptr %9, i64 %15
  store double 0.000000e+00, ptr %33, align 8, !tbaa !8
  %34 = getelementptr inbounds double, ptr %8, i64 %15
  store double 0.000000e+00, ptr %34, align 8, !tbaa !8
  br label %35

35:                                               ; preds = %35, %14
  %36 = phi i64 [ 0, %14 ], [ %43, %35 ]
  %37 = mul nuw nsw i64 %36, %15
  %38 = trunc nuw nsw i64 %37 to i32
  %39 = urem i32 %38, 400
  %40 = uitofp nneg i32 %39 to double
  %41 = fdiv double %40, 4.000000e+02
  %42 = getelementptr inbounds [400 x double], ptr %3, i64 %15, i64 %36
  store double %41, ptr %42, align 8, !tbaa !8
  %43 = add nuw nsw i64 %36, 1
  %44 = icmp eq i64 %43, 400
  br i1 %44, label %12, label %35, !llvm.loop !12

45:                                               ; preds = %12, %63
  %46 = phi i64 [ %64, %63 ], [ 0, %12 ]
  %47 = getelementptr inbounds double, ptr %4, i64 %46
  %48 = getelementptr inbounds double, ptr %6, i64 %46
  br label %49

49:                                               ; preds = %49, %45
  %50 = phi i64 [ 0, %45 ], [ %61, %49 ]
  %51 = getelementptr inbounds [400 x double], ptr %3, i64 %46, i64 %50
  %52 = load double, ptr %51, align 8, !tbaa !8
  %53 = load double, ptr %47, align 8, !tbaa !8
  %54 = getelementptr inbounds double, ptr %5, i64 %50
  %55 = load double, ptr %54, align 8, !tbaa !8
  %56 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %52)
  %57 = load double, ptr %48, align 8, !tbaa !8
  %58 = getelementptr inbounds double, ptr %7, i64 %50
  %59 = load double, ptr %58, align 8, !tbaa !8
  %60 = tail call double @llvm.fmuladd.f64(double %57, double %59, double %56)
  store double %60, ptr %51, align 8, !tbaa !8
  %61 = add nuw nsw i64 %50, 1
  %62 = icmp eq i64 %61, 400
  br i1 %62, label %63, label %49, !llvm.loop !13

63:                                               ; preds = %49
  %64 = add nuw nsw i64 %46, 1
  %65 = icmp eq i64 %64, 400
  br i1 %65, label %66, label %45, !llvm.loop !14

66:                                               ; preds = %63, %81
  %67 = phi i64 [ %82, %81 ], [ 0, %63 ]
  %68 = getelementptr inbounds double, ptr %9, i64 %67
  %69 = load double, ptr %68, align 8, !tbaa !8
  br label %70

70:                                               ; preds = %70, %66
  %71 = phi i64 [ 0, %66 ], [ %79, %70 ]
  %72 = phi double [ %69, %66 ], [ %78, %70 ]
  %73 = getelementptr inbounds [400 x double], ptr %3, i64 %71, i64 %67
  %74 = load double, ptr %73, align 8, !tbaa !8
  %75 = fmul double %74, 1.200000e+00
  %76 = getelementptr inbounds double, ptr %10, i64 %71
  %77 = load double, ptr %76, align 8, !tbaa !8
  %78 = tail call double @llvm.fmuladd.f64(double %75, double %77, double %72)
  store double %78, ptr %68, align 8, !tbaa !8
  %79 = add nuw nsw i64 %71, 1
  %80 = icmp eq i64 %79, 400
  br i1 %80, label %81, label %70, !llvm.loop !15

81:                                               ; preds = %70
  %82 = add nuw nsw i64 %67, 1
  %83 = icmp eq i64 %82, 400
  br i1 %83, label %84, label %66, !llvm.loop !16

84:                                               ; preds = %81, %84
  %85 = phi i64 [ %91, %84 ], [ 0, %81 ]
  %86 = getelementptr inbounds double, ptr %9, i64 %85
  %87 = load double, ptr %86, align 8, !tbaa !8
  %88 = getelementptr inbounds double, ptr %11, i64 %85
  %89 = load double, ptr %88, align 8, !tbaa !8
  %90 = fadd double %87, %89
  store double %90, ptr %86, align 8, !tbaa !8
  %91 = add nuw nsw i64 %85, 1
  %92 = icmp eq i64 %91, 400
  br i1 %92, label %93, label %84, !llvm.loop !17

93:                                               ; preds = %84, %108
  %94 = phi i64 [ %109, %108 ], [ 0, %84 ]
  %95 = getelementptr inbounds double, ptr %8, i64 %94
  %96 = load double, ptr %95, align 8, !tbaa !8
  br label %97

97:                                               ; preds = %97, %93
  %98 = phi i64 [ 0, %93 ], [ %106, %97 ]
  %99 = phi double [ %96, %93 ], [ %105, %97 ]
  %100 = getelementptr inbounds [400 x double], ptr %3, i64 %94, i64 %98
  %101 = load double, ptr %100, align 8, !tbaa !8
  %102 = fmul double %101, 1.500000e+00
  %103 = getelementptr inbounds double, ptr %9, i64 %98
  %104 = load double, ptr %103, align 8, !tbaa !8
  %105 = tail call double @llvm.fmuladd.f64(double %102, double %104, double %99)
  store double %105, ptr %95, align 8, !tbaa !8
  %106 = add nuw nsw i64 %98, 1
  %107 = icmp eq i64 %106, 400
  br i1 %107, label %108, label %97, !llvm.loop !18

108:                                              ; preds = %97
  %109 = add nuw nsw i64 %94, 1
  %110 = icmp eq i64 %109, 400
  br i1 %110, label %111, label %93, !llvm.loop !19

111:                                              ; preds = %108
  %112 = load ptr, ptr @stderr, align 8, !tbaa !20
  %113 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %112, ptr noundef nonnull @.str) #4
  %114 = load ptr, ptr @stderr, align 8, !tbaa !20
  %115 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %114, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %116

116:                                              ; preds = %124, %111
  %117 = phi i64 [ 0, %111 ], [ %129, %124 ]
  %118 = trunc i64 %117 to i16
  %119 = urem i16 %118, 20
  %120 = icmp eq i16 %119, 0
  br i1 %120, label %121, label %124

121:                                              ; preds = %116
  %122 = load ptr, ptr @stderr, align 8, !tbaa !20
  %123 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %122, ptr noundef nonnull @.str.3) #4
  br label %124

124:                                              ; preds = %121, %116
  %125 = load ptr, ptr @stderr, align 8, !tbaa !20
  %126 = getelementptr inbounds double, ptr %8, i64 %117
  %127 = load double, ptr %126, align 8, !tbaa !8
  %128 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %125, ptr noundef nonnull @.str.4, double noundef %127) #4
  %129 = add nuw nsw i64 %117, 1
  %130 = icmp eq i64 %129, 400
  br i1 %130, label %131, label %116, !llvm.loop !22

131:                                              ; preds = %124
  %132 = load ptr, ptr @stderr, align 8, !tbaa !20
  %133 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %132, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %134 = load ptr, ptr @stderr, align 8, !tbaa !20
  %135 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %134, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  tail call void @free(ptr noundef %7) #4
  tail call void @free(ptr noundef nonnull %8) #4
  tail call void @free(ptr noundef %9) #4
  tail call void @free(ptr noundef %10) #4
  tail call void @free(ptr noundef %11) #4
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
!17 = distinct !{!17, !6, !7}
!18 = distinct !{!18, !6, !7}
!19 = distinct !{!19, !6, !7}
!20 = !{!21, !21, i64 0}
!21 = !{!"any pointer", !10, i64 0}
!22 = distinct !{!22, !6, !7}

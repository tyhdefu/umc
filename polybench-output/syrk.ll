; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syrk/syrk.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syrk/syrk.c"
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
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  br label %5

5:                                                ; preds = %18, %2
  %6 = phi i64 [ 0, %2 ], [ %19, %18 ]
  br label %7

7:                                                ; preds = %7, %5
  %8 = phi i64 [ 0, %5 ], [ %16, %7 ]
  %9 = mul nuw nsw i64 %8, %6
  %10 = trunc i64 %9 to i32
  %11 = add i32 %10, 1
  %12 = urem i32 %11, 240
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 2.400000e+02
  %15 = getelementptr inbounds [200 x double], ptr %4, i64 %6, i64 %8
  store double %14, ptr %15, align 8, !tbaa !5
  %16 = add nuw nsw i64 %8, 1
  %17 = icmp eq i64 %16, 200
  br i1 %17, label %18, label %7, !llvm.loop !9

18:                                               ; preds = %7
  %19 = add nuw nsw i64 %6, 1
  %20 = icmp eq i64 %19, 240
  br i1 %20, label %21, label %5, !llvm.loop !12

21:                                               ; preds = %18, %34
  %22 = phi i64 [ %35, %34 ], [ 0, %18 ]
  br label %23

23:                                               ; preds = %23, %21
  %24 = phi i64 [ 0, %21 ], [ %32, %23 ]
  %25 = mul nuw nsw i64 %24, %22
  %26 = trunc i64 %25 to i32
  %27 = add i32 %26, 2
  %28 = urem i32 %27, 200
  %29 = uitofp nneg i32 %28 to double
  %30 = fdiv double %29, 2.000000e+02
  %31 = getelementptr inbounds [240 x double], ptr %3, i64 %22, i64 %24
  store double %30, ptr %31, align 8, !tbaa !5
  %32 = add nuw nsw i64 %24, 1
  %33 = icmp eq i64 %32, 240
  br i1 %33, label %34, label %23, !llvm.loop !13

34:                                               ; preds = %23
  %35 = add nuw nsw i64 %22, 1
  %36 = icmp eq i64 %35, 240
  br i1 %36, label %37, label %21, !llvm.loop !14

37:                                               ; preds = %34, %64
  %38 = phi i64 [ %65, %64 ], [ 0, %34 ]
  %39 = phi i64 [ %66, %64 ], [ 1, %34 ]
  br label %40

40:                                               ; preds = %40, %37
  %41 = phi i64 [ 0, %37 ], [ %45, %40 ]
  %42 = getelementptr inbounds [240 x double], ptr %3, i64 %38, i64 %41
  %43 = load double, ptr %42, align 8, !tbaa !5
  %44 = fmul double %43, 1.200000e+00
  store double %44, ptr %42, align 8, !tbaa !5
  %45 = add nuw nsw i64 %41, 1
  %46 = icmp eq i64 %45, %39
  br i1 %46, label %47, label %40, !llvm.loop !15

47:                                               ; preds = %40, %61
  %48 = phi i64 [ %62, %61 ], [ 0, %40 ]
  %49 = getelementptr inbounds [200 x double], ptr %4, i64 %38, i64 %48
  br label %50

50:                                               ; preds = %50, %47
  %51 = phi i64 [ 0, %47 ], [ %59, %50 ]
  %52 = load double, ptr %49, align 8, !tbaa !5
  %53 = fmul double %52, 1.500000e+00
  %54 = getelementptr inbounds [200 x double], ptr %4, i64 %51, i64 %48
  %55 = load double, ptr %54, align 8, !tbaa !5
  %56 = getelementptr inbounds [240 x double], ptr %3, i64 %38, i64 %51
  %57 = load double, ptr %56, align 8, !tbaa !5
  %58 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %57)
  store double %58, ptr %56, align 8, !tbaa !5
  %59 = add nuw nsw i64 %51, 1
  %60 = icmp eq i64 %59, %39
  br i1 %60, label %61, label %50, !llvm.loop !16

61:                                               ; preds = %50
  %62 = add nuw nsw i64 %48, 1
  %63 = icmp eq i64 %62, 200
  br i1 %63, label %64, label %47, !llvm.loop !17

64:                                               ; preds = %61
  %65 = add nuw nsw i64 %38, 1
  %66 = add nuw nsw i64 %39, 1
  %67 = icmp eq i64 %65, 240
  br i1 %67, label %68, label %37, !llvm.loop !18

68:                                               ; preds = %64
  %69 = load ptr, ptr @stderr, align 8, !tbaa !19
  %70 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %69, ptr noundef nonnull @.str) #4
  %71 = load ptr, ptr @stderr, align 8, !tbaa !19
  %72 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %71, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %73

73:                                               ; preds = %92, %68
  %74 = phi i64 [ 0, %68 ], [ %93, %92 ]
  %75 = mul nuw nsw i64 %74, 240
  br label %76

76:                                               ; preds = %85, %73
  %77 = phi i64 [ 0, %73 ], [ %90, %85 ]
  %78 = add nuw nsw i64 %77, %75
  %79 = trunc nuw nsw i64 %78 to i32
  %80 = urem i32 %79, 20
  %81 = icmp eq i32 %80, 0
  br i1 %81, label %82, label %85

82:                                               ; preds = %76
  %83 = load ptr, ptr @stderr, align 8, !tbaa !19
  %84 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str.3) #4
  br label %85

85:                                               ; preds = %82, %76
  %86 = load ptr, ptr @stderr, align 8, !tbaa !19
  %87 = getelementptr inbounds [240 x double], ptr %3, i64 %74, i64 %77
  %88 = load double, ptr %87, align 8, !tbaa !5
  %89 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %86, ptr noundef nonnull @.str.4, double noundef %88) #4
  %90 = add nuw nsw i64 %77, 1
  %91 = icmp eq i64 %90, 240
  br i1 %91, label %92, label %76, !llvm.loop !21

92:                                               ; preds = %85
  %93 = add nuw nsw i64 %74, 1
  %94 = icmp eq i64 %93, 240
  br i1 %94, label %95, label %73, !llvm.loop !22

95:                                               ; preds = %92
  %96 = load ptr, ptr @stderr, align 8, !tbaa !19
  %97 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %96, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %98 = load ptr, ptr @stderr, align 8, !tbaa !19
  %99 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %98, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef nonnull %3) #4
  tail call void @free(ptr noundef %4) #4
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

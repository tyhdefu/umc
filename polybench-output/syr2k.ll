; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syr2k/syr2k.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syr2k/syr2k.c"
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
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %24, %2
  %7 = phi i64 [ 0, %2 ], [ %25, %24 ]
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi i64 [ 0, %6 ], [ %22, %8 ]
  %10 = mul nuw nsw i64 %9, %7
  %11 = trunc i64 %10 to i32
  %12 = add i32 %11, 1
  %13 = urem i32 %12, 240
  %14 = uitofp nneg i32 %13 to double
  %15 = fdiv double %14, 2.400000e+02
  %16 = getelementptr inbounds [200 x double], ptr %4, i64 %7, i64 %9
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = add i32 %11, 2
  %18 = urem i32 %17, 200
  %19 = uitofp nneg i32 %18 to double
  %20 = fdiv double %19, 2.000000e+02
  %21 = getelementptr inbounds [200 x double], ptr %5, i64 %7, i64 %9
  store double %20, ptr %21, align 8, !tbaa !5
  %22 = add nuw nsw i64 %9, 1
  %23 = icmp eq i64 %22, 200
  br i1 %23, label %24, label %8, !llvm.loop !9

24:                                               ; preds = %8
  %25 = add nuw nsw i64 %7, 1
  %26 = icmp eq i64 %25, 240
  br i1 %26, label %27, label %6, !llvm.loop !12

27:                                               ; preds = %24, %40
  %28 = phi i64 [ %41, %40 ], [ 0, %24 ]
  br label %29

29:                                               ; preds = %29, %27
  %30 = phi i64 [ 0, %27 ], [ %38, %29 ]
  %31 = mul nuw nsw i64 %30, %28
  %32 = trunc i64 %31 to i32
  %33 = add i32 %32, 3
  %34 = urem i32 %33, 240
  %35 = uitofp nneg i32 %34 to double
  %36 = fdiv double %35, 2.000000e+02
  %37 = getelementptr inbounds [240 x double], ptr %3, i64 %28, i64 %30
  store double %36, ptr %37, align 8, !tbaa !5
  %38 = add nuw nsw i64 %30, 1
  %39 = icmp eq i64 %38, 240
  br i1 %39, label %40, label %29, !llvm.loop !13

40:                                               ; preds = %29
  %41 = add nuw nsw i64 %28, 1
  %42 = icmp eq i64 %41, 240
  br i1 %42, label %43, label %27, !llvm.loop !14

43:                                               ; preds = %40, %77
  %44 = phi i64 [ %78, %77 ], [ 0, %40 ]
  %45 = phi i64 [ %79, %77 ], [ 1, %40 ]
  br label %46

46:                                               ; preds = %46, %43
  %47 = phi i64 [ 0, %43 ], [ %51, %46 ]
  %48 = getelementptr inbounds [240 x double], ptr %3, i64 %44, i64 %47
  %49 = load double, ptr %48, align 8, !tbaa !5
  %50 = fmul double %49, 1.200000e+00
  store double %50, ptr %48, align 8, !tbaa !5
  %51 = add nuw nsw i64 %47, 1
  %52 = icmp eq i64 %51, %45
  br i1 %52, label %53, label %46, !llvm.loop !15

53:                                               ; preds = %46, %74
  %54 = phi i64 [ %75, %74 ], [ 0, %46 ]
  %55 = getelementptr inbounds [200 x double], ptr %5, i64 %44, i64 %54
  %56 = getelementptr inbounds [200 x double], ptr %4, i64 %44, i64 %54
  br label %57

57:                                               ; preds = %57, %53
  %58 = phi i64 [ 0, %53 ], [ %72, %57 ]
  %59 = getelementptr inbounds [200 x double], ptr %4, i64 %58, i64 %54
  %60 = load double, ptr %59, align 8, !tbaa !5
  %61 = fmul double %60, 1.500000e+00
  %62 = load double, ptr %55, align 8, !tbaa !5
  %63 = getelementptr inbounds [200 x double], ptr %5, i64 %58, i64 %54
  %64 = load double, ptr %63, align 8, !tbaa !5
  %65 = fmul double %64, 1.500000e+00
  %66 = load double, ptr %56, align 8, !tbaa !5
  %67 = fmul double %65, %66
  %68 = tail call double @llvm.fmuladd.f64(double %61, double %62, double %67)
  %69 = getelementptr inbounds [240 x double], ptr %3, i64 %44, i64 %58
  %70 = load double, ptr %69, align 8, !tbaa !5
  %71 = fadd double %70, %68
  store double %71, ptr %69, align 8, !tbaa !5
  %72 = add nuw nsw i64 %58, 1
  %73 = icmp eq i64 %72, %45
  br i1 %73, label %74, label %57, !llvm.loop !16

74:                                               ; preds = %57
  %75 = add nuw nsw i64 %54, 1
  %76 = icmp eq i64 %75, 200
  br i1 %76, label %77, label %53, !llvm.loop !17

77:                                               ; preds = %74
  %78 = add nuw nsw i64 %44, 1
  %79 = add nuw nsw i64 %45, 1
  %80 = icmp eq i64 %78, 240
  br i1 %80, label %81, label %43, !llvm.loop !18

81:                                               ; preds = %77
  %82 = load ptr, ptr @stderr, align 8, !tbaa !19
  %83 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %82, ptr noundef nonnull @.str) #4
  %84 = load ptr, ptr @stderr, align 8, !tbaa !19
  %85 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %84, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %86

86:                                               ; preds = %105, %81
  %87 = phi i64 [ 0, %81 ], [ %106, %105 ]
  %88 = mul nuw nsw i64 %87, 240
  br label %89

89:                                               ; preds = %98, %86
  %90 = phi i64 [ 0, %86 ], [ %103, %98 ]
  %91 = add nuw nsw i64 %90, %88
  %92 = trunc nuw nsw i64 %91 to i32
  %93 = urem i32 %92, 20
  %94 = icmp eq i32 %93, 0
  br i1 %94, label %95, label %98

95:                                               ; preds = %89
  %96 = load ptr, ptr @stderr, align 8, !tbaa !19
  %97 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %96, ptr noundef nonnull @.str.3) #4
  br label %98

98:                                               ; preds = %95, %89
  %99 = load ptr, ptr @stderr, align 8, !tbaa !19
  %100 = getelementptr inbounds [240 x double], ptr %3, i64 %87, i64 %90
  %101 = load double, ptr %100, align 8, !tbaa !5
  %102 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %99, ptr noundef nonnull @.str.4, double noundef %101) #4
  %103 = add nuw nsw i64 %90, 1
  %104 = icmp eq i64 %103, 240
  br i1 %104, label %105, label %89, !llvm.loop !21

105:                                              ; preds = %98
  %106 = add nuw nsw i64 %87, 1
  %107 = icmp eq i64 %106, 240
  br i1 %107, label %108, label %86, !llvm.loop !22

108:                                              ; preds = %105
  %109 = load ptr, ptr @stderr, align 8, !tbaa !19
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %111 = load ptr, ptr @stderr, align 8, !tbaa !19
  %112 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %111, ptr noundef nonnull @.str.6) #4
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

; ModuleID = '/home/lepton/PolyBenchC-4.2.1/datamining/correlation/correlation.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/datamining/correlation/correlation.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"corr\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62400, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  br label %7

7:                                                ; preds = %21, %2
  %8 = phi i64 [ 0, %2 ], [ %22, %21 ]
  %9 = trunc nuw nsw i64 %8 to i32
  %10 = uitofp nneg i32 %9 to double
  br label %11

11:                                               ; preds = %11, %7
  %12 = phi i64 [ 0, %7 ], [ %19, %11 ]
  %13 = mul nuw nsw i64 %12, %8
  %14 = trunc nuw nsw i64 %13 to i32
  %15 = uitofp nneg i32 %14 to double
  %16 = fdiv double %15, 2.400000e+02
  %17 = fadd double %16, %10
  %18 = getelementptr inbounds [240 x double], ptr %3, i64 %8, i64 %12
  store double %17, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %12, 1
  %20 = icmp eq i64 %19, 240
  br i1 %20, label %21, label %11, !llvm.loop !9

21:                                               ; preds = %11
  %22 = add nuw nsw i64 %8, 1
  %23 = icmp eq i64 %22, 260
  br i1 %23, label %24, label %7, !llvm.loop !12

24:                                               ; preds = %21, %35
  %25 = phi i64 [ %37, %35 ], [ 0, %21 ]
  %26 = getelementptr inbounds double, ptr %5, i64 %25
  store double 0.000000e+00, ptr %26, align 8, !tbaa !5
  br label %27

27:                                               ; preds = %27, %24
  %28 = phi i64 [ 0, %24 ], [ %33, %27 ]
  %29 = phi double [ 0.000000e+00, %24 ], [ %32, %27 ]
  %30 = getelementptr inbounds [240 x double], ptr %3, i64 %28, i64 %25
  %31 = load double, ptr %30, align 8, !tbaa !5
  %32 = fadd double %29, %31
  store double %32, ptr %26, align 8, !tbaa !5
  %33 = add nuw nsw i64 %28, 1
  %34 = icmp eq i64 %33, 260
  br i1 %34, label %35, label %27, !llvm.loop !13

35:                                               ; preds = %27
  %36 = fdiv double %32, 2.600000e+02
  store double %36, ptr %26, align 8, !tbaa !5
  %37 = add nuw nsw i64 %25, 1
  %38 = icmp eq i64 %37, 240
  br i1 %38, label %39, label %24, !llvm.loop !14

39:                                               ; preds = %35, %53
  %40 = phi i64 [ %58, %53 ], [ 0, %35 ]
  %41 = getelementptr inbounds double, ptr %6, i64 %40
  store double 0.000000e+00, ptr %41, align 8, !tbaa !5
  %42 = getelementptr inbounds double, ptr %5, i64 %40
  br label %43

43:                                               ; preds = %43, %39
  %44 = phi double [ 0.000000e+00, %39 ], [ %50, %43 ]
  %45 = phi i64 [ 0, %39 ], [ %51, %43 ]
  %46 = getelementptr inbounds [240 x double], ptr %3, i64 %45, i64 %40
  %47 = load double, ptr %46, align 8, !tbaa !5
  %48 = load double, ptr %42, align 8, !tbaa !5
  %49 = fsub double %47, %48
  %50 = tail call double @llvm.fmuladd.f64(double %49, double %49, double %44)
  store double %50, ptr %41, align 8, !tbaa !5
  %51 = add nuw nsw i64 %45, 1
  %52 = icmp eq i64 %51, 260
  br i1 %52, label %53, label %43, !llvm.loop !15

53:                                               ; preds = %43
  %54 = fdiv double %50, 2.600000e+02
  store double %54, ptr %41, align 8, !tbaa !5
  %55 = tail call double @sqrt(double noundef %54) #4
  %56 = fcmp ugt double %55, 1.000000e-01
  %57 = select i1 %56, double %55, double 1.000000e+00
  store double %57, ptr %41, align 8, !tbaa !5
  %58 = add nuw nsw i64 %40, 1
  %59 = icmp eq i64 %58, 240
  br i1 %59, label %60, label %39, !llvm.loop !16

60:                                               ; preds = %53, %77
  %61 = phi i64 [ %78, %77 ], [ 0, %53 ]
  br label %62

62:                                               ; preds = %62, %60
  %63 = phi i64 [ 0, %60 ], [ %75, %62 ]
  %64 = getelementptr inbounds double, ptr %5, i64 %63
  %65 = load double, ptr %64, align 8, !tbaa !5
  %66 = getelementptr inbounds [240 x double], ptr %3, i64 %61, i64 %63
  %67 = load double, ptr %66, align 8, !tbaa !5
  %68 = fsub double %67, %65
  store double %68, ptr %66, align 8, !tbaa !5
  %69 = tail call double @sqrt(double noundef 2.600000e+02) #4
  %70 = getelementptr inbounds double, ptr %6, i64 %63
  %71 = load double, ptr %70, align 8, !tbaa !5
  %72 = fmul double %69, %71
  %73 = load double, ptr %66, align 8, !tbaa !5
  %74 = fdiv double %73, %72
  store double %74, ptr %66, align 8, !tbaa !5
  %75 = add nuw nsw i64 %63, 1
  %76 = icmp eq i64 %75, 240
  br i1 %76, label %77, label %62, !llvm.loop !17

77:                                               ; preds = %62
  %78 = add nuw nsw i64 %61, 1
  %79 = icmp eq i64 %78, 260
  br i1 %79, label %84, label %60, !llvm.loop !18

80:                                               ; preds = %101
  %81 = add nuw nsw i64 %85, 1
  %82 = add nuw nsw i64 %86, 1
  %83 = icmp eq i64 %81, 239
  br i1 %83, label %105, label %84, !llvm.loop !19

84:                                               ; preds = %77, %80
  %85 = phi i64 [ %81, %80 ], [ 0, %77 ]
  %86 = phi i64 [ %82, %80 ], [ 1, %77 ]
  %87 = getelementptr inbounds [240 x double], ptr %4, i64 %85, i64 %85
  store double 1.000000e+00, ptr %87, align 8, !tbaa !5
  br label %88

88:                                               ; preds = %101, %84
  %89 = phi i64 [ %86, %84 ], [ %103, %101 ]
  %90 = getelementptr inbounds [240 x double], ptr %4, i64 %85, i64 %89
  store double 0.000000e+00, ptr %90, align 8, !tbaa !5
  br label %91

91:                                               ; preds = %91, %88
  %92 = phi i64 [ 0, %88 ], [ %99, %91 ]
  %93 = phi double [ 0.000000e+00, %88 ], [ %98, %91 ]
  %94 = getelementptr inbounds [240 x double], ptr %3, i64 %92, i64 %85
  %95 = load double, ptr %94, align 8, !tbaa !5
  %96 = getelementptr inbounds [240 x double], ptr %3, i64 %92, i64 %89
  %97 = load double, ptr %96, align 8, !tbaa !5
  %98 = tail call double @llvm.fmuladd.f64(double %95, double %97, double %93)
  store double %98, ptr %90, align 8, !tbaa !5
  %99 = add nuw nsw i64 %92, 1
  %100 = icmp eq i64 %99, 260
  br i1 %100, label %101, label %91, !llvm.loop !20

101:                                              ; preds = %91
  %102 = getelementptr inbounds [240 x double], ptr %4, i64 %89, i64 %85
  store double %98, ptr %102, align 8, !tbaa !5
  %103 = add nuw nsw i64 %89, 1
  %104 = icmp eq i64 %103, 240
  br i1 %104, label %80, label %88, !llvm.loop !21

105:                                              ; preds = %80
  %106 = getelementptr inbounds i8, ptr %4, i64 460792
  store double 1.000000e+00, ptr %106, align 8, !tbaa !5
  %107 = load ptr, ptr @stderr, align 8, !tbaa !22
  %108 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %107, ptr noundef nonnull @.str) #4
  %109 = load ptr, ptr @stderr, align 8, !tbaa !22
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %111

111:                                              ; preds = %130, %105
  %112 = phi i64 [ 0, %105 ], [ %131, %130 ]
  %113 = mul nuw nsw i64 %112, 240
  br label %114

114:                                              ; preds = %123, %111
  %115 = phi i64 [ 0, %111 ], [ %128, %123 ]
  %116 = add nuw nsw i64 %115, %113
  %117 = trunc nuw nsw i64 %116 to i32
  %118 = urem i32 %117, 20
  %119 = icmp eq i32 %118, 0
  br i1 %119, label %120, label %123

120:                                              ; preds = %114
  %121 = load ptr, ptr @stderr, align 8, !tbaa !22
  %122 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %121, ptr noundef nonnull @.str.3) #4
  br label %123

123:                                              ; preds = %120, %114
  %124 = load ptr, ptr @stderr, align 8, !tbaa !22
  %125 = getelementptr inbounds [240 x double], ptr %4, i64 %112, i64 %115
  %126 = load double, ptr %125, align 8, !tbaa !5
  %127 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %124, ptr noundef nonnull @.str.4, double noundef %126) #4
  %128 = add nuw nsw i64 %115, 1
  %129 = icmp eq i64 %128, 240
  br i1 %129, label %130, label %114, !llvm.loop !24

130:                                              ; preds = %123
  %131 = add nuw nsw i64 %112, 1
  %132 = icmp eq i64 %131, 240
  br i1 %132, label %133, label %111, !llvm.loop !25

133:                                              ; preds = %130
  %134 = load ptr, ptr @stderr, align 8, !tbaa !22
  %135 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %134, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %136 = load ptr, ptr @stderr, align 8, !tbaa !22
  %137 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %136, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef nonnull %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

; Function Attrs: nounwind
declare double @sqrt(double noundef) local_unnamed_addr #2

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
!21 = distinct !{!21, !10, !11}
!22 = !{!23, !23, i64 0}
!23 = !{!"any pointer", !7, i64 0}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}

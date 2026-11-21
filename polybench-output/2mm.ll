; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/2mm/2mm.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/2mm/2mm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"D\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 34200, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 37800, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 39900, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 41800, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 39600, i32 noundef 8) #4
  br label %8

8:                                                ; preds = %21, %2
  %9 = phi i64 [ 0, %2 ], [ %22, %21 ]
  br label %10

10:                                               ; preds = %10, %8
  %11 = phi i64 [ 0, %8 ], [ %19, %10 ]
  %12 = mul nuw nsw i64 %11, %9
  %13 = trunc i64 %12 to i32
  %14 = add i32 %13, 1
  %15 = urem i32 %14, 180
  %16 = uitofp nneg i32 %15 to double
  %17 = fdiv double %16, 1.800000e+02
  %18 = getelementptr inbounds [210 x double], ptr %4, i64 %9, i64 %11
  store double %17, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %11, 1
  %20 = icmp eq i64 %19, 210
  br i1 %20, label %21, label %10, !llvm.loop !9

21:                                               ; preds = %10
  %22 = add nuw nsw i64 %9, 1
  %23 = icmp eq i64 %22, 180
  br i1 %23, label %24, label %8, !llvm.loop !12

24:                                               ; preds = %21, %36
  %25 = phi i64 [ %37, %36 ], [ 0, %21 ]
  br label %26

26:                                               ; preds = %26, %24
  %27 = phi i64 [ 0, %24 ], [ %28, %26 ]
  %28 = add nuw nsw i64 %27, 1
  %29 = mul nuw nsw i64 %28, %25
  %30 = trunc nuw nsw i64 %29 to i32
  %31 = urem i32 %30, 190
  %32 = uitofp nneg i32 %31 to double
  %33 = fdiv double %32, 1.900000e+02
  %34 = getelementptr inbounds [190 x double], ptr %5, i64 %25, i64 %27
  store double %33, ptr %34, align 8, !tbaa !5
  %35 = icmp eq i64 %28, 190
  br i1 %35, label %36, label %26, !llvm.loop !13

36:                                               ; preds = %26
  %37 = add nuw nsw i64 %25, 1
  %38 = icmp eq i64 %37, 210
  br i1 %38, label %39, label %24, !llvm.loop !14

39:                                               ; preds = %36, %53
  %40 = phi i64 [ %54, %53 ], [ 0, %36 ]
  br label %41

41:                                               ; preds = %41, %39
  %42 = phi i64 [ 0, %39 ], [ %51, %41 ]
  %43 = add nuw nsw i64 %42, 3
  %44 = mul nuw nsw i64 %43, %40
  %45 = trunc i64 %44 to i32
  %46 = add i32 %45, 1
  %47 = urem i32 %46, 220
  %48 = uitofp nneg i32 %47 to double
  %49 = fdiv double %48, 2.200000e+02
  %50 = getelementptr inbounds [220 x double], ptr %6, i64 %40, i64 %42
  store double %49, ptr %50, align 8, !tbaa !5
  %51 = add nuw nsw i64 %42, 1
  %52 = icmp eq i64 %51, 220
  br i1 %52, label %53, label %41, !llvm.loop !15

53:                                               ; preds = %41
  %54 = add nuw nsw i64 %40, 1
  %55 = icmp eq i64 %54, 190
  br i1 %55, label %56, label %39, !llvm.loop !16

56:                                               ; preds = %53, %69
  %57 = phi i64 [ %70, %69 ], [ 0, %53 ]
  br label %58

58:                                               ; preds = %58, %56
  %59 = phi i64 [ 0, %56 ], [ %67, %58 ]
  %60 = add nuw nsw i64 %59, 2
  %61 = mul nuw nsw i64 %60, %57
  %62 = trunc nuw nsw i64 %61 to i32
  %63 = urem i32 %62, 210
  %64 = uitofp nneg i32 %63 to double
  %65 = fdiv double %64, 2.100000e+02
  %66 = getelementptr inbounds [220 x double], ptr %7, i64 %57, i64 %59
  store double %65, ptr %66, align 8, !tbaa !5
  %67 = add nuw nsw i64 %59, 1
  %68 = icmp eq i64 %67, 220
  br i1 %68, label %69, label %58, !llvm.loop !17

69:                                               ; preds = %58
  %70 = add nuw nsw i64 %57, 1
  %71 = icmp eq i64 %70, 180
  br i1 %71, label %72, label %56, !llvm.loop !18

72:                                               ; preds = %69, %91
  %73 = phi i64 [ %92, %91 ], [ 0, %69 ]
  br label %74

74:                                               ; preds = %88, %72
  %75 = phi i64 [ 0, %72 ], [ %89, %88 ]
  %76 = getelementptr inbounds [190 x double], ptr %3, i64 %73, i64 %75
  store double 0.000000e+00, ptr %76, align 8, !tbaa !5
  br label %77

77:                                               ; preds = %77, %74
  %78 = phi i64 [ 0, %74 ], [ %86, %77 ]
  %79 = phi double [ 0.000000e+00, %74 ], [ %85, %77 ]
  %80 = getelementptr inbounds [210 x double], ptr %4, i64 %73, i64 %78
  %81 = load double, ptr %80, align 8, !tbaa !5
  %82 = fmul double %81, 1.500000e+00
  %83 = getelementptr inbounds [190 x double], ptr %5, i64 %78, i64 %75
  %84 = load double, ptr %83, align 8, !tbaa !5
  %85 = tail call double @llvm.fmuladd.f64(double %82, double %84, double %79)
  store double %85, ptr %76, align 8, !tbaa !5
  %86 = add nuw nsw i64 %78, 1
  %87 = icmp eq i64 %86, 210
  br i1 %87, label %88, label %77, !llvm.loop !19

88:                                               ; preds = %77
  %89 = add nuw nsw i64 %75, 1
  %90 = icmp eq i64 %89, 190
  br i1 %90, label %91, label %74, !llvm.loop !20

91:                                               ; preds = %88
  %92 = add nuw nsw i64 %73, 1
  %93 = icmp eq i64 %92, 180
  br i1 %93, label %94, label %72, !llvm.loop !21

94:                                               ; preds = %91, %114
  %95 = phi i64 [ %115, %114 ], [ 0, %91 ]
  br label %96

96:                                               ; preds = %111, %94
  %97 = phi i64 [ 0, %94 ], [ %112, %111 ]
  %98 = getelementptr inbounds [220 x double], ptr %7, i64 %95, i64 %97
  %99 = load double, ptr %98, align 8, !tbaa !5
  %100 = fmul double %99, 1.200000e+00
  store double %100, ptr %98, align 8, !tbaa !5
  br label %101

101:                                              ; preds = %101, %96
  %102 = phi i64 [ 0, %96 ], [ %109, %101 ]
  %103 = phi double [ %100, %96 ], [ %108, %101 ]
  %104 = getelementptr inbounds [190 x double], ptr %3, i64 %95, i64 %102
  %105 = load double, ptr %104, align 8, !tbaa !5
  %106 = getelementptr inbounds [220 x double], ptr %6, i64 %102, i64 %97
  %107 = load double, ptr %106, align 8, !tbaa !5
  %108 = tail call double @llvm.fmuladd.f64(double %105, double %107, double %103)
  store double %108, ptr %98, align 8, !tbaa !5
  %109 = add nuw nsw i64 %102, 1
  %110 = icmp eq i64 %109, 190
  br i1 %110, label %111, label %101, !llvm.loop !22

111:                                              ; preds = %101
  %112 = add nuw nsw i64 %97, 1
  %113 = icmp eq i64 %112, 220
  br i1 %113, label %114, label %96, !llvm.loop !23

114:                                              ; preds = %111
  %115 = add nuw nsw i64 %95, 1
  %116 = icmp eq i64 %115, 180
  br i1 %116, label %117, label %94, !llvm.loop !24

117:                                              ; preds = %114
  %118 = load ptr, ptr @stderr, align 8, !tbaa !25
  %119 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %118, ptr noundef nonnull @.str) #4
  %120 = load ptr, ptr @stderr, align 8, !tbaa !25
  %121 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %120, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %122

122:                                              ; preds = %141, %117
  %123 = phi i64 [ 0, %117 ], [ %142, %141 ]
  %124 = mul nuw nsw i64 %123, 180
  br label %125

125:                                              ; preds = %134, %122
  %126 = phi i64 [ 0, %122 ], [ %139, %134 ]
  %127 = add nuw nsw i64 %126, %124
  %128 = trunc nuw nsw i64 %127 to i32
  %129 = urem i32 %128, 20
  %130 = icmp eq i32 %129, 0
  br i1 %130, label %131, label %134

131:                                              ; preds = %125
  %132 = load ptr, ptr @stderr, align 8, !tbaa !25
  %133 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %132, ptr noundef nonnull @.str.3) #4
  br label %134

134:                                              ; preds = %131, %125
  %135 = load ptr, ptr @stderr, align 8, !tbaa !25
  %136 = getelementptr inbounds [220 x double], ptr %7, i64 %123, i64 %126
  %137 = load double, ptr %136, align 8, !tbaa !5
  %138 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %135, ptr noundef nonnull @.str.4, double noundef %137) #4
  %139 = add nuw nsw i64 %126, 1
  %140 = icmp eq i64 %139, 220
  br i1 %140, label %141, label %125, !llvm.loop !27

141:                                              ; preds = %134
  %142 = add nuw nsw i64 %123, 1
  %143 = icmp eq i64 %142, 180
  br i1 %143, label %144, label %122, !llvm.loop !28

144:                                              ; preds = %141
  %145 = load ptr, ptr @stderr, align 8, !tbaa !25
  %146 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %145, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %147 = load ptr, ptr @stderr, align 8, !tbaa !25
  %148 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %147, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  tail call void @free(ptr noundef nonnull %7) #4
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
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = !{!26, !26, i64 0}
!26 = !{!"any pointer", !7, i64 0}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}

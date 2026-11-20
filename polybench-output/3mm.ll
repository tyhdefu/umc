; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/3mm/3mm.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/3mm/3mm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"G\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 34200, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 36000, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 38000, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 39900, i32 noundef 8) #4
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 41800, i32 noundef 8) #4
  %8 = tail call ptr @polybench_alloc_data(i64 noundef 46200, i32 noundef 8) #4
  %9 = tail call ptr @polybench_alloc_data(i64 noundef 37800, i32 noundef 8) #4
  br label %10

10:                                               ; preds = %23, %2
  %11 = phi i64 [ 0, %2 ], [ %24, %23 ]
  br label %12

12:                                               ; preds = %12, %10
  %13 = phi i64 [ 0, %10 ], [ %21, %12 ]
  %14 = mul nuw nsw i64 %13, %11
  %15 = trunc i64 %14 to i32
  %16 = add i32 %15, 1
  %17 = urem i32 %16, 180
  %18 = uitofp nneg i32 %17 to double
  %19 = fdiv double %18, 9.000000e+02
  %20 = getelementptr inbounds [200 x double], ptr %4, i64 %11, i64 %13
  store double %19, ptr %20, align 8, !tbaa !5
  %21 = add nuw nsw i64 %13, 1
  %22 = icmp eq i64 %21, 200
  br i1 %22, label %23, label %12, !llvm.loop !9

23:                                               ; preds = %12
  %24 = add nuw nsw i64 %11, 1
  %25 = icmp eq i64 %24, 180
  br i1 %25, label %26, label %10, !llvm.loop !12

26:                                               ; preds = %23, %39
  %27 = phi i64 [ %40, %39 ], [ 0, %23 ]
  br label %28

28:                                               ; preds = %28, %26
  %29 = phi i64 [ 0, %26 ], [ %30, %28 ]
  %30 = add nuw nsw i64 %29, 1
  %31 = mul nuw nsw i64 %30, %27
  %32 = trunc i64 %31 to i32
  %33 = add i32 %32, 2
  %34 = urem i32 %33, 190
  %35 = uitofp nneg i32 %34 to double
  %36 = fdiv double %35, 9.500000e+02
  %37 = getelementptr inbounds [190 x double], ptr %5, i64 %27, i64 %29
  store double %36, ptr %37, align 8, !tbaa !5
  %38 = icmp eq i64 %30, 190
  br i1 %38, label %39, label %28, !llvm.loop !13

39:                                               ; preds = %28
  %40 = add nuw nsw i64 %27, 1
  %41 = icmp eq i64 %40, 200
  br i1 %41, label %42, label %26, !llvm.loop !14

42:                                               ; preds = %39, %55
  %43 = phi i64 [ %56, %55 ], [ 0, %39 ]
  br label %44

44:                                               ; preds = %44, %42
  %45 = phi i64 [ 0, %42 ], [ %53, %44 ]
  %46 = add nuw nsw i64 %45, 3
  %47 = mul nuw nsw i64 %46, %43
  %48 = trunc nuw nsw i64 %47 to i32
  %49 = urem i32 %48, 210
  %50 = uitofp nneg i32 %49 to double
  %51 = fdiv double %50, 1.050000e+03
  %52 = getelementptr inbounds [220 x double], ptr %7, i64 %43, i64 %45
  store double %51, ptr %52, align 8, !tbaa !5
  %53 = add nuw nsw i64 %45, 1
  %54 = icmp eq i64 %53, 220
  br i1 %54, label %55, label %44, !llvm.loop !15

55:                                               ; preds = %44
  %56 = add nuw nsw i64 %43, 1
  %57 = icmp eq i64 %56, 190
  br i1 %57, label %58, label %42, !llvm.loop !16

58:                                               ; preds = %55, %72
  %59 = phi i64 [ %73, %72 ], [ 0, %55 ]
  br label %60

60:                                               ; preds = %60, %58
  %61 = phi i64 [ 0, %58 ], [ %70, %60 ]
  %62 = add nuw nsw i64 %61, 2
  %63 = mul nuw nsw i64 %62, %59
  %64 = trunc i64 %63 to i32
  %65 = add i32 %64, 2
  %66 = urem i32 %65, 200
  %67 = uitofp nneg i32 %66 to double
  %68 = fdiv double %67, 1.000000e+03
  %69 = getelementptr inbounds [210 x double], ptr %8, i64 %59, i64 %61
  store double %68, ptr %69, align 8, !tbaa !5
  %70 = add nuw nsw i64 %61, 1
  %71 = icmp eq i64 %70, 210
  br i1 %71, label %72, label %60, !llvm.loop !17

72:                                               ; preds = %60
  %73 = add nuw nsw i64 %59, 1
  %74 = icmp eq i64 %73, 220
  br i1 %74, label %75, label %58, !llvm.loop !18

75:                                               ; preds = %72, %93
  %76 = phi i64 [ %94, %93 ], [ 0, %72 ]
  br label %77

77:                                               ; preds = %90, %75
  %78 = phi i64 [ 0, %75 ], [ %91, %90 ]
  %79 = getelementptr inbounds [190 x double], ptr %3, i64 %76, i64 %78
  store double 0.000000e+00, ptr %79, align 8, !tbaa !5
  br label %80

80:                                               ; preds = %80, %77
  %81 = phi i64 [ 0, %77 ], [ %88, %80 ]
  %82 = phi double [ 0.000000e+00, %77 ], [ %87, %80 ]
  %83 = getelementptr inbounds [200 x double], ptr %4, i64 %76, i64 %81
  %84 = load double, ptr %83, align 8, !tbaa !5
  %85 = getelementptr inbounds [190 x double], ptr %5, i64 %81, i64 %78
  %86 = load double, ptr %85, align 8, !tbaa !5
  %87 = tail call double @llvm.fmuladd.f64(double %84, double %86, double %82)
  store double %87, ptr %79, align 8, !tbaa !5
  %88 = add nuw nsw i64 %81, 1
  %89 = icmp eq i64 %88, 200
  br i1 %89, label %90, label %80, !llvm.loop !19

90:                                               ; preds = %80
  %91 = add nuw nsw i64 %78, 1
  %92 = icmp eq i64 %91, 190
  br i1 %92, label %93, label %77, !llvm.loop !20

93:                                               ; preds = %90
  %94 = add nuw nsw i64 %76, 1
  %95 = icmp eq i64 %94, 180
  br i1 %95, label %96, label %75, !llvm.loop !21

96:                                               ; preds = %93, %114
  %97 = phi i64 [ %115, %114 ], [ 0, %93 ]
  br label %98

98:                                               ; preds = %111, %96
  %99 = phi i64 [ 0, %96 ], [ %112, %111 ]
  %100 = getelementptr inbounds [210 x double], ptr %6, i64 %97, i64 %99
  store double 0.000000e+00, ptr %100, align 8, !tbaa !5
  br label %101

101:                                              ; preds = %101, %98
  %102 = phi i64 [ 0, %98 ], [ %109, %101 ]
  %103 = phi double [ 0.000000e+00, %98 ], [ %108, %101 ]
  %104 = getelementptr inbounds [220 x double], ptr %7, i64 %97, i64 %102
  %105 = load double, ptr %104, align 8, !tbaa !5
  %106 = getelementptr inbounds [210 x double], ptr %8, i64 %102, i64 %99
  %107 = load double, ptr %106, align 8, !tbaa !5
  %108 = tail call double @llvm.fmuladd.f64(double %105, double %107, double %103)
  store double %108, ptr %100, align 8, !tbaa !5
  %109 = add nuw nsw i64 %102, 1
  %110 = icmp eq i64 %109, 220
  br i1 %110, label %111, label %101, !llvm.loop !22

111:                                              ; preds = %101
  %112 = add nuw nsw i64 %99, 1
  %113 = icmp eq i64 %112, 210
  br i1 %113, label %114, label %98, !llvm.loop !23

114:                                              ; preds = %111
  %115 = add nuw nsw i64 %97, 1
  %116 = icmp eq i64 %115, 190
  br i1 %116, label %117, label %96, !llvm.loop !24

117:                                              ; preds = %114, %135
  %118 = phi i64 [ %136, %135 ], [ 0, %114 ]
  br label %119

119:                                              ; preds = %132, %117
  %120 = phi i64 [ 0, %117 ], [ %133, %132 ]
  %121 = getelementptr inbounds [210 x double], ptr %9, i64 %118, i64 %120
  store double 0.000000e+00, ptr %121, align 8, !tbaa !5
  br label %122

122:                                              ; preds = %122, %119
  %123 = phi i64 [ 0, %119 ], [ %130, %122 ]
  %124 = phi double [ 0.000000e+00, %119 ], [ %129, %122 ]
  %125 = getelementptr inbounds [190 x double], ptr %3, i64 %118, i64 %123
  %126 = load double, ptr %125, align 8, !tbaa !5
  %127 = getelementptr inbounds [210 x double], ptr %6, i64 %123, i64 %120
  %128 = load double, ptr %127, align 8, !tbaa !5
  %129 = tail call double @llvm.fmuladd.f64(double %126, double %128, double %124)
  store double %129, ptr %121, align 8, !tbaa !5
  %130 = add nuw nsw i64 %123, 1
  %131 = icmp eq i64 %130, 190
  br i1 %131, label %132, label %122, !llvm.loop !25

132:                                              ; preds = %122
  %133 = add nuw nsw i64 %120, 1
  %134 = icmp eq i64 %133, 210
  br i1 %134, label %135, label %119, !llvm.loop !26

135:                                              ; preds = %132
  %136 = add nuw nsw i64 %118, 1
  %137 = icmp eq i64 %136, 180
  br i1 %137, label %138, label %117, !llvm.loop !27

138:                                              ; preds = %135
  %139 = load ptr, ptr @stderr, align 8, !tbaa !28
  %140 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %139, ptr noundef nonnull @.str) #4
  %141 = load ptr, ptr @stderr, align 8, !tbaa !28
  %142 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %141, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %143

143:                                              ; preds = %162, %138
  %144 = phi i64 [ 0, %138 ], [ %163, %162 ]
  %145 = mul nuw nsw i64 %144, 180
  br label %146

146:                                              ; preds = %155, %143
  %147 = phi i64 [ 0, %143 ], [ %160, %155 ]
  %148 = add nuw nsw i64 %147, %145
  %149 = trunc nuw nsw i64 %148 to i32
  %150 = urem i32 %149, 20
  %151 = icmp eq i32 %150, 0
  br i1 %151, label %152, label %155

152:                                              ; preds = %146
  %153 = load ptr, ptr @stderr, align 8, !tbaa !28
  %154 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %153, ptr noundef nonnull @.str.3) #4
  br label %155

155:                                              ; preds = %152, %146
  %156 = load ptr, ptr @stderr, align 8, !tbaa !28
  %157 = getelementptr inbounds [210 x double], ptr %9, i64 %144, i64 %147
  %158 = load double, ptr %157, align 8, !tbaa !5
  %159 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %156, ptr noundef nonnull @.str.4, double noundef %158) #4
  %160 = add nuw nsw i64 %147, 1
  %161 = icmp eq i64 %160, 210
  br i1 %161, label %162, label %146, !llvm.loop !30

162:                                              ; preds = %155
  %163 = add nuw nsw i64 %144, 1
  %164 = icmp eq i64 %163, 180
  br i1 %164, label %165, label %143, !llvm.loop !31

165:                                              ; preds = %162
  %166 = load ptr, ptr @stderr, align 8, !tbaa !28
  %167 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %166, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %168 = load ptr, ptr @stderr, align 8, !tbaa !28
  %169 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %168, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  tail call void @free(ptr noundef %7) #4
  tail call void @free(ptr noundef %8) #4
  tail call void @free(ptr noundef nonnull %9) #4
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
!25 = distinct !{!25, !10, !11}
!26 = distinct !{!26, !10, !11}
!27 = distinct !{!27, !10, !11}
!28 = !{!29, !29, i64 0}
!29 = !{!"any pointer", !7, i64 0}
!30 = distinct !{!30, !10, !11}
!31 = distinct !{!31, !10, !11}

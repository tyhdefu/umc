; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/cholesky/cholesky.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/cholesky/cholesky.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"A\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  br label %4

4:                                                ; preds = %25, %2
  %5 = phi i64 [ 0, %2 ], [ %18, %25 ]
  %6 = phi i64 [ 1, %2 ], [ %27, %25 ]
  br label %7

7:                                                ; preds = %7, %4
  %8 = phi i64 [ 0, %4 ], [ %15, %7 ]
  %9 = trunc i64 %8 to i32
  %10 = sub i32 0, %9
  %11 = sitofp i32 %10 to double
  %12 = fdiv double %11, 4.000000e+02
  %13 = fadd double %12, 1.000000e+00
  %14 = getelementptr inbounds [400 x double], ptr %3, i64 %5, i64 %8
  store double %13, ptr %14, align 8, !tbaa !5
  %15 = add nuw nsw i64 %8, 1
  %16 = icmp eq i64 %15, %6
  br i1 %16, label %17, label %7, !llvm.loop !9

17:                                               ; preds = %7
  %18 = add nuw nsw i64 %5, 1
  %19 = icmp ult i64 %5, 399
  br i1 %19, label %20, label %25

20:                                               ; preds = %17, %20
  %21 = phi i64 [ %23, %20 ], [ %6, %17 ]
  %22 = getelementptr inbounds [400 x double], ptr %3, i64 %5, i64 %21
  store double 0.000000e+00, ptr %22, align 8, !tbaa !5
  %23 = add nuw nsw i64 %21, 1
  %24 = icmp eq i64 %23, 400
  br i1 %24, label %25, label %20, !llvm.loop !12

25:                                               ; preds = %20, %17
  %26 = getelementptr inbounds [400 x double], ptr %3, i64 %5, i64 %5
  store double 1.000000e+00, ptr %26, align 8, !tbaa !5
  %27 = add nuw nsw i64 %6, 1
  %28 = icmp eq i64 %18, 400
  br i1 %28, label %29, label %4, !llvm.loop !13

29:                                               ; preds = %25
  %30 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  br label %31

31:                                               ; preds = %38, %29
  %32 = phi i64 [ 0, %29 ], [ %39, %38 ]
  br label %33

33:                                               ; preds = %33, %31
  %34 = phi i64 [ 0, %31 ], [ %36, %33 ]
  %35 = getelementptr inbounds [400 x [400 x double]], ptr %30, i64 0, i64 %32, i64 %34
  store double 0.000000e+00, ptr %35, align 8, !tbaa !5
  %36 = add nuw nsw i64 %34, 1
  %37 = icmp eq i64 %36, 400
  br i1 %37, label %38, label %33, !llvm.loop !14

38:                                               ; preds = %33
  %39 = add nuw nsw i64 %32, 1
  %40 = icmp eq i64 %39, 400
  br i1 %40, label %41, label %31, !llvm.loop !15

41:                                               ; preds = %38, %59
  %42 = phi i64 [ %60, %59 ], [ 0, %38 ]
  br label %43

43:                                               ; preds = %56, %41
  %44 = phi i64 [ 0, %41 ], [ %57, %56 ]
  %45 = getelementptr inbounds [400 x double], ptr %3, i64 %44, i64 %42
  br label %46

46:                                               ; preds = %46, %43
  %47 = phi i64 [ 0, %43 ], [ %54, %46 ]
  %48 = load double, ptr %45, align 8, !tbaa !5
  %49 = getelementptr inbounds [400 x double], ptr %3, i64 %47, i64 %42
  %50 = load double, ptr %49, align 8, !tbaa !5
  %51 = getelementptr inbounds [400 x [400 x double]], ptr %30, i64 0, i64 %44, i64 %47
  %52 = load double, ptr %51, align 8, !tbaa !5
  %53 = tail call double @llvm.fmuladd.f64(double %48, double %50, double %52)
  store double %53, ptr %51, align 8, !tbaa !5
  %54 = add nuw nsw i64 %47, 1
  %55 = icmp eq i64 %54, 400
  br i1 %55, label %56, label %46, !llvm.loop !16

56:                                               ; preds = %46
  %57 = add nuw nsw i64 %44, 1
  %58 = icmp eq i64 %57, 400
  br i1 %58, label %59, label %43, !llvm.loop !17

59:                                               ; preds = %56
  %60 = add nuw nsw i64 %42, 1
  %61 = icmp eq i64 %60, 400
  br i1 %61, label %62, label %41, !llvm.loop !18

62:                                               ; preds = %59, %71
  %63 = phi i64 [ %72, %71 ], [ 0, %59 ]
  br label %64

64:                                               ; preds = %64, %62
  %65 = phi i64 [ 0, %62 ], [ %69, %64 ]
  %66 = getelementptr inbounds [400 x [400 x double]], ptr %30, i64 0, i64 %63, i64 %65
  %67 = load double, ptr %66, align 8, !tbaa !5
  %68 = getelementptr inbounds [400 x double], ptr %3, i64 %63, i64 %65
  store double %67, ptr %68, align 8, !tbaa !5
  %69 = add nuw nsw i64 %65, 1
  %70 = icmp eq i64 %69, 400
  br i1 %70, label %71, label %64, !llvm.loop !19

71:                                               ; preds = %64
  %72 = add nuw nsw i64 %63, 1
  %73 = icmp eq i64 %72, 400
  br i1 %73, label %74, label %62, !llvm.loop !20

74:                                               ; preds = %71
  tail call void @free(ptr noundef nonnull %30) #4
  br label %75

75:                                               ; preds = %121, %74
  %76 = phi i64 [ 0, %74 ], [ %125, %121 ]
  %77 = icmp eq i64 %76, 0
  br i1 %77, label %80, label %78

78:                                               ; preds = %75
  %79 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 0
  br label %85

80:                                               ; preds = %75
  %81 = load double, ptr %3, align 8, !tbaa !5
  br label %121

82:                                               ; preds = %104
  %83 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %76
  %84 = load double, ptr %83, align 8, !tbaa !5
  br label %112

85:                                               ; preds = %104, %78
  %86 = phi i64 [ 0, %78 ], [ %110, %104 ]
  %87 = icmp eq i64 %86, 0
  br i1 %87, label %88, label %90

88:                                               ; preds = %85
  %89 = load double, ptr %79, align 8, !tbaa !5
  br label %104

90:                                               ; preds = %85
  %91 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %86
  %92 = load double, ptr %91, align 8, !tbaa !5
  br label %93

93:                                               ; preds = %93, %90
  %94 = phi i64 [ 0, %90 ], [ %102, %93 ]
  %95 = phi double [ %92, %90 ], [ %101, %93 ]
  %96 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %94
  %97 = load double, ptr %96, align 8, !tbaa !5
  %98 = getelementptr inbounds [400 x double], ptr %3, i64 %86, i64 %94
  %99 = load double, ptr %98, align 8, !tbaa !5
  %100 = fneg double %97
  %101 = tail call double @llvm.fmuladd.f64(double %100, double %99, double %95)
  store double %101, ptr %91, align 8, !tbaa !5
  %102 = add nuw nsw i64 %94, 1
  %103 = icmp eq i64 %102, %86
  br i1 %103, label %104, label %93, !llvm.loop !21

104:                                              ; preds = %93, %88
  %105 = phi double [ %89, %88 ], [ %101, %93 ]
  %106 = getelementptr inbounds [400 x double], ptr %3, i64 %86, i64 %86
  %107 = load double, ptr %106, align 8, !tbaa !5
  %108 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %86
  %109 = fdiv double %105, %107
  store double %109, ptr %108, align 8, !tbaa !5
  %110 = add nuw nsw i64 %86, 1
  %111 = icmp eq i64 %110, %76
  br i1 %111, label %82, label %85, !llvm.loop !22

112:                                              ; preds = %112, %82
  %113 = phi i64 [ 0, %82 ], [ %119, %112 ]
  %114 = phi double [ %84, %82 ], [ %118, %112 ]
  %115 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %113
  %116 = load double, ptr %115, align 8, !tbaa !5
  %117 = fneg double %116
  %118 = tail call double @llvm.fmuladd.f64(double %117, double %116, double %114)
  store double %118, ptr %83, align 8, !tbaa !5
  %119 = add nuw nsw i64 %113, 1
  %120 = icmp eq i64 %119, %76
  br i1 %120, label %121, label %112, !llvm.loop !23

121:                                              ; preds = %112, %80
  %122 = phi double [ %81, %80 ], [ %118, %112 ]
  %123 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %76
  %124 = tail call double @sqrt(double noundef %122) #4
  store double %124, ptr %123, align 8, !tbaa !5
  %125 = add nuw nsw i64 %76, 1
  %126 = icmp eq i64 %125, 400
  br i1 %126, label %127, label %75, !llvm.loop !24

127:                                              ; preds = %121
  %128 = load ptr, ptr @stderr, align 8, !tbaa !25
  %129 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %128, ptr noundef nonnull @.str) #4
  %130 = load ptr, ptr @stderr, align 8, !tbaa !25
  %131 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %130, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %132

132:                                              ; preds = %152, %127
  %133 = phi i64 [ 0, %127 ], [ %153, %152 ]
  %134 = phi i64 [ 1, %127 ], [ %154, %152 ]
  %135 = mul nuw nsw i64 %133, 400
  br label %136

136:                                              ; preds = %145, %132
  %137 = phi i64 [ 0, %132 ], [ %150, %145 ]
  %138 = add nuw nsw i64 %137, %135
  %139 = trunc nuw nsw i64 %138 to i32
  %140 = urem i32 %139, 20
  %141 = icmp eq i32 %140, 0
  br i1 %141, label %142, label %145

142:                                              ; preds = %136
  %143 = load ptr, ptr @stderr, align 8, !tbaa !25
  %144 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %143, ptr noundef nonnull @.str.3) #4
  br label %145

145:                                              ; preds = %142, %136
  %146 = load ptr, ptr @stderr, align 8, !tbaa !25
  %147 = getelementptr inbounds [400 x double], ptr %3, i64 %133, i64 %137
  %148 = load double, ptr %147, align 8, !tbaa !5
  %149 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %146, ptr noundef nonnull @.str.4, double noundef %148) #4
  %150 = add nuw nsw i64 %137, 1
  %151 = icmp eq i64 %150, %134
  br i1 %151, label %152, label %136, !llvm.loop !27

152:                                              ; preds = %145
  %153 = add nuw nsw i64 %133, 1
  %154 = add nuw nsw i64 %134, 1
  %155 = icmp eq i64 %153, 400
  br i1 %155, label %156, label %132, !llvm.loop !28

156:                                              ; preds = %152
  %157 = load ptr, ptr @stderr, align 8, !tbaa !25
  %158 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %157, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %159 = load ptr, ptr @stderr, align 8, !tbaa !25
  %160 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %159, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef nonnull %3) #4
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
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = !{!26, !26, i64 0}
!26 = !{!"any pointer", !7, i64 0}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}

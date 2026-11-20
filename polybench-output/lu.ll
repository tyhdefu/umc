; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/lu/lu.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/lu/lu.c"
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

75:                                               ; preds = %127, %74
  %76 = phi i64 [ 0, %74 ], [ %128, %127 ]
  %77 = icmp eq i64 %76, 0
  br i1 %77, label %107, label %78

78:                                               ; preds = %75
  %79 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 0
  br label %80

80:                                               ; preds = %99, %78
  %81 = phi i64 [ 0, %78 ], [ %105, %99 ]
  %82 = icmp eq i64 %81, 0
  br i1 %82, label %83, label %85

83:                                               ; preds = %80
  %84 = load double, ptr %79, align 8, !tbaa !5
  br label %99

85:                                               ; preds = %80
  %86 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %81
  %87 = load double, ptr %86, align 8, !tbaa !5
  br label %88

88:                                               ; preds = %88, %85
  %89 = phi i64 [ 0, %85 ], [ %97, %88 ]
  %90 = phi double [ %87, %85 ], [ %96, %88 ]
  %91 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %89
  %92 = load double, ptr %91, align 8, !tbaa !5
  %93 = getelementptr inbounds [400 x double], ptr %3, i64 %89, i64 %81
  %94 = load double, ptr %93, align 8, !tbaa !5
  %95 = fneg double %92
  %96 = tail call double @llvm.fmuladd.f64(double %95, double %94, double %90)
  store double %96, ptr %86, align 8, !tbaa !5
  %97 = add nuw nsw i64 %89, 1
  %98 = icmp eq i64 %97, %81
  br i1 %98, label %99, label %88, !llvm.loop !21

99:                                               ; preds = %88, %83
  %100 = phi double [ %84, %83 ], [ %96, %88 ]
  %101 = getelementptr inbounds [400 x double], ptr %3, i64 %81, i64 %81
  %102 = load double, ptr %101, align 8, !tbaa !5
  %103 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %81
  %104 = fdiv double %100, %102
  store double %104, ptr %103, align 8, !tbaa !5
  %105 = add nuw nsw i64 %81, 1
  %106 = icmp eq i64 %105, %76
  br i1 %106, label %107, label %80, !llvm.loop !22

107:                                              ; preds = %99, %75
  br label %108

108:                                              ; preds = %107, %124
  %109 = phi i64 [ %125, %124 ], [ %76, %107 ]
  br i1 %77, label %124, label %110

110:                                              ; preds = %108
  %111 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %109
  %112 = load double, ptr %111, align 8, !tbaa !5
  br label %113

113:                                              ; preds = %113, %110
  %114 = phi i64 [ 0, %110 ], [ %122, %113 ]
  %115 = phi double [ %112, %110 ], [ %121, %113 ]
  %116 = getelementptr inbounds [400 x double], ptr %3, i64 %76, i64 %114
  %117 = load double, ptr %116, align 8, !tbaa !5
  %118 = getelementptr inbounds [400 x double], ptr %3, i64 %114, i64 %109
  %119 = load double, ptr %118, align 8, !tbaa !5
  %120 = fneg double %117
  %121 = tail call double @llvm.fmuladd.f64(double %120, double %119, double %115)
  store double %121, ptr %111, align 8, !tbaa !5
  %122 = add nuw nsw i64 %114, 1
  %123 = icmp eq i64 %122, %76
  br i1 %123, label %124, label %113, !llvm.loop !23

124:                                              ; preds = %113, %108
  %125 = add nuw nsw i64 %109, 1
  %126 = icmp eq i64 %125, 400
  br i1 %126, label %127, label %108, !llvm.loop !24

127:                                              ; preds = %124
  %128 = add nuw nsw i64 %76, 1
  %129 = icmp eq i64 %128, 400
  br i1 %129, label %130, label %75, !llvm.loop !25

130:                                              ; preds = %127
  %131 = load ptr, ptr @stderr, align 8, !tbaa !26
  %132 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %131, ptr noundef nonnull @.str) #4
  %133 = load ptr, ptr @stderr, align 8, !tbaa !26
  %134 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %133, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %135

135:                                              ; preds = %154, %130
  %136 = phi i64 [ 0, %130 ], [ %155, %154 ]
  %137 = mul nuw nsw i64 %136, 400
  br label %138

138:                                              ; preds = %147, %135
  %139 = phi i64 [ 0, %135 ], [ %152, %147 ]
  %140 = add nuw nsw i64 %139, %137
  %141 = trunc nuw nsw i64 %140 to i32
  %142 = urem i32 %141, 20
  %143 = icmp eq i32 %142, 0
  br i1 %143, label %144, label %147

144:                                              ; preds = %138
  %145 = load ptr, ptr @stderr, align 8, !tbaa !26
  %146 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %145, ptr noundef nonnull @.str.3) #4
  br label %147

147:                                              ; preds = %144, %138
  %148 = load ptr, ptr @stderr, align 8, !tbaa !26
  %149 = getelementptr inbounds [400 x double], ptr %3, i64 %136, i64 %139
  %150 = load double, ptr %149, align 8, !tbaa !5
  %151 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %148, ptr noundef nonnull @.str.4, double noundef %150) #4
  %152 = add nuw nsw i64 %139, 1
  %153 = icmp eq i64 %152, 400
  br i1 %153, label %154, label %138, !llvm.loop !28

154:                                              ; preds = %147
  %155 = add nuw nsw i64 %136, 1
  %156 = icmp eq i64 %155, 400
  br i1 %156, label %157, label %135, !llvm.loop !29

157:                                              ; preds = %154
  %158 = load ptr, ptr @stderr, align 8, !tbaa !26
  %159 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %158, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %160 = load ptr, ptr @stderr, align 8, !tbaa !26
  %161 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %160, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef nonnull %3) #4
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
!26 = !{!27, !27, i64 0}
!27 = !{!"any pointer", !7, i64 0}
!28 = distinct !{!28, !10, !11}
!29 = distinct !{!29, !10, !11}

; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/gramschmidt/gramschmidt.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/gramschmidt/gramschmidt.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"R\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"Q\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  br label %6

6:                                                ; preds = %20, %2
  %7 = phi i64 [ 0, %2 ], [ %21, %20 ]
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi i64 [ 0, %6 ], [ %18, %8 ]
  %10 = mul nuw nsw i64 %9, %7
  %11 = trunc nuw nsw i64 %10 to i32
  %12 = urem i32 %11, 200
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 2.000000e+02
  %15 = tail call double @llvm.fmuladd.f64(double %14, double 1.000000e+02, double 1.000000e+01)
  %16 = getelementptr inbounds [240 x double], ptr %3, i64 %7, i64 %9
  store double %15, ptr %16, align 8, !tbaa !5
  %17 = getelementptr inbounds [240 x double], ptr %5, i64 %7, i64 %9
  store double 0.000000e+00, ptr %17, align 8, !tbaa !5
  %18 = add nuw nsw i64 %9, 1
  %19 = icmp eq i64 %18, 240
  br i1 %19, label %20, label %8, !llvm.loop !9

20:                                               ; preds = %8
  %21 = add nuw nsw i64 %7, 1
  %22 = icmp eq i64 %21, 200
  br i1 %22, label %23, label %6, !llvm.loop !12

23:                                               ; preds = %20, %30
  %24 = phi i64 [ %31, %30 ], [ 0, %20 ]
  br label %25

25:                                               ; preds = %25, %23
  %26 = phi i64 [ 0, %23 ], [ %28, %25 ]
  %27 = getelementptr inbounds [240 x double], ptr %4, i64 %24, i64 %26
  store double 0.000000e+00, ptr %27, align 8, !tbaa !5
  %28 = add nuw nsw i64 %26, 1
  %29 = icmp eq i64 %28, 240
  br i1 %29, label %30, label %25, !llvm.loop !13

30:                                               ; preds = %25
  %31 = add nuw nsw i64 %24, 1
  %32 = icmp eq i64 %31, 240
  br i1 %32, label %36, label %23, !llvm.loop !14

33:                                               ; preds = %86, %59
  %34 = add nuw nsw i64 %38, 1
  %35 = icmp eq i64 %60, 240
  br i1 %35, label %89, label %36, !llvm.loop !15

36:                                               ; preds = %30, %33
  %37 = phi i64 [ %60, %33 ], [ 0, %30 ]
  %38 = phi i64 [ %34, %33 ], [ 1, %30 ]
  br label %39

39:                                               ; preds = %39, %36
  %40 = phi i64 [ 0, %36 ], [ %45, %39 ]
  %41 = phi double [ 0.000000e+00, %36 ], [ %44, %39 ]
  %42 = getelementptr inbounds [240 x double], ptr %3, i64 %40, i64 %37
  %43 = load double, ptr %42, align 8, !tbaa !5
  %44 = tail call double @llvm.fmuladd.f64(double %43, double %43, double %41)
  %45 = add nuw nsw i64 %40, 1
  %46 = icmp eq i64 %45, 200
  br i1 %46, label %47, label %39, !llvm.loop !16

47:                                               ; preds = %39
  %48 = tail call double @sqrt(double noundef %44) #4
  %49 = getelementptr inbounds [240 x double], ptr %4, i64 %37, i64 %37
  store double %48, ptr %49, align 8, !tbaa !5
  br label %50

50:                                               ; preds = %50, %47
  %51 = phi i64 [ 0, %47 ], [ %57, %50 ]
  %52 = getelementptr inbounds [240 x double], ptr %3, i64 %51, i64 %37
  %53 = load double, ptr %52, align 8, !tbaa !5
  %54 = load double, ptr %49, align 8, !tbaa !5
  %55 = fdiv double %53, %54
  %56 = getelementptr inbounds [240 x double], ptr %5, i64 %51, i64 %37
  store double %55, ptr %56, align 8, !tbaa !5
  %57 = add nuw nsw i64 %51, 1
  %58 = icmp eq i64 %57, 200
  br i1 %58, label %59, label %50, !llvm.loop !17

59:                                               ; preds = %50
  %60 = add nuw nsw i64 %37, 1
  %61 = icmp ult i64 %37, 239
  br i1 %61, label %62, label %33

62:                                               ; preds = %59, %86
  %63 = phi i64 [ %87, %86 ], [ %38, %59 ]
  %64 = getelementptr inbounds [240 x double], ptr %4, i64 %37, i64 %63
  store double 0.000000e+00, ptr %64, align 8, !tbaa !5
  br label %65

65:                                               ; preds = %65, %62
  %66 = phi i64 [ 0, %62 ], [ %73, %65 ]
  %67 = phi double [ 0.000000e+00, %62 ], [ %72, %65 ]
  %68 = getelementptr inbounds [240 x double], ptr %5, i64 %66, i64 %37
  %69 = load double, ptr %68, align 8, !tbaa !5
  %70 = getelementptr inbounds [240 x double], ptr %3, i64 %66, i64 %63
  %71 = load double, ptr %70, align 8, !tbaa !5
  %72 = tail call double @llvm.fmuladd.f64(double %69, double %71, double %67)
  store double %72, ptr %64, align 8, !tbaa !5
  %73 = add nuw nsw i64 %66, 1
  %74 = icmp eq i64 %73, 200
  br i1 %74, label %75, label %65, !llvm.loop !18

75:                                               ; preds = %65, %75
  %76 = phi i64 [ %84, %75 ], [ 0, %65 ]
  %77 = getelementptr inbounds [240 x double], ptr %3, i64 %76, i64 %63
  %78 = load double, ptr %77, align 8, !tbaa !5
  %79 = getelementptr inbounds [240 x double], ptr %5, i64 %76, i64 %37
  %80 = load double, ptr %79, align 8, !tbaa !5
  %81 = load double, ptr %64, align 8, !tbaa !5
  %82 = fneg double %80
  %83 = tail call double @llvm.fmuladd.f64(double %82, double %81, double %78)
  store double %83, ptr %77, align 8, !tbaa !5
  %84 = add nuw nsw i64 %76, 1
  %85 = icmp eq i64 %84, 200
  br i1 %85, label %86, label %75, !llvm.loop !19

86:                                               ; preds = %75
  %87 = add nuw nsw i64 %63, 1
  %88 = icmp eq i64 %87, 240
  br i1 %88, label %33, label %62, !llvm.loop !20

89:                                               ; preds = %33
  %90 = load ptr, ptr @stderr, align 8, !tbaa !21
  %91 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %90, ptr noundef nonnull @.str) #4
  %92 = load ptr, ptr @stderr, align 8, !tbaa !21
  %93 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %92, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %94

94:                                               ; preds = %113, %89
  %95 = phi i64 [ 0, %89 ], [ %114, %113 ]
  %96 = mul nuw nsw i64 %95, 240
  br label %97

97:                                               ; preds = %106, %94
  %98 = phi i64 [ 0, %94 ], [ %111, %106 ]
  %99 = add nuw nsw i64 %98, %96
  %100 = trunc nuw nsw i64 %99 to i32
  %101 = urem i32 %100, 20
  %102 = icmp eq i32 %101, 0
  br i1 %102, label %103, label %106

103:                                              ; preds = %97
  %104 = load ptr, ptr @stderr, align 8, !tbaa !21
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %104, ptr noundef nonnull @.str.3) #4
  br label %106

106:                                              ; preds = %103, %97
  %107 = load ptr, ptr @stderr, align 8, !tbaa !21
  %108 = getelementptr inbounds [240 x double], ptr %4, i64 %95, i64 %98
  %109 = load double, ptr %108, align 8, !tbaa !5
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %107, ptr noundef nonnull @.str.4, double noundef %109) #4
  %111 = add nuw nsw i64 %98, 1
  %112 = icmp eq i64 %111, 240
  br i1 %112, label %113, label %97, !llvm.loop !23

113:                                              ; preds = %106
  %114 = add nuw nsw i64 %95, 1
  %115 = icmp eq i64 %114, 240
  br i1 %115, label %116, label %94, !llvm.loop !24

116:                                              ; preds = %113
  %117 = load ptr, ptr @stderr, align 8, !tbaa !21
  %118 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %117, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %119 = load ptr, ptr @stderr, align 8, !tbaa !21
  %120 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %119, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  br label %121

121:                                              ; preds = %140, %116
  %122 = phi i64 [ 0, %116 ], [ %141, %140 ]
  %123 = mul nuw nsw i64 %122, 240
  br label %124

124:                                              ; preds = %133, %121
  %125 = phi i64 [ 0, %121 ], [ %138, %133 ]
  %126 = add nuw nsw i64 %125, %123
  %127 = trunc nuw nsw i64 %126 to i32
  %128 = urem i32 %127, 20
  %129 = icmp eq i32 %128, 0
  br i1 %129, label %130, label %133

130:                                              ; preds = %124
  %131 = load ptr, ptr @stderr, align 8, !tbaa !21
  %132 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %131, ptr noundef nonnull @.str.3) #4
  br label %133

133:                                              ; preds = %130, %124
  %134 = load ptr, ptr @stderr, align 8, !tbaa !21
  %135 = getelementptr inbounds [240 x double], ptr %5, i64 %122, i64 %125
  %136 = load double, ptr %135, align 8, !tbaa !5
  %137 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %134, ptr noundef nonnull @.str.4, double noundef %136) #4
  %138 = add nuw nsw i64 %125, 1
  %139 = icmp eq i64 %138, 240
  br i1 %139, label %140, label %124, !llvm.loop !25

140:                                              ; preds = %133
  %141 = add nuw nsw i64 %122, 1
  %142 = icmp eq i64 %141, 200
  br i1 %142, label %143, label %121, !llvm.loop !26

143:                                              ; preds = %140
  %144 = load ptr, ptr @stderr, align 8, !tbaa !21
  %145 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %144, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %146 = load ptr, ptr @stderr, align 8, !tbaa !21
  %147 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %146, ptr noundef nonnull @.str.7) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef nonnull %5) #4
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
!21 = !{!22, !22, i64 0}
!22 = !{!"any pointer", !7, i64 0}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}
!26 = distinct !{!26, !10, !11}

; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/doitgen/doitgen.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/doitgen/doitgen.c"
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

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local void @kernel_doitgen(i32 noundef %0, i32 noundef %1, i32 noundef %2, ptr nocapture noundef %3, ptr nocapture noundef readonly %4, ptr nocapture noundef %5) local_unnamed_addr #0 {
  %7 = icmp sgt i32 %0, 0
  br i1 %7, label %8, label %49

8:                                                ; preds = %6
  %9 = icmp sgt i32 %1, 0
  %10 = icmp sgt i32 %2, 0
  %11 = zext nneg i32 %0 to i64
  %12 = zext nneg i32 %1 to i64
  %13 = zext nneg i32 %2 to i64
  %14 = zext nneg i32 %2 to i64
  br label %15

15:                                               ; preds = %8, %46
  %16 = phi i64 [ 0, %8 ], [ %47, %46 ]
  br i1 %9, label %17, label %46

17:                                               ; preds = %15, %43
  %18 = phi i64 [ %44, %43 ], [ 0, %15 ]
  br i1 %10, label %20, label %43

19:                                               ; preds = %33
  br i1 %10, label %36, label %43

20:                                               ; preds = %17, %33
  %21 = phi i64 [ %34, %33 ], [ 0, %17 ]
  %22 = getelementptr inbounds double, ptr %5, i64 %21
  store double 0.000000e+00, ptr %22, align 8, !tbaa !5
  br label %23

23:                                               ; preds = %20, %23
  %24 = phi i64 [ 0, %20 ], [ %31, %23 ]
  %25 = phi double [ 0.000000e+00, %20 ], [ %30, %23 ]
  %26 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %16, i64 %18, i64 %24
  %27 = load double, ptr %26, align 8, !tbaa !5
  %28 = getelementptr inbounds [60 x double], ptr %4, i64 %24, i64 %21
  %29 = load double, ptr %28, align 8, !tbaa !5
  %30 = tail call double @llvm.fmuladd.f64(double %27, double %29, double %25)
  store double %30, ptr %22, align 8, !tbaa !5
  %31 = add nuw nsw i64 %24, 1
  %32 = icmp eq i64 %31, %13
  br i1 %32, label %33, label %23, !llvm.loop !9

33:                                               ; preds = %23
  %34 = add nuw nsw i64 %21, 1
  %35 = icmp eq i64 %34, %13
  br i1 %35, label %19, label %20, !llvm.loop !12

36:                                               ; preds = %19, %36
  %37 = phi i64 [ %41, %36 ], [ 0, %19 ]
  %38 = getelementptr inbounds double, ptr %5, i64 %37
  %39 = load double, ptr %38, align 8, !tbaa !5
  %40 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %16, i64 %18, i64 %37
  store double %39, ptr %40, align 8, !tbaa !5
  %41 = add nuw nsw i64 %37, 1
  %42 = icmp eq i64 %41, %14
  br i1 %42, label %43, label %36, !llvm.loop !13

43:                                               ; preds = %36, %17, %19
  %44 = add nuw nsw i64 %18, 1
  %45 = icmp eq i64 %44, %12
  br i1 %45, label %46, label %17, !llvm.loop !14

46:                                               ; preds = %43, %15
  %47 = add nuw nsw i64 %16, 1
  %48 = icmp eq i64 %47, %11
  br i1 %48, label %49, label %15, !llvm.loop !15

49:                                               ; preds = %46, %6
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #2 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 120000, i32 noundef 8) #5
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 60, i32 noundef 8) #5
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 3600, i32 noundef 8) #5
  br label %6

6:                                                ; preds = %24, %2
  %7 = phi i64 [ 0, %2 ], [ %25, %24 ]
  br label %8

8:                                                ; preds = %21, %6
  %9 = phi i64 [ 0, %6 ], [ %22, %21 ]
  %10 = mul nuw nsw i64 %9, %7
  br label %11

11:                                               ; preds = %11, %8
  %12 = phi i64 [ 0, %8 ], [ %19, %11 ]
  %13 = add nuw nsw i64 %12, %10
  %14 = trunc nuw nsw i64 %13 to i32
  %15 = urem i32 %14, 60
  %16 = uitofp nneg i32 %15 to double
  %17 = fdiv double %16, 6.000000e+01
  %18 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %7, i64 %9, i64 %12
  store double %17, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %12, 1
  %20 = icmp eq i64 %19, 60
  br i1 %20, label %21, label %11, !llvm.loop !16

21:                                               ; preds = %11
  %22 = add nuw nsw i64 %9, 1
  %23 = icmp eq i64 %22, 40
  br i1 %23, label %24, label %8, !llvm.loop !17

24:                                               ; preds = %21
  %25 = add nuw nsw i64 %7, 1
  %26 = icmp eq i64 %25, 50
  br i1 %26, label %27, label %6, !llvm.loop !18

27:                                               ; preds = %24, %39
  %28 = phi i64 [ %40, %39 ], [ 0, %24 ]
  br label %29

29:                                               ; preds = %29, %27
  %30 = phi i64 [ 0, %27 ], [ %37, %29 ]
  %31 = mul nuw nsw i64 %30, %28
  %32 = trunc nuw nsw i64 %31 to i32
  %33 = urem i32 %32, 60
  %34 = uitofp nneg i32 %33 to double
  %35 = fdiv double %34, 6.000000e+01
  %36 = getelementptr inbounds [60 x double], ptr %5, i64 %28, i64 %30
  store double %35, ptr %36, align 8, !tbaa !5
  %37 = add nuw nsw i64 %30, 1
  %38 = icmp eq i64 %37, 60
  br i1 %38, label %39, label %29, !llvm.loop !19

39:                                               ; preds = %29
  %40 = add nuw nsw i64 %28, 1
  %41 = icmp eq i64 %40, 60
  br i1 %41, label %42, label %27, !llvm.loop !20

42:                                               ; preds = %39, %72
  %43 = phi i64 [ %73, %72 ], [ 0, %39 ]
  br label %44

44:                                               ; preds = %69, %42
  %45 = phi i64 [ %70, %69 ], [ 0, %42 ]
  br label %46

46:                                               ; preds = %59, %44
  %47 = phi i64 [ %60, %59 ], [ 0, %44 ]
  %48 = getelementptr inbounds double, ptr %4, i64 %47
  store double 0.000000e+00, ptr %48, align 8, !tbaa !5
  br label %49

49:                                               ; preds = %49, %46
  %50 = phi i64 [ 0, %46 ], [ %57, %49 ]
  %51 = phi double [ 0.000000e+00, %46 ], [ %56, %49 ]
  %52 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %43, i64 %45, i64 %50
  %53 = load double, ptr %52, align 8, !tbaa !5
  %54 = getelementptr inbounds [60 x double], ptr %5, i64 %50, i64 %47
  %55 = load double, ptr %54, align 8, !tbaa !5
  %56 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %51)
  store double %56, ptr %48, align 8, !tbaa !5
  %57 = add nuw nsw i64 %50, 1
  %58 = icmp eq i64 %57, 60
  br i1 %58, label %59, label %49, !llvm.loop !9

59:                                               ; preds = %49
  %60 = add nuw nsw i64 %47, 1
  %61 = icmp eq i64 %60, 60
  br i1 %61, label %62, label %46, !llvm.loop !12

62:                                               ; preds = %59, %62
  %63 = phi i64 [ %67, %62 ], [ 0, %59 ]
  %64 = getelementptr inbounds double, ptr %4, i64 %63
  %65 = load double, ptr %64, align 8, !tbaa !5
  %66 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %43, i64 %45, i64 %63
  store double %65, ptr %66, align 8, !tbaa !5
  %67 = add nuw nsw i64 %63, 1
  %68 = icmp eq i64 %67, 60
  br i1 %68, label %69, label %62, !llvm.loop !13

69:                                               ; preds = %62
  %70 = add nuw nsw i64 %45, 1
  %71 = icmp eq i64 %70, 40
  br i1 %71, label %72, label %44, !llvm.loop !14

72:                                               ; preds = %69
  %73 = add nuw nsw i64 %43, 1
  %74 = icmp eq i64 %73, 50
  br i1 %74, label %75, label %42, !llvm.loop !15

75:                                               ; preds = %72
  %76 = load ptr, ptr @stderr, align 8, !tbaa !21
  %77 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str) #5
  %78 = load ptr, ptr @stderr, align 8, !tbaa !21
  %79 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %78, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #5
  br label %80

80:                                               ; preds = %106, %75
  %81 = phi i64 [ 0, %75 ], [ %107, %106 ]
  %82 = mul nuw nsw i64 %81, 2400
  br label %83

83:                                               ; preds = %103, %80
  %84 = phi i64 [ 0, %80 ], [ %104, %103 ]
  %85 = mul nuw nsw i64 %84, 60
  %86 = add nuw nsw i64 %85, %82
  br label %87

87:                                               ; preds = %96, %83
  %88 = phi i64 [ 0, %83 ], [ %101, %96 ]
  %89 = add nuw nsw i64 %86, %88
  %90 = trunc nuw nsw i64 %89 to i32
  %91 = urem i32 %90, 20
  %92 = icmp eq i32 %91, 0
  br i1 %92, label %93, label %96

93:                                               ; preds = %87
  %94 = load ptr, ptr @stderr, align 8, !tbaa !21
  %95 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %94, ptr noundef nonnull @.str.3) #5
  br label %96

96:                                               ; preds = %93, %87
  %97 = load ptr, ptr @stderr, align 8, !tbaa !21
  %98 = getelementptr inbounds [40 x [60 x double]], ptr %3, i64 %81, i64 %84, i64 %88
  %99 = load double, ptr %98, align 8, !tbaa !5
  %100 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %97, ptr noundef nonnull @.str.4, double noundef %99) #5
  %101 = add nuw nsw i64 %88, 1
  %102 = icmp eq i64 %101, 60
  br i1 %102, label %103, label %87, !llvm.loop !23

103:                                              ; preds = %96
  %104 = add nuw nsw i64 %84, 1
  %105 = icmp eq i64 %104, 40
  br i1 %105, label %106, label %83, !llvm.loop !24

106:                                              ; preds = %103
  %107 = add nuw nsw i64 %81, 1
  %108 = icmp eq i64 %107, 50
  br i1 %108, label %109, label %80, !llvm.loop !25

109:                                              ; preds = %106
  %110 = load ptr, ptr @stderr, align 8, !tbaa !21
  %111 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %110, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #5
  %112 = load ptr, ptr @stderr, align 8, !tbaa !21
  %113 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %112, ptr noundef nonnull @.str.6) #5
  tail call void @free(ptr noundef nonnull %3) #5
  tail call void @free(ptr noundef %4) #5
  tail call void @free(ptr noundef %5) #5
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #4

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #3

attributes #0 = { nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nobuiltin nounwind "no-builtins" }

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

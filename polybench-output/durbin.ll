; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/durbin/durbin.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/durbin/durbin.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"y\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = alloca [400 x double], align 16
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #5
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #5
  br label %6

6:                                                ; preds = %6, %2
  %7 = phi i64 [ 0, %2 ], [ %12, %6 ]
  %8 = trunc i64 %7 to i32
  %9 = sub i32 401, %8
  %10 = uitofp nneg i32 %9 to double
  %11 = getelementptr inbounds double, ptr %4, i64 %7
  store double %10, ptr %11, align 8, !tbaa !5
  %12 = add nuw nsw i64 %7, 1
  %13 = icmp eq i64 %12, 400
  br i1 %13, label %14, label %6, !llvm.loop !9

14:                                               ; preds = %6
  call void @llvm.lifetime.start.p0(i64 3200, ptr nonnull %3) #6
  %15 = load double, ptr %4, align 8, !tbaa !5
  %16 = fneg double %15
  store double %16, ptr %5, align 8, !tbaa !5
  %17 = load double, ptr %4, align 8, !tbaa !5
  %18 = fneg double %17
  br label %19

19:                                               ; preds = %62, %14
  %20 = phi i64 [ 1, %14 ], [ %63, %62 ]
  %21 = phi double [ 1.000000e+00, %14 ], [ %38, %62 ]
  %22 = phi double [ %18, %14 ], [ %42, %62 ]
  %23 = fneg double %22
  %24 = getelementptr double, ptr %4, i64 %20
  br label %25

25:                                               ; preds = %25, %19
  %26 = phi i64 [ 0, %19 ], [ %34, %25 ]
  %27 = phi double [ 0.000000e+00, %19 ], [ %33, %25 ]
  %28 = xor i64 %26, -1
  %29 = getelementptr double, ptr %24, i64 %28
  %30 = load double, ptr %29, align 8, !tbaa !5
  %31 = getelementptr inbounds double, ptr %5, i64 %26
  %32 = load double, ptr %31, align 8, !tbaa !5
  %33 = tail call double @llvm.fmuladd.f64(double %30, double %32, double %27)
  %34 = add nuw nsw i64 %26, 1
  %35 = icmp eq i64 %34, %20
  br i1 %35, label %36, label %25, !llvm.loop !12

36:                                               ; preds = %25
  %37 = tail call double @llvm.fmuladd.f64(double %23, double %22, double 1.000000e+00)
  %38 = fmul double %21, %37
  %39 = load double, ptr %24, align 8, !tbaa !5
  %40 = fadd double %33, %39
  %41 = fneg double %40
  %42 = fdiv double %41, %38
  %43 = getelementptr double, ptr %5, i64 %20
  br label %44

44:                                               ; preds = %44, %36
  %45 = phi i64 [ 0, %36 ], [ %53, %44 ]
  %46 = getelementptr inbounds double, ptr %5, i64 %45
  %47 = load double, ptr %46, align 8, !tbaa !5
  %48 = xor i64 %45, -1
  %49 = getelementptr double, ptr %43, i64 %48
  %50 = load double, ptr %49, align 8, !tbaa !5
  %51 = tail call double @llvm.fmuladd.f64(double %42, double %50, double %47)
  %52 = getelementptr inbounds [400 x double], ptr %3, i64 0, i64 %45
  store double %51, ptr %52, align 8, !tbaa !5
  %53 = add nuw nsw i64 %45, 1
  %54 = icmp eq i64 %53, %20
  br i1 %54, label %55, label %44, !llvm.loop !13

55:                                               ; preds = %44, %55
  %56 = phi i64 [ %60, %55 ], [ 0, %44 ]
  %57 = getelementptr inbounds [400 x double], ptr %3, i64 0, i64 %56
  %58 = load double, ptr %57, align 8, !tbaa !5
  %59 = getelementptr inbounds double, ptr %5, i64 %56
  store double %58, ptr %59, align 8, !tbaa !5
  %60 = add nuw nsw i64 %56, 1
  %61 = icmp eq i64 %60, %20
  br i1 %61, label %62, label %55, !llvm.loop !14

62:                                               ; preds = %55
  store double %42, ptr %43, align 8, !tbaa !5
  %63 = add nuw nsw i64 %20, 1
  %64 = icmp eq i64 %63, 400
  br i1 %64, label %65, label %19, !llvm.loop !15

65:                                               ; preds = %62
  call void @llvm.lifetime.end.p0(i64 3200, ptr nonnull %3) #6
  %66 = load ptr, ptr @stderr, align 8, !tbaa !16
  %67 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %66, ptr noundef nonnull @.str) #5
  %68 = load ptr, ptr @stderr, align 8, !tbaa !16
  %69 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %68, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #5
  br label %70

70:                                               ; preds = %78, %65
  %71 = phi i64 [ 0, %65 ], [ %83, %78 ]
  %72 = trunc i64 %71 to i16
  %73 = urem i16 %72, 20
  %74 = icmp eq i16 %73, 0
  br i1 %74, label %75, label %78

75:                                               ; preds = %70
  %76 = load ptr, ptr @stderr, align 8, !tbaa !16
  %77 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.3) #5
  br label %78

78:                                               ; preds = %75, %70
  %79 = load ptr, ptr @stderr, align 8, !tbaa !16
  %80 = getelementptr inbounds double, ptr %5, i64 %71
  %81 = load double, ptr %80, align 8, !tbaa !5
  %82 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str.4, double noundef %81) #5
  %83 = add nuw nsw i64 %71, 1
  %84 = icmp eq i64 %83, 400
  br i1 %84, label %85, label %70, !llvm.loop !18

85:                                               ; preds = %78
  %86 = load ptr, ptr @stderr, align 8, !tbaa !16
  %87 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %86, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #5
  %88 = load ptr, ptr @stderr, align 8, !tbaa !16
  %89 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %88, ptr noundef nonnull @.str.6) #5
  tail call void @free(ptr noundef nonnull %4) #5
  tail call void @free(ptr noundef nonnull %5) #5
  ret i32 0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #2

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #3

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #4

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #2

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #5 = { nobuiltin nounwind "no-builtins" }
attributes #6 = { nounwind }

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
!16 = !{!17, !17, i64 0}
!17 = !{!"any pointer", !7, i64 0}
!18 = distinct !{!18, !10, !11}

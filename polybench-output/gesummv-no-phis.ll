; ModuleID = './polybench-output/gesummv-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gesummv/gesummv.c"
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
  %.reg2mem = alloca i64, align 8
  %.reg2mem8 = alloca i64, align 8
  %.reg2mem12 = alloca i64, align 8
  %.reg2mem15 = alloca i64, align 8
  %.reg2mem18 = alloca double, align 8
  %.reg2mem21 = alloca ptr, align 8
  %.reg2mem26 = alloca ptr, align 8
  %.reg2mem31 = alloca i64, align 8
  %.reg2mem37 = alloca i64, align 8
  %.reg2mem40 = alloca i64, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem50 = alloca ptr, align 8
  %.reg2mem54 = alloca ptr, align 8
  %.reg2mem58 = alloca ptr, align 8
  %.reg2mem61 = alloca ptr, align 8
  %.reg2mem65 = alloca ptr, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem71 = alloca i64, align 8
  %.reg2mem73 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem77 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62500, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem65, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 62500, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem61, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem58, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem54, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 250, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem50, align 8
  store i64 0, ptr %.reg2mem77, align 8
  br label %8

8:                                                ; preds = %._crit_edge2, %2
  %.reload78 = load i64, ptr %.reg2mem77, align 8
  store i64 %.reload78, ptr %.reg2mem43, align 8
  %.reload49 = load i64, ptr %.reg2mem43, align 8
  %9 = trunc nuw nsw i64 %.reload49 to i32
  %10 = uitofp nneg i32 %9 to double
  %11 = fdiv double %10, 2.500000e+02
  %.reload48 = load i64, ptr %.reg2mem43, align 8
  %.reload56 = load ptr, ptr %.reg2mem54, align 8
  %12 = getelementptr inbounds double, ptr %.reload56, i64 %.reload48
  store double %11, ptr %12, align 8, !tbaa !5
  store i64 0, ptr %.reg2mem75, align 8
  br label %13

13:                                               ; preds = %._crit_edge, %8
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  %.reload47 = load i64, ptr %.reg2mem43, align 8
  %14 = mul nuw nsw i64 %.reload76, %.reload47
  %15 = trunc i64 %14 to i32
  %16 = add i32 %15, 1
  %17 = urem i32 %16, 250
  %18 = uitofp nneg i32 %17 to double
  %19 = fdiv double %18, 2.500000e+02
  %.reload46 = load i64, ptr %.reg2mem43, align 8
  %.reload67 = load ptr, ptr %.reg2mem65, align 8
  %20 = getelementptr inbounds [250 x double], ptr %.reload67, i64 %.reload46, i64 %.reload76
  store double %19, ptr %20, align 8, !tbaa !5
  %21 = add i32 %15, 2
  %22 = urem i32 %21, 250
  %23 = uitofp nneg i32 %22 to double
  %24 = fdiv double %23, 2.500000e+02
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %.reload63 = load ptr, ptr %.reg2mem61, align 8
  %25 = getelementptr inbounds [250 x double], ptr %.reload63, i64 %.reload45, i64 %.reload76
  store double %24, ptr %25, align 8, !tbaa !5
  %26 = add nuw nsw i64 %.reload76, 1
  store i64 %26, ptr %.reg2mem40, align 8
  %.reload42 = load i64, ptr %.reg2mem40, align 8
  %27 = icmp eq i64 %.reload42, 250
  br i1 %27, label %28, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %13
  %.reload41 = load i64, ptr %.reg2mem40, align 8
  store i64 %.reload41, ptr %.reg2mem75, align 8
  br label %13

28:                                               ; preds = %13
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  %29 = add nuw nsw i64 %.reload44, 1
  store i64 %29, ptr %.reg2mem37, align 8
  %.reload39 = load i64, ptr %.reg2mem37, align 8
  %30 = icmp eq i64 %.reload39, 250
  br i1 %30, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %28
  %.reload38 = load i64, ptr %.reg2mem37, align 8
  store i64 %.reload38, ptr %.reg2mem77, align 8
  br label %8

._crit_edge1:                                     ; preds = %28
  store i64 0, ptr %.reg2mem73, align 8
  br label %31

31:                                               ; preds = %._crit_edge4, %._crit_edge1
  %.reload74 = load i64, ptr %.reg2mem73, align 8
  store i64 %.reload74, ptr %.reg2mem31, align 8
  %.reload36 = load i64, ptr %.reg2mem31, align 8
  %.reload59 = load ptr, ptr %.reg2mem58, align 8
  %32 = getelementptr inbounds double, ptr %.reload59, i64 %.reload36
  store ptr %32, ptr %.reg2mem26, align 8
  %.reload30 = load ptr, ptr %.reg2mem26, align 8
  store double 0.000000e+00, ptr %.reload30, align 8, !tbaa !5
  %.reload35 = load i64, ptr %.reg2mem31, align 8
  %.reload52 = load ptr, ptr %.reg2mem50, align 8
  %33 = getelementptr inbounds double, ptr %.reload52, i64 %.reload35
  store ptr %33, ptr %.reg2mem21, align 8
  %.reload25 = load ptr, ptr %.reg2mem21, align 8
  store double 0.000000e+00, ptr %.reload25, align 8, !tbaa !5
  store i64 0, ptr %.reg2mem71, align 8
  br label %34

34:                                               ; preds = %._crit_edge3, %31
  %.reload72 = load i64, ptr %.reg2mem71, align 8
  %.reload34 = load i64, ptr %.reg2mem31, align 8
  %.reload66 = load ptr, ptr %.reg2mem65, align 8
  %35 = getelementptr inbounds [250 x double], ptr %.reload66, i64 %.reload34, i64 %.reload72
  %36 = load double, ptr %35, align 8, !tbaa !5
  %.reload55 = load ptr, ptr %.reg2mem54, align 8
  %37 = getelementptr inbounds double, ptr %.reload55, i64 %.reload72
  %38 = load double, ptr %37, align 8, !tbaa !5
  %.reload29 = load ptr, ptr %.reg2mem26, align 8
  %39 = load double, ptr %.reload29, align 8, !tbaa !5
  %40 = tail call double @llvm.fmuladd.f64(double %36, double %38, double %39)
  %.reload28 = load ptr, ptr %.reg2mem26, align 8
  store double %40, ptr %.reload28, align 8, !tbaa !5
  %.reload33 = load i64, ptr %.reg2mem31, align 8
  %.reload62 = load ptr, ptr %.reg2mem61, align 8
  %41 = getelementptr inbounds [250 x double], ptr %.reload62, i64 %.reload33, i64 %.reload72
  %42 = load double, ptr %41, align 8, !tbaa !5
  %43 = load double, ptr %37, align 8, !tbaa !5
  %.reload24 = load ptr, ptr %.reg2mem21, align 8
  %44 = load double, ptr %.reload24, align 8, !tbaa !5
  %45 = tail call double @llvm.fmuladd.f64(double %42, double %43, double %44)
  store double %45, ptr %.reg2mem18, align 8
  %.reload20 = load double, ptr %.reg2mem18, align 8
  %.reload23 = load ptr, ptr %.reg2mem21, align 8
  store double %.reload20, ptr %.reload23, align 8, !tbaa !5
  %46 = add nuw nsw i64 %.reload72, 1
  store i64 %46, ptr %.reg2mem15, align 8
  %.reload17 = load i64, ptr %.reg2mem15, align 8
  %47 = icmp eq i64 %.reload17, 250
  br i1 %47, label %48, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %34
  %.reload16 = load i64, ptr %.reg2mem15, align 8
  store i64 %.reload16, ptr %.reg2mem71, align 8
  br label %34

48:                                               ; preds = %34
  %.reload27 = load ptr, ptr %.reg2mem26, align 8
  %49 = load double, ptr %.reload27, align 8, !tbaa !5
  %.reload19 = load double, ptr %.reg2mem18, align 8
  %50 = fmul double %.reload19, 1.200000e+00
  %51 = tail call double @llvm.fmuladd.f64(double %49, double 1.500000e+00, double %50)
  %.reload22 = load ptr, ptr %.reg2mem21, align 8
  store double %51, ptr %.reload22, align 8, !tbaa !5
  %.reload32 = load i64, ptr %.reg2mem31, align 8
  %52 = add nuw nsw i64 %.reload32, 1
  store i64 %52, ptr %.reg2mem12, align 8
  %.reload14 = load i64, ptr %.reg2mem12, align 8
  %53 = icmp eq i64 %.reload14, 250
  br i1 %53, label %54, label %._crit_edge4, !llvm.loop !14

._crit_edge4:                                     ; preds = %48
  %.reload13 = load i64, ptr %.reg2mem12, align 8
  store i64 %.reload13, ptr %.reg2mem73, align 8
  br label %31

54:                                               ; preds = %48
  %55 = load ptr, ptr @stderr, align 8, !tbaa !15
  %56 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %55, ptr noundef nonnull @.str) #4
  %57 = load ptr, ptr @stderr, align 8, !tbaa !15
  %58 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %57, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem69, align 8
  br label %59

59:                                               ; preds = %._crit_edge6, %54
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  store i64 %.reload70, ptr %.reg2mem8, align 8
  %.reload11 = load i64, ptr %.reg2mem8, align 8
  %60 = trunc i64 %.reload11 to i8
  %61 = urem i8 %60, 20
  %62 = icmp eq i8 %61, 0
  br i1 %62, label %63, label %._crit_edge5

._crit_edge5:                                     ; preds = %59
  br label %66

63:                                               ; preds = %59
  %64 = load ptr, ptr @stderr, align 8, !tbaa !15
  %65 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %64, ptr noundef nonnull @.str.3) #4
  br label %66

66:                                               ; preds = %._crit_edge5, %63
  %67 = load ptr, ptr @stderr, align 8, !tbaa !15
  %.reload10 = load i64, ptr %.reg2mem8, align 8
  %.reload51 = load ptr, ptr %.reg2mem50, align 8
  %68 = getelementptr inbounds double, ptr %.reload51, i64 %.reload10
  %69 = load double, ptr %68, align 8, !tbaa !5
  %70 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.4, double noundef %69) #4
  %.reload9 = load i64, ptr %.reg2mem8, align 8
  %71 = add nuw nsw i64 %.reload9, 1
  store i64 %71, ptr %.reg2mem, align 8
  %.reload7 = load i64, ptr %.reg2mem, align 8
  %72 = icmp eq i64 %.reload7, 250
  br i1 %72, label %73, label %._crit_edge6, !llvm.loop !17

._crit_edge6:                                     ; preds = %66
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem69, align 8
  br label %59

73:                                               ; preds = %66
  %74 = load ptr, ptr @stderr, align 8, !tbaa !15
  %75 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %74, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %76 = load ptr, ptr @stderr, align 8, !tbaa !15
  %77 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.6) #4
  %.reload68 = load ptr, ptr %.reg2mem65, align 8
  tail call void @free(ptr noundef %.reload68) #4
  %.reload64 = load ptr, ptr %.reg2mem61, align 8
  tail call void @free(ptr noundef %.reload64) #4
  %.reload60 = load ptr, ptr %.reg2mem58, align 8
  tail call void @free(ptr noundef %.reload60) #4
  %.reload57 = load ptr, ptr %.reg2mem54, align 8
  tail call void @free(ptr noundef %.reload57) #4
  %.reload53 = load ptr, ptr %.reg2mem50, align 8
  tail call void @free(ptr noundef nonnull %.reload53) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #1

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
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
!15 = !{!16, !16, i64 0}
!16 = !{!"any pointer", !7, i64 0}
!17 = distinct !{!17, !10, !11}

; ModuleID = './polybench-output/atax-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/atax/atax.c"
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
  %.reg2mem14 = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem21 = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem27 = alloca double, align 8
  %.reg2mem30 = alloca ptr, align 8
  %.reg2mem34 = alloca i64, align 8
  %.reg2mem39 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem55 = alloca ptr, align 8
  %.reg2mem58 = alloca ptr, align 8
  %.reg2mem63 = alloca ptr, align 8
  %.reg2mem67 = alloca ptr, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem74 = alloca i64, align 8
  %.reg2mem76 = alloca double, align 8
  %.reg2mem78 = alloca i64, align 8
  %.reg2mem80 = alloca i64, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem88 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 159900, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem67, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem63, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem58, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem55, align 8
  store i64 0, ptr %.reg2mem88, align 8
  br label %7

7:                                                ; preds = %._crit_edge1, %2
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  %8 = trunc nuw nsw i64 %.reload89 to i32
  %9 = uitofp nneg i32 %8 to double
  %10 = fdiv double %9, 4.100000e+02
  %11 = fadd double %10, 1.000000e+00
  %.reload65 = load ptr, ptr %.reg2mem63, align 8
  %12 = getelementptr inbounds double, ptr %.reload65, i64 %.reload89
  store double %11, ptr %12, align 8, !tbaa !5
  %13 = add nuw nsw i64 %.reload89, 1
  store i64 %13, ptr %.reg2mem52, align 8
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %14 = icmp eq i64 %.reload54, 410
  br i1 %14, label %._crit_edge, label %._crit_edge1, !llvm.loop !9

._crit_edge1:                                     ; preds = %7
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  store i64 %.reload53, ptr %.reg2mem88, align 8
  br label %7

._crit_edge:                                      ; preds = %7
  store i64 0, ptr %.reg2mem86, align 8
  br label %15

15:                                               ; preds = %._crit_edge4, %._crit_edge
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  store i64 %.reload87, ptr %.reg2mem48, align 8
  store i64 0, ptr %.reg2mem84, align 8
  br label %16

16:                                               ; preds = %._crit_edge2, %15
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  %.reload51 = load i64, ptr %.reg2mem48, align 8
  %17 = add nuw nsw i64 %.reload85, %.reload51
  %18 = trunc nuw nsw i64 %17 to i32
  %19 = urem i32 %18, 410
  %20 = uitofp nneg i32 %19 to double
  %21 = fdiv double %20, 1.950000e+03
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %.reload70 = load ptr, ptr %.reg2mem67, align 8
  %22 = getelementptr inbounds [410 x double], ptr %.reload70, i64 %.reload50, i64 %.reload85
  store double %21, ptr %22, align 8, !tbaa !5
  %23 = add nuw nsw i64 %.reload85, 1
  store i64 %23, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %24 = icmp eq i64 %.reload47, 410
  br i1 %24, label %25, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %16
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  store i64 %.reload46, ptr %.reg2mem84, align 8
  br label %16

25:                                               ; preds = %16
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %26 = add nuw nsw i64 %.reload49, 1
  store i64 %26, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %27 = icmp eq i64 %.reload44, 390
  br i1 %27, label %._crit_edge3, label %._crit_edge4, !llvm.loop !13

._crit_edge4:                                     ; preds = %25
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  store i64 %.reload43, ptr %.reg2mem86, align 8
  br label %15

._crit_edge3:                                     ; preds = %25
  store i64 0, ptr %.reg2mem82, align 8
  br label %28

28:                                               ; preds = %._crit_edge6, %._crit_edge3
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %.reload61 = load ptr, ptr %.reg2mem58, align 8
  %29 = getelementptr inbounds double, ptr %.reload61, i64 %.reload83
  store double 0.000000e+00, ptr %29, align 8, !tbaa !5
  %30 = add nuw nsw i64 %.reload83, 1
  store i64 %30, ptr %.reg2mem39, align 8
  %.reload41 = load i64, ptr %.reg2mem39, align 8
  %31 = icmp eq i64 %.reload41, 410
  br i1 %31, label %._crit_edge5, label %._crit_edge6, !llvm.loop !14

._crit_edge6:                                     ; preds = %28
  %.reload40 = load i64, ptr %.reg2mem39, align 8
  store i64 %.reload40, ptr %.reg2mem82, align 8
  br label %28

._crit_edge5:                                     ; preds = %28
  store i64 0, ptr %.reg2mem80, align 8
  br label %32

32:                                               ; preds = %._crit_edge10, %._crit_edge5
  %.reload81 = load i64, ptr %.reg2mem80, align 8
  store i64 %.reload81, ptr %.reg2mem34, align 8
  %.reload38 = load i64, ptr %.reg2mem34, align 8
  %.reload56 = load ptr, ptr %.reg2mem55, align 8
  %33 = getelementptr inbounds double, ptr %.reload56, i64 %.reload38
  store ptr %33, ptr %.reg2mem30, align 8
  %.reload33 = load ptr, ptr %.reg2mem30, align 8
  store double 0.000000e+00, ptr %.reload33, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem76, align 8
  store i64 0, ptr %.reg2mem78, align 8
  br label %34

34:                                               ; preds = %._crit_edge8, %32
  %.reload79 = load i64, ptr %.reg2mem78, align 8
  %.reload77 = load double, ptr %.reg2mem76, align 8
  %.reload37 = load i64, ptr %.reg2mem34, align 8
  %.reload69 = load ptr, ptr %.reg2mem67, align 8
  %35 = getelementptr inbounds [410 x double], ptr %.reload69, i64 %.reload37, i64 %.reload79
  %36 = load double, ptr %35, align 8, !tbaa !5
  %.reload64 = load ptr, ptr %.reg2mem63, align 8
  %37 = getelementptr inbounds double, ptr %.reload64, i64 %.reload79
  %38 = load double, ptr %37, align 8, !tbaa !5
  %39 = tail call double @llvm.fmuladd.f64(double %36, double %38, double %.reload77)
  store double %39, ptr %.reg2mem27, align 8
  %.reload29 = load double, ptr %.reg2mem27, align 8
  %.reload32 = load ptr, ptr %.reg2mem30, align 8
  store double %.reload29, ptr %.reload32, align 8, !tbaa !5
  %40 = add nuw nsw i64 %.reload79, 1
  store i64 %40, ptr %.reg2mem24, align 8
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %41 = icmp eq i64 %.reload26, 410
  br i1 %41, label %._crit_edge7, label %._crit_edge8, !llvm.loop !15

._crit_edge8:                                     ; preds = %34
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  %.reload28 = load double, ptr %.reg2mem27, align 8
  store double %.reload28, ptr %.reg2mem76, align 8
  store i64 %.reload25, ptr %.reg2mem78, align 8
  br label %34

._crit_edge7:                                     ; preds = %34
  store i64 0, ptr %.reg2mem74, align 8
  br label %42

42:                                               ; preds = %._crit_edge9, %._crit_edge7
  %.reload75 = load i64, ptr %.reg2mem74, align 8
  %.reload60 = load ptr, ptr %.reg2mem58, align 8
  %43 = getelementptr inbounds double, ptr %.reload60, i64 %.reload75
  %44 = load double, ptr %43, align 8, !tbaa !5
  %.reload36 = load i64, ptr %.reg2mem34, align 8
  %.reload68 = load ptr, ptr %.reg2mem67, align 8
  %45 = getelementptr inbounds [410 x double], ptr %.reload68, i64 %.reload36, i64 %.reload75
  %46 = load double, ptr %45, align 8, !tbaa !5
  %.reload31 = load ptr, ptr %.reg2mem30, align 8
  %47 = load double, ptr %.reload31, align 8, !tbaa !5
  %48 = tail call double @llvm.fmuladd.f64(double %46, double %47, double %44)
  store double %48, ptr %43, align 8, !tbaa !5
  %49 = add nuw nsw i64 %.reload75, 1
  store i64 %49, ptr %.reg2mem21, align 8
  %.reload23 = load i64, ptr %.reg2mem21, align 8
  %50 = icmp eq i64 %.reload23, 410
  br i1 %50, label %51, label %._crit_edge9, !llvm.loop !16

._crit_edge9:                                     ; preds = %42
  %.reload22 = load i64, ptr %.reg2mem21, align 8
  store i64 %.reload22, ptr %.reg2mem74, align 8
  br label %42

51:                                               ; preds = %42
  %.reload35 = load i64, ptr %.reg2mem34, align 8
  %52 = add nuw nsw i64 %.reload35, 1
  store i64 %52, ptr %.reg2mem18, align 8
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %53 = icmp eq i64 %.reload20, 390
  br i1 %53, label %54, label %._crit_edge10, !llvm.loop !17

._crit_edge10:                                    ; preds = %51
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  store i64 %.reload19, ptr %.reg2mem80, align 8
  br label %32

54:                                               ; preds = %51
  %55 = load ptr, ptr @stderr, align 8, !tbaa !18
  %56 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %55, ptr noundef nonnull @.str) #4
  %57 = load ptr, ptr @stderr, align 8, !tbaa !18
  %58 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %57, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem72, align 8
  br label %59

59:                                               ; preds = %._crit_edge12, %54
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem14, align 8
  %.reload17 = load i64, ptr %.reg2mem14, align 8
  %60 = trunc i64 %.reload17 to i16
  %61 = urem i16 %60, 20
  %62 = icmp eq i16 %61, 0
  br i1 %62, label %63, label %._crit_edge11

._crit_edge11:                                    ; preds = %59
  br label %66

63:                                               ; preds = %59
  %64 = load ptr, ptr @stderr, align 8, !tbaa !18
  %65 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %64, ptr noundef nonnull @.str.3) #4
  br label %66

66:                                               ; preds = %._crit_edge11, %63
  %67 = load ptr, ptr @stderr, align 8, !tbaa !18
  %.reload16 = load i64, ptr %.reg2mem14, align 8
  %.reload59 = load ptr, ptr %.reg2mem58, align 8
  %68 = getelementptr inbounds double, ptr %.reload59, i64 %.reload16
  %69 = load double, ptr %68, align 8, !tbaa !5
  %70 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.4, double noundef %69) #4
  %.reload15 = load i64, ptr %.reg2mem14, align 8
  %71 = add nuw nsw i64 %.reload15, 1
  store i64 %71, ptr %.reg2mem, align 8
  %.reload13 = load i64, ptr %.reg2mem, align 8
  %72 = icmp eq i64 %.reload13, 410
  br i1 %72, label %73, label %._crit_edge12, !llvm.loop !20

._crit_edge12:                                    ; preds = %66
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem72, align 8
  br label %59

73:                                               ; preds = %66
  %74 = load ptr, ptr @stderr, align 8, !tbaa !18
  %75 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %74, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %76 = load ptr, ptr @stderr, align 8, !tbaa !18
  %77 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.6) #4
  %.reload71 = load ptr, ptr %.reg2mem67, align 8
  tail call void @free(ptr noundef %.reload71) #4
  %.reload66 = load ptr, ptr %.reg2mem63, align 8
  tail call void @free(ptr noundef %.reload66) #4
  %.reload62 = load ptr, ptr %.reg2mem58, align 8
  tail call void @free(ptr noundef nonnull %.reload62) #4
  %.reload57 = load ptr, ptr %.reg2mem55, align 8
  tail call void @free(ptr noundef %.reload57) #4
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
!15 = distinct !{!15, !10, !11}
!16 = distinct !{!16, !10, !11}
!17 = distinct !{!17, !10, !11}
!18 = !{!19, !19, i64 0}
!19 = !{!"any pointer", !7, i64 0}
!20 = distinct !{!20, !10, !11}

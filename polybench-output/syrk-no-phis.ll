; ModuleID = './polybench-output/syrk-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syrk/syrk.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"C\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem15 = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem22 = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem28 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem33 = alloca i64, align 8
  %.reg2mem36 = alloca i64, align 8
  %.reg2mem39 = alloca ptr, align 8
  %.reg2mem41 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem57 = alloca i64, align 8
  %.reg2mem60 = alloca i64, align 8
  %.reg2mem63 = alloca i64, align 8
  %.reg2mem67 = alloca i64, align 8
  %.reg2mem70 = alloca i64, align 8
  %.reg2mem73 = alloca i64, align 8
  %.reg2mem77 = alloca ptr, align 8
  %.reg2mem82 = alloca ptr, align 8
  %.reg2mem88 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem92 = alloca i64, align 8
  %.reg2mem94 = alloca i64, align 8
  %.reg2mem96 = alloca i64, align 8
  %.reg2mem98 = alloca i64, align 8
  %.reg2mem100 = alloca i64, align 8
  %.reg2mem102 = alloca i64, align 8
  %.reg2mem104 = alloca i64, align 8
  %.reg2mem106 = alloca i64, align 8
  %.reg2mem108 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem82, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem77, align 8
  store i64 0, ptr %.reg2mem108, align 8
  br label %5

5:                                                ; preds = %._crit_edge2, %2
  %.reload109 = load i64, ptr %.reg2mem108, align 8
  store i64 %.reload109, ptr %.reg2mem73, align 8
  store i64 0, ptr %.reg2mem106, align 8
  br label %6

6:                                                ; preds = %._crit_edge, %5
  %.reload107 = load i64, ptr %.reg2mem106, align 8
  %.reload76 = load i64, ptr %.reg2mem73, align 8
  %7 = mul nuw nsw i64 %.reload107, %.reload76
  %8 = trunc i64 %7 to i32
  %9 = add i32 %8, 1
  %10 = urem i32 %9, 240
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 2.400000e+02
  %.reload75 = load i64, ptr %.reg2mem73, align 8
  %.reload80 = load ptr, ptr %.reg2mem77, align 8
  %13 = getelementptr inbounds [200 x double], ptr %.reload80, i64 %.reload75, i64 %.reload107
  store double %12, ptr %13, align 8, !tbaa !5
  %14 = add nuw nsw i64 %.reload107, 1
  store i64 %14, ptr %.reg2mem70, align 8
  %.reload72 = load i64, ptr %.reg2mem70, align 8
  %15 = icmp eq i64 %.reload72, 200
  br i1 %15, label %16, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %6
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  store i64 %.reload71, ptr %.reg2mem106, align 8
  br label %6

16:                                               ; preds = %6
  %.reload74 = load i64, ptr %.reg2mem73, align 8
  %17 = add nuw nsw i64 %.reload74, 1
  store i64 %17, ptr %.reg2mem67, align 8
  %.reload69 = load i64, ptr %.reg2mem67, align 8
  %18 = icmp eq i64 %.reload69, 240
  br i1 %18, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %16
  %.reload68 = load i64, ptr %.reg2mem67, align 8
  store i64 %.reload68, ptr %.reg2mem108, align 8
  br label %5

._crit_edge1:                                     ; preds = %16
  store i64 0, ptr %.reg2mem104, align 8
  br label %19

19:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload105 = load i64, ptr %.reg2mem104, align 8
  store i64 %.reload105, ptr %.reg2mem63, align 8
  store i64 0, ptr %.reg2mem102, align 8
  br label %20

20:                                               ; preds = %._crit_edge3, %19
  %.reload103 = load i64, ptr %.reg2mem102, align 8
  %.reload66 = load i64, ptr %.reg2mem63, align 8
  %21 = mul nuw nsw i64 %.reload103, %.reload66
  %22 = trunc i64 %21 to i32
  %23 = add i32 %22, 2
  %24 = urem i32 %23, 200
  %25 = uitofp nneg i32 %24 to double
  %26 = fdiv double %25, 2.000000e+02
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %.reload86 = load ptr, ptr %.reg2mem82, align 8
  %27 = getelementptr inbounds [240 x double], ptr %.reload86, i64 %.reload65, i64 %.reload103
  store double %26, ptr %27, align 8, !tbaa !5
  %28 = add nuw nsw i64 %.reload103, 1
  store i64 %28, ptr %.reg2mem60, align 8
  %.reload62 = load i64, ptr %.reg2mem60, align 8
  %29 = icmp eq i64 %.reload62, 240
  br i1 %29, label %30, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %20
  %.reload61 = load i64, ptr %.reg2mem60, align 8
  store i64 %.reload61, ptr %.reg2mem102, align 8
  br label %20

30:                                               ; preds = %20
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  %31 = add nuw nsw i64 %.reload64, 1
  store i64 %31, ptr %.reg2mem57, align 8
  %.reload59 = load i64, ptr %.reg2mem57, align 8
  %32 = icmp eq i64 %.reload59, 240
  br i1 %32, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %30
  %.reload58 = load i64, ptr %.reg2mem57, align 8
  store i64 %.reload58, ptr %.reg2mem104, align 8
  br label %19

._crit_edge4:                                     ; preds = %30
  store i64 1, ptr %.reg2mem98, align 8
  store i64 0, ptr %.reg2mem100, align 8
  br label %33

33:                                               ; preds = %._crit_edge10, %._crit_edge4
  %.reload101 = load i64, ptr %.reg2mem100, align 8
  %.reload99 = load i64, ptr %.reg2mem98, align 8
  store i64 %.reload101, ptr %.reg2mem52, align 8
  store i64 %.reload99, ptr %.reg2mem48, align 8
  store i64 0, ptr %.reg2mem96, align 8
  br label %34

34:                                               ; preds = %._crit_edge7, %33
  %.reload97 = load i64, ptr %.reg2mem96, align 8
  %.reload56 = load i64, ptr %.reg2mem52, align 8
  %.reload85 = load ptr, ptr %.reg2mem82, align 8
  %35 = getelementptr inbounds [240 x double], ptr %.reload85, i64 %.reload56, i64 %.reload97
  %36 = load double, ptr %35, align 8, !tbaa !5
  %37 = fmul double %36, 1.200000e+00
  store double %37, ptr %35, align 8, !tbaa !5
  %38 = add nuw nsw i64 %.reload97, 1
  store i64 %38, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %.reload51 = load i64, ptr %.reg2mem48, align 8
  %39 = icmp eq i64 %.reload47, %.reload51
  br i1 %39, label %._crit_edge6, label %._crit_edge7, !llvm.loop !15

._crit_edge7:                                     ; preds = %34
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  store i64 %.reload46, ptr %.reg2mem96, align 8
  br label %34

._crit_edge6:                                     ; preds = %34
  store i64 0, ptr %.reg2mem94, align 8
  br label %40

40:                                               ; preds = %._crit_edge9, %._crit_edge6
  %.reload95 = load i64, ptr %.reg2mem94, align 8
  store i64 %.reload95, ptr %.reg2mem41, align 8
  %.reload44 = load i64, ptr %.reg2mem41, align 8
  %.reload55 = load i64, ptr %.reg2mem52, align 8
  %.reload79 = load ptr, ptr %.reg2mem77, align 8
  %41 = getelementptr inbounds [200 x double], ptr %.reload79, i64 %.reload55, i64 %.reload44
  store ptr %41, ptr %.reg2mem39, align 8
  store i64 0, ptr %.reg2mem92, align 8
  br label %42

42:                                               ; preds = %._crit_edge8, %40
  %.reload93 = load i64, ptr %.reg2mem92, align 8
  %.reload40 = load ptr, ptr %.reg2mem39, align 8
  %43 = load double, ptr %.reload40, align 8, !tbaa !5
  %44 = fmul double %43, 1.500000e+00
  %.reload43 = load i64, ptr %.reg2mem41, align 8
  %.reload78 = load ptr, ptr %.reg2mem77, align 8
  %45 = getelementptr inbounds [200 x double], ptr %.reload78, i64 %.reload93, i64 %.reload43
  %46 = load double, ptr %45, align 8, !tbaa !5
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %.reload84 = load ptr, ptr %.reg2mem82, align 8
  %47 = getelementptr inbounds [240 x double], ptr %.reload84, i64 %.reload54, i64 %.reload93
  %48 = load double, ptr %47, align 8, !tbaa !5
  %49 = tail call double @llvm.fmuladd.f64(double %44, double %46, double %48)
  store double %49, ptr %47, align 8, !tbaa !5
  %50 = add nuw nsw i64 %.reload93, 1
  store i64 %50, ptr %.reg2mem36, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %51 = icmp eq i64 %.reload38, %.reload50
  br i1 %51, label %52, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %42
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  store i64 %.reload37, ptr %.reg2mem92, align 8
  br label %42

52:                                               ; preds = %42
  %.reload42 = load i64, ptr %.reg2mem41, align 8
  %53 = add nuw nsw i64 %.reload42, 1
  store i64 %53, ptr %.reg2mem33, align 8
  %.reload35 = load i64, ptr %.reg2mem33, align 8
  %54 = icmp eq i64 %.reload35, 200
  br i1 %54, label %55, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %52
  %.reload34 = load i64, ptr %.reg2mem33, align 8
  store i64 %.reload34, ptr %.reg2mem94, align 8
  br label %40

55:                                               ; preds = %52
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  %56 = add nuw nsw i64 %.reload53, 1
  store i64 %56, ptr %.reg2mem30, align 8
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %57 = add nuw nsw i64 %.reload49, 1
  store i64 %57, ptr %.reg2mem28, align 8
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %58 = icmp eq i64 %.reload32, 240
  br i1 %58, label %59, label %._crit_edge10, !llvm.loop !18

._crit_edge10:                                    ; preds = %55
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  store i64 %.reload29, ptr %.reg2mem98, align 8
  store i64 %.reload31, ptr %.reg2mem100, align 8
  br label %33

59:                                               ; preds = %55
  %60 = load ptr, ptr @stderr, align 8, !tbaa !19
  %61 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %60, ptr noundef nonnull @.str) #4
  %62 = load ptr, ptr @stderr, align 8, !tbaa !19
  %63 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %62, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem90, align 8
  br label %64

64:                                               ; preds = %._crit_edge13, %59
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  store i64 %.reload91, ptr %.reg2mem24, align 8
  %.reload27 = load i64, ptr %.reg2mem24, align 8
  %65 = mul nuw nsw i64 %.reload27, 240
  store i64 %65, ptr %.reg2mem22, align 8
  store i64 0, ptr %.reg2mem88, align 8
  br label %66

66:                                               ; preds = %._crit_edge12, %64
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  store i64 %.reload89, ptr %.reg2mem18, align 8
  %.reload21 = load i64, ptr %.reg2mem18, align 8
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  %67 = add nuw nsw i64 %.reload21, %.reload23
  %68 = trunc nuw nsw i64 %67 to i32
  %69 = urem i32 %68, 20
  %70 = icmp eq i32 %69, 0
  br i1 %70, label %71, label %._crit_edge11

._crit_edge11:                                    ; preds = %66
  br label %74

71:                                               ; preds = %66
  %72 = load ptr, ptr @stderr, align 8, !tbaa !19
  %73 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %72, ptr noundef nonnull @.str.3) #4
  br label %74

74:                                               ; preds = %._crit_edge11, %71
  %75 = load ptr, ptr @stderr, align 8, !tbaa !19
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %.reload83 = load ptr, ptr %.reg2mem82, align 8
  %76 = getelementptr inbounds [240 x double], ptr %.reload83, i64 %.reload26, i64 %.reload20
  %77 = load double, ptr %76, align 8, !tbaa !5
  %78 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %75, ptr noundef nonnull @.str.4, double noundef %77) #4
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  %79 = add nuw nsw i64 %.reload19, 1
  store i64 %79, ptr %.reg2mem15, align 8
  %.reload17 = load i64, ptr %.reg2mem15, align 8
  %80 = icmp eq i64 %.reload17, 240
  br i1 %80, label %81, label %._crit_edge12, !llvm.loop !21

._crit_edge12:                                    ; preds = %74
  %.reload16 = load i64, ptr %.reg2mem15, align 8
  store i64 %.reload16, ptr %.reg2mem88, align 8
  br label %66

81:                                               ; preds = %74
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  %82 = add nuw nsw i64 %.reload25, 1
  store i64 %82, ptr %.reg2mem, align 8
  %.reload14 = load i64, ptr %.reg2mem, align 8
  %83 = icmp eq i64 %.reload14, 240
  br i1 %83, label %84, label %._crit_edge13, !llvm.loop !22

._crit_edge13:                                    ; preds = %81
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem90, align 8
  br label %64

84:                                               ; preds = %81
  %85 = load ptr, ptr @stderr, align 8, !tbaa !19
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %87 = load ptr, ptr @stderr, align 8, !tbaa !19
  %88 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %87, ptr noundef nonnull @.str.6) #4
  %.reload87 = load ptr, ptr %.reg2mem82, align 8
  tail call void @free(ptr noundef nonnull %.reload87) #4
  %.reload81 = load ptr, ptr %.reg2mem77, align 8
  tail call void @free(ptr noundef %.reload81) #4
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
!18 = distinct !{!18, !10, !11}
!19 = !{!20, !20, i64 0}
!20 = !{!"any pointer", !7, i64 0}
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}

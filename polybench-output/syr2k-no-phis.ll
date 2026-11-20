; ModuleID = './polybench-output/syr2k-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/syr2k/syr2k.c"
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
  %.reg2mem41 = alloca ptr, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem49 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem56 = alloca i64, align 8
  %.reg2mem62 = alloca i64, align 8
  %.reg2mem65 = alloca i64, align 8
  %.reg2mem68 = alloca i64, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem78 = alloca i64, align 8
  %.reg2mem83 = alloca ptr, align 8
  %.reg2mem88 = alloca ptr, align 8
  %.reg2mem93 = alloca ptr, align 8
  %.reg2mem99 = alloca i64, align 8
  %.reg2mem101 = alloca i64, align 8
  %.reg2mem103 = alloca i64, align 8
  %.reg2mem105 = alloca i64, align 8
  %.reg2mem107 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem111 = alloca i64, align 8
  %.reg2mem113 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem117 = alloca i64, align 8
  %.reg2mem119 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem93, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem88, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem83, align 8
  store i64 0, ptr %.reg2mem119, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload120 = load i64, ptr %.reg2mem119, align 8
  store i64 %.reload120, ptr %.reg2mem78, align 8
  store i64 0, ptr %.reg2mem117, align 8
  br label %7

7:                                                ; preds = %._crit_edge, %6
  %.reload118 = load i64, ptr %.reg2mem117, align 8
  %.reload82 = load i64, ptr %.reg2mem78, align 8
  %8 = mul nuw nsw i64 %.reload118, %.reload82
  %9 = trunc i64 %8 to i32
  %10 = add i32 %9, 1
  %11 = urem i32 %10, 240
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 2.400000e+02
  %.reload81 = load i64, ptr %.reg2mem78, align 8
  %.reload91 = load ptr, ptr %.reg2mem88, align 8
  %14 = getelementptr inbounds [200 x double], ptr %.reload91, i64 %.reload81, i64 %.reload118
  store double %13, ptr %14, align 8, !tbaa !5
  %15 = add i32 %9, 2
  %16 = urem i32 %15, 200
  %17 = uitofp nneg i32 %16 to double
  %18 = fdiv double %17, 2.000000e+02
  %.reload80 = load i64, ptr %.reg2mem78, align 8
  %.reload86 = load ptr, ptr %.reg2mem83, align 8
  %19 = getelementptr inbounds [200 x double], ptr %.reload86, i64 %.reload80, i64 %.reload118
  store double %18, ptr %19, align 8, !tbaa !5
  %20 = add nuw nsw i64 %.reload118, 1
  store i64 %20, ptr %.reg2mem75, align 8
  %.reload77 = load i64, ptr %.reg2mem75, align 8
  %21 = icmp eq i64 %.reload77, 200
  br i1 %21, label %22, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %7
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  store i64 %.reload76, ptr %.reg2mem117, align 8
  br label %7

22:                                               ; preds = %7
  %.reload79 = load i64, ptr %.reg2mem78, align 8
  %23 = add nuw nsw i64 %.reload79, 1
  store i64 %23, ptr %.reg2mem72, align 8
  %.reload74 = load i64, ptr %.reg2mem72, align 8
  %24 = icmp eq i64 %.reload74, 240
  br i1 %24, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %22
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem119, align 8
  br label %6

._crit_edge1:                                     ; preds = %22
  store i64 0, ptr %.reg2mem115, align 8
  br label %25

25:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  store i64 %.reload116, ptr %.reg2mem68, align 8
  store i64 0, ptr %.reg2mem113, align 8
  br label %26

26:                                               ; preds = %._crit_edge3, %25
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  %.reload71 = load i64, ptr %.reg2mem68, align 8
  %27 = mul nuw nsw i64 %.reload114, %.reload71
  %28 = trunc i64 %27 to i32
  %29 = add i32 %28, 3
  %30 = urem i32 %29, 240
  %31 = uitofp nneg i32 %30 to double
  %32 = fdiv double %31, 2.000000e+02
  %.reload70 = load i64, ptr %.reg2mem68, align 8
  %.reload97 = load ptr, ptr %.reg2mem93, align 8
  %33 = getelementptr inbounds [240 x double], ptr %.reload97, i64 %.reload70, i64 %.reload114
  store double %32, ptr %33, align 8, !tbaa !5
  %34 = add nuw nsw i64 %.reload114, 1
  store i64 %34, ptr %.reg2mem65, align 8
  %.reload67 = load i64, ptr %.reg2mem65, align 8
  %35 = icmp eq i64 %.reload67, 240
  br i1 %35, label %36, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %26
  %.reload66 = load i64, ptr %.reg2mem65, align 8
  store i64 %.reload66, ptr %.reg2mem113, align 8
  br label %26

36:                                               ; preds = %26
  %.reload69 = load i64, ptr %.reg2mem68, align 8
  %37 = add nuw nsw i64 %.reload69, 1
  store i64 %37, ptr %.reg2mem62, align 8
  %.reload64 = load i64, ptr %.reg2mem62, align 8
  %38 = icmp eq i64 %.reload64, 240
  br i1 %38, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %36
  %.reload63 = load i64, ptr %.reg2mem62, align 8
  store i64 %.reload63, ptr %.reg2mem115, align 8
  br label %25

._crit_edge4:                                     ; preds = %36
  store i64 1, ptr %.reg2mem109, align 8
  store i64 0, ptr %.reg2mem111, align 8
  br label %39

39:                                               ; preds = %._crit_edge10, %._crit_edge4
  %.reload112 = load i64, ptr %.reg2mem111, align 8
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  store i64 %.reload112, ptr %.reg2mem56, align 8
  store i64 %.reload110, ptr %.reg2mem52, align 8
  store i64 0, ptr %.reg2mem107, align 8
  br label %40

40:                                               ; preds = %._crit_edge7, %39
  %.reload108 = load i64, ptr %.reg2mem107, align 8
  %.reload61 = load i64, ptr %.reg2mem56, align 8
  %.reload96 = load ptr, ptr %.reg2mem93, align 8
  %41 = getelementptr inbounds [240 x double], ptr %.reload96, i64 %.reload61, i64 %.reload108
  %42 = load double, ptr %41, align 8, !tbaa !5
  %43 = fmul double %42, 1.200000e+00
  store double %43, ptr %41, align 8, !tbaa !5
  %44 = add nuw nsw i64 %.reload108, 1
  store i64 %44, ptr %.reg2mem49, align 8
  %.reload51 = load i64, ptr %.reg2mem49, align 8
  %.reload55 = load i64, ptr %.reg2mem52, align 8
  %45 = icmp eq i64 %.reload51, %.reload55
  br i1 %45, label %._crit_edge6, label %._crit_edge7, !llvm.loop !15

._crit_edge7:                                     ; preds = %40
  %.reload50 = load i64, ptr %.reg2mem49, align 8
  store i64 %.reload50, ptr %.reg2mem107, align 8
  br label %40

._crit_edge6:                                     ; preds = %40
  store i64 0, ptr %.reg2mem105, align 8
  br label %46

46:                                               ; preds = %._crit_edge9, %._crit_edge6
  %.reload106 = load i64, ptr %.reg2mem105, align 8
  store i64 %.reload106, ptr %.reg2mem43, align 8
  %.reload48 = load i64, ptr %.reg2mem43, align 8
  %.reload60 = load i64, ptr %.reg2mem56, align 8
  %.reload85 = load ptr, ptr %.reg2mem83, align 8
  %47 = getelementptr inbounds [200 x double], ptr %.reload85, i64 %.reload60, i64 %.reload48
  store ptr %47, ptr %.reg2mem41, align 8
  %.reload47 = load i64, ptr %.reg2mem43, align 8
  %.reload59 = load i64, ptr %.reg2mem56, align 8
  %.reload90 = load ptr, ptr %.reg2mem88, align 8
  %48 = getelementptr inbounds [200 x double], ptr %.reload90, i64 %.reload59, i64 %.reload47
  store ptr %48, ptr %.reg2mem39, align 8
  store i64 0, ptr %.reg2mem103, align 8
  br label %49

49:                                               ; preds = %._crit_edge8, %46
  %.reload104 = load i64, ptr %.reg2mem103, align 8
  %.reload46 = load i64, ptr %.reg2mem43, align 8
  %.reload89 = load ptr, ptr %.reg2mem88, align 8
  %50 = getelementptr inbounds [200 x double], ptr %.reload89, i64 %.reload104, i64 %.reload46
  %51 = load double, ptr %50, align 8, !tbaa !5
  %52 = fmul double %51, 1.500000e+00
  %.reload42 = load ptr, ptr %.reg2mem41, align 8
  %53 = load double, ptr %.reload42, align 8, !tbaa !5
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %.reload84 = load ptr, ptr %.reg2mem83, align 8
  %54 = getelementptr inbounds [200 x double], ptr %.reload84, i64 %.reload104, i64 %.reload45
  %55 = load double, ptr %54, align 8, !tbaa !5
  %56 = fmul double %55, 1.500000e+00
  %.reload40 = load ptr, ptr %.reg2mem39, align 8
  %57 = load double, ptr %.reload40, align 8, !tbaa !5
  %58 = fmul double %56, %57
  %59 = tail call double @llvm.fmuladd.f64(double %52, double %53, double %58)
  %.reload58 = load i64, ptr %.reg2mem56, align 8
  %.reload95 = load ptr, ptr %.reg2mem93, align 8
  %60 = getelementptr inbounds [240 x double], ptr %.reload95, i64 %.reload58, i64 %.reload104
  %61 = load double, ptr %60, align 8, !tbaa !5
  %62 = fadd double %61, %59
  store double %62, ptr %60, align 8, !tbaa !5
  %63 = add nuw nsw i64 %.reload104, 1
  store i64 %63, ptr %.reg2mem36, align 8
  %.reload38 = load i64, ptr %.reg2mem36, align 8
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %64 = icmp eq i64 %.reload38, %.reload54
  br i1 %64, label %65, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %49
  %.reload37 = load i64, ptr %.reg2mem36, align 8
  store i64 %.reload37, ptr %.reg2mem103, align 8
  br label %49

65:                                               ; preds = %49
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  %66 = add nuw nsw i64 %.reload44, 1
  store i64 %66, ptr %.reg2mem33, align 8
  %.reload35 = load i64, ptr %.reg2mem33, align 8
  %67 = icmp eq i64 %.reload35, 200
  br i1 %67, label %68, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %65
  %.reload34 = load i64, ptr %.reg2mem33, align 8
  store i64 %.reload34, ptr %.reg2mem105, align 8
  br label %46

68:                                               ; preds = %65
  %.reload57 = load i64, ptr %.reg2mem56, align 8
  %69 = add nuw nsw i64 %.reload57, 1
  store i64 %69, ptr %.reg2mem30, align 8
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  %70 = add nuw nsw i64 %.reload53, 1
  store i64 %70, ptr %.reg2mem28, align 8
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %71 = icmp eq i64 %.reload32, 240
  br i1 %71, label %72, label %._crit_edge10, !llvm.loop !18

._crit_edge10:                                    ; preds = %68
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  store i64 %.reload29, ptr %.reg2mem109, align 8
  store i64 %.reload31, ptr %.reg2mem111, align 8
  br label %39

72:                                               ; preds = %68
  %73 = load ptr, ptr @stderr, align 8, !tbaa !19
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str) #4
  %75 = load ptr, ptr @stderr, align 8, !tbaa !19
  %76 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %75, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem101, align 8
  br label %77

77:                                               ; preds = %._crit_edge13, %72
  %.reload102 = load i64, ptr %.reg2mem101, align 8
  store i64 %.reload102, ptr %.reg2mem24, align 8
  %.reload27 = load i64, ptr %.reg2mem24, align 8
  %78 = mul nuw nsw i64 %.reload27, 240
  store i64 %78, ptr %.reg2mem22, align 8
  store i64 0, ptr %.reg2mem99, align 8
  br label %79

79:                                               ; preds = %._crit_edge12, %77
  %.reload100 = load i64, ptr %.reg2mem99, align 8
  store i64 %.reload100, ptr %.reg2mem18, align 8
  %.reload21 = load i64, ptr %.reg2mem18, align 8
  %.reload23 = load i64, ptr %.reg2mem22, align 8
  %80 = add nuw nsw i64 %.reload21, %.reload23
  %81 = trunc nuw nsw i64 %80 to i32
  %82 = urem i32 %81, 20
  %83 = icmp eq i32 %82, 0
  br i1 %83, label %84, label %._crit_edge11

._crit_edge11:                                    ; preds = %79
  br label %87

84:                                               ; preds = %79
  %85 = load ptr, ptr @stderr, align 8, !tbaa !19
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.3) #4
  br label %87

87:                                               ; preds = %._crit_edge11, %84
  %88 = load ptr, ptr @stderr, align 8, !tbaa !19
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %.reload94 = load ptr, ptr %.reg2mem93, align 8
  %89 = getelementptr inbounds [240 x double], ptr %.reload94, i64 %.reload26, i64 %.reload20
  %90 = load double, ptr %89, align 8, !tbaa !5
  %91 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %88, ptr noundef nonnull @.str.4, double noundef %90) #4
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  %92 = add nuw nsw i64 %.reload19, 1
  store i64 %92, ptr %.reg2mem15, align 8
  %.reload17 = load i64, ptr %.reg2mem15, align 8
  %93 = icmp eq i64 %.reload17, 240
  br i1 %93, label %94, label %._crit_edge12, !llvm.loop !21

._crit_edge12:                                    ; preds = %87
  %.reload16 = load i64, ptr %.reg2mem15, align 8
  store i64 %.reload16, ptr %.reg2mem99, align 8
  br label %79

94:                                               ; preds = %87
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  %95 = add nuw nsw i64 %.reload25, 1
  store i64 %95, ptr %.reg2mem, align 8
  %.reload14 = load i64, ptr %.reg2mem, align 8
  %96 = icmp eq i64 %.reload14, 240
  br i1 %96, label %97, label %._crit_edge13, !llvm.loop !22

._crit_edge13:                                    ; preds = %94
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem101, align 8
  br label %77

97:                                               ; preds = %94
  %98 = load ptr, ptr @stderr, align 8, !tbaa !19
  %99 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %98, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %100 = load ptr, ptr @stderr, align 8, !tbaa !19
  %101 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %100, ptr noundef nonnull @.str.6) #4
  %.reload98 = load ptr, ptr %.reg2mem93, align 8
  tail call void @free(ptr noundef nonnull %.reload98) #4
  %.reload92 = load ptr, ptr %.reg2mem88, align 8
  tail call void @free(ptr noundef %.reload92) #4
  %.reload87 = load ptr, ptr %.reg2mem83, align 8
  tail call void @free(ptr noundef %.reload87) #4
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

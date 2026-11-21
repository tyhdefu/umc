; ModuleID = './polybench-output/trmm-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/trmm/trmm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"B\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem17 = alloca i64, align 8
  %.reg2mem20 = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem26 = alloca i64, align 8
  %.reg2mem30 = alloca i64, align 8
  %.reg2mem33 = alloca i64, align 8
  %.reg2mem36 = alloca double, align 8
  %.reg2mem40 = alloca i64, align 8
  %.reg2mem45 = alloca double, align 8
  %.reg2mem48 = alloca ptr, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem56 = alloca i1, align 1
  %.reg2mem58 = alloca i64, align 8
  %.reg2mem64 = alloca i64, align 8
  %.reg2mem67 = alloca i64, align 8
  %.reg2mem70 = alloca i64, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem84 = alloca ptr, align 8
  %.reg2mem90 = alloca ptr, align 8
  %.reg2mem95 = alloca i64, align 8
  %.reg2mem97 = alloca i64, align 8
  %.reg2mem99 = alloca double, align 8
  %.reg2mem101 = alloca double, align 8
  %.reg2mem103 = alloca i64, align 8
  %.reg2mem105 = alloca i64, align 8
  %.reg2mem107 = alloca i64, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem111 = alloca i64, align 8
  %.reg2mem113 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 40000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem90, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem84, align 8
  store i64 0, ptr %.reg2mem113, align 8
  br label %5

5:                                                ; preds = %._crit_edge6, %2
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  store i64 %.reload114, ptr %.reg2mem75, align 8
  %.reload83 = load i64, ptr %.reg2mem75, align 8
  %6 = icmp eq i64 %.reload83, 0
  br i1 %6, label %._crit_edge, label %._crit_edge1

._crit_edge1:                                     ; preds = %5
  store i64 0, ptr %.reg2mem111, align 8
  br label %7

._crit_edge:                                      ; preds = %5
  br label %16

7:                                                ; preds = %._crit_edge3, %._crit_edge1
  %.reload112 = load i64, ptr %.reg2mem111, align 8
  %.reload82 = load i64, ptr %.reg2mem75, align 8
  %8 = add nuw nsw i64 %.reload112, %.reload82
  %9 = trunc nuw nsw i64 %8 to i32
  %10 = urem i32 %9, 200
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 2.000000e+02
  %.reload81 = load i64, ptr %.reg2mem75, align 8
  %.reload93 = load ptr, ptr %.reg2mem90, align 8
  %13 = getelementptr inbounds [200 x double], ptr %.reload93, i64 %.reload81, i64 %.reload112
  store double %12, ptr %13, align 8, !tbaa !5
  %14 = add nuw nsw i64 %.reload112, 1
  store i64 %14, ptr %.reg2mem72, align 8
  %.reload74 = load i64, ptr %.reg2mem72, align 8
  %.reload80 = load i64, ptr %.reg2mem75, align 8
  %15 = icmp eq i64 %.reload74, %.reload80
  br i1 %15, label %._crit_edge2, label %._crit_edge3, !llvm.loop !9

._crit_edge3:                                     ; preds = %7
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem111, align 8
  br label %7

._crit_edge2:                                     ; preds = %7
  br label %16

16:                                               ; preds = %._crit_edge2, %._crit_edge
  %.reload79 = load i64, ptr %.reg2mem75, align 8
  %.reload92 = load ptr, ptr %.reg2mem90, align 8
  %17 = getelementptr inbounds [200 x double], ptr %.reload92, i64 %.reload79, i64 %.reload79
  store double 1.000000e+00, ptr %17, align 8, !tbaa !5
  %.reload78 = load i64, ptr %.reg2mem75, align 8
  %18 = add nuw nsw i64 %.reload78, 240
  store i64 %18, ptr %.reg2mem70, align 8
  store i64 0, ptr %.reg2mem109, align 8
  br label %19

19:                                               ; preds = %._crit_edge4, %16
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  %20 = sub nuw nsw i64 %.reload71, %.reload110
  %21 = trunc nuw nsw i64 %20 to i32
  %22 = urem i32 %21, 240
  %23 = uitofp nneg i32 %22 to double
  %24 = fdiv double %23, 2.400000e+02
  %.reload77 = load i64, ptr %.reg2mem75, align 8
  %.reload88 = load ptr, ptr %.reg2mem84, align 8
  %25 = getelementptr inbounds [240 x double], ptr %.reload88, i64 %.reload77, i64 %.reload110
  store double %24, ptr %25, align 8, !tbaa !5
  %26 = add nuw nsw i64 %.reload110, 1
  store i64 %26, ptr %.reg2mem67, align 8
  %.reload69 = load i64, ptr %.reg2mem67, align 8
  %27 = icmp eq i64 %.reload69, 240
  br i1 %27, label %28, label %._crit_edge4, !llvm.loop !12

._crit_edge4:                                     ; preds = %19
  %.reload68 = load i64, ptr %.reg2mem67, align 8
  store i64 %.reload68, ptr %.reg2mem109, align 8
  br label %19

28:                                               ; preds = %19
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  %29 = add nuw nsw i64 %.reload76, 1
  store i64 %29, ptr %.reg2mem64, align 8
  %.reload66 = load i64, ptr %.reg2mem64, align 8
  %30 = icmp eq i64 %.reload66, 200
  br i1 %30, label %._crit_edge5, label %._crit_edge6, !llvm.loop !13

._crit_edge6:                                     ; preds = %28
  %.reload65 = load i64, ptr %.reg2mem64, align 8
  store i64 %.reload65, ptr %.reg2mem113, align 8
  br label %5

._crit_edge5:                                     ; preds = %28
  store i64 0, ptr %.reg2mem107, align 8
  br label %31

31:                                               ; preds = %._crit_edge12, %._crit_edge5
  %.reload108 = load i64, ptr %.reg2mem107, align 8
  store i64 %.reload108, ptr %.reg2mem58, align 8
  %.reload63 = load i64, ptr %.reg2mem58, align 8
  %32 = icmp ult i64 %.reload63, 199
  store i1 %32, ptr %.reg2mem56, align 1
  store i64 0, ptr %.reg2mem105, align 8
  br label %33

33:                                               ; preds = %._crit_edge11, %31
  %.reload106 = load i64, ptr %.reg2mem105, align 8
  store i64 %.reload106, ptr %.reg2mem52, align 8
  %.reload55 = load i64, ptr %.reg2mem52, align 8
  %.reload62 = load i64, ptr %.reg2mem58, align 8
  %.reload87 = load ptr, ptr %.reg2mem84, align 8
  %34 = getelementptr inbounds [240 x double], ptr %.reload87, i64 %.reload62, i64 %.reload55
  store ptr %34, ptr %.reg2mem48, align 8
  %.reload51 = load ptr, ptr %.reg2mem48, align 8
  %35 = load double, ptr %.reload51, align 8, !tbaa !5
  store double %35, ptr %.reg2mem45, align 8
  %.reload57 = load i1, ptr %.reg2mem56, align 1
  br i1 %.reload57, label %._crit_edge7, label %._crit_edge8

._crit_edge8:                                     ; preds = %33
  %.reload47 = load double, ptr %.reg2mem45, align 8
  store double %.reload47, ptr %.reg2mem99, align 8
  br label %44

._crit_edge7:                                     ; preds = %33
  %.reload46 = load double, ptr %.reg2mem45, align 8
  %.reload59 = load i64, ptr %.reg2mem58, align 8
  store double %.reload46, ptr %.reg2mem101, align 8
  store i64 %.reload59, ptr %.reg2mem103, align 8
  br label %36

36:                                               ; preds = %._crit_edge10, %._crit_edge7
  %.reload104 = load i64, ptr %.reg2mem103, align 8
  %.reload102 = load double, ptr %.reg2mem101, align 8
  %37 = add nuw nsw i64 %.reload104, 1
  store i64 %37, ptr %.reg2mem40, align 8
  %.reload44 = load i64, ptr %.reg2mem40, align 8
  %.reload60 = load i64, ptr %.reg2mem58, align 8
  %.reload91 = load ptr, ptr %.reg2mem90, align 8
  %38 = getelementptr inbounds [200 x double], ptr %.reload91, i64 %.reload44, i64 %.reload60
  %39 = load double, ptr %38, align 8, !tbaa !5
  %.reload43 = load i64, ptr %.reg2mem40, align 8
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  %.reload86 = load ptr, ptr %.reg2mem84, align 8
  %40 = getelementptr inbounds [240 x double], ptr %.reload86, i64 %.reload43, i64 %.reload53
  %41 = load double, ptr %40, align 8, !tbaa !5
  %42 = tail call double @llvm.fmuladd.f64(double %39, double %41, double %.reload102)
  store double %42, ptr %.reg2mem36, align 8
  %.reload39 = load double, ptr %.reg2mem36, align 8
  %.reload50 = load ptr, ptr %.reg2mem48, align 8
  store double %.reload39, ptr %.reload50, align 8, !tbaa !5
  %.reload42 = load i64, ptr %.reg2mem40, align 8
  %43 = icmp eq i64 %.reload42, 199
  br i1 %43, label %._crit_edge9, label %._crit_edge10, !llvm.loop !14

._crit_edge10:                                    ; preds = %36
  %.reload38 = load double, ptr %.reg2mem36, align 8
  %.reload41 = load i64, ptr %.reg2mem40, align 8
  store double %.reload38, ptr %.reg2mem101, align 8
  store i64 %.reload41, ptr %.reg2mem103, align 8
  br label %36

._crit_edge9:                                     ; preds = %36
  %.reload37 = load double, ptr %.reg2mem36, align 8
  store double %.reload37, ptr %.reg2mem99, align 8
  br label %44

44:                                               ; preds = %._crit_edge9, %._crit_edge8
  %.reload100 = load double, ptr %.reg2mem99, align 8
  %45 = fmul double %.reload100, 1.500000e+00
  %.reload49 = load ptr, ptr %.reg2mem48, align 8
  store double %45, ptr %.reload49, align 8, !tbaa !5
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %46 = add nuw nsw i64 %.reload54, 1
  store i64 %46, ptr %.reg2mem33, align 8
  %.reload35 = load i64, ptr %.reg2mem33, align 8
  %47 = icmp eq i64 %.reload35, 240
  br i1 %47, label %48, label %._crit_edge11, !llvm.loop !15

._crit_edge11:                                    ; preds = %44
  %.reload34 = load i64, ptr %.reg2mem33, align 8
  store i64 %.reload34, ptr %.reg2mem105, align 8
  br label %33

48:                                               ; preds = %44
  %.reload61 = load i64, ptr %.reg2mem58, align 8
  %49 = add nuw nsw i64 %.reload61, 1
  store i64 %49, ptr %.reg2mem30, align 8
  %.reload32 = load i64, ptr %.reg2mem30, align 8
  %50 = icmp eq i64 %.reload32, 200
  br i1 %50, label %51, label %._crit_edge12, !llvm.loop !16

._crit_edge12:                                    ; preds = %48
  %.reload31 = load i64, ptr %.reg2mem30, align 8
  store i64 %.reload31, ptr %.reg2mem107, align 8
  br label %31

51:                                               ; preds = %48
  %52 = load ptr, ptr @stderr, align 8, !tbaa !17
  %53 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %52, ptr noundef nonnull @.str) #4
  %54 = load ptr, ptr @stderr, align 8, !tbaa !17
  %55 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %54, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem97, align 8
  br label %56

56:                                               ; preds = %._crit_edge15, %51
  %.reload98 = load i64, ptr %.reg2mem97, align 8
  store i64 %.reload98, ptr %.reg2mem26, align 8
  %.reload29 = load i64, ptr %.reg2mem26, align 8
  %57 = mul nuw nsw i64 %.reload29, 200
  store i64 %57, ptr %.reg2mem24, align 8
  store i64 0, ptr %.reg2mem95, align 8
  br label %58

58:                                               ; preds = %._crit_edge14, %56
  %.reload96 = load i64, ptr %.reg2mem95, align 8
  store i64 %.reload96, ptr %.reg2mem20, align 8
  %.reload23 = load i64, ptr %.reg2mem20, align 8
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  %59 = add nuw nsw i64 %.reload23, %.reload25
  %60 = trunc nuw nsw i64 %59 to i32
  %61 = urem i32 %60, 20
  %62 = icmp eq i32 %61, 0
  br i1 %62, label %63, label %._crit_edge13

._crit_edge13:                                    ; preds = %58
  br label %66

63:                                               ; preds = %58
  %64 = load ptr, ptr @stderr, align 8, !tbaa !17
  %65 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %64, ptr noundef nonnull @.str.3) #4
  br label %66

66:                                               ; preds = %._crit_edge13, %63
  %67 = load ptr, ptr @stderr, align 8, !tbaa !17
  %.reload22 = load i64, ptr %.reg2mem20, align 8
  %.reload28 = load i64, ptr %.reg2mem26, align 8
  %.reload85 = load ptr, ptr %.reg2mem84, align 8
  %68 = getelementptr inbounds [240 x double], ptr %.reload85, i64 %.reload28, i64 %.reload22
  %69 = load double, ptr %68, align 8, !tbaa !5
  %70 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.4, double noundef %69) #4
  %.reload21 = load i64, ptr %.reg2mem20, align 8
  %71 = add nuw nsw i64 %.reload21, 1
  store i64 %71, ptr %.reg2mem17, align 8
  %.reload19 = load i64, ptr %.reg2mem17, align 8
  %72 = icmp eq i64 %.reload19, 240
  br i1 %72, label %73, label %._crit_edge14, !llvm.loop !19

._crit_edge14:                                    ; preds = %66
  %.reload18 = load i64, ptr %.reg2mem17, align 8
  store i64 %.reload18, ptr %.reg2mem95, align 8
  br label %58

73:                                               ; preds = %66
  %.reload27 = load i64, ptr %.reg2mem26, align 8
  %74 = add nuw nsw i64 %.reload27, 1
  store i64 %74, ptr %.reg2mem, align 8
  %.reload16 = load i64, ptr %.reg2mem, align 8
  %75 = icmp eq i64 %.reload16, 200
  br i1 %75, label %76, label %._crit_edge15, !llvm.loop !20

._crit_edge15:                                    ; preds = %73
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem97, align 8
  br label %56

76:                                               ; preds = %73
  %77 = load ptr, ptr @stderr, align 8, !tbaa !17
  %78 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %77, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %79 = load ptr, ptr @stderr, align 8, !tbaa !17
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str.6) #4
  %.reload94 = load ptr, ptr %.reg2mem90, align 8
  tail call void @free(ptr noundef %.reload94) #4
  %.reload89 = load ptr, ptr %.reg2mem84, align 8
  tail call void @free(ptr noundef nonnull %.reload89) #4
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
!17 = !{!18, !18, i64 0}
!18 = !{!"any pointer", !7, i64 0}
!19 = distinct !{!19, !10, !11}
!20 = distinct !{!20, !10, !11}

; ModuleID = './polybench-output/gemm-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/blas/gemm/gemm.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"C\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [16 x i8] c"checksum %.17g\0A\00", align 1
@.str.6 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem21 = alloca double, align 8
  %.reg2mem25 = alloca double, align 8
  %.reg2mem27 = alloca i64, align 8
  %.reg2mem31 = alloca i64, align 8
  %.reg2mem33 = alloca double, align 8
  %.reg2mem35 = alloca i64, align 8
  %.reg2mem39 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca ptr, align 8
  %.reg2mem50 = alloca i64, align 8
  %.reg2mem54 = alloca i64, align 8
  %.reg2mem57 = alloca i64, align 8
  %.reg2mem62 = alloca i64, align 8
  %.reg2mem65 = alloca i64, align 8
  %.reg2mem68 = alloca i64, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem79 = alloca i64, align 8
  %.reg2mem83 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem89 = alloca i64, align 8
  %.reg2mem93 = alloca ptr, align 8
  %.reg2mem97 = alloca ptr, align 8
  %.reg2mem101 = alloca ptr, align 8
  %.reg2mem107 = alloca double, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem111 = alloca double, align 8
  %.reg2mem113 = alloca i64, align 8
  %.reg2mem115 = alloca i64, align 8
  %.reg2mem117 = alloca i64, align 8
  %.reg2mem119 = alloca i64, align 8
  %.reg2mem121 = alloca i64, align 8
  %.reg2mem123 = alloca i64, align 8
  %.reg2mem125 = alloca i64, align 8
  %.reg2mem127 = alloca i64, align 8
  %.reg2mem129 = alloca i64, align 8
  %.reg2mem131 = alloca i64, align 8
  %.reg2mem133 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 44000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem101, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 48000, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem97, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 52800, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem93, align 8
  store i64 0, ptr %.reg2mem133, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload134 = load i64, ptr %.reg2mem133, align 8
  store i64 %.reload134, ptr %.reg2mem89, align 8
  store i64 0, ptr %.reg2mem131, align 8
  br label %7

7:                                                ; preds = %._crit_edge, %6
  %.reload132 = load i64, ptr %.reg2mem131, align 8
  %.reload92 = load i64, ptr %.reg2mem89, align 8
  %8 = mul nuw nsw i64 %.reload132, %.reload92
  %9 = trunc i64 %8 to i32
  %10 = add i32 %9, 1
  %11 = urem i32 %10, 200
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 2.000000e+02
  %.reload91 = load i64, ptr %.reg2mem89, align 8
  %.reload105 = load ptr, ptr %.reg2mem101, align 8
  %14 = getelementptr inbounds [220 x double], ptr %.reload105, i64 %.reload91, i64 %.reload132
  store double %13, ptr %14, align 8, !tbaa !5
  %15 = add nuw nsw i64 %.reload132, 1
  store i64 %15, ptr %.reg2mem86, align 8
  %.reload88 = load i64, ptr %.reg2mem86, align 8
  %16 = icmp eq i64 %.reload88, 220
  br i1 %16, label %17, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %7
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  store i64 %.reload87, ptr %.reg2mem131, align 8
  br label %7

17:                                               ; preds = %7
  %.reload90 = load i64, ptr %.reg2mem89, align 8
  %18 = add nuw nsw i64 %.reload90, 1
  store i64 %18, ptr %.reg2mem83, align 8
  %.reload85 = load i64, ptr %.reg2mem83, align 8
  %19 = icmp eq i64 %.reload85, 200
  br i1 %19, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %17
  %.reload84 = load i64, ptr %.reg2mem83, align 8
  store i64 %.reload84, ptr %.reg2mem133, align 8
  br label %6

._crit_edge1:                                     ; preds = %17
  store i64 0, ptr %.reg2mem129, align 8
  br label %20

20:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload130 = load i64, ptr %.reg2mem129, align 8
  store i64 %.reload130, ptr %.reg2mem79, align 8
  store i64 0, ptr %.reg2mem127, align 8
  br label %21

21:                                               ; preds = %._crit_edge3, %20
  %.reload128 = load i64, ptr %.reg2mem127, align 8
  %22 = add nuw nsw i64 %.reload128, 1
  store i64 %22, ptr %.reg2mem75, align 8
  %.reload78 = load i64, ptr %.reg2mem75, align 8
  %.reload82 = load i64, ptr %.reg2mem79, align 8
  %23 = mul nuw nsw i64 %.reload78, %.reload82
  %24 = trunc nuw nsw i64 %23 to i32
  %25 = urem i32 %24, 240
  %26 = uitofp nneg i32 %25 to double
  %27 = fdiv double %26, 2.400000e+02
  %.reload81 = load i64, ptr %.reg2mem79, align 8
  %.reload99 = load ptr, ptr %.reg2mem97, align 8
  %28 = getelementptr inbounds [240 x double], ptr %.reload99, i64 %.reload81, i64 %.reload128
  store double %27, ptr %28, align 8, !tbaa !5
  %.reload77 = load i64, ptr %.reg2mem75, align 8
  %29 = icmp eq i64 %.reload77, 240
  br i1 %29, label %30, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %21
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  store i64 %.reload76, ptr %.reg2mem127, align 8
  br label %21

30:                                               ; preds = %21
  %.reload80 = load i64, ptr %.reg2mem79, align 8
  %31 = add nuw nsw i64 %.reload80, 1
  store i64 %31, ptr %.reg2mem72, align 8
  %.reload74 = load i64, ptr %.reg2mem72, align 8
  %32 = icmp eq i64 %.reload74, 200
  br i1 %32, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %30
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem129, align 8
  br label %20

._crit_edge4:                                     ; preds = %30
  store i64 0, ptr %.reg2mem125, align 8
  br label %33

33:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload126 = load i64, ptr %.reg2mem125, align 8
  store i64 %.reload126, ptr %.reg2mem68, align 8
  store i64 0, ptr %.reg2mem123, align 8
  br label %34

34:                                               ; preds = %._crit_edge6, %33
  %.reload124 = load i64, ptr %.reg2mem123, align 8
  %35 = add nuw nsw i64 %.reload124, 2
  %.reload71 = load i64, ptr %.reg2mem68, align 8
  %36 = mul nuw nsw i64 %35, %.reload71
  %37 = trunc nuw nsw i64 %36 to i32
  %38 = urem i32 %37, 220
  %39 = uitofp nneg i32 %38 to double
  %40 = fdiv double %39, 2.200000e+02
  %.reload70 = load i64, ptr %.reg2mem68, align 8
  %.reload95 = load ptr, ptr %.reg2mem93, align 8
  %41 = getelementptr inbounds [220 x double], ptr %.reload95, i64 %.reload70, i64 %.reload124
  store double %40, ptr %41, align 8, !tbaa !5
  %42 = add nuw nsw i64 %.reload124, 1
  store i64 %42, ptr %.reg2mem65, align 8
  %.reload67 = load i64, ptr %.reg2mem65, align 8
  %43 = icmp eq i64 %.reload67, 220
  br i1 %43, label %44, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %34
  %.reload66 = load i64, ptr %.reg2mem65, align 8
  store i64 %.reload66, ptr %.reg2mem123, align 8
  br label %34

44:                                               ; preds = %34
  %.reload69 = load i64, ptr %.reg2mem68, align 8
  %45 = add nuw nsw i64 %.reload69, 1
  store i64 %45, ptr %.reg2mem62, align 8
  %.reload64 = load i64, ptr %.reg2mem62, align 8
  %46 = icmp eq i64 %.reload64, 240
  br i1 %46, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %44
  %.reload63 = load i64, ptr %.reg2mem62, align 8
  store i64 %.reload63, ptr %.reg2mem125, align 8
  br label %33

._crit_edge7:                                     ; preds = %44
  store i64 0, ptr %.reg2mem121, align 8
  br label %47

47:                                               ; preds = %._crit_edge13, %._crit_edge7
  %.reload122 = load i64, ptr %.reg2mem121, align 8
  store i64 %.reload122, ptr %.reg2mem57, align 8
  store i64 0, ptr %.reg2mem119, align 8
  br label %48

48:                                               ; preds = %._crit_edge10, %47
  %.reload120 = load i64, ptr %.reg2mem119, align 8
  %.reload61 = load i64, ptr %.reg2mem57, align 8
  %.reload104 = load ptr, ptr %.reg2mem101, align 8
  %49 = getelementptr inbounds [220 x double], ptr %.reload104, i64 %.reload61, i64 %.reload120
  %50 = load double, ptr %49, align 8, !tbaa !5
  %51 = fmul double %50, 1.200000e+00
  store double %51, ptr %49, align 8, !tbaa !5
  %52 = add nuw nsw i64 %.reload120, 1
  store i64 %52, ptr %.reg2mem54, align 8
  %.reload56 = load i64, ptr %.reg2mem54, align 8
  %53 = icmp eq i64 %.reload56, 220
  br i1 %53, label %._crit_edge9, label %._crit_edge10, !llvm.loop !17

._crit_edge10:                                    ; preds = %48
  %.reload55 = load i64, ptr %.reg2mem54, align 8
  store i64 %.reload55, ptr %.reg2mem119, align 8
  br label %48

._crit_edge9:                                     ; preds = %48
  store i64 0, ptr %.reg2mem117, align 8
  br label %54

54:                                               ; preds = %._crit_edge12, %._crit_edge9
  %.reload118 = load i64, ptr %.reg2mem117, align 8
  store i64 %.reload118, ptr %.reg2mem50, align 8
  %.reload53 = load i64, ptr %.reg2mem50, align 8
  %.reload60 = load i64, ptr %.reg2mem57, align 8
  %.reload98 = load ptr, ptr %.reg2mem97, align 8
  %55 = getelementptr inbounds [240 x double], ptr %.reload98, i64 %.reload60, i64 %.reload53
  store ptr %55, ptr %.reg2mem48, align 8
  store i64 0, ptr %.reg2mem115, align 8
  br label %56

56:                                               ; preds = %._crit_edge11, %54
  %.reload116 = load i64, ptr %.reg2mem115, align 8
  %.reload49 = load ptr, ptr %.reg2mem48, align 8
  %57 = load double, ptr %.reload49, align 8, !tbaa !5
  %58 = fmul double %57, 1.500000e+00
  %.reload52 = load i64, ptr %.reg2mem50, align 8
  %.reload94 = load ptr, ptr %.reg2mem93, align 8
  %59 = getelementptr inbounds [220 x double], ptr %.reload94, i64 %.reload52, i64 %.reload116
  %60 = load double, ptr %59, align 8, !tbaa !5
  %.reload59 = load i64, ptr %.reg2mem57, align 8
  %.reload103 = load ptr, ptr %.reg2mem101, align 8
  %61 = getelementptr inbounds [220 x double], ptr %.reload103, i64 %.reload59, i64 %.reload116
  %62 = load double, ptr %61, align 8, !tbaa !5
  %63 = tail call double @llvm.fmuladd.f64(double %58, double %60, double %62)
  store double %63, ptr %61, align 8, !tbaa !5
  %64 = add nuw nsw i64 %.reload116, 1
  store i64 %64, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %65 = icmp eq i64 %.reload47, 220
  br i1 %65, label %66, label %._crit_edge11, !llvm.loop !18

._crit_edge11:                                    ; preds = %56
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  store i64 %.reload46, ptr %.reg2mem115, align 8
  br label %56

66:                                               ; preds = %56
  %.reload51 = load i64, ptr %.reg2mem50, align 8
  %67 = add nuw nsw i64 %.reload51, 1
  store i64 %67, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %68 = icmp eq i64 %.reload44, 240
  br i1 %68, label %69, label %._crit_edge12, !llvm.loop !19

._crit_edge12:                                    ; preds = %66
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  store i64 %.reload43, ptr %.reg2mem117, align 8
  br label %54

69:                                               ; preds = %66
  %.reload58 = load i64, ptr %.reg2mem57, align 8
  %70 = add nuw nsw i64 %.reload58, 1
  store i64 %70, ptr %.reg2mem39, align 8
  %.reload41 = load i64, ptr %.reg2mem39, align 8
  %71 = icmp eq i64 %.reload41, 200
  br i1 %71, label %72, label %._crit_edge13, !llvm.loop !20

._crit_edge13:                                    ; preds = %69
  %.reload40 = load i64, ptr %.reg2mem39, align 8
  store i64 %.reload40, ptr %.reg2mem121, align 8
  br label %47

72:                                               ; preds = %69
  %73 = load ptr, ptr @stderr, align 8, !tbaa !21
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str) #4
  %75 = load ptr, ptr @stderr, align 8, !tbaa !21
  %76 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %75, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store double 0.000000e+00, ptr %.reg2mem111, align 8
  store i64 0, ptr %.reg2mem113, align 8
  br label %77

77:                                               ; preds = %._crit_edge16, %72
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  %.reload112 = load double, ptr %.reg2mem111, align 8
  store i64 %.reload114, ptr %.reg2mem35, align 8
  store double %.reload112, ptr %.reg2mem33, align 8
  %.reload38 = load i64, ptr %.reg2mem35, align 8
  %78 = mul nuw nsw i64 %.reload38, 200
  store i64 %78, ptr %.reg2mem31, align 8
  %.reload34 = load double, ptr %.reg2mem33, align 8
  store double %.reload34, ptr %.reg2mem107, align 8
  store i64 0, ptr %.reg2mem109, align 8
  br label %79

79:                                               ; preds = %._crit_edge15, %77
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  %.reload108 = load double, ptr %.reg2mem107, align 8
  store i64 %.reload110, ptr %.reg2mem27, align 8
  store double %.reload108, ptr %.reg2mem25, align 8
  %.reload30 = load i64, ptr %.reg2mem27, align 8
  %.reload32 = load i64, ptr %.reg2mem31, align 8
  %80 = add nuw nsw i64 %.reload30, %.reload32
  %81 = trunc nuw nsw i64 %80 to i32
  %82 = urem i32 %81, 20
  %83 = icmp eq i32 %82, 0
  br i1 %83, label %84, label %._crit_edge14

._crit_edge14:                                    ; preds = %79
  br label %87

84:                                               ; preds = %79
  %85 = load ptr, ptr @stderr, align 8, !tbaa !21
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.3) #4
  br label %87

87:                                               ; preds = %._crit_edge14, %84
  %88 = load ptr, ptr @stderr, align 8, !tbaa !21
  %.reload29 = load i64, ptr %.reg2mem27, align 8
  %.reload37 = load i64, ptr %.reg2mem35, align 8
  %.reload102 = load ptr, ptr %.reg2mem101, align 8
  %89 = getelementptr inbounds [220 x double], ptr %.reload102, i64 %.reload37, i64 %.reload29
  %90 = load double, ptr %89, align 8, !tbaa !5
  %91 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %88, ptr noundef nonnull @.str.4, double noundef %90) #4
  %92 = load double, ptr %89, align 8, !tbaa !5
  %.reload26 = load double, ptr %.reg2mem25, align 8
  %93 = fadd double %.reload26, %92
  store double %93, ptr %.reg2mem21, align 8
  %.reload28 = load i64, ptr %.reg2mem27, align 8
  %94 = add nuw nsw i64 %.reload28, 1
  store i64 %94, ptr %.reg2mem18, align 8
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %95 = icmp eq i64 %.reload20, 220
  br i1 %95, label %96, label %._crit_edge15, !llvm.loop !23

._crit_edge15:                                    ; preds = %87
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  %.reload24 = load double, ptr %.reg2mem21, align 8
  store double %.reload24, ptr %.reg2mem107, align 8
  store i64 %.reload19, ptr %.reg2mem109, align 8
  br label %79

96:                                               ; preds = %87
  %.reload36 = load i64, ptr %.reg2mem35, align 8
  %97 = add nuw nsw i64 %.reload36, 1
  store i64 %97, ptr %.reg2mem, align 8
  %.reload17 = load i64, ptr %.reg2mem, align 8
  %98 = icmp eq i64 %.reload17, 200
  br i1 %98, label %99, label %._crit_edge16, !llvm.loop !24

._crit_edge16:                                    ; preds = %96
  %.reload = load i64, ptr %.reg2mem, align 8
  %.reload23 = load double, ptr %.reg2mem21, align 8
  store double %.reload23, ptr %.reg2mem111, align 8
  store i64 %.reload, ptr %.reg2mem113, align 8
  br label %77

99:                                               ; preds = %96
  %100 = load ptr, ptr @stderr, align 8, !tbaa !21
  %.reload22 = load double, ptr %.reg2mem21, align 8
  %101 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %100, ptr noundef nonnull @.str.5, double noundef %.reload22) #4
  %102 = load ptr, ptr @stderr, align 8, !tbaa !21
  %103 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %102, ptr noundef nonnull @.str.6, ptr noundef nonnull @.str.2) #4
  %104 = load ptr, ptr @stderr, align 8, !tbaa !21
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %104, ptr noundef nonnull @.str.7) #4
  %.reload106 = load ptr, ptr %.reg2mem101, align 8
  tail call void @free(ptr noundef nonnull %.reload106) #4
  %.reload100 = load ptr, ptr %.reg2mem97, align 8
  tail call void @free(ptr noundef %.reload100) #4
  %.reload96 = load ptr, ptr %.reg2mem93, align 8
  tail call void @free(ptr noundef %.reload96) #4
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
!19 = distinct !{!19, !10, !11}
!20 = distinct !{!20, !10, !11}
!21 = !{!22, !22, i64 0}
!22 = !{!"any pointer", !7, i64 0}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}

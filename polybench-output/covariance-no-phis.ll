; ModuleID = './polybench-output/covariance-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/datamining/covariance/covariance.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"cov\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem16 = alloca i64, align 8
  %.reg2mem19 = alloca i64, align 8
  %.reg2mem23 = alloca i64, align 8
  %.reg2mem25 = alloca i64, align 8
  %.reg2mem29 = alloca i64, align 8
  %.reg2mem32 = alloca i64, align 8
  %.reg2mem35 = alloca i64, align 8
  %.reg2mem38 = alloca double, align 8
  %.reg2mem42 = alloca ptr, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem51 = alloca i64, align 8
  %.reg2mem57 = alloca i64, align 8
  %.reg2mem60 = alloca i64, align 8
  %.reg2mem63 = alloca i64, align 8
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem69 = alloca i64, align 8
  %.reg2mem72 = alloca double, align 8
  %.reg2mem76 = alloca ptr, align 8
  %.reg2mem80 = alloca i64, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem87 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem94 = alloca ptr, align 8
  %.reg2mem98 = alloca ptr, align 8
  %.reg2mem103 = alloca ptr, align 8
  %.reg2mem110 = alloca i64, align 8
  %.reg2mem112 = alloca i64, align 8
  %.reg2mem114 = alloca double, align 8
  %.reg2mem116 = alloca i64, align 8
  %.reg2mem118 = alloca i64, align 8
  %.reg2mem120 = alloca i64, align 8
  %.reg2mem122 = alloca i64, align 8
  %.reg2mem124 = alloca i64, align 8
  %.reg2mem126 = alloca double, align 8
  %.reg2mem128 = alloca i64, align 8
  %.reg2mem130 = alloca i64, align 8
  %.reg2mem132 = alloca i64, align 8
  %.reg2mem134 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 62400, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem103, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 57600, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem98, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 240, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem94, align 8
  store i64 0, ptr %.reg2mem134, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload135 = load i64, ptr %.reg2mem134, align 8
  store i64 %.reload135, ptr %.reg2mem90, align 8
  store i64 0, ptr %.reg2mem132, align 8
  br label %7

7:                                                ; preds = %._crit_edge, %6
  %.reload133 = load i64, ptr %.reg2mem132, align 8
  %.reload93 = load i64, ptr %.reg2mem90, align 8
  %8 = mul nuw nsw i64 %.reload133, %.reload93
  %9 = trunc nuw nsw i64 %8 to i32
  %10 = uitofp nneg i32 %9 to double
  %11 = fdiv double %10, 2.400000e+02
  %.reload92 = load i64, ptr %.reg2mem90, align 8
  %.reload108 = load ptr, ptr %.reg2mem103, align 8
  %12 = getelementptr inbounds [240 x double], ptr %.reload108, i64 %.reload92, i64 %.reload133
  store double %11, ptr %12, align 8, !tbaa !5
  %13 = add nuw nsw i64 %.reload133, 1
  store i64 %13, ptr %.reg2mem87, align 8
  %.reload89 = load i64, ptr %.reg2mem87, align 8
  %14 = icmp eq i64 %.reload89, 240
  br i1 %14, label %15, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %7
  %.reload88 = load i64, ptr %.reg2mem87, align 8
  store i64 %.reload88, ptr %.reg2mem132, align 8
  br label %7

15:                                               ; preds = %7
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %16 = add nuw nsw i64 %.reload91, 1
  store i64 %16, ptr %.reg2mem84, align 8
  %.reload86 = load i64, ptr %.reg2mem84, align 8
  %17 = icmp eq i64 %.reload86, 260
  br i1 %17, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %15
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  store i64 %.reload85, ptr %.reg2mem134, align 8
  br label %6

._crit_edge1:                                     ; preds = %15
  store i64 0, ptr %.reg2mem130, align 8
  br label %18

18:                                               ; preds = %._crit_edge5, %._crit_edge1
  %.reload131 = load i64, ptr %.reg2mem130, align 8
  store i64 %.reload131, ptr %.reg2mem80, align 8
  %.reload83 = load i64, ptr %.reg2mem80, align 8
  %.reload96 = load ptr, ptr %.reg2mem94, align 8
  %19 = getelementptr inbounds double, ptr %.reload96, i64 %.reload83
  store ptr %19, ptr %.reg2mem76, align 8
  %.reload79 = load ptr, ptr %.reg2mem76, align 8
  store double 0.000000e+00, ptr %.reload79, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem126, align 8
  store i64 0, ptr %.reg2mem128, align 8
  br label %20

20:                                               ; preds = %._crit_edge3, %18
  %.reload129 = load i64, ptr %.reg2mem128, align 8
  %.reload127 = load double, ptr %.reg2mem126, align 8
  %.reload82 = load i64, ptr %.reg2mem80, align 8
  %.reload107 = load ptr, ptr %.reg2mem103, align 8
  %21 = getelementptr inbounds [240 x double], ptr %.reload107, i64 %.reload129, i64 %.reload82
  %22 = load double, ptr %21, align 8, !tbaa !5
  %23 = fadd double %.reload127, %22
  store double %23, ptr %.reg2mem72, align 8
  %.reload75 = load double, ptr %.reg2mem72, align 8
  %.reload78 = load ptr, ptr %.reg2mem76, align 8
  store double %.reload75, ptr %.reload78, align 8, !tbaa !5
  %24 = add nuw nsw i64 %.reload129, 1
  store i64 %24, ptr %.reg2mem69, align 8
  %.reload71 = load i64, ptr %.reg2mem69, align 8
  %25 = icmp eq i64 %.reload71, 260
  br i1 %25, label %26, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %20
  %.reload70 = load i64, ptr %.reg2mem69, align 8
  %.reload74 = load double, ptr %.reg2mem72, align 8
  store double %.reload74, ptr %.reg2mem126, align 8
  store i64 %.reload70, ptr %.reg2mem128, align 8
  br label %20

26:                                               ; preds = %20
  %.reload73 = load double, ptr %.reg2mem72, align 8
  %27 = fdiv double %.reload73, 2.600000e+02
  %.reload77 = load ptr, ptr %.reg2mem76, align 8
  store double %27, ptr %.reload77, align 8, !tbaa !5
  %.reload81 = load i64, ptr %.reg2mem80, align 8
  %28 = add nuw nsw i64 %.reload81, 1
  store i64 %28, ptr %.reg2mem66, align 8
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %29 = icmp eq i64 %.reload68, 240
  br i1 %29, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %26
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem130, align 8
  br label %18

._crit_edge4:                                     ; preds = %26
  store i64 0, ptr %.reg2mem124, align 8
  br label %30

30:                                               ; preds = %._crit_edge8, %._crit_edge4
  %.reload125 = load i64, ptr %.reg2mem124, align 8
  store i64 %.reload125, ptr %.reg2mem63, align 8
  store i64 0, ptr %.reg2mem122, align 8
  br label %31

31:                                               ; preds = %._crit_edge6, %30
  %.reload123 = load i64, ptr %.reg2mem122, align 8
  %.reload95 = load ptr, ptr %.reg2mem94, align 8
  %32 = getelementptr inbounds double, ptr %.reload95, i64 %.reload123
  %33 = load double, ptr %32, align 8, !tbaa !5
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %.reload106 = load ptr, ptr %.reg2mem103, align 8
  %34 = getelementptr inbounds [240 x double], ptr %.reload106, i64 %.reload65, i64 %.reload123
  %35 = load double, ptr %34, align 8, !tbaa !5
  %36 = fsub double %35, %33
  store double %36, ptr %34, align 8, !tbaa !5
  %37 = add nuw nsw i64 %.reload123, 1
  store i64 %37, ptr %.reg2mem60, align 8
  %.reload62 = load i64, ptr %.reg2mem60, align 8
  %38 = icmp eq i64 %.reload62, 240
  br i1 %38, label %39, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %31
  %.reload61 = load i64, ptr %.reg2mem60, align 8
  store i64 %.reload61, ptr %.reg2mem122, align 8
  br label %31

39:                                               ; preds = %31
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  %40 = add nuw nsw i64 %.reload64, 1
  store i64 %40, ptr %.reg2mem57, align 8
  %.reload59 = load i64, ptr %.reg2mem57, align 8
  %41 = icmp eq i64 %.reload59, 260
  br i1 %41, label %._crit_edge7, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %39
  %.reload58 = load i64, ptr %.reg2mem57, align 8
  store i64 %.reload58, ptr %.reg2mem124, align 8
  br label %30

._crit_edge7:                                     ; preds = %39
  store i64 0, ptr %.reg2mem120, align 8
  br label %42

42:                                               ; preds = %._crit_edge11, %._crit_edge7
  %.reload121 = load i64, ptr %.reg2mem120, align 8
  store i64 %.reload121, ptr %.reg2mem51, align 8
  %.reload52 = load i64, ptr %.reg2mem51, align 8
  store i64 %.reload52, ptr %.reg2mem118, align 8
  br label %43

43:                                               ; preds = %._crit_edge10, %42
  %.reload119 = load i64, ptr %.reg2mem118, align 8
  store i64 %.reload119, ptr %.reg2mem46, align 8
  %.reload50 = load i64, ptr %.reg2mem46, align 8
  %.reload56 = load i64, ptr %.reg2mem51, align 8
  %.reload101 = load ptr, ptr %.reg2mem98, align 8
  %44 = getelementptr inbounds [240 x double], ptr %.reload101, i64 %.reload56, i64 %.reload50
  store ptr %44, ptr %.reg2mem42, align 8
  %.reload45 = load ptr, ptr %.reg2mem42, align 8
  store double 0.000000e+00, ptr %.reload45, align 8, !tbaa !5
  store double 0.000000e+00, ptr %.reg2mem114, align 8
  store i64 0, ptr %.reg2mem116, align 8
  br label %45

45:                                               ; preds = %._crit_edge9, %43
  %.reload117 = load i64, ptr %.reg2mem116, align 8
  %.reload115 = load double, ptr %.reg2mem114, align 8
  %.reload55 = load i64, ptr %.reg2mem51, align 8
  %.reload105 = load ptr, ptr %.reg2mem103, align 8
  %46 = getelementptr inbounds [240 x double], ptr %.reload105, i64 %.reload117, i64 %.reload55
  %47 = load double, ptr %46, align 8, !tbaa !5
  %.reload49 = load i64, ptr %.reg2mem46, align 8
  %.reload104 = load ptr, ptr %.reg2mem103, align 8
  %48 = getelementptr inbounds [240 x double], ptr %.reload104, i64 %.reload117, i64 %.reload49
  %49 = load double, ptr %48, align 8, !tbaa !5
  %50 = tail call double @llvm.fmuladd.f64(double %47, double %49, double %.reload115)
  store double %50, ptr %.reg2mem38, align 8
  %.reload41 = load double, ptr %.reg2mem38, align 8
  %.reload44 = load ptr, ptr %.reg2mem42, align 8
  store double %.reload41, ptr %.reload44, align 8, !tbaa !5
  %51 = add nuw nsw i64 %.reload117, 1
  store i64 %51, ptr %.reg2mem35, align 8
  %.reload37 = load i64, ptr %.reg2mem35, align 8
  %52 = icmp eq i64 %.reload37, 260
  br i1 %52, label %53, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %45
  %.reload36 = load i64, ptr %.reg2mem35, align 8
  %.reload40 = load double, ptr %.reg2mem38, align 8
  store double %.reload40, ptr %.reg2mem114, align 8
  store i64 %.reload36, ptr %.reg2mem116, align 8
  br label %45

53:                                               ; preds = %45
  %.reload39 = load double, ptr %.reg2mem38, align 8
  %54 = fdiv double %.reload39, 2.590000e+02
  %.reload43 = load ptr, ptr %.reg2mem42, align 8
  store double %54, ptr %.reload43, align 8, !tbaa !5
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %.reload54 = load i64, ptr %.reg2mem51, align 8
  %.reload100 = load ptr, ptr %.reg2mem98, align 8
  %55 = getelementptr inbounds [240 x double], ptr %.reload100, i64 %.reload48, i64 %.reload54
  store double %54, ptr %55, align 8, !tbaa !5
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %56 = add nuw nsw i64 %.reload47, 1
  store i64 %56, ptr %.reg2mem32, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %57 = icmp eq i64 %.reload34, 240
  br i1 %57, label %58, label %._crit_edge10, !llvm.loop !18

._crit_edge10:                                    ; preds = %53
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  store i64 %.reload33, ptr %.reg2mem118, align 8
  br label %43

58:                                               ; preds = %53
  %.reload53 = load i64, ptr %.reg2mem51, align 8
  %59 = add nuw nsw i64 %.reload53, 1
  store i64 %59, ptr %.reg2mem29, align 8
  %.reload31 = load i64, ptr %.reg2mem29, align 8
  %60 = icmp eq i64 %.reload31, 240
  br i1 %60, label %61, label %._crit_edge11, !llvm.loop !19

._crit_edge11:                                    ; preds = %58
  %.reload30 = load i64, ptr %.reg2mem29, align 8
  store i64 %.reload30, ptr %.reg2mem120, align 8
  br label %42

61:                                               ; preds = %58
  %62 = load ptr, ptr @stderr, align 8, !tbaa !20
  %63 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %62, ptr noundef nonnull @.str) #4
  %64 = load ptr, ptr @stderr, align 8, !tbaa !20
  %65 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %64, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem112, align 8
  br label %66

66:                                               ; preds = %._crit_edge14, %61
  %.reload113 = load i64, ptr %.reg2mem112, align 8
  store i64 %.reload113, ptr %.reg2mem25, align 8
  %.reload28 = load i64, ptr %.reg2mem25, align 8
  %67 = mul nuw nsw i64 %.reload28, 240
  store i64 %67, ptr %.reg2mem23, align 8
  store i64 0, ptr %.reg2mem110, align 8
  br label %68

68:                                               ; preds = %._crit_edge13, %66
  %.reload111 = load i64, ptr %.reg2mem110, align 8
  store i64 %.reload111, ptr %.reg2mem19, align 8
  %.reload22 = load i64, ptr %.reg2mem19, align 8
  %.reload24 = load i64, ptr %.reg2mem23, align 8
  %69 = add nuw nsw i64 %.reload22, %.reload24
  %70 = trunc nuw nsw i64 %69 to i32
  %71 = urem i32 %70, 20
  %72 = icmp eq i32 %71, 0
  br i1 %72, label %73, label %._crit_edge12

._crit_edge12:                                    ; preds = %68
  br label %76

73:                                               ; preds = %68
  %74 = load ptr, ptr @stderr, align 8, !tbaa !20
  %75 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %74, ptr noundef nonnull @.str.3) #4
  br label %76

76:                                               ; preds = %._crit_edge12, %73
  %77 = load ptr, ptr @stderr, align 8, !tbaa !20
  %.reload21 = load i64, ptr %.reg2mem19, align 8
  %.reload27 = load i64, ptr %.reg2mem25, align 8
  %.reload99 = load ptr, ptr %.reg2mem98, align 8
  %78 = getelementptr inbounds [240 x double], ptr %.reload99, i64 %.reload27, i64 %.reload21
  %79 = load double, ptr %78, align 8, !tbaa !5
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %77, ptr noundef nonnull @.str.4, double noundef %79) #4
  %.reload20 = load i64, ptr %.reg2mem19, align 8
  %81 = add nuw nsw i64 %.reload20, 1
  store i64 %81, ptr %.reg2mem16, align 8
  %.reload18 = load i64, ptr %.reg2mem16, align 8
  %82 = icmp eq i64 %.reload18, 240
  br i1 %82, label %83, label %._crit_edge13, !llvm.loop !22

._crit_edge13:                                    ; preds = %76
  %.reload17 = load i64, ptr %.reg2mem16, align 8
  store i64 %.reload17, ptr %.reg2mem110, align 8
  br label %68

83:                                               ; preds = %76
  %.reload26 = load i64, ptr %.reg2mem25, align 8
  %84 = add nuw nsw i64 %.reload26, 1
  store i64 %84, ptr %.reg2mem, align 8
  %.reload15 = load i64, ptr %.reg2mem, align 8
  %85 = icmp eq i64 %.reload15, 240
  br i1 %85, label %86, label %._crit_edge14, !llvm.loop !23

._crit_edge14:                                    ; preds = %83
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem112, align 8
  br label %66

86:                                               ; preds = %83
  %87 = load ptr, ptr @stderr, align 8, !tbaa !20
  %88 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %87, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %89 = load ptr, ptr @stderr, align 8, !tbaa !20
  %90 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %89, ptr noundef nonnull @.str.6) #4
  %.reload109 = load ptr, ptr %.reg2mem103, align 8
  tail call void @free(ptr noundef %.reload109) #4
  %.reload102 = load ptr, ptr %.reg2mem98, align 8
  tail call void @free(ptr noundef nonnull %.reload102) #4
  %.reload97 = load ptr, ptr %.reg2mem94, align 8
  tail call void @free(ptr noundef %.reload97) #4
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
!20 = !{!21, !21, i64 0}
!21 = !{!"any pointer", !7, i64 0}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}

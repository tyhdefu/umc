; ModuleID = './polybench-output/bicg-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/bicg/bicg.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"s\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"q\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem14 = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem21 = alloca i64, align 8
  %.reg2mem25 = alloca i64, align 8
  %.reg2mem28 = alloca i64, align 8
  %.reg2mem31 = alloca ptr, align 8
  %.reg2mem33 = alloca ptr, align 8
  %.reg2mem37 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem52 = alloca i64, align 8
  %.reg2mem58 = alloca i64, align 8
  %.reg2mem61 = alloca ptr, align 8
  %.reg2mem65 = alloca ptr, align 8
  %.reg2mem69 = alloca ptr, align 8
  %.reg2mem73 = alloca ptr, align 8
  %.reg2mem78 = alloca ptr, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem88 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %.reg2mem92 = alloca i64, align 8
  %.reg2mem94 = alloca i64, align 8
  %.reg2mem96 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 159900, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem78, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem73, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem69, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 390, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem65, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 410, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem61, align 8
  store i64 0, ptr %.reg2mem96, align 8
  br label %8

8:                                                ; preds = %._crit_edge1, %2
  %.reload97 = load i64, ptr %.reg2mem96, align 8
  %9 = trunc nuw nsw i64 %.reload97 to i32
  %10 = uitofp nneg i32 %9 to double
  %11 = fdiv double %10, 3.900000e+02
  %.reload67 = load ptr, ptr %.reg2mem65, align 8
  %12 = getelementptr inbounds double, ptr %.reload67, i64 %.reload97
  store double %11, ptr %12, align 8, !tbaa !5
  %13 = add nuw nsw i64 %.reload97, 1
  store i64 %13, ptr %.reg2mem58, align 8
  %.reload60 = load i64, ptr %.reg2mem58, align 8
  %14 = icmp eq i64 %.reload60, 390
  br i1 %14, label %._crit_edge, label %._crit_edge1, !llvm.loop !9

._crit_edge1:                                     ; preds = %8
  %.reload59 = load i64, ptr %.reg2mem58, align 8
  store i64 %.reload59, ptr %.reg2mem96, align 8
  br label %8

._crit_edge:                                      ; preds = %8
  store i64 0, ptr %.reg2mem94, align 8
  br label %15

15:                                               ; preds = %._crit_edge4, %._crit_edge
  %.reload95 = load i64, ptr %.reg2mem94, align 8
  store i64 %.reload95, ptr %.reg2mem52, align 8
  %.reload57 = load i64, ptr %.reg2mem52, align 8
  %16 = trunc nuw nsw i64 %.reload57 to i32
  %17 = uitofp nneg i32 %16 to double
  %18 = fdiv double %17, 4.100000e+02
  %.reload56 = load i64, ptr %.reg2mem52, align 8
  %.reload63 = load ptr, ptr %.reg2mem61, align 8
  %19 = getelementptr inbounds double, ptr %.reload63, i64 %.reload56
  store double %18, ptr %19, align 8, !tbaa !5
  store i64 0, ptr %.reg2mem92, align 8
  br label %20

20:                                               ; preds = %._crit_edge2, %15
  %.reload93 = load i64, ptr %.reg2mem92, align 8
  %21 = add nuw nsw i64 %.reload93, 1
  store i64 %21, ptr %.reg2mem48, align 8
  %.reload51 = load i64, ptr %.reg2mem48, align 8
  %.reload55 = load i64, ptr %.reg2mem52, align 8
  %22 = mul nuw nsw i64 %.reload51, %.reload55
  %23 = trunc nuw nsw i64 %22 to i32
  %24 = urem i32 %23, 410
  %25 = uitofp nneg i32 %24 to double
  %26 = fdiv double %25, 4.100000e+02
  %.reload54 = load i64, ptr %.reg2mem52, align 8
  %.reload80 = load ptr, ptr %.reg2mem78, align 8
  %27 = getelementptr inbounds [390 x double], ptr %.reload80, i64 %.reload54, i64 %.reload93
  store double %26, ptr %27, align 8, !tbaa !5
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %28 = icmp eq i64 %.reload50, 390
  br i1 %28, label %29, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %20
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  store i64 %.reload49, ptr %.reg2mem92, align 8
  br label %20

29:                                               ; preds = %20
  %.reload53 = load i64, ptr %.reg2mem52, align 8
  %30 = add nuw nsw i64 %.reload53, 1
  store i64 %30, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %31 = icmp eq i64 %.reload47, 410
  br i1 %31, label %._crit_edge3, label %._crit_edge4, !llvm.loop !13

._crit_edge4:                                     ; preds = %29
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  store i64 %.reload46, ptr %.reg2mem94, align 8
  br label %15

._crit_edge3:                                     ; preds = %29
  store i64 0, ptr %.reg2mem90, align 8
  br label %32

32:                                               ; preds = %._crit_edge6, %._crit_edge3
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %.reload76 = load ptr, ptr %.reg2mem73, align 8
  %33 = getelementptr inbounds double, ptr %.reload76, i64 %.reload91
  store double 0.000000e+00, ptr %33, align 8, !tbaa !5
  %34 = add nuw nsw i64 %.reload91, 1
  store i64 %34, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %35 = icmp eq i64 %.reload44, 390
  br i1 %35, label %._crit_edge5, label %._crit_edge6, !llvm.loop !14

._crit_edge6:                                     ; preds = %32
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  store i64 %.reload43, ptr %.reg2mem90, align 8
  br label %32

._crit_edge5:                                     ; preds = %32
  store i64 0, ptr %.reg2mem88, align 8
  br label %36

36:                                               ; preds = %._crit_edge8, %._crit_edge5
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  store i64 %.reload89, ptr %.reg2mem37, align 8
  %.reload41 = load i64, ptr %.reg2mem37, align 8
  %.reload71 = load ptr, ptr %.reg2mem69, align 8
  %37 = getelementptr inbounds double, ptr %.reload71, i64 %.reload41
  store ptr %37, ptr %.reg2mem33, align 8
  %.reload36 = load ptr, ptr %.reg2mem33, align 8
  store double 0.000000e+00, ptr %.reload36, align 8, !tbaa !5
  %.reload40 = load i64, ptr %.reg2mem37, align 8
  %.reload62 = load ptr, ptr %.reg2mem61, align 8
  %38 = getelementptr inbounds double, ptr %.reload62, i64 %.reload40
  store ptr %38, ptr %.reg2mem31, align 8
  store i64 0, ptr %.reg2mem86, align 8
  br label %39

39:                                               ; preds = %._crit_edge7, %36
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  %.reload75 = load ptr, ptr %.reg2mem73, align 8
  %40 = getelementptr inbounds double, ptr %.reload75, i64 %.reload87
  %41 = load double, ptr %40, align 8, !tbaa !5
  %.reload32 = load ptr, ptr %.reg2mem31, align 8
  %42 = load double, ptr %.reload32, align 8, !tbaa !5
  %.reload39 = load i64, ptr %.reg2mem37, align 8
  %.reload79 = load ptr, ptr %.reg2mem78, align 8
  %43 = getelementptr inbounds [390 x double], ptr %.reload79, i64 %.reload39, i64 %.reload87
  %44 = load double, ptr %43, align 8, !tbaa !5
  %45 = tail call double @llvm.fmuladd.f64(double %42, double %44, double %41)
  store double %45, ptr %40, align 8, !tbaa !5
  %.reload35 = load ptr, ptr %.reg2mem33, align 8
  %46 = load double, ptr %.reload35, align 8, !tbaa !5
  %47 = load double, ptr %43, align 8, !tbaa !5
  %.reload66 = load ptr, ptr %.reg2mem65, align 8
  %48 = getelementptr inbounds double, ptr %.reload66, i64 %.reload87
  %49 = load double, ptr %48, align 8, !tbaa !5
  %50 = tail call double @llvm.fmuladd.f64(double %47, double %49, double %46)
  %.reload34 = load ptr, ptr %.reg2mem33, align 8
  store double %50, ptr %.reload34, align 8, !tbaa !5
  %51 = add nuw nsw i64 %.reload87, 1
  store i64 %51, ptr %.reg2mem28, align 8
  %.reload30 = load i64, ptr %.reg2mem28, align 8
  %52 = icmp eq i64 %.reload30, 390
  br i1 %52, label %53, label %._crit_edge7, !llvm.loop !15

._crit_edge7:                                     ; preds = %39
  %.reload29 = load i64, ptr %.reg2mem28, align 8
  store i64 %.reload29, ptr %.reg2mem86, align 8
  br label %39

53:                                               ; preds = %39
  %.reload38 = load i64, ptr %.reg2mem37, align 8
  %54 = add nuw nsw i64 %.reload38, 1
  store i64 %54, ptr %.reg2mem25, align 8
  %.reload27 = load i64, ptr %.reg2mem25, align 8
  %55 = icmp eq i64 %.reload27, 410
  br i1 %55, label %56, label %._crit_edge8, !llvm.loop !16

._crit_edge8:                                     ; preds = %53
  %.reload26 = load i64, ptr %.reg2mem25, align 8
  store i64 %.reload26, ptr %.reg2mem88, align 8
  br label %36

56:                                               ; preds = %53
  %57 = load ptr, ptr @stderr, align 8, !tbaa !17
  %58 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %57, ptr noundef nonnull @.str) #4
  %59 = load ptr, ptr @stderr, align 8, !tbaa !17
  %60 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %59, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem84, align 8
  br label %61

61:                                               ; preds = %._crit_edge10, %56
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  store i64 %.reload85, ptr %.reg2mem21, align 8
  %.reload24 = load i64, ptr %.reg2mem21, align 8
  %62 = trunc i64 %.reload24 to i16
  %63 = urem i16 %62, 20
  %64 = icmp eq i16 %63, 0
  br i1 %64, label %65, label %._crit_edge9

._crit_edge9:                                     ; preds = %61
  br label %68

65:                                               ; preds = %61
  %66 = load ptr, ptr @stderr, align 8, !tbaa !17
  %67 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %66, ptr noundef nonnull @.str.3) #4
  br label %68

68:                                               ; preds = %._crit_edge9, %65
  %69 = load ptr, ptr @stderr, align 8, !tbaa !17
  %.reload23 = load i64, ptr %.reg2mem21, align 8
  %.reload74 = load ptr, ptr %.reg2mem73, align 8
  %70 = getelementptr inbounds double, ptr %.reload74, i64 %.reload23
  %71 = load double, ptr %70, align 8, !tbaa !5
  %72 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %69, ptr noundef nonnull @.str.4, double noundef %71) #4
  %.reload22 = load i64, ptr %.reg2mem21, align 8
  %73 = add nuw nsw i64 %.reload22, 1
  store i64 %73, ptr %.reg2mem18, align 8
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %74 = icmp eq i64 %.reload20, 390
  br i1 %74, label %75, label %._crit_edge10, !llvm.loop !19

._crit_edge10:                                    ; preds = %68
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  store i64 %.reload19, ptr %.reg2mem84, align 8
  br label %61

75:                                               ; preds = %68
  %76 = load ptr, ptr @stderr, align 8, !tbaa !17
  %77 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %76, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %78 = load ptr, ptr @stderr, align 8, !tbaa !17
  %79 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %78, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  store i64 0, ptr %.reg2mem82, align 8
  br label %80

80:                                               ; preds = %._crit_edge12, %75
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  store i64 %.reload83, ptr %.reg2mem14, align 8
  %.reload17 = load i64, ptr %.reg2mem14, align 8
  %81 = trunc i64 %.reload17 to i16
  %82 = urem i16 %81, 20
  %83 = icmp eq i16 %82, 0
  br i1 %83, label %84, label %._crit_edge11

._crit_edge11:                                    ; preds = %80
  br label %87

84:                                               ; preds = %80
  %85 = load ptr, ptr @stderr, align 8, !tbaa !17
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %85, ptr noundef nonnull @.str.3) #4
  br label %87

87:                                               ; preds = %._crit_edge11, %84
  %88 = load ptr, ptr @stderr, align 8, !tbaa !17
  %.reload16 = load i64, ptr %.reg2mem14, align 8
  %.reload70 = load ptr, ptr %.reg2mem69, align 8
  %89 = getelementptr inbounds double, ptr %.reload70, i64 %.reload16
  %90 = load double, ptr %89, align 8, !tbaa !5
  %91 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %88, ptr noundef nonnull @.str.4, double noundef %90) #4
  %.reload15 = load i64, ptr %.reg2mem14, align 8
  %92 = add nuw nsw i64 %.reload15, 1
  store i64 %92, ptr %.reg2mem, align 8
  %.reload13 = load i64, ptr %.reg2mem, align 8
  %93 = icmp eq i64 %.reload13, 410
  br i1 %93, label %94, label %._crit_edge12, !llvm.loop !20

._crit_edge12:                                    ; preds = %87
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem82, align 8
  br label %80

94:                                               ; preds = %87
  %95 = load ptr, ptr @stderr, align 8, !tbaa !17
  %96 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %95, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %97 = load ptr, ptr @stderr, align 8, !tbaa !17
  %98 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %97, ptr noundef nonnull @.str.7) #4
  %.reload81 = load ptr, ptr %.reg2mem78, align 8
  tail call void @free(ptr noundef %.reload81) #4
  %.reload77 = load ptr, ptr %.reg2mem73, align 8
  tail call void @free(ptr noundef %.reload77) #4
  %.reload72 = load ptr, ptr %.reg2mem69, align 8
  tail call void @free(ptr noundef nonnull %.reload72) #4
  %.reload68 = load ptr, ptr %.reg2mem65, align 8
  tail call void @free(ptr noundef %.reload68) #4
  %.reload64 = load ptr, ptr %.reg2mem61, align 8
  tail call void @free(ptr noundef %.reload64) #4
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

; ModuleID = './polybench-output/trisolv-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/trisolv/trisolv.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"x\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.4 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem11 = alloca i64, align 8
  %.reg2mem15 = alloca i64, align 8
  %.reg2mem18 = alloca i64, align 8
  %.reg2mem21 = alloca double, align 8
  %.reg2mem25 = alloca ptr, align 8
  %.reg2mem29 = alloca double, align 8
  %.reg2mem33 = alloca i64, align 8
  %.reg2mem41 = alloca i64, align 8
  %.reg2mem43 = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem49 = alloca i64, align 8
  %.reg2mem51 = alloca i64, align 8
  %.reg2mem54 = alloca i64, align 8
  %.reg2mem61 = alloca ptr, align 8
  %.reg2mem65 = alloca ptr, align 8
  %.reg2mem71 = alloca ptr, align 8
  %.reg2mem76 = alloca i64, align 8
  %.reg2mem78 = alloca double, align 8
  %.reg2mem80 = alloca double, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem86 = alloca i64, align 8
  %.reg2mem88 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem71, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem65, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem61, align 8
  store i64 1, ptr %.reg2mem88, align 8
  store i64 0, ptr %.reg2mem90, align 8
  br label %6

6:                                                ; preds = %._crit_edge2, %2
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  store i64 %.reload91, ptr %.reg2mem54, align 8
  store i64 %.reload89, ptr %.reg2mem51, align 8
  %.reload60 = load i64, ptr %.reg2mem54, align 8
  %.reload69 = load ptr, ptr %.reg2mem65, align 8
  %7 = getelementptr inbounds double, ptr %.reload69, i64 %.reload60
  store double -9.990000e+02, ptr %7, align 8, !tbaa !5
  %.reload59 = load i64, ptr %.reg2mem54, align 8
  %8 = trunc nuw nsw i64 %.reload59 to i32
  %9 = uitofp nneg i32 %8 to double
  %.reload58 = load i64, ptr %.reg2mem54, align 8
  %.reload63 = load ptr, ptr %.reg2mem61, align 8
  %10 = getelementptr inbounds double, ptr %.reload63, i64 %.reload58
  store double %9, ptr %10, align 8, !tbaa !5
  %.reload57 = load i64, ptr %.reg2mem54, align 8
  %11 = add nuw nsw i64 %.reload57, 401
  store i64 %11, ptr %.reg2mem49, align 8
  store i64 0, ptr %.reg2mem86, align 8
  br label %12

12:                                               ; preds = %._crit_edge, %6
  %.reload87 = load i64, ptr %.reg2mem86, align 8
  %.reload50 = load i64, ptr %.reg2mem49, align 8
  %13 = sub nuw nsw i64 %.reload50, %.reload87
  %14 = trunc nuw nsw i64 %13 to i32
  %15 = sitofp i32 %14 to double
  %16 = fmul double %15, 2.000000e+00
  %17 = fdiv double %16, 4.000000e+02
  %.reload56 = load i64, ptr %.reg2mem54, align 8
  %.reload74 = load ptr, ptr %.reg2mem71, align 8
  %18 = getelementptr inbounds [400 x double], ptr %.reload74, i64 %.reload56, i64 %.reload87
  store double %17, ptr %18, align 8, !tbaa !5
  %19 = add nuw nsw i64 %.reload87, 1
  store i64 %19, ptr %.reg2mem46, align 8
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %.reload53 = load i64, ptr %.reg2mem51, align 8
  %20 = icmp eq i64 %.reload48, %.reload53
  br i1 %20, label %21, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %12
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  store i64 %.reload47, ptr %.reg2mem86, align 8
  br label %12

21:                                               ; preds = %12
  %.reload55 = load i64, ptr %.reg2mem54, align 8
  %22 = add nuw nsw i64 %.reload55, 1
  store i64 %22, ptr %.reg2mem43, align 8
  %.reload52 = load i64, ptr %.reg2mem51, align 8
  %23 = add nuw nsw i64 %.reload52, 1
  store i64 %23, ptr %.reg2mem41, align 8
  %.reload45 = load i64, ptr %.reg2mem43, align 8
  %24 = icmp eq i64 %.reload45, 400
  br i1 %24, label %._crit_edge1, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %21
  %.reload42 = load i64, ptr %.reg2mem41, align 8
  %.reload44 = load i64, ptr %.reg2mem43, align 8
  store i64 %.reload42, ptr %.reg2mem88, align 8
  store i64 %.reload44, ptr %.reg2mem90, align 8
  br label %6

._crit_edge1:                                     ; preds = %21
  store i64 0, ptr %.reg2mem84, align 8
  br label %25

25:                                               ; preds = %._crit_edge7, %._crit_edge1
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  store i64 %.reload85, ptr %.reg2mem33, align 8
  %.reload40 = load i64, ptr %.reg2mem33, align 8
  %.reload62 = load ptr, ptr %.reg2mem61, align 8
  %26 = getelementptr inbounds double, ptr %.reload62, i64 %.reload40
  %27 = load double, ptr %26, align 8, !tbaa !5
  store double %27, ptr %.reg2mem29, align 8
  %.reload39 = load i64, ptr %.reg2mem33, align 8
  %.reload68 = load ptr, ptr %.reg2mem65, align 8
  %28 = getelementptr inbounds double, ptr %.reload68, i64 %.reload39
  store ptr %28, ptr %.reg2mem25, align 8
  %.reload28 = load ptr, ptr %.reg2mem25, align 8
  %.reload32 = load double, ptr %.reg2mem29, align 8
  store double %.reload32, ptr %.reload28, align 8, !tbaa !5
  %.reload38 = load i64, ptr %.reg2mem33, align 8
  %29 = icmp eq i64 %.reload38, 0
  br i1 %29, label %._crit_edge3, label %._crit_edge4

._crit_edge4:                                     ; preds = %25
  %.reload30 = load double, ptr %.reg2mem29, align 8
  store double %.reload30, ptr %.reg2mem80, align 8
  store i64 0, ptr %.reg2mem82, align 8
  br label %30

._crit_edge3:                                     ; preds = %25
  %.reload31 = load double, ptr %.reg2mem29, align 8
  store double %.reload31, ptr %.reg2mem78, align 8
  br label %39

30:                                               ; preds = %._crit_edge6, %._crit_edge4
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %.reload81 = load double, ptr %.reg2mem80, align 8
  %.reload37 = load i64, ptr %.reg2mem33, align 8
  %.reload73 = load ptr, ptr %.reg2mem71, align 8
  %31 = getelementptr inbounds [400 x double], ptr %.reload73, i64 %.reload37, i64 %.reload83
  %32 = load double, ptr %31, align 8, !tbaa !5
  %.reload67 = load ptr, ptr %.reg2mem65, align 8
  %33 = getelementptr inbounds double, ptr %.reload67, i64 %.reload83
  %34 = load double, ptr %33, align 8, !tbaa !5
  %35 = fneg double %32
  %36 = tail call double @llvm.fmuladd.f64(double %35, double %34, double %.reload81)
  store double %36, ptr %.reg2mem21, align 8
  %.reload24 = load double, ptr %.reg2mem21, align 8
  %.reload27 = load ptr, ptr %.reg2mem25, align 8
  store double %.reload24, ptr %.reload27, align 8, !tbaa !5
  %37 = add nuw nsw i64 %.reload83, 1
  store i64 %37, ptr %.reg2mem18, align 8
  %.reload20 = load i64, ptr %.reg2mem18, align 8
  %.reload36 = load i64, ptr %.reg2mem33, align 8
  %38 = icmp eq i64 %.reload20, %.reload36
  br i1 %38, label %._crit_edge5, label %._crit_edge6, !llvm.loop !13

._crit_edge6:                                     ; preds = %30
  %.reload19 = load i64, ptr %.reg2mem18, align 8
  %.reload23 = load double, ptr %.reg2mem21, align 8
  store double %.reload23, ptr %.reg2mem80, align 8
  store i64 %.reload19, ptr %.reg2mem82, align 8
  br label %30

._crit_edge5:                                     ; preds = %30
  %.reload22 = load double, ptr %.reg2mem21, align 8
  store double %.reload22, ptr %.reg2mem78, align 8
  br label %39

39:                                               ; preds = %._crit_edge5, %._crit_edge3
  %.reload79 = load double, ptr %.reg2mem78, align 8
  %.reload35 = load i64, ptr %.reg2mem33, align 8
  %.reload72 = load ptr, ptr %.reg2mem71, align 8
  %40 = getelementptr inbounds [400 x double], ptr %.reload72, i64 %.reload35, i64 %.reload35
  %41 = load double, ptr %40, align 8, !tbaa !5
  %42 = fdiv double %.reload79, %41
  %.reload26 = load ptr, ptr %.reg2mem25, align 8
  store double %42, ptr %.reload26, align 8, !tbaa !5
  %.reload34 = load i64, ptr %.reg2mem33, align 8
  %43 = add nuw nsw i64 %.reload34, 1
  store i64 %43, ptr %.reg2mem15, align 8
  %.reload17 = load i64, ptr %.reg2mem15, align 8
  %44 = icmp eq i64 %.reload17, 400
  br i1 %44, label %45, label %._crit_edge7, !llvm.loop !14

._crit_edge7:                                     ; preds = %39
  %.reload16 = load i64, ptr %.reg2mem15, align 8
  store i64 %.reload16, ptr %.reg2mem84, align 8
  br label %25

45:                                               ; preds = %39
  %46 = load ptr, ptr @stderr, align 8, !tbaa !15
  %47 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %46, ptr noundef nonnull @.str) #4
  %48 = load ptr, ptr @stderr, align 8, !tbaa !15
  %49 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %48, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem76, align 8
  br label %50

50:                                               ; preds = %._crit_edge9, %45
  %.reload77 = load i64, ptr %.reg2mem76, align 8
  store i64 %.reload77, ptr %.reg2mem11, align 8
  %51 = load ptr, ptr @stderr, align 8, !tbaa !15
  %.reload14 = load i64, ptr %.reg2mem11, align 8
  %.reload66 = load ptr, ptr %.reg2mem65, align 8
  %52 = getelementptr inbounds double, ptr %.reload66, i64 %.reload14
  %53 = load double, ptr %52, align 8, !tbaa !5
  %54 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %51, ptr noundef nonnull @.str.3, double noundef %53) #4
  %.reload13 = load i64, ptr %.reg2mem11, align 8
  %55 = trunc i64 %.reload13 to i16
  %56 = urem i16 %55, 20
  %57 = icmp eq i16 %56, 0
  br i1 %57, label %58, label %._crit_edge8

._crit_edge8:                                     ; preds = %50
  br label %61

58:                                               ; preds = %50
  %59 = load ptr, ptr @stderr, align 8, !tbaa !15
  %60 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %59, ptr noundef nonnull @.str.4) #4
  br label %61

61:                                               ; preds = %._crit_edge8, %58
  %.reload12 = load i64, ptr %.reg2mem11, align 8
  %62 = add nuw nsw i64 %.reload12, 1
  store i64 %62, ptr %.reg2mem, align 8
  %.reload10 = load i64, ptr %.reg2mem, align 8
  %63 = icmp eq i64 %.reload10, 400
  br i1 %63, label %64, label %._crit_edge9, !llvm.loop !17

._crit_edge9:                                     ; preds = %61
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem76, align 8
  br label %50

64:                                               ; preds = %61
  %65 = load ptr, ptr @stderr, align 8, !tbaa !15
  %66 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %65, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %67 = load ptr, ptr @stderr, align 8, !tbaa !15
  %68 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.6) #4
  %.reload75 = load ptr, ptr %.reg2mem71, align 8
  tail call void @free(ptr noundef %.reload75) #4
  %.reload70 = load ptr, ptr %.reg2mem65, align 8
  tail call void @free(ptr noundef nonnull %.reload70) #4
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
!15 = !{!16, !16, i64 0}
!16 = !{!"any pointer", !7, i64 0}
!17 = distinct !{!17, !10, !11}

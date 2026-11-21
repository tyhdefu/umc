; ModuleID = './polybench-output/durbin-no-phis.bc'
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
  %.reg2mem = alloca i64, align 8
  %.reg2mem9 = alloca i64, align 8
  %.reg2mem13 = alloca i64, align 8
  %.reg2mem16 = alloca i64, align 8
  %.reg2mem19 = alloca i64, align 8
  %.reg2mem22 = alloca ptr, align 8
  %.reg2mem25 = alloca double, align 8
  %.reg2mem29 = alloca double, align 8
  %.reg2mem32 = alloca i64, align 8
  %.reg2mem35 = alloca double, align 8
  %.reg2mem38 = alloca ptr, align 8
  %.reg2mem41 = alloca double, align 8
  %.reg2mem43 = alloca double, align 8
  %.reg2mem46 = alloca double, align 8
  %.reg2mem48 = alloca i64, align 8
  %.reg2mem55 = alloca double, align 8
  %.reg2mem57 = alloca i64, align 8
  %.reg2mem60 = alloca ptr, align 8
  %.reg2mem68 = alloca ptr, align 8
  %.reg2mem74 = alloca i64, align 8
  %.reg2mem76 = alloca i64, align 8
  %.reg2mem78 = alloca i64, align 8
  %.reg2mem80 = alloca double, align 8
  %.reg2mem82 = alloca i64, align 8
  %.reg2mem84 = alloca double, align 8
  %.reg2mem86 = alloca double, align 8
  %.reg2mem88 = alloca i64, align 8
  %.reg2mem90 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #5
  store ptr %4, ptr %.reg2mem68, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #5
  store ptr %5, ptr %.reg2mem60, align 8
  store i64 0, ptr %.reg2mem90, align 8
  br label %6

6:                                                ; preds = %._crit_edge, %2
  %.reload91 = load i64, ptr %.reg2mem90, align 8
  %7 = trunc i64 %.reload91 to i32
  %8 = sub i32 401, %7
  %9 = uitofp nneg i32 %8 to double
  %.reload72 = load ptr, ptr %.reg2mem68, align 8
  %10 = getelementptr inbounds double, ptr %.reload72, i64 %.reload91
  store double %9, ptr %10, align 8, !tbaa !5
  %11 = add nuw nsw i64 %.reload91, 1
  store i64 %11, ptr %.reg2mem57, align 8
  %.reload59 = load i64, ptr %.reg2mem57, align 8
  %12 = icmp eq i64 %.reload59, 400
  br i1 %12, label %13, label %._crit_edge, !llvm.loop !9

._crit_edge:                                      ; preds = %6
  %.reload58 = load i64, ptr %.reg2mem57, align 8
  store i64 %.reload58, ptr %.reg2mem90, align 8
  br label %6

13:                                               ; preds = %6
  call void @llvm.lifetime.start.p0(i64 3200, ptr nonnull %3) #6
  %.reload71 = load ptr, ptr %.reg2mem68, align 8
  %14 = load double, ptr %.reload71, align 8, !tbaa !5
  %15 = fneg double %14
  %.reload66 = load ptr, ptr %.reg2mem60, align 8
  store double %15, ptr %.reload66, align 8, !tbaa !5
  %.reload70 = load ptr, ptr %.reg2mem68, align 8
  %16 = load double, ptr %.reload70, align 8, !tbaa !5
  %17 = fneg double %16
  store double %17, ptr %.reg2mem55, align 8
  %.reload56 = load double, ptr %.reg2mem55, align 8
  store double %.reload56, ptr %.reg2mem84, align 8
  store double 1.000000e+00, ptr %.reg2mem86, align 8
  store i64 1, ptr %.reg2mem88, align 8
  br label %18

18:                                               ; preds = %._crit_edge5, %13
  %.reload89 = load i64, ptr %.reg2mem88, align 8
  %.reload87 = load double, ptr %.reg2mem86, align 8
  %.reload85 = load double, ptr %.reg2mem84, align 8
  store i64 %.reload89, ptr %.reg2mem48, align 8
  store double %.reload87, ptr %.reg2mem46, align 8
  store double %.reload85, ptr %.reg2mem43, align 8
  %.reload45 = load double, ptr %.reg2mem43, align 8
  %19 = fneg double %.reload45
  store double %19, ptr %.reg2mem41, align 8
  %.reload54 = load i64, ptr %.reg2mem48, align 8
  %.reload69 = load ptr, ptr %.reg2mem68, align 8
  %20 = getelementptr double, ptr %.reload69, i64 %.reload54
  store ptr %20, ptr %.reg2mem38, align 8
  store double 0.000000e+00, ptr %.reg2mem80, align 8
  store i64 0, ptr %.reg2mem82, align 8
  br label %21

21:                                               ; preds = %._crit_edge1, %18
  %.reload83 = load i64, ptr %.reg2mem82, align 8
  %.reload81 = load double, ptr %.reg2mem80, align 8
  %22 = xor i64 %.reload83, -1
  %.reload40 = load ptr, ptr %.reg2mem38, align 8
  %23 = getelementptr double, ptr %.reload40, i64 %22
  %24 = load double, ptr %23, align 8, !tbaa !5
  %.reload65 = load ptr, ptr %.reg2mem60, align 8
  %25 = getelementptr inbounds double, ptr %.reload65, i64 %.reload83
  %26 = load double, ptr %25, align 8, !tbaa !5
  %27 = tail call double @llvm.fmuladd.f64(double %24, double %26, double %.reload81)
  store double %27, ptr %.reg2mem35, align 8
  %28 = add nuw nsw i64 %.reload83, 1
  store i64 %28, ptr %.reg2mem32, align 8
  %.reload34 = load i64, ptr %.reg2mem32, align 8
  %.reload53 = load i64, ptr %.reg2mem48, align 8
  %29 = icmp eq i64 %.reload34, %.reload53
  br i1 %29, label %30, label %._crit_edge1, !llvm.loop !12

._crit_edge1:                                     ; preds = %21
  %.reload33 = load i64, ptr %.reg2mem32, align 8
  %.reload37 = load double, ptr %.reg2mem35, align 8
  store double %.reload37, ptr %.reg2mem80, align 8
  store i64 %.reload33, ptr %.reg2mem82, align 8
  br label %21

30:                                               ; preds = %21
  %.reload42 = load double, ptr %.reg2mem41, align 8
  %.reload44 = load double, ptr %.reg2mem43, align 8
  %31 = tail call double @llvm.fmuladd.f64(double %.reload42, double %.reload44, double 1.000000e+00)
  %.reload47 = load double, ptr %.reg2mem46, align 8
  %32 = fmul double %.reload47, %31
  store double %32, ptr %.reg2mem29, align 8
  %.reload39 = load ptr, ptr %.reg2mem38, align 8
  %33 = load double, ptr %.reload39, align 8, !tbaa !5
  %.reload36 = load double, ptr %.reg2mem35, align 8
  %34 = fadd double %.reload36, %33
  %35 = fneg double %34
  %.reload31 = load double, ptr %.reg2mem29, align 8
  %36 = fdiv double %35, %.reload31
  store double %36, ptr %.reg2mem25, align 8
  %.reload52 = load i64, ptr %.reg2mem48, align 8
  %.reload64 = load ptr, ptr %.reg2mem60, align 8
  %37 = getelementptr double, ptr %.reload64, i64 %.reload52
  store ptr %37, ptr %.reg2mem22, align 8
  store i64 0, ptr %.reg2mem78, align 8
  br label %38

38:                                               ; preds = %._crit_edge3, %30
  %.reload79 = load i64, ptr %.reg2mem78, align 8
  %.reload63 = load ptr, ptr %.reg2mem60, align 8
  %39 = getelementptr inbounds double, ptr %.reload63, i64 %.reload79
  %40 = load double, ptr %39, align 8, !tbaa !5
  %41 = xor i64 %.reload79, -1
  %.reload24 = load ptr, ptr %.reg2mem22, align 8
  %42 = getelementptr double, ptr %.reload24, i64 %41
  %43 = load double, ptr %42, align 8, !tbaa !5
  %.reload28 = load double, ptr %.reg2mem25, align 8
  %44 = tail call double @llvm.fmuladd.f64(double %.reload28, double %43, double %40)
  %45 = getelementptr inbounds [400 x double], ptr %3, i64 0, i64 %.reload79
  store double %44, ptr %45, align 8, !tbaa !5
  %46 = add nuw nsw i64 %.reload79, 1
  store i64 %46, ptr %.reg2mem19, align 8
  %.reload21 = load i64, ptr %.reg2mem19, align 8
  %.reload51 = load i64, ptr %.reg2mem48, align 8
  %47 = icmp eq i64 %.reload21, %.reload51
  br i1 %47, label %._crit_edge2, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %38
  %.reload20 = load i64, ptr %.reg2mem19, align 8
  store i64 %.reload20, ptr %.reg2mem78, align 8
  br label %38

._crit_edge2:                                     ; preds = %38
  store i64 0, ptr %.reg2mem76, align 8
  br label %48

48:                                               ; preds = %._crit_edge4, %._crit_edge2
  %.reload77 = load i64, ptr %.reg2mem76, align 8
  %49 = getelementptr inbounds [400 x double], ptr %3, i64 0, i64 %.reload77
  %50 = load double, ptr %49, align 8, !tbaa !5
  %.reload62 = load ptr, ptr %.reg2mem60, align 8
  %51 = getelementptr inbounds double, ptr %.reload62, i64 %.reload77
  store double %50, ptr %51, align 8, !tbaa !5
  %52 = add nuw nsw i64 %.reload77, 1
  store i64 %52, ptr %.reg2mem16, align 8
  %.reload18 = load i64, ptr %.reg2mem16, align 8
  %.reload50 = load i64, ptr %.reg2mem48, align 8
  %53 = icmp eq i64 %.reload18, %.reload50
  br i1 %53, label %54, label %._crit_edge4, !llvm.loop !14

._crit_edge4:                                     ; preds = %48
  %.reload17 = load i64, ptr %.reg2mem16, align 8
  store i64 %.reload17, ptr %.reg2mem76, align 8
  br label %48

54:                                               ; preds = %48
  %.reload23 = load ptr, ptr %.reg2mem22, align 8
  %.reload27 = load double, ptr %.reg2mem25, align 8
  store double %.reload27, ptr %.reload23, align 8, !tbaa !5
  %.reload49 = load i64, ptr %.reg2mem48, align 8
  %55 = add nuw nsw i64 %.reload49, 1
  store i64 %55, ptr %.reg2mem13, align 8
  %.reload15 = load i64, ptr %.reg2mem13, align 8
  %56 = icmp eq i64 %.reload15, 400
  br i1 %56, label %57, label %._crit_edge5, !llvm.loop !15

._crit_edge5:                                     ; preds = %54
  %.reload14 = load i64, ptr %.reg2mem13, align 8
  %.reload26 = load double, ptr %.reg2mem25, align 8
  %.reload30 = load double, ptr %.reg2mem29, align 8
  store double %.reload26, ptr %.reg2mem84, align 8
  store double %.reload30, ptr %.reg2mem86, align 8
  store i64 %.reload14, ptr %.reg2mem88, align 8
  br label %18

57:                                               ; preds = %54
  call void @llvm.lifetime.end.p0(i64 3200, ptr nonnull %3) #6
  %58 = load ptr, ptr @stderr, align 8, !tbaa !16
  %59 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %58, ptr noundef nonnull @.str) #5
  %60 = load ptr, ptr @stderr, align 8, !tbaa !16
  %61 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %60, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #5
  store i64 0, ptr %.reg2mem74, align 8
  br label %62

62:                                               ; preds = %._crit_edge7, %57
  %.reload75 = load i64, ptr %.reg2mem74, align 8
  store i64 %.reload75, ptr %.reg2mem9, align 8
  %.reload12 = load i64, ptr %.reg2mem9, align 8
  %63 = trunc i64 %.reload12 to i16
  %64 = urem i16 %63, 20
  %65 = icmp eq i16 %64, 0
  br i1 %65, label %66, label %._crit_edge6

._crit_edge6:                                     ; preds = %62
  br label %69

66:                                               ; preds = %62
  %67 = load ptr, ptr @stderr, align 8, !tbaa !16
  %68 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %67, ptr noundef nonnull @.str.3) #5
  br label %69

69:                                               ; preds = %._crit_edge6, %66
  %70 = load ptr, ptr @stderr, align 8, !tbaa !16
  %.reload11 = load i64, ptr %.reg2mem9, align 8
  %.reload61 = load ptr, ptr %.reg2mem60, align 8
  %71 = getelementptr inbounds double, ptr %.reload61, i64 %.reload11
  %72 = load double, ptr %71, align 8, !tbaa !5
  %73 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %70, ptr noundef nonnull @.str.4, double noundef %72) #5
  %.reload10 = load i64, ptr %.reg2mem9, align 8
  %74 = add nuw nsw i64 %.reload10, 1
  store i64 %74, ptr %.reg2mem, align 8
  %.reload8 = load i64, ptr %.reg2mem, align 8
  %75 = icmp eq i64 %.reload8, 400
  br i1 %75, label %76, label %._crit_edge7, !llvm.loop !18

._crit_edge7:                                     ; preds = %69
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem74, align 8
  br label %62

76:                                               ; preds = %69
  %77 = load ptr, ptr @stderr, align 8, !tbaa !16
  %78 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %77, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #5
  %79 = load ptr, ptr @stderr, align 8, !tbaa !16
  %80 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %79, ptr noundef nonnull @.str.6) #5
  %.reload73 = load ptr, ptr %.reg2mem68, align 8
  tail call void @free(ptr noundef nonnull %.reload73) #5
  %.reload67 = load ptr, ptr %.reg2mem60, align 8
  tail call void @free(ptr noundef nonnull %.reload67) #5
  ret i32 0
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #2

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #4

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #2

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
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

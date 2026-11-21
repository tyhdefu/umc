; ModuleID = './polybench-output/mvt-no-phis.bc'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/kernels/mvt/mvt.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [3 x i8] c"x1\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [3 x i8] c"x2\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %.reg2mem = alloca i64, align 8
  %.reg2mem13 = alloca i64, align 8
  %.reg2mem17 = alloca i64, align 8
  %.reg2mem20 = alloca i64, align 8
  %.reg2mem24 = alloca i64, align 8
  %.reg2mem27 = alloca i64, align 8
  %.reg2mem30 = alloca double, align 8
  %.reg2mem33 = alloca double, align 8
  %.reg2mem35 = alloca ptr, align 8
  %.reg2mem38 = alloca i64, align 8
  %.reg2mem42 = alloca i64, align 8
  %.reg2mem45 = alloca i64, align 8
  %.reg2mem48 = alloca double, align 8
  %.reg2mem51 = alloca double, align 8
  %.reg2mem53 = alloca ptr, align 8
  %.reg2mem56 = alloca i64, align 8
  %.reg2mem60 = alloca i64, align 8
  %.reg2mem63 = alloca i1, align 1
  %.reg2mem66 = alloca i64, align 8
  %.reg2mem70 = alloca i64, align 8
  %.reg2mem81 = alloca ptr, align 8
  %.reg2mem85 = alloca ptr, align 8
  %.reg2mem89 = alloca ptr, align 8
  %.reg2mem94 = alloca ptr, align 8
  %.reg2mem99 = alloca ptr, align 8
  %.reg2mem104 = alloca i64, align 8
  %.reg2mem106 = alloca i64, align 8
  %.reg2mem108 = alloca double, align 8
  %.reg2mem110 = alloca i64, align 8
  %.reg2mem112 = alloca i64, align 8
  %.reg2mem114 = alloca double, align 8
  %.reg2mem116 = alloca i64, align 8
  %.reg2mem118 = alloca i64, align 8
  %.reg2mem120 = alloca i64, align 8
  %.reg2mem122 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem99, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem94, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem89, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem85, align 8
  %7 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %7, ptr %.reg2mem81, align 8
  store i64 0, ptr %.reg2mem122, align 8
  br label %9

8:                                                ; preds = %33
  %.reload64 = load i1, ptr %.reg2mem63, align 1
  br i1 %.reload64, label %._crit_edge, label %._crit_edge1, !llvm.loop !5

._crit_edge1:                                     ; preds = %8
  %.reload67 = load i64, ptr %.reg2mem66, align 8
  store i64 %.reload67, ptr %.reg2mem122, align 8
  br label %9

._crit_edge:                                      ; preds = %8
  store i64 0, ptr %.reg2mem118, align 8
  br label %42

9:                                                ; preds = %._crit_edge1, %2
  %.reload123 = load i64, ptr %.reg2mem122, align 8
  store i64 %.reload123, ptr %.reg2mem70, align 8
  %.reload80 = load i64, ptr %.reg2mem70, align 8
  %10 = trunc nuw nsw i64 %.reload80 to i32
  %11 = uitofp nneg i32 %10 to double
  %12 = fdiv double %11, 4.000000e+02
  %.reload79 = load i64, ptr %.reg2mem70, align 8
  %.reload97 = load ptr, ptr %.reg2mem94, align 8
  %13 = getelementptr inbounds double, ptr %.reload97, i64 %.reload79
  store double %12, ptr %13, align 8, !tbaa !8
  %.reload78 = load i64, ptr %.reg2mem70, align 8
  %14 = add nuw nsw i64 %.reload78, 1
  store i64 %14, ptr %.reg2mem66, align 8
  %.reload69 = load i64, ptr %.reg2mem66, align 8
  %15 = icmp eq i64 %.reload69, 400
  store i1 %15, ptr %.reg2mem63, align 1
  %.reload68 = load i64, ptr %.reg2mem66, align 8
  %16 = trunc nuw nsw i64 %.reload68 to i32
  %17 = uitofp nneg i32 %16 to double
  %18 = fdiv double %17, 4.000000e+02
  %.reload65 = load i1, ptr %.reg2mem63, align 1
  %19 = select i1 %.reload65, double 0.000000e+00, double %18
  %.reload77 = load i64, ptr %.reg2mem70, align 8
  %.reload92 = load ptr, ptr %.reg2mem89, align 8
  %20 = getelementptr inbounds double, ptr %.reload92, i64 %.reload77
  store double %19, ptr %20, align 8, !tbaa !8
  %.reload76 = load i64, ptr %.reg2mem70, align 8
  %21 = icmp ult i64 %.reload76, 397
  %22 = select i1 %21, i32 3, i32 -397
  %23 = add nsw i32 %22, %10
  %24 = uitofp nneg i32 %23 to double
  %25 = fdiv double %24, 4.000000e+02
  %.reload75 = load i64, ptr %.reg2mem70, align 8
  %.reload87 = load ptr, ptr %.reg2mem85, align 8
  %26 = getelementptr inbounds double, ptr %.reload87, i64 %.reload75
  store double %25, ptr %26, align 8, !tbaa !8
  %.reload74 = load i64, ptr %.reg2mem70, align 8
  %27 = icmp ult i64 %.reload74, 396
  %28 = select i1 %27, i32 4, i32 -396
  %29 = add nsw i32 %28, %10
  %30 = uitofp nneg i32 %29 to double
  %31 = fdiv double %30, 4.000000e+02
  %.reload73 = load i64, ptr %.reg2mem70, align 8
  %.reload83 = load ptr, ptr %.reg2mem81, align 8
  %32 = getelementptr inbounds double, ptr %.reload83, i64 %.reload73
  store double %31, ptr %32, align 8, !tbaa !8
  store i64 0, ptr %.reg2mem120, align 8
  br label %33

33:                                               ; preds = %._crit_edge2, %9
  %.reload121 = load i64, ptr %.reg2mem120, align 8
  %.reload72 = load i64, ptr %.reg2mem70, align 8
  %34 = mul nuw nsw i64 %.reload121, %.reload72
  %35 = trunc nuw nsw i64 %34 to i32
  %36 = urem i32 %35, 400
  %37 = uitofp nneg i32 %36 to double
  %38 = fdiv double %37, 4.000000e+02
  %.reload71 = load i64, ptr %.reg2mem70, align 8
  %.reload102 = load ptr, ptr %.reg2mem99, align 8
  %39 = getelementptr inbounds [400 x double], ptr %.reload102, i64 %.reload71, i64 %.reload121
  store double %38, ptr %39, align 8, !tbaa !8
  %40 = add nuw nsw i64 %.reload121, 1
  store i64 %40, ptr %.reg2mem60, align 8
  %.reload62 = load i64, ptr %.reg2mem60, align 8
  %41 = icmp eq i64 %.reload62, 400
  br i1 %41, label %8, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %33
  %.reload61 = load i64, ptr %.reg2mem60, align 8
  store i64 %.reload61, ptr %.reg2mem120, align 8
  br label %33

42:                                               ; preds = %._crit_edge5, %._crit_edge
  %.reload119 = load i64, ptr %.reg2mem118, align 8
  store i64 %.reload119, ptr %.reg2mem56, align 8
  %.reload59 = load i64, ptr %.reg2mem56, align 8
  %.reload96 = load ptr, ptr %.reg2mem94, align 8
  %43 = getelementptr inbounds double, ptr %.reload96, i64 %.reload59
  store ptr %43, ptr %.reg2mem53, align 8
  %.reload55 = load ptr, ptr %.reg2mem53, align 8
  %44 = load double, ptr %.reload55, align 8, !tbaa !8
  store double %44, ptr %.reg2mem51, align 8
  %.reload52 = load double, ptr %.reg2mem51, align 8
  store double %.reload52, ptr %.reg2mem114, align 8
  store i64 0, ptr %.reg2mem116, align 8
  br label %45

45:                                               ; preds = %._crit_edge3, %42
  %.reload117 = load i64, ptr %.reg2mem116, align 8
  %.reload115 = load double, ptr %.reg2mem114, align 8
  %.reload58 = load i64, ptr %.reg2mem56, align 8
  %.reload101 = load ptr, ptr %.reg2mem99, align 8
  %46 = getelementptr inbounds [400 x double], ptr %.reload101, i64 %.reload58, i64 %.reload117
  %47 = load double, ptr %46, align 8, !tbaa !8
  %.reload86 = load ptr, ptr %.reg2mem85, align 8
  %48 = getelementptr inbounds double, ptr %.reload86, i64 %.reload117
  %49 = load double, ptr %48, align 8, !tbaa !8
  %50 = tail call double @llvm.fmuladd.f64(double %47, double %49, double %.reload115)
  store double %50, ptr %.reg2mem48, align 8
  %.reload50 = load double, ptr %.reg2mem48, align 8
  %.reload54 = load ptr, ptr %.reg2mem53, align 8
  store double %.reload50, ptr %.reload54, align 8, !tbaa !8
  %51 = add nuw nsw i64 %.reload117, 1
  store i64 %51, ptr %.reg2mem45, align 8
  %.reload47 = load i64, ptr %.reg2mem45, align 8
  %52 = icmp eq i64 %.reload47, 400
  br i1 %52, label %53, label %._crit_edge3, !llvm.loop !13

._crit_edge3:                                     ; preds = %45
  %.reload46 = load i64, ptr %.reg2mem45, align 8
  %.reload49 = load double, ptr %.reg2mem48, align 8
  store double %.reload49, ptr %.reg2mem114, align 8
  store i64 %.reload46, ptr %.reg2mem116, align 8
  br label %45

53:                                               ; preds = %45
  %.reload57 = load i64, ptr %.reg2mem56, align 8
  %54 = add nuw nsw i64 %.reload57, 1
  store i64 %54, ptr %.reg2mem42, align 8
  %.reload44 = load i64, ptr %.reg2mem42, align 8
  %55 = icmp eq i64 %.reload44, 400
  br i1 %55, label %._crit_edge4, label %._crit_edge5, !llvm.loop !14

._crit_edge5:                                     ; preds = %53
  %.reload43 = load i64, ptr %.reg2mem42, align 8
  store i64 %.reload43, ptr %.reg2mem118, align 8
  br label %42

._crit_edge4:                                     ; preds = %53
  store i64 0, ptr %.reg2mem112, align 8
  br label %56

56:                                               ; preds = %._crit_edge7, %._crit_edge4
  %.reload113 = load i64, ptr %.reg2mem112, align 8
  store i64 %.reload113, ptr %.reg2mem38, align 8
  %.reload41 = load i64, ptr %.reg2mem38, align 8
  %.reload91 = load ptr, ptr %.reg2mem89, align 8
  %57 = getelementptr inbounds double, ptr %.reload91, i64 %.reload41
  store ptr %57, ptr %.reg2mem35, align 8
  %.reload37 = load ptr, ptr %.reg2mem35, align 8
  %58 = load double, ptr %.reload37, align 8, !tbaa !8
  store double %58, ptr %.reg2mem33, align 8
  %.reload34 = load double, ptr %.reg2mem33, align 8
  store double %.reload34, ptr %.reg2mem108, align 8
  store i64 0, ptr %.reg2mem110, align 8
  br label %59

59:                                               ; preds = %._crit_edge6, %56
  %.reload111 = load i64, ptr %.reg2mem110, align 8
  %.reload109 = load double, ptr %.reg2mem108, align 8
  %.reload40 = load i64, ptr %.reg2mem38, align 8
  %.reload100 = load ptr, ptr %.reg2mem99, align 8
  %60 = getelementptr inbounds [400 x double], ptr %.reload100, i64 %.reload111, i64 %.reload40
  %61 = load double, ptr %60, align 8, !tbaa !8
  %.reload82 = load ptr, ptr %.reg2mem81, align 8
  %62 = getelementptr inbounds double, ptr %.reload82, i64 %.reload111
  %63 = load double, ptr %62, align 8, !tbaa !8
  %64 = tail call double @llvm.fmuladd.f64(double %61, double %63, double %.reload109)
  store double %64, ptr %.reg2mem30, align 8
  %.reload32 = load double, ptr %.reg2mem30, align 8
  %.reload36 = load ptr, ptr %.reg2mem35, align 8
  store double %.reload32, ptr %.reload36, align 8, !tbaa !8
  %65 = add nuw nsw i64 %.reload111, 1
  store i64 %65, ptr %.reg2mem27, align 8
  %.reload29 = load i64, ptr %.reg2mem27, align 8
  %66 = icmp eq i64 %.reload29, 400
  br i1 %66, label %67, label %._crit_edge6, !llvm.loop !15

._crit_edge6:                                     ; preds = %59
  %.reload28 = load i64, ptr %.reg2mem27, align 8
  %.reload31 = load double, ptr %.reg2mem30, align 8
  store double %.reload31, ptr %.reg2mem108, align 8
  store i64 %.reload28, ptr %.reg2mem110, align 8
  br label %59

67:                                               ; preds = %59
  %.reload39 = load i64, ptr %.reg2mem38, align 8
  %68 = add nuw nsw i64 %.reload39, 1
  store i64 %68, ptr %.reg2mem24, align 8
  %.reload26 = load i64, ptr %.reg2mem24, align 8
  %69 = icmp eq i64 %.reload26, 400
  br i1 %69, label %70, label %._crit_edge7, !llvm.loop !16

._crit_edge7:                                     ; preds = %67
  %.reload25 = load i64, ptr %.reg2mem24, align 8
  store i64 %.reload25, ptr %.reg2mem112, align 8
  br label %56

70:                                               ; preds = %67
  %71 = load ptr, ptr @stderr, align 8, !tbaa !17
  %72 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %71, ptr noundef nonnull @.str) #4
  %73 = load ptr, ptr @stderr, align 8, !tbaa !17
  %74 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %73, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem106, align 8
  br label %75

75:                                               ; preds = %._crit_edge9, %70
  %.reload107 = load i64, ptr %.reg2mem106, align 8
  store i64 %.reload107, ptr %.reg2mem20, align 8
  %.reload23 = load i64, ptr %.reg2mem20, align 8
  %76 = trunc i64 %.reload23 to i16
  %77 = urem i16 %76, 20
  %78 = icmp eq i16 %77, 0
  br i1 %78, label %79, label %._crit_edge8

._crit_edge8:                                     ; preds = %75
  br label %82

79:                                               ; preds = %75
  %80 = load ptr, ptr @stderr, align 8, !tbaa !17
  %81 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %80, ptr noundef nonnull @.str.3) #4
  br label %82

82:                                               ; preds = %._crit_edge8, %79
  %83 = load ptr, ptr @stderr, align 8, !tbaa !17
  %.reload22 = load i64, ptr %.reg2mem20, align 8
  %.reload95 = load ptr, ptr %.reg2mem94, align 8
  %84 = getelementptr inbounds double, ptr %.reload95, i64 %.reload22
  %85 = load double, ptr %84, align 8, !tbaa !8
  %86 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %83, ptr noundef nonnull @.str.4, double noundef %85) #4
  %.reload21 = load i64, ptr %.reg2mem20, align 8
  %87 = add nuw nsw i64 %.reload21, 1
  store i64 %87, ptr %.reg2mem17, align 8
  %.reload19 = load i64, ptr %.reg2mem17, align 8
  %88 = icmp eq i64 %.reload19, 400
  br i1 %88, label %89, label %._crit_edge9, !llvm.loop !19

._crit_edge9:                                     ; preds = %82
  %.reload18 = load i64, ptr %.reg2mem17, align 8
  store i64 %.reload18, ptr %.reg2mem106, align 8
  br label %75

89:                                               ; preds = %82
  %90 = load ptr, ptr @stderr, align 8, !tbaa !17
  %91 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %90, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %92 = load ptr, ptr @stderr, align 8, !tbaa !17
  %93 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %92, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.6) #4
  store i64 0, ptr %.reg2mem104, align 8
  br label %94

94:                                               ; preds = %._crit_edge11, %89
  %.reload105 = load i64, ptr %.reg2mem104, align 8
  store i64 %.reload105, ptr %.reg2mem13, align 8
  %.reload16 = load i64, ptr %.reg2mem13, align 8
  %95 = trunc i64 %.reload16 to i16
  %96 = urem i16 %95, 20
  %97 = icmp eq i16 %96, 0
  br i1 %97, label %98, label %._crit_edge10

._crit_edge10:                                    ; preds = %94
  br label %101

98:                                               ; preds = %94
  %99 = load ptr, ptr @stderr, align 8, !tbaa !17
  %100 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %99, ptr noundef nonnull @.str.3) #4
  br label %101

101:                                              ; preds = %._crit_edge10, %98
  %102 = load ptr, ptr @stderr, align 8, !tbaa !17
  %.reload15 = load i64, ptr %.reg2mem13, align 8
  %.reload90 = load ptr, ptr %.reg2mem89, align 8
  %103 = getelementptr inbounds double, ptr %.reload90, i64 %.reload15
  %104 = load double, ptr %103, align 8, !tbaa !8
  %105 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %102, ptr noundef nonnull @.str.4, double noundef %104) #4
  %.reload14 = load i64, ptr %.reg2mem13, align 8
  %106 = add nuw nsw i64 %.reload14, 1
  store i64 %106, ptr %.reg2mem, align 8
  %.reload12 = load i64, ptr %.reg2mem, align 8
  %107 = icmp eq i64 %.reload12, 400
  br i1 %107, label %108, label %._crit_edge11, !llvm.loop !20

._crit_edge11:                                    ; preds = %101
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem104, align 8
  br label %94

108:                                              ; preds = %101
  %109 = load ptr, ptr @stderr, align 8, !tbaa !17
  %110 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %109, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.6) #4
  %111 = load ptr, ptr @stderr, align 8, !tbaa !17
  %112 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %111, ptr noundef nonnull @.str.7) #4
  %.reload103 = load ptr, ptr %.reg2mem99, align 8
  tail call void @free(ptr noundef %.reload103) #4
  %.reload98 = load ptr, ptr %.reg2mem94, align 8
  tail call void @free(ptr noundef %.reload98) #4
  %.reload93 = load ptr, ptr %.reg2mem89, align 8
  tail call void @free(ptr noundef nonnull %.reload93) #4
  %.reload88 = load ptr, ptr %.reg2mem85, align 8
  tail call void @free(ptr noundef %.reload88) #4
  %.reload84 = load ptr, ptr %.reg2mem81, align 8
  tail call void @free(ptr noundef %.reload84) #4
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
!5 = distinct !{!5, !6, !7}
!6 = !{!"llvm.loop.mustprogress"}
!7 = !{!"llvm.loop.unroll.disable"}
!8 = !{!9, !9, i64 0}
!9 = !{!"double", !10, i64 0}
!10 = !{!"omnipotent char", !11, i64 0}
!11 = !{!"Simple C/C++ TBAA"}
!12 = distinct !{!12, !6, !7}
!13 = distinct !{!13, !6, !7}
!14 = distinct !{!14, !6, !7}
!15 = distinct !{!15, !6, !7}
!16 = distinct !{!16, !6, !7}
!17 = !{!18, !18, i64 0}
!18 = !{!"any pointer", !10, i64 0}
!19 = distinct !{!19, !6, !7}
!20 = distinct !{!20, !6, !7}

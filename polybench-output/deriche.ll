; ModuleID = '/home/lepton/PolyBenchC-4.2.1/medley/deriche/deriche.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/medley/deriche/deriche.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [7 x i8] c"imgOut\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"%0.2f \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 345600, i32 noundef 4) #4
  br label %7

7:                                                ; preds = %21, %2
  %8 = phi i64 [ 0, %2 ], [ %22, %21 ]
  %9 = mul nuw nsw i64 %8, 313
  br label %10

10:                                               ; preds = %10, %7
  %11 = phi i64 [ 0, %7 ], [ %19, %10 ]
  %12 = mul nuw nsw i64 %11, 991
  %13 = add nuw nsw i64 %12, %9
  %14 = trunc nuw nsw i64 %13 to i32
  %15 = and i32 %14, 65535
  %16 = uitofp nneg i32 %15 to float
  %17 = fdiv float %16, 6.553500e+04
  %18 = getelementptr inbounds [480 x float], ptr %3, i64 %8, i64 %11
  store float %17, ptr %18, align 4, !tbaa !5
  %19 = add nuw nsw i64 %11, 1
  %20 = icmp eq i64 %19, 480
  br i1 %20, label %21, label %10, !llvm.loop !9

21:                                               ; preds = %10
  %22 = add nuw nsw i64 %8, 1
  %23 = icmp eq i64 %22, 720
  br i1 %23, label %24, label %7, !llvm.loop !12

24:                                               ; preds = %21
  %25 = tail call float @expf(float noundef -2.500000e-01) #4
  %26 = fsub float 1.000000e+00, %25
  %27 = tail call float @expf(float noundef -2.500000e-01) #4
  %28 = fsub float 1.000000e+00, %27
  %29 = fmul float %26, %28
  %30 = tail call float @expf(float noundef -2.500000e-01) #4
  %31 = tail call float @llvm.fmuladd.f32(float %30, float 5.000000e-01, float 1.000000e+00)
  %32 = tail call float @expf(float noundef 5.000000e-01) #4
  %33 = fsub float %31, %32
  %34 = fdiv float %29, %33
  %35 = tail call float @expf(float noundef -2.500000e-01) #4
  %36 = fmul float %35, %34
  %37 = fmul float %36, -7.500000e-01
  %38 = tail call float @expf(float noundef -2.500000e-01) #4
  %39 = fmul float %34, %38
  %40 = tail call float @expf(float noundef -5.000000e-01) #4
  %41 = tail call float @powf(float noundef 2.000000e+00, float noundef -2.500000e-01) #4
  %42 = tail call float @expf(float noundef -5.000000e-01) #4
  %43 = fneg float %42
  br label %44

44:                                               ; preds = %61, %24
  %45 = phi i64 [ 0, %24 ], [ %62, %61 ]
  br label %46

46:                                               ; preds = %46, %44
  %47 = phi i64 [ 0, %44 ], [ %59, %46 ]
  %48 = phi float [ 0.000000e+00, %44 ], [ %58, %46 ]
  %49 = phi float [ 0.000000e+00, %44 ], [ %56, %46 ]
  %50 = phi float [ 0.000000e+00, %44 ], [ %49, %46 ]
  %51 = getelementptr inbounds [480 x float], ptr %3, i64 %45, i64 %47
  %52 = load float, ptr %51, align 4, !tbaa !5
  %53 = fmul float %37, %48
  %54 = tail call float @llvm.fmuladd.f32(float %34, float %52, float %53)
  %55 = tail call float @llvm.fmuladd.f32(float %41, float %49, float %54)
  %56 = tail call float @llvm.fmuladd.f32(float %43, float %50, float %55)
  %57 = getelementptr inbounds [480 x float], ptr %5, i64 %45, i64 %47
  store float %56, ptr %57, align 4, !tbaa !5
  %58 = load float, ptr %51, align 4, !tbaa !5
  %59 = add nuw nsw i64 %47, 1
  %60 = icmp eq i64 %59, 480
  br i1 %60, label %61, label %46, !llvm.loop !13

61:                                               ; preds = %46
  %62 = add nuw nsw i64 %45, 1
  %63 = icmp eq i64 %62, 720
  br i1 %63, label %64, label %44, !llvm.loop !14

64:                                               ; preds = %61
  %65 = fmul float %39, 1.250000e+00
  %66 = fneg float %34
  %67 = fmul float %40, %66
  br label %68

68:                                               ; preds = %85, %64
  %69 = phi i64 [ 0, %64 ], [ %86, %85 ]
  br label %70

70:                                               ; preds = %70, %68
  %71 = phi i64 [ 479, %68 ], [ %83, %70 ]
  %72 = phi float [ 0.000000e+00, %68 ], [ %82, %70 ]
  %73 = phi float [ 0.000000e+00, %68 ], [ %72, %70 ]
  %74 = phi float [ 0.000000e+00, %68 ], [ %79, %70 ]
  %75 = phi float [ 0.000000e+00, %68 ], [ %74, %70 ]
  %76 = fmul float %67, %73
  %77 = tail call float @llvm.fmuladd.f32(float %65, float %72, float %76)
  %78 = tail call float @llvm.fmuladd.f32(float %41, float %74, float %77)
  %79 = tail call float @llvm.fmuladd.f32(float %43, float %75, float %78)
  %80 = getelementptr inbounds [480 x float], ptr %6, i64 %69, i64 %71
  store float %79, ptr %80, align 4, !tbaa !5
  %81 = getelementptr inbounds [480 x float], ptr %3, i64 %69, i64 %71
  %82 = load float, ptr %81, align 4, !tbaa !5
  %83 = add nsw i64 %71, -1
  %84 = icmp eq i64 %71, 0
  br i1 %84, label %85, label %70, !llvm.loop !15

85:                                               ; preds = %70
  %86 = add nuw nsw i64 %69, 1
  %87 = icmp eq i64 %86, 720
  br i1 %87, label %88, label %68, !llvm.loop !16

88:                                               ; preds = %85, %100
  %89 = phi i64 [ %101, %100 ], [ 0, %85 ]
  br label %90

90:                                               ; preds = %90, %88
  %91 = phi i64 [ 0, %88 ], [ %98, %90 ]
  %92 = getelementptr inbounds [480 x float], ptr %5, i64 %89, i64 %91
  %93 = load float, ptr %92, align 4, !tbaa !5
  %94 = getelementptr inbounds [480 x float], ptr %6, i64 %89, i64 %91
  %95 = load float, ptr %94, align 4, !tbaa !5
  %96 = fadd float %93, %95
  %97 = getelementptr inbounds [480 x float], ptr %4, i64 %89, i64 %91
  store float %96, ptr %97, align 4, !tbaa !5
  %98 = add nuw nsw i64 %91, 1
  %99 = icmp eq i64 %98, 480
  br i1 %99, label %100, label %90, !llvm.loop !17

100:                                              ; preds = %90
  %101 = add nuw nsw i64 %89, 1
  %102 = icmp eq i64 %101, 720
  br i1 %102, label %103, label %88, !llvm.loop !18

103:                                              ; preds = %100, %120
  %104 = phi i64 [ %121, %120 ], [ 0, %100 ]
  br label %105

105:                                              ; preds = %105, %103
  %106 = phi i64 [ 0, %103 ], [ %118, %105 ]
  %107 = phi float [ 0.000000e+00, %103 ], [ %117, %105 ]
  %108 = phi float [ 0.000000e+00, %103 ], [ %115, %105 ]
  %109 = phi float [ 0.000000e+00, %103 ], [ %108, %105 ]
  %110 = getelementptr inbounds [480 x float], ptr %4, i64 %106, i64 %104
  %111 = load float, ptr %110, align 4, !tbaa !5
  %112 = fmul float %37, %107
  %113 = tail call float @llvm.fmuladd.f32(float %34, float %111, float %112)
  %114 = tail call float @llvm.fmuladd.f32(float %41, float %108, float %113)
  %115 = tail call float @llvm.fmuladd.f32(float %43, float %109, float %114)
  %116 = getelementptr inbounds [480 x float], ptr %5, i64 %106, i64 %104
  store float %115, ptr %116, align 4, !tbaa !5
  %117 = load float, ptr %110, align 4, !tbaa !5
  %118 = add nuw nsw i64 %106, 1
  %119 = icmp eq i64 %118, 720
  br i1 %119, label %120, label %105, !llvm.loop !19

120:                                              ; preds = %105
  %121 = add nuw nsw i64 %104, 1
  %122 = icmp eq i64 %121, 480
  br i1 %122, label %123, label %103, !llvm.loop !20

123:                                              ; preds = %120, %140
  %124 = phi i64 [ %141, %140 ], [ 0, %120 ]
  br label %125

125:                                              ; preds = %125, %123
  %126 = phi i64 [ 719, %123 ], [ %138, %125 ]
  %127 = phi float [ 0.000000e+00, %123 ], [ %137, %125 ]
  %128 = phi float [ 0.000000e+00, %123 ], [ %127, %125 ]
  %129 = phi float [ 0.000000e+00, %123 ], [ %134, %125 ]
  %130 = phi float [ 0.000000e+00, %123 ], [ %129, %125 ]
  %131 = fmul float %67, %128
  %132 = tail call float @llvm.fmuladd.f32(float %65, float %127, float %131)
  %133 = tail call float @llvm.fmuladd.f32(float %41, float %129, float %132)
  %134 = tail call float @llvm.fmuladd.f32(float %43, float %130, float %133)
  %135 = getelementptr inbounds [480 x float], ptr %6, i64 %126, i64 %124
  store float %134, ptr %135, align 4, !tbaa !5
  %136 = getelementptr inbounds [480 x float], ptr %4, i64 %126, i64 %124
  %137 = load float, ptr %136, align 4, !tbaa !5
  %138 = add nsw i64 %126, -1
  %139 = icmp eq i64 %126, 0
  br i1 %139, label %140, label %125, !llvm.loop !21

140:                                              ; preds = %125
  %141 = add nuw nsw i64 %124, 1
  %142 = icmp eq i64 %141, 480
  br i1 %142, label %143, label %123, !llvm.loop !22

143:                                              ; preds = %140, %155
  %144 = phi i64 [ %156, %155 ], [ 0, %140 ]
  br label %145

145:                                              ; preds = %145, %143
  %146 = phi i64 [ 0, %143 ], [ %153, %145 ]
  %147 = getelementptr inbounds [480 x float], ptr %5, i64 %144, i64 %146
  %148 = load float, ptr %147, align 4, !tbaa !5
  %149 = getelementptr inbounds [480 x float], ptr %6, i64 %144, i64 %146
  %150 = load float, ptr %149, align 4, !tbaa !5
  %151 = fadd float %148, %150
  %152 = getelementptr inbounds [480 x float], ptr %4, i64 %144, i64 %146
  store float %151, ptr %152, align 4, !tbaa !5
  %153 = add nuw nsw i64 %146, 1
  %154 = icmp eq i64 %153, 480
  br i1 %154, label %155, label %145, !llvm.loop !23

155:                                              ; preds = %145
  %156 = add nuw nsw i64 %144, 1
  %157 = icmp eq i64 %156, 720
  br i1 %157, label %158, label %143, !llvm.loop !24

158:                                              ; preds = %155
  %159 = load ptr, ptr @stderr, align 8, !tbaa !25
  %160 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %159, ptr noundef nonnull @.str) #4
  %161 = load ptr, ptr @stderr, align 8, !tbaa !25
  %162 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %161, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %163

163:                                              ; preds = %183, %158
  %164 = phi i64 [ 0, %158 ], [ %184, %183 ]
  %165 = mul nuw nsw i64 %164, 480
  br label %166

166:                                              ; preds = %175, %163
  %167 = phi i64 [ 0, %163 ], [ %181, %175 ]
  %168 = add nuw nsw i64 %167, %165
  %169 = trunc nuw nsw i64 %168 to i32
  %170 = urem i32 %169, 20
  %171 = icmp eq i32 %170, 0
  br i1 %171, label %172, label %175

172:                                              ; preds = %166
  %173 = load ptr, ptr @stderr, align 8, !tbaa !25
  %174 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %173, ptr noundef nonnull @.str.3) #4
  br label %175

175:                                              ; preds = %172, %166
  %176 = load ptr, ptr @stderr, align 8, !tbaa !25
  %177 = getelementptr inbounds [480 x float], ptr %4, i64 %164, i64 %167
  %178 = load float, ptr %177, align 4, !tbaa !5
  %179 = fpext float %178 to double
  %180 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %176, ptr noundef nonnull @.str.4, double noundef %179) #4
  %181 = add nuw nsw i64 %167, 1
  %182 = icmp eq i64 %181, 480
  br i1 %182, label %183, label %166, !llvm.loop !27

183:                                              ; preds = %175
  %184 = add nuw nsw i64 %164, 1
  %185 = icmp eq i64 %184, 720
  br i1 %185, label %186, label %163, !llvm.loop !28

186:                                              ; preds = %183
  %187 = load ptr, ptr @stderr, align 8, !tbaa !25
  %188 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %187, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %189 = load ptr, ptr @stderr, align 8, !tbaa !25
  %190 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %189, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef nonnull %4) #4
  tail call void @free(ptr noundef %5) #4
  tail call void @free(ptr noundef %6) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: nounwind
declare float @expf(float noundef) local_unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fmuladd.f32(float, float, float) #3

; Function Attrs: nounwind
declare float @powf(float noundef, float noundef) local_unnamed_addr #2

declare i32 @fprintf(ptr noundef, ptr noundef, ...) local_unnamed_addr #1

attributes #0 = { nounwind uwtable "min-legal-vector-width"="0" "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "no-builtins" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nobuiltin nounwind "no-builtins" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"Ubuntu clang version 19.1.7 (20ubuntu4)"}
!5 = !{!6, !6, i64 0}
!6 = !{!"float", !7, i64 0}
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
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = !{!26, !26, i64 0}
!26 = !{!"any pointer", !7, i64 0}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}

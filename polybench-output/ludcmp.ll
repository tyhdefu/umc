; ModuleID = '/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/ludcmp/ludcmp.c'
source_filename = "/home/lepton/PolyBenchC-4.2.1/linear-algebra/solvers/ludcmp/ludcmp.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@stderr = external local_unnamed_addr global ptr, align 8
@.str = private unnamed_addr constant [23 x i8] c"==BEGIN DUMP_ARRAYS==\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"begin dump: %s\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"x\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"%0.2lf \00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"\0Aend   dump: %s\0A\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"==END   DUMP_ARRAYS==\0A\00", align 1

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @main(i32 noundef %0, ptr nocapture noundef readnone %1) local_unnamed_addr #0 {
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  br label %7

7:                                                ; preds = %7, %2
  %8 = phi i64 [ 0, %2 ], [ %11, %7 ]
  %9 = getelementptr inbounds double, ptr %5, i64 %8
  store double 0.000000e+00, ptr %9, align 8, !tbaa !5
  %10 = getelementptr inbounds double, ptr %6, i64 %8
  store double 0.000000e+00, ptr %10, align 8, !tbaa !5
  %11 = add nuw nsw i64 %8, 1
  %12 = trunc nuw nsw i64 %11 to i32
  %13 = uitofp nneg i32 %12 to double
  %14 = fdiv double %13, 4.000000e+02
  %15 = fmul double %14, 5.000000e-01
  %16 = fadd double %15, 4.000000e+00
  %17 = getelementptr inbounds double, ptr %4, i64 %8
  store double %16, ptr %17, align 8, !tbaa !5
  %18 = icmp eq i64 %11, 400
  br i1 %18, label %19, label %7, !llvm.loop !9

19:                                               ; preds = %7, %40
  %20 = phi i64 [ %33, %40 ], [ 0, %7 ]
  %21 = phi i64 [ %42, %40 ], [ 1, %7 ]
  br label %22

22:                                               ; preds = %22, %19
  %23 = phi i64 [ 0, %19 ], [ %30, %22 ]
  %24 = trunc i64 %23 to i32
  %25 = sub i32 0, %24
  %26 = sitofp i32 %25 to double
  %27 = fdiv double %26, 4.000000e+02
  %28 = fadd double %27, 1.000000e+00
  %29 = getelementptr inbounds [400 x double], ptr %3, i64 %20, i64 %23
  store double %28, ptr %29, align 8, !tbaa !5
  %30 = add nuw nsw i64 %23, 1
  %31 = icmp eq i64 %30, %21
  br i1 %31, label %32, label %22, !llvm.loop !12

32:                                               ; preds = %22
  %33 = add nuw nsw i64 %20, 1
  %34 = icmp ult i64 %20, 399
  br i1 %34, label %35, label %40

35:                                               ; preds = %32, %35
  %36 = phi i64 [ %38, %35 ], [ %21, %32 ]
  %37 = getelementptr inbounds [400 x double], ptr %3, i64 %20, i64 %36
  store double 0.000000e+00, ptr %37, align 8, !tbaa !5
  %38 = add nuw nsw i64 %36, 1
  %39 = icmp eq i64 %38, 400
  br i1 %39, label %40, label %35, !llvm.loop !13

40:                                               ; preds = %35, %32
  %41 = getelementptr inbounds [400 x double], ptr %3, i64 %20, i64 %20
  store double 1.000000e+00, ptr %41, align 8, !tbaa !5
  %42 = add nuw nsw i64 %21, 1
  %43 = icmp eq i64 %33, 400
  br i1 %43, label %44, label %19, !llvm.loop !14

44:                                               ; preds = %40
  %45 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  br label %46

46:                                               ; preds = %53, %44
  %47 = phi i64 [ 0, %44 ], [ %54, %53 ]
  br label %48

48:                                               ; preds = %48, %46
  %49 = phi i64 [ 0, %46 ], [ %51, %48 ]
  %50 = getelementptr inbounds [400 x [400 x double]], ptr %45, i64 0, i64 %47, i64 %49
  store double 0.000000e+00, ptr %50, align 8, !tbaa !5
  %51 = add nuw nsw i64 %49, 1
  %52 = icmp eq i64 %51, 400
  br i1 %52, label %53, label %48, !llvm.loop !15

53:                                               ; preds = %48
  %54 = add nuw nsw i64 %47, 1
  %55 = icmp eq i64 %54, 400
  br i1 %55, label %56, label %46, !llvm.loop !16

56:                                               ; preds = %53, %74
  %57 = phi i64 [ %75, %74 ], [ 0, %53 ]
  br label %58

58:                                               ; preds = %71, %56
  %59 = phi i64 [ 0, %56 ], [ %72, %71 ]
  %60 = getelementptr inbounds [400 x double], ptr %3, i64 %59, i64 %57
  br label %61

61:                                               ; preds = %61, %58
  %62 = phi i64 [ 0, %58 ], [ %69, %61 ]
  %63 = load double, ptr %60, align 8, !tbaa !5
  %64 = getelementptr inbounds [400 x double], ptr %3, i64 %62, i64 %57
  %65 = load double, ptr %64, align 8, !tbaa !5
  %66 = getelementptr inbounds [400 x [400 x double]], ptr %45, i64 0, i64 %59, i64 %62
  %67 = load double, ptr %66, align 8, !tbaa !5
  %68 = tail call double @llvm.fmuladd.f64(double %63, double %65, double %67)
  store double %68, ptr %66, align 8, !tbaa !5
  %69 = add nuw nsw i64 %62, 1
  %70 = icmp eq i64 %69, 400
  br i1 %70, label %71, label %61, !llvm.loop !17

71:                                               ; preds = %61
  %72 = add nuw nsw i64 %59, 1
  %73 = icmp eq i64 %72, 400
  br i1 %73, label %74, label %58, !llvm.loop !18

74:                                               ; preds = %71
  %75 = add nuw nsw i64 %57, 1
  %76 = icmp eq i64 %75, 400
  br i1 %76, label %77, label %56, !llvm.loop !19

77:                                               ; preds = %74, %86
  %78 = phi i64 [ %87, %86 ], [ 0, %74 ]
  br label %79

79:                                               ; preds = %79, %77
  %80 = phi i64 [ 0, %77 ], [ %84, %79 ]
  %81 = getelementptr inbounds [400 x [400 x double]], ptr %45, i64 0, i64 %78, i64 %80
  %82 = load double, ptr %81, align 8, !tbaa !5
  %83 = getelementptr inbounds [400 x double], ptr %3, i64 %78, i64 %80
  store double %82, ptr %83, align 8, !tbaa !5
  %84 = add nuw nsw i64 %80, 1
  %85 = icmp eq i64 %84, 400
  br i1 %85, label %86, label %79, !llvm.loop !20

86:                                               ; preds = %79
  %87 = add nuw nsw i64 %78, 1
  %88 = icmp eq i64 %87, 400
  br i1 %88, label %89, label %77, !llvm.loop !21

89:                                               ; preds = %86
  tail call void @free(ptr noundef nonnull %45) #4
  br label %90

90:                                               ; preds = %136, %89
  %91 = phi i64 [ 0, %89 ], [ %137, %136 ]
  %92 = icmp eq i64 %91, 0
  br i1 %92, label %116, label %93

93:                                               ; preds = %90, %109
  %94 = phi i64 [ %114, %109 ], [ 0, %90 ]
  %95 = getelementptr inbounds [400 x double], ptr %3, i64 %91, i64 %94
  %96 = load double, ptr %95, align 8, !tbaa !5
  %97 = icmp eq i64 %94, 0
  br i1 %97, label %109, label %98

98:                                               ; preds = %93, %98
  %99 = phi i64 [ %107, %98 ], [ 0, %93 ]
  %100 = phi double [ %106, %98 ], [ %96, %93 ]
  %101 = getelementptr inbounds [400 x double], ptr %3, i64 %91, i64 %99
  %102 = load double, ptr %101, align 8, !tbaa !5
  %103 = getelementptr inbounds [400 x double], ptr %3, i64 %99, i64 %94
  %104 = load double, ptr %103, align 8, !tbaa !5
  %105 = fneg double %102
  %106 = tail call double @llvm.fmuladd.f64(double %105, double %104, double %100)
  %107 = add nuw nsw i64 %99, 1
  %108 = icmp eq i64 %107, %94
  br i1 %108, label %109, label %98, !llvm.loop !22

109:                                              ; preds = %98, %93
  %110 = phi double [ %96, %93 ], [ %106, %98 ]
  %111 = getelementptr inbounds [400 x double], ptr %3, i64 %94, i64 %94
  %112 = load double, ptr %111, align 8, !tbaa !5
  %113 = fdiv double %110, %112
  store double %113, ptr %95, align 8, !tbaa !5
  %114 = add nuw nsw i64 %94, 1
  %115 = icmp eq i64 %114, %91
  br i1 %115, label %116, label %93, !llvm.loop !23

116:                                              ; preds = %109, %90
  br label %117

117:                                              ; preds = %116, %132
  %118 = phi i64 [ %134, %132 ], [ %91, %116 ]
  %119 = getelementptr inbounds [400 x double], ptr %3, i64 %91, i64 %118
  %120 = load double, ptr %119, align 8, !tbaa !5
  br i1 %92, label %132, label %121

121:                                              ; preds = %117, %121
  %122 = phi i64 [ %130, %121 ], [ 0, %117 ]
  %123 = phi double [ %129, %121 ], [ %120, %117 ]
  %124 = getelementptr inbounds [400 x double], ptr %3, i64 %91, i64 %122
  %125 = load double, ptr %124, align 8, !tbaa !5
  %126 = getelementptr inbounds [400 x double], ptr %3, i64 %122, i64 %118
  %127 = load double, ptr %126, align 8, !tbaa !5
  %128 = fneg double %125
  %129 = tail call double @llvm.fmuladd.f64(double %128, double %127, double %123)
  %130 = add nuw nsw i64 %122, 1
  %131 = icmp eq i64 %130, %91
  br i1 %131, label %132, label %121, !llvm.loop !24

132:                                              ; preds = %121, %117
  %133 = phi double [ %120, %117 ], [ %129, %121 ]
  store double %133, ptr %119, align 8, !tbaa !5
  %134 = add nuw nsw i64 %118, 1
  %135 = icmp eq i64 %134, 400
  br i1 %135, label %136, label %117, !llvm.loop !25

136:                                              ; preds = %132
  %137 = add nuw nsw i64 %91, 1
  %138 = icmp eq i64 %137, 400
  br i1 %138, label %139, label %90, !llvm.loop !26

139:                                              ; preds = %136, %155
  %140 = phi i64 [ %158, %155 ], [ 0, %136 ]
  %141 = getelementptr inbounds double, ptr %4, i64 %140
  %142 = load double, ptr %141, align 8, !tbaa !5
  %143 = icmp eq i64 %140, 0
  br i1 %143, label %155, label %144

144:                                              ; preds = %139, %144
  %145 = phi i64 [ %153, %144 ], [ 0, %139 ]
  %146 = phi double [ %152, %144 ], [ %142, %139 ]
  %147 = getelementptr inbounds [400 x double], ptr %3, i64 %140, i64 %145
  %148 = load double, ptr %147, align 8, !tbaa !5
  %149 = getelementptr inbounds double, ptr %6, i64 %145
  %150 = load double, ptr %149, align 8, !tbaa !5
  %151 = fneg double %148
  %152 = tail call double @llvm.fmuladd.f64(double %151, double %150, double %146)
  %153 = add nuw nsw i64 %145, 1
  %154 = icmp eq i64 %153, %140
  br i1 %154, label %155, label %144, !llvm.loop !27

155:                                              ; preds = %144, %139
  %156 = phi double [ %142, %139 ], [ %152, %144 ]
  %157 = getelementptr inbounds double, ptr %6, i64 %140
  store double %156, ptr %157, align 8, !tbaa !5
  %158 = add nuw nsw i64 %140, 1
  %159 = icmp eq i64 %158, 400
  br i1 %159, label %160, label %139, !llvm.loop !28

160:                                              ; preds = %155, %176
  %161 = phi i64 [ %182, %176 ], [ 399, %155 ]
  %162 = getelementptr inbounds double, ptr %6, i64 %161
  %163 = load double, ptr %162, align 8, !tbaa !5
  %164 = icmp ult i64 %161, 399
  br i1 %164, label %165, label %176

165:                                              ; preds = %160, %165
  %166 = phi i64 [ %168, %165 ], [ %161, %160 ]
  %167 = phi double [ %174, %165 ], [ %163, %160 ]
  %168 = add nuw nsw i64 %166, 1
  %169 = getelementptr inbounds [400 x double], ptr %3, i64 %161, i64 %168
  %170 = load double, ptr %169, align 8, !tbaa !5
  %171 = getelementptr inbounds double, ptr %5, i64 %168
  %172 = load double, ptr %171, align 8, !tbaa !5
  %173 = fneg double %170
  %174 = tail call double @llvm.fmuladd.f64(double %173, double %172, double %167)
  %175 = icmp eq i64 %168, 399
  br i1 %175, label %176, label %165, !llvm.loop !29

176:                                              ; preds = %165, %160
  %177 = phi double [ %163, %160 ], [ %174, %165 ]
  %178 = getelementptr inbounds [400 x double], ptr %3, i64 %161, i64 %161
  %179 = load double, ptr %178, align 8, !tbaa !5
  %180 = fdiv double %177, %179
  %181 = getelementptr inbounds double, ptr %5, i64 %161
  store double %180, ptr %181, align 8, !tbaa !5
  %182 = add nsw i64 %161, -1
  %183 = icmp eq i64 %161, 0
  br i1 %183, label %184, label %160, !llvm.loop !30

184:                                              ; preds = %176
  %185 = load ptr, ptr @stderr, align 8, !tbaa !31
  %186 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %185, ptr noundef nonnull @.str) #4
  %187 = load ptr, ptr @stderr, align 8, !tbaa !31
  %188 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %187, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  br label %189

189:                                              ; preds = %197, %184
  %190 = phi i64 [ 0, %184 ], [ %202, %197 ]
  %191 = trunc i64 %190 to i16
  %192 = urem i16 %191, 20
  %193 = icmp eq i16 %192, 0
  br i1 %193, label %194, label %197

194:                                              ; preds = %189
  %195 = load ptr, ptr @stderr, align 8, !tbaa !31
  %196 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %195, ptr noundef nonnull @.str.3) #4
  br label %197

197:                                              ; preds = %194, %189
  %198 = load ptr, ptr @stderr, align 8, !tbaa !31
  %199 = getelementptr inbounds double, ptr %5, i64 %190
  %200 = load double, ptr %199, align 8, !tbaa !5
  %201 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %198, ptr noundef nonnull @.str.4, double noundef %200) #4
  %202 = add nuw nsw i64 %190, 1
  %203 = icmp eq i64 %202, 400
  br i1 %203, label %204, label %189, !llvm.loop !33

204:                                              ; preds = %197
  %205 = load ptr, ptr @stderr, align 8, !tbaa !31
  %206 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %205, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %207 = load ptr, ptr @stderr, align 8, !tbaa !31
  %208 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %207, ptr noundef nonnull @.str.6) #4
  tail call void @free(ptr noundef %3) #4
  tail call void @free(ptr noundef %4) #4
  tail call void @free(ptr noundef nonnull %5) #4
  tail call void @free(ptr noundef %6) #4
  ret i32 0
}

declare ptr @polybench_alloc_data(i64 noundef, i32 noundef) local_unnamed_addr #1

; Function Attrs: nounwind
declare void @free(ptr noundef) local_unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fmuladd.f64(double, double, double) #3

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
!21 = distinct !{!21, !10, !11}
!22 = distinct !{!22, !10, !11}
!23 = distinct !{!23, !10, !11}
!24 = distinct !{!24, !10, !11}
!25 = distinct !{!25, !10, !11}
!26 = distinct !{!26, !10, !11}
!27 = distinct !{!27, !10, !11}
!28 = distinct !{!28, !10, !11}
!29 = distinct !{!29, !10, !11}
!30 = distinct !{!30, !10, !11}
!31 = !{!32, !32, i64 0}
!32 = !{!"any pointer", !7, i64 0}
!33 = distinct !{!33, !10, !11}

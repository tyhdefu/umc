; ModuleID = './polybench-output/ludcmp-no-phis.bc'
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
  %.reg2mem = alloca i64, align 8
  %.reg2mem46 = alloca i64, align 8
  %.reg2mem50 = alloca i64, align 8
  %.reg2mem52 = alloca double, align 8
  %.reg2mem55 = alloca i64, align 8
  %.reg2mem60 = alloca double, align 8
  %.reg2mem63 = alloca i64, align 8
  %.reg2mem72 = alloca i64, align 8
  %.reg2mem75 = alloca i64, align 8
  %.reg2mem78 = alloca double, align 8
  %.reg2mem81 = alloca double, align 8
  %.reg2mem84 = alloca i64, align 8
  %.reg2mem91 = alloca i64, align 8
  %.reg2mem94 = alloca i64, align 8
  %.reg2mem97 = alloca i64, align 8
  %.reg2mem100 = alloca double, align 8
  %.reg2mem103 = alloca double, align 8
  %.reg2mem106 = alloca ptr, align 8
  %.reg2mem109 = alloca i64, align 8
  %.reg2mem113 = alloca i64, align 8
  %.reg2mem116 = alloca i64, align 8
  %.reg2mem119 = alloca double, align 8
  %.reg2mem122 = alloca double, align 8
  %.reg2mem125 = alloca ptr, align 8
  %.reg2mem128 = alloca i64, align 8
  %.reg2mem135 = alloca i1, align 1
  %.reg2mem138 = alloca i64, align 8
  %.reg2mem148 = alloca i64, align 8
  %.reg2mem151 = alloca i64, align 8
  %.reg2mem154 = alloca i64, align 8
  %.reg2mem158 = alloca i64, align 8
  %.reg2mem161 = alloca i64, align 8
  %.reg2mem164 = alloca i64, align 8
  %.reg2mem167 = alloca ptr, align 8
  %.reg2mem169 = alloca i64, align 8
  %.reg2mem173 = alloca i64, align 8
  %.reg2mem177 = alloca i64, align 8
  %.reg2mem180 = alloca i64, align 8
  %.reg2mem183 = alloca i64, align 8
  %.reg2mem186 = alloca ptr, align 8
  %.reg2mem191 = alloca i64, align 8
  %.reg2mem193 = alloca i64, align 8
  %.reg2mem196 = alloca i64, align 8
  %.reg2mem199 = alloca i64, align 8
  %.reg2mem202 = alloca i64, align 8
  %.reg2mem206 = alloca i64, align 8
  %.reg2mem212 = alloca i64, align 8
  %.reg2mem216 = alloca ptr, align 8
  %.reg2mem222 = alloca ptr, align 8
  %.reg2mem228 = alloca ptr, align 8
  %.reg2mem232 = alloca ptr, align 8
  %.reg2mem250 = alloca i64, align 8
  %.reg2mem252 = alloca double, align 8
  %.reg2mem254 = alloca double, align 8
  %.reg2mem256 = alloca i64, align 8
  %.reg2mem258 = alloca i64, align 8
  %.reg2mem260 = alloca double, align 8
  %.reg2mem262 = alloca double, align 8
  %.reg2mem264 = alloca i64, align 8
  %.reg2mem266 = alloca i64, align 8
  %.reg2mem268 = alloca double, align 8
  %.reg2mem270 = alloca double, align 8
  %.reg2mem272 = alloca i64, align 8
  %.reg2mem274 = alloca i64, align 8
  %.reg2mem276 = alloca double, align 8
  %.reg2mem278 = alloca double, align 8
  %.reg2mem280 = alloca i64, align 8
  %.reg2mem282 = alloca i64, align 8
  %.reg2mem284 = alloca i64, align 8
  %.reg2mem286 = alloca i64, align 8
  %.reg2mem288 = alloca i64, align 8
  %.reg2mem290 = alloca i64, align 8
  %.reg2mem292 = alloca i64, align 8
  %.reg2mem294 = alloca i64, align 8
  %.reg2mem296 = alloca i64, align 8
  %.reg2mem298 = alloca i64, align 8
  %.reg2mem300 = alloca i64, align 8
  %.reg2mem302 = alloca i64, align 8
  %.reg2mem304 = alloca i64, align 8
  %.reg2mem306 = alloca i64, align 8
  %.reg2mem308 = alloca i64, align 8
  %"reg2mem alloca point" = bitcast i32 0 to i32
  %3 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %3, ptr %.reg2mem232, align 8
  %4 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %4, ptr %.reg2mem228, align 8
  %5 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %5, ptr %.reg2mem222, align 8
  %6 = tail call ptr @polybench_alloc_data(i64 noundef 400, i32 noundef 8) #4
  store ptr %6, ptr %.reg2mem216, align 8
  store i64 0, ptr %.reg2mem308, align 8
  br label %7

7:                                                ; preds = %._crit_edge1, %2
  %.reload309 = load i64, ptr %.reg2mem308, align 8
  %.reload226 = load ptr, ptr %.reg2mem222, align 8
  %8 = getelementptr inbounds double, ptr %.reload226, i64 %.reload309
  store double 0.000000e+00, ptr %8, align 8, !tbaa !5
  %.reload220 = load ptr, ptr %.reg2mem216, align 8
  %9 = getelementptr inbounds double, ptr %.reload220, i64 %.reload309
  store double 0.000000e+00, ptr %9, align 8, !tbaa !5
  %10 = add nuw nsw i64 %.reload309, 1
  store i64 %10, ptr %.reg2mem212, align 8
  %.reload215 = load i64, ptr %.reg2mem212, align 8
  %11 = trunc nuw nsw i64 %.reload215 to i32
  %12 = uitofp nneg i32 %11 to double
  %13 = fdiv double %12, 4.000000e+02
  %14 = fmul double %13, 5.000000e-01
  %15 = fadd double %14, 4.000000e+00
  %.reload230 = load ptr, ptr %.reg2mem228, align 8
  %16 = getelementptr inbounds double, ptr %.reload230, i64 %.reload309
  store double %15, ptr %16, align 8, !tbaa !5
  %.reload214 = load i64, ptr %.reg2mem212, align 8
  %17 = icmp eq i64 %.reload214, 400
  br i1 %17, label %._crit_edge, label %._crit_edge1, !llvm.loop !9

._crit_edge1:                                     ; preds = %7
  %.reload213 = load i64, ptr %.reg2mem212, align 8
  store i64 %.reload213, ptr %.reg2mem308, align 8
  br label %7

._crit_edge:                                      ; preds = %7
  store i64 1, ptr %.reg2mem304, align 8
  store i64 0, ptr %.reg2mem306, align 8
  br label %18

18:                                               ; preds = %._crit_edge7, %._crit_edge
  %.reload307 = load i64, ptr %.reg2mem306, align 8
  %.reload305 = load i64, ptr %.reg2mem304, align 8
  store i64 %.reload307, ptr %.reg2mem206, align 8
  store i64 %.reload305, ptr %.reg2mem202, align 8
  store i64 0, ptr %.reg2mem302, align 8
  br label %19

19:                                               ; preds = %._crit_edge2, %18
  %.reload303 = load i64, ptr %.reg2mem302, align 8
  %20 = trunc i64 %.reload303 to i32
  %21 = sub i32 0, %20
  %22 = sitofp i32 %21 to double
  %23 = fdiv double %22, 4.000000e+02
  %24 = fadd double %23, 1.000000e+00
  %.reload211 = load i64, ptr %.reg2mem206, align 8
  %.reload248 = load ptr, ptr %.reg2mem232, align 8
  %25 = getelementptr inbounds [400 x double], ptr %.reload248, i64 %.reload211, i64 %.reload303
  store double %24, ptr %25, align 8, !tbaa !5
  %26 = add nuw nsw i64 %.reload303, 1
  store i64 %26, ptr %.reg2mem199, align 8
  %.reload201 = load i64, ptr %.reg2mem199, align 8
  %.reload205 = load i64, ptr %.reg2mem202, align 8
  %27 = icmp eq i64 %.reload201, %.reload205
  br i1 %27, label %28, label %._crit_edge2, !llvm.loop !12

._crit_edge2:                                     ; preds = %19
  %.reload200 = load i64, ptr %.reg2mem199, align 8
  store i64 %.reload200, ptr %.reg2mem302, align 8
  br label %19

28:                                               ; preds = %19
  %.reload210 = load i64, ptr %.reg2mem206, align 8
  %29 = add nuw nsw i64 %.reload210, 1
  store i64 %29, ptr %.reg2mem196, align 8
  %.reload209 = load i64, ptr %.reg2mem206, align 8
  %30 = icmp ult i64 %.reload209, 399
  br i1 %30, label %._crit_edge3, label %._crit_edge4

._crit_edge4:                                     ; preds = %28
  br label %35

._crit_edge3:                                     ; preds = %28
  %.reload203 = load i64, ptr %.reg2mem202, align 8
  store i64 %.reload203, ptr %.reg2mem300, align 8
  br label %31

31:                                               ; preds = %._crit_edge6, %._crit_edge3
  %.reload301 = load i64, ptr %.reg2mem300, align 8
  %.reload207 = load i64, ptr %.reg2mem206, align 8
  %.reload243 = load ptr, ptr %.reg2mem232, align 8
  %32 = getelementptr inbounds [400 x double], ptr %.reload243, i64 %.reload207, i64 %.reload301
  store double 0.000000e+00, ptr %32, align 8, !tbaa !5
  %33 = add nuw nsw i64 %.reload301, 1
  store i64 %33, ptr %.reg2mem193, align 8
  %.reload195 = load i64, ptr %.reg2mem193, align 8
  %34 = icmp eq i64 %.reload195, 400
  br i1 %34, label %._crit_edge5, label %._crit_edge6, !llvm.loop !13

._crit_edge6:                                     ; preds = %31
  %.reload194 = load i64, ptr %.reg2mem193, align 8
  store i64 %.reload194, ptr %.reg2mem300, align 8
  br label %31

._crit_edge5:                                     ; preds = %31
  br label %35

35:                                               ; preds = %._crit_edge5, %._crit_edge4
  %.reload208 = load i64, ptr %.reg2mem206, align 8
  %.reload247 = load ptr, ptr %.reg2mem232, align 8
  %36 = getelementptr inbounds [400 x double], ptr %.reload247, i64 %.reload208, i64 %.reload208
  store double 1.000000e+00, ptr %36, align 8, !tbaa !5
  %.reload204 = load i64, ptr %.reg2mem202, align 8
  %37 = add nuw nsw i64 %.reload204, 1
  store i64 %37, ptr %.reg2mem191, align 8
  %.reload198 = load i64, ptr %.reg2mem196, align 8
  %38 = icmp eq i64 %.reload198, 400
  br i1 %38, label %39, label %._crit_edge7, !llvm.loop !14

._crit_edge7:                                     ; preds = %35
  %.reload192 = load i64, ptr %.reg2mem191, align 8
  %.reload197 = load i64, ptr %.reg2mem196, align 8
  store i64 %.reload192, ptr %.reg2mem304, align 8
  store i64 %.reload197, ptr %.reg2mem306, align 8
  br label %18

39:                                               ; preds = %35
  %40 = tail call ptr @polybench_alloc_data(i64 noundef 160000, i32 noundef 8) #4
  store ptr %40, ptr %.reg2mem186, align 8
  store i64 0, ptr %.reg2mem298, align 8
  br label %41

41:                                               ; preds = %._crit_edge10, %39
  %.reload299 = load i64, ptr %.reg2mem298, align 8
  store i64 %.reload299, ptr %.reg2mem183, align 8
  store i64 0, ptr %.reg2mem296, align 8
  br label %42

42:                                               ; preds = %._crit_edge8, %41
  %.reload297 = load i64, ptr %.reg2mem296, align 8
  %.reload185 = load i64, ptr %.reg2mem183, align 8
  %.reload190 = load ptr, ptr %.reg2mem186, align 8
  %43 = getelementptr inbounds [400 x [400 x double]], ptr %.reload190, i64 0, i64 %.reload185, i64 %.reload297
  store double 0.000000e+00, ptr %43, align 8, !tbaa !5
  %44 = add nuw nsw i64 %.reload297, 1
  store i64 %44, ptr %.reg2mem180, align 8
  %.reload182 = load i64, ptr %.reg2mem180, align 8
  %45 = icmp eq i64 %.reload182, 400
  br i1 %45, label %46, label %._crit_edge8, !llvm.loop !15

._crit_edge8:                                     ; preds = %42
  %.reload181 = load i64, ptr %.reg2mem180, align 8
  store i64 %.reload181, ptr %.reg2mem296, align 8
  br label %42

46:                                               ; preds = %42
  %.reload184 = load i64, ptr %.reg2mem183, align 8
  %47 = add nuw nsw i64 %.reload184, 1
  store i64 %47, ptr %.reg2mem177, align 8
  %.reload179 = load i64, ptr %.reg2mem177, align 8
  %48 = icmp eq i64 %.reload179, 400
  br i1 %48, label %._crit_edge9, label %._crit_edge10, !llvm.loop !16

._crit_edge10:                                    ; preds = %46
  %.reload178 = load i64, ptr %.reg2mem177, align 8
  store i64 %.reload178, ptr %.reg2mem298, align 8
  br label %41

._crit_edge9:                                     ; preds = %46
  store i64 0, ptr %.reg2mem294, align 8
  br label %49

49:                                               ; preds = %._crit_edge14, %._crit_edge9
  %.reload295 = load i64, ptr %.reg2mem294, align 8
  store i64 %.reload295, ptr %.reg2mem173, align 8
  store i64 0, ptr %.reg2mem292, align 8
  br label %50

50:                                               ; preds = %._crit_edge12, %49
  %.reload293 = load i64, ptr %.reg2mem292, align 8
  store i64 %.reload293, ptr %.reg2mem169, align 8
  %.reload172 = load i64, ptr %.reg2mem169, align 8
  %.reload176 = load i64, ptr %.reg2mem173, align 8
  %.reload246 = load ptr, ptr %.reg2mem232, align 8
  %51 = getelementptr inbounds [400 x double], ptr %.reload246, i64 %.reload172, i64 %.reload176
  store ptr %51, ptr %.reg2mem167, align 8
  store i64 0, ptr %.reg2mem290, align 8
  br label %52

52:                                               ; preds = %._crit_edge11, %50
  %.reload291 = load i64, ptr %.reg2mem290, align 8
  %.reload168 = load ptr, ptr %.reg2mem167, align 8
  %53 = load double, ptr %.reload168, align 8, !tbaa !5
  %.reload175 = load i64, ptr %.reg2mem173, align 8
  %.reload245 = load ptr, ptr %.reg2mem232, align 8
  %54 = getelementptr inbounds [400 x double], ptr %.reload245, i64 %.reload291, i64 %.reload175
  %55 = load double, ptr %54, align 8, !tbaa !5
  %.reload171 = load i64, ptr %.reg2mem169, align 8
  %.reload189 = load ptr, ptr %.reg2mem186, align 8
  %56 = getelementptr inbounds [400 x [400 x double]], ptr %.reload189, i64 0, i64 %.reload171, i64 %.reload291
  %57 = load double, ptr %56, align 8, !tbaa !5
  %58 = tail call double @llvm.fmuladd.f64(double %53, double %55, double %57)
  store double %58, ptr %56, align 8, !tbaa !5
  %59 = add nuw nsw i64 %.reload291, 1
  store i64 %59, ptr %.reg2mem164, align 8
  %.reload166 = load i64, ptr %.reg2mem164, align 8
  %60 = icmp eq i64 %.reload166, 400
  br i1 %60, label %61, label %._crit_edge11, !llvm.loop !17

._crit_edge11:                                    ; preds = %52
  %.reload165 = load i64, ptr %.reg2mem164, align 8
  store i64 %.reload165, ptr %.reg2mem290, align 8
  br label %52

61:                                               ; preds = %52
  %.reload170 = load i64, ptr %.reg2mem169, align 8
  %62 = add nuw nsw i64 %.reload170, 1
  store i64 %62, ptr %.reg2mem161, align 8
  %.reload163 = load i64, ptr %.reg2mem161, align 8
  %63 = icmp eq i64 %.reload163, 400
  br i1 %63, label %64, label %._crit_edge12, !llvm.loop !18

._crit_edge12:                                    ; preds = %61
  %.reload162 = load i64, ptr %.reg2mem161, align 8
  store i64 %.reload162, ptr %.reg2mem292, align 8
  br label %50

64:                                               ; preds = %61
  %.reload174 = load i64, ptr %.reg2mem173, align 8
  %65 = add nuw nsw i64 %.reload174, 1
  store i64 %65, ptr %.reg2mem158, align 8
  %.reload160 = load i64, ptr %.reg2mem158, align 8
  %66 = icmp eq i64 %.reload160, 400
  br i1 %66, label %._crit_edge13, label %._crit_edge14, !llvm.loop !19

._crit_edge14:                                    ; preds = %64
  %.reload159 = load i64, ptr %.reg2mem158, align 8
  store i64 %.reload159, ptr %.reg2mem294, align 8
  br label %49

._crit_edge13:                                    ; preds = %64
  store i64 0, ptr %.reg2mem288, align 8
  br label %67

67:                                               ; preds = %._crit_edge16, %._crit_edge13
  %.reload289 = load i64, ptr %.reg2mem288, align 8
  store i64 %.reload289, ptr %.reg2mem154, align 8
  store i64 0, ptr %.reg2mem286, align 8
  br label %68

68:                                               ; preds = %._crit_edge15, %67
  %.reload287 = load i64, ptr %.reg2mem286, align 8
  %.reload157 = load i64, ptr %.reg2mem154, align 8
  %.reload188 = load ptr, ptr %.reg2mem186, align 8
  %69 = getelementptr inbounds [400 x [400 x double]], ptr %.reload188, i64 0, i64 %.reload157, i64 %.reload287
  %70 = load double, ptr %69, align 8, !tbaa !5
  %.reload156 = load i64, ptr %.reg2mem154, align 8
  %.reload244 = load ptr, ptr %.reg2mem232, align 8
  %71 = getelementptr inbounds [400 x double], ptr %.reload244, i64 %.reload156, i64 %.reload287
  store double %70, ptr %71, align 8, !tbaa !5
  %72 = add nuw nsw i64 %.reload287, 1
  store i64 %72, ptr %.reg2mem151, align 8
  %.reload153 = load i64, ptr %.reg2mem151, align 8
  %73 = icmp eq i64 %.reload153, 400
  br i1 %73, label %74, label %._crit_edge15, !llvm.loop !20

._crit_edge15:                                    ; preds = %68
  %.reload152 = load i64, ptr %.reg2mem151, align 8
  store i64 %.reload152, ptr %.reg2mem286, align 8
  br label %68

74:                                               ; preds = %68
  %.reload155 = load i64, ptr %.reg2mem154, align 8
  %75 = add nuw nsw i64 %.reload155, 1
  store i64 %75, ptr %.reg2mem148, align 8
  %.reload150 = load i64, ptr %.reg2mem148, align 8
  %76 = icmp eq i64 %.reload150, 400
  br i1 %76, label %77, label %._crit_edge16, !llvm.loop !21

._crit_edge16:                                    ; preds = %74
  %.reload149 = load i64, ptr %.reg2mem148, align 8
  store i64 %.reload149, ptr %.reg2mem288, align 8
  br label %67

77:                                               ; preds = %74
  %.reload187 = load ptr, ptr %.reg2mem186, align 8
  tail call void @free(ptr noundef nonnull %.reload187) #4
  store i64 0, ptr %.reg2mem284, align 8
  br label %78

78:                                               ; preds = %._crit_edge31, %77
  %.reload285 = load i64, ptr %.reg2mem284, align 8
  store i64 %.reload285, ptr %.reg2mem138, align 8
  %.reload147 = load i64, ptr %.reg2mem138, align 8
  %79 = icmp eq i64 %.reload147, 0
  store i1 %79, ptr %.reg2mem135, align 1
  %.reload137 = load i1, ptr %.reg2mem135, align 1
  br i1 %.reload137, label %._crit_edge17, label %._crit_edge18

._crit_edge18:                                    ; preds = %78
  store i64 0, ptr %.reg2mem282, align 8
  br label %80

._crit_edge17:                                    ; preds = %78
  br label %99

80:                                               ; preds = %._crit_edge24, %._crit_edge18
  %.reload283 = load i64, ptr %.reg2mem282, align 8
  store i64 %.reload283, ptr %.reg2mem128, align 8
  %.reload134 = load i64, ptr %.reg2mem128, align 8
  %.reload146 = load i64, ptr %.reg2mem138, align 8
  %.reload242 = load ptr, ptr %.reg2mem232, align 8
  %81 = getelementptr inbounds [400 x double], ptr %.reload242, i64 %.reload146, i64 %.reload134
  store ptr %81, ptr %.reg2mem125, align 8
  %.reload127 = load ptr, ptr %.reg2mem125, align 8
  %82 = load double, ptr %.reload127, align 8, !tbaa !5
  store double %82, ptr %.reg2mem122, align 8
  %.reload133 = load i64, ptr %.reg2mem128, align 8
  %83 = icmp eq i64 %.reload133, 0
  br i1 %83, label %._crit_edge19, label %._crit_edge20

._crit_edge20:                                    ; preds = %80
  %.reload123 = load double, ptr %.reg2mem122, align 8
  store double %.reload123, ptr %.reg2mem278, align 8
  store i64 0, ptr %.reg2mem280, align 8
  br label %84

._crit_edge19:                                    ; preds = %80
  %.reload124 = load double, ptr %.reg2mem122, align 8
  store double %.reload124, ptr %.reg2mem276, align 8
  br label %93

84:                                               ; preds = %._crit_edge22, %._crit_edge20
  %.reload281 = load i64, ptr %.reg2mem280, align 8
  %.reload279 = load double, ptr %.reg2mem278, align 8
  %.reload145 = load i64, ptr %.reg2mem138, align 8
  %.reload241 = load ptr, ptr %.reg2mem232, align 8
  %85 = getelementptr inbounds [400 x double], ptr %.reload241, i64 %.reload145, i64 %.reload281
  %86 = load double, ptr %85, align 8, !tbaa !5
  %.reload132 = load i64, ptr %.reg2mem128, align 8
  %.reload240 = load ptr, ptr %.reg2mem232, align 8
  %87 = getelementptr inbounds [400 x double], ptr %.reload240, i64 %.reload281, i64 %.reload132
  %88 = load double, ptr %87, align 8, !tbaa !5
  %89 = fneg double %86
  %90 = tail call double @llvm.fmuladd.f64(double %89, double %88, double %.reload279)
  store double %90, ptr %.reg2mem119, align 8
  %91 = add nuw nsw i64 %.reload281, 1
  store i64 %91, ptr %.reg2mem116, align 8
  %.reload118 = load i64, ptr %.reg2mem116, align 8
  %.reload131 = load i64, ptr %.reg2mem128, align 8
  %92 = icmp eq i64 %.reload118, %.reload131
  br i1 %92, label %._crit_edge21, label %._crit_edge22, !llvm.loop !22

._crit_edge22:                                    ; preds = %84
  %.reload117 = load i64, ptr %.reg2mem116, align 8
  %.reload121 = load double, ptr %.reg2mem119, align 8
  store double %.reload121, ptr %.reg2mem278, align 8
  store i64 %.reload117, ptr %.reg2mem280, align 8
  br label %84

._crit_edge21:                                    ; preds = %84
  %.reload120 = load double, ptr %.reg2mem119, align 8
  store double %.reload120, ptr %.reg2mem276, align 8
  br label %93

93:                                               ; preds = %._crit_edge21, %._crit_edge19
  %.reload277 = load double, ptr %.reg2mem276, align 8
  %.reload130 = load i64, ptr %.reg2mem128, align 8
  %.reload239 = load ptr, ptr %.reg2mem232, align 8
  %94 = getelementptr inbounds [400 x double], ptr %.reload239, i64 %.reload130, i64 %.reload130
  %95 = load double, ptr %94, align 8, !tbaa !5
  %96 = fdiv double %.reload277, %95
  %.reload126 = load ptr, ptr %.reg2mem125, align 8
  store double %96, ptr %.reload126, align 8, !tbaa !5
  %.reload129 = load i64, ptr %.reg2mem128, align 8
  %97 = add nuw nsw i64 %.reload129, 1
  store i64 %97, ptr %.reg2mem113, align 8
  %.reload115 = load i64, ptr %.reg2mem113, align 8
  %.reload144 = load i64, ptr %.reg2mem138, align 8
  %98 = icmp eq i64 %.reload115, %.reload144
  br i1 %98, label %._crit_edge23, label %._crit_edge24, !llvm.loop !23

._crit_edge24:                                    ; preds = %93
  %.reload114 = load i64, ptr %.reg2mem113, align 8
  store i64 %.reload114, ptr %.reg2mem282, align 8
  br label %80

._crit_edge23:                                    ; preds = %93
  br label %99

99:                                               ; preds = %._crit_edge23, %._crit_edge17
  %.reload139 = load i64, ptr %.reg2mem138, align 8
  store i64 %.reload139, ptr %.reg2mem274, align 8
  br label %100

100:                                              ; preds = %._crit_edge29, %99
  %.reload275 = load i64, ptr %.reg2mem274, align 8
  store i64 %.reload275, ptr %.reg2mem109, align 8
  %.reload112 = load i64, ptr %.reg2mem109, align 8
  %.reload143 = load i64, ptr %.reg2mem138, align 8
  %.reload238 = load ptr, ptr %.reg2mem232, align 8
  %101 = getelementptr inbounds [400 x double], ptr %.reload238, i64 %.reload143, i64 %.reload112
  store ptr %101, ptr %.reg2mem106, align 8
  %.reload108 = load ptr, ptr %.reg2mem106, align 8
  %102 = load double, ptr %.reload108, align 8, !tbaa !5
  store double %102, ptr %.reg2mem103, align 8
  %.reload136 = load i1, ptr %.reg2mem135, align 1
  br i1 %.reload136, label %._crit_edge25, label %._crit_edge26

._crit_edge26:                                    ; preds = %100
  %.reload104 = load double, ptr %.reg2mem103, align 8
  store double %.reload104, ptr %.reg2mem270, align 8
  store i64 0, ptr %.reg2mem272, align 8
  br label %103

._crit_edge25:                                    ; preds = %100
  %.reload105 = load double, ptr %.reg2mem103, align 8
  store double %.reload105, ptr %.reg2mem268, align 8
  br label %112

103:                                              ; preds = %._crit_edge28, %._crit_edge26
  %.reload273 = load i64, ptr %.reg2mem272, align 8
  %.reload271 = load double, ptr %.reg2mem270, align 8
  %.reload142 = load i64, ptr %.reg2mem138, align 8
  %.reload237 = load ptr, ptr %.reg2mem232, align 8
  %104 = getelementptr inbounds [400 x double], ptr %.reload237, i64 %.reload142, i64 %.reload273
  %105 = load double, ptr %104, align 8, !tbaa !5
  %.reload111 = load i64, ptr %.reg2mem109, align 8
  %.reload236 = load ptr, ptr %.reg2mem232, align 8
  %106 = getelementptr inbounds [400 x double], ptr %.reload236, i64 %.reload273, i64 %.reload111
  %107 = load double, ptr %106, align 8, !tbaa !5
  %108 = fneg double %105
  %109 = tail call double @llvm.fmuladd.f64(double %108, double %107, double %.reload271)
  store double %109, ptr %.reg2mem100, align 8
  %110 = add nuw nsw i64 %.reload273, 1
  store i64 %110, ptr %.reg2mem97, align 8
  %.reload99 = load i64, ptr %.reg2mem97, align 8
  %.reload141 = load i64, ptr %.reg2mem138, align 8
  %111 = icmp eq i64 %.reload99, %.reload141
  br i1 %111, label %._crit_edge27, label %._crit_edge28, !llvm.loop !24

._crit_edge28:                                    ; preds = %103
  %.reload98 = load i64, ptr %.reg2mem97, align 8
  %.reload102 = load double, ptr %.reg2mem100, align 8
  store double %.reload102, ptr %.reg2mem270, align 8
  store i64 %.reload98, ptr %.reg2mem272, align 8
  br label %103

._crit_edge27:                                    ; preds = %103
  %.reload101 = load double, ptr %.reg2mem100, align 8
  store double %.reload101, ptr %.reg2mem268, align 8
  br label %112

112:                                              ; preds = %._crit_edge27, %._crit_edge25
  %.reload269 = load double, ptr %.reg2mem268, align 8
  %.reload107 = load ptr, ptr %.reg2mem106, align 8
  store double %.reload269, ptr %.reload107, align 8, !tbaa !5
  %.reload110 = load i64, ptr %.reg2mem109, align 8
  %113 = add nuw nsw i64 %.reload110, 1
  store i64 %113, ptr %.reg2mem94, align 8
  %.reload96 = load i64, ptr %.reg2mem94, align 8
  %114 = icmp eq i64 %.reload96, 400
  br i1 %114, label %115, label %._crit_edge29, !llvm.loop !25

._crit_edge29:                                    ; preds = %112
  %.reload95 = load i64, ptr %.reg2mem94, align 8
  store i64 %.reload95, ptr %.reg2mem274, align 8
  br label %100

115:                                              ; preds = %112
  %.reload140 = load i64, ptr %.reg2mem138, align 8
  %116 = add nuw nsw i64 %.reload140, 1
  store i64 %116, ptr %.reg2mem91, align 8
  %.reload93 = load i64, ptr %.reg2mem91, align 8
  %117 = icmp eq i64 %.reload93, 400
  br i1 %117, label %._crit_edge30, label %._crit_edge31, !llvm.loop !26

._crit_edge31:                                    ; preds = %115
  %.reload92 = load i64, ptr %.reg2mem91, align 8
  store i64 %.reload92, ptr %.reg2mem284, align 8
  br label %78

._crit_edge30:                                    ; preds = %115
  store i64 0, ptr %.reg2mem266, align 8
  br label %118

118:                                              ; preds = %._crit_edge37, %._crit_edge30
  %.reload267 = load i64, ptr %.reg2mem266, align 8
  store i64 %.reload267, ptr %.reg2mem84, align 8
  %.reload90 = load i64, ptr %.reg2mem84, align 8
  %.reload229 = load ptr, ptr %.reg2mem228, align 8
  %119 = getelementptr inbounds double, ptr %.reload229, i64 %.reload90
  %120 = load double, ptr %119, align 8, !tbaa !5
  store double %120, ptr %.reg2mem81, align 8
  %.reload89 = load i64, ptr %.reg2mem84, align 8
  %121 = icmp eq i64 %.reload89, 0
  br i1 %121, label %._crit_edge32, label %._crit_edge33

._crit_edge33:                                    ; preds = %118
  %.reload82 = load double, ptr %.reg2mem81, align 8
  store double %.reload82, ptr %.reg2mem262, align 8
  store i64 0, ptr %.reg2mem264, align 8
  br label %122

._crit_edge32:                                    ; preds = %118
  %.reload83 = load double, ptr %.reg2mem81, align 8
  store double %.reload83, ptr %.reg2mem260, align 8
  br label %131

122:                                              ; preds = %._crit_edge35, %._crit_edge33
  %.reload265 = load i64, ptr %.reg2mem264, align 8
  %.reload263 = load double, ptr %.reg2mem262, align 8
  %.reload88 = load i64, ptr %.reg2mem84, align 8
  %.reload235 = load ptr, ptr %.reg2mem232, align 8
  %123 = getelementptr inbounds [400 x double], ptr %.reload235, i64 %.reload88, i64 %.reload265
  %124 = load double, ptr %123, align 8, !tbaa !5
  %.reload219 = load ptr, ptr %.reg2mem216, align 8
  %125 = getelementptr inbounds double, ptr %.reload219, i64 %.reload265
  %126 = load double, ptr %125, align 8, !tbaa !5
  %127 = fneg double %124
  %128 = tail call double @llvm.fmuladd.f64(double %127, double %126, double %.reload263)
  store double %128, ptr %.reg2mem78, align 8
  %129 = add nuw nsw i64 %.reload265, 1
  store i64 %129, ptr %.reg2mem75, align 8
  %.reload77 = load i64, ptr %.reg2mem75, align 8
  %.reload87 = load i64, ptr %.reg2mem84, align 8
  %130 = icmp eq i64 %.reload77, %.reload87
  br i1 %130, label %._crit_edge34, label %._crit_edge35, !llvm.loop !27

._crit_edge35:                                    ; preds = %122
  %.reload76 = load i64, ptr %.reg2mem75, align 8
  %.reload80 = load double, ptr %.reg2mem78, align 8
  store double %.reload80, ptr %.reg2mem262, align 8
  store i64 %.reload76, ptr %.reg2mem264, align 8
  br label %122

._crit_edge34:                                    ; preds = %122
  %.reload79 = load double, ptr %.reg2mem78, align 8
  store double %.reload79, ptr %.reg2mem260, align 8
  br label %131

131:                                              ; preds = %._crit_edge34, %._crit_edge32
  %.reload261 = load double, ptr %.reg2mem260, align 8
  %.reload86 = load i64, ptr %.reg2mem84, align 8
  %.reload218 = load ptr, ptr %.reg2mem216, align 8
  %132 = getelementptr inbounds double, ptr %.reload218, i64 %.reload86
  store double %.reload261, ptr %132, align 8, !tbaa !5
  %.reload85 = load i64, ptr %.reg2mem84, align 8
  %133 = add nuw nsw i64 %.reload85, 1
  store i64 %133, ptr %.reg2mem72, align 8
  %.reload74 = load i64, ptr %.reg2mem72, align 8
  %134 = icmp eq i64 %.reload74, 400
  br i1 %134, label %._crit_edge36, label %._crit_edge37, !llvm.loop !28

._crit_edge37:                                    ; preds = %131
  %.reload73 = load i64, ptr %.reg2mem72, align 8
  store i64 %.reload73, ptr %.reg2mem266, align 8
  br label %118

._crit_edge36:                                    ; preds = %131
  store i64 399, ptr %.reg2mem258, align 8
  br label %135

135:                                              ; preds = %._crit_edge42, %._crit_edge36
  %.reload259 = load i64, ptr %.reg2mem258, align 8
  store i64 %.reload259, ptr %.reg2mem63, align 8
  %.reload71 = load i64, ptr %.reg2mem63, align 8
  %.reload217 = load ptr, ptr %.reg2mem216, align 8
  %136 = getelementptr inbounds double, ptr %.reload217, i64 %.reload71
  %137 = load double, ptr %136, align 8, !tbaa !5
  store double %137, ptr %.reg2mem60, align 8
  %.reload70 = load i64, ptr %.reg2mem63, align 8
  %138 = icmp ult i64 %.reload70, 399
  br i1 %138, label %._crit_edge38, label %._crit_edge39

._crit_edge39:                                    ; preds = %135
  %.reload62 = load double, ptr %.reg2mem60, align 8
  store double %.reload62, ptr %.reg2mem252, align 8
  br label %148

._crit_edge38:                                    ; preds = %135
  %.reload61 = load double, ptr %.reg2mem60, align 8
  %.reload64 = load i64, ptr %.reg2mem63, align 8
  store double %.reload61, ptr %.reg2mem254, align 8
  store i64 %.reload64, ptr %.reg2mem256, align 8
  br label %139

139:                                              ; preds = %._crit_edge41, %._crit_edge38
  %.reload257 = load i64, ptr %.reg2mem256, align 8
  %.reload255 = load double, ptr %.reg2mem254, align 8
  %140 = add nuw nsw i64 %.reload257, 1
  store i64 %140, ptr %.reg2mem55, align 8
  %.reload59 = load i64, ptr %.reg2mem55, align 8
  %.reload65 = load i64, ptr %.reg2mem63, align 8
  %.reload233 = load ptr, ptr %.reg2mem232, align 8
  %141 = getelementptr inbounds [400 x double], ptr %.reload233, i64 %.reload65, i64 %.reload59
  %142 = load double, ptr %141, align 8, !tbaa !5
  %.reload58 = load i64, ptr %.reg2mem55, align 8
  %.reload224 = load ptr, ptr %.reg2mem222, align 8
  %143 = getelementptr inbounds double, ptr %.reload224, i64 %.reload58
  %144 = load double, ptr %143, align 8, !tbaa !5
  %145 = fneg double %142
  %146 = tail call double @llvm.fmuladd.f64(double %145, double %144, double %.reload255)
  store double %146, ptr %.reg2mem52, align 8
  %.reload57 = load i64, ptr %.reg2mem55, align 8
  %147 = icmp eq i64 %.reload57, 399
  br i1 %147, label %._crit_edge40, label %._crit_edge41, !llvm.loop !29

._crit_edge41:                                    ; preds = %139
  %.reload54 = load double, ptr %.reg2mem52, align 8
  %.reload56 = load i64, ptr %.reg2mem55, align 8
  store double %.reload54, ptr %.reg2mem254, align 8
  store i64 %.reload56, ptr %.reg2mem256, align 8
  br label %139

._crit_edge40:                                    ; preds = %139
  %.reload53 = load double, ptr %.reg2mem52, align 8
  store double %.reload53, ptr %.reg2mem252, align 8
  br label %148

148:                                              ; preds = %._crit_edge40, %._crit_edge39
  %.reload253 = load double, ptr %.reg2mem252, align 8
  %.reload69 = load i64, ptr %.reg2mem63, align 8
  %.reload234 = load ptr, ptr %.reg2mem232, align 8
  %149 = getelementptr inbounds [400 x double], ptr %.reload234, i64 %.reload69, i64 %.reload69
  %150 = load double, ptr %149, align 8, !tbaa !5
  %151 = fdiv double %.reload253, %150
  %.reload68 = load i64, ptr %.reg2mem63, align 8
  %.reload225 = load ptr, ptr %.reg2mem222, align 8
  %152 = getelementptr inbounds double, ptr %.reload225, i64 %.reload68
  store double %151, ptr %152, align 8, !tbaa !5
  %.reload67 = load i64, ptr %.reg2mem63, align 8
  %153 = add nsw i64 %.reload67, -1
  store i64 %153, ptr %.reg2mem50, align 8
  %.reload66 = load i64, ptr %.reg2mem63, align 8
  %154 = icmp eq i64 %.reload66, 0
  br i1 %154, label %155, label %._crit_edge42, !llvm.loop !30

._crit_edge42:                                    ; preds = %148
  %.reload51 = load i64, ptr %.reg2mem50, align 8
  store i64 %.reload51, ptr %.reg2mem258, align 8
  br label %135

155:                                              ; preds = %148
  %156 = load ptr, ptr @stderr, align 8, !tbaa !31
  %157 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %156, ptr noundef nonnull @.str) #4
  %158 = load ptr, ptr @stderr, align 8, !tbaa !31
  %159 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %158, ptr noundef nonnull @.str.1, ptr noundef nonnull @.str.2) #4
  store i64 0, ptr %.reg2mem250, align 8
  br label %160

160:                                              ; preds = %._crit_edge44, %155
  %.reload251 = load i64, ptr %.reg2mem250, align 8
  store i64 %.reload251, ptr %.reg2mem46, align 8
  %.reload49 = load i64, ptr %.reg2mem46, align 8
  %161 = trunc i64 %.reload49 to i16
  %162 = urem i16 %161, 20
  %163 = icmp eq i16 %162, 0
  br i1 %163, label %164, label %._crit_edge43

._crit_edge43:                                    ; preds = %160
  br label %167

164:                                              ; preds = %160
  %165 = load ptr, ptr @stderr, align 8, !tbaa !31
  %166 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %165, ptr noundef nonnull @.str.3) #4
  br label %167

167:                                              ; preds = %._crit_edge43, %164
  %168 = load ptr, ptr @stderr, align 8, !tbaa !31
  %.reload48 = load i64, ptr %.reg2mem46, align 8
  %.reload223 = load ptr, ptr %.reg2mem222, align 8
  %169 = getelementptr inbounds double, ptr %.reload223, i64 %.reload48
  %170 = load double, ptr %169, align 8, !tbaa !5
  %171 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %168, ptr noundef nonnull @.str.4, double noundef %170) #4
  %.reload47 = load i64, ptr %.reg2mem46, align 8
  %172 = add nuw nsw i64 %.reload47, 1
  store i64 %172, ptr %.reg2mem, align 8
  %.reload45 = load i64, ptr %.reg2mem, align 8
  %173 = icmp eq i64 %.reload45, 400
  br i1 %173, label %174, label %._crit_edge44, !llvm.loop !33

._crit_edge44:                                    ; preds = %167
  %.reload = load i64, ptr %.reg2mem, align 8
  store i64 %.reload, ptr %.reg2mem250, align 8
  br label %160

174:                                              ; preds = %167
  %175 = load ptr, ptr @stderr, align 8, !tbaa !31
  %176 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %175, ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.2) #4
  %177 = load ptr, ptr @stderr, align 8, !tbaa !31
  %178 = tail call i32 (ptr, ptr, ...) @fprintf(ptr noundef %177, ptr noundef nonnull @.str.6) #4
  %.reload249 = load ptr, ptr %.reg2mem232, align 8
  tail call void @free(ptr noundef %.reload249) #4
  %.reload231 = load ptr, ptr %.reg2mem228, align 8
  tail call void @free(ptr noundef %.reload231) #4
  %.reload227 = load ptr, ptr %.reg2mem222, align 8
  tail call void @free(ptr noundef nonnull %.reload227) #4
  %.reload221 = load ptr, ptr %.reg2mem216, align 8
  tail call void @free(ptr noundef %.reload221) #4
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

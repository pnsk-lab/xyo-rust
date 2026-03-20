; ModuleID = './c/dtoa.c'
source_filename = "./c/dtoa.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%struct.JSDTOATempMem = type { [37 x i64] }

@dtoa_max_digits_table = internal unnamed_addr constant [35 x i8] c"6#\1C\18\16\14\13\12\11\11\10\10\0F\0F\0F\0E\0E\0E\0E\0E\0D\0D\0D\0D\0D\0D\0D\0C\0C\0C\0C\0C\0C\0C\0C", align 16
@.str.1 = private unnamed_addr constant [17 x i8] c"./c/./lib/dtoa.c\00", align 1
@__PRETTY_FUNCTION__.js_dtoa = private unnamed_addr constant [60 x i8] c"int js_dtoa(char *, double, int, int, int, JSDTOATempMem *)\00", align 1
@.str.2 = private unnamed_addr constant [9 x i8] c"Infinity\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"NaN\00", align 1
@.str.4 = private unnamed_addr constant [48 x i8] c"n_digits >= 0 && n_digits <= JS_DTOA_MAX_DIGITS\00", align 1
@.str.5 = private unnamed_addr constant [48 x i8] c"n_digits >= 1 && n_digits <= JS_DTOA_MAX_DIGITS\00", align 1
@atod_max_digits_table = internal unnamed_addr constant [35 x i8] c"@P 71-\15(&%#\22! \10\1F\1E\1E\1D\1D\1C\1C\1B\1B\1B\1A\1A\1A\1A\19\0C\19\19\18\18", align 16
@digits_per_limb_table = internal unnamed_addr constant [35 x i8] c" \14\10\0D\0C\0B\0A\0A\09\09\08\08\08\08\08\07\07\07\07\07\07\07\06\06\06\06\06\06\06\06\06\06\06\06\06", align 16
@radix_base_table = internal unnamed_addr constant [35 x i32] [i32 0, i32 -808182895, i32 0, i32 1220703125, i32 -2118184960, i32 1977326743, i32 1073741824, i32 -808182895, i32 1000000000, i32 -1937019605, i32 429981696, i32 815730721, i32 1475789056, i32 -1732076671, i32 0, i32 410338673, i32 612220032, i32 893871739, i32 1280000000, i32 1801088541, i32 -1800609408, i32 -890141849, i32 191102976, i32 244140625, i32 308915776, i32 387420489, i32 481890304, i32 594823321, i32 729000000, i32 887503681, i32 1073741824, i32 1291467969, i32 1544804416, i32 1838265625, i32 -2118184960], align 16
@max_exponent = internal unnamed_addr constant [35 x i16] [i16 1024, i16 647, i16 512, i16 442, i16 397, i16 365, i16 342, i16 324, i16 309, i16 297, i16 286, i16 277, i16 269, i16 263, i16 256, i16 251, i16 246, i16 242, i16 237, i16 234, i16 230, i16 227, i16 224, i16 221, i16 218, i16 216, i16 214, i16 211, i16 209, i16 207, i16 205, i16 203, i16 202, i16 200, i16 199], align 16
@min_exponent = internal unnamed_addr constant [35 x i16] [i16 -1075, i16 -679, i16 -538, i16 -463, i16 -416, i16 -383, i16 -359, i16 -340, i16 -324, i16 -311, i16 -300, i16 -291, i16 -283, i16 -276, i16 -269, i16 -263, i16 -258, i16 -254, i16 -249, i16 -245, i16 -242, i16 -238, i16 -235, i16 -232, i16 -229, i16 -227, i16 -224, i16 -222, i16 -220, i16 -217, i16 -215, i16 -214, i16 -212, i16 -210, i16 -208], align 16
@mul_log2_radix_table = internal unnamed_addr constant [35 x i32] [i32 0, i32 10585245, i32 0, i32 7225554, i32 6490313, i32 5976165, i32 0, i32 5292622, i32 5050445, i32 4849703, i32 4679886, i32 4533844, i32 4406528, i32 4294263, i32 0, i32 4104555, i32 4023386, i32 3949506, i32 3881882, i32 3819673, i32 3762187, i32 3708851, i32 3659183, i32 3612777, i32 3569286, i32 3528415, i32 3489906, i32 3453537, i32 3419114, i32 3386466, i32 0, i32 3325913, i32 3297757, i32 3270870, i32 3245157], align 16
@pow5_table = internal unnamed_addr constant [17 x i32] [i32 5, i32 25, i32 125, i32 625, i32 3125, i32 15625, i32 78125, i32 390625, i32 1953125, i32 9765625, i32 48828125, i32 244140625, i32 1220703125, i32 1808548329, i32 452807053, i32 -2030932031, i32 -1564725563], align 16
@pow5h_table = internal unnamed_addr constant [4 x i8] c"\01\07#\B1", align 1
@pow5_inv_table = internal unnamed_addr constant [13 x i32] [i32 -1717986919, i32 1202590842, i32 103079215, i32 -1553060175, i32 1334532238, i32 208632331, i32 -1384175189, i32 1469640227, i32 316718722, i32 -1211236963, i32 1607990807, i32 427399186, i32 -1034148220], align 16
@.str.7 = private unnamed_addr constant [32 x i8] c"shift >= 1 && shift < LIMB_BITS\00", align 1
@__PRETTY_FUNCTION__.mp_shl = private unnamed_addr constant [64 x i8] c"limb_t mp_shl(limb_t *, const limb_t *, mp_size_t, int, limb_t)\00", align 1

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @u32toa(ptr noundef writeonly captures(none) %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i8], align 1
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #10
  %4 = getelementptr inbounds nuw i8, ptr %3, i64 10
  br label %5

5:                                                ; preds = %5, %2
  %6 = phi ptr [ %4, %2 ], [ %11, %5 ]
  %7 = phi i32 [ %1, %2 ], [ %12, %5 ]
  %8 = urem i32 %7, 10
  %9 = trunc nuw nsw i32 %8 to i8
  %10 = or disjoint i8 %9, 48
  %11 = getelementptr inbounds i8, ptr %6, i64 -1
  store i8 %10, ptr %11, align 1, !tbaa !5
  %12 = udiv i32 %7, 10
  %13 = icmp ult i32 %7, 10
  br i1 %13, label %14, label %5, !llvm.loop !8

14:                                               ; preds = %5
  %15 = ptrtoint ptr %4 to i64
  %16 = ptrtoint ptr %11 to i64
  %17 = sub i64 %15, %16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %11, i64 %17, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #10
  ret i64 %17
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #1

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @i32toa(ptr noundef writeonly captures(none) %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i8], align 1
  %4 = alloca [10 x i8], align 1
  %5 = icmp sgt i32 %1, -1
  br i1 %5, label %6, label %21

6:                                                ; preds = %2
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %4) #10
  %7 = getelementptr inbounds nuw i8, ptr %4, i64 10
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi ptr [ %7, %6 ], [ %14, %8 ]
  %10 = phi i32 [ %1, %6 ], [ %15, %8 ]
  %11 = urem i32 %10, 10
  %12 = trunc nuw nsw i32 %11 to i8
  %13 = or disjoint i8 %12, 48
  %14 = getelementptr inbounds i8, ptr %9, i64 -1
  store i8 %13, ptr %14, align 1, !tbaa !5
  %15 = udiv i32 %10, 10
  %16 = icmp ult i32 %10, 10
  br i1 %16, label %17, label %8, !llvm.loop !8

17:                                               ; preds = %8
  %18 = ptrtoint ptr %7 to i64
  %19 = ptrtoint ptr %14 to i64
  %20 = sub i64 %18, %19
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %14, i64 %20, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %4) #10
  br label %39

21:                                               ; preds = %2
  store i8 45, ptr %0, align 1, !tbaa !5
  %22 = sub i32 0, %1
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #10
  %23 = getelementptr inbounds nuw i8, ptr %3, i64 10
  br label %24

24:                                               ; preds = %24, %21
  %25 = phi ptr [ %23, %21 ], [ %30, %24 ]
  %26 = phi i32 [ %22, %21 ], [ %31, %24 ]
  %27 = urem i32 %26, 10
  %28 = trunc nuw nsw i32 %27 to i8
  %29 = or disjoint i8 %28, 48
  %30 = getelementptr inbounds i8, ptr %25, i64 -1
  store i8 %29, ptr %30, align 1, !tbaa !5
  %31 = udiv i32 %26, 10
  %32 = icmp ult i32 %26, 10
  br i1 %32, label %33, label %24, !llvm.loop !8

33:                                               ; preds = %24
  %34 = getelementptr inbounds nuw i8, ptr %0, i64 1
  %35 = ptrtoint ptr %23 to i64
  %36 = ptrtoint ptr %30 to i64
  %37 = sub i64 %35, %36
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %34, ptr noundef nonnull align 1 dereferenceable(1) %30, i64 %37, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #10
  %38 = add i64 %37, 1
  br label %39

39:                                               ; preds = %33, %17
  %40 = phi i64 [ %20, %17 ], [ %38, %33 ]
  ret i64 %40
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @u64toa(ptr noundef %0, i64 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i8], align 1
  %4 = alloca [10 x i8], align 1
  %5 = icmp ult i64 %1, 4294967296
  br i1 %5, label %6, label %22

6:                                                ; preds = %2
  %7 = trunc nuw i64 %1 to i32
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %4) #10
  %8 = getelementptr inbounds nuw i8, ptr %4, i64 10
  br label %9

9:                                                ; preds = %9, %6
  %10 = phi ptr [ %8, %6 ], [ %15, %9 ]
  %11 = phi i32 [ %7, %6 ], [ %16, %9 ]
  %12 = urem i32 %11, 10
  %13 = trunc nuw nsw i32 %12 to i8
  %14 = or disjoint i8 %13, 48
  %15 = getelementptr inbounds i8, ptr %10, i64 -1
  store i8 %14, ptr %15, align 1, !tbaa !5
  %16 = udiv i32 %11, 10
  %17 = icmp ult i32 %11, 10
  br i1 %17, label %18, label %9, !llvm.loop !8

18:                                               ; preds = %9
  %19 = ptrtoint ptr %8 to i64
  %20 = ptrtoint ptr %15 to i64
  %21 = sub i64 %19, %20
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %15, i64 %21, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %4) #10
  br label %153

22:                                               ; preds = %2
  %23 = udiv i64 %1, 1000000000
  %24 = urem i64 %1, 1000000000
  %25 = icmp ugt i64 %1, 4294967295999999999
  br i1 %25, label %26, label %85

26:                                               ; preds = %22
  %27 = udiv i64 %1, 1000000000000000000
  %28 = trunc nuw nsw i64 %27 to i8
  %29 = urem i64 %23, 1000000000
  %30 = icmp ugt i64 %1, -8446744073709551617
  br i1 %30, label %31, label %34

31:                                               ; preds = %26
  %32 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 49, ptr %0, align 1, !tbaa !5
  %33 = add nsw i8 %28, -10
  br label %34

34:                                               ; preds = %31, %26
  %35 = phi ptr [ %32, %31 ], [ %0, %26 ]
  %36 = phi i8 [ %33, %31 ], [ %28, %26 ]
  %37 = add nuw nsw i8 %36, 48
  %38 = getelementptr inbounds nuw i8, ptr %35, i64 1
  store i8 %37, ptr %35, align 1, !tbaa !5
  %39 = trunc nuw nsw i64 %29 to i32
  %40 = urem i32 %39, 10
  %41 = udiv i32 %39, 10
  %42 = trunc nuw nsw i32 %40 to i8
  %43 = or disjoint i8 %42, 48
  %44 = getelementptr inbounds nuw i8, ptr %35, i64 9
  store i8 %43, ptr %44, align 1, !tbaa !5
  %45 = urem i32 %41, 10
  %46 = udiv i32 %39, 100
  %47 = trunc nuw nsw i32 %45 to i8
  %48 = or disjoint i8 %47, 48
  %49 = getelementptr inbounds nuw i8, ptr %35, i64 8
  store i8 %48, ptr %49, align 1, !tbaa !5
  %50 = urem i32 %46, 10
  %51 = udiv i32 %39, 1000
  %52 = trunc nuw nsw i32 %50 to i8
  %53 = or disjoint i8 %52, 48
  %54 = getelementptr inbounds nuw i8, ptr %35, i64 7
  store i8 %53, ptr %54, align 1, !tbaa !5
  %55 = urem i32 %51, 10
  %56 = udiv i32 %39, 10000
  %57 = trunc nuw nsw i32 %55 to i8
  %58 = or disjoint i8 %57, 48
  %59 = getelementptr inbounds nuw i8, ptr %35, i64 6
  store i8 %58, ptr %59, align 1, !tbaa !5
  %60 = urem i32 %56, 10
  %61 = udiv i32 %39, 100000
  %62 = trunc nuw nsw i32 %60 to i8
  %63 = or disjoint i8 %62, 48
  %64 = getelementptr inbounds nuw i8, ptr %35, i64 5
  store i8 %63, ptr %64, align 1, !tbaa !5
  %65 = trunc nuw nsw i32 %61 to i16
  %66 = urem i16 %65, 10
  %67 = udiv i32 %39, 1000000
  %68 = trunc nuw nsw i16 %66 to i8
  %69 = or disjoint i8 %68, 48
  %70 = getelementptr inbounds nuw i8, ptr %35, i64 4
  store i8 %69, ptr %70, align 1, !tbaa !5
  %71 = trunc nuw nsw i32 %67 to i16
  %72 = urem i16 %71, 10
  %73 = udiv i32 %39, 10000000
  %74 = trunc nuw nsw i16 %72 to i8
  %75 = or disjoint i8 %74, 48
  %76 = getelementptr inbounds nuw i8, ptr %35, i64 3
  store i8 %75, ptr %76, align 1, !tbaa !5
  %77 = trunc nuw nsw i32 %73 to i8
  %78 = urem i8 %77, 10
  %79 = udiv i32 %39, 100000000
  %80 = or disjoint i8 %78, 48
  %81 = getelementptr inbounds nuw i8, ptr %35, i64 2
  store i8 %80, ptr %81, align 1, !tbaa !5
  %82 = trunc nuw nsw i32 %79 to i8
  %83 = or disjoint i8 %82, 48
  store i8 %83, ptr %38, align 1, !tbaa !5
  %84 = getelementptr inbounds nuw i8, ptr %35, i64 10
  br label %102

85:                                               ; preds = %22
  %86 = trunc nuw i64 %23 to i32
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #10
  %87 = getelementptr inbounds nuw i8, ptr %3, i64 10
  br label %88

88:                                               ; preds = %88, %85
  %89 = phi ptr [ %87, %85 ], [ %94, %88 ]
  %90 = phi i32 [ %86, %85 ], [ %95, %88 ]
  %91 = urem i32 %90, 10
  %92 = trunc nuw nsw i32 %91 to i8
  %93 = or disjoint i8 %92, 48
  %94 = getelementptr inbounds i8, ptr %89, i64 -1
  store i8 %93, ptr %94, align 1, !tbaa !5
  %95 = udiv i32 %90, 10
  %96 = icmp ult i32 %90, 10
  br i1 %96, label %97, label %88, !llvm.loop !8

97:                                               ; preds = %88
  %98 = ptrtoint ptr %87 to i64
  %99 = ptrtoint ptr %94 to i64
  %100 = sub i64 %98, %99
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %94, i64 %100, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #10
  %101 = getelementptr inbounds nuw i8, ptr %0, i64 %100
  br label %102

102:                                              ; preds = %97, %34
  %103 = phi ptr [ %84, %34 ], [ %101, %97 ]
  %104 = trunc nuw nsw i64 %24 to i32
  %105 = urem i32 %104, 10
  %106 = udiv i32 %104, 10
  %107 = trunc nuw nsw i32 %105 to i8
  %108 = or disjoint i8 %107, 48
  %109 = getelementptr inbounds nuw i8, ptr %103, i64 8
  store i8 %108, ptr %109, align 1, !tbaa !5
  %110 = urem i32 %106, 10
  %111 = udiv i32 %104, 100
  %112 = trunc nuw nsw i32 %110 to i8
  %113 = or disjoint i8 %112, 48
  %114 = getelementptr inbounds nuw i8, ptr %103, i64 7
  store i8 %113, ptr %114, align 1, !tbaa !5
  %115 = urem i32 %111, 10
  %116 = udiv i32 %104, 1000
  %117 = trunc nuw nsw i32 %115 to i8
  %118 = or disjoint i8 %117, 48
  %119 = getelementptr inbounds nuw i8, ptr %103, i64 6
  store i8 %118, ptr %119, align 1, !tbaa !5
  %120 = urem i32 %116, 10
  %121 = udiv i32 %104, 10000
  %122 = trunc nuw nsw i32 %120 to i8
  %123 = or disjoint i8 %122, 48
  %124 = getelementptr inbounds nuw i8, ptr %103, i64 5
  store i8 %123, ptr %124, align 1, !tbaa !5
  %125 = urem i32 %121, 10
  %126 = udiv i32 %104, 100000
  %127 = trunc nuw nsw i32 %125 to i8
  %128 = or disjoint i8 %127, 48
  %129 = getelementptr inbounds nuw i8, ptr %103, i64 4
  store i8 %128, ptr %129, align 1, !tbaa !5
  %130 = trunc nuw nsw i32 %126 to i16
  %131 = urem i16 %130, 10
  %132 = udiv i32 %104, 1000000
  %133 = trunc nuw nsw i16 %131 to i8
  %134 = or disjoint i8 %133, 48
  %135 = getelementptr inbounds nuw i8, ptr %103, i64 3
  store i8 %134, ptr %135, align 1, !tbaa !5
  %136 = trunc nuw nsw i32 %132 to i16
  %137 = urem i16 %136, 10
  %138 = udiv i32 %104, 10000000
  %139 = trunc nuw nsw i16 %137 to i8
  %140 = or disjoint i8 %139, 48
  %141 = getelementptr inbounds nuw i8, ptr %103, i64 2
  store i8 %140, ptr %141, align 1, !tbaa !5
  %142 = trunc nuw nsw i32 %138 to i8
  %143 = urem i8 %142, 10
  %144 = udiv i32 %104, 100000000
  %145 = or disjoint i8 %143, 48
  %146 = getelementptr inbounds nuw i8, ptr %103, i64 1
  store i8 %145, ptr %146, align 1, !tbaa !5
  %147 = trunc nuw nsw i32 %144 to i8
  %148 = or disjoint i8 %147, 48
  store i8 %148, ptr %103, align 1, !tbaa !5
  %149 = getelementptr inbounds nuw i8, ptr %103, i64 9
  %150 = ptrtoint ptr %149 to i64
  %151 = ptrtoint ptr %0 to i64
  %152 = sub i64 %150, %151
  br label %153

153:                                              ; preds = %102, %18
  %154 = phi i64 [ %21, %18 ], [ %152, %102 ]
  ret i64 %154
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @i64toa(ptr noundef %0, i64 noundef %1) local_unnamed_addr #0 {
  %3 = icmp sgt i64 %1, -1
  br i1 %3, label %4, label %6

4:                                                ; preds = %2
  %5 = tail call i64 @u64toa(ptr noundef %0, i64 noundef %1)
  br label %11

6:                                                ; preds = %2
  store i8 45, ptr %0, align 1, !tbaa !5
  %7 = getelementptr inbounds nuw i8, ptr %0, i64 1
  %8 = sub i64 0, %1
  %9 = tail call i64 @u64toa(ptr noundef nonnull %7, i64 noundef %8)
  %10 = add i64 %9, 1
  br label %11

11:                                               ; preds = %6, %4
  %12 = phi i64 [ %5, %4 ], [ %10, %6 ]
  ret i64 %12
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @u64toa_radix(ptr noundef %0, i64 noundef %1, i32 noundef %2) local_unnamed_addr #0 {
  %4 = alloca [41 x i8], align 16
  %5 = icmp eq i32 %2, 10
  br i1 %5, label %6, label %8

6:                                                ; preds = %3
  %7 = tail call i64 @u64toa(ptr noundef %0, i64 noundef %1)
  br label %93

8:                                                ; preds = %3
  %9 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %10 = icmp samesign ult i32 %9, 2
  br i1 %10, label %11, label %74

11:                                               ; preds = %8
  %12 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %13 = sub nsw i32 31, %12
  %14 = icmp eq i64 %1, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %11
  store i8 48, ptr %0, align 1, !tbaa !5
  br label %93

16:                                               ; preds = %11
  %17 = lshr i64 %1, 1
  %18 = tail call range(i64 1, 65) i64 @llvm.ctlz.i64(i64 %17, i1 false)
  %19 = trunc nuw nsw i64 %18 to i32
  %20 = sub nsw i32 %13, %19
  %21 = trunc nsw i32 %20 to i8
  %22 = add nsw i8 %21, 64
  %23 = trunc nsw i32 %13 to i8
  %24 = sdiv i8 %22, %23
  %25 = shl nsw i32 -1, %13
  %26 = xor i32 %25, -1
  %27 = icmp sgt i8 %24, 0
  br i1 %27, label %28, label %72

28:                                               ; preds = %16
  %29 = zext nneg i32 %13 to i64
  %30 = zext nneg i8 %24 to i64
  %31 = and i64 %30, 1
  %32 = icmp eq i64 %31, 0
  br i1 %32, label %44, label %33

33:                                               ; preds = %28
  %34 = add nsw i64 %30, -1
  %35 = trunc i64 %1 to i32
  %36 = and i32 %35, %26
  %37 = lshr i64 %1, %29
  %38 = icmp samesign ult i32 %36, 10
  %39 = or disjoint i32 %36, 48
  %40 = add nuw nsw i32 %36, 87
  %41 = select i1 %38, i32 %39, i32 %40
  %42 = trunc i32 %41 to i8
  %43 = getelementptr inbounds nuw i8, ptr %0, i64 %34
  store i8 %42, ptr %43, align 1, !tbaa !5
  br label %44

44:                                               ; preds = %33, %28
  %45 = phi i64 [ %30, %28 ], [ %34, %33 ]
  %46 = phi i64 [ %1, %28 ], [ %37, %33 ]
  %47 = icmp eq i8 %24, 1
  br i1 %47, label %72, label %48

48:                                               ; preds = %44, %48
  %49 = phi i64 [ %61, %48 ], [ %45, %44 ]
  %50 = phi i64 [ %64, %48 ], [ %46, %44 ]
  %51 = add nsw i64 %49, -1
  %52 = trunc i64 %50 to i32
  %53 = and i32 %52, %26
  %54 = lshr i64 %50, %29
  %55 = icmp samesign ult i32 %53, 10
  %56 = or disjoint i32 %53, 48
  %57 = add nuw nsw i32 %53, 87
  %58 = select i1 %55, i32 %56, i32 %57
  %59 = trunc i32 %58 to i8
  %60 = getelementptr inbounds nuw i8, ptr %0, i64 %51
  store i8 %59, ptr %60, align 1, !tbaa !5
  %61 = add nsw i64 %49, -2
  %62 = trunc i64 %54 to i32
  %63 = and i32 %62, %26
  %64 = lshr i64 %54, %29
  %65 = icmp samesign ult i32 %63, 10
  %66 = or disjoint i32 %63, 48
  %67 = add nuw nsw i32 %63, 87
  %68 = select i1 %65, i32 %66, i32 %67
  %69 = trunc i32 %68 to i8
  %70 = getelementptr inbounds nuw i8, ptr %0, i64 %61
  store i8 %69, ptr %70, align 1, !tbaa !5
  %71 = icmp samesign ugt i64 %51, 1
  br i1 %71, label %48, label %72, !llvm.loop !10

72:                                               ; preds = %44, %48, %16
  %73 = sext i8 %24 to i64
  br label %93

74:                                               ; preds = %8
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %4) #10
  %75 = getelementptr inbounds nuw i8, ptr %4, i64 41
  %76 = zext i32 %2 to i64
  br label %77

77:                                               ; preds = %77, %74
  %78 = phi i64 [ %1, %74 ], [ %82, %77 ]
  %79 = phi ptr [ %75, %74 ], [ %87, %77 ]
  %80 = urem i64 %78, %76
  %81 = trunc nuw i64 %80 to i32
  %82 = udiv i64 %78, %76
  %83 = icmp slt i32 %81, 10
  %84 = select i1 %83, i32 48, i32 87
  %85 = add nsw i32 %84, %81
  %86 = trunc i32 %85 to i8
  %87 = getelementptr inbounds i8, ptr %79, i64 -1
  store i8 %86, ptr %87, align 1, !tbaa !5
  %88 = icmp ult i64 %78, %76
  br i1 %88, label %89, label %77, !llvm.loop !11

89:                                               ; preds = %77
  %90 = ptrtoint ptr %75 to i64
  %91 = ptrtoint ptr %87 to i64
  %92 = sub i64 %90, %91
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %87, i64 %92, i1 false)
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %4) #10
  br label %93

93:                                               ; preds = %15, %72, %89, %6
  %94 = phi i64 [ %7, %6 ], [ %92, %89 ], [ %73, %72 ], [ 1, %15 ]
  ret i64 %94
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @i64toa_radix(ptr noundef %0, i64 noundef %1, i32 noundef %2) local_unnamed_addr #0 {
  %4 = alloca [41 x i8], align 16
  %5 = alloca [41 x i8], align 16
  %6 = icmp sgt i64 %1, -1
  br i1 %6, label %7, label %96

7:                                                ; preds = %3
  %8 = icmp eq i32 %2, 10
  br i1 %8, label %9, label %11

9:                                                ; preds = %7
  %10 = tail call i64 @u64toa(ptr noundef %0, i64 noundef %1)
  br label %187

11:                                               ; preds = %7
  %12 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %13 = icmp samesign ult i32 %12, 2
  br i1 %13, label %14, label %77

14:                                               ; preds = %11
  %15 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %16 = sub nsw i32 31, %15
  %17 = icmp eq i64 %1, 0
  br i1 %17, label %18, label %19

18:                                               ; preds = %14
  store i8 48, ptr %0, align 1, !tbaa !5
  br label %187

19:                                               ; preds = %14
  %20 = lshr i64 %1, 1
  %21 = tail call range(i64 1, 65) i64 @llvm.ctlz.i64(i64 %20, i1 false)
  %22 = trunc nuw nsw i64 %21 to i32
  %23 = sub nsw i32 %16, %22
  %24 = trunc nsw i32 %23 to i8
  %25 = add nsw i8 %24, 64
  %26 = trunc nsw i32 %16 to i8
  %27 = sdiv i8 %25, %26
  %28 = shl nsw i32 -1, %16
  %29 = xor i32 %28, -1
  %30 = icmp sgt i8 %27, 0
  br i1 %30, label %31, label %75

31:                                               ; preds = %19
  %32 = zext nneg i32 %16 to i64
  %33 = zext nneg i8 %27 to i64
  %34 = and i64 %33, 1
  %35 = icmp eq i64 %34, 0
  br i1 %35, label %47, label %36

36:                                               ; preds = %31
  %37 = add nsw i64 %33, -1
  %38 = trunc i64 %1 to i32
  %39 = and i32 %38, %29
  %40 = lshr i64 %1, %32
  %41 = icmp samesign ult i32 %39, 10
  %42 = or disjoint i32 %39, 48
  %43 = add nuw nsw i32 %39, 87
  %44 = select i1 %41, i32 %42, i32 %43
  %45 = trunc i32 %44 to i8
  %46 = getelementptr inbounds nuw i8, ptr %0, i64 %37
  store i8 %45, ptr %46, align 1, !tbaa !5
  br label %47

47:                                               ; preds = %36, %31
  %48 = phi i64 [ %33, %31 ], [ %37, %36 ]
  %49 = phi i64 [ %1, %31 ], [ %40, %36 ]
  %50 = icmp eq i8 %27, 1
  br i1 %50, label %75, label %51

51:                                               ; preds = %47, %51
  %52 = phi i64 [ %64, %51 ], [ %48, %47 ]
  %53 = phi i64 [ %67, %51 ], [ %49, %47 ]
  %54 = add nsw i64 %52, -1
  %55 = trunc i64 %53 to i32
  %56 = and i32 %55, %29
  %57 = lshr i64 %53, %32
  %58 = icmp samesign ult i32 %56, 10
  %59 = or disjoint i32 %56, 48
  %60 = add nuw nsw i32 %56, 87
  %61 = select i1 %58, i32 %59, i32 %60
  %62 = trunc i32 %61 to i8
  %63 = getelementptr inbounds nuw i8, ptr %0, i64 %54
  store i8 %62, ptr %63, align 1, !tbaa !5
  %64 = add nsw i64 %52, -2
  %65 = trunc i64 %57 to i32
  %66 = and i32 %65, %29
  %67 = lshr i64 %57, %32
  %68 = icmp samesign ult i32 %66, 10
  %69 = or disjoint i32 %66, 48
  %70 = add nuw nsw i32 %66, 87
  %71 = select i1 %68, i32 %69, i32 %70
  %72 = trunc i32 %71 to i8
  %73 = getelementptr inbounds nuw i8, ptr %0, i64 %64
  store i8 %72, ptr %73, align 1, !tbaa !5
  %74 = icmp samesign ugt i64 %54, 1
  br i1 %74, label %51, label %75, !llvm.loop !10

75:                                               ; preds = %47, %51, %19
  %76 = sext i8 %27 to i64
  br label %187

77:                                               ; preds = %11
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %5) #10
  %78 = getelementptr inbounds nuw i8, ptr %5, i64 41
  %79 = zext i32 %2 to i64
  br label %80

80:                                               ; preds = %80, %77
  %81 = phi i64 [ %1, %77 ], [ %85, %80 ]
  %82 = phi ptr [ %78, %77 ], [ %90, %80 ]
  %83 = urem i64 %81, %79
  %84 = trunc nuw i64 %83 to i32
  %85 = udiv i64 %81, %79
  %86 = icmp slt i32 %84, 10
  %87 = select i1 %86, i32 48, i32 87
  %88 = add nsw i32 %87, %84
  %89 = trunc i32 %88 to i8
  %90 = getelementptr inbounds i8, ptr %82, i64 -1
  store i8 %89, ptr %90, align 1, !tbaa !5
  %91 = icmp ult i64 %81, %79
  br i1 %91, label %92, label %80, !llvm.loop !11

92:                                               ; preds = %80
  %93 = ptrtoint ptr %78 to i64
  %94 = ptrtoint ptr %90 to i64
  %95 = sub i64 %93, %94
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %0, ptr noundef nonnull align 1 dereferenceable(1) %90, i64 %95, i1 false)
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %5) #10
  br label %187

96:                                               ; preds = %3
  store i8 45, ptr %0, align 1, !tbaa !5
  %97 = getelementptr inbounds nuw i8, ptr %0, i64 1
  %98 = sub i64 0, %1
  %99 = icmp eq i32 %2, 10
  br i1 %99, label %100, label %102

100:                                              ; preds = %96
  %101 = tail call i64 @u64toa(ptr noundef nonnull %97, i64 noundef %98)
  br label %184

102:                                              ; preds = %96
  %103 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %104 = icmp samesign ult i32 %103, 2
  br i1 %104, label %105, label %165

105:                                              ; preds = %102
  %106 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %107 = sub nsw i32 31, %106
  %108 = lshr i64 %98, 1
  %109 = tail call range(i64 1, 65) i64 @llvm.ctlz.i64(i64 %108, i1 false)
  %110 = trunc nuw nsw i64 %109 to i32
  %111 = sub nsw i32 %107, %110
  %112 = trunc nsw i32 %111 to i8
  %113 = add nsw i8 %112, 64
  %114 = trunc nsw i32 %107 to i8
  %115 = sdiv i8 %113, %114
  %116 = shl nsw i32 -1, %107
  %117 = xor i32 %116, -1
  %118 = icmp sgt i8 %115, 0
  br i1 %118, label %119, label %163

119:                                              ; preds = %105
  %120 = zext nneg i32 %107 to i64
  %121 = zext nneg i8 %115 to i64
  %122 = and i64 %121, 1
  %123 = icmp eq i64 %122, 0
  br i1 %123, label %135, label %124

124:                                              ; preds = %119
  %125 = add nsw i64 %121, -1
  %126 = trunc i64 %98 to i32
  %127 = and i32 %126, %117
  %128 = lshr i64 %98, %120
  %129 = icmp samesign ult i32 %127, 10
  %130 = or disjoint i32 %127, 48
  %131 = add nuw nsw i32 %127, 87
  %132 = select i1 %129, i32 %130, i32 %131
  %133 = trunc i32 %132 to i8
  %134 = getelementptr i8, ptr %0, i64 %121
  store i8 %133, ptr %134, align 1, !tbaa !5
  br label %135

135:                                              ; preds = %124, %119
  %136 = phi i64 [ %121, %119 ], [ %125, %124 ]
  %137 = phi i64 [ %98, %119 ], [ %128, %124 ]
  %138 = icmp eq i8 %115, 1
  br i1 %138, label %163, label %139

139:                                              ; preds = %135, %139
  %140 = phi i64 [ %152, %139 ], [ %136, %135 ]
  %141 = phi i64 [ %155, %139 ], [ %137, %135 ]
  %142 = add nsw i64 %140, -1
  %143 = trunc i64 %141 to i32
  %144 = and i32 %143, %117
  %145 = lshr i64 %141, %120
  %146 = icmp samesign ult i32 %144, 10
  %147 = or disjoint i32 %144, 48
  %148 = add nuw nsw i32 %144, 87
  %149 = select i1 %146, i32 %147, i32 %148
  %150 = trunc i32 %149 to i8
  %151 = getelementptr i8, ptr %0, i64 %140
  store i8 %150, ptr %151, align 1, !tbaa !5
  %152 = add nsw i64 %140, -2
  %153 = trunc i64 %145 to i32
  %154 = and i32 %153, %117
  %155 = lshr i64 %145, %120
  %156 = icmp samesign ult i32 %154, 10
  %157 = or disjoint i32 %154, 48
  %158 = add nuw nsw i32 %154, 87
  %159 = select i1 %156, i32 %157, i32 %158
  %160 = trunc i32 %159 to i8
  %161 = getelementptr i8, ptr %0, i64 %142
  store i8 %160, ptr %161, align 1, !tbaa !5
  %162 = icmp samesign ugt i64 %142, 1
  br i1 %162, label %139, label %163, !llvm.loop !10

163:                                              ; preds = %135, %139, %105
  %164 = sext i8 %115 to i64
  br label %184

165:                                              ; preds = %102
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %4) #10
  %166 = getelementptr inbounds nuw i8, ptr %4, i64 41
  %167 = zext i32 %2 to i64
  br label %168

168:                                              ; preds = %168, %165
  %169 = phi i64 [ %98, %165 ], [ %173, %168 ]
  %170 = phi ptr [ %166, %165 ], [ %178, %168 ]
  %171 = urem i64 %169, %167
  %172 = trunc nuw i64 %171 to i32
  %173 = udiv i64 %169, %167
  %174 = icmp slt i32 %172, 10
  %175 = select i1 %174, i32 48, i32 87
  %176 = add nsw i32 %175, %172
  %177 = trunc i32 %176 to i8
  %178 = getelementptr inbounds i8, ptr %170, i64 -1
  store i8 %177, ptr %178, align 1, !tbaa !5
  %179 = icmp ult i64 %169, %167
  br i1 %179, label %180, label %168, !llvm.loop !11

180:                                              ; preds = %168
  %181 = ptrtoint ptr %166 to i64
  %182 = ptrtoint ptr %178 to i64
  %183 = sub i64 %181, %182
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %97, ptr noundef nonnull align 1 dereferenceable(1) %178, i64 %183, i1 false)
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %4) #10
  br label %184

184:                                              ; preds = %100, %163, %180
  %185 = phi i64 [ %101, %100 ], [ %183, %180 ], [ %164, %163 ]
  %186 = add i64 %185, 1
  br label %187

187:                                              ; preds = %92, %75, %18, %9, %184
  %188 = phi i64 [ %186, %184 ], [ %10, %9 ], [ %95, %92 ], [ %76, %75 ], [ 1, %18 ]
  ret i64 %188
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define dso_local range(i32 9, -2147483648) i32 @js_dtoa_max_len(double noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #3 {
  %5 = and i32 %3, 3
  switch i32 %5, label %12 [
    i32 2, label %54
    i32 0, label %6
  ]

6:                                                ; preds = %4
  %7 = add nsw i32 %1, -2
  %8 = sext i32 %7 to i64
  %9 = getelementptr inbounds [35 x i8], ptr @dtoa_max_digits_table, i64 0, i64 %8
  %10 = load i8, ptr %9, align 1, !tbaa !5
  %11 = zext i8 %10 to i32
  br label %12

12:                                               ; preds = %4, %6
  %13 = phi i32 [ %11, %6 ], [ %2, %4 ]
  %14 = and i32 %3, 12
  %15 = icmp eq i32 %14, 8
  br i1 %15, label %16, label %52

16:                                               ; preds = %12
  %17 = bitcast double %0 to i64
  %18 = lshr i64 %17, 52
  %19 = trunc nuw nsw i64 %18 to i32
  %20 = and i32 %19, 2047
  %21 = icmp eq i32 %20, 2047
  br i1 %21, label %94, label %22

22:                                               ; preds = %16
  %23 = add nsw i32 %20, -1024
  %24 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %1)
  %25 = icmp samesign ult i32 %24, 2
  br i1 %25, label %26, label %37

26:                                               ; preds = %22
  %27 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %1, i1 false)
  %28 = icmp samesign ult i32 %20, 1024
  %29 = add nuw nsw i32 %27, 65506
  %30 = select i1 %28, i32 %29, i32 0
  %31 = add nsw i32 %30, %23
  %32 = trunc i32 %31 to i16
  %33 = trunc nuw nsw i32 %27 to i16
  %34 = sub nsw i16 31, %33
  %35 = sdiv i16 %32, %34
  %36 = sext i16 %35 to i32
  br label %47

37:                                               ; preds = %22
  %38 = add nsw i32 %1, -2
  %39 = sext i32 %38 to i64
  %40 = getelementptr inbounds [35 x i32], ptr @mul_log2_radix_table, i64 0, i64 %39
  %41 = load i32, ptr %40, align 4, !tbaa !12
  %42 = sext i32 %23 to i64
  %43 = sext i32 %41 to i64
  %44 = mul nsw i64 %43, %42
  %45 = lshr i64 %44, 24
  %46 = trunc i64 %45 to i32
  br label %47

47:                                               ; preds = %26, %37
  %48 = phi i32 [ %36, %26 ], [ %46, %37 ]
  %49 = tail call i32 @llvm.abs.i32(i32 %48, i1 true)
  %50 = add i32 %13, 10
  %51 = add i32 %50, %49
  br label %94

52:                                               ; preds = %12
  %53 = add nsw i32 %13, 8
  br label %94

54:                                               ; preds = %4
  %55 = bitcast double %0 to i64
  %56 = lshr i64 %55, 52
  %57 = trunc nuw nsw i64 %56 to i32
  %58 = and i32 %57, 2047
  %59 = icmp eq i32 %58, 2047
  br i1 %59, label %94, label %60

60:                                               ; preds = %54
  %61 = icmp samesign ult i32 %58, 1023
  br i1 %61, label %90, label %62

62:                                               ; preds = %60
  %63 = add nsw i32 %58, -1024
  %64 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %1)
  %65 = icmp samesign ult i32 %64, 2
  br i1 %65, label %66, label %77

66:                                               ; preds = %62
  %67 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %1, i1 false)
  %68 = icmp eq i32 %58, 1023
  %69 = add nuw nsw i32 %67, 65506
  %70 = select i1 %68, i32 %69, i32 0
  %71 = add nsw i32 %70, %63
  %72 = trunc i32 %71 to i16
  %73 = trunc nuw nsw i32 %67 to i16
  %74 = sub nsw i16 31, %73
  %75 = sdiv i16 %72, %74
  %76 = sext i16 %75 to i32
  br label %87

77:                                               ; preds = %62
  %78 = add nsw i32 %1, -2
  %79 = sext i32 %78 to i64
  %80 = getelementptr inbounds [35 x i32], ptr @mul_log2_radix_table, i64 0, i64 %79
  %81 = load i32, ptr %80, align 4, !tbaa !12
  %82 = sext i32 %63 to i64
  %83 = sext i32 %81 to i64
  %84 = mul nsw i64 %83, %82
  %85 = lshr i64 %84, 24
  %86 = trunc i64 %85 to i32
  br label %87

87:                                               ; preds = %66, %77
  %88 = phi i32 [ %76, %66 ], [ %86, %77 ]
  %89 = add nsw i32 %88, 2
  br label %90

90:                                               ; preds = %60, %87
  %91 = phi i32 [ %89, %87 ], [ 1, %60 ]
  %92 = add nsw i32 %2, 3
  %93 = add nsw i32 %92, %91
  br label %94

94:                                               ; preds = %54, %16, %90, %52, %47
  %95 = phi i32 [ %51, %47 ], [ %53, %52 ], [ %93, %90 ], [ 0, %16 ], [ 0, %54 ]
  %96 = tail call range(i32 0, -2147483648) i32 @llvm.smax.i32(i32 %95, i32 9)
  ret i32 %96
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.abs.i32(i32, i1 immarg) #4

; Function Attrs: nounwind uwtable
define dso_local noundef i32 @js_dtoa(ptr noundef %0, double noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, ptr noundef captures(none) %5) local_unnamed_addr #5 {
  %7 = alloca [10 x i8], align 1
  %8 = alloca [41 x i8], align 16
  %9 = and i32 %4, 3
  %10 = getelementptr inbounds nuw i8, ptr %5, i64 216
  %11 = icmp ne i32 %2, 0
  %12 = and i32 %2, 1
  %13 = icmp eq i32 %12, 0
  %14 = and i1 %11, %13
  br i1 %14, label %15, label %24

15:                                               ; preds = %6, %15
  %16 = phi i32 [ %19, %15 ], [ 0, %6 ]
  %17 = phi i32 [ %18, %15 ], [ %2, %6 ]
  %18 = lshr exact i32 %17, 1
  %19 = add nuw nsw i32 %16, 1
  %20 = icmp ne i32 %17, 0
  %21 = and i32 %17, 2
  %22 = icmp eq i32 %21, 0
  %23 = and i1 %20, %22
  br i1 %23, label %15, label %24, !llvm.loop !14

24:                                               ; preds = %15, %6
  %25 = phi i32 [ 0, %6 ], [ %19, %15 ]
  %26 = ashr i32 %2, %25
  %27 = bitcast double %1 to i64
  %28 = lshr i64 %27, 52
  %29 = trunc nuw nsw i64 %28 to i32
  %30 = and i32 %29, 2047
  %31 = and i64 %27, 4503599627370495
  switch i32 %30, label %66 [
    i32 2047, label %32
    i32 0, label %43
  ]

32:                                               ; preds = %24
  %33 = icmp eq i64 %31, 0
  br i1 %33, label %34, label %41

34:                                               ; preds = %32
  %35 = icmp sgt i64 %27, -1
  br i1 %35, label %38, label %36

36:                                               ; preds = %34
  %37 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 45, ptr %0, align 1, !tbaa !5
  br label %38

38:                                               ; preds = %36, %34
  %39 = phi ptr [ %37, %36 ], [ %0, %34 ]
  store i64 8751735898823355977, ptr %39, align 1
  %40 = getelementptr inbounds nuw i8, ptr %39, i64 8
  br label %555

41:                                               ; preds = %32
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %0, ptr noundef nonnull align 1 dereferenceable(3) @.str.3, i64 3, i1 false)
  %42 = getelementptr inbounds nuw i8, ptr %0, i64 3
  br label %555

43:                                               ; preds = %24
  %44 = icmp eq i64 %31, 0
  br i1 %44, label %45, label %58

45:                                               ; preds = %43
  store i32 1, ptr %5, align 4, !tbaa !12
  %46 = getelementptr inbounds nuw i8, ptr %5, i64 4
  store i32 0, ptr %46, align 4, !tbaa !12
  switch i32 %9, label %49 [
    i32 0, label %50
    i32 2, label %47
  ]

47:                                               ; preds = %45
  %48 = add nsw i32 %3, 1
  br label %50

49:                                               ; preds = %45
  br label %50

50:                                               ; preds = %45, %47, %49
  %51 = phi i32 [ %48, %47 ], [ %3, %49 ], [ 1, %45 ]
  %52 = icmp sgt i64 %27, -1
  %53 = and i32 %4, 16
  %54 = icmp eq i32 %53, 0
  %55 = or i1 %52, %54
  br i1 %55, label %468, label %56

56:                                               ; preds = %50
  %57 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 45, ptr %0, align 1, !tbaa !5
  br label %468

58:                                               ; preds = %43
  %59 = lshr i64 %31, 1
  %60 = tail call range(i64 1, 65) i64 @llvm.ctlz.i64(i64 %59, i1 false)
  %61 = trunc nuw nsw i64 %60 to i32
  %62 = add nuw nsw i64 %60, 4294967284
  %63 = sub nsw i32 13, %61
  %64 = and i64 %62, 4294967295
  %65 = shl i64 %31, %64
  br label %68

66:                                               ; preds = %24
  %67 = or disjoint i64 %31, 4503599627370496
  br label %68

68:                                               ; preds = %58, %66
  %69 = phi i32 [ %63, %58 ], [ %30, %66 ]
  %70 = phi i64 [ %65, %58 ], [ %67, %66 ]
  %71 = icmp sgt i64 %27, -1
  br i1 %71, label %74, label %72

72:                                               ; preds = %68
  %73 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 45, ptr %0, align 1, !tbaa !5
  br label %74

74:                                               ; preds = %72, %68
  %75 = phi ptr [ %73, %72 ], [ %0, %68 ]
  %76 = add nsw i32 %69, -1022
  %77 = icmp eq i32 %9, 0
  %78 = add nsw i32 %69, -1023
  %79 = icmp ult i32 %78, 53
  %80 = select i1 %77, i1 %79, i1 false
  br i1 %80, label %81, label %184

81:                                               ; preds = %74
  %82 = sub nuw nsw i32 1075, %69
  %83 = zext nneg i32 %82 to i64
  %84 = shl nsw i64 -1, %83
  %85 = xor i64 %84, -1
  %86 = and i64 %70, %85
  %87 = icmp ne i64 %86, 0
  %88 = and i32 %4, 12
  %89 = icmp eq i32 %88, 4
  %90 = or i1 %89, %87
  br i1 %90, label %184, label %91

91:                                               ; preds = %81
  %92 = lshr i64 %70, %83
  %93 = icmp eq i32 %2, 10
  br i1 %93, label %94, label %96

94:                                               ; preds = %91
  %95 = tail call i64 @u64toa(ptr noundef %75, i64 noundef %92)
  br label %181

96:                                               ; preds = %91
  %97 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %98 = icmp samesign ult i32 %97, 2
  br i1 %98, label %99, label %162

99:                                               ; preds = %96
  %100 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %101 = sub nsw i32 31, %100
  %102 = icmp eq i64 %92, 0
  br i1 %102, label %103, label %104

103:                                              ; preds = %99
  store i8 48, ptr %75, align 1, !tbaa !5
  br label %181

104:                                              ; preds = %99
  %105 = lshr i64 %92, 1
  %106 = tail call range(i64 1, 65) i64 @llvm.ctlz.i64(i64 %105, i1 false)
  %107 = trunc nuw nsw i64 %106 to i32
  %108 = sub nsw i32 %101, %107
  %109 = trunc nsw i32 %108 to i8
  %110 = add nsw i8 %109, 64
  %111 = trunc nsw i32 %101 to i8
  %112 = sdiv i8 %110, %111
  %113 = shl nsw i32 -1, %101
  %114 = xor i32 %113, -1
  %115 = icmp sgt i8 %112, 0
  br i1 %115, label %116, label %160

116:                                              ; preds = %104
  %117 = zext nneg i32 %101 to i64
  %118 = zext nneg i8 %112 to i64
  %119 = and i64 %118, 1
  %120 = icmp eq i64 %119, 0
  br i1 %120, label %132, label %121

121:                                              ; preds = %116
  %122 = add nsw i64 %118, -1
  %123 = trunc i64 %92 to i32
  %124 = and i32 %123, %114
  %125 = lshr i64 %92, %117
  %126 = icmp samesign ult i32 %124, 10
  %127 = or disjoint i32 %124, 48
  %128 = add nuw nsw i32 %124, 87
  %129 = select i1 %126, i32 %127, i32 %128
  %130 = trunc i32 %129 to i8
  %131 = getelementptr inbounds nuw i8, ptr %75, i64 %122
  store i8 %130, ptr %131, align 1, !tbaa !5
  br label %132

132:                                              ; preds = %121, %116
  %133 = phi i64 [ %118, %116 ], [ %122, %121 ]
  %134 = phi i64 [ %92, %116 ], [ %125, %121 ]
  %135 = icmp eq i8 %112, 1
  br i1 %135, label %160, label %136

136:                                              ; preds = %132, %136
  %137 = phi i64 [ %149, %136 ], [ %133, %132 ]
  %138 = phi i64 [ %152, %136 ], [ %134, %132 ]
  %139 = add nsw i64 %137, -1
  %140 = trunc i64 %138 to i32
  %141 = and i32 %140, %114
  %142 = lshr i64 %138, %117
  %143 = icmp samesign ult i32 %141, 10
  %144 = or disjoint i32 %141, 48
  %145 = add nuw nsw i32 %141, 87
  %146 = select i1 %143, i32 %144, i32 %145
  %147 = trunc i32 %146 to i8
  %148 = getelementptr inbounds nuw i8, ptr %75, i64 %139
  store i8 %147, ptr %148, align 1, !tbaa !5
  %149 = add nsw i64 %137, -2
  %150 = trunc i64 %142 to i32
  %151 = and i32 %150, %114
  %152 = lshr i64 %142, %117
  %153 = icmp samesign ult i32 %151, 10
  %154 = or disjoint i32 %151, 48
  %155 = add nuw nsw i32 %151, 87
  %156 = select i1 %153, i32 %154, i32 %155
  %157 = trunc i32 %156 to i8
  %158 = getelementptr inbounds nuw i8, ptr %75, i64 %149
  store i8 %157, ptr %158, align 1, !tbaa !5
  %159 = icmp samesign ugt i64 %139, 1
  br i1 %159, label %136, label %160, !llvm.loop !10

160:                                              ; preds = %132, %136, %104
  %161 = sext i8 %112 to i64
  br label %181

162:                                              ; preds = %96
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %8) #10
  %163 = getelementptr inbounds nuw i8, ptr %8, i64 41
  %164 = zext i32 %2 to i64
  br label %165

165:                                              ; preds = %165, %162
  %166 = phi i64 [ %92, %162 ], [ %170, %165 ]
  %167 = phi ptr [ %163, %162 ], [ %175, %165 ]
  %168 = urem i64 %166, %164
  %169 = trunc nuw i64 %168 to i32
  %170 = udiv i64 %166, %164
  %171 = icmp slt i32 %169, 10
  %172 = select i1 %171, i32 48, i32 87
  %173 = add nsw i32 %172, %169
  %174 = trunc i32 %173 to i8
  %175 = getelementptr inbounds i8, ptr %167, i64 -1
  store i8 %174, ptr %175, align 1, !tbaa !5
  %176 = icmp ult i64 %166, %164
  br i1 %176, label %177, label %165, !llvm.loop !11

177:                                              ; preds = %165
  %178 = ptrtoint ptr %163 to i64
  %179 = ptrtoint ptr %175 to i64
  %180 = sub i64 %178, %179
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %75, ptr noundef nonnull align 1 dereferenceable(1) %175, i64 %180, i1 false)
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %8) #10
  br label %181

181:                                              ; preds = %94, %103, %160, %177
  %182 = phi i64 [ %95, %94 ], [ %180, %177 ], [ %161, %160 ], [ 1, %103 ]
  %183 = getelementptr inbounds nuw i8, ptr %75, i64 %182
  br label %555

184:                                              ; preds = %81, %74
  %185 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %186 = icmp samesign ult i32 %185, 2
  br i1 %186, label %187, label %198

187:                                              ; preds = %184
  %188 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %189 = icmp slt i32 %69, 1023
  %190 = add nuw nsw i32 %188, 65506
  %191 = select i1 %189, i32 %190, i32 0
  %192 = add nsw i32 %191, %78
  %193 = trunc i32 %192 to i16
  %194 = trunc nuw nsw i32 %188 to i16
  %195 = sub nsw i16 31, %194
  %196 = sdiv i16 %193, %195
  %197 = sext i16 %196 to i32
  br label %208

198:                                              ; preds = %184
  %199 = add nsw i32 %2, -2
  %200 = sext i32 %199 to i64
  %201 = getelementptr inbounds [35 x i32], ptr @mul_log2_radix_table, i64 0, i64 %200
  %202 = load i32, ptr %201, align 4, !tbaa !12
  %203 = sext i32 %78 to i64
  %204 = sext i32 %202 to i64
  %205 = mul nsw i64 %204, %203
  %206 = lshr i64 %205, 24
  %207 = trunc i64 %206 to i32
  br label %208

208:                                              ; preds = %187, %198
  %209 = phi i32 [ %197, %187 ], [ %207, %198 ]
  %210 = add nsw i32 %209, 1
  br i1 %77, label %211, label %389

211:                                              ; preds = %208
  %212 = add nsw i32 %2, -2
  %213 = sext i32 %212 to i64
  %214 = getelementptr inbounds [35 x i8], ptr @dtoa_max_digits_table, i64 0, i64 %213
  %215 = load i8, ptr %214, align 1, !tbaa !5
  %216 = zext i8 %215 to i32
  %217 = add nsw i32 %69, -1075
  %218 = trunc i64 %70 to i32
  %219 = getelementptr inbounds nuw i8, ptr %5, i64 4
  %220 = lshr i64 %70, 32
  %221 = trunc nuw i64 %220 to i32
  %222 = getelementptr inbounds nuw i8, ptr %5, i64 8
  %223 = icmp ult i64 %70, 4294967296
  %224 = select i1 %223, i32 1, i32 2
  %225 = zext i32 %2 to i64
  %226 = icmp eq i32 %2, 5
  %227 = icmp eq i32 %2, 10
  %228 = or i1 %226, %227
  %229 = sext i32 %2 to i64
  %230 = mul nuw i64 %225, %225
  br label %231

231:                                              ; preds = %377, %211
  %232 = phi i32 [ %216, %211 ], [ %379, %377 ]
  %233 = phi i32 [ 0, %211 ], [ %298, %377 ]
  %234 = phi i32 [ 0, %211 ], [ %324, %377 ]
  %235 = phi i64 [ 0, %211 ], [ %325, %377 ]
  switch i32 %232, label %237 [
    i32 0, label %295
    i32 1, label %236
  ]

236:                                              ; preds = %231
  br label %295

237:                                              ; preds = %231
  %238 = icmp ult i32 %232, 18
  %239 = and i1 %228, %238
  br i1 %239, label %240, label %260

240:                                              ; preds = %237
  %241 = add nsw i32 %232, -1
  %242 = zext nneg i32 %241 to i64
  %243 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %242
  %244 = load i32, ptr %243, align 4, !tbaa !12
  %245 = zext i32 %244 to i64
  %246 = icmp samesign ugt i32 %232, 13
  br i1 %246, label %247, label %255

247:                                              ; preds = %240
  %248 = add nsw i32 %232, -14
  %249 = zext nneg i32 %248 to i64
  %250 = getelementptr inbounds nuw [4 x i8], ptr @pow5h_table, i64 0, i64 %249
  %251 = load i8, ptr %250, align 1, !tbaa !5
  %252 = zext i8 %251 to i64
  %253 = shl nuw nsw i64 %252, 32
  %254 = or disjoint i64 %253, %245
  br label %255

255:                                              ; preds = %247, %240
  %256 = phi i64 [ %254, %247 ], [ %245, %240 ]
  %257 = select i1 %227, i32 %232, i32 0
  %258 = zext nneg i32 %257 to i64
  %259 = shl nuw nsw i64 %256, %258
  br label %295

260:                                              ; preds = %237
  %261 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %232, i1 false)
  %262 = sub nsw i32 30, %261
  %263 = and i32 %261, 1
  %264 = icmp eq i32 %263, 0
  br i1 %264, label %265, label %272

265:                                              ; preds = %260
  %266 = shl nuw i32 1, %262
  %267 = and i32 %266, %232
  %268 = icmp eq i32 %267, 0
  %269 = select i1 %268, i64 1, i64 %225
  %270 = mul i64 %230, %269
  %271 = sub nsw i32 29, %261
  br label %272

272:                                              ; preds = %265, %260
  %273 = phi i64 [ poison, %260 ], [ %270, %265 ]
  %274 = phi i64 [ %225, %260 ], [ %270, %265 ]
  %275 = phi i32 [ %262, %260 ], [ %271, %265 ]
  %276 = icmp eq i32 %261, 30
  br i1 %276, label %295, label %277

277:                                              ; preds = %272, %277
  %278 = phi i64 [ %292, %277 ], [ %274, %272 ]
  %279 = phi i32 [ %293, %277 ], [ %275, %272 ]
  %280 = mul i64 %278, %278
  %281 = shl nuw i32 1, %279
  %282 = and i32 %281, %232
  %283 = icmp eq i32 %282, 0
  %284 = select i1 %283, i64 1, i64 %225
  %285 = mul i64 %280, %284
  %286 = add nsw i32 %279, -1
  %287 = mul i64 %285, %285
  %288 = shl nuw i32 1, %286
  %289 = and i32 %288, %232
  %290 = icmp eq i32 %289, 0
  %291 = select i1 %290, i64 1, i64 %225
  %292 = mul i64 %287, %291
  %293 = add nsw i32 %279, -2
  %294 = icmp eq i32 %286, 0
  br i1 %294, label %295, label %277, !llvm.loop !15

295:                                              ; preds = %272, %277, %231, %236, %255
  %296 = phi i64 [ %225, %236 ], [ %259, %255 ], [ 1, %231 ], [ %273, %272 ], [ %292, %277 ]
  br label %297

297:                                              ; preds = %309, %295
  %298 = phi i32 [ %210, %295 ], [ %312, %309 ]
  %299 = sub nsw i32 %232, %298
  store i32 %218, ptr %219, align 4, !tbaa !12
  store i32 %221, ptr %222, align 4, !tbaa !12
  store i32 %224, ptr %5, align 4, !tbaa !12
  %300 = tail call fastcc i32 @mul_pow(ptr noundef nonnull %5, i32 noundef %26, i32 noundef %25, i32 noundef %299, i32 noundef 1, i32 noundef range(i32 -2147483648, 2147482573) %217)
  %301 = sub nsw i32 %300, %217
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %5, i32 noundef %301, i32 noundef 0)
  %302 = load i32, ptr %5, align 4, !tbaa !12
  %303 = icmp eq i32 %302, 1
  br i1 %303, label %304, label %307

304:                                              ; preds = %297
  %305 = load i32, ptr %219, align 4, !tbaa !12
  %306 = zext i32 %305 to i64
  br label %309

307:                                              ; preds = %297
  %308 = load i64, ptr %219, align 4
  br label %309

309:                                              ; preds = %304, %307
  %310 = phi i64 [ %306, %304 ], [ %308, %307 ]
  %311 = icmp ult i64 %310, %296
  %312 = add nsw i32 %298, 1
  br i1 %311, label %313, label %297

313:                                              ; preds = %309
  %314 = urem i64 %310, %229
  %315 = icmp eq i64 %314, 0
  br i1 %315, label %316, label %323

316:                                              ; preds = %313, %316
  %317 = phi i64 [ %319, %316 ], [ %310, %313 ]
  %318 = phi i32 [ %320, %316 ], [ %232, %313 ]
  %319 = udiv i64 %317, %229
  %320 = add nsw i32 %318, -1
  %321 = urem i64 %319, %229
  %322 = icmp eq i64 %321, 0
  br i1 %322, label %316, label %323, !llvm.loop !16

323:                                              ; preds = %316, %313
  %324 = phi i32 [ %232, %313 ], [ %320, %316 ]
  %325 = phi i64 [ %310, %313 ], [ %319, %316 ]
  %326 = icmp eq i32 %234, 0
  br i1 %326, label %377, label %327

327:                                              ; preds = %323
  %328 = trunc i64 %325 to i32
  store i32 %328, ptr %219, align 4, !tbaa !12
  %329 = lshr i64 %325, 32
  %330 = trunc nuw i64 %329 to i32
  store i32 %330, ptr %222, align 4, !tbaa !12
  %331 = icmp ult i64 %325, 4294967296
  %332 = select i1 %331, i32 1, i32 2
  store i32 %332, ptr %5, align 4, !tbaa !12
  %333 = sub nsw i32 %298, %324
  %334 = tail call fastcc i32 @mul_pow(ptr noundef nonnull %5, i32 noundef %26, i32 noundef %25, i32 noundef %333, i32 noundef 0, i32 noundef 55)
  %335 = load i32, ptr %219, align 4, !tbaa !12
  %336 = icmp eq i32 %335, 0
  %337 = load i32, ptr %5, align 4, !tbaa !12
  %338 = icmp eq i32 %337, 1
  %339 = select i1 %336, i1 %338, i1 false
  br i1 %339, label %371, label %340

340:                                              ; preds = %327
  %341 = add nsw i32 %337, -1
  %342 = sext i32 %341 to i64
  %343 = getelementptr inbounds [0 x i32], ptr %219, i64 0, i64 %342
  %344 = load i32, ptr %343, align 4, !tbaa !12
  %345 = icmp eq i32 %344, 0
  %346 = shl nsw i32 %337, 5
  %347 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %344, i1 true)
  %348 = sub i32 %346, %347
  %349 = select i1 %345, i32 0, i32 %348
  %350 = sub nsw i32 %349, %334
  %351 = tail call i32 @llvm.smin.i32(i32 %350, i32 -1021)
  %352 = add i32 %349, -1074
  %353 = sub i32 %352, %351
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %5, i32 noundef %353, i32 noundef 0)
  %354 = load i32, ptr %5, align 4, !tbaa !12
  %355 = icmp eq i32 %354, 1
  br i1 %355, label %356, label %359

356:                                              ; preds = %340
  %357 = load i32, ptr %219, align 4, !tbaa !12
  %358 = zext i32 %357 to i64
  br label %361

359:                                              ; preds = %340
  %360 = load i64, ptr %219, align 4
  br label %361

361:                                              ; preds = %359, %356
  %362 = phi i64 [ %358, %356 ], [ %360, %359 ]
  %363 = sub nuw nsw i32 -1021, %351
  %364 = zext nneg i32 %363 to i64
  %365 = shl i64 %362, %364
  %366 = icmp ugt i64 %365, 9007199254740991
  %367 = zext i1 %366 to i64
  %368 = lshr i64 %365, %367
  %369 = zext i1 %366 to i32
  %370 = add nsw i32 %350, %369
  br label %371

371:                                              ; preds = %327, %361
  %372 = phi i64 [ %368, %361 ], [ 0, %327 ]
  %373 = phi i32 [ %370, %361 ], [ 0, %327 ]
  %374 = icmp eq i64 %372, %70
  %375 = icmp eq i32 %373, %76
  %376 = select i1 %374, i1 %375, i1 false
  br i1 %376, label %377, label %380

377:                                              ; preds = %371, %323
  %378 = icmp eq i32 %324, 1
  %379 = add nsw i32 %324, -1
  br i1 %378, label %380, label %231

380:                                              ; preds = %371, %377
  %381 = phi i32 [ %298, %377 ], [ %233, %371 ]
  %382 = phi i32 [ 1, %377 ], [ %234, %371 ]
  %383 = phi i64 [ %325, %377 ], [ %235, %371 ]
  %384 = trunc i64 %383 to i32
  store i32 %384, ptr %219, align 4, !tbaa !12
  %385 = lshr i64 %383, 32
  %386 = trunc nuw i64 %385 to i32
  store i32 %386, ptr %222, align 4, !tbaa !12
  %387 = icmp ult i64 %383, 4294967296
  %388 = select i1 %387, i32 1, i32 2
  store i32 %388, ptr %5, align 4, !tbaa !12
  br label %468

389:                                              ; preds = %208
  %390 = icmp eq i32 %9, 2
  br i1 %390, label %391, label %424

391:                                              ; preds = %389
  %392 = icmp ult i32 %3, 102
  br i1 %392, label %394, label %393

393:                                              ; preds = %391
  tail call void @__assert_fail(ptr noundef nonnull @.str.4, ptr noundef nonnull @.str.1, i32 noundef 1244, ptr noundef nonnull @__PRETTY_FUNCTION__.js_dtoa) #11
  unreachable

394:                                              ; preds = %391
  %395 = add nsw i32 %69, -1075
  %396 = trunc i64 %70 to i32
  %397 = getelementptr inbounds nuw i8, ptr %5, i64 4
  store i32 %396, ptr %397, align 4, !tbaa !12
  %398 = lshr i64 %70, 32
  %399 = trunc nuw i64 %398 to i32
  %400 = getelementptr inbounds nuw i8, ptr %5, i64 8
  store i32 %399, ptr %400, align 4, !tbaa !12
  %401 = icmp ult i64 %70, 4294967296
  %402 = select i1 %401, i32 1, i32 2
  store i32 %402, ptr %5, align 4, !tbaa !12
  %403 = tail call fastcc i32 @mul_pow(ptr noundef nonnull %5, i32 noundef %26, i32 noundef %25, i32 noundef %3, i32 noundef 1, i32 noundef range(i32 -2147483648, 2147482573) %395)
  %404 = sub nsw i32 %403, %395
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %5, i32 noundef %404, i32 noundef 1)
  %405 = tail call i32 @llvm.smax.i32(i32 %209, i32 -1)
  %406 = add nsw i32 %405, 2
  %407 = add nuw nsw i32 %406, %3
  %408 = tail call fastcc i32 @output_digits(ptr noundef %75, ptr noundef nonnull %5, i32 noundef %2, i32 noundef %407, i32 noundef %406)
  %409 = load i8, ptr %75, align 1, !tbaa !5
  %410 = icmp eq i8 %409, 48
  %411 = icmp sgt i32 %408, 1
  %412 = select i1 %410, i1 %411, i1 false
  br i1 %412, label %413, label %420

413:                                              ; preds = %394
  %414 = getelementptr inbounds nuw i8, ptr %75, i64 1
  %415 = load i8, ptr %414, align 1, !tbaa !5
  %416 = icmp eq i8 %415, 46
  br i1 %416, label %420, label %417

417:                                              ; preds = %413
  %418 = add nsw i32 %408, -1
  %419 = zext nneg i32 %418 to i64
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %75, ptr nonnull align 1 %414, i64 %419, i1 false)
  br label %420

420:                                              ; preds = %417, %413, %394
  %421 = phi i32 [ %418, %417 ], [ %408, %413 ], [ %408, %394 ]
  %422 = sext i32 %421 to i64
  %423 = getelementptr inbounds i8, ptr %75, i64 %422
  br label %555

424:                                              ; preds = %389
  %425 = add i32 %3, -1
  %426 = icmp ult i32 %425, 101
  br i1 %426, label %428, label %427

427:                                              ; preds = %424
  tail call void @__assert_fail(ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.1, i32 noundef 1261, ptr noundef nonnull @__PRETTY_FUNCTION__.js_dtoa) #11
  unreachable

428:                                              ; preds = %424
  store i32 1, ptr %10, align 4, !tbaa !12
  %429 = getelementptr inbounds nuw i8, ptr %5, i64 220
  store i32 1, ptr %429, align 4, !tbaa !12
  %430 = tail call fastcc i32 @mul_pow(ptr noundef nonnull %10, i32 noundef %26, i32 noundef %25, i32 noundef %3, i32 noundef 0, i32 noundef 0)
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %10, i32 noundef %430, i32 noundef 2)
  %431 = add nsw i32 %69, -1075
  %432 = trunc i64 %70 to i32
  %433 = getelementptr inbounds nuw i8, ptr %5, i64 4
  %434 = lshr i64 %70, 32
  %435 = trunc nuw i64 %434 to i32
  %436 = getelementptr inbounds nuw i8, ptr %5, i64 8
  %437 = icmp ult i64 %70, 4294967296
  %438 = select i1 %437, i32 1, i32 2
  br label %439

439:                                              ; preds = %466, %428
  %440 = phi i32 [ %210, %428 ], [ %467, %466 ]
  %441 = sub nsw i32 %3, %440
  store i32 %432, ptr %433, align 4, !tbaa !12
  store i32 %435, ptr %436, align 4, !tbaa !12
  store i32 %438, ptr %5, align 4, !tbaa !12
  %442 = tail call fastcc i32 @mul_pow(ptr noundef nonnull %5, i32 noundef %26, i32 noundef %25, i32 noundef %441, i32 noundef 1, i32 noundef range(i32 -2147483648, 2147482573) %431)
  %443 = sub nsw i32 %442, %431
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %5, i32 noundef %443, i32 noundef 1)
  %444 = load i32, ptr %5, align 4, !tbaa !12
  %445 = load i32, ptr %10, align 4, !tbaa !12
  %446 = icmp slt i32 %444, %445
  br i1 %446, label %468, label %447

447:                                              ; preds = %439
  %448 = icmp sle i32 %444, %445
  %449 = icmp sgt i32 %444, 0
  %450 = and i1 %449, %448
  br i1 %450, label %451, label %466

451:                                              ; preds = %447
  %452 = add nsw i32 %444, -1
  %453 = zext nneg i32 %452 to i64
  br label %457

454:                                              ; preds = %457
  %455 = add nsw i64 %458, -1
  %456 = icmp sgt i64 %458, 0
  br i1 %456, label %457, label %466, !llvm.loop !17

457:                                              ; preds = %454, %451
  %458 = phi i64 [ %453, %451 ], [ %455, %454 ]
  %459 = getelementptr inbounds nuw [0 x i32], ptr %433, i64 0, i64 %458
  %460 = load i32, ptr %459, align 4, !tbaa !12
  %461 = getelementptr inbounds nuw [0 x i32], ptr %429, i64 0, i64 %458
  %462 = load i32, ptr %461, align 4, !tbaa !12
  %463 = icmp eq i32 %460, %462
  br i1 %463, label %454, label %464

464:                                              ; preds = %457
  %465 = icmp ult i32 %460, %462
  br i1 %465, label %468, label %466

466:                                              ; preds = %454, %447, %464
  %467 = add nsw i32 %440, 1
  br label %439

468:                                              ; preds = %464, %439, %380, %50, %56
  %469 = phi ptr [ %57, %56 ], [ %0, %50 ], [ %75, %380 ], [ %75, %439 ], [ %75, %464 ]
  %470 = phi i32 [ %51, %56 ], [ %51, %50 ], [ %382, %380 ], [ %3, %439 ], [ %3, %464 ]
  %471 = phi i32 [ 1, %56 ], [ 1, %50 ], [ %381, %380 ], [ %440, %439 ], [ %440, %464 ]
  %472 = icmp eq i32 %9, 1
  br i1 %472, label %480, label %473

473:                                              ; preds = %468
  %474 = add nsw i32 %2, -2
  %475 = sext i32 %474 to i64
  %476 = getelementptr inbounds [35 x i8], ptr @dtoa_max_digits_table, i64 0, i64 %475
  %477 = load i8, ptr %476, align 1, !tbaa !5
  %478 = zext i8 %477 to i32
  %479 = add nuw nsw i32 %478, 4
  br label %480

480:                                              ; preds = %468, %473
  %481 = phi i32 [ %479, %473 ], [ %3, %468 ]
  %482 = and i32 %4, 12
  switch i32 %482, label %523 [
    i32 4, label %487
    i32 0, label %483
  ]

483:                                              ; preds = %480
  %484 = icmp slt i32 %471, -5
  %485 = icmp sgt i32 %471, %481
  %486 = select i1 %484, i1 true, i1 %485
  br i1 %486, label %487, label %523

487:                                              ; preds = %480, %483
  %488 = tail call fastcc i32 @output_digits(ptr noundef %469, ptr noundef nonnull %5, i32 noundef %2, i32 noundef %470, i32 noundef 1)
  %489 = sext i32 %488 to i64
  %490 = getelementptr inbounds i8, ptr %469, i64 %489
  %491 = add nsw i32 %471, -1
  %492 = icmp eq i32 %2, 10
  br i1 %492, label %500, label %493

493:                                              ; preds = %487
  %494 = icmp eq i32 %26, 1
  %495 = icmp slt i32 %25, 5
  %496 = and i1 %495, %494
  %497 = select i1 %496, i8 112, i8 64
  %498 = select i1 %496, i32 %25, i32 1
  %499 = mul nsw i32 %491, %498
  br label %500

500:                                              ; preds = %493, %487
  %501 = phi i8 [ 101, %487 ], [ %497, %493 ]
  %502 = phi i32 [ %491, %487 ], [ %499, %493 ]
  store i8 %501, ptr %490, align 1, !tbaa !5
  %503 = getelementptr inbounds nuw i8, ptr %490, i64 1
  %504 = icmp slt i32 %502, 0
  %505 = select i1 %504, i8 45, i8 43
  %506 = tail call i32 @llvm.abs.i32(i32 %502, i1 true)
  store i8 %505, ptr %503, align 1, !tbaa !5
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %7) #10
  %507 = getelementptr inbounds nuw i8, ptr %7, i64 10
  br label %508

508:                                              ; preds = %508, %500
  %509 = phi ptr [ %507, %500 ], [ %514, %508 ]
  %510 = phi i32 [ %506, %500 ], [ %515, %508 ]
  %511 = urem i32 %510, 10
  %512 = trunc nuw nsw i32 %511 to i8
  %513 = or disjoint i8 %512, 48
  %514 = getelementptr inbounds i8, ptr %509, i64 -1
  store i8 %513, ptr %514, align 1, !tbaa !5
  %515 = udiv i32 %510, 10
  %516 = icmp samesign ult i32 %510, 10
  br i1 %516, label %517, label %508, !llvm.loop !8

517:                                              ; preds = %508
  %518 = getelementptr inbounds nuw i8, ptr %490, i64 2
  %519 = ptrtoint ptr %507 to i64
  %520 = ptrtoint ptr %514 to i64
  %521 = sub i64 %519, %520
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %518, ptr noundef nonnull align 1 dereferenceable(1) %514, i64 %521, i1 false)
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %7) #10
  %522 = getelementptr inbounds nuw i8, ptr %518, i64 %521
  br label %555

523:                                              ; preds = %483, %480
  %524 = icmp slt i32 %471, 1
  br i1 %524, label %525, label %540

525:                                              ; preds = %523
  %526 = getelementptr inbounds nuw i8, ptr %469, i64 1
  store i8 48, ptr %469, align 1, !tbaa !5
  %527 = getelementptr i8, ptr %469, i64 2
  store i8 46, ptr %526, align 1, !tbaa !5
  %528 = icmp slt i32 %471, 0
  br i1 %528, label %529, label %535

529:                                              ; preds = %525
  %530 = sub nsw i32 0, %471
  %531 = zext nneg i32 %530 to i64
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %527, i8 48, i64 %531, i1 false), !tbaa !5
  %532 = sub i32 2, %471
  %533 = zext i32 %532 to i64
  %534 = getelementptr i8, ptr %469, i64 %533
  br label %535

535:                                              ; preds = %529, %525
  %536 = phi ptr [ %527, %525 ], [ %534, %529 ]
  %537 = tail call fastcc i32 @output_digits(ptr noundef nonnull %536, ptr noundef nonnull %5, i32 noundef %2, i32 noundef %470, i32 noundef %470)
  %538 = sext i32 %537 to i64
  %539 = getelementptr inbounds i8, ptr %536, i64 %538
  br label %555

540:                                              ; preds = %523
  %541 = tail call noundef i32 @llvm.smin.i32(i32 %470, i32 %471)
  %542 = tail call fastcc i32 @output_digits(ptr noundef %469, ptr noundef nonnull %5, i32 noundef %2, i32 noundef %470, i32 noundef %541)
  %543 = sext i32 %542 to i64
  %544 = getelementptr i8, ptr %469, i64 %543
  %545 = sub nsw i32 %471, %470
  %546 = icmp sgt i32 %545, 0
  br i1 %546, label %547, label %555

547:                                              ; preds = %540
  %548 = zext nneg i32 %545 to i64
  tail call void @llvm.memset.p0.i64(ptr align 1 %544, i8 48, i64 %548, i1 false), !tbaa !5
  %549 = xor i32 %470, -1
  %550 = add i32 %471, %549
  %551 = zext i32 %550 to i64
  %552 = getelementptr i8, ptr %469, i64 %543
  %553 = getelementptr i8, ptr %552, i64 %551
  %554 = getelementptr i8, ptr %553, i64 1
  br label %555

555:                                              ; preds = %547, %540, %420, %517, %535, %38, %41, %181
  %556 = phi ptr [ %40, %38 ], [ %42, %41 ], [ %522, %517 ], [ %539, %535 ], [ %183, %181 ], [ %423, %420 ], [ %544, %540 ], [ %554, %547 ]
  store i8 0, ptr %556, align 1, !tbaa !5
  %557 = ptrtoint ptr %556 to i64
  %558 = ptrtoint ptr %0 to i64
  %559 = sub i64 %557, %558
  %560 = trunc i64 %559 to i32
  ret i32 %560
}

; Function Attrs: cold noreturn nounwind
declare void @__assert_fail(ptr noundef, ptr noundef, i32 noundef, ptr noundef) local_unnamed_addr #6

; Function Attrs: nounwind uwtable
define internal fastcc range(i64 0, -9223372036854775808) i64 @mul_pow_round_to_d(ptr noundef nonnull writeonly captures(none) initializes((0, 4)) %0, ptr noundef captures(none) %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) unnamed_addr #5 {
  %6 = tail call fastcc i32 @mul_pow(ptr noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef 0, i32 noundef 55)
  %7 = getelementptr inbounds nuw i8, ptr %1, i64 4
  %8 = load i32, ptr %7, align 4, !tbaa !12
  %9 = icmp eq i32 %8, 0
  %10 = load i32, ptr %1, align 4, !tbaa !12
  %11 = icmp eq i32 %10, 1
  %12 = select i1 %9, i1 %11, i1 false
  br i1 %12, label %44, label %13

13:                                               ; preds = %5
  %14 = add nsw i32 %10, -1
  %15 = sext i32 %14 to i64
  %16 = getelementptr inbounds [0 x i32], ptr %7, i64 0, i64 %15
  %17 = load i32, ptr %16, align 4, !tbaa !12
  %18 = icmp eq i32 %17, 0
  %19 = shl nsw i32 %10, 5
  %20 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %17, i1 true)
  %21 = sub i32 %19, %20
  %22 = select i1 %18, i32 0, i32 %21
  %23 = sub nsw i32 %22, %6
  %24 = tail call i32 @llvm.smin.i32(i32 %23, i32 -1021)
  %25 = add i32 %22, -1074
  %26 = sub i32 %25, %24
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %1, i32 noundef %26, i32 noundef 0)
  %27 = load i32, ptr %1, align 4, !tbaa !12
  %28 = icmp eq i32 %27, 1
  br i1 %28, label %29, label %32

29:                                               ; preds = %13
  %30 = load i32, ptr %7, align 4, !tbaa !12
  %31 = zext i32 %30 to i64
  br label %34

32:                                               ; preds = %13
  %33 = load i64, ptr %7, align 4
  br label %34

34:                                               ; preds = %32, %29
  %35 = phi i64 [ %31, %29 ], [ %33, %32 ]
  %36 = sub nuw nsw i32 -1021, %24
  %37 = zext nneg i32 %36 to i64
  %38 = shl i64 %35, %37
  %39 = icmp ugt i64 %38, 9007199254740991
  %40 = zext i1 %39 to i64
  %41 = lshr i64 %38, %40
  %42 = zext i1 %39 to i32
  %43 = add nsw i32 %23, %42
  br label %44

44:                                               ; preds = %5, %34
  %45 = phi i64 [ %41, %34 ], [ 0, %5 ]
  %46 = phi i32 [ %43, %34 ], [ 0, %5 ]
  store i32 %46, ptr %0, align 4, !tbaa !12
  ret i64 %45
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal fastcc i32 @output_digits(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) unnamed_addr #0 {
  %6 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %2)
  %7 = icmp samesign ugt i32 %6, 1
  %8 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %2, i1 false)
  %9 = add nsw i32 %2, -2
  %10 = sext i32 %9 to i64
  %11 = getelementptr inbounds [35 x i8], ptr @digits_per_limb_table, i64 0, i64 %10
  %12 = load i8, ptr %11, align 1, !tbaa !5
  %13 = zext i8 %12 to i32
  %14 = icmp eq i32 %8, 31
  %15 = select i1 %7, i1 true, i1 %14
  br i1 %15, label %23, label %16

16:                                               ; preds = %5
  %17 = sub nsw i32 31, %8
  %18 = shl nsw i32 -1, %17
  %19 = xor i32 %18, -1
  %20 = getelementptr inbounds nuw i8, ptr %1, i64 4
  %21 = zext nneg i32 %17 to i64
  %22 = mul nsw i32 %17, %13
  br label %106

23:                                               ; preds = %5
  %24 = icmp eq i32 %3, 0
  br i1 %24, label %193, label %25

25:                                               ; preds = %23
  %26 = getelementptr inbounds nuw i8, ptr %1, i64 4
  %27 = getelementptr inbounds [35 x i32], ptr @radix_base_table, i64 0, i64 %10
  %28 = icmp eq i32 %2, 10
  br i1 %28, label %29, label %134

29:                                               ; preds = %25, %104
  %30 = phi i32 [ %32, %104 ], [ %3, %25 ]
  %31 = tail call noundef i32 @llvm.smin.i32(i32 %30, i32 %13)
  %32 = sub nsw i32 %30, %31
  %33 = load i32, ptr %1, align 4, !tbaa !12
  %34 = add i32 %33, -1
  %35 = icmp sgt i32 %34, -1
  br i1 %35, label %36, label %67

36:                                               ; preds = %29
  %37 = load i32, ptr %27, align 4, !tbaa !12
  %38 = zext i32 %37 to i64
  %39 = zext nneg i32 %34 to i64
  br label %40

40:                                               ; preds = %40, %36
  %41 = phi i64 [ %39, %36 ], [ %51, %40 ]
  %42 = phi i64 [ 0, %36 ], [ %50, %40 ]
  %43 = shl nuw i64 %42, 32
  %44 = getelementptr inbounds nuw i32, ptr %26, i64 %41
  %45 = load i32, ptr %44, align 4, !tbaa !12
  %46 = zext i32 %45 to i64
  %47 = or disjoint i64 %43, %46
  %48 = udiv i64 %47, %38
  %49 = trunc i64 %48 to i32
  store i32 %49, ptr %44, align 4, !tbaa !12
  %50 = urem i64 %47, %38
  %51 = add nsw i64 %41, -1
  %52 = icmp eq i64 %41, 0
  br i1 %52, label %53, label %40, !llvm.loop !18

53:                                               ; preds = %40
  %54 = trunc nuw i64 %50 to i32
  %55 = icmp sgt i32 %33, 1
  br i1 %55, label %56, label %67

56:                                               ; preds = %53
  %57 = zext nneg i32 %33 to i64
  br label %58

58:                                               ; preds = %64, %56
  %59 = phi i64 [ %57, %56 ], [ %60, %64 ]
  %60 = add nsw i64 %59, -1
  %61 = getelementptr inbounds nuw [0 x i32], ptr %26, i64 0, i64 %60
  %62 = load i32, ptr %61, align 4, !tbaa !12
  %63 = icmp eq i32 %62, 0
  br i1 %63, label %64, label %67

64:                                               ; preds = %58
  %65 = trunc nuw nsw i64 %60 to i32
  store i32 %65, ptr %1, align 4, !tbaa !12
  %66 = icmp samesign ugt i64 %59, 2
  br i1 %66, label %58, label %67, !llvm.loop !19

67:                                               ; preds = %58, %64, %53, %29
  %68 = phi i32 [ %54, %53 ], [ 0, %29 ], [ %54, %64 ], [ %54, %58 ]
  %69 = sext i32 %32 to i64
  %70 = getelementptr inbounds i8, ptr %0, i64 %69
  %71 = add i32 %31, -1
  %72 = icmp sgt i32 %71, -1
  br i1 %72, label %73, label %104

73:                                               ; preds = %67
  %74 = zext nneg i32 %71 to i64
  %75 = and i64 %74, 1
  %76 = icmp eq i64 %75, 0
  br i1 %76, label %77, label %84

77:                                               ; preds = %73
  %78 = urem i32 %68, 10
  %79 = udiv i32 %68, 10
  %80 = trunc nuw nsw i32 %78 to i8
  %81 = or disjoint i8 %80, 48
  %82 = getelementptr inbounds nuw i8, ptr %70, i64 %74
  store i8 %81, ptr %82, align 1, !tbaa !5
  %83 = add nsw i64 %74, -1
  br label %84

84:                                               ; preds = %77, %73
  %85 = phi i64 [ %74, %73 ], [ %83, %77 ]
  %86 = phi i32 [ %68, %73 ], [ %79, %77 ]
  %87 = icmp eq i32 %71, 0
  br i1 %87, label %104, label %88

88:                                               ; preds = %84, %88
  %89 = phi i64 [ %102, %88 ], [ %85, %84 ]
  %90 = phi i32 [ %98, %88 ], [ %86, %84 ]
  %91 = urem i32 %90, 10
  %92 = udiv i32 %90, 10
  %93 = trunc nuw nsw i32 %91 to i8
  %94 = or disjoint i8 %93, 48
  %95 = getelementptr inbounds nuw i8, ptr %70, i64 %89
  store i8 %94, ptr %95, align 1, !tbaa !5
  %96 = add nsw i64 %89, -1
  %97 = urem i32 %92, 10
  %98 = udiv i32 %90, 100
  %99 = trunc nuw nsw i32 %97 to i8
  %100 = or disjoint i8 %99, 48
  %101 = getelementptr inbounds nuw i8, ptr %70, i64 %96
  store i8 %100, ptr %101, align 1, !tbaa !5
  %102 = add nsw i64 %89, -2
  %103 = icmp eq i64 %96, 0
  br i1 %103, label %104, label %88, !llvm.loop !20

104:                                              ; preds = %84, %88, %67
  %105 = icmp eq i32 %32, 0
  br i1 %105, label %193, label %29, !llvm.loop !21

106:                                              ; preds = %16, %133
  %107 = phi i32 [ %109, %133 ], [ %3, %16 ]
  %108 = tail call noundef i32 @llvm.smin.i32(i32 %107, i32 %13)
  %109 = sub nsw i32 %107, %108
  %110 = sext i32 %109 to i64
  %111 = getelementptr inbounds i8, ptr %0, i64 %110
  %112 = icmp sgt i32 %108, 0
  br i1 %112, label %113, label %131

113:                                              ; preds = %106
  %114 = load i32, ptr %20, align 4, !tbaa !12
  %115 = zext i32 %114 to i64
  %116 = zext nneg i32 %108 to i64
  br label %117

117:                                              ; preds = %117, %113
  %118 = phi i64 [ %116, %113 ], [ %120, %117 ]
  %119 = phi i64 [ %115, %113 ], [ %123, %117 ]
  %120 = add nsw i64 %118, -1
  %121 = trunc nuw i64 %119 to i32
  %122 = and i32 %121, %19
  %123 = lshr i64 %119, %21
  %124 = icmp samesign ult i32 %122, 10
  %125 = or disjoint i32 %122, 48
  %126 = add nuw nsw i32 %122, 87
  %127 = select i1 %124, i32 %125, i32 %126
  %128 = trunc i32 %127 to i8
  %129 = getelementptr inbounds nuw i8, ptr %111, i64 %120
  store i8 %128, ptr %129, align 1, !tbaa !5
  %130 = icmp samesign ugt i64 %118, 1
  br i1 %130, label %117, label %131, !llvm.loop !10

131:                                              ; preds = %117, %106
  %132 = icmp eq i32 %109, 0
  br i1 %132, label %193, label %133

133:                                              ; preds = %131
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %1, i32 noundef %22, i32 noundef 2)
  br label %106

134:                                              ; preds = %25, %191
  %135 = phi i32 [ %137, %191 ], [ %3, %25 ]
  %136 = tail call noundef i32 @llvm.smin.i32(i32 %135, i32 %13)
  %137 = sub nsw i32 %135, %136
  %138 = load i32, ptr %1, align 4, !tbaa !12
  %139 = add i32 %138, -1
  %140 = icmp sgt i32 %139, -1
  br i1 %140, label %141, label %172

141:                                              ; preds = %134
  %142 = load i32, ptr %27, align 4, !tbaa !12
  %143 = zext i32 %142 to i64
  %144 = zext nneg i32 %139 to i64
  br label %145

145:                                              ; preds = %145, %141
  %146 = phi i64 [ %144, %141 ], [ %156, %145 ]
  %147 = phi i64 [ 0, %141 ], [ %155, %145 ]
  %148 = shl nuw i64 %147, 32
  %149 = getelementptr inbounds nuw i32, ptr %26, i64 %146
  %150 = load i32, ptr %149, align 4, !tbaa !12
  %151 = zext i32 %150 to i64
  %152 = or disjoint i64 %148, %151
  %153 = udiv i64 %152, %143
  %154 = trunc i64 %153 to i32
  store i32 %154, ptr %149, align 4, !tbaa !12
  %155 = urem i64 %152, %143
  %156 = add nsw i64 %146, -1
  %157 = icmp eq i64 %146, 0
  br i1 %157, label %158, label %145, !llvm.loop !18

158:                                              ; preds = %145
  %159 = trunc nuw i64 %155 to i32
  %160 = icmp sgt i32 %138, 1
  br i1 %160, label %161, label %172

161:                                              ; preds = %158
  %162 = zext nneg i32 %138 to i64
  br label %163

163:                                              ; preds = %169, %161
  %164 = phi i64 [ %162, %161 ], [ %165, %169 ]
  %165 = add nsw i64 %164, -1
  %166 = getelementptr inbounds nuw [0 x i32], ptr %26, i64 0, i64 %165
  %167 = load i32, ptr %166, align 4, !tbaa !12
  %168 = icmp eq i32 %167, 0
  br i1 %168, label %169, label %172

169:                                              ; preds = %163
  %170 = trunc nuw nsw i64 %165 to i32
  store i32 %170, ptr %1, align 4, !tbaa !12
  %171 = icmp samesign ugt i64 %164, 2
  br i1 %171, label %163, label %172, !llvm.loop !19

172:                                              ; preds = %163, %169, %134, %158
  %173 = phi i32 [ %159, %158 ], [ 0, %134 ], [ %159, %169 ], [ %159, %163 ]
  %174 = sext i32 %137 to i64
  %175 = getelementptr inbounds i8, ptr %0, i64 %174
  %176 = icmp sgt i32 %136, 0
  br i1 %176, label %177, label %191

177:                                              ; preds = %172
  %178 = zext nneg i32 %136 to i64
  br label %179

179:                                              ; preds = %179, %177
  %180 = phi i64 [ %178, %177 ], [ %182, %179 ]
  %181 = phi i32 [ %173, %177 ], [ %184, %179 ]
  %182 = add nsw i64 %180, -1
  %183 = urem i32 %181, %2
  %184 = udiv i32 %181, %2
  %185 = icmp slt i32 %183, 10
  %186 = select i1 %185, i32 48, i32 87
  %187 = add nsw i32 %186, %183
  %188 = trunc i32 %187 to i8
  %189 = getelementptr inbounds nuw i8, ptr %175, i64 %182
  store i8 %188, ptr %189, align 1, !tbaa !5
  %190 = icmp samesign ugt i64 %180, 1
  br i1 %190, label %179, label %191, !llvm.loop !22

191:                                              ; preds = %179, %172
  %192 = icmp eq i32 %137, 0
  br i1 %192, label %193, label %134, !llvm.loop !21

193:                                              ; preds = %131, %191, %104, %23
  %194 = icmp eq i32 %4, %3
  br i1 %194, label %202, label %195

195:                                              ; preds = %193
  %196 = sext i32 %4 to i64
  %197 = getelementptr inbounds i8, ptr %0, i64 %196
  %198 = getelementptr inbounds nuw i8, ptr %197, i64 1
  %199 = sub nsw i32 %3, %4
  %200 = sext i32 %199 to i64
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %198, ptr align 1 %197, i64 %200, i1 false)
  store i8 46, ptr %197, align 1, !tbaa !5
  %201 = add nsw i32 %3, 1
  br label %202

202:                                              ; preds = %195, %193
  %203 = phi i32 [ %201, %195 ], [ %3, %193 ]
  ret i32 %203
}

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #2

; Function Attrs: nounwind uwtable
define internal fastcc i32 @mul_pow(ptr noundef captures(none) %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef range(i32 0, 2) %4, i32 noundef range(i32 -2147483648, 2147482573) %5) unnamed_addr #5 {
  %7 = sub nsw i32 0, %3
  %8 = mul nsw i32 %2, %7
  %9 = icmp eq i32 %1, 1
  br i1 %9, label %402, label %10

10:                                               ; preds = %6
  %11 = add nsw i32 %1, -2
  %12 = sext i32 %11 to i64
  %13 = getelementptr inbounds [35 x i8], ptr @digits_per_limb_table, i64 0, i64 %12
  %14 = load i8, ptr %13, align 1, !tbaa !5
  %15 = zext i8 %14 to i32
  %16 = icmp sgt i32 %3, -1
  br i1 %16, label %17, label %155

17:                                               ; preds = %10
  %18 = icmp eq i32 %3, 0
  br i1 %18, label %402, label %19

19:                                               ; preds = %17
  %20 = zext i32 %1 to i64
  %21 = icmp eq i32 %1, 5
  %22 = icmp eq i32 %1, 10
  %23 = or i1 %21, %22
  %24 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %25 = mul nuw i64 %20, %20
  %26 = getelementptr inbounds i8, ptr %24, i64 4
  br label %27

27:                                               ; preds = %19, %152
  %28 = phi i32 [ %3, %19 ], [ %153, %152 ]
  %29 = phi i32 [ 0, %19 ], [ %98, %152 ]
  %30 = phi i32 [ 0, %19 ], [ %97, %152 ]
  %31 = tail call noundef i32 @llvm.smin.i32(i32 %28, i32 %15)
  %32 = icmp eq i32 %31, %30
  br i1 %32, label %96, label %33

33:                                               ; preds = %27
  switch i32 %31, label %35 [
    i32 0, label %93
    i32 1, label %34
  ]

34:                                               ; preds = %33
  br label %93

35:                                               ; preds = %33
  %36 = icmp ult i32 %31, 18
  %37 = and i1 %23, %36
  br i1 %37, label %38, label %58

38:                                               ; preds = %35
  %39 = add nsw i32 %31, -1
  %40 = zext nneg i32 %39 to i64
  %41 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %40
  %42 = load i32, ptr %41, align 4, !tbaa !12
  %43 = zext i32 %42 to i64
  %44 = icmp samesign ugt i32 %31, 13
  br i1 %44, label %45, label %53

45:                                               ; preds = %38
  %46 = add nsw i32 %31, -14
  %47 = zext nneg i32 %46 to i64
  %48 = getelementptr inbounds nuw [4 x i8], ptr @pow5h_table, i64 0, i64 %47
  %49 = load i8, ptr %48, align 1, !tbaa !5
  %50 = zext i8 %49 to i64
  %51 = shl nuw nsw i64 %50, 32
  %52 = or disjoint i64 %51, %43
  br label %53

53:                                               ; preds = %45, %38
  %54 = phi i64 [ %52, %45 ], [ %43, %38 ]
  %55 = select i1 %22, i32 %31, i32 0
  %56 = zext nneg i32 %55 to i64
  %57 = shl nuw nsw i64 %54, %56
  br label %93

58:                                               ; preds = %35
  %59 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %31, i1 false)
  %60 = sub nsw i32 30, %59
  %61 = and i32 %59, 1
  %62 = icmp eq i32 %61, 0
  br i1 %62, label %63, label %70

63:                                               ; preds = %58
  %64 = shl nuw i32 1, %60
  %65 = and i32 %64, %31
  %66 = icmp eq i32 %65, 0
  %67 = select i1 %66, i64 1, i64 %20
  %68 = mul i64 %25, %67
  %69 = sub nsw i32 29, %59
  br label %70

70:                                               ; preds = %63, %58
  %71 = phi i64 [ poison, %58 ], [ %68, %63 ]
  %72 = phi i64 [ %20, %58 ], [ %68, %63 ]
  %73 = phi i32 [ %60, %58 ], [ %69, %63 ]
  %74 = icmp eq i32 %59, 30
  br i1 %74, label %93, label %75

75:                                               ; preds = %70, %75
  %76 = phi i64 [ %90, %75 ], [ %72, %70 ]
  %77 = phi i32 [ %91, %75 ], [ %73, %70 ]
  %78 = mul i64 %76, %76
  %79 = shl nuw i32 1, %77
  %80 = and i32 %79, %31
  %81 = icmp eq i32 %80, 0
  %82 = select i1 %81, i64 1, i64 %20
  %83 = mul i64 %78, %82
  %84 = add nsw i32 %77, -1
  %85 = mul i64 %83, %83
  %86 = shl nuw i32 1, %84
  %87 = and i32 %86, %31
  %88 = icmp eq i32 %87, 0
  %89 = select i1 %88, i64 1, i64 %20
  %90 = mul i64 %85, %89
  %91 = add nsw i32 %77, -2
  %92 = icmp eq i32 %84, 0
  br i1 %92, label %93, label %75, !llvm.loop !15

93:                                               ; preds = %70, %75, %33, %34, %53
  %94 = phi i64 [ %20, %34 ], [ %57, %53 ], [ 1, %33 ], [ %71, %70 ], [ %90, %75 ]
  %95 = trunc i64 %94 to i32
  br label %96

96:                                               ; preds = %93, %27
  %97 = phi i32 [ %31, %93 ], [ %30, %27 ]
  %98 = phi i32 [ %95, %93 ], [ %29, %27 ]
  %99 = load i32, ptr %0, align 4, !tbaa !12
  %100 = icmp eq i32 %99, 0
  br i1 %100, label %152, label %101

101:                                              ; preds = %96
  %102 = zext i32 %98 to i64
  %103 = zext i32 %99 to i64
  %104 = and i64 %103, 1
  %105 = icmp eq i32 %99, 1
  br i1 %105, label %129, label %106

106:                                              ; preds = %101
  %107 = and i64 %103, 4294967294
  br label %108

108:                                              ; preds = %108, %106
  %109 = phi i64 [ 0, %106 ], [ %126, %108 ]
  %110 = phi i64 [ 0, %106 ], [ %125, %108 ]
  %111 = phi i64 [ 0, %106 ], [ %127, %108 ]
  %112 = getelementptr inbounds nuw i32, ptr %24, i64 %109
  %113 = load i32, ptr %112, align 4, !tbaa !12
  %114 = zext i32 %113 to i64
  %115 = mul nuw i64 %114, %102
  %116 = add nuw i64 %115, %110
  %117 = trunc i64 %116 to i32
  store i32 %117, ptr %112, align 4, !tbaa !12
  %118 = lshr i64 %116, 32
  %119 = getelementptr inbounds i32, ptr %26, i64 %109
  %120 = load i32, ptr %119, align 4, !tbaa !12
  %121 = zext i32 %120 to i64
  %122 = mul nuw i64 %121, %102
  %123 = add nuw i64 %122, %118
  %124 = trunc i64 %123 to i32
  store i32 %124, ptr %119, align 4, !tbaa !12
  %125 = lshr i64 %123, 32
  %126 = add nuw nsw i64 %109, 2
  %127 = add i64 %111, 2
  %128 = icmp eq i64 %127, %107
  br i1 %128, label %129, label %108, !llvm.loop !23

129:                                              ; preds = %108, %101
  %130 = phi i64 [ poison, %101 ], [ %123, %108 ]
  %131 = phi i64 [ poison, %101 ], [ %125, %108 ]
  %132 = phi i64 [ 0, %101 ], [ %126, %108 ]
  %133 = phi i64 [ 0, %101 ], [ %125, %108 ]
  %134 = icmp eq i64 %104, 0
  br i1 %134, label %143, label %135

135:                                              ; preds = %129
  %136 = getelementptr inbounds nuw i32, ptr %24, i64 %132
  %137 = load i32, ptr %136, align 4, !tbaa !12
  %138 = zext i32 %137 to i64
  %139 = mul nuw i64 %138, %102
  %140 = add nuw i64 %139, %133
  %141 = trunc i64 %140 to i32
  store i32 %141, ptr %136, align 4, !tbaa !12
  %142 = lshr i64 %140, 32
  br label %143

143:                                              ; preds = %129, %135
  %144 = phi i64 [ %130, %129 ], [ %140, %135 ]
  %145 = phi i64 [ %131, %129 ], [ %142, %135 ]
  %146 = icmp ult i64 %144, 4294967296
  br i1 %146, label %152, label %147

147:                                              ; preds = %143
  %148 = trunc nuw i64 %145 to i32
  %149 = add nsw i32 %99, 1
  store i32 %149, ptr %0, align 4, !tbaa !12
  %150 = sext i32 %99 to i64
  %151 = getelementptr inbounds [0 x i32], ptr %24, i64 0, i64 %150
  store i32 %148, ptr %151, align 4, !tbaa !12
  br label %152

152:                                              ; preds = %96, %147, %143
  %153 = sub nsw i32 %28, %31
  %154 = icmp eq i32 %153, 0
  br i1 %154, label %402, label %27, !llvm.loop !24

155:                                              ; preds = %10
  %156 = xor i32 %3, -1
  %157 = add nuw i32 %15, %156
  %158 = sdiv i32 %157, %15
  %159 = shl nsw i32 %158, 5
  %160 = add nsw i32 %159, %8
  %161 = icmp eq i32 %4, 0
  br i1 %161, label %162, label %178

162:                                              ; preds = %155
  %163 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %164 = load i32, ptr %0, align 4, !tbaa !12
  %165 = add nsw i32 %164, -1
  %166 = sext i32 %165 to i64
  %167 = getelementptr inbounds [0 x i32], ptr %163, i64 0, i64 %166
  %168 = load i32, ptr %167, align 4, !tbaa !12
  %169 = icmp eq i32 %168, 0
  br i1 %169, label %175, label %170

170:                                              ; preds = %162
  %171 = shl nsw i32 %164, 5
  %172 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %168, i1 true)
  %173 = xor i32 %172, -1
  %174 = add i32 %171, %173
  br label %175

175:                                              ; preds = %162, %170
  %176 = phi i32 [ %174, %170 ], [ -1, %162 ]
  %177 = sub nsw i32 %5, %176
  br label %181

178:                                              ; preds = %155
  %179 = add nsw i32 %5, 2
  %180 = sub i32 %179, %160
  br label %181

181:                                              ; preds = %178, %175
  %182 = phi i32 [ %180, %178 ], [ %177, %175 ]
  %183 = tail call range(i32 0, -2147483648) i32 @llvm.smax.i32(i32 %182, i32 0)
  %184 = add nsw i32 %183, %160
  %185 = add nsw i32 %159, %183
  %186 = sub nsw i32 0, %185
  tail call fastcc void @mpb_shr_round(ptr noundef %0, i32 noundef %186, i32 noundef 2)
  %187 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %188 = load i32, ptr %0, align 4, !tbaa !12
  %189 = icmp eq i32 %1, 5
  %190 = zext i32 %1 to i64
  %191 = icmp eq i32 %1, 10
  %192 = or i1 %189, %191
  %193 = mul nuw i64 %190, %190
  %194 = getelementptr inbounds i8, ptr %187, i64 4
  br label %195

195:                                              ; preds = %181, %390
  %196 = phi i32 [ 0, %181 ], [ %292, %390 ]
  %197 = phi i32 [ 0, %181 ], [ %394, %390 ]
  %198 = phi i32 [ %7, %181 ], [ %395, %390 ]
  %199 = phi i32 [ 0, %181 ], [ %291, %390 ]
  %200 = phi i32 [ 0, %181 ], [ %290, %390 ]
  %201 = phi i32 [ 0, %181 ], [ %289, %390 ]
  %202 = phi i32 [ %188, %181 ], [ %391, %390 ]
  %203 = tail call noundef i32 @llvm.smin.i32(i32 %198, i32 %15)
  %204 = icmp eq i32 %203, %199
  br i1 %204, label %288, label %205

205:                                              ; preds = %195
  %206 = add i32 %203, -1
  %207 = icmp ult i32 %203, 14
  %208 = and i1 %189, %207
  br i1 %208, label %209, label %217

209:                                              ; preds = %205
  %210 = zext nneg i32 %206 to i64
  %211 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %210
  %212 = load i32, ptr %211, align 4, !tbaa !12
  %213 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %212, i1 false)
  %214 = shl i32 %212, %213
  %215 = getelementptr inbounds nuw [13 x i32], ptr @pow5_inv_table, i64 0, i64 %210
  %216 = load i32, ptr %215, align 4, !tbaa !12
  br label %288

217:                                              ; preds = %205
  switch i32 %203, label %219 [
    i32 0, label %276
    i32 1, label %218
  ]

218:                                              ; preds = %217
  br label %276

219:                                              ; preds = %217
  %220 = icmp ult i32 %203, 18
  %221 = and i1 %192, %220
  br i1 %221, label %222, label %241

222:                                              ; preds = %219
  %223 = zext nneg i32 %206 to i64
  %224 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %223
  %225 = load i32, ptr %224, align 4, !tbaa !12
  %226 = zext i32 %225 to i64
  %227 = icmp samesign ugt i32 %203, 13
  br i1 %227, label %228, label %236

228:                                              ; preds = %222
  %229 = add nsw i32 %203, -14
  %230 = zext nneg i32 %229 to i64
  %231 = getelementptr inbounds nuw [4 x i8], ptr @pow5h_table, i64 0, i64 %230
  %232 = load i8, ptr %231, align 1, !tbaa !5
  %233 = zext i8 %232 to i64
  %234 = shl nuw nsw i64 %233, 32
  %235 = or disjoint i64 %234, %226
  br label %236

236:                                              ; preds = %228, %222
  %237 = phi i64 [ %235, %228 ], [ %226, %222 ]
  %238 = select i1 %191, i32 %203, i32 0
  %239 = zext nneg i32 %238 to i64
  %240 = shl nuw nsw i64 %237, %239
  br label %276

241:                                              ; preds = %219
  %242 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %203, i1 true)
  %243 = sub nsw i32 30, %242
  %244 = and i32 %242, 1
  %245 = icmp eq i32 %244, 0
  br i1 %245, label %246, label %253

246:                                              ; preds = %241
  %247 = shl nuw nsw i32 1, %243
  %248 = and i32 %247, %203
  %249 = icmp eq i32 %248, 0
  %250 = select i1 %249, i64 1, i64 %190
  %251 = mul i64 %193, %250
  %252 = sub nsw i32 29, %242
  br label %253

253:                                              ; preds = %246, %241
  %254 = phi i64 [ poison, %241 ], [ %251, %246 ]
  %255 = phi i64 [ %190, %241 ], [ %251, %246 ]
  %256 = phi i32 [ %243, %241 ], [ %252, %246 ]
  %257 = icmp eq i32 %242, 30
  br i1 %257, label %276, label %258

258:                                              ; preds = %253, %258
  %259 = phi i64 [ %273, %258 ], [ %255, %253 ]
  %260 = phi i32 [ %274, %258 ], [ %256, %253 ]
  %261 = mul i64 %259, %259
  %262 = shl nuw i32 1, %260
  %263 = and i32 %262, %203
  %264 = icmp eq i32 %263, 0
  %265 = select i1 %264, i64 1, i64 %190
  %266 = mul i64 %261, %265
  %267 = add nsw i32 %260, -1
  %268 = mul i64 %266, %266
  %269 = shl nuw i32 1, %267
  %270 = and i32 %269, %203
  %271 = icmp eq i32 %270, 0
  %272 = select i1 %271, i64 1, i64 %190
  %273 = mul i64 %268, %272
  %274 = add nsw i32 %260, -2
  %275 = icmp eq i32 %267, 0
  br i1 %275, label %276, label %258, !llvm.loop !15

276:                                              ; preds = %253, %258, %236, %218, %217
  %277 = phi i64 [ %190, %218 ], [ %240, %236 ], [ 1, %217 ], [ %254, %253 ], [ %273, %258 ]
  %278 = trunc i64 %277 to i32
  %279 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %278, i1 false)
  %280 = shl i32 %278, %279
  %281 = xor i32 %280, -1
  %282 = zext i32 %281 to i64
  %283 = shl nuw i64 %282, 32
  %284 = or disjoint i64 %283, 4294967295
  %285 = zext i32 %280 to i64
  %286 = udiv i64 %284, %285
  %287 = trunc i64 %286 to i32
  br label %288

288:                                              ; preds = %276, %209, %195
  %289 = phi i32 [ %201, %195 ], [ %213, %209 ], [ %279, %276 ]
  %290 = phi i32 [ %200, %195 ], [ %216, %209 ], [ %287, %276 ]
  %291 = phi i32 [ %199, %195 ], [ %203, %209 ], [ %203, %276 ]
  %292 = phi i32 [ %196, %195 ], [ %214, %209 ], [ %280, %276 ]
  %293 = icmp eq i32 %289, 0
  br i1 %293, label %334, label %294

294:                                              ; preds = %288
  %295 = zext i32 %202 to i64
  %296 = icmp samesign ult i32 %289, 32
  br i1 %296, label %297, label %305

297:                                              ; preds = %294
  %298 = icmp eq i32 %202, 0
  br i1 %298, label %390, label %299

299:                                              ; preds = %297
  %300 = sub nuw nsw i32 32, %289
  %301 = and i64 %295, 1
  %302 = icmp eq i32 %202, 1
  br i1 %302, label %323, label %303

303:                                              ; preds = %299
  %304 = and i64 %295, 4294967294
  br label %306

305:                                              ; preds = %294
  tail call void @__assert_fail(ptr noundef nonnull @.str.7, ptr noundef nonnull @.str.1, i32 noundef 175, ptr noundef nonnull @__PRETTY_FUNCTION__.mp_shl) #11
  unreachable

306:                                              ; preds = %306, %303
  %307 = phi i64 [ 0, %303 ], [ %320, %306 ]
  %308 = phi i32 [ 0, %303 ], [ %319, %306 ]
  %309 = phi i64 [ 0, %303 ], [ %321, %306 ]
  %310 = getelementptr inbounds nuw i32, ptr %187, i64 %307
  %311 = load i32, ptr %310, align 4, !tbaa !12
  %312 = shl i32 %311, %289
  %313 = or i32 %312, %308
  store i32 %313, ptr %310, align 4, !tbaa !12
  %314 = lshr i32 %311, %300
  %315 = getelementptr inbounds i32, ptr %194, i64 %307
  %316 = load i32, ptr %315, align 4, !tbaa !12
  %317 = shl i32 %316, %289
  %318 = or disjoint i32 %317, %314
  store i32 %318, ptr %315, align 4, !tbaa !12
  %319 = lshr i32 %316, %300
  %320 = add nuw nsw i64 %307, 2
  %321 = add i64 %309, 2
  %322 = icmp eq i64 %321, %304
  br i1 %322, label %323, label %306, !llvm.loop !25

323:                                              ; preds = %306, %299
  %324 = phi i32 [ poison, %299 ], [ %319, %306 ]
  %325 = phi i64 [ 0, %299 ], [ %320, %306 ]
  %326 = phi i32 [ 0, %299 ], [ %319, %306 ]
  %327 = icmp eq i64 %301, 0
  br i1 %327, label %334, label %328

328:                                              ; preds = %323
  %329 = getelementptr inbounds nuw i32, ptr %187, i64 %325
  %330 = load i32, ptr %329, align 4, !tbaa !12
  %331 = shl i32 %330, %289
  %332 = or i32 %331, %326
  store i32 %332, ptr %329, align 4, !tbaa !12
  %333 = lshr i32 %330, %300
  br label %334

334:                                              ; preds = %328, %323, %288
  %335 = phi i32 [ 0, %288 ], [ %324, %323 ], [ %333, %328 ]
  %336 = add i32 %202, -1
  %337 = icmp sgt i32 %336, -1
  br i1 %337, label %338, label %390

338:                                              ; preds = %334
  %339 = zext i32 %290 to i64
  %340 = zext i32 %292 to i64
  %341 = zext nneg i32 %336 to i64
  br label %342

342:                                              ; preds = %342, %338
  %343 = phi i64 [ %341, %338 ], [ %373, %342 ]
  %344 = phi i32 [ %335, %338 ], [ %372, %342 ]
  %345 = getelementptr inbounds nuw i32, ptr %187, i64 %343
  %346 = load i32, ptr %345, align 4, !tbaa !12
  %347 = ashr i32 %346, 31
  %348 = and i32 %347, %292
  %349 = add i32 %348, %346
  %350 = sub i32 %344, %347
  %351 = zext i32 %350 to i64
  %352 = mul nuw i64 %351, %339
  %353 = zext i32 %349 to i64
  %354 = add nuw i64 %352, %353
  %355 = lshr i64 %354, 32
  %356 = zext i32 %344 to i64
  %357 = trunc nuw i64 %355 to i32
  %358 = add i32 %344, %357
  %359 = shl nuw i64 %356, 32
  %360 = zext i32 %346 to i64
  %361 = or disjoint i64 %359, %360
  %362 = zext i32 %358 to i64
  %363 = xor i64 %362, -1
  %364 = mul i64 %363, %340
  %365 = add i64 %361, %364
  %366 = lshr i64 %365, 32
  %367 = trunc nuw i64 %366 to i32
  %368 = add i32 %358, 1
  %369 = add i32 %368, %367
  %370 = trunc i64 %365 to i32
  %371 = and i32 %292, %367
  %372 = add i32 %371, %370
  store i32 %369, ptr %345, align 4, !tbaa !12
  %373 = add nsw i64 %343, -1
  %374 = icmp eq i64 %343, 0
  br i1 %374, label %375, label %342, !llvm.loop !26

375:                                              ; preds = %342
  %376 = icmp sgt i32 %202, 1
  br i1 %376, label %377, label %390

377:                                              ; preds = %375
  %378 = zext nneg i32 %202 to i64
  br label %379

379:                                              ; preds = %385, %377
  %380 = phi i64 [ %378, %377 ], [ %381, %385 ]
  %381 = add nsw i64 %380, -1
  %382 = getelementptr inbounds nuw [0 x i32], ptr %187, i64 0, i64 %381
  %383 = load i32, ptr %382, align 4, !tbaa !12
  %384 = icmp eq i32 %383, 0
  br i1 %384, label %385, label %388

385:                                              ; preds = %379
  %386 = trunc i64 %381 to i32
  store i32 %386, ptr %0, align 4, !tbaa !12
  %387 = icmp samesign ugt i64 %380, 2
  br i1 %387, label %379, label %390, !llvm.loop !19

388:                                              ; preds = %379
  %389 = trunc i64 %380 to i32
  br label %390

390:                                              ; preds = %385, %388, %334, %297, %375
  %391 = phi i32 [ %202, %375 ], [ 0, %297 ], [ %202, %334 ], [ %389, %388 ], [ %386, %385 ]
  %392 = phi i32 [ %372, %375 ], [ 0, %297 ], [ %335, %334 ], [ %372, %388 ], [ %372, %385 ]
  %393 = lshr i32 %392, %289
  %394 = or i32 %393, %197
  %395 = sub nsw i32 %198, %203
  %396 = icmp eq i32 %395, 0
  br i1 %396, label %397, label %195, !llvm.loop !27

397:                                              ; preds = %390
  %398 = icmp ne i32 %394, 0
  %399 = zext i1 %398 to i32
  %400 = load i32, ptr %187, align 4, !tbaa !12
  %401 = or i32 %400, %399
  store i32 %401, ptr %187, align 4, !tbaa !12
  br label %402

402:                                              ; preds = %152, %17, %397, %6
  %403 = phi i32 [ %184, %397 ], [ %8, %6 ], [ 0, %17 ], [ %8, %152 ]
  ret i32 %403
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal fastcc void @mpb_shr_round(ptr noundef captures(none) %0, i32 noundef %1, i32 noundef range(i32 0, 3) %2) unnamed_addr #0 {
  %4 = icmp eq i32 %1, 0
  br i1 %4, label %316, label %5

5:                                                ; preds = %3
  %6 = icmp slt i32 %1, 0
  br i1 %6, label %7, label %116

7:                                                ; preds = %5
  %8 = sub nsw i32 0, %1
  %9 = lshr i32 %8, 5
  %10 = and i32 %8, 31
  %11 = icmp eq i32 %10, 0
  br i1 %11, label %69, label %12

12:                                               ; preds = %7
  %13 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %14 = load i32, ptr %0, align 4, !tbaa !12
  %15 = sext i32 %14 to i64
  %16 = icmp sgt i32 %14, 0
  br i1 %16, label %17, label %52

17:                                               ; preds = %12
  %18 = sub nuw nsw i32 32, %10
  %19 = icmp eq i32 %14, 1
  br i1 %19, label %40, label %20

20:                                               ; preds = %17
  %21 = and i64 %15, 2147483646
  %22 = getelementptr inbounds i8, ptr %13, i64 4
  br label %23

23:                                               ; preds = %23, %20
  %24 = phi i64 [ 0, %20 ], [ %37, %23 ]
  %25 = phi i32 [ 0, %20 ], [ %36, %23 ]
  %26 = phi i64 [ 0, %20 ], [ %38, %23 ]
  %27 = getelementptr inbounds nuw i32, ptr %13, i64 %24
  %28 = load i32, ptr %27, align 4, !tbaa !12
  %29 = shl i32 %28, %10
  %30 = or i32 %29, %25
  store i32 %30, ptr %27, align 4, !tbaa !12
  %31 = lshr i32 %28, %18
  %32 = getelementptr inbounds i32, ptr %22, i64 %24
  %33 = load i32, ptr %32, align 4, !tbaa !12
  %34 = shl i32 %33, %10
  %35 = or disjoint i32 %34, %31
  store i32 %35, ptr %32, align 4, !tbaa !12
  %36 = lshr i32 %33, %18
  %37 = add nuw nsw i64 %24, 2
  %38 = add i64 %26, 2
  %39 = icmp eq i64 %38, %21
  br i1 %39, label %40, label %23, !llvm.loop !25

40:                                               ; preds = %23, %17
  %41 = phi i32 [ poison, %17 ], [ %36, %23 ]
  %42 = phi i64 [ 0, %17 ], [ %37, %23 ]
  %43 = phi i32 [ 0, %17 ], [ %36, %23 ]
  %44 = and i32 %14, 1
  %45 = icmp eq i32 %44, 0
  br i1 %45, label %52, label %46

46:                                               ; preds = %40
  %47 = getelementptr inbounds nuw i32, ptr %13, i64 %42
  %48 = load i32, ptr %47, align 4, !tbaa !12
  %49 = shl i32 %48, %10
  %50 = or i32 %49, %43
  store i32 %50, ptr %47, align 4, !tbaa !12
  %51 = lshr i32 %48, %18
  br label %52

52:                                               ; preds = %46, %40, %12
  %53 = phi i32 [ 0, %12 ], [ %41, %40 ], [ %51, %46 ]
  %54 = getelementptr inbounds [0 x i32], ptr %13, i64 0, i64 %15
  store i32 %53, ptr %54, align 4, !tbaa !12
  %55 = load i32, ptr %0, align 4, !tbaa !12
  %56 = add nsw i32 %55, 1
  store i32 %56, ptr %0, align 4, !tbaa !12
  %57 = icmp sgt i32 %55, 0
  br i1 %57, label %58, label %69

58:                                               ; preds = %52
  %59 = zext nneg i32 %56 to i64
  br label %60

60:                                               ; preds = %66, %58
  %61 = phi i64 [ %59, %58 ], [ %62, %66 ]
  %62 = add nsw i64 %61, -1
  %63 = getelementptr inbounds nuw [0 x i32], ptr %13, i64 0, i64 %62
  %64 = load i32, ptr %63, align 4, !tbaa !12
  %65 = icmp eq i32 %64, 0
  br i1 %65, label %66, label %69

66:                                               ; preds = %60
  %67 = trunc nuw nsw i64 %62 to i32
  store i32 %67, ptr %0, align 4, !tbaa !12
  %68 = icmp samesign ugt i64 %61, 2
  br i1 %68, label %60, label %69, !llvm.loop !19

69:                                               ; preds = %66, %60, %52, %7
  %70 = icmp ult i32 %8, 32
  br i1 %70, label %316, label %71

71:                                               ; preds = %69
  %72 = load i32, ptr %0, align 4, !tbaa !12
  %73 = icmp sgt i32 %72, 0
  br i1 %73, label %74, label %110

74:                                               ; preds = %71
  %75 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %76 = zext nneg i32 %72 to i64
  %77 = zext nneg i32 %9 to i64
  %78 = icmp ult i32 %72, 8
  br i1 %78, label %100, label %79

79:                                               ; preds = %74
  %80 = and i64 %76, 2147483640
  %81 = and i64 %76, 7
  %82 = add i64 %76, %77
  br label %83

83:                                               ; preds = %83, %79
  %84 = phi i64 [ 0, %79 ], [ %96, %83 ]
  %85 = xor i64 %84, -1
  %86 = add i64 %85, %76
  %87 = getelementptr inbounds nuw [0 x i32], ptr %75, i64 0, i64 %86
  %88 = getelementptr inbounds i8, ptr %87, i64 -12
  %89 = getelementptr inbounds i8, ptr %87, i64 -28
  %90 = load <4 x i32>, ptr %88, align 4, !tbaa !12
  %91 = load <4 x i32>, ptr %89, align 4, !tbaa !12
  %92 = add i64 %85, %82
  %93 = getelementptr inbounds nuw [0 x i32], ptr %75, i64 0, i64 %92
  %94 = getelementptr inbounds i8, ptr %93, i64 -12
  %95 = getelementptr inbounds i8, ptr %93, i64 -28
  store <4 x i32> %90, ptr %94, align 4, !tbaa !12
  store <4 x i32> %91, ptr %95, align 4, !tbaa !12
  %96 = add nuw i64 %84, 8
  %97 = icmp eq i64 %96, %80
  br i1 %97, label %98, label %83, !llvm.loop !28

98:                                               ; preds = %83
  %99 = icmp eq i64 %80, %76
  br i1 %99, label %110, label %100

100:                                              ; preds = %74, %98
  %101 = phi i64 [ %76, %74 ], [ %81, %98 ]
  br label %102

102:                                              ; preds = %100, %102
  %103 = phi i64 [ %104, %102 ], [ %101, %100 ]
  %104 = add nsw i64 %103, -1
  %105 = getelementptr inbounds nuw [0 x i32], ptr %75, i64 0, i64 %104
  %106 = load i32, ptr %105, align 4, !tbaa !12
  %107 = add nuw nsw i64 %104, %77
  %108 = getelementptr inbounds nuw [0 x i32], ptr %75, i64 0, i64 %107
  store i32 %106, ptr %108, align 4, !tbaa !12
  %109 = icmp samesign ugt i64 %103, 1
  br i1 %109, label %102, label %110, !llvm.loop !31

110:                                              ; preds = %102, %98, %71
  %111 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %112 = lshr i32 %8, 3
  %113 = and i32 %112, 268435452
  %114 = zext nneg i32 %113 to i64
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %111, i8 0, i64 %114, i1 false), !tbaa !12
  %115 = add nsw i32 %72, %9
  store i32 %115, ptr %0, align 4, !tbaa !12
  br label %316

116:                                              ; preds = %5
  %117 = icmp samesign ult i32 %2, 2
  %118 = load i32, ptr %0, align 4, !tbaa !12
  br i1 %117, label %119, label %186

119:                                              ; preds = %116
  %120 = add nsw i32 %1, -1
  %121 = lshr i32 %120, 5
  %122 = icmp slt i32 %121, %118
  br i1 %122, label %123, label %186

123:                                              ; preds = %119
  %124 = and i32 %120, 31
  %125 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %126 = zext nneg i32 %121 to i64
  %127 = getelementptr inbounds nuw [0 x i32], ptr %125, i64 0, i64 %126
  %128 = load i32, ptr %127, align 4, !tbaa !12
  %129 = shl nuw i32 1, %124
  %130 = and i32 %128, %129
  %131 = icmp eq i32 %130, 0
  br i1 %131, label %186, label %132

132:                                              ; preds = %123
  %133 = icmp eq i32 %2, 1
  br i1 %133, label %186, label %134

134:                                              ; preds = %132
  %135 = icmp eq i32 %1, 1
  br i1 %135, label %176, label %136

136:                                              ; preds = %134
  %137 = icmp samesign ult i32 %1, 33
  br i1 %137, label %169, label %138

138:                                              ; preds = %136
  %139 = icmp ult i32 %1, 257
  br i1 %139, label %158, label %140

140:                                              ; preds = %138
  %141 = and i64 %126, 134217720
  br label %142

142:                                              ; preds = %142, %140
  %143 = phi i64 [ 0, %140 ], [ %152, %142 ]
  %144 = phi <4 x i32> [ zeroinitializer, %140 ], [ %150, %142 ]
  %145 = phi <4 x i32> [ zeroinitializer, %140 ], [ %151, %142 ]
  %146 = getelementptr inbounds nuw [0 x i32], ptr %125, i64 0, i64 %143
  %147 = getelementptr inbounds nuw i8, ptr %146, i64 16
  %148 = load <4 x i32>, ptr %146, align 4, !tbaa !12
  %149 = load <4 x i32>, ptr %147, align 4, !tbaa !12
  %150 = or <4 x i32> %148, %144
  %151 = or <4 x i32> %149, %145
  %152 = add nuw i64 %143, 8
  %153 = icmp eq i64 %152, %141
  br i1 %153, label %154, label %142, !llvm.loop !32

154:                                              ; preds = %142
  %155 = or <4 x i32> %151, %150
  %156 = tail call i32 @llvm.vector.reduce.or.v4i32(<4 x i32> %155)
  %157 = icmp eq i64 %141, %126
  br i1 %157, label %169, label %158

158:                                              ; preds = %138, %154
  %159 = phi i64 [ 0, %138 ], [ %141, %154 ]
  %160 = phi i32 [ 0, %138 ], [ %156, %154 ]
  br label %161

161:                                              ; preds = %158, %161
  %162 = phi i64 [ %167, %161 ], [ %159, %158 ]
  %163 = phi i32 [ %166, %161 ], [ %160, %158 ]
  %164 = getelementptr inbounds nuw [0 x i32], ptr %125, i64 0, i64 %162
  %165 = load i32, ptr %164, align 4, !tbaa !12
  %166 = or i32 %165, %163
  %167 = add nuw nsw i64 %162, 1
  %168 = icmp eq i64 %167, %126
  br i1 %168, label %169, label %161, !llvm.loop !33

169:                                              ; preds = %161, %154, %136
  %170 = phi i32 [ 0, %136 ], [ %156, %154 ], [ %166, %161 ]
  %171 = shl nsw i32 -1, %124
  %172 = xor i32 %171, -1
  %173 = and i32 %128, %172
  %174 = or i32 %173, %170
  %175 = icmp eq i32 %174, 0
  br i1 %175, label %176, label %186

176:                                              ; preds = %134, %169
  %177 = lshr i32 %1, 5
  %178 = icmp samesign ult i32 %177, %118
  br i1 %178, label %179, label %186

179:                                              ; preds = %176
  %180 = and i32 %1, 31
  %181 = zext nneg i32 %177 to i64
  %182 = getelementptr inbounds nuw [0 x i32], ptr %125, i64 0, i64 %181
  %183 = load i32, ptr %182, align 4, !tbaa !12
  %184 = lshr i32 %183, %180
  %185 = and i32 %184, 1
  br label %186

186:                                              ; preds = %132, %119, %179, %176, %123, %169, %116
  %187 = phi i32 [ 0, %116 ], [ 1, %169 ], [ 0, %123 ], [ %185, %179 ], [ 0, %176 ], [ 0, %119 ], [ 1, %132 ]
  %188 = lshr i32 %1, 5
  %189 = and i32 %1, 31
  %190 = icmp slt i32 %188, %118
  br i1 %190, label %193, label %191

191:                                              ; preds = %186
  store i32 1, ptr %0, align 4, !tbaa !12
  %192 = getelementptr inbounds nuw i8, ptr %0, i64 4
  store i32 %187, ptr %192, align 4, !tbaa !12
  br label %316

193:                                              ; preds = %186
  %194 = icmp samesign ult i32 %1, 32
  br i1 %194, label %228, label %195

195:                                              ; preds = %193
  %196 = sub nsw i32 %118, %188
  store i32 %196, ptr %0, align 4, !tbaa !12
  %197 = icmp sgt i32 %196, 0
  br i1 %197, label %198, label %228

198:                                              ; preds = %195
  %199 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %200 = zext nneg i32 %188 to i64
  %201 = zext nneg i32 %196 to i64
  %202 = icmp ult i32 %196, 8
  br i1 %202, label %218, label %203

203:                                              ; preds = %198
  %204 = and i64 %201, 2147483640
  br label %205

205:                                              ; preds = %205, %203
  %206 = phi i64 [ 0, %203 ], [ %214, %205 ]
  %207 = add nuw nsw i64 %206, %200
  %208 = getelementptr inbounds nuw [0 x i32], ptr %199, i64 0, i64 %207
  %209 = getelementptr inbounds nuw i8, ptr %208, i64 16
  %210 = load <4 x i32>, ptr %208, align 4, !tbaa !12
  %211 = load <4 x i32>, ptr %209, align 4, !tbaa !12
  %212 = getelementptr inbounds nuw [0 x i32], ptr %199, i64 0, i64 %206
  %213 = getelementptr inbounds nuw i8, ptr %212, i64 16
  store <4 x i32> %210, ptr %212, align 4, !tbaa !12
  store <4 x i32> %211, ptr %213, align 4, !tbaa !12
  %214 = add nuw i64 %206, 8
  %215 = icmp eq i64 %214, %204
  br i1 %215, label %216, label %205, !llvm.loop !34

216:                                              ; preds = %205
  %217 = icmp eq i64 %204, %201
  br i1 %217, label %228, label %218

218:                                              ; preds = %198, %216
  %219 = phi i64 [ 0, %198 ], [ %204, %216 ]
  br label %220

220:                                              ; preds = %218, %220
  %221 = phi i64 [ %226, %220 ], [ %219, %218 ]
  %222 = add nuw nsw i64 %221, %200
  %223 = getelementptr inbounds nuw [0 x i32], ptr %199, i64 0, i64 %222
  %224 = load i32, ptr %223, align 4, !tbaa !12
  %225 = getelementptr inbounds nuw [0 x i32], ptr %199, i64 0, i64 %221
  store i32 %224, ptr %225, align 4, !tbaa !12
  %226 = add nuw nsw i64 %221, 1
  %227 = icmp eq i64 %226, %201
  br i1 %227, label %228, label %220, !llvm.loop !35

228:                                              ; preds = %220, %216, %195, %193
  %229 = phi i32 [ %196, %195 ], [ %118, %193 ], [ %196, %216 ], [ %196, %220 ]
  %230 = icmp eq i32 %189, 0
  br i1 %230, label %296, label %231

231:                                              ; preds = %228
  %232 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %233 = icmp sgt i32 %229, 0
  br i1 %233, label %234, label %296

234:                                              ; preds = %231
  %235 = zext nneg i32 %229 to i64
  %236 = and i64 %235, 3
  %237 = icmp ult i32 %229, 4
  br i1 %237, label %262, label %238

238:                                              ; preds = %234
  %239 = and i64 %235, 2147483644
  %240 = getelementptr i8, ptr %0, i64 -4
  %241 = getelementptr i8, ptr %0, i64 -8
  %242 = getelementptr i8, ptr %0, i64 -12
  br label %243

243:                                              ; preds = %243, %238
  %244 = phi i64 [ %235, %238 ], [ %256, %243 ]
  %245 = phi i32 [ 0, %238 ], [ %258, %243 ]
  %246 = phi i64 [ 0, %238 ], [ %260, %243 ]
  %247 = getelementptr i32, ptr %0, i64 %244
  %248 = load i32, ptr %247, align 4, !tbaa !12
  %249 = tail call i32 @llvm.fshr.i32(i32 %245, i32 %248, i32 range(i32 1, 32) %1)
  store i32 %249, ptr %247, align 4, !tbaa !12
  %250 = getelementptr i32, ptr %240, i64 %244
  %251 = load i32, ptr %250, align 4, !tbaa !12
  %252 = tail call i32 @llvm.fshr.i32(i32 %248, i32 %251, i32 range(i32 1, 32) %1)
  store i32 %252, ptr %250, align 4, !tbaa !12
  %253 = getelementptr i32, ptr %241, i64 %244
  %254 = load i32, ptr %253, align 4, !tbaa !12
  %255 = tail call i32 @llvm.fshr.i32(i32 %251, i32 %254, i32 range(i32 1, 32) %1)
  store i32 %255, ptr %253, align 4, !tbaa !12
  %256 = add nsw i64 %244, -4
  %257 = getelementptr i32, ptr %242, i64 %244
  %258 = load i32, ptr %257, align 4, !tbaa !12
  %259 = tail call i32 @llvm.fshr.i32(i32 %254, i32 %258, i32 range(i32 1, 32) %1)
  store i32 %259, ptr %257, align 4, !tbaa !12
  %260 = add i64 %246, 4
  %261 = icmp eq i64 %260, %239
  br i1 %261, label %262, label %243, !llvm.loop !36

262:                                              ; preds = %243, %234
  %263 = phi i64 [ %235, %234 ], [ %256, %243 ]
  %264 = phi i32 [ 0, %234 ], [ %258, %243 ]
  %265 = icmp eq i64 %236, 0
  br i1 %265, label %276, label %266

266:                                              ; preds = %262, %266
  %267 = phi i64 [ %270, %266 ], [ %263, %262 ]
  %268 = phi i32 [ %272, %266 ], [ %264, %262 ]
  %269 = phi i64 [ %274, %266 ], [ 0, %262 ]
  %270 = add nsw i64 %267, -1
  %271 = getelementptr i32, ptr %0, i64 %267
  %272 = load i32, ptr %271, align 4, !tbaa !12
  %273 = tail call i32 @llvm.fshr.i32(i32 %268, i32 %272, i32 range(i32 1, 32) %1)
  store i32 %273, ptr %271, align 4, !tbaa !12
  %274 = add i64 %269, 1
  %275 = icmp eq i64 %274, %236
  br i1 %275, label %276, label %266, !llvm.loop !37

276:                                              ; preds = %266, %262
  %277 = load i32, ptr %0, align 4, !tbaa !12
  %278 = icmp sgt i32 %277, 1
  br i1 %278, label %279, label %296

279:                                              ; preds = %276
  %280 = zext nneg i32 %277 to i64
  %281 = add nsw i64 %280, -1
  %282 = getelementptr inbounds nuw [0 x i32], ptr %232, i64 0, i64 %281
  %283 = load i32, ptr %282, align 4, !tbaa !12
  %284 = icmp eq i32 %283, 0
  br i1 %284, label %290, label %296

285:                                              ; preds = %290
  %286 = add nsw i64 %291, -1
  %287 = getelementptr inbounds nuw [0 x i32], ptr %232, i64 0, i64 %286
  %288 = load i32, ptr %287, align 4, !tbaa !12
  %289 = icmp eq i32 %288, 0
  br i1 %289, label %290, label %296, !llvm.loop !19

290:                                              ; preds = %279, %285
  %291 = phi i64 [ %286, %285 ], [ %281, %279 ]
  %292 = phi i64 [ %291, %285 ], [ %280, %279 ]
  %293 = trunc nuw nsw i64 %291 to i32
  store i32 %293, ptr %0, align 4, !tbaa !12
  %294 = icmp samesign ugt i64 %292, 2
  br i1 %294, label %285, label %295, !llvm.loop !19

295:                                              ; preds = %290
  br label %296, !llvm.loop !19

296:                                              ; preds = %285, %279, %295, %231, %276, %228
  %297 = phi i32 [ %277, %276 ], [ %229, %228 ], [ %229, %231 ], [ %293, %295 ], [ %277, %279 ], [ %293, %285 ]
  %298 = icmp eq i32 %187, 0
  br i1 %298, label %316, label %299

299:                                              ; preds = %296
  %300 = getelementptr inbounds nuw i8, ptr %0, i64 4
  %301 = sext i32 %297 to i64
  %302 = icmp eq i32 %297, 0
  br i1 %302, label %313, label %303

303:                                              ; preds = %299, %303
  %304 = phi i64 [ %309, %303 ], [ 0, %299 ]
  %305 = getelementptr inbounds nuw i32, ptr %300, i64 %304
  %306 = load i32, ptr %305, align 4, !tbaa !12
  %307 = add i32 %306, 1
  %308 = icmp eq i32 %307, 0
  store i32 %307, ptr %305, align 4, !tbaa !12
  %309 = add nuw i64 %304, 1
  %310 = icmp ult i64 %309, %301
  %311 = select i1 %310, i1 %308, i1 false
  br i1 %311, label %303, label %312, !llvm.loop !39

312:                                              ; preds = %303
  br i1 %308, label %313, label %316

313:                                              ; preds = %299, %312
  %314 = add nsw i32 %297, 1
  store i32 %314, ptr %0, align 4, !tbaa !12
  %315 = getelementptr inbounds [0 x i32], ptr %300, i64 0, i64 %301
  store i32 1, ptr %315, align 4, !tbaa !12
  br label %316

316:                                              ; preds = %110, %69, %312, %313, %296, %191, %3
  ret void
}

; Function Attrs: nounwind uwtable
define dso_local double @js_atod(ptr noundef %0, ptr noundef writeonly captures(address_is_null) %1, i32 noundef %2, i32 noundef %3, ptr noundef captures(none) %4) local_unnamed_addr #5 {
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %6) #10
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %7) #10
  %8 = and i32 %3, 8
  %9 = icmp eq i32 %8, 0
  %10 = select i1 %9, i32 256, i32 95
  store ptr %0, ptr %6, align 8, !tbaa !40
  %11 = load i8, ptr %0, align 1, !tbaa !5
  switch i8 %11, label %17 [
    i8 43, label %13
    i8 45, label %12
  ]

12:                                               ; preds = %5
  br label %13

13:                                               ; preds = %5, %12
  %14 = phi i64 [ -9223372036854775808, %12 ], [ 0, %5 ]
  %15 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store ptr %15, ptr %6, align 8, !tbaa !40
  %16 = load i8, ptr %15, align 1, !tbaa !5
  br label %17

17:                                               ; preds = %13, %5
  %18 = phi i8 [ %16, %13 ], [ %11, %5 ]
  %19 = phi i64 [ %14, %13 ], [ 0, %5 ]
  %20 = phi ptr [ %15, %13 ], [ %0, %5 ]
  %21 = icmp eq i8 %18, 48
  br i1 %21, label %22, label %94

22:                                               ; preds = %17
  %23 = getelementptr inbounds nuw i8, ptr %20, i64 1
  %24 = load i8, ptr %23, align 1, !tbaa !5
  switch i8 %24, label %32 [
    i8 120, label %25
    i8 88, label %25
    i8 111, label %36
  ]

25:                                               ; preds = %22, %22
  %26 = and i32 %2, -17
  %27 = icmp eq i32 %26, 0
  br i1 %27, label %28, label %30

28:                                               ; preds = %25
  %29 = getelementptr inbounds nuw i8, ptr %20, i64 2
  br label %73

30:                                               ; preds = %25
  %31 = icmp eq i8 %24, 111
  br i1 %31, label %103, label %43

32:                                               ; preds = %22
  %33 = icmp eq i8 %24, 79
  %34 = icmp eq i32 %2, 0
  %35 = and i1 %34, %33
  br i1 %35, label %38, label %43

36:                                               ; preds = %22
  %37 = icmp eq i32 %2, 0
  br i1 %37, label %38, label %103

38:                                               ; preds = %32, %36
  %39 = and i32 %3, 2
  %40 = icmp eq i32 %39, 0
  br i1 %40, label %103, label %41

41:                                               ; preds = %38
  %42 = getelementptr inbounds nuw i8, ptr %20, i64 2
  br label %73

43:                                               ; preds = %30, %32
  %44 = phi i1 [ %34, %32 ], [ false, %30 ]
  %45 = icmp eq i8 %24, 98
  br i1 %45, label %49, label %46

46:                                               ; preds = %43
  %47 = icmp eq i8 %24, 66
  %48 = and i1 %44, %47
  br i1 %48, label %50, label %55

49:                                               ; preds = %43
  br i1 %44, label %50, label %103

50:                                               ; preds = %46, %49
  %51 = and i32 %3, 2
  %52 = icmp eq i32 %51, 0
  br i1 %52, label %103, label %53

53:                                               ; preds = %50
  %54 = getelementptr inbounds nuw i8, ptr %20, i64 2
  br label %73

55:                                               ; preds = %46
  %56 = icmp sgt i8 %24, 47
  %57 = icmp samesign ult i8 %24, 58
  %58 = and i1 %44, %57
  %59 = select i1 %56, i1 %58, i1 false
  br i1 %59, label %60, label %100

60:                                               ; preds = %55
  %61 = and i32 %3, 4
  %62 = icmp eq i32 %61, 0
  br i1 %62, label %103, label %63

63:                                               ; preds = %60, %63
  %64 = phi i64 [ %69, %63 ], [ 1, %60 ]
  %65 = getelementptr inbounds nuw i8, ptr %20, i64 %64
  %66 = load i8, ptr %65, align 1, !tbaa !5
  %67 = and i8 %66, -8
  %68 = icmp eq i8 %67, 48
  %69 = add nuw nsw i64 %64, 1
  br i1 %68, label %63, label %70, !llvm.loop !43

70:                                               ; preds = %63
  %71 = and i8 %66, -2
  %72 = icmp eq i8 %71, 56
  br i1 %72, label %103, label %73

73:                                               ; preds = %70, %41, %53, %28
  %74 = phi ptr [ %42, %41 ], [ %54, %53 ], [ %29, %28 ], [ %23, %70 ]
  %75 = phi i32 [ %10, %41 ], [ %10, %53 ], [ %10, %28 ], [ 256, %70 ]
  %76 = phi i32 [ 8, %41 ], [ 2, %53 ], [ 16, %28 ], [ 8, %70 ]
  store ptr %74, ptr %6, align 8, !tbaa !40
  %77 = load i8, ptr %74, align 1, !tbaa !5
  %78 = zext i8 %77 to i32
  %79 = add nsw i32 %78, -48
  %80 = icmp ult i32 %79, 10
  br i1 %80, label %91, label %81

81:                                               ; preds = %73
  %82 = add i8 %77, -65
  %83 = icmp ult i8 %82, 26
  br i1 %83, label %84, label %86

84:                                               ; preds = %81
  %85 = add nsw i32 %78, -55
  br label %91

86:                                               ; preds = %81
  %87 = add i8 %77, -97
  %88 = icmp ult i8 %87, 26
  %89 = add nsw i32 %78, -87
  %90 = select i1 %88, i32 %89, i32 36
  br label %91

91:                                               ; preds = %73, %84, %86
  %92 = phi i32 [ %85, %84 ], [ %90, %86 ], [ %79, %73 ]
  %93 = icmp slt i32 %92, %76
  br i1 %93, label %103, label %690

94:                                               ; preds = %17
  %95 = and i32 %3, 1
  %96 = icmp eq i32 %95, 0
  br i1 %96, label %97, label %100

97:                                               ; preds = %94
  %98 = call i32 @strstart(ptr noundef nonnull %20, ptr noundef nonnull @.str.2, ptr noundef nonnull %6) #10
  %99 = icmp eq i32 %98, 0
  br i1 %99, label %100, label %686

100:                                              ; preds = %94, %97, %55
  %101 = icmp eq i32 %2, 0
  %102 = select i1 %101, i32 10, i32 %2
  br label %103

103:                                              ; preds = %30, %38, %100, %49, %36, %50, %70, %60, %91
  %104 = phi i32 [ %75, %91 ], [ 256, %70 ], [ %10, %60 ], [ %10, %50 ], [ %10, %36 ], [ %10, %49 ], [ %10, %100 ], [ %10, %38 ], [ %10, %30 ]
  %105 = phi i32 [ %76, %91 ], [ 10, %70 ], [ 10, %60 ], [ 10, %50 ], [ %2, %36 ], [ %2, %49 ], [ %102, %100 ], [ 10, %38 ], [ %2, %30 ]
  %106 = add nsw i32 %105, -2
  %107 = sext i32 %106 to i64
  %108 = getelementptr inbounds [35 x i8], ptr @atod_max_digits_table, i64 0, i64 %107
  %109 = load i8, ptr %108, align 1, !tbaa !5
  %110 = zext i8 %109 to i32
  %111 = getelementptr inbounds [35 x i8], ptr @digits_per_limb_table, i64 0, i64 %107
  %112 = load i8, ptr %111, align 1, !tbaa !5
  %113 = zext i8 %112 to i32
  %114 = getelementptr inbounds [35 x i32], ptr @radix_base_table, i64 0, i64 %107
  %115 = load i32, ptr %114, align 4, !tbaa !12
  %116 = and i32 %105, 1
  %117 = icmp eq i32 %116, 0
  br i1 %117, label %118, label %127

118:                                              ; preds = %103, %118
  %119 = phi i32 [ %122, %118 ], [ 0, %103 ]
  %120 = phi i32 [ %121, %118 ], [ %105, %103 ]
  %121 = lshr exact i32 %120, 1
  %122 = add nuw nsw i32 %119, 1
  %123 = icmp ne i32 %120, 0
  %124 = and i32 %120, 2
  %125 = icmp eq i32 %124, 0
  %126 = and i1 %123, %125
  br i1 %126, label %118, label %127, !llvm.loop !14

127:                                              ; preds = %118, %103
  %128 = phi i32 [ 0, %103 ], [ %122, %118 ]
  %129 = ashr i32 %105, %128
  %130 = icmp eq i32 %129, 1
  %131 = select i1 %130, i32 %128, i32 0
  store i32 1, ptr %4, align 4, !tbaa !12
  %132 = getelementptr inbounds nuw i8, ptr %4, i64 4
  store i32 0, ptr %132, align 4, !tbaa !12
  %133 = load ptr, ptr %6, align 8, !tbaa !40
  %134 = and i32 %3, 1
  %135 = icmp eq i32 %134, 0
  br label %136

136:                                              ; preds = %188, %127
  %137 = phi ptr [ %133, %127 ], [ %189, %188 ]
  %138 = phi i32 [ -1, %127 ], [ %173, %188 ]
  %139 = phi i32 [ 0, %127 ], [ %190, %188 ]
  %140 = load i8, ptr %137, align 1, !tbaa !5
  %141 = icmp eq i8 %140, 46
  br i1 %141, label %142, label %170

142:                                              ; preds = %136
  %143 = icmp ugt ptr %137, %20
  br i1 %143, label %164, label %144

144:                                              ; preds = %142
  %145 = getelementptr inbounds nuw i8, ptr %137, i64 1
  %146 = load i8, ptr %145, align 1, !tbaa !5
  %147 = sext i8 %146 to i32
  %148 = add nsw i32 %147, -48
  %149 = icmp ult i32 %148, 10
  br i1 %149, label %160, label %150

150:                                              ; preds = %144
  %151 = add nsw i32 %147, -65
  %152 = icmp ult i32 %151, 26
  br i1 %152, label %153, label %155

153:                                              ; preds = %150
  %154 = add nsw i32 %147, -55
  br label %160

155:                                              ; preds = %150
  %156 = add nsw i32 %147, -97
  %157 = icmp ult i32 %156, 26
  %158 = add nsw i32 %147, -87
  %159 = select i1 %157, i32 %158, i32 36
  br label %160

160:                                              ; preds = %144, %153, %155
  %161 = phi i32 [ %154, %153 ], [ %159, %155 ], [ %148, %144 ]
  %162 = icmp slt i32 %161, %105
  %163 = and i1 %135, %162
  br i1 %163, label %165, label %170

164:                                              ; preds = %142
  br i1 %135, label %165, label %170

165:                                              ; preds = %164, %160
  %166 = icmp sgt i32 %138, -1
  br i1 %166, label %191, label %167

167:                                              ; preds = %165
  %168 = getelementptr inbounds nuw i8, ptr %137, i64 1
  store ptr %168, ptr %6, align 8, !tbaa !40
  %169 = load i8, ptr %168, align 1, !tbaa !5
  br label %170

170:                                              ; preds = %167, %164, %160, %136
  %171 = phi i8 [ 46, %164 ], [ %169, %167 ], [ 46, %160 ], [ %140, %136 ]
  %172 = phi ptr [ %137, %164 ], [ %168, %167 ], [ %137, %160 ], [ %137, %136 ]
  %173 = phi i32 [ %138, %164 ], [ %139, %167 ], [ %138, %160 ], [ %138, %136 ]
  %174 = sext i8 %171 to i32
  %175 = icmp eq i32 %104, %174
  %176 = icmp ugt ptr %172, %20
  %177 = and i1 %175, %176
  br i1 %177, label %178, label %184

178:                                              ; preds = %170
  %179 = getelementptr inbounds nuw i8, ptr %172, i64 1
  %180 = load i8, ptr %179, align 1, !tbaa !5
  %181 = icmp eq i8 %180, 48
  br i1 %181, label %182, label %184

182:                                              ; preds = %178
  store ptr %179, ptr %6, align 8, !tbaa !40
  %183 = load i8, ptr %179, align 1, !tbaa !5
  br label %184

184:                                              ; preds = %182, %178, %170
  %185 = phi i8 [ %183, %182 ], [ %171, %178 ], [ %171, %170 ]
  %186 = phi ptr [ %179, %182 ], [ %172, %178 ], [ %172, %170 ]
  %187 = icmp eq i8 %185, 48
  br i1 %187, label %188, label %191

188:                                              ; preds = %184
  %189 = getelementptr inbounds nuw i8, ptr %186, i64 1
  store ptr %189, ptr %6, align 8, !tbaa !40
  %190 = add nuw nsw i32 %139, 1
  br label %136

191:                                              ; preds = %184, %165
  %192 = phi i8 [ %185, %184 ], [ 46, %165 ]
  %193 = phi ptr [ %186, %184 ], [ %137, %165 ]
  %194 = phi i32 [ %173, %184 ], [ %138, %165 ]
  %195 = shl nuw i64 1, %107
  %196 = and i64 %195, 16389
  %197 = icmp eq i64 %196, 0
  %198 = zext i32 %115 to i64
  %199 = getelementptr i8, ptr %4, i64 8
  %200 = getelementptr inbounds i8, ptr %132, i64 4
  br label %201

201:                                              ; preds = %395, %191
  %202 = phi i32 [ 1, %191 ], [ %396, %395 ]
  %203 = phi i32 [ 1, %191 ], [ %397, %395 ]
  %204 = phi i32 [ 1, %191 ], [ %398, %395 ]
  %205 = phi i8 [ %192, %191 ], [ %403, %395 ]
  %206 = phi ptr [ %193, %191 ], [ %291, %395 ]
  %207 = phi i32 [ 0, %191 ], [ %399, %395 ]
  %208 = phi i32 [ 0, %191 ], [ %400, %395 ]
  %209 = phi i32 [ 0, %191 ], [ %401, %395 ]
  %210 = phi i32 [ 0, %191 ], [ %402, %395 ]
  %211 = phi i32 [ %194, %191 ], [ %245, %395 ]
  %212 = phi i32 [ %139, %191 ], [ %292, %395 ]
  %213 = icmp eq i8 %205, 46
  br i1 %213, label %214, label %242

214:                                              ; preds = %201
  %215 = icmp ugt ptr %206, %20
  br i1 %215, label %236, label %216

216:                                              ; preds = %214
  %217 = getelementptr inbounds nuw i8, ptr %206, i64 1
  %218 = load i8, ptr %217, align 1, !tbaa !5
  %219 = sext i8 %218 to i32
  %220 = add nsw i32 %219, -48
  %221 = icmp ult i32 %220, 10
  br i1 %221, label %232, label %222

222:                                              ; preds = %216
  %223 = add nsw i32 %219, -65
  %224 = icmp ult i32 %223, 26
  br i1 %224, label %225, label %227

225:                                              ; preds = %222
  %226 = add nsw i32 %219, -55
  br label %232

227:                                              ; preds = %222
  %228 = add nsw i32 %219, -97
  %229 = icmp ult i32 %228, 26
  %230 = add nsw i32 %219, -87
  %231 = select i1 %229, i32 %230, i32 36
  br label %232

232:                                              ; preds = %216, %225, %227
  %233 = phi i32 [ %226, %225 ], [ %231, %227 ], [ %220, %216 ]
  %234 = icmp slt i32 %233, %105
  %235 = and i1 %135, %234
  br i1 %235, label %237, label %242

236:                                              ; preds = %214
  br i1 %135, label %237, label %242

237:                                              ; preds = %236, %232
  %238 = icmp sgt i32 %211, -1
  br i1 %238, label %404, label %239

239:                                              ; preds = %237
  %240 = getelementptr inbounds nuw i8, ptr %206, i64 1
  store ptr %240, ptr %6, align 8, !tbaa !40
  %241 = load i8, ptr %240, align 1, !tbaa !5
  br label %242

242:                                              ; preds = %239, %236, %232, %201
  %243 = phi i8 [ 46, %236 ], [ %241, %239 ], [ 46, %232 ], [ %205, %201 ]
  %244 = phi ptr [ %206, %236 ], [ %240, %239 ], [ %206, %232 ], [ %206, %201 ]
  %245 = phi i32 [ %211, %236 ], [ %212, %239 ], [ %211, %232 ], [ %211, %201 ]
  %246 = sext i8 %243 to i32
  %247 = icmp eq i32 %104, %246
  %248 = icmp ugt ptr %244, %20
  %249 = select i1 %247, i1 %248, i1 false
  br i1 %249, label %250, label %272

250:                                              ; preds = %242
  %251 = getelementptr inbounds nuw i8, ptr %244, i64 1
  %252 = load i8, ptr %251, align 1, !tbaa !5
  %253 = sext i8 %252 to i32
  %254 = add nsw i32 %253, -48
  %255 = icmp ult i32 %254, 10
  br i1 %255, label %266, label %256

256:                                              ; preds = %250
  %257 = add nsw i32 %253, -65
  %258 = icmp ult i32 %257, 26
  br i1 %258, label %259, label %261

259:                                              ; preds = %256
  %260 = add nsw i32 %253, -55
  br label %266

261:                                              ; preds = %256
  %262 = add nsw i32 %253, -97
  %263 = icmp ult i32 %262, 26
  %264 = add nsw i32 %253, -87
  %265 = select i1 %263, i32 %264, i32 36
  br label %266

266:                                              ; preds = %250, %259, %261
  %267 = phi i32 [ %260, %259 ], [ %265, %261 ], [ %254, %250 ]
  %268 = icmp slt i32 %267, %105
  br i1 %268, label %269, label %272

269:                                              ; preds = %266
  store ptr %251, ptr %6, align 8, !tbaa !40
  %270 = load i8, ptr %251, align 1, !tbaa !5
  %271 = sext i8 %270 to i32
  br label %272

272:                                              ; preds = %269, %266, %242
  %273 = phi i32 [ %271, %269 ], [ %246, %266 ], [ %246, %242 ]
  %274 = phi ptr [ %251, %269 ], [ %244, %266 ], [ %244, %242 ]
  %275 = add nsw i32 %273, -48
  %276 = icmp ult i32 %275, 10
  br i1 %276, label %287, label %277

277:                                              ; preds = %272
  %278 = add nsw i32 %273, -65
  %279 = icmp ult i32 %278, 26
  br i1 %279, label %280, label %282

280:                                              ; preds = %277
  %281 = add nsw i32 %273, -55
  br label %287

282:                                              ; preds = %277
  %283 = add nsw i32 %273, -97
  %284 = icmp ult i32 %283, 26
  %285 = add nsw i32 %273, -87
  %286 = select i1 %284, i32 %285, i32 36
  br label %287

287:                                              ; preds = %272, %280, %282
  %288 = phi i32 [ %281, %280 ], [ %286, %282 ], [ %275, %272 ]
  %289 = icmp ult i32 %288, %105
  br i1 %289, label %290, label %404

290:                                              ; preds = %287
  %291 = getelementptr inbounds nuw i8, ptr %274, i64 1
  store ptr %291, ptr %6, align 8, !tbaa !40
  %292 = add nuw nsw i32 %212, 1
  %293 = icmp slt i32 %208, %110
  br i1 %293, label %294, label %393

294:                                              ; preds = %290
  %295 = mul i32 %210, %105
  %296 = add i32 %288, %295
  %297 = add nsw i32 %207, 1
  %298 = icmp eq i32 %297, %113
  br i1 %298, label %299, label %386

299:                                              ; preds = %294
  %300 = load i32, ptr %132, align 4, !tbaa !12
  %301 = icmp eq i32 %300, 0
  br i1 %301, label %302, label %305

302:                                              ; preds = %299
  %303 = icmp eq i32 %203, 1
  br i1 %303, label %304, label %305

304:                                              ; preds = %302
  store i32 %296, ptr %132, align 4, !tbaa !12
  br label %386

305:                                              ; preds = %299, %302
  %306 = phi i32 [ %203, %302 ], [ %204, %299 ]
  br i1 %197, label %314, label %307

307:                                              ; preds = %305
  %308 = icmp sgt i32 %306, -1
  br i1 %308, label %309, label %313

309:                                              ; preds = %307
  %310 = zext nneg i32 %306 to i64
  %311 = shl nuw nsw i64 %310, 2
  %312 = add nuw nsw i64 %311, 4
  call void @llvm.memmove.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %199, ptr noundef nonnull align 4 dereferenceable(1) %132, i64 %312, i1 false), !tbaa !12
  br label %313

313:                                              ; preds = %309, %307
  store i32 %296, ptr %132, align 4, !tbaa !12
  br label %365

314:                                              ; preds = %305
  %315 = icmp eq i32 %306, 0
  br i1 %315, label %360, label %316

316:                                              ; preds = %314
  %317 = zext i32 %306 to i64
  %318 = and i64 %317, 1
  %319 = icmp eq i32 %306, 1
  br i1 %319, label %345, label %320

320:                                              ; preds = %316
  %321 = and i64 %317, 4294967294
  br label %322

322:                                              ; preds = %322, %320
  %323 = phi i64 [ 0, %320 ], [ %342, %322 ]
  %324 = phi i32 [ %296, %320 ], [ %341, %322 ]
  %325 = phi i64 [ 0, %320 ], [ %343, %322 ]
  %326 = getelementptr inbounds nuw i32, ptr %132, i64 %323
  %327 = load i32, ptr %326, align 4, !tbaa !12
  %328 = zext i32 %327 to i64
  %329 = mul nuw i64 %328, %198
  %330 = zext i32 %324 to i64
  %331 = add nuw i64 %329, %330
  %332 = trunc i64 %331 to i32
  store i32 %332, ptr %326, align 4, !tbaa !12
  %333 = lshr i64 %331, 32
  %334 = getelementptr inbounds i32, ptr %200, i64 %323
  %335 = load i32, ptr %334, align 4, !tbaa !12
  %336 = zext i32 %335 to i64
  %337 = mul nuw i64 %336, %198
  %338 = add nuw i64 %337, %333
  %339 = trunc i64 %338 to i32
  store i32 %339, ptr %334, align 4, !tbaa !12
  %340 = lshr i64 %338, 32
  %341 = trunc nuw i64 %340 to i32
  %342 = add nuw nsw i64 %323, 2
  %343 = add i64 %325, 2
  %344 = icmp eq i64 %343, %321
  br i1 %344, label %345, label %322, !llvm.loop !23

345:                                              ; preds = %322, %316
  %346 = phi i32 [ poison, %316 ], [ %341, %322 ]
  %347 = phi i64 [ 0, %316 ], [ %342, %322 ]
  %348 = phi i32 [ %296, %316 ], [ %341, %322 ]
  %349 = icmp eq i64 %318, 0
  br i1 %349, label %360, label %350

350:                                              ; preds = %345
  %351 = getelementptr inbounds nuw i32, ptr %132, i64 %347
  %352 = load i32, ptr %351, align 4, !tbaa !12
  %353 = zext i32 %352 to i64
  %354 = mul nuw i64 %353, %198
  %355 = zext i32 %348 to i64
  %356 = add nuw i64 %354, %355
  %357 = trunc i64 %356 to i32
  store i32 %357, ptr %351, align 4, !tbaa !12
  %358 = lshr i64 %356, 32
  %359 = trunc nuw i64 %358 to i32
  br label %360

360:                                              ; preds = %350, %345, %314
  %361 = phi i32 [ %296, %314 ], [ %346, %345 ], [ %359, %350 ]
  %362 = sext i32 %306 to i64
  %363 = getelementptr inbounds [0 x i32], ptr %132, i64 0, i64 %362
  store i32 %361, ptr %363, align 4, !tbaa !12
  %364 = load i32, ptr %4, align 4, !tbaa !12
  br label %365

365:                                              ; preds = %360, %313
  %366 = phi i32 [ %364, %360 ], [ %306, %313 ]
  %367 = add nsw i32 %366, 1
  store i32 %367, ptr %4, align 4, !tbaa !12
  %368 = icmp sgt i32 %366, 0
  br i1 %368, label %369, label %386

369:                                              ; preds = %365
  %370 = zext nneg i32 %367 to i64
  %371 = add nsw i64 %370, -1
  %372 = getelementptr inbounds nuw [0 x i32], ptr %132, i64 0, i64 %371
  %373 = load i32, ptr %372, align 4, !tbaa !12
  %374 = icmp eq i32 %373, 0
  br i1 %374, label %380, label %386

375:                                              ; preds = %380
  %376 = add nsw i64 %381, -1
  %377 = getelementptr inbounds nuw [0 x i32], ptr %132, i64 0, i64 %376
  %378 = load i32, ptr %377, align 4, !tbaa !12
  %379 = icmp eq i32 %378, 0
  br i1 %379, label %380, label %386, !llvm.loop !19

380:                                              ; preds = %369, %375
  %381 = phi i64 [ %376, %375 ], [ %371, %369 ]
  %382 = phi i64 [ %381, %375 ], [ %370, %369 ]
  %383 = trunc nuw nsw i64 %381 to i32
  store i32 %383, ptr %4, align 4, !tbaa !12
  %384 = icmp samesign ugt i64 %382, 2
  br i1 %384, label %375, label %385, !llvm.loop !19

385:                                              ; preds = %380
  br label %386, !llvm.loop !19

386:                                              ; preds = %375, %369, %385, %365, %304, %294
  %387 = phi i32 [ %202, %294 ], [ %202, %304 ], [ %367, %365 ], [ %383, %385 ], [ %367, %369 ], [ %383, %375 ]
  %388 = phi i32 [ %203, %294 ], [ 1, %304 ], [ %367, %365 ], [ %383, %385 ], [ %367, %369 ], [ %383, %375 ]
  %389 = phi i32 [ %204, %294 ], [ 1, %304 ], [ %367, %365 ], [ %383, %385 ], [ %367, %369 ], [ %383, %375 ]
  %390 = phi i32 [ %297, %294 ], [ 0, %304 ], [ 0, %365 ], [ 0, %385 ], [ 0, %369 ], [ 0, %375 ]
  %391 = phi i32 [ %296, %294 ], [ 0, %304 ], [ 0, %365 ], [ 0, %385 ], [ 0, %369 ], [ 0, %375 ]
  %392 = add nsw i32 %208, 1
  br label %395

393:                                              ; preds = %290
  %394 = or i32 %288, %209
  br label %395

395:                                              ; preds = %386, %393
  %396 = phi i32 [ %387, %386 ], [ %202, %393 ]
  %397 = phi i32 [ %388, %386 ], [ %203, %393 ]
  %398 = phi i32 [ %389, %386 ], [ %204, %393 ]
  %399 = phi i32 [ %390, %386 ], [ %207, %393 ]
  %400 = phi i32 [ %392, %386 ], [ %208, %393 ]
  %401 = phi i32 [ %209, %386 ], [ %394, %393 ]
  %402 = phi i32 [ %391, %386 ], [ %210, %393 ]
  %403 = load i8, ptr %291, align 1, !tbaa !5
  br label %201

404:                                              ; preds = %237, %287
  %405 = phi ptr [ %274, %287 ], [ %206, %237 ]
  %406 = phi i32 [ %245, %287 ], [ %211, %237 ]
  switch i32 %207, label %409 [
    i32 0, label %556
    i32 1, label %407
  ]

407:                                              ; preds = %404
  %408 = zext i32 %105 to i64
  br label %472

409:                                              ; preds = %404
  %410 = icmp eq i32 %105, 5
  %411 = icmp eq i32 %105, 10
  %412 = or i1 %410, %411
  %413 = icmp ult i32 %207, 18
  %414 = and i1 %412, %413
  br i1 %414, label %415, label %435

415:                                              ; preds = %409
  %416 = add nsw i32 %207, -1
  %417 = zext nneg i32 %416 to i64
  %418 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %417
  %419 = load i32, ptr %418, align 4, !tbaa !12
  %420 = zext i32 %419 to i64
  %421 = icmp samesign ugt i32 %207, 13
  br i1 %421, label %422, label %430

422:                                              ; preds = %415
  %423 = add nsw i32 %207, -14
  %424 = zext nneg i32 %423 to i64
  %425 = getelementptr inbounds nuw [4 x i8], ptr @pow5h_table, i64 0, i64 %424
  %426 = load i8, ptr %425, align 1, !tbaa !5
  %427 = zext i8 %426 to i64
  %428 = shl nuw nsw i64 %427, 32
  %429 = or disjoint i64 %428, %420
  br label %430

430:                                              ; preds = %422, %415
  %431 = phi i64 [ %429, %422 ], [ %420, %415 ]
  %432 = select i1 %411, i32 %207, i32 0
  %433 = zext nneg i32 %432 to i64
  %434 = shl nuw nsw i64 %431, %433
  br label %472

435:                                              ; preds = %409
  %436 = zext i32 %105 to i64
  %437 = call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %207, i1 false)
  %438 = sub nsw i32 30, %437
  %439 = and i32 %437, 1
  %440 = icmp eq i32 %439, 0
  br i1 %440, label %441, label %449

441:                                              ; preds = %435
  %442 = mul nuw i64 %436, %436
  %443 = shl nuw i32 1, %438
  %444 = and i32 %443, %207
  %445 = icmp eq i32 %444, 0
  %446 = select i1 %445, i64 1, i64 %436
  %447 = mul i64 %442, %446
  %448 = sub nsw i32 29, %437
  br label %449

449:                                              ; preds = %441, %435
  %450 = phi i64 [ poison, %435 ], [ %447, %441 ]
  %451 = phi i64 [ %436, %435 ], [ %447, %441 ]
  %452 = phi i32 [ %438, %435 ], [ %448, %441 ]
  %453 = icmp eq i32 %437, 30
  br i1 %453, label %472, label %454

454:                                              ; preds = %449, %454
  %455 = phi i64 [ %469, %454 ], [ %451, %449 ]
  %456 = phi i32 [ %470, %454 ], [ %452, %449 ]
  %457 = mul i64 %455, %455
  %458 = shl nuw i32 1, %456
  %459 = and i32 %458, %207
  %460 = icmp eq i32 %459, 0
  %461 = select i1 %460, i64 1, i64 %436
  %462 = mul i64 %457, %461
  %463 = add nsw i32 %456, -1
  %464 = mul i64 %462, %462
  %465 = shl nuw i32 1, %463
  %466 = and i32 %465, %207
  %467 = icmp eq i32 %466, 0
  %468 = select i1 %467, i64 1, i64 %436
  %469 = mul i64 %464, %468
  %470 = add nsw i32 %456, -2
  %471 = icmp eq i32 %463, 0
  br i1 %471, label %472, label %454, !llvm.loop !15

472:                                              ; preds = %449, %454, %407, %430
  %473 = phi i64 [ %408, %407 ], [ %434, %430 ], [ %450, %449 ], [ %469, %454 ]
  %474 = load i32, ptr %132, align 4, !tbaa !12
  %475 = icmp eq i32 %474, 0
  %476 = icmp eq i32 %202, 1
  %477 = select i1 %475, i1 %476, i1 false
  br i1 %477, label %478, label %479

478:                                              ; preds = %472
  store i32 %210, ptr %132, align 4, !tbaa !12
  br label %556

479:                                              ; preds = %472
  %480 = and i64 %473, 4294967295
  %481 = icmp eq i64 %480, 0
  br i1 %481, label %482, label %489

482:                                              ; preds = %479
  %483 = icmp sgt i32 %202, -1
  br i1 %483, label %484, label %488

484:                                              ; preds = %482
  %485 = zext nneg i32 %202 to i64
  %486 = shl nuw nsw i64 %485, 2
  %487 = add nuw nsw i64 %486, 4
  call void @llvm.memmove.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %199, ptr noundef nonnull align 4 dereferenceable(1) %132, i64 %487, i1 false), !tbaa !12
  br label %488

488:                                              ; preds = %484, %482
  store i32 %210, ptr %132, align 4, !tbaa !12
  br label %541

489:                                              ; preds = %479
  %490 = icmp eq i32 %202, 0
  br i1 %490, label %536, label %491

491:                                              ; preds = %489
  %492 = zext i32 %202 to i64
  %493 = and i64 %492, 1
  %494 = icmp eq i32 %202, 1
  br i1 %494, label %521, label %495

495:                                              ; preds = %491
  %496 = and i64 %492, 4294967294
  %497 = getelementptr inbounds i8, ptr %132, i64 4
  br label %498

498:                                              ; preds = %498, %495
  %499 = phi i64 [ 0, %495 ], [ %518, %498 ]
  %500 = phi i32 [ %210, %495 ], [ %517, %498 ]
  %501 = phi i64 [ 0, %495 ], [ %519, %498 ]
  %502 = getelementptr inbounds nuw i32, ptr %132, i64 %499
  %503 = load i32, ptr %502, align 4, !tbaa !12
  %504 = zext i32 %503 to i64
  %505 = mul nuw i64 %480, %504
  %506 = zext i32 %500 to i64
  %507 = add nuw i64 %505, %506
  %508 = trunc i64 %507 to i32
  store i32 %508, ptr %502, align 4, !tbaa !12
  %509 = lshr i64 %507, 32
  %510 = getelementptr inbounds i32, ptr %497, i64 %499
  %511 = load i32, ptr %510, align 4, !tbaa !12
  %512 = zext i32 %511 to i64
  %513 = mul nuw i64 %480, %512
  %514 = add nuw i64 %513, %509
  %515 = trunc i64 %514 to i32
  store i32 %515, ptr %510, align 4, !tbaa !12
  %516 = lshr i64 %514, 32
  %517 = trunc nuw i64 %516 to i32
  %518 = add nuw nsw i64 %499, 2
  %519 = add i64 %501, 2
  %520 = icmp eq i64 %519, %496
  br i1 %520, label %521, label %498, !llvm.loop !23

521:                                              ; preds = %498, %491
  %522 = phi i32 [ poison, %491 ], [ %517, %498 ]
  %523 = phi i64 [ 0, %491 ], [ %518, %498 ]
  %524 = phi i32 [ %210, %491 ], [ %517, %498 ]
  %525 = icmp eq i64 %493, 0
  br i1 %525, label %536, label %526

526:                                              ; preds = %521
  %527 = getelementptr inbounds nuw i32, ptr %132, i64 %523
  %528 = load i32, ptr %527, align 4, !tbaa !12
  %529 = zext i32 %528 to i64
  %530 = mul nuw i64 %480, %529
  %531 = zext i32 %524 to i64
  %532 = add nuw i64 %530, %531
  %533 = trunc i64 %532 to i32
  store i32 %533, ptr %527, align 4, !tbaa !12
  %534 = lshr i64 %532, 32
  %535 = trunc nuw i64 %534 to i32
  br label %536

536:                                              ; preds = %526, %521, %489
  %537 = phi i32 [ %210, %489 ], [ %522, %521 ], [ %535, %526 ]
  %538 = sext i32 %202 to i64
  %539 = getelementptr inbounds [0 x i32], ptr %132, i64 0, i64 %538
  store i32 %537, ptr %539, align 4, !tbaa !12
  %540 = load i32, ptr %4, align 4, !tbaa !12
  br label %541

541:                                              ; preds = %536, %488
  %542 = phi i32 [ %540, %536 ], [ %202, %488 ]
  %543 = add nsw i32 %542, 1
  store i32 %543, ptr %4, align 4, !tbaa !12
  %544 = icmp sgt i32 %542, 0
  br i1 %544, label %545, label %556

545:                                              ; preds = %541
  %546 = zext nneg i32 %543 to i64
  br label %547

547:                                              ; preds = %553, %545
  %548 = phi i64 [ %546, %545 ], [ %549, %553 ]
  %549 = add nsw i64 %548, -1
  %550 = getelementptr inbounds nuw [0 x i32], ptr %132, i64 0, i64 %549
  %551 = load i32, ptr %550, align 4, !tbaa !12
  %552 = icmp eq i32 %551, 0
  br i1 %552, label %553, label %556

553:                                              ; preds = %547
  %554 = trunc nuw nsw i64 %549 to i32
  store i32 %554, ptr %4, align 4, !tbaa !12
  %555 = icmp samesign ugt i64 %548, 2
  br i1 %555, label %547, label %556, !llvm.loop !19

556:                                              ; preds = %553, %547, %404, %541, %478
  %557 = icmp ne i32 %208, 0
  %558 = icmp slt i32 %406, 0
  %559 = select i1 %558, i32 %212, i32 %406
  %560 = add nsw i32 %208, %139
  %561 = sub i32 %560, %559
  %562 = icmp ne i32 %131, 0
  %563 = icmp ne i32 %209, 0
  %564 = select i1 %562, i1 %563, i1 false
  br i1 %564, label %565, label %568

565:                                              ; preds = %556
  %566 = load i32, ptr %132, align 4, !tbaa !12
  %567 = or i32 %566, 1
  store i32 %567, ptr %132, align 4, !tbaa !12
  br label %568

568:                                              ; preds = %565, %556
  br i1 %135, label %569, label %631

569:                                              ; preds = %568
  %570 = icmp eq i32 %105, 10
  %571 = load i8, ptr %405, align 1, !tbaa !5
  br i1 %570, label %572, label %573

572:                                              ; preds = %569
  switch i8 %571, label %631 [
    i8 101, label %579
    i8 69, label %579
  ]

573:                                              ; preds = %569
  %574 = icmp eq i8 %571, 64
  br i1 %574, label %579, label %575

575:                                              ; preds = %573
  %576 = add i32 %131, -1
  %577 = icmp ult i32 %576, 4
  br i1 %577, label %578, label %631

578:                                              ; preds = %575
  switch i8 %571, label %631 [
    i8 112, label %579
    i8 80, label %579
  ]

579:                                              ; preds = %578, %578, %572, %572, %573
  %580 = phi i8 [ %571, %578 ], [ %571, %578 ], [ %571, %572 ], [ %571, %572 ], [ 64, %573 ]
  %581 = icmp ugt ptr %405, %20
  br i1 %581, label %582, label %631

582:                                              ; preds = %579
  %583 = and i8 %580, -33
  %584 = icmp ne i8 %583, 80
  %585 = getelementptr inbounds nuw i8, ptr %405, i64 1
  store ptr %585, ptr %6, align 8, !tbaa !40
  %586 = load i8, ptr %585, align 1, !tbaa !5
  switch i8 %586, label %591 [
    i8 43, label %588
    i8 45, label %587
  ]

587:                                              ; preds = %582
  br label %588

588:                                              ; preds = %582, %587
  %589 = phi i1 [ false, %587 ], [ true, %582 ]
  %590 = getelementptr inbounds nuw i8, ptr %405, i64 2
  store ptr %590, ptr %6, align 8, !tbaa !40
  br label %591

591:                                              ; preds = %588, %582
  %592 = phi ptr [ %585, %582 ], [ %590, %588 ]
  %593 = phi i1 [ true, %582 ], [ %589, %588 ]
  %594 = load i8, ptr %592, align 1, !tbaa !5
  %595 = sext i8 %594 to i32
  %596 = add nsw i32 %595, -48
  %597 = icmp ult i32 %596, 10
  br i1 %597, label %598, label %690

598:                                              ; preds = %591, %625
  %599 = phi ptr [ %617, %625 ], [ %592, %591 ]
  %600 = phi i32 [ %630, %625 ], [ %596, %591 ]
  %601 = phi i1 [ %627, %625 ], [ false, %591 ]
  %602 = getelementptr inbounds nuw i8, ptr %599, i64 1
  store ptr %602, ptr %6, align 8, !tbaa !40
  %603 = load i8, ptr %602, align 1, !tbaa !5
  %604 = sext i8 %603 to i32
  %605 = icmp eq i32 %104, %604
  br i1 %605, label %606, label %615

606:                                              ; preds = %598
  %607 = getelementptr inbounds nuw i8, ptr %599, i64 2
  %608 = load i8, ptr %607, align 1, !tbaa !5
  %609 = sext i8 %608 to i32
  %610 = add nsw i32 %609, -48
  %611 = icmp ult i32 %610, 10
  br i1 %611, label %612, label %615

612:                                              ; preds = %606
  store ptr %607, ptr %6, align 8, !tbaa !40
  %613 = load i8, ptr %607, align 1, !tbaa !5
  %614 = sext i8 %613 to i32
  br label %615

615:                                              ; preds = %606, %612, %598
  %616 = phi i32 [ %604, %606 ], [ %614, %612 ], [ %604, %598 ]
  %617 = phi ptr [ %602, %606 ], [ %607, %612 ], [ %602, %598 ]
  %618 = add nsw i32 %616, -48
  %619 = icmp ult i32 %618, 10
  br i1 %619, label %625, label %620

620:                                              ; preds = %615
  %621 = sub nsw i32 0, %600
  %622 = select i1 %593, i32 %600, i32 %621
  %623 = and i1 %557, %601
  %624 = select i1 %593, i64 9218868437227405312, i64 0
  br i1 %623, label %686, label %631

625:                                              ; preds = %615
  %626 = icmp sgt i32 %600, 214748363
  %627 = select i1 %601, i1 true, i1 %626
  %628 = mul nsw i32 %600, 10
  %629 = add nsw i32 %618, %628
  %630 = select i1 %627, i32 %600, i32 %629
  br label %598

631:                                              ; preds = %620, %572, %578, %579, %575, %568
  %632 = phi ptr [ %405, %568 ], [ %405, %579 ], [ %405, %575 ], [ %405, %578 ], [ %405, %572 ], [ %617, %620 ]
  %633 = phi i32 [ 0, %568 ], [ 0, %579 ], [ 0, %575 ], [ 0, %578 ], [ 0, %572 ], [ %622, %620 ]
  %634 = phi i1 [ true, %568 ], [ true, %579 ], [ true, %575 ], [ true, %578 ], [ true, %572 ], [ %584, %620 ]
  %635 = icmp eq ptr %632, %20
  br i1 %635, label %690, label %636

636:                                              ; preds = %631
  br i1 %557, label %637, label %686

637:                                              ; preds = %636
  br i1 %562, label %638, label %652

638:                                              ; preds = %637
  %639 = select i1 %634, i32 %131, i32 1
  %640 = mul nsw i32 %639, %633
  %641 = mul nsw i32 %561, %131
  %642 = sub nsw i32 %640, %641
  %643 = mul nsw i32 %208, %131
  %644 = add nsw i32 %642, %643
  %645 = add nsw i32 %131, 1024
  %646 = icmp slt i32 %644, %645
  br i1 %646, label %647, label %686

647:                                              ; preds = %638
  %648 = icmp slt i32 %644, -1074
  br i1 %648, label %686, label %649

649:                                              ; preds = %647
  %650 = sub nsw i32 0, %642
  %651 = call fastcc i64 @round_to_d(ptr noundef %7, ptr noundef nonnull %4, i32 noundef %650)
  br label %666

652:                                              ; preds = %637
  %653 = sub nsw i32 %633, %561
  %654 = add nsw i32 %653, %208
  %655 = getelementptr inbounds [35 x i16], ptr @max_exponent, i64 0, i64 %107
  %656 = load i16, ptr %655, align 2, !tbaa !44
  %657 = sext i16 %656 to i32
  %658 = icmp sgt i32 %654, %657
  br i1 %658, label %686, label %659

659:                                              ; preds = %652
  %660 = getelementptr inbounds [35 x i16], ptr @min_exponent, i64 0, i64 %107
  %661 = load i16, ptr %660, align 2, !tbaa !44
  %662 = sext i16 %661 to i32
  %663 = icmp sgt i32 %654, %662
  br i1 %663, label %664, label %686

664:                                              ; preds = %659
  %665 = call fastcc i64 @mul_pow_round_to_d(ptr noundef %7, ptr noundef nonnull %4, i32 noundef %129, i32 noundef %128, i32 noundef %653)
  br label %666

666:                                              ; preds = %664, %649
  %667 = phi i64 [ %651, %649 ], [ %665, %664 ]
  %668 = icmp eq i64 %667, 0
  br i1 %668, label %686, label %669

669:                                              ; preds = %666
  %670 = load i32, ptr %7, align 4, !tbaa !12
  %671 = icmp sgt i32 %670, 1024
  br i1 %671, label %686, label %672

672:                                              ; preds = %669
  %673 = icmp slt i32 %670, -1073
  br i1 %673, label %686, label %674

674:                                              ; preds = %672
  %675 = icmp slt i32 %670, -1021
  br i1 %675, label %676, label %680

676:                                              ; preds = %674
  %677 = sub nuw nsw i32 -1021, %670
  %678 = zext nneg i32 %677 to i64
  %679 = lshr i64 %667, %678
  br label %686

680:                                              ; preds = %674
  %681 = add nsw i32 %670, 1022
  %682 = zext nneg i32 %681 to i64
  %683 = shl nuw nsw i64 %682, 52
  %684 = and i64 %667, 4503599627370495
  %685 = or disjoint i64 %683, %684
  br label %686

686:                                              ; preds = %620, %672, %97, %638, %652, %669, %647, %659, %666, %636, %676, %680
  %687 = phi i64 [ %679, %676 ], [ %685, %680 ], [ 0, %636 ], [ 0, %666 ], [ 0, %659 ], [ 0, %647 ], [ 9218868437227405312, %669 ], [ 9218868437227405312, %652 ], [ 9218868437227405312, %638 ], [ 9218868437227405312, %97 ], [ 0, %672 ], [ %624, %620 ]
  %688 = or i64 %687, %19
  %689 = bitcast i64 %688 to double
  br label %690

690:                                              ; preds = %591, %91, %631, %686
  %691 = phi double [ %689, %686 ], [ 0x7FF8000000000000, %631 ], [ 0x7FF8000000000000, %91 ], [ 0x7FF8000000000000, %591 ]
  %692 = icmp eq ptr %1, null
  br i1 %692, label %695, label %693

693:                                              ; preds = %690
  %694 = load ptr, ptr %6, align 8, !tbaa !40
  store ptr %694, ptr %1, align 8, !tbaa !40
  br label %695

695:                                              ; preds = %690, %693
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %7) #10
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %6) #10
  ret double %691
}

declare i32 @strstart(ptr noundef, ptr noundef, ptr noundef) local_unnamed_addr #7

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal fastcc range(i64 0, -9223372036854775808) i64 @round_to_d(ptr noundef nonnull writeonly captures(none) initializes((0, 4)) %0, ptr noundef captures(none) %1, i32 noundef %2) unnamed_addr #0 {
  %4 = getelementptr inbounds nuw i8, ptr %1, i64 4
  %5 = load i32, ptr %4, align 4, !tbaa !12
  %6 = icmp eq i32 %5, 0
  %7 = load i32, ptr %1, align 4, !tbaa !12
  %8 = icmp eq i32 %7, 1
  %9 = select i1 %6, i1 %8, i1 false
  br i1 %9, label %41, label %10

10:                                               ; preds = %3
  %11 = add nsw i32 %7, -1
  %12 = sext i32 %11 to i64
  %13 = getelementptr inbounds [0 x i32], ptr %4, i64 0, i64 %12
  %14 = load i32, ptr %13, align 4, !tbaa !12
  %15 = icmp eq i32 %14, 0
  %16 = shl nsw i32 %7, 5
  %17 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %14, i1 true)
  %18 = sub i32 %16, %17
  %19 = select i1 %15, i32 0, i32 %18
  %20 = sub nsw i32 %19, %2
  %21 = tail call i32 @llvm.smin.i32(i32 %20, i32 -1021)
  %22 = add i32 %19, -1074
  %23 = sub i32 %22, %21
  tail call fastcc void @mpb_shr_round(ptr noundef nonnull %1, i32 noundef %23, i32 noundef 0)
  %24 = load i32, ptr %1, align 4, !tbaa !12
  %25 = icmp eq i32 %24, 1
  br i1 %25, label %26, label %29

26:                                               ; preds = %10
  %27 = load i32, ptr %4, align 4, !tbaa !12
  %28 = zext i32 %27 to i64
  br label %31

29:                                               ; preds = %10
  %30 = load i64, ptr %4, align 4
  br label %31

31:                                               ; preds = %26, %29
  %32 = phi i64 [ %28, %26 ], [ %30, %29 ]
  %33 = sub nuw nsw i32 -1021, %21
  %34 = zext nneg i32 %33 to i64
  %35 = shl i64 %32, %34
  %36 = icmp ugt i64 %35, 9007199254740991
  %37 = zext i1 %36 to i64
  %38 = lshr i64 %35, %37
  %39 = zext i1 %36 to i32
  %40 = add nsw i32 %20, %39
  br label %41

41:                                               ; preds = %3, %31
  %42 = phi i64 [ %38, %31 ], [ 0, %3 ]
  %43 = phi i32 [ %40, %31 ], [ 0, %3 ]
  store i32 %43, ptr %0, align 4, !tbaa !12
  ret i64 %42
}

; Function Attrs: nounwind uwtable
define dso_local noundef nonnull ptr @hello() local_unnamed_addr #5 {
  %1 = alloca [1024 x i8], align 16
  %2 = alloca %struct.JSDTOATempMem, align 8
  call void @llvm.lifetime.start.p0(i64 1024, ptr nonnull %1) #10
  call void @llvm.lifetime.start.p0(i64 296, ptr nonnull %2) #10
  %3 = call i32 @js_dtoa(ptr noundef nonnull %1, double noundef 1.000000e+22, i32 noundef 10, i32 noundef 0, i32 noundef 0, ptr noundef nonnull %2)
  call void @llvm.lifetime.end.p0(i64 296, ptr nonnull %2) #10
  call void @llvm.lifetime.end.p0(i64 1024, ptr nonnull %1) #10
  ret ptr %1
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctpop.i32(i32) #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.fshr.i32(i32, i32, i32) #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctlz.i32(i32, i1 immarg) #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32) #8

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #9

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.vector.reduce.or.v4i32(<4 x i32>) #8

attributes #0 = { nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #5 = { nounwind uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { cold noreturn nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #7 = { "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #8 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #9 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #10 = { nounwind }
attributes #11 = { cold noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"clang version 21.1.8 (https://github.com/llvm/llvm-project.git 2078da43e25a4623cab2d0d60decddf709aaea28)"}
!5 = !{!6, !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
!8 = distinct !{!8, !9}
!9 = !{!"llvm.loop.mustprogress"}
!10 = distinct !{!10, !9}
!11 = distinct !{!11, !9}
!12 = !{!13, !13, i64 0}
!13 = !{!"int", !6, i64 0}
!14 = distinct !{!14, !9}
!15 = distinct !{!15, !9}
!16 = distinct !{!16, !9}
!17 = distinct !{!17, !9}
!18 = distinct !{!18, !9}
!19 = distinct !{!19, !9}
!20 = distinct !{!20, !9}
!21 = distinct !{!21, !9}
!22 = distinct !{!22, !9}
!23 = distinct !{!23, !9}
!24 = distinct !{!24, !9}
!25 = distinct !{!25, !9}
!26 = distinct !{!26, !9}
!27 = distinct !{!27, !9}
!28 = distinct !{!28, !9, !29, !30}
!29 = !{!"llvm.loop.isvectorized", i32 1}
!30 = !{!"llvm.loop.unroll.runtime.disable"}
!31 = distinct !{!31, !9, !30, !29}
!32 = distinct !{!32, !9, !29, !30}
!33 = distinct !{!33, !9, !30, !29}
!34 = distinct !{!34, !9, !29, !30}
!35 = distinct !{!35, !9, !30, !29}
!36 = distinct !{!36, !9}
!37 = distinct !{!37, !38}
!38 = !{!"llvm.loop.unroll.disable"}
!39 = distinct !{!39, !9}
!40 = !{!41, !41, i64 0}
!41 = !{!"p1 omnipotent char", !42, i64 0}
!42 = !{!"any pointer", !6, i64 0}
!43 = distinct !{!43, !9}
!44 = !{!45, !45, i64 0}
!45 = !{!"short", !6, i64 0}

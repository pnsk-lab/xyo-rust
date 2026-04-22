; ModuleID = 'xyojit'
source_filename = "xyojit"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.__va_list_tag = type { i32, i32, ptr, ptr }
%struct.anon = type { ptr, i64, i32 }
%struct.JSATODTempMem = type { [27 x i64] }
%struct.JSDTOATempMem = type { [37 x i64] }
%struct.timespec = type { i64, i64 }

@dtoa_max_digits_table = internal unnamed_addr constant [35 x i8] c"6#\1C\18\16\14\13\12\11\11\10\10\0F\0F\0F\0E\0E\0E\0E\0E\0D\0D\0D\0D\0D\0D\0D\0C\0C\0C\0C\0C\0C\0C\0C", align 16
@.str.1 = private unnamed_addr constant [47 x i8] c"/home/kicky/dev/xyo-rust/bitcodes/c/lib/dtoa.c\00", align 1
@__PRETTY_FUNCTION__.js_dtoa = private unnamed_addr constant [60 x i8] c"int js_dtoa(char *, double, int, int, int, JSDTOATempMem *)\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"NaN\00", align 1
@.str.4 = private unnamed_addr constant [48 x i8] c"n_digits >= 0 && n_digits <= JS_DTOA_MAX_DIGITS\00", align 1
@.str.5 = private unnamed_addr constant [48 x i8] c"n_digits >= 1 && n_digits <= JS_DTOA_MAX_DIGITS\00", align 1
@atod_max_digits_table = internal unnamed_addr constant [35 x i8] c"@P 71-\15(&%#\22! \10\1F\1E\1E\1D\1D\1C\1C\1B\1B\1B\1A\1A\1A\1A\19\0C\19\19\18\18", align 16
@digits_per_limb_table = internal unnamed_addr constant [35 x i8] c" \14\10\0D\0C\0B\0A\0A\09\09\08\08\08\08\08\07\07\07\07\07\07\07\06\06\06\06\06\06\06\06\06\06\06\06\06", align 16
@radix_base_table = internal unnamed_addr constant [35 x i32] [i32 0, i32 -808182895, i32 0, i32 1220703125, i32 -2118184960, i32 1977326743, i32 1073741824, i32 -808182895, i32 1000000000, i32 -1937019605, i32 429981696, i32 815730721, i32 1475789056, i32 -1732076671, i32 0, i32 410338673, i32 612220032, i32 893871739, i32 1280000000, i32 1801088541, i32 -1800609408, i32 -890141849, i32 191102976, i32 244140625, i32 308915776, i32 387420489, i32 481890304, i32 594823321, i32 729000000, i32 887503681, i32 1073741824, i32 1291467969, i32 1544804416, i32 1838265625, i32 -2118184960], align 16
@max_exponent = internal unnamed_addr constant [35 x i16] [i16 1024, i16 647, i16 512, i16 442, i16 397, i16 365, i16 342, i16 324, i16 309, i16 297, i16 286, i16 277, i16 269, i16 263, i16 256, i16 251, i16 246, i16 242, i16 237, i16 234, i16 230, i16 227, i16 224, i16 221, i16 218, i16 216, i16 214, i16 211, i16 209, i16 207, i16 205, i16 203, i16 202, i16 200, i16 199], align 16
@min_exponent = internal unnamed_addr constant [35 x i16] [i16 -1075, i16 -679, i16 -538, i16 -463, i16 -416, i16 -383, i16 -359, i16 -340, i16 -324, i16 -311, i16 -300, i16 -291, i16 -283, i16 -276, i16 -269, i16 -263, i16 -258, i16 -254, i16 -249, i16 -245, i16 -242, i16 -238, i16 -235, i16 -232, i16 -229, i16 -227, i16 -224, i16 -222, i16 -220, i16 -217, i16 -215, i16 -214, i16 -212, i16 -210, i16 -208], align 16
@utf8_first_code_mask = internal unnamed_addr constant [5 x i8] c"\1F\0F\07\03\01", align 1
@utf8_min_code = internal unnamed_addr constant [5 x i32] [i32 128, i32 2048, i32 65536, i32 2097152, i32 67108864], align 16
@mul_log2_radix_table = internal unnamed_addr constant [35 x i32] [i32 0, i32 10585245, i32 0, i32 7225554, i32 6490313, i32 5976165, i32 0, i32 5292622, i32 5050445, i32 4849703, i32 4679886, i32 4533844, i32 4406528, i32 4294263, i32 0, i32 4104555, i32 4023386, i32 3949506, i32 3881882, i32 3819673, i32 3762187, i32 3708851, i32 3659183, i32 3612777, i32 3569286, i32 3528415, i32 3489906, i32 3453537, i32 3419114, i32 3386466, i32 0, i32 3325913, i32 3297757, i32 3270870, i32 3245157], align 16
@pow5_table = internal unnamed_addr constant [17 x i32] [i32 5, i32 25, i32 125, i32 625, i32 3125, i32 15625, i32 78125, i32 390625, i32 1953125, i32 9765625, i32 48828125, i32 244140625, i32 1220703125, i32 1808548329, i32 452807053, i32 -2030932031, i32 -1564725563], align 16
@pow5h_table = internal unnamed_addr constant [4 x i8] c"\01\07#\B1", align 1
@pow5_inv_table = internal unnamed_addr constant [13 x i32] [i32 -1717986919, i32 1202590842, i32 103079215, i32 -1553060175, i32 1334532238, i32 208632331, i32 -1384175189, i32 1469640227, i32 316718722, i32 -1211236963, i32 1607990807, i32 427399186, i32 -1034148220], align 16
@.str.7 = private unnamed_addr constant [32 x i8] c"shift >= 1 && shift < LIMB_BITS\00", align 1
@__PRETTY_FUNCTION__.mp_shl = private unnamed_addr constant [64 x i8] c"limb_t mp_shl(limb_t *, const limb_t *, mp_size_t, int, limb_t)\00", align 1
@switch.table.unicode_from_utf8 = private unnamed_addr constant [62 x i32] [i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 1, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 2, i32 3, i32 3, i32 3, i32 3, i32 3, i32 3, i32 3, i32 3, i32 4, i32 4, i32 4, i32 4, i32 5, i32 5], align 4
@str_to_bool.false_word = internal constant [6 x i16] [i16 102, i16 97, i16 108, i16 115, i16 101, i16 0], align 2
@.str = private unnamed_addr constant [1 x i8] zeroinitializer, align 1
@xorshift128_state_0 = global i64 3770411098252876512
@xorshift128_state_1 = global i64 2895387529709324227
@string_struct = global { i64, ptr, i64, i64 } { i64 2, ptr @string_data, i64 6108732, i64 6918762 }
@string_data = global [2 x i16] [i16 65, i16 97]
@string_struct.1 = global { i64, ptr, i64, i64 } { i64 2, ptr @string_data.2, i64 9116028, i64 10324842 }
@string_data.2 = global [2 x i16] [i16 97, i16 65]
@string_struct.3 = global { i64, ptr, i64, i64 } { i64 1, ptr @string_data.4, i64 48, i64 48 }
@string_data.4 = global [1 x i16] [i16 48]

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @u32toa(ptr noundef writeonly captures(none) %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i8], align 1
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #22
  ret i64 %17
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #2

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #1

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i64 @i32toa(ptr noundef writeonly captures(none) %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i8], align 1
  %4 = alloca [10 x i8], align 1
  %5 = icmp sgt i32 %1, -1
  br i1 %5, label %6, label %21

6:                                                ; preds = %2
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %4) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %4) #22
  br label %39

21:                                               ; preds = %2
  store i8 45, ptr %0, align 1, !tbaa !5
  %22 = sub i32 0, %1
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #22
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
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %4) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %4) #22
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
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %3) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %3) #22
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
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %4) #22
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
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %4) #22
  br label %93

93:                                               ; preds = %15, %72, %89, %6
  %94 = phi i64 [ %7, %6 ], [ %92, %89 ], [ %73, %72 ], [ 1, %15 ]
  ret i64 %94
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctpop.i32(i32) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctlz.i32(i32, i1 immarg) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #3

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
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %5) #22
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
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %5) #22
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
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %4) #22
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
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %4) #22
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
define dso_local range(i32 9, -2147483648) i32 @js_dtoa_max_len(double noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #4 {
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

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.abs.i32(i32, i1 immarg) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #3

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
  call void @llvm.lifetime.start.p0(i64 41, ptr nonnull %8) #22
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
  call void @llvm.lifetime.end.p0(i64 41, ptr nonnull %8) #22
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
  tail call void @__assert_fail(ptr noundef nonnull @.str.4, ptr noundef nonnull @.str.1, i32 noundef 1244, ptr noundef nonnull @__PRETTY_FUNCTION__.js_dtoa) #23
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
  tail call void @__assert_fail(ptr noundef nonnull @.str.5, ptr noundef nonnull @.str.1, i32 noundef 1261, ptr noundef nonnull @__PRETTY_FUNCTION__.js_dtoa) #23
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
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %7) #22
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
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %7) #22
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
  br i1 %128, label %129, label %108, !llvm.loop !18

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
  br i1 %154, label %402, label %27, !llvm.loop !19

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
  tail call void @__assert_fail(ptr noundef nonnull @.str.7, ptr noundef nonnull @.str.1, i32 noundef 175, ptr noundef nonnull @__PRETTY_FUNCTION__.mp_shl) #23
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
  br i1 %322, label %323, label %306, !llvm.loop !20

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
  br i1 %374, label %375, label %342, !llvm.loop !21

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
  br i1 %387, label %379, label %390, !llvm.loop !22

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
  br i1 %396, label %397, label %195, !llvm.loop !23

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
  br i1 %39, label %40, label %23, !llvm.loop !20

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
  br i1 %68, label %60, label %69, !llvm.loop !22

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
  br i1 %97, label %98, label %83, !llvm.loop !24

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
  br i1 %109, label %102, label %110, !llvm.loop !27

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
  br i1 %153, label %154, label %142, !llvm.loop !28

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
  br i1 %168, label %169, label %161, !llvm.loop !29

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
  br i1 %215, label %216, label %205, !llvm.loop !30

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
  br i1 %227, label %228, label %220, !llvm.loop !31

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
  br i1 %261, label %262, label %243, !llvm.loop !32

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
  br i1 %275, label %276, label %266, !llvm.loop !33

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
  br i1 %289, label %290, label %296, !llvm.loop !22

290:                                              ; preds = %279, %285
  %291 = phi i64 [ %286, %285 ], [ %281, %279 ]
  %292 = phi i64 [ %291, %285 ], [ %280, %279 ]
  %293 = trunc nuw nsw i64 %291 to i32
  store i32 %293, ptr %0, align 4, !tbaa !12
  %294 = icmp samesign ugt i64 %292, 2
  br i1 %294, label %285, label %295, !llvm.loop !22

295:                                              ; preds = %290
  br label %296, !llvm.loop !22

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
  br i1 %311, label %303, label %312, !llvm.loop !35

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

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32) #3

; Function Attrs: cold noreturn nounwind
declare void @__assert_fail(ptr noundef, ptr noundef, i32 noundef, ptr noundef) local_unnamed_addr #6

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
  br i1 %52, label %53, label %40, !llvm.loop !36

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
  br i1 %66, label %58, label %67, !llvm.loop !22

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
  br i1 %103, label %104, label %88, !llvm.loop !37

104:                                              ; preds = %84, %88, %67
  %105 = icmp eq i32 %32, 0
  br i1 %105, label %193, label %29, !llvm.loop !38

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
  br i1 %157, label %158, label %145, !llvm.loop !36

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
  br i1 %171, label %163, label %172, !llvm.loop !22

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
  br i1 %190, label %179, label %191, !llvm.loop !39

191:                                              ; preds = %179, %172
  %192 = icmp eq i32 %137, 0
  br i1 %192, label %193, label %134, !llvm.loop !38

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

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #7

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.vector.reduce.or.v4i32(<4 x i32>) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.fshr.i32(i32, i32, i32) #3

; Function Attrs: nounwind uwtable
define dso_local double @js_atod(ptr noundef %0, ptr noundef writeonly captures(address_is_null) %1, i32 noundef %2, i32 noundef %3, ptr noundef captures(none) %4) local_unnamed_addr #5 {
  %6 = alloca i32, align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %6) #22
  %7 = and i32 %3, 8
  %8 = icmp eq i32 %7, 0
  %9 = select i1 %8, i32 256, i32 95
  %10 = load i8, ptr %0, align 1, !tbaa !5
  switch i8 %10, label %16 [
    i8 43, label %12
    i8 45, label %11
  ]

11:                                               ; preds = %5
  br label %12

12:                                               ; preds = %5, %11
  %13 = phi i64 [ -9223372036854775808, %11 ], [ 0, %5 ]
  %14 = getelementptr inbounds nuw i8, ptr %0, i64 1
  %15 = load i8, ptr %14, align 1, !tbaa !5
  br label %16

16:                                               ; preds = %12, %5
  %17 = phi i8 [ %15, %12 ], [ %10, %5 ]
  %18 = phi ptr [ %14, %12 ], [ %0, %5 ]
  %19 = phi i64 [ %13, %12 ], [ 0, %5 ]
  %20 = icmp eq i8 %17, 48
  br i1 %20, label %21, label %93

21:                                               ; preds = %16
  %22 = getelementptr inbounds nuw i8, ptr %18, i64 1
  %23 = load i8, ptr %22, align 1, !tbaa !5
  switch i8 %23, label %31 [
    i8 120, label %24
    i8 88, label %24
    i8 111, label %35
  ]

24:                                               ; preds = %21, %21
  %25 = and i32 %2, -17
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %27, label %29

27:                                               ; preds = %24
  %28 = getelementptr inbounds nuw i8, ptr %18, i64 2
  br label %72

29:                                               ; preds = %24
  %30 = icmp eq i8 %23, 111
  br i1 %30, label %130, label %42

31:                                               ; preds = %21
  %32 = icmp eq i8 %23, 79
  %33 = icmp eq i32 %2, 0
  %34 = and i1 %33, %32
  br i1 %34, label %37, label %42

35:                                               ; preds = %21
  %36 = icmp eq i32 %2, 0
  br i1 %36, label %37, label %130

37:                                               ; preds = %31, %35
  %38 = and i32 %3, 2
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %130, label %40

40:                                               ; preds = %37
  %41 = getelementptr inbounds nuw i8, ptr %18, i64 2
  br label %72

42:                                               ; preds = %29, %31
  %43 = phi i1 [ %33, %31 ], [ false, %29 ]
  %44 = icmp eq i8 %23, 98
  br i1 %44, label %48, label %45

45:                                               ; preds = %42
  %46 = icmp eq i8 %23, 66
  %47 = and i1 %43, %46
  br i1 %47, label %49, label %54

48:                                               ; preds = %42
  br i1 %43, label %49, label %130

49:                                               ; preds = %45, %48
  %50 = and i32 %3, 2
  %51 = icmp eq i32 %50, 0
  br i1 %51, label %130, label %52

52:                                               ; preds = %49
  %53 = getelementptr inbounds nuw i8, ptr %18, i64 2
  br label %72

54:                                               ; preds = %45
  %55 = icmp sgt i8 %23, 47
  %56 = icmp samesign ult i8 %23, 58
  %57 = and i1 %43, %56
  %58 = select i1 %55, i1 %57, i1 false
  br i1 %58, label %59, label %127

59:                                               ; preds = %54
  %60 = and i32 %3, 4
  %61 = icmp eq i32 %60, 0
  br i1 %61, label %130, label %62

62:                                               ; preds = %59, %62
  %63 = phi i64 [ %68, %62 ], [ 1, %59 ]
  %64 = getelementptr inbounds nuw i8, ptr %18, i64 %63
  %65 = load i8, ptr %64, align 1, !tbaa !5
  %66 = and i8 %65, -8
  %67 = icmp eq i8 %66, 48
  %68 = add nuw nsw i64 %63, 1
  br i1 %67, label %62, label %69, !llvm.loop !40

69:                                               ; preds = %62
  %70 = and i8 %65, -2
  %71 = icmp eq i8 %70, 56
  br i1 %71, label %130, label %72

72:                                               ; preds = %69, %40, %52, %27
  %73 = phi ptr [ %53, %52 ], [ %41, %40 ], [ %28, %27 ], [ %22, %69 ]
  %74 = phi i32 [ %9, %52 ], [ %9, %40 ], [ %9, %27 ], [ 256, %69 ]
  %75 = phi i32 [ 2, %52 ], [ 8, %40 ], [ 16, %27 ], [ 8, %69 ]
  %76 = load i8, ptr %73, align 1, !tbaa !5
  %77 = zext i8 %76 to i32
  %78 = add nsw i32 %77, -48
  %79 = icmp ult i32 %78, 10
  br i1 %79, label %90, label %80

80:                                               ; preds = %72
  %81 = add i8 %76, -65
  %82 = icmp ult i8 %81, 26
  br i1 %82, label %83, label %85

83:                                               ; preds = %80
  %84 = add nsw i32 %77, -55
  br label %90

85:                                               ; preds = %80
  %86 = add i8 %76, -97
  %87 = icmp ult i8 %86, 26
  %88 = add nsw i32 %77, -87
  %89 = select i1 %87, i32 %88, i32 36
  br label %90

90:                                               ; preds = %72, %83, %85
  %91 = phi i32 [ %84, %83 ], [ %89, %85 ], [ %78, %72 ]
  %92 = icmp slt i32 %91, %75
  br i1 %92, label %130, label %715

93:                                               ; preds = %16
  %94 = and i32 %3, 1
  %95 = icmp eq i32 %94, 0
  br i1 %95, label %96, label %127

96:                                               ; preds = %93
  %97 = getelementptr i8, ptr %18, i64 8
  %98 = icmp eq i8 %17, 73
  br i1 %98, label %99, label %127

99:                                               ; preds = %96
  %100 = getelementptr inbounds nuw i8, ptr %18, i64 1
  %101 = load i8, ptr %100, align 1, !tbaa !5
  %102 = icmp eq i8 %101, 110
  br i1 %102, label %103, label %127

103:                                              ; preds = %99
  %104 = getelementptr inbounds nuw i8, ptr %18, i64 2
  %105 = load i8, ptr %104, align 1, !tbaa !5
  %106 = icmp eq i8 %105, 102
  br i1 %106, label %107, label %127

107:                                              ; preds = %103
  %108 = getelementptr inbounds nuw i8, ptr %18, i64 3
  %109 = load i8, ptr %108, align 1, !tbaa !5
  %110 = icmp eq i8 %109, 105
  br i1 %110, label %111, label %127

111:                                              ; preds = %107
  %112 = getelementptr inbounds nuw i8, ptr %18, i64 4
  %113 = load i8, ptr %112, align 1, !tbaa !5
  %114 = icmp eq i8 %113, 110
  br i1 %114, label %115, label %127

115:                                              ; preds = %111
  %116 = getelementptr inbounds nuw i8, ptr %18, i64 5
  %117 = load i8, ptr %116, align 1, !tbaa !5
  %118 = icmp eq i8 %117, 105
  br i1 %118, label %119, label %127

119:                                              ; preds = %115
  %120 = getelementptr inbounds nuw i8, ptr %18, i64 6
  %121 = load i8, ptr %120, align 1, !tbaa !5
  %122 = icmp eq i8 %121, 116
  br i1 %122, label %123, label %127

123:                                              ; preds = %119
  %124 = getelementptr inbounds nuw i8, ptr %18, i64 7
  %125 = load i8, ptr %124, align 1, !tbaa !5
  %126 = icmp eq i8 %125, 121
  br i1 %126, label %710, label %127

127:                                              ; preds = %96, %99, %103, %107, %111, %115, %119, %123, %93, %54
  %128 = icmp eq i32 %2, 0
  %129 = select i1 %128, i32 10, i32 %2
  br label %130

130:                                              ; preds = %29, %37, %127, %48, %35, %49, %69, %59, %90
  %131 = phi i32 [ %74, %90 ], [ 256, %69 ], [ %9, %59 ], [ %9, %49 ], [ %9, %35 ], [ %9, %48 ], [ %9, %127 ], [ %9, %37 ], [ %9, %29 ]
  %132 = phi ptr [ %73, %90 ], [ %18, %69 ], [ %18, %59 ], [ %18, %49 ], [ %18, %35 ], [ %18, %48 ], [ %18, %127 ], [ %18, %37 ], [ %18, %29 ]
  %133 = phi i32 [ %75, %90 ], [ 10, %69 ], [ 10, %59 ], [ 10, %49 ], [ %2, %35 ], [ %2, %48 ], [ %129, %127 ], [ 10, %37 ], [ %2, %29 ]
  %134 = add nsw i32 %133, -2
  %135 = sext i32 %134 to i64
  %136 = getelementptr inbounds [35 x i8], ptr @atod_max_digits_table, i64 0, i64 %135
  %137 = load i8, ptr %136, align 1, !tbaa !5
  %138 = zext i8 %137 to i32
  %139 = getelementptr inbounds [35 x i8], ptr @digits_per_limb_table, i64 0, i64 %135
  %140 = load i8, ptr %139, align 1, !tbaa !5
  %141 = zext i8 %140 to i32
  %142 = getelementptr inbounds [35 x i32], ptr @radix_base_table, i64 0, i64 %135
  %143 = load i32, ptr %142, align 4, !tbaa !12
  %144 = and i32 %133, 1
  %145 = icmp eq i32 %144, 0
  br i1 %145, label %146, label %155

146:                                              ; preds = %130, %146
  %147 = phi i32 [ %150, %146 ], [ 0, %130 ]
  %148 = phi i32 [ %149, %146 ], [ %133, %130 ]
  %149 = lshr exact i32 %148, 1
  %150 = add nuw nsw i32 %147, 1
  %151 = icmp ne i32 %148, 0
  %152 = and i32 %148, 2
  %153 = icmp eq i32 %152, 0
  %154 = and i1 %151, %153
  br i1 %154, label %146, label %155, !llvm.loop !14

155:                                              ; preds = %146, %130
  %156 = phi i32 [ 0, %130 ], [ %150, %146 ]
  %157 = ashr i32 %133, %156
  %158 = icmp eq i32 %157, 1
  %159 = select i1 %158, i32 %156, i32 0
  store i32 1, ptr %4, align 4, !tbaa !12
  %160 = getelementptr inbounds nuw i8, ptr %4, i64 4
  store i32 0, ptr %160, align 4, !tbaa !12
  %161 = and i32 %3, 1
  %162 = icmp eq i32 %161, 0
  br label %163

163:                                              ; preds = %211, %155
  %164 = phi ptr [ %132, %155 ], [ %213, %211 ]
  %165 = phi i32 [ -1, %155 ], [ %200, %211 ]
  %166 = phi i32 [ 0, %155 ], [ %214, %211 ]
  %167 = load i8, ptr %164, align 1, !tbaa !5
  %168 = icmp eq i8 %167, 46
  br i1 %168, label %169, label %197

169:                                              ; preds = %163
  %170 = icmp ugt ptr %164, %18
  br i1 %170, label %191, label %171

171:                                              ; preds = %169
  %172 = getelementptr inbounds nuw i8, ptr %164, i64 1
  %173 = load i8, ptr %172, align 1, !tbaa !5
  %174 = sext i8 %173 to i32
  %175 = add nsw i32 %174, -48
  %176 = icmp ult i32 %175, 10
  br i1 %176, label %187, label %177

177:                                              ; preds = %171
  %178 = add nsw i32 %174, -65
  %179 = icmp ult i32 %178, 26
  br i1 %179, label %180, label %182

180:                                              ; preds = %177
  %181 = add nsw i32 %174, -55
  br label %187

182:                                              ; preds = %177
  %183 = add nsw i32 %174, -97
  %184 = icmp ult i32 %183, 26
  %185 = add nsw i32 %174, -87
  %186 = select i1 %184, i32 %185, i32 36
  br label %187

187:                                              ; preds = %171, %180, %182
  %188 = phi i32 [ %181, %180 ], [ %186, %182 ], [ %175, %171 ]
  %189 = icmp slt i32 %188, %133
  %190 = and i1 %162, %189
  br i1 %190, label %192, label %197

191:                                              ; preds = %169
  br i1 %162, label %192, label %197

192:                                              ; preds = %191, %187
  %193 = icmp sgt i32 %165, -1
  br i1 %193, label %215, label %194

194:                                              ; preds = %192
  %195 = getelementptr inbounds nuw i8, ptr %164, i64 1
  %196 = load i8, ptr %195, align 1, !tbaa !5
  br label %197

197:                                              ; preds = %194, %191, %187, %163
  %198 = phi i8 [ %196, %194 ], [ 46, %191 ], [ 46, %187 ], [ %167, %163 ]
  %199 = phi ptr [ %195, %194 ], [ %164, %191 ], [ %164, %187 ], [ %164, %163 ]
  %200 = phi i32 [ %166, %194 ], [ %165, %191 ], [ %165, %187 ], [ %165, %163 ]
  %201 = sext i8 %198 to i32
  %202 = icmp eq i32 %131, %201
  %203 = icmp ugt ptr %199, %18
  %204 = and i1 %203, %202
  br i1 %204, label %205, label %209

205:                                              ; preds = %197
  %206 = getelementptr inbounds nuw i8, ptr %199, i64 1
  %207 = load i8, ptr %206, align 1, !tbaa !5
  %208 = icmp eq i8 %207, 48
  br i1 %208, label %211, label %209

209:                                              ; preds = %205, %197
  %210 = icmp eq i8 %198, 48
  br i1 %210, label %211, label %215

211:                                              ; preds = %205, %209
  %212 = phi ptr [ %199, %209 ], [ %206, %205 ]
  %213 = getelementptr inbounds nuw i8, ptr %212, i64 1
  %214 = add nuw nsw i32 %166, 1
  br label %163

215:                                              ; preds = %209, %192
  %216 = phi i8 [ 46, %192 ], [ %198, %209 ]
  %217 = phi ptr [ %164, %192 ], [ %199, %209 ]
  %218 = phi i32 [ %165, %192 ], [ %200, %209 ]
  %219 = shl nuw i64 1, %135
  %220 = and i64 %219, 16389
  %221 = icmp eq i64 %220, 0
  %222 = zext i32 %143 to i64
  %223 = getelementptr i8, ptr %4, i64 8
  %224 = getelementptr inbounds i8, ptr %160, i64 4
  br label %225

225:                                              ; preds = %419, %215
  %226 = phi i32 [ 1, %215 ], [ %420, %419 ]
  %227 = phi i32 [ 1, %215 ], [ %421, %419 ]
  %228 = phi i32 [ 1, %215 ], [ %422, %419 ]
  %229 = phi i8 [ %216, %215 ], [ %427, %419 ]
  %230 = phi ptr [ %217, %215 ], [ %315, %419 ]
  %231 = phi i32 [ 0, %215 ], [ %423, %419 ]
  %232 = phi i32 [ 0, %215 ], [ %424, %419 ]
  %233 = phi i32 [ 0, %215 ], [ %425, %419 ]
  %234 = phi i32 [ 0, %215 ], [ %426, %419 ]
  %235 = phi i32 [ %218, %215 ], [ %269, %419 ]
  %236 = phi i32 [ %166, %215 ], [ %316, %419 ]
  %237 = icmp eq i8 %229, 46
  br i1 %237, label %238, label %266

238:                                              ; preds = %225
  %239 = icmp ugt ptr %230, %18
  br i1 %239, label %260, label %240

240:                                              ; preds = %238
  %241 = getelementptr inbounds nuw i8, ptr %230, i64 1
  %242 = load i8, ptr %241, align 1, !tbaa !5
  %243 = sext i8 %242 to i32
  %244 = add nsw i32 %243, -48
  %245 = icmp ult i32 %244, 10
  br i1 %245, label %256, label %246

246:                                              ; preds = %240
  %247 = add nsw i32 %243, -65
  %248 = icmp ult i32 %247, 26
  br i1 %248, label %249, label %251

249:                                              ; preds = %246
  %250 = add nsw i32 %243, -55
  br label %256

251:                                              ; preds = %246
  %252 = add nsw i32 %243, -97
  %253 = icmp ult i32 %252, 26
  %254 = add nsw i32 %243, -87
  %255 = select i1 %253, i32 %254, i32 36
  br label %256

256:                                              ; preds = %240, %249, %251
  %257 = phi i32 [ %250, %249 ], [ %255, %251 ], [ %244, %240 ]
  %258 = icmp slt i32 %257, %133
  %259 = and i1 %162, %258
  br i1 %259, label %261, label %266

260:                                              ; preds = %238
  br i1 %162, label %261, label %266

261:                                              ; preds = %260, %256
  %262 = icmp sgt i32 %235, -1
  br i1 %262, label %428, label %263

263:                                              ; preds = %261
  %264 = getelementptr inbounds nuw i8, ptr %230, i64 1
  %265 = load i8, ptr %264, align 1, !tbaa !5
  br label %266

266:                                              ; preds = %263, %260, %256, %225
  %267 = phi i8 [ %265, %263 ], [ 46, %260 ], [ 46, %256 ], [ %229, %225 ]
  %268 = phi ptr [ %264, %263 ], [ %230, %260 ], [ %230, %256 ], [ %230, %225 ]
  %269 = phi i32 [ %236, %263 ], [ %235, %260 ], [ %235, %256 ], [ %235, %225 ]
  %270 = sext i8 %267 to i32
  %271 = icmp eq i32 %131, %270
  %272 = icmp ugt ptr %268, %18
  %273 = and i1 %272, %271
  br i1 %273, label %274, label %296

274:                                              ; preds = %266
  %275 = getelementptr inbounds nuw i8, ptr %268, i64 1
  %276 = load i8, ptr %275, align 1, !tbaa !5
  %277 = sext i8 %276 to i32
  %278 = add nsw i32 %277, -48
  %279 = icmp ult i32 %278, 10
  br i1 %279, label %290, label %280

280:                                              ; preds = %274
  %281 = add nsw i32 %277, -65
  %282 = icmp ult i32 %281, 26
  br i1 %282, label %283, label %285

283:                                              ; preds = %280
  %284 = add nsw i32 %277, -55
  br label %290

285:                                              ; preds = %280
  %286 = add nsw i32 %277, -97
  %287 = icmp ult i32 %286, 26
  %288 = add nsw i32 %277, -87
  %289 = select i1 %287, i32 %288, i32 36
  br label %290

290:                                              ; preds = %274, %283, %285
  %291 = phi i32 [ %284, %283 ], [ %289, %285 ], [ %278, %274 ]
  %292 = icmp slt i32 %291, %133
  %293 = select i1 %292, ptr %275, ptr %268
  %294 = load i8, ptr %293, align 1, !tbaa !5
  %295 = sext i8 %294 to i32
  br label %296

296:                                              ; preds = %290, %266
  %297 = phi i32 [ %295, %290 ], [ %270, %266 ]
  %298 = phi ptr [ %293, %290 ], [ %268, %266 ]
  %299 = add nsw i32 %297, -48
  %300 = icmp ult i32 %299, 10
  br i1 %300, label %311, label %301

301:                                              ; preds = %296
  %302 = add nsw i32 %297, -65
  %303 = icmp ult i32 %302, 26
  br i1 %303, label %304, label %306

304:                                              ; preds = %301
  %305 = add nsw i32 %297, -55
  br label %311

306:                                              ; preds = %301
  %307 = add nsw i32 %297, -97
  %308 = icmp ult i32 %307, 26
  %309 = add nsw i32 %297, -87
  %310 = select i1 %308, i32 %309, i32 36
  br label %311

311:                                              ; preds = %296, %304, %306
  %312 = phi i32 [ %305, %304 ], [ %310, %306 ], [ %299, %296 ]
  %313 = icmp ult i32 %312, %133
  br i1 %313, label %314, label %428

314:                                              ; preds = %311
  %315 = getelementptr inbounds nuw i8, ptr %298, i64 1
  %316 = add nuw nsw i32 %236, 1
  %317 = icmp slt i32 %232, %138
  br i1 %317, label %318, label %417

318:                                              ; preds = %314
  %319 = mul i32 %234, %133
  %320 = add i32 %312, %319
  %321 = add nsw i32 %231, 1
  %322 = icmp eq i32 %321, %141
  br i1 %322, label %323, label %410

323:                                              ; preds = %318
  %324 = load i32, ptr %160, align 4, !tbaa !12
  %325 = icmp eq i32 %324, 0
  br i1 %325, label %326, label %329

326:                                              ; preds = %323
  %327 = icmp eq i32 %227, 1
  br i1 %327, label %328, label %329

328:                                              ; preds = %326
  store i32 %320, ptr %160, align 4, !tbaa !12
  br label %410

329:                                              ; preds = %323, %326
  %330 = phi i32 [ %227, %326 ], [ %228, %323 ]
  br i1 %221, label %338, label %331

331:                                              ; preds = %329
  %332 = icmp sgt i32 %330, -1
  br i1 %332, label %333, label %337

333:                                              ; preds = %331
  %334 = zext nneg i32 %330 to i64
  %335 = shl nuw nsw i64 %334, 2
  %336 = add nuw nsw i64 %335, 4
  tail call void @llvm.memmove.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %223, ptr noundef nonnull align 4 dereferenceable(1) %160, i64 %336, i1 false), !tbaa !12
  br label %337

337:                                              ; preds = %333, %331
  store i32 %320, ptr %160, align 4, !tbaa !12
  br label %389

338:                                              ; preds = %329
  %339 = icmp eq i32 %330, 0
  br i1 %339, label %384, label %340

340:                                              ; preds = %338
  %341 = zext i32 %330 to i64
  %342 = and i64 %341, 1
  %343 = icmp eq i32 %330, 1
  br i1 %343, label %369, label %344

344:                                              ; preds = %340
  %345 = and i64 %341, 4294967294
  br label %346

346:                                              ; preds = %346, %344
  %347 = phi i64 [ 0, %344 ], [ %366, %346 ]
  %348 = phi i32 [ %320, %344 ], [ %365, %346 ]
  %349 = phi i64 [ 0, %344 ], [ %367, %346 ]
  %350 = getelementptr inbounds nuw i32, ptr %160, i64 %347
  %351 = load i32, ptr %350, align 4, !tbaa !12
  %352 = zext i32 %351 to i64
  %353 = mul nuw i64 %352, %222
  %354 = zext i32 %348 to i64
  %355 = add nuw i64 %353, %354
  %356 = trunc i64 %355 to i32
  store i32 %356, ptr %350, align 4, !tbaa !12
  %357 = lshr i64 %355, 32
  %358 = getelementptr inbounds i32, ptr %224, i64 %347
  %359 = load i32, ptr %358, align 4, !tbaa !12
  %360 = zext i32 %359 to i64
  %361 = mul nuw i64 %360, %222
  %362 = add nuw i64 %361, %357
  %363 = trunc i64 %362 to i32
  store i32 %363, ptr %358, align 4, !tbaa !12
  %364 = lshr i64 %362, 32
  %365 = trunc nuw i64 %364 to i32
  %366 = add nuw nsw i64 %347, 2
  %367 = add i64 %349, 2
  %368 = icmp eq i64 %367, %345
  br i1 %368, label %369, label %346, !llvm.loop !18

369:                                              ; preds = %346, %340
  %370 = phi i32 [ poison, %340 ], [ %365, %346 ]
  %371 = phi i64 [ 0, %340 ], [ %366, %346 ]
  %372 = phi i32 [ %320, %340 ], [ %365, %346 ]
  %373 = icmp eq i64 %342, 0
  br i1 %373, label %384, label %374

374:                                              ; preds = %369
  %375 = getelementptr inbounds nuw i32, ptr %160, i64 %371
  %376 = load i32, ptr %375, align 4, !tbaa !12
  %377 = zext i32 %376 to i64
  %378 = mul nuw i64 %377, %222
  %379 = zext i32 %372 to i64
  %380 = add nuw i64 %378, %379
  %381 = trunc i64 %380 to i32
  store i32 %381, ptr %375, align 4, !tbaa !12
  %382 = lshr i64 %380, 32
  %383 = trunc nuw i64 %382 to i32
  br label %384

384:                                              ; preds = %374, %369, %338
  %385 = phi i32 [ %320, %338 ], [ %370, %369 ], [ %383, %374 ]
  %386 = sext i32 %330 to i64
  %387 = getelementptr inbounds [0 x i32], ptr %160, i64 0, i64 %386
  store i32 %385, ptr %387, align 4, !tbaa !12
  %388 = load i32, ptr %4, align 4, !tbaa !12
  br label %389

389:                                              ; preds = %384, %337
  %390 = phi i32 [ %388, %384 ], [ %330, %337 ]
  %391 = add nsw i32 %390, 1
  store i32 %391, ptr %4, align 4, !tbaa !12
  %392 = icmp sgt i32 %390, 0
  br i1 %392, label %393, label %410

393:                                              ; preds = %389
  %394 = zext nneg i32 %391 to i64
  %395 = add nsw i64 %394, -1
  %396 = getelementptr inbounds nuw [0 x i32], ptr %160, i64 0, i64 %395
  %397 = load i32, ptr %396, align 4, !tbaa !12
  %398 = icmp eq i32 %397, 0
  br i1 %398, label %404, label %410

399:                                              ; preds = %404
  %400 = add nsw i64 %405, -1
  %401 = getelementptr inbounds nuw [0 x i32], ptr %160, i64 0, i64 %400
  %402 = load i32, ptr %401, align 4, !tbaa !12
  %403 = icmp eq i32 %402, 0
  br i1 %403, label %404, label %410, !llvm.loop !22

404:                                              ; preds = %393, %399
  %405 = phi i64 [ %400, %399 ], [ %395, %393 ]
  %406 = phi i64 [ %405, %399 ], [ %394, %393 ]
  %407 = trunc nuw nsw i64 %405 to i32
  store i32 %407, ptr %4, align 4, !tbaa !12
  %408 = icmp samesign ugt i64 %406, 2
  br i1 %408, label %399, label %409, !llvm.loop !22

409:                                              ; preds = %404
  br label %410, !llvm.loop !22

410:                                              ; preds = %399, %393, %409, %389, %328, %318
  %411 = phi i32 [ %226, %318 ], [ %226, %328 ], [ %391, %389 ], [ %407, %409 ], [ %391, %393 ], [ %407, %399 ]
  %412 = phi i32 [ %227, %318 ], [ 1, %328 ], [ %391, %389 ], [ %407, %409 ], [ %391, %393 ], [ %407, %399 ]
  %413 = phi i32 [ %228, %318 ], [ 1, %328 ], [ %391, %389 ], [ %407, %409 ], [ %391, %393 ], [ %407, %399 ]
  %414 = phi i32 [ %321, %318 ], [ 0, %328 ], [ 0, %389 ], [ 0, %409 ], [ 0, %393 ], [ 0, %399 ]
  %415 = phi i32 [ %320, %318 ], [ 0, %328 ], [ 0, %389 ], [ 0, %409 ], [ 0, %393 ], [ 0, %399 ]
  %416 = add nsw i32 %232, 1
  br label %419

417:                                              ; preds = %314
  %418 = or i32 %312, %233
  br label %419

419:                                              ; preds = %410, %417
  %420 = phi i32 [ %411, %410 ], [ %226, %417 ]
  %421 = phi i32 [ %412, %410 ], [ %227, %417 ]
  %422 = phi i32 [ %413, %410 ], [ %228, %417 ]
  %423 = phi i32 [ %414, %410 ], [ %231, %417 ]
  %424 = phi i32 [ %416, %410 ], [ %232, %417 ]
  %425 = phi i32 [ %233, %410 ], [ %418, %417 ]
  %426 = phi i32 [ %415, %410 ], [ %234, %417 ]
  %427 = load i8, ptr %315, align 1, !tbaa !5
  br label %225

428:                                              ; preds = %261, %311
  %429 = phi ptr [ %298, %311 ], [ %230, %261 ]
  %430 = phi i32 [ %269, %311 ], [ %235, %261 ]
  switch i32 %231, label %433 [
    i32 0, label %580
    i32 1, label %431
  ]

431:                                              ; preds = %428
  %432 = zext i32 %133 to i64
  br label %496

433:                                              ; preds = %428
  %434 = icmp eq i32 %133, 5
  %435 = icmp eq i32 %133, 10
  %436 = or i1 %434, %435
  %437 = icmp ult i32 %231, 18
  %438 = and i1 %436, %437
  br i1 %438, label %439, label %459

439:                                              ; preds = %433
  %440 = add nsw i32 %231, -1
  %441 = zext nneg i32 %440 to i64
  %442 = getelementptr inbounds nuw [17 x i32], ptr @pow5_table, i64 0, i64 %441
  %443 = load i32, ptr %442, align 4, !tbaa !12
  %444 = zext i32 %443 to i64
  %445 = icmp samesign ugt i32 %231, 13
  br i1 %445, label %446, label %454

446:                                              ; preds = %439
  %447 = add nsw i32 %231, -14
  %448 = zext nneg i32 %447 to i64
  %449 = getelementptr inbounds nuw [4 x i8], ptr @pow5h_table, i64 0, i64 %448
  %450 = load i8, ptr %449, align 1, !tbaa !5
  %451 = zext i8 %450 to i64
  %452 = shl nuw nsw i64 %451, 32
  %453 = or disjoint i64 %452, %444
  br label %454

454:                                              ; preds = %446, %439
  %455 = phi i64 [ %453, %446 ], [ %444, %439 ]
  %456 = select i1 %435, i32 %231, i32 0
  %457 = zext nneg i32 %456 to i64
  %458 = shl nuw nsw i64 %455, %457
  br label %496

459:                                              ; preds = %433
  %460 = zext i32 %133 to i64
  %461 = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %231, i1 false)
  %462 = sub nsw i32 30, %461
  %463 = and i32 %461, 1
  %464 = icmp eq i32 %463, 0
  br i1 %464, label %465, label %473

465:                                              ; preds = %459
  %466 = mul nuw i64 %460, %460
  %467 = shl nuw i32 1, %462
  %468 = and i32 %467, %231
  %469 = icmp eq i32 %468, 0
  %470 = select i1 %469, i64 1, i64 %460
  %471 = mul i64 %466, %470
  %472 = sub nsw i32 29, %461
  br label %473

473:                                              ; preds = %465, %459
  %474 = phi i64 [ poison, %459 ], [ %471, %465 ]
  %475 = phi i64 [ %460, %459 ], [ %471, %465 ]
  %476 = phi i32 [ %462, %459 ], [ %472, %465 ]
  %477 = icmp eq i32 %461, 30
  br i1 %477, label %496, label %478

478:                                              ; preds = %473, %478
  %479 = phi i64 [ %493, %478 ], [ %475, %473 ]
  %480 = phi i32 [ %494, %478 ], [ %476, %473 ]
  %481 = mul i64 %479, %479
  %482 = shl nuw i32 1, %480
  %483 = and i32 %482, %231
  %484 = icmp eq i32 %483, 0
  %485 = select i1 %484, i64 1, i64 %460
  %486 = mul i64 %481, %485
  %487 = add nsw i32 %480, -1
  %488 = mul i64 %486, %486
  %489 = shl nuw i32 1, %487
  %490 = and i32 %489, %231
  %491 = icmp eq i32 %490, 0
  %492 = select i1 %491, i64 1, i64 %460
  %493 = mul i64 %488, %492
  %494 = add nsw i32 %480, -2
  %495 = icmp eq i32 %487, 0
  br i1 %495, label %496, label %478, !llvm.loop !15

496:                                              ; preds = %473, %478, %431, %454
  %497 = phi i64 [ %432, %431 ], [ %458, %454 ], [ %474, %473 ], [ %493, %478 ]
  %498 = load i32, ptr %160, align 4, !tbaa !12
  %499 = icmp eq i32 %498, 0
  %500 = icmp eq i32 %226, 1
  %501 = select i1 %499, i1 %500, i1 false
  br i1 %501, label %502, label %503

502:                                              ; preds = %496
  store i32 %234, ptr %160, align 4, !tbaa !12
  br label %580

503:                                              ; preds = %496
  %504 = and i64 %497, 4294967295
  %505 = icmp eq i64 %504, 0
  br i1 %505, label %506, label %513

506:                                              ; preds = %503
  %507 = icmp sgt i32 %226, -1
  br i1 %507, label %508, label %512

508:                                              ; preds = %506
  %509 = zext nneg i32 %226 to i64
  %510 = shl nuw nsw i64 %509, 2
  %511 = add nuw nsw i64 %510, 4
  tail call void @llvm.memmove.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %223, ptr noundef nonnull align 4 dereferenceable(1) %160, i64 %511, i1 false), !tbaa !12
  br label %512

512:                                              ; preds = %508, %506
  store i32 %234, ptr %160, align 4, !tbaa !12
  br label %565

513:                                              ; preds = %503
  %514 = icmp eq i32 %226, 0
  br i1 %514, label %560, label %515

515:                                              ; preds = %513
  %516 = zext i32 %226 to i64
  %517 = and i64 %516, 1
  %518 = icmp eq i32 %226, 1
  br i1 %518, label %545, label %519

519:                                              ; preds = %515
  %520 = and i64 %516, 4294967294
  %521 = getelementptr inbounds i8, ptr %160, i64 4
  br label %522

522:                                              ; preds = %522, %519
  %523 = phi i64 [ 0, %519 ], [ %542, %522 ]
  %524 = phi i32 [ %234, %519 ], [ %541, %522 ]
  %525 = phi i64 [ 0, %519 ], [ %543, %522 ]
  %526 = getelementptr inbounds nuw i32, ptr %160, i64 %523
  %527 = load i32, ptr %526, align 4, !tbaa !12
  %528 = zext i32 %527 to i64
  %529 = mul nuw i64 %504, %528
  %530 = zext i32 %524 to i64
  %531 = add nuw i64 %529, %530
  %532 = trunc i64 %531 to i32
  store i32 %532, ptr %526, align 4, !tbaa !12
  %533 = lshr i64 %531, 32
  %534 = getelementptr inbounds i32, ptr %521, i64 %523
  %535 = load i32, ptr %534, align 4, !tbaa !12
  %536 = zext i32 %535 to i64
  %537 = mul nuw i64 %504, %536
  %538 = add nuw i64 %537, %533
  %539 = trunc i64 %538 to i32
  store i32 %539, ptr %534, align 4, !tbaa !12
  %540 = lshr i64 %538, 32
  %541 = trunc nuw i64 %540 to i32
  %542 = add nuw nsw i64 %523, 2
  %543 = add i64 %525, 2
  %544 = icmp eq i64 %543, %520
  br i1 %544, label %545, label %522, !llvm.loop !18

545:                                              ; preds = %522, %515
  %546 = phi i32 [ poison, %515 ], [ %541, %522 ]
  %547 = phi i64 [ 0, %515 ], [ %542, %522 ]
  %548 = phi i32 [ %234, %515 ], [ %541, %522 ]
  %549 = icmp eq i64 %517, 0
  br i1 %549, label %560, label %550

550:                                              ; preds = %545
  %551 = getelementptr inbounds nuw i32, ptr %160, i64 %547
  %552 = load i32, ptr %551, align 4, !tbaa !12
  %553 = zext i32 %552 to i64
  %554 = mul nuw i64 %504, %553
  %555 = zext i32 %548 to i64
  %556 = add nuw i64 %554, %555
  %557 = trunc i64 %556 to i32
  store i32 %557, ptr %551, align 4, !tbaa !12
  %558 = lshr i64 %556, 32
  %559 = trunc nuw i64 %558 to i32
  br label %560

560:                                              ; preds = %550, %545, %513
  %561 = phi i32 [ %234, %513 ], [ %546, %545 ], [ %559, %550 ]
  %562 = sext i32 %226 to i64
  %563 = getelementptr inbounds [0 x i32], ptr %160, i64 0, i64 %562
  store i32 %561, ptr %563, align 4, !tbaa !12
  %564 = load i32, ptr %4, align 4, !tbaa !12
  br label %565

565:                                              ; preds = %560, %512
  %566 = phi i32 [ %564, %560 ], [ %226, %512 ]
  %567 = add nsw i32 %566, 1
  store i32 %567, ptr %4, align 4, !tbaa !12
  %568 = icmp sgt i32 %566, 0
  br i1 %568, label %569, label %580

569:                                              ; preds = %565
  %570 = zext nneg i32 %567 to i64
  br label %571

571:                                              ; preds = %577, %569
  %572 = phi i64 [ %570, %569 ], [ %573, %577 ]
  %573 = add nsw i64 %572, -1
  %574 = getelementptr inbounds nuw [0 x i32], ptr %160, i64 0, i64 %573
  %575 = load i32, ptr %574, align 4, !tbaa !12
  %576 = icmp eq i32 %575, 0
  br i1 %576, label %577, label %580

577:                                              ; preds = %571
  %578 = trunc nuw nsw i64 %573 to i32
  store i32 %578, ptr %4, align 4, !tbaa !12
  %579 = icmp samesign ugt i64 %572, 2
  br i1 %579, label %571, label %580, !llvm.loop !22

580:                                              ; preds = %577, %571, %428, %565, %502
  %581 = icmp ne i32 %232, 0
  %582 = icmp slt i32 %430, 0
  %583 = select i1 %582, i32 %236, i32 %430
  %584 = add nsw i32 %232, %166
  %585 = sub i32 %584, %583
  %586 = icmp ne i32 %159, 0
  %587 = icmp ne i32 %233, 0
  %588 = select i1 %586, i1 %587, i1 false
  br i1 %588, label %589, label %592

589:                                              ; preds = %580
  %590 = load i32, ptr %160, align 4, !tbaa !12
  %591 = or i32 %590, 1
  store i32 %591, ptr %160, align 4, !tbaa !12
  br label %592

592:                                              ; preds = %589, %580
  br i1 %162, label %593, label %655

593:                                              ; preds = %592
  %594 = icmp eq i32 %133, 10
  %595 = load i8, ptr %429, align 1, !tbaa !5
  br i1 %594, label %596, label %597

596:                                              ; preds = %593
  switch i8 %595, label %655 [
    i8 101, label %603
    i8 69, label %603
  ]

597:                                              ; preds = %593
  %598 = icmp eq i8 %595, 64
  br i1 %598, label %603, label %599

599:                                              ; preds = %597
  %600 = add i32 %159, -1
  %601 = icmp ult i32 %600, 4
  br i1 %601, label %602, label %655

602:                                              ; preds = %599
  switch i8 %595, label %655 [
    i8 112, label %603
    i8 80, label %603
  ]

603:                                              ; preds = %602, %602, %596, %596, %597
  %604 = phi i8 [ %595, %602 ], [ %595, %602 ], [ %595, %596 ], [ %595, %596 ], [ 64, %597 ]
  %605 = icmp ugt ptr %429, %18
  br i1 %605, label %606, label %655

606:                                              ; preds = %603
  %607 = and i8 %604, -33
  %608 = icmp ne i8 %607, 80
  %609 = getelementptr inbounds nuw i8, ptr %429, i64 1
  %610 = load i8, ptr %609, align 1, !tbaa !5
  switch i8 %610, label %615 [
    i8 43, label %611
    i8 45, label %613
  ]

611:                                              ; preds = %606
  %612 = getelementptr inbounds nuw i8, ptr %429, i64 2
  br label %615

613:                                              ; preds = %606
  %614 = getelementptr inbounds nuw i8, ptr %429, i64 2
  br label %615

615:                                              ; preds = %606, %613, %611
  %616 = phi ptr [ %609, %606 ], [ %612, %611 ], [ %614, %613 ]
  %617 = phi i1 [ true, %606 ], [ true, %611 ], [ false, %613 ]
  %618 = load i8, ptr %616, align 1, !tbaa !5
  %619 = sext i8 %618 to i32
  %620 = add nsw i32 %619, -48
  %621 = icmp ult i32 %620, 10
  br i1 %621, label %622, label %715

622:                                              ; preds = %615, %649
  %623 = phi ptr [ %641, %649 ], [ %616, %615 ]
  %624 = phi i32 [ %654, %649 ], [ %620, %615 ]
  %625 = phi i1 [ %651, %649 ], [ false, %615 ]
  %626 = getelementptr inbounds nuw i8, ptr %623, i64 1
  %627 = load i8, ptr %626, align 1, !tbaa !5
  %628 = sext i8 %627 to i32
  %629 = icmp eq i32 %131, %628
  br i1 %629, label %630, label %639

630:                                              ; preds = %622
  %631 = getelementptr inbounds nuw i8, ptr %623, i64 2
  %632 = load i8, ptr %631, align 1, !tbaa !5
  %633 = sext i8 %632 to i32
  %634 = add nsw i32 %633, -48
  %635 = icmp ult i32 %634, 10
  %636 = select i1 %635, i8 %632, i8 %627
  %637 = select i1 %635, ptr %631, ptr %626
  %638 = sext i8 %636 to i32
  br label %639

639:                                              ; preds = %630, %622
  %640 = phi i32 [ %638, %630 ], [ %628, %622 ]
  %641 = phi ptr [ %637, %630 ], [ %626, %622 ]
  %642 = add nsw i32 %640, -48
  %643 = icmp ult i32 %642, 10
  br i1 %643, label %649, label %644

644:                                              ; preds = %639
  %645 = sub nsw i32 0, %624
  %646 = select i1 %617, i32 %624, i32 %645
  %647 = and i1 %581, %625
  %648 = select i1 %617, i64 9218868437227405312, i64 0
  br i1 %647, label %710, label %655

649:                                              ; preds = %639
  %650 = icmp sgt i32 %624, 214748363
  %651 = select i1 %625, i1 true, i1 %650
  %652 = mul nsw i32 %624, 10
  %653 = add nsw i32 %642, %652
  %654 = select i1 %651, i32 %624, i32 %653
  br label %622

655:                                              ; preds = %644, %596, %602, %603, %599, %592
  %656 = phi ptr [ %429, %603 ], [ %429, %602 ], [ %429, %599 ], [ %429, %592 ], [ %429, %596 ], [ %641, %644 ]
  %657 = phi i32 [ 0, %603 ], [ 0, %602 ], [ 0, %599 ], [ 0, %592 ], [ 0, %596 ], [ %646, %644 ]
  %658 = phi i1 [ true, %603 ], [ true, %602 ], [ true, %599 ], [ true, %592 ], [ true, %596 ], [ %608, %644 ]
  %659 = icmp eq ptr %656, %18
  br i1 %659, label %715, label %660

660:                                              ; preds = %655
  br i1 %581, label %661, label %710

661:                                              ; preds = %660
  br i1 %586, label %662, label %676

662:                                              ; preds = %661
  %663 = select i1 %658, i32 %159, i32 1
  %664 = mul nsw i32 %663, %657
  %665 = mul nsw i32 %585, %159
  %666 = sub nsw i32 %664, %665
  %667 = mul nsw i32 %232, %159
  %668 = add nsw i32 %666, %667
  %669 = add nsw i32 %159, 1024
  %670 = icmp slt i32 %668, %669
  br i1 %670, label %671, label %710

671:                                              ; preds = %662
  %672 = icmp slt i32 %668, -1074
  br i1 %672, label %710, label %673

673:                                              ; preds = %671
  %674 = sub nsw i32 0, %666
  %675 = call fastcc i64 @round_to_d(ptr noundef %6, ptr noundef nonnull %4, i32 noundef %674)
  br label %690

676:                                              ; preds = %661
  %677 = sub nsw i32 %657, %585
  %678 = add nsw i32 %677, %232
  %679 = getelementptr inbounds [35 x i16], ptr @max_exponent, i64 0, i64 %135
  %680 = load i16, ptr %679, align 2, !tbaa !41
  %681 = sext i16 %680 to i32
  %682 = icmp sgt i32 %678, %681
  br i1 %682, label %710, label %683

683:                                              ; preds = %676
  %684 = getelementptr inbounds [35 x i16], ptr @min_exponent, i64 0, i64 %135
  %685 = load i16, ptr %684, align 2, !tbaa !41
  %686 = sext i16 %685 to i32
  %687 = icmp sgt i32 %678, %686
  br i1 %687, label %688, label %710

688:                                              ; preds = %683
  %689 = call fastcc i64 @mul_pow_round_to_d(ptr noundef %6, ptr noundef nonnull %4, i32 noundef %157, i32 noundef %156, i32 noundef %677)
  br label %690

690:                                              ; preds = %688, %673
  %691 = phi i64 [ %675, %673 ], [ %689, %688 ]
  %692 = icmp eq i64 %691, 0
  br i1 %692, label %710, label %693

693:                                              ; preds = %690
  %694 = load i32, ptr %6, align 4, !tbaa !12
  %695 = icmp sgt i32 %694, 1024
  br i1 %695, label %710, label %696

696:                                              ; preds = %693
  %697 = icmp slt i32 %694, -1073
  br i1 %697, label %710, label %698

698:                                              ; preds = %696
  %699 = icmp slt i32 %694, -1021
  br i1 %699, label %700, label %704

700:                                              ; preds = %698
  %701 = sub nuw nsw i32 -1021, %694
  %702 = zext nneg i32 %701 to i64
  %703 = lshr i64 %691, %702
  br label %710

704:                                              ; preds = %698
  %705 = add nsw i32 %694, 1022
  %706 = zext nneg i32 %705 to i64
  %707 = shl nuw nsw i64 %706, 52
  %708 = and i64 %691, 4503599627370495
  %709 = or disjoint i64 %707, %708
  br label %710

710:                                              ; preds = %123, %644, %696, %662, %676, %693, %671, %683, %690, %660, %700, %704
  %711 = phi ptr [ %656, %671 ], [ %656, %690 ], [ %656, %693 ], [ %656, %696 ], [ %656, %700 ], [ %656, %704 ], [ %656, %662 ], [ %656, %676 ], [ %656, %683 ], [ %656, %660 ], [ %641, %644 ], [ %97, %123 ]
  %712 = phi i64 [ 0, %671 ], [ 0, %690 ], [ 9218868437227405312, %693 ], [ 0, %696 ], [ %703, %700 ], [ %709, %704 ], [ 9218868437227405312, %662 ], [ 9218868437227405312, %676 ], [ 0, %683 ], [ 0, %660 ], [ %648, %644 ], [ 9218868437227405312, %123 ]
  %713 = or i64 %712, %19
  %714 = bitcast i64 %713 to double
  br label %715

715:                                              ; preds = %615, %90, %655, %710
  %716 = phi ptr [ %656, %655 ], [ %711, %710 ], [ %73, %90 ], [ %616, %615 ]
  %717 = phi double [ 0x7FF8000000000000, %655 ], [ %714, %710 ], [ 0x7FF8000000000000, %90 ], [ 0x7FF8000000000000, %615 ]
  %718 = icmp eq ptr %1, null
  br i1 %718, label %720, label %719

719:                                              ; preds = %715
  store ptr %716, ptr %1, align 8, !tbaa !43
  br label %720

720:                                              ; preds = %715, %719
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %6) #22
  ret double %717
}

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
define dso_local range(i32 0, 2) i32 @strstart(ptr noundef %0, ptr noundef readonly captures(none) %1, ptr noundef writeonly captures(address_is_null) %2) local_unnamed_addr #0 {
  %4 = load i8, ptr %1, align 1, !tbaa !5
  %5 = icmp eq i8 %4, 0
  br i1 %5, label %17, label %6

6:                                                ; preds = %3, %12
  %7 = phi i8 [ %15, %12 ], [ %4, %3 ]
  %8 = phi ptr [ %14, %12 ], [ %1, %3 ]
  %9 = phi ptr [ %13, %12 ], [ %0, %3 ]
  %10 = load i8, ptr %9, align 1, !tbaa !5
  %11 = icmp eq i8 %10, %7
  br i1 %11, label %12, label %21

12:                                               ; preds = %6
  %13 = getelementptr inbounds nuw i8, ptr %9, i64 1
  %14 = getelementptr inbounds nuw i8, ptr %8, i64 1
  %15 = load i8, ptr %14, align 1, !tbaa !5
  %16 = icmp eq i8 %15, 0
  br i1 %16, label %17, label %6, !llvm.loop !46

17:                                               ; preds = %12, %3
  %18 = phi ptr [ %0, %3 ], [ %13, %12 ]
  %19 = icmp eq ptr %2, null
  br i1 %19, label %21, label %20

20:                                               ; preds = %17
  store ptr %18, ptr %2, align 8, !tbaa !43
  br label %21

21:                                               ; preds = %6, %17, %20
  %22 = phi i32 [ 1, %20 ], [ 1, %17 ], [ 0, %6 ]
  ret i32 %22
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local void @pstrcpy(ptr noundef writeonly captures(address) %0, i32 noundef %1, ptr noundef readonly captures(none) %2) local_unnamed_addr #0 {
  %4 = icmp slt i32 %1, 1
  br i1 %4, label %25, label %5

5:                                                ; preds = %3
  %6 = zext nneg i32 %1 to i64
  %7 = getelementptr inbounds nuw i8, ptr %0, i64 %6
  %8 = getelementptr inbounds i8, ptr %7, i64 -1
  %9 = load i8, ptr %2, align 1, !tbaa !5
  %10 = icmp ne i8 %9, 0
  %11 = icmp ne i32 %1, 1
  %12 = and i1 %10, %11
  br i1 %12, label %13, label %23

13:                                               ; preds = %5, %13
  %14 = phi i8 [ %19, %13 ], [ %9, %5 ]
  %15 = phi ptr [ %18, %13 ], [ %0, %5 ]
  %16 = phi ptr [ %17, %13 ], [ %2, %5 ]
  %17 = getelementptr inbounds nuw i8, ptr %16, i64 1
  %18 = getelementptr inbounds nuw i8, ptr %15, i64 1
  store i8 %14, ptr %15, align 1, !tbaa !5
  %19 = load i8, ptr %17, align 1, !tbaa !5
  %20 = icmp ne i8 %19, 0
  %21 = icmp ult ptr %18, %8
  %22 = select i1 %20, i1 %21, i1 false
  br i1 %22, label %13, label %23

23:                                               ; preds = %13, %5
  %24 = phi ptr [ %0, %5 ], [ %18, %13 ]
  store i8 0, ptr %24, align 1, !tbaa !5
  br label %25

25:                                               ; preds = %3, %23
  ret void
}

; Function Attrs: nofree norecurse nounwind memory(argmem: readwrite) uwtable
define dso_local noundef ptr @pstrcat(ptr noundef returned captures(address, ret: address, provenance) %0, i32 noundef %1, ptr noundef readonly captures(none) %2) local_unnamed_addr #8 {
  %4 = tail call i64 @strlen(ptr noundef nonnull dereferenceable(1) %0) #24
  %5 = trunc i64 %4 to i32
  %6 = icmp sgt i32 %1, %5
  br i1 %6, label %7, label %31

7:                                                ; preds = %3
  %8 = sub nsw i32 %1, %5
  %9 = shl i64 %4, 32
  %10 = ashr exact i64 %9, 32
  %11 = getelementptr inbounds i8, ptr %0, i64 %10
  %12 = zext nneg i32 %8 to i64
  %13 = getelementptr inbounds nuw i8, ptr %11, i64 %12
  %14 = getelementptr inbounds i8, ptr %13, i64 -1
  %15 = load i8, ptr %2, align 1, !tbaa !5
  %16 = icmp ne i8 %15, 0
  %17 = icmp ne i32 %8, 1
  %18 = and i1 %17, %16
  br i1 %18, label %19, label %29

19:                                               ; preds = %7, %19
  %20 = phi i8 [ %25, %19 ], [ %15, %7 ]
  %21 = phi ptr [ %24, %19 ], [ %11, %7 ]
  %22 = phi ptr [ %23, %19 ], [ %2, %7 ]
  %23 = getelementptr inbounds nuw i8, ptr %22, i64 1
  %24 = getelementptr inbounds nuw i8, ptr %21, i64 1
  store i8 %20, ptr %21, align 1, !tbaa !5
  %25 = load i8, ptr %23, align 1, !tbaa !5
  %26 = icmp ne i8 %25, 0
  %27 = icmp ult ptr %24, %14
  %28 = select i1 %26, i1 %27, i1 false
  br i1 %28, label %19, label %29

29:                                               ; preds = %19, %7
  %30 = phi ptr [ %11, %7 ], [ %24, %19 ]
  store i8 0, ptr %30, align 1, !tbaa !5
  br label %31

31:                                               ; preds = %29, %3
  ret ptr %0
}

; Function Attrs: mustprogress nofree nounwind willreturn memory(argmem: read)
declare i64 @strlen(ptr noundef captures(none)) local_unnamed_addr #9

; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(argmem: read) uwtable
define dso_local range(i32 0, 2) i32 @has_suffix(ptr noundef readonly captures(none) %0, ptr noundef readonly captures(none) %1) local_unnamed_addr #10 {
  %3 = tail call i64 @strlen(ptr noundef nonnull dereferenceable(1) %0) #24
  %4 = tail call i64 @strlen(ptr noundef nonnull dereferenceable(1) %1) #24
  %5 = icmp ult i64 %3, %4
  br i1 %5, label %13, label %6

6:                                                ; preds = %2
  %7 = getelementptr inbounds nuw i8, ptr %0, i64 %3
  %8 = sub i64 0, %4
  %9 = getelementptr inbounds i8, ptr %7, i64 %8
  %10 = tail call i32 @bcmp(ptr nonnull %9, ptr nonnull %1, i64 %4)
  %11 = icmp eq i32 %10, 0
  %12 = zext i1 %11 to i32
  br label %13

13:                                               ; preds = %6, %2
  %14 = phi i32 [ 0, %2 ], [ %12, %6 ]
  ret i32 %14
}

; Function Attrs: nofree nounwind willreturn memory(argmem: read)
declare i32 @bcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #11

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write) uwtable
define dso_local void @dbuf_init2(ptr noundef writeonly captures(none) initializes((0, 48)) %0, ptr noundef %1, ptr noundef %2) local_unnamed_addr #12 {
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, i8 0, i64 32, i1 false)
  %4 = icmp eq ptr %2, null
  %5 = select i1 %4, ptr @dbuf_default_realloc, ptr %2
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 40
  store ptr %1, ptr %6, align 8, !tbaa !47
  %7 = getelementptr inbounds nuw i8, ptr %0, i64 32
  store ptr %5, ptr %7, align 8, !tbaa !50
  ret void
}

; Function Attrs: mustprogress nounwind willreturn memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable
define internal noalias noundef ptr @dbuf_default_realloc(ptr readnone captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) #13 {
  %4 = tail call ptr @realloc(ptr noundef %1, i64 noundef %2) #25
  ret ptr %4
}

; Function Attrs: mustprogress nounwind willreturn allockind("realloc") allocsize(1) memory(argmem: readwrite, inaccessiblemem: readwrite)
declare noalias noundef ptr @realloc(ptr allocptr noundef captures(none), i64 noundef) local_unnamed_addr #14

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write) uwtable
define dso_local void @dbuf_init(ptr noundef writeonly captures(none) initializes((0, 48)) %0) local_unnamed_addr #12 {
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, i8 0, i64 32, i1 false)
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 40
  store ptr null, ptr %2, align 8, !tbaa !47
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 32
  store ptr @dbuf_default_realloc, ptr %3, align 8, !tbaa !50
  ret void
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @dbuf_claim(ptr noundef captures(none) %0, i64 noundef %1) local_unnamed_addr #5 {
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %4 = load i64, ptr %3, align 8, !tbaa !51
  %5 = add i64 %4, %1
  %6 = icmp ult i64 %5, %1
  br i1 %6, label %30, label %7

7:                                                ; preds = %2
  %8 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %9 = load i64, ptr %8, align 8, !tbaa !52
  %10 = icmp ugt i64 %5, %9
  br i1 %10, label %11, label %30

11:                                               ; preds = %7
  %12 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %13 = load i32, ptr %12, align 8, !tbaa !53
  %14 = icmp eq i32 %13, 0
  br i1 %14, label %15, label %30

15:                                               ; preds = %11
  %16 = lshr i64 %9, 1
  %17 = add i64 %16, %9
  %18 = icmp ult i64 %17, %9
  br i1 %18, label %30, label %19

19:                                               ; preds = %15
  %20 = tail call i64 @llvm.umax.i64(i64 %17, i64 %5)
  %21 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %22 = load ptr, ptr %21, align 8, !tbaa !50
  %23 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %24 = load ptr, ptr %23, align 8, !tbaa !47
  %25 = load ptr, ptr %0, align 8, !tbaa !54
  %26 = tail call ptr %22(ptr noundef %24, ptr noundef %25, i64 noundef %20) #22
  %27 = icmp eq ptr %26, null
  br i1 %27, label %28, label %29

28:                                               ; preds = %19
  store i32 1, ptr %12, align 8, !tbaa !53
  br label %30

29:                                               ; preds = %19
  store ptr %26, ptr %0, align 8, !tbaa !54
  store i64 %20, ptr %8, align 8, !tbaa !52
  br label %30

30:                                               ; preds = %7, %29, %15, %11, %2, %28
  %31 = phi i32 [ -1, %28 ], [ -1, %2 ], [ -1, %11 ], [ -1, %15 ], [ 0, %29 ], [ 0, %7 ]
  ret i32 %31
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #3

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @dbuf_put(ptr noundef captures(none) %0, ptr noundef readonly captures(none) %1, i64 noundef %2) local_unnamed_addr #5 {
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %5 = load i64, ptr %4, align 8, !tbaa !52
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %7 = load i64, ptr %6, align 8, !tbaa !51
  %8 = sub i64 %5, %7
  %9 = icmp ult i64 %8, %2
  br i1 %9, label %10, label %41

10:                                               ; preds = %3
  %11 = add i64 %7, %2
  %12 = icmp ult i64 %11, %2
  br i1 %12, label %51, label %13

13:                                               ; preds = %10
  %14 = icmp ugt i64 %11, %5
  br i1 %14, label %17, label %15

15:                                               ; preds = %13
  %16 = load ptr, ptr %0, align 8, !tbaa !54
  br label %37

17:                                               ; preds = %13
  %18 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %19 = load i32, ptr %18, align 8, !tbaa !53
  %20 = icmp eq i32 %19, 0
  br i1 %20, label %21, label %51

21:                                               ; preds = %17
  %22 = lshr i64 %5, 1
  %23 = add i64 %22, %5
  %24 = icmp ult i64 %23, %5
  br i1 %24, label %51, label %25

25:                                               ; preds = %21
  %26 = tail call i64 @llvm.umax.i64(i64 %23, i64 %11)
  %27 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %28 = load ptr, ptr %27, align 8, !tbaa !50
  %29 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %30 = load ptr, ptr %29, align 8, !tbaa !47
  %31 = load ptr, ptr %0, align 8, !tbaa !54
  %32 = tail call ptr %28(ptr noundef %30, ptr noundef %31, i64 noundef %26) #22
  %33 = icmp eq ptr %32, null
  br i1 %33, label %34, label %35

34:                                               ; preds = %25
  store i32 1, ptr %18, align 8, !tbaa !53
  br label %51

35:                                               ; preds = %25
  store ptr %32, ptr %0, align 8, !tbaa !54
  store i64 %26, ptr %4, align 8, !tbaa !52
  %36 = load i64, ptr %6, align 8, !tbaa !51
  br label %37

37:                                               ; preds = %15, %35
  %38 = phi i64 [ %7, %15 ], [ %36, %35 ]
  %39 = phi ptr [ %16, %15 ], [ %32, %35 ]
  %40 = getelementptr inbounds nuw i8, ptr %39, i64 %38
  br label %45

41:                                               ; preds = %3
  %42 = load ptr, ptr %0, align 8, !tbaa !54
  %43 = getelementptr inbounds nuw i8, ptr %42, i64 %7
  %44 = icmp eq i64 %2, 0
  br i1 %44, label %48, label %45

45:                                               ; preds = %37, %41
  %46 = phi ptr [ %40, %37 ], [ %43, %41 ]
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 1 %46, ptr readonly align 1 %1, i64 %2, i1 false)
  %47 = load i64, ptr %6, align 8, !tbaa !51
  br label %48

48:                                               ; preds = %41, %45
  %49 = phi i64 [ %7, %41 ], [ %47, %45 ]
  %50 = add i64 %49, %2
  store i64 %50, ptr %6, align 8, !tbaa !51
  br label %51

51:                                               ; preds = %21, %17, %10, %34, %48
  %52 = phi i32 [ 0, %48 ], [ -1, %34 ], [ -1, %10 ], [ -1, %17 ], [ -1, %21 ]
  ret i32 %52
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @dbuf_put_self(ptr noundef captures(none) %0, i64 noundef %1, i64 noundef %2) local_unnamed_addr #5 {
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %5 = load i64, ptr %4, align 8, !tbaa !52
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %7 = load i64, ptr %6, align 8, !tbaa !51
  %8 = sub i64 %5, %7
  %9 = icmp ult i64 %8, %2
  br i1 %9, label %10, label %35

10:                                               ; preds = %3
  %11 = add i64 %7, %2
  %12 = icmp ult i64 %11, %2
  br i1 %12, label %42, label %13

13:                                               ; preds = %10
  %14 = icmp ugt i64 %11, %5
  br i1 %14, label %15, label %35

15:                                               ; preds = %13
  %16 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %17 = load i32, ptr %16, align 8, !tbaa !53
  %18 = icmp eq i32 %17, 0
  br i1 %18, label %19, label %42

19:                                               ; preds = %15
  %20 = lshr i64 %5, 1
  %21 = add i64 %20, %5
  %22 = icmp ult i64 %21, %5
  br i1 %22, label %42, label %23

23:                                               ; preds = %19
  %24 = tail call i64 @llvm.umax.i64(i64 %21, i64 %11)
  %25 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %26 = load ptr, ptr %25, align 8, !tbaa !50
  %27 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %28 = load ptr, ptr %27, align 8, !tbaa !47
  %29 = load ptr, ptr %0, align 8, !tbaa !54
  %30 = tail call ptr %26(ptr noundef %28, ptr noundef %29, i64 noundef %24) #22
  %31 = icmp eq ptr %30, null
  br i1 %31, label %32, label %33

32:                                               ; preds = %23
  store i32 1, ptr %16, align 8, !tbaa !53
  br label %42

33:                                               ; preds = %23
  store ptr %30, ptr %0, align 8, !tbaa !54
  store i64 %24, ptr %4, align 8, !tbaa !52
  %34 = load i64, ptr %6, align 8, !tbaa !51
  br label %35

35:                                               ; preds = %33, %13, %3
  %36 = phi i64 [ %34, %33 ], [ %7, %13 ], [ %7, %3 ]
  %37 = load ptr, ptr %0, align 8, !tbaa !54
  %38 = getelementptr inbounds nuw i8, ptr %37, i64 %36
  %39 = getelementptr inbounds nuw i8, ptr %37, i64 %1
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 1 %38, ptr align 1 %39, i64 %2, i1 false)
  %40 = load i64, ptr %6, align 8, !tbaa !51
  %41 = add i64 %40, %2
  store i64 %41, ptr %6, align 8, !tbaa !51
  br label %42

42:                                               ; preds = %19, %15, %10, %32, %35
  %43 = phi i32 [ 0, %35 ], [ -1, %32 ], [ -1, %10 ], [ -1, %15 ], [ -1, %19 ]
  ret i32 %43
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @__dbuf_putc(ptr noundef captures(none) %0, i8 noundef zeroext %1) local_unnamed_addr #5 {
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %4 = load i64, ptr %3, align 8, !tbaa !52
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %6 = load i64, ptr %5, align 8, !tbaa !51
  %7 = icmp eq i64 %4, %6
  br i1 %7, label %8, label %32

8:                                                ; preds = %2
  %9 = add i64 %4, 1
  %10 = icmp eq i64 %9, 0
  br i1 %10, label %39, label %11

11:                                               ; preds = %8
  %12 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %13 = load i32, ptr %12, align 8, !tbaa !53
  %14 = icmp eq i32 %13, 0
  br i1 %14, label %15, label %39

15:                                               ; preds = %11
  %16 = lshr i64 %4, 1
  %17 = add i64 %16, %4
  %18 = icmp ult i64 %17, %4
  br i1 %18, label %39, label %19

19:                                               ; preds = %15
  %20 = tail call i64 @llvm.umax.i64(i64 %17, i64 %9)
  %21 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %22 = load ptr, ptr %21, align 8, !tbaa !50
  %23 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %24 = load ptr, ptr %23, align 8, !tbaa !47
  %25 = load ptr, ptr %0, align 8, !tbaa !54
  %26 = tail call ptr %22(ptr noundef %24, ptr noundef %25, i64 noundef %20) #22
  %27 = icmp eq ptr %26, null
  br i1 %27, label %28, label %29

28:                                               ; preds = %19
  store i32 1, ptr %12, align 8, !tbaa !53
  br label %39

29:                                               ; preds = %19
  store ptr %26, ptr %0, align 8, !tbaa !54
  store i64 %20, ptr %3, align 8, !tbaa !52
  %30 = load i64, ptr %5, align 8, !tbaa !51
  %31 = getelementptr inbounds nuw i8, ptr %26, i64 %30
  br label %35

32:                                               ; preds = %2
  %33 = load ptr, ptr %0, align 8, !tbaa !54
  %34 = getelementptr inbounds nuw i8, ptr %33, i64 %6
  br label %35

35:                                               ; preds = %32, %29
  %36 = phi ptr [ %31, %29 ], [ %34, %32 ]
  store i8 %1, ptr %36, align 1
  %37 = load i64, ptr %5, align 8, !tbaa !51
  %38 = add i64 %37, 1
  store i64 %38, ptr %5, align 8, !tbaa !51
  br label %39

39:                                               ; preds = %8, %11, %15, %28, %35
  %40 = phi i32 [ 0, %35 ], [ -1, %28 ], [ -1, %8 ], [ -1, %11 ], [ -1, %15 ]
  ret i32 %40
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @__dbuf_put_u16(ptr noundef captures(none) %0, i16 noundef zeroext %1) local_unnamed_addr #5 {
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %4 = load i64, ptr %3, align 8, !tbaa !52
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %6 = load i64, ptr %5, align 8, !tbaa !51
  %7 = sub i64 %4, %6
  %8 = icmp ult i64 %7, 2
  br i1 %8, label %9, label %40

9:                                                ; preds = %2
  %10 = add i64 %6, 2
  %11 = icmp ugt i64 %6, -3
  br i1 %11, label %47, label %12

12:                                               ; preds = %9
  %13 = icmp ugt i64 %10, %4
  br i1 %13, label %16, label %14

14:                                               ; preds = %12
  %15 = load ptr, ptr %0, align 8, !tbaa !54
  br label %36

16:                                               ; preds = %12
  %17 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %18 = load i32, ptr %17, align 8, !tbaa !53
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %20, label %47

20:                                               ; preds = %16
  %21 = lshr i64 %4, 1
  %22 = add i64 %21, %4
  %23 = icmp ult i64 %22, %4
  br i1 %23, label %47, label %24

24:                                               ; preds = %20
  %25 = tail call i64 @llvm.umax.i64(i64 %22, i64 %10)
  %26 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %27 = load ptr, ptr %26, align 8, !tbaa !50
  %28 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %29 = load ptr, ptr %28, align 8, !tbaa !47
  %30 = load ptr, ptr %0, align 8, !tbaa !54
  %31 = tail call ptr %27(ptr noundef %29, ptr noundef %30, i64 noundef %25) #22
  %32 = icmp eq ptr %31, null
  br i1 %32, label %33, label %34

33:                                               ; preds = %24
  store i32 1, ptr %17, align 8, !tbaa !53
  br label %47

34:                                               ; preds = %24
  store ptr %31, ptr %0, align 8, !tbaa !54
  store i64 %25, ptr %3, align 8, !tbaa !52
  %35 = load i64, ptr %5, align 8, !tbaa !51
  br label %36

36:                                               ; preds = %34, %14
  %37 = phi i64 [ %6, %14 ], [ %35, %34 ]
  %38 = phi ptr [ %15, %14 ], [ %31, %34 ]
  %39 = getelementptr inbounds nuw i8, ptr %38, i64 %37
  br label %43

40:                                               ; preds = %2
  %41 = load ptr, ptr %0, align 8, !tbaa !54
  %42 = getelementptr inbounds nuw i8, ptr %41, i64 %6
  br label %43

43:                                               ; preds = %40, %36
  %44 = phi ptr [ %39, %36 ], [ %42, %40 ]
  store i16 %1, ptr %44, align 1
  %45 = load i64, ptr %5, align 8, !tbaa !51
  %46 = add i64 %45, 2
  store i64 %46, ptr %5, align 8, !tbaa !51
  br label %47

47:                                               ; preds = %9, %16, %20, %33, %43
  %48 = phi i32 [ 0, %43 ], [ -1, %33 ], [ -1, %9 ], [ -1, %16 ], [ -1, %20 ]
  ret i32 %48
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @__dbuf_put_u32(ptr noundef captures(none) %0, i32 noundef %1) local_unnamed_addr #5 {
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %4 = load i64, ptr %3, align 8, !tbaa !52
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %6 = load i64, ptr %5, align 8, !tbaa !51
  %7 = sub i64 %4, %6
  %8 = icmp ult i64 %7, 4
  br i1 %8, label %9, label %40

9:                                                ; preds = %2
  %10 = add i64 %6, 4
  %11 = icmp ugt i64 %6, -5
  br i1 %11, label %47, label %12

12:                                               ; preds = %9
  %13 = icmp ugt i64 %10, %4
  br i1 %13, label %16, label %14

14:                                               ; preds = %12
  %15 = load ptr, ptr %0, align 8, !tbaa !54
  br label %36

16:                                               ; preds = %12
  %17 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %18 = load i32, ptr %17, align 8, !tbaa !53
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %20, label %47

20:                                               ; preds = %16
  %21 = lshr i64 %4, 1
  %22 = add i64 %21, %4
  %23 = icmp ult i64 %22, %4
  br i1 %23, label %47, label %24

24:                                               ; preds = %20
  %25 = tail call i64 @llvm.umax.i64(i64 %22, i64 %10)
  %26 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %27 = load ptr, ptr %26, align 8, !tbaa !50
  %28 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %29 = load ptr, ptr %28, align 8, !tbaa !47
  %30 = load ptr, ptr %0, align 8, !tbaa !54
  %31 = tail call ptr %27(ptr noundef %29, ptr noundef %30, i64 noundef %25) #22
  %32 = icmp eq ptr %31, null
  br i1 %32, label %33, label %34

33:                                               ; preds = %24
  store i32 1, ptr %17, align 8, !tbaa !53
  br label %47

34:                                               ; preds = %24
  store ptr %31, ptr %0, align 8, !tbaa !54
  store i64 %25, ptr %3, align 8, !tbaa !52
  %35 = load i64, ptr %5, align 8, !tbaa !51
  br label %36

36:                                               ; preds = %34, %14
  %37 = phi i64 [ %6, %14 ], [ %35, %34 ]
  %38 = phi ptr [ %15, %14 ], [ %31, %34 ]
  %39 = getelementptr inbounds nuw i8, ptr %38, i64 %37
  br label %43

40:                                               ; preds = %2
  %41 = load ptr, ptr %0, align 8, !tbaa !54
  %42 = getelementptr inbounds nuw i8, ptr %41, i64 %6
  br label %43

43:                                               ; preds = %40, %36
  %44 = phi ptr [ %39, %36 ], [ %42, %40 ]
  store i32 %1, ptr %44, align 1
  %45 = load i64, ptr %5, align 8, !tbaa !51
  %46 = add i64 %45, 4
  store i64 %46, ptr %5, align 8, !tbaa !51
  br label %47

47:                                               ; preds = %9, %16, %20, %33, %43
  %48 = phi i32 [ 0, %43 ], [ -1, %33 ], [ -1, %9 ], [ -1, %16 ], [ -1, %20 ]
  ret i32 %48
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @__dbuf_put_u64(ptr noundef captures(none) %0, i64 noundef %1) local_unnamed_addr #5 {
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %4 = load i64, ptr %3, align 8, !tbaa !52
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %6 = load i64, ptr %5, align 8, !tbaa !51
  %7 = sub i64 %4, %6
  %8 = icmp ult i64 %7, 8
  br i1 %8, label %9, label %40

9:                                                ; preds = %2
  %10 = add i64 %6, 8
  %11 = icmp ugt i64 %6, -9
  br i1 %11, label %47, label %12

12:                                               ; preds = %9
  %13 = icmp ugt i64 %10, %4
  br i1 %13, label %16, label %14

14:                                               ; preds = %12
  %15 = load ptr, ptr %0, align 8, !tbaa !54
  br label %36

16:                                               ; preds = %12
  %17 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %18 = load i32, ptr %17, align 8, !tbaa !53
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %20, label %47

20:                                               ; preds = %16
  %21 = lshr i64 %4, 1
  %22 = add i64 %21, %4
  %23 = icmp ult i64 %22, %4
  br i1 %23, label %47, label %24

24:                                               ; preds = %20
  %25 = tail call i64 @llvm.umax.i64(i64 %22, i64 %10)
  %26 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %27 = load ptr, ptr %26, align 8, !tbaa !50
  %28 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %29 = load ptr, ptr %28, align 8, !tbaa !47
  %30 = load ptr, ptr %0, align 8, !tbaa !54
  %31 = tail call ptr %27(ptr noundef %29, ptr noundef %30, i64 noundef %25) #22
  %32 = icmp eq ptr %31, null
  br i1 %32, label %33, label %34

33:                                               ; preds = %24
  store i32 1, ptr %17, align 8, !tbaa !53
  br label %47

34:                                               ; preds = %24
  store ptr %31, ptr %0, align 8, !tbaa !54
  store i64 %25, ptr %3, align 8, !tbaa !52
  %35 = load i64, ptr %5, align 8, !tbaa !51
  br label %36

36:                                               ; preds = %34, %14
  %37 = phi i64 [ %6, %14 ], [ %35, %34 ]
  %38 = phi ptr [ %15, %14 ], [ %31, %34 ]
  %39 = getelementptr inbounds nuw i8, ptr %38, i64 %37
  br label %43

40:                                               ; preds = %2
  %41 = load ptr, ptr %0, align 8, !tbaa !54
  %42 = getelementptr inbounds nuw i8, ptr %41, i64 %6
  br label %43

43:                                               ; preds = %40, %36
  %44 = phi ptr [ %39, %36 ], [ %42, %40 ]
  store i64 %1, ptr %44, align 1
  %45 = load i64, ptr %5, align 8, !tbaa !51
  %46 = add i64 %45, 8
  store i64 %46, ptr %5, align 8, !tbaa !51
  br label %47

47:                                               ; preds = %9, %16, %20, %33, %43
  %48 = phi i32 [ 0, %43 ], [ -1, %33 ], [ -1, %9 ], [ -1, %16 ], [ -1, %20 ]
  ret i32 %48
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @dbuf_putstr(ptr noundef captures(none) %0, ptr noundef readonly captures(none) %1) local_unnamed_addr #5 {
  %3 = tail call i64 @strlen(ptr noundef nonnull dereferenceable(1) %1) #24
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %5 = load i64, ptr %4, align 8, !tbaa !52
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %7 = load i64, ptr %6, align 8, !tbaa !51
  %8 = sub i64 %5, %7
  %9 = icmp ult i64 %8, %3
  br i1 %9, label %10, label %41

10:                                               ; preds = %2
  %11 = add i64 %7, %3
  %12 = icmp ult i64 %11, %3
  br i1 %12, label %51, label %13

13:                                               ; preds = %10
  %14 = icmp ugt i64 %11, %5
  br i1 %14, label %17, label %15

15:                                               ; preds = %13
  %16 = load ptr, ptr %0, align 8, !tbaa !54
  br label %37

17:                                               ; preds = %13
  %18 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %19 = load i32, ptr %18, align 8, !tbaa !53
  %20 = icmp eq i32 %19, 0
  br i1 %20, label %21, label %51

21:                                               ; preds = %17
  %22 = lshr i64 %5, 1
  %23 = add i64 %22, %5
  %24 = icmp ult i64 %23, %5
  br i1 %24, label %51, label %25

25:                                               ; preds = %21
  %26 = tail call i64 @llvm.umax.i64(i64 %23, i64 %11)
  %27 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %28 = load ptr, ptr %27, align 8, !tbaa !50
  %29 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %30 = load ptr, ptr %29, align 8, !tbaa !47
  %31 = load ptr, ptr %0, align 8, !tbaa !54
  %32 = tail call ptr %28(ptr noundef %30, ptr noundef %31, i64 noundef %26) #22
  %33 = icmp eq ptr %32, null
  br i1 %33, label %34, label %35

34:                                               ; preds = %25
  store i32 1, ptr %18, align 8, !tbaa !53
  br label %51

35:                                               ; preds = %25
  store ptr %32, ptr %0, align 8, !tbaa !54
  store i64 %26, ptr %4, align 8, !tbaa !52
  %36 = load i64, ptr %6, align 8, !tbaa !51
  br label %37

37:                                               ; preds = %35, %15
  %38 = phi i64 [ %7, %15 ], [ %36, %35 ]
  %39 = phi ptr [ %16, %15 ], [ %32, %35 ]
  %40 = getelementptr inbounds nuw i8, ptr %39, i64 %38
  br label %45

41:                                               ; preds = %2
  %42 = load ptr, ptr %0, align 8, !tbaa !54
  %43 = getelementptr inbounds nuw i8, ptr %42, i64 %7
  %44 = icmp eq i64 %3, 0
  br i1 %44, label %48, label %45

45:                                               ; preds = %41, %37
  %46 = phi ptr [ %40, %37 ], [ %43, %41 ]
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 1 %46, ptr nonnull readonly align 1 %1, i64 %3, i1 false)
  %47 = load i64, ptr %6, align 8, !tbaa !51
  br label %48

48:                                               ; preds = %45, %41
  %49 = phi i64 [ %7, %41 ], [ %47, %45 ]
  %50 = add i64 %49, %3
  store i64 %50, ptr %6, align 8, !tbaa !51
  br label %51

51:                                               ; preds = %10, %17, %21, %34, %48
  %52 = phi i32 [ 0, %48 ], [ -1, %34 ], [ -1, %10 ], [ -1, %17 ], [ -1, %21 ]
  ret i32 %52
}

; Function Attrs: nounwind uwtable
define dso_local range(i32 -1, 1) i32 @dbuf_printf(ptr noundef captures(none) %0, ptr noundef readonly captures(none) %1, ...) local_unnamed_addr #5 {
  %3 = alloca [1 x %struct.__va_list_tag], align 16
  %4 = alloca [128 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %3) #22
  call void @llvm.lifetime.start.p0(i64 128, ptr nonnull %4) #22
  call void @llvm.va_start.p0(ptr nonnull %3)
  %5 = call i32 @vsnprintf(ptr noundef nonnull %4, i64 noundef 128, ptr noundef %1, ptr noundef nonnull %3) #22
  call void @llvm.va_end.p0(ptr nonnull %3)
  %6 = icmp slt i32 %5, 0
  br i1 %6, label %95, label %7

7:                                                ; preds = %2
  %8 = zext nneg i32 %5 to i64
  %9 = icmp samesign ult i32 %5, 128
  %10 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %11 = load i64, ptr %10, align 8, !tbaa !51
  br i1 %9, label %12, label %58

12:                                               ; preds = %7
  %13 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %14 = load i64, ptr %13, align 8, !tbaa !52
  %15 = sub i64 %14, %11
  %16 = icmp ult i64 %15, %8
  br i1 %16, label %17, label %48

17:                                               ; preds = %12
  %18 = add i64 %11, %8
  %19 = icmp ult i64 %18, %8
  br i1 %19, label %95, label %20

20:                                               ; preds = %17
  %21 = icmp ugt i64 %18, %14
  br i1 %21, label %24, label %22

22:                                               ; preds = %20
  %23 = load ptr, ptr %0, align 8, !tbaa !54
  br label %44

24:                                               ; preds = %20
  %25 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %26 = load i32, ptr %25, align 8, !tbaa !53
  %27 = icmp eq i32 %26, 0
  br i1 %27, label %28, label %95

28:                                               ; preds = %24
  %29 = lshr i64 %14, 1
  %30 = add i64 %29, %14
  %31 = icmp ult i64 %30, %14
  br i1 %31, label %95, label %32

32:                                               ; preds = %28
  %33 = call i64 @llvm.umax.i64(i64 %30, i64 %18)
  %34 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %35 = load ptr, ptr %34, align 8, !tbaa !50
  %36 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %37 = load ptr, ptr %36, align 8, !tbaa !47
  %38 = load ptr, ptr %0, align 8, !tbaa !54
  %39 = call ptr %35(ptr noundef %37, ptr noundef %38, i64 noundef %33) #22
  %40 = icmp eq ptr %39, null
  br i1 %40, label %41, label %42

41:                                               ; preds = %32
  store i32 1, ptr %25, align 8, !tbaa !53
  br label %95

42:                                               ; preds = %32
  store ptr %39, ptr %0, align 8, !tbaa !54
  store i64 %33, ptr %13, align 8, !tbaa !52
  %43 = load i64, ptr %10, align 8, !tbaa !51
  br label %44

44:                                               ; preds = %42, %22
  %45 = phi i64 [ %11, %22 ], [ %43, %42 ]
  %46 = phi ptr [ %23, %22 ], [ %39, %42 ]
  %47 = getelementptr inbounds nuw i8, ptr %46, i64 %45
  br label %52

48:                                               ; preds = %12
  %49 = load ptr, ptr %0, align 8, !tbaa !54
  %50 = getelementptr inbounds nuw i8, ptr %49, i64 %11
  %51 = icmp eq i32 %5, 0
  br i1 %51, label %55, label %52

52:                                               ; preds = %48, %44
  %53 = phi ptr [ %47, %44 ], [ %50, %48 ]
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %53, ptr nonnull readonly align 16 %4, i64 %8, i1 false)
  %54 = load i64, ptr %10, align 8, !tbaa !51
  br label %55

55:                                               ; preds = %52, %48
  %56 = phi i64 [ %11, %48 ], [ %54, %52 ]
  %57 = add i64 %56, %8
  store i64 %57, ptr %10, align 8, !tbaa !51
  br label %95

58:                                               ; preds = %7
  %59 = add nuw nsw i32 %5, 1
  %60 = zext nneg i32 %59 to i64
  %61 = add i64 %11, %60
  %62 = icmp ult i64 %61, %60
  br i1 %62, label %95, label %63

63:                                               ; preds = %58
  %64 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %65 = load i64, ptr %64, align 8, !tbaa !52
  %66 = icmp ugt i64 %61, %65
  br i1 %66, label %67, label %86

67:                                               ; preds = %63
  %68 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %69 = load i32, ptr %68, align 8, !tbaa !53
  %70 = icmp eq i32 %69, 0
  br i1 %70, label %71, label %95

71:                                               ; preds = %67
  %72 = lshr i64 %65, 1
  %73 = add i64 %72, %65
  %74 = icmp ult i64 %73, %65
  br i1 %74, label %95, label %75

75:                                               ; preds = %71
  %76 = call i64 @llvm.umax.i64(i64 %73, i64 %61)
  %77 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %78 = load ptr, ptr %77, align 8, !tbaa !50
  %79 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %80 = load ptr, ptr %79, align 8, !tbaa !47
  %81 = load ptr, ptr %0, align 8, !tbaa !54
  %82 = call ptr %78(ptr noundef %80, ptr noundef %81, i64 noundef %76) #22
  %83 = icmp eq ptr %82, null
  br i1 %83, label %84, label %85

84:                                               ; preds = %75
  store i32 1, ptr %68, align 8, !tbaa !53
  br label %95

85:                                               ; preds = %75
  store ptr %82, ptr %0, align 8, !tbaa !54
  store i64 %76, ptr %64, align 8, !tbaa !52
  br label %86

86:                                               ; preds = %85, %63
  call void @llvm.va_start.p0(ptr nonnull %3)
  %87 = load ptr, ptr %0, align 8, !tbaa !54
  %88 = load i64, ptr %10, align 8, !tbaa !51
  %89 = getelementptr inbounds nuw i8, ptr %87, i64 %88
  %90 = load i64, ptr %64, align 8, !tbaa !52
  %91 = sub i64 %90, %88
  %92 = call i32 @vsnprintf(ptr noundef %89, i64 noundef %91, ptr noundef %1, ptr noundef nonnull %3) #22
  call void @llvm.va_end.p0(ptr nonnull %3)
  %93 = load i64, ptr %10, align 8, !tbaa !51
  %94 = add i64 %93, %8
  store i64 %94, ptr %10, align 8, !tbaa !51
  br label %95

95:                                               ; preds = %71, %67, %58, %84, %55, %41, %28, %24, %17, %2, %86
  %96 = phi i32 [ 0, %86 ], [ -1, %2 ], [ 0, %55 ], [ -1, %41 ], [ -1, %17 ], [ -1, %24 ], [ -1, %28 ], [ -1, %84 ], [ -1, %58 ], [ -1, %67 ], [ -1, %71 ]
  call void @llvm.lifetime.end.p0(i64 128, ptr nonnull %4) #22
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %3) #22
  ret i32 %96
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.va_start.p0(ptr) #15

; Function Attrs: nofree nounwind
declare noundef i32 @vsnprintf(ptr noundef captures(none), i64 noundef, ptr noundef readonly captures(none), ptr noundef) local_unnamed_addr #16

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.va_end.p0(ptr) #15

; Function Attrs: nounwind uwtable
define dso_local void @dbuf_free(ptr noundef captures(none) initializes((8, 32)) %0) local_unnamed_addr #5 {
  %2 = load ptr, ptr %0, align 8, !tbaa !54
  %3 = icmp eq ptr %2, null
  br i1 %3, label %10, label %4

4:                                                ; preds = %1
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %6 = load ptr, ptr %5, align 8, !tbaa !50
  %7 = getelementptr inbounds nuw i8, ptr %0, i64 40
  %8 = load ptr, ptr %7, align 8, !tbaa !47
  %9 = tail call ptr %6(ptr noundef %8, ptr noundef nonnull %2, i64 noundef 0) #22
  br label %10

10:                                               ; preds = %4, %1
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, i8 0, i64 48, i1 false)
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write) uwtable
define dso_local i32 @unicode_to_utf8(ptr noundef %0, i32 noundef %1) local_unnamed_addr #12 {
  %3 = icmp ult i32 %1, 128
  br i1 %3, label %4, label %7

4:                                                ; preds = %2
  %5 = trunc nuw nsw i32 %1 to i8
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 %5, ptr %0, align 1, !tbaa !5
  br label %72

7:                                                ; preds = %2
  %8 = icmp ult i32 %1, 2048
  br i1 %8, label %9, label %13

9:                                                ; preds = %7
  %10 = lshr i32 %1, 6
  %11 = trunc nuw nsw i32 %10 to i8
  %12 = or disjoint i8 %11, -64
  store i8 %12, ptr %0, align 1, !tbaa !5
  br label %65

13:                                               ; preds = %7
  %14 = icmp ult i32 %1, 65536
  br i1 %14, label %15, label %20

15:                                               ; preds = %13
  %16 = lshr i32 %1, 12
  %17 = trunc nuw nsw i32 %16 to i8
  %18 = or disjoint i8 %17, -32
  %19 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 %18, ptr %0, align 1, !tbaa !5
  br label %59

20:                                               ; preds = %13
  %21 = icmp ult i32 %1, 2097152
  br i1 %21, label %22, label %26

22:                                               ; preds = %20
  %23 = lshr i32 %1, 18
  %24 = trunc nuw nsw i32 %23 to i8
  %25 = or disjoint i8 %24, -16
  store i8 %25, ptr %0, align 1, !tbaa !5
  br label %51

26:                                               ; preds = %20
  %27 = icmp ult i32 %1, 67108864
  br i1 %27, label %28, label %33

28:                                               ; preds = %26
  %29 = lshr i32 %1, 24
  %30 = trunc nuw nsw i32 %29 to i8
  %31 = or disjoint i8 %30, -8
  %32 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 %31, ptr %0, align 1, !tbaa !5
  br label %45

33:                                               ; preds = %26
  %34 = icmp sgt i32 %1, -1
  br i1 %34, label %35, label %78

35:                                               ; preds = %33
  %36 = lshr i32 %1, 30
  %37 = trunc nuw nsw i32 %36 to i8
  %38 = or disjoint i8 %37, -4
  %39 = getelementptr inbounds nuw i8, ptr %0, i64 1
  store i8 %38, ptr %0, align 1, !tbaa !5
  %40 = lshr i32 %1, 24
  %41 = trunc nuw nsw i32 %40 to i8
  %42 = and i8 %41, 63
  %43 = or disjoint i8 %42, -128
  %44 = getelementptr inbounds nuw i8, ptr %0, i64 2
  store i8 %43, ptr %39, align 1, !tbaa !5
  br label %45

45:                                               ; preds = %35, %28
  %46 = phi ptr [ %32, %28 ], [ %44, %35 ]
  %47 = lshr i32 %1, 18
  %48 = trunc i32 %47 to i8
  %49 = and i8 %48, 63
  %50 = or disjoint i8 %49, -128
  store i8 %50, ptr %46, align 1, !tbaa !5
  br label %51

51:                                               ; preds = %45, %22
  %52 = phi ptr [ %0, %22 ], [ %46, %45 ]
  %53 = getelementptr inbounds nuw i8, ptr %52, i64 1
  %54 = lshr i32 %1, 12
  %55 = trunc i32 %54 to i8
  %56 = and i8 %55, 63
  %57 = or disjoint i8 %56, -128
  %58 = getelementptr inbounds nuw i8, ptr %52, i64 2
  store i8 %57, ptr %53, align 1, !tbaa !5
  br label %59

59:                                               ; preds = %51, %15
  %60 = phi ptr [ %19, %15 ], [ %58, %51 ]
  %61 = lshr i32 %1, 6
  %62 = trunc i32 %61 to i8
  %63 = and i8 %62, 63
  %64 = or disjoint i8 %63, -128
  store i8 %64, ptr %60, align 1, !tbaa !5
  br label %65

65:                                               ; preds = %59, %9
  %66 = phi ptr [ %0, %9 ], [ %60, %59 ]
  %67 = getelementptr inbounds nuw i8, ptr %66, i64 1
  %68 = trunc i32 %1 to i8
  %69 = and i8 %68, 63
  %70 = or disjoint i8 %69, -128
  %71 = getelementptr inbounds nuw i8, ptr %66, i64 2
  store i8 %70, ptr %67, align 1, !tbaa !5
  br label %72

72:                                               ; preds = %65, %4
  %73 = phi ptr [ %6, %4 ], [ %71, %65 ]
  %74 = ptrtoint ptr %73 to i64
  %75 = ptrtoint ptr %0 to i64
  %76 = sub i64 %74, %75
  %77 = trunc i64 %76 to i32
  br label %78

78:                                               ; preds = %33, %72
  %79 = phi i32 [ %77, %72 ], [ 0, %33 ]
  ret i32 %79
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define dso_local i32 @unicode_from_utf8(ptr noundef %0, i32 noundef %1, ptr noundef writeonly captures(none) %2) local_unnamed_addr #0 {
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 1
  %5 = load i8, ptr %0, align 1, !tbaa !5
  %6 = icmp sgt i8 %5, -1
  br i1 %6, label %7, label %9

7:                                                ; preds = %3
  %8 = zext nneg i8 %5 to i32
  br label %44

9:                                                ; preds = %3
  %10 = add nsw i8 %5, 64
  %11 = icmp ult i8 %10, 62
  br i1 %11, label %12, label %47

12:                                               ; preds = %9
  %13 = zext nneg i8 %10 to i64
  %14 = getelementptr inbounds nuw [62 x i32], ptr @switch.table.unicode_from_utf8, i64 0, i64 %13
  %15 = load i32, ptr %14, align 4
  %16 = icmp slt i32 %15, %1
  br i1 %16, label %17, label %47

17:                                               ; preds = %12
  %18 = add nsw i32 %15, -1
  %19 = zext nneg i32 %18 to i64
  %20 = getelementptr inbounds nuw [5 x i8], ptr @utf8_first_code_mask, i64 0, i64 %19
  %21 = load i8, ptr %20, align 1, !tbaa !5
  %22 = and i8 %21, %5
  %23 = zext i8 %22 to i32
  %24 = getelementptr i8, ptr %0, i64 %19
  %25 = getelementptr i8, ptr %24, i64 2
  br label %26

26:                                               ; preds = %17, %32
  %27 = phi i32 [ 0, %17 ], [ %38, %32 ]
  %28 = phi i32 [ %23, %17 ], [ %37, %32 ]
  %29 = phi ptr [ %4, %17 ], [ %33, %32 ]
  %30 = load i8, ptr %29, align 1, !tbaa !5
  %31 = icmp sgt i8 %30, -65
  br i1 %31, label %47, label %32

32:                                               ; preds = %26
  %33 = getelementptr inbounds nuw i8, ptr %29, i64 1
  %34 = shl i32 %28, 6
  %35 = and i8 %30, 63
  %36 = zext nneg i8 %35 to i32
  %37 = or disjoint i32 %34, %36
  %38 = add nuw nsw i32 %27, 1
  %39 = icmp eq i32 %38, %15
  br i1 %39, label %40, label %26, !llvm.loop !55

40:                                               ; preds = %32
  %41 = getelementptr inbounds nuw [5 x i32], ptr @utf8_min_code, i64 0, i64 %19
  %42 = load i32, ptr %41, align 4, !tbaa !12
  %43 = icmp ult i32 %37, %42
  br i1 %43, label %47, label %44

44:                                               ; preds = %40, %7
  %45 = phi ptr [ %4, %7 ], [ %25, %40 ]
  %46 = phi i32 [ %8, %7 ], [ %37, %40 ]
  store ptr %45, ptr %2, align 8, !tbaa !43
  br label %47

47:                                               ; preds = %26, %9, %44, %40, %12
  %48 = phi i32 [ -1, %9 ], [ -1, %12 ], [ -1, %40 ], [ %46, %44 ], [ -1, %26 ]
  ret i32 %48
}

; Function Attrs: nounwind uwtable
define dso_local void @rqsort(ptr noundef %0, i64 noundef %1, i64 noundef %2, ptr noundef readonly captures(none) %3, ptr noundef %4) local_unnamed_addr #5 {
  %6 = alloca [50 x %struct.anon], align 16
  call void @llvm.lifetime.start.p0(i64 1200, ptr nonnull %6) #22
  %7 = ptrtoint ptr %0 to i64
  %8 = or i64 %2, %7
  %9 = and i64 %8, 15
  switch i64 %9, label %22 [
    i64 0, label %10
    i64 8, label %13
    i64 4, label %16
    i64 12, label %16
    i64 2, label %19
    i64 6, label %19
    i64 10, label %19
    i64 14, label %19
  ]

10:                                               ; preds = %5
  %11 = icmp eq i64 %2, 16
  %12 = select i1 %11, ptr @exchange_one_int128, ptr @exchange_int128s
  br label %31

13:                                               ; preds = %5
  %14 = icmp eq i64 %2, 8
  %15 = select i1 %14, ptr @exchange_one_int64, ptr @exchange_int64s
  br label %31

16:                                               ; preds = %5, %5
  %17 = icmp eq i64 %2, 4
  %18 = select i1 %17, ptr @exchange_one_int32, ptr @exchange_int32s
  br label %25

19:                                               ; preds = %5, %5, %5, %5
  %20 = icmp eq i64 %2, 2
  %21 = select i1 %20, ptr @exchange_one_int16, ptr @exchange_int16s
  br label %25

22:                                               ; preds = %5
  %23 = icmp eq i64 %2, 1
  %24 = select i1 %23, ptr @exchange_one_byte, ptr @exchange_bytes
  br label %25

25:                                               ; preds = %16, %19, %22
  %26 = phi ptr [ %18, %16 ], [ %21, %19 ], [ %24, %22 ]
  switch i64 %9, label %30 [
    i64 0, label %31
    i64 8, label %27
    i64 4, label %28
    i64 12, label %28
    i64 2, label %29
    i64 6, label %29
    i64 10, label %29
    i64 14, label %29
  ]

27:                                               ; preds = %25
  br label %31

28:                                               ; preds = %25, %25
  br label %31

29:                                               ; preds = %25, %25, %25, %25
  br label %31

30:                                               ; preds = %25
  br label %31

31:                                               ; preds = %13, %10, %25, %27, %28, %29, %30
  %32 = phi ptr [ %26, %28 ], [ %26, %29 ], [ %26, %30 ], [ %26, %25 ], [ %12, %10 ], [ %15, %13 ], [ %26, %27 ]
  %33 = phi ptr [ @exchange_int32s, %28 ], [ @exchange_int16s, %29 ], [ @exchange_bytes, %30 ], [ @exchange_int128s, %25 ], [ @exchange_int128s, %10 ], [ @exchange_int64s, %13 ], [ @exchange_int64s, %27 ]
  %34 = icmp ult i64 %1, 2
  %35 = icmp eq i64 %2, 0
  %36 = or i1 %34, %35
  br i1 %36, label %282, label %37

37:                                               ; preds = %31
  store ptr %0, ptr %6, align 16, !tbaa !56
  %38 = getelementptr inbounds nuw i8, ptr %6, i64 8
  store i64 %1, ptr %38, align 8, !tbaa !58
  %39 = getelementptr inbounds nuw i8, ptr %6, i64 16
  store i32 0, ptr %39, align 16, !tbaa !59
  %40 = getelementptr inbounds nuw i8, ptr %6, i64 24
  %41 = sub i64 0, %2
  %42 = icmp eq i64 %2, 2
  %43 = select i1 %42, ptr @exchange_one_int16, ptr @exchange_int16s
  %44 = icmp eq i64 %2, 4
  %45 = select i1 %44, ptr @exchange_one_int32, ptr @exchange_int32s
  %46 = icmp eq i64 %2, 8
  %47 = select i1 %46, ptr @exchange_one_int64, ptr @exchange_int64s
  %48 = icmp eq i64 %2, 16
  %49 = select i1 %48, ptr @exchange_one_int128, ptr @exchange_int128s
  %50 = icmp eq i64 %2, 1
  %51 = select i1 %50, ptr @exchange_one_byte, ptr @exchange_bytes
  br label %54

52:                                               ; preds = %279, %260
  %53 = icmp ugt ptr %262, %6
  br i1 %53, label %54, label %282, !llvm.loop !60

54:                                               ; preds = %37, %52
  %55 = phi ptr [ %40, %37 ], [ %262, %52 ]
  %56 = getelementptr inbounds i8, ptr %55, i64 -24
  %57 = load ptr, ptr %56, align 8, !tbaa !56
  %58 = getelementptr inbounds i8, ptr %55, i64 -16
  %59 = load i64, ptr %58, align 8, !tbaa !58
  %60 = icmp ugt i64 %59, 6
  br i1 %60, label %61, label %260

61:                                               ; preds = %54
  %62 = getelementptr inbounds i8, ptr %55, i64 -8
  %63 = load i32, ptr %62, align 8, !tbaa !59
  %64 = call i32 @llvm.smax.i32(i32 %63, i32 50)
  br label %65

65:                                               ; preds = %61, %251
  %66 = phi i64 [ %255, %251 ], [ %59, %61 ]
  %67 = phi ptr [ %258, %251 ], [ %56, %61 ]
  %68 = phi i32 [ %70, %251 ], [ %63, %61 ]
  %69 = phi ptr [ %254, %251 ], [ %57, %61 ]
  %70 = add nsw i32 %68, 1
  %71 = icmp eq i32 %68, %64
  br i1 %71, label %72, label %146

72:                                               ; preds = %65
  %73 = ptrtoint ptr %69 to i64
  %74 = or i64 %2, %73
  %75 = and i64 %74, 15
  switch i64 %75, label %79 [
    i64 0, label %80
    i64 8, label %76
    i64 4, label %77
    i64 12, label %77
    i64 2, label %78
    i64 6, label %78
    i64 10, label %78
    i64 14, label %78
  ]

76:                                               ; preds = %72
  br label %80

77:                                               ; preds = %72, %72
  br label %80

78:                                               ; preds = %72, %72, %72, %72
  br label %80

79:                                               ; preds = %72
  br label %80

80:                                               ; preds = %72, %79, %78, %77, %76
  %81 = phi ptr [ %47, %76 ], [ %45, %77 ], [ %43, %78 ], [ %51, %79 ], [ %49, %72 ]
  %82 = lshr i64 %66, 1
  %83 = mul i64 %82, %2
  %84 = mul i64 %66, %2
  %85 = icmp eq i64 %83, 0
  %86 = sub i64 %84, %2
  br i1 %85, label %87, label %89

87:                                               ; preds = %116, %80
  %88 = icmp eq i64 %86, 0
  br i1 %88, label %260, label %118

89:                                               ; preds = %80, %116
  %90 = phi i64 [ %91, %116 ], [ %83, %80 ]
  %91 = sub i64 %90, %2
  %92 = shl i64 %91, 1
  %93 = add i64 %92, %2
  %94 = icmp ult i64 %93, %84
  br i1 %94, label %95, label %116

95:                                               ; preds = %89, %112
  %96 = phi i64 [ %114, %112 ], [ %93, %89 ]
  %97 = phi i64 [ %107, %112 ], [ %91, %89 ]
  %98 = icmp ult i64 %96, %86
  br i1 %98, label %99, label %106

99:                                               ; preds = %95
  %100 = getelementptr inbounds nuw i8, ptr %69, i64 %96
  %101 = getelementptr inbounds nuw i8, ptr %100, i64 %2
  %102 = call i32 %3(ptr noundef %100, ptr noundef nonnull %101, ptr noundef %4) #22
  %103 = icmp slt i32 %102, 1
  %104 = select i1 %103, i64 %2, i64 0
  %105 = add i64 %104, %96
  br label %106

106:                                              ; preds = %99, %95
  %107 = phi i64 [ %96, %95 ], [ %105, %99 ]
  %108 = getelementptr inbounds nuw i8, ptr %69, i64 %97
  %109 = getelementptr inbounds nuw i8, ptr %69, i64 %107
  %110 = call i32 %3(ptr noundef %108, ptr noundef %109, ptr noundef %4) #22
  %111 = icmp sgt i32 %110, 0
  br i1 %111, label %116, label %112

112:                                              ; preds = %106
  call void %81(ptr noundef %108, ptr noundef %109, i64 noundef range(i64 1, 0) %2) #22
  %113 = shl i64 %107, 1
  %114 = add i64 %113, %2
  %115 = icmp ult i64 %114, %84
  br i1 %115, label %95, label %116, !llvm.loop !61

116:                                              ; preds = %112, %106, %89
  %117 = icmp eq i64 %91, 0
  br i1 %117, label %87, label %89, !llvm.loop !62

118:                                              ; preds = %87, %144
  %119 = phi i64 [ %122, %144 ], [ %86, %87 ]
  %120 = getelementptr inbounds nuw i8, ptr %69, i64 %119
  call void %81(ptr noundef %69, ptr noundef nonnull %120, i64 noundef range(i64 1, 0) %2) #22
  %121 = icmp ult i64 %2, %119
  %122 = sub i64 %119, %2
  br i1 %121, label %123, label %144

123:                                              ; preds = %118, %140
  %124 = phi i64 [ %142, %140 ], [ %2, %118 ]
  %125 = phi i64 [ %135, %140 ], [ 0, %118 ]
  %126 = icmp ult i64 %124, %122
  br i1 %126, label %127, label %134

127:                                              ; preds = %123
  %128 = getelementptr inbounds nuw i8, ptr %69, i64 %124
  %129 = getelementptr inbounds nuw i8, ptr %128, i64 %2
  %130 = call i32 %3(ptr noundef %128, ptr noundef nonnull %129, ptr noundef %4) #22
  %131 = icmp slt i32 %130, 1
  %132 = select i1 %131, i64 %2, i64 0
  %133 = add i64 %132, %124
  br label %134

134:                                              ; preds = %127, %123
  %135 = phi i64 [ %124, %123 ], [ %133, %127 ]
  %136 = getelementptr inbounds nuw i8, ptr %69, i64 %125
  %137 = getelementptr inbounds nuw i8, ptr %69, i64 %135
  %138 = call i32 %3(ptr noundef %136, ptr noundef %137, ptr noundef %4) #22
  %139 = icmp sgt i32 %138, 0
  br i1 %139, label %144, label %140

140:                                              ; preds = %134
  call void %81(ptr noundef %136, ptr noundef %137, i64 noundef range(i64 1, 0) %2) #22
  %141 = shl i64 %135, 1
  %142 = add i64 %141, %2
  %143 = icmp ult i64 %142, %119
  br i1 %143, label %123, label %144, !llvm.loop !63

144:                                              ; preds = %140, %134, %118
  %145 = icmp eq i64 %122, 0
  br i1 %145, label %260, label %118, !llvm.loop !64

146:                                              ; preds = %65
  %147 = lshr i64 %66, 2
  %148 = mul i64 %147, %2
  %149 = getelementptr inbounds nuw i8, ptr %69, i64 %148
  %150 = shl i64 %148, 1
  %151 = getelementptr inbounds nuw i8, ptr %69, i64 %150
  %152 = mul i64 %148, 3
  %153 = getelementptr inbounds nuw i8, ptr %69, i64 %152
  %154 = call i32 %3(ptr noundef %149, ptr noundef %151, ptr noundef %4) #22
  %155 = icmp slt i32 %154, 0
  %156 = call i32 %3(ptr noundef %151, ptr noundef %153, ptr noundef %4) #22
  br i1 %155, label %157, label %163

157:                                              ; preds = %146
  %158 = icmp slt i32 %156, 0
  br i1 %158, label %169, label %159

159:                                              ; preds = %157
  %160 = call i32 %3(ptr noundef %149, ptr noundef %153, ptr noundef %4) #22
  %161 = icmp slt i32 %160, 0
  %162 = select i1 %161, ptr %153, ptr %149
  br label %169

163:                                              ; preds = %146
  %164 = icmp sgt i32 %156, 0
  br i1 %164, label %169, label %165

165:                                              ; preds = %163
  %166 = call i32 %3(ptr noundef %149, ptr noundef %153, ptr noundef %4) #22
  %167 = icmp slt i32 %166, 0
  %168 = select i1 %167, ptr %149, ptr %153
  br label %169

169:                                              ; preds = %157, %159, %163, %165
  %170 = phi ptr [ %162, %159 ], [ %168, %165 ], [ %151, %157 ], [ %151, %163 ]
  call void %32(ptr noundef %69, ptr noundef %170, i64 noundef %2) #22
  %171 = getelementptr inbounds nuw i8, ptr %69, i64 %2
  %172 = mul i64 %66, %2
  %173 = getelementptr inbounds nuw i8, ptr %69, i64 %172
  br label %174

174:                                              ; preds = %224, %169
  %175 = phi ptr [ %173, %169 ], [ %209, %224 ]
  %176 = phi ptr [ %171, %169 ], [ %202, %224 ]
  %177 = phi ptr [ %173, %169 ], [ %211, %224 ]
  %178 = phi ptr [ %171, %169 ], [ %226, %224 ]
  %179 = phi i64 [ 1, %169 ], [ %225, %224 ]
  %180 = phi i64 [ 1, %169 ], [ %205, %224 ]
  %181 = phi i64 [ %66, %169 ], [ %210, %224 ]
  %182 = icmp ult ptr %178, %175
  br i1 %182, label %183, label %201

183:                                              ; preds = %174, %195
  %184 = phi i64 [ %197, %195 ], [ %180, %174 ]
  %185 = phi i64 [ %198, %195 ], [ %179, %174 ]
  %186 = phi ptr [ %199, %195 ], [ %178, %174 ]
  %187 = phi ptr [ %196, %195 ], [ %176, %174 ]
  %188 = call i32 %3(ptr noundef %69, ptr noundef %186, ptr noundef %4) #22
  %189 = icmp sgt i32 %188, -1
  br i1 %189, label %190, label %201

190:                                              ; preds = %183
  %191 = icmp eq i32 %188, 0
  br i1 %191, label %192, label %195

192:                                              ; preds = %190
  call void %32(ptr noundef %187, ptr noundef %186, i64 noundef %2) #22
  %193 = add i64 %184, 1
  %194 = getelementptr inbounds nuw i8, ptr %187, i64 %2
  br label %195

195:                                              ; preds = %192, %190
  %196 = phi ptr [ %194, %192 ], [ %187, %190 ]
  %197 = phi i64 [ %193, %192 ], [ %184, %190 ]
  %198 = add i64 %185, 1
  %199 = getelementptr inbounds nuw i8, ptr %186, i64 %2
  %200 = icmp ult ptr %199, %175
  br i1 %200, label %183, label %201, !llvm.loop !65

201:                                              ; preds = %183, %195, %174
  %202 = phi ptr [ %176, %174 ], [ %196, %195 ], [ %187, %183 ]
  %203 = phi ptr [ %178, %174 ], [ %199, %195 ], [ %186, %183 ]
  %204 = phi i64 [ %179, %174 ], [ %198, %195 ], [ %185, %183 ]
  %205 = phi i64 [ %180, %174 ], [ %197, %195 ], [ %184, %183 ]
  %206 = getelementptr inbounds i8, ptr %175, i64 %41
  %207 = icmp ult ptr %203, %206
  br i1 %207, label %208, label %227

208:                                              ; preds = %201, %219
  %209 = phi ptr [ %222, %219 ], [ %206, %201 ]
  %210 = phi i64 [ %221, %219 ], [ %181, %201 ]
  %211 = phi ptr [ %220, %219 ], [ %177, %201 ]
  %212 = call i32 %3(ptr noundef %69, ptr noundef nonnull %209, ptr noundef %4) #22
  %213 = icmp slt i32 %212, 1
  br i1 %213, label %214, label %224

214:                                              ; preds = %208
  %215 = icmp eq i32 %212, 0
  br i1 %215, label %216, label %219

216:                                              ; preds = %214
  %217 = add i64 %210, -1
  %218 = getelementptr inbounds i8, ptr %211, i64 %41
  call void %32(ptr noundef nonnull %218, ptr noundef nonnull %209, i64 noundef %2) #22
  br label %219

219:                                              ; preds = %216, %214
  %220 = phi ptr [ %218, %216 ], [ %211, %214 ]
  %221 = phi i64 [ %217, %216 ], [ %210, %214 ]
  %222 = getelementptr inbounds i8, ptr %209, i64 %41
  %223 = icmp ult ptr %203, %222
  br i1 %223, label %208, label %227, !llvm.loop !66

224:                                              ; preds = %208
  call void %32(ptr noundef %203, ptr noundef nonnull %209, i64 noundef %2) #22
  %225 = add i64 %204, 1
  %226 = getelementptr inbounds nuw i8, ptr %203, i64 %2
  br label %174

227:                                              ; preds = %201, %219
  %228 = phi ptr [ %220, %219 ], [ %177, %201 ]
  %229 = phi i64 [ %221, %219 ], [ %181, %201 ]
  %230 = ptrtoint ptr %202 to i64
  %231 = ptrtoint ptr %69 to i64
  %232 = sub i64 %230, %231
  %233 = ptrtoint ptr %203 to i64
  %234 = sub i64 %233, %230
  %235 = sub i64 %204, %205
  %236 = call i64 @llvm.umin.i64(i64 %232, i64 %234)
  %237 = sub i64 0, %236
  %238 = getelementptr inbounds i8, ptr %203, i64 %237
  call void %33(ptr noundef %69, ptr noundef %238, i64 noundef %236) #22
  %239 = ptrtoint ptr %173 to i64
  %240 = ptrtoint ptr %228 to i64
  %241 = sub i64 %239, %240
  %242 = sub i64 %240, %233
  %243 = sub i64 0, %242
  %244 = getelementptr inbounds i8, ptr %173, i64 %243
  %245 = sub i64 %229, %204
  %246 = call i64 @llvm.umin.i64(i64 %241, i64 %242)
  %247 = sub i64 0, %246
  %248 = getelementptr inbounds i8, ptr %173, i64 %247
  call void %33(ptr noundef %203, ptr noundef %248, i64 noundef %246) #22
  %249 = icmp ugt i64 %235, %245
  br i1 %249, label %251, label %250

250:                                              ; preds = %227
  br label %251

251:                                              ; preds = %227, %250
  %252 = phi ptr [ %244, %250 ], [ %69, %227 ]
  %253 = phi i64 [ %245, %250 ], [ %235, %227 ]
  %254 = phi ptr [ %69, %250 ], [ %244, %227 ]
  %255 = phi i64 [ %235, %250 ], [ %245, %227 ]
  store ptr %252, ptr %67, align 8, !tbaa !56
  %256 = getelementptr inbounds nuw i8, ptr %67, i64 8
  store i64 %253, ptr %256, align 8, !tbaa !58
  %257 = getelementptr inbounds nuw i8, ptr %67, i64 16
  store i32 %70, ptr %257, align 8, !tbaa !59
  %258 = getelementptr inbounds nuw i8, ptr %67, i64 24
  %259 = icmp ugt i64 %255, 6
  br i1 %259, label %65, label %260, !llvm.loop !67

260:                                              ; preds = %251, %144, %54, %87
  %261 = phi ptr [ %69, %87 ], [ %57, %54 ], [ %69, %144 ], [ %254, %251 ]
  %262 = phi ptr [ %67, %87 ], [ %56, %54 ], [ %67, %144 ], [ %258, %251 ]
  %263 = phi i64 [ 0, %87 ], [ %59, %54 ], [ 0, %144 ], [ %255, %251 ]
  %264 = mul i64 %263, %2
  %265 = getelementptr inbounds nuw i8, ptr %261, i64 %264
  %266 = icmp samesign ult i64 %2, %264
  br i1 %266, label %267, label %52

267:                                              ; preds = %260
  %268 = getelementptr inbounds nuw i8, ptr %261, i64 %2
  br label %269

269:                                              ; preds = %267, %279
  %270 = phi ptr [ %280, %279 ], [ %268, %267 ]
  %271 = icmp ugt ptr %270, %261
  br i1 %271, label %272, label %279

272:                                              ; preds = %269, %277
  %273 = phi ptr [ %274, %277 ], [ %270, %269 ]
  %274 = getelementptr inbounds i8, ptr %273, i64 %41
  %275 = call i32 %3(ptr noundef nonnull %274, ptr noundef nonnull %273, ptr noundef %4) #22
  %276 = icmp sgt i32 %275, 0
  br i1 %276, label %277, label %279

277:                                              ; preds = %272
  call void %32(ptr noundef nonnull %273, ptr noundef nonnull %274, i64 noundef %2) #22
  %278 = icmp ugt ptr %274, %261
  br i1 %278, label %272, label %279, !llvm.loop !68

279:                                              ; preds = %277, %272, %269
  %280 = getelementptr inbounds nuw i8, ptr %270, i64 %2
  %281 = icmp ult ptr %280, %265
  br i1 %281, label %269, label %52, !llvm.loop !69

282:                                              ; preds = %52, %31
  call void @llvm.lifetime.end.p0(i64 1200, ptr nonnull %6) #22
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @exchange_one_int128(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 %2) unnamed_addr #17 {
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %5 = load i64, ptr %1, align 8, !tbaa !70
  %6 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %7 = load <2 x i64>, ptr %0, align 8, !tbaa !70
  store i64 %5, ptr %0, align 8, !tbaa !70
  %8 = load i64, ptr %6, align 8, !tbaa !70
  store i64 %8, ptr %4, align 8, !tbaa !70
  store <2 x i64> %7, ptr %1, align 8, !tbaa !70
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal void @exchange_int128s(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) unnamed_addr #0 {
  %4 = icmp ult i64 %2, 16
  br i1 %4, label %43, label %5

5:                                                ; preds = %3
  %6 = lshr i64 %2, 4
  %7 = and i64 %2, 16
  %8 = icmp eq i64 %7, 0
  br i1 %8, label %18, label %9

9:                                                ; preds = %5
  %10 = add nsw i64 %6, -1
  %11 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %12 = load i64, ptr %1, align 8, !tbaa !70
  %13 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %14 = load <2 x i64>, ptr %0, align 8, !tbaa !70
  store i64 %12, ptr %0, align 8, !tbaa !70
  %15 = load i64, ptr %13, align 8, !tbaa !70
  store i64 %15, ptr %11, align 8, !tbaa !70
  store <2 x i64> %14, ptr %1, align 8, !tbaa !70
  %16 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %17 = getelementptr inbounds nuw i8, ptr %1, i64 16
  br label %18

18:                                               ; preds = %9, %5
  %19 = phi i64 [ %6, %5 ], [ %10, %9 ]
  %20 = phi ptr [ %0, %5 ], [ %16, %9 ]
  %21 = phi ptr [ %1, %5 ], [ %17, %9 ]
  %22 = icmp eq i64 %6, 1
  br i1 %22, label %43, label %23

23:                                               ; preds = %18, %23
  %24 = phi i64 [ %34, %23 ], [ %19, %18 ]
  %25 = phi ptr [ %40, %23 ], [ %20, %18 ]
  %26 = phi ptr [ %41, %23 ], [ %21, %18 ]
  %27 = getelementptr inbounds nuw i8, ptr %25, i64 8
  %28 = load i64, ptr %26, align 8, !tbaa !70
  %29 = getelementptr inbounds nuw i8, ptr %26, i64 8
  %30 = load <2 x i64>, ptr %25, align 8, !tbaa !70
  store i64 %28, ptr %25, align 8, !tbaa !70
  %31 = load i64, ptr %29, align 8, !tbaa !70
  store i64 %31, ptr %27, align 8, !tbaa !70
  store <2 x i64> %30, ptr %26, align 8, !tbaa !70
  %32 = getelementptr inbounds nuw i8, ptr %25, i64 16
  %33 = getelementptr inbounds nuw i8, ptr %26, i64 16
  %34 = add nsw i64 %24, -2
  %35 = getelementptr inbounds nuw i8, ptr %25, i64 24
  %36 = load i64, ptr %33, align 8, !tbaa !70
  %37 = getelementptr inbounds nuw i8, ptr %26, i64 24
  %38 = load <2 x i64>, ptr %32, align 8, !tbaa !70
  store i64 %36, ptr %32, align 8, !tbaa !70
  %39 = load i64, ptr %37, align 8, !tbaa !70
  store i64 %39, ptr %35, align 8, !tbaa !70
  store <2 x i64> %38, ptr %33, align 8, !tbaa !70
  %40 = getelementptr inbounds nuw i8, ptr %25, i64 32
  %41 = getelementptr inbounds nuw i8, ptr %26, i64 32
  %42 = icmp eq i64 %34, 0
  br i1 %42, label %43, label %23, !llvm.loop !71

43:                                               ; preds = %18, %23, %3
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @exchange_one_int64(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 %2) unnamed_addr #17 {
  %4 = load i64, ptr %0, align 8, !tbaa !70
  %5 = load i64, ptr %1, align 8, !tbaa !70
  store i64 %5, ptr %0, align 8, !tbaa !70
  store i64 %4, ptr %1, align 8, !tbaa !70
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal void @exchange_int64s(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) unnamed_addr #0 {
  %4 = icmp ult i64 %2, 8
  br i1 %4, label %84, label %5

5:                                                ; preds = %3
  %6 = lshr i64 %2, 3
  %7 = icmp ult i64 %2, 64
  br i1 %7, label %38, label %8

8:                                                ; preds = %5
  %9 = and i64 %2, -8
  %10 = getelementptr i8, ptr %0, i64 %9
  %11 = getelementptr i8, ptr %1, i64 %9
  %12 = icmp ult ptr %0, %11
  %13 = icmp ult ptr %1, %10
  %14 = and i1 %12, %13
  br i1 %14, label %38, label %15

15:                                               ; preds = %8
  %16 = and i64 %6, 2305843009213693948
  %17 = and i64 %6, 3
  %18 = shl nuw i64 %16, 3
  %19 = getelementptr i8, ptr %1, i64 %18
  %20 = shl nuw i64 %16, 3
  %21 = getelementptr i8, ptr %0, i64 %20
  br label %22

22:                                               ; preds = %22, %15
  %23 = phi i64 [ 0, %15 ], [ %34, %22 ]
  %24 = shl i64 %23, 3
  %25 = getelementptr i8, ptr %1, i64 %24
  %26 = shl i64 %23, 3
  %27 = getelementptr i8, ptr %0, i64 %26
  %28 = getelementptr i8, ptr %27, i64 16
  %29 = load <2 x i64>, ptr %27, align 8, !tbaa !70, !alias.scope !72, !noalias !75
  %30 = load <2 x i64>, ptr %28, align 8, !tbaa !70, !alias.scope !72, !noalias !75
  %31 = getelementptr i8, ptr %25, i64 16
  %32 = load <2 x i64>, ptr %25, align 8, !tbaa !70, !alias.scope !75
  %33 = load <2 x i64>, ptr %31, align 8, !tbaa !70, !alias.scope !75
  store <2 x i64> %32, ptr %27, align 8, !tbaa !70, !alias.scope !72, !noalias !75
  store <2 x i64> %33, ptr %28, align 8, !tbaa !70, !alias.scope !72, !noalias !75
  store <2 x i64> %29, ptr %25, align 8, !tbaa !70, !alias.scope !75
  store <2 x i64> %30, ptr %31, align 8, !tbaa !70, !alias.scope !75
  %34 = add nuw i64 %23, 4
  %35 = icmp eq i64 %34, %16
  br i1 %35, label %36, label %22, !llvm.loop !77

36:                                               ; preds = %22
  %37 = icmp eq i64 %6, %16
  br i1 %37, label %84, label %38

38:                                               ; preds = %8, %5, %36
  %39 = phi i64 [ %6, %8 ], [ %6, %5 ], [ %17, %36 ]
  %40 = phi ptr [ %1, %8 ], [ %1, %5 ], [ %19, %36 ]
  %41 = phi ptr [ %0, %8 ], [ %0, %5 ], [ %21, %36 ]
  %42 = add nsw i64 %39, -1
  %43 = and i64 %39, 3
  %44 = icmp eq i64 %43, 0
  br i1 %44, label %57, label %45

45:                                               ; preds = %38, %45
  %46 = phi i64 [ %50, %45 ], [ %39, %38 ]
  %47 = phi ptr [ %54, %45 ], [ %40, %38 ]
  %48 = phi ptr [ %53, %45 ], [ %41, %38 ]
  %49 = phi i64 [ %55, %45 ], [ 0, %38 ]
  %50 = add nsw i64 %46, -1
  %51 = load i64, ptr %48, align 8, !tbaa !70
  %52 = load i64, ptr %47, align 8, !tbaa !70
  %53 = getelementptr inbounds nuw i8, ptr %48, i64 8
  store i64 %52, ptr %48, align 8, !tbaa !70
  %54 = getelementptr inbounds nuw i8, ptr %47, i64 8
  store i64 %51, ptr %47, align 8, !tbaa !70
  %55 = add i64 %49, 1
  %56 = icmp eq i64 %55, %43
  br i1 %56, label %57, label %45, !llvm.loop !78

57:                                               ; preds = %45, %38
  %58 = phi i64 [ %39, %38 ], [ %50, %45 ]
  %59 = phi ptr [ %40, %38 ], [ %54, %45 ]
  %60 = phi ptr [ %41, %38 ], [ %53, %45 ]
  %61 = icmp ult i64 %42, 3
  br i1 %61, label %84, label %62

62:                                               ; preds = %57, %62
  %63 = phi i64 [ %78, %62 ], [ %58, %57 ]
  %64 = phi ptr [ %82, %62 ], [ %59, %57 ]
  %65 = phi ptr [ %81, %62 ], [ %60, %57 ]
  %66 = load i64, ptr %65, align 8, !tbaa !70
  %67 = load i64, ptr %64, align 8, !tbaa !70
  %68 = getelementptr inbounds nuw i8, ptr %65, i64 8
  store i64 %67, ptr %65, align 8, !tbaa !70
  %69 = getelementptr inbounds nuw i8, ptr %64, i64 8
  store i64 %66, ptr %64, align 8, !tbaa !70
  %70 = load i64, ptr %68, align 8, !tbaa !70
  %71 = load i64, ptr %69, align 8, !tbaa !70
  %72 = getelementptr inbounds nuw i8, ptr %65, i64 16
  store i64 %71, ptr %68, align 8, !tbaa !70
  %73 = getelementptr inbounds nuw i8, ptr %64, i64 16
  store i64 %70, ptr %69, align 8, !tbaa !70
  %74 = load i64, ptr %72, align 8, !tbaa !70
  %75 = load i64, ptr %73, align 8, !tbaa !70
  %76 = getelementptr inbounds nuw i8, ptr %65, i64 24
  store i64 %75, ptr %72, align 8, !tbaa !70
  %77 = getelementptr inbounds nuw i8, ptr %64, i64 24
  store i64 %74, ptr %73, align 8, !tbaa !70
  %78 = add nsw i64 %63, -4
  %79 = load i64, ptr %76, align 8, !tbaa !70
  %80 = load i64, ptr %77, align 8, !tbaa !70
  %81 = getelementptr inbounds nuw i8, ptr %65, i64 32
  store i64 %80, ptr %76, align 8, !tbaa !70
  %82 = getelementptr inbounds nuw i8, ptr %64, i64 32
  store i64 %79, ptr %77, align 8, !tbaa !70
  %83 = icmp eq i64 %78, 0
  br i1 %83, label %84, label %62, !llvm.loop !79

84:                                               ; preds = %57, %62, %36, %3
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @exchange_one_int32(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 %2) unnamed_addr #17 {
  %4 = load i32, ptr %0, align 4, !tbaa !12
  %5 = load i32, ptr %1, align 4, !tbaa !12
  store i32 %5, ptr %0, align 4, !tbaa !12
  store i32 %4, ptr %1, align 4, !tbaa !12
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal void @exchange_int32s(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) unnamed_addr #0 {
  %4 = icmp ult i64 %2, 4
  br i1 %4, label %84, label %5

5:                                                ; preds = %3
  %6 = lshr i64 %2, 2
  %7 = icmp ult i64 %2, 32
  br i1 %7, label %38, label %8

8:                                                ; preds = %5
  %9 = and i64 %2, -4
  %10 = getelementptr i8, ptr %0, i64 %9
  %11 = getelementptr i8, ptr %1, i64 %9
  %12 = icmp ult ptr %0, %11
  %13 = icmp ult ptr %1, %10
  %14 = and i1 %12, %13
  br i1 %14, label %38, label %15

15:                                               ; preds = %8
  %16 = and i64 %6, 4611686018427387896
  %17 = and i64 %6, 7
  %18 = shl nuw i64 %16, 2
  %19 = getelementptr i8, ptr %1, i64 %18
  %20 = shl nuw i64 %16, 2
  %21 = getelementptr i8, ptr %0, i64 %20
  br label %22

22:                                               ; preds = %22, %15
  %23 = phi i64 [ 0, %15 ], [ %34, %22 ]
  %24 = shl i64 %23, 2
  %25 = getelementptr i8, ptr %1, i64 %24
  %26 = shl i64 %23, 2
  %27 = getelementptr i8, ptr %0, i64 %26
  %28 = getelementptr i8, ptr %27, i64 16
  %29 = load <4 x i32>, ptr %27, align 4, !tbaa !12, !alias.scope !80, !noalias !83
  %30 = load <4 x i32>, ptr %28, align 4, !tbaa !12, !alias.scope !80, !noalias !83
  %31 = getelementptr i8, ptr %25, i64 16
  %32 = load <4 x i32>, ptr %25, align 4, !tbaa !12, !alias.scope !83
  %33 = load <4 x i32>, ptr %31, align 4, !tbaa !12, !alias.scope !83
  store <4 x i32> %32, ptr %27, align 4, !tbaa !12, !alias.scope !80, !noalias !83
  store <4 x i32> %33, ptr %28, align 4, !tbaa !12, !alias.scope !80, !noalias !83
  store <4 x i32> %29, ptr %25, align 4, !tbaa !12, !alias.scope !83
  store <4 x i32> %30, ptr %31, align 4, !tbaa !12, !alias.scope !83
  %34 = add nuw i64 %23, 8
  %35 = icmp eq i64 %34, %16
  br i1 %35, label %36, label %22, !llvm.loop !85

36:                                               ; preds = %22
  %37 = icmp eq i64 %6, %16
  br i1 %37, label %84, label %38

38:                                               ; preds = %8, %5, %36
  %39 = phi i64 [ %6, %8 ], [ %6, %5 ], [ %17, %36 ]
  %40 = phi ptr [ %1, %8 ], [ %1, %5 ], [ %19, %36 ]
  %41 = phi ptr [ %0, %8 ], [ %0, %5 ], [ %21, %36 ]
  %42 = add nsw i64 %39, -1
  %43 = and i64 %39, 3
  %44 = icmp eq i64 %43, 0
  br i1 %44, label %57, label %45

45:                                               ; preds = %38, %45
  %46 = phi i64 [ %50, %45 ], [ %39, %38 ]
  %47 = phi ptr [ %54, %45 ], [ %40, %38 ]
  %48 = phi ptr [ %53, %45 ], [ %41, %38 ]
  %49 = phi i64 [ %55, %45 ], [ 0, %38 ]
  %50 = add nsw i64 %46, -1
  %51 = load i32, ptr %48, align 4, !tbaa !12
  %52 = load i32, ptr %47, align 4, !tbaa !12
  %53 = getelementptr inbounds nuw i8, ptr %48, i64 4
  store i32 %52, ptr %48, align 4, !tbaa !12
  %54 = getelementptr inbounds nuw i8, ptr %47, i64 4
  store i32 %51, ptr %47, align 4, !tbaa !12
  %55 = add i64 %49, 1
  %56 = icmp eq i64 %55, %43
  br i1 %56, label %57, label %45, !llvm.loop !86

57:                                               ; preds = %45, %38
  %58 = phi i64 [ %39, %38 ], [ %50, %45 ]
  %59 = phi ptr [ %40, %38 ], [ %54, %45 ]
  %60 = phi ptr [ %41, %38 ], [ %53, %45 ]
  %61 = icmp ult i64 %42, 3
  br i1 %61, label %84, label %62

62:                                               ; preds = %57, %62
  %63 = phi i64 [ %78, %62 ], [ %58, %57 ]
  %64 = phi ptr [ %82, %62 ], [ %59, %57 ]
  %65 = phi ptr [ %81, %62 ], [ %60, %57 ]
  %66 = load i32, ptr %65, align 4, !tbaa !12
  %67 = load i32, ptr %64, align 4, !tbaa !12
  %68 = getelementptr inbounds nuw i8, ptr %65, i64 4
  store i32 %67, ptr %65, align 4, !tbaa !12
  %69 = getelementptr inbounds nuw i8, ptr %64, i64 4
  store i32 %66, ptr %64, align 4, !tbaa !12
  %70 = load i32, ptr %68, align 4, !tbaa !12
  %71 = load i32, ptr %69, align 4, !tbaa !12
  %72 = getelementptr inbounds nuw i8, ptr %65, i64 8
  store i32 %71, ptr %68, align 4, !tbaa !12
  %73 = getelementptr inbounds nuw i8, ptr %64, i64 8
  store i32 %70, ptr %69, align 4, !tbaa !12
  %74 = load i32, ptr %72, align 4, !tbaa !12
  %75 = load i32, ptr %73, align 4, !tbaa !12
  %76 = getelementptr inbounds nuw i8, ptr %65, i64 12
  store i32 %75, ptr %72, align 4, !tbaa !12
  %77 = getelementptr inbounds nuw i8, ptr %64, i64 12
  store i32 %74, ptr %73, align 4, !tbaa !12
  %78 = add nsw i64 %63, -4
  %79 = load i32, ptr %76, align 4, !tbaa !12
  %80 = load i32, ptr %77, align 4, !tbaa !12
  %81 = getelementptr inbounds nuw i8, ptr %65, i64 16
  store i32 %80, ptr %76, align 4, !tbaa !12
  %82 = getelementptr inbounds nuw i8, ptr %64, i64 16
  store i32 %79, ptr %77, align 4, !tbaa !12
  %83 = icmp eq i64 %78, 0
  br i1 %83, label %84, label %62, !llvm.loop !87

84:                                               ; preds = %57, %62, %36, %3
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @exchange_one_int16(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 %2) unnamed_addr #17 {
  %4 = load i16, ptr %0, align 2, !tbaa !41
  %5 = load i16, ptr %1, align 2, !tbaa !41
  store i16 %5, ptr %0, align 2, !tbaa !41
  store i16 %4, ptr %1, align 2, !tbaa !41
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal void @exchange_int16s(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) unnamed_addr #0 {
  %4 = icmp ult i64 %2, 2
  br i1 %4, label %109, label %5

5:                                                ; preds = %3
  %6 = lshr i64 %2, 1
  %7 = icmp ult i64 %2, 8
  br i1 %7, label %63, label %8

8:                                                ; preds = %5
  %9 = and i64 %2, -2
  %10 = getelementptr i8, ptr %0, i64 %9
  %11 = getelementptr i8, ptr %1, i64 %9
  %12 = icmp ult ptr %0, %11
  %13 = icmp ult ptr %1, %10
  %14 = and i1 %12, %13
  br i1 %14, label %63, label %15

15:                                               ; preds = %8
  %16 = icmp ult i64 %2, 32
  br i1 %16, label %43, label %17

17:                                               ; preds = %15
  %18 = and i64 %6, 9223372036854775792
  br label %19

19:                                               ; preds = %19, %17
  %20 = phi i64 [ 0, %17 ], [ %31, %19 ]
  %21 = shl i64 %20, 1
  %22 = getelementptr i8, ptr %1, i64 %21
  %23 = shl i64 %20, 1
  %24 = getelementptr i8, ptr %0, i64 %23
  %25 = getelementptr i8, ptr %24, i64 16
  %26 = load <8 x i16>, ptr %24, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  %27 = load <8 x i16>, ptr %25, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  %28 = getelementptr i8, ptr %22, i64 16
  %29 = load <8 x i16>, ptr %22, align 2, !tbaa !41, !alias.scope !91
  %30 = load <8 x i16>, ptr %28, align 2, !tbaa !41, !alias.scope !91
  store <8 x i16> %29, ptr %24, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  store <8 x i16> %30, ptr %25, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  store <8 x i16> %26, ptr %22, align 2, !tbaa !41, !alias.scope !91
  store <8 x i16> %27, ptr %28, align 2, !tbaa !41, !alias.scope !91
  %31 = add nuw i64 %20, 16
  %32 = icmp eq i64 %31, %18
  br i1 %32, label %33, label %19, !llvm.loop !93

33:                                               ; preds = %19
  %34 = icmp eq i64 %6, %18
  br i1 %34, label %109, label %35

35:                                               ; preds = %33
  %36 = and i64 %6, 15
  %37 = shl nuw i64 %18, 1
  %38 = getelementptr i8, ptr %1, i64 %37
  %39 = shl nuw i64 %18, 1
  %40 = getelementptr i8, ptr %0, i64 %39
  %41 = and i64 %2, 24
  %42 = icmp eq i64 %41, 0
  br i1 %42, label %63, label %43

43:                                               ; preds = %35, %15
  %44 = phi i64 [ %18, %35 ], [ 0, %15 ]
  %45 = and i64 %6, 9223372036854775804
  %46 = and i64 %6, 3
  %47 = shl nuw i64 %45, 1
  %48 = getelementptr i8, ptr %1, i64 %47
  %49 = shl nuw i64 %45, 1
  %50 = getelementptr i8, ptr %0, i64 %49
  br label %51

51:                                               ; preds = %51, %43
  %52 = phi i64 [ %44, %43 ], [ %59, %51 ]
  %53 = shl i64 %52, 1
  %54 = getelementptr i8, ptr %1, i64 %53
  %55 = shl i64 %52, 1
  %56 = getelementptr i8, ptr %0, i64 %55
  %57 = load <4 x i16>, ptr %56, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  %58 = load <4 x i16>, ptr %54, align 2, !tbaa !41, !alias.scope !91
  store <4 x i16> %58, ptr %56, align 2, !tbaa !41, !alias.scope !88, !noalias !91
  store <4 x i16> %57, ptr %54, align 2, !tbaa !41, !alias.scope !91
  %59 = add nuw i64 %52, 4
  %60 = icmp eq i64 %59, %45
  br i1 %60, label %61, label %51, !llvm.loop !94

61:                                               ; preds = %51
  %62 = icmp eq i64 %6, %45
  br i1 %62, label %109, label %63

63:                                               ; preds = %35, %61, %8, %5
  %64 = phi i64 [ %6, %5 ], [ %6, %8 ], [ %36, %35 ], [ %46, %61 ]
  %65 = phi ptr [ %1, %5 ], [ %1, %8 ], [ %38, %35 ], [ %48, %61 ]
  %66 = phi ptr [ %0, %5 ], [ %0, %8 ], [ %40, %35 ], [ %50, %61 ]
  %67 = add nsw i64 %64, -1
  %68 = and i64 %64, 3
  %69 = icmp eq i64 %68, 0
  br i1 %69, label %82, label %70

70:                                               ; preds = %63, %70
  %71 = phi i64 [ %75, %70 ], [ %64, %63 ]
  %72 = phi ptr [ %79, %70 ], [ %65, %63 ]
  %73 = phi ptr [ %78, %70 ], [ %66, %63 ]
  %74 = phi i64 [ %80, %70 ], [ 0, %63 ]
  %75 = add nsw i64 %71, -1
  %76 = load i16, ptr %73, align 2, !tbaa !41
  %77 = load i16, ptr %72, align 2, !tbaa !41
  %78 = getelementptr inbounds nuw i8, ptr %73, i64 2
  store i16 %77, ptr %73, align 2, !tbaa !41
  %79 = getelementptr inbounds nuw i8, ptr %72, i64 2
  store i16 %76, ptr %72, align 2, !tbaa !41
  %80 = add i64 %74, 1
  %81 = icmp eq i64 %80, %68
  br i1 %81, label %82, label %70, !llvm.loop !95

82:                                               ; preds = %70, %63
  %83 = phi i64 [ %64, %63 ], [ %75, %70 ]
  %84 = phi ptr [ %65, %63 ], [ %79, %70 ]
  %85 = phi ptr [ %66, %63 ], [ %78, %70 ]
  %86 = icmp ult i64 %67, 3
  br i1 %86, label %109, label %87

87:                                               ; preds = %82, %87
  %88 = phi i64 [ %103, %87 ], [ %83, %82 ]
  %89 = phi ptr [ %107, %87 ], [ %84, %82 ]
  %90 = phi ptr [ %106, %87 ], [ %85, %82 ]
  %91 = load i16, ptr %90, align 2, !tbaa !41
  %92 = load i16, ptr %89, align 2, !tbaa !41
  %93 = getelementptr inbounds nuw i8, ptr %90, i64 2
  store i16 %92, ptr %90, align 2, !tbaa !41
  %94 = getelementptr inbounds nuw i8, ptr %89, i64 2
  store i16 %91, ptr %89, align 2, !tbaa !41
  %95 = load i16, ptr %93, align 2, !tbaa !41
  %96 = load i16, ptr %94, align 2, !tbaa !41
  %97 = getelementptr inbounds nuw i8, ptr %90, i64 4
  store i16 %96, ptr %93, align 2, !tbaa !41
  %98 = getelementptr inbounds nuw i8, ptr %89, i64 4
  store i16 %95, ptr %94, align 2, !tbaa !41
  %99 = load i16, ptr %97, align 2, !tbaa !41
  %100 = load i16, ptr %98, align 2, !tbaa !41
  %101 = getelementptr inbounds nuw i8, ptr %90, i64 6
  store i16 %100, ptr %97, align 2, !tbaa !41
  %102 = getelementptr inbounds nuw i8, ptr %89, i64 6
  store i16 %99, ptr %98, align 2, !tbaa !41
  %103 = add nsw i64 %88, -4
  %104 = load i16, ptr %101, align 2, !tbaa !41
  %105 = load i16, ptr %102, align 2, !tbaa !41
  %106 = getelementptr inbounds nuw i8, ptr %90, i64 8
  store i16 %105, ptr %101, align 2, !tbaa !41
  %107 = getelementptr inbounds nuw i8, ptr %89, i64 8
  store i16 %104, ptr %102, align 2, !tbaa !41
  %108 = icmp eq i64 %103, 0
  br i1 %108, label %109, label %87, !llvm.loop !96

109:                                              ; preds = %82, %87, %33, %61, %3
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @exchange_one_byte(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 %2) unnamed_addr #17 {
  %4 = load i8, ptr %0, align 1, !tbaa !5
  %5 = load i8, ptr %1, align 1, !tbaa !5
  store i8 %5, ptr %0, align 1, !tbaa !5
  store i8 %4, ptr %1, align 1, !tbaa !5
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define internal void @exchange_bytes(ptr noundef captures(none) %0, ptr noundef captures(none) %1, i64 noundef %2) unnamed_addr #0 {
  %4 = icmp eq i64 %2, 0
  br i1 %4, label %99, label %5

5:                                                ; preds = %3
  %6 = icmp ult i64 %2, 4
  br i1 %6, label %53, label %7

7:                                                ; preds = %5
  %8 = getelementptr i8, ptr %0, i64 %2
  %9 = getelementptr i8, ptr %1, i64 %2
  %10 = icmp ult ptr %0, %9
  %11 = icmp ult ptr %1, %8
  %12 = and i1 %10, %11
  br i1 %12, label %53, label %13

13:                                               ; preds = %7
  %14 = icmp ult i64 %2, 32
  br i1 %14, label %37, label %15

15:                                               ; preds = %13
  %16 = and i64 %2, -32
  br label %17

17:                                               ; preds = %17, %15
  %18 = phi i64 [ 0, %15 ], [ %27, %17 ]
  %19 = getelementptr i8, ptr %1, i64 %18
  %20 = getelementptr i8, ptr %0, i64 %18
  %21 = getelementptr i8, ptr %20, i64 16
  %22 = load <16 x i8>, ptr %20, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  %23 = load <16 x i8>, ptr %21, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  %24 = getelementptr i8, ptr %19, i64 16
  %25 = load <16 x i8>, ptr %19, align 1, !tbaa !5, !alias.scope !100
  %26 = load <16 x i8>, ptr %24, align 1, !tbaa !5, !alias.scope !100
  store <16 x i8> %25, ptr %20, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  store <16 x i8> %26, ptr %21, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  store <16 x i8> %22, ptr %19, align 1, !tbaa !5, !alias.scope !100
  store <16 x i8> %23, ptr %24, align 1, !tbaa !5, !alias.scope !100
  %27 = add nuw i64 %18, 32
  %28 = icmp eq i64 %27, %16
  br i1 %28, label %29, label %17, !llvm.loop !102

29:                                               ; preds = %17
  %30 = icmp eq i64 %2, %16
  br i1 %30, label %99, label %31

31:                                               ; preds = %29
  %32 = and i64 %2, 31
  %33 = getelementptr i8, ptr %1, i64 %16
  %34 = getelementptr i8, ptr %0, i64 %16
  %35 = and i64 %2, 28
  %36 = icmp eq i64 %35, 0
  br i1 %36, label %53, label %37

37:                                               ; preds = %31, %13
  %38 = phi i64 [ %16, %31 ], [ 0, %13 ]
  %39 = and i64 %2, -4
  %40 = and i64 %2, 3
  %41 = getelementptr i8, ptr %1, i64 %39
  %42 = getelementptr i8, ptr %0, i64 %39
  br label %43

43:                                               ; preds = %43, %37
  %44 = phi i64 [ %38, %37 ], [ %49, %43 ]
  %45 = getelementptr i8, ptr %1, i64 %44
  %46 = getelementptr i8, ptr %0, i64 %44
  %47 = load <4 x i8>, ptr %46, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  %48 = load <4 x i8>, ptr %45, align 1, !tbaa !5, !alias.scope !100
  store <4 x i8> %48, ptr %46, align 1, !tbaa !5, !alias.scope !97, !noalias !100
  store <4 x i8> %47, ptr %45, align 1, !tbaa !5, !alias.scope !100
  %49 = add nuw i64 %44, 4
  %50 = icmp eq i64 %49, %39
  br i1 %50, label %51, label %43, !llvm.loop !103

51:                                               ; preds = %43
  %52 = icmp eq i64 %2, %39
  br i1 %52, label %99, label %53

53:                                               ; preds = %31, %51, %7, %5
  %54 = phi i64 [ %2, %5 ], [ %2, %7 ], [ %32, %31 ], [ %40, %51 ]
  %55 = phi ptr [ %1, %5 ], [ %1, %7 ], [ %33, %31 ], [ %41, %51 ]
  %56 = phi ptr [ %0, %5 ], [ %0, %7 ], [ %34, %31 ], [ %42, %51 ]
  %57 = add i64 %54, -1
  %58 = and i64 %54, 3
  %59 = icmp eq i64 %58, 0
  br i1 %59, label %72, label %60

60:                                               ; preds = %53, %60
  %61 = phi i64 [ %65, %60 ], [ %54, %53 ]
  %62 = phi ptr [ %69, %60 ], [ %55, %53 ]
  %63 = phi ptr [ %68, %60 ], [ %56, %53 ]
  %64 = phi i64 [ %70, %60 ], [ 0, %53 ]
  %65 = add i64 %61, -1
  %66 = load i8, ptr %63, align 1, !tbaa !5
  %67 = load i8, ptr %62, align 1, !tbaa !5
  %68 = getelementptr inbounds nuw i8, ptr %63, i64 1
  store i8 %67, ptr %63, align 1, !tbaa !5
  %69 = getelementptr inbounds nuw i8, ptr %62, i64 1
  store i8 %66, ptr %62, align 1, !tbaa !5
  %70 = add i64 %64, 1
  %71 = icmp eq i64 %70, %58
  br i1 %71, label %72, label %60, !llvm.loop !104

72:                                               ; preds = %60, %53
  %73 = phi i64 [ %54, %53 ], [ %65, %60 ]
  %74 = phi ptr [ %55, %53 ], [ %69, %60 ]
  %75 = phi ptr [ %56, %53 ], [ %68, %60 ]
  %76 = icmp ult i64 %57, 3
  br i1 %76, label %99, label %77

77:                                               ; preds = %72, %77
  %78 = phi i64 [ %93, %77 ], [ %73, %72 ]
  %79 = phi ptr [ %97, %77 ], [ %74, %72 ]
  %80 = phi ptr [ %96, %77 ], [ %75, %72 ]
  %81 = load i8, ptr %80, align 1, !tbaa !5
  %82 = load i8, ptr %79, align 1, !tbaa !5
  %83 = getelementptr inbounds nuw i8, ptr %80, i64 1
  store i8 %82, ptr %80, align 1, !tbaa !5
  %84 = getelementptr inbounds nuw i8, ptr %79, i64 1
  store i8 %81, ptr %79, align 1, !tbaa !5
  %85 = load i8, ptr %83, align 1, !tbaa !5
  %86 = load i8, ptr %84, align 1, !tbaa !5
  %87 = getelementptr inbounds nuw i8, ptr %80, i64 2
  store i8 %86, ptr %83, align 1, !tbaa !5
  %88 = getelementptr inbounds nuw i8, ptr %79, i64 2
  store i8 %85, ptr %84, align 1, !tbaa !5
  %89 = load i8, ptr %87, align 1, !tbaa !5
  %90 = load i8, ptr %88, align 1, !tbaa !5
  %91 = getelementptr inbounds nuw i8, ptr %80, i64 3
  store i8 %90, ptr %87, align 1, !tbaa !5
  %92 = getelementptr inbounds nuw i8, ptr %79, i64 3
  store i8 %89, ptr %88, align 1, !tbaa !5
  %93 = add i64 %78, -4
  %94 = load i8, ptr %91, align 1, !tbaa !5
  %95 = load i8, ptr %92, align 1, !tbaa !5
  %96 = getelementptr inbounds nuw i8, ptr %80, i64 4
  store i8 %95, ptr %91, align 1, !tbaa !5
  %97 = getelementptr inbounds nuw i8, ptr %79, i64 4
  store i8 %94, ptr %92, align 1, !tbaa !5
  %98 = icmp eq i64 %93, 0
  br i1 %98, label %99, label %77, !llvm.loop !105

99:                                               ; preds = %72, %77, %29, %51, %3
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #3

; Function Attrs: nounwind uwtable
define dso_local double @xyo_atod(ptr noundef readonly captures(address_is_null) %0) local_unnamed_addr #5 {
  %2 = alloca ptr, align 8
  %3 = alloca %struct.JSATODTempMem, align 8
  %4 = icmp eq ptr %0, null
  br i1 %4, label %66, label %5

5:                                                ; preds = %1
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %7 = load ptr, ptr %6, align 8, !tbaa !106
  %8 = icmp eq ptr %7, null
  br i1 %8, label %66, label %9

9:                                                ; preds = %5
  %10 = load i64, ptr %0, align 8, !tbaa !109
  %11 = icmp eq i64 %10, 0
  br i1 %11, label %31, label %12

12:                                               ; preds = %9, %17
  %13 = phi i64 [ %18, %17 ], [ 0, %9 ]
  %14 = getelementptr inbounds nuw i16, ptr %7, i64 %13
  %15 = load i16, ptr %14, align 2, !tbaa !41
  %16 = tail call fastcc zeroext i1 @is_ecma_ws_or_lt_u16(i16 noundef zeroext %15)
  br i1 %16, label %17, label %20

17:                                               ; preds = %12
  %18 = add nuw i64 %13, 1
  %19 = icmp eq i64 %18, %10
  br i1 %19, label %66, label %12, !llvm.loop !110

20:                                               ; preds = %12
  %21 = getelementptr i8, ptr %7, i64 -2
  %22 = icmp ugt i64 %10, %13
  br i1 %22, label %23, label %31

23:                                               ; preds = %20, %28
  %24 = phi i64 [ %29, %28 ], [ %10, %20 ]
  %25 = getelementptr i16, ptr %21, i64 %24
  %26 = load i16, ptr %25, align 2, !tbaa !41
  %27 = tail call fastcc zeroext i1 @is_ecma_ws_or_lt_u16(i16 noundef zeroext %26)
  br i1 %27, label %28, label %31

28:                                               ; preds = %23
  %29 = add i64 %24, -1
  %30 = icmp ugt i64 %29, %13
  br i1 %30, label %23, label %66, !llvm.loop !111

31:                                               ; preds = %23, %9, %20
  %32 = phi i64 [ %13, %20 ], [ 0, %9 ], [ %13, %23 ]
  %33 = phi i64 [ %10, %20 ], [ 0, %9 ], [ %24, %23 ]
  %34 = icmp eq i64 %32, %33
  br i1 %34, label %66, label %35

35:                                               ; preds = %31
  %36 = sub i64 %33, %32
  %37 = add i64 %36, 1
  %38 = tail call ptr @llvm.stacksave.p0()
  %39 = alloca i8, i64 %37, align 16
  %40 = load ptr, ptr %6, align 8, !tbaa !106
  %41 = getelementptr i16, ptr %40, i64 %32
  br label %42

42:                                               ; preds = %35, %48
  %43 = phi i64 [ 0, %35 ], [ %51, %48 ]
  %44 = getelementptr i16, ptr %41, i64 %43
  %45 = load i16, ptr %44, align 2, !tbaa !41
  switch i16 %45, label %46 [
    i16 110, label %64
    i16 95, label %64
  ]

46:                                               ; preds = %42
  %47 = icmp ugt i16 %45, 127
  br i1 %47, label %64, label %48

48:                                               ; preds = %46
  %49 = trunc nuw nsw i16 %45 to i8
  %50 = getelementptr inbounds nuw i8, ptr %39, i64 %43
  store i8 %49, ptr %50, align 1, !tbaa !5
  %51 = add nuw i64 %43, 1
  %52 = icmp eq i64 %51, %36
  br i1 %52, label %53, label %42, !llvm.loop !112

53:                                               ; preds = %48
  %54 = getelementptr inbounds nuw i8, ptr %39, i64 %36
  store i8 0, ptr %54, align 1, !tbaa !5
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %2) #22
  store ptr null, ptr %2, align 8, !tbaa !43
  call void @llvm.lifetime.start.p0(i64 216, ptr nonnull %3) #22
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(216) %3, i8 0, i64 216, i1 false)
  %55 = call double @js_atod(ptr noundef nonnull %39, ptr noundef nonnull %2, i32 noundef 0, i32 noundef 2, ptr noundef nonnull %3)
  %56 = load ptr, ptr %2, align 8, !tbaa !43
  %57 = icmp eq ptr %56, null
  br i1 %57, label %62, label %58

58:                                               ; preds = %53
  %59 = load i8, ptr %56, align 1, !tbaa !5
  %60 = icmp eq i8 %59, 0
  br i1 %60, label %61, label %62

61:                                               ; preds = %58
  br label %62

62:                                               ; preds = %53, %58, %61
  %63 = phi double [ %55, %61 ], [ 0x7FF8000000000000, %58 ], [ 0x7FF8000000000000, %53 ]
  call void @llvm.lifetime.end.p0(i64 216, ptr nonnull %3) #22
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %2) #22
  br label %64

64:                                               ; preds = %46, %42, %42, %62
  %65 = phi double [ %63, %62 ], [ 0x7FF8000000000000, %42 ], [ 0x7FF8000000000000, %42 ], [ 0x7FF8000000000000, %46 ]
  call void @llvm.stackrestore.p0(ptr %38)
  br label %66

66:                                               ; preds = %17, %28, %64, %31, %1, %5
  %67 = phi double [ 0x7FF8000000000000, %5 ], [ 0x7FF8000000000000, %1 ], [ %65, %64 ], [ 0.000000e+00, %31 ], [ 0.000000e+00, %28 ], [ 0.000000e+00, %17 ]
  ret double %67
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal fastcc noundef zeroext i1 @is_ecma_ws_or_lt_u16(i16 noundef zeroext %0) unnamed_addr #4 {
  switch i16 %0, label %2 [
    i16 9, label %3
    i16 10, label %3
    i16 11, label %3
    i16 12, label %3
    i16 13, label %3
    i16 32, label %3
    i16 160, label %3
    i16 5760, label %3
    i16 8192, label %3
    i16 8193, label %3
    i16 8194, label %3
    i16 8195, label %3
    i16 8196, label %3
    i16 8197, label %3
    i16 8198, label %3
    i16 8199, label %3
    i16 8200, label %3
    i16 8201, label %3
    i16 8202, label %3
    i16 8232, label %3
    i16 8233, label %3
    i16 8239, label %3
    i16 8287, label %3
    i16 12288, label %3
    i16 -257, label %3
  ]

2:                                                ; preds = %1
  br label %3

3:                                                ; preds = %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %1, %2
  %4 = phi i1 [ false, %2 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ], [ true, %1 ]
  ret i1 %4
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare ptr @llvm.stacksave.p0() #15

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.stackrestore.p0(ptr) #15

; Function Attrs: nounwind uwtable
define dso_local noundef ptr @xyo_dtoa(double noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4) local_unnamed_addr #5 {
  %6 = alloca [1024 x i8], align 16
  %7 = alloca %struct.JSDTOATempMem, align 8
  call void @llvm.lifetime.start.p0(i64 1024, ptr nonnull %6) #22
  call void @llvm.lifetime.start.p0(i64 296, ptr nonnull %7) #22
  %8 = call i32 @js_dtoa(ptr noundef nonnull %6, double noundef %0, i32 noundef 10, i32 noundef 0, i32 noundef 0, ptr noundef nonnull %7)
  %9 = sext i32 %8 to i64
  %10 = shl nsw i64 %9, 1
  %11 = add nsw i64 %10, 32
  %12 = call noalias ptr @malloc(i64 noundef %11) #26
  store i64 %9, ptr %12, align 8, !tbaa !109
  %13 = getelementptr inbounds nuw i8, ptr %12, i64 32
  %14 = getelementptr inbounds nuw i8, ptr %12, i64 8
  store ptr %13, ptr %14, align 8, !tbaa !106
  %15 = icmp eq i32 %8, 0
  br i1 %15, label %24, label %16

16:                                               ; preds = %5
  %17 = zext i64 %2 to i128
  %18 = zext i64 %1 to i128
  %19 = zext i64 %4 to i128
  %20 = zext i64 %3 to i128
  br label %29

21:                                               ; preds = %29
  %22 = trunc nuw i128 %40 to i64
  %23 = trunc nuw i128 %43 to i64
  br label %24

24:                                               ; preds = %21, %5
  %25 = phi i64 [ 0, %5 ], [ %23, %21 ]
  %26 = phi i64 [ 0, %5 ], [ %22, %21 ]
  %27 = getelementptr inbounds nuw i8, ptr %12, i64 16
  store i64 %26, ptr %27, align 8, !tbaa !113
  %28 = getelementptr inbounds nuw i8, ptr %12, i64 24
  store i64 %25, ptr %28, align 8, !tbaa !114
  call void @llvm.lifetime.end.p0(i64 296, ptr nonnull %7) #22
  call void @llvm.lifetime.end.p0(i64 1024, ptr nonnull %6) #22
  ret ptr %12

29:                                               ; preds = %16, %29
  %30 = phi i128 [ 0, %16 ], [ %40, %29 ]
  %31 = phi i64 [ 0, %16 ], [ %44, %29 ]
  %32 = phi i128 [ 0, %16 ], [ %43, %29 ]
  %33 = getelementptr inbounds nuw [1024 x i8], ptr %6, i64 0, i64 %31
  %34 = load i8, ptr %33, align 1, !tbaa !5
  %35 = sext i8 %34 to i16
  %36 = getelementptr inbounds nuw i16, ptr %13, i64 %31
  store i16 %35, ptr %36, align 2, !tbaa !41
  %37 = mul nuw i128 %30, %17
  %38 = zext i16 %35 to i128
  %39 = add nuw i128 %37, %38
  %40 = urem i128 %39, %18
  %41 = mul nuw i128 %32, %19
  %42 = add nuw i128 %41, %38
  %43 = urem i128 %42, %20
  %44 = add nuw i64 %31, 1
  %45 = icmp eq i64 %44, %9
  br i1 %45, label %21, label %29, !llvm.loop !115
}

; Function Attrs: mustprogress nofree nounwind willreturn allockind("alloc,uninitialized") allocsize(0) memory(inaccessiblemem: readwrite)
declare noalias noundef ptr @malloc(i64 noundef) local_unnamed_addr #18

; Function Attrs: nofree norecurse nosync nounwind memory(read, inaccessiblemem: none) uwtable
define dso_local noundef zeroext i1 @str_is_num(ptr noundef readonly captures(none) %0) local_unnamed_addr #19 {
  %2 = load i64, ptr %0, align 8, !tbaa !109
  %3 = icmp eq i64 %2, 0
  br i1 %3, label %15, label %4

4:                                                ; preds = %1
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %6 = load ptr, ptr %5, align 8, !tbaa !106
  br label %7

7:                                                ; preds = %7, %4
  %8 = phi i64 [ 0, %4 ], [ %12, %7 ]
  %9 = getelementptr inbounds nuw i16, ptr %6, i64 %8
  %10 = load i16, ptr %9, align 2, !tbaa !41
  %11 = icmp eq i16 %10, 46
  %12 = add nuw i64 %8, 1
  %13 = icmp eq i64 %12, %2
  %14 = select i1 %11, i1 true, i1 %13
  br i1 %14, label %15, label %7, !llvm.loop !116

15:                                               ; preds = %7, %1
  %16 = phi i1 [ false, %1 ], [ %11, %7 ]
  ret i1 %16
}

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @str_is_double(ptr noundef readonly captures(address_is_null) %0) local_unnamed_addr #5 {
  %2 = alloca ptr, align 8
  %3 = alloca %struct.JSATODTempMem, align 8
  %4 = icmp eq ptr %0, null
  br i1 %4, label %65, label %5

5:                                                ; preds = %1
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %7 = load ptr, ptr %6, align 8, !tbaa !106
  %8 = icmp eq ptr %7, null
  br i1 %8, label %65, label %9

9:                                                ; preds = %5
  %10 = load i64, ptr %0, align 8, !tbaa !109
  %11 = icmp eq i64 %10, 0
  br i1 %11, label %31, label %12

12:                                               ; preds = %9, %17
  %13 = phi i64 [ %18, %17 ], [ 0, %9 ]
  %14 = getelementptr inbounds nuw i16, ptr %7, i64 %13
  %15 = load i16, ptr %14, align 2, !tbaa !41
  %16 = tail call fastcc zeroext i1 @is_ecma_ws_or_lt_u16(i16 noundef zeroext %15)
  br i1 %16, label %17, label %20

17:                                               ; preds = %12
  %18 = add nuw i64 %13, 1
  %19 = icmp eq i64 %18, %10
  br i1 %19, label %65, label %12, !llvm.loop !117

20:                                               ; preds = %12
  %21 = getelementptr i8, ptr %7, i64 -2
  %22 = icmp ugt i64 %10, %13
  br i1 %22, label %23, label %31

23:                                               ; preds = %20, %28
  %24 = phi i64 [ %29, %28 ], [ %10, %20 ]
  %25 = getelementptr i16, ptr %21, i64 %24
  %26 = load i16, ptr %25, align 2, !tbaa !41
  %27 = tail call fastcc zeroext i1 @is_ecma_ws_or_lt_u16(i16 noundef zeroext %26)
  br i1 %27, label %28, label %31

28:                                               ; preds = %23
  %29 = add i64 %24, -1
  %30 = icmp ugt i64 %29, %13
  br i1 %30, label %23, label %65, !llvm.loop !118

31:                                               ; preds = %23, %9, %20
  %32 = phi i64 [ %13, %20 ], [ 0, %9 ], [ %13, %23 ]
  %33 = phi i64 [ %10, %20 ], [ 0, %9 ], [ %24, %23 ]
  %34 = icmp eq i64 %32, %33
  br i1 %34, label %65, label %35

35:                                               ; preds = %31
  %36 = sub i64 %33, %32
  %37 = add i64 %36, 1
  %38 = tail call ptr @llvm.stacksave.p0()
  %39 = alloca i8, i64 %37, align 16
  %40 = load ptr, ptr %6, align 8, !tbaa !106
  %41 = getelementptr i16, ptr %40, i64 %32
  br label %42

42:                                               ; preds = %35, %48
  %43 = phi i64 [ 0, %35 ], [ %51, %48 ]
  %44 = getelementptr i16, ptr %41, i64 %43
  %45 = load i16, ptr %44, align 2, !tbaa !41
  switch i16 %45, label %46 [
    i16 110, label %63
    i16 95, label %63
  ]

46:                                               ; preds = %42
  %47 = icmp ugt i16 %45, 127
  br i1 %47, label %63, label %48

48:                                               ; preds = %46
  %49 = trunc nuw nsw i16 %45 to i8
  %50 = getelementptr inbounds nuw i8, ptr %39, i64 %43
  store i8 %49, ptr %50, align 1, !tbaa !5
  %51 = add nuw i64 %43, 1
  %52 = icmp eq i64 %51, %36
  br i1 %52, label %53, label %42, !llvm.loop !119

53:                                               ; preds = %48
  %54 = getelementptr inbounds nuw i8, ptr %39, i64 %36
  store i8 0, ptr %54, align 1, !tbaa !5
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %2) #22
  store ptr null, ptr %2, align 8, !tbaa !43
  call void @llvm.lifetime.start.p0(i64 216, ptr nonnull %3) #22
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(216) %3, i8 0, i64 216, i1 false)
  %55 = call double @js_atod(ptr noundef nonnull %39, ptr noundef nonnull %2, i32 noundef 0, i32 noundef 2, ptr noundef nonnull %3)
  %56 = load ptr, ptr %2, align 8, !tbaa !43
  %57 = icmp eq ptr %56, null
  br i1 %57, label %61, label %58

58:                                               ; preds = %53
  %59 = load i8, ptr %56, align 1, !tbaa !5
  %60 = icmp eq i8 %59, 0
  br label %61

61:                                               ; preds = %58, %53
  %62 = phi i1 [ false, %53 ], [ %60, %58 ]
  call void @llvm.lifetime.end.p0(i64 216, ptr nonnull %3) #22
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %2) #22
  br label %63

63:                                               ; preds = %46, %42, %42, %61
  %64 = phi i1 [ %62, %61 ], [ false, %42 ], [ false, %42 ], [ false, %46 ]
  call void @llvm.stackrestore.p0(ptr %38)
  br label %65

65:                                               ; preds = %17, %28, %63, %31, %1, %5
  %66 = phi i1 [ true, %5 ], [ true, %1 ], [ %64, %63 ], [ true, %31 ], [ true, %28 ], [ true, %17 ]
  ret i1 %66
}

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @str_to_bool(ptr noundef readnone captures(none) %0, ptr noundef readonly captures(address_is_null) %1) local_unnamed_addr #5 {
  %3 = alloca i32, align 4
  %4 = alloca [16 x i16], align 16
  %5 = icmp eq ptr %1, null
  br i1 %5, label %57, label %6

6:                                                ; preds = %2
  %7 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %8 = load ptr, ptr %7, align 8, !tbaa !106
  %9 = icmp eq ptr %8, null
  br i1 %9, label %57, label %10

10:                                               ; preds = %6
  %11 = load i64, ptr %1, align 8, !tbaa !109
  %12 = trunc i64 %11 to i32
  %13 = icmp sgt i32 %12, 0
  br i1 %13, label %14, label %40

14:                                               ; preds = %10, %21
  %15 = phi i32 [ %23, %21 ], [ %12, %10 ]
  %16 = phi ptr [ %22, %21 ], [ %8, %10 ]
  %17 = load i16, ptr %16, align 2, !tbaa !41
  %18 = zext i16 %17 to i32
  %19 = tail call signext i8 @u_isUWhiteSpace_76(i32 noundef %18) #22
  %20 = icmp eq i8 %19, 0
  br i1 %20, label %25, label %21

21:                                               ; preds = %14
  %22 = getelementptr inbounds nuw i8, ptr %16, i64 2
  %23 = add nsw i32 %15, -1
  %24 = icmp sgt i32 %15, 1
  br i1 %24, label %14, label %57, !llvm.loop !120

25:                                               ; preds = %14
  %26 = getelementptr i8, ptr %16, i64 -2
  %27 = zext nneg i32 %15 to i64
  br label %28

28:                                               ; preds = %25, %35
  %29 = phi i64 [ %27, %25 ], [ %36, %35 ]
  %30 = getelementptr i16, ptr %26, i64 %29
  %31 = load i16, ptr %30, align 2, !tbaa !41
  %32 = zext i16 %31 to i32
  %33 = tail call signext i8 @u_isUWhiteSpace_76(i32 noundef %32) #22
  %34 = icmp eq i8 %33, 0
  br i1 %34, label %38, label %35

35:                                               ; preds = %28
  %36 = add nsw i64 %29, -1
  %37 = icmp sgt i64 %29, 1
  br i1 %37, label %28, label %57, !llvm.loop !121

38:                                               ; preds = %28
  %39 = trunc nuw nsw i64 %29 to i32
  br label %40

40:                                               ; preds = %38, %10
  %41 = phi ptr [ %8, %10 ], [ %16, %38 ]
  %42 = phi i32 [ %12, %10 ], [ %39, %38 ]
  switch i32 %42, label %46 [
    i32 0, label %57
    i32 1, label %43
  ]

43:                                               ; preds = %40
  %44 = load i16, ptr %41, align 2, !tbaa !41
  %45 = icmp eq i16 %44, 48
  br i1 %45, label %57, label %46

46:                                               ; preds = %40, %43
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %3) #22
  store i32 0, ptr %3, align 4, !tbaa !12
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %4) #22
  %47 = call i32 @u_strToLower_76(ptr noundef nonnull %4, i32 noundef 16, ptr noundef nonnull %41, i32 noundef %42, ptr noundef nonnull @.str, ptr noundef nonnull %3) #22
  %48 = load i32, ptr %3, align 4, !tbaa !12
  %49 = icmp sgt i32 %48, 0
  %50 = icmp ne i32 %47, 5
  %51 = select i1 %49, i1 true, i1 %50
  br i1 %51, label %55, label %52

52:                                               ; preds = %46
  %53 = call i32 @u_strncmp_76(ptr noundef nonnull %4, ptr noundef nonnull @str_to_bool.false_word, i32 noundef 5) #22
  %54 = icmp ne i32 %53, 0
  br label %55

55:                                               ; preds = %52, %46
  %56 = phi i1 [ true, %46 ], [ %54, %52 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %4) #22
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %3) #22
  br label %57

57:                                               ; preds = %21, %35, %55, %40, %43, %2, %6
  %58 = phi i1 [ false, %6 ], [ false, %2 ], [ %56, %55 ], [ false, %40 ], [ false, %43 ], [ false, %35 ], [ false, %21 ]
  ret i1 %58
}

declare signext i8 @u_isUWhiteSpace_76(i32 noundef) local_unnamed_addr #20

declare i32 @u_strToLower_76(ptr noundef, i32 noundef, ptr noundef, i32 noundef, ptr noundef, ptr noundef) local_unnamed_addr #20

declare i32 @u_strncmp_76(ptr noundef, ptr noundef, i32 noundef) local_unnamed_addr #20

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @str_cmp_gt(ptr noundef readonly captures(none) %0, ptr noundef readonly captures(none) %1) local_unnamed_addr #5 {
  %3 = load i64, ptr %0, align 8, !tbaa !109
  %4 = getelementptr i8, ptr %0, i64 8
  %5 = load ptr, ptr %4, align 8, !tbaa !106
  %6 = load i64, ptr %1, align 8, !tbaa !109
  %7 = getelementptr i8, ptr %1, i64 8
  %8 = load ptr, ptr %7, align 8, !tbaa !106
  %9 = tail call fastcc i32 @str_cmp_lowered(i64 %3, ptr %5, i64 %6, ptr %8)
  %10 = icmp sgt i32 %9, 0
  ret i1 %10
}

; Function Attrs: nounwind uwtable
define internal fastcc i32 @str_cmp_lowered(i64 %0, ptr readonly captures(none) %1, i64 %2, ptr readonly captures(none) %3) unnamed_addr #5 {
  %5 = trunc i64 %0 to i32
  %6 = trunc i64 %2 to i32
  br label %7

7:                                                ; preds = %60, %4
  %8 = phi i32 [ 0, %4 ], [ %61, %60 ]
  %9 = phi i32 [ 0, %4 ], [ %38, %60 ]
  %10 = phi i32 [ undef, %4 ], [ %68, %60 ]
  %11 = icmp slt i32 %9, %5
  %12 = icmp slt i32 %8, %6
  %13 = select i1 %11, i1 %12, i1 false
  br i1 %13, label %14, label %70

14:                                               ; preds = %7
  %15 = add nsw i32 %9, 1
  %16 = sext i32 %9 to i64
  %17 = getelementptr inbounds i16, ptr %1, i64 %16
  %18 = load i16, ptr %17, align 2, !tbaa !41
  %19 = zext i16 %18 to i32
  %20 = and i32 %19, 64512
  %21 = icmp ne i32 %20, 55296
  %22 = icmp eq i32 %15, %5
  %23 = select i1 %21, i1 true, i1 %22
  br i1 %23, label %36, label %24

24:                                               ; preds = %14
  %25 = sext i32 %15 to i64
  %26 = getelementptr inbounds i16, ptr %1, i64 %25
  %27 = load i16, ptr %26, align 2, !tbaa !41
  %28 = zext i16 %27 to i32
  %29 = and i32 %28, 64512
  %30 = icmp eq i32 %29, 56320
  br i1 %30, label %31, label %36

31:                                               ; preds = %24
  %32 = add nsw i32 %9, 2
  %33 = shl nuw nsw i32 %19, 10
  %34 = add nsw i32 %33, -56613888
  %35 = add nuw nsw i32 %34, %28
  br label %36

36:                                               ; preds = %24, %31, %14
  %37 = phi i32 [ %19, %14 ], [ %35, %31 ], [ %19, %24 ]
  %38 = phi i32 [ %15, %14 ], [ %32, %31 ], [ %15, %24 ]
  %39 = add nsw i32 %8, 1
  %40 = sext i32 %8 to i64
  %41 = getelementptr inbounds i16, ptr %3, i64 %40
  %42 = load i16, ptr %41, align 2, !tbaa !41
  %43 = zext i16 %42 to i32
  %44 = and i32 %43, 64512
  %45 = icmp ne i32 %44, 55296
  %46 = icmp eq i32 %39, %6
  %47 = select i1 %45, i1 true, i1 %46
  br i1 %47, label %60, label %48

48:                                               ; preds = %36
  %49 = sext i32 %39 to i64
  %50 = getelementptr inbounds i16, ptr %3, i64 %49
  %51 = load i16, ptr %50, align 2, !tbaa !41
  %52 = zext i16 %51 to i32
  %53 = and i32 %52, 64512
  %54 = icmp eq i32 %53, 56320
  br i1 %54, label %55, label %60

55:                                               ; preds = %48
  %56 = add nsw i32 %8, 2
  %57 = shl nuw nsw i32 %43, 10
  %58 = add nsw i32 %57, -56613888
  %59 = add nuw nsw i32 %58, %52
  br label %60

60:                                               ; preds = %48, %55, %36
  %61 = phi i32 [ %39, %36 ], [ %56, %55 ], [ %39, %48 ]
  %62 = phi i32 [ %43, %36 ], [ %59, %55 ], [ %43, %48 ]
  %63 = tail call i32 @u_tolower_76(i32 noundef %37) #22
  %64 = tail call i32 @u_tolower_76(i32 noundef %62) #22
  %65 = icmp slt i32 %63, %64
  %66 = icmp sgt i32 %63, %64
  %67 = select i1 %66, i32 1, i32 %10
  %68 = select i1 %65, i32 -1, i32 %67
  %69 = icmp eq i32 %63, %64
  br i1 %69, label %7, label %73, !llvm.loop !122

70:                                               ; preds = %7
  %71 = sext i1 %12 to i32
  %72 = select i1 %11, i32 1, i32 %71
  br label %73

73:                                               ; preds = %60, %70
  %74 = phi i32 [ %72, %70 ], [ %68, %60 ]
  ret i32 %74
}

declare i32 @u_tolower_76(i32 noundef) local_unnamed_addr #20

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @str_cmp_lt(ptr noundef readonly captures(none) %0, ptr noundef readonly captures(none) %1) local_unnamed_addr #5 {
  %3 = load i64, ptr %0, align 8, !tbaa !109
  %4 = getelementptr i8, ptr %0, i64 8
  %5 = load ptr, ptr %4, align 8, !tbaa !106
  %6 = load i64, ptr %1, align 8, !tbaa !109
  %7 = getelementptr i8, ptr %1, i64 8
  %8 = load ptr, ptr %7, align 8, !tbaa !106
  %9 = tail call fastcc i32 @str_cmp_lowered(i64 %3, ptr %5, i64 %6, ptr %8)
  %10 = icmp slt i32 %9, 0
  ret i1 %10
}

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @str_cmp_eq(ptr noundef readonly captures(none) %0, ptr noundef readonly captures(none) %1) local_unnamed_addr #5 {
  %3 = load i64, ptr %0, align 8, !tbaa !109
  %4 = getelementptr i8, ptr %0, i64 8
  %5 = load ptr, ptr %4, align 8, !tbaa !106
  %6 = load i64, ptr %1, align 8, !tbaa !109
  %7 = getelementptr i8, ptr %1, i64 8
  %8 = load ptr, ptr %7, align 8, !tbaa !106
  %9 = tail call fastcc i32 @str_cmp_lowered(i64 %3, ptr %5, i64 %6, ptr %8)
  %10 = icmp eq i32 %9, 0
  ret i1 %10
}

; Function Attrs: nounwind uwtable
define dso_local i64 @xyo_now_ns() local_unnamed_addr #5 {
  %1 = alloca %struct.timespec, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %1) #22
  %2 = call i32 @clock_gettime(i32 noundef 1, ptr noundef nonnull %1) #22
  %3 = load i64, ptr %1, align 8, !tbaa !123
  %4 = mul nsw i64 %3, 1000000000
  %5 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %6 = load i64, ptr %5, align 8, !tbaa !125
  %7 = add nsw i64 %4, %6
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %1) #22
  ret i64 %7
}

; Function Attrs: nounwind
declare i32 @clock_gettime(i32 noundef, ptr noundef) local_unnamed_addr #21

; Function Attrs: nounwind uwtable
define dso_local void @xyo_sleep_until_ns(i64 noundef %0) local_unnamed_addr #5 {
  %2 = alloca %struct.timespec, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %2) #22
  %3 = sdiv i64 %0, 1000000000
  store i64 %3, ptr %2, align 8, !tbaa !123
  %4 = srem i64 %0, 1000000000
  %5 = getelementptr inbounds nuw i8, ptr %2, i64 8
  store i64 %4, ptr %5, align 8, !tbaa !125
  br label %6

6:                                                ; preds = %6, %1
  %7 = call i32 @clock_nanosleep(i32 noundef 1, i32 noundef 1, ptr noundef nonnull %2, ptr noundef null) #22
  %8 = icmp eq i32 %7, 4
  br i1 %8, label %6, label %9, !llvm.loop !126

9:                                                ; preds = %6
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %2) #22
  ret void
}

declare i32 @clock_nanosleep(i32 noundef, i32 noundef, ptr noundef, ptr noundef) local_unnamed_addr #20

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.floor.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.ceil.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sin.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.cos.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.tan.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.asin.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.acos.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.atan.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.log.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.log10.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.exp.f64(double) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.exp10.f64(double) #3

define i64 @xorshift128plus() {
entry:
  %x = load i64, ptr @xorshift128_state_0, align 8
  %y = load i64, ptr @xorshift128_state_1, align 8
  store i64 %y, ptr @xorshift128_state_0, align 8
  %x_shift = shl i64 %x, 23
  %x2 = xor i64 %x, %x_shift
  %x_shift2 = lshr i64 %x2, 17
  %x3 = xor i64 %x2, %x_shift2
  %x4 = xor i64 %x3, %y
  %y_shift2 = lshr i64 %y, 26
  %x5 = xor i64 %x4, %y_shift2
  store i64 %x5, ptr @xorshift128_state_1, align 8
  %ret = add i64 %x5, %y
  ret i64 %ret
}

define void @func_a(ptr %0) {
entry:
  %field0 = getelementptr inbounds nuw { double, double, double }, ptr %0, i32 0, i32 0
  %field01 = getelementptr inbounds nuw { double, double, double }, ptr %0, i32 0, i32 1
  %str_cmp_eq = call i1 @str_cmp_eq(ptr @string_struct, ptr @string_struct.1)
  %num_bool = select i1 %str_cmp_eq, double 1.000000e+00, double 0.000000e+00
  store double %num_bool, ptr %field0, align 8
  store double 0.000000e+00, ptr %field01, align 8
  ret void
}

attributes #0 = { nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { cold noreturn nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #7 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #8 = { nofree norecurse nounwind memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #9 = { mustprogress nofree nounwind willreturn memory(argmem: read) "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #10 = { mustprogress nofree norecurse nounwind willreturn memory(argmem: read) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #11 = { nofree nounwind willreturn memory(argmem: read) }
attributes #12 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #13 = { mustprogress nounwind willreturn memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #14 = { mustprogress nounwind willreturn allockind("realloc") allocsize(1) memory(argmem: readwrite, inaccessiblemem: readwrite) "alloc-family"="malloc" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #15 = { nocallback nofree nosync nounwind willreturn }
attributes #16 = { nofree nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #17 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #18 = { mustprogress nofree nounwind willreturn allockind("alloc,uninitialized") allocsize(0) memory(inaccessiblemem: readwrite) "alloc-family"="malloc" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #19 = { nofree norecurse nosync nounwind memory(read, inaccessiblemem: none) uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #20 = { "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #21 = { nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #22 = { nounwind }
attributes #23 = { cold noreturn nounwind }
attributes #24 = { nounwind willreturn memory(read) }
attributes #25 = { nounwind allocsize(1) }
attributes #26 = { nounwind allocsize(0) }

!llvm.ident = !{!0, !0, !0}
!llvm.module.flags = !{!1, !2, !3, !4}

!0 = !{!"Debian clang version 21.1.8 (++20251221033036+2078da43e25a-1~exp1~20251221153213.50)"}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"PIE Level", i32 2}
!4 = !{i32 7, !"uwtable", i32 2}
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
!24 = distinct !{!24, !9, !25, !26}
!25 = !{!"llvm.loop.isvectorized", i32 1}
!26 = !{!"llvm.loop.unroll.runtime.disable"}
!27 = distinct !{!27, !9, !26, !25}
!28 = distinct !{!28, !9, !25, !26}
!29 = distinct !{!29, !9, !26, !25}
!30 = distinct !{!30, !9, !25, !26}
!31 = distinct !{!31, !9, !26, !25}
!32 = distinct !{!32, !9}
!33 = distinct !{!33, !34}
!34 = !{!"llvm.loop.unroll.disable"}
!35 = distinct !{!35, !9}
!36 = distinct !{!36, !9}
!37 = distinct !{!37, !9}
!38 = distinct !{!38, !9}
!39 = distinct !{!39, !9}
!40 = distinct !{!40, !9}
!41 = !{!42, !42, i64 0}
!42 = !{!"short", !6, i64 0}
!43 = !{!44, !44, i64 0}
!44 = !{!"p1 omnipotent char", !45, i64 0}
!45 = !{!"any pointer", !6, i64 0}
!46 = distinct !{!46, !9}
!47 = !{!48, !45, i64 40}
!48 = !{!"DynBuf", !44, i64 0, !49, i64 8, !49, i64 16, !13, i64 24, !45, i64 32, !45, i64 40}
!49 = !{!"long", !6, i64 0}
!50 = !{!48, !45, i64 32}
!51 = !{!48, !49, i64 8}
!52 = !{!48, !49, i64 16}
!53 = !{!48, !13, i64 24}
!54 = !{!48, !44, i64 0}
!55 = distinct !{!55, !9}
!56 = !{!57, !44, i64 0}
!57 = !{!"", !44, i64 0, !49, i64 8, !13, i64 16}
!58 = !{!57, !49, i64 8}
!59 = !{!57, !13, i64 16}
!60 = distinct !{!60, !9}
!61 = distinct !{!61, !9}
!62 = distinct !{!62, !9}
!63 = distinct !{!63, !9}
!64 = distinct !{!64, !9}
!65 = distinct !{!65, !9}
!66 = distinct !{!66, !9}
!67 = distinct !{!67, !9}
!68 = distinct !{!68, !9}
!69 = distinct !{!69, !9}
!70 = !{!49, !49, i64 0}
!71 = distinct !{!71, !9}
!72 = !{!73}
!73 = distinct !{!73, !74}
!74 = distinct !{!74, !"LVerDomain"}
!75 = !{!76}
!76 = distinct !{!76, !74}
!77 = distinct !{!77, !9, !25, !26}
!78 = distinct !{!78, !34}
!79 = distinct !{!79, !9, !25}
!80 = !{!81}
!81 = distinct !{!81, !82}
!82 = distinct !{!82, !"LVerDomain"}
!83 = !{!84}
!84 = distinct !{!84, !82}
!85 = distinct !{!85, !9, !25, !26}
!86 = distinct !{!86, !34}
!87 = distinct !{!87, !9, !25}
!88 = !{!89}
!89 = distinct !{!89, !90}
!90 = distinct !{!90, !"LVerDomain"}
!91 = !{!92}
!92 = distinct !{!92, !90}
!93 = distinct !{!93, !9, !25, !26}
!94 = distinct !{!94, !9, !25, !26}
!95 = distinct !{!95, !34}
!96 = distinct !{!96, !9, !25}
!97 = !{!98}
!98 = distinct !{!98, !99}
!99 = distinct !{!99, !"LVerDomain"}
!100 = !{!101}
!101 = distinct !{!101, !99}
!102 = distinct !{!102, !9, !25, !26}
!103 = distinct !{!103, !9, !25, !26}
!104 = distinct !{!104, !34}
!105 = distinct !{!105, !9, !25}
!106 = !{!107, !108, i64 8}
!107 = !{!"xyo_string_struct", !49, i64 0, !108, i64 8, !49, i64 16, !49, i64 24}
!108 = !{!"p1 short", !45, i64 0}
!109 = !{!107, !49, i64 0}
!110 = distinct !{!110, !9}
!111 = distinct !{!111, !9}
!112 = distinct !{!112, !9}
!113 = !{!107, !49, i64 16}
!114 = !{!107, !49, i64 24}
!115 = distinct !{!115, !9}
!116 = distinct !{!116, !9}
!117 = distinct !{!117, !9}
!118 = distinct !{!118, !9}
!119 = distinct !{!119, !9}
!120 = distinct !{!120, !9}
!121 = distinct !{!121, !9}
!122 = distinct !{!122, !9}
!123 = !{!124, !49, i64 0}
!124 = !{!"timespec", !49, i64 0, !49, i64 8}
!125 = !{!124, !49, i64 8}
!126 = distinct !{!126, !9}

pub union NativeEndian {
    pub nativeendian_dummy: i32,
    pub nativeendian_little: i8,
}
pub const NATIVE_ENDIAN: NativeEndian = NativeEndian { nativeendian_dummy: 1 };

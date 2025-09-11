pub union NativeEndian {
    pub dummy: i32,
    pub little: libc::c_char,
}

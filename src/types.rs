pub type lua_KContext = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}

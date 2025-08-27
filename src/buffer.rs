use crate::c::*;
use crate::state::*;
use crate::new::*;
use crate::onelua::*;
#[derive(Copy, Clone)]
pub struct Buffer {
    pub pointer: *mut i8,
    pub size: u64,
    pub length: u64,
    pub state: *mut State,
    pub buffer_initial: BufferInitial,
}
#[derive(Copy, Clone)]
pub union BufferInitial {
    pub block: [i8; 1024],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            pointer: std::ptr::null_mut(),
            size: 0,
            length: 0,
            state: std::ptr::null_mut(),
            buffer_initial: BufferInitial { block: [0; 1024] },
        };
    }
}
impl Buffer {
    pub unsafe extern "C" fn lual_buffinitsize(
        & mut self,
        state: *mut State,
        size: u64,
    ) -> *mut i8 { unsafe {
        self.lual_buffinit(state);
        return self.prepbuffsize(size, -1);
    }}
    pub unsafe extern "C" fn lual_pushresultsize(& mut self, size: u64) { unsafe {
        self.length = (self.length as u64).wrapping_add(size) as u64 as u64;
        self.lual_pushresult();
    }}
    pub unsafe extern "C" fn newbuffsize(& mut self, size: u64) -> u64 { unsafe {
        let mut new_size: u64 = (self.size)
            .wrapping_div(2 as u64)
            .wrapping_mul(3 as u64);
        if (((!(0u64)).wrapping_sub(size) < self.length) as i32 != 0) as i32 as i64 != 0 {
            return lual_error(self.state, b"buffer too large\0" as *const u8 as *const i8) as u64;
        }
        if new_size < (self.length).wrapping_add(size) {
            new_size = (self.length).wrapping_add(size);
        }
        return new_size;
    }}
    pub unsafe extern "C" fn prepbuffsize(& mut self, size: u64, boxidx: i32) -> *mut i8 { unsafe {
        if (self.size).wrapping_sub(self.length) >= size {
            return (self.pointer).offset(self.length as isize);
        } else {
            let state: *mut State = self.state;
            let newbuff: *mut i8;
            let new_size: u64 = self.newbuffsize(size);
            if self.pointer != (self.buffer_initial.block).as_mut_ptr() {
                newbuff = resizebox(state, boxidx, new_size) as *mut i8;
            } else {
                lua_rotate(state, boxidx, -1);
                lua_settop(state, -1 - 1);
                newbox(state);
                lua_rotate(state, boxidx, 1);
                lua_toclose(state, boxidx);
                newbuff = resizebox(state, boxidx, new_size) as *mut i8;
                memcpy(
                    newbuff as *mut libc::c_void,
                    self.pointer as *const libc::c_void,
                    (self.length).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
            }
            self.pointer = newbuff;
            self.size = new_size;
            return newbuff.offset(self.length as isize);
        };
    }}
    pub unsafe extern "C" fn lual_prepbuffsize(& mut self, size: u64) -> *mut i8 { unsafe {
        return self.prepbuffsize(size, -1);
    }}
    pub unsafe extern "C" fn lual_addlstring(& mut self, s: *const i8, l: u64) { unsafe {
        if l > 0u64 {
            let b: *mut i8 = self.prepbuffsize(l, -1);
            memcpy(
                b as *mut libc::c_void,
                s as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            self.length = (self.length as u64).wrapping_add(l) as u64 as u64;
        }
    }}
    pub unsafe extern "C" fn lual_addstring(& mut self, s: *const i8) { unsafe {
        self.lual_addlstring(s, strlen(s));
    }}
    pub unsafe extern "C" fn lual_pushresult(& mut self) { unsafe {
        let state: *mut State = self.state;
        lua_pushlstring(state, self.pointer, self.length);
        if self.pointer != (self.buffer_initial.block).as_mut_ptr() {
            lua_closeslot(state, -(2));
        }
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
    }}
    pub unsafe extern "C" fn lual_addvalue(& mut self) { unsafe {
        let state: *mut State = self.state;
        let mut length: u64 = 0;
        let s: *const i8 = lua_tolstring(state, -1, &mut length);
        let b: *mut i8 = self.prepbuffsize(length, -(2));
        memcpy(
            b as *mut libc::c_void,
            s as *const libc::c_void,
            length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        self.length = (self.length as u64).wrapping_add(length) as u64 as u64;
        lua_settop(state, -1 - 1);
    }}
    pub unsafe extern "C" fn lual_buffinit(& mut self, state: *mut State) { unsafe {
        self.state = state;
        self.pointer = (self.buffer_initial.block).as_mut_ptr();
        self.length = 0;
        self.size = (16 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
            .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as u64;
        lua_pushlightuserdata(state, self as *mut Buffer as *mut libc::c_void);
    }}
}

#![feature(extern_types, c_variadic)]
mod abslineinfo;
mod blockcnt;
mod bufffs;
mod c;
mod c2rustunnamed_23;
mod c2rustunnamed_24;
mod c2rustunnamed_25;
mod c2rustunnamed_27;
mod c2rustunnamed_30;
mod callinfo;
mod calls;
mod closep;
mod closure;
mod conscontrol;
mod dumpstate;
mod dyndata;
mod expdesc;
mod funcstate;
mod gcobject;
mod gcunion;
mod gmatchstate;
mod header;
mod instruction;
mod labeldesc;
mod labellist;
mod lexstate;
mod lg;
mod lhs_assign;
mod loadf;
mod loads;
mod loadstate;
mod locvar;
mod lua_debug;
mod lua_reader;
mod lua_writer;
mod lual_buffer;
mod lual_reg;
mod lual_stream;
mod lx;
mod matchstate;
mod mbuffer;
mod node;
mod onelua;
mod proto;
mod ranstate;
mod rn;
mod sparser;
mod state;
mod stkidrel;
mod str_writer;
mod table;
mod tm;
mod token;
mod tstring;
mod ubox;
mod udata;
mod upvaldesc;
mod zio;
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(crate::onelua::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}

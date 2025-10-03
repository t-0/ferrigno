#![feature(default_field_values, extern_types, c_variadic)]
use std::ptr::*;
mod absolutelineinfo;
mod blockcontrol;
mod buffer;
mod bufffs;
mod c;
mod callinfo;
mod calls;
mod character;
mod closeprotected;
mod closure;
mod constructorcontrol;
mod coroutine;
mod debuginfo;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod expressionkind;
mod f2i;
mod forloop;
mod functions;
mod functionstate;
mod global;
mod gmatchstate;
mod header;
mod instruction;
mod interpreter;
mod k;
mod labeldescription;
mod lexicalstate;
mod library;
mod tloadable;
mod loadf;
mod loads;
mod loadstate;
mod localvariable;
mod longjump;
mod macros;
mod matchstate;
mod nativeendian;
mod node;
mod object;
mod objectwithgclist;
mod objectwithmetatable;
mod opcode;
mod operator_;
mod operatorbinary;
mod operatorunary;
mod opmode;
mod priority;
mod prototype;
mod randomstate;
mod registeredfunction;
mod repl;
mod rn;
mod sparser;
mod status;
mod stkidrel;
mod stream;
mod streamwriter;
mod stringtable;
mod table;
mod tag;
mod tdefaultnew;
mod tm;
mod tobject;
mod tobjectwithgclist;
mod tobjectwithmetatable;
mod token;
mod tstring;
mod tvalue;
mod upvalue;
mod upvaluedescription;
mod user;
mod userbox;
mod utility;
mod value;
mod variabledescription;
mod vectort;
mod zio;
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(null_mut());
    unsafe { ::std::process::exit(crate::repl::main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32) }
}

use crate::functions::*;
use crate::interpreter::*;
use crate::status::*;
use crate::tvalue::*;
use libc::*;
#[repr(C)]
pub struct CallS {
    function: *mut TValue,
    count_results: usize,
}
impl CallS {
    fn new(function: *mut TValue) -> Self {
        CallS { function: function, count_results: 0 }
    }
    unsafe fn raw_call(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            luad_callnoyield(interpreter, self.function, self.count_results as i32);
        }
    }
    unsafe fn unwrap_call(interpreter: *mut Interpreter, arbitrary_data: *mut c_void) {
        unsafe {
            let calls = arbitrary_data as *mut CallS;
            (*calls).raw_call(interpreter);
        }
    }
    pub unsafe fn api_call(interpreter: *mut Interpreter, count_arguments: i32, count_results: i32, error_function: i32, context: i64, context_function: ContextFunction) -> Status {
        unsafe {
            let function: i64 = if error_function == 0 {
                0
            } else {
                let tvalue: *mut TValue = index2stack(interpreter, error_function);
                (tvalue as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64
            };
            let mut calls = CallS::new((*interpreter).top.stkidrel_pointer.offset(-((count_arguments + 1) as isize)));
            let status: Status = if context_function.is_none() || !((*interpreter).count_c_calls & 0xffff0000 as u32 == 0) {
                calls.count_results = count_results as usize;
                luad_pcall(
                    interpreter,
                    Some(CallS::unwrap_call as unsafe fn(*mut Interpreter, *mut c_void) -> ()),
                    &mut calls as *mut CallS as *mut c_void,
                    (calls.function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
                    function,
                )
            } else {
                let callinfo = (*interpreter).callinfo;
                (*callinfo).call_info_u.c.context_function = context_function;
                (*callinfo).call_info_u.c.context = context;
                (*callinfo).call_info_u2.funcidx = (calls.function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i32;
                (*callinfo).call_info_u.c.old_error_function = (*interpreter).error_function;
                (*interpreter).error_function = function;
                (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !(1 << 0) | (*interpreter).allow_hook as i32) as u16;
                (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 | 1 << 4) as u16;
                ccall(interpreter, calls.function, count_results, 1);
                (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !(1 << 4)) as u16;
                (*interpreter).error_function = (*callinfo).call_info_u.c.old_error_function;
                Status::OK
            };
            if count_results <= -1 && (*(*interpreter).callinfo).call_info_top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer {
                (*(*interpreter).callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
            }
            return status;
        }
    }
}

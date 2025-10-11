use crate::functions::*;
use crate::interpreter::*;
use crate::status::*;
use crate::tvalue::*;
use libc::*;
#[repr(C)]
pub struct CallS {
    calls_function: *mut TValue,
    calls_countresults: usize,
}
impl CallS {
    fn new(function: *mut TValue) -> Self {
        CallS { calls_function: function, calls_countresults: 0 }
    }
    unsafe fn raw_call(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            luad_callnoyield(interpreter, self.calls_function, self.calls_countresults as i32);
        }
    }
    unsafe fn unwrap_call(interpreter: *mut Interpreter, arbitrary_data: *mut c_void) {
        unsafe {
            let calls = arbitrary_data as *mut CallS;
            (*calls).raw_call(interpreter);
        }
    }
    pub unsafe fn api_call(
        interpreter: *mut Interpreter, count_arguments: i32, count_results: i32, error_function: i32, context: i64,
        context_function: ContextFunction,
    ) -> Status {
        unsafe {
            let function: i64 = if error_function == 0 {
                0
            } else {
                let tvalue: *mut TValue = index2stack(interpreter, error_function);
                (tvalue as *mut i8).offset_from((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8) as i64
            };
            let mut calls = CallS::new(
                (*interpreter)
                    .interpreter_top
                    .stkidrel_pointer
                    .offset(-((count_arguments + 1) as isize)),
            );
            let status: Status = if context_function.is_none() || (*interpreter).in_nny() {
                calls.calls_countresults = count_results as usize;
                luad_pcall(
                    interpreter,
                    Some(CallS::unwrap_call as unsafe fn(*mut Interpreter, *mut c_void) -> ()),
                    &mut calls as *mut CallS as *mut c_void,
                    (calls.calls_function as *mut i8).offset_from((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8)
                        as i64,
                    function,
                )
            } else {
                let callinfo = (*interpreter).interpreter_callinfo;
                (*callinfo).callinfo_u.c.context_function = context_function;
                (*callinfo).callinfo_u.c.context = context;
                (*callinfo).callinfo_u2.callinfoconstituentb_funcidx = (calls.calls_function as *mut i8)
                    .offset_from((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8)
                    as i32;
                (*callinfo).callinfo_u.c.old_error_function = (*interpreter).interpreter_errorfunction;
                (*interpreter).interpreter_errorfunction = function;
                (*callinfo).callinfo_callstatus =
                    ((*callinfo).callinfo_callstatus as i32 & !(1 << 0) | (*interpreter).interpreter_allowhook as i32) as u16;
                (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 | 1 << 4) as u16;
                ccall(interpreter, calls.calls_function, count_results, 1);
                (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !(1 << 4)) as u16;
                (*interpreter).interpreter_errorfunction = (*callinfo).callinfo_u.c.old_error_function;
                Status::OK
            };
            if count_results <= -1
                && (*(*interpreter).interpreter_callinfo).callinfo_top.stkidrel_pointer
                    < (*interpreter).interpreter_top.stkidrel_pointer
            {
                (*(*interpreter).interpreter_callinfo).callinfo_top.stkidrel_pointer =
                    (*interpreter).interpreter_top.stkidrel_pointer;
            }
            return status;
        }
    }
}

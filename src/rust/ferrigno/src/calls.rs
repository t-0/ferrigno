use crate::functions::*;
use crate::state::*;
use crate::status::*;
use crate::tvalue::*;
#[repr(C)]
pub struct CallS {
    calls_function: *mut TValue,
    calls_count_results: usize,
}
impl CallS {
    fn new(function: *mut TValue) -> Self {
        CallS {
            calls_function: function,
            calls_count_results: 0,
        }
    }
    unsafe fn raw_call(&mut self, state: *mut State) {
        unsafe {
            luad_callnoyield(state, self.calls_function, self.calls_count_results as i32);
        }
    }
    unsafe fn unwrap_call(state: *mut State, arbitrary_data: *mut std::ffi::c_void) {
        unsafe {
            let calls = arbitrary_data as *mut CallS;
            (*calls).raw_call(state);
        }
    }
    pub unsafe fn api_call(
        state: *mut State,
        count_arguments: i32,
        count_results: i32,
        error_function: i32,
        context: i64,
        context_function: ContextFunction,
    ) -> Status {
        unsafe {
            let function: i64 = if error_function == 0 {
                0
            } else {
                let tvalue: *mut TValue = index2stack(state, error_function);
                (tvalue as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64
            };
            let mut calls = CallS::new(
                (*state)
                    .interpreter_top
                    .stkidrel_pointer
                    .sub((count_arguments + 1) as usize),
            );
            let status: Status = if context_function.is_none() || !(*state).is_yieldable() {
                calls.calls_count_results = count_results as usize;
                luad_pcall(
                    state,
                    Some(CallS::unwrap_call as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
                    &mut calls as *mut CallS as *mut std::ffi::c_void,
                    (calls.calls_function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64,
                    function,
                )
            } else {
                let callinfo = (*state).interpreter_callinfo;
                (*callinfo).callinfo_u.c.context_function = context_function;
                (*callinfo).callinfo_u.c.context = context;
                (*callinfo).callinfo_u2.callinfoconstituentb_funcidx =
                    (calls.calls_function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i32;
                (*callinfo).callinfo_u.c.old_error_function = (*state).interpreter_error_function;
                (*state).interpreter_error_function = function;
                (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !CALLSTATUS_ALLOWHOOK
                    | (*state).interpreter_allow_hook as i32) as u16;
                (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 | CALLSTATUS_YPCALL) as u16;
                ccall(state, calls.calls_function, count_results, 1);
                (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !CALLSTATUS_YPCALL) as u16;
                (*state).interpreter_error_function = (*callinfo).callinfo_u.c.old_error_function;
                Status::OK
            };
            if count_results <= -1
                && (*(*state).interpreter_callinfo)
                    .callinfo_top
                    .stkidrel_pointer
                    < (*state).interpreter_top.stkidrel_pointer
            {
                (*(*state).interpreter_callinfo)
                    .callinfo_top
                    .stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer;
            }
            status
        }
    }
}

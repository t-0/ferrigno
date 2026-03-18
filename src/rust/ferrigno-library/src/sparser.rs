use crate::buffer::*;
use crate::closure::*;
use crate::dynamicdata::*;
use crate::functions::*;
use crate::labeldescription::*;
use crate::loadstate::*;
use crate::state::*;
use crate::status::*;
use crate::tdefaultnew::*;
use crate::utility::*;
use crate::variabledescription::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
struct SParser {
    sparser_zio: ZIO,
    sparser_name: *const i8,
    sparser_mode: *const i8,
    sparser_buffer: Buffer,
    sparser_dynamicdata: DynamicData,
}
impl SParser {
    fn new(state: *mut State, name: *const i8, mode: *const i8, reader: Reader, data: *mut std::ffi::c_void) -> Self {
        let mut ret = SParser {
            sparser_zio: ZIO::new(state, reader, data),
            sparser_name: if name.is_null() { c"?".as_ptr() } else { name },
            sparser_mode: mode,
            sparser_buffer: Buffer::new(),
            sparser_dynamicdata: DynamicData::new(),
        };
        ret.sparser_buffer.buffer_loads.initialize();
        ret.sparser_dynamicdata.dynamicdata_active_variables.initialize();
        ret.sparser_dynamicdata.dynamicdata_goto.initialize();
        ret.sparser_dynamicdata.dynamicdata_labels.initialize();
        ret
    }
    fn clean(&mut self, state: *mut State) {
        unsafe {
            (*state).free_memory(
                self.sparser_dynamicdata.dynamicdata_labels.vectort_pointer as *mut std::ffi::c_void,
                self.sparser_dynamicdata
                    .dynamicdata_labels
                    .get_size()
                    .wrapping_mul(size_of::<LabelDescription>()),
            );
            (*state).free_memory(
                self.sparser_dynamicdata.dynamicdata_goto.vectort_pointer as *mut std::ffi::c_void,
                self.sparser_dynamicdata
                    .dynamicdata_goto
                    .get_size()
                    .wrapping_mul(size_of::<LabelDescription>()),
            );
            (*state).free_memory(
                self.sparser_dynamicdata.dynamicdata_active_variables.vectort_pointer as *mut std::ffi::c_void,
                self.sparser_dynamicdata
                    .dynamicdata_active_variables
                    .get_size()
                    .wrapping_mul(size_of::<VariableDescription>()),
            );
            self.sparser_buffer.buffer_loads.destroy(state);
        }
    }
    unsafe fn do_parser(&mut self, state: *mut State) {
        unsafe {
            let character: i32 = self.sparser_zio.get_char();
            if character == 0x1B {
                let fixed = !self.sparser_mode.is_null() && !cstr_chr(self.sparser_mode, 'B' as i8).is_null();
                if !fixed {
                    checkmode(state, self.sparser_mode, c"binary".as_ptr());
                }
                let closure = load_closure_fixed(state, &mut self.sparser_zio, self.sparser_name, fixed);
                Closure::luaf_initupvals(state, closure);
            } else {
                checkmode(state, self.sparser_mode, c"text".as_ptr());
                let closure = luay_parser(
                    state, &mut self.sparser_zio, &mut self.sparser_buffer, &mut self.sparser_dynamicdata, self.sparser_name,
                    character,
                );
                Closure::luaf_initupvals(state, closure);
            }
        }
    }
}
unsafe fn f_parser(state: *mut State, pointer: *mut std::ffi::c_void) {
    unsafe {
        let mut sparser = *(pointer as *mut SParser);
        sparser.do_parser(state);
    }
}
pub unsafe fn do_protected_parser(
    state: *mut State, name: *const i8, mode: *const i8, reader: Reader, data: *mut std::ffi::c_void,
) -> Status {
    unsafe {
        let mut sparser = SParser::new(state, name, mode, reader, data);
        (*state).increment_noyield();
        let status = luad_pcall(
            state,
            Some(f_parser as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
            &mut sparser as *mut SParser as *mut std::ffi::c_void,
            ((*state).interpreter_top.stkidrel_pointer as *mut i8)
                .offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64,
            (*state).interpreter_error_function,
        );
        (*state).decrement_noyield();
        sparser.clean(state);
        status
    }
}

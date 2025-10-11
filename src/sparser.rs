use crate::buffer::*;
use crate::closure::*;
use crate::dynamicdata::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::labeldescription::*;
use crate::loadstate::*;
use crate::status::*;
use crate::tdefaultnew::*;
use crate::variabledescription::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
struct SParser {
    m_zio: ZIO,
    m_name: *const i8,
    m_mode: *const i8,
    m_buffer: Buffer,
    m_dynamicdata: DynamicData,
}
impl SParser {
    fn new(interpreter: *mut Interpreter, name: *const i8, mode: *const i8, reader: Reader, data: *mut libc::c_void) -> Self {
        let mut ret = SParser {
            m_zio: ZIO::new(interpreter, reader, data),
            m_name: if name.is_null() { c"?".as_ptr() } else { name },
            m_mode: mode,
            m_buffer: Buffer::new(),
            m_dynamicdata: DynamicData::new(),
        };
        ret.m_buffer.buffer_loads.initialize();
        ret.m_dynamicdata.dynamicdata_activevariables.initialize();
        ret.m_dynamicdata.dynamicdata_goto.initialize();
        ret.m_dynamicdata.dynamicdata_labels.initialize();
        return ret;
    }
    fn clean(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            (*interpreter).free_memory(
                self.m_dynamicdata.dynamicdata_labels.vectort_pointer as *mut libc::c_void,
                (self.m_dynamicdata.dynamicdata_labels.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize)
                    as usize,
            );
            (*interpreter).free_memory(
                self.m_dynamicdata.dynamicdata_goto.vectort_pointer as *mut libc::c_void,
                (self.m_dynamicdata.dynamicdata_goto.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize)
                    as usize,
            );
            (*interpreter).free_memory(
                self.m_dynamicdata.dynamicdata_activevariables.vectort_pointer as *mut libc::c_void,
                (self.m_dynamicdata.dynamicdata_activevariables.get_size() as usize)
                    .wrapping_mul(size_of::<VariableDescription>() as usize) as usize,
            );
            self.m_buffer.buffer_loads.destroy(interpreter);
        }
    }
    unsafe fn do_parser(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let character: i32 = self.m_zio.get_char();
            if character == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
                checkmode(interpreter, self.m_mode, c"binary".as_ptr());
                let closure = load_closure(interpreter, &mut self.m_zio, self.m_name);
                Closure::luaf_initupvals(interpreter, closure);
            } else {
                checkmode(interpreter, self.m_mode, c"text".as_ptr());
                let closure = luay_parser(
                    interpreter, &mut self.m_zio, &mut self.m_buffer, &mut self.m_dynamicdata, self.m_name, character,
                );
                Closure::luaf_initupvals(interpreter, closure);
            }
        }
    }
}
unsafe fn f_parser(interpreter: *mut Interpreter, pointer: *mut libc::c_void) {
    unsafe {
        let mut sparser = *(pointer as *mut SParser);
        sparser.do_parser(interpreter);
    }
}
pub unsafe fn do_protected_parser(
    interpreter: *mut Interpreter, name: *const i8, mode: *const i8, reader: Reader, data: *mut libc::c_void,
) -> Status {
    unsafe {
        let mut sparser = SParser::new(interpreter, name, mode, reader, data);
        (*interpreter).increment_noyield();
        let status = luad_pcall(
            interpreter,
            Some(f_parser as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()),
            &mut sparser as *mut SParser as *mut libc::c_void,
            ((*interpreter).interpreter_top.stkidrel_pointer as *mut i8)
                .offset_from((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8) as i64,
            (*interpreter).interpreter_errorfunction,
        );
        (*interpreter).decrement_noyield();
        sparser.clean(interpreter);
        return status;
    }
}

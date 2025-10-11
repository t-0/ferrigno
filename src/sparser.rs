use crate::buffer::*;
use crate::closure::*;
use crate::dynamicdata::*;
use crate::interpreter::*;
use crate::labeldescription::*;
use crate::functions::*;
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
        SParser {
            m_zio: ZIO::new(interpreter, reader, data),
            m_name: if name.is_null() { c"?".as_ptr() } else { name },
            m_mode: mode,
            m_buffer: Buffer::new(),
            m_dynamicdata: DynamicData::new(),
        }
    }
}
unsafe fn f_parser(interpreter: *mut Interpreter, pointer: *mut libc::c_void) {
    unsafe {
        let mut sparser = *(pointer as *mut SParser);
        let character: i32 = sparser.m_zio.get_char();
        if character == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            checkmode(interpreter, sparser.m_mode, c"binary".as_ptr());
            let closure = load_closure(interpreter, &mut sparser.m_zio, sparser.m_name);
            Closure::luaf_initupvals(interpreter, closure);
        } else {
            checkmode(interpreter, sparser.m_mode, c"text".as_ptr());
            let closure = luay_parser(
                interpreter, &mut sparser.m_zio, &mut sparser.m_buffer, &mut sparser.m_dynamicdata, sparser.m_name, character,
            );
            Closure::luaf_initupvals(interpreter, closure);
        }
    }
}
pub unsafe fn do_protected_parser(interpreter: *mut Interpreter, name: *const i8, mode: *const i8, reader: Reader, data: *mut libc::c_void) -> Status {
    unsafe {
        let mut sparser = SParser::new(interpreter, name, mode, reader, data);
        (*interpreter).interpreter_countccalls =
            ((*interpreter).interpreter_countccalls as u32).wrapping_add(0x10000 as u32) as u32;
        sparser.m_dynamicdata.dynamicdata_activevariables.initialize();
        sparser.m_dynamicdata.dynamicdata_goto.initialize();
        sparser.m_dynamicdata.dynamicdata_labels.initialize();
        sparser.m_buffer.buffer_loads.initialize();
        let status = luad_pcall(
            interpreter,
            Some(f_parser as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()),
            &mut sparser as *mut SParser as *mut libc::c_void,
            ((*interpreter).interpreter_top.stkidrel_pointer as *mut i8)
                .offset_from((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8) as i64,
            (*interpreter).interpreter_errorfunction,
        );
        sparser.m_buffer.buffer_loads.destroy(interpreter);
        (*interpreter).free_memory(
            sparser.m_dynamicdata.dynamicdata_activevariables.vectort_pointer as *mut libc::c_void,
            (sparser.m_dynamicdata.dynamicdata_activevariables.get_size() as usize)
                .wrapping_mul(size_of::<VariableDescription>() as usize) as usize,
        );
        (*interpreter).free_memory(
            sparser.m_dynamicdata.dynamicdata_goto.vectort_pointer as *mut libc::c_void,
            (sparser.m_dynamicdata.dynamicdata_goto.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize)
                as usize,
        );
        (*interpreter).free_memory(
            sparser.m_dynamicdata.dynamicdata_labels.vectort_pointer as *mut libc::c_void,
            (sparser.m_dynamicdata.dynamicdata_labels.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize)
                as usize,
        );
        (*interpreter).interpreter_countccalls =
            ((*interpreter).interpreter_countccalls as u32).wrapping_sub(0x10000 as u32) as u32;
        return status;
    }
}

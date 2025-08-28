use crate::buffer::*;
use crate::dynamicdata::*;
use crate::functionstate::*;
use crate::gcunion::*;
use crate::new::*;
use crate::object::*;
use crate::onelua::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::token::*;
use crate::tstring::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexicalState {
    pub current: i32,
    pub line_number: i32,
    pub last_line: i32,
    pub t: Token,
    pub look_ahead: Token,
    pub fs: *mut FunctionState,
    pub state: *mut State,
    pub zio: *mut ZIO,
    pub buffer: *mut Buffer,
    pub h: *mut Table,
    pub dynamic_data: *mut DynamicData,
    pub source: *mut TString,
    pub envn: *mut TString,
}
impl New for LexicalState {
    fn new() -> Self {
        return LexicalState {
            current: 0,
            line_number: 0,
            last_line: 0,
            t: Token::new(),
            look_ahead: Token::new(),
            fs: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
            zio: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            h: std::ptr::null_mut(),
            dynamic_data: std::ptr::null_mut(),
            source: std::ptr::null_mut(),
            envn: std::ptr::null_mut(),
        };
    }
}
impl LexicalState {
    pub unsafe extern "C" fn add_prototype(&mut self) -> *mut Prototype {
        unsafe {
            let fs: *mut FunctionState = self.fs;
            let f: *mut Prototype = (*fs).f;
            if (*fs).np >= (*f).size_p {
                let mut old_size: i32 = (*f).size_p;
                (*f).p = luam_growaux_(
                    self.state,
                    (*f).p as *mut libc::c_void,
                    (*fs).np,
                    &mut (*f).size_p,
                    ::core::mem::size_of::<*mut Prototype>() as u64 as i32,
                    (if ((1 << 8 + 8 + 1) - 1) as u64
                        <= (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
                    {
                        ((1 << 8 + 8 + 1) - 1) as u32
                    } else {
                        (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
                            as u32
                    }) as i32,
                    b"functions\0" as *const u8 as *const i8,
                ) as *mut *mut Prototype;
                while old_size < (*f).size_p {
                    let fresh45 = old_size;
                    old_size = old_size + 1;
                    let ref mut fresh46 = *((*f).p).offset(fresh45 as isize);
                    *fresh46 = std::ptr::null_mut();
                }
            }
            let clp: *mut Prototype = luaf_newproto(self.state);
            let np = (*fs).np;
            (*fs).np = (*fs).np + 1;
            let ref mut target = *((*f).p).offset(np as isize);
            *target = clp;
            if (*f).get_marked() & 1 << 5 != 0 && (*clp).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    self.state,
                    &mut (*(f as *mut GCUnion)).object,
                    &mut (*(clp as *mut GCUnion)).object,
                );
            } else {
            };
            return clp;
        }
    }
}

use crate::nativeendian::*;
use crate::character::*;
use crate::interpreter::*;
use crate::k::*;
use crate::utility::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    m_interpreter: *mut Interpreter,
    m_islittleendian: bool,
    m_maxmimumalignment: i32,
}
impl Header {
    pub fn new(interpreter: *mut Interpreter) -> Self {
        unsafe {
            Header {
                m_interpreter: interpreter,
                m_islittleendian: NATIVE_ENDIAN.nativeendian_little != 0,
                m_maxmimumalignment: 1,
            }
        }
    }
    pub fn is_little_endian(&self) -> bool {
        self.m_islittleendian
    }
    pub unsafe fn getnumlimit(&self, fmt: *mut *const i8, df: i32) -> i32 {
        unsafe {
            let size: i32 = getnum(fmt, df);
            if size > 16 as i32 || size <= 0 {
                return lual_error(
                    self.m_interpreter,
                    c"integral size (%d) out of limits [1,%d]".as_ptr(),
                    size,
                    16 as i32,
                );
            }
            return size;
        }
    }
    pub unsafe fn getoption(&mut self, fmt: *mut *const i8, size: *mut i32) -> K {
        unsafe {
            let fresh180 = *fmt;
            *fmt = (*fmt).offset(1);
            let opt: i32 = *fresh180 as i32;
            *size = 0;
            match Character::from(opt) {
                | Character::LowerB => {
                    *size = 1 as i32;
                    return K::Integer;
                },
                | Character::UpperB => {
                    *size = 1 as i32;
                    return K::Unsigned;
                },
                | Character::LowerH => {
                    *size = size_of::<i16>() as i32;
                    return K::Integer;
                },
                | Character::UpperH => {
                    *size = size_of::<i16>() as i32;
                    return K::Unsigned;
                },
                | Character::LowerL => {
                    *size = size_of::<i64>() as i32;
                    return K::Integer;
                },
                | Character::UpperL => {
                    *size = size_of::<i64>() as i32;
                    return K::Unsigned;
                },
                | Character::LowerJ => {
                    *size = size_of::<i64>() as i32;
                    return K::Integer;
                },
                | Character::UpperJ => {
                    *size = size_of::<i64>() as i32;
                    return K::Unsigned;
                },
                | Character::UpperT => {
                    *size = size_of::<usize>() as i32;
                    return K::Unsigned;
                },
                | Character::LowerF => {
                    *size = size_of::<f32>() as i32;
                    return K::Float;
                },
                | Character::LowerN => {
                    *size = size_of::<f64>() as i32;
                    return K::Number;
                },
                | Character::LowerD => {
                    *size = size_of::<f64>() as i32;
                    return K::Double;
                },
                | Character::LowerI => {
                    *size = self.getnumlimit(fmt, size_of::<i32>() as i32);
                    return K::Integer;
                },
                | Character::UpperI => {
                    *size = self.getnumlimit(fmt, size_of::<i32>() as i32);
                    return K::Unsigned;
                },
                | Character::LowerS => {
                    *size = self.getnumlimit(fmt, size_of::<usize>() as i32);
                    return K::String;
                },
                | Character::LowerC => {
                    *size = getnum(fmt, -1);
                    if *size == -1 {
                        lual_error(
                            self.m_interpreter,
                            c"missing size for format option Character::LowerC".as_ptr(),
                        );
                    }
                    return K::Character;
                },
                | Character::LowerZ => return K::ZString,
                | Character::LowerX => {
                    *size = 1;
                    return K::Padding;
                },
                | Character::UpperX => return K::PaddingAlignment,
                | Character::Space => {},
                | Character::AngleLeft => {
                    self.m_islittleendian = true;
                },
                | Character::AngleRight => {
                    self.m_islittleendian = false;
                },
                | Character::Equal => {
                    self.m_islittleendian = NATIVE_ENDIAN.nativeendian_little != 0;
                },
                | Character::Exclamation => {
                    let maxalign: i32 = 8;
                    self.m_maxmimumalignment = self.getnumlimit(fmt, maxalign);
                },
                | _ => {
                    lual_error(self.m_interpreter, c"invalid format option '%c'".as_ptr(), opt);
                },
            }
            return K::NoOperator;
        }
    }
    pub unsafe fn getdetails(&mut self, totalsize: usize, fmt: *mut *const i8, total_size: *mut i32, ntoalign: *mut i32,
    ) -> K {
        unsafe {
            let opt: K = self.getoption(fmt, total_size);
            let mut align: i32 = *total_size;
            if opt as u32 == K::PaddingAlignment as u32 {
                if **fmt as i32 == Character::Null as i32
                    || self.getoption(fmt, &mut align) as u32 == K::Character as u32
                    || align == 0
                {
                    lual_argerror(self.m_interpreter, 1, c"invalid next option for option X".as_ptr());
                }
            }
            if align <= 1 || opt as u32 == K::Character as u32 {
                *ntoalign = 0;
            } else {
                if align > self.m_maxmimumalignment {
                    align = self.m_maxmimumalignment;
                }
                if align & align - 1 != 0 {
                    lual_argerror(
                        self.m_interpreter,
                        1,
                        c"format asks for alignment not power of 2".as_ptr(),
                    );
                }
                *ntoalign = align - (totalsize & (align - 1) as usize) as i32 & align - 1;
            }
            return opt;
        }
    }
}

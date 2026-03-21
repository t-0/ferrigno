use crate::character::*;
use crate::nativeendian::*;
use crate::packingtype::*;
use crate::state::*;
use crate::utility::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    header_interpreter: *mut State,
    header_islittleendian: bool,
    header_maxmimumalignment: i32,
}
impl Header {
    pub fn new(state: *mut State) -> Self {
        unsafe {
            Header {
                header_interpreter: state,
                header_islittleendian: NATIVE_ENDIAN.nativeendian_little != 0,
                header_maxmimumalignment: 1,
            }
        }
    }
    pub fn is_little_endian(&self) -> bool {
        self.header_islittleendian
    }
    pub unsafe fn getnumlimit(&self, fmt: *mut *const i8, df: i32) -> i32 {
        unsafe {
            let size: usize = getnum(fmt, df as usize);
            if size > 16 || size == 0 {
                return lual_error(
                    self.header_interpreter,
                    c"integral size (%d) out of limits [1,%d]".as_ptr(),
                    &[(size as i32).into(), 16_i32.into()],
                );
            }
            size as i32
        }
    }
    pub unsafe fn getoption(&mut self, fmt: *mut *const i8, size: *mut usize) -> PackingType {
        unsafe {
            let current_char = *fmt;
            *fmt = (*fmt).add(1);
            let opt: i32 = *current_char as i32;
            *size = 0;
            match Character::from(opt) {
                Character::LowerB => {
                    *size = 1;
                    return PackingType::Integer;
                }
                Character::UpperB => {
                    *size = 1;
                    return PackingType::Unsigned;
                }
                Character::LowerH => {
                    *size = size_of::<i16>();
                    return PackingType::Integer;
                }
                Character::UpperH => {
                    *size = size_of::<i16>();
                    return PackingType::Unsigned;
                }
                Character::LowerL => {
                    *size = size_of::<i64>();
                    return PackingType::Integer;
                }
                Character::UpperL => {
                    *size = size_of::<i64>();
                    return PackingType::Unsigned;
                }
                Character::LowerJ => {
                    *size = size_of::<i64>();
                    return PackingType::Integer;
                }
                Character::UpperJ => {
                    *size = size_of::<i64>();
                    return PackingType::Unsigned;
                }
                Character::UpperT => {
                    *size = size_of::<usize>();
                    return PackingType::Unsigned;
                }
                Character::LowerF => {
                    *size = size_of::<f32>();
                    return PackingType::Float;
                }
                Character::LowerN => {
                    *size = size_of::<f64>();
                    return PackingType::Number;
                }
                Character::LowerD => {
                    *size = size_of::<f64>();
                    return PackingType::Double;
                }
                Character::LowerI => {
                    *size = self.getnumlimit(fmt, size_of::<i32>() as i32) as usize;
                    return PackingType::Integer;
                }
                Character::UpperI => {
                    *size = self.getnumlimit(fmt, size_of::<i32>() as i32) as usize;
                    return PackingType::Unsigned;
                }
                Character::LowerS => {
                    *size = self.getnumlimit(fmt, size_of::<usize>() as i32) as usize;
                    return PackingType::String;
                }
                Character::LowerC => {
                    let csize: usize = getnum(fmt, usize::MAX);
                    if csize == usize::MAX {
                        lual_error(
                            self.header_interpreter,
                            c"missing size for format option 'c'".as_ptr(),
                            &[],
                        );
                    }
                    *size = csize;
                    return PackingType::Character;
                }
                Character::LowerZ => return PackingType::ZString,
                Character::LowerX => {
                    *size = 1;
                    return PackingType::Padding;
                }
                Character::UpperX => return PackingType::PaddingAlignment,
                Character::Space => {}
                Character::AngleLeft => {
                    self.header_islittleendian = true;
                }
                Character::AngleRight => {
                    self.header_islittleendian = false;
                }
                Character::Equal => {
                    self.header_islittleendian = NATIVE_ENDIAN.nativeendian_little != 0;
                }
                Character::Exclamation => {
                    let maxalign: i32 = 8;
                    self.header_maxmimumalignment = self.getnumlimit(fmt, maxalign);
                }
                _ => {
                    lual_error(
                        self.header_interpreter,
                        c"invalid format option '%c'".as_ptr(),
                        &[opt.into()],
                    );
                }
            }
            PackingType::NoOperator
        }
    }
    pub unsafe fn getdetails(
        &mut self,
        totalsize: usize,
        fmt: *mut *const i8,
        total_size: *mut usize,
        ntoalign: *mut usize,
    ) -> PackingType {
        unsafe {
            let opt: PackingType = self.getoption(fmt, total_size);
            let mut align: usize = *total_size;
            if opt as u32 == PackingType::PaddingAlignment as u32
                && (**fmt as i32 == Character::Null as i32
                    || self.getoption(fmt, &mut align) as u32 == PackingType::Character as u32
                    || align == 0)
            {
                lual_argerror(
                    self.header_interpreter,
                    1,
                    c"invalid next option for option X".as_ptr(),
                );
            }
            if align <= 1 || opt as u32 == PackingType::Character as u32 {
                *ntoalign = 0;
            } else {
                if align > self.header_maxmimumalignment as usize {
                    align = self.header_maxmimumalignment as usize;
                }
                if align & (align - 1) != 0 {
                    lual_argerror(
                        self.header_interpreter,
                        1,
                        c"format asks for alignment not power of 2".as_ptr(),
                    );
                }
                *ntoalign = (align - (totalsize & (align - 1))) & (align - 1);
            }
            opt
        }
    }
}

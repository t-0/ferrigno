#![allow(unpredictable_function_pointer_comparisons, unused)]
use crate::character::*;
use crate::closure::*;
use crate::f2i::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::object::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::tm::*;
use crate::tstring::*;
use crate::user::*;
use crate::utility::*;
use crate::value::*;
use libc::*;
use rlua::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value: Value,
    tag: u8,
    collectable: bool,
    pub delta: u16,
}
impl TValue {
    pub unsafe fn to_number(&self, result: *mut f64) -> bool {
        unsafe {
            let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
            if (*self).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                *result = (*self).value.integer as f64;
                return true;
            } else if l_strton(self, &mut tvalue) {
                *result = if tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                    tvalue.value.integer as f64
                } else {
                    tvalue.value.number
                };
                return true;
            } else {
                return false;
            };
        }
    }
    pub unsafe fn to_pointer(&self) -> *mut libc::c_void {
        unsafe {
            match self.get_tag_variant() {
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    ::core::mem::transmute::<CFunction, usize>(self.value.function)
                        as *mut libc::c_void
                }
                TAG_VARIANT_USER => (*(self.value.object as *mut User)).get_raw_memory_mut(),
                TAG_VARIANT_POINTER => self.value.pointer,
                _ => {
                    if self.is_collectable() {
                        self.value.object as *mut libc::c_void
                    } else {
                        null_mut()
                    }
                }
            }
        }
    }
    pub const fn new(tag: u8) -> Self {
        TValue {
            value: Value::new_object(null_mut()),
            tag: tag,
            collectable: false,
            delta: 0,
        }
    }
    pub fn is_tagtype_nil(&self) -> bool {
        self.get_tag_type() == TagType::Nil
    }
    pub fn is_tagtype_string(&self) -> bool {
        self.get_tag_type() == TagType::String
    }
    pub fn is_tagtype_numeric(&self) -> bool {
        self.get_tag_type() == TagType::Numeric
    }
    pub fn is_tagtype_boolean(&self) -> bool {
        self.get_tag_type() == TagType::Boolean
    }
    pub fn is_tagtype_closure(&self) -> bool {
        self.get_tag_type() == TagType::Closure
    }
    fn get_tag(&self) -> u8 {
        self.tag
    }
    pub fn copy_from(&mut self, other: &Self) {
        self.value = other.value;
        self.tag = other.tag;
        self.collectable = other.collectable;
    }
    pub fn get_tag_type(&self) -> TagType {
        get_tag_type(self.get_tag())
    }
    pub fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    pub fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
        self.collectable = (0 == TAG_COLLECTABLE & tag);
    }
    pub fn set_tag_variant(&mut self, tag: u8) {
        self.set_tag((tag & !TAG_COLLECTABLE) | (self.tag & TAG_COLLECTABLE));
        self.collectable = 0 != (TAG_COLLECTABLE & tag);
    }
    pub fn is_collectable(&self) -> bool {
        self.collectable
    }
    pub fn set_collectable(&mut self, value: bool) {
        if value {
            self.set_tag(set_collectable(self.get_tag()));
            self.collectable = true;
        } else {
            self.set_tag(self.get_tag_variant());
            self.collectable = false;
        }
    }
}
pub unsafe fn aux_upvalue(
    fi: *mut TValue,
    n: i32,
    value: *mut *mut TValue,
    owner: *mut *mut Object,
) -> *const i8 {
    unsafe {
        match (*fi).get_tag_variant() {
            TAG_VARIANT_CLOSURE_C => {
                let closure: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
                if !((n as u32).wrapping_sub(1 as u32) < (*closure).count_upvalues as u32) {
                    return null();
                }
                *value = &mut *((*closure).upvalues)
                    .c_tvalues
                    .as_mut_ptr()
                    .offset((n - 1) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut (*(closure as *mut Object));
                }
                return make_cstring!("");
            }
            TAG_VARIANT_CLOSURE_L => {
                let f_0: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
                let p: *mut Prototype = (*f_0).payload.l_prototype;
                if !((n as u32).wrapping_sub(1 as u32) < (*p).prototype_upvalues.get_size() as u32)
                {
                    return null();
                }
                *value = (**((*f_0).upvalues)
                    .l_upvalues
                    .as_mut_ptr()
                    .offset((n - 1) as isize))
                .v
                .p;
                if !owner.is_null() {
                    *owner = &mut (*(*((*f_0).upvalues)
                        .l_upvalues
                        .as_mut_ptr()
                        .offset((n - 1) as isize)
                        as *mut Object));
                }
                let name: *mut TString =
                    (*((*p).prototype_upvalues.vectort_pointer).offset((n - 1) as isize)).name;
                return if name.is_null() {
                    make_cstring!("(no name)")
                } else {
                    ((*name).get_contents_mut()) as *const i8
                };
            }
            _ => return null(),
        };
    }
}
pub unsafe fn luao_str2num(s: *const i8, o: *mut TValue) -> usize {
    unsafe {
        let mut i: i64 = 0;
        let mut n: f64 = 0.0;
        let mut e: *const i8 = l_str2int(s, &mut i);
        if e.is_null() {
            e = l_str2d(s, &mut n);
            if e.is_null() {
                return 0usize;
            } else {
                (*o).value.number = n;
                (*o).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
            }
        } else {
            (*o).value.integer = i;
            (*o).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        }
        return (e.offset_from(s) as i64 + 1) as usize;
    }
}
pub unsafe fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> usize {
    unsafe {
        let mut length: usize;
        if (*obj).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            length = snprintf(buffer, 44, make_cstring!("%lld"), (*obj).value.integer) as usize;
        } else {
            length = snprintf(buffer, 44, make_cstring!("%.14g"), (*obj).value.number) as usize;
            if *buffer.offset(strspn(buffer, make_cstring!("-0123456789")) as isize) as i32
                == Character::Null as i32
            {
                let fresh = length;
                length = length + 1;
                *buffer.offset(fresh as isize) = CHARACTER_PERIOD as i8;
                let fresh11 = length;
                length = length + 1;
                *buffer.offset(fresh11 as isize) = CHARACTER_0 as i8;
            }
        }
        return length;
    }
}
pub unsafe fn luao_tostring(interpreter: *mut Interpreter, obj: *mut TValue) {
    unsafe {
        let mut buffer: [i8; 44] = [0; 44];
        let length = tostringbuff(obj, buffer.as_mut_ptr());
        let io: *mut TValue = obj;
        let ts: *mut TString = luas_newlstr(interpreter, buffer.as_mut_ptr(), length as usize);
        (*io).value.object = &mut (*(ts as *mut Object));
        (*io).set_tag_variant((*ts).get_tag_variant());
        (*io).set_collectable(true);
    }
}
pub const ABSENT_KEY: TValue = { TValue::new(TAG_VARIANT_NIL_ABSENTKEY) };
pub unsafe fn arrayindex(k: i64) -> u32 {
    if (k as usize).wrapping_sub(1 as usize)
        < (if ((1 as u32)
            << (size_of::<i32>() as usize)
                .wrapping_mul(8 as usize)
                .wrapping_sub(1 as usize) as i32) as usize
            <= (!(0usize)).wrapping_div(size_of::<TValue>() as usize)
        {
            (1 as u32)
                << (size_of::<i32>() as usize)
                    .wrapping_mul(8 as usize)
                    .wrapping_sub(1 as usize) as i32
        } else {
            (!(0usize)).wrapping_div(size_of::<TValue>() as usize) as u32
        }) as usize
    {
        return k as u32;
    } else {
        return 0u32;
    };
}
pub unsafe fn binsearch(array: *const TValue, mut i: u32, mut j: u32) -> u32 {
    unsafe {
        while j.wrapping_sub(i) > 1 as u32 {
            let m: u32 = i.wrapping_add(j).wrapping_div(2 as u32);
            if ((*array.offset(m.wrapping_sub(1 as u32) as isize)).get_tag_type()) == TagType::Nil {
                j = m;
            } else {
                i = m;
            }
        }
        return i;
    }
}
pub unsafe fn l_strton(obj: *const TValue, result: *mut TValue) -> bool {
    unsafe {
        if (*obj).is_tagtype_string() {
            let st: *mut TString = &mut (*((*obj).value.object as *mut TString));
            return luao_str2num((*st).get_contents_mut(), result)
                == (*st).get_length().wrapping_add(1) as usize;
        } else {
            return false;
        };
    }
}
pub unsafe fn lessthanothers(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> i32 {
    unsafe {
        if (*l).is_tagtype_string() && (*r).is_tagtype_string() {
            return (l_strcmp(
                &mut (*((*l).value.object as *mut TString)),
                &mut (*((*r).value.object as *mut TString)),
            ) < 0) as i32;
        } else {
            return luat_callordertm(interpreter, l, r, TM_LT);
        };
    }
}
pub unsafe fn luav_lessthan(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if (*l).is_tagtype_numeric() && (*r).is_tagtype_numeric() {
            return ltnum(l, r);
        } else {
            return 0 != lessthanothers(interpreter, l, r);
        };
    }
}
pub unsafe fn lessequalothers(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if (*l).is_tagtype_string() && (*r).is_tagtype_string() {
            return l_strcmp(
                &mut (*((*l).value.object as *mut TString)),
                &mut (*((*r).value.object as *mut TString)),
            ) <= 0;
        } else {
            return 0 != luat_callordertm(interpreter, l, r, TM_LE);
        }
    }
}
pub unsafe fn luav_lessequal(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if (*l).is_tagtype_numeric() && (*r).is_tagtype_numeric() {
            return lenum(l, r);
        } else {
            return lessequalothers(interpreter, l, r);
        };
    }
}
pub unsafe fn luav_equalobj(
    interpreter: *mut Interpreter,
    t1: *const TValue,
    t2: *const TValue,
) -> bool {
    unsafe {
        let mut tm: *const TValue;
        if (*t1).get_tag_variant() != (*t2).get_tag_variant() {
            if (*t1).get_tag_type() != (*t2).get_tag_type() || !(*t1).is_tagtype_numeric() {
                return false;
            } else {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                return luav_tointegerns(t1, &mut i1, F2I::Equal) != 0
                    && luav_tointegerns(t2, &mut i2, F2I::Equal) != 0
                    && i1 == i2;
            }
        }
        match (*t1).get_tag_variant() {
            TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => {
                return true
            }
            TAG_VARIANT_NUMERIC_INTEGER => return (*t1).value.integer == (*t2).value.integer,
            TAG_VARIANT_NUMERIC_NUMBER => return (*t1).value.number == (*t2).value.number,
            TAG_VARIANT_POINTER => return (*t1).value.pointer == (*t2).value.pointer,
            TAG_VARIANT_CLOSURE_CFUNCTION => return (*t1).value.function == (*t2).value.function,
            TAG_VARIANT_STRING_SHORT => {
                return &mut (*((*t1).value.object as *mut TString)) as *mut TString
                    == &mut (*((*t2).value.object as *mut TString)) as *mut TString;
            }
            TAG_VARIANT_STRING_LONG => {
                return luas_eqlngstr(
                    &mut (*((*t1).value.object as *mut TString)),
                    &mut (*((*t2).value.object as *mut TString)),
                );
            }
            TAG_VARIANT_USER => {
                if &mut (*((*t1).value.object as *mut User)) as *mut User
                    == &mut (*((*t2).value.object as *mut User)) as *mut User
                {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).value.object as *mut User)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).value.object as *mut User)).get_metatable()).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    null()
                } else {
                    luat_gettm(
                        (*((*t1).value.object as *mut User)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).global).tm_name[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.object as *mut User)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).value.object as *mut User)).get_metatable()).flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        null()
                    } else {
                        luat_gettm(
                            (*((*t2).value.object as *mut User)).get_metatable(),
                            TM_EQ,
                            (*(*interpreter).global).tm_name[TM_EQ as usize],
                        )
                    };
                }
            }
            TAG_VARIANT_TABLE => {
                if &mut (*((*t1).value.object as *mut Table)) as *mut Table
                    == &mut (*((*t2).value.object as *mut Table)) as *mut Table
                {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).value.object as *mut Table)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).value.object as *mut Table)).get_metatable()).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    null()
                } else {
                    luat_gettm(
                        (*((*t1).value.object as *mut Table)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).global).tm_name[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.object as *mut Table)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).value.object as *mut Table)).get_metatable()).flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        null()
                    } else {
                        luat_gettm(
                            (*((*t2).value.object as *mut Table)).get_metatable(),
                            TM_EQ,
                            (*(*interpreter).global).tm_name[TM_EQ as usize],
                        )
                    };
                }
            }
            _ => return (*t1).value.object == (*t2).value.object,
        }
        if tm.is_null() {
            return false;
        } else {
            luat_calltmres(interpreter, tm, t1, t2, (*interpreter).top.stkidrel_pointer);
            return !((*(*interpreter).top.stkidrel_pointer).get_tag_variant()
                == TAG_VARIANT_BOOLEAN_FALSE
                || (*(*interpreter).top.stkidrel_pointer).is_tagtype_nil());
        };
    }
}
pub unsafe fn luav_objlen(interpreter: *mut Interpreter, ra: *mut TValue, rb: *const TValue) {
    unsafe {
        let tm: *const TValue;
        match (*rb).get_tag_variant() {
            TAG_VARIANT_TABLE => {
                let h: *mut Table = &mut (*((*rb).value.object as *mut Table));
                tm = if ((*h).get_metatable()).is_null() {
                    null()
                } else if (*(*h).get_metatable()).flags as u32 & (1 as u32) << TM_LEN as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*h).get_metatable(),
                        TM_LEN,
                        (*(*interpreter).global).tm_name[TM_LEN as usize],
                    )
                };
                if tm.is_null() {
                    let io: *mut TValue = &mut (*ra);
                    (*io).value.integer = luah_getn(h) as i64;
                    (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    return;
                }
            }
            TAG_VARIANT_STRING_SHORT => {
                let io_0: *mut TValue = &mut (*ra);
                (*io_0).value.integer = (*((*rb).value.object as *mut TString)).get_length() as i64;
                (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                return;
            }
            TAG_VARIANT_STRING_LONG => {
                let io_1: *mut TValue = &mut (*ra);
                (*io_1).value.integer = (*((*rb).value.object as *mut TString)).get_length() as i64;
                (*io_1).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                return;
            }
            _ => {
                tm = luat_gettmbyobj(interpreter, rb, TM_LEN);
                if ((*tm).get_tag_type()) == TagType::Nil {
                    luag_typeerror(interpreter, rb, make_cstring!("get length of"));
                }
            }
        }
        luat_calltmres(interpreter, tm, rb, rb, ra);
    }
}

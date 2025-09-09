#![allow(unpredictable_function_pointer_comparisons)]
use crate::character::*;
use crate::tag::*;
use crate::value::*;
use crate::object::*;
use crate::tstring::*;
use crate::closure::*;
use crate::stackvalue::*;
use crate::interpreter::*;
use crate::prototype::*;
use crate::tm::*;
use crate::table::*;
use crate::user::*;
use crate::f2i::*;
use crate::utility::*;
use libc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value: Value,
    tag: u8,
    collectable: bool,
}
impl TValue {
    pub const fn new(tag: u8) -> Self {
        TValue {
            value: Value::new(),
            tag: tag,
            collectable: false,
        }
    }
    pub fn get_tag2(&self) -> u8 {
        self.tag
    }
    pub fn set_tag(&mut self, tag: u8) {
        if 0 == TAG_COLLECTABLE & tag {
            self.tag = tag;
            self.collectable = false;
        } else {
            self.tag = tag;
            self.collectable = true;
        }
    }
    pub fn is_collectable(&self) -> bool {
        self.collectable
    }
    pub fn set_collectable(&mut self, value: bool) {
        if value {
            self.set_tag(set_collectable(self.get_tag2()));
            self.collectable = true;
        } else {
            self.set_tag(self.get_tag2() & !TAG_COLLECTABLE);
            self.collectable = false;
        }
    }
    pub fn get_tag_type(&self) -> TagType {
        get_tag_type(self.get_tag2())
    }
    pub fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag2())
    }
}
pub unsafe extern "C" fn aux_upvalue(
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
                    return std::ptr::null();
                }
                *value = &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut (*(closure as *mut Object));
                }
                return b"\0" as *const u8 as *const i8;
            }
            TAG_VARIANT_CLOSURE_L => {
                let f_0: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
                let p: *mut Prototype = (*f_0).payload.l_prototype;
                if !((n as u32).wrapping_sub(1 as u32) < (*p).size_upvalues as u32) {
                    return std::ptr::null();
                }
                *value = (**((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize))
                    .v
                    .p;
                if !owner.is_null() {
                    *owner = &mut (*(*((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize)
                        as *mut Object));
                }
                let name: *mut TString = (*((*p).upvalues).offset((n - 1) as isize)).name;
                return if name.is_null() {
                    b"(no name)\0" as *const u8 as *const i8
                } else {
                    ((*name).get_contents_mut()) as *const i8
                };
            }
            _ => return std::ptr::null(),
        };
    }
}
pub unsafe extern "C" fn luao_str2num(s: *const i8, o: *mut TValue) -> u64 {
    unsafe {
        let mut i: i64 = 0;
        let mut n: f64 = 0.0;
        let mut e: *const i8 = l_str2int(s, &mut i);
        if e.is_null() {
            e = l_str2d(s, &mut n);
            if e.is_null() {
                return 0u64;
            } else {
                (*o).value.number = n;
                (*o).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            }
        } else {
            (*o).value.integer = i;
            (*o).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        }
        return (e.offset_from(s) as i64 + 1) as u64;
    }
}
pub unsafe extern "C" fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> u64 {
    unsafe {
        let mut length: u64;
        if (*obj).get_tag2() == TAG_VARIANT_NUMERIC_INTEGER {
            length = snprintf(
                buffer,
                44,
                b"%lld\0" as *const u8 as *const i8,
                (*obj).value.integer,
            ) as u64;
        } else {
            length = snprintf(
                buffer,
                44,
                b"%.14g\0" as *const u8 as *const i8,
                (*obj).value.number,
            ) as u64;
            if *buffer.offset(strspn(buffer, b"-0123456789\0" as *const u8 as *const i8) as isize)
                as i32
                == CHARACTER_NUL as i32
            {
                let fresh10 = length;
                length = length + 1;
                *buffer.offset(fresh10 as isize) = CHARACTER_PERIOD as i8;
                let fresh11 = length;
                length = length + 1;
                *buffer.offset(fresh11 as isize) = CHARACTER_0 as i8;
            }
        }
        return length;
    }
}
pub unsafe extern "C" fn luao_tostring(interpreter: *mut Interpreter, obj: *mut TValue) {
    unsafe {
        let mut buffer: [i8; 44] = [0; 44];
        let length = tostringbuff(obj, buffer.as_mut_ptr());
        let io: *mut TValue = obj;
        let x_: *mut TString = luas_newlstr(interpreter, buffer.as_mut_ptr(), length);
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag((*x_).get_tag());
        (*io).set_collectable(true);
    }
}
pub const ABSENT_KEY: TValue = {
    TValue::new(TAG_VARIANT_NIL_ABSENTKEY)
};
pub unsafe extern "C" fn arrayindex(k: i64) -> u32 {
    if (k as u64).wrapping_sub(1 as u64)
        < (if ((1 as u32)
            << (::core::mem::size_of::<i32>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(1 as u64) as i32) as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64)
        {
            (1 as u32)
                << (::core::mem::size_of::<i32>() as u64)
                    .wrapping_mul(8 as u64)
                    .wrapping_sub(1 as u64) as i32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64) as u32
        }) as u64
    {
        return k as u32;
    } else {
        return 0u32;
    };
}
pub unsafe extern "C" fn binsearch(array: *const TValue, mut i: u32, mut j: u32) -> u32 {
    unsafe {
        while j.wrapping_sub(i) > 1 as u32 {
            let m: u32 = i.wrapping_add(j).wrapping_div(2 as u32);
            if get_tag_type((*array.offset(m.wrapping_sub(1 as u32) as isize)).get_tag2())
                == TagType::Nil
            {
                j = m;
            } else {
                i = m;
            }
        }
        return i;
    }
}
pub unsafe extern "C" fn l_strton(obj: *const TValue, result: *mut TValue) -> i32 {
    unsafe {
        if !(get_tag_type((*obj).get_tag2()) == TagType::String) {
            return 0;
        } else {
            let st: *mut TString = &mut (*((*obj).value.object as *mut TString));
            return (luao_str2num((*st).get_contents_mut(), result)
                == (*st).get_length().wrapping_add(1 as u64)) as i32;
        };
    }
}
pub unsafe extern "C" fn luav_tonumber_(obj: *const TValue, n: *mut f64) -> bool {
    unsafe {
        let mut v: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        if (*obj).get_tag2() == TAG_VARIANT_NUMERIC_INTEGER {
            *n = (*obj).value.integer as f64;
            return true;
        } else if l_strton(obj, &mut v) != 0 {
            *n = if v.get_tag2() == TAG_VARIANT_NUMERIC_INTEGER {
                v.value.integer as f64
            } else {
                v.value.number
            };
            return true;
        } else {
            return false;
        };
    }
}
pub unsafe extern "C" fn lessthanothers(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> i32 {
    unsafe {
        if get_tag_type((*l).get_tag2()) == TagType::String
            && get_tag_type((*r).get_tag2()) == TagType::String
        {
            return (l_strcmp(
                &mut (*((*l).value.object as *mut TString)),
                &mut (*((*r).value.object as *mut TString)),
            ) < 0) as i32;
        } else {
            return luat_callordertm(interpreter, l, r, TM_LT);
        };
    }
}
pub unsafe extern "C" fn luav_lessthan(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if get_tag_type((*l).get_tag2()) == TagType::Numeric
            && get_tag_type((*r).get_tag2()) == TagType::Numeric
        {
            return ltnum(l, r);
        } else {
            return 0 != lessthanothers(interpreter, l, r);
        };
    }
}
pub unsafe extern "C" fn lessequalothers(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if get_tag_type((*l).get_tag2()) == TagType::String
            && get_tag_type((*r).get_tag2()) == TagType::String
        {
            return l_strcmp(
                &mut (*((*l).value.object as *mut TString)),
                &mut (*((*r).value.object as *mut TString)),
            ) <= 0;
        } else {
            return 0 != luat_callordertm(interpreter, l, r, TM_LE);
        }
    }
}
pub unsafe extern "C" fn luav_lessequal(
    interpreter: *mut Interpreter,
    l: *const TValue,
    r: *const TValue,
) -> bool {
    unsafe {
        if get_tag_type((*l).get_tag2()) == TagType::Numeric
            && get_tag_type((*r).get_tag2()) == TagType::Numeric
        {
            return lenum(l, r);
        } else {
            return lessequalothers(interpreter, l, r);
        };
    }
}
pub unsafe extern "C" fn luav_equalobj(
    interpreter: *mut Interpreter,
    t1: *const TValue,
    t2: *const TValue,
) -> bool {
    unsafe {
        let mut tm: *const TValue;
        if (*t1).get_tag_variant() != (*t2).get_tag_variant() {
            if (*t1).get_tag_type() != (*t2).get_tag_type()
                || (*t1).get_tag_type() != TagType::Numeric
            {
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
            TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => return true,
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
                    std::ptr::null()
                } else if (*(*((*t1).value.object as *mut User)).get_metatable()).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*((*t1).value.object as *mut User)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).global).tm_name[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.object as *mut User)).get_metatable()).is_null() {
                        std::ptr::null()
                    } else if (*(*((*t2).value.object as *mut User)).get_metatable()).flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        std::ptr::null()
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
                    std::ptr::null()
                } else if (*(*((*t1).value.object as *mut Table)).get_metatable()).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*((*t1).value.object as *mut Table)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).global).tm_name[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.object as *mut Table)).get_metatable()).is_null() {
                        std::ptr::null()
                    } else if (*(*((*t2).value.object as *mut Table)).get_metatable()).flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        std::ptr::null()
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
            return !((*(*interpreter).top.stkidrel_pointer).tvalue.get_tag2() == TAG_VARIANT_BOOLEAN_FALSE
                || get_tag_type((*(*interpreter).top.stkidrel_pointer).tvalue.get_tag2()) == TagType::Nil);
        };
    }
}
pub unsafe extern "C" fn luav_objlen(interpreter: *mut Interpreter, ra: StackValuePointer, rb: *const TValue) {
    unsafe {
        let tm: *const TValue;
        match (*rb).get_tag_variant() {
            TAG_VARIANT_TABLE => {
                let h: *mut Table = &mut (*((*rb).value.object as *mut Table));
                tm = if ((*h).get_metatable()).is_null() {
                    std::ptr::null()
                } else if (*(*h).get_metatable()).flags as u32 & (1 as u32) << TM_LEN as i32 != 0 {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*h).get_metatable(),
                        TM_LEN,
                        (*(*interpreter).global).tm_name[TM_LEN as usize],
                    )
                };
                if tm.is_null() {
                    let io: *mut TValue = &mut (*ra).tvalue;
                    (*io).value.integer = luah_getn(h) as i64;
                    (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    return;
                }
            }
            TAG_VARIANT_STRING_SHORT => {
                let io_0: *mut TValue = &mut (*ra).tvalue;
                (*io_0).value.integer = (*((*rb).value.object as *mut TString)).get_length() as i64;
                (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                return;
            }
            TAG_VARIANT_STRING_LONG => {
                let io_1: *mut TValue = &mut (*ra).tvalue;
                (*io_1).value.integer = (*((*rb).value.object as *mut TString)).get_length() as i64;
                (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                return;
            }
            _ => {
                tm = luat_gettmbyobj(interpreter, rb, TM_LEN);
                if ((get_tag_type((*tm).get_tag2()) == TagType::Nil) as i32 != 0) as i64 != 0
                {
                    luag_typeerror(interpreter, rb, b"get length of\0" as *const u8 as *const i8);
                }
            }
        }
        luat_calltmres(interpreter, tm, rb, rb, ra);
    }
}

#![allow(unpredictable_function_pointer_comparisons, unused)]
use crate::c::*;
use crate::character::*;
use crate::closure::*;
use crate::f2i::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::object::*;
use crate::object::*;
use crate::prototype::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tagtype::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tobjectwithmetatable::*;
use crate::tstring::*;
use crate::user::*;
use crate::utility::*;
use crate::value::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub tvalue_value: Value,
    tvalue_tagvariant: TagVariant,
    tvalue_collectable: bool,
    pub tvalue_delta: u16,
}
impl TValue {
    pub unsafe fn from_string_to_number(&mut self, obj: *const TValue) -> bool {
        unsafe {
            if (*obj).get_tagvariant().to_tag_type().is_string() {
                let tstring: *mut TString = &mut (*((*obj).tvalue_value.value_object as *mut TString));
                return luao_str2num((*tstring).get_contents_mut(), self) == (*tstring).get_length().wrapping_add(1) as usize;
            } else {
                return false;
            };
        }
    }
    pub unsafe fn to_number(&self, result: *mut f64) -> bool {
        unsafe {
            let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
            if (*self).get_tagvariant() == TagVariant::NumericInteger {
                *result = (*self).tvalue_value.value_integer as f64;
                return true;
            } else if tvalue.from_string_to_number(self) {
                *result = if tvalue.get_tagvariant() == TagVariant::NumericInteger {
                    tvalue.tvalue_value.value_integer as f64
                } else {
                    tvalue.tvalue_value.value_number
                };
                return true;
            } else {
                return false;
            };
        }
    }
    pub unsafe fn to_pointer(&self) -> *mut libc::c_void {
        unsafe {
            match self.get_tagvariant() {
                | TagVariant::ClosureCFunction => {
                    ::core::mem::transmute::<CFunction, usize>(self.tvalue_value.value_function) as *mut libc::c_void
                },
                | TagVariant::User => (*(self.tvalue_value.value_object as *mut User)).get_raw_memory_mut(),
                | TagVariant::Pointer => self.tvalue_value.value_pointer,
                | _ => {
                    if self.is_collectable() {
                        self.tvalue_value.value_object as *mut libc::c_void
                    } else {
                        null_mut()
                    }
                },
            }
        }
    }
    pub const fn new(tagvariant: TagVariant) -> Self {
        TValue {
            tvalue_value: Value::new_object(null_mut()),
            tvalue_tagvariant: tagvariant,
            tvalue_collectable: false,
            tvalue_delta: 0,
        }
    }
    pub fn copy_from(&mut self, other: &Self) {
        self.tvalue_value = other.tvalue_value;
        self.tvalue_tagvariant = other.tvalue_tagvariant;
        self.tvalue_collectable = other.tvalue_collectable;
    }
    pub fn get_tagvariant(&self) -> TagVariant {
        self.tvalue_tagvariant
    }
    pub fn tvalue_set_tag_variant(&mut self, tagvariant: TagVariant) {
        self.tvalue_tagvariant = tagvariant;
        self.tvalue_collectable = false;
    }
    pub fn is_collectable(&self) -> bool {
        self.tvalue_collectable
    }
    pub fn set_collectable(&mut self, value: bool) {
        self.tvalue_collectable = value;
    }
    pub unsafe fn from_interpreter_to_string(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let mut buffer: [i8; 44] = [0; 44];
            let length = tostringbuff(self, buffer.as_mut_ptr());
            let tstring: *mut TString = luas_newlstr(interpreter, buffer.as_mut_ptr(), length as usize);
            self.tvalue_value.value_object = &mut (*(tstring as *mut Object));
            self.tvalue_set_tag_variant((*tstring).get_tagvariant());
            self.set_collectable(true);
        }
    }
}
pub unsafe fn aux_upvalue(fi: *mut TValue, n: i32, value: *mut *mut TValue, owner: *mut *mut Object) -> *const i8 {
    unsafe {
        match (*fi).get_tagvariant() {
            | TagVariant::ClosureC => {
                let closure: *mut Closure = &mut (*((*fi).tvalue_value.value_object as *mut Closure));
                if n > (*closure).count_upvalues as i32 {
                    return null();
                }
                *value = &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut (*(closure as *mut Object));
                }
                return c"".as_ptr();
            },
            | TagVariant::ClosureL => {
                let f_0: *mut Closure = &mut (*((*fi).tvalue_value.value_object as *mut Closure));
                let p: *mut Prototype = (*f_0).payload.l_prototype;
                if !((n as u32).wrapping_sub(1 as u32) < (*p).prototype_upvalues.get_size() as u32) {
                    return null();
                }
                *value = (**((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize))
                    .upvalue_v
                    .upvaluea_p;
                if !owner.is_null() {
                    *owner = &mut (*(*((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize) as *mut Object));
                }
                let name: *mut TString =
                    (*((*p).prototype_upvalues.vectort_pointer).offset((n - 1) as isize)).upvaluedescription_name;
                return if name.is_null() {
                    c"(no name)".as_ptr()
                } else {
                    ((*name).get_contents_mut()) as *const i8
                };
            },
            | _ => return null(),
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
                (*o).tvalue_value.value_number = n;
                (*o).tvalue_set_tag_variant(TagVariant::NumericNumber);
            }
        } else {
            (*o).tvalue_value.value_integer = i;
            (*o).tvalue_set_tag_variant(TagVariant::NumericInteger);
        }
        return (e.offset_from(s) as i64 + 1) as usize;
    }
}
pub unsafe fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> usize {
    unsafe {
        let mut length: usize;
        if (*obj).get_tagvariant() == TagVariant::NumericInteger {
            length = libc::snprintf(buffer, 44, c"%lld".as_ptr(), (*obj).tvalue_value.value_integer) as usize;
        } else {
            length = libc::snprintf(buffer, 44, c"%.14g".as_ptr(), (*obj).tvalue_value.value_number) as usize;
            if *buffer.offset(libc::strspn(buffer, c"-0123456789".as_ptr()) as isize) as i32 == Character::Null as i32 {
                let fresh = length;
                length = length + 1;
                *buffer.offset(fresh as isize) = Character::Period as i8;
                let fresh11 = length;
                length = length + 1;
                *buffer.offset(fresh11 as isize) = Character::Digit0 as i8;
            }
        }
        return length;
    }
}
pub const ABSENT_KEY: TValue = { TValue::new(TagVariant::NilAbsentKey) };
pub unsafe fn arrayindex(k: i64) -> u32 {
    if (k as usize).wrapping_sub(1 as usize)
        < (if ((1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32) as usize
            <= (((!0usize) / size_of::<TValue>()) as usize)
        {
            (1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32
        } else {
            ((!0usize) / size_of::<TValue>()) as u32
        }) as usize
    {
        return k as u32;
    } else {
        return 0;
    };
}
pub unsafe fn binsearch(array: *const TValue, mut i: u32, mut j: u32) -> u32 {
    unsafe {
        while j.wrapping_sub(i) > 1 {
            let m = i.wrapping_add(j) / 2;
            if (*array.offset((m - 1) as isize)).get_tagvariant().to_tag_type() == TagType::Nil {
                j = m;
            } else {
                i = m;
            }
        }
        return i;
    }
}
pub unsafe fn lessthanothers(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> i32 {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_string() && (*r).get_tagvariant().to_tag_type().is_string() {
            return (l_strcmp(
                &mut (*((*l).tvalue_value.value_object as *mut TString)),
                &mut (*((*r).tvalue_value.value_object as *mut TString)),
            ) < 0) as i32;
        } else {
            return luat_callordertm(interpreter, l, r, TM_LT);
        };
    }
}
pub unsafe fn luav_lessthan(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_numeric() && (*r).get_tagvariant().to_tag_type().is_numeric() {
            return ltnum(l, r);
        } else {
            return 0 != lessthanothers(interpreter, l, r);
        };
    }
}
pub unsafe fn lessequalothers(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_string() && (*r).get_tagvariant().to_tag_type().is_string() {
            return l_strcmp(
                &mut (*((*l).tvalue_value.value_object as *mut TString)),
                &mut (*((*r).tvalue_value.value_object as *mut TString)),
            ) <= 0;
        } else {
            return 0 != luat_callordertm(interpreter, l, r, TM_LE);
        }
    }
}
pub unsafe fn luav_lessequal(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_numeric() && (*r).get_tagvariant().to_tag_type().is_numeric() {
            return lenum(l, r);
        } else {
            return lessequalothers(interpreter, l, r);
        };
    }
}
pub unsafe fn luav_equalobj(interpreter: *mut Interpreter, t1: *const TValue, t2: *const TValue) -> bool {
    unsafe {
        let mut tm: *const TValue;
        if (*t1).get_tagvariant() != (*t2).get_tagvariant() {
            if (*t1).get_tagvariant().to_tag_type() != (*t2).get_tagvariant().to_tag_type() || !(*t1).get_tagvariant().to_tag_type().is_numeric() {
                return false;
            } else {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                return F2I::Equal.convert_tv_i64(t1, &mut i1) != 0
                    && F2I::Equal.convert_tv_i64(t2, &mut i2) != 0
                    && i1 == i2;
            }
        }
        match (*t1).get_tagvariant() {
            | TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => return true,
            | TagVariant::NumericInteger => return (*t1).tvalue_value.value_integer == (*t2).tvalue_value.value_integer,
            | TagVariant::NumericNumber => return (*t1).tvalue_value.value_number == (*t2).tvalue_value.value_number,
            | TagVariant::Pointer => return (*t1).tvalue_value.value_pointer == (*t2).tvalue_value.value_pointer,
            | TagVariant::ClosureCFunction => return (*t1).tvalue_value.value_function == (*t2).tvalue_value.value_function,
            | TagVariant::StringShort => {
                return &mut (*((*t1).tvalue_value.value_object as *mut TString)) as *mut TString
                    == &mut (*((*t2).tvalue_value.value_object as *mut TString)) as *mut TString;
            },
            | TagVariant::StringLong => {
                return luas_eqlngstr(
                    &mut (*((*t1).tvalue_value.value_object as *mut TString)),
                    &mut (*((*t2).tvalue_value.value_object as *mut TString)),
                );
            },
            | TagVariant::User => {
                if &mut (*((*t1).tvalue_value.value_object as *mut User)) as *mut User
                    == &mut (*((*t2).tvalue_value.value_object as *mut User)) as *mut User
                {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).tvalue_value.value_object as *mut User)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).tvalue_value.value_object as *mut User)).get_metatable()).table_flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    null()
                } else {
                    luat_gettm(
                        (*((*t1).tvalue_value.value_object as *mut User)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).interpreter_global).global_tmname[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).tvalue_value.value_object as *mut User)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).tvalue_value.value_object as *mut User)).get_metatable()).table_flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        null()
                    } else {
                        luat_gettm(
                            (*((*t2).tvalue_value.value_object as *mut User)).get_metatable(),
                            TM_EQ,
                            (*(*interpreter).interpreter_global).global_tmname[TM_EQ as usize],
                        )
                    };
                }
            },
            | TagVariant::Table => {
                if &mut (*((*t1).tvalue_value.value_object as *mut Table)) as *mut Table
                    == &mut (*((*t2).tvalue_value.value_object as *mut Table)) as *mut Table
                {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).tvalue_value.value_object as *mut Table)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).tvalue_value.value_object as *mut Table)).get_metatable()).table_flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    null()
                } else {
                    luat_gettm(
                        (*((*t1).tvalue_value.value_object as *mut Table)).get_metatable(),
                        TM_EQ,
                        (*(*interpreter).interpreter_global).global_tmname[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*((*t2).tvalue_value.value_object as *mut Table)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).tvalue_value.value_object as *mut Table)).get_metatable()).table_flags as u32
                        & (1 as u32) << TM_EQ as i32
                        != 0
                    {
                        null()
                    } else {
                        luat_gettm(
                            (*((*t2).tvalue_value.value_object as *mut Table)).get_metatable(),
                            TM_EQ,
                            (*(*interpreter).interpreter_global).global_tmname[TM_EQ as usize],
                        )
                    };
                }
            },
            | _ => return (*t1).tvalue_value.value_object == (*t2).tvalue_value.value_object,
        }
        if tm.is_null() {
            return false;
        } else {
            luat_calltmres(interpreter, tm, t1, t2, (*interpreter).interpreter_top.stkidrel_pointer);
            return !((*(*interpreter).interpreter_top.stkidrel_pointer).get_tagvariant() == TagVariant::BooleanFalse
                || (*(*interpreter).interpreter_top.stkidrel_pointer).get_tagvariant().to_tag_type().is_nil());
        };
    }
}
pub unsafe fn luav_objlen(interpreter: *mut Interpreter, ra: *mut TValue, rb: *const TValue) {
    unsafe {
        let tvalue: *const TValue;
        match (*rb).get_tagvariant() {
            | TagVariant::Table => {
                let table: *mut Table = &mut (*((*rb).tvalue_value.value_object as *mut Table));
                tvalue = if ((*table).get_metatable()).is_null() {
                    null()
                } else if (*(*table).get_metatable()).table_flags as u32 & (1 as u32) << TM_LEN as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*table).get_metatable(),
                        TM_LEN,
                        (*(*interpreter).interpreter_global).global_tmname[TM_LEN as usize],
                    )
                };
                if tvalue.is_null() {
                    let io: *mut TValue = &mut (*ra);
                    (*io).tvalue_value.value_integer = luah_getn(table) as i64;
                    (*io).tvalue_set_tag_variant(TagVariant::NumericInteger);
                    return;
                }
            },
            | TagVariant::StringShort | TagVariant::StringLong => {
                let io: *mut TValue = &mut (*ra);
                (*io).tvalue_value.value_integer = (*((*rb).tvalue_value.value_object as *mut TString)).get_length() as i64;
                (*io).tvalue_set_tag_variant(TagVariant::NumericInteger);
                return;
            },
            | _ => {
                tvalue = luat_gettmbyobj(interpreter, rb, TM_LEN);
                if (*tvalue).get_tagvariant().to_tag_type() == TagType::Nil {
                    luag_typeerror(interpreter, rb, c"get length of".as_ptr());
                }
            },
        }
        luat_calltmres(interpreter, tvalue, rb, rb, ra);
    }
}

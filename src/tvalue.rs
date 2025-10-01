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
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value: Value,
    tag: TagVariant,
    collectable: bool,
    pub delta: u16,
}
impl TValue {
    pub unsafe fn from_string_to_number(&mut self, obj: *const TValue) -> bool {
        unsafe {
            if (*obj).is_tagtype_string() {
                let tstring: *mut TString = &mut (*((*obj).value.value_object as *mut TString));
                return luao_str2num((*tstring).get_contents_mut(), self) == (*tstring).get_length().wrapping_add(1) as usize;
            } else {
                return false;
            };
        }
    }
    pub unsafe fn to_number(&self, result: *mut f64) -> bool {
        unsafe {
            let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
            if (*self).get_tag_variant() == TagVariant::NumericInteger {
                *result = (*self).value.value_integer as f64;
                return true;
            } else if tvalue.from_string_to_number(self) {
                *result = if tvalue.get_tag_variant() == TagVariant::NumericInteger { tvalue.value.value_integer as f64 } else { tvalue.value.value_number };
                return true;
            } else {
                return false;
            };
        }
    }
    pub unsafe fn to_pointer(&self) -> *mut libc::c_void {
        unsafe {
            match self.get_tag_variant() {
                TagVariant::ClosureCFunction => ::core::mem::transmute::<CFunction, usize>(self.value.value_function) as *mut libc::c_void,
                TagVariant::User => (*(self.value.value_object as *mut User)).get_raw_memory_mut(),
                TagVariant::Pointer => self.value.value_pointer,
                _ => {
                    if self.is_collectable() {
                        self.value.value_object as *mut libc::c_void
                    } else {
                        null_mut()
                    }
                },
            }
        }
    }
    pub const fn new(tagvariant: TagVariant) -> Self {
        TValue { value: Value::new_object(null_mut()), tag: tagvariant, collectable: false, delta: 0 }
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
    pub fn copy_from(&mut self, other: &Self) {
        self.value = other.value;
        self.tag = other.tag;
        self.collectable = other.collectable;
    }
    pub fn get_tag_type(&self) -> TagType {
        get_tag_type(self.get_tag_variant())
    }
    pub fn get_tag_variant(&self) -> TagVariant {
        self.tag
    }
    pub fn set_tag_variant(&mut self, tagvariant: TagVariant) {
        self.tag = tagvariant;
        self.collectable = false;
    }
    pub fn is_collectable(&self) -> bool {
        self.collectable
    }
    pub fn set_collectable(&mut self, value: bool) {
        self.collectable = value;
    }
    pub unsafe fn from_interpreter_to_string(& mut self, interpreter: *mut Interpreter) {
        unsafe {
            let mut buffer: [i8; 44] = [0; 44];
            let length = tostringbuff(self, buffer.as_mut_ptr());
            let tstring: *mut TString = luas_newlstr(interpreter, buffer.as_mut_ptr(), length as usize);
            self.value.value_object = &mut (*(tstring as *mut Object));
            self.set_tag_variant((*tstring).get_tag_variant());
            self.set_collectable(true);
        }
    }
}
pub unsafe fn aux_upvalue(fi: *mut TValue, n: i32, value: *mut *mut TValue, owner: *mut *mut Object) -> *const i8 {
    unsafe {
        match (*fi).get_tag_variant() {
            TagVariant::ClosureC => {
                let closure: *mut Closure = &mut (*((*fi).value.value_object as *mut Closure));
                if n > (*closure).count_upvalues as i32 {
                    return null();
                }
                *value = &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut (*(closure as *mut Object));
                }
                return c"".as_ptr();
            },
            TagVariant::ClosureL => {
                let f_0: *mut Closure = &mut (*((*fi).value.value_object as *mut Closure));
                let p: *mut Prototype = (*f_0).payload.l_prototype;
                if !((n as u32).wrapping_sub(1 as u32) < (*p).prototype_upvalues.get_size() as u32) {
                    return null();
                }
                *value = (**((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize)).v.p;
                if !owner.is_null() {
                    *owner = &mut (*(*((*f_0).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize) as *mut Object));
                }
                let name: *mut TString = (*((*p).prototype_upvalues.vectort_pointer).offset((n - 1) as isize)).upvaluedescription_name;
                return if name.is_null() { c"(no name)".as_ptr() } else { ((*name).get_contents_mut()) as *const i8 };
            },
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
                (*o).value.value_number = n;
                (*o).set_tag_variant(TagVariant::NumericNumber);
            }
        } else {
            (*o).value.value_integer = i;
            (*o).set_tag_variant(TagVariant::NumericInteger);
        }
        return (e.offset_from(s) as i64 + 1) as usize;
    }
}
pub unsafe fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> usize {
    unsafe {
        let mut length: usize;
        if (*obj).get_tag_variant() == TagVariant::NumericInteger {
            length = snprintf(buffer, 44, c"%lld".as_ptr(), (*obj).value.value_integer) as usize;
        } else {
            length = snprintf(buffer, 44, c"%.14g".as_ptr(), (*obj).value.value_number) as usize;
            if *buffer.offset(strspn(buffer, c"-0123456789".as_ptr()) as isize) as i32 == Character::Null as i32 {
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
        < (if ((1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32) as usize <= (((!0usize) / size_of::<TValue>()) as usize) {
            (1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32
        } else {
            ((!0usize) / size_of::<TValue>()) as u32
        }) as usize
    {
        return k as u32;
    } else {
        return 0u32;
    };
}
pub unsafe fn binsearch(array: *const TValue, mut i: u32, mut j: u32) -> u32 {
    unsafe {
        while j.wrapping_sub(i) > 1 {
            let m = i.wrapping_add(j) / 2;
            if ((*array.offset((m - 1) as isize)).get_tag_type()) == TagType::Nil {
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
        if (*l).is_tagtype_string() && (*r).is_tagtype_string() {
            return (l_strcmp(&mut (*((*l).value.value_object as *mut TString)), &mut (*((*r).value.value_object as *mut TString))) < 0) as i32;
        } else {
            return luat_callordertm(interpreter, l, r, TM_LT);
        };
    }
}
pub unsafe fn luav_lessthan(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).is_tagtype_numeric() && (*r).is_tagtype_numeric() {
            return ltnum(l, r);
        } else {
            return 0 != lessthanothers(interpreter, l, r);
        };
    }
}
pub unsafe fn lessequalothers(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).is_tagtype_string() && (*r).is_tagtype_string() {
            return l_strcmp(&mut (*((*l).value.value_object as *mut TString)), &mut (*((*r).value.value_object as *mut TString))) <= 0;
        } else {
            return 0 != luat_callordertm(interpreter, l, r, TM_LE);
        }
    }
}
pub unsafe fn luav_lessequal(interpreter: *mut Interpreter, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).is_tagtype_numeric() && (*r).is_tagtype_numeric() {
            return lenum(l, r);
        } else {
            return lessequalothers(interpreter, l, r);
        };
    }
}
pub unsafe fn luav_equalobj(interpreter: *mut Interpreter, t1: *const TValue, t2: *const TValue) -> bool {
    unsafe {
        let mut tm: *const TValue;
        if (*t1).get_tag_variant() != (*t2).get_tag_variant() {
            if (*t1).get_tag_type() != (*t2).get_tag_type() || !(*t1).is_tagtype_numeric() {
                return false;
            } else {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                return luav_tointegerns(t1, &mut i1, F2I::Equal) != 0 && luav_tointegerns(t2, &mut i2, F2I::Equal) != 0 && i1 == i2;
            }
        }
        match (*t1).get_tag_variant() {
            TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => return true,
            TagVariant::NumericInteger => return (*t1).value.value_integer == (*t2).value.value_integer,
            TagVariant::NumericNumber => return (*t1).value.value_number == (*t2).value.value_number,
            TagVariant::Pointer => return (*t1).value.value_pointer == (*t2).value.value_pointer,
            TagVariant::ClosureCFunction => return (*t1).value.value_function == (*t2).value.value_function,
            TagVariant::StringShort => {
                return &mut (*((*t1).value.value_object as *mut TString)) as *mut TString == &mut (*((*t2).value.value_object as *mut TString)) as *mut TString;
            },
            TagVariant::StringLong => {
                return luas_eqlngstr(&mut (*((*t1).value.value_object as *mut TString)), &mut (*((*t2).value.value_object as *mut TString)));
            },
            TagVariant::User => {
                if &mut (*((*t1).value.value_object as *mut User)) as *mut User == &mut (*((*t2).value.value_object as *mut User)) as *mut User {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).value.value_object as *mut User)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).value.value_object as *mut User)).get_metatable()).flags as u32 & (1 as u32) << TM_EQ as i32 != 0 {
                    null()
                } else {
                    luat_gettm((*((*t1).value.value_object as *mut User)).get_metatable(), TM_EQ, (*(*interpreter).global).tm_name[TM_EQ as usize])
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.value_object as *mut User)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).value.value_object as *mut User)).get_metatable()).flags as u32 & (1 as u32) << TM_EQ as i32 != 0 {
                        null()
                    } else {
                        luat_gettm((*((*t2).value.value_object as *mut User)).get_metatable(), TM_EQ, (*(*interpreter).global).tm_name[TM_EQ as usize])
                    };
                }
            },
            TagVariant::Table => {
                if &mut (*((*t1).value.value_object as *mut Table)) as *mut Table == &mut (*((*t2).value.value_object as *mut Table)) as *mut Table {
                    return true;
                } else if interpreter.is_null() {
                    return false;
                }
                tm = if ((*((*t1).value.value_object as *mut Table)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t1).value.value_object as *mut Table)).get_metatable()).flags as u32 & (1 as u32) << TM_EQ as i32 != 0 {
                    null()
                } else {
                    luat_gettm((*((*t1).value.value_object as *mut Table)).get_metatable(), TM_EQ, (*(*interpreter).global).tm_name[TM_EQ as usize])
                };
                if tm.is_null() {
                    tm = if ((*((*t2).value.value_object as *mut Table)).get_metatable()).is_null() {
                        null()
                    } else if (*(*((*t2).value.value_object as *mut Table)).get_metatable()).flags as u32 & (1 as u32) << TM_EQ as i32 != 0 {
                        null()
                    } else {
                        luat_gettm((*((*t2).value.value_object as *mut Table)).get_metatable(), TM_EQ, (*(*interpreter).global).tm_name[TM_EQ as usize])
                    };
                }
            },
            _ => return (*t1).value.value_object == (*t2).value.value_object,
        }
        if tm.is_null() {
            return false;
        } else {
            luat_calltmres(interpreter, tm, t1, t2, (*interpreter).top.stkidrel_pointer);
            return !((*(*interpreter).top.stkidrel_pointer).get_tag_variant() == TagVariant::BooleanFalse || (*(*interpreter).top.stkidrel_pointer).is_tagtype_nil());
        };
    }
}
pub unsafe fn luav_objlen(interpreter: *mut Interpreter, ra: *mut TValue, rb: *const TValue) {
    unsafe {
        let tvalue: *const TValue;
        match (*rb).get_tag_variant() {
            TagVariant::Table => {
                let table: *mut Table = &mut (*((*rb).value.value_object as *mut Table));
                tvalue = if ((*table).get_metatable()).is_null() {
                    null()
                } else if (*(*table).get_metatable()).flags as u32 & (1 as u32) << TM_LEN as i32 != 0 {
                    null()
                } else {
                    luat_gettm((*table).get_metatable(), TM_LEN, (*(*interpreter).global).tm_name[TM_LEN as usize])
                };
                if tvalue.is_null() {
                    let io: *mut TValue = &mut (*ra);
                    (*io).value.value_integer = luah_getn(table) as i64;
                    (*io).set_tag_variant(TagVariant::NumericInteger);
                    return;
                }
            },
            TagVariant::StringShort | TagVariant::StringLong => {
                let io: *mut TValue = &mut (*ra);
                (*io).value.value_integer = (*((*rb).value.value_object as *mut TString)).get_length() as i64;
                (*io).set_tag_variant(TagVariant::NumericInteger);
                return;
            },
            _ => {
                tvalue = luat_gettmbyobj(interpreter, rb, TM_LEN);
                if ((*tvalue).get_tag_type()) == TagType::Nil {
                    luag_typeerror(interpreter, rb, c"get length of".as_ptr());
                }
            },
        }
        luat_calltmres(interpreter, tvalue, rb, rb, ra);
    }
}

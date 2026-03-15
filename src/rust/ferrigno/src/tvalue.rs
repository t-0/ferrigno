#![allow(unpredictable_function_pointer_comparisons, unused)]
#![allow(unused_imports)]
use crate::character::*;
use crate::closure::*;
use crate::f2i::*;
use crate::functions::*;
use crate::functionstate::LUA_N2SBUFFSZ;
use crate::object::*;
use crate::object::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tagtype::*;
use crate::tagvariant::*;
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
    tvalue_value: Value,
    tvalue_tagvariant: TagVariant,
    tvalue_collectable: bool,
    pub tvalue_delta: u16,
}
impl TValue {
    pub unsafe fn from_string_to_number(&mut self, obj: *const TValue) -> bool {
        unsafe {
            if (*obj).get_tagvariant().to_tag_type().is_string() {
                let tstring: *mut TString = (*obj).as_string().unwrap();
                luao_str2num((*tstring).get_contents_mut(), self) == (*tstring).get_length() + 1
            } else {
                false
            }
        }
    }
    pub unsafe fn to_number(&self, result: *mut f64) -> bool {
        unsafe {
            let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
            if let Some(i) = (*self).as_integer() {
                *result = i as f64;
                true
            } else if tvalue.from_string_to_number(self) {
                *result = tvalue.as_float().unwrap();
                true
            } else {
                false
            }
        }
    }
    pub unsafe fn to_pointer(&self) -> *mut std::ffi::c_void {
        unsafe {
            match self.get_tagvariant() {
                | TagVariant::ClosureCFunction => {
                    ::core::mem::transmute::<CFunction, usize>(self.as_function().unwrap()) as *mut std::ffi::c_void
                },
                | TagVariant::User => (*self.as_user().unwrap()).get_raw_memory_mut(),
                | TagVariant::Pointer => self.as_pointer().unwrap(),
                | _ => {
                    if let Some(obj) = self.as_object() {
                        obj as *mut std::ffi::c_void
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
    pub fn tvalue_set_tag_variant_collectable(&mut self, tagvariant: TagVariant) {
        self.tvalue_tagvariant = tagvariant;
        self.tvalue_collectable = true;
    }
    pub fn is_collectable(&self) -> bool {
        self.tvalue_collectable
    }
    pub fn set_collectable(&mut self, value: bool) {
        self.tvalue_collectable = value;
    }
    // --- Typed read accessors ---
    pub fn as_integer(&self) -> Option<i64> {
        if self.tvalue_tagvariant == TagVariant::NumericInteger {
            Some(unsafe { self.tvalue_value.value_integer })
        } else {
            None
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        if self.tvalue_tagvariant == TagVariant::NumericNumber {
            Some(unsafe { self.tvalue_value.value_number })
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self.tvalue_tagvariant {
            | TagVariant::NumericNumber => Some(unsafe { self.tvalue_value.value_number }),
            | TagVariant::NumericInteger => Some(unsafe { self.tvalue_value.value_integer } as f64),
            | _ => None,
        }
    }
    pub fn as_boolean(&self) -> Option<bool> {
        match self.tvalue_tagvariant {
            | TagVariant::BooleanTrue => Some(true),
            | TagVariant::BooleanFalse => Some(false),
            | _ => None,
        }
    }
    pub fn as_string(&self) -> Option<*mut TString> {
        match self.tvalue_tagvariant {
            | TagVariant::StringShort | TagVariant::StringLong => Some(unsafe { self.tvalue_value.value_object as *mut TString }),
            | _ => None,
        }
    }
    pub fn as_table(&self) -> Option<*mut Table> {
        if self.tvalue_tagvariant == TagVariant::Table {
            Some(unsafe { self.tvalue_value.value_object as *mut Table })
        } else {
            None
        }
    }
    pub fn as_closure(&self) -> Option<*mut Closure> {
        match self.tvalue_tagvariant {
            | TagVariant::ClosureL | TagVariant::ClosureC | TagVariant::ClosureCFunction => {
                Some(unsafe { self.tvalue_value.value_object as *mut Closure })
            },
            | _ => None,
        }
    }
    pub fn as_object(&self) -> Option<*mut Object> {
        if self.tvalue_collectable {
            Some(unsafe { self.tvalue_value.value_object })
        } else {
            None
        }
    }
    pub fn as_function(&self) -> Option<CFunction> {
        if self.tvalue_tagvariant == TagVariant::ClosureCFunction {
            Some(unsafe { self.tvalue_value.value_function })
        } else {
            None
        }
    }
    pub fn as_pointer(&self) -> Option<*mut std::ffi::c_void> {
        if self.tvalue_tagvariant == TagVariant::Pointer {
            Some(unsafe { self.tvalue_value.value_pointer })
        } else {
            None
        }
    }
    pub fn as_user(&self) -> Option<*mut User> {
        if self.tvalue_tagvariant == TagVariant::User {
            Some(unsafe { self.tvalue_value.value_object as *mut User })
        } else {
            None
        }
    }
    // --- Typed write accessors ---
    pub fn set_integer(&mut self, value: i64) {
        self.tvalue_value.value_integer = value;
        self.tvalue_tagvariant = TagVariant::NumericInteger;
        self.tvalue_collectable = false;
    }
    pub fn set_number(&mut self, value: f64) {
        self.tvalue_value.value_number = value;
        self.tvalue_tagvariant = TagVariant::NumericNumber;
        self.tvalue_collectable = false;
    }
    pub fn set_boolean(&mut self, value: bool) {
        self.tvalue_tagvariant = if value { TagVariant::BooleanTrue } else { TagVariant::BooleanFalse };
        self.tvalue_collectable = false;
    }
    pub fn set_nil(&mut self) {
        self.tvalue_tagvariant = TagVariant::NilNil;
        self.tvalue_collectable = false;
    }
    pub fn set_table(&mut self, t: *mut Table) {
        self.tvalue_value.value_object = t as *mut Object;
        self.tvalue_tagvariant = TagVariant::Table;
        self.tvalue_collectable = true;
    }
    pub fn set_pointer(&mut self, p: *mut std::ffi::c_void) {
        self.tvalue_value.value_pointer = p;
        self.tvalue_tagvariant = TagVariant::Pointer;
        self.tvalue_collectable = false;
    }
    pub fn set_function(&mut self, f: CFunction) {
        self.tvalue_value.value_function = f;
        self.tvalue_tagvariant = TagVariant::ClosureCFunction;
        self.tvalue_collectable = false;
    }
    pub fn set_object(&mut self, obj: *mut Object, variant: TagVariant) {
        self.tvalue_value.value_object = obj;
        self.tvalue_tagvariant = variant;
        self.tvalue_collectable = true;
    }
    // --- Raw Value accessors for array slot operations ---
    pub fn get_raw_value(&self) -> Value {
        self.tvalue_value
    }
    pub fn set_raw_value(&mut self, v: Value) {
        self.tvalue_value = v;
    }
    pub unsafe fn raw_object_ptr(&self) -> *mut Object {
        unsafe { self.tvalue_value.value_object }
    }
    pub unsafe fn from_interpreter_to_string(&mut self, state: *mut State) {
        unsafe {
            let mut buffer: [i8; LUA_N2SBUFFSZ] = [0; LUA_N2SBUFFSZ];
            let length = tostringbuff(self, buffer.as_mut_ptr());
            let tstring: *mut TString = luas_newlstr(state, buffer.as_mut_ptr(), length);
            self.set_object(tstring as *mut Object, (*tstring).get_tagvariant());
        }
    }
}
pub unsafe fn aux_upvalue(fi: *mut TValue, n: i32, value: *mut *mut TValue, owner: *mut *mut Object) -> *const i8 {
    unsafe {
        match (*fi).get_tagvariant() {
            | TagVariant::ClosureC => {
                let closure: *mut Closure = (*fi).as_closure().unwrap();
                if n > (*closure).closure_count_upvalues as i32 {
                    return null();
                }
                *value = &mut *((*closure).closure_upvalues)
                    .closureupvalue_tvalues
                    .as_mut_ptr()
                    .add((n - 1) as usize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut *(closure as *mut Object);
                }
                c"".as_ptr()
            },
            | TagVariant::ClosureL => {
                let closure: *mut Closure = (*fi).as_closure().unwrap();
                let p: *mut Prototype = (*closure).closure_payload.closurepayload_lprototype;
                if ((n as u32).wrapping_sub(1_u32) >= (*p).prototype_upvalues.get_size() as u32) {
                    return null();
                }
                *value = (**((*closure).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add((n - 1) as usize))
                .upvalue_v
                .upvaluea_p;
                if !owner.is_null() {
                    *owner = &mut *(*((*closure).closure_upvalues)
                        .closureupvalue_lvalues
                        .as_mut_ptr()
                        .add((n - 1) as usize) as *mut Object);
                }
                let name: *mut TString = (*((*p).prototype_upvalues.vectort_pointer).add((n - 1) as usize)).upvaluedescription_name;
                if name.is_null() {
                    c"(no name)".as_ptr()
                } else {
                    ((*name).get_contents_mut()) as *const i8
                }
            },
            | _ => null(),
        }
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
                (*o).set_number(n);
            }
        } else {
            (*o).set_integer(i);
        }
        (e.offset_from(s) as i64 + 1) as usize
    }
}
pub unsafe fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> usize {
    unsafe {
        let buf = std::slice::from_raw_parts_mut(buffer as *mut u8, LUA_N2SBUFFSZ);
        let mut length: usize;
        if let Some(i) = (*obj).as_integer() {
            use std::io::Write;
            let mut pos: &mut [u8] = &mut *buf;
            write!(pos, "{}", i).ok();
            length = LUA_N2SBUFFSZ - pos.len();
        } else {
            length = crate::utility::format_float_roundtrip((*obj).as_number().unwrap(), buf);
            let all_digits = buf[..length].iter().all(|&b| b == b'-' || b.is_ascii_digit());
            if all_digits {
                buf[length] = b'.';
                length += 1;
                buf[length] = b'0';
                length += 1;
            }
        }
        length
    }
}
pub const ABSENT_KEY: TValue = { TValue::new(TagVariant::NilAbsentKey) };
pub unsafe fn arrayindex(k: i64) -> u32 {
    if (k as usize).wrapping_sub(1_usize)
        < (if (1_u32 << size_of::<i32>().wrapping_mul(8_usize).wrapping_sub(1_usize) as i32) as usize
            <= (((!0usize) / (size_of::<Value>() + 1)) as usize)
        {
            1_u32 << size_of::<i32>().wrapping_mul(8_usize).wrapping_sub(1_usize) as i32
        } else {
            ((!0usize) / (size_of::<Value>() + 1)) as u32
        }) as usize
    {
        k as u32
    } else {
        0
    }
}
pub unsafe fn lessthanothers(state: *mut State, l: *const TValue, r: *const TValue) -> i32 {
    unsafe {
        if let (Some(ls), Some(rs)) = ((*l).as_string(), (*r).as_string()) {
            (l_strcmp(&*ls, &*rs) < 0) as i32
        } else {
            luat_callordertm(state, l, r, TM_LT)
        }
    }
}
pub unsafe fn luav_lessthan(state: *mut State, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_numeric() && (*r).get_tagvariant().to_tag_type().is_numeric() {
            ltnum(l, r)
        } else {
            0 != lessthanothers(state, l, r)
        }
    }
}
pub unsafe fn lessequalothers(state: *mut State, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if let (Some(ls), Some(rs)) = ((*l).as_string(), (*r).as_string()) {
            l_strcmp(&*ls, &*rs) <= 0
        } else {
            0 != luat_callordertm(state, l, r, TM_LE)
        }
    }
}
pub unsafe fn luav_lessequal(state: *mut State, l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant().to_tag_type().is_numeric() && (*r).get_tagvariant().to_tag_type().is_numeric() {
            lenum(l, r)
        } else {
            lessequalothers(state, l, r)
        }
    }
}
pub unsafe fn luav_equalobj(state: *mut State, t1: *const TValue, t2: *const TValue) -> bool {
    unsafe {
        let mut tm: *const TValue;
        if (*t1).get_tagvariant() != (*t2).get_tagvariant() {
            if (*t1).get_tagvariant().to_tag_type() != (*t2).get_tagvariant().to_tag_type()
                || !(*t1).get_tagvariant().to_tag_type().is_numeric()
            {
                return false;
            } else {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                return F2I::Equal.convert_tv_i64(t1, &mut i1) != 0 && F2I::Equal.convert_tv_i64(t2, &mut i2) != 0 && i1 == i2;
            }
        }
        match (*t1).get_tagvariant() {
            | TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => return true,
            | TagVariant::NumericInteger => return (*t1).as_integer().unwrap() == (*t2).as_integer().unwrap(),
            | TagVariant::NumericNumber => return (*t1).as_number().unwrap() == (*t2).as_number().unwrap(),
            | TagVariant::Pointer => return (*t1).as_pointer().unwrap() == (*t2).as_pointer().unwrap(),
            | TagVariant::ClosureCFunction => return (*t1).as_function().unwrap() == (*t2).as_function().unwrap(),
            | TagVariant::StringShort => {
                return (*t1).as_string().unwrap() == (*t2).as_string().unwrap();
            },
            | TagVariant::StringLong => {
                return luas_eqlngstr(&mut *(*t1).as_string().unwrap(), &mut *(*t2).as_string().unwrap());
            },
            | TagVariant::User => {
                let u1 = (*t1).as_user().unwrap();
                let u2 = (*t2).as_user().unwrap();
                if u1 == u2 {
                    return true;
                } else if state.is_null() {
                    return false;
                }
                tm = if ((*u1).get_metatable()).is_null() {
                    null()
                } else if (*(*u1).get_metatable()).table_flags as u32 & 1_u32 << TM_EQ as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*u1).get_metatable(),
                        TM_EQ,
                        (*(*state).interpreter_global).global_tmname[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*u2).get_metatable()).is_null() {
                        null()
                    } else if (*(*u2).get_metatable()).table_flags as u32 & 1_u32 << TM_EQ as i32 != 0 {
                        null()
                    } else {
                        luat_gettm(
                            (*u2).get_metatable(),
                            TM_EQ,
                            (*(*state).interpreter_global).global_tmname[TM_EQ as usize],
                        )
                    };
                }
            },
            | TagVariant::Table => {
                let tb1 = (*t1).as_table().unwrap();
                let tb2 = (*t2).as_table().unwrap();
                if tb1 == tb2 {
                    return true;
                } else if state.is_null() {
                    return false;
                }
                tm = if ((*tb1).get_metatable()).is_null() {
                    null()
                } else if (*(*tb1).get_metatable()).table_flags as u32 & 1_u32 << TM_EQ as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*tb1).get_metatable(),
                        TM_EQ,
                        (*(*state).interpreter_global).global_tmname[TM_EQ as usize],
                    )
                };
                if tm.is_null() {
                    tm = if ((*tb2).get_metatable()).is_null() {
                        null()
                    } else if (*(*tb2).get_metatable()).table_flags as u32 & 1_u32 << TM_EQ as i32 != 0 {
                        null()
                    } else {
                        luat_gettm(
                            (*tb2).get_metatable(),
                            TM_EQ,
                            (*(*state).interpreter_global).global_tmname[TM_EQ as usize],
                        )
                    };
                }
            },
            | _ => return (*t1).as_object() == (*t2).as_object(),
        }
        if tm.is_null() {
            false
        } else {
            luat_calltmres(state, tm, t1, t2, (*state).interpreter_top.stkidrel_pointer);
            !((*(*state).interpreter_top.stkidrel_pointer).get_tagvariant() == TagVariant::BooleanFalse
                || (*(*state).interpreter_top.stkidrel_pointer)
                    .get_tagvariant()
                    .to_tag_type()
                    .is_nil())
        }
    }
}
pub unsafe fn luav_objlen(state: *mut State, ra: *mut TValue, rb: *const TValue) {
    unsafe {
        let tvalue: *const TValue;
        match (*rb).get_tagvariant() {
            | TagVariant::Table => {
                let table: *mut Table = (*rb).as_table().unwrap();
                tvalue = if ((*table).get_metatable()).is_null() {
                    null()
                } else if (*(*table).get_metatable()).table_flags as u32 & 1_u32 << TM_LEN as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*table).get_metatable(),
                        TM_LEN,
                        (*(*state).interpreter_global).global_tmname[TM_LEN as usize],
                    )
                };
                if tvalue.is_null() {
                    (*ra).set_integer(luah_getn(state, table) as i64);
                    return;
                }
            },
            | TagVariant::StringShort | TagVariant::StringLong => {
                (*ra).set_integer((*(*rb).as_string().unwrap()).get_length() as i64);
                return;
            },
            | _ => {
                tvalue = luat_gettmbyobj(state, rb, TM_LEN);
                if (*tvalue).get_tagvariant().to_tag_type() == TagType::Nil {
                    luag_typeerror(state, rb, c"get length of".as_ptr());
                }
            },
        }
        luat_calltmres(state, tvalue, rb, rb, ra);
    }
}

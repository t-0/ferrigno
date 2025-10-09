use crate::f2i::*;
use crate::interpreter::*;
use crate::tagvariant::*;
use crate::tvalue::*;
use crate::utility::*;
pub unsafe fn forlimit(interpreter: *mut Interpreter, initial: i64, lim: *const TValue, p: *mut i64, step: i64) -> i32 {
    unsafe {
        if luav_tointeger(lim, p, if step < 0 { F2I::Ceiling } else { F2I::Floor }) == 0 {
            let mut flim: f64 = 0.0;
            if if (*lim).get_tagvariant() == TagVariant::NumericNumber {
                flim = (*lim).tvalue_value.value_number;
                1
            } else {
                if (*lim).to_number(&mut flim) { 1 } else { 0 }
            } == 0
            {
                luag_forerror(interpreter, lim, c"limit".as_ptr());
            }
            if (0.0) < flim {
                if step < 0 {
                    return 1;
                }
                *p = MAXIMUM_SIZE as i64;
            } else {
                if step > 0 {
                    return 1;
                }
                *p = -(MAXIMUM_SIZE as i64) - 1 as i64;
            }
        }
        return if step > 0 { (initial > *p) as i32 } else { (initial < *p) as i32 };
    }
}
pub unsafe fn forprep(interpreter: *mut Interpreter, ra: *mut TValue) -> i32 {
    unsafe {
        let pinit = &mut (*ra);
        let plimit = &mut (*ra.offset(1 as isize));
        let pstep = &mut (*ra.offset(2 as isize));
        if (*pinit).get_tagvariant() == TagVariant::NumericInteger && (*pstep).get_tagvariant() == TagVariant::NumericInteger {
            let initial: i64 = (*pinit).tvalue_value.value_integer;
            let step: i64 = (*pstep).tvalue_value.value_integer;
            let mut limit: i64 = 0;
            if step == 0 {
                luag_runerror(interpreter, c"'for' step is zero".as_ptr());
            }
            let io = &mut (*ra.offset(3 as isize));
            (*io).tvalue_value.value_integer = initial;
            (*io).tvalue_set_tag_variant(TagVariant::NumericInteger);
            if forlimit(interpreter, initial, plimit, &mut limit, step) != 0 {
                return 1;
            } else {
                let mut count: usize;
                if step > 0 {
                    count = (limit as usize).wrapping_sub(initial as usize);
                    if step != 1 {
                        count /= step as usize;
                    }
                } else {
                    count = (initial as usize).wrapping_sub(limit as usize);
                    count = count / ((-(step + 1) as usize) + 1) as usize;
                }
                (*plimit).tvalue_value.value_integer = count as i64;
                (*plimit).tvalue_set_tag_variant(TagVariant::NumericInteger);
            }
        } else {
            let mut init_0: f64 = 0.0;
            let mut limit_0: f64 = 0.0;
            let mut step_0: f64 = 0.0;
            if (((if (*plimit).get_tagvariant() == TagVariant::NumericNumber {
                limit_0 = (*plimit).tvalue_value.value_number;
                1
            } else {
                if (*plimit).to_number(&mut limit_0) { 1 } else { 0 }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(interpreter, plimit, c"limit".as_ptr());
            }
            if (((if (*pstep).get_tagvariant() == TagVariant::NumericNumber {
                step_0 = (*pstep).tvalue_value.value_number;
                1
            } else {
                if (*pstep).to_number(&mut step_0) { 1 } else { 0 }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(interpreter, pstep, c"step".as_ptr());
            }
            if (((if (*pinit).get_tagvariant() == TagVariant::NumericNumber {
                init_0 = (*pinit).tvalue_value.value_number;
                1
            } else {
                if (*pinit).to_number(&mut init_0) { 1 } else { 0 }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(interpreter, pinit, c"initial value".as_ptr());
            }
            if step_0 == 0.0 {
                luag_runerror(interpreter, c"'for' step is zero".as_ptr());
            }
            if if (0.0) < step_0 {
                (limit_0 < init_0) as i32
            } else {
                (init_0 < limit_0) as i32
            } != 0
            {
                return 1;
            } else {
                (*plimit).tvalue_value.value_number = limit_0;
                (*plimit).tvalue_set_tag_variant(TagVariant::NumericNumber);
                (*pstep).tvalue_value.value_number = step_0;
                (*pstep).tvalue_set_tag_variant(TagVariant::NumericNumber);
                let io_3: *mut TValue = &mut (*ra);
                (*io_3).tvalue_value.value_number = init_0;
                (*io_3).tvalue_set_tag_variant(TagVariant::NumericNumber);
                let io_4: *mut TValue = &mut (*ra.offset(3 as isize));
                (*io_4).tvalue_value.value_number = init_0;
                (*io_4).tvalue_set_tag_variant(TagVariant::NumericNumber);
            }
        }
        return 0;
    }
}
pub unsafe fn floatforloop(ra: *mut TValue) -> i32 {
    unsafe {
        let step: f64 = (*ra.offset(2 as isize)).tvalue_value.value_number;
        let limit: f64 = (*ra.offset(1 as isize)).tvalue_value.value_number;
        let mut index: f64 = (*ra).tvalue_value.value_number;
        index = index + step;
        if if (0.0) < step { (index <= limit) as i32 } else { (limit <= index) as i32 } != 0 {
            let io: *mut TValue = &mut (*ra);
            (*io).tvalue_value.value_number = index;
            let io_0: *mut TValue = &mut (*ra.offset(3 as isize));
            (*io_0).tvalue_value.value_number = index;
            (*io_0).tvalue_set_tag_variant(TagVariant::NumericNumber);
            return 1;
        } else {
            return 0;
        };
    }
}

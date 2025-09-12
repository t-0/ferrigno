use rlua::*;
use crate::f2i::*;
use crate::interpreter::*;
use crate::stackvalue::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::*;
pub unsafe extern "C" fn forlimit(
    interpreter: *mut Interpreter,
    init: i64,
    lim: *const TValue,
    p: *mut i64,
    step: i64,
) -> i32 {
    unsafe {
        if luav_tointeger(lim, p, if step < 0 { F2I::Ceiling } else { F2I::Floor }) == 0 {
            let mut flim: f64 = 0.0;
            if if (*lim).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                flim = (*lim).value.number;
                1
            } else {
                if (*lim).to_number(&mut flim) {
                    1
                } else {
                    0
                }
            } == 0
            {
                luag_forerror(interpreter, lim, make_cstring!("limit"));
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
        return if step > 0 {
            (init > *p) as i32
        } else {
            (init < *p) as i32
        };
    }
}
pub unsafe extern "C" fn forprep(interpreter: *mut Interpreter, ra: StackValuePointer) -> i32 {
    unsafe {
        let pinit: *mut TValue = &mut (*ra).tvalue;
        let plimit: *mut TValue = &mut (*ra.offset(1 as isize)).tvalue;
        let pstep: *mut TValue = &mut (*ra.offset(2 as isize)).tvalue;
        if (*pinit).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
            && (*pstep).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
        {
            let init: i64 = (*pinit).value.integer;
            let step: i64 = (*pstep).value.integer;
            let mut limit: i64 = 0;
            if step == 0 {
                luag_runerror(
                    interpreter,
                    make_cstring!("'for' step is zero"),
                );
            }
            let io: *mut TValue = &mut (*ra.offset(3 as isize)).tvalue;
            (*io).value.integer = init;
            (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
            if forlimit(interpreter, init, plimit, &mut limit, step) != 0 {
                return 1;
            } else {
                let mut count: usize;
                if step > 0 {
                    count = (limit as usize).wrapping_sub(init as usize);
                    if step != 1 {
                        count = (count as usize).wrapping_div(step as usize) as usize;
                    }
                } else {
                    count = (init as usize).wrapping_sub(limit as usize);
                    count = (count as usize)
                        .wrapping_div((-(step + 1) as usize).wrapping_add(1 as usize))
                        as usize;
                }
                let io_0: *mut TValue = plimit;
                (*io_0).value.integer = count as i64;
                (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
            }
        } else {
            let mut init_0: f64 = 0.0;
            let mut limit_0: f64 = 0.0;
            let mut step_0: f64 = 0.0;
            if (((if (*plimit).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                limit_0 = (*plimit).value.number;
                1
            } else {
                if (*plimit).to_number(&mut limit_0) {
                    1
                } else {
                    0
                }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(interpreter, plimit, make_cstring!("limit"));
            }
            if (((if (*pstep).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                step_0 = (*pstep).value.number;
                1
            } else {
                if (*pstep).to_number(&mut step_0) {
                    1
                } else {
                    0
                }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(interpreter, pstep, make_cstring!("step"));
            }
            if (((if (*pinit).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                init_0 = (*pinit).value.number;
                1
            } else {
                if (*pinit).to_number(&mut init_0) {
                    1
                } else {
                    0
                }
            }) == 0) as i32
                != 0) as i64
                != 0
            {
                luag_forerror(
                    interpreter,
                    pinit,
                    make_cstring!("initial value"),
                );
            }
            if step_0 == 0.0 {
                luag_runerror(
                    interpreter,
                    make_cstring!("'for' step is zero"),
                );
            }
            if if (0.0) < step_0 {
                (limit_0 < init_0) as i32
            } else {
                (init_0 < limit_0) as i32
            } != 0
            {
                return 1;
            } else {
                let io_1: *mut TValue = plimit;
                (*io_1).value.number = limit_0;
                (*io_1).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                let io_2: *mut TValue = pstep;
                (*io_2).value.number = step_0;
                (*io_2).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                let io_3: *mut TValue = &mut (*ra).tvalue;
                (*io_3).value.number = init_0;
                (*io_3).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                let io_4: *mut TValue = &mut (*ra.offset(3 as isize)).tvalue;
                (*io_4).value.number = init_0;
                (*io_4).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn floatforloop(ra: StackValuePointer) -> i32 {
    unsafe {
        let step: f64 = (*ra.offset(2 as isize)).tvalue.value.number;
        let limit: f64 = (*ra.offset(1 as isize)).tvalue.value.number;
        let mut index: f64 = (*ra).tvalue.value.number;
        index = index + step;
        if if (0.0) < step {
            (index <= limit) as i32
        } else {
            (limit <= index) as i32
        } != 0
        {
            let io: *mut TValue = &mut (*ra).tvalue;
            (*io).value.number = index;
            let io_0: *mut TValue = &mut (*ra.offset(3 as isize)).tvalue;
            (*io_0).value.number = index;
            (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
            return 1;
        } else {
            return 0;
        };
    }
}

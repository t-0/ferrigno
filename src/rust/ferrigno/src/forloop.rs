use crate::f2i::*;
use crate::state::*;
use crate::tvalue::*;
use crate::utility::*;
pub unsafe fn forlimit(state: *mut State, initial: i64, lim: *const TValue, p: *mut i64, step: i64) -> i32 {
    unsafe {
        if if step < 0 { F2I::Ceiling } else { F2I::Floor }.convert_tv_i64(lim, p) == 0 {
            let mut flim: f64 = 0.0;
            if if let Some(n) = (*lim).as_number() {
                flim = n;
                1
            } else {
                if (*lim).to_number(&mut flim) { 1 } else { 0 }
            } == 0
            {
                luag_forerror(state, lim, c"limit".as_ptr());
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
                *p = -(MAXIMUM_SIZE as i64) - 1_i64;
            }
        }
        if step > 0 { (initial > *p) as i32 } else { (initial < *p) as i32 }
    }
}
pub unsafe fn forprep(state: *mut State, ra: *mut TValue) -> i32 {
    unsafe {
        let pinit = &mut (*ra);
        let plimit = &mut (*ra.add(1));
        let pstep = &mut (*ra.add(2));
        if let (Some(initial), Some(step)) = ((*pinit).as_integer(), (*pstep).as_integer()) {
            let mut limit: i64 = 0;
            if step == 0 {
                luag_runerror(state, c"'for' step is zero".as_ptr());
            }
            (*ra.add(3)).set_integer(initial);
            if forlimit(state, initial, plimit, &mut limit, step) != 0 {
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
                    count /= (-(step + 1) as usize) + 1;
                }
                (*plimit).set_integer(count as i64);
            }
        } else {
            let mut init: f64 = 0.0;
            let mut limit: f64 = 0.0;
            let mut step: f64 = 0.0;
            if (if let Some(n) = (*plimit).as_number() {
                limit = n;
                1
            } else {
                if (*plimit).to_number(&mut limit) { 1 } else { 0 }
            }) == 0
            {
                luag_forerror(state, plimit, c"limit".as_ptr());
            }
            if (if let Some(n) = (*pstep).as_number() {
                step = n;
                1
            } else {
                if (*pstep).to_number(&mut step) { 1 } else { 0 }
            }) == 0
            {
                luag_forerror(state, pstep, c"step".as_ptr());
            }
            if (if let Some(n) = (*pinit).as_number() {
                init = n;
                1
            } else {
                if (*pinit).to_number(&mut init) { 1 } else { 0 }
            }) == 0
            {
                luag_forerror(state, pinit, c"initial value".as_ptr());
            }
            if step == 0.0 {
                luag_runerror(state, c"'for' step is zero".as_ptr());
            }
            if if (0.0) < step { (limit < init) as i32 } else { (init < limit) as i32 } != 0 {
                return 1;
            } else {
                (*plimit).set_number(limit);
                (*pstep).set_number(step);
                (*ra).set_number(init);
                (*ra.add(3)).set_number(init);
            }
        }
        0
    }
}
pub unsafe fn floatforloop(ra: *mut TValue) -> i32 {
    unsafe {
        let step: f64 = (*ra.add(2)).as_number().unwrap();
        let limit: f64 = (*ra.add(1)).as_number().unwrap();
        let mut index: f64 = (*ra).as_number().unwrap();
        index += step;
        if if (0.0) < step { (index <= limit) as i32 } else { (limit <= index) as i32 } != 0 {
            (*ra).set_number(index);
            (*ra.add(3)).set_number(index);
            1
        } else {
            0
        }
    }
}

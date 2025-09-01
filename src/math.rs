#![allow(static_mut_refs, unsafe_code)]
use crate::utility::c::*;
use crate::randomstate::*;
use crate::state::*;
pub const PI: f64 = 3.141592653589793238462643383279502884f64;
pub unsafe extern "C" fn push_numericcc(state: *mut State, d: f64) {
    unsafe {
        let mut n: i64 = 0;
        if d >= (-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64
            && d < -((-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64)
            && {
                n = d as i64;
                1 != 0
            }
        {
            (*state).push_integer(n);
        } else {
            (*state).push_number(d);
        };
    }
}
pub unsafe extern "C" fn rotate_left(x: u64, n: i32) -> u64 {
    (x << n) | ((x & 0xffffffffffffffff as u64) >> (64 - n))
}
pub unsafe extern "C" fn next_random(randomstate: *mut u64) -> u64 {
    unsafe {
        let state0: u64 = *randomstate.offset(0 as isize);
        let state1: u64 = *randomstate.offset(1 as isize);
        let state2: u64 = *randomstate.offset(2 as isize) ^ state0;
        let state3: u64 = *randomstate.offset(3 as isize) ^ state1;
        let res: u64 =
            (rotate_left(state1.wrapping_mul(5 as u64), 7)).wrapping_mul(9 as u64);
        *randomstate.offset(0 as isize) = state0 ^ state3;
        *randomstate.offset(1 as isize) = state1 ^ state2;
        *randomstate.offset(2 as isize) = state2 ^ state1 << 17 as i32;
        *randomstate.offset(3 as isize) = rotate_left(state3, 45 as i32);
        res
    }
}
pub unsafe extern "C" fn i2d(x: u64) -> f64 {
    let sx: i64 = ((x & 0xffffffffffffffff as u64) >> (64 - 53)) as i64;
    let mut res: f64 = sx as f64 * (0.5f64 / ((1 as u64) << (53 - 1)) as f64);
    if sx < 0 {
        res += 1.0f64;
    }
    res
}
pub unsafe extern "C" fn project(mut ran: u64, n: u64, ransate: *mut RandomState) -> u64 {
    unsafe {
        if n & n.wrapping_add(1 as u64) == 0u64 {
            return ran & n;
        } else {
            let mut lim: u64 = n;
            lim |= lim >> 1;
            lim |= lim >> 2;
            lim |= lim >> 4;
            lim |= lim >> 8;
            lim |= lim >> 16;
            lim |= lim >> 32;
            loop {
                ran &= lim;
                if !(ran > n) {
                    break;
                }
                ran = (next_random(((*ransate).s).as_mut_ptr()) & 0xffffffffffffffff as u64) as u64;
            }
            return ran;
        };
    }
}
pub unsafe extern "C" fn set_seed(state: *mut State, randomstate: *mut u64, n1: u64, n2: u64) {
    unsafe {
        let mut i: i32;
        *randomstate.offset(0 as isize) = n1 as u64;
        *randomstate.offset(1 as isize) = 0xFF as u64;
        *randomstate.offset(2 as isize) = n2 as u64;
        *randomstate.offset(3 as isize) = 0;
        i = 0;
        while i < 16 as i32 {
            next_random(randomstate);
            i += 1;
        }
        (*state).push_integer(n1 as i64);
        (*state).push_integer(n2 as i64);
    }
}
pub unsafe extern "C" fn random_seed(state: *mut State, randomstate: *mut RandomState) {
    unsafe {
        let seed1: u64 = time(std::ptr::null_mut()) as u64;
        let seed2: u64 = state as u64;
        set_seed(state, ((*randomstate).s).as_mut_ptr(), seed1, seed2);
    }
}

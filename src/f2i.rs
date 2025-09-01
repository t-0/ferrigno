use crate::tvalue::*;
use crate::tag::*;
use crate::value::*;
#[derive(PartialEq)]
#[repr(C)]
pub enum F2I {
    Equal,
    Floor,
    Ceiling,
}
pub unsafe extern "C" fn luav_flttointeger(n: f64, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut f: f64 = n.floor();
        if n != f {
            if mode == F2I::Equal {
                return 0;
            } else if mode == F2I::Ceiling {
                f += 1.0;
            }
        }
        return (f >= (-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64
            && f < -((-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64)
            && {
                *p = f as i64;
                1 != 0
            }) as i32;
    }
}
pub unsafe extern "C" fn luav_tointegerns(obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        if (*obj).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            return luav_flttointeger((*obj).value.n, p, mode);
        } else if (*obj).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            *p = (*obj).value.i;
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn luav_tointeger(mut obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut v: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        if l_strton(obj, &mut v) != 0 {
            obj = &mut v;
        }
        return luav_tointegerns(obj, p, mode);
    }
}

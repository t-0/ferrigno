use crate::f2i::*;
use crate::opcode::*;
use crate::operatorbinary::*;
use crate::operatorunary::*;
use crate::tagvariant::*;
use crate::tm::*;
use crate::tvalue::*;
pub unsafe fn validop(op: i32, v1: *mut TValue, v2: *mut TValue) -> i32 {
    unsafe {
        match op {
            | 7 | 8 | 9 | 10 | 11 | 13 => {
                let mut i: i64 = 0;
                return (F2I::Equal.convert_tv_i64(v1, &mut i) != 0 && F2I::Equal.convert_tv_i64(v2, &mut i) != 0) as i32;
            },
            | 5 | 6 | 3 => {
                return ((if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                    (*v2).tvalue_value.value_integer as f64
                } else {
                    (*v2).tvalue_value.value_number
                }) != 0.0) as i32;
            },
            | _ => return 1,
        };
    }
}
pub unsafe fn binopr2op(binary: OperatorBinary, baser: OperatorBinary, base: u32) -> u32 {
    return (binary as i32 - baser as i32 + base as i32) as u32;
}
pub unsafe fn unopr2op(unary: OperatorUnary) -> u32 {
    return (unary as i32 - OperatorUnary::Minus as i32 + OPCODE_UNM as i32) as u32;
}
pub unsafe fn binopr2tm(binary: OperatorBinary) -> u32 {
    return (binary as i32 - OperatorBinary::Add as i32 + TM_ADD as i32) as u32;
}

use crate::f2i::*;
use crate::lexical::operatorbinary::*;
use crate::lexical::operatorunary::*;
use crate::tag::*;
use crate::tm::*;
use crate::tvalue::*;
use crate::vm::opcode::*;
pub unsafe fn validop(op: i32, v1: *mut TValue, v2: *mut TValue) -> i32 {
    unsafe {
        match op {
            7 | 8 | 9 | 10 | 11 | 13 => {
                let mut i: i64 = 0;
                return (luav_tointegerns(v1, &mut i, F2I::Equal) != 0
                    && luav_tointegerns(v2, &mut i, F2I::Equal) != 0)
                    as i32;
            }
            5 | 6 | 3 => {
                return ((if (*v2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                    (*v2).value.integer as f64
                } else {
                    (*v2).value.number
                }) != 0.0) as i32;
            }
            _ => return 1,
        };
    }
}
pub unsafe fn binopr2op(binary: OperatorBinary, baser: OperatorBinary, base: u32) -> u32 {
    return (binary as i32 - baser as i32 + base as i32) as u32;
}
pub unsafe fn unopr2op(unary: OperatorUnary) -> u32 {
    return (unary as i32 - OperatorUnary::Minus as i32 + OP_UNM as i32) as u32;
}
pub unsafe fn binopr2tm(binary: OperatorBinary) -> u32 {
    return (binary as i32 - OperatorBinary::Add as i32 + TM_ADD as i32) as u32;
}

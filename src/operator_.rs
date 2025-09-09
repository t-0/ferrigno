use crate::tvalue::*;
use crate::tag::*;
use crate::f2i::*;
use crate::tm::*;
use crate::vm::opcode::*;
use crate::lexical::operatorunary::*;
pub const OPR_NOBINOPR: u32 = 21;
pub const OPR_OR: u32 = 20;
pub const OPR_AND: u32 = 19;
pub const OPR_GE: u32 = 18;
pub const OPR_GT: u32 = 17;
pub const OPR_NE: u32 = 16;
pub const OPR_LE: u32 = 15;
pub const OPR_LT: u32 = 14;
pub const OPR_EQ: u32 = 13;
pub const OPR_CONCAT: u32 = 12;
pub const OPR_SHR: u32 = 11;
pub const OPR_SHL: u32 = 10;
pub const OPR_BXOR: u32 = 9;
pub const OPR_BOR: u32 = 8;
pub const OPR_BAND: u32 = 7;
pub const OPR_IDIV: u32 = 6;
pub const OPR_DIV: u32 = 5;
pub const OPR_POW: u32 = 4;
pub const OPR_MOD: u32 = 3;
pub const OPR_MUL: u32 = 2;
pub const OPR_SUB: u32 = 1;
pub const OPR_ADD: u32 = 0;
pub unsafe extern "C" fn validop(op: i32, v1: *mut TValue, v2: *mut TValue) -> i32 {
    unsafe {
        match op {
            7 | 8 | 9 | 10 | 11 | 13 => {
                let mut i: i64 = 0;
                return (luav_tointegerns(v1, &mut i, F2I::Equal) != 0
                    && luav_tointegerns(v2, &mut i, F2I::Equal) != 0)
                    as i32;
            }
            5 | 6 | 3 => {
                return ((if (*v2).get_tag2() == TAG_VARIANT_NUMERIC_INTEGER {
                    (*v2).value.integer as f64
                } else {
                    (*v2).value.number
                }) != 0.0) as i32;
            }
            _ => return 1,
        };
    }
}
pub unsafe extern "C" fn binopr2op(opr: u32, baser: u32, base: u32) -> u32 {
    return (opr as i32 - baser as i32 + base as i32) as u32;
}
pub unsafe extern "C" fn unopr2op(unary: OperatorUnary) -> u32 {
    return (unary as i32 - OperatorUnary::Minus as i32 + OP_UNM as i32) as u32;
}
pub unsafe extern "C" fn binopr2tm(opr: u32) -> u32 {
    return (opr as i32 - OPR_ADD as i32 + TM_ADD as i32) as u32;
}

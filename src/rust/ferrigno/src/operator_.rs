use crate::f2i::*;
use crate::opcode::*;
use crate::operatorbinary::*;
use crate::operatorunary::*;
use crate::tm::*;
use crate::tvalue::*;
pub unsafe fn validop(op: i32, v1: *mut TValue, v2: *mut TValue) -> i32 {
    unsafe {
        const BITWISE_AND: i32 = OperatorBinary::BitwiseAnd as i32;
        const BITWISE_OR: i32 = OperatorBinary::BitwiseOr as i32;
        const BITWISE_XOR: i32 = OperatorBinary::BitwiseExclusiveOr as i32;
        const SHIFT_LEFT: i32 = OperatorBinary::ShiftLeft as i32;
        const SHIFT_RIGHT: i32 = OperatorBinary::ShiftRight as i32;
        const EQUAL: i32 = OperatorBinary::Equal as i32;
        const DIVIDE: i32 = OperatorBinary::Divide as i32;
        const INTEGRAL_DIVIDE: i32 = OperatorBinary::IntegralDivide as i32;
        const MODULUS: i32 = OperatorBinary::Modulus as i32;
        match op {
            BITWISE_AND | BITWISE_OR | BITWISE_XOR | SHIFT_LEFT | SHIFT_RIGHT | EQUAL => {
                let mut i: i64 = 0;
                (F2I::Equal.convert_tv_i64(v1, &mut i) != 0 && F2I::Equal.convert_tv_i64(v2, &mut i) != 0) as i32
            }
            DIVIDE | INTEGRAL_DIVIDE | MODULUS => ((*v2).as_float().unwrap() != 0.0) as i32,
            _ => 1,
        }
    }
}
pub unsafe fn binopr2op(binary: OperatorBinary, baser: OperatorBinary, base: u32) -> u32 {
    (binary as i32 - baser as i32 + base as i32) as u32
}
pub unsafe fn unopr2op(unary: OperatorUnary) -> u32 {
    (unary as i32 - OperatorUnary::Minus as i32 + OPCODE_UNM as i32) as u32
}
pub unsafe fn binopr2tm(binary: OperatorBinary) -> u32 {
    (binary as i32 - OperatorBinary::Add as i32 + TM_ADD as i32) as u32
}

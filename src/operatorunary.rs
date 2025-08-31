use crate::token::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub enum OperatorUnary {
    Minus,
    BitwiseNot,
    Not,
    Length,
    None_,
}
pub unsafe extern "C" fn getunopr(op: i32) -> OperatorUnary {
    match op {
        TK_NOT => return OperatorUnary::Not,
        45 => return OperatorUnary::Minus,
        126 => return OperatorUnary::BitwiseNot,
        35 => return OperatorUnary::Length,
        _ => return OperatorUnary::None_,
    };
}

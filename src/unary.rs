use crate::token::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub enum Unary {
    Minus,
    BitwiseNot,
    Not,
    Length,
    None_,
}
pub unsafe extern "C" fn getunopr(op: i32) -> Unary {
    match op {
        TK_NOT => return Unary::Not,
        45 => return Unary::Minus,
        126 => return Unary::BitwiseNot,
        35 => return Unary::Length,
        _ => return Unary::None_,
    };
}

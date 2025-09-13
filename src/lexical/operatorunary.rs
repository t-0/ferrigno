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
impl OperatorUnary {
    pub unsafe fn from_token(token: i32) -> Self {
        match token {
            TK_NOT => return OperatorUnary::Not,
            TK_CHARACTER_HYPHEN => return OperatorUnary::Minus,
            TK_CHARACTER_TILDE => return OperatorUnary::BitwiseNot,
            TK_CHARACTER_OCTOTHORPE => return OperatorUnary::Length,
            _ => return OperatorUnary::None_,
        };
    }
}

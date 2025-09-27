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
        const TK_NOT: i32 = Token::Not as i32;
        const TK_CHARACTER_HYPHEN: i32 = Token::CharacterHyphen as i32;
        const TK_CHARACTER_TILDE: i32 = Token::CharacterTilde as i32;
        const TK_CHARACTER_OCTOTHORPE: i32 = Token::CharacterOctothorpe as i32;
match token {
            TK_NOT => return OperatorUnary::Not,
            TK_CHARACTER_HYPHEN => return OperatorUnary::Minus,
            TK_CHARACTER_TILDE => return OperatorUnary::BitwiseNot,
            TK_CHARACTER_OCTOTHORPE => return OperatorUnary::Length,
            _ => return OperatorUnary::None_,
        };
    }
}

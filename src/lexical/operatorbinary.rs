use crate::token::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub enum OperatorBinary {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    Modulus = 3,
    Power = 4,
    Divide = 5,
    IntegralDivide = 6,
    BitwiseAnd = 7,
    BitwiseOr = 8,
    BitwiseExclusiveOr = 9,
    ShiftLeft = 10,
    ShiftRight = 11,
    Concatenate = 12,
    Equal = 13,
    Less = 14,
    LessEqual = 15,
    Inequal = 16,
    Greater = 17,
    GreaterEqual = 18,
    And = 19,
    Or = 20,
    NoBinaryOperation = 21,
}
impl OperatorBinary {
    pub unsafe fn from_token(token: i32) -> OperatorBinary {
        match token {
            TK_CHARACTER_PLUS => return OperatorBinary::Add,
            TK_CHARACTER_HYPHEN => return OperatorBinary::Subtract,
            TK_CHARACTER_ASTERISK => return OperatorBinary::Multiply,
            TK_CHARACTER_PERCENT => return OperatorBinary::Modulus,
            TK_CHARACTER_CARET => return OperatorBinary::Power,
            TK_CHARACTER_SOLIDUS => return OperatorBinary::Divide,
            TK_CHARACTER_ANGLE_LEFT => return OperatorBinary::Less,
            TK_CHARACTER_ANGLE_RIGHT => return OperatorBinary::Greater,
            TK_CHARACTER_AMPERSAND => return OperatorBinary::BitwiseAnd,
            TK_CHARACTER_BAR => return OperatorBinary::BitwiseOr,
            TK_CHARACTER_TILDE => return OperatorBinary::BitwiseExclusiveOr,
            TK_INTEGRALDIVIDE => return OperatorBinary::IntegralDivide,
            TK_SHIFTLEFT => return OperatorBinary::ShiftLeft,
            TK_SHIFTRIGHT => return OperatorBinary::ShiftRight,
            TK_CONCATENATE => return OperatorBinary::Concatenate,
            TK_INEQUAL => return OperatorBinary::Inequal,
            TK_EQUAL => return OperatorBinary::Equal,
            TK_LESSEQUAL => return OperatorBinary::LessEqual,
            TK_GREATEREQUAL => return OperatorBinary::GreaterEqual,
            TK_AND => return OperatorBinary::And,
            TK_OR => return OperatorBinary::Or,
            _ => return OperatorBinary::NoBinaryOperation,
        };
    }
}

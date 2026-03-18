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
        const TK_CHARACTER_HYPHEN: i32 = Token::CharacterHyphen as i32;
        const TK_CHARACTER_PLUS: i32 = Token::CharacterPlus as i32;
        const TK_CHARACTER_ASTERISK: i32 = Token::CharacterAsterisk as i32;
        const TK_CHARACTER_PERCENT: i32 = Token::CharacterPercent as i32;
        const TK_CHARACTER_CARET: i32 = Token::CharacterCaret as i32;
        const TK_CHARACTER_SOLIDUS: i32 = Token::CharacterSolidus as i32;
        const TK_CHARACTER_ANGLE_LEFT: i32 = Token::CharacterAngleLeft as i32;
        const TK_CHARACTER_ANGLE_RIGHT: i32 = Token::CharacterAngleRight as i32;
        const TK_CHARACTER_AMPERSAND: i32 = Token::CharacterAmpersand as i32;
        const TK_CHARACTER_BAR: i32 = Token::CharacterBar as i32;
        const TK_CHARACTER_TILDE: i32 = Token::CharacterTilde as i32;
        const TK_INTEGRALDIVIDE: i32 = Token::IntegralDivide as i32;
        const TK_SHIFTLEFT: i32 = Token::ShiftLeft as i32;
        const TK_SHIFTRIGHT: i32 = Token::ShiftRight as i32;
        const TK_CONCATENATE: i32 = Token::Concatenate as i32;
        const TK_INEQUAL: i32 = Token::Inequality as i32;
        const TK_EQUAL: i32 = Token::Equality as i32;
        const TK_LESSEQUAL: i32 = Token::LessEqual as i32;
        const TK_GREATEREQUAL: i32 = Token::GreaterEqual as i32;
        const TK_AND: i32 = Token::And as i32;
        const TK_OR: i32 = Token::Or as i32;
        match token {
            | TK_CHARACTER_PLUS => OperatorBinary::Add,
            | TK_CHARACTER_HYPHEN => OperatorBinary::Subtract,
            | TK_CHARACTER_ASTERISK => OperatorBinary::Multiply,
            | TK_CHARACTER_PERCENT => OperatorBinary::Modulus,
            | TK_CHARACTER_CARET => OperatorBinary::Power,
            | TK_CHARACTER_SOLIDUS => OperatorBinary::Divide,
            | TK_CHARACTER_ANGLE_LEFT => OperatorBinary::Less,
            | TK_CHARACTER_ANGLE_RIGHT => OperatorBinary::Greater,
            | TK_CHARACTER_AMPERSAND => OperatorBinary::BitwiseAnd,
            | TK_CHARACTER_BAR => OperatorBinary::BitwiseOr,
            | TK_CHARACTER_TILDE => OperatorBinary::BitwiseExclusiveOr,
            | TK_INTEGRALDIVIDE => OperatorBinary::IntegralDivide,
            | TK_SHIFTLEFT => OperatorBinary::ShiftLeft,
            | TK_SHIFTRIGHT => OperatorBinary::ShiftRight,
            | TK_CONCATENATE => OperatorBinary::Concatenate,
            | TK_INEQUAL => OperatorBinary::Inequal,
            | TK_EQUAL => OperatorBinary::Equal,
            | TK_LESSEQUAL => OperatorBinary::LessEqual,
            | TK_GREATEREQUAL => OperatorBinary::GreaterEqual,
            | TK_AND => OperatorBinary::And,
            | TK_OR => OperatorBinary::Or,
            | _ => OperatorBinary::NoBinaryOperation,
        }
    }
}

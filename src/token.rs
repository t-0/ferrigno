use crate::character::*;
use crate::new::*;
use crate::value::*;
use rlua::*;
use std::ptr::*;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum Token {
    CharacterHyphen = Character::Hyphen as i32,
    CharacterTilde = Character::Tilde as i32,
    CharacterEqual = Character::Equal as i32,
    CharacterComma = Character::Comma as i32,
    CharacterColon = Character::Colon as i32,
    CharacterPeriod = Character::Period as i32,
    CharacterBracketLeft = Character::BracketLeft as i32,
    CharacterSemicolon = Character::Semicolon as i32,
    CharacterOctothorpe = Character::Octothorpe as i32,
    CharacterAmpersand = Character::Ampersand as i32,
    CharacterBar = Character::Bar as i32,
    CharacterAngleLeft = Character::AngleLeft as i32,
    CharacterBraceLeft = Character::BraceLeft as i32,
    CharacterParenthesisLeft = Character::ParenthesisLeft as i32,
    CharacterAngleRight = Character::AngleRight as i32,
    CharacterPlus = Character::Plus as i32,
    CharacterAsterisk = Character::Asterisk as i32,
    CharacterPercent = Character::Percent as i32,
    CharacterCaret = Character::Caret as i32,
    CharacterSolidus = Character::Solidus as i32,
    And = 256,
    Break = 257,
    Do = 258,
    Else = 259,
    Elseif = 260,
    End = 261,
    False = 262,
    For = 263,
    Function = 264,
    Goto = 265,
    If = 266,
    In = 267,
    Local = 268,
    Nil = 269,
    Not = 270,
    Or = 271,
    Repeat = 272,
    Return = 273,
    Then = 274,
    True = 275,
    Until = 276,
    While = 277,
    IntegralDivide = 278,
    Concatenate = 279,
    Dots = 280,
    Equal = 281,
    GreaterEqual = 282,
    LessEqual = 283,
    Inequal = 284,
    ShiftLeft = 285,
    ShiftRight = 286,
    Dbcolon = 287,
    EndOfStream = 288,
    Float = 289,
    Integer = 290,
    Name = 291,
    String = 292,
}
impl Token {
    pub fn from (character: i32) -> Token {
        const CharacterHyphen: i32 = Token::CharacterHyphen as i32;
        const CharacterTilde: i32 = Token::CharacterTilde as i32;
        const CharacterEqual: i32 = Token::CharacterEqual as i32;
        const CharacterComma: i32 = Token::CharacterComma as i32;
        const CharacterColon: i32 = Token::CharacterColon as i32;
        const CharacterPeriod: i32 = Token::CharacterPeriod as i32;
        const CharacterBracketLeft: i32 = Token::CharacterBracketLeft as i32;
        const CharacterSemicolon: i32 = Token::CharacterSemicolon as i32;
        const CharacterOctothorpe: i32 = Token::CharacterOctothorpe as i32;
        const CharacterAmpersand: i32 = Token::CharacterAmpersand as i32;
        const CharacterBar: i32 = Token::CharacterBar as i32;
        const CharacterAngleLeft: i32 = Token::CharacterAngleLeft as i32;
        const CharacterBraceLeft: i32 = Token::CharacterBraceLeft as i32;
        const CharacterParenthesisLeft: i32 = Token::CharacterParenthesisLeft as i32;
        const CharacterAngleRight: i32 = Token::CharacterAngleRight as i32;
        const CharacterPlus: i32 = Token::CharacterPlus as i32;
        const CharacterAsterisk: i32 = Token::CharacterAsterisk as i32;
        const CharacterPercent: i32 = Token::CharacterPercent as i32;
        const CharacterCaret: i32 = Token::CharacterCaret as i32;
        const CharacterSolidus: i32 = Token::CharacterSolidus as i32;
        match character {
            CharacterHyphen => Token::CharacterHyphen,
            CharacterTilde => Token::CharacterTilde,
            CharacterEqual => Token::CharacterEqual,
            CharacterComma => Token::CharacterComma,
            CharacterColon => Token::CharacterColon,
            CharacterPeriod => Token::CharacterPeriod,
            CharacterBracketLeft => Token::CharacterBracketLeft,
            CharacterSemicolon => Token::CharacterSemicolon,
            CharacterOctothorpe => Token::CharacterOctothorpe,
            CharacterAmpersand => Token::CharacterAmpersand,
            CharacterBar => Token::CharacterBar,
            CharacterAngleLeft => Token::CharacterAngleLeft,
            CharacterBraceLeft => Token::CharacterBraceLeft,
            CharacterParenthesisLeft => Token::CharacterParenthesisLeft,
            CharacterAngleRight => Token::CharacterAngleRight,
            CharacterPlus => Token::CharacterPlus,
            CharacterAsterisk => Token::CharacterAsterisk,
            CharacterPercent => Token::CharacterPercent,
            CharacterCaret => Token::CharacterCaret,
            CharacterSolidus => Token::CharacterSolidus,
            256 => Token::And,
            257 => Token::Break,
            258 => Token::Do,
            259 => Token::Else,
            260 => Token::Elseif,
            261 => Token::End,
            262 => Token::False,
            263 => Token::For,
            264 => Token::Function,
            265 => Token::Goto,
            266 => Token::If,
            267 => Token::In,
            268 => Token::Local,
            269 => Token::Nil,
            270 => Token::Not,
            271 => Token::Or,
            272 => Token::Repeat,
            273 => Token::Return,
            274 => Token::Then,
            275 => Token::True,
            276 => Token::Until,
            277 => Token::While,
            278 => Token::IntegralDivide,
            279 => Token::Concatenate,
            280 => Token::Dots,
            281 => Token::Equal,
            282 => Token::GreaterEqual,
            283 => Token::LessEqual,
            284 => Token::Inequal,
            285 => Token::ShiftLeft,
            286 => Token::ShiftRight,
            287 => Token::Dbcolon,
            288 => Token::EndOfStream,
            289 => Token::Float,
            290 => Token::Integer,
            291 => Token::Name,
            292 => Token::String,
            _ => Token::Nil,
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenInfo {
    pub token: i32,
    pub semantic_info: Value,
}
impl New for TokenInfo {
    fn new() -> Self {
        return TokenInfo { token: 0, semantic_info: Value::new_object(null_mut()) };
    }
}
pub const TK_CHARACTER_HYPHEN: i32 = Character::Hyphen as i32;
pub const TK_CHARACTER_TILDE: i32 = Character::Tilde as i32;
pub const TK_CHARACTER_EQUAL: i32 = Character::Equal as i32;
pub const TK_CHARACTER_COMMA: i32 = Character::Comma as i32;
pub const TK_CHARACTER_COLON: i32 = Character::Colon as i32;
pub const TK_CHARACTER_PERIOD: i32 = Character::Period as i32;
pub const TK_CHARACTER_BRACKET_LEFT: i32 = Character::BracketLeft as i32;
pub const TK_CHARACTER_SEMICOLON: i32 = Character::Semicolon as i32;
pub const TK_CHARACTER_OCTOTHORPE: i32 = Character::Octothorpe as i32;
pub const TK_CHARACTER_AMPERSAND: i32 = Character::Ampersand as i32;
pub const TK_CHARACTER_BAR: i32 = Character::Bar as i32;
pub const TK_CHARACTER_ANGLE_LEFT: i32 = Character::AngleLeft as i32;
pub const TK_CHARACTER_BRACE_LEFT: i32 = Character::BraceLeft as i32;
pub const TK_CHARACTER_PARENTHESIS_LEFT: i32 = Character::ParenthesisLeft as i32;
pub const TK_CHARACTER_ANGLE_RIGHT: i32 = Character::AngleRight as i32;
pub const TK_CHARACTER_PLUS: i32 = Character::Plus as i32;
pub const TK_CHARACTER_ASTERISK: i32 = Character::Asterisk as i32;
pub const TK_CHARACTER_PERCENT: i32 = Character::Percent as i32;
pub const TK_CHARACTER_CARET: i32 = Character::Caret as i32;
pub const TK_CHARACTER_SOLIDUS: i32 = Character::Solidus as i32;
pub const TK_WHILE: i32 = 277;
pub const TK_ENDOFSTREAM: i32 = 288;
pub const TK_INTEGER: i32 = 290;
pub const TK_FLOAT: i32 = 289;
pub const TK_STRING: i32 = 292;
pub const TK_NAME: i32 = 291;
pub const TK_CONCATENATE: i32 = 279;
pub const TK_DOTS: i32 = 280;
pub const TK_DOUBLECOLON: i32 = 287;
pub const TK_INEQUAL: i32 = 284;
pub const TK_INTEGRALDIVIDE: i32 = 278;
pub const TK_SHIFTRIGHT: i32 = 286;
pub const TK_GREATEREQUAL: i32 = 282;
pub const TK_SHIFTLEFT: i32 = 285;
pub const TK_LESSEQUAL: i32 = 283;
pub const TK_EQUAL: i32 = 281;
pub const TK_OR: i32 = 271;
pub const TK_AND: i32 = 256;
pub const TK_FUNCTION: i32 = 264;
pub const TK_END: i32 = 261;
pub const TK_FALSE: i32 = 262;
pub const TK_TRUE: i32 = 275;
pub const TK_NIL: i32 = 269;
pub const TK_NOT: i32 = 270;
pub const TK_GOTO: i32 = 265;
pub const TK_BREAK: i32 = 257;
pub const TK_UNTIL: i32 = 276;
pub const TK_ELSEIF: i32 = 260;
pub const TK_ELSE: i32 = 259;
pub const TK_RETURN: i32 = 273;
pub const TK_LOCAL: i32 = 268;
pub const TK_REPEAT: i32 = 272;
pub const TK_FOR: i32 = 263;
pub const TK_DO: i32 = 258;
pub const TK_IN: i32 = 267;
pub const TK_IF: i32 = 266;
pub const TK_THEN: i32 = 274;
pub const TOKENS: [*const i8; 37] = [
    make_cstring!("and"),
    make_cstring!("break"),
    make_cstring!("do"),
    make_cstring!("else"),
    make_cstring!("elseif"),
    make_cstring!("end"),
    make_cstring!("false"),
    make_cstring!("for"),
    make_cstring!("function"),
    make_cstring!("goto"),
    make_cstring!("if"),
    make_cstring!("in"),
    make_cstring!("local"),
    make_cstring!("nil"),
    make_cstring!("not"),
    make_cstring!("or"),
    make_cstring!("repeat"),
    make_cstring!("return"),
    make_cstring!("then"),
    make_cstring!("true"),
    make_cstring!("until"),
    make_cstring!("while"),
    make_cstring!("//"),
    make_cstring!(".."),
    make_cstring!("..."),
    make_cstring!("=="),
    make_cstring!(">="),
    make_cstring!("<="),
    make_cstring!("~="),
    make_cstring!("<<"),
    make_cstring!(">>"),
    make_cstring!("::"),
    make_cstring!("<eof>"),
    make_cstring!("<number>"),
    make_cstring!("<integer>"),
    make_cstring!("<name>"),
    make_cstring!("<string>"),
];

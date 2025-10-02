#![allow(unused,dead_code)]
use crate::character::*;
use crate::value::*;
use crate::tdefaultnew::*;

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
    Equality = 281,
    GreaterEqual = 282,
    LessEqual = 283,
    Inequality = 284,
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
        const CHARACTERHYPHEN: i32 = Token::CharacterHyphen as i32;
        const CHARACTERTILDE: i32 = Token::CharacterTilde as i32;
        const CHARACTEREQUAL: i32 = Token::CharacterEqual as i32;
        const CHARACTERCOMMA: i32 = Token::CharacterComma as i32;
        const CHARACTERCOLON: i32 = Token::CharacterColon as i32;
        const CHARACTERPERIOD: i32 = Token::CharacterPeriod as i32;
        const CHARACTERBRACKETLEFT: i32 = Token::CharacterBracketLeft as i32;
        const CHARACTERSEMICOLON: i32 = Token::CharacterSemicolon as i32;
        const CHARACTEROCTOTHORPE: i32 = Token::CharacterOctothorpe as i32;
        const CHARACTERAMPERSAND: i32 = Token::CharacterAmpersand as i32;
        const CHARACTERBAR: i32 = Token::CharacterBar as i32;
        const CHARACTERANGLELEFT: i32 = Token::CharacterAngleLeft as i32;
        const CHARACTERBRACELEFT: i32 = Token::CharacterBraceLeft as i32;
        const CHARACTERPARENTHESISLEFT: i32 = Token::CharacterParenthesisLeft as i32;
        const CHARACTERANGLERIGHT: i32 = Token::CharacterAngleRight as i32;
        const CHARACTERPLUS: i32 = Token::CharacterPlus as i32;
        const CHARACTERASTERISK: i32 = Token::CharacterAsterisk as i32;
        const CHARACTERPERCENT: i32 = Token::CharacterPercent as i32;
        const CHARACTERCARET: i32 = Token::CharacterCaret as i32;
        const CHARACTERSOLIDUS: i32 = Token::CharacterSolidus as i32;
        match character {
            CHARACTERHYPHEN => Token::CharacterHyphen,
            CHARACTERTILDE => Token::CharacterTilde,
            CHARACTEREQUAL => Token::CharacterEqual,
            CHARACTERCOMMA => Token::CharacterComma,
            CHARACTERCOLON => Token::CharacterColon,
            CHARACTERPERIOD => Token::CharacterPeriod,
            CHARACTERBRACKETLEFT => Token::CharacterBracketLeft,
            CHARACTERSEMICOLON => Token::CharacterSemicolon,
            CHARACTEROCTOTHORPE => Token::CharacterOctothorpe,
            CHARACTERAMPERSAND => Token::CharacterAmpersand,
            CHARACTERBAR => Token::CharacterBar,
            CHARACTERANGLELEFT => Token::CharacterAngleLeft,
            CHARACTERBRACELEFT => Token::CharacterBraceLeft,
            CHARACTERPARENTHESISLEFT => Token::CharacterParenthesisLeft,
            CHARACTERANGLERIGHT => Token::CharacterAngleRight,
            CHARACTERPLUS => Token::CharacterPlus,
            CHARACTERASTERISK => Token::CharacterAsterisk,
            CHARACTERPERCENT => Token::CharacterPercent,
            CHARACTERCARET => Token::CharacterCaret,
            CHARACTERSOLIDUS => Token::CharacterSolidus,
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
            281 => Token::Equality,
            282 => Token::GreaterEqual,
            283 => Token::LessEqual,
            284 => Token::Inequality,
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
impl TDefaultNew for TokenInfo {
    fn new() -> Self {
        return TokenInfo { token: 0, semantic_info: Value::new_object(null_mut()) };
    }
}
impl TokenInfo {
}
pub const TOKENS: [*const i8; 37] = [
    c"and".as_ptr(),
    c"break".as_ptr(),
    c"do".as_ptr(),
    c"else".as_ptr(),
    c"elseif".as_ptr(),
    c"end".as_ptr(),
    c"false".as_ptr(),
    c"for".as_ptr(),
    c"function".as_ptr(),
    c"goto".as_ptr(),
    c"if".as_ptr(),
    c"in".as_ptr(),
    c"local".as_ptr(),
    c"nil".as_ptr(),
    c"not".as_ptr(),
    c"or".as_ptr(),
    c"repeat".as_ptr(),
    c"return".as_ptr(),
    c"then".as_ptr(),
    c"true".as_ptr(),
    c"until".as_ptr(),
    c"while".as_ptr(),
    c"//".as_ptr(),
    c"..".as_ptr(),
    c"...".as_ptr(),
    c"==".as_ptr(),
    c">=".as_ptr(),
    c"<=".as_ptr(),
    c"~=".as_ptr(),
    c"<<".as_ptr(),
    c">>".as_ptr(),
    c"::".as_ptr(),
    c"<eof>".as_ptr(),
    c"<number>".as_ptr(),
    c"<integer>".as_ptr(),
    c"<name>".as_ptr(),
    c"<string>".as_ptr(),
];

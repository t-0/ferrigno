#![allow(unused, dead_code)]
use crate::character::*;
use crate::tdefaultnew::*;
use crate::value::*;

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
    Global = 265,
    Goto = 266,
    If = 267,
    In = 268,
    Local = 269,
    Nil = 270,
    Not = 271,
    Or = 272,
    Repeat = 273,
    Return = 274,
    Then = 275,
    True = 276,
    Until = 277,
    While = 278,
    IntegralDivide = 279,
    Concatenate = 280,
    Dots = 281,
    Equality = 282,
    GreaterEqual = 283,
    LessEqual = 284,
    Inequality = 285,
    ShiftLeft = 286,
    ShiftRight = 287,
    Dbcolon = 288,
    EndOfStream = 289,
    Float = 290,
    Integer = 291,
    Name = 292,
    String = 293,
}
impl Token {
    pub fn from(character: i32) -> Token {
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
            265 => Token::Global,
            266 => Token::Goto,
            267 => Token::If,
            268 => Token::In,
            269 => Token::Local,
            270 => Token::Nil,
            271 => Token::Not,
            272 => Token::Or,
            273 => Token::Repeat,
            274 => Token::Return,
            275 => Token::Then,
            276 => Token::True,
            277 => Token::Until,
            278 => Token::While,
            279 => Token::IntegralDivide,
            280 => Token::Concatenate,
            281 => Token::Dots,
            282 => Token::Equality,
            283 => Token::GreaterEqual,
            284 => Token::LessEqual,
            285 => Token::Inequality,
            286 => Token::ShiftLeft,
            287 => Token::ShiftRight,
            288 => Token::Dbcolon,
            289 => Token::EndOfStream,
            290 => Token::Float,
            291 => Token::Integer,
            292 => Token::Name,
            293 => Token::String,
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
        TokenInfo {
            token: 0,
            semantic_info: Value::new_object(null_mut()),
        }
    }
}
impl TokenInfo {}
pub const TOKENS: [*const i8; 38] = [
    c"and".as_ptr(),
    c"break".as_ptr(),
    c"do".as_ptr(),
    c"else".as_ptr(),
    c"elseif".as_ptr(),
    c"end".as_ptr(),
    c"false".as_ptr(),
    c"for".as_ptr(),
    c"function".as_ptr(),
    c"global".as_ptr(),
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

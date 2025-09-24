use crate::character::*;
use crate::new::*;
use crate::value::*;
use rlua::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: i32,
    pub semantic_info: Value,
}
impl New for Token {
    fn new() -> Self {
        return Token { token: 0, semantic_info: Value::new_object(null_mut()) };
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
pub const TK_EOS: i32 = 288;
pub const TK_INT: i32 = 290;
pub const TK_FLT: i32 = 289;
pub const TK_STRING: i32 = 292;
pub const TK_NAME: i32 = 291;
pub const TK_CONCAT: i32 = 279;
pub const TK_DOTS: i32 = 280;
pub const TK_DBCOLON: i32 = 287;
pub const TK_NE: i32 = 284;
pub const TK_IDIV: i32 = 278;
pub const TK_SHR: i32 = 286;
pub const TK_GE: i32 = 282;
pub const TK_SHL: i32 = 285;
pub const TK_LE: i32 = 283;
pub const TK_EQ: i32 = 281;
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

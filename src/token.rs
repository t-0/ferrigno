use crate::new::*;
use crate::character::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: i32,
    pub semantic_info: Value,
}
impl New for Token {
    fn new() -> Self {
        return Token {
            token: 0,
            semantic_info: Value::new(),
        };
    }
}
pub const TK_CHARACTER_HYPHEN: i32 = CHARACTER_HYPHEN;
pub const TK_CHARACTER_TILDE: i32 = CHARACTER_TILDE;
pub const TK_CHARACTER_OCTOTHORPE: i32 = CHARACTER_OCTOTHORPE;
pub const TK_CHARACTER_AMPERSAND: i32 = CHARACTER_AMPERSAND;
pub const TK_CHARACTER_BAR: i32 = CHARACTER_BAR;
pub const TK_CHARACTER_ANGLE_LEFT: i32 = CHARACTER_ANGLE_LEFT;
pub const TK_CHARACTER_ANGLE_RIGHT: i32 = CHARACTER_ANGLE_RIGHT;
pub const TK_CHARACTER_PLUS: i32 = CHARACTER_PLUS;
pub const TK_CHARACTER_ASTERISK: i32 = CHARACTER_ASTERISK;
pub const TK_CHARACTER_PERCENT: i32 = CHARACTER_PERCENT;
pub const TK_CHARACTER_CARET: i32 = CHARACTER_CARET;
pub const TK_CHARACTER_SOLIDUS: i32 = CHARACTER_SOLIDUS;
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
pub const TOKENS: [*const libc::c_char; 37] = [
    b"and\0" as *const u8 as *const libc::c_char,
    b"break\0" as *const u8 as *const libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char,
    b"elseif\0" as *const u8 as *const libc::c_char,
    b"end\0" as *const u8 as *const libc::c_char,
    b"false\0" as *const u8 as *const libc::c_char,
    b"for\0" as *const u8 as *const libc::c_char,
    b"function\0" as *const u8 as *const libc::c_char,
    b"goto\0" as *const u8 as *const libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char,
    b"in\0" as *const u8 as *const libc::c_char,
    b"local\0" as *const u8 as *const libc::c_char,
    b"nil\0" as *const u8 as *const libc::c_char,
    b"not\0" as *const u8 as *const libc::c_char,
    b"or\0" as *const u8 as *const libc::c_char,
    b"repeat\0" as *const u8 as *const libc::c_char,
    b"return\0" as *const u8 as *const libc::c_char,
    b"then\0" as *const u8 as *const libc::c_char,
    b"true\0" as *const u8 as *const libc::c_char,
    b"until\0" as *const u8 as *const libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char,
    b"//\0" as *const u8 as *const libc::c_char,
    b"..\0" as *const u8 as *const libc::c_char,
    b"...\0" as *const u8 as *const libc::c_char,
    b"==\0" as *const u8 as *const libc::c_char,
    b">=\0" as *const u8 as *const libc::c_char,
    b"<=\0" as *const u8 as *const libc::c_char,
    b"~=\0" as *const u8 as *const libc::c_char,
    b"<<\0" as *const u8 as *const libc::c_char,
    b">>\0" as *const u8 as *const libc::c_char,
    b"::\0" as *const u8 as *const libc::c_char,
    b"<eof>\0" as *const u8 as *const libc::c_char,
    b"<number>\0" as *const u8 as *const libc::c_char,
    b"<integer>\0" as *const u8 as *const libc::c_char,
    b"<name>\0" as *const u8 as *const libc::c_char,
    b"<string>\0" as *const u8 as *const libc::c_char,
];

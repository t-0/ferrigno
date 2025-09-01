use crate::token::*;
use crate::operator_::*;
pub unsafe extern "C" fn getbinopr(op: i32) -> u32 {
    match op {
        TK_CHARACTER_PLUS => return OPR_ADD,
        TK_CHARACTER_HYPHEN => return OPR_SUB,
        TK_CHARACTER_ASTERISK => return OPR_MUL,
        TK_CHARACTER_PERCENT => return OPR_MOD,
        TK_CHARACTER_CARET => return OPR_POW,
        TK_CHARACTER_SOLIDUS => return OPR_DIV,
        TK_CHARACTER_ANGLE_LEFT => return OPR_LT,
        TK_CHARACTER_ANGLE_RIGHT => return OPR_GT,
        TK_CHARACTER_AMPERSAND => return OPR_BAND,
        TK_CHARACTER_BAR => return OPR_BOR,
        TK_CHARACTER_TILDE => return OPR_BXOR,
        TK_IDIV => return OPR_IDIV,
        TK_SHL => return OPR_SHL,
        TK_SHR => return OPR_SHR,
        TK_CONCAT => return OPR_CONCAT,
        TK_NE => return OPR_NE,
        TK_EQ => return OPR_EQ,
        TK_LE => return OPR_LE,
        TK_GE => return OPR_GE,
        TK_AND => return OPR_AND,
        TK_OR => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}

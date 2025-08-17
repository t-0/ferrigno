#![allow(
    static_mut_refs,
    unsafe_code,
    unsafe_attr_outside_unsafe,
    unsafe_op_in_unsafe_fn,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unpredictable_function_pointer_comparisons,
    unused_imports,
)]
use libc::{tolower, toupper, remove, rename, setlocale};
use crate::c::*;
use crate::state::*;
use crate::gcobject::*;
use crate::lua_debug::*;
use crate::tm::*;
use crate::tstring::*;
use crate::lexstate::*;
use crate::mbuffer::*;
use crate::sparser::*;
use crate::blockcnt::*;
use crate::callinfo::*;
use crate::stkidrel::*;
use crate::node::*;
use crate::table::*;
use crate::tstring::*;
use crate::lg::*;
use crate::lx::*;
use crate::proto::*;
use crate::gcunion::*;
use crate::udata::*;
use crate::closure::*;
use crate::locvar::*;
use crate::abslineinfo::*;
use crate::calls::*;
use crate::zio::*;
use crate::upvaldesc::*;
use crate::lua_reader::*;
use crate::lua_writer::*;
use crate::bufffs::*;
use crate::closep::*;
use crate::instruction::*;
use crate::dyndata::*;
use crate::labellist::*;
use crate::labeldesc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: i32,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: f64,
    pub i: i64,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub left: u8,
    pub right: u8,
}
pub const TK_WHILE: u32 = 277;
pub const TK_EOS: u32 = 288;
pub const TK_INT: u32 = 290;
pub const TK_FLT: u32 = 289;
pub const TK_STRING: u32 = 292;
pub const TK_NAME: u32 = 291;
pub const TK_CONCAT: u32 = 279;
pub const TK_DOTS: u32 = 280;
pub const TK_DBCOLON: u32 = 287;
pub const TK_NE: u32 = 284;
pub const TK_IDIV: u32 = 278;
pub const TK_SHR: u32 = 286;
pub const TK_GE: u32 = 282;
pub const TK_SHL: u32 = 285;
pub const TK_LE: u32 = 283;
pub const TK_EQ: u32 = 281;
pub const TK_OR: u32 = 271;
pub const TK_AND: u32 = 256;
pub const TK_FUNCTION: u32 = 264;
pub const TK_END: u32 = 261;
pub const TK_FALSE: u32 = 262;
pub const TK_TRUE: u32 = 275;
pub const TK_NIL: u32 = 269;
pub const OPR_NOUNOPR: u32 = 4;
pub const OPR_LEN: u32 = 3;
pub const OPR_NOT: u32 = 2;
pub const OPR_BNOT: u32 = 1;
pub const OPR_MINUS: u32 = 0;
pub const TK_NOT: u32 = 270;
pub const TK_GOTO: u32 = 265;
pub const TK_BREAK: u32 = 257;
pub const TK_UNTIL: u32 = 276;
pub const TK_ELSEIF: u32 = 260;
pub const TK_ELSE: u32 = 259;
pub const TK_RETURN: u32 = 273;
pub const TK_LOCAL: u32 = 268;
pub const TK_REPEAT: u32 = 272;
pub const TK_FOR: u32 = 263;
pub const TK_DO: u32 = 258;
pub const TK_IN: u32 = 267;
pub const TK_IF: u32 = 266;
pub const TK_THEN: u32 = 274;

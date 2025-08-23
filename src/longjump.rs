use crate::c::*;
use crate::callinfo::*;
use crate::debug::*;
use crate::object::*;
use crate::stkidrel::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}

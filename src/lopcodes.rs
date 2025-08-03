#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type lu_byte = libc::c_uchar;
pub type OpMode = libc::c_uint;
pub const isJ: OpMode = 4;
pub const iAx: OpMode = 3;
pub const iAsBx: OpMode = 2;
pub const iABx: OpMode = 1;
pub const iABC: OpMode = 0;
#[unsafe (no_mangle)]
pub static mut luaP_opmodes: [lu_byte; 83] = [
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iAsBx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iAsBx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((1 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((1 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((1 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | isJ as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABx as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | iABC as libc::c_int) as lu_byte,
    ((0 as libc::c_int) << 7 as libc::c_int | (0 as libc::c_int) << 6 as libc::c_int
        | (0 as libc::c_int) << 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (0 as libc::c_int) << 3 as libc::c_int | iAx as libc::c_int) as lu_byte,
];

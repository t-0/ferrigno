#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type OpMode = libc::c_uint;
pub const isJ: OpMode = 4;
pub const iAx: OpMode = 3;
pub const iAsBx: OpMode = 2;
pub const iABx: OpMode = 1;
pub const iABC: OpMode = 0;
#[unsafe (no_mangle)]
pub static mut luaP_opmodes: [u8; 83] = [
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iAsBx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iAsBx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((1 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((1 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((1 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | isJ as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (1 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (1 as i32) << 6 as i32
        | (1 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (1 as i32) << 6 as i32
        | (1 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (1 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (1 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABx as i32) as u8,
    ((0 as i32) << 7 as i32 | (1 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (1 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 3 as i32 | iABC as i32) as u8,
    ((0 as i32) << 7 as i32 | (0 as i32) << 6 as i32
        | (0 as i32) << 5 as i32 | (0 as i32) << 4 as i32
        | (0 as i32) << 3 as i32 | iAx as i32) as u8,
];

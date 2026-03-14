use crate::instruction::*;

pub const OPMODE_A: i32 = 1 << 3;
pub const OPMODE_T: i32 = 1 << 4;
pub const OPMODE_IT: i32 = 1 << 5;
pub const OPMODE_OT: i32 = 1 << 6;
pub const OPMODE_MM: i32 = 1 << 7;

const fn opmode(mm: i32, ot: i32, it: i32, t: i32, a: i32, m: u32) -> u8 {
    (mm << 7 | ot << 6 | it << 5 | t << 4 | a << 3 | m as i32) as u8
}

pub const OPMODES: [u8; 85] = [
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IASBX),
    opmode(0, 0, 0, 0, 1, IASBX),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(1, 0, 0, 0, 0, IABC),
    opmode(1, 0, 0, 0, 0, IABC),
    opmode(1, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 1, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, ISJ),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 0, IABC),
    opmode(0, 0, 0, 1, 1, IABC),
    opmode(0, 1, 1, 0, 1, IABC),
    opmode(0, 1, 1, 0, 1, IABC),
    opmode(0, 0, 1, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 0, 0, 0, 0, IABX),
    opmode(0, 0, 0, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 0, 1, 0, 0, IABC),
    opmode(0, 0, 0, 0, 1, IABX),
    opmode(0, 1, 0, 0, 1, IABC), // VARARG
    opmode(0, 0, 0, 0, 1, IABC), // GETVARG
    opmode(0, 0, 0, 0, 0, IABX), // ERRNNIL
    opmode(0, 0, 1, 0, 1, IABC), // VARARGPREP
    opmode(0, 0, 0, 0, 0, IAX),  // EXTRAARG
];

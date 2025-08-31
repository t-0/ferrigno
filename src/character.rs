#![allow(unused)]
pub const CHARACTER_EXCLAMATION: i32 = '!' as i32;
pub const CHARACTER_COLON: i32 = ':' as i32;
pub const CHARACTER_SEMICOLON: i32 = ';' as i32;
pub const CHARACTER_PARENTHESIS_LEFT: i32 = '(' as i32;
pub const CHARACTER_PARENTHESIS_RIGHT: i32 = ')' as i32;
pub const CHARACTER_BRACE_LEFT: i32 = '{' as i32;
pub const CHARACTER_BRACE_RIGHT: i32 = '}' as i32;
pub const CHARACTER_BRACKET_LEFT: i32 = '[' as i32;
pub const CHARACTER_BRACKET_RIGHT: i32 = ']' as i32;
pub const CHARACTER_0: i32 = '0' as i32;
pub const CHARACTER_1: i32 = '1' as i32;
pub const CHARACTER_2: i32 = '2' as i32;
pub const CHARACTER_3: i32 = '3' as i32;
pub const CHARACTER_4: i32 = '4' as i32;
pub const CHARACTER_5: i32 = '5' as i32;
pub const CHARACTER_6: i32 = '6' as i32;
pub const CHARACTER_7: i32 = '7' as i32;
pub const CHARACTER_8: i32 = '8' as i32;
pub const CHARACTER_9: i32 = '9' as i32;
pub const CHARACTER_UPPER_A: i32 = 'A' as i32;
pub const CHARACTER_UPPER_B: i32 = 'B' as i32;
pub const CHARACTER_UPPER_C: i32 = 'C' as i32;
pub const CHARACTER_UPPER_D: i32 = 'D' as i32;
pub const CHARACTER_UPPER_E: i32 = 'E' as i32;
pub const CHARACTER_UPPER_F: i32 = 'F' as i32;
pub const CHARACTER_UPPER_G: i32 = 'G' as i32;
pub const CHARACTER_UPPER_H: i32 = 'H' as i32;
pub const CHARACTER_UPPER_I: i32 = 'I' as i32;
pub const CHARACTER_UPPER_J: i32 = 'J' as i32;
pub const CHARACTER_UPPER_K: i32 = 'K' as i32;
pub const CHARACTER_UPPER_L: i32 = 'L' as i32;
pub const CHARACTER_UPPER_M: i32 = 'M' as i32;
pub const CHARACTER_UPPER_N: i32 = 'N' as i32;
pub const CHARACTER_UPPER_O: i32 = 'O' as i32;
pub const CHARACTER_UPPER_P: i32 = 'P' as i32;
pub const CHARACTER_UPPER_Q: i32 = 'Q' as i32;
pub const CHARACTER_UPPER_R: i32 = 'R' as i32;
pub const CHARACTER_UPPER_S: i32 = 'S' as i32;
pub const CHARACTER_UPPER_T: i32 = 'T' as i32;
pub const CHARACTER_UPPER_U: i32 = 'U' as i32;
pub const CHARACTER_UPPER_V: i32 = 'V' as i32;
pub const CHARACTER_UPPER_W: i32 = 'W' as i32;
pub const CHARACTER_UPPER_X: i32 = 'X' as i32;
pub const CHARACTER_UPPER_Y: i32 = 'Y' as i32;
pub const CHARACTER_UPPER_Z: i32 = 'Z' as i32;
pub const CHARACTER_LOWER_A: i32 = 'a' as i32;
pub const CHARACTER_LOWER_B: i32 = 'b' as i32;
pub const CHARACTER_LOWER_C: i32 = 'c' as i32;
pub const CHARACTER_LOWER_D: i32 = 'd' as i32;
pub const CHARACTER_LOWER_E: i32 = 'e' as i32;
pub const CHARACTER_LOWER_F: i32 = 'f' as i32;
pub const CHARACTER_LOWER_G: i32 = 'g' as i32;
pub const CHARACTER_LOWER_H: i32 = 'h' as i32;
pub const CHARACTER_LOWER_I: i32 = 'i' as i32;
pub const CHARACTER_LOWER_J: i32 = 'j' as i32;
pub const CHARACTER_LOWER_K: i32 = 'k' as i32;
pub const CHARACTER_LOWER_L: i32 = 'l' as i32;
pub const CHARACTER_LOWER_M: i32 = 'm' as i32;
pub const CHARACTER_LOWER_N: i32 = 'n' as i32;
pub const CHARACTER_LOWER_O: i32 = 'o' as i32;
pub const CHARACTER_LOWER_P: i32 = 'p' as i32;
pub const CHARACTER_LOWER_Q: i32 = 'q' as i32;
pub const CHARACTER_LOWER_R: i32 = 'r' as i32;
pub const CHARACTER_LOWER_S: i32 = 's' as i32;
pub const CHARACTER_LOWER_T: i32 = 't' as i32;
pub const CHARACTER_LOWER_U: i32 = 'u' as i32;
pub const CHARACTER_LOWER_V: i32 = 'v' as i32;
pub const CHARACTER_LOWER_W: i32 = 'w' as i32;
pub const CHARACTER_LOWER_X: i32 = 'x' as i32;
pub const CHARACTER_LOWER_Y: i32 = 'y' as i32;
pub const CHARACTER_LOWER_Z: i32 = 'z' as i32;
pub unsafe extern "C" fn luao_hexavalue(c: i32) -> i32 {
    unsafe {
        if CHARACTER_TYPE[(c + 1) as usize] as i32 & 1 << 1 != 0 {
            return c - '0' as i32;
        } else {
            return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10 as i32;
        };
    }
}
pub const CHARACTER_TYPE: [u8; 257] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x8 as u8,
    0x8 as u8,
    0x8 as u8,
    0x8 as u8,
    0x8 as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0xc as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x16 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x5 as u8,
    0x4 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x15 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x5 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0x4 as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn luao_utf8esc(buffer: *mut i8, mut x: u64) -> i32 {
    unsafe {
        let mut n: i32 = 1;
        if x < 0x80 as u64 {
            *buffer.offset((8 - 1) as isize) = x as i8;
        } else {
            let mut mfb: u32 = 0x3f as u32;
            loop {
                let fresh9 = n;
                n = n + 1;
                *buffer.offset((8 - fresh9) as isize) =
                    (0x80 as u64 | x & 0x3f as u64) as i8;
                x >>= 6;
                mfb >>= 1;
                if !(x > mfb as u64) {
                    break;
                }
            }
            *buffer.offset((8 - n) as isize) = ((!mfb << 1) as u64 | x) as i8;
        }
        return n;
    }
}

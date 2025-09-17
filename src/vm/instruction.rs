use crate::functionstate::*;
use crate::vm::opcode::*;
pub const IAX: u32 = 3;
pub const IABC: u32 = 0;
pub const IABX: u32 = 1;
pub const ISJ: u32 = 4;
pub const IASBX: u32 = 2;
pub unsafe fn filter_program_counter(program_counter: i32, jump_target: i32) -> i32 {
    if program_counter < jump_target {
        return -1;
    } else {
        return program_counter;
    };
}
pub unsafe fn final_target(code: *mut u32, mut index: i32) -> i32 {
    unsafe {
        for _ in 0..100 {
            let program_counter: u32 = *code.offset(index as isize);
            if (program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_JMP as u32 {
                break;
            }
            index += (program_counter >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1;
        }
        return index;
    }
}

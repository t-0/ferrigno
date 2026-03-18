use crate::functionstate::*;
use crate::opcode::*;
pub const IAX: u32 = 3;
pub const IABC: u32 = 0;
pub const IABX: u32 = 1;
pub const ISJ: u32 = 4;
pub const IASBX: u32 = 2;
pub unsafe fn filter_program_counter(program_counter: i32, jump_target: i32) -> i32 {
    if program_counter < jump_target { -1 } else { program_counter }
}
const MAX_JUMP_CHAIN: i32 = 100;
pub unsafe fn final_target(code: *mut u32, mut index: i32) -> i32 {
    unsafe {
        for _ in 0..MAX_JUMP_CHAIN {
            let program_counter: u32 = *code.add(index as usize);
            if (program_counter & MASK_OP) != OPCODE_JMP {
                break;
            }
            index += (program_counter >> POSITION_A & MASK_AX) as i32 - OFFSET_SJ + 1;
        }
        index
    }
}

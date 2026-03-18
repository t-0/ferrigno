#![allow(unpredictable_function_pointer_comparisons)]
use crate::closure::*;
use crate::functionstate::*;
use crate::instruction::*;
use crate::opcode::*;
use crate::opmode::*;
use crate::prototype::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tstring::*;
use crate::tvalue::*;
use std::io::Write;
use std::ptr::*;

const OPCODE_NAMES: [&str; 85] = [
    "MOVE", "LOADI", "LOADF", "LOADK", "LOADKX", "LOADFALSE", "LFALSESKIP", "LOADTRUE", "LOADNIL",
    "GETUPVAL", "SETUPVAL", "GETTABUP", "GETTABLE", "GETI", "GETFIELD", "SETTABUP", "SETTABLE",
    "SETI", "SETFIELD", "NEWTABLE", "SELF", "ADDI", "ADDK", "SUBK", "MULK", "MODK", "POWK",
    "DIVK", "IDIVK", "BANDK", "BORK", "BXORK", "SHRI", "SHLI", "ADD", "SUB", "MUL", "MOD", "POW",
    "DIV", "IDIV", "BAND", "BOR", "BXOR", "SHL", "SHR", "MMBIN", "MMBINI", "MMBINK", "UNM",
    "BNOT", "NOT", "LEN", "CONCAT", "CLOSE", "TBC", "JMP", "EQ", "LT", "LE", "EQK", "EQI", "LTI",
    "LEI", "GTI", "GEI", "TEST", "TESTSET", "CALL", "TAILCALL", "RETURN", "RETURN0", "RETURN1",
    "FORLOOP", "FORPREP", "TFORPREP", "TFORCALL", "TFORLOOP", "SETLIST", "CLOSURE", "VARARG",
    "GETVARG", "ERRNNIL", "VARARGPREP", "EXTRAARG",
];

unsafe fn get_prototype_from_arg(state: *mut State, idx: i32) -> *const Prototype {
    unsafe {
        let val: &TValue = (*state).index_to_value(idx);
        match val.get_tagvariant() {
            | TagVariant::ClosureL => {
                let closure: *mut Closure = val.as_closure().unwrap();
                (*closure).closure_payload.closurepayload_lprototype
            },
            | _ => null(),
        }
    }
}

unsafe fn disassemble_c_closure(state: *mut State, func_idx: i32, buf: &mut String) {
    unsafe {
        let val: &TValue = (*state).index_to_value(func_idx);
        let nup = match val.get_tagvariant() {
            | TagVariant::ClosureC => {
                let closure = val.as_closure().unwrap();
                buf.push_str(&format!("C closure ({} upvalues)\n", (*closure).closure_count_upvalues));
                (*closure).closure_count_upvalues
            },
            | TagVariant::ClosureCFunction => {
                buf.push_str("C function (light, no upvalues)\n");
                0
            },
            | _ => 0,
        };
        for i in 1..=nup as i32 {
            let name = lua_getupvalue(state, func_idx, i);
            if name.is_null() {
                break;
            }
            let top = (*state).get_top();
            let upval: &TValue = (*state).index_to_value(top);
            buf.push_str(&format!("\tupvalue {}: ", i));
            match upval.get_tagvariant() {
                | TagVariant::ClosureL => {
                    buf.push_str("Lua function\n");
                    let proto = (*upval.as_closure().unwrap()).closure_payload.closurepayload_lprototype;
                    disassemble_prototype(proto, buf, 1);
                },
                | TagVariant::ClosureC => {
                    buf.push_str("C closure\n");
                },
                | TagVariant::ClosureCFunction => {
                    buf.push_str("C function\n");
                },
                | TagVariant::Table => {
                    buf.push_str("table\n");
                },
                | TagVariant::NilNil => buf.push_str("nil\n"),
                | TagVariant::BooleanTrue => buf.push_str("true\n"),
                | TagVariant::BooleanFalse => buf.push_str("false\n"),
                | TagVariant::NumericInteger => {
                    buf.push_str(&format!("{}\n", upval.as_integer().unwrap()));
                },
                | TagVariant::NumericNumber => {
                    buf.push_str(&format!("{}\n", upval.as_number().unwrap()));
                },
                | TagVariant::StringShort | TagVariant::StringLong => {
                    let ts = upval.as_string().unwrap();
                    let len = (*ts).get_length();
                    let ptr = (*ts).get_contents_mut() as *const u8;
                    let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
                    buf.push_str(&format!("\"{}\"\n", s));
                },
                | _ => {
                    buf.push_str(&format!("{:?}\n", upval.get_tagvariant()));
                },
            }
            lua_settop(state, -2); // pop upvalue
        }
    }
}

unsafe fn format_constant(prototype: *const Prototype, idx: usize, buf: &mut String) {
    unsafe {
        let k: *const TValue = (*prototype).prototype_constants.vectort_pointer.add(idx);
        match (*k).get_tagvariant() {
            | TagVariant::NilNil => buf.push_str("nil"),
            | TagVariant::BooleanTrue => buf.push_str("true"),
            | TagVariant::BooleanFalse => buf.push_str("false"),
            | TagVariant::NumericInteger => {
                let v = (*k).as_integer().unwrap();
                buf.push_str(&format!("{}", v));
            },
            | TagVariant::NumericNumber => {
                let v = (*k).as_number().unwrap();
                buf.push_str(&format!("{}", v));
            },
            | TagVariant::StringShort | TagVariant::StringLong => {
                let ts: *mut TString = (*k).as_string().unwrap();
                let len = (*ts).get_length();
                let ptr = (*ts).get_contents_mut() as *const u8;
                let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
                buf.push('"');
                for ch in s.chars() {
                    match ch {
                        | '\n' => buf.push_str("\\n"),
                        | '\r' => buf.push_str("\\r"),
                        | '\t' => buf.push_str("\\t"),
                        | '\\' => buf.push_str("\\\\"),
                        | '"' => buf.push_str("\\\""),
                        | c if c.is_ascii_graphic() || c == ' ' => buf.push(c),
                        | c => buf.push_str(&format!("\\x{:02x}", c as u32)),
                    }
                }
                buf.push('"');
            },
            | _ => buf.push_str("?"),
        }
    }
}

unsafe fn format_upvalue_name(prototype: *const Prototype, idx: usize, buf: &mut String) {
    unsafe {
        if idx < (*prototype).prototype_upvalues.get_size() {
            let desc = (*prototype).prototype_upvalues.vectort_pointer.add(idx);
            let name = (*desc).upvaluedescription_name;
            if !name.is_null() {
                let len = (*name).get_length();
                let ptr = (*name).get_contents_mut() as *const u8;
                let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
                buf.push_str(s);
                return;
            }
        }
        buf.push('?');
    }
}

unsafe fn disassemble_prototype(prototype: *const Prototype, buf: &mut String, indent: usize) {
    unsafe {
        let prefix: String = " ".repeat(indent);
        let code_len = (*prototype).prototype_code.get_size();
        let source = (*prototype).prototype_source;
        // header
        if !source.is_null() {
            let slen = (*source).get_length();
            let sptr = (*source).get_contents_mut() as *const u8;
            let sname = std::str::from_utf8_unchecked(std::slice::from_raw_parts(sptr, slen));
            buf.push_str(&format!(
                "{}function <{}:{}-{}> ({} instructions)\n",
                prefix,
                sname,
                (*prototype).prototype_linedefined,
                (*prototype).prototype_lastlinedefined,
                code_len,
            ));
        } else {
            buf.push_str(&format!(
                "{}function <(?):{}-{}> ({} instructions)\n",
                prefix,
                (*prototype).prototype_linedefined,
                (*prototype).prototype_lastlinedefined,
                code_len,
            ));
        }
        buf.push_str(&format!(
            "{}{} params, {} stack, {} upvalues, {} constants\n",
            prefix,
            (*prototype).prototype_countparameters,
            (*prototype).prototype_maximumstacksize,
            (*prototype).prototype_upvalues.get_size(),
            (*prototype).prototype_constants.get_size(),
        ));
        // instructions
        for pc in 0..code_len {
            let instr: u32 = *(*prototype).prototype_code.vectort_pointer.add(pc);
            let op = instr & MASK_OP;
            let a = (instr >> POSITION_A) & MASK_A;
            let line = luag_getfuncline(prototype, pc as i32);
            let opname = if (op as usize) < OPCODE_NAMES.len() { OPCODE_NAMES[op as usize] } else { "???" };
            buf.push_str(&format!("{}{:>4}\t[{}]\t{:<12}", prefix, pc + 1, line, opname));
            let mode = if (op as usize) < OPMODES.len() { OPMODES[op as usize] } else { 0 };
            let fmt = (mode & 0x07) as u32;
            match fmt {
                | IABC => {
                    let k_flag = (instr >> POSITION_K) & MASK_K;
                    let b = (instr >> POSITION_B) & MASK_B;
                    let c = (instr >> POSITION_C) & MASK_C;
                    buf.push_str(&format!("\t{}", a));
                    buf.push_str(&format!(" {}", b));
                    buf.push_str(&format!(" {}", c));
                    if k_flag != 0 {
                        buf.push_str(" [k]");
                    }
                    format_instruction_comment(prototype, op, a, b, c, k_flag, 0, buf);
                },
                | IABX => {
                    let bx = (instr >> POSITION_K) & MASK_BX;
                    buf.push_str(&format!("\t{} {}", a, bx));
                    // for LOADK show the constant value
                    if op == OPCODE_LOADK {
                        buf.push_str("\t; ");
                        format_constant(prototype, bx as usize, buf);
                    } else if op == OPCODE_CLOSURE {
                        buf.push_str(&format!("\t; function <{}>", bx));
                    } else if op == OPCODE_GET_FIELD || op == OPCODE_SETFIELD {
                        buf.push_str("\t; ");
                        format_constant(prototype, bx as usize, buf);
                    }
                },
                | IASBX => {
                    let sbx = ((instr >> POSITION_K) & MASK_BX) as i32 - OFFSET_SBX;
                    buf.push_str(&format!("\t{} {}", a, sbx));
                },
                | IAX => {
                    let ax = (instr >> POSITION_A) & MASK_AX;
                    buf.push_str(&format!("\t{}", ax));
                },
                | ISJ => {
                    let sj = ((instr >> POSITION_A) & MASK_AX) as i32 - OFFSET_SJ;
                    buf.push_str(&format!("\t{}", sj));
                    buf.push_str(&format!("\t; to {}", (pc as i32) + sj + 2));
                },
                | _ => {},
            }
            buf.push('\n');
        }
        // constants table
        let nk = (*prototype).prototype_constants.get_size();
        if nk > 0 {
            buf.push_str(&format!("{}constants ({}):\n", prefix, nk));
            for i in 0..nk {
                buf.push_str(&format!("{}\t{}\t", prefix, i));
                format_constant(prototype, i, buf);
                buf.push('\n');
            }
        }
        // upvalues
        let nup = (*prototype).prototype_upvalues.get_size();
        if nup > 0 {
            buf.push_str(&format!("{}upvalues ({}):\n", prefix, nup));
            for i in 0..nup {
                let desc = (*prototype).prototype_upvalues.vectort_pointer.add(i);
                buf.push_str(&format!(
                    "{}\t{}\t",
                    prefix, i,
                ));
                format_upvalue_name(prototype, i, buf);
                buf.push_str(&format!(
                    "\tinstack={}\tidx={}\n",
                    (*desc).upvaluedescription_isinstack as i32,
                    (*desc).upvaluedescription_index,
                ));
            }
        }
        // nested prototypes
        let np = (*prototype).prototype_prototypes.get_size();
        for i in 0..np {
            buf.push('\n');
            let child = *(*prototype).prototype_prototypes.vectort_pointer.add(i);
            disassemble_prototype(child, buf, indent);
        }
    }
}

unsafe fn format_instruction_comment(
    prototype: *const Prototype, op: u32, a: u32, b: u32, c: u32, k: u32, _bx: u32, buf: &mut String,
) {
    unsafe {
        match op {
            | OPCODE_GET_UPVALUE | OPCODE_SETUPVAL => {
                buf.push_str("\t; ");
                format_upvalue_name(prototype, b as usize, buf);
            },
            | OPCODE_GET_TABLE_UPVALUE => {
                buf.push_str("\t; ");
                format_upvalue_name(prototype, b as usize, buf);
                buf.push(' ');
                format_constant(prototype, c as usize, buf);
            },
            | OPCODE_SETTABUP => {
                buf.push_str("\t; ");
                format_upvalue_name(prototype, a as usize, buf);
                buf.push(' ');
                format_constant(prototype, b as usize, buf);
            },
            | OPCODE_GET_FIELD => {
                buf.push_str("\t; ");
                format_constant(prototype, c as usize, buf);
            },
            | OPCODE_SETFIELD => {
                buf.push_str("\t; ");
                format_constant(prototype, b as usize, buf);
                if k != 0 {
                    buf.push(' ');
                    format_constant(prototype, c as usize, buf);
                }
            },
            | OPCODE_SELF => {
                if k != 0 {
                    buf.push_str("\t; ");
                    format_constant(prototype, c as usize, buf);
                }
            },
            | OPCODE_ADDK | OPCODE_SUBK | OPCODE_MULK | OPCODE_MODK | OPCODE_POWK | OPCODE_DIVK | OPCODE_IDIVK
            | OPCODE_BANDK | OPCODE_BORK | OPCODE_BXORK => {
                buf.push_str("\t; ");
                format_constant(prototype, c as usize, buf);
            },
            | OPCODE_LOADK => {
                // handled in IABX branch
            },
            | OPCODE_EQK => {
                buf.push_str("\t; ");
                format_constant(prototype, b as usize, buf);
            },
            | _ => {},
        }
    }
}

/// dis.dis(f) - disassemble a Lua function, printing to stdout
pub unsafe fn dis_dis(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let mut buf = String::new();
        let prototype = get_prototype_from_arg(state, 1);
        if !prototype.is_null() {
            disassemble_prototype(prototype, &mut buf, 0);
        } else {
            disassemble_c_closure(state, 1, &mut buf);
        }
        let mut out = std::io::stdout().lock();
        out.write_all(buf.as_bytes()).ok();
        out.flush().ok();
        0
    }
}

/// dis.code(f) - return disassembly as a string
pub unsafe fn dis_code(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let mut buf = String::new();
        let prototype = get_prototype_from_arg(state, 1);
        if !prototype.is_null() {
            disassemble_prototype(prototype, &mut buf, 0);
        } else {
            disassemble_c_closure(state, 1, &mut buf);
        }
        lua_pushlstring(state, buf.as_ptr() as *const i8, buf.len());
        1
    }
}

/// dis.info(f) - return a table with function metadata
pub unsafe fn dis_info(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let prototype = get_prototype_from_arg(state, 1);
        if prototype.is_null() {
            // C closure info
            let val: &TValue = (*state).index_to_value(1);
            (*state).lua_createtable();
            let nup = match val.get_tagvariant() {
                | TagVariant::ClosureC => (*val.as_closure().unwrap()).closure_count_upvalues as i64,
                | _ => 0,
            };
            lua_pushstring(state, c"C".as_ptr());
            lua_setfield(state, -2, c"type".as_ptr());
            (*state).push_integer(nup);
            lua_setfield(state, -2, c"upvalues".as_ptr());
            return 1;
        }
        (*state).lua_createtable();
        (*state).push_integer((*prototype).prototype_code.get_size() as i64);
        lua_setfield(state, -2, c"instructions".as_ptr());
        (*state).push_integer((*prototype).prototype_constants.get_size() as i64);
        lua_setfield(state, -2, c"constants".as_ptr());
        (*state).push_integer((*prototype).prototype_upvalues.get_size() as i64);
        lua_setfield(state, -2, c"upvalues".as_ptr());
        (*state).push_integer((*prototype).prototype_countparameters as i64);
        lua_setfield(state, -2, c"params".as_ptr());
        (*state).push_integer((*prototype).prototype_maximumstacksize as i64);
        lua_setfield(state, -2, c"stacksize".as_ptr());
        (*state).push_boolean((*prototype).prototype_isvariablearguments);
        lua_setfield(state, -2, c"vararg".as_ptr());
        (*state).push_integer((*prototype).prototype_linedefined as i64);
        lua_setfield(state, -2, c"linedefined".as_ptr());
        (*state).push_integer((*prototype).prototype_lastlinedefined as i64);
        lua_setfield(state, -2, c"lastlinedefined".as_ptr());
        (*state).push_integer((*prototype).prototype_prototypes.get_size() as i64);
        lua_setfield(state, -2, c"children".as_ptr());
        let source = (*prototype).prototype_source;
        if !source.is_null() {
            lua_pushlstring(
                state,
                (*source).get_contents_mut(),
                (*source).get_length(),
            );
            lua_setfield(state, -2, c"source".as_ptr());
        }
        1
    }
}

/// dis.opcodes(f) - return an array of {opcode, opname, line, a, b, c, ...} tables
pub unsafe fn dis_opcodes(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let prototype = get_prototype_from_arg(state, 1);
        if prototype.is_null() {
            (*state).lua_createtable(); // empty table for C closures
            return 1;
        }
        (*state).lua_createtable();
        let code_len = (*prototype).prototype_code.get_size();
        for pc in 0..code_len {
            let instr: u32 = *(*prototype).prototype_code.vectort_pointer.add(pc);
            let op = instr & MASK_OP;
            let a = (instr >> POSITION_A) & MASK_A;
            let line = luag_getfuncline(prototype, pc as i32);
            let mode = if (op as usize) < OPMODES.len() { OPMODES[op as usize] } else { 0 };
            let fmt = (mode & 0x07) as u32;
            (*state).lua_createtable();
            // opcode number
            (*state).push_integer(op as i64);
            lua_setfield(state, -2, c"op".as_ptr());
            // opcode name
            let opname = if (op as usize) < OPCODE_NAMES.len() { OPCODE_NAMES[op as usize] } else { "???" };
            lua_pushlstring(state, opname.as_ptr() as *const i8, opname.len());
            lua_setfield(state, -2, c"name".as_ptr());
            // line
            (*state).push_integer(line as i64);
            lua_setfield(state, -2, c"line".as_ptr());
            // A register
            (*state).push_integer(a as i64);
            lua_setfield(state, -2, c"a".as_ptr());
            match fmt {
                | IABC => {
                    let k_flag = (instr >> POSITION_K) & MASK_K;
                    let b = (instr >> POSITION_B) & MASK_B;
                    let c = (instr >> POSITION_C) & MASK_C;
                    (*state).push_integer(b as i64);
                    lua_setfield(state, -2, c"b".as_ptr());
                    (*state).push_integer(c as i64);
                    lua_setfield(state, -2, c"c".as_ptr());
                    (*state).push_boolean(k_flag != 0);
                    lua_setfield(state, -2, c"k".as_ptr());
                },
                | IABX => {
                    let bx = (instr >> POSITION_K) & MASK_BX;
                    (*state).push_integer(bx as i64);
                    lua_setfield(state, -2, c"bx".as_ptr());
                },
                | IASBX => {
                    let sbx = ((instr >> POSITION_K) & MASK_BX) as i32 - OFFSET_SBX;
                    (*state).push_integer(sbx as i64);
                    lua_setfield(state, -2, c"sbx".as_ptr());
                },
                | IAX => {
                    let ax = (instr >> POSITION_A) & MASK_AX;
                    (*state).push_integer(ax as i64);
                    lua_setfield(state, -2, c"ax".as_ptr());
                },
                | ISJ => {
                    let sj = ((instr >> POSITION_A) & MASK_AX) as i32 - OFFSET_SJ;
                    (*state).push_integer(sj as i64);
                    lua_setfield(state, -2, c"sj".as_ptr());
                },
                | _ => {},
            }
            lua_rawseti(state, -2, (pc + 1) as i64);
        }
        1
    }
}

/// dis.constants(f) - return the constants table
pub unsafe fn dis_constants(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let prototype = get_prototype_from_arg(state, 1);
        if prototype.is_null() {
            (*state).lua_createtable(); // empty table for C closures
            return 1;
        }
        (*state).lua_createtable();
        let nk = (*prototype).prototype_constants.get_size();
        for i in 0..nk {
            let k: *const TValue = (*prototype).prototype_constants.vectort_pointer.add(i);
            // Push the constant value itself onto the stack
            match (*k).get_tagvariant() {
                | TagVariant::NilNil => (*state).push_nil(),
                | TagVariant::BooleanTrue => (*state).push_boolean(true),
                | TagVariant::BooleanFalse => (*state).push_boolean(false),
                | TagVariant::NumericInteger => (*state).push_integer((*k).as_integer().unwrap()),
                | TagVariant::NumericNumber => (*state).push_number((*k).as_number().unwrap()),
                | TagVariant::StringShort | TagVariant::StringLong => {
                    let ts: *mut TString = (*k).as_string().unwrap();
                    lua_pushlstring(state, (*ts).get_contents_mut(), (*ts).get_length());
                },
                | _ => (*state).push_nil(),
            }
            lua_rawseti(state, -2, (i + 1) as i64);
        }
        1
    }
}

pub const DIS_FUNCTIONS: [RegisteredFunction; 5] = [
    RegisteredFunction {
        registeredfunction_name: c"dis".as_ptr(),
        registeredfunction_function: Some(dis_dis as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"code".as_ptr(),
        registeredfunction_function: Some(dis_code as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"info".as_ptr(),
        registeredfunction_function: Some(dis_info as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"opcodes".as_ptr(),
        registeredfunction_function: Some(dis_opcodes as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"constants".as_ptr(),
        registeredfunction_function: Some(dis_constants as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn luaopen_dis(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, DIS_FUNCTIONS.as_ptr(), DIS_FUNCTIONS.len(), 0);
        1
    }
}

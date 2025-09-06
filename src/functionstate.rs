use crate::lexical::blockcontrol::*;
use crate::lexical::lexicalstate::*;
use crate::operator_::*;
use crate::labeldescription::*;
use crate::vm::instruction::*;
use crate::vm::opmode::*;
use crate::vm::opcode::*;
use crate::lexical::constructorcontrol::*;
use crate::expressionkind::*;
use crate::labellist::*;
use crate::tvalue::*;
use crate::tm::*;
use crate::state::*;
use crate::lexical::operatorunary::*;
use crate::tstring::*;
use crate::variabledescription::*;
use crate::debugger::absolutelineinfo::*;
use crate::localvariable::*;
use crate::expressiondescription::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::f2i::*;
use crate::tag::*;
use crate::object::*;
use crate::new::*;
use crate::table::*;
use crate::prototype::*;
use crate::value::*;
use libc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub prototype: *mut Prototype,
    pub previous: *mut FunctionState,
    pub lexical_state: *mut LexicalState,
    pub block_control: *mut BlockControl,
    pub program_counter: i32,
    pub last_target: i32,
    pub previous_line: i32,
    pub count_k: i32,
    pub count_p: i32,
    pub count_abslineinfo: i32,
    pub first_local: i32,
    pub first_label: i32,
    pub count_debug_variables: i16,
    pub count_active_variables: u8,
    pub count_upvalues: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needs_close: bool,
}
impl New for FunctionState {
    fn new() -> Self {
        return FunctionState {
            prototype: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
            lexical_state: std::ptr::null_mut(),
            block_control: std::ptr::null_mut(),
            program_counter: 0,
            last_target: 0,
            previous_line: 0,
            count_k: 0,
            count_p: 0,
            count_abslineinfo: 0,
            first_local: 0,
            first_label: 0,
            count_debug_variables: 0,
            count_active_variables: 0,
            count_upvalues: 0,
            freereg: 0,
            iwthabs: 0,
            needs_close: false,
        };
    }
}
pub unsafe extern "C" fn movegotosout(function_state: *mut FunctionState, block_control: *mut BlockControl) {
    unsafe {
        let gl: *mut LabelList = &mut (*(*(*function_state).lexical_state).dynamic_data).gt;
        for i in (*block_control).first_goto..(*gl).n {
            let gt: *mut LabelDescription =
                &mut *((*gl).pointer).offset(i as isize) as *mut LabelDescription;
            if reglevel(function_state, (*gt).count_active_variables as i32)
                > reglevel(function_state, (*block_control).count_active_variables as i32)
            {
                (*gt).close = ((*gt).close as i32 | (*block_control).count_upvalues as i32) as u8;
            }
            (*gt).count_active_variables = (*block_control).count_active_variables;
        }
    }
}
pub unsafe extern "C" fn enterblock(
    function_state: *mut FunctionState,
    block_control: *mut BlockControl,
    is_loop: bool,
) {
    unsafe {
        (*block_control).is_loop = is_loop;
        (*block_control).count_active_variables = (*function_state).count_active_variables;
        (*block_control).first_label = (*(*(*function_state).lexical_state).dynamic_data).label.n;
        (*block_control).first_goto = (*(*(*function_state).lexical_state).dynamic_data).gt.n;
        (*block_control).count_upvalues = 0;
        (*block_control).is_inside_tbc =
            !((*function_state).block_control).is_null() && (*(*function_state).block_control).is_inside_tbc as i32 != 0;
        (*block_control).previous = (*function_state).block_control;
        (*function_state).block_control = block_control;
    }
}
pub unsafe extern "C" fn leaveblock(function_state: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*function_state).block_control;
        let lexical_state: *mut LexicalState = (*function_state).lexical_state;
        let mut has_close = false;
        let stklevel: i32 = reglevel(function_state, (*block_control).count_active_variables as i32);
        removevars(function_state, (*block_control).count_active_variables as i32);
        if (*block_control).is_loop {
            has_close = (*lexical_state).create_label(
                luas_newlstr(
                    (*lexical_state).state,
                    b"break\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ),
                0,
                false,
            );
        }
        if !has_close
            && !((*block_control).previous).is_null()
            && (*block_control).count_upvalues as i32 != 0
        {
            luak_code_abck(function_state, OP_CLOSE, stklevel, 0, 0, 0);
        }
        (*function_state).freereg = stklevel as u8;
        (*(*lexical_state).dynamic_data).label.n = (*block_control).first_label;
        (*function_state).block_control = (*block_control).previous;
        if !((*block_control).previous).is_null() {
            movegotosout(function_state, block_control);
        } else if (*block_control).first_goto < (*(*lexical_state).dynamic_data).gt.n {
            undefgoto(
                lexical_state,
                &mut *((*(*lexical_state).dynamic_data).gt.pointer)
                    .offset((*block_control).first_goto as isize),
            );
        }
    }
}
pub unsafe extern "C" fn closelistfield(function_state: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).expression_description.expression_kind == ExpressionKind::VVOID {
            return;
        }
        luak_exp2nextreg(function_state, &mut (*cc).expression_description);
        (*cc).expression_description.expression_kind = ExpressionKind::VVOID;
        if (*cc).to_store == 50 as i32 {
            luak_setlist(function_state, (*(*cc).t).value.info, (*cc).na, (*cc).to_store);
            (*cc).na += (*cc).to_store;
            (*cc).to_store = 0;
        }
    }
}
pub unsafe extern "C" fn lastlistfield(function_state: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).to_store == 0 {
            return;
        }
        if (*cc).expression_description.expression_kind as u32 == ExpressionKind::VCALL as u32 || (*cc).expression_description.expression_kind as u32 == ExpressionKind::VVARARG as u32 {
            luak_setreturns(function_state, &mut (*cc).expression_description, -1);
            luak_setlist(function_state, (*(*cc).t).value.info, (*cc).na, -1);
            (*cc).na -= 1;
            (*cc).na;
        } else {
            if (*cc).expression_description.expression_kind != ExpressionKind::VVOID {
                luak_exp2nextreg(function_state, &mut (*cc).expression_description);
            }
            luak_setlist(function_state, (*(*cc).t).value.info, (*cc).na, (*cc).to_store);
        }
        (*cc).na += (*cc).to_store;
    }
}
pub unsafe extern "C" fn setvararg(function_state: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*function_state).prototype).is_variable_arguments = true;
        luak_code_abck(function_state, OP_VARARGPREP, nparams, 0, 0, 0);
    }
}
pub unsafe extern "C" fn errorlimit(function_state: *mut FunctionState, limit: i32, what: *const i8) -> ! {
    unsafe {
        let state: *mut State = (*(*function_state).lexical_state).state;
        let message: *const i8;
        let line: i32 = (*(*function_state).prototype).line_defined;
        let where_0: *const i8 = if line == 0 {
            b"main function\0" as *const u8 as *const i8
        } else {
            luao_pushfstring(
                state,
                b"function at line %d\0" as *const u8 as *const i8,
                line,
            )
        };
        message = luao_pushfstring(
            state,
            b"too many %s (limit is %d) in %s\0" as *const u8 as *const i8,
            what,
            limit,
            where_0,
        );
        luax_syntaxerror((*function_state).lexical_state, message);
    }
}
pub unsafe extern "C" fn checklimit(function_state: *mut FunctionState, v: i32, length: i32, what: *const i8) {
    unsafe {
        if v > length {
            errorlimit(function_state, length, what);
        }
    }
}
pub unsafe extern "C" fn getlocalvardesc(
    function_state: *mut FunctionState,
    vidx: i32,
) -> *mut VariableDescription {
    unsafe {
        return &mut *((*(*(*function_state).lexical_state).dynamic_data)
            .active_variable
            .pointer)
            .offset(((*function_state).first_local + vidx) as isize) as *mut VariableDescription;
    }
}
pub unsafe extern "C" fn reglevel(function_state: *mut FunctionState, mut nvar: i32) -> i32 {
    unsafe {
        loop {
            let fresh38 = nvar;
            nvar = nvar - 1;
            if !(fresh38 > 0) {
                break;
            }
            let variable_description: *mut VariableDescription = getlocalvardesc(function_state, nvar);
            if (*variable_description).content.kind as i32 != 3 {
                return (*variable_description).content.register_index as i32 + 1;
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn luay_nvarstack(function_state: *mut FunctionState) -> i32 {
    unsafe {
        return reglevel(function_state, (*function_state).count_active_variables as i32);
    }
}
pub unsafe extern "C" fn localdebuginfo(function_state: *mut FunctionState, vidx: i32) -> *mut LocalVariable {
    unsafe {
        let variable_description: *mut VariableDescription = getlocalvardesc(function_state, vidx);
        if (*variable_description).content.kind as i32 == 3 {
            return std::ptr::null_mut();
        } else {
            let index: i32 = (*variable_description).content.pidx as i32;
            return &mut *((*(*function_state).prototype).local_variables).offset(index as isize) as *mut LocalVariable;
        };
    }
}
pub unsafe extern "C" fn init_var(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
    vidx: i32,
) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).expression_kind = ExpressionKind::VLOCAL;
        (*e).value.variable.value_index = vidx as u16;
        (*e).value.variable.register_index = (*getlocalvardesc(function_state, vidx)).content.register_index;
    }
}
pub unsafe extern "C" fn removevars(function_state: *mut FunctionState, tolevel: i32) {
    unsafe {
        (*(*(*function_state).lexical_state).dynamic_data).active_variable.length -=
            (*function_state).count_active_variables as i32 - tolevel;
        while (*function_state).count_active_variables as i32 > tolevel {
            (*function_state).count_active_variables = ((*function_state).count_active_variables).wrapping_sub(1);
            let var: *mut LocalVariable = localdebuginfo(function_state, (*function_state).count_active_variables as i32);
            if !var.is_null() {
                (*var).end_program_counter = (*function_state).program_counter;
            }
        }
    }
}
pub unsafe extern "C" fn searchupvalue(function_state: *mut FunctionState, name: *mut TString) -> i32 {
    unsafe {
        let up: *mut UpValueDescription = (*(*function_state).prototype).upvalues;
        for i in 0..(*function_state).count_upvalues {
            if (*up.offset(i as isize)).name == name {
                return i as i32;
            }
        }
        return -1;
    }
}
pub unsafe extern "C" fn allocupvalue(function_state: *mut FunctionState) -> *mut UpValueDescription {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut old_size: i32 = (*prototype).size_upvalues;
        checklimit(
            function_state,
            (*function_state).count_upvalues as i32 + 1,
            255 as i32,
            b"upvalues\0" as *const u8 as *const i8,
        );
        (*prototype).upvalues = luam_growaux_(
            (*(*function_state).lexical_state).state,
            (*prototype).upvalues as *mut libc::c_void,
            (*function_state).count_upvalues as i32,
            &mut (*prototype).size_upvalues,
            ::core::mem::size_of::<UpValueDescription>() as i32,
            (if 255 as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<UpValueDescription>() as u64)
            {
                255 as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<UpValueDescription>() as u64) as u32
            }) as i32,
            b"upvalues\0" as *const u8 as *const i8,
        ) as *mut UpValueDescription;
        while old_size < (*prototype).size_upvalues {
            let fresh41 = old_size;
            old_size = old_size + 1;
            let ref mut fresh42 = (*((*prototype).upvalues).offset(fresh41 as isize)).name;
            *fresh42 = std::ptr::null_mut();
        }
        let fresh43 = (*function_state).count_upvalues;
        (*function_state).count_upvalues = ((*function_state).count_upvalues).wrapping_add(1);
        return &mut *((*prototype).upvalues).offset(fresh43 as isize) as *mut UpValueDescription;
    }
}
pub unsafe extern "C" fn newupvalue(
    function_state: *mut FunctionState,
    name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let up: *mut UpValueDescription = allocupvalue(function_state);
        let previous: *mut FunctionState = (*function_state).previous;
        if (*v).expression_kind as u32 == ExpressionKind::VLOCAL as u32 {
            (*up).is_in_stack = true;
            (*up).index = (*v).value.variable.register_index;
            (*up).kind = (*getlocalvardesc(previous, (*v).value.variable.value_index as i32)).content.kind;
        } else {
            (*up).is_in_stack = false;
            (*up).index = (*v).value.info as u8;
            (*up).kind = (*((*(*previous).prototype).upvalues).offset((*v).value.info as isize)).kind;
        }
        (*up).name = name;
        if (*(*function_state).prototype).get_marked() & 1 << 5 != 0 && (*name).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                (*(*function_state).lexical_state).state,
                &mut (*((*function_state).prototype as *mut Object)),
                &mut (*(name as *mut Object)),
            );
        } else {
        };
        return (*function_state).count_upvalues as i32 - 1;
    }
}
pub unsafe extern "C" fn searchvar(
    function_state: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let mut i: i32;
        i = (*function_state).count_active_variables as i32 - 1;
        while i >= 0 {
            let variable_description: *mut VariableDescription = getlocalvardesc(function_state, i);
            if n == (*variable_description).content.name {
                if (*variable_description).content.kind as i32 == 3 {
                    init_exp(var, ExpressionKind::VCONST, (*function_state).first_local + i);
                } else {
                    init_var(function_state, var, i);
                }
                return (*var).expression_kind as i32;
            }
            i -= 1;
        }
        return -1;
    }
}
pub unsafe extern "C" fn markupval(function_state: *mut FunctionState, level: i32) {
    unsafe {
        let mut block_control: *mut BlockControl = (*function_state).block_control;
        while (*block_control).count_active_variables as i32 > level {
            block_control = (*block_control).previous;
        }
        (*block_control).count_upvalues = 1;
        (*function_state).needs_close = true;
    }
}
pub unsafe extern "C" fn marktobeclosed(function_state: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*function_state).block_control;
        (*block_control).count_upvalues = 1;
        (*block_control).is_inside_tbc = true;
        (*function_state).needs_close = true;
    }
}
pub unsafe extern "C" fn singlevaraux(
    function_state: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
    base: i32,
) {
    unsafe {
        if function_state.is_null() {
            init_exp(var, ExpressionKind::VVOID, 0);
        } else {
            let v: i32 = searchvar(function_state, n, var);
            if v >= 0 {
                if v == ExpressionKind::VLOCAL as i32 && base == 0 {
                    markupval(function_state, (*var).value.variable.value_index as i32);
                }
            } else {
                let mut index: i32 = searchupvalue(function_state, n);
                if index < 0 {
                    singlevaraux((*function_state).previous, n, var, 0);
                    if (*var).expression_kind == ExpressionKind::VLOCAL
                        || (*var).expression_kind == ExpressionKind::VUPVAL
                    {
                        index = newupvalue(function_state, n, var);
                    } else {
                        return;
                    }
                }
                init_exp(var, ExpressionKind::VUPVAL, index);
            }
        };
    }
}
pub unsafe extern "C" fn fixforjump(
    function_state: *mut FunctionState,
    program_counter: i32,
    dest: i32,
    back: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).prototype).code).offset(program_counter as isize) as *mut u32;
        let mut offset: i32 = dest - (program_counter + 1);
        if back != 0 {
            offset = -offset;
        }
        if ((offset > (1 << 8 + 8 + 1) - 1) as i32 != 0) as i64 != 0 {
            luax_syntaxerror(
                (*function_state).lexical_state,
                b"control structure too long\0" as *const u8 as *const i8,
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1) << POSITION_K)
            | (offset as u32) << POSITION_K & !(!(0u32) << 8 + 8 + 1) << POSITION_K;
    }
}
pub unsafe extern "C" fn checktoclose(function_state: *mut FunctionState, level: i32) {
    unsafe {
        if level != -1 {
            marktobeclosed(function_state);
            luak_code_abck(function_state, OP_TBC, reglevel(function_state, level), 0, 0, 0);
        }
    }
}
pub unsafe extern "C" fn previousinstruction(function_state: *mut FunctionState) -> *mut u32 {
    unsafe {
        pub const INVALID_INSTRUCTION: u32 = !(0u32);
        if (*function_state).program_counter > (*function_state).last_target {
            return &mut *((*(*function_state).prototype).code).offset(((*function_state).program_counter - 1) as isize)
                as *mut u32;
        } else {
            return &INVALID_INSTRUCTION as *const u32 as *mut u32;
        };
    }
}
pub unsafe extern "C" fn luak_nil(function_state: *mut FunctionState, mut from: i32, n: i32) {
    unsafe {
        let mut length: i32 = from + n - 1;
        let previous: *mut u32 = previousinstruction(function_state);
        if (*previous >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_LOADNIL as u32 {
            let pfrom: i32 = (*previous >> POSITION_A & !(!(0u32) << 8) << 0) as i32;
            let pl: i32 = pfrom + (*previous >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= length + 1 {
                if pfrom < from {
                    from = pfrom;
                }
                if pl > length {
                    length = pl;
                }
                *previous = *previous & !(!(!(0u32) << 8) << POSITION_A)
                    | (from as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
                *previous = *previous & !(!(!(0u32) << 8) << POSITION_B)
                    | ((length - from) as u32) << POSITION_B & !(!(0u32) << 8) << POSITION_B;
                return;
            }
        }
        luak_code_abck(function_state, OP_LOADNIL, from, n - 1, 0, 0);
    }
}
pub unsafe extern "C" fn getjump(function_state: *mut FunctionState, program_counter: i32) -> i32 {
    unsafe {
        let offset: i32 = (*((*(*function_state).prototype).code).offset(program_counter as isize) >> POSITION_A
            & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
            - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
        if offset == -1 {
            return -1;
        } else {
            return program_counter + 1 + offset;
        };
    }
}
pub unsafe extern "C" fn fixjump(function_state: *mut FunctionState, program_counter: i32, dest: i32) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).prototype).code).offset(program_counter as isize) as *mut u32;
        let offset: i32 = dest - (program_counter + 1);
        if !(-((1 << 8 + 8 + 1 + 8) - 1 >> 1) <= offset
            && offset <= (1 << 8 + 8 + 1 + 8) - 1 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1))
        {
            luax_syntaxerror(
                (*function_state).lexical_state,
                b"control structure too long\0" as *const u8 as *const i8,
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1 + 8) << POSITION_A)
            | ((offset + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32) << POSITION_A
                & !(!(0u32) << 8 + 8 + 1 + 8) << POSITION_A;
    }
}
pub unsafe extern "C" fn luak_concat(function_state: *mut FunctionState, l1: *mut i32, l2: i32) {
    unsafe {
        if l2 == -1 {
            return;
        } else if *l1 == -1 {
            *l1 = l2;
        } else {
            let mut list: i32 = *l1;
            let mut next: i32;
            loop {
                next = getjump(function_state, list);
                if !(next != -1) {
                    break;
                }
                list = next;
            }
            fixjump(function_state, list, l2);
        };
    }
}
pub unsafe extern "C" fn luak_jump(function_state: *mut FunctionState) -> i32 {
    unsafe {
        return codesj(function_state, OP_JMP, -1, 0);
    }
}
pub unsafe extern "C" fn luak_ret(function_state: *mut FunctionState, first: i32, nret: i32) {
    unsafe {
        let op: u32;
        match nret {
            0 => {
                op = OP_RETURN0;
            }
            1 => {
                op = OP_RETURN1;
            }
            _ => {
                op = OP_RETURN;
            }
        }
        luak_code_abck(function_state, op, first, nret + 1, 0, 0);
    }
}
pub unsafe extern "C" fn condjump(
    function_state: *mut FunctionState,
    op: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        luak_code_abck(function_state, op, a, b, c, k);
        return luak_jump(function_state);
    }
}
pub unsafe extern "C" fn luak_getlabel(function_state: *mut FunctionState) -> i32 {
    unsafe {
        (*function_state).last_target = (*function_state).program_counter;
        return (*function_state).program_counter;
    }
}
pub unsafe extern "C" fn getjumpcontrol(function_state: *mut FunctionState, program_counter: i32) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*function_state).prototype).code).offset(program_counter as isize) as *mut u32;
        if program_counter >= 1
            && OPMODES[(*pi.offset(-(1 as isize)) >> 0 & !(!(0u32) << 7) << 0) as usize]
                as i32
                & 1 << 4
                != 0
        {
            return pi.offset(-(1 as isize));
        } else {
            return pi;
        };
    }
}
pub unsafe extern "C" fn patchtestreg(function_state: *mut FunctionState, node: i32, reg: i32) -> i32 {
    unsafe {
        let i: *mut u32 = getjumpcontrol(function_state, node);
        if (*i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
            return 0;
        }
        if reg != (1 << 8) - 1 && reg != (*i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 {
            *i =
                *i & !(!(!(0u32) << 8) << POSITION_A) | (reg as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
        } else {
            *i = (OP_TEST as u32) << 0
                | ((*i >> POSITION_B & !(!(0u32) << 8) << 0) as u32) << POSITION_A
                | (0u32) << POSITION_B
                | (0u32) << POSITION_C
                | ((*i >> POSITION_K & !(!(0u32) << 1) << 0) as u32) << POSITION_K;
        }
        return 1;
    }
}
pub unsafe extern "C" fn removevalues(function_state: *mut FunctionState, mut list: i32) {
    unsafe {
        while list != -1 {
            patchtestreg(function_state, list, (1 << 8) - 1);
            list = getjump(function_state, list);
        }
    }
}
pub unsafe extern "C" fn patchlistaux(
    function_state: *mut FunctionState,
    mut list: i32,
    vtarget: i32,
    reg: i32,
    dtarget: i32,
) {
    unsafe {
        while list != -1 {
            let next: i32 = getjump(function_state, list);
            if patchtestreg(function_state, list, reg) != 0 {
                fixjump(function_state, list, vtarget);
            } else {
                fixjump(function_state, list, dtarget);
            }
            list = next;
        }
    }
}
pub unsafe extern "C" fn luak_patchlist(function_state: *mut FunctionState, list: i32, target: i32) {
    unsafe {
        patchlistaux(function_state, list, target, (1 << 8) - 1, target);
    }
}
pub unsafe extern "C" fn luak_patchtohere(function_state: *mut FunctionState, list: i32) {
    unsafe {
        let hr: i32 = luak_getlabel(function_state);
        luak_patchlist(function_state, list, hr);
    }
}
pub unsafe extern "C" fn savelineinfo(function_state: *mut FunctionState, prototype: *mut Prototype, line: i32) {
    unsafe {
        let mut linedif: i32 = line - (*function_state).previous_line;
        let program_counter: i32 = (*function_state).program_counter - 1;
        if abs(linedif) >= 0x80 as i32 || {
            let fresh132 = (*function_state).iwthabs;
            (*function_state).iwthabs = ((*function_state).iwthabs).wrapping_add(1);
            fresh132 as i32 >= 128 as i32
        } {
            (*prototype).absolute_line_info = luam_growaux_(
                (*(*function_state).lexical_state).state,
                (*prototype).absolute_line_info as *mut libc::c_void,
                (*function_state).count_abslineinfo,
                &mut (*prototype).size_absolute_line_info,
                ::core::mem::size_of::<AbsoluteLineInfo>() as i32,
                (if 0x7FFFFFFF as u64
                    <= (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64)
                {
                    0x7FFFFFFF as u32
                } else {
                    (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64) as u32
                }) as i32,
                b"lines\0" as *const u8 as *const i8,
            ) as *mut AbsoluteLineInfo;
            (*((*prototype).absolute_line_info).offset((*function_state).count_abslineinfo as isize)).program_counter =
                program_counter;
            let fresh133 = (*function_state).count_abslineinfo;
            (*function_state).count_abslineinfo = (*function_state).count_abslineinfo + 1;
            (*((*prototype).absolute_line_info).offset(fresh133 as isize)).line = line;
            linedif = -(0x80 as i32);
            (*function_state).iwthabs = 1;
        }
        (*prototype).line_info = luam_growaux_(
            (*(*function_state).lexical_state).state,
            (*prototype).line_info as *mut libc::c_void,
            program_counter,
            &mut (*prototype).size_line_info,
            ::core::mem::size_of::<i8>() as i32,
            (if 0x7FFFFFFF as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64)
            {
                0x7FFFFFFF as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64) as u32
            }) as i32,
            b"opcodes\0" as *const u8 as *const i8,
        ) as *mut i8;
        *((*prototype).line_info).offset(program_counter as isize) = linedif as i8;
        (*function_state).previous_line = line;
    }
}
pub unsafe extern "C" fn removelastlineinfo(function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let program_counter: i32 = (*function_state).program_counter - 1;
        if *((*prototype).line_info).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            (*function_state).previous_line -= *((*prototype).line_info).offset(program_counter as isize) as i32;
            (*function_state).iwthabs = ((*function_state).iwthabs).wrapping_sub(1);
            (*function_state).iwthabs;
        } else {
            (*function_state).count_abslineinfo -= 1;
            (*function_state).count_abslineinfo;
            (*function_state).iwthabs = (128 as i32 + 1) as u8;
        };
    }
}
pub unsafe extern "C" fn removelastinstruction(function_state: *mut FunctionState) {
    unsafe {
        removelastlineinfo(function_state);
        (*function_state).program_counter -= 1;
        (*function_state).program_counter;
    }
}
pub unsafe extern "C" fn luak_code(function_state: *mut FunctionState, i: u32) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        (*prototype).code = luam_growaux_(
            (*(*function_state).lexical_state).state,
            (*prototype).code as *mut libc::c_void,
            (*function_state).program_counter,
            &mut (*prototype).size_code,
            ::core::mem::size_of::<u32>() as i32,
            (if 0x7FFFFFFF as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<u32>() as u64)
            {
                0x7FFFFFFF as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<u32>() as u64) as u32
            }) as i32,
            b"opcodes\0" as *const u8 as *const i8,
        ) as *mut u32;
        let fresh134 = (*function_state).program_counter;
        (*function_state).program_counter = (*function_state).program_counter + 1;
        *((*prototype).code).offset(fresh134 as isize) = i;
        savelineinfo(function_state, prototype, (*(*function_state).lexical_state).last_line);
        return (*function_state).program_counter - 1;
    }
}
pub unsafe extern "C" fn luak_code_abck(
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        return luak_code(
            function_state,
            (o as u32) << 0
                | (a as u32) << POSITION_A
                | (b as u32) << POSITION_B
                | (c as u32) << POSITION_C
                | (k as u32) << POSITION_K,
        );
    }
}
pub unsafe extern "C" fn luak_codeabx(function_state: *mut FunctionState, o: u32, a: i32, bc: u32) -> i32 {
    unsafe {
        return luak_code(function_state, (o as u32) << 0 | (a as u32) << POSITION_A | bc << POSITION_K);
    }
}
pub unsafe extern "C" fn codeasbx(function_state: *mut FunctionState, o: u32, a: i32, bc: i32) -> i32 {
    unsafe {
        let b: u32 = (bc + ((1 << 8 + 8 + 1) - 1 >> 1)) as u32;
        return luak_code(function_state, (o as u32) << 0 | (a as u32) << POSITION_A | b << POSITION_K);
    }
}
pub unsafe extern "C" fn codesj(function_state: *mut FunctionState, o: u32, sj: i32, k: i32) -> i32 {
    unsafe {
        let j: u32 = (sj + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32;
        return luak_code(function_state, (o as u32) << 0 | j << POSITION_A | (k as u32) << POSITION_K);
    }
}
pub unsafe extern "C" fn codeextraarg(function_state: *mut FunctionState, a: i32) -> i32 {
    unsafe {
        return luak_code(function_state, (OP_EXTRAARG as u32) << 0 | (a as u32) << POSITION_A);
    }
}
pub unsafe extern "C" fn luak_codek(function_state: *mut FunctionState, reg: i32, k: i32) -> i32 {
    unsafe {
        if k <= (1 << 8 + 8 + 1) - 1 {
            return luak_codeabx(function_state, OP_LOADK, reg, k as u32);
        } else {
            let p: i32 = luak_codeabx(function_state, OP_LOADKX, reg, 0u32);
            codeextraarg(function_state, k);
            return p;
        };
    }
}
pub unsafe extern "C" fn luak_checkstack(function_state: *mut FunctionState, n: i32) {
    unsafe {
        let newstack: i32 = (*function_state).freereg as i32 + n;
        if newstack > (*(*function_state).prototype).maximum_stack_size as i32 {
            if newstack >= 255 as i32 {
                luax_syntaxerror(
                    (*function_state).lexical_state,
                    b"function or expression needs too many registers\0" as *const u8 as *const i8,
                );
            }
            (*(*function_state).prototype).maximum_stack_size = newstack as u8;
        }
    }
}
pub unsafe extern "C" fn luak_reserveregs(function_state: *mut FunctionState, n: i32) {
    unsafe {
        luak_checkstack(function_state, n);
        (*function_state).freereg = ((*function_state).freereg as i32 + n) as u8;
    }
}
pub unsafe extern "C" fn freereg(function_state: *mut FunctionState, reg: i32) {
    unsafe {
        if reg >= luay_nvarstack(function_state) {
            (*function_state).freereg = ((*function_state).freereg).wrapping_sub(1);
            (*function_state).freereg;
        }
    }
}
pub unsafe extern "C" fn freeregs(function_state: *mut FunctionState, r1: i32, r2: i32) {
    unsafe {
        if r1 > r2 {
            freereg(function_state, r1);
            freereg(function_state, r2);
        } else {
            freereg(function_state, r2);
            freereg(function_state, r1);
        };
    }
}
pub unsafe extern "C" fn freeexp(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).expression_kind as u32 == ExpressionKind::VNONRELOC as u32 {
            freereg(function_state, (*e).value.info);
        }
    }
}
pub unsafe extern "C" fn freeexps(
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32 = if (*e1).expression_kind as u32 == ExpressionKind::VNONRELOC as u32 {
            (*e1).value.info
        } else {
            -1
        };
        let r2: i32 = if (*e2).expression_kind as u32 == ExpressionKind::VNONRELOC as u32 {
            (*e2).value.info
        } else {
            -1
        };
        freeregs(function_state, r1, r2);
    }
}
pub unsafe extern "C" fn addk(function_state: *mut FunctionState, key: *mut TValue, v: *mut TValue) -> i32 {
    unsafe {
        let mut value: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let state: *mut State = (*(*function_state).lexical_state).state;
        let prototype: *mut Prototype = (*function_state).prototype;
        let index: *const TValue = luah_get((*(*function_state).lexical_state).table, key);
        let mut k: i32;
        if (*index).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            k = (*index).value.integer as i32;
            if k < (*function_state).count_k
                && (*((*prototype).k).offset(k as isize)).get_tag_variant() == (*v).get_tag_variant()
                && luav_equalobj(std::ptr::null_mut(), &mut *((*prototype).k).offset(k as isize), v)
            {
                return k;
            }
        }
        let mut old_size: i32 = (*prototype).size_k;
        k = (*function_state).count_k;
        let io: *mut TValue = &mut value;
        (*io).value.integer = k as i64;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        luah_finishset(state, (*(*function_state).lexical_state).table, key, index, &mut value);
        (*prototype).k = luam_growaux_(
            state,
            (*prototype).k as *mut libc::c_void,
            k,
            &mut (*prototype).size_k,
            ::core::mem::size_of::<TValue>() as i32,
            (if ((1 << 8 + 8 + 1 + 8) - 1) as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64)
            {
                ((1 << 8 + 8 + 1 + 8) - 1) as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64) as u32
            }) as i32,
            b"constants\0" as *const u8 as *const i8,
        ) as *mut TValue;
        while old_size < (*prototype).size_k {
            let fresh135 = old_size;
            old_size = old_size + 1;
            (*((*prototype).k).offset(fresh135 as isize)).set_tag(TAG_VARIANT_NIL_NIL);
        }
        let io1: *mut TValue = &mut *((*prototype).k).offset(k as isize) as *mut TValue;
        let io2: *const TValue = v;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        (*function_state).count_k += 1;
        (*function_state).count_k;
        if (*v).is_collectable() {
            if (*prototype).get_marked() & 1 << 5 != 0
                && (*(*v).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    state,
                    &mut (*(prototype as *mut Object)),
                    &mut (*((*v).value.object as *mut Object)),
                );
            } else {
            };
        } else {
        };
        return k;
    }
}
pub unsafe extern "C" fn string_k(function_state: *mut FunctionState, s: *mut TString) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut o;
        let x_: *mut TString = s;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag((*x_).get_tag());
        (*io).set_collectable();
        return addk(function_state, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn luak_int_k(function_state: *mut FunctionState, n: i64) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut o;
        (*io).value.integer = n;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        return addk(function_state, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn luak_number_k(function_state: *mut FunctionState, r: f64) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let mut ik: i64 = 0;
        let io: *mut TValue = &mut o;
        (*io).value.number = r;
        (*io).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        if !luav_flttointeger(r, &mut ik, F2I::Equal) {
            return addk(function_state, &mut o, &mut o);
        } else {
            let nbm: i32 = 53 as i32;
            let q: f64 = ldexp_(1.0f64, -nbm + 1);
            let k: f64 = if ik == 0 { q } else { r + r * q };
            let mut kv: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io_0: *mut TValue = &mut kv;
            (*io_0).value.number = k;
            (*io_0).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            return addk(function_state, &mut kv, &mut o);
        };
    }
}
pub unsafe extern "C" fn bool_false(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        o.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
        return addk(function_state, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn bool_true(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        o.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
        return addk(function_state, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn nil_k(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut k: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let mut v: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        v.set_tag(TAG_VARIANT_NIL_NIL);
        let io: *mut TValue = &mut k;
        let x_: *mut Table = (*(*function_state).lexical_state).table;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_TABLE);
        (*io).set_collectable();
        return addk(function_state, &mut k, &mut v);
    }
}
pub unsafe extern "C" fn luak_int(function_state: *mut FunctionState, reg: i32, i: i64) {
    unsafe {
        if fits_bx(i) {
            codeasbx(function_state, OP_LOADI, reg, i as i32);
        } else {
            luak_codek(function_state, reg, luak_int_k(function_state, i));
        };
    }
}
pub unsafe extern "C" fn luak_float(function_state: *mut FunctionState, reg: i32, number: f64) {
    unsafe {
        let mut fi: i64 = 0;
        if luav_flttointeger(number, &mut fi, F2I::Equal) && fits_bx(fi) {
            codeasbx(function_state, OP_LOADF, reg, fi as i32);
        } else {
            luak_codek(function_state, reg, luak_number_k(function_state, number));
        };
    }
}
pub unsafe extern "C" fn luak_setreturns(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
    count_results: i32,
) {
    unsafe {
        let program_counter: *mut u32 =
            &mut *((*(*function_state).prototype).code).offset((*e).value.info as isize) as *mut u32;
        if (*e).expression_kind as u32 == ExpressionKind::VCALL as u32 {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C
                    & !(!(0u32) << 8) << POSITION_C;
        } else {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C
                    & !(!(0u32) << 8) << POSITION_C;
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_A)
                | ((*function_state).freereg as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
            luak_reserveregs(function_state, 1);
        };
    }
}
pub unsafe extern "C" fn str_to_k(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        (*e).value.info = string_k(function_state, (*e).value.tstring);
        (*e).expression_kind = ExpressionKind::VK;
    }
}
pub unsafe extern "C" fn luak_setoneret(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).expression_kind as u32 == ExpressionKind::VCALL as u32 {
            (*e).expression_kind = ExpressionKind::VNONRELOC;
            (*e).value.info = (*((*(*function_state).prototype).code).offset((*e).value.info as isize) >> POSITION_A
                & !(!(0u32) << 8) << 0) as i32;
        } else if (*e).expression_kind as u32 == ExpressionKind::VVARARG as u32 {
            *((*(*function_state).prototype).code).offset((*e).value.info as isize) = *((*(*function_state).prototype).code)
                .offset((*e).value.info as isize)
                & !(!(!(0u32) << 8) << POSITION_C)
                | (2 as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
            (*e).expression_kind = ExpressionKind::VRELOC;
        }
    }
}
pub unsafe extern "C" fn luak_dischargevars(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        match (*e).expression_kind {
            ExpressionKind::VCONST => {
                const2exp(const2val(function_state, e), e);
            }
            ExpressionKind::VLOCAL => {
                let temp: i32 = (*e).value.variable.register_index as i32;
                (*e).value.info = temp;
                (*e).expression_kind = ExpressionKind::VNONRELOC;
            }
            ExpressionKind::VUPVAL => {
                (*e).value.info = luak_code_abck(function_state, OP_GETUPVAL, 0, (*e).value.info, 0, 0);
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            ExpressionKind::VINDEXUP => {
                (*e).value.info = luak_code_abck(
                    function_state,
                    OP_GETTABUP,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            ExpressionKind::VINDEXI => {
                freereg(function_state, (*e).value.index.reference_tag as i32);
                (*e).value.info = luak_code_abck(
                    function_state,
                    OP_GETI,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            ExpressionKind::VINDEXSTR => {
                freereg(function_state, (*e).value.index.reference_tag as i32);
                (*e).value.info = luak_code_abck(
                    function_state,
                    OP_GETFIELD,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            ExpressionKind::VINDEXED => {
                freeregs(function_state, (*e).value.index.reference_tag as i32, (*e).value.index.reference_index as i32);
                (*e).value.info = luak_code_abck(
                    function_state,
                    OP_GETTABLE,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            ExpressionKind::VVARARG | ExpressionKind::VCALL => {
                luak_setoneret(function_state, e);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn discharge2reg(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
    reg: i32,
) {
    unsafe {
        luak_dischargevars(function_state, e);
        let current_block_14: u64;
        match (*e).expression_kind as u32 {
            1 => {
                luak_nil(function_state, reg, 1);
                current_block_14 = 13242334135786603907;
            }
            3 => {
                luak_code_abck(function_state, OP_LOADFALSE, reg, 0, 0, 0);
                current_block_14 = 13242334135786603907;
            }
            2 => {
                luak_code_abck(function_state, OP_LOADTRUE, reg, 0, 0, 0);
                current_block_14 = 13242334135786603907;
            }
            7 => {
                str_to_k(function_state, e);
                current_block_14 = 6937071982253665452;
            }
            4 => {
                current_block_14 = 6937071982253665452;
            }
            5 => {
                luak_float(function_state, reg, (*e).value.number);
                current_block_14 = 13242334135786603907;
            }
            6 => {
                luak_int(function_state, reg, (*e).value.integer);
                current_block_14 = 13242334135786603907;
            }
            17 => {
                let program_counter: *mut u32 =
                    &mut *((*(*function_state).prototype).code).offset((*e).value.info as isize) as *mut u32;
                *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_A)
                    | (reg as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
                current_block_14 = 13242334135786603907;
            }
            8 => {
                if reg != (*e).value.info {
                    luak_code_abck(function_state, OP_MOVE, reg, (*e).value.info, 0, 0);
                }
                current_block_14 = 13242334135786603907;
            }
            _ => return,
        }
        match current_block_14 {
            6937071982253665452 => {
                luak_codek(function_state, reg, (*e).value.info);
            }
            _ => {}
        }
        (*e).value.info = reg;
        (*e).expression_kind = ExpressionKind::VNONRELOC;
    }
}
pub unsafe extern "C" fn discharge2anyreg(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).expression_kind as u32 != ExpressionKind::VNONRELOC as u32 {
            luak_reserveregs(function_state, 1);
            discharge2reg(function_state, e, (*function_state).freereg as i32 - 1);
        }
    }
}
pub unsafe extern "C" fn code_loadbool(function_state: *mut FunctionState, a: i32, op: u32) -> i32 {
    unsafe {
        luak_getlabel(function_state);
        return luak_code_abck(function_state, op, a, 0, 0, 0);
    }
}
pub unsafe extern "C" fn need_value(function_state: *mut FunctionState, mut list: i32) -> i32 {
    unsafe {
        while list != -1 {
            let i: u32 = *getjumpcontrol(function_state, list);
            if (i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
                return 1;
            }
            list = getjump(function_state, list);
        }
        return 0;
    }
}
pub unsafe extern "C" fn exp2reg(function_state: *mut FunctionState, e: *mut ExpressionDescription, reg: i32) {
    unsafe {
        discharge2reg(function_state, e, reg);
        if (*e).expression_kind as u32 == ExpressionKind::VJMP as u32 {
            luak_concat(function_state, &mut (*e).t, (*e).value.info);
        }
        if (*e).t != (*e).f {
            let mut p_f: i32 = -1;
            let mut p_t: i32 = -1;
            if need_value(function_state, (*e).t) != 0 || need_value(function_state, (*e).f) != 0 {
                let fj: i32 = if (*e).expression_kind as u32 == ExpressionKind::VJMP as u32 {
                    -1
                } else {
                    luak_jump(function_state)
                };
                p_f = code_loadbool(function_state, reg, OP_LFALSESKIP);
                p_t = code_loadbool(function_state, reg, OP_LOADTRUE);
                luak_patchtohere(function_state, fj);
            }
            let final_0: i32 = luak_getlabel(function_state);
            patchlistaux(function_state, (*e).f, final_0, reg, p_f);
            patchlistaux(function_state, (*e).t, final_0, reg, p_t);
        }
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).value.info = reg;
        (*e).expression_kind = ExpressionKind::VNONRELOC;
    }
}
pub unsafe extern "C" fn luak_exp2nextreg(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        luak_dischargevars(function_state, e);
        freeexp(function_state, e);
        luak_reserveregs(function_state, 1);
        exp2reg(function_state, e, (*function_state).freereg as i32 - 1);
    }
}
pub unsafe extern "C" fn luak_exp2anyreg(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        luak_dischargevars(function_state, e);
        if (*e).expression_kind as u32 == ExpressionKind::VNONRELOC as u32 {
            if !((*e).t != (*e).f) {
                return (*e).value.info;
            }
            if (*e).value.info >= luay_nvarstack(function_state) {
                exp2reg(function_state, e, (*e).value.info);
                return (*e).value.info;
            }
        }
        luak_exp2nextreg(function_state, e);
        return (*e).value.info;
    }
}
pub unsafe extern "C" fn luak_exp2anyregup(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).expression_kind as u32 != ExpressionKind::VUPVAL as u32 || (*e).t != (*e).f {
            luak_exp2anyreg(function_state, e);
        }
    }
}
pub unsafe extern "C" fn luak_exp2val(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).expression_kind as u32 == ExpressionKind::VJMP as u32 || (*e).t != (*e).f {
            luak_exp2anyreg(function_state, e);
        } else {
            luak_dischargevars(function_state, e);
        };
    }
}
pub unsafe extern "C" fn luak_exp2k(function_state: *mut FunctionState, e: *mut ExpressionDescription) -> i32 {
    unsafe {
        if !((*e).t != (*e).f) {
            let info: i32;
            match (*e).expression_kind as u32 {
                2 => {
                    info = bool_true(function_state);
                }
                3 => {
                    info = bool_false(function_state);
                }
                1 => {
                    info = nil_k(function_state);
                }
                6 => {
                    info = luak_int_k(function_state, (*e).value.integer);
                }
                5 => {
                    info = luak_number_k(function_state, (*e).value.number);
                }
                7 => {
                    info = string_k(function_state, (*e).value.tstring);
                }
                4 => {
                    info = (*e).value.info;
                }
                _ => return 0,
            }
            if info <= (1 << 8) - 1 {
                (*e).expression_kind = ExpressionKind::VK;
                (*e).value.info = info;
                return 1;
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn exp2rk(function_state: *mut FunctionState, e: *mut ExpressionDescription) -> i32 {
    unsafe {
        if luak_exp2k(function_state, e) != 0 {
            return 1;
        } else {
            luak_exp2anyreg(function_state, e);
            return 0;
        };
    }
}
pub unsafe extern "C" fn codeabrk(
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    ec: *mut ExpressionDescription,
) {
    unsafe {
        let k: i32 = exp2rk(function_state, ec);
        luak_code_abck(function_state, o, a, b, (*ec).value.info, k);
    }
}
pub unsafe extern "C" fn luak_storevar(
    function_state: *mut FunctionState,
    var: *mut ExpressionDescription,
    ex: *mut ExpressionDescription,
) {
    unsafe {
        match (*var).expression_kind {
            ExpressionKind::VLOCAL => {
                freeexp(function_state, ex);
                exp2reg(function_state, ex, (*var).value.variable.register_index as i32);
                return;
            }
            ExpressionKind::VUPVAL => {
                let e: i32 = luak_exp2anyreg(function_state, ex);
                luak_code_abck(function_state, OP_SETUPVAL, e, (*var).value.info, 0, 0);
            }
            ExpressionKind::VINDEXUP => {
                codeabrk(
                    function_state,
                    OP_SETTABUP,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::VINDEXI => {
                codeabrk(
                    function_state,
                    OP_SETI,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::VINDEXSTR => {
                codeabrk(
                    function_state,
                    OP_SETFIELD,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::VINDEXED => {
                codeabrk(
                    function_state,
                    OP_SETTABLE,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            _ => {}
        }
        freeexp(function_state, ex);
    }
}
pub unsafe extern "C" fn luak_self(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
    key: *mut ExpressionDescription,
) {
    unsafe {
        luak_exp2anyreg(function_state, e);
        let ereg: i32 = (*e).value.info;
        freeexp(function_state, e);
        (*e).value.info = (*function_state).freereg as i32;
        (*e).expression_kind = ExpressionKind::VNONRELOC;
        luak_reserveregs(function_state, 2);
        codeabrk(function_state, OP_SELF, (*e).value.info, ereg, key);
        freeexp(function_state, key);
    }
}
pub unsafe extern "C" fn negatecondition(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: *mut u32 = getjumpcontrol(function_state, (*e).value.info);
        *program_counter = *program_counter & !(!(!(0u32) << 1) << POSITION_K)
            | (((*program_counter >> POSITION_K & !(!(0u32) << 1) << 0) as i32 ^ 1) as u32)
                << POSITION_K
                & !(!(0u32) << 1) << POSITION_K;
    }
}
pub unsafe extern "C" fn jumponcond(
    function_state: *mut FunctionState,
    e: *mut ExpressionDescription,
    cond_0: i32,
) -> i32 {
    unsafe {
        if (*e).expression_kind as u32 == ExpressionKind::VRELOC as u32 {
            let ie: u32 = *((*(*function_state).prototype).code).offset((*e).value.info as isize);
            if (ie >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_NOT as u32 {
                removelastinstruction(function_state);
                return condjump(
                    function_state,
                    OP_TEST,
                    (ie >> POSITION_B & !(!(0u32) << 8) << 0) as i32,
                    0,
                    0,
                    (cond_0 == 0) as i32,
                );
            }
        }
        discharge2anyreg(function_state, e);
        freeexp(function_state, e);
        return condjump(function_state, OP_TESTSET, (1 << 8) - 1, (*e).value.info, 0, cond_0);
    }
}
pub unsafe extern "C" fn luak_goiftrue(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(function_state, e);
        match (*e).expression_kind as u32 {
            16 => {
                negatecondition(function_state, e);
                program_counter = (*e).value.info;
            }
            4 | 5 | 6 | 7 | 2 => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(function_state, e, 0);
            }
        }
        luak_concat(function_state, &mut (*e).f, program_counter);
        luak_patchtohere(function_state, (*e).t);
        (*e).t = -1;
    }
}
pub unsafe extern "C" fn luak_goiffalse(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(function_state, e);
        match (*e).expression_kind as u32 {
            16 => {
                program_counter = (*e).value.info;
            }
            1 | 3 => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(function_state, e, 1);
            }
        }
        luak_concat(function_state, &mut (*e).t, program_counter);
        luak_patchtohere(function_state, (*e).f);
        (*e).f = -1;
    }
}
pub unsafe extern "C" fn codenot(function_state: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        match (*e).expression_kind as u32 {
            1 | 3 => {
                (*e).expression_kind = ExpressionKind::VTRUE;
            }
            4 | 5 | 6 | 7 | 2 => {
                (*e).expression_kind = ExpressionKind::VFALSE;
            }
            16 => {
                negatecondition(function_state, e);
            }
            17 | 8 => {
                discharge2anyreg(function_state, e);
                freeexp(function_state, e);
                (*e).value.info = luak_code_abck(function_state, OP_NOT, 0, (*e).value.info, 0, 0);
                (*e).expression_kind = ExpressionKind::VRELOC;
            }
            _ => {}
        }
        let temp: i32 = (*e).f;
        (*e).f = (*e).t;
        (*e).t = temp;
        removevalues(function_state, (*e).f);
        removevalues(function_state, (*e).t);
    }
}
pub unsafe extern "C" fn is_k_string(function_state: *mut FunctionState, e: *mut ExpressionDescription) -> bool{
    unsafe {
        return (*e).expression_kind == ExpressionKind::VK
            && !((*e).t != (*e).f)
            && (*e).value.info <= ((1 << 8) - 1)
            && (*((*(*function_state).prototype).k).offset((*e).value.info as isize)).get_tag_variant()
                == TAG_VARIANT_STRING_SHORT;
    }
}
pub unsafe extern "C" fn constfolding(
    function_state: *mut FunctionState,
    op: i32,
    e1: *mut ExpressionDescription,
    e2: *const ExpressionDescription,
) -> i32 {
    unsafe {
        let mut v1: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let mut v2: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let mut res: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        if !tonumeral(e1, &mut v1)
            || !tonumeral(e2, &mut v2)
            || validop(op, &mut v1, &mut v2) == 0
        {
            return 0;
        }
        luao_rawarith((*(*function_state).lexical_state).state, op, &mut v1, &mut v2, &mut res);
        if res.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            (*e1).expression_kind = ExpressionKind::VKINT;
            (*e1).value.integer = res.value.integer;
        } else {
            let n: f64 = res.value.number;
            if !(n == n) || n == 0.0 {
                return 0;
            }
            (*e1).expression_kind = ExpressionKind::VKFLT;
            (*e1).value.number = n;
        }
        return 1;
    }
}
pub unsafe extern "C" fn codeunexpval(
    function_state: *mut FunctionState,
    op: u32,
    e: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let r: i32 = luak_exp2anyreg(function_state, e);
        freeexp(function_state, e);
        (*e).value.info = luak_code_abck(function_state, op, 0, r, 0, 0);
        (*e).expression_kind = ExpressionKind::VRELOC;
        luak_fixline(function_state, line);
    }
}
pub unsafe extern "C" fn finishbinexpval(
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    op: u32,
    v2: i32,
    flip: i32,
    line: i32,
    mmop: u32,
    event: u32,
) {
    unsafe {
        let v1: i32 = luak_exp2anyreg(function_state, e1);
        let program_counter: i32 = luak_code_abck(function_state, op, 0, v1, v2, 0);
        freeexps(function_state, e1, e2);
        (*e1).value.info = program_counter;
        (*e1).expression_kind = ExpressionKind::VRELOC;
        luak_fixline(function_state, line);
        luak_code_abck(function_state, mmop, v1, v2, event as i32, flip);
        luak_fixline(function_state, line);
    }
}
pub unsafe extern "C" fn codebinexpval(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let op: u32 = binopr2op(opr, OPR_ADD, OP_ADD);
        let v2: i32 = luak_exp2anyreg(function_state, e2);
        finishbinexpval(function_state, e1, e2, op, v2, 0, line, OP_MMBIN, binopr2tm(opr));
    }
}
pub unsafe extern "C" fn codebini(
    function_state: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
    event: u32,
) {
    unsafe {
        let v2: i32 = (*e2).value.integer as i32 + ((1 << 8) - 1 >> 1);
        finishbinexpval(function_state, e1, e2, op, v2, flip, line, OP_MMBINI, event);
    }
}
pub unsafe extern "C" fn codebink(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        let event: u32 = binopr2tm(opr);
        let v2: i32 = (*e2).value.info;
        let op: u32 = binopr2op(opr, OPR_ADD, OP_ADDK);
        finishbinexpval(function_state, e1, e2, op, v2, flip, line, OP_MMBINK, event);
    }
}
pub unsafe extern "C" fn finishbinexpneg(
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    op: u32,
    line: i32,
    event: u32,
) -> i32 {
    unsafe {
        if !is_k_int(e2) {
            return 0;
        } else {
            let i2: i64 = (*e2).value.integer;
            if !(fits_c(i2) && fits_c(-i2)) {
                return 0;
            } else {
                let v2: i32 = i2 as i32;
                finishbinexpval(
                    function_state,
                    e1,
                    e2,
                    op,
                    -v2 + ((1 << 8) - 1 >> 1),
                    0,
                    line,
                    OP_MMBINI,
                    event,
                );
                *((*(*function_state).prototype).code).offset(((*function_state).program_counter - 1) as isize) =
                    *((*(*function_state).prototype).code).offset(((*function_state).program_counter - 1) as isize)
                        & !(!(!(0u32) << 8) << POSITION_B)
                        | ((v2 + ((1 << 8) - 1 >> 1)) as u32) << POSITION_B
                            & !(!(0u32) << 8) << POSITION_B;
                return 1;
            }
        };
    }
}
pub unsafe extern "C" fn codebinnok(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if flip != 0 {
            swapexps(e1, e2);
        }
        codebinexpval(function_state, opr, e1, e2, line);
    }
}
pub unsafe extern "C" fn codearith(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if tonumeral(e2, std::ptr::null_mut()) && luak_exp2k(function_state, e2) != 0{
            codebink(function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codecommutative(
    function_state: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if tonumeral(e1, std::ptr::null_mut()) {
            swapexps(e1, e2);
            flip = 1;
        }
        if op as u32 == OPR_ADD as u32 && is_sc_int(e2) {
            codebini(function_state, OP_ADDI, e1, e2, flip, line, TM_ADD);
        } else {
            codearith(function_state, op, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codebitwise(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if (*e1).expression_kind == ExpressionKind::VKINT {
            swapexps(e1, e2);
            flip = 1;
        }
        if (*e2).expression_kind == ExpressionKind::VKINT && luak_exp2k(function_state, e2) != 0 {
            codebink(function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codeorder(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32;
        let r2: i32;
        let mut im: i32 = 0;
        let mut is_float: bool = false;
        let op: u32;
        if is_sc_number(e2, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(function_state, e1);
            r2 = im;
            op = binopr2op(opr, OPR_LT, OP_LTI);
        } else if is_sc_number(e1, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(function_state, e2);
            r2 = im;
            op = binopr2op(opr, OPR_LT, OP_GTI);
        } else {
            r1 = luak_exp2anyreg(function_state, e1);
            r2 = luak_exp2anyreg(function_state, e2);
            op = binopr2op(opr, OPR_LT, OP_LT);
        }
        freeexps(function_state, e1, e2);
        (*e1).value.info = condjump(function_state, op, r1, r2, is_float as i32, 1);
        (*e1).expression_kind = ExpressionKind::VJMP;
    }
}
pub unsafe extern "C" fn codeeq(
    function_state: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32;
        let r2: i32;
        let mut im: i32 = 0;
        let mut is_float: bool = false;
        let op: u32;
        if (*e1).expression_kind as u32 != ExpressionKind::VNONRELOC as u32 {
            swapexps(e1, e2);
        }
        r1 = luak_exp2anyreg(function_state, e1);
        if is_sc_number(e2, &mut im, &mut is_float) != 0 {
            op = OP_EQI;
            r2 = im;
        } else if exp2rk(function_state, e2) != 0 {
            op = OP_EQK;
            r2 = (*e2).value.info;
        } else {
            op = OP_EQ;
            r2 = luak_exp2anyreg(function_state, e2);
        }
        freeexps(function_state, e1, e2);
        (*e1).value.info = condjump(
            function_state,
            op,
            r1,
            r2,
            is_float as i32,
            (opr as u32 == OPR_EQ as u32) as i32,
        );
        (*e1).expression_kind = ExpressionKind::VJMP;
    }
}
pub unsafe extern "C" fn luak_prefix(
    function_state: *mut FunctionState,
    unary: OperatorUnary,
    e: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        pub const EF: ExpressionDescription = {
            let init = ExpressionDescription {
                expression_kind: ExpressionKind::VKINT,
                value: Value { integer: 0 },
                t: -1,
                f: -1,
            };
            init
        };
        luak_dischargevars(function_state, e);
        let current_block_3: u64;
        match unary {
            OperatorUnary::Minus | OperatorUnary::BitwiseNot => {
                if constfolding(
                    function_state,
                    (unary as u32).wrapping_add(12 as u32) as i32,
                    e,
                    &EF,
                ) != 0
                {
                    current_block_3 = 7815301370352969686;
                } else {
                    current_block_3 = 4051245927518328098;
                }
            }
            OperatorUnary::Length => {
                current_block_3 = 4051245927518328098;
            }
            OperatorUnary::Not => {
                codenot(function_state, e);
                current_block_3 = 7815301370352969686;
            }
            _ => {
                current_block_3 = 7815301370352969686;
            }
        }
        match current_block_3 {
            4051245927518328098 => {
                codeunexpval(function_state, unopr2op(unary), e, line);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luak_infix(
    function_state: *mut FunctionState,
    op: u32,
    v: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(function_state, v);
        match op as u32 {
            OP_NEWTABLE => {
                luak_goiftrue(function_state, v);
            }
            OP_SELF => {
                luak_goiffalse(function_state, v);
            }
            OP_GETTABLE => {
                luak_exp2nextreg(function_state, v);
            }
            OP_MOVE | OP_LOADI | OP_LOADF | OP_LOADFALSE | OP_LFALSESKIP | OP_LOADK | OP_LOADKX | OP_LOADTRUE | OP_LOADNIL | OP_GETUPVAL | OP_SETUPVAL | OP_GETTABUP => {
                if !tonumeral(v, std::ptr::null_mut()) {
                    luak_exp2anyreg(function_state, v);
                }
            }
            OP_GETI | OP_SETTABLE => {
                if !tonumeral(v, std::ptr::null_mut()) {
                    exp2rk(function_state, v);
                }
            }
            OP_GETFIELD | OP_SETTABUP | OP_SETI | OP_SETFIELD => {
                let mut dummy: i32 = 0;
                let mut dummy2: bool = false;
                if is_sc_number(v, &mut dummy, &mut dummy2) == 0 {
                    luak_exp2anyreg(function_state, v);
                }
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn codeconcat(
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let ie2: *mut u32 = previousinstruction(function_state);
        if (*ie2 >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_CONCAT as u32 {
            let n: i32 = (*ie2 >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
            freeexp(function_state, e2);
            *ie2 = *ie2 & !(!(!(0u32) << 8) << POSITION_A)
                | ((*e1).value.info as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
            *ie2 = *ie2 & !(!(!(0u32) << 8) << POSITION_B)
                | ((n + 1) as u32) << POSITION_B & !(!(0u32) << 8) << POSITION_B;
        } else {
            luak_code_abck(function_state, OP_CONCAT, (*e1).value.info, 2, 0, 0);
            freeexp(function_state, e2);
            luak_fixline(function_state, line);
        };
    }
}
pub unsafe extern "C" fn luak_posfix(
    function_state: *mut FunctionState,
    mut opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        luak_dischargevars(function_state, e2);
        if opr as u32 <= OPR_SHR as u32
            && constfolding(function_state, (opr as u32).wrapping_add(0u32) as i32, e1, e2) != 0
        {
            return;
        }
        let current_block_30: u64;
        match opr as u32 {
            OP_NEWTABLE => {
                luak_concat(function_state, &mut (*e2).f, (*e1).f);
                *e1 = *e2;
                current_block_30 = 8180496224585318153;
            }
            OPR_OR => {
                luak_concat(function_state, &mut (*e2).t, (*e1).t);
                *e1 = *e2;
                current_block_30 = 8180496224585318153;
            }
            OPR_CONCAT => {
                luak_exp2nextreg(function_state, e2);
                codeconcat(function_state, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            OPR_ADD | OPR_MUL => {
                codecommutative(function_state, opr, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            OPR_SUB => {
                if finishbinexpneg(function_state, e1, e2, OP_ADDI, line, TM_SUB) != 0 {
                    current_block_30 = 8180496224585318153;
                } else {
                    current_block_30 = 12599329904712511516;
                }
            }
            OPR_POW | OPR_MOD | OPR_DIV | OPR_IDIV => {
                current_block_30 = 12599329904712511516;
            }
            OPR_BAND | OPR_BOR | OPR_BXOR => {
                codebitwise(function_state, opr, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            OPR_SHL => {
                if is_sc_int(e1) {
                    swapexps(e1, e2);
                    codebini(function_state, OP_SHLI, e1, e2, 1, line, TM_SHL);
                } else if !(finishbinexpneg(function_state, e1, e2, OP_SHRI, line, TM_SHL) != 0) {
                    codebinexpval(function_state, opr, e1, e2, line);
                }
                current_block_30 = 8180496224585318153;
            }
            OPR_SHR => {
                if is_sc_int(e2) {
                    codebini(function_state, OP_SHRI, e1, e2, 0, line, TM_SHR);
                } else {
                    codebinexpval(function_state, opr, e1, e2, line);
                }
                current_block_30 = 8180496224585318153;
            }
            OPR_EQ | OPR_NE => {
                codeeq(function_state, opr, e1, e2);
                current_block_30 = 8180496224585318153;
            }
            OPR_GE | OPR_GT => {
                swapexps(e1, e2);
                opr = (opr as u32)
                    .wrapping_sub(OPR_GT as u32)
                    .wrapping_add(OPR_LT as u32) as u32;
                current_block_30 = 1118134448028020070;
            }
            OPR_LE | OPR_LT => {
                current_block_30 = 1118134448028020070;
            }
            _ => {
                current_block_30 = 8180496224585318153;
            }
        }
        match current_block_30 {
            12599329904712511516 => {
                codearith(function_state, opr, e1, e2, 0, line);
            }
            1118134448028020070 => {
                codeorder(function_state, opr, e1, e2);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luak_fixline(function_state: *mut FunctionState, line: i32) {
    unsafe {
        removelastlineinfo(function_state);
        savelineinfo(function_state, (*function_state).prototype, line);
    }
}
pub const POSITION_A: usize = 7;
pub const POSITION_K: usize = POSITION_A + 8;
pub const POSITION_B: usize = POSITION_K + 1;
pub const POSITION_C: usize = POSITION_B + 8;
pub unsafe extern "C" fn luak_settablesize(
    function_state: *mut FunctionState,
    program_counter: i32,
    ra: i32,
    asize: i32,
    hsize: i32,
) {
    unsafe {
        let inst: *mut u32 = &mut *((*(*function_state).prototype).code).offset(program_counter as isize) as *mut u32;
        let rb: i32 = if hsize == 0 {
            0
        } else {
            1 + hsize.ilog2() as i32
        };
        let extra: i32 = asize / ((1 << 8) - 1 + 1);
        let rc: i32 = asize % ((1 << 8) - 1 + 1);
        let k: i32 = (extra > 0) as i32;
        *inst = (OP_NEWTABLE as u32) << 0
            | (ra as u32) << POSITION_A
            | (rb as u32) << POSITION_B
            | (rc as u32) << POSITION_C
            | (k as u32) << POSITION_K;
        *inst.offset(1 as isize) = (OP_EXTRAARG as u32) << 0 | (extra as u32) << POSITION_A;
    }
}
pub unsafe extern "C" fn luak_setlist(
    function_state: *mut FunctionState,
    base: i32,
    mut count_elements: i32,
    mut tostore: i32,
) {
    unsafe {
        if tostore == -1 {
            tostore = 0;
        }
        if count_elements <= (1 << 8) - 1 {
            luak_code_abck(function_state, OP_SETLIST, base, tostore, count_elements, 0);
        } else {
            let extra: i32 = count_elements / ((1 << 8) - 1 + 1);
            count_elements %= (1 << 8) - 1 + 1;
            luak_code_abck(function_state, OP_SETLIST, base, tostore, count_elements, 1);
            codeextraarg(function_state, extra);
        }
        (*function_state).freereg = (base + 1) as u8;
    }
}
pub unsafe extern "C" fn luak_finish(function_state: *mut FunctionState) {
    unsafe {
        let p: *mut Prototype = (*function_state).prototype;
        for i in 0..(*function_state).program_counter {
            let program_counter: *mut u32 = &mut *((*p).code).offset(i as isize) as *mut u32;
            let current_block_7: u64;
            match (*program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 {
                OP_RETURN0 | OP_RETURN1 => {
                    if !((*function_state).needs_close || (*p).is_variable_arguments as i32 != 0) {
                        current_block_7 = 12599329904712511516;
                    } else {
                        *program_counter = *program_counter & !(!(!(0u32) << 7) << 0)
                            | (OP_RETURN as u32) << 0 & !(!(0u32) << 7) << 0;
                        current_block_7 = 11006700562992250127;
                    }
                }
                OP_RETURN | OP_TAILCALL => {
                    current_block_7 = 11006700562992250127;
                }
                OP_JMP => {
                    let target: i32 = final_target((*p).code, i);
                    fixjump(function_state, i, target);
                    current_block_7 = 12599329904712511516;
                }
                _ => {
                    current_block_7 = 12599329904712511516;
                }
            }
            match current_block_7 {
                11006700562992250127 => {
                    if (*function_state).needs_close {
                        *program_counter = *program_counter & !(!(!(0u32) << 1) << POSITION_K)
                            | (1 as u32) << POSITION_K & !(!(0u32) << 1) << POSITION_K;
                    }
                    if (*p).is_variable_arguments {
                        *program_counter = *program_counter
                            & !(!(!(0u32) << 8) << POSITION_C)
                            | (((*p).count_parameters as i32 + 1) as u32) << POSITION_C
                                & !(!(0u32) << 8) << POSITION_C;
                    }
                }
                _ => {}
            }
        }
    }
}

use rlua::*;
use crate::lexical::operatorbinary::*;
use crate::debugger::absolutelineinfo::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::f2i::*;
use crate::interpreter::*;
use crate::labeldescription::*;
use crate::lexical::blockcontrol::*;
use crate::lexical::constructorcontrol::*;
use crate::lexical::lexicalstate::*;
use crate::lexical::operatorunary::*;
use crate::localvariable::*;
use crate::new::*;
use crate::object::*;
use crate::operator_::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::tm::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::variabledescription::*;
use crate::vectort::*;
use crate::vm::instruction::*;
use crate::vm::opcode::*;
use crate::vm::opmode::*;
use libc::*;
use std::ptr::*;
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
    pub count_constants: i32,
    pub count_prototypes: i32,
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
            prototype: null_mut(),
            previous: null_mut(),
            lexical_state: null_mut(),
            block_control: null_mut(),
            program_counter: 0,
            last_target: 0,
            previous_line: 0,
            count_constants: 0,
            count_prototypes: 0,
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
pub unsafe fn movegotosout(
    function_state: *mut FunctionState,
    block_control: *mut BlockControl,
) {
    unsafe {
        let gl: *mut VectorT<LabelDescription> =
            &mut (*(*(*function_state).lexical_state).dynamic_data).goto_;
        for i in (*block_control).first_goto..(*gl).get_length() as i32 {
            let gt: *mut LabelDescription =
                &mut *((*gl).vectort_pointer).offset(i as isize) as *mut LabelDescription;
            if reglevel(function_state, (*gt).count_active_variables as i32)
                > reglevel(
                    function_state,
                    (*block_control).count_active_variables as i32,
                )
            {
                (*gt).close = ((*gt).close as i32 | (*block_control).count_upvalues as i32) as u8;
            }
            (*gt).count_active_variables = (*block_control).count_active_variables;
        }
    }
}
pub unsafe fn enterblock(
    function_state: *mut FunctionState,
    block_control: *mut BlockControl,
    is_loop: bool,
) {
    unsafe {
        (*block_control).is_loop = is_loop;
        (*block_control).count_active_variables = (*function_state).count_active_variables;
        (*block_control).first_label = (*(*(*function_state).lexical_state).dynamic_data)
            .labels
            .get_length() as i32;
        (*block_control).first_goto = (*(*(*function_state).lexical_state).dynamic_data)
            .goto_
            .get_length() as i32;
        (*block_control).count_upvalues = 0;
        (*block_control).is_inside_tbc = !((*function_state).block_control).is_null()
            && (*(*function_state).block_control).is_inside_tbc as i32 != 0;
        (*block_control).previous = (*function_state).block_control;
        (*function_state).block_control = block_control;
    }
}
pub unsafe fn leaveblock(function_state: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*function_state).block_control;
        let lexical_state: *mut LexicalState = (*function_state).lexical_state;
        let mut has_close = false;
        let stklevel: i32 = reglevel(
            function_state,
            (*block_control).count_active_variables as i32,
        );
        removevars(
            function_state,
            (*block_control).count_active_variables as i32,
        );
        if (*block_control).is_loop {
            has_close = (*lexical_state).create_label(
                luas_newlstr(
                    (*lexical_state).interpreter,
                    make_cstring!("break"),
                    (size_of::<[i8; 6]>())
                        .wrapping_div(size_of::<i8>())
                        .wrapping_sub(1),
                ),
                0,
                false,
            );
        }
        if !has_close
            && !((*block_control).previous).is_null()
            && (*block_control).count_upvalues as i32 != 0
        {
            code_abck(function_state, OP_CLOSE, stklevel, 0, 0, 0);
        }
        (*function_state).freereg = stklevel as u8;
        (*(*lexical_state).dynamic_data)
            .labels
            .set_length((*block_control).first_label as usize);
        (*function_state).block_control = (*block_control).previous;
        if !((*block_control).previous).is_null() {
            movegotosout(function_state, block_control);
        } else if (*block_control).first_goto < (*(*lexical_state).dynamic_data).goto_.get_length()  as i32 {
            undefgoto(
                lexical_state,
                &mut *((*(*lexical_state).dynamic_data).goto_.vectort_pointer)
                    .offset((*block_control).first_goto as isize),
            );
        }
    }
}
pub unsafe fn closelistfield(
    function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        if (*constructor_control).expression_description.expression_kind == ExpressionKind::Void {
            return;
        }
        luak_exp2nextreg(function_state, &mut (*constructor_control).expression_description);
        (*constructor_control).expression_description.expression_kind = ExpressionKind::Void;
        if (*constructor_control).count_to_store == 50 as i32 {
            luak_setlist(
                function_state,
                (*(*constructor_control).constructor_control_table).value.info,
                (*constructor_control).count_array as usize,
                (*constructor_control).count_to_store,
            );
            (*constructor_control).count_array += (*constructor_control).count_to_store;
            (*constructor_control).count_to_store = 0;
        }
    }
}
pub unsafe fn lastlistfield(
    function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        if (*constructor_control).count_to_store == 0 {
            return;
        }
        if (*constructor_control).expression_description.expression_kind == ExpressionKind::Call
            || (*constructor_control).expression_description.expression_kind == ExpressionKind::VariableArguments
        {
            luak_setreturns(function_state, &mut (*constructor_control).expression_description, -1);
            luak_setlist(function_state, (*(*constructor_control).constructor_control_table).value.info, (*constructor_control).count_array as usize, -1);
            (*constructor_control).count_array -= 1;
            (*constructor_control).count_array;
        } else {
            if (*constructor_control).expression_description.expression_kind != ExpressionKind::Void {
                luak_exp2nextreg(function_state, &mut (*constructor_control).expression_description);
            }
            luak_setlist(
                function_state,
                (*(*constructor_control).constructor_control_table).value.info,
                (*constructor_control).count_array as usize,
                (*constructor_control).count_to_store,
            );
        }
        (*constructor_control).count_array += (*constructor_control).count_to_store;
    }
}
pub unsafe fn setvararg(function_state: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*function_state).prototype).prototype_is_variable_arguments = true;
        code_abck(function_state, OP_VARARGPREP, nparams, 0, 0, 0);
    }
}
pub unsafe fn errorlimit(
    function_state: *mut FunctionState,
    limit: i32,
    what: *const i8,
) -> ! {
    unsafe {
        let interpreter: *mut Interpreter = (*(*function_state).lexical_state).interpreter;
        let message: *const i8;
        let line: i32 = (*(*function_state).prototype).prototype_line_defined;
        let where_0: *const i8 = if line == 0 {
            make_cstring!("main function")
        } else {
            luao_pushfstring(
                interpreter,
                make_cstring!("function at line %d"),
                line,
            )
        };
        message = luao_pushfstring(
            interpreter,
            make_cstring!("too many %s (limit is %d) in %s"),
            what,
            limit,
            where_0,
        );
        luax_syntaxerror((*function_state).lexical_state, message);
    }
}
pub unsafe fn checklimit(
    function_state: *mut FunctionState,
    v: i32,
    length: i32,
    what: *const i8,
) {
    unsafe {
        if v > length {
            errorlimit(function_state, length, what);
        }
    }
}
pub unsafe fn getlocalvardesc(
    function_state: *mut FunctionState,
    vidx: i32,
) -> *mut VariableDescription {
    unsafe {
        return &mut *((*(*(*function_state).lexical_state).dynamic_data)
            .active_variables
            .vectort_pointer)
            .offset(((*function_state).first_local + vidx) as isize)
            as *mut VariableDescription;
    }
}
pub unsafe fn reglevel(function_state: *mut FunctionState, mut nvar: i32) -> i32 {
    unsafe {
        loop {
            let fresh38 = nvar;
            nvar = nvar - 1;
            if !(fresh38 > 0) {
                break;
            }
            let variable_description: *mut VariableDescription =
                getlocalvardesc(function_state, nvar);
            if (*variable_description).content.kind as i32 != 3 {
                return (*variable_description).content.register_index as i32 + 1;
            }
        }
        return 0;
    }
}
pub unsafe fn luay_nvarstack(function_state: *mut FunctionState) -> i32 {
    unsafe {
        return reglevel(
            function_state,
            (*function_state).count_active_variables as i32,
        );
    }
}
pub unsafe fn localdebuginfo(
    function_state: *mut FunctionState,
    vidx: i32,
) -> *mut LocalVariable {
    unsafe {
        let variable_description: *mut VariableDescription = getlocalvardesc(function_state, vidx);
        if (*variable_description).content.kind as i32 == 3 {
            return null_mut();
        } else {
            let index: i32 = (*variable_description).content.pidx as i32;
            return (*(*function_state).prototype)
                .prototype_local_variables
                .at_mut(index as isize);
        };
    }
}
pub unsafe fn init_var(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    vidx: i32,
) {
    unsafe {
        (*expression_description).t = -1;
        (*expression_description).f = (*expression_description).t;
        (*expression_description).expression_kind = ExpressionKind::Local;
        (*expression_description).value.variable.value_index = vidx as u16;
        (*expression_description).value.variable.register_index = (*getlocalvardesc(function_state, vidx))
            .content
            .register_index;
    }
}
pub unsafe fn removevars(function_state: *mut FunctionState, tolevel: i32) {
    unsafe {
        (*(*(*function_state).lexical_state).dynamic_data)
            .active_variables
            .subtract_length(((*function_state).count_active_variables as i32 - tolevel) as usize);
        while (*function_state).count_active_variables as i32 > tolevel {
            (*function_state).count_active_variables =
                ((*function_state).count_active_variables).wrapping_sub(1);
            let var: *mut LocalVariable = localdebuginfo(
                function_state,
                (*function_state).count_active_variables as i32,
            );
            if !var.is_null() {
                (*var).end_program_counter = (*function_state).program_counter;
            }
        }
    }
}
pub unsafe fn searchupvalue(
    function_state: *mut FunctionState,
    name: *mut TString,
) -> i32 {
    unsafe {
        let up: *mut UpValueDescription = (*(*function_state).prototype)
            .prototype_upvalues
            .vectort_pointer;
        for i in 0..(*function_state).count_upvalues {
            if (*up.offset(i as isize)).name == name {
                return i as i32;
            }
        }
        return -1;
    }
}
pub unsafe fn allocate_upvalue_description(
    function_state: *mut FunctionState,
) -> *mut UpValueDescription {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut old_size = (*prototype).prototype_upvalues.get_size();
        checklimit(
            function_state,
            (*function_state).count_upvalues as i32 + 1,
            255 as i32,
            make_cstring!("upvalues"),
        );
        (*prototype).prototype_upvalues.grow(
            (*(*function_state).lexical_state).interpreter,
            (*function_state).count_upvalues as usize,
            if 255 as usize <= (!(0usize)).wrapping_div(size_of::<UpValueDescription>() as usize) {
                255
            } else {
                (!(0usize)).wrapping_div(size_of::<UpValueDescription>() as usize)
            },
            make_cstring!("upvalues"),
        );
        while old_size < (*prototype).prototype_upvalues.get_size() {
            let fresh41 = old_size;
            old_size = old_size + 1;
            let ref mut fresh42 =
                (*((*prototype).prototype_upvalues.vectort_pointer).offset(fresh41 as isize)).name;
            *fresh42 = null_mut();
        }
        let fresh43 = (*function_state).count_upvalues;
        (*function_state).count_upvalues = ((*function_state).count_upvalues).wrapping_add(1);
        return &mut *((*prototype).prototype_upvalues.vectort_pointer).offset(fresh43 as isize)
            as *mut UpValueDescription;
    }
}
pub unsafe fn newupvalue(
    function_state: *mut FunctionState,
    name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let up: *mut UpValueDescription = allocate_upvalue_description(function_state);
        let previous: *mut FunctionState = (*function_state).previous;
        if (*v).expression_kind == ExpressionKind::Local {
            (*up).is_in_stack = true;
            (*up).index = (*v).value.variable.register_index;
            (*up).kind = (*getlocalvardesc(previous, (*v).value.variable.value_index as i32))
                .content
                .kind;
        } else {
            (*up).is_in_stack = false;
            (*up).index = (*v).value.info as u8;
            (*up).kind = (*((*(*previous).prototype).prototype_upvalues.vectort_pointer)
                .offset((*v).value.info as isize))
            .kind;
        }
        (*up).name = name;
        if (*(*function_state).prototype).get_marked() & 1 << 5 != 0
            && (*name).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                (*(*function_state).lexical_state).interpreter,
                &mut (*((*function_state).prototype as *mut Object)),
                &mut (*(name as *mut Object)),
            );
        } else {
        };
        return (*function_state).count_upvalues as i32 - 1;
    }
}
pub unsafe fn searchvar(
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
                    init_exp(
                        var,
                        ExpressionKind::Constant2,
                        (*function_state).first_local + i,
                    );
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
pub unsafe fn markupval(function_state: *mut FunctionState, level: i32) {
    unsafe {
        let mut block_control: *mut BlockControl = (*function_state).block_control;
        while (*block_control).count_active_variables as i32 > level {
            block_control = (*block_control).previous;
        }
        (*block_control).count_upvalues = 1;
        (*function_state).needs_close = true;
    }
}
pub unsafe fn marktobeclosed(function_state: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*function_state).block_control;
        (*block_control).count_upvalues = 1;
        (*block_control).is_inside_tbc = true;
        (*function_state).needs_close = true;
    }
}
pub unsafe fn singlevaraux(
    function_state: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
    base: i32,
) {
    unsafe {
        if function_state.is_null() {
            init_exp(var, ExpressionKind::Void, 0);
        } else {
            let v: i32 = searchvar(function_state, n, var);
            if v >= 0 {
                if v == ExpressionKind::Local as i32 && base == 0 {
                    markupval(function_state, (*var).value.variable.value_index as i32);
                }
            } else {
                let mut index: i32 = searchupvalue(function_state, n);
                if index < 0 {
                    singlevaraux((*function_state).previous, n, var, 0);
                    if (*var).expression_kind == ExpressionKind::Local
                        || (*var).expression_kind == ExpressionKind::UpValue
                    {
                        index = newupvalue(function_state, n, var);
                    } else {
                        return;
                    }
                }
                init_exp(var, ExpressionKind::UpValue, index);
            }
        };
    }
}
pub unsafe fn fixforjump(
    function_state: *mut FunctionState,
    program_counter: i32,
    dest: i32,
    back: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        let mut offset: i32 = dest - (program_counter + 1);
        if back != 0 {
            offset = -offset;
        }
        if offset > (1 << 8 + 8 + 1) - 1 {
            luax_syntaxerror(
                (*function_state).lexical_state,
                make_cstring!("control structure too long"),
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1) << POSITION_K)
            | (offset as u32) << POSITION_K & !(!(0u32) << 8 + 8 + 1) << POSITION_K;
    }
}
pub unsafe fn checktoclose(function_state: *mut FunctionState, level: i32) {
    unsafe {
        if level != -1 {
            marktobeclosed(function_state);
            code_abck(
                function_state,
                OP_TBC,
                reglevel(function_state, level),
                0,
                0,
                0,
            );
        }
    }
}
pub unsafe fn previousinstruction(function_state: *mut FunctionState) -> *mut u32 {
    unsafe {
        pub const INVALID_INSTRUCTION: u32 = !(0u32);
        if (*function_state).program_counter > (*function_state).last_target {
            return &mut *((*(*function_state).prototype)
                .prototype_code
                .vectort_pointer)
                .offset(((*function_state).program_counter - 1) as isize)
                as *mut u32;
        } else {
            return &INVALID_INSTRUCTION as *const u32 as *mut u32;
        };
    }
}
pub unsafe fn code_constant_nil(function_state: *mut FunctionState, mut from: i32, n: i32) {
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
        code_abck(function_state, OP_LOADNIL, from, n - 1, 0, 0);
    }
}
pub unsafe fn code_get_jump(function_state: *mut FunctionState, program_counter: i32) -> i32 {
    unsafe {
        let offset: i32 = (*((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset(program_counter as isize)
            >> POSITION_A
            & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
            - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
        if offset == -1 {
            return -1;
        } else {
            return program_counter + 1 + offset;
        };
    }
}
pub unsafe fn fixjump(
    function_state: *mut FunctionState,
    program_counter: i32,
    dest: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        let offset: i32 = dest - (program_counter + 1);
        if !(-((1 << 8 + 8 + 1 + 8) - 1 >> 1) <= offset
            && offset <= (1 << 8 + 8 + 1 + 8) - 1 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1))
        {
            luax_syntaxerror(
                (*function_state).lexical_state,
                make_cstring!("control structure too long"),
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1 + 8) << POSITION_A)
            | ((offset + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32) << POSITION_A
                & !(!(0u32) << 8 + 8 + 1 + 8) << POSITION_A;
    }
}
pub unsafe fn luak_concat(function_state: *mut FunctionState, l1: *mut i32, l2: i32) {
    unsafe {
        if l2 == -1 {
            return;
        } else if *l1 == -1 {
            *l1 = l2;
        } else {
            let mut list: i32 = *l1;
            let mut next: i32;
            loop {
                next = code_get_jump(function_state, list);
                if !(next != -1) {
                    break;
                }
                list = next;
            }
            fixjump(function_state, list, l2);
        };
    }
}
pub unsafe fn luak_jump(function_state: *mut FunctionState) -> i32 {
    unsafe {
        return codesj(function_state, OP_JMP, -1, 0);
    }
}
pub unsafe fn luak_ret(function_state: *mut FunctionState, first: i32, nret: i32) {
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
        code_abck(function_state, op, first, nret + 1, 0, 0);
    }
}
pub unsafe fn condjump(
    function_state: *mut FunctionState,
    op: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        code_abck(function_state, op, a, b, c, k);
        return luak_jump(function_state);
    }
}
pub unsafe fn code_get_label(function_state: *mut FunctionState) -> i32 {
    unsafe {
        (*function_state).last_target = (*function_state).program_counter;
        return (*function_state).program_counter;
    }
}
pub unsafe fn code_get_jump_control(
    function_state: *mut FunctionState,
    program_counter: i32,
) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        if program_counter >= 1
            && OPMODES[(*pi.offset(-(1 as isize)) >> 0 & !(!(0u32) << 7) << 0) as usize] as i32
                & 1 << 4
                != 0
        {
            return pi.offset(-(1 as isize));
        } else {
            return pi;
        };
    }
}
pub unsafe fn patchtestreg(
    function_state: *mut FunctionState,
    node: i32,
    reg: i32,
) -> i32 {
    unsafe {
        let i: *mut u32 = code_get_jump_control(function_state, node);
        if (*i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
            return 0;
        }
        if reg != (1 << 8) - 1 && reg != (*i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 {
            *i = *i & !(!(!(0u32) << 8) << POSITION_A)
                | (reg as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
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
pub unsafe fn removevalues(function_state: *mut FunctionState, mut list: i32) {
    unsafe {
        while list != -1 {
            patchtestreg(function_state, list, (1 << 8) - 1);
            list = code_get_jump(function_state, list);
        }
    }
}
pub unsafe fn patchlistaux(
    function_state: *mut FunctionState,
    mut list: i32,
    vtarget: i32,
    reg: i32,
    dtarget: i32,
) {
    unsafe {
        while list != -1 {
            let next: i32 = code_get_jump(function_state, list);
            if patchtestreg(function_state, list, reg) != 0 {
                fixjump(function_state, list, vtarget);
            } else {
                fixjump(function_state, list, dtarget);
            }
            list = next;
        }
    }
}
pub unsafe fn luak_patchlist(
    function_state: *mut FunctionState,
    list: i32,
    target: i32,
) {
    unsafe {
        patchlistaux(function_state, list, target, (1 << 8) - 1, target);
    }
}
pub unsafe fn luak_patchtohere(function_state: *mut FunctionState, list: i32) {
    unsafe {
        let hr: i32 = code_get_label(function_state);
        luak_patchlist(function_state, list, hr);
    }
}
pub unsafe fn savelineinfo(
    function_state: *mut FunctionState,
    prototype: *mut Prototype,
    line: i32,
) {
    unsafe {
        let mut linedif: i32 = line - (*function_state).previous_line;
        let program_counter: i32 = (*function_state).program_counter - 1;
        if abs(linedif) >= 0x80 as i32 || {
            let fresh132 = (*function_state).iwthabs;
            (*function_state).iwthabs = ((*function_state).iwthabs).wrapping_add(1);
            fresh132 as i32 >= 128 as i32
        } {
            (*prototype).prototype_absolute_line_info.grow(
                (*(*function_state).lexical_state).interpreter,
                (*function_state).count_abslineinfo as usize,
                if 0x7FFFFFFF as usize
                    <= (!(0usize)).wrapping_div(size_of::<AbsoluteLineInfo>() as usize)
                {
                    0x7FFFFFFF
                } else {
                    (!(0usize)).wrapping_div(size_of::<AbsoluteLineInfo>() as usize)
                },
                make_cstring!("lines"),
            );
            (*((*prototype).prototype_absolute_line_info.vectort_pointer)
                .offset((*function_state).count_abslineinfo as isize))
            .program_counter = program_counter;
            let fresh133 = (*function_state).count_abslineinfo;
            (*function_state).count_abslineinfo = (*function_state).count_abslineinfo + 1;
            (*((*prototype).prototype_absolute_line_info.vectort_pointer)
                .offset(fresh133 as isize))
            .line = line;
            linedif = -(0x80 as i32);
            (*function_state).iwthabs = 1;
        }
        (*prototype).prototype_line_info.grow(
            (*(*function_state).lexical_state).interpreter,
            program_counter as usize,
            if 0x7FFFFFFF <= (!(0usize)).wrapping_div(size_of::<i8>()) {
                0x7FFFFFFF
            } else {
                (!(0usize)).wrapping_div(size_of::<i8>() as usize)
            },
            make_cstring!("opcodes"),
        );
        *((*prototype).prototype_line_info.vectort_pointer).offset(program_counter as isize) =
            linedif as i8;
        (*function_state).previous_line = line;
    }
}
pub unsafe fn removelastlineinfo(function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let program_counter: i32 = (*function_state).program_counter - 1;
        if *((*prototype).prototype_line_info.vectort_pointer).offset(program_counter as isize)
            as i32
            != -(0x80 as i32)
        {
            (*function_state).previous_line -= *((*prototype).prototype_line_info.vectort_pointer)
                .offset(program_counter as isize)
                as i32;
            (*function_state).iwthabs = ((*function_state).iwthabs).wrapping_sub(1);
            (*function_state).iwthabs;
        } else {
            (*function_state).count_abslineinfo -= 1;
            (*function_state).count_abslineinfo;
            (*function_state).iwthabs = (128 as i32 + 1) as u8;
        };
    }
}
pub unsafe fn removelastinstruction(function_state: *mut FunctionState) {
    unsafe {
        removelastlineinfo(function_state);
        (*function_state).program_counter -= 1;
        (*function_state).program_counter;
    }
}
pub unsafe fn luak_code(function_state: *mut FunctionState, i: u32) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        (*prototype).prototype_code.grow(
            (*(*function_state).lexical_state).interpreter,
            (*function_state).program_counter as usize,
            if 0x7FFFFFFF as usize <= (!(0usize)).wrapping_div(size_of::<u32>() as usize) {
                0x7FFFFFFF
            } else {
                (!(0usize)).wrapping_div(size_of::<u32>() as usize)
            },
            make_cstring!("opcodes"),
        );
        let fresh134 = (*function_state).program_counter;
        (*function_state).program_counter = (*function_state).program_counter + 1;
        *((*prototype).prototype_code.vectort_pointer).offset(fresh134 as isize) = i;
        savelineinfo(
            function_state,
            prototype,
            (*(*function_state).lexical_state).last_line,
        );
        return (*function_state).program_counter - 1;
    }
}
pub unsafe fn code_abck(
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
pub unsafe fn luak_codeabx(
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    bc: u32,
) -> i32 {
    unsafe {
        return luak_code(
            function_state,
            (o as u32) << 0 | (a as u32) << POSITION_A | bc << POSITION_K,
        );
    }
}
pub unsafe fn codeasbx(
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    bc: i32,
) -> i32 {
    unsafe {
        let b: u32 = (bc + ((1 << 8 + 8 + 1) - 1 >> 1)) as u32;
        return luak_code(
            function_state,
            (o as u32) << 0 | (a as u32) << POSITION_A | b << POSITION_K,
        );
    }
}
pub unsafe fn codesj(
    function_state: *mut FunctionState,
    o: u32,
    sj: i32,
    k: i32,
) -> i32 {
    unsafe {
        let j: u32 = (sj + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32;
        return luak_code(
            function_state,
            (o as u32) << 0 | j << POSITION_A | (k as u32) << POSITION_K,
        );
    }
}
pub unsafe fn codeextraarg(function_state: *mut FunctionState, a: i32) -> i32 {
    unsafe {
        return luak_code(
            function_state,
            (OP_EXTRAARG as u32) << 0 | (a as u32) << POSITION_A,
        );
    }
}
pub unsafe fn code_constant(function_state: *mut FunctionState, reg: i32, k: i32) -> i32 {
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
pub unsafe fn luak_checkstack(function_state: *mut FunctionState, n: i32) {
    unsafe {
        let newstack: i32 = (*function_state).freereg as i32 + n;
        if newstack > (*(*function_state).prototype).prototype_maximum_stack_size as i32 {
            if newstack >= 255 as i32 {
                luax_syntaxerror(
                    (*function_state).lexical_state,
                    make_cstring!("function or expression needs too many registers"),
                );
            }
            (*(*function_state).prototype).prototype_maximum_stack_size = newstack as u8;
        }
    }
}
pub unsafe fn luak_reserveregs(function_state: *mut FunctionState, n: i32) {
    unsafe {
        luak_checkstack(function_state, n);
        (*function_state).freereg = ((*function_state).freereg as i32 + n) as u8;
    }
}
pub unsafe fn freereg(function_state: *mut FunctionState, reg: i32) {
    unsafe {
        if reg >= luay_nvarstack(function_state) {
            (*function_state).freereg = ((*function_state).freereg).wrapping_sub(1);
            (*function_state).freereg;
        }
    }
}
pub unsafe fn freeregs(function_state: *mut FunctionState, r1: i32, r2: i32) {
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
pub unsafe fn freeexp(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expression_kind == ExpressionKind::Nonrelocatable {
            freereg(function_state, (*expression_description).value.info);
        }
    }
}
pub unsafe fn freeexps(
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32 = if (*e1).expression_kind == ExpressionKind::Nonrelocatable {
            (*e1).value.info
        } else {
            -1
        };
        let r2: i32 = if (*e2).expression_kind == ExpressionKind::Nonrelocatable {
            (*e2).value.info
        } else {
            -1
        };
        freeregs(function_state, r1, r2);
    }
}
pub unsafe fn addk(
    function_state: *mut FunctionState,
    key: *mut TValue,
    v: *mut TValue,
) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let interpreter: *mut Interpreter = (*(*function_state).lexical_state).interpreter;
        let prototype: *mut Prototype = (*function_state).prototype;
        let index: *const TValue = luah_get((*(*function_state).lexical_state).table, key);
        let mut count_constants: i32;
        if (*index).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            count_constants = (*index).value.integer as i32;
            if count_constants < (*function_state).count_constants
                && (*((*prototype).prototype_constants.vectort_pointer)
                    .offset(count_constants as isize))
                .get_tag_variant()
                    == (*v).get_tag_variant()
                && luav_equalobj(
                    null_mut(),
                    &mut *((*prototype).prototype_constants.vectort_pointer)
                        .offset(count_constants as isize),
                    v,
                )
            {
                return count_constants;
            }
        }
        let mut old_size = (*prototype).prototype_constants.get_size();
        count_constants = (*function_state).count_constants;
        let io: *mut TValue = &mut value;
        (*io).value.integer = count_constants as i64;
        (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        luah_finishset(
            interpreter,
            (*(*function_state).lexical_state).table,
            key,
            index,
            &mut value,
        );
        (*prototype).prototype_constants.grow(
            interpreter,
            count_constants as usize,
            if ((1 << 8 + 8 + 1 + 8) - 1) as usize
                <= (!(0usize)).wrapping_div(size_of::<TValue>() as usize)
            {
                (1 << 8 + 8 + 1 + 8) - 1
            } else {
                (!(0usize)).wrapping_div(size_of::<TValue>() as usize)
            },
            make_cstring!("constants"),
        );
        while old_size < (*prototype).prototype_constants.get_size() {
            let fresh135 = old_size;
            old_size = old_size + 1;
            (*((*prototype).prototype_constants.vectort_pointer).offset(fresh135 as isize))
                .set_tag_variant(TagVariant::NilNil as u8);
        }
        let io1: *mut TValue = &mut *((*prototype).prototype_constants.vectort_pointer)
            .offset(count_constants as isize) as *mut TValue;
        let io2: *const TValue = v;
        (*io1).copy_from(&*io2);
        (*function_state).count_constants += 1;
        (*function_state).count_constants;
        if (*v).is_collectable() {
            if (*prototype).get_marked() & 1 << 5 != 0
                && (*(*v).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    interpreter,
                    &mut (*(prototype as *mut Object)),
                    &mut (*((*v).value.object as *mut Object)),
                );
            } else {
            };
        } else {
        };
        return count_constants;
    }
}
pub unsafe fn string_constant(function_state: *mut FunctionState, s: *mut TString) -> i32 {
    unsafe {
        let mut o: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let io: *mut TValue = &mut o;
        (*io).value.object = &mut (*(s as *mut Object));
        (*io).set_tag_variant((*s).get_tag_variant());
        (*io).set_collectable(true);
        return addk(function_state, &mut o, &mut o);
    }
}
pub unsafe fn luak_int_k(function_state: *mut FunctionState, integer: i64) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        tvalue.value.integer = integer;
        tvalue.set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        return addk(function_state, &mut tvalue, &mut tvalue);
    }
}
pub unsafe fn luak_number_k(function_state: *mut FunctionState, number: f64) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let mut ik: i64 = 0;
        tvalue.value.number = number;
        tvalue.set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
        if !luav_flttointeger(number, &mut ik, F2I::Equal) {
            return addk(function_state, &mut tvalue, &mut tvalue);
        } else {
            let nbm: i32 = 53 as i32;
            let q: f64 = ldexp_(1.0f64, -nbm + 1);
            let k: f64 = if ik == 0 { q } else { number + number * q };
            let mut kv: TValue = TValue::new(TAG_VARIANT_NUMERIC_NUMBER);
            kv.value.number = k;
            return addk(function_state, &mut kv, &mut tvalue);
        };
    }
}
pub unsafe fn bool_false(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TAG_VARIANT_BOOLEAN_FALSE);
        return addk(function_state, &mut tvalue, &mut tvalue);
    }
}
pub unsafe fn bool_true(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TAG_VARIANT_BOOLEAN_TRUE);
        return addk(function_state, &mut value, &mut value);
    }
}
pub unsafe fn nil_k(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut key: TValue = TValue::new(TAG_VARIANT_TABLE);
        let mut value: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let table: *mut Table = (*(*function_state).lexical_state).table;
        key.value.object = &mut (*(table as *mut Object));
        key.set_collectable(true);
        return addk(function_state, &mut key, &mut value);
    }
}
pub unsafe fn code_constant_integer(function_state: *mut FunctionState, reg: i32, i: i64) {
    unsafe {
        if fits_bx(i) {
            codeasbx(function_state, OP_LOADI, reg, i as i32);
        } else {
            code_constant(function_state, reg, luak_int_k(function_state, i));
        };
    }
}
pub unsafe fn code_constant_number(function_state: *mut FunctionState, reg: i32, number: f64) {
    unsafe {
        let mut fi: i64 = 0;
        if luav_flttointeger(number, &mut fi, F2I::Equal) && fits_bx(fi) {
            codeasbx(function_state, OP_LOADF, reg, fi as i32);
        } else {
            code_constant(function_state, reg, luak_number_k(function_state, number));
        };
    }
}
pub unsafe fn luak_setreturns(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    count_results: i32,
) {
    unsafe {
        let program_counter: *mut u32 = &mut *((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset((*expression_description).value.info as isize) as *mut u32;
        if (*expression_description).expression_kind == ExpressionKind::Call {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
        } else {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
            *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_A)
                | ((*function_state).freereg as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
            luak_reserveregs(function_state, 1);
        };
    }
}
pub unsafe fn string_to_constant(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        (*expression_description).value.info = string_constant(function_state, (*expression_description).value.tstring);
        (*expression_description).expression_kind = ExpressionKind::Constant;
    }
}
pub unsafe fn luak_setoneret(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expression_kind == ExpressionKind::Call {
            (*expression_description).expression_kind = ExpressionKind::Nonrelocatable;
            (*expression_description).value.info = (*((*(*function_state).prototype)
                .prototype_code
                .vectort_pointer)
                .offset((*expression_description).value.info as isize)
                >> POSITION_A
                & !(!(0u32) << 8) << 0) as i32;
        } else if (*expression_description).expression_kind == ExpressionKind::VariableArguments {
            *((*(*function_state).prototype)
                .prototype_code
                .vectort_pointer)
                .offset((*expression_description).value.info as isize) = *((*(*function_state).prototype)
                .prototype_code
                .vectort_pointer)
                .offset((*expression_description).value.info as isize)
                & !(!(!(0u32) << 8) << POSITION_C)
                | (2 as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
            (*expression_description).expression_kind = ExpressionKind::Relocatable;
        }
    }
}
pub unsafe fn luak_dischargevars(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expression_kind {
            ExpressionKind::Constant2 => {
                const2exp(const2val(function_state, expression_description), expression_description);
            }
            ExpressionKind::Local => {
                let temporary = (*expression_description).value.variable.register_index as i32;
                (*expression_description).value.info = temporary;
                (*expression_description).expression_kind = ExpressionKind::Nonrelocatable;
            }
            ExpressionKind::UpValue => {
                (*expression_description).value.info =
                    code_abck(function_state, OP_GETUPVAL, 0, (*expression_description).value.info, 0, 0);
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            ExpressionKind::IndexUpValue => {
                (*expression_description).value.info = code_abck(
                    function_state,
                    OP_GETTABUP,
                    0,
                    (*expression_description).value.index.reference_tag as i32,
                    (*expression_description).value.index.reference_index as i32,
                    0,
                );
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            ExpressionKind::IndexInteger => {
                freereg(function_state, (*expression_description).value.index.reference_tag as i32);
                (*expression_description).value.info = code_abck(
                    function_state,
                    OP_GETI,
                    0,
                    (*expression_description).value.index.reference_tag as i32,
                    (*expression_description).value.index.reference_index as i32,
                    0,
                );
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            ExpressionKind::Field => {
                freereg(function_state, (*expression_description).value.index.reference_tag as i32);
                (*expression_description).value.info = code_abck(
                    function_state,
                    OP_GETFIELD,
                    0,
                    (*expression_description).value.index.reference_tag as i32,
                    (*expression_description).value.index.reference_index as i32,
                    0,
                );
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            ExpressionKind::Indexed => {
                freeregs(
                    function_state,
                    (*expression_description).value.index.reference_tag as i32,
                    (*expression_description).value.index.reference_index as i32,
                );
                (*expression_description).value.info = code_abck(
                    function_state,
                    OP_GETTABLE,
                    0,
                    (*expression_description).value.index.reference_tag as i32,
                    (*expression_description).value.index.reference_index as i32,
                    0,
                );
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            ExpressionKind::Call => {
                luak_setoneret(function_state, expression_description);
            }
            ExpressionKind::VariableArguments => {
                luak_setoneret(function_state, expression_description);
            }
            _ => {}
        };
    }
}
pub unsafe fn discharge2reg(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    register: i32,
) {
    unsafe {
        luak_dischargevars(function_state, expression_description);
        match (*expression_description).expression_kind {
            ExpressionKind::Nil => {
                code_constant_nil(function_state, register, 1);
            }
            ExpressionKind::False => {
                code_abck(function_state, OP_LOADFALSE, register, 0, 0, 0);
            }
            ExpressionKind::True => {
                code_abck(function_state, OP_LOADTRUE, register, 0, 0, 0);
            }
            ExpressionKind::ConstantString => {
                string_to_constant(function_state, expression_description);
                code_constant(function_state, register, (*expression_description).value.info);
            }
            ExpressionKind::Constant => {
                code_constant(function_state, register, (*expression_description).value.info);
            }
            ExpressionKind::ConstantNumber => {
                code_constant_number(function_state, register, (*expression_description).value.number);
            }
            ExpressionKind::ConstantInteger => {
                code_constant_integer(function_state, register, (*expression_description).value.integer);
            }
            ExpressionKind::Relocatable => {
                let program_counter = &mut *((*(*function_state).prototype)
                    .prototype_code
                    .vectort_pointer)
                    .offset((*expression_description).value.info as isize);
                *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_A)
                    | (register as u32) << POSITION_A & !(!(0u32) << 8) << POSITION_A;
            }
            ExpressionKind::Nonrelocatable => {
                if register != (*expression_description).value.info {
                    code_abck(function_state, OP_MOVE, register, (*expression_description).value.info, 0, 0);
                }
            }
            _ => return,
        }
        (*expression_description).value.info = register;
        (*expression_description).expression_kind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn discharge2anyreg(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expression_kind != ExpressionKind::Nonrelocatable {
            luak_reserveregs(function_state, 1);
            discharge2reg(function_state, expression_description, (*function_state).freereg as i32 - 1);
        }
    }
}
pub unsafe fn code_loadbool(function_state: *mut FunctionState, a: i32, op: u32) -> i32 {
    unsafe {
        code_get_label(function_state);
        return code_abck(function_state, op, a, 0, 0, 0);
    }
}
pub unsafe fn need_value(function_state: *mut FunctionState, mut list: i32) -> i32 {
    unsafe {
        while list != -1 {
            let i: u32 = *code_get_jump_control(function_state, list);
            if (i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
                return 1;
            }
            list = code_get_jump(function_state, list);
        }
        return 0;
    }
}
pub unsafe fn exp2reg(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    reg: i32,
) {
    unsafe {
        discharge2reg(function_state, expression_description, reg);
        if (*expression_description).expression_kind == ExpressionKind::Jump {
            luak_concat(function_state, &mut (*expression_description).t, (*expression_description).value.info);
        }
        if (*expression_description).t != (*expression_description).f {
            let mut p_f: i32 = -1;
            let mut p_t: i32 = -1;
            if need_value(function_state, (*expression_description).t) != 0 || need_value(function_state, (*expression_description).f) != 0 {
                let fj: i32 = if (*expression_description).expression_kind == ExpressionKind::Jump {
                    -1
                } else {
                    luak_jump(function_state)
                };
                p_f = code_loadbool(function_state, reg, OP_LFALSESKIP);
                p_t = code_loadbool(function_state, reg, OP_LOADTRUE);
                luak_patchtohere(function_state, fj);
            }
            let final_0: i32 = code_get_label(function_state);
            patchlistaux(function_state, (*expression_description).f, final_0, reg, p_f);
            patchlistaux(function_state, (*expression_description).t, final_0, reg, p_t);
        }
        (*expression_description).t = -1;
        (*expression_description).f = (*expression_description).t;
        (*expression_description).value.info = reg;
        (*expression_description).expression_kind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn luak_exp2nextreg(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(function_state, expression_description);
        freeexp(function_state, expression_description);
        luak_reserveregs(function_state, 1);
        exp2reg(function_state, expression_description, (*function_state).freereg as i32 - 1);
    }
}
pub unsafe fn luak_exp2anyreg(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        luak_dischargevars(function_state, expression_description);
        if (*expression_description).expression_kind == ExpressionKind::Nonrelocatable {
            if !((*expression_description).t != (*expression_description).f) {
                return (*expression_description).value.info;
            }
            if (*expression_description).value.info >= luay_nvarstack(function_state) {
                exp2reg(function_state, expression_description, (*expression_description).value.info);
                return (*expression_description).value.info;
            }
        }
        luak_exp2nextreg(function_state, expression_description);
        return (*expression_description).value.info;
    }
}
pub unsafe fn luak_exp2anyregup(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expression_kind != ExpressionKind::UpValue || (*expression_description).t != (*expression_description).f {
            luak_exp2anyreg(function_state, expression_description);
        }
    }
}
pub unsafe fn code_expression_to_value(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expression_kind == ExpressionKind::Jump || (*expression_description).t != (*expression_description).f {
            luak_exp2anyreg(function_state, expression_description);
        } else {
            luak_dischargevars(function_state, expression_description);
        };
    }
}
pub unsafe fn code_expression_to_constant(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if (*expression_description).t == (*expression_description).f {
            let info: i32;
            match (*expression_description).expression_kind {
                ExpressionKind::True => {
                    info = bool_true(function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::False => {
                    info = bool_false(function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::Nil => {
                    info = nil_k(function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::ConstantInteger => {
                    info = luak_int_k(function_state, (*expression_description).value.integer);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::ConstantNumber => {
                    info = luak_number_k(function_state, (*expression_description).value.number);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::ConstantString => {
                    info = string_constant(function_state, (*expression_description).value.tstring);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                ExpressionKind::Constant => {
                    info = (*expression_description).value.info;
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expression_kind = ExpressionKind::Constant;
                        (*expression_description).value.info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                }
                _ => {
                    return 0;
                }
            }
        } else {
            return 0;
        }
    }
}
pub unsafe fn exp2rk(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if code_expression_to_constant(function_state, expression_description) != 0 {
            return 1;
        } else {
            luak_exp2anyreg(function_state, expression_description);
            return 0;
        };
    }
}
pub unsafe fn codeabrk(
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    ec: *mut ExpressionDescription,
) {
    unsafe {
        let k: i32 = exp2rk(function_state, ec);
        code_abck(function_state, o, a, b, (*ec).value.info, k);
    }
}
pub unsafe fn luak_storevar(
    function_state: *mut FunctionState,
    var: *mut ExpressionDescription,
    ex: *mut ExpressionDescription,
) {
    unsafe {
        match (*var).expression_kind {
            ExpressionKind::Local => {
                freeexp(function_state, ex);
                exp2reg(
                    function_state,
                    ex,
                    (*var).value.variable.register_index as i32,
                );
                return;
            }
            ExpressionKind::UpValue => {
                let e: i32 = luak_exp2anyreg(function_state, ex);
                code_abck(function_state, OP_SETUPVAL, e, (*var).value.info, 0, 0);
            }
            ExpressionKind::IndexUpValue => {
                codeabrk(
                    function_state,
                    OP_SETTABUP,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::IndexInteger => {
                codeabrk(
                    function_state,
                    OP_SETI,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::Field => {
                codeabrk(
                    function_state,
                    OP_SETFIELD,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            ExpressionKind::Indexed => {
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
pub unsafe fn luak_self(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    key: *mut ExpressionDescription,
) {
    unsafe {
        luak_exp2anyreg(function_state, expression_description);
        let ereg: i32 = (*expression_description).value.info;
        freeexp(function_state, expression_description);
        (*expression_description).value.info = (*function_state).freereg as i32;
        (*expression_description).expression_kind = ExpressionKind::Nonrelocatable;
        luak_reserveregs(function_state, 2);
        codeabrk(function_state, OP_SELF, (*expression_description).value.info, ereg, key);
        freeexp(function_state, key);
    }
}
pub unsafe fn negatecondition(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: *mut u32 = code_get_jump_control(function_state, (*expression_description).value.info);
        *program_counter = *program_counter & !(!(!(0u32) << 1) << POSITION_K)
            | (((*program_counter >> POSITION_K & !(!(0u32) << 1) << 0) as i32 ^ 1) as u32)
                << POSITION_K
                & !(!(0u32) << 1) << POSITION_K;
    }
}
pub unsafe fn jumponcond(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    cond_0: i32,
) -> i32 {
    unsafe {
        if (*expression_description).expression_kind == ExpressionKind::Relocatable {
            let ie: u32 = *((*(*function_state).prototype)
                .prototype_code
                .vectort_pointer)
                .offset((*expression_description).value.info as isize);
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
        discharge2anyreg(function_state, expression_description);
        freeexp(function_state, expression_description);
        return condjump(
            function_state,
            OP_TESTSET,
            (1 << 8) - 1,
            (*expression_description).value.info,
            0,
            cond_0,
        );
    }
}
pub unsafe fn luak_goiftrue(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(function_state, expression_description);
        match (*expression_description).expression_kind {
            ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
                program_counter = (*expression_description).value.info;
            }
            ExpressionKind::True | ExpressionKind::Constant | ExpressionKind::ConstantNumber | ExpressionKind::ConstantInteger | ExpressionKind::ConstantString => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(function_state, expression_description, 0);
            }
        }
        luak_concat(function_state, &mut (*expression_description).f, program_counter);
        luak_patchtohere(function_state, (*expression_description).t);
        (*expression_description).t = -1;
    }
}
pub unsafe fn luak_goiffalse(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(function_state, expression_description);
        match (*expression_description).expression_kind {
            ExpressionKind::Jump => {
                program_counter = (*expression_description).value.info;
            }
            ExpressionKind::Nil | ExpressionKind::False => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(function_state, expression_description, 1);
            }
        }
        luak_concat(function_state, &mut (*expression_description).t, program_counter);
        luak_patchtohere(function_state, (*expression_description).f);
        (*expression_description).f = -1;
    }
}
pub unsafe fn codenot(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expression_kind {
            ExpressionKind::Nil | ExpressionKind::False => {
                (*expression_description).expression_kind = ExpressionKind::True;
            }
            ExpressionKind::Constant | ExpressionKind::ConstantNumber | ExpressionKind::ConstantInteger | ExpressionKind::ConstantString | ExpressionKind::True => {
                (*expression_description).expression_kind = ExpressionKind::False;
            }
            ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
            }
            ExpressionKind::Relocatable | ExpressionKind::Nonrelocatable => {
                discharge2anyreg(function_state, expression_description);
                freeexp(function_state, expression_description);
                (*expression_description).value.info = code_abck(function_state, OP_NOT, 0, (*expression_description).value.info, 0, 0);
                (*expression_description).expression_kind = ExpressionKind::Relocatable;
            }
            _ => {}
        }
        let temp: i32 = (*expression_description).f;
        (*expression_description).f = (*expression_description).t;
        (*expression_description).t = temp;
        removevalues(function_state, (*expression_description).f);
        removevalues(function_state, (*expression_description).t);
    }
}
pub unsafe fn is_k_string(
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> bool {
    unsafe {
        return (*expression_description).expression_kind == ExpressionKind::Constant
            && !((*expression_description).t != (*expression_description).f)
            && (*expression_description).value.info <= ((1 << 8) - 1)
            && (*((*(*function_state).prototype)
                .prototype_constants
                .vectort_pointer)
                .offset((*expression_description).value.info as isize))
            .get_tag_variant()
                == TAG_VARIANT_STRING_SHORT;
    }
}
pub unsafe fn constfolding(
    function_state: *mut FunctionState,
    op: i32,
    e1: *mut ExpressionDescription,
    e2: *const ExpressionDescription,
) -> i32 {
    unsafe {
        let mut v1: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let mut v2: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let mut res: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        if !tonumeral(e1, &mut v1) || !tonumeral(e2, &mut v2) || validop(op, &mut v1, &mut v2) == 0
        {
            return 0;
        }
        luao_rawarith(
            (*(*function_state).lexical_state).interpreter,
            op,
            &mut v1,
            &mut v2,
            &mut res,
        );
        if res.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            (*e1).expression_kind = ExpressionKind::ConstantInteger;
            (*e1).value.integer = res.value.integer;
        } else {
            let n: f64 = res.value.number;
            if !(n == n) || n == 0.0 {
                return 0;
            }
            (*e1).expression_kind = ExpressionKind::ConstantNumber;
            (*e1).value.number = n;
        }
        return 1;
    }
}
pub unsafe fn code_unary_expression_value(
    function_state: *mut FunctionState,
    op: u32,
    expression_description: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let register = luak_exp2anyreg(function_state, expression_description);
        freeexp(function_state, expression_description);
        (*expression_description).value.info = code_abck(function_state, op, 0, register, 0, 0);
        (*expression_description).expression_kind = ExpressionKind::Relocatable;
        luak_fixline(function_state, line);
    }
}
pub unsafe fn finishbinexpval(
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
        let program_counter: i32 = code_abck(function_state, op, 0, v1, v2, 0);
        freeexps(function_state, e1, e2);
        (*e1).value.info = program_counter;
        (*e1).expression_kind = ExpressionKind::Relocatable;
        luak_fixline(function_state, line);
        code_abck(function_state, mmop, v1, v2, event as i32, flip);
        luak_fixline(function_state, line);
    }
}
pub unsafe fn codebinexpval(
    function_state: *mut FunctionState,
    binary: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let op = binopr2op(binary, OperatorBinary::Add, OP_ADD);
        let v2: i32 = luak_exp2anyreg(function_state, e2);
        finishbinexpval(
            function_state,
            e1,
            e2,
            op,
            v2,
            0,
            line,
            OP_MMBIN,
            binopr2tm(binary),
        );
    }
}
pub unsafe fn codebini(
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
pub unsafe fn codebink(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        let event: u32 = binopr2tm(opr);
        let v2: i32 = (*e2).value.info;
        let op: u32 = binopr2op(opr, OperatorBinary::Add, OP_ADDK);
        finishbinexpval(function_state, e1, e2, op, v2, flip, line, OP_MMBINK, event);
    }
}
pub unsafe fn finishbinexpneg(
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
                *((*(*function_state).prototype)
                    .prototype_code
                    .vectort_pointer)
                    .offset(((*function_state).program_counter - 1) as isize) =
                    *((*(*function_state).prototype)
                        .prototype_code
                        .vectort_pointer)
                        .offset(((*function_state).program_counter - 1) as isize)
                        & !(!(!(0u32) << 8) << POSITION_B)
                        | ((v2 + ((1 << 8) - 1 >> 1)) as u32) << POSITION_B
                            & !(!(0u32) << 8) << POSITION_B;
                return 1;
            }
        };
    }
}
pub unsafe fn codebinnok(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
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
pub unsafe fn codearith(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if tonumeral(e2, null_mut()) && code_expression_to_constant(function_state, e2) != 0 {
            codebink(function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe fn codebitwise(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if (*e1).expression_kind == ExpressionKind::ConstantInteger {
            swapexps(e1, e2);
            flip = 1;
        }
        if (*e2).expression_kind == ExpressionKind::ConstantInteger && code_expression_to_constant(function_state, e2) != 0 {
            codebink(function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe fn codeorder(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32;
        let r2: i32;
        let mut im: i32 = 0;
        let mut is_float = false;
        let op: u32;
        if is_sc_number(e2, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(function_state, e1);
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OP_LTI);
        } else if is_sc_number(e1, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(function_state, e2);
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OP_GTI);
        } else {
            r1 = luak_exp2anyreg(function_state, e1);
            r2 = luak_exp2anyreg(function_state, e2);
            op = binopr2op(opr, OperatorBinary::Less, OP_LT);
        }
        freeexps(function_state, e1, e2);
        (*e1).value.info = condjump(function_state, op, r1, r2, is_float as i32, 1);
        (*e1).expression_kind = ExpressionKind::Jump;
    }
}
pub unsafe fn codeeq(
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32;
        let r2: i32;
        let mut im: i32 = 0;
        let mut is_float = false;
        let op: u32;
        if (*e1).expression_kind != ExpressionKind::Nonrelocatable {
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
            (opr as u32 == OperatorBinary::Equal as u32) as i32,
        );
        (*e1).expression_kind = ExpressionKind::Jump;
    }
}
pub unsafe fn luak_prefix(
    function_state: *mut FunctionState,
    unary: OperatorUnary,
    expression_description: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        pub const EF: ExpressionDescription = ExpressionDescription::new_from_integer(0);
        luak_dischargevars(function_state, expression_description);
        match unary {
            OperatorUnary::BitwiseNot => {
                if constfolding(
                    function_state,
                    (unary as u32).wrapping_add(12 as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0 {
                    code_unary_expression_value(function_state, unopr2op(unary), expression_description, line);
                }
            },
            OperatorUnary::Minus => {
                if constfolding(
                    function_state,
                    (unary as u32).wrapping_add(12 as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0 {
                    code_unary_expression_value(function_state, unopr2op(unary), expression_description, line);
                }
            },
            OperatorUnary::Length => {
                code_unary_expression_value(function_state, unopr2op(unary), expression_description, line);
            },
            OperatorUnary::Not => {
                codenot(function_state, expression_description);
            },
            _ => {},
        }
    }
}
pub unsafe fn luak_infix(
    function_state: *mut FunctionState,
    op: OperatorBinary,
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
            OP_MOVE | OP_LOADI | OP_LOADF | OP_LOADFALSE | OP_LFALSESKIP | OP_LOADK | OP_LOADKX
            | OP_LOADTRUE | OP_LOADNIL | OP_GETUPVAL | OP_SETUPVAL | OP_GETTABUP => {
                if !tonumeral(v, null_mut()) {
                    luak_exp2anyreg(function_state, v);
                }
            }
            OP_GETI | OP_SETTABLE => {
                if !tonumeral(v, null_mut()) {
                    exp2rk(function_state, v);
                }
            }
            OP_GETFIELD | OP_SETTABUP | OP_SETI | OP_SETFIELD => {
                let mut dummy: i32 = 0;
                let mut dummy2 = false;
                if is_sc_number(v, &mut dummy, &mut dummy2) == 0 {
                    luak_exp2anyreg(function_state, v);
                }
            }
            _ => {}
        };
    }
}
pub unsafe fn codeconcat(
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
            code_abck(function_state, OP_CONCAT, (*e1).value.info, 2, 0, 0);
            freeexp(function_state, e2);
            luak_fixline(function_state, line);
        };
    }
}
pub unsafe fn luak_posfix(
    function_state: *mut FunctionState,
    mut binary: OperatorBinary,
    expression_description_a: *mut ExpressionDescription,
    expression_description_b: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        luak_dischargevars(function_state, expression_description_b);
        if binary as u32 <= OperatorBinary::ShiftRight as u32
            && constfolding(
                function_state,
                (binary as u32).wrapping_add(0u32) as i32,
                expression_description_a,
                expression_description_b,
            ) != 0
        {
            return;
        }
        match binary {
            OperatorBinary::And => {
                luak_concat(function_state, &mut (*expression_description_b).f, (*expression_description_a).f);
                *expression_description_a = *expression_description_b;
            }
            OperatorBinary::Or => {
                luak_concat(function_state, &mut (*expression_description_b).t, (*expression_description_a).t);
                *expression_description_a = *expression_description_b;
            }
            OperatorBinary::Concatenate => {
                luak_exp2nextreg(function_state, expression_description_b);
                codeconcat(function_state, expression_description_a, expression_description_b, line);
            }
            OperatorBinary::Add => {
                let mut flip: i32 = 0;
                if tonumeral(expression_description_a, null_mut()) {
                    swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                if is_sc_int(expression_description_b) {
                    codebini(function_state, OP_ADDI, expression_description_a, expression_description_b, flip, line, TM_ADD);
                } else {
                    codearith(function_state, binary, expression_description_a, expression_description_b, flip, line);
                };
            }
            OperatorBinary::Multiply => {
                let mut flip: i32 = 0;
                if tonumeral(expression_description_a, null_mut()) {
                    swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                codearith(function_state, binary, expression_description_a, expression_description_b, flip, line);
            }
            OperatorBinary::Subtract => {
                if finishbinexpneg(function_state, expression_description_a, expression_description_b, OP_ADDI, line, TM_SUB) == 0 {
                    codearith(function_state, binary, expression_description_a, expression_description_b, 0, line);
                }
            }
            OperatorBinary::Power  => {
                codearith(function_state, binary, expression_description_a, expression_description_b, 0, line);
            }
            OperatorBinary::Modulus => {
                codearith(function_state, binary, expression_description_a, expression_description_b, 0, line);
            }
            OperatorBinary::Divide => {
                codearith(function_state, binary, expression_description_a, expression_description_b, 0, line);
            }
            OperatorBinary::IntegralDivide => {
                codearith(function_state, binary, expression_description_a, expression_description_b, 0, line);
            }
            OperatorBinary::BitwiseAnd => {
                codebitwise(function_state, binary, expression_description_a, expression_description_b, line);
            }
            OperatorBinary::BitwiseOr => {
                codebitwise(function_state, binary, expression_description_a, expression_description_b, line);
            }
            OperatorBinary::BitwiseExclusiveOr => {
                codebitwise(function_state, binary, expression_description_a, expression_description_b, line);
            }
            OperatorBinary::ShiftLeft => {
                if is_sc_int(expression_description_a) {
                    swapexps(expression_description_a, expression_description_b);
                    codebini(function_state, OP_SHLI, expression_description_a, expression_description_b, 1, line, TM_SHL);
                } else if !(finishbinexpneg(function_state, expression_description_a, expression_description_b, OP_SHRI, line, TM_SHL) != 0) {
                    codebinexpval(function_state, binary, expression_description_a, expression_description_b, line);
                }
            }
            OperatorBinary::ShiftRight => {
                if is_sc_int(expression_description_b) {
                    codebini(function_state, OP_SHRI, expression_description_a, expression_description_b, 0, line, TM_SHR);
                } else {
                    codebinexpval(function_state, binary, expression_description_a, expression_description_b, line);
                }
            }
            OperatorBinary::Inequal => {
                codeeq(function_state, binary, expression_description_a, expression_description_b);
            }
            OperatorBinary::Equal => {
                codeeq(function_state, binary, expression_description_a, expression_description_b);
            }
            OperatorBinary::GreaterEqual => {
                swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::LessEqual;
                codeorder(function_state, binary, expression_description_a, expression_description_b);
            }
            OperatorBinary::Greater => {
                swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::Less;
                codeorder(function_state, binary, expression_description_a, expression_description_b);
            }
            OperatorBinary::Less => {
                codeorder(function_state, binary, expression_description_a, expression_description_b);
            },
            OperatorBinary::LessEqual => {
                codeorder(function_state, binary, expression_description_a, expression_description_b);
            },
            _ => {},
        }
    }
}
pub unsafe fn luak_fixline(function_state: *mut FunctionState, line: i32) {
    unsafe {
        removelastlineinfo(function_state);
        savelineinfo(function_state, (*function_state).prototype, line);
    }
}
pub const POSITION_A: usize = 7;
pub const POSITION_K: usize = POSITION_A + 8;
pub const POSITION_B: usize = POSITION_K + 1;
pub const POSITION_C: usize = POSITION_B + 8;
pub unsafe fn luak_settablesize(
    function_state: *mut FunctionState,
    program_counter: i32,
    ra: i32,
    asize: i32,
    hsize: i32,
) {
    unsafe {
        let inst: *mut u32 = &mut *((*(*function_state).prototype)
            .prototype_code
            .vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
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
pub unsafe fn luak_setlist(
    function_state: *mut FunctionState,
    base: i32,
    mut count_elements: usize,
    mut tostore: i32,
) {
    unsafe {
        if tostore == -1 {
            tostore = 0;
        }
        if count_elements <= (1 << 8) - 1 {
            code_abck(
                function_state,
                OP_SETLIST,
                base,
                tostore,
                count_elements as i32,
                0,
            );
        } else {
            let extra = count_elements / ((1 << 8) - 1 + 1);
            count_elements %= (1 << 8) - 1 + 1;
            code_abck(
                function_state,
                OP_SETLIST,
                base,
                tostore,
                count_elements as i32,
                1,
            );
            codeextraarg(function_state, extra as i32);
        }
        (*function_state).freereg = (base + 1) as u8;
    }
}
pub unsafe fn luak_finish(function_state: *mut FunctionState) {
    unsafe {
        let p: *mut Prototype = (*function_state).prototype;
        for i in 0..(*function_state).program_counter {
            let program_counter: *mut u32 =
                &mut *((*p).prototype_code.vectort_pointer).offset(i as isize) as *mut u32;
            let current_block_7: usize;
            match (*program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 {
                OP_RETURN0 | OP_RETURN1 => {
                    if !((*function_state).needs_close
                        || (*p).prototype_is_variable_arguments as i32 != 0)
                    {
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
                    let target: i32 = final_target((*p).prototype_code.vectort_pointer, i);
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
                    if (*p).prototype_is_variable_arguments {
                        *program_counter = *program_counter & !(!(!(0u32) << 8) << POSITION_C)
                            | (((*p).prototype_count_parameters as i32 + 1) as u32) << POSITION_C
                                & !(!(0u32) << 8) << POSITION_C;
                    }
                }
                _ => {}
            }
        }
    }
}

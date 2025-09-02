use crate::lexical::blockcontrol::*;
use crate::lexical::lexicalstate::*;
use crate::operator_::*;
use crate::labeldescription::*;
use crate::vm::instruction::*;
use crate::lexical::constructorcontrol::*;
use crate::v::*;
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
pub unsafe extern "C" fn movegotosout(fs: *mut FunctionState, block_control: *mut BlockControl) {
    unsafe {
        let mut i: i32;
        let gl: *mut LabelList = &mut (*(*(*fs).lexical_state).dynamic_data).gt;
        i = (*block_control).first_goto;
        while i < (*gl).n {
            let gt: *mut LabelDescription =
                &mut *((*gl).pointer).offset(i as isize) as *mut LabelDescription;
            if reglevel(fs, (*gt).count_active_variables as i32)
                > reglevel(fs, (*block_control).count_active_variables as i32)
            {
                (*gt).close = ((*gt).close as i32 | (*block_control).count_upvalues as i32) as u8;
            }
            (*gt).count_active_variables = (*block_control).count_active_variables;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn enterblock(
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
    is_loop: bool,
) {
    unsafe {
        (*block_control).is_loop = is_loop;
        (*block_control).count_active_variables = (*fs).count_active_variables;
        (*block_control).first_label = (*(*(*fs).lexical_state).dynamic_data).label.n;
        (*block_control).first_goto = (*(*(*fs).lexical_state).dynamic_data).gt.n;
        (*block_control).count_upvalues = 0;
        (*block_control).is_inside_tbc =
            !((*fs).block_control).is_null() && (*(*fs).block_control).is_inside_tbc as i32 != 0;
        (*block_control).previous = (*fs).block_control;
        (*fs).block_control = block_control;
    }
}
pub unsafe extern "C" fn leaveblock(fs: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*fs).block_control;
        let lexical_state: *mut LexicalState = (*fs).lexical_state;
        let mut has_close = false;
        let stklevel: i32 = reglevel(fs, (*block_control).count_active_variables as i32);
        removevars(fs, (*block_control).count_active_variables as i32);
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
            luak_code_abck(fs, OP_CLOSE, stklevel, 0, 0, 0);
        }
        (*fs).freereg = stklevel as u8;
        (*(*lexical_state).dynamic_data).label.n = (*block_control).first_label;
        (*fs).block_control = (*block_control).previous;
        if !((*block_control).previous).is_null() {
            movegotosout(fs, block_control);
        } else if (*block_control).first_goto < (*(*lexical_state).dynamic_data).gt.n {
            undefgoto(
                lexical_state,
                &mut *((*(*lexical_state).dynamic_data).gt.pointer)
                    .offset((*block_control).first_goto as isize),
            );
        }
    }
}
pub unsafe extern "C" fn closelistfield(fs: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).expression_description.kind == V::VVOID {
            return;
        }
        luak_exp2nextreg(fs, &mut (*cc).expression_description);
        (*cc).expression_description.kind = V::VVOID;
        if (*cc).to_store == 50 as i32 {
            luak_setlist(fs, (*(*cc).t).value.info, (*cc).na, (*cc).to_store);
            (*cc).na += (*cc).to_store;
            (*cc).to_store = 0;
        }
    }
}
pub unsafe extern "C" fn lastlistfield(fs: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).to_store == 0 {
            return;
        }
        if (*cc).expression_description.kind as u32 == V::VCALL as u32 || (*cc).expression_description.kind as u32 == V::VVARARG as u32 {
            luak_setreturns(fs, &mut (*cc).expression_description, -1);
            luak_setlist(fs, (*(*cc).t).value.info, (*cc).na, -1);
            (*cc).na -= 1;
            (*cc).na;
        } else {
            if (*cc).expression_description.kind != V::VVOID {
                luak_exp2nextreg(fs, &mut (*cc).expression_description);
            }
            luak_setlist(fs, (*(*cc).t).value.info, (*cc).na, (*cc).to_store);
        }
        (*cc).na += (*cc).to_store;
    }
}
pub unsafe extern "C" fn setvararg(fs: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*fs).prototype).is_variable_arguments = true;
        luak_code_abck(fs, OP_VARARGPREP, nparams, 0, 0, 0);
    }
}
pub unsafe extern "C" fn errorlimit(fs: *mut FunctionState, limit: i32, what: *const i8) -> ! {
    unsafe {
        let state: *mut State = (*(*fs).lexical_state).state;
        let message: *const i8;
        let line: i32 = (*(*fs).prototype).line_defined;
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
        luax_syntaxerror((*fs).lexical_state, message);
    }
}
pub unsafe extern "C" fn checklimit(fs: *mut FunctionState, v: i32, l: i32, what: *const i8) {
    unsafe {
        if v > l {
            errorlimit(fs, l, what);
        }
    }
}
pub unsafe extern "C" fn getlocalvardesc(
    fs: *mut FunctionState,
    vidx: i32,
) -> *mut VariableDescription {
    unsafe {
        return &mut *((*(*(*fs).lexical_state).dynamic_data)
            .active_variable
            .pointer)
            .offset(((*fs).first_local + vidx) as isize) as *mut VariableDescription;
    }
}
pub unsafe extern "C" fn reglevel(fs: *mut FunctionState, mut nvar: i32) -> i32 {
    unsafe {
        loop {
            let fresh38 = nvar;
            nvar = nvar - 1;
            if !(fresh38 > 0) {
                break;
            }
            let vd: *mut VariableDescription = getlocalvardesc(fs, nvar);
            if (*vd).content.kind as i32 != 3 {
                return (*vd).content.ridx as i32 + 1;
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn luay_nvarstack(fs: *mut FunctionState) -> i32 {
    unsafe {
        return reglevel(fs, (*fs).count_active_variables as i32);
    }
}
pub unsafe extern "C" fn localdebuginfo(fs: *mut FunctionState, vidx: i32) -> *mut LocalVariable {
    unsafe {
        let vd: *mut VariableDescription = getlocalvardesc(fs, vidx);
        if (*vd).content.kind as i32 == 3 {
            return std::ptr::null_mut();
        } else {
            let index: i32 = (*vd).content.pidx as i32;
            return &mut *((*(*fs).prototype).local_variables).offset(index as isize) as *mut LocalVariable;
        };
    }
}
pub unsafe extern "C" fn init_var(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    vidx: i32,
) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).kind = V::VLOCAL;
        (*e).value.variable.value_index = vidx as u16;
        (*e).value.variable.register_index = (*getlocalvardesc(fs, vidx)).content.ridx;
    }
}
pub unsafe extern "C" fn removevars(fs: *mut FunctionState, tolevel: i32) {
    unsafe {
        (*(*(*fs).lexical_state).dynamic_data).active_variable.length -=
            (*fs).count_active_variables as i32 - tolevel;
        while (*fs).count_active_variables as i32 > tolevel {
            (*fs).count_active_variables = ((*fs).count_active_variables).wrapping_sub(1);
            let var: *mut LocalVariable = localdebuginfo(fs, (*fs).count_active_variables as i32);
            if !var.is_null() {
                (*var).end_program_counter = (*fs).program_counter;
            }
        }
    }
}
pub unsafe extern "C" fn searchupvalue(fs: *mut FunctionState, name: *mut TString) -> i32 {
    unsafe {
        let mut i: i32;
        let up: *mut UpValueDescription = (*(*fs).prototype).upvalues;
        i = 0;
        while i < (*fs).count_upvalues as i32 {
            if (*up.offset(i as isize)).name == name {
                return i;
            }
            i += 1;
        }
        return -1;
    }
}
pub unsafe extern "C" fn allocupvalue(fs: *mut FunctionState) -> *mut UpValueDescription {
    unsafe {
        let f: *mut Prototype = (*fs).prototype;
        let mut old_size: i32 = (*f).size_upvalues;
        checklimit(
            fs,
            (*fs).count_upvalues as i32 + 1,
            255 as i32,
            b"upvalues\0" as *const u8 as *const i8,
        );
        (*f).upvalues = luam_growaux_(
            (*(*fs).lexical_state).state,
            (*f).upvalues as *mut libc::c_void,
            (*fs).count_upvalues as i32,
            &mut (*f).size_upvalues,
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
        while old_size < (*f).size_upvalues {
            let fresh41 = old_size;
            old_size = old_size + 1;
            let ref mut fresh42 = (*((*f).upvalues).offset(fresh41 as isize)).name;
            *fresh42 = std::ptr::null_mut();
        }
        let fresh43 = (*fs).count_upvalues;
        (*fs).count_upvalues = ((*fs).count_upvalues).wrapping_add(1);
        return &mut *((*f).upvalues).offset(fresh43 as isize) as *mut UpValueDescription;
    }
}
pub unsafe extern "C" fn newupvalue(
    fs: *mut FunctionState,
    name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let up: *mut UpValueDescription = allocupvalue(fs);
        let previous: *mut FunctionState = (*fs).previous;
        if (*v).kind as u32 == V::VLOCAL as u32 {
            (*up).is_in_stack = true;
            (*up).index = (*v).value.variable.register_index;
            (*up).kind = (*getlocalvardesc(previous, (*v).value.variable.value_index as i32)).content.kind;
        } else {
            (*up).is_in_stack = false;
            (*up).index = (*v).value.info as u8;
            (*up).kind = (*((*(*previous).prototype).upvalues).offset((*v).value.info as isize)).kind;
        }
        (*up).name = name;
        if (*(*fs).prototype).get_marked() & 1 << 5 != 0 && (*name).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                (*(*fs).lexical_state).state,
                &mut (*((*fs).prototype as *mut Object)),
                &mut (*(name as *mut Object)),
            );
        } else {
        };
        return (*fs).count_upvalues as i32 - 1;
    }
}
pub unsafe extern "C" fn searchvar(
    fs: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let mut i: i32;
        i = (*fs).count_active_variables as i32 - 1;
        while i >= 0 {
            let vd: *mut VariableDescription = getlocalvardesc(fs, i);
            if n == (*vd).content.name {
                if (*vd).content.kind as i32 == 3 {
                    init_exp(var, V::VCONST, (*fs).first_local + i);
                } else {
                    init_var(fs, var, i);
                }
                return (*var).kind as i32;
            }
            i -= 1;
        }
        return -1;
    }
}
pub unsafe extern "C" fn markupval(fs: *mut FunctionState, level: i32) {
    unsafe {
        let mut block_control: *mut BlockControl = (*fs).block_control;
        while (*block_control).count_active_variables as i32 > level {
            block_control = (*block_control).previous;
        }
        (*block_control).count_upvalues = 1;
        (*fs).needs_close = true;
    }
}
pub unsafe extern "C" fn marktobeclosed(fs: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*fs).block_control;
        (*block_control).count_upvalues = 1;
        (*block_control).is_inside_tbc = true;
        (*fs).needs_close = true;
    }
}
pub unsafe extern "C" fn singlevaraux(
    fs: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
    base: i32,
) {
    unsafe {
        if fs.is_null() {
            init_exp(var, V::VVOID, 0);
        } else {
            let v: i32 = searchvar(fs, n, var);
            if v >= 0 {
                if v == V::VLOCAL as i32 && base == 0 {
                    markupval(fs, (*var).value.variable.value_index as i32);
                }
            } else {
                let mut index: i32 = searchupvalue(fs, n);
                if index < 0 {
                    singlevaraux((*fs).previous, n, var, 0);
                    if (*var).kind == V::VLOCAL
                        || (*var).kind == V::VUPVAL
                    {
                        index = newupvalue(fs, n, var);
                    } else {
                        return;
                    }
                }
                init_exp(var, V::VUPVAL, index);
            }
        };
    }
}
pub unsafe extern "C" fn fixforjump(
    fs: *mut FunctionState,
    program_counter: i32,
    dest: i32,
    back: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*fs).prototype).code).offset(program_counter as isize) as *mut u32;
        let mut offset: i32 = dest - (program_counter + 1);
        if back != 0 {
            offset = -offset;
        }
        if ((offset > (1 << 8 + 8 + 1) - 1) as i32 != 0) as i64 != 0 {
            luax_syntaxerror(
                (*fs).lexical_state,
                b"control structure too long\0" as *const u8 as *const i8,
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1) << 0 + 7 + 8)
            | (offset as u32) << 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0 + 7 + 8;
    }
}
pub unsafe extern "C" fn checktoclose(fs: *mut FunctionState, level: i32) {
    unsafe {
        if level != -1 {
            marktobeclosed(fs);
            luak_code_abck(fs, OP_TBC, reglevel(fs, level), 0, 0, 0);
        }
    }
}
pub unsafe extern "C" fn previousinstruction(fs: *mut FunctionState) -> *mut u32 {
    unsafe {
        pub const INVALID_INSTRUCTION: u32 = !(0u32);
        if (*fs).program_counter > (*fs).last_target {
            return &mut *((*(*fs).prototype).code).offset(((*fs).program_counter - 1) as isize)
                as *mut u32;
        } else {
            return &INVALID_INSTRUCTION as *const u32 as *mut u32;
        };
    }
}
pub unsafe extern "C" fn luak_nil(fs: *mut FunctionState, mut from: i32, n: i32) {
    unsafe {
        let mut l: i32 = from + n - 1;
        let previous: *mut u32 = previousinstruction(fs);
        if (*previous >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_LOADNIL as u32 {
            let pfrom: i32 = (*previous >> 0 + 7 & !(!(0u32) << 8) << 0) as i32;
            let pl: i32 = pfrom + (*previous >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= l + 1 {
                if pfrom < from {
                    from = pfrom;
                }
                if pl > l {
                    l = pl;
                }
                *previous = *previous & !(!(!(0u32) << 8) << 0 + 7)
                    | (from as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
                *previous = *previous & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1)
                    | ((l - from) as u32) << 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
                return;
            }
        }
        luak_code_abck(fs, OP_LOADNIL, from, n - 1, 0, 0);
    }
}
pub unsafe extern "C" fn getjump(fs: *mut FunctionState, program_counter: i32) -> i32 {
    unsafe {
        let offset: i32 = (*((*(*fs).prototype).code).offset(program_counter as isize) >> 0 + 7
            & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
            - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
        if offset == -1 {
            return -1;
        } else {
            return program_counter + 1 + offset;
        };
    }
}
pub unsafe extern "C" fn fixjump(fs: *mut FunctionState, program_counter: i32, dest: i32) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*fs).prototype).code).offset(program_counter as isize) as *mut u32;
        let offset: i32 = dest - (program_counter + 1);
        if !(-((1 << 8 + 8 + 1 + 8) - 1 >> 1) <= offset
            && offset <= (1 << 8 + 8 + 1 + 8) - 1 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1))
        {
            luax_syntaxerror(
                (*fs).lexical_state,
                b"control structure too long\0" as *const u8 as *const i8,
            );
        }
        *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1 + 8) << 0 + 7)
            | ((offset + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32) << 0 + 7
                & !(!(0u32) << 8 + 8 + 1 + 8) << 0 + 7;
    }
}
pub unsafe extern "C" fn luak_concat(fs: *mut FunctionState, l1: *mut i32, l2: i32) {
    unsafe {
        if l2 == -1 {
            return;
        } else if *l1 == -1 {
            *l1 = l2;
        } else {
            let mut list: i32 = *l1;
            let mut next: i32;
            loop {
                next = getjump(fs, list);
                if !(next != -1) {
                    break;
                }
                list = next;
            }
            fixjump(fs, list, l2);
        };
    }
}
pub unsafe extern "C" fn luak_jump(fs: *mut FunctionState) -> i32 {
    unsafe {
        return codesj(fs, OP_JMP, -1, 0);
    }
}
pub unsafe extern "C" fn luak_ret(fs: *mut FunctionState, first: i32, nret: i32) {
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
        luak_code_abck(fs, op, first, nret + 1, 0, 0);
    }
}
pub unsafe extern "C" fn condjump(
    fs: *mut FunctionState,
    op: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        luak_code_abck(fs, op, a, b, c, k);
        return luak_jump(fs);
    }
}
pub unsafe extern "C" fn luak_getlabel(fs: *mut FunctionState) -> i32 {
    unsafe {
        (*fs).last_target = (*fs).program_counter;
        return (*fs).program_counter;
    }
}
pub unsafe extern "C" fn getjumpcontrol(fs: *mut FunctionState, program_counter: i32) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*fs).prototype).code).offset(program_counter as isize) as *mut u32;
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
pub unsafe extern "C" fn patchtestreg(fs: *mut FunctionState, node: i32, reg: i32) -> i32 {
    unsafe {
        let i: *mut u32 = getjumpcontrol(fs, node);
        if (*i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
            return 0;
        }
        if reg != (1 << 8) - 1 && reg != (*i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32 {
            *i =
                *i & !(!(!(0u32) << 8) << 0 + 7) | (reg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
        } else {
            *i = (OP_TEST as u32) << 0
                | ((*i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as u32) << 0 + 7
                | (0u32) << 0 + 7 + 8 + 1
                | (0u32) << 0 + 7 + 8 + 1 + 8
                | ((*i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as u32) << 0 + 7 + 8;
        }
        return 1;
    }
}
pub unsafe extern "C" fn removevalues(fs: *mut FunctionState, mut list: i32) {
    unsafe {
        while list != -1 {
            patchtestreg(fs, list, (1 << 8) - 1);
            list = getjump(fs, list);
        }
    }
}
pub unsafe extern "C" fn patchlistaux(
    fs: *mut FunctionState,
    mut list: i32,
    vtarget: i32,
    reg: i32,
    dtarget: i32,
) {
    unsafe {
        while list != -1 {
            let next: i32 = getjump(fs, list);
            if patchtestreg(fs, list, reg) != 0 {
                fixjump(fs, list, vtarget);
            } else {
                fixjump(fs, list, dtarget);
            }
            list = next;
        }
    }
}
pub unsafe extern "C" fn luak_patchlist(fs: *mut FunctionState, list: i32, target: i32) {
    unsafe {
        patchlistaux(fs, list, target, (1 << 8) - 1, target);
    }
}
pub unsafe extern "C" fn luak_patchtohere(fs: *mut FunctionState, list: i32) {
    unsafe {
        let hr: i32 = luak_getlabel(fs);
        luak_patchlist(fs, list, hr);
    }
}
pub unsafe extern "C" fn savelineinfo(fs: *mut FunctionState, f: *mut Prototype, line: i32) {
    unsafe {
        let mut linedif: i32 = line - (*fs).previous_line;
        let program_counter: i32 = (*fs).program_counter - 1;
        if abs(linedif) >= 0x80 as i32 || {
            let fresh132 = (*fs).iwthabs;
            (*fs).iwthabs = ((*fs).iwthabs).wrapping_add(1);
            fresh132 as i32 >= 128 as i32
        } {
            (*f).absolute_line_info = luam_growaux_(
                (*(*fs).lexical_state).state,
                (*f).absolute_line_info as *mut libc::c_void,
                (*fs).count_abslineinfo,
                &mut (*f).size_absolute_line_info,
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
            (*((*f).absolute_line_info).offset((*fs).count_abslineinfo as isize)).program_counter =
                program_counter;
            let fresh133 = (*fs).count_abslineinfo;
            (*fs).count_abslineinfo = (*fs).count_abslineinfo + 1;
            (*((*f).absolute_line_info).offset(fresh133 as isize)).line = line;
            linedif = -(0x80 as i32);
            (*fs).iwthabs = 1;
        }
        (*f).line_info = luam_growaux_(
            (*(*fs).lexical_state).state,
            (*f).line_info as *mut libc::c_void,
            program_counter,
            &mut (*f).size_line_info,
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
        *((*f).line_info).offset(program_counter as isize) = linedif as i8;
        (*fs).previous_line = line;
    }
}
pub unsafe extern "C" fn removelastlineinfo(fs: *mut FunctionState) {
    unsafe {
        let f: *mut Prototype = (*fs).prototype;
        let program_counter: i32 = (*fs).program_counter - 1;
        if *((*f).line_info).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            (*fs).previous_line -= *((*f).line_info).offset(program_counter as isize) as i32;
            (*fs).iwthabs = ((*fs).iwthabs).wrapping_sub(1);
            (*fs).iwthabs;
        } else {
            (*fs).count_abslineinfo -= 1;
            (*fs).count_abslineinfo;
            (*fs).iwthabs = (128 as i32 + 1) as u8;
        };
    }
}
pub unsafe extern "C" fn removelastinstruction(fs: *mut FunctionState) {
    unsafe {
        removelastlineinfo(fs);
        (*fs).program_counter -= 1;
        (*fs).program_counter;
    }
}
pub unsafe extern "C" fn luak_code(fs: *mut FunctionState, i: u32) -> i32 {
    unsafe {
        let f: *mut Prototype = (*fs).prototype;
        (*f).code = luam_growaux_(
            (*(*fs).lexical_state).state,
            (*f).code as *mut libc::c_void,
            (*fs).program_counter,
            &mut (*f).size_code,
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
        let fresh134 = (*fs).program_counter;
        (*fs).program_counter = (*fs).program_counter + 1;
        *((*f).code).offset(fresh134 as isize) = i;
        savelineinfo(fs, f, (*(*fs).lexical_state).last_line);
        return (*fs).program_counter - 1;
    }
}
pub unsafe extern "C" fn luak_code_abck(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        return luak_code(
            fs,
            (o as u32) << 0
                | (a as u32) << 0 + 7
                | (b as u32) << 0 + 7 + 8 + 1
                | (c as u32) << 0 + 7 + 8 + 1 + 8
                | (k as u32) << 0 + 7 + 8,
        );
    }
}
pub unsafe extern "C" fn luak_codeabx(fs: *mut FunctionState, o: u32, a: i32, bc: u32) -> i32 {
    unsafe {
        return luak_code(fs, (o as u32) << 0 | (a as u32) << 0 + 7 | bc << 0 + 7 + 8);
    }
}
pub unsafe extern "C" fn codeasbx(fs: *mut FunctionState, o: u32, a: i32, bc: i32) -> i32 {
    unsafe {
        let b: u32 = (bc + ((1 << 8 + 8 + 1) - 1 >> 1)) as u32;
        return luak_code(fs, (o as u32) << 0 | (a as u32) << 0 + 7 | b << 0 + 7 + 8);
    }
}
pub unsafe extern "C" fn codesj(fs: *mut FunctionState, o: u32, sj: i32, k: i32) -> i32 {
    unsafe {
        let j: u32 = (sj + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32;
        return luak_code(fs, (o as u32) << 0 | j << 0 + 7 | (k as u32) << 0 + 7 + 8);
    }
}
pub unsafe extern "C" fn codeextraarg(fs: *mut FunctionState, a: i32) -> i32 {
    unsafe {
        return luak_code(fs, (OP_EXTRAARG as u32) << 0 | (a as u32) << 0 + 7);
    }
}
pub unsafe extern "C" fn luak_codek(fs: *mut FunctionState, reg: i32, k: i32) -> i32 {
    unsafe {
        if k <= (1 << 8 + 8 + 1) - 1 {
            return luak_codeabx(fs, OP_LOADK, reg, k as u32);
        } else {
            let p: i32 = luak_codeabx(fs, OP_LOADKX, reg, 0u32);
            codeextraarg(fs, k);
            return p;
        };
    }
}
pub unsafe extern "C" fn luak_checkstack(fs: *mut FunctionState, n: i32) {
    unsafe {
        let newstack: i32 = (*fs).freereg as i32 + n;
        if newstack > (*(*fs).prototype).maximum_stack_size as i32 {
            if newstack >= 255 as i32 {
                luax_syntaxerror(
                    (*fs).lexical_state,
                    b"function or expression needs too many registers\0" as *const u8 as *const i8,
                );
            }
            (*(*fs).prototype).maximum_stack_size = newstack as u8;
        }
    }
}
pub unsafe extern "C" fn luak_reserveregs(fs: *mut FunctionState, n: i32) {
    unsafe {
        luak_checkstack(fs, n);
        (*fs).freereg = ((*fs).freereg as i32 + n) as u8;
    }
}
pub unsafe extern "C" fn freereg(fs: *mut FunctionState, reg: i32) {
    unsafe {
        if reg >= luay_nvarstack(fs) {
            (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
            (*fs).freereg;
        }
    }
}
pub unsafe extern "C" fn freeregs(fs: *mut FunctionState, r1: i32, r2: i32) {
    unsafe {
        if r1 > r2 {
            freereg(fs, r1);
            freereg(fs, r2);
        } else {
            freereg(fs, r2);
            freereg(fs, r1);
        };
    }
}
pub unsafe extern "C" fn freeexp(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).kind as u32 == V::VNONRELOC as u32 {
            freereg(fs, (*e).value.info);
        }
    }
}
pub unsafe extern "C" fn freeexps(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32 = if (*e1).kind as u32 == V::VNONRELOC as u32 {
            (*e1).value.info
        } else {
            -1
        };
        let r2: i32 = if (*e2).kind as u32 == V::VNONRELOC as u32 {
            (*e2).value.info
        } else {
            -1
        };
        freeregs(fs, r1, r2);
    }
}
pub unsafe extern "C" fn addk(fs: *mut FunctionState, key: *mut TValue, v: *mut TValue) -> i32 {
    unsafe {
        let mut value: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let state: *mut State = (*(*fs).lexical_state).state;
        let f: *mut Prototype = (*fs).prototype;
        let index: *const TValue = luah_get((*(*fs).lexical_state).table, key);
        let mut k: i32;
        if (*index).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            k = (*index).value.integer as i32;
            if k < (*fs).count_k
                && (*((*f).k).offset(k as isize)).get_tag_variant() == (*v).get_tag_variant()
                && luav_equalobj(std::ptr::null_mut(), &mut *((*f).k).offset(k as isize), v)
            {
                return k;
            }
        }
        let mut old_size: i32 = (*f).size_k;
        k = (*fs).count_k;
        let io: *mut TValue = &mut value;
        (*io).value.integer = k as i64;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        luah_finishset(state, (*(*fs).lexical_state).table, key, index, &mut value);
        (*f).k = luam_growaux_(
            state,
            (*f).k as *mut libc::c_void,
            k,
            &mut (*f).size_k,
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
        while old_size < (*f).size_k {
            let fresh135 = old_size;
            old_size = old_size + 1;
            (*((*f).k).offset(fresh135 as isize)).set_tag(TAG_VARIANT_NIL_NIL);
        }
        let io1: *mut TValue = &mut *((*f).k).offset(k as isize) as *mut TValue;
        let io2: *const TValue = v;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        (*fs).count_k += 1;
        (*fs).count_k;
        if (*v).is_collectable() {
            if (*f).get_marked() & 1 << 5 != 0
                && (*(*v).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    state,
                    &mut (*(f as *mut Object)),
                    &mut (*((*v).value.object as *mut Object)),
                );
            } else {
            };
        } else {
        };
        return k;
    }
}
pub unsafe extern "C" fn string_k(fs: *mut FunctionState, s: *mut TString) -> i32 {
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
        return addk(fs, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn luak_int_k(fs: *mut FunctionState, n: i64) -> i32 {
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
        return addk(fs, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn luak_number_k(fs: *mut FunctionState, r: f64) -> i32 {
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
            return addk(fs, &mut o, &mut o);
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
            return addk(fs, &mut kv, &mut o);
        };
    }
}
pub unsafe extern "C" fn bool_false(fs: *mut FunctionState) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        o.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
        return addk(fs, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn bool_true(fs: *mut FunctionState) -> i32 {
    unsafe {
        let mut o: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        o.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
        return addk(fs, &mut o, &mut o);
    }
}
pub unsafe extern "C" fn nil_k(fs: *mut FunctionState) -> i32 {
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
        let x_: *mut Table = (*(*fs).lexical_state).table;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_TABLE);
        (*io).set_collectable();
        return addk(fs, &mut k, &mut v);
    }
}
pub unsafe extern "C" fn luak_int(fs: *mut FunctionState, reg: i32, i: i64) {
    unsafe {
        if fits_bx(i) {
            codeasbx(fs, OP_LOADI, reg, i as i32);
        } else {
            luak_codek(fs, reg, luak_int_k(fs, i));
        };
    }
}
pub unsafe extern "C" fn luak_float(fs: *mut FunctionState, reg: i32, f: f64) {
    unsafe {
        let mut fi: i64 = 0;
        if luav_flttointeger(f, &mut fi, F2I::Equal) && fits_bx(fi) {
            codeasbx(fs, OP_LOADF, reg, fi as i32);
        } else {
            luak_codek(fs, reg, luak_number_k(fs, f));
        };
    }
}
pub unsafe extern "C" fn luak_setreturns(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    count_results: i32,
) {
    unsafe {
        let program_counter: *mut u32 =
            &mut *((*(*fs).prototype).code).offset((*e).value.info as isize) as *mut u32;
        if (*e).kind as u32 == V::VCALL as u32 {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
                | ((count_results + 1) as u32) << 0 + 7 + 8 + 1 + 8
                    & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
        } else {
            *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
                | ((count_results + 1) as u32) << 0 + 7 + 8 + 1 + 8
                    & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
            *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7)
                | ((*fs).freereg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
            luak_reserveregs(fs, 1);
        };
    }
}
pub unsafe extern "C" fn str_to_k(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        (*e).value.info = string_k(fs, (*e).value.tstring);
        (*e).kind = V::VK;
    }
}
pub unsafe extern "C" fn luak_setoneret(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).kind as u32 == V::VCALL as u32 {
            (*e).kind = V::VNONRELOC;
            (*e).value.info = (*((*(*fs).prototype).code).offset((*e).value.info as isize) >> 0 + 7
                & !(!(0u32) << 8) << 0) as i32;
        } else if (*e).kind as u32 == V::VVARARG as u32 {
            *((*(*fs).prototype).code).offset((*e).value.info as isize) = *((*(*fs).prototype).code)
                .offset((*e).value.info as isize)
                & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
                | (2 as u32) << 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
            (*e).kind = V::VRELOC;
        }
    }
}
pub unsafe extern "C" fn luak_dischargevars(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        match (*e).kind {
            V::VCONST => {
                const2exp(const2val(fs, e), e);
            }
            V::VLOCAL => {
                let temp: i32 = (*e).value.variable.register_index as i32;
                (*e).value.info = temp;
                (*e).kind = V::VNONRELOC;
            }
            V::VUPVAL => {
                (*e).value.info = luak_code_abck(fs, OP_GETUPVAL, 0, (*e).value.info, 0, 0);
                (*e).kind = V::VRELOC;
            }
            V::VINDEXUP => {
                (*e).value.info = luak_code_abck(
                    fs,
                    OP_GETTABUP,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).kind = V::VRELOC;
            }
            V::VINDEXI => {
                freereg(fs, (*e).value.index.reference_tag as i32);
                (*e).value.info = luak_code_abck(
                    fs,
                    OP_GETI,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).kind = V::VRELOC;
            }
            V::VINDEXSTR => {
                freereg(fs, (*e).value.index.reference_tag as i32);
                (*e).value.info = luak_code_abck(
                    fs,
                    OP_GETFIELD,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).kind = V::VRELOC;
            }
            V::VINDEXED => {
                freeregs(fs, (*e).value.index.reference_tag as i32, (*e).value.index.reference_index as i32);
                (*e).value.info = luak_code_abck(
                    fs,
                    OP_GETTABLE,
                    0,
                    (*e).value.index.reference_tag as i32,
                    (*e).value.index.reference_index as i32,
                    0,
                );
                (*e).kind = V::VRELOC;
            }
            V::VVARARG | V::VCALL => {
                luak_setoneret(fs, e);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn discharge2reg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    reg: i32,
) {
    unsafe {
        luak_dischargevars(fs, e);
        let current_block_14: u64;
        match (*e).kind as u32 {
            1 => {
                luak_nil(fs, reg, 1);
                current_block_14 = 13242334135786603907;
            }
            3 => {
                luak_code_abck(fs, OP_LOADFALSE, reg, 0, 0, 0);
                current_block_14 = 13242334135786603907;
            }
            2 => {
                luak_code_abck(fs, OP_LOADTRUE, reg, 0, 0, 0);
                current_block_14 = 13242334135786603907;
            }
            7 => {
                str_to_k(fs, e);
                current_block_14 = 6937071982253665452;
            }
            4 => {
                current_block_14 = 6937071982253665452;
            }
            5 => {
                luak_float(fs, reg, (*e).value.number);
                current_block_14 = 13242334135786603907;
            }
            6 => {
                luak_int(fs, reg, (*e).value.integer);
                current_block_14 = 13242334135786603907;
            }
            17 => {
                let program_counter: *mut u32 =
                    &mut *((*(*fs).prototype).code).offset((*e).value.info as isize) as *mut u32;
                *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7)
                    | (reg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
                current_block_14 = 13242334135786603907;
            }
            8 => {
                if reg != (*e).value.info {
                    luak_code_abck(fs, OP_MOVE, reg, (*e).value.info, 0, 0);
                }
                current_block_14 = 13242334135786603907;
            }
            _ => return,
        }
        match current_block_14 {
            6937071982253665452 => {
                luak_codek(fs, reg, (*e).value.info);
            }
            _ => {}
        }
        (*e).value.info = reg;
        (*e).kind = V::VNONRELOC;
    }
}
pub unsafe extern "C" fn discharge2anyreg(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).kind as u32 != V::VNONRELOC as u32 {
            luak_reserveregs(fs, 1);
            discharge2reg(fs, e, (*fs).freereg as i32 - 1);
        }
    }
}
pub unsafe extern "C" fn code_loadbool(fs: *mut FunctionState, a: i32, op: u32) -> i32 {
    unsafe {
        luak_getlabel(fs);
        return luak_code_abck(fs, op, a, 0, 0, 0);
    }
}
pub unsafe extern "C" fn need_value(fs: *mut FunctionState, mut list: i32) -> i32 {
    unsafe {
        while list != -1 {
            let i: u32 = *getjumpcontrol(fs, list);
            if (i >> 0 & !(!(0u32) << 7) << 0) as u32 != OP_TESTSET as u32 {
                return 1;
            }
            list = getjump(fs, list);
        }
        return 0;
    }
}
pub unsafe extern "C" fn exp2reg(fs: *mut FunctionState, e: *mut ExpressionDescription, reg: i32) {
    unsafe {
        discharge2reg(fs, e, reg);
        if (*e).kind as u32 == V::VJMP as u32 {
            luak_concat(fs, &mut (*e).t, (*e).value.info);
        }
        if (*e).t != (*e).f {
            let mut p_f: i32 = -1;
            let mut p_t: i32 = -1;
            if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
                let fj: i32 = if (*e).kind as u32 == V::VJMP as u32 {
                    -1
                } else {
                    luak_jump(fs)
                };
                p_f = code_loadbool(fs, reg, OP_LFALSESKIP);
                p_t = code_loadbool(fs, reg, OP_LOADTRUE);
                luak_patchtohere(fs, fj);
            }
            let final_0: i32 = luak_getlabel(fs);
            patchlistaux(fs, (*e).f, final_0, reg, p_f);
            patchlistaux(fs, (*e).t, final_0, reg, p_t);
        }
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).value.info = reg;
        (*e).kind = V::VNONRELOC;
    }
}
pub unsafe extern "C" fn luak_exp2nextreg(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        luak_dischargevars(fs, e);
        freeexp(fs, e);
        luak_reserveregs(fs, 1);
        exp2reg(fs, e, (*fs).freereg as i32 - 1);
    }
}
pub unsafe extern "C" fn luak_exp2anyreg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        luak_dischargevars(fs, e);
        if (*e).kind as u32 == V::VNONRELOC as u32 {
            if !((*e).t != (*e).f) {
                return (*e).value.info;
            }
            if (*e).value.info >= luay_nvarstack(fs) {
                exp2reg(fs, e, (*e).value.info);
                return (*e).value.info;
            }
        }
        luak_exp2nextreg(fs, e);
        return (*e).value.info;
    }
}
pub unsafe extern "C" fn luak_exp2anyregup(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).kind as u32 != V::VUPVAL as u32 || (*e).t != (*e).f {
            luak_exp2anyreg(fs, e);
        }
    }
}
pub unsafe extern "C" fn luak_exp2val(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        if (*e).kind as u32 == V::VJMP as u32 || (*e).t != (*e).f {
            luak_exp2anyreg(fs, e);
        } else {
            luak_dischargevars(fs, e);
        };
    }
}
pub unsafe extern "C" fn luak_exp2k(fs: *mut FunctionState, e: *mut ExpressionDescription) -> i32 {
    unsafe {
        if !((*e).t != (*e).f) {
            let info: i32;
            match (*e).kind as u32 {
                2 => {
                    info = bool_true(fs);
                }
                3 => {
                    info = bool_false(fs);
                }
                1 => {
                    info = nil_k(fs);
                }
                6 => {
                    info = luak_int_k(fs, (*e).value.integer);
                }
                5 => {
                    info = luak_number_k(fs, (*e).value.number);
                }
                7 => {
                    info = string_k(fs, (*e).value.tstring);
                }
                4 => {
                    info = (*e).value.info;
                }
                _ => return 0,
            }
            if info <= (1 << 8) - 1 {
                (*e).kind = V::VK;
                (*e).value.info = info;
                return 1;
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn exp2rk(fs: *mut FunctionState, e: *mut ExpressionDescription) -> i32 {
    unsafe {
        if luak_exp2k(fs, e) != 0 {
            return 1;
        } else {
            luak_exp2anyreg(fs, e);
            return 0;
        };
    }
}
pub unsafe extern "C" fn codeabrk(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    ec: *mut ExpressionDescription,
) {
    unsafe {
        let k: i32 = exp2rk(fs, ec);
        luak_code_abck(fs, o, a, b, (*ec).value.info, k);
    }
}
pub unsafe extern "C" fn luak_storevar(
    fs: *mut FunctionState,
    var: *mut ExpressionDescription,
    ex: *mut ExpressionDescription,
) {
    unsafe {
        match (*var).kind {
            V::VLOCAL => {
                freeexp(fs, ex);
                exp2reg(fs, ex, (*var).value.variable.register_index as i32);
                return;
            }
            V::VUPVAL => {
                let e: i32 = luak_exp2anyreg(fs, ex);
                luak_code_abck(fs, OP_SETUPVAL, e, (*var).value.info, 0, 0);
            }
            V::VINDEXUP => {
                codeabrk(
                    fs,
                    OP_SETTABUP,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            V::VINDEXI => {
                codeabrk(
                    fs,
                    OP_SETI,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            V::VINDEXSTR => {
                codeabrk(
                    fs,
                    OP_SETFIELD,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            V::VINDEXED => {
                codeabrk(
                    fs,
                    OP_SETTABLE,
                    (*var).value.index.reference_tag as i32,
                    (*var).value.index.reference_index as i32,
                    ex,
                );
            }
            _ => {}
        }
        freeexp(fs, ex);
    }
}
pub unsafe extern "C" fn luak_self(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    key: *mut ExpressionDescription,
) {
    unsafe {
        luak_exp2anyreg(fs, e);
        let ereg: i32 = (*e).value.info;
        freeexp(fs, e);
        (*e).value.info = (*fs).freereg as i32;
        (*e).kind = V::VNONRELOC;
        luak_reserveregs(fs, 2);
        codeabrk(fs, OP_SELF, (*e).value.info, ereg, key);
        freeexp(fs, key);
    }
}
pub unsafe extern "C" fn negatecondition(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: *mut u32 = getjumpcontrol(fs, (*e).value.info);
        *program_counter = *program_counter & !(!(!(0u32) << 1) << 0 + 7 + 8)
            | (((*program_counter >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 ^ 1) as u32)
                << 0 + 7 + 8
                & !(!(0u32) << 1) << 0 + 7 + 8;
    }
}
pub unsafe extern "C" fn jumponcond(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    cond_0: i32,
) -> i32 {
    unsafe {
        if (*e).kind as u32 == V::VRELOC as u32 {
            let ie: u32 = *((*(*fs).prototype).code).offset((*e).value.info as isize);
            if (ie >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_NOT as u32 {
                removelastinstruction(fs);
                return condjump(
                    fs,
                    OP_TEST,
                    (ie >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32,
                    0,
                    0,
                    (cond_0 == 0) as i32,
                );
            }
        }
        discharge2anyreg(fs, e);
        freeexp(fs, e);
        return condjump(fs, OP_TESTSET, (1 << 8) - 1, (*e).value.info, 0, cond_0);
    }
}
pub unsafe extern "C" fn luak_goiftrue(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(fs, e);
        match (*e).kind as u32 {
            16 => {
                negatecondition(fs, e);
                program_counter = (*e).value.info;
            }
            4 | 5 | 6 | 7 | 2 => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(fs, e, 0);
            }
        }
        luak_concat(fs, &mut (*e).f, program_counter);
        luak_patchtohere(fs, (*e).t);
        (*e).t = -1;
    }
}
pub unsafe extern "C" fn luak_goiffalse(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(fs, e);
        match (*e).kind as u32 {
            16 => {
                program_counter = (*e).value.info;
            }
            1 | 3 => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(fs, e, 1);
            }
        }
        luak_concat(fs, &mut (*e).t, program_counter);
        luak_patchtohere(fs, (*e).f);
        (*e).f = -1;
    }
}
pub unsafe extern "C" fn codenot(fs: *mut FunctionState, e: *mut ExpressionDescription) {
    unsafe {
        match (*e).kind as u32 {
            1 | 3 => {
                (*e).kind = V::VTRUE;
            }
            4 | 5 | 6 | 7 | 2 => {
                (*e).kind = V::VFALSE;
            }
            16 => {
                negatecondition(fs, e);
            }
            17 | 8 => {
                discharge2anyreg(fs, e);
                freeexp(fs, e);
                (*e).value.info = luak_code_abck(fs, OP_NOT, 0, (*e).value.info, 0, 0);
                (*e).kind = V::VRELOC;
            }
            _ => {}
        }
        let temp: i32 = (*e).f;
        (*e).f = (*e).t;
        (*e).t = temp;
        removevalues(fs, (*e).f);
        removevalues(fs, (*e).t);
    }
}
pub unsafe extern "C" fn is_k_string(fs: *mut FunctionState, e: *mut ExpressionDescription) -> bool{
    unsafe {
        return (*e).kind == V::VK
            && !((*e).t != (*e).f)
            && (*e).value.info <= ((1 << 8) - 1)
            && (*((*(*fs).prototype).k).offset((*e).value.info as isize)).get_tag_variant()
                == TAG_VARIANT_STRING_SHORT;
    }
}
pub unsafe extern "C" fn constfolding(
    fs: *mut FunctionState,
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
        luao_rawarith((*(*fs).lexical_state).state, op, &mut v1, &mut v2, &mut res);
        if res.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            (*e1).kind = V::VKINT;
            (*e1).value.integer = res.value.integer;
        } else {
            let n: f64 = res.value.number;
            if !(n == n) || n == 0.0 {
                return 0;
            }
            (*e1).kind = V::VKFLT;
            (*e1).value.number = n;
        }
        return 1;
    }
}
pub unsafe extern "C" fn codeunexpval(
    fs: *mut FunctionState,
    op: u32,
    e: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let r: i32 = luak_exp2anyreg(fs, e);
        freeexp(fs, e);
        (*e).value.info = luak_code_abck(fs, op, 0, r, 0, 0);
        (*e).kind = V::VRELOC;
        luak_fixline(fs, line);
    }
}
pub unsafe extern "C" fn finishbinexpval(
    fs: *mut FunctionState,
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
        let v1: i32 = luak_exp2anyreg(fs, e1);
        let program_counter: i32 = luak_code_abck(fs, op, 0, v1, v2, 0);
        freeexps(fs, e1, e2);
        (*e1).value.info = program_counter;
        (*e1).kind = V::VRELOC;
        luak_fixline(fs, line);
        luak_code_abck(fs, mmop, v1, v2, event as i32, flip);
        luak_fixline(fs, line);
    }
}
pub unsafe extern "C" fn codebinexpval(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let op: u32 = binopr2op(opr, OPR_ADD, OP_ADD);
        let v2: i32 = luak_exp2anyreg(fs, e2);
        finishbinexpval(fs, e1, e2, op, v2, 0, line, OP_MMBIN, binopr2tm(opr));
    }
}
pub unsafe extern "C" fn codebini(
    fs: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
    event: u32,
) {
    unsafe {
        let v2: i32 = (*e2).value.integer as i32 + ((1 << 8) - 1 >> 1);
        finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINI, event);
    }
}
pub unsafe extern "C" fn codebink(
    fs: *mut FunctionState,
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
        finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINK, event);
    }
}
pub unsafe extern "C" fn finishbinexpneg(
    fs: *mut FunctionState,
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
                    fs,
                    e1,
                    e2,
                    op,
                    -v2 + ((1 << 8) - 1 >> 1),
                    0,
                    line,
                    OP_MMBINI,
                    event,
                );
                *((*(*fs).prototype).code).offset(((*fs).program_counter - 1) as isize) =
                    *((*(*fs).prototype).code).offset(((*fs).program_counter - 1) as isize)
                        & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1)
                        | ((v2 + ((1 << 8) - 1 >> 1)) as u32) << 0 + 7 + 8 + 1
                            & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
                return 1;
            }
        };
    }
}
pub unsafe extern "C" fn codebinnok(
    fs: *mut FunctionState,
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
        codebinexpval(fs, opr, e1, e2, line);
    }
}
pub unsafe extern "C" fn codearith(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if tonumeral(e2, std::ptr::null_mut()) && luak_exp2k(fs, e2) != 0{
            codebink(fs, opr, e1, e2, flip, line);
        } else {
            codebinnok(fs, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codecommutative(
    fs: *mut FunctionState,
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
            codebini(fs, OP_ADDI, e1, e2, flip, line, TM_ADD);
        } else {
            codearith(fs, op, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codebitwise(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if (*e1).kind == V::VKINT {
            swapexps(e1, e2);
            flip = 1;
        }
        if (*e2).kind == V::VKINT && luak_exp2k(fs, e2) != 0 {
            codebink(fs, opr, e1, e2, flip, line);
        } else {
            codebinnok(fs, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe extern "C" fn codeorder(
    fs: *mut FunctionState,
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
            r1 = luak_exp2anyreg(fs, e1);
            r2 = im;
            op = binopr2op(opr, OPR_LT, OP_LTI);
        } else if is_sc_number(e1, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(fs, e2);
            r2 = im;
            op = binopr2op(opr, OPR_LT, OP_GTI);
        } else {
            r1 = luak_exp2anyreg(fs, e1);
            r2 = luak_exp2anyreg(fs, e2);
            op = binopr2op(opr, OPR_LT, OP_LT);
        }
        freeexps(fs, e1, e2);
        (*e1).value.info = condjump(fs, op, r1, r2, is_float as i32, 1);
        (*e1).kind = V::VJMP;
    }
}
pub unsafe extern "C" fn codeeq(
    fs: *mut FunctionState,
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
        if (*e1).kind as u32 != V::VNONRELOC as u32 {
            swapexps(e1, e2);
        }
        r1 = luak_exp2anyreg(fs, e1);
        if is_sc_number(e2, &mut im, &mut is_float) != 0 {
            op = OP_EQI;
            r2 = im;
        } else if exp2rk(fs, e2) != 0 {
            op = OP_EQK;
            r2 = (*e2).value.info;
        } else {
            op = OP_EQ;
            r2 = luak_exp2anyreg(fs, e2);
        }
        freeexps(fs, e1, e2);
        (*e1).value.info = condjump(
            fs,
            op,
            r1,
            r2,
            is_float as i32,
            (opr as u32 == OPR_EQ as u32) as i32,
        );
        (*e1).kind = V::VJMP;
    }
}
pub unsafe extern "C" fn luak_prefix(
    fs: *mut FunctionState,
    opr: OperatorUnary,
    e: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        pub const EF: ExpressionDescription = {
            let init = ExpressionDescription {
                kind: V::VKINT,
                value: Value { integer: 0 },
                t: -1,
                f: -1,
            };
            init
        };
        luak_dischargevars(fs, e);
        let current_block_3: u64;
        match opr as u32 {
            0 | 1 => {
                if constfolding(
                    fs,
                    (opr as u32).wrapping_add(12 as u32) as i32,
                    e,
                    &EF,
                ) != 0
                {
                    current_block_3 = 7815301370352969686;
                } else {
                    current_block_3 = 4051245927518328098;
                }
            }
            3 => {
                current_block_3 = 4051245927518328098;
            }
            2 => {
                codenot(fs, e);
                current_block_3 = 7815301370352969686;
            }
            _ => {
                current_block_3 = 7815301370352969686;
            }
        }
        match current_block_3 {
            4051245927518328098 => {
                codeunexpval(fs, unopr2op(opr), e, line);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luak_infix(
    fs: *mut FunctionState,
    op: u32,
    v: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(fs, v);
        match op as u32 {
            19 => {
                luak_goiftrue(fs, v);
            }
            20 => {
                luak_goiffalse(fs, v);
            }
            12 => {
                luak_exp2nextreg(fs, v);
            }
            0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
                if !tonumeral(v, std::ptr::null_mut()) {
                    luak_exp2anyreg(fs, v);
                }
            }
            13 | 16 => {
                if !tonumeral(v, std::ptr::null_mut()) {
                    exp2rk(fs, v);
                }
            }
            14 | 15 | 17 | 18 => {
                let mut dummy: i32 = 0;
                let mut dummy2: bool = false;
                if is_sc_number(v, &mut dummy, &mut dummy2) == 0 {
                    luak_exp2anyreg(fs, v);
                }
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn codeconcat(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let ie2: *mut u32 = previousinstruction(fs);
        if (*ie2 >> 0 & !(!(0u32) << 7) << 0) as u32 == OP_CONCAT as u32 {
            let n: i32 = (*ie2 >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
            freeexp(fs, e2);
            *ie2 = *ie2 & !(!(!(0u32) << 8) << 0 + 7)
                | ((*e1).value.info as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
            *ie2 = *ie2 & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1)
                | ((n + 1) as u32) << 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
        } else {
            luak_code_abck(fs, OP_CONCAT, (*e1).value.info, 2, 0, 0);
            freeexp(fs, e2);
            luak_fixline(fs, line);
        };
    }
}
pub unsafe extern "C" fn luak_posfix(
    fs: *mut FunctionState,
    mut opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        luak_dischargevars(fs, e2);
        if opr as u32 <= OPR_SHR as u32
            && constfolding(fs, (opr as u32).wrapping_add(0u32) as i32, e1, e2) != 0
        {
            return;
        }
        let current_block_30: u64;
        match opr as u32 {
            19 => {
                luak_concat(fs, &mut (*e2).f, (*e1).f);
                *e1 = *e2;
                current_block_30 = 8180496224585318153;
            }
            20 => {
                luak_concat(fs, &mut (*e2).t, (*e1).t);
                *e1 = *e2;
                current_block_30 = 8180496224585318153;
            }
            12 => {
                luak_exp2nextreg(fs, e2);
                codeconcat(fs, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            0 | 2 => {
                codecommutative(fs, opr, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            1 => {
                if finishbinexpneg(fs, e1, e2, OP_ADDI, line, TM_SUB) != 0 {
                    current_block_30 = 8180496224585318153;
                } else {
                    current_block_30 = 12599329904712511516;
                }
            }
            5 | 6 | 3 | 4 => {
                current_block_30 = 12599329904712511516;
            }
            7 | 8 | 9 => {
                codebitwise(fs, opr, e1, e2, line);
                current_block_30 = 8180496224585318153;
            }
            10 => {
                if is_sc_int(e1) {
                    swapexps(e1, e2);
                    codebini(fs, OP_SHLI, e1, e2, 1, line, TM_SHL);
                } else if !(finishbinexpneg(fs, e1, e2, OP_SHRI, line, TM_SHL) != 0) {
                    codebinexpval(fs, opr, e1, e2, line);
                }
                current_block_30 = 8180496224585318153;
            }
            11 => {
                if is_sc_int(e2) {
                    codebini(fs, OP_SHRI, e1, e2, 0, line, TM_SHR);
                } else {
                    codebinexpval(fs, opr, e1, e2, line);
                }
                current_block_30 = 8180496224585318153;
            }
            13 | 16 => {
                codeeq(fs, opr, e1, e2);
                current_block_30 = 8180496224585318153;
            }
            17 | 18 => {
                swapexps(e1, e2);
                opr = (opr as u32)
                    .wrapping_sub(OPR_GT as u32)
                    .wrapping_add(OPR_LT as u32) as u32;
                current_block_30 = 1118134448028020070;
            }
            14 | 15 => {
                current_block_30 = 1118134448028020070;
            }
            _ => {
                current_block_30 = 8180496224585318153;
            }
        }
        match current_block_30 {
            12599329904712511516 => {
                codearith(fs, opr, e1, e2, 0, line);
            }
            1118134448028020070 => {
                codeorder(fs, opr, e1, e2);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luak_fixline(fs: *mut FunctionState, line: i32) {
    unsafe {
        removelastlineinfo(fs);
        savelineinfo(fs, (*fs).prototype, line);
    }
}
pub unsafe extern "C" fn luak_settablesize(
    fs: *mut FunctionState,
    program_counter: i32,
    ra: i32,
    asize: i32,
    hsize: i32,
) {
    unsafe {
        let inst: *mut u32 = &mut *((*(*fs).prototype).code).offset(program_counter as isize) as *mut u32;
        let rb: i32 = if hsize != 0 {
            ceiling_log2(hsize as u64) as i32 + 1
        } else {
            0
        };
        let extra: i32 = asize / ((1 << 8) - 1 + 1);
        let rc: i32 = asize % ((1 << 8) - 1 + 1);
        let k: i32 = (extra > 0) as i32;
        *inst = (OP_NEWTABLE as u32) << 0
            | (ra as u32) << 0 + 7
            | (rb as u32) << 0 + 7 + 8 + 1
            | (rc as u32) << 0 + 7 + 8 + 1 + 8
            | (k as u32) << 0 + 7 + 8;
        *inst.offset(1 as isize) = (OP_EXTRAARG as u32) << 0 | (extra as u32) << 0 + 7;
    }
}
pub unsafe extern "C" fn luak_setlist(
    fs: *mut FunctionState,
    base: i32,
    mut count_elements: i32,
    mut tostore: i32,
) {
    unsafe {
        if tostore == -1 {
            tostore = 0;
        }
        if count_elements <= (1 << 8) - 1 {
            luak_code_abck(fs, OP_SETLIST, base, tostore, count_elements, 0);
        } else {
            let extra: i32 = count_elements / ((1 << 8) - 1 + 1);
            count_elements %= (1 << 8) - 1 + 1;
            luak_code_abck(fs, OP_SETLIST, base, tostore, count_elements, 1);
            codeextraarg(fs, extra);
        }
        (*fs).freereg = (base + 1) as u8;
    }
}
pub unsafe extern "C" fn luak_finish(fs: *mut FunctionState) {
    unsafe {
        let mut i: i32;
        let p: *mut Prototype = (*fs).prototype;
        i = 0;
        while i < (*fs).program_counter {
            let program_counter: *mut u32 = &mut *((*p).code).offset(i as isize) as *mut u32;
            let current_block_7: u64;
            match (*program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 {
                71 | 72 => {
                    if !((*fs).needs_close || (*p).is_variable_arguments as i32 != 0) {
                        current_block_7 = 12599329904712511516;
                    } else {
                        *program_counter = *program_counter & !(!(!(0u32) << 7) << 0)
                            | (OP_RETURN as u32) << 0 & !(!(0u32) << 7) << 0;
                        current_block_7 = 11006700562992250127;
                    }
                }
                70 | 69 => {
                    current_block_7 = 11006700562992250127;
                }
                56 => {
                    let target: i32 = finaltarget((*p).code, i);
                    fixjump(fs, i, target);
                    current_block_7 = 12599329904712511516;
                }
                _ => {
                    current_block_7 = 12599329904712511516;
                }
            }
            match current_block_7 {
                11006700562992250127 => {
                    if (*fs).needs_close {
                        *program_counter = *program_counter & !(!(!(0u32) << 1) << 0 + 7 + 8)
                            | (1 as u32) << 0 + 7 + 8 & !(!(0u32) << 1) << 0 + 7 + 8;
                    }
                    if (*p).is_variable_arguments {
                        *program_counter = *program_counter
                            & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
                            | (((*p).count_parameters as i32 + 1) as u32) << 0 + 7 + 8 + 1 + 8
                                & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

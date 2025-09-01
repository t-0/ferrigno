use crate::blockcontrol::*;
use crate::lexicalstate::*;
use crate::labeldescription::*;
use crate::instruction::*;
use crate::constructorcontrol::*;
use crate::v::*;
use crate::labellist::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::state::*;
use crate::variabledescription::*;
use crate::absolutelineinfo::*;
use crate::localvariable::*;
use crate::expressiondescription::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::f2i::*;
use crate::tag::*;
use crate::object::*;
use crate::onelua::*;
use crate::new::*;
use crate::table::*;
use crate::prototype::*;
use crate::value::*;
use libc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub f: *mut Prototype,
    pub previous: *mut FunctionState,
    pub lexical_state: *mut LexicalState,
    pub block_control: *mut BlockControl,
    pub program_counter: i32,
    pub lasttarget: i32,
    pub previousline: i32,
    pub nk: i32,
    pub np: i32,
    pub nabslineinfo: i32,
    pub firstlocal: i32,
    pub first_label: i32,
    pub ndebugvars: i16,
    pub count_active_variables: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
}
impl New for FunctionState {
    fn new() -> Self {
        return FunctionState {
            f: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
            lexical_state: std::ptr::null_mut(),
            block_control: std::ptr::null_mut(),
            program_counter: 0,
            lasttarget: 0,
            previousline: 0,
            nk: 0,
            np: 0,
            nabslineinfo: 0,
            firstlocal: 0,
            first_label: 0,
            ndebugvars: 0,
            count_active_variables: 0,
            nups: 0,
            freereg: 0,
            iwthabs: 0,
            needclose: 0,
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
        let mut hasclose: i32 = 0;
        let stklevel: i32 = reglevel(fs, (*block_control).count_active_variables as i32);
        removevars(fs, (*block_control).count_active_variables as i32);
        if (*block_control).is_loop {
            hasclose = createlabel(
                lexical_state,
                luas_newlstr(
                    (*lexical_state).state,
                    b"break\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ),
                0,
                0,
            );
        }
        if hasclose == 0
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
        if (*cc).v.k as u32 == VVOID as u32 {
            return;
        }
        luak_exp2nextreg(fs, &mut (*cc).v);
        (*cc).v.k = VVOID;
        if (*cc).tostore == 50 as i32 {
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
            (*cc).na += (*cc).tostore;
            (*cc).tostore = 0;
        }
    }
}
pub unsafe extern "C" fn lastlistfield(fs: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).tostore == 0 {
            return;
        }
        if (*cc).v.k as u32 == VCALL as u32 || (*cc).v.k as u32 == VVARARG as u32 {
            luak_setreturns(fs, &mut (*cc).v, -1);
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, -1);
            (*cc).na -= 1;
            (*cc).na;
        } else {
            if (*cc).v.k as u32 != VVOID as u32 {
                luak_exp2nextreg(fs, &mut (*cc).v);
            }
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        }
        (*cc).na += (*cc).tostore;
    }
}
pub unsafe extern "C" fn setvararg(fs: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*fs).f).is_variable_arguments = true;
        luak_code_abck(fs, OP_VARARGPREP, nparams, 0, 0, 0);
    }
}
pub unsafe extern "C" fn errorlimit(fs: *mut FunctionState, limit: i32, what: *const i8) -> ! {
    unsafe {
        let state: *mut State = (*(*fs).lexical_state).state;
        let message: *const i8;
        let line: i32 = (*(*fs).f).line_defined;
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
            .offset(((*fs).firstlocal + vidx) as isize) as *mut VariableDescription;
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
            if (*vd).vd.kind as i32 != 3 {
                return (*vd).vd.ridx as i32 + 1;
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
        if (*vd).vd.kind as i32 == 3 {
            return std::ptr::null_mut();
        } else {
            let index: i32 = (*vd).vd.pidx as i32;
            return &mut *((*(*fs).f).local_variables).offset(index as isize) as *mut LocalVariable;
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
        (*e).k = VLOCAL;
        (*e).u.var.vidx = vidx as u16;
        (*e).u.var.ridx = (*getlocalvardesc(fs, vidx)).vd.ridx;
    }
}
pub unsafe extern "C" fn removevars(fs: *mut FunctionState, tolevel: i32) {
    unsafe {
        (*(*(*fs).lexical_state).dynamic_data).active_variable.n -=
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
        let up: *mut UpValueDescription = (*(*fs).f).upvalues;
        i = 0;
        while i < (*fs).nups as i32 {
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
        let f: *mut Prototype = (*fs).f;
        let mut old_size: i32 = (*f).size_upvalues;
        checklimit(
            fs,
            (*fs).nups as i32 + 1,
            255 as i32,
            b"upvalues\0" as *const u8 as *const i8,
        );
        (*f).upvalues = luam_growaux_(
            (*(*fs).lexical_state).state,
            (*f).upvalues as *mut libc::c_void,
            (*fs).nups as i32,
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
        let fresh43 = (*fs).nups;
        (*fs).nups = ((*fs).nups).wrapping_add(1);
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
        if (*v).k as u32 == VLOCAL as u32 {
            (*up).is_in_stack = true;
            (*up).index = (*v).u.var.ridx;
            (*up).kind = (*getlocalvardesc(previous, (*v).u.var.vidx as i32)).vd.kind;
        } else {
            (*up).is_in_stack = false;
            (*up).index = (*v).u.info as u8;
            (*up).kind = (*((*(*previous).f).upvalues).offset((*v).u.info as isize)).kind;
        }
        (*up).name = name;
        if (*(*fs).f).get_marked() & 1 << 5 != 0 && (*name).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                (*(*fs).lexical_state).state,
                &mut (*((*fs).f as *mut Object)),
                &mut (*(name as *mut Object)),
            );
        } else {
        };
        return (*fs).nups as i32 - 1;
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
            if n == (*vd).vd.name {
                if (*vd).vd.kind as i32 == 3 {
                    init_exp(var, VCONST, (*fs).firstlocal + i);
                } else {
                    init_var(fs, var, i);
                }
                return (*var).k as i32;
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
        (*fs).needclose = 1;
    }
}
pub unsafe extern "C" fn marktobeclosed(fs: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*fs).block_control;
        (*block_control).count_upvalues = 1;
        (*block_control).is_inside_tbc = true;
        (*fs).needclose = 1;
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
            init_exp(var, VVOID, 0);
        } else {
            let v: i32 = searchvar(fs, n, var);
            if v >= 0 {
                if v == VLOCAL as i32 && base == 0 {
                    markupval(fs, (*var).u.var.vidx as i32);
                }
            } else {
                let mut index: i32 = searchupvalue(fs, n);
                if index < 0 {
                    singlevaraux((*fs).previous, n, var, 0);
                    if (*var).k as u32 == VLOCAL as u32
                        || (*var).k as u32 == VUPVAL as u32
                    {
                        index = newupvalue(fs, n, var);
                    } else {
                        return;
                    }
                }
                init_exp(var, VUPVAL, index);
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
        let jmp: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
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
        if (*fs).program_counter > (*fs).lasttarget {
            return &mut *((*(*fs).f).code).offset(((*fs).program_counter - 1) as isize)
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
        let offset: i32 = (*((*(*fs).f).code).offset(program_counter as isize) >> 0 + 7
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
        let jmp: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
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
        (*fs).lasttarget = (*fs).program_counter;
        return (*fs).program_counter;
    }
}
pub unsafe extern "C" fn getjumpcontrol(fs: *mut FunctionState, program_counter: i32) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
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
        let mut linedif: i32 = line - (*fs).previousline;
        let program_counter: i32 = (*fs).program_counter - 1;
        if abs(linedif) >= 0x80 as i32 || {
            let fresh132 = (*fs).iwthabs;
            (*fs).iwthabs = ((*fs).iwthabs).wrapping_add(1);
            fresh132 as i32 >= 128 as i32
        } {
            (*f).absolute_line_info = luam_growaux_(
                (*(*fs).lexical_state).state,
                (*f).absolute_line_info as *mut libc::c_void,
                (*fs).nabslineinfo,
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
            (*((*f).absolute_line_info).offset((*fs).nabslineinfo as isize)).program_counter =
                program_counter;
            let fresh133 = (*fs).nabslineinfo;
            (*fs).nabslineinfo = (*fs).nabslineinfo + 1;
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
        (*fs).previousline = line;
    }
}
pub unsafe extern "C" fn removelastlineinfo(fs: *mut FunctionState) {
    unsafe {
        let f: *mut Prototype = (*fs).f;
        let program_counter: i32 = (*fs).program_counter - 1;
        if *((*f).line_info).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            (*fs).previousline -= *((*f).line_info).offset(program_counter as isize) as i32;
            (*fs).iwthabs = ((*fs).iwthabs).wrapping_sub(1);
            (*fs).iwthabs;
        } else {
            (*fs).nabslineinfo -= 1;
            (*fs).nabslineinfo;
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
        let f: *mut Prototype = (*fs).f;
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
        if newstack > (*(*fs).f).maximum_stack_size as i32 {
            if newstack >= 255 as i32 {
                luax_syntaxerror(
                    (*fs).lexical_state,
                    b"function or expression needs too many registers\0" as *const u8 as *const i8,
                );
            }
            (*(*fs).f).maximum_stack_size = newstack as u8;
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
        if (*e).k as u32 == VNONRELOC as u32 {
            freereg(fs, (*e).u.info);
        }
    }
}
pub unsafe extern "C" fn freeexps(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32 = if (*e1).k as u32 == VNONRELOC as u32 {
            (*e1).u.info
        } else {
            -1
        };
        let r2: i32 = if (*e2).k as u32 == VNONRELOC as u32 {
            (*e2).u.info
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
        let f: *mut Prototype = (*fs).f;
        let index: *const TValue = luah_get((*(*fs).lexical_state).h, key);
        let mut k: i32;
        if (*index).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            k = (*index).value.i as i32;
            if k < (*fs).nk
                && (*((*f).k).offset(k as isize)).get_tag_variant() == (*v).get_tag_variant()
                && luav_equalobj(std::ptr::null_mut(), &mut *((*f).k).offset(k as isize), v)
            {
                return k;
            }
        }
        let mut old_size: i32 = (*f).size_k;
        k = (*fs).nk;
        let io: *mut TValue = &mut value;
        (*io).value.i = k as i64;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        luah_finishset(state, (*(*fs).lexical_state).h, key, index, &mut value);
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
        (*fs).nk += 1;
        (*fs).nk;
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
        (*io).value.i = n;
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
        (*io).value.n = r;
        (*io).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        if luav_flttointeger(r, &mut ik, F2I::Equal) == 0 {
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
            (*io_0).value.n = k;
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
        let x_: *mut Table = (*(*fs).lexical_state).h;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_TABLE);
        (*io).set_collectable();
        return addk(fs, &mut k, &mut v);
    }
}

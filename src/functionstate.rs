use crate::blockcontrol::*;
use crate::lexicalstate::*;
use crate::labeldescription::*;
use crate::instruction::*;
use crate::constructorcontrol::*;
use crate::v::*;
use crate::labellist::*;
use crate::tstring::*;
use crate::state::*;
use crate::variabledescription::*;
use crate::localvariable::*;
use crate::expressiondescription::*;
use crate::upvaluedescription::*;
use crate::object::*;
use crate::onelua::*;
use crate::new::*;
use crate::prototype::*;
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

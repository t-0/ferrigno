use crate::buffer::*;
use crate::dynamicdata::*;
use crate::labeldescription::*;
use crate::character::*;
use crate::labellist::*;
use crate::instruction::*;
use crate::functionstate::*;
use crate::new::*;
use crate::object::*;
use crate::onelua::*;
use crate::prototype::*;
use crate::state::*;
use crate::blockcontrol::*;
use crate::absolutelineinfo::*;
use crate::tvalue::*;
use crate::expressiondescription::*;
use crate::rawvalue::*;
use crate::v::*;
use crate::constructorcontrol::*;
use crate::upvaldesc::*;
use crate::localvariable::*;
use crate::table::*;
use crate::token::*;
use crate::tstring::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexicalState {
    pub current: i32,
    pub line_number: i32,
    pub last_line: i32,
    pub t: Token,
    pub look_ahead: Token,
    pub fs: *mut FunctionState,
    pub state: *mut State,
    pub zio: *mut ZIO,
    pub buffer: *mut Buffer,
    pub h: *mut Table,
    pub dynamic_data: *mut DynamicData,
    pub source: *mut TString,
    pub envn: *mut TString,
}
impl New for LexicalState {
    fn new() -> Self {
        return LexicalState {
            current: 0,
            line_number: 0,
            last_line: 0,
            t: Token::new(),
            look_ahead: Token::new(),
            fs: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
            zio: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            h: std::ptr::null_mut(),
            dynamic_data: std::ptr::null_mut(),
            source: std::ptr::null_mut(),
            envn: std::ptr::null_mut(),
        };
    }
}
impl LexicalState {
    pub unsafe extern "C" fn add_prototype(&mut self) -> *mut Prototype {
        unsafe {
            let fs: *mut FunctionState = self.fs;
            let f: *mut Prototype = (*fs).f;
            if (*fs).np >= (*f).size_p {
                let mut old_size: i32 = (*f).size_p;
                (*f).p = luam_growaux_(
                    self.state,
                    (*f).p as *mut libc::c_void,
                    (*fs).np,
                    &mut (*f).size_p,
                    ::core::mem::size_of::<*mut Prototype>() as i32,
                    (if ((1 << 8 + 8 + 1) - 1) as u64
                        <= (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
                    {
                        ((1 << 8 + 8 + 1) - 1) as u32
                    } else {
                        (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
                            as u32
                    }) as i32,
                    b"functions\0" as *const u8 as *const i8,
                ) as *mut *mut Prototype;
                while old_size < (*f).size_p {
                    let fresh45 = old_size;
                    old_size = old_size + 1;
                    let ref mut fresh46 = *((*f).p).offset(fresh45 as isize);
                    *fresh46 = std::ptr::null_mut();
                }
            }
            let clp: *mut Prototype = luaf_newproto(self.state);
            let np = (*fs).np;
            (*fs).np = (*fs).np + 1;
            let ref mut target = *((*f).p).offset(np as isize);
            *target = clp;
            if (*f).get_marked() & 1 << 5 != 0 && (*clp).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    self.state,
                    &mut (*(f as *mut Object)),
                    &mut (*(clp as *mut Object)),
                );
            } else {
            };
            return clp;
        }
    }
}
pub unsafe extern "C" fn findlabel(
    lexical_state: *mut LexicalState,
    name: *mut TString,
) -> *mut LabelDescription {
    unsafe {
        let mut i: i32;
        let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
        i = (*(*lexical_state).fs).first_label;
        while i < (*dynamic_data).label.n {
            let lb: *mut LabelDescription =
                &mut *((*dynamic_data).label.pointer).offset(i as isize) as *mut LabelDescription;
            if (*lb).name == name {
                return lb;
            }
            i += 1;
        }
        return std::ptr::null_mut();
    }
}
pub unsafe extern "C" fn newlabelentry(
    lexical_state: *mut LexicalState,
    l: *mut LabelList,
    name: *mut TString,
    line: i32,
    program_counter: i32,
) -> i32 {
    unsafe {
        let n: i32 = (*l).n;
        (*l).pointer = luam_growaux_(
            (*lexical_state).state,
            (*l).pointer as *mut libc::c_void,
            n,
            &mut (*l).size,
            ::core::mem::size_of::<LabelDescription>() as i32,
            (if 32767 as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<LabelDescription>() as u64)
            {
                32767 as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<LabelDescription>() as u64) as u32
            }) as i32,
            b"labels/gotos\0" as *const u8 as *const i8,
        ) as *mut LabelDescription;
        let ref mut fresh44 = (*((*l).pointer).offset(n as isize)).name;
        *fresh44 = name;
        (*((*l).pointer).offset(n as isize)).line = line;
        (*((*l).pointer).offset(n as isize)).count_active_variables =
            (*(*lexical_state).fs).count_active_variables;
        (*((*l).pointer).offset(n as isize)).close = 0;
        (*((*l).pointer).offset(n as isize)).program_counter = program_counter;
        (*l).n = n + 1;
        return n;
    }
}
pub unsafe extern "C" fn newgotoentry(
    lexical_state: *mut LexicalState,
    name: *mut TString,
    line: i32,
    program_counter: i32,
) -> i32 {
    unsafe {
        return newlabelentry(
            lexical_state,
            &mut (*(*lexical_state).dynamic_data).gt,
            name,
            line,
            program_counter,
        );
    }
}
pub unsafe extern "C" fn solvegotos(
    lexical_state: *mut LexicalState,
    lb: *mut LabelDescription,
) -> bool {
    unsafe {
        let gl: *mut LabelList = &mut (*(*lexical_state).dynamic_data).gt;
        let mut i: i32 = (*(*(*lexical_state).fs).block_control).first_goto;
        let mut needsclose = false;
        while i < (*gl).n {
            if (*((*gl).pointer).offset(i as isize)).name == (*lb).name {
                needsclose = needsclose || (0 != (*((*gl).pointer).offset(i as isize)).close);
                solvegoto(lexical_state, i, lb);
            } else {
                i += 1;
            }
        }
        return needsclose;
    }
}
pub unsafe extern "C" fn createlabel(
    lexical_state: *mut LexicalState,
    name: *mut TString,
    line: i32,
    last: i32,
) -> i32 {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let ll: *mut LabelList = &mut (*(*lexical_state).dynamic_data).label;
        let l: i32 = newlabelentry(lexical_state, ll, name, line, luak_getlabel(fs));
        if last != 0 {
            (*((*ll).pointer).offset(l as isize)).count_active_variables =
                (*(*fs).block_control).count_active_variables;
        }
        if solvegotos(lexical_state, &mut *((*ll).pointer).offset(l as isize)) {
            luak_code_abck(fs, OP_CLOSE, luay_nvarstack(fs), 0, 0, 0);
            return 1;
        }
        return 0;
    }
}
pub unsafe extern "C" fn undefgoto(
    lexical_state: *mut LexicalState,
    gt: *mut LabelDescription,
) -> ! {
    unsafe {
        let mut message: *const i8;
        if (*gt).name
            == luas_newlstr(
                (*lexical_state).state,
                b"break\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            )
        {
            message = b"break outside loop at line %d\0" as *const u8 as *const i8;
            message = luao_pushfstring((*lexical_state).state, message, (*gt).line);
        } else {
            message = b"no visible label '%s' for <goto> at line %d\0" as *const u8 as *const i8;
            message = luao_pushfstring(
                (*lexical_state).state,
                message,
                (*(*gt).name).get_contents(),
                (*gt).line,
            );
        }
        luak_semerror(lexical_state, message);
    }
}
pub unsafe extern "C" fn codeclosure(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) {
    unsafe {
        let fs: *mut FunctionState = (*(*lexical_state).fs).previous;
        init_exp(
            v,
            VRELOC,
            luak_codeabx(fs, OP_CLOSURE, 0, ((*fs).np - 1) as u32),
        );
        luak_exp2nextreg(fs, v);
    }
}
pub unsafe extern "C" fn open_func(
    lexical_state: *mut LexicalState,
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
) {
    unsafe {
        let f: *mut Prototype = (*fs).f;
        (*fs).previous = (*lexical_state).fs;
        (*fs).lexical_state = lexical_state;
        (*lexical_state).fs = fs;
        (*fs).program_counter = 0;
        (*fs).previousline = (*f).line_defined;
        (*fs).iwthabs = 0;
        (*fs).lasttarget = 0;
        (*fs).freereg = 0;
        (*fs).nk = 0;
        (*fs).nabslineinfo = 0;
        (*fs).np = 0;
        (*fs).nups = 0;
        (*fs).ndebugvars = 0 as i16;
        (*fs).count_active_variables = 0;
        (*fs).needclose = 0;
        (*fs).firstlocal = (*(*lexical_state).dynamic_data).active_variable.n;
        (*fs).first_label = (*(*lexical_state).dynamic_data).label.n;
        (*fs).block_control = std::ptr::null_mut();
        (*f).source = (*lexical_state).source;
        if (*f).get_marked() & 1 << 5 != 0 && (*(*f).source).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                (*lexical_state).state,
                &mut (*(f as *mut Object)),
                &mut (*((*f).source as *mut Object)),
            );
        } else {
        };
        (*f).maximum_stack_size = 2 as u8;
        enterblock(fs, block_control, false);
    }
}
pub unsafe extern "C" fn close_func(lexical_state: *mut LexicalState) {
    unsafe {
        let state: *mut State = (*lexical_state).state;
        let fs: *mut FunctionState = (*lexical_state).fs;
        let f: *mut Prototype = (*fs).f;
        luak_ret(fs, luay_nvarstack(fs), 0);
        leaveblock(fs);
        luak_finish(fs);
        (*f).code = luam_shrinkvector_(
            state,
            (*f).code as *mut libc::c_void,
            &mut (*f).size_code,
            (*fs).program_counter,
            ::core::mem::size_of::<u32>() as i32,
        ) as *mut u32;
        (*f).line_info = luam_shrinkvector_(
            state,
            (*f).line_info as *mut libc::c_void,
            &mut (*f).size_line_info,
            (*fs).program_counter,
            ::core::mem::size_of::<i8>() as i32,
        ) as *mut i8;
        (*f).absolute_line_info = luam_shrinkvector_(
            state,
            (*f).absolute_line_info as *mut libc::c_void,
            &mut (*f).size_absolute_line_info,
            (*fs).nabslineinfo,
            ::core::mem::size_of::<AbsoluteLineInfo>() as i32,
        ) as *mut AbsoluteLineInfo;
        (*f).k = luam_shrinkvector_(
            state,
            (*f).k as *mut libc::c_void,
            &mut (*f).size_k,
            (*fs).nk,
            ::core::mem::size_of::<TValue>() as i32,
        ) as *mut TValue;
        (*f).p = luam_shrinkvector_(
            state,
            (*f).p as *mut libc::c_void,
            &mut (*f).size_p,
            (*fs).np,
            ::core::mem::size_of::<*mut Prototype>() as i32,
        ) as *mut *mut Prototype;
        (*f).local_variables = luam_shrinkvector_(
            state,
            (*f).local_variables as *mut libc::c_void,
            &mut (*f).size_local_variables,
            (*fs).ndebugvars as i32,
            ::core::mem::size_of::<LocalVariable>() as i32,
        ) as *mut LocalVariable;
        (*f).upvalues = luam_shrinkvector_(
            state,
            (*f).upvalues as *mut libc::c_void,
            &mut (*f).size_upvalues,
            (*fs).nups as i32,
            ::core::mem::size_of::<Upvaldesc>() as i32,
        ) as *mut Upvaldesc;
        (*lexical_state).fs = (*fs).previous;
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
    }
}
pub unsafe extern "C" fn block_follow(lexical_state: *mut LexicalState, withuntil: i32) -> i32 {
    unsafe {
        match (*lexical_state).t.token {
            259 | 260 | 261 | 288 => return 1,
            276 => return withuntil,
            _ => return 0,
        };
    }
}
pub unsafe extern "C" fn statlist(lexical_state: *mut LexicalState) {
    unsafe {
        while block_follow(lexical_state, 1) == 0 {
            if (*lexical_state).t.token == TK_RETURN as i32 {
                statement(lexical_state);
                return;
            }
            statement(lexical_state);
        }
    }
}
pub unsafe extern "C" fn fieldsel(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let mut key: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        luak_exp2anyregup(fs, v);
        luax_next(lexical_state);
        codename(lexical_state, &mut key);
        luak_indexed(fs, v, &mut key);
    }
}
pub unsafe extern "C" fn yindex(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        luax_next(lexical_state);
        expr(lexical_state, v);
        luak_exp2val((*lexical_state).fs, v);
        checknext(lexical_state, ']' as i32);
    }
}
pub unsafe extern "C" fn recfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let reg: i32 = (*(*lexical_state).fs).freereg as i32;
        let mut key: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        let mut value: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        if (*lexical_state).t.token == TK_NAME as i32 {
            codename(lexical_state, &mut key);
        } else {
            yindex(lexical_state, &mut key);
        }
        checklimit(
            fs,
            (*cc).nh,
            0x7FFFFFFF as i32,
            b"items in a constructor\0" as *const u8 as *const i8,
        );
        (*cc).nh += 1;
        (*cc).nh;
        checknext(lexical_state, '=' as i32);
        let mut tab: ExpressionDescription = *(*cc).t;
        luak_indexed(fs, &mut tab, &mut key);
        expr(lexical_state, &mut value);
        luak_storevar(fs, &mut tab, &mut value);
        (*fs).freereg = reg as u8;
    }
}
pub unsafe extern "C" fn listfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        expr(lexical_state, &mut (*cc).v);
        (*cc).tostore += 1;
        (*cc).tostore;
    }
}
pub unsafe extern "C" fn field(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        match (*lexical_state).t.token {
            291 => {
                if luax_lookahead(lexical_state) != '=' as i32 {
                    listfield(lexical_state, cc);
                } else {
                    recfield(lexical_state, cc);
                }
            }
            91 => {
                recfield(lexical_state, cc);
            }
            _ => {
                listfield(lexical_state, cc);
            }
        };
    }
}
pub unsafe extern "C" fn constructor(
    lexical_state: *mut LexicalState,
    t: *mut ExpressionDescription,
) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let line: i32 = (*lexical_state).line_number;
        let program_counter: i32 = luak_code_abck(fs, OP_NEWTABLE, 0, 0, 0, 0);
        let mut cc: ConstructorControl = ConstructorControl {
            v: ExpressionDescription {
                k: VVOID,
                u: RawValue { ival: 0 },
                t: 0,
                f: 0,
            },
            t: std::ptr::null_mut(),
            nh: 0,
            na: 0,
            tostore: 0,
        };
        luak_code(fs, 0u32);
        cc.tostore = 0;
        cc.nh = cc.tostore;
        cc.na = cc.nh;
        cc.t = t;
        init_exp(t, VNONRELOC, (*fs).freereg as i32);
        luak_reserveregs(fs, 1);
        init_exp(&mut cc.v, VVOID, 0);
        checknext(lexical_state, '{' as i32);
        while !((*lexical_state).t.token == '}' as i32) {
            closelistfield(fs, &mut cc);
            field(lexical_state, &mut cc);
            if !(testnext(lexical_state, ',' as i32) != 0
                || testnext(lexical_state, ';' as i32) != 0)
            {
                break;
            }
        }
        check_match(lexical_state, '}' as i32, '{' as i32, line);
        lastlistfield(fs, &mut cc);
        luak_settablesize(fs, program_counter, (*t).u.info, cc.na, cc.nh);
    }
}
pub unsafe extern "C" fn parlist(lexical_state: *mut LexicalState) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let f: *mut Prototype = (*fs).f;
        let mut nparams: i32 = 0;
        let mut is_variable_arguments = false;
        if (*lexical_state).t.token != ')' as i32 {
            loop {
                match (*lexical_state).t.token {
                    291 => {
                        new_localvar(lexical_state, str_checkname(lexical_state));
                        nparams += 1;
                    }
                    280 => {
                        luax_next(lexical_state);
                        is_variable_arguments = true;
                    }
                    _ => {
                        luax_syntaxerror(
                            lexical_state,
                            b"<name> or '...' expected\0" as *const u8 as *const i8,
                        );
                    }
                }
                if !(!is_variable_arguments && testnext(lexical_state, ',' as i32) != 0) {
                    break;
                }
            }
        }
        adjustlocalvars(lexical_state, nparams);
        (*f).count_parameters = (*fs).count_active_variables;
        if is_variable_arguments {
            setvararg(fs, (*f).count_parameters as i32);
        }
        luak_reserveregs(fs, (*fs).count_active_variables as i32);
    }
}
pub unsafe extern "C" fn body(
    lexical_state: *mut LexicalState,
    e: *mut ExpressionDescription,
    ismethod: i32,
    line: i32,
) {
    unsafe {
        let mut new_fs: FunctionState = FunctionState {
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
        let mut block_control = BlockControl::new();
        new_fs.f = (*lexical_state).add_prototype();
        (*new_fs.f).line_defined = line;
        open_func(lexical_state, &mut new_fs, &mut block_control);
        checknext(lexical_state, '(' as i32);
        if ismethod != 0 {
            new_localvar(
                lexical_state,
                luax_newstring(
                    lexical_state,
                    b"self\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ),
            );
            adjustlocalvars(lexical_state, 1);
        }
        parlist(lexical_state);
        checknext(lexical_state, ')' as i32);
        statlist(lexical_state);
        (*new_fs.f).last_line_defined = (*lexical_state).line_number;
        check_match(lexical_state, TK_END as i32, TK_FUNCTION as i32, line);
        codeclosure(lexical_state, e);
        close_func(lexical_state);
    }
}
pub unsafe extern "C" fn explist(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let mut n: i32 = 1;
        expr(lexical_state, v);
        while testnext(lexical_state, ',' as i32) != 0 {
            luak_exp2nextreg((*lexical_state).fs, v);
            expr(lexical_state, v);
            n += 1;
        }
        return n;
    }
}
pub unsafe extern "C" fn funcargs(lexical_state: *mut LexicalState, f: *mut ExpressionDescription) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        let mut args: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        let line: i32 = (*lexical_state).line_number;
        match (*lexical_state).t.token {
            CHARACTER_PARENTHESIS_LEFT => {
                luax_next(lexical_state);
                if (*lexical_state).t.token == ')' as i32 {
                    args.k = VVOID;
                } else {
                    explist(lexical_state, &mut args);
                    if args.k as u32 == VCALL as u32
                        || args.k as u32 == VVARARG as u32
                    {
                        luak_setreturns(fs, &mut args, -1);
                    }
                }
                check_match(lexical_state, ')' as i32, '(' as i32, line);
            }
            CHARACTER_BRACE_LEFT => {
                constructor(lexical_state, &mut args);
            }
            292 => {
                codestring(&mut args, (*lexical_state).t.semantic_info.ts);
                luax_next(lexical_state);
            }
            _ => {
                luax_syntaxerror(
                    lexical_state,
                    b"function arguments expected\0" as *const u8 as *const i8,
                );
            }
        }
        let base: i32 = (*f).u.info;
        let nparams: i32;
        if args.k as u32 == VCALL as u32 || args.k as u32 == VVARARG as u32 {
            nparams = -1;
        } else {
            if args.k as u32 != VVOID as u32 {
                luak_exp2nextreg(fs, &mut args);
            }
            nparams = (*fs).freereg as i32 - (base + 1);
        }
        init_exp(
            f,
            VCALL,
            luak_code_abck(fs, OP_CALL, base, nparams + 1, 2, 0),
        );
        luak_fixline(fs, line);
        (*fs).freereg = (base + 1) as u8;
    }
}
pub unsafe extern "C" fn primaryexp(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) {
    unsafe {
        match (*lexical_state).t.token {
            40 => {
                let line: i32 = (*lexical_state).line_number;
                luax_next(lexical_state);
                expr(lexical_state, v);
                check_match(lexical_state, ')' as i32, '(' as i32, line);
                luak_dischargevars((*lexical_state).fs, v);
                return;
            }
            291 => {
                singlevar(lexical_state, v);
                return;
            }
            _ => {
                luax_syntaxerror(
                    lexical_state,
                    b"unexpected symbol\0" as *const u8 as *const i8,
                );
            }
        };
    }
}
pub unsafe extern "C" fn suffixedexp(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) {
    unsafe {
        let fs: *mut FunctionState = (*lexical_state).fs;
        primaryexp(lexical_state, v);
        loop {
            match (*lexical_state).t.token {
                46 => {
                    fieldsel(lexical_state, v);
                }
                91 => {
                    let mut key: ExpressionDescription = ExpressionDescription {
                        k: VVOID,
                        u: RawValue { ival: 0 },
                        t: 0,
                        f: 0,
                    };
                    luak_exp2anyregup(fs, v);
                    yindex(lexical_state, &mut key);
                    luak_indexed(fs, v, &mut key);
                }
                58 => {
                    let mut key_0: ExpressionDescription = ExpressionDescription {
                        k: VVOID,
                        u: RawValue { ival: 0 },
                        t: 0,
                        f: 0,
                    };
                    luax_next(lexical_state);
                    codename(lexical_state, &mut key_0);
                    luak_self(fs, v, &mut key_0);
                    funcargs(lexical_state, v);
                }
                40 | 292 | 123 => {
                    luak_exp2nextreg(fs, v);
                    funcargs(lexical_state, v);
                }
                _ => return,
            }
        }
    }
}
pub unsafe extern "C" fn simpleexp(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) {
    unsafe {
        match (*lexical_state).t.token {
            TK_FLT => {
                init_exp(v, VKFLT, 0);
                (*v).u.nval = (*lexical_state).t.semantic_info.r;
            }
            TK_INT => {
                init_exp(v, VKINT, 0);
                (*v).u.ival = (*lexical_state).t.semantic_info.i;
            }
            TK_STRING => {
                codestring(v, (*lexical_state).t.semantic_info.ts);
            }
            TK_NIL => {
                init_exp(v, VNIL, 0);
            }
            TK_TRUE => {
                init_exp(v, VTRUE, 0);
            }
            TK_FALSE => {
                init_exp(v, VFALSE, 0);
            }
            TK_DOTS => {
                let fs: *mut FunctionState = (*lexical_state).fs;
                if !(*(*fs).f).is_variable_arguments {
                    luax_syntaxerror(
                        lexical_state,
                        b"cannot use '...' outside a vararg function\0" as *const u8 as *const i8,
                    );
                }
                init_exp(v, VVARARG, luak_code_abck(fs, OP_VARARG, 0, 0, 1, 0));
            }
            123 => {
                constructor(lexical_state, v);
                return;
            }
            TK_FUNCTION => {
                luax_next(lexical_state);
                body(lexical_state, v, 0, (*lexical_state).line_number);
                return;
            }
            _ => {
                suffixedexp(lexical_state, v);
                return;
            }
        }
        luax_next(lexical_state);
    }
}

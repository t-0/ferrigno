use libc::*;
use crate::buffer::*;
use crate::utility::*;
use crate::dynamicdata::*;
use crate::node::*;
use crate::labeldescription::*;
use crate::character::*;
use crate::labellist::*;
use crate::expressionkind::*;
use crate::vm::opcode::*;
use crate::lexical::operatorunary::*;
use crate::operator_::*;
use crate::functionstate::*;
use crate::tag::*;
use crate::value::*;
use crate::lexical::priority::*;
use crate::lexical::operatorbinary::*;
use crate::lexical::lhsassign::*;
use crate::new::*;
use crate::variabledescription::*;
use crate::object::*;
use crate::prototype::*;
use crate::interpreter::*;
use crate::lexical::blockcontrol::*;
use crate::debugger::absolutelineinfo::*;
use crate::tvalue::*;
use crate::expressiondescription::*;
use crate::lexical::constructorcontrol::*;
use crate::upvaluedescription::*;
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
    pub token: Token,
    pub look_ahead: Token,
    pub function_state: *mut FunctionState,
    pub interpreter: *mut Interpreter,
    pub zio: *mut ZIO,
    pub buffer: *mut Buffer,
    pub table: *mut Table,
    pub dynamic_data: *mut DynamicData,
    pub source: *mut TString,
    pub environment: *mut TString,
}
impl New for LexicalState {
    fn new() -> Self {
        return LexicalState {
            current: 0,
            line_number: 0,
            last_line: 0,
            token: Token::new(),
            look_ahead: Token::new(),
            function_state: std::ptr::null_mut(),
            interpreter: std::ptr::null_mut(),
            zio: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            table: std::ptr::null_mut(),
            dynamic_data: std::ptr::null_mut(),
            source: std::ptr::null_mut(),
            environment: std::ptr::null_mut(),
        };
    }
}
impl LexicalState {
    pub unsafe fn parse_statement(& mut self) {
        unsafe {
            let line: i32 = self.line_number;
            (*(self.interpreter)).luae_inccstack();
            match self.token.token {
                CHARACTER_SEMICOLON => {
                    luax_next(self);
                }
                TK_IF => {
                    ifstat(self, line);
                }
                TK_WHILE => {
                    whilestat(self, line);
                }
                TK_DO => {
                    luax_next(self);
                    block(self);
                    check_match(self, TK_END as i32, TK_DO as i32, line);
                }
                TK_FOR => {
                    forstat(self, line);
                }
                TK_REPEAT => {
                    repeatstat(self, line);
                }
                TK_FUNCTION => {
                    funcstat(self, line);
                }
                TK_LOCAL => {
                    luax_next(self);
                    if testnext(self, TK_FUNCTION as i32) != 0 {
                        localfunc(self);
                    } else {
                        localstat(self);
                    }
                }
                TK_DBCOLON => {
                    luax_next(self);
                    labelstat(self, str_checkname(self), line);
                }
                TK_RETURN => {
                    luax_next(self);
                    retstat(self);
                }
                TK_BREAK => {
                    breakstat(self);
                }
                TK_GOTO => {
                    luax_next(self);
                    gotostat(self);
                }
                _ => {
                    exprstat(self);
                }
            }
            (*self.function_state).freereg = luay_nvarstack(self.function_state) as u8;
            (*self.interpreter).count_c_calls =
                (*self.interpreter).count_c_calls.wrapping_sub(1);
            (*self.interpreter).count_c_calls;
        }
    }
    pub unsafe fn create_label(
        & mut self,
        name: *mut TString,
        line: i32,
        is_last: bool,
    ) -> bool {
        unsafe {
            let function_state: *mut FunctionState = self.function_state;
            let ll: *mut LabelList = &mut (*self.dynamic_data).label;
            let l: i32 = newlabelentry(self, ll, name, line, luak_getlabel(function_state));
            if is_last {
                (*((*ll).pointer).offset(l as isize)).count_active_variables =
                    (*(*function_state).block_control).count_active_variables;
            }
            if solvegotos(self, &mut *((*ll).pointer).offset(l as isize)) {
                luak_code_abck(function_state, OP_CLOSE, luay_nvarstack(function_state), 0, 0, 0);
                return true;
            }
            return false;
        }
    }
    pub fn block_follow(& mut self, with_until: bool) -> bool {
        match self.token.token {
            TK_ELSE | TK_ELSEIF | TK_END | TK_EOS => return true,
            TK_UNTIL => return with_until,
            _ => return false,
        };
    }
    pub unsafe fn parse_expression(& mut self, expression_description: *mut ExpressionDescription) {
        unsafe {
            subexpr(self, expression_description, 0);
        }
    }
    pub unsafe fn parse_expression_list(
        &mut self,
        expression_description: *mut ExpressionDescription,
    ) -> i32 {
        unsafe {
            let mut count: i32 = 1;
            self.parse_expression(expression_description);
            while testnext(self, CHARACTER_COMMA as i32) != 0 {
                luak_exp2nextreg(self.function_state, expression_description);
                self.parse_expression(expression_description);
                count += 1;
            }
            return count;
        }
    }
    pub unsafe extern "C" fn add_prototype(&mut self) -> *mut Prototype {
        unsafe {
            let function_state: *mut FunctionState = self.function_state;
            let prototype: *mut Prototype = (*function_state).prototype;
            if (*function_state).count_p >= (*prototype).size_p {
                let mut old_size: i32 = (*prototype).size_p;
                (*prototype).p = luam_growaux_(
                    self.interpreter,
                    (*prototype).p as *mut libc::c_void,
                    (*function_state).count_p as usize,
                    &mut (*prototype).size_p,
                    ::core::mem::size_of::<*mut Prototype>(),
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
                while old_size < (*prototype).size_p {
                    let fresh45 = old_size;
                    old_size = old_size + 1;
                    let ref mut fresh46 = *((*prototype).p).offset(fresh45 as isize);
                    *fresh46 = std::ptr::null_mut();
                }
            }
            let clp: *mut Prototype = luaf_newproto(self.interpreter);
            let np = (*function_state).count_p;
            (*function_state).count_p = (*function_state).count_p + 1;
            let ref mut target = *((*prototype).p).offset(np as isize);
            *target = clp;
            if (*prototype).get_marked() & 1 << 5 != 0 && (*clp).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    self.interpreter,
                    &mut (*(prototype as *mut Object)),
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
        let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
        for i in (*(*lexical_state).function_state).first_label..(*dynamic_data).label.length {
            let lb: *mut LabelDescription =
                &mut *((*dynamic_data).label.pointer).offset(i as isize) as *mut LabelDescription;
            if (*lb).name == name {
                return lb;
            }
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
        let n: i32 = (*l).length;
        (*l).pointer = luam_growaux_(
            (*lexical_state).interpreter,
            (*l).pointer as *mut libc::c_void,
            n as usize,
            &mut (*l).size,
            ::core::mem::size_of::<LabelDescription>(),
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
            (*(*lexical_state).function_state).count_active_variables;
        (*((*l).pointer).offset(n as isize)).close = 0;
        (*((*l).pointer).offset(n as isize)).program_counter = program_counter;
        (*l).length = n + 1;
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
        let mut i: i32 = (*(*(*lexical_state).function_state).block_control).first_goto;
        let mut needsclose = false;
        while i < (*gl).length {
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
pub unsafe extern "C" fn undefgoto(
    lexical_state: *mut LexicalState,
    gt: *mut LabelDescription,
) -> ! {
    unsafe {
        let mut message: *const i8;
        if (*gt).name
            == luas_newlstr(
                (*lexical_state).interpreter,
                b"break\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            )
        {
            message = b"break outside loop at line %d\0" as *const u8 as *const i8;
            message = luao_pushfstring((*lexical_state).interpreter, message, (*gt).line);
        } else {
            message = b"no visible label '%s' for <goto> at line %d\0" as *const u8 as *const i8;
            message = luao_pushfstring(
                (*lexical_state).interpreter,
                message,
                (*(*gt).name).get_contents_mut(),
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
        let function_state: *mut FunctionState = (*(*lexical_state).function_state).previous;
        init_exp(
            v,
            ExpressionKind::VRELOC,
            luak_codeabx(function_state, OP_CLOSURE, 0, ((*function_state).count_p - 1) as u32),
        );
        luak_exp2nextreg(function_state, v);
    }
}
pub unsafe extern "C" fn open_func(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    block_control: *mut BlockControl,
) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        (*function_state).previous = (*lexical_state).function_state;
        (*function_state).lexical_state = lexical_state;
        (*lexical_state).function_state = function_state;
        (*function_state).program_counter = 0;
        (*function_state).previous_line = (*prototype).line_defined;
        (*function_state).iwthabs = 0;
        (*function_state).last_target = 0;
        (*function_state).freereg = 0;
        (*function_state).count_k = 0;
        (*function_state).count_abslineinfo = 0;
        (*function_state).count_p = 0;
        (*function_state).count_upvalues = 0;
        (*function_state).count_debug_variables = 0 as i16;
        (*function_state).count_active_variables = 0;
        (*function_state).needs_close = false;
        (*function_state).first_local = (*(*lexical_state).dynamic_data).active_variable.length;
        (*function_state).first_label = (*(*lexical_state).dynamic_data).label.length;
        (*function_state).block_control = std::ptr::null_mut();
        (*prototype).source = (*lexical_state).source;
        if (*prototype).get_marked() & 1 << 5 != 0 && (*(*prototype).source).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                (*lexical_state).interpreter,
                &mut (*(prototype as *mut Object)),
                &mut (*((*prototype).source as *mut Object)),
            );
        } else {
        };
        (*prototype).maximum_stack_size = 2 as u8;
        enterblock(function_state, block_control, false);
    }
}
pub unsafe extern "C" fn close_func(lexical_state: *mut LexicalState) {
    unsafe {
        let interpreter: *mut Interpreter = (*lexical_state).interpreter;
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let prototype: *mut Prototype = (*function_state).prototype;
        luak_ret(function_state, luay_nvarstack(function_state), 0);
        leaveblock(function_state);
        luak_finish(function_state);
        (*prototype).code = luam_shrinkvector_(
            interpreter,
            (*prototype).code as *mut libc::c_void,
            &mut (*prototype).size_code,
            (*function_state).program_counter as usize,
            ::core::mem::size_of::<u32>(),
        ) as *mut u32;
        (*prototype).line_info = luam_shrinkvector_(
            interpreter,
            (*prototype).line_info as *mut libc::c_void,
            &mut (*prototype).size_line_info,
            (*function_state).program_counter as usize,
            ::core::mem::size_of::<i8>(),
        ) as *mut i8;
        (*prototype).absolute_line_info = luam_shrinkvector_(
            interpreter,
            (*prototype).absolute_line_info as *mut libc::c_void,
            &mut (*prototype).size_absolute_line_info,
            (*function_state).count_abslineinfo as usize,
            ::core::mem::size_of::<AbsoluteLineInfo>(),
        ) as *mut AbsoluteLineInfo;
        (*prototype).k = luam_shrinkvector_(
            interpreter,
            (*prototype).k as *mut libc::c_void,
            &mut (*prototype).size_k,
            (*function_state).count_k as usize,
            ::core::mem::size_of::<TValue>(),
        ) as *mut TValue;
        (*prototype).p = luam_shrinkvector_(
            interpreter,
            (*prototype).p as *mut libc::c_void,
            &mut (*prototype).size_p,
            (*function_state).count_p as usize,
            ::core::mem::size_of::<*mut Prototype>(),
        ) as *mut *mut Prototype;
        (*prototype).local_variables = luam_shrinkvector_(
            interpreter,
            (*prototype).local_variables as *mut libc::c_void,
            &mut (*prototype).size_local_variables,
            (*function_state).count_debug_variables as usize,
            ::core::mem::size_of::<LocalVariable>(),
        ) as *mut LocalVariable;
        (*prototype).upvalues = luam_shrinkvector_(
            interpreter,
            (*prototype).upvalues as *mut libc::c_void,
            &mut (*prototype).size_upvalues,
            (*function_state).count_upvalues as usize,
            ::core::mem::size_of::<UpValueDescription>(),
        ) as *mut UpValueDescription;
        (*lexical_state).function_state = (*function_state).previous;
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
    }
}
pub unsafe extern "C" fn statlist(lexical_state: *mut LexicalState) {
    unsafe {
        while !(*lexical_state).block_follow(true) {
            if (*lexical_state).token.token == TK_RETURN {
                (*lexical_state).parse_statement();
                return;
            } else {
                (*lexical_state).parse_statement();
            }
        }
    }
}
pub unsafe extern "C" fn fieldsel(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut key: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        luak_exp2anyregup(function_state, v);
        luax_next(lexical_state);
        codename(lexical_state, &mut key);
        luak_indexed(function_state, v, &mut key);
    }
}
pub unsafe extern "C" fn yindex(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        luax_next(lexical_state);
        (*lexical_state).parse_expression(v);
        luak_exp2val((*lexical_state).function_state, v);
        checknext(lexical_state, CHARACTER_BRACKET_RIGHT as i32);
    }
}
pub unsafe extern "C" fn recfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let reg: i32 = (*(*lexical_state).function_state).freereg as i32;
        let mut key: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let mut value: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        if (*lexical_state).token.token == TK_NAME as i32 {
            codename(lexical_state, &mut key);
        } else {
            yindex(lexical_state, &mut key);
        }
        checklimit(
            function_state,
            (*cc).nh,
            0x7FFFFFFF as i32,
            b"items in a constructor\0" as *const u8 as *const i8,
        );
        (*cc).nh += 1;
        (*cc).nh;
        checknext(lexical_state, CHARACTER_EQUAL as i32);
        let mut tab: ExpressionDescription = *(*cc).t;
        luak_indexed(function_state, &mut tab, &mut key);
        (*lexical_state).parse_expression(&mut value);
        luak_storevar(function_state, &mut tab, &mut value);
        (*function_state).freereg = reg as u8;
    }
}
pub unsafe extern "C" fn listfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        (*lexical_state).parse_expression(&mut (*cc).expression_description);
        (*cc).to_store += 1;
        (*cc).to_store;
    }
}
pub unsafe extern "C" fn field(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) {
    unsafe {
        match (*lexical_state).token.token {
            291 => {
                if luax_lookahead(lexical_state) != CHARACTER_EQUAL as i32 {
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
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let line: i32 = (*lexical_state).line_number;
        let program_counter: i32 = luak_code_abck(function_state, OP_NEWTABLE, 0, 0, 0, 0);
        let mut cc: ConstructorControl = ConstructorControl {
            expression_description: ExpressionDescription {
                expression_kind: ExpressionKind::VVOID,
                value: Value { integer: 0 },
                t: 0,
                f: 0,
            },
            t: std::ptr::null_mut(),
            nh: 0,
            na: 0,
            to_store: 0,
        };
        luak_code(function_state, 0u32);
        cc.to_store = 0;
        cc.nh = cc.to_store;
        cc.na = cc.nh;
        cc.t = t;
        init_exp(t, ExpressionKind::VNONRELOC, (*function_state).freereg as i32);
        luak_reserveregs(function_state, 1);
        init_exp(&mut cc.expression_description, ExpressionKind::VVOID, 0);
        checknext(lexical_state, CHARACTER_BRACE_LEFT as i32);
        while !((*lexical_state).token.token == CHARACTER_BRACE_RIGHT as i32) {
            closelistfield(function_state, &mut cc);
            field(lexical_state, &mut cc);
            if !(testnext(lexical_state, CHARACTER_COMMA as i32) != 0
                || testnext(lexical_state, CHARACTER_SEMICOLON as i32) != 0)
            {
                break;
            }
        }
        check_match(lexical_state, CHARACTER_BRACE_RIGHT as i32, CHARACTER_BRACE_LEFT as i32, line);
        lastlistfield(function_state, &mut cc);
        luak_settablesize(function_state, program_counter, (*t).value.info, cc.na, cc.nh);
    }
}
pub unsafe extern "C" fn parlist(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut nparams: i32 = 0;
        let mut is_variable_arguments = false;
        if (*lexical_state).token.token != CHARACTER_PARENTHESIS_RIGHT as i32 {
            loop {
                match (*lexical_state).token.token {
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
                if !(!is_variable_arguments && testnext(lexical_state, CHARACTER_COMMA as i32) != 0) {
                    break;
                }
            }
        }
        adjustlocalvars(lexical_state, nparams);
        (*prototype).count_parameters = (*function_state).count_active_variables;
        if is_variable_arguments {
            setvararg(function_state, (*prototype).count_parameters as i32);
        }
        luak_reserveregs(function_state, (*function_state).count_active_variables as i32);
    }
}
pub unsafe extern "C" fn body(
    lexical_state: *mut LexicalState,
    e: *mut ExpressionDescription,
    is_method: bool,
    line: i32,
) {
    unsafe {
        let mut new_fs: FunctionState = FunctionState {
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
        let mut block_control = BlockControl::new();
        new_fs.prototype = (*lexical_state).add_prototype();
        (*new_fs.prototype).line_defined = line;
        open_func(lexical_state, &mut new_fs, &mut block_control);
        checknext(lexical_state, CHARACTER_PARENTHESIS_LEFT as i32);
        if is_method {
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
        checknext(lexical_state, CHARACTER_PARENTHESIS_RIGHT as i32);
        statlist(lexical_state);
        (*new_fs.prototype).last_line_defined = (*lexical_state).line_number;
        check_match(lexical_state, TK_END as i32, TK_FUNCTION as i32, line);
        codeclosure(lexical_state, e);
        close_func(lexical_state);
    }
}
pub unsafe extern "C" fn funcargs(lexical_state: *mut LexicalState, expression_description: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut args: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let line: i32 = (*lexical_state).line_number;
        match (*lexical_state).token.token {
            CHARACTER_PARENTHESIS_LEFT => {
                luax_next(lexical_state);
                if (*lexical_state).token.token == CHARACTER_PARENTHESIS_RIGHT as i32 {
                    args.expression_kind = ExpressionKind::VVOID;
                } else {
                    (*lexical_state).parse_expression_list(&mut args);
                    if args.expression_kind as u32 == ExpressionKind::VCALL as u32
                        || args.expression_kind as u32 == ExpressionKind::VVARARG as u32
                    {
                        luak_setreturns(function_state, &mut args, -1);
                    }
                }
                check_match(lexical_state, CHARACTER_PARENTHESIS_RIGHT as i32, CHARACTER_PARENTHESIS_LEFT as i32, line);
            }
            CHARACTER_BRACE_LEFT => {
                constructor(lexical_state, &mut args);
            }
            292 => {
                codestring(&mut args, (*lexical_state).token.semantic_info.tstring);
                luax_next(lexical_state);
            }
            _ => {
                luax_syntaxerror(
                    lexical_state,
                    b"function arguments expected\0" as *const u8 as *const i8,
                );
            }
        }
        let base: i32 = (*expression_description).value.info;
        let nparams: i32;
        if args.expression_kind as u32 == ExpressionKind::VCALL as u32 || args.expression_kind as u32 == ExpressionKind::VVARARG as u32 {
            nparams = -1;
        } else {
            if args.expression_kind as u32 != ExpressionKind::VVOID as u32 {
                luak_exp2nextreg(function_state, &mut args);
            }
            nparams = (*function_state).freereg as i32 - (base + 1);
        }
        init_exp(
            expression_description,
            ExpressionKind::VCALL,
            luak_code_abck(function_state, OP_CALL, base, nparams + 1, 2, 0),
        );
        luak_fixline(function_state, line);
        (*function_state).freereg = (base + 1) as u8;
    }
}
pub unsafe extern "C" fn primaryexp(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) {
    unsafe {
        match (*lexical_state).token.token {
            40 => {
                let line: i32 = (*lexical_state).line_number;
                luax_next(lexical_state);
                (*lexical_state).parse_expression(v);
                check_match(lexical_state, CHARACTER_PARENTHESIS_RIGHT as i32, CHARACTER_PARENTHESIS_LEFT as i32, line);
                luak_dischargevars((*lexical_state).function_state, v);
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
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        primaryexp(lexical_state, v);
        loop {
            match (*lexical_state).token.token {
                46 => {
                    fieldsel(lexical_state, v);
                }
                91 => {
                    let mut key: ExpressionDescription = ExpressionDescription {
                        expression_kind: ExpressionKind::VVOID,
                        value: Value { integer: 0 },
                        t: 0,
                        f: 0,
                    };
                    luak_exp2anyregup(function_state, v);
                    yindex(lexical_state, &mut key);
                    luak_indexed(function_state, v, &mut key);
                }
                58 => {
                    let mut key_0: ExpressionDescription = ExpressionDescription {
                        expression_kind: ExpressionKind::VVOID,
                        value: Value { integer: 0 },
                        t: 0,
                        f: 0,
                    };
                    luax_next(lexical_state);
                    codename(lexical_state, &mut key_0);
                    luak_self(function_state, v, &mut key_0);
                    funcargs(lexical_state, v);
                }
                40 | 292 | 123 => {
                    luak_exp2nextreg(function_state, v);
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
        match (*lexical_state).token.token {
            TK_FLT => {
                init_exp(v, ExpressionKind::VKFLT, 0);
                (*v).value.number = (*lexical_state).token.semantic_info.number;
            }
            TK_INT => {
                init_exp(v, ExpressionKind::VKINT, 0);
                (*v).value.integer = (*lexical_state).token.semantic_info.integer;
            }
            TK_STRING => {
                codestring(v, (*lexical_state).token.semantic_info.tstring);
            }
            TK_NIL => {
                init_exp(v, ExpressionKind::VNIL, 0);
            }
            TK_TRUE => {
                init_exp(v, ExpressionKind::VTRUE, 0);
            }
            TK_FALSE => {
                init_exp(v, ExpressionKind::VFALSE, 0);
            }
            TK_DOTS => {
                let function_state: *mut FunctionState = (*lexical_state).function_state;
                if !(*(*function_state).prototype).is_variable_arguments {
                    luax_syntaxerror(
                        lexical_state,
                        b"cannot use '...' outside a vararg function\0" as *const u8 as *const i8,
                    );
                }
                init_exp(v, ExpressionKind::VVARARG, luak_code_abck(function_state, OP_VARARG, 0, 0, 1, 0));
            }
            123 => {
                constructor(lexical_state, v);
                return;
            }
            TK_FUNCTION => {
                luax_next(lexical_state);
                body(lexical_state, v, false, (*lexical_state).line_number);
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
pub unsafe extern "C" fn error_expected(lexical_state: *mut LexicalState, token: i32) -> ! {
    unsafe {
        luax_syntaxerror(
            lexical_state,
            luao_pushfstring(
                (*lexical_state).interpreter,
                b"%s expected\0" as *const u8 as *const i8,
                luax_token2str(lexical_state, token),
            ),
        );
    }
}
pub unsafe extern "C" fn testnext(lexical_state: *mut LexicalState, c: i32) -> i32 {
    unsafe {
        if (*lexical_state).token.token == c {
            luax_next(lexical_state);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn check(lexical_state: *mut LexicalState, c: i32) {
    unsafe {
        if (*lexical_state).token.token != c {
            error_expected(lexical_state, c);
        }
    }
}
pub unsafe extern "C" fn checknext(lexical_state: *mut LexicalState, c: i32) {
    unsafe {
        check(lexical_state, c);
        luax_next(lexical_state);
    }
}
pub unsafe extern "C" fn check_match(
    lexical_state: *mut LexicalState,
    what: i32,
    who: i32,
    where_0: i32,
) {
    unsafe {
        if ((testnext(lexical_state, what) == 0) as i32 != 0) as i64 != 0 {
            if where_0 == (*lexical_state).line_number {
                error_expected(lexical_state, what);
            } else {
                luax_syntaxerror(
                    lexical_state,
                    luao_pushfstring(
                        (*lexical_state).interpreter,
                        b"%s expected (to close %s at line %d)\0" as *const u8 as *const i8,
                        luax_token2str(lexical_state, what),
                        luax_token2str(lexical_state, who),
                        where_0,
                    ),
                );
            }
        }
    }
}
pub unsafe extern "C" fn str_checkname(lexical_state: *mut LexicalState) -> *mut TString {
    unsafe {
        check(lexical_state, TK_NAME as i32);
        let ts: *mut TString = (*lexical_state).token.semantic_info.tstring;
        luax_next(lexical_state);
        return ts;
    }
}
pub unsafe extern "C" fn codename(lexical_state: *mut LexicalState, e: *mut ExpressionDescription) {
    unsafe {
        codestring(e, str_checkname(lexical_state));
    }
}
pub unsafe extern "C" fn registerlocalvar(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    variable_name: *mut TString,
) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut old_size: i32 = (*prototype).size_local_variables;
        (*prototype).local_variables = luam_growaux_(
            (*lexical_state).interpreter,
            (*prototype).local_variables as *mut libc::c_void,
            (*function_state).count_debug_variables as usize,
            &mut (*prototype).size_local_variables,
            ::core::mem::size_of::<LocalVariable>(),
            (if 32767 as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64)
            {
                32767 as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64) as u32
            }) as i32,
            b"local variables\0" as *const u8 as *const i8,
        ) as *mut LocalVariable;
        while old_size < (*prototype).size_local_variables {
            let fresh33 = old_size;
            old_size = old_size + 1;
            let ref mut fresh34 = (*((*prototype).local_variables).offset(fresh33 as isize)).variable_name;
            *fresh34 = std::ptr::null_mut();
        }
        let ref mut fresh35 =
            (*((*prototype).local_variables).offset((*function_state).count_debug_variables as isize)).variable_name;
        *fresh35 = variable_name;
        (*((*prototype).local_variables).offset((*function_state).count_debug_variables as isize)).start_program_counter =
            (*function_state).program_counter;
        if (*prototype).get_marked() & 1 << 5 != 0 && (*variable_name).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                (*lexical_state).interpreter,
                &mut (*(prototype as *mut Object)),
                &mut (*(variable_name as *mut Object)),
            );
        } else {
        };
        let fresh36 = (*function_state).count_debug_variables;
        (*function_state).count_debug_variables = (*function_state).count_debug_variables + 1;
        return fresh36 as i32;
    }
}
pub unsafe extern "C" fn new_localvar(lexical_state: *mut LexicalState, name: *mut TString) -> i32 {
    unsafe {
        let interpreter: *mut Interpreter = (*lexical_state).interpreter;
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
        let var: *mut VariableDescription;
        checklimit(
            function_state,
            (*dynamic_data).active_variable.length + 1 - (*function_state).first_local,
            200 as i32,
            b"local variables\0" as *const u8 as *const i8,
        );
        (*dynamic_data).active_variable.pointer = luam_growaux_(
            interpreter,
            (*dynamic_data).active_variable.pointer as *mut libc::c_void,
            ((*dynamic_data).active_variable.length + 1) as usize,
            &mut (*dynamic_data).active_variable.size,
            ::core::mem::size_of::<VariableDescription>(),
            (if 32767 as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<VariableDescription>() as u64)
            {
                32767 as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<VariableDescription>() as u64) as u32
            }) as i32,
            b"local variables\0" as *const u8 as *const i8,
        ) as *mut VariableDescription;
        let fresh37 = (*dynamic_data).active_variable.length;
        (*dynamic_data).active_variable.length = (*dynamic_data).active_variable.length + 1;
        var = &mut *((*dynamic_data).active_variable.pointer).offset(fresh37 as isize)
            as *mut VariableDescription;
        (*var).content.kind = 0;
        (*var).content.name = name;
        return (*dynamic_data).active_variable.length - 1 - (*function_state).first_local;
    }
}
pub unsafe extern "C" fn check_readonly(
    lexical_state: *mut LexicalState,
    e: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut variable_name: *mut TString = std::ptr::null_mut();
        match (*e).expression_kind {
            ExpressionKind::VCONST => {
                variable_name = (*((*(*lexical_state).dynamic_data).active_variable.pointer)
                    .offset((*e).value.info as isize))
                .content
                .name;
            }
            ExpressionKind::VLOCAL => {
                let vardesc: *mut VariableDescription = getlocalvardesc(function_state, (*e).value.variable.value_index as i32);
                if (*vardesc).content.kind as i32 != 0 {
                    variable_name = (*vardesc).content.name;
                }
            }
            ExpressionKind::VUPVAL => {
                let up: *mut UpValueDescription =
                    &mut *((*(*function_state).prototype).upvalues).offset((*e).value.info as isize) as *mut UpValueDescription;
                if (*up).kind as i32 != 0 {
                    variable_name = (*up).name;
                }
            }
            _ => return,
        }
        if !variable_name.is_null() {
            let message: *const i8 = luao_pushfstring(
                (*lexical_state).interpreter,
                b"attempt to assign to const variable '%s'\0" as *const u8 as *const i8,
                (*variable_name).get_contents_mut(),
            );
            luak_semerror(lexical_state, message);
        }
    }
}
pub unsafe extern "C" fn adjustlocalvars(lexical_state: *mut LexicalState, nvars: i32) {
    unsafe {
        let function_state = (*lexical_state).function_state;
        let mut reglevel_0 = luay_nvarstack(function_state);
        for _ in 0..nvars {
            let fresh39 = (*function_state).count_active_variables;
            (*function_state).count_active_variables = ((*function_state).count_active_variables).wrapping_add(1);
            let vidx = fresh39 as i32;
            let var = getlocalvardesc(function_state, vidx);
            let fresh40 = reglevel_0;
            reglevel_0 += 1;
            (*var).content.register_index = fresh40 as u8;
            (*var).content.pidx = registerlocalvar(lexical_state, function_state, (*var).content.name) as i16;
        }
    }
}
pub unsafe extern "C" fn singlevar(
    lexical_state: *mut LexicalState,
    var: *mut ExpressionDescription,
) {
    unsafe {
        let variable_name: *mut TString = str_checkname(lexical_state);
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        singlevaraux(function_state, variable_name, var, 1);
        if (*var).expression_kind as u32 == ExpressionKind::VVOID as u32 {
            let mut key: ExpressionDescription = ExpressionDescription {
                expression_kind: ExpressionKind::VVOID,
                value: Value { integer: 0 },
                t: 0,
                f: 0,
            };
            singlevaraux(function_state, (*lexical_state).environment, var, 1);
            luak_exp2anyregup(function_state, var);
            codestring(&mut key, variable_name);
            luak_indexed(function_state, var, &mut key);
        }
    }
}
pub unsafe extern "C" fn adjust_assign(
    lexical_state: *mut LexicalState,
    nvars: i32,
    nexps: i32,
    e: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let needed: i32 = nvars - nexps;
        if (*e).expression_kind as u32 == ExpressionKind::VCALL as u32 || (*e).expression_kind as u32 == ExpressionKind::VVARARG as u32 {
            let mut extra: i32 = needed + 1;
            if extra < 0 {
                extra = 0;
            }
            luak_setreturns(function_state, e, extra);
        } else {
            if (*e).expression_kind as u32 != ExpressionKind::VVOID as u32 {
                luak_exp2nextreg(function_state, e);
            }
            if needed > 0 {
                luak_nil(function_state, (*function_state).freereg as i32, needed);
            }
        }
        if needed > 0 {
            luak_reserveregs(function_state, needed);
        } else {
            (*function_state).freereg = ((*function_state).freereg as i32 + needed) as u8;
        };
    }
}
pub unsafe extern "C" fn jumpscopeerror(
    lexical_state: *mut LexicalState,
    gt: *mut LabelDescription,
) -> ! {
    unsafe {
        let variable_name: *const i8 =
            (*(*getlocalvardesc((*lexical_state).function_state, (*gt).count_active_variables as i32))
                .content
                .name)
                .get_contents_mut();
        let mut message: *const i8 =
            b"<goto %s> at line %d jumps into the scope of local '%s'\0" as *const u8 as *const i8;
        message = luao_pushfstring(
            (*lexical_state).interpreter,
            message,
            (*(*gt).name).get_contents_mut(),
            (*gt).line,
            variable_name,
        );
        luak_semerror(lexical_state, message);
    }
}
pub unsafe extern "C" fn solvegoto(
    lexical_state: *mut LexicalState,
    goto_offset: i32,
    label: *mut LabelDescription,
) {
    unsafe {
        let goto_label_list: *mut LabelList = &mut (*(*lexical_state).dynamic_data).gt;
        let goto_label_description: *mut LabelDescription =
            &mut *((*goto_label_list).pointer).offset(goto_offset as isize) as *mut LabelDescription;
        if ((((*goto_label_description).count_active_variables as i32) < (*label).count_active_variables as i32) as i32
            != 0) as i64
            != 0
        {
            jumpscopeerror(lexical_state, goto_label_description);
        }
        luak_patchlist(
            (*lexical_state).function_state,
            (*goto_label_description).program_counter,
            (*label).program_counter,
        );
        let mut i: i32 = goto_offset;
        while i < (*goto_label_list).length - 1 {
            *((*goto_label_list).pointer).offset(i as isize) = *((*goto_label_list).pointer).offset((i + 1) as isize);
            i += 1;
        }
        (*goto_label_list).length -= 1;
    }
}
pub unsafe extern "C" fn subexpr(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
    limit: i32,
) -> u32 {
    unsafe {
        (*((*lexical_state).interpreter)).luae_inccstack();
        let uop = getunopr((*lexical_state).token.token);
        if uop as u32 != OperatorUnary::None_ as u32 {
            let line: i32 = (*lexical_state).line_number;
            luax_next(lexical_state);
            subexpr(lexical_state, v, 12 as i32);
            luak_prefix((*lexical_state).function_state, uop, v, line);
        } else {
            simpleexp(lexical_state, v);
        }
        let mut op: u32 = getbinopr((*lexical_state).token.token);
        while op as u32 != OPR_NOBINOPR as u32 && PRIORITY[op as usize].left as i32 > limit {
            let mut v2: ExpressionDescription = ExpressionDescription {
                expression_kind: ExpressionKind::VVOID,
                value: Value { integer: 0 },
                t: 0,
                f: 0,
            };
            let line_0: i32 = (*lexical_state).line_number;
            luax_next(lexical_state);
            luak_infix((*lexical_state).function_state, op, v);
            let nextop: u32 = subexpr(lexical_state, &mut v2, PRIORITY[op as usize].right as i32);
            luak_posfix((*lexical_state).function_state, op, v, &mut v2, line_0);
            op = nextop;
        }
        (*(*lexical_state).interpreter).count_c_calls =
            ((*(*lexical_state).interpreter).count_c_calls).wrapping_sub(1);
        (*(*lexical_state).interpreter).count_c_calls;
        return op;
    }
}
pub unsafe extern "C" fn block(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut block_control: BlockControl = BlockControl::new();
        enterblock(function_state, &mut block_control, false);
        statlist(lexical_state);
        leaveblock(function_state);
    }
}
pub unsafe extern "C" fn check_conflict(
    lexical_state: *mut LexicalState,
    mut lh: *mut LHSAssign,
    v: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let extra: i32 = (*function_state).freereg as i32;
        let mut conflict: i32 = 0;
        while !lh.is_null() {
            if ExpressionKind::VINDEXED as u32 <= (*lh).expression_description.expression_kind as u32
                && (*lh).expression_description.expression_kind as u32 <= ExpressionKind::VINDEXSTR as u32
            {
                if (*lh).expression_description.expression_kind as u32 == ExpressionKind::VINDEXUP as u32 {
                    if (*v).expression_kind as u32 == ExpressionKind::VUPVAL as u32
                        && (*lh).expression_description.value.index.reference_tag as i32 == (*v).value.info
                    {
                        conflict = 1;
                        (*lh).expression_description.expression_kind = ExpressionKind::VINDEXSTR;
                        (*lh).expression_description.value.index.reference_tag = extra as u8;
                    }
                } else {
                    if (*v).expression_kind as u32 == ExpressionKind::VLOCAL as u32
                        && (*lh).expression_description.value.index.reference_tag as i32 == (*v).value.variable.register_index as i32
                    {
                        conflict = 1;
                        (*lh).expression_description.value.index.reference_tag = extra as u8;
                    }
                    if (*lh).expression_description.expression_kind as u32 == ExpressionKind::VINDEXED as u32
                        && (*v).expression_kind as u32 == ExpressionKind::VLOCAL as u32
                        && (*lh).expression_description.value.index.reference_index as i32 == (*v).value.variable.register_index as i32
                    {
                        conflict = 1;
                        (*lh).expression_description.value.index.reference_index = extra as i16;
                    }
                }
            }
            lh = (*lh).previous;
        }
        if conflict != 0 {
            if (*v).expression_kind == ExpressionKind::VLOCAL {
                luak_code_abck(function_state, OP_MOVE, extra, (*v).value.variable.register_index as i32, 0, 0);
            } else {
                luak_code_abck(function_state, OP_GETUPVAL, extra, (*v).value.info, 0, 0);
            }
            luak_reserveregs(function_state, 1);
        }
    }
}
pub unsafe extern "C" fn restassign(
    lexical_state: *mut LexicalState,
    lh: *mut LHSAssign,
    nvars: i32,
) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        if !(ExpressionKind::VLOCAL as u32 <= (*lh).expression_description.expression_kind as u32
            && (*lh).expression_description.expression_kind as u32 <= ExpressionKind::VINDEXSTR as u32)
        {
            luax_syntaxerror(lexical_state, b"syntax error\0" as *const u8 as *const i8);
        }
        check_readonly(lexical_state, &mut (*lh).expression_description);
        if testnext(lexical_state, CHARACTER_COMMA as i32) != 0 {
            let mut nv: LHSAssign = LHSAssign {
                previous: std::ptr::null_mut(),
                expression_description: ExpressionDescription {
                    expression_kind: ExpressionKind::VVOID,
                    value: Value { integer: 0 },
                    t: 0,
                    f: 0,
                },
            };
            nv.previous = lh;
            suffixedexp(lexical_state, &mut nv.expression_description);
            if !(ExpressionKind::VINDEXED as u32 <= nv.expression_description.expression_kind as u32
                && nv.expression_description.expression_kind as u32 <= ExpressionKind::VINDEXSTR as u32)
            {
                check_conflict(lexical_state, lh, &mut nv.expression_description);
            }
            (*((*lexical_state).interpreter)).luae_inccstack();
            restassign(lexical_state, &mut nv, nvars + 1);
            (*(*lexical_state).interpreter).count_c_calls =
                ((*(*lexical_state).interpreter).count_c_calls).wrapping_sub(1);
            (*(*lexical_state).interpreter).count_c_calls;
        } else {
            checknext(lexical_state, CHARACTER_EQUAL as i32);
            let nexps: i32 = (*lexical_state).parse_expression_list(&mut expression_description);
            if nexps != nvars {
                adjust_assign(lexical_state, nvars, nexps, &mut expression_description);
            } else {
                luak_setoneret((*lexical_state).function_state, &mut expression_description);
                luak_storevar((*lexical_state).function_state, &mut (*lh).expression_description, &mut expression_description);
                return;
            }
        }
        init_exp(&mut expression_description, ExpressionKind::VNONRELOC, (*(*lexical_state).function_state).freereg as i32 - 1);
        luak_storevar((*lexical_state).function_state, &mut (*lh).expression_description, &mut expression_description);
    }
}
pub unsafe extern "C" fn cond(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        (*lexical_state).parse_expression(&mut v);
        if v.expression_kind as u32 == ExpressionKind::VNIL as u32 {
            v.expression_kind = ExpressionKind::VFALSE;
        }
        luak_goiftrue((*lexical_state).function_state, &mut v);
        return v.f;
    }
}
pub unsafe extern "C" fn gotostat(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let line: i32 = (*lexical_state).line_number;
        let name: *mut TString = str_checkname(lexical_state);
        let lb: *mut LabelDescription = findlabel(lexical_state, name);
        if lb.is_null() {
            newgotoentry(lexical_state, name, line, luak_jump(function_state));
        } else {
            let lblevel: i32 = reglevel(function_state, (*lb).count_active_variables as i32);
            if luay_nvarstack(function_state) > lblevel {
                luak_code_abck(function_state, OP_CLOSE, lblevel, 0, 0, 0);
            }
            luak_patchlist(function_state, luak_jump(function_state), (*lb).program_counter);
        };
    }
}
pub unsafe extern "C" fn breakstat(lexical_state: *mut LexicalState) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        luax_next(lexical_state);
        newgotoentry(
            lexical_state,
            luas_newlstr(
                (*lexical_state).interpreter,
                b"break\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            ),
            line,
            luak_jump((*lexical_state).function_state),
        );
    }
}
pub unsafe extern "C" fn checkrepeated(lexical_state: *mut LexicalState, name: *mut TString) {
    unsafe {
        let lb: *mut LabelDescription = findlabel(lexical_state, name);
        if ((lb != std::ptr::null_mut() as *mut LabelDescription) as i32 != 0) as i64 != 0 {
            let mut message: *const i8 =
                b"label '%s' already defined on line %d\0" as *const u8 as *const i8;
            message = luao_pushfstring(
                (*lexical_state).interpreter,
                message,
                (*name).get_contents_mut(),
                (*lb).line,
            );
            luak_semerror(lexical_state, message);
        }
    }
}
pub unsafe extern "C" fn labelstat(
    lexical_state: *mut LexicalState,
    name: *mut TString,
    line: i32,
) {
    unsafe {
        checknext(lexical_state, TK_DBCOLON as i32);
        while (*lexical_state).token.token == CHARACTER_SEMICOLON as i32
            || (*lexical_state).token.token == TK_DBCOLON as i32
        {
            (*lexical_state).parse_statement();
        }
        checkrepeated(lexical_state, name);
        (*lexical_state).create_label(name, line, (*lexical_state).block_follow(false));
    }
}
pub unsafe extern "C" fn whilestat(lexical_state: *mut LexicalState, line: i32) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut block_control: BlockControl = BlockControl::new();
        luax_next(lexical_state);
        let whileinit: i32 = luak_getlabel(function_state);
        let condexit: i32 = cond(lexical_state);
        enterblock(function_state, &mut block_control, true);
        checknext(lexical_state, TK_DO as i32);
        block(lexical_state);
        luak_patchlist(function_state, luak_jump(function_state), whileinit);
        check_match(lexical_state, TK_END as i32, TK_WHILE as i32, line);
        leaveblock(function_state);
        luak_patchtohere(function_state, condexit);
    }
}
pub unsafe extern "C" fn repeatstat(lexical_state: *mut LexicalState, line: i32) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let repeat_init: i32 = luak_getlabel(function_state);
        let mut bl1: BlockControl = BlockControl::new();
        let mut bl2: BlockControl = BlockControl::new();
        enterblock(function_state, &mut bl1, true);
        enterblock(function_state, &mut bl2, false);
        luax_next(lexical_state);
        statlist(lexical_state);
        check_match(lexical_state, TK_UNTIL as i32, TK_REPEAT as i32, line);
        let mut condexit: i32 = cond(lexical_state);
        leaveblock(function_state);
        if bl2.count_upvalues != 0 {
            let exit_0: i32 = luak_jump(function_state);
            luak_patchtohere(function_state, condexit);
            luak_code_abck(
                function_state,
                OP_CLOSE,
                reglevel(function_state, bl2.count_active_variables as i32),
                0,
                0,
                0,
            );
            condexit = luak_jump(function_state);
            luak_patchtohere(function_state, exit_0);
        }
        luak_patchlist(function_state, condexit, repeat_init);
        leaveblock(function_state);
    }
}
pub unsafe extern "C" fn exp1(lexical_state: *mut LexicalState) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        (*lexical_state).parse_expression(&mut expression_description);
        luak_exp2nextreg((*lexical_state).function_state, &mut expression_description);
    }
}
pub unsafe extern "C" fn forbody(
    lexical_state: *mut LexicalState,
    base: i32,
    line: i32,
    nvars: i32,
    isgen: i32,
) {
    unsafe {
        static mut FOR_PREP: [u32; 2] = [OP_FORPREP, OP_TFORPREP];
        static mut FOR_LOOP: [u32; 2] = [OP_FORLOOP, OP_TFORLOOP];
        let mut block_control: BlockControl = BlockControl::new();
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        checknext(lexical_state, TK_DO as i32);
        let prep: i32 = luak_codeabx(function_state, FOR_PREP[isgen as usize], base, 0u32);
        enterblock(function_state, &mut block_control, false);
        adjustlocalvars(lexical_state, nvars);
        luak_reserveregs(function_state, nvars);
        block(lexical_state);
        leaveblock(function_state);
        fixforjump(function_state, prep, luak_getlabel(function_state), 0);
        if isgen != 0 {
            luak_code_abck(function_state, OP_TFORCALL, base, 0, nvars, 0);
            luak_fixline(function_state, line);
        }
        let endfor: i32 = luak_codeabx(function_state, FOR_LOOP[isgen as usize], base, 0u32);
        fixforjump(function_state, endfor, prep + 1, 1);
        luak_fixline(function_state, line);
    }
}
pub unsafe extern "C" fn fornum(
    lexical_state: *mut LexicalState,
    variable_name: *mut TString,
    line: i32,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let base: i32 = (*function_state).freereg as i32;
        let s = b"(for interpreter)\0" as *const u8 as *const i8;
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(lexical_state, variable_name);
        checknext(lexical_state, CHARACTER_EQUAL as i32);
        exp1(lexical_state);
        checknext(lexical_state, CHARACTER_COMMA as i32);
        exp1(lexical_state);
        if testnext(lexical_state, CHARACTER_COMMA as i32) != 0 {
            exp1(lexical_state);
        } else {
            luak_int(function_state, (*function_state).freereg as i32, 1 as i64);
            luak_reserveregs(function_state, 1);
        }
        adjustlocalvars(lexical_state, 3);
        forbody(lexical_state, base, line, 1, 0);
    }
}
pub unsafe extern "C" fn forlist(lexical_state: *mut LexicalState, indexname: *mut TString) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut expression_description: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let mut nvars: i32 = 5;
        let base: i32 = (*function_state).freereg as i32;
        let s = b"(for interpreter)\0" as *const u8 as *const i8;
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(
            lexical_state,
            luax_newstring(lexical_state, s, strlen(s) as u64),
        );
        new_localvar(lexical_state, indexname);
        while testnext(lexical_state, CHARACTER_COMMA as i32) != 0 {
            new_localvar(lexical_state, str_checkname(lexical_state));
            nvars += 1;
        }
        checknext(lexical_state, TK_IN as i32);
        let line: i32 = (*lexical_state).line_number;
        adjust_assign(lexical_state, 4, (*lexical_state).parse_expression_list(&mut expression_description), &mut expression_description);
        adjustlocalvars(lexical_state, 4);
        marktobeclosed(function_state);
        luak_checkstack(function_state, 3);
        forbody(lexical_state, base, line, nvars - 4, 1);
    }
}
pub unsafe extern "C" fn forstat(lexical_state: *mut LexicalState, line: i32) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut block_control: BlockControl = BlockControl::new();
        enterblock(function_state, &mut block_control, true);
        luax_next(lexical_state);
        let variable_name: *mut TString = str_checkname(lexical_state);
        match (*lexical_state).token.token {
            CHARACTER_EQUAL => {
                fornum(lexical_state, variable_name, line);
            }
            CHARACTER_COMMA | TK_IN => {
                forlist(lexical_state, variable_name);
            }
            _ => {
                luax_syntaxerror(
                    lexical_state,
                    b"CHARACTER_EQUAL or 'in' expected\0" as *const u8 as *const i8,
                );
            }
        }
        check_match(lexical_state, TK_END as i32, TK_FOR as i32, line);
        leaveblock(function_state);
    }
}
pub unsafe extern "C" fn test_then_block(lexical_state: *mut LexicalState, escapelist: *mut i32) {
    unsafe {
        let mut block_control: BlockControl = BlockControl::new();
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut v: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let jf;
        luax_next(lexical_state);
        (*lexical_state).parse_expression(&mut v);
        checknext(lexical_state, TK_THEN as i32);
        if (*lexical_state).token.token == TK_BREAK as i32 {
            let line: i32 = (*lexical_state).line_number;
            luak_goiffalse((*lexical_state).function_state, &mut v);
            luax_next(lexical_state);
            enterblock(function_state, &mut block_control, false);
            newgotoentry(
                lexical_state,
                luas_newlstr(
                    (*lexical_state).interpreter,
                    b"break\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ),
                line,
                v.t,
            );
            while testnext(lexical_state, CHARACTER_SEMICOLON as i32) != 0 {}
            if (*lexical_state).block_follow(false) {
                leaveblock(function_state);
                return;
            } else {
                jf = luak_jump(function_state);
            }
        } else {
            luak_goiftrue((*lexical_state).function_state, &mut v);
            enterblock(function_state, &mut block_control, false);
            jf = v.f;
        }
        statlist(lexical_state);
        leaveblock(function_state);
        if (*lexical_state).token.token == TK_ELSE as i32
            || (*lexical_state).token.token == TK_ELSEIF as i32
        {
            luak_concat(function_state, escapelist, luak_jump(function_state));
        }
        luak_patchtohere(function_state, jf);
    }
}
pub unsafe extern "C" fn ifstat(lexical_state: *mut LexicalState, line: i32) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut escapelist: i32 = -1;
        test_then_block(lexical_state, &mut escapelist);
        while (*lexical_state).token.token == TK_ELSEIF as i32 {
            test_then_block(lexical_state, &mut escapelist);
        }
        if testnext(lexical_state, TK_ELSE as i32) != 0 {
            block(lexical_state);
        }
        check_match(lexical_state, TK_END as i32, TK_IF as i32, line);
        luak_patchtohere(function_state, escapelist);
    }
}
pub unsafe extern "C" fn localfunc(lexical_state: *mut LexicalState) {
    unsafe {
        let mut b: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let fvar: i32 = (*function_state).count_active_variables as i32;
        new_localvar(lexical_state, str_checkname(lexical_state));
        adjustlocalvars(lexical_state, 1);
        body(lexical_state, &mut b, false, (*lexical_state).line_number);
        (*localdebuginfo(function_state, fvar)).start_program_counter = (*function_state).program_counter;
    }
}
pub unsafe extern "C" fn getlocalattribute(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        if testnext(lexical_state, CHARACTER_ANGLE_LEFT) != 0 {
            let attr: *const i8 = (*str_checkname(lexical_state)).get_contents_mut();
            checknext(lexical_state, CHARACTER_ANGLE_RIGHT);
            if strcmp(attr, b"const\0" as *const u8 as *const i8) == 0 {
                return 1;
            } else if strcmp(attr, b"close\0" as *const u8 as *const i8) == 0 {
                return 2;
            } else {
                luak_semerror(
                    lexical_state,
                    luao_pushfstring(
                        (*lexical_state).interpreter,
                        b"unknown attribute '%s'\0" as *const u8 as *const i8,
                        attr,
                    ),
                );
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn localstat(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut toclose: i32 = -1;
        let var: *mut VariableDescription;
        let mut vidx: i32;
        let mut kind: i32;
        let mut nvars: i32 = 0;
        let nexps: i32;
        let mut expression_description: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        loop {
            vidx = new_localvar(lexical_state, str_checkname(lexical_state));
            kind = getlocalattribute(lexical_state);
            (*getlocalvardesc(function_state, vidx)).content.kind = kind as u8;
            if kind == 2 {
                if toclose != -1 {
                    luak_semerror(
                        lexical_state,
                        b"multiple to-be-closed variables in local list\0" as *const u8
                            as *const i8,
                    );
                }
                toclose = (*function_state).count_active_variables as i32 + nvars;
            }
            nvars += 1;
            if !(testnext(lexical_state, CHARACTER_COMMA as i32) != 0) {
                break;
            }
        }
        if testnext(lexical_state, CHARACTER_EQUAL as i32) != 0 {
            nexps = (*lexical_state).parse_expression_list(&mut expression_description);
        } else {
            expression_description.expression_kind = ExpressionKind::VVOID;
            nexps = 0;
        }
        var = getlocalvardesc(function_state, vidx);
        if nvars == nexps
            && (*var).content.kind as i32 == 1
            && luak_exp2const(function_state, &mut expression_description, &mut (*var).k)
        {
            (*var).content.kind = 3 as u8;
            adjustlocalvars(lexical_state, nvars - 1);
            (*function_state).count_active_variables = ((*function_state).count_active_variables).wrapping_add(1);
            (*function_state).count_active_variables;
        } else {
            adjust_assign(lexical_state, nvars, nexps, &mut expression_description);
            adjustlocalvars(lexical_state, nvars);
        }
        checktoclose(function_state, toclose);
    }
}
pub unsafe extern "C" fn funcname(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
) -> bool {
    unsafe {
        let mut is_method: bool = false;
        singlevar(lexical_state, v);
        while (*lexical_state).token.token == CHARACTER_PERIOD as i32 {
            fieldsel(lexical_state, v);
        }
        if (*lexical_state).token.token == CHARACTER_COLON as i32 {
            is_method = true;
            fieldsel(lexical_state, v);
        }
        return is_method;
    }
}
pub unsafe extern "C" fn funcstat(lexical_state: *mut LexicalState, line: i32) {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let mut b: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        luax_next(lexical_state);
        let is_method = funcname(lexical_state, &mut v);
        body(lexical_state, &mut b, is_method, line);
        check_readonly(lexical_state, &mut v);
        luak_storevar((*lexical_state).function_state, &mut v, &mut b);
        luak_fixline((*lexical_state).function_state, line);
    }
}
pub unsafe extern "C" fn exprstat(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut v: LHSAssign = LHSAssign {
            previous: std::ptr::null_mut(),
            expression_description: ExpressionDescription {
                expression_kind: ExpressionKind::VVOID,
                value: Value { integer: 0 },
                t: 0,
                f: 0,
            },
        };
        suffixedexp(lexical_state, &mut v.expression_description);
        if (*lexical_state).token.token == CHARACTER_EQUAL as i32 || (*lexical_state).token.token == CHARACTER_COMMA as i32 {
            v.previous = std::ptr::null_mut();
            restassign(lexical_state, &mut v, 1);
        } else {
            if !(v.expression_description.expression_kind as u32 == ExpressionKind::VCALL as u32) {
                luax_syntaxerror(lexical_state, b"syntax error\0" as *const u8 as *const i8);
            }
            let inst: *mut u32 = &mut *((*(*function_state).prototype).code).offset(v.expression_description.value.info as isize) as *mut u32;
            *inst = *inst & !(!(!(0u32) << 8) << POSITION_C)
                | (1 as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
        };
    }
}
pub unsafe extern "C" fn retstat(lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).function_state;
        let mut expression_description: ExpressionDescription = ExpressionDescription {
            expression_kind: ExpressionKind::VVOID,
            value: Value { integer: 0 },
            t: 0,
            f: 0,
        };
        let mut nret: i32;
        let mut first: i32 = luay_nvarstack(function_state);
        if (*lexical_state).block_follow(true) || (*lexical_state).token.token == CHARACTER_SEMICOLON as i32 {
            nret = 0;
        } else {
            nret = (*lexical_state).parse_expression_list(&mut expression_description);
            if expression_description.expression_kind as u32 == ExpressionKind::VCALL as u32 || expression_description.expression_kind as u32 == ExpressionKind::VVARARG as u32 {
                luak_setreturns(function_state, &mut expression_description, -1);
                if expression_description.expression_kind as u32 == ExpressionKind::VCALL as u32
                    && nret == 1
                    && !(*(*function_state).block_control).is_inside_tbc
                {
                    *((*(*function_state).prototype).code).offset(expression_description.value.info as isize) =
                        *((*(*function_state).prototype).code).offset(expression_description.value.info as isize) & !(!(!(0u32) << 7) << 0)
                            | (OP_TAILCALL as u32) << 0 & !(!(0u32) << 7) << 0;
                }
                nret = -1;
            } else if nret == 1 {
                first = luak_exp2anyreg(function_state, &mut expression_description);
            } else {
                luak_exp2nextreg(function_state, &mut expression_description);
            }
        }
        luak_ret(function_state, first, nret);
        testnext(lexical_state, CHARACTER_SEMICOLON as i32);
    }
}
pub unsafe extern "C" fn mainfunc(lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut block_control: BlockControl = BlockControl::new();
        let env: *mut UpValueDescription;
        open_func(lexical_state, function_state, &mut block_control);
        setvararg(function_state, 0);
        env = allocupvalue(function_state);
        (*env).is_in_stack = true;
        (*env).index = 0;
        (*env).kind = 0;
        (*env).name = (*lexical_state).environment;
        if (*(*function_state).prototype).get_marked() & 1 << 5 != 0
            && (*(*env).name).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                (*lexical_state).interpreter,
                &mut (*((*function_state).prototype as *mut Object)),
                &mut (*((*env).name as *mut Object)),
            );
        } else {
        };
        luax_next(lexical_state);
        statlist(lexical_state);
        check(lexical_state, TK_EOS as i32);
        close_func(lexical_state);
    }
}
pub unsafe extern "C" fn save(lexical_state: *mut LexicalState, c: i32) {
    unsafe {
        let b: *mut Buffer = (*lexical_state).buffer;
        if ((*b).length).wrapping_add(1 as usize) > (*b).size {
            if (*b).size
                >= (if (::core::mem::size_of::<u64>()) < ::core::mem::size_of::<i64>()
                {
                    !(0usize)
                } else {
                    MAXIMUM_SIZE
                })
                .wrapping_div(2)
            {
                lexerror(
                    lexical_state,
                    b"lexical element too long\0" as *const u8 as *const i8,
                    0,
                );
            }
            let new_size = (*b).size.wrapping_mul(2);
            (*b).pointer = luam_saferealloc_(
                (*lexical_state).interpreter,
                (*b).pointer as *mut libc::c_void,
                (*b).size.wrapping_mul(::core::mem::size_of::<i8>()),
                new_size.wrapping_mul(::core::mem::size_of::<i8>()),
            ) as *mut i8;
            (*b).size = new_size;
        }
        let fresh49 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        *((*b).pointer).offset(fresh49 as isize) = c as i8;
    }
}
pub unsafe extern "C" fn luax_token2str(lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        if token < 127 as i32 * 2 + 1 + 1 {
            if is_printable(token + 1) {
                return luao_pushfstring(
                    (*lexical_state).interpreter,
                    b"'%c'\0" as *const u8 as *const i8,
                    token,
                );
            } else {
                return luao_pushfstring(
                    (*lexical_state).interpreter,
                    b"'<\\%d>'\0" as *const u8 as *const i8,
                    token,
                );
            }
        } else {
            let s: *const i8 = TOKENS[(token - (127 as i32 * 2 + 1 + 1)) as usize];
            if token < TK_EOS as i32 {
                return luao_pushfstring(
                    (*lexical_state).interpreter,
                    b"'%s'\0" as *const u8 as *const i8,
                    s,
                );
            } else {
                return s;
            }
        };
    }
}
pub unsafe extern "C" fn text_token(lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        match token {
            TK_NAME | TK_STRING | TK_FLT | TK_INT => {
                save(lexical_state, Character::Null as i32);
                return luao_pushfstring(
                    (*lexical_state).interpreter,
                    b"'%s'\0" as *const u8 as *const i8,
                    (*(*lexical_state).buffer).pointer,
                );
            }
            _ => return luax_token2str(lexical_state, token),
        };
    }
}
pub unsafe extern "C" fn lexerror(
    lexical_state: *mut LexicalState,
    mut message: *const i8,
    token: i32,
) -> ! {
    unsafe {
        message = luag_addinfo(
            (*lexical_state).interpreter,
            message,
            (*lexical_state).source,
            (*lexical_state).line_number,
        );
        if token != 0 {
            luao_pushfstring(
                (*lexical_state).interpreter,
                b"%s near %s\0" as *const u8 as *const i8,
                message,
                text_token(lexical_state, token),
            );
        }
        luad_throw((*lexical_state).interpreter, 3);
    }
}
pub unsafe extern "C" fn luax_syntaxerror(
    lexical_state: *mut LexicalState,
    message: *const i8,
) -> ! {
    unsafe {
        lexerror(lexical_state, message, (*lexical_state).token.token);
    }
}
pub unsafe extern "C" fn luax_newstring(
    lexical_state: *mut LexicalState,
    str: *const i8,
    length: u64,
) -> *mut TString {
    unsafe {
        let interpreter: *mut Interpreter = (*lexical_state).interpreter;
        let mut ts: *mut TString = luas_newlstr(interpreter, str, length);
        let o: *const TValue = luah_getstr((*lexical_state).table, ts);
        if !(get_tag_type((*o).get_tag2()) == TagType::Nil) {
            ts = &mut (*((*(o as *mut Node)).key.value.object as *mut TString));
        } else {
            let fresh50 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let stv: *mut TValue = &mut (*fresh50).tvalue;
            let io: *mut TValue = stv;
            let x_: *mut TString = ts;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable(true);
            luah_finishset(interpreter, (*lexical_state).table, stv, o, stv);
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        }
        return ts;
    }
}
pub unsafe extern "C" fn inclinenumber(lexical_state: *mut LexicalState) {
    unsafe {
        let old: i32 = (*lexical_state).current;
        let fresh51 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh51 > 0 {
            let fresh52 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh52 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        if ((*lexical_state).current == CHARACTER_LF as i32 || (*lexical_state).current == CHARACTER_CR as i32)
            && (*lexical_state).current != old
        {
            let fresh53 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh53 > 0 {
                let fresh54 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh54 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
        }
        (*lexical_state).line_number += 1;
        if (*lexical_state).line_number >= 0x7FFFFFFF as i32 {
            lexerror(
                lexical_state,
                b"chunk has too many lines\0" as *const u8 as *const i8,
                0,
            );
        }
    }
}
pub unsafe extern "C" fn luax_setinput(
    interpreter: *mut Interpreter,
    lexical_state: *mut LexicalState,
    zio: *mut ZIO,
    source: *mut TString,
    firstchar: i32,
) {
    unsafe {
        (*lexical_state).token.token = 0;
        (*lexical_state).interpreter = interpreter;
        (*lexical_state).current = firstchar;
        (*lexical_state).look_ahead.token = TK_EOS as i32;
        (*lexical_state).zio = zio;
        (*lexical_state).function_state = std::ptr::null_mut();
        (*lexical_state).line_number = 1;
        (*lexical_state).last_line = 1;
        (*lexical_state).source = source;
        (*lexical_state).environment = luas_newlstr(
            interpreter,
            b"_ENV\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        (*(*lexical_state).buffer).pointer = luam_saferealloc_(
            (*lexical_state).interpreter,
            (*(*lexical_state).buffer).pointer as *mut libc::c_void,
            ((*(*lexical_state).buffer).size as usize).wrapping_mul(::core::mem::size_of::<i8>()),
            (32 as usize).wrapping_mul(::core::mem::size_of::<i8>()),
        ) as *mut i8;
        (*(*lexical_state).buffer).size = 32;
    }
}
pub unsafe extern "C" fn check_next1(lexical_state: *mut LexicalState, c: i32) -> i32 {
    unsafe {
        if (*lexical_state).current == c {
            let fresh55 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh55 > 0 {
                let fresh56 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh56 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn check_next2(lexical_state: *mut LexicalState, set: *const i8) -> i32 {
    unsafe {
        if (*lexical_state).current == *set.offset(0 as isize) as i32
            || (*lexical_state).current == *set.offset(1 as isize) as i32
        {
            save(lexical_state, (*lexical_state).current);
            let fresh57 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh57 > 0 {
                let fresh58 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh58 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn read_numeral(
    lexical_state: *mut LexicalState,
    semantic_info: *mut Value,
) -> i32 {
    unsafe {
        let mut obj: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let mut expo: *const i8 = b"Ee\0" as *const u8 as *const i8;
        let first: i32 = (*lexical_state).current;
        save(lexical_state, (*lexical_state).current);
        let fresh59 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh59 > 0 {
            let fresh60 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh60 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        if first == CHARACTER_0 as i32 && check_next2(lexical_state, b"xX\0" as *const u8 as *const i8) != 0
        {
            expo = b"Pp\0" as *const u8 as *const i8;
        }
        loop {
            if check_next2(lexical_state, expo) != 0 {
                check_next2(lexical_state, b"-+\0" as *const u8 as *const i8);
            } else {
                if !(is_digit_hexadecimal((*lexical_state).current + 1)
                    || (*lexical_state).current == CHARACTER_PERIOD as i32)
                {
                    break;
                }
                save(lexical_state, (*lexical_state).current);
                let fresh61 = (*(*lexical_state).zio).length;
                (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                (*lexical_state).current = if fresh61 > 0 {
                    let fresh62 = (*(*lexical_state).zio).pointer;
                    (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                    *fresh62 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
            }
        }
        if is_identifier((*lexical_state).current + 1) {
            save(lexical_state, (*lexical_state).current);
            let fresh63 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh63 > 0 {
                let fresh64 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh64 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
        }
        save(lexical_state, Character::Null as i32);
        if luao_str2num((*(*lexical_state).buffer).pointer, &mut obj) == 0 {
            lexerror(
                lexical_state,
                b"malformed number\0" as *const u8 as *const i8,
                TK_FLT as i32,
            );
        }
        if obj.get_tag2() == TAG_VARIANT_NUMERIC_INTEGER {
            (*semantic_info).integer = obj.value.integer;
            return TK_INT as i32;
        } else {
            (*semantic_info).number = obj.value.number;
            return TK_FLT as i32;
        };
    }
}
pub unsafe extern "C" fn skip_sep(lexical_state: *mut LexicalState) -> u64 {
    unsafe {
        let mut count: u64 = 0;
        let s: i32 = (*lexical_state).current;
        save(lexical_state, (*lexical_state).current);
        let fresh65 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh65 > 0 {
            let fresh66 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh66 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        while (*lexical_state).current == CHARACTER_EQUAL as i32 {
            save(lexical_state, (*lexical_state).current);
            let fresh67 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh67 > 0 {
                let fresh68 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh68 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
            count = count.wrapping_add(1);
        }
        return if (*lexical_state).current == s {
            count.wrapping_add(2 as u64)
        } else {
            (if count == 0 { 1 } else { 0 }) as u64
        };
    }
}
pub unsafe extern "C" fn read_long_string(
    lexical_state: *mut LexicalState,
    semantic_info: *mut Value,
    sep: u64,
) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        save(lexical_state, (*lexical_state).current);
        let fresh69 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh69 > 0 {
            let fresh70 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh70 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        if (*lexical_state).current == CHARACTER_LF as i32 || (*lexical_state).current == CHARACTER_CR as i32 {
            inclinenumber(lexical_state);
        }
        loop {
            match (*lexical_state).current {
                -1 => {
                    let what: *const i8 = if !semantic_info.is_null() {
                        b"string\0" as *const u8 as *const i8
                    } else {
                        b"comment\0" as *const u8 as *const i8
                    };
                    let message: *const i8 = luao_pushfstring(
                        (*lexical_state).interpreter,
                        b"unfinished long %s (starting at line %d)\0" as *const u8 as *const i8,
                        what,
                        line,
                    );
                    lexerror(lexical_state, message, TK_EOS as i32);
                }
                CHARACTER_BRACKET_RIGHT => {
                    if !(skip_sep(lexical_state) == sep) {
                        continue;
                    }
                    save(lexical_state, (*lexical_state).current);
                    let fresh71 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh71 > 0 {
                        let fresh72 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh72 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    break;
                }
                CHARACTER_LF | CHARACTER_CR => {
                    save(lexical_state, CHARACTER_LF as i32);
                    inclinenumber(lexical_state);
                    if semantic_info.is_null() {
                        (*(*lexical_state).buffer).length = 0;
                    }
                }
                _ => {
                    if !semantic_info.is_null() {
                        save(lexical_state, (*lexical_state).current);
                        let fresh73 = (*(*lexical_state).zio).length;
                        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                        (*lexical_state).current = if fresh73 > 0 {
                            let fresh74 = (*(*lexical_state).zio).pointer;
                            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                            *fresh74 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                    } else {
                        let fresh75 = (*(*lexical_state).zio).length;
                        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                        (*lexical_state).current = if fresh75 > 0 {
                            let fresh76 = (*(*lexical_state).zio).pointer;
                            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                            *fresh76 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                    }
                }
            }
        }
        if !semantic_info.is_null() {
            (*semantic_info).tstring = luax_newstring(
                lexical_state,
                ((*(*lexical_state).buffer).pointer).offset(sep as isize),
                ((*(*lexical_state).buffer).length).wrapping_sub((2 as usize).wrapping_mul(sep as usize)) as u64,
            );
        }
    }
}
pub unsafe extern "C" fn esccheck(lexical_state: *mut LexicalState, condition: bool, message: *const i8) {
    unsafe {
        if !condition {
            if (*lexical_state).current != -1 {
                save(lexical_state, (*lexical_state).current);
                let fresh77 = (*(*lexical_state).zio).length;
                (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                (*lexical_state).current = if fresh77 > 0 {
                    let fresh78 = (*(*lexical_state).zio).pointer;
                    (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                    *fresh78 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
            }
            lexerror(lexical_state, message, TK_STRING as i32);
        }
    }
}
pub unsafe extern "C" fn gethexa(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        save(lexical_state, (*lexical_state).current);
        let fresh79 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh79 > 0 {
            let fresh80 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh80 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        esccheck(
            lexical_state,
            is_digit_hexadecimal((*lexical_state).current + 1),
            b"hexadecimal digit expected\0" as *const u8 as *const i8,
        );
        return get_hexadecimal_digit_value((*lexical_state).current);
    }
}
pub unsafe extern "C" fn readhexaesc(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut r: i32 = gethexa(lexical_state);
        r = (r << 4) + gethexa(lexical_state);
        (*(*lexical_state).buffer).length =
            ((*(*lexical_state).buffer).length).wrapping_sub(2);
        return r;
    }
}
pub unsafe extern "C" fn readutf8esc(lexical_state: *mut LexicalState) -> u64 {
    unsafe {
        let mut i: i32 = 4;
        save(lexical_state, (*lexical_state).current);
        let fresh81 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh81 > 0 {
            let fresh82 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh82 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        esccheck(
            lexical_state,
            (*lexical_state).current == CHARACTER_BRACE_LEFT,
            b"missing CHARACTER_BRACE_LEFT\0" as *const u8 as *const i8,
        );
        let mut r: u64 = gethexa(lexical_state) as u64;
        loop {
            save(lexical_state, (*lexical_state).current);
            let fresh83 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh83 > 0 {
                let fresh84 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh84 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
            if !is_digit_hexadecimal((*lexical_state).current + 1) {
                break;
            }
            i += 1;
            esccheck(
                lexical_state,
                r <= (0x7fffffff as u64 >> 4),
                b"UTF-8 value too large\0" as *const u8 as *const i8,
            );
            r = (r << 4).wrapping_add(get_hexadecimal_digit_value((*lexical_state).current) as u64);
        }
        esccheck(
            lexical_state,
            (*lexical_state).current == CHARACTER_BRACE_RIGHT,
            b"missing CHARACTER_BRACE_RIGHT\0" as *const u8 as *const i8,
        );
        let fresh85 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh85 > 0 {
            let fresh86 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh86 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        (*(*lexical_state).buffer).length =
            ((*(*lexical_state).buffer).length).wrapping_sub(i as usize);
        return r;
    }
}
pub unsafe extern "C" fn utf8esc(lexical_state: *mut LexicalState) {
    unsafe {
        let mut buffer: [i8; 8] = [0; 8];
        let mut n: i32 = luao_utf8esc(buffer.as_mut_ptr(), readutf8esc(lexical_state));
        while n > 0 {
            save(lexical_state, buffer[(8 - n) as usize] as i32);
            n -= 1;
        }
    }
}
pub unsafe extern "C" fn readdecesc(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut i: i32;
        let mut r: i32 = 0;
        i = 0;
        while i < 3 && is_digit_decimal((*lexical_state).current + 1) {
            r = 10 as i32 * r + (*lexical_state).current - CHARACTER_0 as i32;
            save(lexical_state, (*lexical_state).current);
            let fresh87 = (*(*lexical_state).zio).length;
            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
            (*lexical_state).current = if fresh87 > 0 {
                let fresh88 = (*(*lexical_state).zio).pointer;
                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                *fresh88 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
            i += 1;
        }
        esccheck(
            lexical_state,
            r <= 127 * 2 + 1,
            b"decimal escape too large\0" as *const u8 as *const i8,
        );
        (*(*lexical_state).buffer).length =
            ((*(*lexical_state).buffer).length).wrapping_sub(i as usize);
        return r;
    }
}
pub unsafe extern "C" fn read_string(
    lexical_state: *mut LexicalState,
    del: i32,
    semantic_info: *mut Value,
) {
    unsafe {
        let mut current_block: u64;
        save(lexical_state, (*lexical_state).current);
        let fresh89 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh89 > 0 {
            let fresh90 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh90 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        while (*lexical_state).current != del {
            match (*lexical_state).current {
                -1 => {
                    lexerror(
                        lexical_state,
                        b"unfinished string\0" as *const u8 as *const i8,
                        TK_EOS as i32,
                    );
                }
                CHARACTER_LF | CHARACTER_CR => {
                    lexerror(
                        lexical_state,
                        b"unfinished string\0" as *const u8 as *const i8,
                        TK_STRING as i32,
                    );
                }
                CHARACTER_BACKSLASH => {
                    let c: i32;
                    save(lexical_state, (*lexical_state).current);
                    let fresh91 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh91 > 0 {
                        let fresh92 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh92 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    match (*lexical_state).current {
                        CHARACTER_LOWER_A => {
                            c = CHARACTER_BEL as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_B => {
                            c = CHARACTER_BS as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_F => {
                            c = CHARACTER_FF as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_N => {
                            c = CHARACTER_LF as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_R => {
                            c = CHARACTER_CR as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_T => {
                            c = CHARACTER_HT as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_V => {
                            c = CHARACTER_VT as i32;
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_X => {
                            c = readhexaesc(lexical_state);
                            current_block = 15029063370732930705;
                        }
                        CHARACTER_LOWER_U => {
                            utf8esc(lexical_state);
                            continue;
                        }
                        CHARACTER_CR | CHARACTER_LF => {
                            inclinenumber(lexical_state);
                            c = CHARACTER_LF as i32;
                            current_block = 7010296663004816197;
                        }
                        CHARACTER_BACKSLASH | CHARACTER_DOUBLEQUOTE | CHARACTER_QUOTE => {
                            c = (*lexical_state).current;
                            current_block = 15029063370732930705;
                        }
                        -1 => {
                            continue;
                        }
                        CHARACTER_LOWER_Z => {
                            (*(*lexical_state).buffer).length =
                                ((*(*lexical_state).buffer).length).wrapping_sub(1);
                            let fresh93 = (*(*lexical_state).zio).length;
                            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                            (*lexical_state).current = if fresh93 > 0 {
                                let fresh94 = (*(*lexical_state).zio).pointer;
                                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                *fresh94 as u8 as i32
                            } else {
                                luaz_fill((*lexical_state).zio)
                            };
                            while is_whitespace((*lexical_state).current + 1) {
                                if (*lexical_state).current == CHARACTER_LF as i32
                                    || (*lexical_state).current == CHARACTER_CR as i32
                                {
                                    inclinenumber(lexical_state);
                                } else {
                                    let fresh95 = (*(*lexical_state).zio).length;
                                    (*(*lexical_state).zio).length =
                                        ((*(*lexical_state).zio).length).wrapping_sub(1);
                                    (*lexical_state).current = if fresh95 > 0 {
                                        let fresh96 = (*(*lexical_state).zio).pointer;
                                        (*(*lexical_state).zio).pointer =
                                            ((*(*lexical_state).zio).pointer).offset(1);
                                        *fresh96 as u8 as i32
                                    } else {
                                        luaz_fill((*lexical_state).zio)
                                    };
                                }
                            }
                            continue;
                        }
                        _ => {
                            esccheck(
                                lexical_state,
                                is_digit_decimal((*lexical_state).current + 1),
                                b"invalid escape sequence\0" as *const u8 as *const i8,
                            );
                            c = readdecesc(lexical_state);
                            current_block = 7010296663004816197;
                        }
                    }
                    match current_block {
                        15029063370732930705 => {
                            let fresh97 = (*(*lexical_state).zio).length;
                            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                            (*lexical_state).current = if fresh97 > 0 {
                                let fresh98 = (*(*lexical_state).zio).pointer;
                                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                *fresh98 as u8 as i32
                            } else {
                                luaz_fill((*lexical_state).zio)
                            };
                        }
                        _ => {}
                    }
                    (*(*lexical_state).buffer).length = ((*(*lexical_state).buffer).length).wrapping_sub(1);
                    save(lexical_state, c);
                }
                _ => {
                    save(lexical_state, (*lexical_state).current);
                    let fresh99 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh99 > 0 {
                        let fresh100 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh100 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                }
            }
        }
        save(lexical_state, (*lexical_state).current);
        let fresh101 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh101 > 0 {
            let fresh102 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh102 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        (*semantic_info).tstring = luax_newstring(
            lexical_state,
            ((*(*lexical_state).buffer).pointer).offset(1 as isize),
            ((*(*lexical_state).buffer).length).wrapping_sub(2) as u64,
        );
    }
}
pub unsafe extern "C" fn llex(
    lexical_state: *mut LexicalState,
    semantic_info: *mut Value,
) -> i32 {
    unsafe {
        (*(*lexical_state).buffer).length = 0;
        loop {
            let current_block_85: u64;
            match (*lexical_state).current {
                CHARACTER_LF | CHARACTER_CR => {
                    inclinenumber(lexical_state);
                }
                CHARACTER_SPACE | CHARACTER_FF | CHARACTER_HT | CHARACTER_VT => {
                    let fresh103 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh103 > 0 {
                        let fresh104 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh104 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                }
                CHARACTER_HYPHEN => {
                    let fresh105 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh105 > 0 {
                        let fresh106 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh106 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if (*lexical_state).current != CHARACTER_HYPHEN as i32 {
                        return CHARACTER_HYPHEN as i32;
                    }
                    let fresh107 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh107 > 0 {
                        let fresh108 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh108 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if (*lexical_state).current == CHARACTER_BRACKET_LEFT as i32 {
                        let sep: u64 = skip_sep(lexical_state);
                        (*(*lexical_state).buffer).length = 0;
                        if sep >= 2 as u64 {
                            read_long_string(lexical_state, std::ptr::null_mut(), sep);
                            (*(*lexical_state).buffer).length = 0;
                            current_block_85 = 10512632378975961025;
                        } else {
                            current_block_85 = 3512920355445576850;
                        }
                    } else {
                        current_block_85 = 3512920355445576850;
                    }
                    match current_block_85 {
                        10512632378975961025 => {}
                        _ => {
                            while !((*lexical_state).current == CHARACTER_LF as i32
                                || (*lexical_state).current == CHARACTER_CR as i32)
                                && (*lexical_state).current != -1
                            {
                                let fresh109 = (*(*lexical_state).zio).length;
                                (*(*lexical_state).zio).length =
                                    ((*(*lexical_state).zio).length).wrapping_sub(1);
                                (*lexical_state).current = if fresh109 > 0 {
                                    let fresh110 = (*(*lexical_state).zio).pointer;
                                    (*(*lexical_state).zio).pointer =
                                        ((*(*lexical_state).zio).pointer).offset(1);
                                    *fresh110 as u8 as i32
                                } else {
                                    luaz_fill((*lexical_state).zio)
                                };
                            }
                        }
                    }
                }
                CHARACTER_BRACKET_LEFT => {
                    let sep_0: u64 = skip_sep(lexical_state);
                    if sep_0 >= 2 as u64 {
                        read_long_string(lexical_state, semantic_info, sep_0);
                        return TK_STRING as i32;
                    } else if sep_0 == 0 {
                        lexerror(
                            lexical_state,
                            b"invalid long string delimiter\0" as *const u8 as *const i8,
                            TK_STRING as i32,
                        );
                    }
                    return CHARACTER_BRACKET_LEFT as i32;
                }
                CHARACTER_EQUAL => {
                    let fresh111 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh111 > 0 {
                        let fresh112 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh112 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_EQUAL as i32) != 0 {
                        return TK_EQ as i32;
                    } else {
                        return CHARACTER_EQUAL as i32;
                    }
                }
                CHARACTER_ANGLE_LEFT => {
                    let fresh113 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh113 > 0 {
                        let fresh114 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh114 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_EQUAL as i32) != 0 {
                        return TK_LE as i32;
                    } else if check_next1(lexical_state, CHARACTER_ANGLE_LEFT) != 0 {
                        return TK_SHL as i32;
                    } else {
                        return CHARACTER_ANGLE_LEFT;
                    }
                }
                CHARACTER_ANGLE_RIGHT => {
                    let fresh115 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh115 > 0 {
                        let fresh116 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh116 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_EQUAL) != 0 {
                        return TK_GE as i32;
                    } else if check_next1(lexical_state, CHARACTER_ANGLE_RIGHT) != 0 {
                        return TK_SHR as i32;
                    } else {
                        return CHARACTER_ANGLE_RIGHT;
                    }
                }
                CHARACTER_SOLIDUS => {
                    let fresh117 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh117 > 0 {
                        let fresh118 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh118 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_SOLIDUS) != 0 {
                        return TK_IDIV as i32;
                    } else {
                        return CHARACTER_SOLIDUS;
                    }
                }
                CHARACTER_TILDE => {
                    let fresh119 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh119 > 0 {
                        let fresh120 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh120 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_EQUAL) != 0 {
                        return TK_NE as i32;
                    } else {
                        return CHARACTER_TILDE;
                    }
                }
                CHARACTER_COLON => {
                    let fresh121 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh121 > 0 {
                        let fresh122 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh122 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_COLON as i32) != 0 {
                        return TK_DBCOLON as i32;
                    } else {
                        return CHARACTER_COLON as i32;
                    }
                }
                CHARACTER_QUOTE | CHARACTER_DOUBLEQUOTE => {
                    read_string(lexical_state, (*lexical_state).current, semantic_info);
                    return TK_STRING as i32;
                }
                CHARACTER_PERIOD => {
                    save(lexical_state, (*lexical_state).current);
                    let fresh123 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh123 > 0 {
                        let fresh124 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh124 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    if check_next1(lexical_state, CHARACTER_PERIOD as i32) != 0 {
                        if check_next1(lexical_state, CHARACTER_PERIOD as i32) != 0 {
                            return TK_DOTS as i32;
                        } else {
                            return TK_CONCAT as i32;
                        }
                    } else if !is_digit_decimal((*lexical_state).current + 1) {
                        return CHARACTER_PERIOD as i32;
                    } else {
                        return read_numeral(lexical_state, semantic_info);
                    }
                }
                CHARACTER_0 | CHARACTER_1 | CHARACTER_2 | CHARACTER_3 | CHARACTER_4 | CHARACTER_5 | CHARACTER_6 | CHARACTER_7 | CHARACTER_8 | CHARACTER_9 => {
                    return read_numeral(lexical_state, semantic_info);
                }
                -1 => return TK_EOS as i32,
                _ => {
                    if is_identifier((*lexical_state).current + 1) {
                        loop {
                            save(lexical_state, (*lexical_state).current);
                            let fresh125 = (*(*lexical_state).zio).length;
                            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                            (*lexical_state).current = if fresh125 > 0 {
                                let fresh126 = (*(*lexical_state).zio).pointer;
                                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                *fresh126 as u8 as i32
                            } else {
                                luaz_fill((*lexical_state).zio)
                            };
                            if !is_alphanumeric((*lexical_state).current + 1)
                            {
                                break;
                            }
                        }
                        let ts: *mut TString = luax_newstring(
                            lexical_state,
                            (*(*lexical_state).buffer).pointer,
                            (*(*lexical_state).buffer).length as u64,
                        );
                        (*semantic_info).tstring = ts;
                        if (*ts).get_tag() == TAG_VARIANT_STRING_SHORT && (*ts).extra as i32 > 0 {
                            return (*ts).extra as i32 - 1 + (127 as i32 * 2 + 1 + 1);
                        } else {
                            return TK_NAME as i32;
                        }
                    } else {
                        let c: i32 = (*lexical_state).current;
                        let fresh127 = (*(*lexical_state).zio).length;
                        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                        (*lexical_state).current = if fresh127 > 0 {
                            let fresh128 = (*(*lexical_state).zio).pointer;
                            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                            *fresh128 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                        return c;
                    }
                }
            }
        }
    }
}
pub unsafe extern "C" fn luax_next(lexical_state: *mut LexicalState) {
    unsafe {
        (*lexical_state).last_line = (*lexical_state).line_number;
        if (*lexical_state).look_ahead.token != TK_EOS as i32 {
            (*lexical_state).token = (*lexical_state).look_ahead;
            (*lexical_state).look_ahead.token = TK_EOS as i32;
        } else {
            (*lexical_state).token.token = llex(lexical_state, &mut (*lexical_state).token.semantic_info);
        };
    }
}
pub unsafe extern "C" fn luax_lookahead(lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        (*lexical_state).look_ahead.token = llex(
            lexical_state,
            &mut (*lexical_state).look_ahead.semantic_info,
        );
        return (*lexical_state).look_ahead.token;
    }
}
pub unsafe extern "C" fn luak_semerror(lexical_state: *mut LexicalState, message: *const i8) -> ! {
    unsafe {
        (*lexical_state).token.token = 0;
        luax_syntaxerror(lexical_state, message);
    }
}

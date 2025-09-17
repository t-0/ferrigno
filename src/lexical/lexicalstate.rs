use crate::buffer::*;
use crate::character::*;
use crate::dynamicdata::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::functionstate::*;
use crate::interpreter::*;
use crate::labeldescription::*;
use crate::lexical::blockcontrol::*;
use crate::lexical::constructorcontrol::*;
use crate::lexical::operatorbinary::*;
use crate::lexical::operatorunary::*;
use crate::lexical::priority::*;
use crate::localvariable::*;
use crate::new::*;
use crate::node::*;
use crate::object::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::token::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::value::*;
use crate::variabledescription::*;
use crate::vectort::*;
use crate::vm::opcode::*;
use crate::zio::*;
use libc::*;
use rlua::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexicalState {
    pub current: i32,
    pub line_number: i32,
    pub last_line: i32,
    pub token: Token,
    pub look_ahead: Token,
    pub lexical_state_function_state: *mut FunctionState,
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
            lexical_state_function_state: null_mut(),
            zio: null_mut(),
            buffer: null_mut(),
            table: null_mut(),
            dynamic_data: null_mut(),
            source: null_mut(),
            environment: null_mut(),
        };
    }
}
impl LexicalState {
    pub unsafe fn create_label(&mut self, interpreter: *mut Interpreter, function_state: *mut FunctionState, name: *mut TString, line: i32, is_last: bool) -> bool {
        unsafe {
            let ll: *mut VectorT<LabelDescription> = &mut (*self.dynamic_data).labels;
            let l: i32 = newlabelentry(interpreter, self, function_state, ll, name, line, (*function_state).code_get_label());
            if is_last {
                (*((*ll).vectort_pointer).offset(l as isize)).count_active_variables = (*(*function_state).block_control).count_active_variables;
            }
            if solvegotos(interpreter, self, function_state, &mut *((*ll).vectort_pointer).offset(l as isize)) {
                code_abck(interpreter, self, function_state, OP_CLOSE, luay_nvarstack(self, function_state), 0, 0, 0);
                return true;
            }
            return false;
        }
    }
    pub unsafe fn parse_expression(&mut self, interpreter: *mut Interpreter, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
        unsafe {
            subexpr(interpreter, self, function_state, expression_description, 0);
        }
    }
    pub unsafe fn parse_expression_list(&mut self, interpreter: *mut Interpreter, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) -> i32 {
        unsafe {
            let mut count: i32 = 1;
            self.parse_expression(interpreter, function_state, expression_description);
            while testnext(interpreter, self, function_state, CHARACTER_COMMA as i32) != 0 {
                luak_exp2nextreg(interpreter, self, self.lexical_state_function_state, expression_description);
                self.parse_expression(interpreter, function_state, expression_description);
                count += 1;
            }
            return count;
        }
    }
    pub unsafe fn add_prototype(&mut self, interpreter: *mut Interpreter) -> *mut Prototype {
        unsafe {
            let function_state: *mut FunctionState = self.lexical_state_function_state;
            let prototype: *mut Prototype = (*function_state).prototype;
            if (*function_state).count_prototypes >= (*prototype).prototype_prototypes.get_size() as i32 {
                let mut old_size = (*prototype).prototype_prototypes.get_size();
                (*prototype).prototype_prototypes.grow(
                    interpreter,
                    (*function_state).count_prototypes as usize,
                    if ((1 << 8 + 8 + 1) - 1) as usize <= (!(0usize)).wrapping_div(size_of::<*mut Prototype>() as usize) {
                        (1 << 8 + 8 + 1) - 1
                    } else {
                        (!(0usize)).wrapping_div(size_of::<*mut Prototype>() as usize)
                    },
                    make_cstring!("functions"),
                );
                while old_size < (*prototype).prototype_prototypes.get_size() {
                    let fresh45 = old_size;
                    old_size = old_size + 1;
                    let ref mut fresh46 = *((*prototype).prototype_prototypes.vectort_pointer).offset(fresh45 as isize);
                    *fresh46 = null_mut();
                }
            }
            let clp: *mut Prototype = luaf_newproto(interpreter);
            let np = (*function_state).count_prototypes;
            (*function_state).count_prototypes = (*function_state).count_prototypes + 1;
            let ref mut target = *((*prototype).prototype_prototypes.vectort_pointer).offset(np as isize);
            *target = clp;
            if (*prototype).get_marked() & 1 << 5 != 0 && (*clp).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(interpreter, &mut (*(prototype as *mut Object)), &mut (*(clp as *mut Object)));
            } else {
            };
            return clp;
        }
    }
}
pub unsafe fn find_label(function_state: *mut FunctionState, dynamic_data: *mut DynamicData, name: *mut TString) -> *mut LabelDescription {
    unsafe {
        for i in (*function_state).first_label..(*dynamic_data).labels.get_length() as i32 {
            let candidate = &mut *((*dynamic_data).labels.vectort_pointer).offset(i as isize);
            if (*candidate).name == name {
                return candidate;
            }
        }
        return null_mut();
    }
}
pub unsafe fn newlabelentry(interpreter: *mut Interpreter, _lexical_state: *mut LexicalState, function_state: *mut FunctionState, l: *mut VectorT<LabelDescription>, name: *mut TString, line: i32, program_counter: i32) -> i32 {
    unsafe {
        let n = (*l).get_length();
        (*l).grow(
            interpreter,
            n as usize,
            if 32767 as usize <= (!(0usize)).wrapping_div(size_of::<LabelDescription>() as usize) {
                32767
            } else {
                (!(0usize)).wrapping_div(size_of::<LabelDescription>() as usize)
            },
            make_cstring!("labels/gotos"),
        );
        let ref mut fresh44 = (*((*l).vectort_pointer).offset(n as isize)).name;
        *fresh44 = name;
        (*((*l).vectort_pointer).offset(n as isize)).line = line;
        (*((*l).vectort_pointer).offset(n as isize)).count_active_variables = (*function_state).count_active_variables;
        (*((*l).vectort_pointer).offset(n as isize)).close = 0;
        (*((*l).vectort_pointer).offset(n as isize)).program_counter = program_counter;
        (*l).set_length(n as usize + 1);
        return n as i32;
    }
}
pub unsafe fn newgotoentry(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, name: *mut TString, line: i32, program_counter: i32) -> i32 {
    unsafe {
        return newlabelentry(interpreter, lexical_state, function_state, &mut (*(*lexical_state).dynamic_data).goto_, name, line, program_counter);
    }
}
pub unsafe fn solvegotos(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, label_description: *mut LabelDescription) -> bool {
    unsafe {
        let gl: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).dynamic_data).goto_;
        let mut i: i32 = (*(*function_state).block_control).first_goto;
        let mut needsclose = false;
        while i < (*gl).get_length() as i32 {
            if (*((*gl).vectort_pointer).offset(i as isize)).name == (*label_description).name {
                needsclose = needsclose || (0 != (*((*gl).vectort_pointer).offset(i as isize)).close);
                solvegoto(interpreter, lexical_state, i, label_description);
            } else {
                i += 1;
            }
        }
        return needsclose;
    }
}
pub unsafe fn undefgoto(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, _function_state: *mut FunctionState, goto_label_description: *mut LabelDescription) -> ! {
    unsafe {
        let mut message: *const i8;
        if (*goto_label_description).name == luas_newlstr(interpreter, make_cstring!("break"), (size_of::<[i8; 6]>()).wrapping_div(size_of::<i8>()).wrapping_sub(1)) {
            message = make_cstring!("break outside loop at line %d");
            message = luao_pushfstring(interpreter, message, (*goto_label_description).line);
        } else {
            message = make_cstring!("no visible label '%s' for <goto> at line %d");
            message = luao_pushfstring(interpreter, message, (*(*goto_label_description).name).get_contents_mut(), (*goto_label_description).line);
        }
        luak_semerror(interpreter, lexical_state, message);
    }
}
pub unsafe fn codeclosure(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*(*lexical_state).lexical_state_function_state).function_state_previous;
        init_exp(
            v,
            ExpressionKind::Relocatable,
            luak_codeabx(interpreter, lexical_state, function_state, OP_CLOSURE, 0, ((*function_state).count_prototypes - 1) as u32),
        );
        luak_exp2nextreg(interpreter, lexical_state, function_state, v);
    }
}
pub unsafe fn open_function(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, previous: *mut FunctionState, block_control: *mut BlockControl) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        (*function_state).function_state_previous = previous;
        (*lexical_state).lexical_state_function_state = function_state;
        (*function_state).program_counter = 0;
        (*function_state).previous_line = (*prototype).prototype_line_defined;
        (*function_state).iwthabs = 0;
        (*function_state).last_target = 0;
        (*function_state).freereg = 0;
        (*function_state).count_constants = 0;
        (*function_state).count_abslineinfo = 0;
        (*function_state).count_prototypes = 0;
        (*function_state).count_upvalues = 0;
        (*function_state).count_debug_variables = 0 as i16;
        (*function_state).count_active_variables = 0;
        (*function_state).needs_close = false;
        (*function_state).first_local = (*(*lexical_state).dynamic_data).active_variables.get_length() as i32;
        (*function_state).first_label = (*(*lexical_state).dynamic_data).labels.get_length() as i32;
        (*function_state).block_control = null_mut();
        (*prototype).prototype_source = (*lexical_state).source;
        if (*prototype).get_marked() & 1 << 5 != 0 && (*(*prototype).prototype_source).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(prototype as *mut Object)), &mut (*((*prototype).prototype_source as *mut Object)));
        } else {
        };
        (*prototype).prototype_maximum_stack_size = 2 as u8;
        enterblock(lexical_state, function_state, block_control, false);
    }
}
pub unsafe fn close_function(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        luak_ret(interpreter, lexical_state, function_state, luay_nvarstack(lexical_state, function_state), 0);
        leaveblock(interpreter, lexical_state, function_state);
        luak_finish(interpreter, lexical_state, function_state, (*function_state).prototype);
        (*prototype).prototype_code.shrink(&mut *interpreter, (*function_state).program_counter as usize);
        (*prototype).prototype_line_info.shrink(&mut *interpreter, (*function_state).program_counter as usize);
        (*prototype).prototype_absolute_line_info.shrink(&mut *interpreter, (*function_state).count_abslineinfo as usize);
        (*prototype).prototype_constants.shrink(&mut *interpreter, (*function_state).count_constants as usize);
        (*prototype).prototype_prototypes.shrink(&mut *interpreter, (*function_state).count_prototypes as usize);
        (*prototype).prototype_local_variables.shrink(&mut *interpreter, (*function_state).count_debug_variables as usize);
        (*prototype).prototype_upvalues.shrink(&mut *interpreter, (*function_state).count_upvalues as usize);
        (*lexical_state).lexical_state_function_state = (*function_state).function_state_previous;
        if (*(*interpreter).global).gc_debt > 0 {
            (*interpreter).luac_step();
        }
    }
}
pub unsafe fn parse_statement_list(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        while !block_follow_with_until((*lexical_state).token.token) {
            if (*lexical_state).token.token == TK_RETURN {
                parse_statement(interpreter, lexical_state, function_state);
                return;
            } else {
                parse_statement(interpreter, lexical_state, function_state);
            }
        }
    }
}
pub unsafe fn fieldsel(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) {
    unsafe {
        let mut key: ExpressionDescription = ExpressionDescription::new();
        luak_exp2anyregup(interpreter, lexical_state, function_state, v);
        luax_next(interpreter, lexical_state);
        codename(interpreter, lexical_state, function_state, &mut key);
        luak_indexed(interpreter, lexical_state, function_state, v, &mut key);
    }
}
pub unsafe fn yindex(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) {
    unsafe {
        luax_next(interpreter, lexical_state);
        (*lexical_state).parse_expression(interpreter, function_state, v);
        code_expression_to_value(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, v);
        checknext(interpreter, lexical_state, function_state, CHARACTER_BRACKET_RIGHT as i32);
    }
}
pub unsafe fn recfield(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, constructor_control: *mut ConstructorControl) {
    unsafe {
        let reg: i32 = (*(*lexical_state).lexical_state_function_state).freereg as i32;
        let mut key: ExpressionDescription = ExpressionDescription::new();
        let mut value: ExpressionDescription = ExpressionDescription::new();
        if (*lexical_state).token.token == TK_NAME as i32 {
            codename(interpreter, lexical_state, function_state, &mut key);
        } else {
            yindex(interpreter, lexical_state, function_state, &mut key);
        }
        checklimit(
            interpreter,
            lexical_state,
            function_state,
            (*constructor_control).count_table,
            0x7FFFFFFF as i32,
            make_cstring!("items in a constructor"),
        );
        (*constructor_control).count_table += 1;
        checknext(interpreter, lexical_state, function_state, CHARACTER_EQUAL as i32);
        let mut table: ExpressionDescription = *(*constructor_control).constructor_control_table;
        luak_indexed(interpreter, lexical_state, function_state, &mut table, &mut key);
        (*lexical_state).parse_expression(interpreter, function_state, &mut value);
        luak_storevar(interpreter, lexical_state, function_state, &mut table, &mut value);
        (*function_state).freereg = reg as u8;
    }
}
pub unsafe fn listfield(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, constructor_control: *mut ConstructorControl) {
    unsafe {
        (*lexical_state).parse_expression(interpreter, function_state, &mut (*constructor_control).expression_description);
        (*constructor_control).count_to_store += 1;
        (*constructor_control).count_to_store;
    }
}
pub unsafe fn field(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, constructor_control: *mut ConstructorControl) {
    unsafe {
        match (*lexical_state).token.token {
            291 => {
                if luax_lookahead(interpreter, lexical_state) != CHARACTER_EQUAL as i32 {
                    listfield(interpreter, lexical_state, function_state, constructor_control);
                } else {
                    recfield(interpreter, lexical_state, function_state, constructor_control);
                }
            },
            91 => {
                recfield(interpreter, lexical_state, function_state, constructor_control);
            },
            _ => {
                listfield(interpreter, lexical_state, function_state, constructor_control);
            },
        };
    }
}
pub unsafe fn constructor(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, t: *mut ExpressionDescription) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        let program_counter: i32 = code_abck(interpreter, lexical_state, function_state, OP_NEWTABLE, 0, 0, 0, 0);
        let mut constructor_control: ConstructorControl = ConstructorControl::new();
        luak_code(interpreter, lexical_state, function_state, 0u32);
        constructor_control.count_to_store = 0;
        constructor_control.count_table = 0;
        constructor_control.count_array = 0;
        constructor_control.constructor_control_table = t;
        init_exp(t, ExpressionKind::Nonrelocatable, (*function_state).freereg as i32);
        luak_reserveregs(interpreter, lexical_state, function_state, 1);
        init_exp(&mut constructor_control.expression_description, ExpressionKind::Void, 0);
        checknext(interpreter, lexical_state, function_state, CHARACTER_BRACE_LEFT as i32);
        while !((*lexical_state).token.token == CHARACTER_BRACE_RIGHT as i32) {
            closelistfield(interpreter, lexical_state, function_state, &mut constructor_control);
            field(interpreter, lexical_state, function_state, &mut constructor_control);
            if !(testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0 || testnext(interpreter, lexical_state, function_state, CHARACTER_SEMICOLON as i32) != 0) {
                break;
            }
        }
        check_match(interpreter, lexical_state, function_state, CHARACTER_BRACE_RIGHT as i32, CHARACTER_BRACE_LEFT as i32, line);
        lastlistfield(interpreter, lexical_state, function_state, &mut constructor_control);
        luak_settablesize(interpreter, function_state, program_counter, (*t).value.info, constructor_control.count_array, constructor_control.count_table);
    }
}
pub unsafe fn parlist(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut nparams: i32 = 0;
        let mut is_variable_arguments = false;
        if (*lexical_state).token.token != CHARACTER_PARENTHESIS_RIGHT as i32 {
            loop {
                match (*lexical_state).token.token {
                    291 => {
                        new_localvar(interpreter, lexical_state, str_checkname(interpreter, lexical_state, function_state));
                        nparams += 1;
                    },
                    280 => {
                        luax_next(interpreter, lexical_state);
                        is_variable_arguments = true;
                    },
                    _ => {
                        luax_syntaxerror(interpreter, lexical_state, make_cstring!("<name> or '...' expected"));
                    },
                }
                if !(!is_variable_arguments && testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0) {
                    break;
                }
            }
        }
        adjustlocalvars(interpreter, lexical_state, nparams);
        (*prototype).prototype_count_parameters = (*function_state).count_active_variables;
        if is_variable_arguments {
            setvararg(interpreter, lexical_state, function_state, (*prototype).prototype_count_parameters as i32);
        }
        luak_reserveregs(interpreter, lexical_state, function_state, (*function_state).count_active_variables as i32);
    }
}
pub unsafe fn body(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription, is_method: bool, line: i32) {
    unsafe {
        let mut new_fs: FunctionState = FunctionState {
            prototype: null_mut(),
            function_state_previous: null_mut(),
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
        let mut block_control = BlockControl::new();
        new_fs.prototype = (*lexical_state).add_prototype(interpreter);
        (*new_fs.prototype).prototype_line_defined = line;
        open_function(interpreter, lexical_state, &mut new_fs, function_state, &mut block_control);
        checknext(interpreter, lexical_state, function_state, CHARACTER_PARENTHESIS_LEFT as i32);
        if is_method {
            new_localvar(
                interpreter,
                lexical_state,
                luax_newstring(interpreter, lexical_state, make_cstring!("self"), (size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize)),
            );
            adjustlocalvars(interpreter, lexical_state, 1);
        }
        parlist(interpreter, lexical_state);
        checknext(interpreter, lexical_state, function_state, CHARACTER_PARENTHESIS_RIGHT as i32);
        parse_statement_list(interpreter, lexical_state, function_state);
        (*new_fs.prototype).prototype_last_line_defined = (*lexical_state).line_number;
        check_match(interpreter, lexical_state, function_state, TK_END as i32, TK_FUNCTION as i32, line);
        codeclosure(interpreter, lexical_state, expression_description);
        close_function(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
    }
}
pub unsafe fn funcargs(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        let mut args: ExpressionDescription = ExpressionDescription::new();
        let line: i32 = (*lexical_state).line_number;
        match (*lexical_state).token.token {
            CHARACTER_PARENTHESIS_LEFT => {
                luax_next(interpreter, lexical_state);
                if (*lexical_state).token.token == CHARACTER_PARENTHESIS_RIGHT as i32 {
                    args.expression_kind = ExpressionKind::Void;
                } else {
                    (*lexical_state).parse_expression_list(interpreter, function_state, &mut args);
                    if args.expression_kind == ExpressionKind::Call || args.expression_kind == ExpressionKind::VariableArguments {
                        luak_setreturns(interpreter, lexical_state, function_state, &mut args, -1);
                    }
                }
                check_match(interpreter, lexical_state, function_state, CHARACTER_PARENTHESIS_RIGHT as i32, CHARACTER_PARENTHESIS_LEFT as i32, line);
            },
            CHARACTER_BRACE_LEFT => {
                constructor(interpreter, lexical_state, function_state, &mut args);
            },
            292 => {
                codestring(&mut args, (*lexical_state).token.semantic_info.tstring);
                luax_next(interpreter, lexical_state);
            },
            _ => {
                luax_syntaxerror(interpreter, lexical_state, make_cstring!("function arguments expected"));
            },
        }
        let base: i32 = (*expression_description).value.info;
        let nparams: i32;
        if args.expression_kind == ExpressionKind::Call || args.expression_kind == ExpressionKind::VariableArguments {
            nparams = -1;
        } else {
            if args.expression_kind != ExpressionKind::Void {
                luak_exp2nextreg(interpreter, lexical_state, function_state, &mut args);
            }
            nparams = (*function_state).freereg as i32 - (base + 1);
        }
        init_exp(
            expression_description,
            ExpressionKind::Call,
            code_abck(interpreter, lexical_state, function_state, OP_CALL, base, nparams + 1, 2, 0),
        );
        luak_fixline(interpreter, lexical_state, function_state, line);
        (*function_state).freereg = (base + 1) as u8;
    }
}
pub unsafe fn primaryexp(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) {
    unsafe {
        match (*lexical_state).token.token {
            40 => {
                let line: i32 = (*lexical_state).line_number;
                luax_next(interpreter, lexical_state);
                (*lexical_state).parse_expression(interpreter, function_state, v);
                check_match(interpreter, lexical_state, function_state, CHARACTER_PARENTHESIS_RIGHT as i32, CHARACTER_PARENTHESIS_LEFT as i32, line);
                luak_dischargevars(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, v);
                return;
            },
            291 => {
                singlevar(interpreter, lexical_state, function_state, v);
                return;
            },
            _ => {
                luax_syntaxerror(interpreter, lexical_state, make_cstring!("unexpected symbol"));
            },
        };
    }
}
pub unsafe fn suffixedexp(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) {
    unsafe {
        primaryexp(interpreter, lexical_state, function_state, v);
        loop {
            match (*lexical_state).token.token {
                46 => {
                    fieldsel(interpreter, lexical_state, function_state, v);
                },
                91 => {
                    let mut key: ExpressionDescription = ExpressionDescription::new();
                    luak_exp2anyregup(interpreter, lexical_state, function_state, v);
                    yindex(interpreter, lexical_state, function_state, &mut key);
                    luak_indexed(interpreter, lexical_state, function_state, v, &mut key);
                },
                58 => {
                    let mut key_0: ExpressionDescription = ExpressionDescription::new();
                    luax_next(interpreter, lexical_state);
                    codename(interpreter, lexical_state, function_state, &mut key_0);
                    luak_self(interpreter, lexical_state, function_state, v, &mut key_0);
                    funcargs(interpreter, lexical_state, function_state, v);
                },
                40 | 292 | 123 => {
                    luak_exp2nextreg(interpreter, lexical_state, function_state, v);
                    funcargs(interpreter, lexical_state, function_state, v);
                },
                _ => return,
            }
        }
    }
}
pub unsafe fn simpleexp(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) {
    unsafe {
        match (*lexical_state).token.token {
            TK_FLT => {
                init_exp(v, ExpressionKind::ConstantNumber, 0);
                (*v).value.number = (*lexical_state).token.semantic_info.number;
            },
            TK_INT => {
                init_exp(v, ExpressionKind::ConstantInteger, 0);
                (*v).value.integer = (*lexical_state).token.semantic_info.integer;
            },
            TK_STRING => {
                codestring(v, (*lexical_state).token.semantic_info.tstring);
            },
            TK_NIL => {
                init_exp(v, ExpressionKind::Nil, 0);
            },
            TK_TRUE => {
                init_exp(v, ExpressionKind::True, 0);
            },
            TK_FALSE => {
                init_exp(v, ExpressionKind::False, 0);
            },
            TK_DOTS => {
                let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
                if !(*(*function_state).prototype).prototype_is_variable_arguments {
                    luax_syntaxerror(interpreter, lexical_state, make_cstring!("cannot use '...' outside a vararg function"));
                }
                init_exp(v, ExpressionKind::VariableArguments, code_abck(interpreter, lexical_state, function_state, OP_VARARG, 0, 0, 1, 0));
            },
            123 => {
                constructor(interpreter, lexical_state, function_state, v);
                return;
            },
            TK_FUNCTION => {
                luax_next(interpreter, lexical_state);
                body(interpreter, lexical_state, function_state, v, false, (*lexical_state).line_number);
                return;
            },
            _ => {
                suffixedexp(interpreter, lexical_state, function_state, v);
                return;
            },
        }
        luax_next(interpreter, lexical_state);
    }
}
pub unsafe fn error_expected(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, _function_state: *mut FunctionState, token: i32) -> ! {
    unsafe {
        luax_syntaxerror(
            interpreter,
            lexical_state,
            luao_pushfstring(interpreter, make_cstring!("%s expected"), luax_token2str(interpreter, lexical_state, token)),
        );
    }
}
pub unsafe fn testnext(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, _function_state: *mut FunctionState, c: i32) -> i32 {
    unsafe {
        if (*lexical_state).token.token == c {
            luax_next(interpreter, lexical_state);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe fn check(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, c: i32) {
    unsafe {
        if (*lexical_state).token.token != c {
            error_expected(interpreter, lexical_state, function_state, c);
        }
    }
}
pub unsafe fn checknext(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, c: i32) {
    unsafe {
        check(interpreter, lexical_state, function_state, c);
        luax_next(interpreter, lexical_state);
    }
}
pub unsafe fn check_match(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, what: i32, who: i32, where_0: i32) {
    unsafe {
        if testnext(interpreter, lexical_state, function_state, what) == 0 {
            if where_0 == (*lexical_state).line_number {
                error_expected(interpreter, lexical_state, function_state, what);
            } else {
                luax_syntaxerror(
                    interpreter,
                    lexical_state,
                    luao_pushfstring(
                        interpreter,
                        make_cstring!("%s expected (to close %s at line %d)"),
                        luax_token2str(interpreter, lexical_state, what),
                        luax_token2str(interpreter, lexical_state, who),
                        where_0,
                    ),
                );
            }
        }
    }
}
pub unsafe fn str_checkname(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> *mut TString {
    unsafe {
        check(interpreter, lexical_state, function_state, TK_NAME as i32);
        let ts: *mut TString = (*lexical_state).token.semantic_info.tstring;
        luax_next(interpreter, lexical_state);
        return ts;
    }
}
pub unsafe fn codename(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        codestring(expression_description, str_checkname(interpreter, lexical_state, function_state));
    }
}
pub unsafe fn registerlocalvar(interpreter: *mut Interpreter, _lexical_state: *mut LexicalState, function_state: *mut FunctionState, variable_name: *mut TString) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).prototype;
        let mut old_size = (*prototype).prototype_local_variables.get_size();
        (*prototype).prototype_local_variables.grow(
            interpreter,
            (*function_state).count_debug_variables as usize,
            if 32767 as usize <= (!(0usize)).wrapping_div(size_of::<LocalVariable>() as usize) {
                32767
            } else {
                (!(0usize)).wrapping_div(size_of::<LocalVariable>() as usize)
            },
            make_cstring!("local variables"),
        );
        while old_size < (*prototype).prototype_local_variables.get_size() {
            let fresh33 = old_size;
            old_size = old_size + 1;
            let ref mut fresh34 = (*((*prototype).prototype_local_variables.vectort_pointer).offset(fresh33 as isize)).variable_name;
            *fresh34 = null_mut();
        }
        let ref mut fresh35 = (*((*prototype).prototype_local_variables.vectort_pointer).offset((*function_state).count_debug_variables as isize)).variable_name;
        *fresh35 = variable_name;
        (*((*prototype).prototype_local_variables.vectort_pointer).offset((*function_state).count_debug_variables as isize)).start_program_counter = (*function_state).program_counter;
        if (*prototype).get_marked() & 1 << 5 != 0 && (*variable_name).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(prototype as *mut Object)), &mut (*(variable_name as *mut Object)));
        } else {
        };
        let fresh36 = (*function_state).count_debug_variables;
        (*function_state).count_debug_variables = (*function_state).count_debug_variables + 1;
        return fresh36 as i32;
    }
}
pub unsafe fn new_localvar(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, name: *mut TString) -> i32 {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
        let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
        let var: *mut VariableDescription;
        checklimit(
            interpreter,
            lexical_state,
            function_state,
            (*dynamic_data).active_variables.get_length() as i32 + 1 - (*function_state).first_local,
            200,
            make_cstring!("local variables"),
        );
        (*dynamic_data).active_variables.grow(
            interpreter,
            ((*dynamic_data).active_variables.get_length() + 1) as usize,
            if 32767 as usize <= (!(0usize)).wrapping_div(size_of::<VariableDescription>()) {
                32767
            } else {
                (!(0usize)).wrapping_div(size_of::<VariableDescription>() as usize)
            },
            make_cstring!("local variables"),
        );
        let fresh37 = (*dynamic_data).active_variables.get_length();
        (*dynamic_data).active_variables.set_length(((*dynamic_data).active_variables.get_length() + 1) as usize);
        var = &mut *((*dynamic_data).active_variables.vectort_pointer).offset(fresh37 as isize) as *mut VariableDescription;
        (*var).content.kind = 0;
        (*var).content.name = name;
        return (*dynamic_data).active_variables.get_length() as i32 - 1 - (*function_state).first_local;
    }
}
pub unsafe fn check_readonly(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, expression_description: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
        let mut variable_name: *mut TString = null_mut();
        match (*expression_description).expression_kind {
            ExpressionKind::Constant2 => {
                variable_name = (*((*(*lexical_state).dynamic_data).active_variables.vectort_pointer).offset((*expression_description).value.info as isize))
                    .content
                    .name;
            },
            ExpressionKind::Local => {
                let vardesc: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, (*expression_description).value.variable.value_index as i32);
                if (*vardesc).content.kind as i32 != 0 {
                    variable_name = (*vardesc).content.name;
                }
            },
            ExpressionKind::UpValue => {
                let up: *mut UpValueDescription = &mut *((*(*function_state).prototype).prototype_upvalues.vectort_pointer).offset((*expression_description).value.info as isize) as *mut UpValueDescription;
                if (*up).kind as i32 != 0 {
                    variable_name = (*up).name;
                }
            },
            _ => return,
        }
        if !variable_name.is_null() {
            let message: *const i8 = luao_pushfstring(interpreter, make_cstring!("attempt to assign to const variable '%s'"), (*variable_name).get_contents_mut());
            luak_semerror(interpreter, lexical_state, message);
        }
    }
}
pub unsafe fn adjustlocalvars(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, count_variables: i32) {
    unsafe {
        let function_state = (*lexical_state).lexical_state_function_state;
        let mut reglevel_0 = luay_nvarstack(lexical_state, function_state);
        for _ in 0..count_variables {
            let fresh39 = (*function_state).count_active_variables;
            (*function_state).count_active_variables = ((*function_state).count_active_variables).wrapping_add(1);
            let vidx = fresh39 as i32;
            let var = getlocalvardesc(lexical_state, function_state, vidx);
            let fresh40 = reglevel_0;
            reglevel_0 += 1;
            (*var).content.register_index = fresh40 as u8;
            (*var).content.pidx = registerlocalvar(interpreter, lexical_state, function_state, (*var).content.name) as i16;
        }
    }
}
pub unsafe fn singlevar(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, var: *mut ExpressionDescription) {
    unsafe {
        let variable_name: *mut TString = str_checkname(interpreter, lexical_state, function_state);
        singlevaraux(interpreter, lexical_state, function_state, variable_name, var, 1);
        if (*var).expression_kind == ExpressionKind::Void {
            let mut key: ExpressionDescription = ExpressionDescription::new();
            singlevaraux(interpreter, lexical_state, function_state, (*lexical_state).environment, var, 1);
            luak_exp2anyregup(interpreter, lexical_state, function_state, var);
            codestring(&mut key, variable_name);
            luak_indexed(interpreter, lexical_state, function_state, var, &mut key);
        }
    }
}
pub unsafe fn adjust_assign(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, count_variables: i32, count_expressions: i32, expression_description: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
        let needed: i32 = count_variables - count_expressions;
        if (*expression_description).expression_kind == ExpressionKind::Call || (*expression_description).expression_kind == ExpressionKind::VariableArguments {
            let mut extra: i32 = needed + 1;
            if extra < 0 {
                extra = 0;
            }
            luak_setreturns(interpreter, lexical_state, function_state, expression_description, extra);
        } else {
            if (*expression_description).expression_kind != ExpressionKind::Void {
                luak_exp2nextreg(interpreter, lexical_state, function_state, expression_description);
            }
            if needed > 0 {
                code_constant_nil(interpreter, lexical_state, function_state, (*function_state).freereg as i32, needed);
            }
        }
        if needed > 0 {
            luak_reserveregs(interpreter, lexical_state, function_state, needed);
        } else {
            (*function_state).freereg = ((*function_state).freereg as i32 + needed) as u8;
        };
    }
}
pub unsafe fn jumpscopeerror(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, goto_label_description: *mut LabelDescription) -> ! {
    unsafe {
        let variable_name: *const i8 = (*(*getlocalvardesc(lexical_state, (*lexical_state).lexical_state_function_state, (*goto_label_description).count_active_variables as i32))
            .content
            .name)
            .get_contents_mut();
        let mut message: *const i8 = make_cstring!("<goto %s> at line %d jumps into the scope of local '%s'");
        message = luao_pushfstring(interpreter, message, (*(*goto_label_description).name).get_contents_mut(), (*goto_label_description).line, variable_name);
        luak_semerror(interpreter, lexical_state, message);
    }
}
pub unsafe fn solvegoto(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, goto_offset: i32, label_description: *mut LabelDescription) {
    unsafe {
        let goto_label_list: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).dynamic_data).goto_;
        let goto_label_description = &mut *((*goto_label_list).vectort_pointer).offset(goto_offset as isize);
        if ((((*goto_label_description).count_active_variables as i32) < (*label_description).count_active_variables as i32) as i32 != 0) as i64 != 0 {
            jumpscopeerror(interpreter, lexical_state, goto_label_description);
        }
        luak_patchlist(
            interpreter,
            lexical_state,
            (*lexical_state).lexical_state_function_state,
            (*goto_label_description).program_counter,
            (*label_description).program_counter,
        );
        let mut i: i32 = goto_offset;
        while i < (*goto_label_list).get_length() as i32 - 1 {
            *((*goto_label_list).vectort_pointer).offset(i as isize) = *((*goto_label_list).vectort_pointer).offset((i + 1) as isize);
            i += 1;
        }
        (*goto_label_list).subtract_length(1);
    }
}
pub unsafe fn subexpr(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription, limit: i32) -> OperatorBinary {
    unsafe {
        (*(interpreter)).luae_inccstack();
        let uop = OperatorUnary::from_token((*lexical_state).token.token);
        if uop as u32 != OperatorUnary::None_ as u32 {
            let line: i32 = (*lexical_state).line_number;
            luax_next(interpreter, lexical_state);
            subexpr(interpreter, lexical_state, function_state, v, 12 as i32);
            luak_prefix(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, uop, v, line);
        } else {
            simpleexp(interpreter, lexical_state, function_state, v);
        }
        let mut op = OperatorBinary::from_token((*lexical_state).token.token);
        while op as u32 != OperatorBinary::NoBinaryOperation as u32 && PRIORITY[op as usize].left as i32 > limit {
            let mut v2: ExpressionDescription = ExpressionDescription::new();
            let line_0: i32 = (*lexical_state).line_number;
            luax_next(interpreter, lexical_state);
            luak_infix(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, op, v);
            let nextop = subexpr(interpreter, lexical_state, function_state, &mut v2, PRIORITY[op as usize].right as i32);
            luak_posfix(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, op, v, &mut v2, line_0);
            op = nextop;
        }
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls).wrapping_sub(1);
        (*interpreter).count_c_calls;
        return op;
    }
}
pub unsafe fn handle_block(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut block_control = BlockControl::new();
        enterblock(lexical_state, function_state, &mut block_control, false);
        parse_statement_list(interpreter, lexical_state, function_state);
        leaveblock(interpreter, lexical_state, function_state);
    }
}
pub unsafe fn check_conflict(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, mut lhs_assign: *mut ExpressionDescription, v: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexical_state_function_state;
        let extra: i32 = (*function_state).freereg as i32;
        let mut conflict: i32 = 0;
        while !lhs_assign.is_null() {
            if (*lhs_assign).expression_kind.is_index() {
                if (*lhs_assign).expression_kind == ExpressionKind::IndexUpValue {
                    if (*v).expression_kind == ExpressionKind::UpValue && (*lhs_assign).value.index.reference_tag as i32 == (*v).value.info {
                        conflict = 1;
                        (*lhs_assign).expression_kind = ExpressionKind::Field;
                        (*lhs_assign).value.index.reference_tag = extra as u8;
                    }
                } else {
                    if (*v).expression_kind == ExpressionKind::Local && (*lhs_assign).value.index.reference_tag as i32 == (*v).value.variable.register_index as i32 {
                        conflict = 1;
                        (*lhs_assign).value.index.reference_tag = extra as u8;
                    }
                    if (*lhs_assign).expression_kind == ExpressionKind::Indexed && (*v).expression_kind == ExpressionKind::Local && (*lhs_assign).value.index.reference_index as i32 == (*v).value.variable.register_index as i32 {
                        conflict = 1;
                        (*lhs_assign).value.index.reference_index = extra as i16;
                    }
                }
            }
            lhs_assign = (*lhs_assign).previous;
        }
        if conflict != 0 {
            if (*v).expression_kind == ExpressionKind::Local {
                code_abck(interpreter, lexical_state, function_state, OP_MOVE, extra, (*v).value.variable.register_index as i32, 0, 0);
            } else {
                code_abck(interpreter, lexical_state, function_state, OPCODE_GET_UPVALUE, extra, (*v).value.info, 0, 0);
            }
            luak_reserveregs(interpreter, lexical_state, function_state, 1);
        }
    }
}
pub unsafe fn restassign(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, lhs_assign: *mut ExpressionDescription, count_variables: i32) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        if !((*lhs_assign).expression_kind.is_index_plus()) {
            luax_syntaxerror(interpreter, lexical_state, make_cstring!("syntax error"));
        }
        check_readonly(interpreter, lexical_state, &mut (*lhs_assign));
        if testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0 {
            let mut new_lhs_assign: ExpressionDescription = ExpressionDescription::new_with_previous(lhs_assign);
            suffixedexp(interpreter, lexical_state, function_state, &mut new_lhs_assign);
            if !(new_lhs_assign.expression_kind.is_index()) {
                check_conflict(interpreter, lexical_state, lhs_assign, &mut new_lhs_assign);
            }
            (*(interpreter)).luae_inccstack();
            restassign(interpreter, lexical_state, function_state, &mut new_lhs_assign, count_variables + 1);
            (*interpreter).count_c_calls = ((*interpreter).count_c_calls).wrapping_sub(1);
            (*interpreter).count_c_calls;
        } else {
            checknext(interpreter, lexical_state, function_state, CHARACTER_EQUAL as i32);
            let count_expressions: i32 = (*lexical_state).parse_expression_list(interpreter, function_state, &mut expression_description);
            if count_expressions != count_variables {
                adjust_assign(interpreter, lexical_state, count_variables, count_expressions, &mut expression_description);
            } else {
                luak_setoneret((*lexical_state).lexical_state_function_state, &mut expression_description);
                luak_storevar(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut (*lhs_assign), &mut expression_description);
                return;
            }
        }
        init_exp(&mut expression_description, ExpressionKind::Nonrelocatable, (*(*lexical_state).lexical_state_function_state).freereg as i32 - 1);
        luak_storevar(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut (*lhs_assign), &mut expression_description);
    }
}
pub unsafe fn cond(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription::new();
        (*lexical_state).parse_expression(interpreter, function_state, &mut v);
        if v.expression_kind == ExpressionKind::Nil {
            v.expression_kind = ExpressionKind::False;
        }
        luak_goiftrue(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut v);
        return v.f;
    }
}
pub unsafe fn gotostat(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        let name: *mut TString = str_checkname(interpreter, lexical_state, function_state);
        let label_description = find_label((*lexical_state).lexical_state_function_state, (*lexical_state).dynamic_data, name);
        if label_description.is_null() {
            newgotoentry(interpreter, lexical_state, function_state, name, line, luak_jump(interpreter, lexical_state, function_state));
        } else {
            let level: i32 = reglevel(lexical_state, function_state, (*label_description).count_active_variables as i32);
            if luay_nvarstack(lexical_state, function_state) > level {
                code_abck(interpreter, lexical_state, function_state, OP_CLOSE, level, 0, 0, 0);
            }
            luak_patchlist(
                interpreter,
                lexical_state,
                function_state,
                luak_jump(interpreter, lexical_state, function_state),
                (*label_description).program_counter,
            );
        };
    }
}
pub unsafe fn breakstat(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        luax_next(interpreter, lexical_state);
        newgotoentry(
            interpreter,
            lexical_state,
            function_state,
            luas_newlstr(interpreter, make_cstring!("break"), (size_of::<[i8; 6]>()).wrapping_div(size_of::<i8>()).wrapping_sub(1)),
            line,
            luak_jump(interpreter, lexical_state, function_state),
        );
    }
}
pub unsafe fn checkrepeated(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, name: *mut TString) {
    unsafe {
        let lb = find_label((*lexical_state).lexical_state_function_state, (*lexical_state).dynamic_data, name);
        if !lb.is_null() {
            let mut message: *const i8 = make_cstring!("label '%s' already defined on line %d");
            message = luao_pushfstring(interpreter, message, (*name).get_contents_mut(), (*lb).line);
            luak_semerror(interpreter, lexical_state, message);
        }
    }
}
pub unsafe fn handle_label_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, name: *mut TString, line: i32) {
    unsafe {
        checknext(interpreter, lexical_state, function_state, TK_DBCOLON as i32);
        while (*lexical_state).token.token == CHARACTER_SEMICOLON as i32 || (*lexical_state).token.token == TK_DBCOLON as i32 {
            parse_statement(interpreter, lexical_state, function_state);
        }
        checkrepeated(interpreter, lexical_state, name);
        (*lexical_state).create_label(interpreter, function_state, name, line, block_follow_without_until((*lexical_state).token.token));
    }
}
pub unsafe fn handle_while_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let mut block_control = BlockControl::new();
        luax_next(interpreter, lexical_state);
        let whileinit: i32 = (*function_state).code_get_label();
        let condexit: i32 = cond(interpreter, lexical_state, function_state);
        enterblock(lexical_state, function_state, &mut block_control, true);
        checknext(interpreter, lexical_state, function_state, TK_DO as i32);
        handle_block(interpreter, lexical_state, function_state);
        luak_patchlist(interpreter, lexical_state, function_state, luak_jump(interpreter, lexical_state, function_state), whileinit);
        check_match(interpreter, lexical_state, function_state, TK_END as i32, TK_WHILE as i32, line);
        leaveblock(interpreter, lexical_state, function_state);
        luak_patchtohere(interpreter, lexical_state, function_state, condexit);
    }
}
pub unsafe fn handle_repeat_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let repeat_init: i32 = (*function_state).code_get_label();
        let mut bl1 = BlockControl::new();
        let mut bl2 = BlockControl::new();
        enterblock(lexical_state, function_state, &mut bl1, true);
        enterblock(lexical_state, function_state, &mut bl2, false);
        luax_next(interpreter, lexical_state);
        parse_statement_list(interpreter, lexical_state, function_state);
        check_match(interpreter, lexical_state, function_state, TK_UNTIL as i32, TK_REPEAT as i32, line);
        let mut condexit: i32 = cond(interpreter, lexical_state, function_state);
        leaveblock(interpreter, lexical_state, function_state);
        if bl2.count_upvalues != 0 {
            let exit_0: i32 = luak_jump(interpreter, lexical_state, function_state);
            luak_patchtohere(interpreter, lexical_state, function_state, condexit);
            code_abck(
                interpreter,
                lexical_state,
                function_state,
                OP_CLOSE,
                reglevel(lexical_state, function_state, bl2.count_active_variables as i32),
                0,
                0,
                0,
            );
            condexit = luak_jump(interpreter, lexical_state, function_state);
            luak_patchtohere(interpreter, lexical_state, function_state, exit_0);
        }
        luak_patchlist(interpreter, lexical_state, function_state, condexit, repeat_init);
        leaveblock(interpreter, lexical_state, function_state);
    }
}
pub unsafe fn exp1(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description = ExpressionDescription::new();
        (*lexical_state).parse_expression(interpreter, function_state, &mut expression_description);
        luak_exp2nextreg(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut expression_description);
    }
}
pub unsafe fn handle_forbody(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, base: i32, line: i32, count_variables: i32, isgen: i32) {
    unsafe {
        static mut FOR_PREP: [u32; 2] = [OP_FORPREP, OP_TFORPREP];
        static mut FOR_LOOP: [u32; 2] = [OP_FORLOOP, OP_TFORLOOP];
        let mut block_control = BlockControl::new();
        checknext(interpreter, lexical_state, function_state, TK_DO as i32);
        let prep: i32 = luak_codeabx(interpreter, lexical_state, function_state, FOR_PREP[isgen as usize], base, 0u32);
        enterblock(lexical_state, function_state, &mut block_control, false);
        adjustlocalvars(interpreter, lexical_state, count_variables);
        luak_reserveregs(interpreter, lexical_state, function_state, count_variables);
        handle_block(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
        leaveblock(interpreter, lexical_state, function_state);
        fixforjump(interpreter, lexical_state, function_state, prep, (*function_state).code_get_label(), 0);
        if isgen != 0 {
            code_abck(interpreter, lexical_state, function_state, OP_TFORCALL, base, 0, count_variables, 0);
            luak_fixline(interpreter, lexical_state, function_state, line);
        }
        let endfor: i32 = luak_codeabx(interpreter, lexical_state, function_state, FOR_LOOP[isgen as usize], base, 0);
        fixforjump(interpreter, lexical_state, function_state, endfor, prep + 1, 1);
        luak_fixline(interpreter, lexical_state, function_state, line);
    }
}
pub unsafe fn handle_for_numeric(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, variable_name: *mut TString, line: i32) {
    unsafe {
        let base: i32 = (*function_state).freereg as i32;
        let s = make_cstring!("(for interpreter)");
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, variable_name);
        checknext(interpreter, lexical_state, function_state, CHARACTER_EQUAL as i32);
        exp1(interpreter, lexical_state, function_state);
        checknext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32);
        exp1(interpreter, lexical_state, function_state);
        if testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0 {
            exp1(interpreter, lexical_state, function_state);
        } else {
            code_constant_integer(interpreter, lexical_state, function_state, (*function_state).freereg as i32, 1 as i64);
            luak_reserveregs(interpreter, lexical_state, function_state, 1);
        }
        adjustlocalvars(interpreter, lexical_state, 3);
        handle_forbody(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, base, line, 1, 0);
    }
}
pub unsafe fn handle_for_list(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, indexname: *mut TString) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let mut count_variables: i32 = 5;
        let base: i32 = (*function_state).freereg as i32;
        let s = make_cstring!("(for interpreter)");
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, luax_newstring(interpreter, lexical_state, s, strlen(s) as usize));
        new_localvar(interpreter, lexical_state, indexname);
        while testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0 {
            new_localvar(interpreter, lexical_state, str_checkname(interpreter, lexical_state, function_state));
            count_variables += 1;
        }
        checknext(interpreter, lexical_state, function_state, TK_IN as i32);
        let line: i32 = (*lexical_state).line_number;
        adjust_assign(
            interpreter,
            lexical_state,
            4,
            (*lexical_state).parse_expression_list(interpreter, function_state, &mut expression_description),
            &mut expression_description,
        );
        adjustlocalvars(interpreter, lexical_state, 4);
        marktobeclosed(function_state);
        luak_checkstack(interpreter, lexical_state, function_state, 3);
        handle_forbody(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, base, line, count_variables - 4, 1);
    }
}
pub unsafe fn handle_for_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let mut block_control = BlockControl::new();
        enterblock(lexical_state, function_state, &mut block_control, true);
        luax_next(interpreter, lexical_state);
        let variable_name: *mut TString = str_checkname(interpreter, lexical_state, function_state);
        match (*lexical_state).token.token {
            CHARACTER_EQUAL => {
                handle_for_numeric(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, variable_name, line);
            },
            CHARACTER_COMMA | TK_IN => {
                handle_for_list(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, variable_name);
            },
            _ => {
                luax_syntaxerror(interpreter, lexical_state, make_cstring!("CHARACTER_EQUAL or 'in' expected"));
            },
        }
        check_match(interpreter, lexical_state, function_state, TK_END as i32, TK_FOR as i32, line);
        leaveblock(interpreter, lexical_state, function_state);
    }
}
pub unsafe fn handle_test_then_block(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, escapelist: *mut i32) {
    unsafe {
        let mut block_control = BlockControl::new();
        let mut v: ExpressionDescription = ExpressionDescription::new();
        let jf;
        luax_next(interpreter, lexical_state);
        (*lexical_state).parse_expression(interpreter, function_state, &mut v);
        checknext(interpreter, lexical_state, function_state, TK_THEN as i32);
        if (*lexical_state).token.token == TK_BREAK as i32 {
            let line: i32 = (*lexical_state).line_number;
            luak_goiffalse(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut v);
            luax_next(interpreter, lexical_state);
            enterblock(lexical_state, function_state, &mut block_control, false);
            newgotoentry(
                interpreter,
                lexical_state,
                function_state,
                luas_newlstr(interpreter, make_cstring!("break"), (size_of::<[i8; 6]>()).wrapping_div(size_of::<i8>()).wrapping_sub(1)),
                line,
                v.t,
            );
            while testnext(interpreter, lexical_state, function_state, CHARACTER_SEMICOLON as i32) != 0 {}
            if block_follow_without_until((*lexical_state).token.token) {
                leaveblock(interpreter, lexical_state, function_state);
                return;
            } else {
                jf = luak_jump(interpreter, lexical_state, function_state);
            }
        } else {
            luak_goiftrue(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut v);
            enterblock(lexical_state, function_state, &mut block_control, false);
            jf = v.f;
        }
        parse_statement_list(interpreter, lexical_state, function_state);
        leaveblock(interpreter, lexical_state, function_state);
        if (*lexical_state).token.token == TK_ELSE as i32 || (*lexical_state).token.token == TK_ELSEIF as i32 {
            luak_concat(interpreter, lexical_state, function_state, escapelist, luak_jump(interpreter, lexical_state, function_state));
        }
        luak_patchtohere(interpreter, lexical_state, function_state, jf);
    }
}
pub unsafe fn handle_if_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let mut escape_list: i32 = -1;
        handle_test_then_block(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut escape_list);
        while (*lexical_state).token.token == TK_ELSEIF as i32 {
            handle_test_then_block(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, &mut escape_list);
        }
        if testnext(interpreter, lexical_state, function_state, TK_ELSE as i32) != 0 {
            handle_block(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
        }
        check_match(interpreter, lexical_state, function_state, TK_END as i32, TK_IF as i32, line);
        luak_patchtohere(interpreter, lexical_state, function_state, escape_list);
    }
}
pub unsafe fn handle_local_function(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let fvar: i32 = (*function_state).count_active_variables as i32;
        new_localvar(interpreter, lexical_state, str_checkname(interpreter, lexical_state, function_state));
        adjustlocalvars(interpreter, lexical_state, 1);
        body(interpreter, lexical_state, function_state, &mut expression_description, false, (*lexical_state).line_number);
        (*localdebuginfo(lexical_state, function_state, fvar)).start_program_counter = (*function_state).program_counter;
    }
}
pub unsafe fn getlocalattribute(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        if testnext(interpreter, lexical_state, function_state, CHARACTER_ANGLE_LEFT) != 0 {
            let attr: *const i8 = (*str_checkname(interpreter, lexical_state, function_state)).get_contents_mut();
            checknext(interpreter, lexical_state, function_state, CHARACTER_ANGLE_RIGHT);
            if strcmp(attr, make_cstring!("const")) == 0 {
                return 1;
            } else if strcmp(attr, make_cstring!("close")) == 0 {
                return 2;
            } else {
                luak_semerror(interpreter, lexical_state, luao_pushfstring(interpreter, make_cstring!("unknown attribute '%s'"), attr));
            }
        }
        return 0;
    }
}
pub unsafe fn handle_local_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut toclose: i32 = -1;
        let var: *mut VariableDescription;
        let mut vidx: i32;
        let mut kind: i32;
        let mut count_variables: i32 = 0;
        let count_expressions: i32;
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        loop {
            vidx = new_localvar(interpreter, lexical_state, str_checkname(interpreter, lexical_state, function_state));
            kind = getlocalattribute(interpreter, lexical_state, function_state);
            (*getlocalvardesc(lexical_state, function_state, vidx)).content.kind = kind as u8;
            if kind == 2 {
                if toclose != -1 {
                    luak_semerror(interpreter, lexical_state, make_cstring!("multiple to-be-closed variables in local list"));
                }
                toclose = (*function_state).count_active_variables as i32 + count_variables;
            }
            count_variables += 1;
            if !(testnext(interpreter, lexical_state, function_state, CHARACTER_COMMA as i32) != 0) {
                break;
            }
        }
        if testnext(interpreter, lexical_state, function_state, CHARACTER_EQUAL as i32) != 0 {
            count_expressions = (*lexical_state).parse_expression_list(interpreter, function_state, &mut expression_description);
        } else {
            expression_description.expression_kind = ExpressionKind::Void;
            count_expressions = 0;
        }
        var = getlocalvardesc(lexical_state, function_state, vidx);
        if count_variables == count_expressions && (*var).content.kind as i32 == 1 && luak_exp2const(lexical_state, function_state, &mut expression_description, &mut (*var).k) {
            (*var).content.kind = 3 as u8;
            adjustlocalvars(interpreter, lexical_state, count_variables - 1);
            (*function_state).count_active_variables = ((*function_state).count_active_variables).wrapping_add(1);
            (*function_state).count_active_variables;
        } else {
            adjust_assign(interpreter, lexical_state, count_variables, count_expressions, &mut expression_description);
            adjustlocalvars(interpreter, lexical_state, count_variables);
        }
        checktoclose(interpreter, lexical_state, function_state, toclose);
    }
}
pub unsafe fn handle_function_name(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription) -> bool {
    unsafe {
        let mut is_method = false;
        singlevar(interpreter, lexical_state, function_state, v);
        while (*lexical_state).token.token == CHARACTER_PERIOD as i32 {
            fieldsel(interpreter, lexical_state, function_state, v);
        }
        if (*lexical_state).token.token == CHARACTER_COLON as i32 {
            is_method = true;
            fieldsel(interpreter, lexical_state, function_state, v);
        }
        return is_method;
    }
}
pub unsafe fn handle_function_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription::new();
        let mut b: ExpressionDescription = ExpressionDescription::new();
        luax_next(interpreter, lexical_state);
        let is_method = handle_function_name(interpreter, lexical_state, function_state, &mut v);
        body(interpreter, lexical_state, function_state, &mut b, is_method, line);
        check_readonly(interpreter, lexical_state, &mut v);
        luak_storevar(interpreter, lexical_state, function_state, &mut v, &mut b);
        luak_fixline(interpreter, lexical_state, function_state, line);
    }
}
pub unsafe fn handle_expression_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut new_lhs_assign: ExpressionDescription = ExpressionDescription::new();
        suffixedexp(interpreter, lexical_state, function_state, &mut new_lhs_assign);
        if (*lexical_state).token.token == CHARACTER_EQUAL as i32 || (*lexical_state).token.token == CHARACTER_COMMA as i32 {
            new_lhs_assign.previous = null_mut();
            restassign(interpreter, lexical_state, function_state, &mut new_lhs_assign, 1);
        } else {
            if !(new_lhs_assign.expression_kind == ExpressionKind::Call) {
                luax_syntaxerror(interpreter, lexical_state, make_cstring!("syntax error"));
            }
            let inst: *mut u32 = &mut *((*(*function_state).prototype).prototype_code.vectort_pointer).offset(new_lhs_assign.value.info as isize) as *mut u32;
            *inst = *inst & !(!(!(0u32) << 8) << POSITION_C) | (1 as u32) << POSITION_C & !(!(0u32) << 8) << POSITION_C;
        };
    }
}
pub unsafe fn handle_return_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let mut nret: i32;
        let mut first: i32 = luay_nvarstack(lexical_state, function_state);
        if block_follow_with_until((*lexical_state).token.token) || (*lexical_state).token.token == CHARACTER_SEMICOLON as i32 {
            nret = 0;
        } else {
            nret = (*lexical_state).parse_expression_list(interpreter, function_state, &mut expression_description);
            if expression_description.expression_kind == ExpressionKind::Call || expression_description.expression_kind == ExpressionKind::VariableArguments {
                luak_setreturns(interpreter, lexical_state, function_state, &mut expression_description, -1);
                if expression_description.expression_kind == ExpressionKind::Call && nret == 1 && !(*(*function_state).block_control).is_inside_tbc {
                    *((*(*function_state).prototype).prototype_code.vectort_pointer).offset(expression_description.value.info as isize) =
                        *((*(*function_state).prototype).prototype_code.vectort_pointer).offset(expression_description.value.info as isize) & !(!(!(0u32) << 7) << 0) | (OP_TAILCALL as u32) << 0 & !(!(0u32) << 7) << 0;
                }
                nret = -1;
            } else if nret == 1 {
                first = luak_exp2anyreg(interpreter, lexical_state, function_state, &mut expression_description);
            } else {
                luak_exp2nextreg(interpreter, lexical_state, function_state, &mut expression_description);
            }
        }
        luak_ret(interpreter, lexical_state, function_state, first, nret);
        testnext(interpreter, lexical_state, function_state, CHARACTER_SEMICOLON as i32);
    }
}
pub unsafe fn handle_main_function(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut block_control = BlockControl::new();
        let env: *mut UpValueDescription;
        open_function(interpreter, lexical_state, function_state, (*lexical_state).lexical_state_function_state, &mut block_control);
        setvararg(interpreter, lexical_state, function_state, 0);
        env = allocate_upvalue_description(interpreter, lexical_state, function_state, (*function_state).prototype);
        (*env).is_in_stack = true;
        (*env).index = 0;
        (*env).kind = 0;
        (*env).name = (*lexical_state).environment;
        if (*(*function_state).prototype).get_marked() & 1 << 5 != 0 && (*(*env).name).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*((*function_state).prototype as *mut Object)), &mut (*((*env).name as *mut Object)));
        } else {
        };
        luax_next(interpreter, lexical_state);
        parse_statement_list(interpreter, lexical_state, function_state);
        check(interpreter, lexical_state, function_state, TK_EOS as i32);
        close_function(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
    }
}
pub unsafe fn save(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, c: i32) {
    unsafe {
        let b = (*lexical_state).buffer;
        if ((*b).loads.get_length() as usize).wrapping_add(1 as usize) > (*b).loads.get_size() as usize {
            if (*b).loads.get_size() as usize >= (if (size_of::<usize>()) < size_of::<i64>() { !(0usize) } else { MAXIMUM_SIZE }).wrapping_div(2) {
                lexerror(interpreter, lexical_state, make_cstring!("lexical element too long"), 0);
            }
            let new_size = (*b).loads.get_size().wrapping_mul(2);
            (*b).loads.resize(interpreter, new_size as usize);
        }
        let fresh49 = (*b).loads.get_length();
        (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
        *((*b).loads.loads_pointer).offset(fresh49 as isize) = c as i8;
    }
}
pub unsafe fn luax_token2str(interpreter: *mut Interpreter, _lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        if token < 127 as i32 * 2 + 1 + 1 {
            if is_printable(token + 1) {
                return luao_pushfstring(interpreter, make_cstring!("'%c'"), token);
            } else {
                return luao_pushfstring(interpreter, make_cstring!("'<\\%d>'"), token);
            }
        } else {
            let s: *const i8 = TOKENS[(token - (127 as i32 * 2 + 1 + 1)) as usize];
            if token < TK_EOS as i32 {
                return luao_pushfstring(interpreter, make_cstring!("'%s'"), s);
            } else {
                return s;
            }
        };
    }
}
pub unsafe fn text_token(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        match token {
            TK_NAME | TK_STRING | TK_FLT | TK_INT => {
                save(interpreter, lexical_state, Character::Null as i32);
                return luao_pushfstring(interpreter, make_cstring!("'%s'"), (*(*lexical_state).buffer).loads.loads_pointer);
            },
            _ => return luax_token2str(interpreter, lexical_state, token),
        };
    }
}
pub unsafe fn lexerror(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, mut message: *const i8, token: i32) -> ! {
    unsafe {
        message = luag_addinfo(interpreter, message, (*lexical_state).source, (*lexical_state).line_number);
        if token != 0 {
            luao_pushfstring(interpreter, make_cstring!("%s near %s"), message, text_token(interpreter, lexical_state, token));
        }
        luad_throw(interpreter, 3);
    }
}
pub unsafe fn luax_syntaxerror(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, message: *const i8) -> ! {
    unsafe {
        lexerror(interpreter, lexical_state, message, (*lexical_state).token.token);
    }
}
pub unsafe fn luax_newstring(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, str: *const i8, length: usize) -> *mut TString {
    unsafe {
        let mut ts: *mut TString = luas_newlstr(interpreter, str, length as usize);
        let o: *const TValue = luah_getstr((*lexical_state).table, ts);
        if (*o).is_tagtype_nil() {
            let fresh50 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let stv: *mut TValue = &mut (*fresh50);
            let io: *mut TValue = stv;
            let ts: *mut TString = ts;
            (*io).value.object = &mut (*(ts as *mut Object));
            (*io).set_tag_variant((*ts).get_tag_variant());
            (*io).set_collectable(true);
            luah_finishset(interpreter, (*lexical_state).table, stv, o, stv);
            if (*(*interpreter).global).gc_debt > 0 {
                (*interpreter).luac_step();
            }
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        } else {
            ts = &mut (*((*(o as *mut Node)).key.value.object as *mut TString));
        }
        return ts;
    }
}
pub unsafe fn inclinenumber(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) {
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
        if ((*lexical_state).current == CHARACTER_LF as i32 || (*lexical_state).current == CHARACTER_CR as i32) && (*lexical_state).current != old {
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
            lexerror(interpreter, lexical_state, make_cstring!("chunk has too many lines"), 0);
        }
    }
}
pub unsafe fn luax_setinput(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, zio: *mut ZIO, source: *mut TString, firstchar: i32) {
    unsafe {
        (*lexical_state).token.token = 0;
        (*lexical_state).current = firstchar;
        (*lexical_state).look_ahead.token = TK_EOS as i32;
        (*lexical_state).zio = zio;
        (*lexical_state).lexical_state_function_state = null_mut();
        (*lexical_state).line_number = 1;
        (*lexical_state).last_line = 1;
        (*lexical_state).source = source;
        (*lexical_state).environment = luas_newlstr(interpreter, make_cstring!("_ENV"), (size_of::<[i8; 5]>()).wrapping_div(size_of::<i8>()).wrapping_sub(1));
        (*(*lexical_state).buffer).loads.resize(interpreter, 32 as usize);
    }
}
pub unsafe fn check_next1(lexical_state: *mut LexicalState, c: i32) -> i32 {
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
pub unsafe fn check_next2(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, set: *const i8) -> i32 {
    unsafe {
        if (*lexical_state).current == *set.offset(0 as isize) as i32 || (*lexical_state).current == *set.offset(1 as isize) as i32 {
            save(interpreter, lexical_state, (*lexical_state).current);
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
pub unsafe fn read_numeral(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, semantic_info: *mut Value) -> i32 {
    unsafe {
        let mut obj: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let mut expo: *const i8 = make_cstring!("Ee");
        let first: i32 = (*lexical_state).current;
        save(interpreter, lexical_state, (*lexical_state).current);
        let fresh59 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh59 > 0 {
            let fresh60 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh60 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        if first == CHARACTER_0 as i32 && check_next2(interpreter, lexical_state, make_cstring!("xX")) != 0 {
            expo = make_cstring!("Pp");
        }
        loop {
            if check_next2(interpreter, lexical_state, expo) != 0 {
                check_next2(interpreter, lexical_state, make_cstring!("-+"));
            } else {
                if !(is_digit_hexadecimal((*lexical_state).current + 1) || (*lexical_state).current == CHARACTER_PERIOD as i32) {
                    break;
                }
                save(interpreter, lexical_state, (*lexical_state).current);
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
            save(interpreter, lexical_state, (*lexical_state).current);
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
        save(interpreter, lexical_state, Character::Null as i32);
        if luao_str2num((*(*lexical_state).buffer).loads.loads_pointer, &mut obj) == 0 {
            lexerror(interpreter, lexical_state, make_cstring!("malformed number"), TK_FLT as i32);
        }
        if obj.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            (*semantic_info).integer = obj.value.integer;
            return TK_INT as i32;
        } else {
            (*semantic_info).number = obj.value.number;
            return TK_FLT as i32;
        };
    }
}
pub unsafe fn skip_sep(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> usize {
    unsafe {
        let mut count: usize = 0;
        let s: i32 = (*lexical_state).current;
        save(interpreter, lexical_state, (*lexical_state).current);
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
            save(interpreter, lexical_state, (*lexical_state).current);
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
        return if (*lexical_state).current == s { count.wrapping_add(2 as usize) } else { (if count == 0 { 1 } else { 0 }) as usize };
    }
}
pub unsafe fn read_long_string(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, semantic_info: *mut Value, sep: usize) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        save(interpreter, lexical_state, (*lexical_state).current);
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
            inclinenumber(interpreter, lexical_state);
        }
        loop {
            match (*lexical_state).current {
                -1 => {
                    let what: *const i8 = if !semantic_info.is_null() { make_cstring!("string") } else { make_cstring!("comment") };
                    let message: *const i8 = luao_pushfstring(interpreter, make_cstring!("unfinished long %s (starting at line %d)"), what, line);
                    lexerror(interpreter, lexical_state, message, TK_EOS as i32);
                },
                CHARACTER_BRACKET_RIGHT => {
                    if !(skip_sep(interpreter, lexical_state) == sep) {
                        continue;
                    }
                    save(interpreter, lexical_state, (*lexical_state).current);
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
                },
                CHARACTER_LF | CHARACTER_CR => {
                    save(interpreter, lexical_state, CHARACTER_LF as i32);
                    inclinenumber(interpreter, lexical_state);
                    if semantic_info.is_null() {
                        (*(*lexical_state).buffer).loads.zero_length();
                    }
                },
                _ => {
                    if !semantic_info.is_null() {
                        save(interpreter, lexical_state, (*lexical_state).current);
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
                },
            }
        }
        if !semantic_info.is_null() {
            (*semantic_info).tstring = luax_newstring(
                interpreter,
                lexical_state,
                ((*(*lexical_state).buffer).loads.loads_pointer).offset(sep as isize),
                ((*(*lexical_state).buffer).loads.get_length() as usize).wrapping_sub((2 as usize).wrapping_mul(sep as usize)) as usize,
            );
        }
    }
}
pub unsafe fn esccheck(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, condition: bool, message: *const i8) {
    unsafe {
        if !condition {
            if (*lexical_state).current != -1 {
                save(interpreter, lexical_state, (*lexical_state).current);
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
            lexerror(interpreter, lexical_state, message, TK_STRING as i32);
        }
    }
}
pub unsafe fn gethexa(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        save(interpreter, lexical_state, (*lexical_state).current);
        let fresh79 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh79 > 0 {
            let fresh80 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh80 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        esccheck(interpreter, lexical_state, is_digit_hexadecimal((*lexical_state).current + 1), make_cstring!("hexadecimal digit expected"));
        return get_hexadecimal_digit_value((*lexical_state).current);
    }
}
pub unsafe fn readhexaesc(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut r: i32 = gethexa(interpreter, lexical_state);
        r = (r << 4) + gethexa(interpreter, lexical_state);
        (*(*lexical_state).buffer).loads.set_length(((*(*lexical_state).buffer).loads.get_length()).wrapping_sub(2) as usize);
        return r;
    }
}
pub unsafe fn readutf8esc(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> usize {
    unsafe {
        let mut i: i32 = 4;
        save(interpreter, lexical_state, (*lexical_state).current);
        let fresh81 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh81 > 0 {
            let fresh82 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh82 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        esccheck(interpreter, lexical_state, (*lexical_state).current == CHARACTER_BRACE_LEFT, make_cstring!("missing CHARACTER_BRACE_LEFT"));
        let mut r: usize = gethexa(interpreter, lexical_state) as usize;
        loop {
            save(interpreter, lexical_state, (*lexical_state).current);
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
            esccheck(interpreter, lexical_state, r <= (0x7fffffff as usize >> 4), make_cstring!("UTF-8 value too large"));
            r = (r << 4).wrapping_add(get_hexadecimal_digit_value((*lexical_state).current) as usize);
        }
        esccheck(interpreter, lexical_state, (*lexical_state).current == CHARACTER_BRACE_RIGHT, make_cstring!("missing CHARACTER_BRACE_RIGHT"));
        let fresh85 = (*(*lexical_state).zio).length;
        (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
        (*lexical_state).current = if fresh85 > 0 {
            let fresh86 = (*(*lexical_state).zio).pointer;
            (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
            *fresh86 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        (*(*lexical_state).buffer).loads.set_length(((*(*lexical_state).buffer).loads.get_length() as usize).wrapping_sub(i as usize));
        return r;
    }
}
pub unsafe fn utf8esc(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) {
    unsafe {
        let mut buffer: [i8; 8] = [0; 8];
        let mut n: i32 = luao_utf8esc(buffer.as_mut_ptr(), readutf8esc(interpreter, lexical_state));
        while n > 0 {
            save(interpreter, lexical_state, buffer[(8 - n) as usize] as i32);
            n -= 1;
        }
    }
}
pub unsafe fn readdecesc(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut i: i32;
        let mut r: i32 = 0;
        i = 0;
        while i < 3 && is_digit_decimal((*lexical_state).current + 1) {
            r = 10 as i32 * r + (*lexical_state).current - CHARACTER_0 as i32;
            save(interpreter, lexical_state, (*lexical_state).current);
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
        esccheck(interpreter, lexical_state, r <= 127 * 2 + 1, make_cstring!("decimal escape too large"));
        (*(*lexical_state).buffer).loads.set_length(((*(*lexical_state).buffer).loads.get_length() as usize).wrapping_sub(i as usize));
        return r;
    }
}
pub unsafe fn read_string(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, del: i32, semantic_info: *mut Value) {
    unsafe {
        let mut current_block: usize;
        save(interpreter, lexical_state, (*lexical_state).current);
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
                    lexerror(interpreter, lexical_state, make_cstring!("unfinished string"), TK_EOS as i32);
                },
                CHARACTER_LF | CHARACTER_CR => {
                    lexerror(interpreter, lexical_state, make_cstring!("unfinished string"), TK_STRING as i32);
                },
                CHARACTER_BACKSLASH => {
                    let c: i32;
                    save(interpreter, lexical_state, (*lexical_state).current);
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
                        },
                        CHARACTER_LOWER_B => {
                            c = CHARACTER_BS as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_F => {
                            c = CHARACTER_FF as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_N => {
                            c = CHARACTER_LF as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_R => {
                            c = CHARACTER_CR as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_T => {
                            c = CHARACTER_HT as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_V => {
                            c = CHARACTER_VT as i32;
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_X => {
                            c = readhexaesc(interpreter, lexical_state);
                            current_block = 15029063370732930705;
                        },
                        CHARACTER_LOWER_U => {
                            utf8esc(interpreter, lexical_state);
                            continue;
                        },
                        CHARACTER_CR | CHARACTER_LF => {
                            inclinenumber(interpreter, lexical_state);
                            c = CHARACTER_LF as i32;
                            current_block = 7010296663004816197;
                        },
                        CHARACTER_BACKSLASH | CHARACTER_DOUBLEQUOTE | CHARACTER_QUOTE => {
                            c = (*lexical_state).current;
                            current_block = 15029063370732930705;
                        },
                        -1 => {
                            continue;
                        },
                        CHARACTER_LOWER_Z => {
                            (*(*lexical_state).buffer).loads.set_length(((*(*lexical_state).buffer).loads.get_length()).wrapping_sub(1) as usize);
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
                                if (*lexical_state).current == CHARACTER_LF as i32 || (*lexical_state).current == CHARACTER_CR as i32 {
                                    inclinenumber(interpreter, lexical_state);
                                } else {
                                    let fresh95 = (*(*lexical_state).zio).length;
                                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                                    (*lexical_state).current = if fresh95 > 0 {
                                        let fresh96 = (*(*lexical_state).zio).pointer;
                                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                        *fresh96 as u8 as i32
                                    } else {
                                        luaz_fill((*lexical_state).zio)
                                    };
                                }
                            }
                            continue;
                        },
                        _ => {
                            esccheck(interpreter, lexical_state, is_digit_decimal((*lexical_state).current + 1), make_cstring!("invalid escape sequence"));
                            c = readdecesc(interpreter, lexical_state);
                            current_block = 7010296663004816197;
                        },
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
                        },
                        _ => {},
                    }
                    (*(*lexical_state).buffer).loads.set_length(((*(*lexical_state).buffer).loads.get_length()).wrapping_sub(1) as usize);
                    save(interpreter, lexical_state, c);
                },
                _ => {
                    save(interpreter, lexical_state, (*lexical_state).current);
                    let fresh99 = (*(*lexical_state).zio).length;
                    (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                    (*lexical_state).current = if fresh99 > 0 {
                        let fresh100 = (*(*lexical_state).zio).pointer;
                        (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                        *fresh100 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                },
            }
        }
        save(interpreter, lexical_state, (*lexical_state).current);
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
            interpreter,
            lexical_state,
            ((*(*lexical_state).buffer).loads.loads_pointer).offset(1 as isize),
            ((*(*lexical_state).buffer).loads.get_length()).wrapping_sub(2) as usize,
        );
    }
}
pub unsafe fn llex(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, semantic_info: *mut Value) -> i32 {
    unsafe {
        (*(*lexical_state).buffer).loads.zero_length();
        loop {
            let current_block_85: usize;
            match (*lexical_state).current {
                CHARACTER_LF | CHARACTER_CR => {
                    inclinenumber(interpreter, lexical_state);
                },
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
                },
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
                        let sep: usize = skip_sep(interpreter, lexical_state);
                        (*(*lexical_state).buffer).loads.zero_length();
                        if sep >= 2 as usize {
                            read_long_string(interpreter, lexical_state, null_mut(), sep);
                            (*(*lexical_state).buffer).loads.zero_length();
                            current_block_85 = 10512632378975961025;
                        } else {
                            current_block_85 = 3512920355445576850;
                        }
                    } else {
                        current_block_85 = 3512920355445576850;
                    }
                    match current_block_85 {
                        10512632378975961025 => {},
                        _ => {
                            while !((*lexical_state).current == CHARACTER_LF as i32 || (*lexical_state).current == CHARACTER_CR as i32) && (*lexical_state).current != -1 {
                                let fresh109 = (*(*lexical_state).zio).length;
                                (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                                (*lexical_state).current = if fresh109 > 0 {
                                    let fresh110 = (*(*lexical_state).zio).pointer;
                                    (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                    *fresh110 as u8 as i32
                                } else {
                                    luaz_fill((*lexical_state).zio)
                                };
                            }
                        },
                    }
                },
                CHARACTER_BRACKET_LEFT => {
                    let sep_0: usize = skip_sep(interpreter, lexical_state);
                    if sep_0 >= 2 as usize {
                        read_long_string(interpreter, lexical_state, semantic_info, sep_0);
                        return TK_STRING as i32;
                    } else if sep_0 == 0 {
                        lexerror(interpreter, lexical_state, make_cstring!("invalid long string delimiter"), TK_STRING as i32);
                    }
                    return CHARACTER_BRACKET_LEFT as i32;
                },
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
                },
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
                },
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
                },
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
                },
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
                },
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
                },
                CHARACTER_QUOTE | CHARACTER_DOUBLEQUOTE => {
                    read_string(interpreter, lexical_state, (*lexical_state).current, semantic_info);
                    return TK_STRING as i32;
                },
                CHARACTER_PERIOD => {
                    save(interpreter, lexical_state, (*lexical_state).current);
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
                        return read_numeral(interpreter, lexical_state, semantic_info);
                    }
                },
                CHARACTER_0 | CHARACTER_1 | CHARACTER_2 | CHARACTER_3 | CHARACTER_4 | CHARACTER_5 | CHARACTER_6 | CHARACTER_7 | CHARACTER_8 | CHARACTER_9 => {
                    return read_numeral(interpreter, lexical_state, semantic_info);
                },
                -1 => return TK_EOS as i32,
                _ => {
                    if is_identifier((*lexical_state).current + 1) {
                        loop {
                            save(interpreter, lexical_state, (*lexical_state).current);
                            let fresh125 = (*(*lexical_state).zio).length;
                            (*(*lexical_state).zio).length = ((*(*lexical_state).zio).length).wrapping_sub(1);
                            (*lexical_state).current = if fresh125 > 0 {
                                let fresh126 = (*(*lexical_state).zio).pointer;
                                (*(*lexical_state).zio).pointer = ((*(*lexical_state).zio).pointer).offset(1);
                                *fresh126 as u8 as i32
                            } else {
                                luaz_fill((*lexical_state).zio)
                            };
                            if !is_alphanumeric((*lexical_state).current + 1) {
                                break;
                            }
                        }
                        let ts: *mut TString = luax_newstring(interpreter, lexical_state, (*(*lexical_state).buffer).loads.loads_pointer, (*(*lexical_state).buffer).loads.get_length() as usize);
                        (*semantic_info).tstring = ts;
                        if (*ts).get_tag_variant() == TAG_VARIANT_STRING_SHORT && (*ts).extra as i32 > 0 {
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
                },
            }
        }
    }
}
pub unsafe fn luax_next(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) {
    unsafe {
        (*lexical_state).last_line = (*lexical_state).line_number;
        if (*lexical_state).look_ahead.token != TK_EOS as i32 {
            (*lexical_state).token = (*lexical_state).look_ahead;
            (*lexical_state).look_ahead.token = TK_EOS as i32;
        } else {
            (*lexical_state).token.token = llex(interpreter, lexical_state, &mut (*lexical_state).token.semantic_info);
        };
    }
}
pub unsafe fn luax_lookahead(interpreter: *mut Interpreter, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        (*lexical_state).look_ahead.token = llex(interpreter, lexical_state, &mut (*lexical_state).look_ahead.semantic_info);
        return (*lexical_state).look_ahead.token;
    }
}
pub unsafe fn luak_semerror(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, message: *const i8) -> ! {
    unsafe {
        (*lexical_state).token.token = 0;
        luax_syntaxerror(interpreter, lexical_state, message);
    }
}
pub fn block_follow_without_until(token: i32) -> bool {
    match token {
        TK_ELSE | TK_ELSEIF | TK_END | TK_EOS => return true,
        _ => return false,
    };
}
pub fn block_follow_with_until(token: i32) -> bool {
    match token {
        TK_ELSE | TK_ELSEIF | TK_END | TK_EOS | TK_UNTIL => return true,
        _ => return false,
    };
}
pub unsafe fn parse_statement(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let line: i32 = (*lexical_state).line_number;
        (*interpreter).luae_inccstack();
        match (*lexical_state).token.token {
            CHARACTER_SEMICOLON => {
                luax_next(interpreter, lexical_state);
            },
            TK_IF => {
                handle_if_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, line);
            },
            TK_WHILE => {
                handle_while_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, line);
            },
            TK_DO => {
                luax_next(interpreter, lexical_state);
                handle_block(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
                check_match(interpreter, lexical_state, function_state, TK_END as i32, TK_DO as i32, line);
            },
            TK_FOR => {
                handle_for_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, line);
            },
            TK_REPEAT => {
                handle_repeat_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, line);
            },
            TK_FUNCTION => {
                handle_function_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state, line);
            },
            TK_LOCAL => {
                luax_next(interpreter, lexical_state);
                if testnext(interpreter, lexical_state, function_state, TK_FUNCTION as i32) != 0 {
                    handle_local_function(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
                } else {
                    handle_local_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
                }
            },
            TK_DBCOLON => {
                luax_next(interpreter, lexical_state);
                handle_label_statement(
                    interpreter,
                    lexical_state,
                    (*lexical_state).lexical_state_function_state,
                    str_checkname(interpreter, lexical_state, function_state),
                    line,
                );
            },
            TK_RETURN => {
                luax_next(interpreter, lexical_state);
                handle_return_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
            },
            TK_BREAK => {
                breakstat(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
            },
            TK_GOTO => {
                luax_next(interpreter, lexical_state);
                gotostat(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
            },
            _ => {
                handle_expression_statement(interpreter, lexical_state, (*lexical_state).lexical_state_function_state);
            },
        }
        (*(*lexical_state).lexical_state_function_state).freereg = luay_nvarstack(lexical_state, (*lexical_state).lexical_state_function_state) as u8;
        (*interpreter).count_c_calls = (*interpreter).count_c_calls.wrapping_sub(1);
        (*interpreter).count_c_calls;
    }
}

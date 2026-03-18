use crate::blockcontrol::*;
use crate::buffer::*;
use crate::character::*;
use crate::constructorcontrol::*;
use crate::dynamicdata::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::functionstate::*;
use crate::labeldescription::*;
use crate::localvariable::*;
use crate::node::*;
use crate::object::*;
use crate::opcode::*;
use crate::operatorbinary::*;
use crate::operatorunary::*;
use crate::priority::*;
use crate::prototype::*;
use crate::state::*;
use crate::status::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tdefaultnew::*;
use crate::tobject::*;
use crate::token::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::value::*;
use crate::variabledescription::*;
use crate::vectort::*;
use crate::zio::*;
use std::ptr::*;
const MAXVARS: i32 = 200;
const UNARY_PRIORITY: i32 = 12;
pub const FIRST_RESERVED: i32 = u8::MAX as i32 + 1;
const LUA_MINBUFFER: usize = 32;

use std::sync::atomic::{AtomicBool, Ordering};

static FERRIGNO_EXTENSION_BRACE: AtomicBool = AtomicBool::new(true);
static FERRIGNO_EXTENSION_FSTRING: AtomicBool = AtomicBool::new(true);
static FERRIGNO_EXTENSION_BACKTICK: AtomicBool = AtomicBool::new(true);

pub fn ferrigno_extensions_init() {
    for (name, flag) in [
        ("FERRIGNO_EXTENSION_BRACE", &FERRIGNO_EXTENSION_BRACE),
        ("FERRIGNO_EXTENSION_FSTRING", &FERRIGNO_EXTENSION_FSTRING),
        ("FERRIGNO_EXTENSION_BACKTICK", &FERRIGNO_EXTENSION_BACKTICK),
    ] {
        if let Ok(val) = std::env::var(name) {
            flag.store(
                !matches!(val.as_str(), "0" | "false" | "no" | "off"),
                Ordering::Relaxed,
            );
        }
    }
}

// fstring state machine phases
const FSTRING_NONE: i32 = 0;
const FSTRING_OPEN_PAREN: i32 = 1;
const FSTRING_SCAN_LITERAL: i32 = 2;
const FSTRING_CONCAT: i32 = 3;
const FSTRING_EXPR: i32 = 4;
const FSTRING_CLOSE_PAREN: i32 = 5;
const FSTRING_EXPR_OPEN: i32 = 6;   // emit ( before expression
const FSTRING_EXPR_CLOSE: i32 = 7;  // emit ) after expression, then check what follows
const FSTRING_CONCAT_AFTER_EXPR: i32 = 8; // emit .. after expression close paren
const FSTRING_SCAN_LONG_LITERAL: i32 = 9; // scan literal in $[[...]] long fstring
const FSTRING_LONG: i32 = 0; // fstring_delimiter value signalling long string mode
const FSTRING_BACKTICK_CALL_OPEN: i32 = 10; // emit ( to open __ferrigno_backtick call
const FSTRING_BACKTICK_CALL_CLOSE: i32 = 11; // emit ) to close __ferrigno_backtick call
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexicalState {
    pub lexicalstate_current: i32,
    pub lexicalstate_linenumber: i32,
    pub lexicalstate_lastline: i32,
    pub lexicalstate_token: TokenInfo,
    pub lexicalstate_lookahead: TokenInfo,
    pub lexicalstate_functionstate: *mut FunctionState,
    pub lexicalstate_zio: *mut ZIO,
    pub lexicalstate_buffer: *mut Buffer,
    pub lexicalstate_table: *mut Table,
    pub lexicalstate_dynamicdata: *mut DynamicData,
    pub lexicalstate_source: *mut TString,
    pub lexicalstate_environment: *mut TString,
    pub lexicalstate_glbn: *mut TString,
    pub fstring_phase: i32,
    pub fstring_delimiter: i32,
    pub fstring_brace_depth: i32,
    pub fstring_long_sep: usize,
}
impl TDefaultNew for LexicalState {
    fn new() -> Self {
        LexicalState {
            lexicalstate_current: 0,
            lexicalstate_linenumber: 0,
            lexicalstate_lastline: 0,
            lexicalstate_token: TokenInfo::new(),
            lexicalstate_lookahead: TokenInfo::new(),
            lexicalstate_functionstate: null_mut(),
            lexicalstate_zio: null_mut(),
            lexicalstate_buffer: null_mut(),
            lexicalstate_table: null_mut(),
            lexicalstate_dynamicdata: null_mut(),
            lexicalstate_source: null_mut(),
            lexicalstate_environment: null_mut(),
            lexicalstate_glbn: null_mut(),
            fstring_phase: FSTRING_NONE,
            fstring_delimiter: 0,
            fstring_brace_depth: 0,
            fstring_long_sep: 0,
        }
    }
}
impl LexicalState {
    pub unsafe fn create_label(
        &mut self, state: *mut State, function_state: *mut FunctionState, name: *mut TString, line: i32, is_last: bool,
    ) -> bool {
        unsafe {
            let labeldescriptions: *mut VectorT<LabelDescription> = &mut (*self.lexicalstate_dynamicdata).dynamicdata_labels;
            let l: i32 = newlabelentry(
                state,
                self,
                function_state,
                labeldescriptions,
                name,
                line,
                (*function_state).code_get_label(),
            );
            if is_last {
                (*((*labeldescriptions).vectort_pointer).add(l as usize)).labeldescription_countactivevariables =
                    (*(*function_state).functionstate_blockcontrol).get_count_active_variables();
            }
            if solvegotos(
                state,
                self,
                function_state,
                &mut *((*labeldescriptions).vectort_pointer).add(l as usize),
            ) {
                code_abck(
                    state,
                    self,
                    function_state,
                    OPCODE_CLOSE,
                    luay_nvarstack(self, function_state),
                    0,
                    0,
                    0,
                );
                return true;
            }
            false
        }
    }
    pub unsafe fn parse_expression(
        &mut self, state: *mut State, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription,
    ) {
        unsafe {
            subexpr(state, self, function_state, expression_description, 0);
        }
    }
    pub unsafe fn parse_expression_list(
        &mut self, state: *mut State, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription,
    ) -> i32 {
        unsafe {
            let mut count: i32 = 1;
            self.parse_expression(state, function_state, expression_description);
            while testnext(state, self, function_state, Character::Comma as i32) != 0 {
                luak_exp2nextreg(state, self, self.lexicalstate_functionstate, expression_description);
                self.parse_expression(state, function_state, expression_description);
                count += 1;
            }
            count
        }
    }
    pub unsafe fn add_prototype(&mut self, state: *mut State) -> *mut Prototype {
        unsafe {
            let function_state: *mut FunctionState = self.lexicalstate_functionstate;
            let prototype: *mut Prototype = (*function_state).functionstate_prototype;
            if (*function_state).functionstate_count_prototypes >= (*prototype).prototype_prototypes.get_size() as i32 {
                let mut oldsize = (*prototype).prototype_prototypes.get_size();
                (*prototype).prototype_prototypes.grow(
                    state,
                    (*function_state).functionstate_count_prototypes as usize,
                    if ((1 << (8 + 8 + 1)) - 1) as usize <= (!0usize) / size_of::<*mut Prototype>() {
                        (1 << (8 + 8 + 1)) - 1
                    } else {
                        (!0usize) / size_of::<*mut Prototype>()
                    },
                    c"functions".as_ptr(),
                );
                while oldsize < (*prototype).prototype_prototypes.get_size() {
                    let prev_size = oldsize;
                    oldsize += 1;
                    let slot = &mut *((*prototype).prototype_prototypes.vectort_pointer).add(prev_size);
                    *slot = null_mut();
                }
            }
            let clp: *mut Prototype = luaf_newproto(state);
            let np = (*function_state).functionstate_count_prototypes;
            (*function_state).functionstate_count_prototypes += 1;
            let target = &mut *((*prototype).prototype_prototypes.vectort_pointer).add(np as usize);
            *target = clp;
            if (*prototype).get_marked() & BLACKBIT != 0 && (*clp).get_marked() & WHITEBITS != 0 {
                Object::luac_barrier_(state, prototype as *mut Object, clp as *mut Object);
            }
            clp
        }
    }
}
pub unsafe fn find_label(
    function_state: *mut FunctionState, dynamic_data: *mut DynamicData, name: *mut TString,
) -> *mut LabelDescription {
    unsafe {
        for i in (*function_state).functionstate_first_label..(*dynamic_data).dynamicdata_labels.get_length() as i32 {
            let candidate = &mut *((*dynamic_data).dynamicdata_labels.vectort_pointer).add(i as usize);
            if candidate.labeldescription_name == name {
                return candidate;
            }
        }
        null_mut()
    }
}
pub unsafe fn newlabelentry(
    state: *mut State, _lexical_state: *mut LexicalState, function_state: *mut FunctionState, l: *mut VectorT<LabelDescription>,
    name: *mut TString, line: i32, program_counter: i32,
) -> i32 {
    unsafe {
        let n = (*l).get_length();
        (*l).grow(
            state,
            n,
            if i16::MAX as usize <= (!0usize) / size_of::<LabelDescription>() {
                i16::MAX as usize
            } else {
                (!0usize) / size_of::<LabelDescription>()
            },
            c"labels/gotos".as_ptr(),
        );
        let name_slot = &mut (*((*l).vectort_pointer).add(n)).labeldescription_name;
        *name_slot = name;
        (*((*l).vectort_pointer).add(n)).labeldescription_line = line;
        (*((*l).vectort_pointer).add(n)).labeldescription_countactivevariables =
            (*function_state).functionstate_count_active_variables;
        (*((*l).vectort_pointer).add(n)).labeldescription_close = 0;
        (*((*l).vectort_pointer).add(n)).labeldescription_programcounter = program_counter;
        (*l).set_length(n + 1);
        n as i32
    }
}
pub unsafe fn newgotoentry(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, name: *mut TString, line: i32,
    program_counter: i32,
) -> i32 {
    unsafe {
        newlabelentry(
            state,
            lexical_state,
            function_state,
            &mut (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto,
            name,
            line,
            program_counter,
        )
    }
}
pub unsafe fn solvegotos(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    label_description: *mut LabelDescription,
) -> bool {
    unsafe {
        let gl: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto;
        let mut i = (*function_state).get_first_goto();
        let mut needsclose = false;
        while i < (*gl).get_length() {
            if (*((*gl).vectort_pointer).add(i)).labeldescription_name == (*label_description).labeldescription_name {
                needsclose = needsclose || (0 != (*((*gl).vectort_pointer).add(i)).labeldescription_close);
                solvegoto(state, lexical_state, i, label_description);
            } else {
                i += 1;
            }
        }
        needsclose
    }
}
pub unsafe fn undefgoto(
    state: *mut State, lexical_state: *mut LexicalState, _function_state: *mut FunctionState,
    goto_label_description: *mut LabelDescription,
) -> ! {
    unsafe {
        let mut message: *const i8;
        if (*goto_label_description).labeldescription_name == luas_newlstr(state, c"break".as_ptr(), size_of::<[i8; 6]>() - 1) {
            message = c"break outside loop at line %d".as_ptr();
            message = luao_pushfstring(state, message, &[(*goto_label_description).labeldescription_line.into()]);
        } else {
            message = c"no visible label '%s' for <goto> at line %d".as_ptr();
            message = luao_pushfstring(
                state,
                message,
                &[(*(*goto_label_description).labeldescription_name).get_contents_mut().into(),
                (*goto_label_description).labeldescription_line.into()],
            );
        }
        luak_semerror(state, lexical_state, message);
    }
}
pub unsafe fn codeclosure(state: *mut State, lexical_state: *mut LexicalState, v: *mut ExpressionDescription) {
    unsafe {
        let function_state: *mut FunctionState = (*(*lexical_state).lexicalstate_functionstate).functionstate_previous;
        ExpressionDescription::init_exp(
            v,
            ExpressionKind::Relocatable,
            luak_codeabx(
                state,
                lexical_state,
                function_state,
                OPCODE_CLOSURE,
                0,
                ((*function_state).functionstate_count_prototypes - 1) as u32,
            ),
        );
        luak_exp2nextreg(state, lexical_state, function_state, v);
    }
}
pub unsafe fn open_function(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, previous: *mut FunctionState,
    block_control: *mut BlockControl,
) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        (*function_state).functionstate_previous = previous;
        (*lexical_state).lexicalstate_functionstate = function_state;
        (*function_state).functionstate_program_counter = 0;
        (*function_state).functionstate_previous_line = (*prototype).prototype_linedefined;
        (*function_state).functionstate_i_width_absolute = 0;
        (*function_state).functionstate_last_target = 0;
        (*function_state).functionstate_free_register = 0;
        (*function_state).functionstate_count_constants = 0;
        (*function_state).functionstate_count_absolute_line_info = 0;
        (*function_state).functionstate_count_prototypes = 0;
        (*function_state).functionstate_count_upvalues = 0;
        (*function_state).functionstate_count_debug_variables = 0;
        (*function_state).functionstate_count_active_variables = 0;
        (*function_state).functionstate_needs_close = false;
        (*function_state).functionstate_first_local = (*(*lexical_state).lexicalstate_dynamicdata)
            .dynamicdata_active_variables
            .get_length() as i32;
        (*function_state).functionstate_first_label =
            (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_labels.get_length() as i32;
        (*function_state).functionstate_blockcontrol = null_mut();
        (*prototype).prototype_source = (*lexical_state).lexicalstate_source;
        if (*prototype).get_marked() & BLACKBIT != 0 && (*(*prototype).prototype_source).get_marked() & WHITEBITS != 0 {
            Object::luac_barrier_(state, prototype as *mut Object, (*prototype).prototype_source as *mut Object);
        }
        (*prototype).prototype_maximumstacksize = 2_u8;
        (*block_control).enter_block(lexical_state, function_state, false);
        (*function_state).functionstate_blockcontrol = block_control;
    }
}
pub unsafe fn close_function(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        luak_ret(
            state,
            lexical_state,
            function_state,
            luay_nvarstack(lexical_state, function_state),
            0,
        );
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
        luak_finish(state, lexical_state, function_state, (*function_state).functionstate_prototype);
        (*prototype)
            .prototype_code
            .shrink(&mut *state, (*function_state).functionstate_program_counter as usize);
        (*prototype)
            .prototype_lineinfo
            .shrink(&mut *state, (*function_state).functionstate_program_counter as usize);
        (*prototype)
            .prototype_absolutelineinfo
            .shrink(&mut *state, (*function_state).functionstate_count_absolute_line_info as usize);
        (*prototype)
            .prototype_constants
            .shrink(&mut *state, (*function_state).functionstate_count_constants as usize);
        (*prototype)
            .prototype_prototypes
            .shrink(&mut *state, (*function_state).functionstate_count_prototypes as usize);
        (*prototype)
            .prototype_localvariables
            .shrink(&mut *state, (*function_state).functionstate_count_debug_variables);
        (*prototype)
            .prototype_upvalues
            .shrink(&mut *state, (*function_state).functionstate_count_upvalues);
        (*lexical_state).lexicalstate_functionstate = (*function_state).functionstate_previous;
        (*state).do_gc_step_if_should_step();
    }
}
pub unsafe fn parse_statement_list(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        const TK_RETURN: i32 = Token::Return as i32;
        while !block_follow_with_until((*lexical_state).lexicalstate_token.token) {
            if (*lexical_state).lexicalstate_token.token == TK_RETURN {
                parse_statement(state, lexical_state, function_state);
                return;
            } else {
                parse_statement(state, lexical_state, function_state);
            }
        }
    }
}
pub unsafe fn fieldsel(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) {
    unsafe {
        let mut key: ExpressionDescription = ExpressionDescription::new();
        luak_exp2anyregup(state, lexical_state, function_state, v);
        luax_next(state, lexical_state);
        codename(state, lexical_state, function_state, &mut key);
        ExpressionDescription::luak_indexed(state, lexical_state, function_state, v, &mut key);
    }
}
pub unsafe fn yindex(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) {
    unsafe {
        luax_next(state, lexical_state);
        (*lexical_state).parse_expression(state, function_state, v);
        code_expression_to_value(state, lexical_state, (*lexical_state).lexicalstate_functionstate, v);
        checknext(state, lexical_state, function_state, Character::BracketRight as i32);
    }
}
pub unsafe fn recfield(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        let reg: i32 = (*(*lexical_state).lexicalstate_functionstate).functionstate_free_register as i32;
        let mut key: ExpressionDescription = ExpressionDescription::new();
        let mut value: ExpressionDescription = ExpressionDescription::new();
        if (*lexical_state).lexicalstate_token.token == Token::Name as i32 {
            codename(state, lexical_state, function_state, &mut key);
        } else {
            yindex(state, lexical_state, function_state, &mut key);
        }
        (*constructor_control).constructorcontrol_count_table += 1;
        checknext(state, lexical_state, function_state, Character::Equal as i32);
        let mut table: ExpressionDescription = *(*constructor_control).constructorcontrol_table;
        ExpressionDescription::luak_indexed(state, lexical_state, function_state, &mut table, &mut key);
        (*lexical_state).parse_expression(state, function_state, &mut value);
        luak_storevar(state, lexical_state, function_state, &mut table, &mut value);
        (*function_state).functionstate_free_register = reg as u8;
    }
}
pub unsafe fn listfield(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        (*lexical_state).parse_expression(
            state,
            function_state,
            &mut (*constructor_control).constructorcontrol_expressiondescription,
        );
        (*constructor_control).constructorcontrol_count_to_store += 1;
        (*constructor_control).constructorcontrol_count_to_store;
    }
}
pub unsafe fn field(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        const TK_CHARACTER_BRACKET_LEFT: i32 = Character::BracketLeft as i32;
        const TK_NAME: i32 = Token::Name as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_NAME => {
                if luax_lookahead(state, lexical_state) != Character::Equal as i32 {
                    listfield(state, lexical_state, function_state, constructor_control);
                } else {
                    recfield(state, lexical_state, function_state, constructor_control);
                }
            },
            | TK_CHARACTER_BRACKET_LEFT => {
                recfield(state, lexical_state, function_state, constructor_control);
            },
            | _ => {
                listfield(state, lexical_state, function_state, constructor_control);
            },
        };
    }
}
pub unsafe fn constructor(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, t: *mut ExpressionDescription,
) {
    unsafe {
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        let program_counter: i32 = code_abck(state, lexical_state, function_state, OPCODE_NEWTABLE, 0, 0, 0, 0);
        let mut constructor_control: ConstructorControl = ConstructorControl::new();
        luak_code(state, lexical_state, function_state, 0);
        constructor_control.constructorcontrol_count_to_store = 0;
        constructor_control.constructorcontrol_count_table = 0;
        constructor_control.constructorcontrol_count_array = 0;
        constructor_control.constructorcontrol_table = t;
        ExpressionDescription::init_exp(
            t,
            ExpressionKind::Nonrelocatable,
            (*function_state).functionstate_free_register as i32,
        );
        luak_reserveregs(state, lexical_state, function_state, 1);
        ExpressionDescription::init_exp(
            &mut constructor_control.constructorcontrol_expressiondescription,
            ExpressionKind::Void,
            0,
        );
        checknext(state, lexical_state, function_state, Character::BraceLeft as i32);
        constructor_control.constructorcontrol_max_to_store = maxtostore(function_state);
        while (*lexical_state).lexicalstate_token.token != Character::BraceRight as i32 {
            if constructor_control
                .constructorcontrol_expressiondescription
                .expressiondescription_expressionkind
                != ExpressionKind::Void
            {
                closelistfield(state, lexical_state, function_state, &mut constructor_control);
            }
            field(state, lexical_state, function_state, &mut constructor_control);
            checklimit(
                state,
                lexical_state,
                function_state,
                constructor_control.constructorcontrol_count_to_store
                    + constructor_control.constructorcontrol_count_array
                    + constructor_control.constructorcontrol_count_table,
                i32::MAX / 2,
                c"items in a constructor".as_ptr(),
            );
            if !(testnext(state, lexical_state, function_state, Character::Comma as i32) != 0
                || testnext(state, lexical_state, function_state, Character::Semicolon as i32) != 0)
            {
                break;
            }
        }
        check_match(
            state,
            lexical_state,
            function_state,
            Character::BraceRight as i32,
            Character::BraceLeft as i32,
            line,
        );
        lastlistfield(state, lexical_state, function_state, &mut constructor_control);
        luak_settablesize(
            state,
            function_state,
            program_counter,
            (*t).expressiondescription_value.value_info,
            constructor_control.constructorcontrol_count_array,
            constructor_control.constructorcontrol_count_table,
        );
    }
}
pub unsafe fn parlist(state: *mut State, lexical_state: *mut LexicalState) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let mut nparams: i32 = 0;
        let mut is_variable_arguments = false;
        const TK_NAME: i32 = Token::Name as i32;
        const TK_DOTS: i32 = Token::Dots as i32;
        if (*lexical_state).lexicalstate_token.token != Character::ParenthesisRight as i32 {
            loop {
                match (*lexical_state).lexicalstate_token.token {
                    | TK_NAME => {
                        new_localvar(state, lexical_state, str_checkname(state, lexical_state, function_state));
                        nparams += 1;
                    },
                    | TK_DOTS => {
                        luax_next(state, lexical_state);
                        is_variable_arguments = true;
                        if (*lexical_state).lexicalstate_token.token == TK_NAME {
                            // Named vararg: ...name
                            let vidx = new_localvar(state, lexical_state, str_checkname(state, lexical_state, function_state));
                            (*getlocalvardesc(lexical_state, function_state, vidx))
                                .variabledescription_content
                                .variabledescriptioncontent_kind = RDKVAVAR as u8;
                        } else {
                            let s = c"(vararg table)";
                            new_localvar(
                                state,
                                lexical_state,
                                luax_newstring(state, lexical_state, s.as_ptr(), cstr_len(s.as_ptr())),
                            );
                        }
                    },
                    | _ => {
                        luax_syntaxerror(state, lexical_state, c"<name> or '...' expected".as_ptr());
                    },
                }
                if !(!is_variable_arguments && testnext(state, lexical_state, function_state, Character::Comma as i32) != 0) {
                    break;
                }
            }
        }
        adjustlocalvars(state, lexical_state, nparams);
        (*prototype).prototype_countparameters = (*function_state).functionstate_count_active_variables;
        if is_variable_arguments {
            setvararg(
                state,
                lexical_state,
                function_state,
                (*prototype).prototype_countparameters as i32,
            );
            adjustlocalvars(state, lexical_state, 1);
        }
        luak_reserveregs(
            state,
            lexical_state,
            function_state,
            (*function_state).functionstate_count_active_variables as i32,
        );
    }
}
pub unsafe fn body(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, is_method: bool, line: i32,
) {
    unsafe {
        let mut new_fs: FunctionState = FunctionState {
            functionstate_prototype: null_mut(),
            functionstate_previous: null_mut(),
            functionstate_blockcontrol: null_mut(),
            functionstate_program_counter: 0,
            functionstate_last_target: 0,
            functionstate_previous_line: 0,
            functionstate_count_constants: 0,
            functionstate_count_prototypes: 0,
            functionstate_count_absolute_line_info: 0,
            functionstate_first_local: 0,
            functionstate_first_label: 0,
            functionstate_count_debug_variables: 0,
            functionstate_count_active_variables: 0,
            functionstate_count_upvalues: 0,
            functionstate_free_register: 0,
            functionstate_i_width_absolute: 0,
            functionstate_needs_close: false,
        };
        let mut block_control = BlockControl::new();
        new_fs.functionstate_prototype = (*lexical_state).add_prototype(state);
        (*new_fs.functionstate_prototype).prototype_linedefined = line;
        open_function(state, lexical_state, &mut new_fs, function_state, &mut block_control);
        checknext(state, lexical_state, function_state, Character::ParenthesisLeft as i32);
        if is_method {
            new_localvar(
                state,
                lexical_state,
                luax_newstring(state, lexical_state, c"self".as_ptr(), size_of::<[i8; 5]>() - 1),
            );
            adjustlocalvars(state, lexical_state, 1);
        }
        parlist(state, lexical_state);
        checknext(state, lexical_state, function_state, Character::ParenthesisRight as i32);
        let brace_body = FERRIGNO_EXTENSION_BRACE.load(Ordering::Relaxed)
            && (*lexical_state).lexicalstate_token.token == Character::BraceLeft as i32;
        if brace_body {
            luax_next(state, lexical_state);
        }
        parse_statement_list(state, lexical_state, function_state);
        (*new_fs.functionstate_prototype).prototype_lastlinedefined = (*lexical_state).lexicalstate_linenumber;
        if brace_body {
            check_match(
                state,
                lexical_state,
                function_state,
                Character::BraceRight as i32,
                Character::BraceLeft as i32,
                line,
            );
        } else {
            check_match(
                state,
                lexical_state,
                function_state,
                Token::End as i32,
                Token::Function as i32,
                line,
            );
        }
        codeclosure(state, lexical_state, expression_description);
        close_function(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
    }
}
pub unsafe fn funcargs(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let mut args: ExpressionDescription = ExpressionDescription::new();
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        const TK_CHARACTER_BRACE_LEFT: i32 = Character::BraceLeft as i32;
        const TK_CHARACTER_PARENTHESIS_LEFT: i32 = Character::ParenthesisLeft as i32;
        const TK_STRING: i32 = Token::String as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_CHARACTER_PARENTHESIS_LEFT => {
                luax_next(state, lexical_state);
                if (*lexical_state).lexicalstate_token.token == Character::ParenthesisRight as i32 {
                    args.expressiondescription_expressionkind = ExpressionKind::Void;
                } else {
                    (*lexical_state).parse_expression_list(state, function_state, &mut args);
                    if args.expressiondescription_expressionkind == ExpressionKind::Call
                        || args.expressiondescription_expressionkind == ExpressionKind::VariableArguments
                    {
                        luak_setreturns(state, lexical_state, function_state, &mut args, -1);
                    }
                }
                check_match(
                    state,
                    lexical_state,
                    function_state,
                    Character::ParenthesisRight as i32,
                    Character::ParenthesisLeft as i32,
                    line,
                );
            },
            | TK_CHARACTER_BRACE_LEFT => {
                constructor(state, lexical_state, function_state, &mut args);
            },
            | TK_STRING => {
                ExpressionDescription::codestring(&mut args, (*lexical_state).lexicalstate_token.semantic_info.value_tstring);
                luax_next(state, lexical_state);
            },
            | _ => {
                luax_syntaxerror(state, lexical_state, c"function arguments expected".as_ptr());
            },
        }
        let base: i32 = (*expression_description).expressiondescription_value.value_info;
        let nparams: i32;
        if args.expressiondescription_expressionkind == ExpressionKind::Call
            || args.expressiondescription_expressionkind == ExpressionKind::VariableArguments
        {
            nparams = -1;
        } else {
            if args.expressiondescription_expressionkind != ExpressionKind::Void {
                luak_exp2nextreg(state, lexical_state, function_state, &mut args);
            }
            nparams = (*function_state).functionstate_free_register as i32 - (base + 1);
        }
        ExpressionDescription::init_exp(
            expression_description,
            ExpressionKind::Call,
            code_abck(state, lexical_state, function_state, OPCODE_CALL, base, nparams + 1, 2, 0),
        );
        luak_fixline(state, lexical_state, function_state, line);
        (*function_state).functionstate_free_register = (base + 1) as u8;
    }
}
pub unsafe fn primaryexp(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) {
    unsafe {
        const TK_CHARACTER_PARENTHESIS_LEFT: i32 = Character::ParenthesisLeft as i32;
        const TK_NAME: i32 = Token::Name as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_CHARACTER_PARENTHESIS_LEFT => {
                let line: i32 = (*lexical_state).lexicalstate_linenumber;
                luax_next(state, lexical_state);
                (*lexical_state).parse_expression(state, function_state, v);
                check_match(
                    state,
                    lexical_state,
                    function_state,
                    Character::ParenthesisRight as i32,
                    Character::ParenthesisLeft as i32,
                    line,
                );
                luak_dischargevars(state, lexical_state, (*lexical_state).lexicalstate_functionstate, v);
            },
            | TK_NAME => {
                singlevar(state, lexical_state, function_state, v);
            },
            | _ => {
                luax_syntaxerror(state, lexical_state, c"unexpected symbol".as_ptr());
            },
        }
    }
}
pub unsafe fn suffixedexp(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) {
    unsafe {
        primaryexp(state, lexical_state, function_state, v);
        const TK_CHARACTER_COLON: i32 = Character::Colon as i32;
        const TK_CHARACTER_PERIOD: i32 = Character::Period as i32;
        const TK_CHARACTER_BRACKET_LEFT: i32 = Character::BracketLeft as i32;
        const TK_CHARACTER_BRACE_LEFT: i32 = Character::BraceLeft as i32;
        const TK_CHARACTER_PARENTHESIS_LEFT: i32 = Character::ParenthesisLeft as i32;
        const TK_STRING: i32 = Token::String as i32;
        loop {
            match (*lexical_state).lexicalstate_token.token {
                | TK_CHARACTER_PERIOD => {
                    fieldsel(state, lexical_state, function_state, v);
                },
                | TK_CHARACTER_BRACKET_LEFT => {
                    let mut key: ExpressionDescription = ExpressionDescription::new();
                    luak_exp2anyregup(state, lexical_state, function_state, v);
                    yindex(state, lexical_state, function_state, &mut key);
                    ExpressionDescription::luak_indexed(state, lexical_state, function_state, v, &mut key);
                },
                | TK_CHARACTER_COLON => {
                    let mut key: ExpressionDescription = ExpressionDescription::new();
                    luax_next(state, lexical_state);
                    codename(state, lexical_state, function_state, &mut key);
                    luak_self(state, lexical_state, function_state, v, &mut key);
                    funcargs(state, lexical_state, function_state, v);
                },
                | TK_CHARACTER_PARENTHESIS_LEFT | TK_STRING | TK_CHARACTER_BRACE_LEFT => {
                    luak_exp2nextreg(state, lexical_state, function_state, v);
                    funcargs(state, lexical_state, function_state, v);
                },
                | _ => return,
            }
        }
    }
}
pub unsafe fn simpleexp(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) {
    unsafe {
        const TK_CHARACTER_BRACE_LEFT: i32 = Character::BraceLeft as i32;
        const TK_INTEGER: i32 = Token::Integer as i32;
        const TK_FLOAT: i32 = Token::Float as i32;
        const TK_STRING: i32 = Token::String as i32;
        const TK_DOTS: i32 = Token::Dots as i32;
        const TK_FUNCTION: i32 = Token::Function as i32;
        const TK_FALSE: i32 = Token::False as i32;
        const TK_TRUE: i32 = Token::True as i32;
        const TK_NIL: i32 = Token::Nil as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_FLOAT => {
                ExpressionDescription::init_exp(v, ExpressionKind::ConstantNumber, 0);
                (*v).expressiondescription_value.value_number = (*lexical_state).lexicalstate_token.semantic_info.value_number;
            },
            | TK_INTEGER => {
                ExpressionDescription::init_exp(v, ExpressionKind::ConstantInteger, 0);
                (*v).expressiondescription_value.value_integer = (*lexical_state).lexicalstate_token.semantic_info.value_integer;
            },
            | TK_STRING => {
                ExpressionDescription::codestring(v, (*lexical_state).lexicalstate_token.semantic_info.value_tstring);
            },
            | TK_NIL => {
                ExpressionDescription::init_exp(v, ExpressionKind::Nil, 0);
            },
            | TK_TRUE => {
                ExpressionDescription::init_exp(v, ExpressionKind::True, 0);
            },
            | TK_FALSE => {
                ExpressionDescription::init_exp(v, ExpressionKind::False, 0);
            },
            | TK_DOTS => {
                let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
                if !(*(*function_state).functionstate_prototype).prototype_isvariablearguments
                    && !(*(*function_state).functionstate_prototype).prototype_needsvarargtable
                {
                    luax_syntaxerror(state, lexical_state, c"cannot use '...' outside a vararg function".as_ptr());
                }
                ExpressionDescription::init_exp(
                    v,
                    ExpressionKind::VariableArguments,
                    code_abck(
                        state,
                        lexical_state,
                        function_state,
                        OPCODE_VARARG,
                        0,
                        (*(*function_state).functionstate_prototype).prototype_countparameters as i32,
                        1,
                        0,
                    ),
                );
            },
            | TK_CHARACTER_BRACE_LEFT => {
                constructor(state, lexical_state, function_state, v);
                return;
            },
            | TK_FUNCTION => {
                luax_next(state, lexical_state);
                body(
                    state,
                    lexical_state,
                    function_state,
                    v,
                    false,
                    (*lexical_state).lexicalstate_linenumber,
                );
                return;
            },
            | _ => {
                suffixedexp(state, lexical_state, function_state, v);
                return;
            },
        }
        luax_next(state, lexical_state);
    }
}
pub unsafe fn error_expected(
    state: *mut State, lexical_state: *mut LexicalState, _function_state: *mut FunctionState, token: i32,
) -> ! {
    unsafe {
        luax_syntaxerror(
            state,
            lexical_state,
            luao_pushfstring(state, c"%s expected".as_ptr(), &[luax_token2str(state, lexical_state, token).into()]),
        );
    }
}
pub unsafe fn testnext(state: *mut State, lexical_state: *mut LexicalState, _function_state: *mut FunctionState, c: i32) -> i32 {
    unsafe {
        if (*lexical_state).lexicalstate_token.token == c {
            luax_next(state, lexical_state);
            1
        } else {
            0
        }
    }
}
pub unsafe fn check(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, c: i32) {
    unsafe {
        if (*lexical_state).lexicalstate_token.token != c {
            error_expected(state, lexical_state, function_state, c);
        }
    }
}
pub unsafe fn checknext(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, c: i32) {
    unsafe {
        check(state, lexical_state, function_state, c);
        luax_next(state, lexical_state);
    }
}
pub unsafe fn check_match(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, what: i32, who: i32, where_pos: i32,
) {
    unsafe {
        if testnext(state, lexical_state, function_state, what) == 0 {
            if where_pos == (*lexical_state).lexicalstate_linenumber {
                error_expected(state, lexical_state, function_state, what);
            } else {
                luax_syntaxerror(
                    state,
                    lexical_state,
                    luao_pushfstring(
                        state,
                        c"%s expected (to close %s at line %d)".as_ptr(),
                        &[luax_token2str(state, lexical_state, what).into(),
                        luax_token2str(state, lexical_state, who).into(),
                        where_pos.into()],
                    ),
                );
            }
        }
    }
}
pub unsafe fn str_checkname(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
) -> *mut TString {
    unsafe {
        check(state, lexical_state, function_state, Token::Name as i32);
        let tstring: *mut TString = (*lexical_state).lexicalstate_token.semantic_info.value_tstring;
        luax_next(state, lexical_state);
        tstring
    }
}
pub unsafe fn codename(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        ExpressionDescription::codestring(expression_description, str_checkname(state, lexical_state, function_state));
    }
}
pub unsafe fn registerlocalvar(
    state: *mut State, _lexical_state: *mut LexicalState, function_state: *mut FunctionState, variable_name: *mut TString,
) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let mut oldsize = (*prototype).prototype_localvariables.get_size();
        (*prototype).prototype_localvariables.grow(
            state,
            (*function_state).functionstate_count_debug_variables,
            if i16::MAX as usize <= (!0usize) / size_of::<LocalVariable>() {
                i16::MAX as usize
            } else {
                (!0usize) / size_of::<LocalVariable>()
            },
            c"local variables".as_ptr(),
        );
        while oldsize < (*prototype).prototype_localvariables.get_size() {
            let prev_size = oldsize;
            oldsize += 1;
            let name_slot =
                &mut (*((*prototype).prototype_localvariables.vectort_pointer).add(prev_size)).localvariable_variablename;
            *name_slot = null_mut();
        }
        let name_slot = &mut (*((*prototype).prototype_localvariables.vectort_pointer)
            .add((*function_state).functionstate_count_debug_variables))
        .localvariable_variablename;
        *name_slot = variable_name;
        (*((*prototype).prototype_localvariables.vectort_pointer).add((*function_state).functionstate_count_debug_variables))
            .localvariable_startprogramcounter = (*function_state).functionstate_program_counter;
        if (*prototype).get_marked() & BLACKBIT != 0 && (*variable_name).get_marked() & WHITEBITS != 0 {
            Object::luac_barrier_(state, prototype as *mut Object, variable_name as *mut Object);
        }
        let prev_count = (*function_state).functionstate_count_debug_variables;
        (*function_state).functionstate_count_debug_variables += 1;
        prev_count as i32
    }
}
pub unsafe fn new_localvar(state: *mut State, lexical_state: *mut LexicalState, name: *mut TString) -> i32 {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
        let dynamic_data: *mut DynamicData = (*lexical_state).lexicalstate_dynamicdata;

        (*dynamic_data).dynamicdata_active_variables.grow(
            state,
            (*dynamic_data).dynamicdata_active_variables.get_length() + 1,
            if i16::MAX as usize <= (!0usize) / size_of::<VariableDescription>() {
                i16::MAX as usize
            } else {
                (!0usize) / size_of::<VariableDescription>()
            },
            c"local variables".as_ptr(),
        );
        let prev_length = (*dynamic_data).dynamicdata_active_variables.get_length();
        (*dynamic_data)
            .dynamicdata_active_variables
            .set_length((*dynamic_data).dynamicdata_active_variables.get_length() + 1);
        let var: *mut VariableDescription =
            &mut *((*dynamic_data).dynamicdata_active_variables.vectort_pointer).add(prev_length) as *mut VariableDescription;
        (*var).variabledescription_content.variabledescriptioncontent_kind = 0;
        (*var).variabledescription_content.variabledescriptioncontent_name = name;
        (*dynamic_data).dynamicdata_active_variables.get_length() as i32 - 1 - (*function_state).functionstate_first_local
    }
}
pub unsafe fn check_readonly(
    state: *mut State, lexical_state: *mut LexicalState, expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
        let mut variable_name: *mut TString = null_mut();
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Constant2 => {
                variable_name = (*((*(*lexical_state).lexicalstate_dynamicdata)
                    .dynamicdata_active_variables
                    .vectort_pointer)
                    .add((*expression_description).expressiondescription_value.value_info as usize))
                .variabledescription_content
                .variabledescriptioncontent_name;
            },
            | ExpressionKind::Local | ExpressionKind::VarargVariable => {
                let vardesc: *mut VariableDescription = getlocalvardesc(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_variable
                        .valueregister_valueindex as i32,
                );
                if (*vardesc).variabledescription_content.variabledescriptioncontent_kind as i32 != 0 {
                    variable_name = (*vardesc).variabledescription_content.variabledescriptioncontent_name;
                }
            },
            | ExpressionKind::UpValue => {
                let upvaluedescription: *mut UpValueDescription =
                    &mut *((*(*function_state).functionstate_prototype).prototype_upvalues.vectort_pointer)
                        .add((*expression_description).expressiondescription_value.value_info as usize)
                        as *mut UpValueDescription;
                if (*upvaluedescription).upvaluedescription_kind as i32 != 0 {
                    variable_name = (*upvaluedescription).upvaluedescription_name;
                }
            },
            | ExpressionKind::VarargIndex => {
                (*(*function_state).functionstate_prototype).prototype_needsvarargtable = true;
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Indexed;
                // fall through to indexed check
                if (*expression_description)
                    .expressiondescription_value
                    .value_index
                    .valuereference_readonly
                    != 0
                {
                    let keystr = (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_keystr;
                    variable_name = (*(*function_state).functionstate_prototype)
                        .prototype_constants
                        .vectort_pointer
                        .add(keystr as usize)
                        .as_ref()
                        .unwrap()
                        .as_string()
                        .unwrap();
                }
            },
            | ExpressionKind::IndexUpValue | ExpressionKind::Indexed | ExpressionKind::IndexInteger | ExpressionKind::Field => {
                if (*expression_description)
                    .expressiondescription_value
                    .value_index
                    .valuereference_readonly
                    != 0
                {
                    let keystr = (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_keystr;
                    variable_name = (*(*function_state).functionstate_prototype)
                        .prototype_constants
                        .vectort_pointer
                        .add(keystr as usize)
                        .as_ref()
                        .unwrap()
                        .as_string()
                        .unwrap();
                }
            },
            | _ => return,
        }
        if !variable_name.is_null() {
            let message: *const i8 = luao_pushfstring(
                state,
                c"attempt to assign to const variable '%s'".as_ptr(),
                &[(*variable_name).get_contents_mut().into()],
            );
            luak_semerror(state, lexical_state, message);
        }
    }
}
pub unsafe fn adjustlocalvars(state: *mut State, lexical_state: *mut LexicalState, count_variables: i32) {
    unsafe {
        let function_state = (*lexical_state).lexicalstate_functionstate;
        let mut reglevel = luay_nvarstack(lexical_state, function_state);
        for _ in 0..count_variables {
            let prev_count = (*function_state).functionstate_count_active_variables;
            (*function_state).functionstate_count_active_variables = (*function_state).functionstate_count_active_variables + 1;
            let vidx = prev_count as i32;
            let var = getlocalvardesc(lexical_state, function_state, vidx);
            let prev_reglevel = reglevel;
            reglevel += 1;
            (*var).variabledescription_content.variabledescriptioncontent_registerindex = prev_reglevel as u8;
            (*var).variabledescription_content.variabledescriptioncontent_pidx = registerlocalvar(
                state,
                lexical_state,
                function_state,
                (*var).variabledescription_content.variabledescriptioncontent_name,
            ) as i16;
            checklimit(
                state,
                lexical_state,
                function_state,
                reglevel,
                MAXVARS,
                c"local variables".as_ptr(),
            );
        }
    }
}
pub unsafe fn buildglobal(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, varname: *mut TString,
    var: *mut ExpressionDescription,
) {
    unsafe {
        let mut key: ExpressionDescription = ExpressionDescription::new();
        ExpressionDescription::init_exp(var, ExpressionKind::Global, -1);
        singlevaraux(
            state,
            lexical_state,
            function_state,
            (*lexical_state).lexicalstate_environment,
            var,
            1,
        );
        if (*var).expressiondescription_expressionkind == ExpressionKind::Global {
            luak_semerror(
                state,
                lexical_state,
                luao_pushfstring(
                    state,
                    c"_ENV is global when accessing variable '%s'".as_ptr(),
                    &[(*varname).get_contents_mut().into()],
                ),
            );
        }
        luak_exp2anyregup(state, lexical_state, function_state, var);
        ExpressionDescription::codestring(&mut key, varname);
        ExpressionDescription::luak_indexed(state, lexical_state, function_state, var, &mut key);
    }
}
pub unsafe fn singlevar(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, var: *mut ExpressionDescription,
) {
    unsafe {
        let variable_name: *mut TString = str_checkname(state, lexical_state, function_state);
        ExpressionDescription::init_exp(var, ExpressionKind::Global, -1);
        singlevaraux(state, lexical_state, function_state, variable_name, var, 1);
        if (*var).expressiondescription_expressionkind == ExpressionKind::Global {
            let info = (*var).expressiondescription_value.value_info;
            if info == -2 {
                luak_semerror(
                    state,
                    lexical_state,
                    luao_pushfstring(
                        state,
                        c"variable '%s' not declared".as_ptr(),
                        &[(*variable_name).get_contents_mut().into()],
                    ),
                );
            }
            buildglobal(state, lexical_state, function_state, variable_name, var);
            if info >= 0 {
                let vd = &*(*(*lexical_state).lexicalstate_dynamicdata)
                    .dynamicdata_active_variables
                    .vectort_pointer
                    .add(info as usize);
                let kind = vd.variabledescription_content.variabledescriptioncontent_kind as i32;
                if kind == GDKCONST {
                    (*var).expressiondescription_value.value_index.valuereference_readonly = 1;
                }
            }
        }
    }
}
pub unsafe fn adjust_assign(
    state: *mut State, lexical_state: *mut LexicalState, count_variables: i32, count_expressions: i32,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
        let needed: i32 = count_variables - count_expressions;
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Call
            || (*expression_description).expressiondescription_expressionkind == ExpressionKind::VariableArguments
        {
            let mut extra: i32 = needed + 1;
            if extra < 0 {
                extra = 0;
            }
            luak_setreturns(state, lexical_state, function_state, expression_description, extra);
        } else {
            if (*expression_description).expressiondescription_expressionkind != ExpressionKind::Void {
                luak_exp2nextreg(state, lexical_state, function_state, expression_description);
            }
            if needed > 0 {
                code_constant_nil(
                    state,
                    lexical_state,
                    function_state,
                    (*function_state).functionstate_free_register as i32,
                    needed,
                );
            }
        }
        if needed > 0 {
            luak_reserveregs(state, lexical_state, function_state, needed);
        } else {
            (*function_state).functionstate_free_register = ((*function_state).functionstate_free_register as i32 + needed) as u8;
        };
    }
}
pub unsafe fn jumpscopeerror(
    state: *mut State, lexical_state: *mut LexicalState, goto_label_description: *mut LabelDescription,
) -> ! {
    unsafe {
        let vd_name = (*getlocalvardesc(
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            (*goto_label_description).labeldescription_countactivevariables as i32,
        ))
        .variabledescription_content
        .variabledescriptioncontent_name;
        let variable_name: *const i8 = if vd_name.is_null() { c"*".as_ptr() } else { (*vd_name).get_contents_mut() };
        let mut message: *const i8 = c"<goto %s> at line %d jumps into the scope of '%s'".as_ptr();
        message = luao_pushfstring(
            state,
            message,
            &[(*(*goto_label_description).labeldescription_name).get_contents_mut().into(),
            (*goto_label_description).labeldescription_line.into(),
            variable_name.into()],
        );
        luak_semerror(state, lexical_state, message);
    }
}
pub unsafe fn solvegoto(
    state: *mut State, lexical_state: *mut LexicalState, goto_offset: usize, label_description: *mut LabelDescription,
) {
    unsafe {
        let goto_label_list: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto;
        let goto_label_description = &mut *((*goto_label_list).vectort_pointer).add(goto_offset);
        if (goto_label_description.labeldescription_countactivevariables as i32)
            < (*label_description).labeldescription_countactivevariables as i32
        {
            jumpscopeerror(state, lexical_state, goto_label_description);
        }
        luak_patchlist(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            goto_label_description.labeldescription_programcounter,
            (*label_description).labeldescription_programcounter,
        );
        let mut i = goto_offset;
        while i < (*goto_label_list).get_length() - 1 {
            *((*goto_label_list).vectort_pointer).add(i) = *((*goto_label_list).vectort_pointer).add(i + 1);
            i += 1;
        }
        (*goto_label_list).subtract_length(1);
    }
}
pub unsafe fn subexpr(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
    limit: i32,
) -> OperatorBinary {
    unsafe {
        (*(state)).increment_c_stack();
        let uop = OperatorUnary::from_token((*lexical_state).lexicalstate_token.token);
        if uop as u32 != OperatorUnary::None_ as u32 {
            let line: i32 = (*lexical_state).lexicalstate_linenumber;
            luax_next(state, lexical_state);
            subexpr(state, lexical_state, function_state, v, UNARY_PRIORITY);
            luak_prefix(state, lexical_state, (*lexical_state).lexicalstate_functionstate, uop, v, line);
        } else {
            simpleexp(state, lexical_state, function_state, v);
        }
        let mut op = OperatorBinary::from_token((*lexical_state).lexicalstate_token.token);
        while op as u32 != OperatorBinary::NoBinaryOperation as u32 && PRIORITY[op as usize].priority_left as i32 > limit {
            let mut v2: ExpressionDescription = ExpressionDescription::new();
            let line: i32 = (*lexical_state).lexicalstate_linenumber;
            luax_next(state, lexical_state);
            luak_infix(state, lexical_state, (*lexical_state).lexicalstate_functionstate, op, v);
            let nextop = subexpr(
                state, lexical_state, function_state, &mut v2, PRIORITY[op as usize].priority_right as i32,
            );
            luak_posfix(
                state,
                lexical_state,
                (*lexical_state).lexicalstate_functionstate,
                op,
                v,
                &mut v2,
                line,
            );
            op = nextop;
        }
        (*state).decrement_c_stack();
        op
    }
}
pub unsafe fn handle_block(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut block_control = BlockControl::new();
        block_control.enter_block(lexical_state, function_state, false);
        (*function_state).functionstate_blockcontrol = &mut block_control;
        parse_statement_list(state, lexical_state, function_state);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
    }
}
pub unsafe fn check_conflict(
    state: *mut State, lexical_state: *mut LexicalState, mut lhs_assign: *mut ExpressionDescription, v: *mut ExpressionDescription,
) {
    unsafe {
        let function_state: *mut FunctionState = (*lexical_state).lexicalstate_functionstate;
        let extra: i32 = (*function_state).functionstate_free_register as i32;
        let mut conflict: i32 = 0;
        while !lhs_assign.is_null() {
            if (*lhs_assign).expressiondescription_expressionkind.is_index() {
                if (*lhs_assign).expressiondescription_expressionkind == ExpressionKind::IndexUpValue {
                    if (*v).expressiondescription_expressionkind == ExpressionKind::UpValue
                        && (*lhs_assign).expressiondescription_value.value_index.valuereference_tag as i32
                            == (*v).expressiondescription_value.value_info
                    {
                        conflict = 1;
                        (*lhs_assign).expressiondescription_expressionkind = ExpressionKind::Field;
                        (*lhs_assign).expressiondescription_value.value_index.valuereference_tag = extra as u8;
                    }
                } else {
                    if (*v).expressiondescription_expressionkind == ExpressionKind::Local
                        && (*lhs_assign).expressiondescription_value.value_index.valuereference_tag as i32
                            == (*v).expressiondescription_value.value_variable.valueregister_registerindex as i32
                    {
                        conflict = 1;
                        (*lhs_assign).expressiondescription_value.value_index.valuereference_tag = extra as u8;
                    }
                    if (*lhs_assign).expressiondescription_expressionkind == ExpressionKind::Indexed
                        && (*v).expressiondescription_expressionkind == ExpressionKind::Local
                        && (*lhs_assign).expressiondescription_value.value_index.valuereference_index as i32
                            == (*v).expressiondescription_value.value_variable.valueregister_registerindex as i32
                    {
                        conflict = 1;
                        (*lhs_assign).expressiondescription_value.value_index.valuereference_index = extra as i16;
                    }
                }
            }
            lhs_assign = (*lhs_assign).expressiondescription_previous;
        }
        if conflict != 0 {
            if (*v).expressiondescription_expressionkind == ExpressionKind::Local {
                code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_MOVE,
                    extra,
                    (*v).expressiondescription_value.value_variable.valueregister_registerindex as i32,
                    0,
                    0,
                );
            } else {
                code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GET_UPVALUE,
                    extra,
                    (*v).expressiondescription_value.value_info,
                    0,
                    0,
                );
            }
            luak_reserveregs(state, lexical_state, function_state, 1);
        }
    }
}
pub unsafe fn restassign(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    lhs_assign: *mut ExpressionDescription, count_variables: i32,
) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        if !((*lhs_assign).expressiondescription_expressionkind.is_index_plus()) {
            luax_syntaxerror(state, lexical_state, c"syntax error".as_ptr());
        }
        check_readonly(state, lexical_state, &mut (*lhs_assign));
        if testnext(state, lexical_state, function_state, Character::Comma as i32) != 0 {
            let mut new_lhs_assign: ExpressionDescription = ExpressionDescription::new_with_previous(lhs_assign);
            suffixedexp(state, lexical_state, function_state, &mut new_lhs_assign);
            if !(new_lhs_assign.expressiondescription_expressionkind.is_index()) {
                check_conflict(state, lexical_state, lhs_assign, &mut new_lhs_assign);
            }
            (*(state)).increment_c_stack();
            restassign(state, lexical_state, function_state, &mut new_lhs_assign, count_variables + 1);
            (*state).decrement_c_stack();
        } else {
            checknext(state, lexical_state, function_state, Character::Equal as i32);
            let count_expressions: i32 = (*lexical_state).parse_expression_list(state, function_state, &mut expression_description);
            if count_expressions != count_variables {
                adjust_assign(
                    state, lexical_state, count_variables, count_expressions, &mut expression_description,
                );
            } else {
                luak_setoneret((*lexical_state).lexicalstate_functionstate, &mut expression_description);
                luak_storevar(
                    state,
                    lexical_state,
                    (*lexical_state).lexicalstate_functionstate,
                    &mut (*lhs_assign),
                    &mut expression_description,
                );
                return;
            }
        }
        ExpressionDescription::init_exp(
            &mut expression_description,
            ExpressionKind::Nonrelocatable,
            (*(*lexical_state).lexicalstate_functionstate).functionstate_free_register as i32 - 1,
        );
        luak_storevar(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            &mut (*lhs_assign),
            &mut expression_description,
        );
    }
}
pub unsafe fn cond(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription::new();
        (*lexical_state).parse_expression(state, function_state, &mut v);
        if v.expressiondescription_expressionkind == ExpressionKind::Nil {
            v.expressiondescription_expressionkind = ExpressionKind::False;
        }
        luak_goiftrue(state, lexical_state, (*lexical_state).lexicalstate_functionstate, &mut v);
        v.expressiondescription_f
    }
}
pub unsafe fn gotostat(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        let name: *mut TString = str_checkname(state, lexical_state, function_state);
        let label_description = find_label(
            (*lexical_state).lexicalstate_functionstate,
            (*lexical_state).lexicalstate_dynamicdata,
            name,
        );
        if label_description.is_null() {
            newgotoentry(
                state,
                lexical_state,
                function_state,
                name,
                line,
                luak_jump(state, lexical_state, function_state),
            );
        } else {
            let level: i32 = reglevel(
                lexical_state,
                function_state,
                (*label_description).labeldescription_countactivevariables as i32,
            );
            if luay_nvarstack(lexical_state, function_state) > level {
                code_abck(state, lexical_state, function_state, OPCODE_CLOSE, level, 0, 0, 0);
            }
            luak_patchlist(
                state,
                lexical_state,
                function_state,
                luak_jump(state, lexical_state, function_state),
                (*label_description).labeldescription_programcounter,
            );
        };
    }
}
pub unsafe fn breakstat(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        luax_next(state, lexical_state);
        newgotoentry(
            state,
            lexical_state,
            function_state,
            luas_newlstr(state, c"break".as_ptr(), size_of::<[i8; 6]>() - 1),
            line,
            luak_jump(state, lexical_state, function_state),
        );
    }
}
pub unsafe fn checkrepeated(state: *mut State, lexical_state: *mut LexicalState, name: *mut TString) {
    unsafe {
        let lb = find_label(
            (*lexical_state).lexicalstate_functionstate,
            (*lexical_state).lexicalstate_dynamicdata,
            name,
        );
        if !lb.is_null() {
            let mut message: *const i8 = c"label '%s' already defined on line %d".as_ptr();
            message = luao_pushfstring(state, message, &[(*name).get_contents_mut().into(), (*lb).labeldescription_line.into()]);
            luak_semerror(state, lexical_state, message);
        }
    }
}
pub unsafe fn handle_label_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, name: *mut TString, line: i32,
) {
    unsafe {
        checknext(state, lexical_state, function_state, Token::Dbcolon as i32);
        while (*lexical_state).lexicalstate_token.token == Character::Semicolon as i32
            || (*lexical_state).lexicalstate_token.token == Token::Dbcolon as i32
        {
            parse_statement(state, lexical_state, function_state);
        }
        checkrepeated(state, lexical_state, name);
        (*lexical_state).create_label(
            state,
            function_state,
            name,
            line,
            block_follow_without_until((*lexical_state).lexicalstate_token.token),
        );
    }
}
pub unsafe fn handle_while_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        let mut block_control = BlockControl::new();
        luax_next(state, lexical_state);
        let whileinit: i32 = (*function_state).code_get_label();
        let condexit: i32 = cond(state, lexical_state, function_state);
        block_control.enter_block(lexical_state, function_state, true);
        (*function_state).functionstate_blockcontrol = &mut block_control;
        checknext(state, lexical_state, function_state, Token::Do as i32);
        handle_block(state, lexical_state, function_state);
        luak_patchlist(
            state,
            lexical_state,
            function_state,
            luak_jump(state, lexical_state, function_state),
            whileinit,
        );
        check_match(
            state,
            lexical_state,
            function_state,
            Token::End as i32,
            Token::While as i32,
            line,
        );
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
        luak_patchtohere(state, lexical_state, function_state, condexit);
    }
}
pub unsafe fn handle_repeat_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        let repeat_init: i32 = (*function_state).code_get_label();
        let mut block_a = BlockControl::new();
        let mut block_b = BlockControl::new();
        block_a.enter_block(lexical_state, function_state, true);
        (*function_state).functionstate_blockcontrol = &mut block_a;
        block_b.enter_block(lexical_state, function_state, false);
        (*function_state).functionstate_blockcontrol = &mut block_b;
        luax_next(state, lexical_state);
        parse_statement_list(state, lexical_state, function_state);
        check_match(
            state,
            lexical_state,
            function_state,
            Token::Until as i32,
            Token::Repeat as i32,
            line,
        );
        let mut condexit: i32 = cond(state, lexical_state, function_state);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
        if block_b.get_count_upvalues() != 0 {
            let exit_jump: i32 = luak_jump(state, lexical_state, function_state);
            luak_patchtohere(state, lexical_state, function_state, condexit);
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_CLOSE,
                reglevel(lexical_state, function_state, block_b.get_count_active_variables() as i32),
                0,
                0,
                0,
            );
            condexit = luak_jump(state, lexical_state, function_state);
            luak_patchtohere(state, lexical_state, function_state, exit_jump);
        }
        luak_patchlist(state, lexical_state, function_state, condexit, repeat_init);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
    }
}
pub unsafe fn exp1(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description = ExpressionDescription::new();
        (*lexical_state).parse_expression(state, function_state, &mut expression_description);
        luak_exp2nextreg(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            &mut expression_description,
        );
    }
}
pub unsafe fn handle_forbody(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, base: i32, line: i32,
    count_variables: i32, isgen: i32,
) {
    unsafe {
        static FOR_PREP: [u32; 2] = [OPCODE_FORPREP, OPCODE_TFORPREP];
        static FOR_LOOP: [u32; 2] = [OPCODE_FORLOOP, OPCODE_TFORLOOP];
        let mut block_control = BlockControl::new();
        checknext(state, lexical_state, function_state, Token::Do as i32);
        let prep: i32 = luak_codeabx(state, lexical_state, function_state, FOR_PREP[isgen as usize], base, 0);
        block_control.enter_block(lexical_state, function_state, false);
        (*function_state).functionstate_blockcontrol = &mut block_control;
        adjustlocalvars(state, lexical_state, count_variables);
        luak_reserveregs(state, lexical_state, function_state, count_variables);
        handle_block(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
        fixforjump(
            state,
            lexical_state,
            function_state,
            prep,
            (*function_state).code_get_label(),
            0,
        );
        if isgen != 0 {
            code_abck(
                state, lexical_state, function_state, OPCODE_TFORCALL, base, 0, count_variables, 0,
            );
            luak_fixline(state, lexical_state, function_state, line);
        }
        let endfor: i32 = luak_codeabx(state, lexical_state, function_state, FOR_LOOP[isgen as usize], base, 0);
        fixforjump(state, lexical_state, function_state, endfor, prep + 1, 1);
        luak_fixline(state, lexical_state, function_state, line);
    }
}
pub unsafe fn handle_for_numeric(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, variable_name: *mut TString, line: i32,
) {
    unsafe {
        let base: i32 = (*function_state).functionstate_free_register as i32;
        let s = c"(for state)".as_ptr();
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        let vidx = new_localvar(state, lexical_state, variable_name);
        (*getlocalvardesc(lexical_state, function_state, vidx))
            .variabledescription_content
            .variabledescriptioncontent_kind = RDKCONST as u8;
        checknext(state, lexical_state, function_state, Character::Equal as i32);
        exp1(state, lexical_state, function_state);
        checknext(state, lexical_state, function_state, Character::Comma as i32);
        exp1(state, lexical_state, function_state);
        if testnext(state, lexical_state, function_state, Character::Comma as i32) != 0 {
            exp1(state, lexical_state, function_state);
        } else {
            code_constant_integer(
                state,
                lexical_state,
                function_state,
                (*function_state).functionstate_free_register as i32,
                1_i64,
            );
            luak_reserveregs(state, lexical_state, function_state, 1);
        }
        adjustlocalvars(state, lexical_state, 3);
        handle_forbody(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            base,
            line,
            1,
            0,
        );
    }
}
pub unsafe fn handle_for_list(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, indexname: *mut TString,
) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let mut count_variables: i32 = 4;
        let base: i32 = (*function_state).functionstate_free_register as i32;
        let s = c"(for state)".as_ptr();
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        new_localvar(state, lexical_state, luax_newstring(state, lexical_state, s, cstr_len(s)));
        let vidx = new_localvar(state, lexical_state, indexname);
        (*getlocalvardesc(lexical_state, function_state, vidx))
            .variabledescription_content
            .variabledescriptioncontent_kind = RDKCONST as u8;
        while testnext(state, lexical_state, function_state, Character::Comma as i32) != 0 {
            new_localvar(state, lexical_state, str_checkname(state, lexical_state, function_state));
            count_variables += 1;
        }
        checknext(state, lexical_state, function_state, Token::In as i32);
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        adjust_assign(
            state,
            lexical_state,
            4,
            (*lexical_state).parse_expression_list(state, function_state, &mut expression_description),
            &mut expression_description,
        );
        adjustlocalvars(state, lexical_state, 3);
        (*function_state).functionstate_free_register = luay_nvarstack(lexical_state, function_state) as u8;
        (*function_state).marktobeclosed();
        luak_checkstack(state, lexical_state, function_state, 2);
        handle_forbody(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            base,
            line,
            count_variables - 3,
            1,
        );
    }
}
pub unsafe fn handle_for_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        let mut block_control = BlockControl::new();
        block_control.enter_block(lexical_state, function_state, true);
        (*function_state).functionstate_blockcontrol = &mut block_control;
        luax_next(state, lexical_state);
        let variable_name: *mut TString = str_checkname(state, lexical_state, function_state);
        const TK_CHARACTER_EQUAL: i32 = Character::Equal as i32;
        const TK_CHARACTER_COMMA: i32 = Character::Comma as i32;
        const TK_FOR: i32 = Token::For as i32;
        const TK_IN: i32 = Token::In as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_CHARACTER_EQUAL => {
                handle_for_numeric(
                    state,
                    lexical_state,
                    (*lexical_state).lexicalstate_functionstate,
                    variable_name,
                    line,
                );
            },
            | TK_CHARACTER_COMMA | TK_IN => {
                handle_for_list(state, lexical_state, (*lexical_state).lexicalstate_functionstate, variable_name);
            },
            | _ => {
                luax_syntaxerror(state, lexical_state, c"Character::Equal or 'in' expected".as_ptr());
            },
        }
        check_match(state, lexical_state, function_state, Token::End as i32, TK_FOR, line);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
    }
}
pub unsafe fn handle_test_then_block(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, escapelist: *mut i32,
) {
    unsafe {
        let mut block_control = BlockControl::new();
        let mut v: ExpressionDescription = ExpressionDescription::new();
        let jf;
        luax_next(state, lexical_state);
        (*lexical_state).parse_expression(state, function_state, &mut v);
        let savedline = (*lexical_state).lexicalstate_lastline;
        checknext(state, lexical_state, function_state, Token::Then as i32);
        if (*lexical_state).lexicalstate_token.token == Token::Break as i32 {
            let line: i32 = (*lexical_state).lexicalstate_linenumber;
            luak_goiffalse(state, lexical_state, (*lexical_state).lexicalstate_functionstate, &mut v);
            luax_next(state, lexical_state);
            block_control.enter_block(lexical_state, function_state, false);
            (*function_state).functionstate_blockcontrol = &mut block_control;
            newgotoentry(
                state,
                lexical_state,
                function_state,
                luas_newlstr(state, c"break".as_ptr(), size_of::<[i8; 6]>() - 1),
                line,
                v.expressiondescription_t,
            );
            while testnext(state, lexical_state, function_state, Character::Semicolon as i32) != 0 {}
            if block_follow_without_until((*lexical_state).lexicalstate_token.token) {
                (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
                return;
            } else {
                jf = luak_jump(state, lexical_state, function_state);
            }
        } else {
            (*lexical_state).lexicalstate_lastline = savedline;
            luak_goiftrue(state, lexical_state, (*lexical_state).lexicalstate_functionstate, &mut v);
            block_control.enter_block(lexical_state, function_state, false);
            (*function_state).functionstate_blockcontrol = &mut block_control;
            jf = v.expressiondescription_f;
        }
        parse_statement_list(state, lexical_state, function_state);
        (*(*function_state).functionstate_blockcontrol).leave_block(state, lexical_state, function_state);
        if (*lexical_state).lexicalstate_token.token == Token::Else as i32
            || (*lexical_state).lexicalstate_token.token == Token::Elseif as i32
        {
            luak_concat(
                state,
                lexical_state,
                function_state,
                escapelist,
                luak_jump(state, lexical_state, function_state),
            );
        }
        luak_patchtohere(state, lexical_state, function_state, jf);
    }
}
pub unsafe fn handle_if_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        let mut escape_list: i32 = -1;
        handle_test_then_block(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_functionstate,
            &mut escape_list,
        );
        while (*lexical_state).lexicalstate_token.token == Token::Elseif as i32 {
            handle_test_then_block(
                state,
                lexical_state,
                (*lexical_state).lexicalstate_functionstate,
                &mut escape_list,
            );
        }
        if testnext(state, lexical_state, function_state, Token::Else as i32) != 0 {
            handle_block(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
        }
        check_match(state, lexical_state, function_state, Token::End as i32, Token::If as i32, line);
        luak_patchtohere(state, lexical_state, function_state, escape_list);
    }
}
pub unsafe fn handle_local_function(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let fvar: i32 = (*function_state).functionstate_count_active_variables as i32;
        new_localvar(state, lexical_state, str_checkname(state, lexical_state, function_state));
        adjustlocalvars(state, lexical_state, 1);
        body(
            state,
            lexical_state,
            function_state,
            &mut expression_description,
            false,
            (*lexical_state).lexicalstate_linenumber,
        );
        (*localdebuginfo(lexical_state, function_state, fvar)).localvariable_startprogramcounter =
            (*function_state).functionstate_program_counter;
    }
}
pub unsafe fn getlocalattribute(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, default_kind: i32,
) -> i32 {
    unsafe {
        if testnext(state, lexical_state, function_state, Character::AngleLeft as i32) != 0 {
            let attr: *const i8 = (*str_checkname(state, lexical_state, function_state)).get_contents_mut();
            checknext(state, lexical_state, function_state, Character::AngleRight as i32);
            if std::ffi::CStr::from_ptr(attr) == c"const" {
                return RDKCONST;
            } else if std::ffi::CStr::from_ptr(attr) == c"close" {
                return RDKTOCLOSE;
            } else {
                luak_semerror(
                    state,
                    lexical_state,
                    luao_pushfstring(state, c"unknown attribute '%s'".as_ptr(), &[attr.into()]),
                );
            }
        }
        default_kind
    }
}
pub unsafe fn handle_local_statement(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut toclose: i32 = -1;

        let mut vidx: i32;
        let mut kind: i32;
        let mut count_variables: i32 = 0;
        let count_expressions: i32;
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let defkind: i32 = getlocalattribute(state, lexical_state, function_state, 0);
        loop {
            vidx = new_localvar(state, lexical_state, str_checkname(state, lexical_state, function_state));
            kind = getlocalattribute(state, lexical_state, function_state, defkind);
            (*getlocalvardesc(lexical_state, function_state, vidx))
                .variabledescription_content
                .variabledescriptioncontent_kind = kind as u8;
            if kind == RDKTOCLOSE {
                if toclose != -1 {
                    luak_semerror(state, lexical_state, c"multiple to-be-closed variables in local list".as_ptr());
                }
                toclose = (*function_state).functionstate_count_active_variables as i32 + count_variables;
            }
            count_variables += 1;
            if testnext(state, lexical_state, function_state, Character::Comma as i32) == 0 {
                break;
            }
        }
        if testnext(state, lexical_state, function_state, Character::Equal as i32) != 0 {
            count_expressions = (*lexical_state).parse_expression_list(state, function_state, &mut expression_description);
        } else {
            expression_description.expressiondescription_expressionkind = ExpressionKind::Void;
            count_expressions = 0;
        }
        let var: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, vidx);
        if count_variables == count_expressions
            && (*var).variabledescription_content.variabledescriptioncontent_kind as i32 == RDKCONST
            && expression_description.luak_exp2const(lexical_state, function_state, &mut (*var).variabledescription_k)
        {
            (*var).variabledescription_content.variabledescriptioncontent_kind = RDKCTC as u8;
            adjustlocalvars(state, lexical_state, count_variables - 1);
            (*function_state).functionstate_count_active_variables += 1;
        } else {
            adjust_assign(
                state, lexical_state, count_variables, count_expressions, &mut expression_description,
            );
            adjustlocalvars(state, lexical_state, count_variables);
        }
        checktoclose(state, lexical_state, function_state, toclose);
    }
}
pub unsafe fn getglobalattribute(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, default_kind: i32,
) -> i32 {
    unsafe {
        let kind = getlocalattribute(state, lexical_state, function_state, default_kind);
        match kind {
            | RDKTOCLOSE => {
                luak_semerror(state, lexical_state, c"global variables cannot be to-be-closed".as_ptr());
            },
            | RDKCONST => GDKCONST,
            | _ => kind,
        }
    }
}
pub unsafe fn codecheckglobal(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, var: *mut ExpressionDescription,
    k: i32, line: i32,
) {
    unsafe {
        luak_exp2anyreg(state, lexical_state, function_state, var);
        luak_fixline(state, lexical_state, function_state, line);
        let bx = if k >= MAXARG_BX { 0 } else { (k + 1) as u32 };
        luak_codeabx(
            state,
            lexical_state,
            function_state,
            OPCODE_ERRNNIL,
            (*var).expressiondescription_value.value_info,
            bx,
        );
        luak_fixline(state, lexical_state, function_state, line);
        freeexp(lexical_state, function_state, var);
    }
}
pub unsafe fn checkglobal(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, varname: *mut TString, line: i32,
) {
    unsafe {
        let mut var: ExpressionDescription = ExpressionDescription::new();
        buildglobal(state, lexical_state, function_state, varname, &mut var);
        let k = var.expressiondescription_value.value_index.valuereference_keystr;
        codecheckglobal(state, lexical_state, function_state, &mut var, k, line);
    }
}
pub unsafe fn initglobal(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, nvars: i32, firstidx: i32, n: i32,
    line: i32,
) {
    unsafe {
        if n == nvars {
            let mut e: ExpressionDescription = ExpressionDescription::new();
            let nexps = (*lexical_state).parse_expression_list(state, function_state, &mut e);
            adjust_assign(state, lexical_state, nvars, nexps, &mut e);
        } else {
            let mut var: ExpressionDescription = ExpressionDescription::new();
            let varname = (*getlocalvardesc(lexical_state, function_state, firstidx + n))
                .variabledescription_content
                .variabledescriptioncontent_name;
            buildglobal(state, lexical_state, function_state, varname, &mut var);
            (*state).increment_c_stack();
            initglobal(state, lexical_state, function_state, nvars, firstidx, n + 1, line);
            (*state).decrement_c_stack();
            checkglobal(state, lexical_state, function_state, varname, line);
            // storevartop: store the top-of-stack value into var
            let mut e: ExpressionDescription = ExpressionDescription::new();
            ExpressionDescription::init_exp(
                &mut e,
                ExpressionKind::Nonrelocatable,
                (*function_state).functionstate_free_register as i32 - 1,
            );
            luak_storevar(state, lexical_state, function_state, &mut var, &mut e);
        }
    }
}
pub unsafe fn globalnames(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, defkind: i32) {
    unsafe {
        let mut nvars: i32 = 0;
        let mut lastidx: i32;
        loop {
            let vname = str_checkname(state, lexical_state, function_state);
            let kind = getglobalattribute(state, lexical_state, function_state, defkind);
            lastidx = new_localvar(state, lexical_state, vname);
            (*getlocalvardesc(lexical_state, function_state, lastidx))
                .variabledescription_content
                .variabledescriptioncontent_kind = kind as u8;
            nvars += 1;
            if testnext(state, lexical_state, function_state, Character::Comma as i32) == 0 {
                break;
            }
        }
        if testnext(state, lexical_state, function_state, Character::Equal as i32) != 0 {
            initglobal(
                state,
                lexical_state,
                function_state,
                nvars,
                lastidx - nvars + 1,
                0,
                (*lexical_state).lexicalstate_linenumber,
            );
        }
        (*function_state).functionstate_count_active_variables =
            ((*function_state).functionstate_count_active_variables as i32 + nvars) as usize;
    }
}
pub unsafe fn globalstat(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let defkind = getglobalattribute(state, lexical_state, function_state, GDKREG);
        if testnext(state, lexical_state, function_state, Character::Asterisk as i32) == 0 {
            globalnames(state, lexical_state, function_state, defkind);
        } else {
            // global * — collective declaration
            let idx = new_localvar(state, lexical_state, null_mut());
            (*getlocalvardesc(lexical_state, function_state, idx))
                .variabledescription_content
                .variabledescriptioncontent_kind = defkind as u8;
            (*function_state).functionstate_count_active_variables += 1;
        }
    }
}
pub unsafe fn globalfunc(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        let mut var: ExpressionDescription = ExpressionDescription::new();
        let mut b: ExpressionDescription = ExpressionDescription::new();
        let fname = str_checkname(state, lexical_state, function_state);
        // declare global variable
        let idx = new_localvar(state, lexical_state, fname);
        (*getlocalvardesc(lexical_state, function_state, idx))
            .variabledescription_content
            .variabledescriptioncontent_kind = GDKREG as u8;
        (*function_state).functionstate_count_active_variables += 1;
        buildglobal(state, lexical_state, function_state, fname, &mut var);
        body(
            state,
            lexical_state,
            function_state,
            &mut b,
            false,
            (*lexical_state).lexicalstate_linenumber,
        );
        checkglobal(state, lexical_state, function_state, fname, line);
        luak_storevar(state, lexical_state, function_state, &mut var, &mut b);
        luak_fixline(state, lexical_state, function_state, line);
    }
}
pub unsafe fn globalstatfunc(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        luax_next(state, lexical_state); // skip 'global'
        if testnext(state, lexical_state, function_state, Token::Function as i32) != 0 {
            globalfunc(state, lexical_state, function_state, line);
        } else {
            globalstat(state, lexical_state, function_state);
        }
    }
}
pub unsafe fn handle_function_name(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: *mut ExpressionDescription,
) -> bool {
    unsafe {
        let mut is_method = false;
        singlevar(state, lexical_state, function_state, v);
        while (*lexical_state).lexicalstate_token.token == Character::Period as i32 {
            fieldsel(state, lexical_state, function_state, v);
        }
        if (*lexical_state).lexicalstate_token.token == Character::Colon as i32 {
            is_method = true;
            fieldsel(state, lexical_state, function_state, v);
        }
        is_method
    }
}
pub unsafe fn handle_function_statement(
    state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        let mut v: ExpressionDescription = ExpressionDescription::new();
        let mut b: ExpressionDescription = ExpressionDescription::new();
        luax_next(state, lexical_state);
        let is_method = handle_function_name(state, lexical_state, function_state, &mut v);
        body(state, lexical_state, function_state, &mut b, is_method, line);
        (*lexical_state).lexicalstate_lastline = line;
        check_readonly(state, lexical_state, &mut v);
        luak_storevar(state, lexical_state, function_state, &mut v, &mut b);
        luak_fixline(state, lexical_state, function_state, line);
    }
}
pub unsafe fn handle_expression_statement(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut new_lhs_assign: ExpressionDescription = ExpressionDescription::new();
        suffixedexp(state, lexical_state, function_state, &mut new_lhs_assign);
        if (*lexical_state).lexicalstate_token.token == Character::Equal as i32
            || (*lexical_state).lexicalstate_token.token == Character::Comma as i32
        {
            new_lhs_assign.expressiondescription_previous = null_mut();
            restassign(state, lexical_state, function_state, &mut new_lhs_assign, 1);
        } else {
            if !(new_lhs_assign.expressiondescription_expressionkind == ExpressionKind::Call) {
                luax_syntaxerror(state, lexical_state, c"syntax error".as_ptr());
            }
            let inst: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                .add(new_lhs_assign.expressiondescription_value.value_info as usize) as *mut u32;
            *inst = *inst & !(MASK_C << POSITION_C) | 1_u32 << POSITION_C & MASK_C << POSITION_C;
        };
    }
}
pub unsafe fn handle_return_statement(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut expression_description: ExpressionDescription = ExpressionDescription::new();
        let mut nret: i32;
        let mut first: i32 = luay_nvarstack(lexical_state, function_state);
        if block_follow_with_until((*lexical_state).lexicalstate_token.token)
            || (*lexical_state).lexicalstate_token.token == Character::Semicolon as i32
        {
            nret = 0;
        } else {
            nret = (*lexical_state).parse_expression_list(state, function_state, &mut expression_description);
            if expression_description.expressiondescription_expressionkind == ExpressionKind::Call
                || expression_description.expressiondescription_expressionkind == ExpressionKind::VariableArguments
            {
                luak_setreturns(state, lexical_state, function_state, &mut expression_description, -1);
                if expression_description.expressiondescription_expressionkind == ExpressionKind::Call
                    && nret == 1
                    && !(*(*function_state).functionstate_blockcontrol).is_inside_tbc()
                {
                    *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                        .add(expression_description.expressiondescription_value.value_info as usize) =
                        *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                            .add(expression_description.expressiondescription_value.value_info as usize)
                            & !(MASK_OP)
                            | OPCODE_TAILCALL & MASK_OP;
                }
                nret = -1;
            } else if nret == 1 {
                first = luak_exp2anyreg(state, lexical_state, function_state, &mut expression_description) as i32;
            } else {
                luak_exp2nextreg(state, lexical_state, function_state, &mut expression_description);
            }
        }
        luak_ret(state, lexical_state, function_state, first, nret);
        testnext(state, lexical_state, function_state, Character::Semicolon as i32);
    }
}
pub unsafe fn handle_main_function(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        let mut block_control = BlockControl::new();

        open_function(
            state,
            lexical_state,
            function_state,
            (*lexical_state).lexicalstate_functionstate,
            &mut block_control,
        );
        setvararg(state, lexical_state, function_state, 0);
        let env: *mut UpValueDescription =
            allocate_upvalue_description(state, lexical_state, function_state, (*function_state).functionstate_prototype);
        (*env).upvaluedescription_isinstack = true;
        (*env).upvaluedescription_index = 0;
        (*env).upvaluedescription_kind = 0;
        (*env).upvaluedescription_name = (*lexical_state).lexicalstate_environment;
        if (*(*function_state).functionstate_prototype).get_marked() & BLACKBIT != 0
            && (*(*env).upvaluedescription_name).get_marked() & WHITEBITS != 0
        {
            Object::luac_barrier_(
                state,
                (*function_state).functionstate_prototype as *mut Object,
                (*env).upvaluedescription_name as *mut Object,
            );
        }
        luax_next(state, lexical_state);
        parse_statement_list(state, lexical_state, function_state);
        check(state, lexical_state, function_state, Token::EndOfStream as i32);
        close_function(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
    }
}
pub unsafe fn save(state: *mut State, lexical_state: *mut LexicalState, c: i32) {
    unsafe {
        let b = (*lexical_state).lexicalstate_buffer;
        if (*b).buffer_loads.get_length() as usize + 1 > (*b).buffer_loads.get_size() as usize {
            if (*b).buffer_loads.get_size() as usize
                >= (if (size_of::<usize>()) < size_of::<i64>() { !0usize } else { MAXIMUM_SIZE }) / 2
            {
                lexerror(state, lexical_state, c"lexical element too long".as_ptr(), 0);
            }
            let newsize = (*b).buffer_loads.get_size().wrapping_mul(2);
            (*b).buffer_loads.resize(state, newsize as usize);
        }
        let write_offset = (*b).buffer_loads.get_length();
        (*b).buffer_loads.set_length((*b).buffer_loads.get_length() as usize + 1);
        *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = c as i8;
    }
}
pub unsafe fn luax_token2str(state: *mut State, _lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        if token < FIRST_RESERVED {
            if Character::from(token).is_printable() {
                luao_pushfstring(state, c"'%c'".as_ptr(), &[token.into()])
            } else {
                luao_pushfstring(state, c"'<\\%d>'".as_ptr(), &[token.into()])
            }
        } else {
            let s: *const i8 = TOKENS[(token - (FIRST_RESERVED)) as usize];
            if token < Token::EndOfStream as i32 {
                luao_pushfstring(state, c"'%s'".as_ptr(), &[s.into()])
            } else {
                s
            }
        }
    }
}
pub unsafe fn text_token(state: *mut State, lexical_state: *mut LexicalState, token: i32) -> *const i8 {
    unsafe {
        const TK_INTEGER: i32 = Token::Integer as i32;
        const TK_FLOAT: i32 = Token::Float as i32;
        const TK_STRING: i32 = Token::String as i32;
        const TK_NAME: i32 = Token::Name as i32;
        match token {
            | TK_NAME | TK_STRING | TK_FLOAT | TK_INTEGER => {
                save(state, lexical_state, Character::Null as i32);
                luao_pushfstring(
                    state,
                    c"'%s'".as_ptr(),
                    &[(*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer.into()],
                )
            },
            | _ => luax_token2str(state, lexical_state, token),
        }
    }
}
pub unsafe fn lexerror(state: *mut State, lexical_state: *mut LexicalState, mut message: *const i8, token: i32) -> ! {
    unsafe {
        message = luag_addinfo(
            state,
            message,
            (*lexical_state).lexicalstate_source,
            (*lexical_state).lexicalstate_linenumber,
        );
        if token != 0 {
            luao_pushfstring(state, c"%s near %s".as_ptr(), &[message.into(), text_token(state, lexical_state, token).into()]);
        }
        luad_throw(state, Status::SyntaxError);
    }
}
pub unsafe fn luax_syntaxerror(state: *mut State, lexical_state: *mut LexicalState, message: *const i8) -> ! {
    unsafe {
        lexerror(state, lexical_state, message, (*lexical_state).lexicalstate_token.token);
    }
}
pub unsafe fn luax_newstring(state: *mut State, lexical_state: *mut LexicalState, str: *const i8, length: usize) -> *mut TString {
    unsafe {
        let mut tstring: *mut TString = luas_newlstr(state, str, length);
        let tvalue: *const TValue = luah_getstr((*lexical_state).lexicalstate_table, tstring);
        if (*tvalue).get_tagvariant().to_tag_type().is_nil() {
            let top = (*state).interpreter_top.stkidrel_pointer;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            let stv: *mut TValue = &mut (*top);
            let io: *mut TValue = stv;
            let tstring: *mut TString = tstring;
            (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            luah_set(state, (*lexical_state).lexicalstate_table, stv, stv);
            (*state).do_gc_step_if_should_step();
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        } else {
            tstring = (*(tvalue as *mut Node)).node_key.as_string().unwrap();
        }
        tstring
    }
}
pub unsafe fn inclinenumber(state: *mut State, lexical_state: *mut LexicalState) {
    unsafe {
        let old = (*lexical_state).lexicalstate_current;
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        if ((*lexical_state).lexicalstate_current == Character::LineFeed as i32
            || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32)
            && (*lexical_state).lexicalstate_current != old
        {
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        }
        (*lexical_state).lexicalstate_linenumber += 1;
        if (*lexical_state).lexicalstate_linenumber >= MAX_INT as i32 {
            lexerror(state, lexical_state, c"chunk has too many lines".as_ptr(), 0);
        }
    }
}
pub unsafe fn luax_setinput(
    state: *mut State, lexical_state: *mut LexicalState, zio: *mut ZIO, source: *mut TString, firstchar: i32,
) {
    unsafe {
        (*lexical_state).lexicalstate_token.token = 0;
        (*lexical_state).lexicalstate_current = firstchar;
        (*lexical_state).lexicalstate_lookahead.token = Token::EndOfStream as i32;
        (*lexical_state).lexicalstate_zio = zio;
        (*lexical_state).lexicalstate_functionstate = null_mut();
        (*lexical_state).lexicalstate_linenumber = 1;
        (*lexical_state).lexicalstate_lastline = 1;
        (*lexical_state).lexicalstate_source = source;
        (*lexical_state).lexicalstate_environment = luas_newlstr(state, c"_ENV".as_ptr(), size_of::<[i8; 5]>() - 1);
        // LUA_COMPAT_GLOBAL: "global" is not a reserved word
        let glbn = luas_newlstr(state, c"global".as_ptr(), size_of::<[i8; 7]>() - 1);
        (*glbn).set_extra(0);
        (*lexical_state).lexicalstate_glbn = glbn;
        (*(*lexical_state).lexicalstate_buffer)
            .buffer_loads
            .resize(state, LUA_MINBUFFER);
    }
}
pub unsafe fn check_next1(lexical_state: *mut LexicalState, ch: i32) -> bool {
    unsafe {
        if (*lexical_state).lexicalstate_current == ch {
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            true
        } else {
            false
        }
    }
}
pub unsafe fn check_next2(state: *mut State, lexical_state: *mut LexicalState, set: *const i8) -> bool {
    unsafe {
        if (*lexical_state).lexicalstate_current == *set.add(0) as i32
            || (*lexical_state).lexicalstate_current == *set.add(1) as i32
        {
            save(state, lexical_state, (*lexical_state).lexicalstate_current);
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            true
        } else {
            false
        }
    }
}
pub unsafe fn read_numeral(state: *mut State, lexical_state: *mut LexicalState, semantic_info: *mut Value) -> i32 {
    unsafe {
        let mut obj: TValue = TValue::new(TagVariant::NilNil);
        let mut expo: *const i8 = c"Ee".as_ptr();
        let first: i32 = (*lexical_state).lexicalstate_current;
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        if first == Character::Digit0 as i32 && check_next2(state, lexical_state, c"xX".as_ptr()) {
            expo = c"Pp".as_ptr();
        }
        loop {
            if check_next2(state, lexical_state, expo) {
                check_next2(state, lexical_state, c"-+".as_ptr());
            } else {
                if !(Character::from((*lexical_state).lexicalstate_current).is_digit_hexadecimal()
                    || (*lexical_state).lexicalstate_current == Character::Period as i32)
                {
                    break;
                }
                save(state, lexical_state, (*lexical_state).lexicalstate_current);
                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            }
        }
        if Character::from((*lexical_state).lexicalstate_current).is_identifier() {
            save(state, lexical_state, (*lexical_state).lexicalstate_current);
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        }
        save(state, lexical_state, Character::Null as i32);
        if luao_str2num((*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer, &mut obj) == 0 {
            lexerror(state, lexical_state, c"malformed number".as_ptr(), Token::Float as i32);
        }
        if let Some(i) = obj.as_integer() {
            (*semantic_info).value_integer = i;
            Token::Integer as i32
        } else {
            (*semantic_info).value_number = obj.as_number().unwrap();
            Token::Float as i32
        }
    }
}
pub unsafe fn skip_sep(state: *mut State, lexical_state: *mut LexicalState) -> usize {
    unsafe {
        let mut count: usize = 0;
        let s: i32 = (*lexical_state).lexicalstate_current;
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        while (*lexical_state).lexicalstate_current == Character::Equal as i32 {
            save(state, lexical_state, (*lexical_state).lexicalstate_current);
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            count += 1;
        }
        if (*lexical_state).lexicalstate_current == s {
            count + 2
        } else {
            (if count == 0 { 1 } else { 0 }) as usize
        }
    }
}
pub unsafe fn read_long_string(state: *mut State, lexical_state: *mut LexicalState, semantic_info: *mut Value, sep: usize) {
    unsafe {
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        if (*lexical_state).lexicalstate_current == Character::LineFeed as i32
            || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32
        {
            inclinenumber(state, lexical_state);
        }
        loop {
            match Character::from_negative((*lexical_state).lexicalstate_current) {
                | None => {
                    let what: *const i8 = if !semantic_info.is_null() { c"string".as_ptr() } else { c"comment".as_ptr() };
                    let message: *const i8 =
                        luao_pushfstring(state, c"unfinished long %s (starting at line %d)".as_ptr(), &[what.into(), line.into()]);
                    lexerror(state, lexical_state, message, Token::EndOfStream as i32);
                },
                | Some(Character::BracketRight) => {
                    if skip_sep(state, lexical_state) != sep {
                        continue;
                    }
                    save(state, lexical_state, (*lexical_state).lexicalstate_current);
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    break;
                },
                | Some(Character::LineFeed) | Some(Character::CarriageReturn) => {
                    save(state, lexical_state, Character::LineFeed as i32);
                    inclinenumber(state, lexical_state);
                    if semantic_info.is_null() {
                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.zero_length();
                    }
                },
                | _ => {
                    if !semantic_info.is_null() {
                        save(state, lexical_state, (*lexical_state).lexicalstate_current);
                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    } else {
                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    }
                },
            }
        }
        if !semantic_info.is_null() {
            (*semantic_info).value_tstring = luax_newstring(
                state,
                lexical_state,
                ((*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer).add(sep),
                ((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize)
                    .wrapping_sub(2_usize.wrapping_mul(sep)),
            );
        }
    }
}
pub unsafe fn esccheck(state: *mut State, lexical_state: *mut LexicalState, condition: bool, message: *const i8) {
    unsafe {
        if !condition {
            if (*lexical_state).lexicalstate_current != -1 {
                save(state, lexical_state, (*lexical_state).lexicalstate_current);
                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            }
            lexerror(state, lexical_state, message, Token::String as i32);
        }
    }
}
pub unsafe fn gethexa(state: *mut State, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        esccheck(
            state,
            lexical_state,
            Character::from((*lexical_state).lexicalstate_current).is_digit_hexadecimal(),
            c"hexadecimal digit expected".as_ptr(),
        );
        Character::from((*lexical_state).lexicalstate_current).get_hexadecimal_digit_value() as i32
    }
}
pub unsafe fn readhexaesc(state: *mut State, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut r: i32 = gethexa(state, lexical_state);
        r = (r << 4) + gethexa(state, lexical_state);
        (*(*lexical_state).lexicalstate_buffer)
            .buffer_loads
            .set_length((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 2);
        r
    }
}
pub unsafe fn readutf8esc(state: *mut State, lexical_state: *mut LexicalState) -> usize {
    unsafe {
        let mut i: i32 = 4;
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        esccheck(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_current == Character::BraceLeft as i32,
            c"missing Character::BraceLeft".as_ptr(),
        );
        let mut r: usize = gethexa(state, lexical_state) as usize;
        loop {
            save(state, lexical_state, (*lexical_state).lexicalstate_current);
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            if !Character::from((*lexical_state).lexicalstate_current).is_digit_hexadecimal() {
                break;
            }
            i += 1;
            esccheck(state, lexical_state, r <= (MAX_INT >> 4), c"UTF-8 value too large".as_ptr());
            r = (r << 4) + Character::from((*lexical_state).lexicalstate_current).get_hexadecimal_digit_value() as usize;
        }
        esccheck(
            state,
            lexical_state,
            (*lexical_state).lexicalstate_current == Character::BraceRight as i32,
            c"missing Character::BraceRight".as_ptr(),
        );
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        (*(*lexical_state).lexicalstate_buffer)
            .buffer_loads
            .set_length((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - i as usize);
        r
    }
}
pub unsafe fn utf8esc(state: *mut State, lexical_state: *mut LexicalState) {
    unsafe {
        let mut buffer: [i8; 8] = [0; 8];
        let mut n: i32 = luao_utf8esc(buffer.as_mut_ptr(), readutf8esc(state, lexical_state));
        while n > 0 {
            save(state, lexical_state, buffer[(8 - n) as usize] as i32);
            n -= 1;
        }
    }
}
const MAX_DEC_ESC_DIGITS: i32 = 3;
const DECIMAL_RADIX: i32 = 10;
pub unsafe fn readdecesc(state: *mut State, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        while i < MAX_DEC_ESC_DIGITS && Character::from((*lexical_state).lexicalstate_current).is_digit_decimal() {
            r = DECIMAL_RADIX * r + (*lexical_state).lexicalstate_current - Character::Digit0 as i32;
            save(state, lexical_state, (*lexical_state).lexicalstate_current);
            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
            i += 1;
        }
        esccheck(state, lexical_state, r <= u8::MAX as i32, c"decimal escape too large".as_ptr());
        (*(*lexical_state).lexicalstate_buffer)
            .buffer_loads
            .set_length((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - i as usize);
        r
    }
}
pub unsafe fn read_string(state: *mut State, lexical_state: *mut LexicalState, del: i32, semantic_info: *mut Value) {
    unsafe {
        const ESCAPE_ADVANCE: usize = 0;
        const ESCAPE_ALREADY_CONSUMED: usize = 1;
        let mut escape_action: usize;
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        while (*lexical_state).lexicalstate_current != del {
            match Character::from_negative((*lexical_state).lexicalstate_current) {
                | None => {
                    lexerror(state, lexical_state, c"unfinished string".as_ptr(), Token::EndOfStream as i32);
                },
                | Some(Character::LineFeed) | Some(Character::CarriageReturn) => {
                    lexerror(state, lexical_state, c"unfinished string".as_ptr(), Token::String as i32);
                },
                | Some(Character::Backslash) => {
                    let c: i32;
                    save(state, lexical_state, (*lexical_state).lexicalstate_current);
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    match Character::from_negative((*lexical_state).lexicalstate_current) {
                        | Some(Character::LowerA) => {
                            c = Character::Bell as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerB) => {
                            c = Character::Backspace as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerF) => {
                            c = Character::FormFeed as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerN) => {
                            c = Character::LineFeed as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerR) => {
                            c = Character::CarriageReturn as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerT) => {
                            c = Character::HorizontalTab as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerV) => {
                            c = Character::VerticalTab as i32;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerX) => {
                            c = readhexaesc(state, lexical_state);
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | Some(Character::LowerU) => {
                            utf8esc(state, lexical_state);
                            continue;
                        },
                        | Some(Character::CarriageReturn) | Some(Character::LineFeed) => {
                            inclinenumber(state, lexical_state);
                            c = Character::LineFeed as i32;
                            escape_action = ESCAPE_ALREADY_CONSUMED;
                        },
                        | Some(Character::Backslash) | Some(Character::DoubleQuote) | Some(Character::Quote) => {
                            c = (*lexical_state).lexicalstate_current;
                            escape_action = ESCAPE_ADVANCE;
                        },
                        | None | Some(Character::Null) => {
                            continue;
                        },
                        | Some(Character::LowerZ) => {
                            (*(*lexical_state).lexicalstate_buffer)
                                .buffer_loads
                                .set_length((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1);
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            while Character::from((*lexical_state).lexicalstate_current).is_whitespace() {
                                if (*lexical_state).lexicalstate_current == Character::LineFeed as i32
                                    || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32
                                {
                                    inclinenumber(state, lexical_state);
                                } else {
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                }
                            }
                            continue;
                        },
                        | _ => {
                            esccheck(
                                state,
                                lexical_state,
                                Character::from((*lexical_state).lexicalstate_current).is_digit_decimal(),
                                c"invalid escape sequence".as_ptr(),
                            );
                            c = readdecesc(state, lexical_state);
                            escape_action = ESCAPE_ALREADY_CONSUMED;
                        },
                    }
                    match escape_action {
                        | ESCAPE_ADVANCE => {
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                        },
                        | _ => {},
                    }
                    (*(*lexical_state).lexicalstate_buffer)
                        .buffer_loads
                        .set_length((*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1);
                    save(state, lexical_state, c);
                },
                | _ => {
                    save(state, lexical_state, (*lexical_state).lexicalstate_current);
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                },
            }
        }
        save(state, lexical_state, (*lexical_state).lexicalstate_current);
        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
        (*semantic_info).value_tstring = luax_newstring(
            state,
            lexical_state,
            ((*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer).add(1),
            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 2,
        );
    }
}
pub unsafe fn llex(state: *mut State, lexical_state: *mut LexicalState, semantic_info: *mut Value) -> i32 {
    unsafe {
        (*(*lexical_state).lexicalstate_buffer).buffer_loads.zero_length();
        // fstring state machine — handle synthetic tokens before normal lexing
        if (*lexical_state).fstring_phase != FSTRING_NONE {
            loop {
                match (*lexical_state).fstring_phase {
                    | FSTRING_OPEN_PAREN => {
                        (*lexical_state).fstring_phase = if (*lexical_state).fstring_delimiter == FSTRING_LONG {
                            FSTRING_SCAN_LONG_LITERAL
                        } else {
                            FSTRING_SCAN_LITERAL
                        };
                        return Character::ParenthesisLeft as i32;
                    },
                    | FSTRING_SCAN_LITERAL => {
                        // scan literal chars from ZIO until { or delimiter
                        let del = (*lexical_state).fstring_delimiter;
                        let mut has_chars = false;
                        // save opening quote position for the string token
                        save(state, lexical_state, del);
                        while (*lexical_state).lexicalstate_current != del {
                            match Character::from_negative((*lexical_state).lexicalstate_current) {
                                | None => {
                                    lexerror(state, lexical_state, c"unfinished string".as_ptr(), Token::EndOfStream as i32);
                                },
                                | Some(Character::LineFeed) | Some(Character::CarriageReturn) => {
                                    lexerror(state, lexical_state, c"unfinished string".as_ptr(), Token::String as i32);
                                },
                                | _ => {},
                            }
                            if (*lexical_state).lexicalstate_current == Character::BraceLeft as i32 {
                                // peek at next char for {{ escape
                                let next = (*(*lexical_state).lexicalstate_zio).peek_char();
                                if next == Character::BraceLeft as i32 {
                                    // {{ → literal {
                                    save(state, lexical_state, Character::BraceLeft as i32);
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    has_chars = true;
                                    continue;
                                }
                                // start of expression — stop scanning literal
                                break;
                            }
                            if (*lexical_state).lexicalstate_current == Character::BraceRight as i32 {
                                let next = (*(*lexical_state).lexicalstate_zio).peek_char();
                                if next == Character::BraceRight as i32 {
                                    // }} → literal }
                                    save(state, lexical_state, Character::BraceRight as i32);
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    has_chars = true;
                                    continue;
                                }
                                lexerror(state, lexical_state, c"single '}' in f-string literal (use '}}')".as_ptr(), Token::String as i32);
                            }
                            // handle backslash escapes in literal parts
                            if (*lexical_state).lexicalstate_current == Character::Backslash as i32 {
                                // let read_string handle escape by re-saving
                                save(state, lexical_state, (*lexical_state).lexicalstate_current);
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                match Character::from_negative((*lexical_state).lexicalstate_current) {
                                    | Some(Character::LowerN) => {
                                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1,
                                        );
                                        save(state, lexical_state, Character::LineFeed as i32);
                                    },
                                    | Some(Character::LowerT) => {
                                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1,
                                        );
                                        save(state, lexical_state, Character::HorizontalTab as i32);
                                    },
                                    | Some(Character::Backslash) => {
                                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1,
                                        );
                                        save(state, lexical_state, Character::Backslash as i32);
                                    },
                                    | Some(Character::DoubleQuote) | Some(Character::Quote) => {
                                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 1,
                                        );
                                        save(state, lexical_state, (*lexical_state).lexicalstate_current);
                                    },
                                    | _ => {
                                        // keep backslash + char as-is for other escapes
                                        save(state, lexical_state, (*lexical_state).lexicalstate_current);
                                    },
                                }
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                has_chars = true;
                                continue;
                            }
                            save(state, lexical_state, (*lexical_state).lexicalstate_current);
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            has_chars = true;
                        }
                        if (*lexical_state).lexicalstate_current == del {
                            // end of fstring
                            if has_chars {
                                // emit the literal string, then close paren next
                                save(state, lexical_state, del);
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                (*semantic_info).value_tstring = luax_newstring(
                                    state,
                                    lexical_state,
                                    ((*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer).add(1),
                                    (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 2,
                                );
                                (*lexical_state).fstring_phase = FSTRING_CLOSE_PAREN;
                                return Token::String as i32;
                            } else {
                                // no more content, just close
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                (*lexical_state).fstring_phase = if (*lexical_state).fstring_delimiter == Character::Grave as i32 {
                                    FSTRING_BACKTICK_CALL_CLOSE
                                } else {
                                    FSTRING_NONE
                                };
                                return Character::ParenthesisRight as i32;
                            }
                        } else {
                            // we stopped at { — start of expression
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume {
                            if has_chars {
                                // emit literal, then concat, then expression
                                save(state, lexical_state, del);
                                (*semantic_info).value_tstring = luax_newstring(
                                    state,
                                    lexical_state,
                                    ((*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer).add(1),
                                    (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize - 2,
                                );
                                (*lexical_state).fstring_phase = FSTRING_CONCAT;
                                (*lexical_state).fstring_brace_depth = 0;
                                return Token::String as i32;
                            } else {
                                // no literal before { — emit ( for expression grouping
                                (*lexical_state).fstring_phase = FSTRING_EXPR;
                                (*lexical_state).fstring_brace_depth = 0;
                                return Character::ParenthesisLeft as i32;
                            }
                        }
                    },
                    | FSTRING_CONCAT => {
                        (*lexical_state).fstring_phase = FSTRING_EXPR_OPEN;
                        return Token::Concatenate as i32;
                    },
                    | FSTRING_EXPR => {
                        // check for end of expression: } at depth 0
                        // skip whitespace first
                        loop {
                            match Character::from_negative((*lexical_state).lexicalstate_current) {
                                | Some(Character::Space) | Some(Character::FormFeed)
                                | Some(Character::HorizontalTab) | Some(Character::VerticalTab) => {
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                },
                                | Some(Character::LineFeed) | Some(Character::CarriageReturn) => {
                                    inclinenumber(state, lexical_state);
                                },
                                | _ => break,
                            }
                        }
                        if (*lexical_state).lexicalstate_current == Character::BraceRight as i32
                            && (*lexical_state).fstring_brace_depth == 0
                        {
                            // end of expression — consume } and transition to close
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            (*lexical_state).fstring_phase = FSTRING_EXPR_CLOSE;
                            continue; // re-enter state machine to emit )
                        }
                        // track brace nesting for expressions like {t[k]}
                        // we'll check after normal lex returns
                        break; // fall through to normal lexing
                    },
                    | FSTRING_CLOSE_PAREN => {
                        (*lexical_state).fstring_phase = if (*lexical_state).fstring_delimiter == Character::Grave as i32 {
                            FSTRING_BACKTICK_CALL_CLOSE
                        } else {
                            FSTRING_NONE
                        };
                        return Character::ParenthesisRight as i32;
                    },
                    | FSTRING_EXPR_OPEN => {
                        (*lexical_state).fstring_phase = FSTRING_EXPR;
                        return Character::ParenthesisLeft as i32;
                    },
                    | FSTRING_EXPR_CLOSE => {
                        // emit ) for expression paren
                        let at_end = if (*lexical_state).fstring_delimiter != FSTRING_LONG {
                            // short fstring: check for closing quote
                            if (*lexical_state).lexicalstate_current == (*lexical_state).fstring_delimiter {
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                true
                            } else { false }
                        } else {
                            // long fstring: check for ]] with matching sep
                            if (*lexical_state).lexicalstate_current == Character::BracketRight as i32 {
                                let saved_len = (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length();
                                let sep = skip_sep(state, lexical_state);
                                if sep == (*lexical_state).fstring_long_sep {
                                    // consume closing bracket
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    true
                                } else {
                                    // not the right separator, restore buffer
                                    (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(saved_len as usize);
                                    false
                                }
                            } else { false }
                        };
                        if at_end {
                            (*lexical_state).fstring_phase = FSTRING_CLOSE_PAREN;
                        } else {
                            (*lexical_state).fstring_phase = FSTRING_CONCAT_AFTER_EXPR;
                        }
                        return Character::ParenthesisRight as i32;
                    },
                    | FSTRING_CONCAT_AFTER_EXPR => {
                        (*lexical_state).fstring_phase = if (*lexical_state).fstring_delimiter == FSTRING_LONG {
                            FSTRING_SCAN_LONG_LITERAL
                        } else {
                            FSTRING_SCAN_LITERAL
                        };
                        return Token::Concatenate as i32;
                    },
                    | FSTRING_SCAN_LONG_LITERAL => {
                        let sep = (*lexical_state).fstring_long_sep;
                        let mut has_chars = false;
                        loop {
                            match Character::from_negative((*lexical_state).lexicalstate_current) {
                                | None => {
                                    lexerror(state, lexical_state, c"unfinished f-string".as_ptr(), Token::EndOfStream as i32);
                                },
                                | _ => {},
                            }
                            // check for { expression start
                            if (*lexical_state).lexicalstate_current == Character::BraceLeft as i32 {
                                let next = (*(*lexical_state).lexicalstate_zio).peek_char();
                                if next == Character::BraceLeft as i32 {
                                    // {{ → literal {
                                    save(state, lexical_state, Character::BraceLeft as i32);
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    has_chars = true;
                                    continue;
                                }
                                break; // start of expression
                            }
                            // check for }} escape
                            if (*lexical_state).lexicalstate_current == Character::BraceRight as i32 {
                                let next = (*(*lexical_state).lexicalstate_zio).peek_char();
                                if next == Character::BraceRight as i32 {
                                    save(state, lexical_state, Character::BraceRight as i32);
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    has_chars = true;
                                    continue;
                                }
                                lexerror(state, lexical_state, c"single '}' in f-string literal (use '}}')".as_ptr(), Token::String as i32);
                            }
                            // check for ]] closing long string
                            if (*lexical_state).lexicalstate_current == Character::BracketRight as i32 {
                                let saved_len = (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length();
                                let found_sep = skip_sep(state, lexical_state);
                                if found_sep == sep {
                                    // end of long fstring
                                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                    // restore buffer (skip_sep saved chars)
                                    (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(saved_len as usize);
                                    if has_chars {
                                        (*semantic_info).value_tstring = luax_newstring(
                                            state,
                                            lexical_state,
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer,
                                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize,
                                        );
                                        (*lexical_state).fstring_phase = FSTRING_CLOSE_PAREN;
                                        return Token::String as i32;
                                    } else {
                                        (*lexical_state).fstring_phase = if (*lexical_state).fstring_delimiter == Character::Grave as i32 {
                                            FSTRING_BACKTICK_CALL_CLOSE
                                        } else {
                                            FSTRING_NONE
                                        };
                                        return Character::ParenthesisRight as i32;
                                    }
                                }
                                // not matching sep — the chars saved by skip_sep are literal content
                                has_chars = true;
                                continue;
                            }
                            // newlines: normalize to \n like read_long_string
                            if (*lexical_state).lexicalstate_current == Character::LineFeed as i32
                                || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32
                            {
                                save(state, lexical_state, Character::LineFeed as i32);
                                inclinenumber(state, lexical_state);
                                has_chars = true;
                                continue;
                            }
                            // regular character
                            save(state, lexical_state, (*lexical_state).lexicalstate_current);
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            has_chars = true;
                        }
                        // stopped at { — start of expression
                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume {
                        if has_chars {
                            (*semantic_info).value_tstring = luax_newstring(
                                state,
                                lexical_state,
                                (*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer,
                                (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize,
                            );
                            (*lexical_state).fstring_phase = FSTRING_CONCAT;
                            (*lexical_state).fstring_brace_depth = 0;
                            return Token::String as i32;
                        } else {
                            // no literal before { — emit ( for expression grouping
                            (*lexical_state).fstring_phase = FSTRING_EXPR;
                            (*lexical_state).fstring_brace_depth = 0;
                            return Character::ParenthesisLeft as i32;
                        }
                    },
                    | FSTRING_BACKTICK_CALL_OPEN => {
                        // emit ( for the __ferrigno_backtick(...) call, then start fstring
                        (*lexical_state).fstring_phase = FSTRING_OPEN_PAREN;
                        return Character::ParenthesisLeft as i32;
                    },
                    | FSTRING_BACKTICK_CALL_CLOSE => {
                        // emit ) to close the __ferrigno_backtick(...) call
                        (*lexical_state).fstring_phase = FSTRING_NONE;
                        return Character::ParenthesisRight as i32;
                    },
                    | _ => break,
                }
            }
        }
        loop {
            const LONG_COMMENT_DONE: usize = 0;
            const SKIP_LINE_COMMENT: usize = 1;
            let comment_action: usize;
            match Character::from_negative((*lexical_state).lexicalstate_current) {
                | None => {
                    return Token::EndOfStream as i32;
                },
                | Some(Character::LineFeed) | Some(Character::CarriageReturn) => {
                    inclinenumber(state, lexical_state);
                },
                | Some(Character::Space)
                | Some(Character::FormFeed)
                | Some(Character::HorizontalTab)
                | Some(Character::VerticalTab) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                },
                | Some(Character::Hyphen) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if (*lexical_state).lexicalstate_current != Character::Hyphen as i32 {
                        return Character::Hyphen as i32;
                    }
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if (*lexical_state).lexicalstate_current == Character::BracketLeft as i32 {
                        let sep: usize = skip_sep(state, lexical_state);
                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.zero_length();
                        if sep >= 2_usize {
                            read_long_string(state, lexical_state, null_mut(), sep);
                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.zero_length();
                            comment_action = LONG_COMMENT_DONE;
                        } else {
                            comment_action = SKIP_LINE_COMMENT;
                        }
                    } else {
                        comment_action = SKIP_LINE_COMMENT;
                    }
                    match comment_action {
                        | LONG_COMMENT_DONE => {},
                        | _ => {
                            while !((*lexical_state).lexicalstate_current == Character::LineFeed as i32
                                || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32)
                                && (*lexical_state).lexicalstate_current != -1
                            {
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            }
                        },
                    }
                },
                | Some(Character::BracketLeft) => {
                    let sep: usize = skip_sep(state, lexical_state);
                    if sep >= 2_usize {
                        read_long_string(state, lexical_state, semantic_info, sep);
                        return Token::String as i32;
                    } else if sep == 0 {
                        lexerror(
                            state,
                            lexical_state,
                            c"invalid long string delimiter".as_ptr(),
                            Token::String as i32,
                        );
                    }
                    return Token::CharacterBracketLeft as i32;
                },
                | Some(Character::Equal) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Equal as i32) {
                        return Token::Equality as i32;
                    } else {
                        return Character::Equal as i32;
                    }
                },
                | Some(Character::AngleLeft) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Equal as i32) {
                        return Token::LessEqual as i32;
                    } else if check_next1(lexical_state, Character::AngleLeft as i32) {
                        return Token::ShiftLeft as i32;
                    } else {
                        return Token::CharacterAngleLeft as i32;
                    }
                },
                | Some(Character::AngleRight) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Equal as i32) {
                        return Token::GreaterEqual as i32;
                    } else if check_next1(lexical_state, Character::AngleRight as i32) {
                        return Token::ShiftRight as i32;
                    } else {
                        return Token::CharacterAngleRight as i32;
                    }
                },
                | Some(Character::Solidus) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Solidus as i32) {
                        return Token::IntegralDivide as i32;
                    } else {
                        return Token::CharacterSolidus as i32;
                    }
                },
                | Some(Character::Tilde) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Equal as i32) {
                        return Token::Inequality as i32;
                    } else {
                        return Character::Tilde as i32;
                    }
                },
                | Some(Character::Colon) => {
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Colon as i32) {
                        return Token::Dbcolon as i32;
                    } else {
                        return Character::Colon as i32;
                    }
                },
                | Some(Character::Quote) | Some(Character::DoubleQuote) => {
                    read_string(state, lexical_state, (*lexical_state).lexicalstate_current, semantic_info);
                    return Token::String as i32;
                },
                | Some(Character::Period) => {
                    save(state, lexical_state, (*lexical_state).lexicalstate_current);
                    (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                    if check_next1(lexical_state, Character::Period as i32) {
                        if check_next1(lexical_state, Character::Period as i32) {
                            return Token::Dots as i32;
                        } else {
                            return Token::Concatenate as i32;
                        }
                    } else if !Character::from((*lexical_state).lexicalstate_current).is_digit_decimal() {
                        return Token::CharacterPeriod as i32;
                    } else {
                        return read_numeral(state, lexical_state, semantic_info);
                    }
                },
                | Some(Character::Digit0)
                | Some(Character::Digit1)
                | Some(Character::Digit2)
                | Some(Character::Digit3)
                | Some(Character::Digit4)
                | Some(Character::Digit5)
                | Some(Character::Digit6)
                | Some(Character::Digit7)
                | Some(Character::Digit8)
                | Some(Character::Digit9) => {
                    return read_numeral(state, lexical_state, semantic_info);
                },
                | _ => {
                    // $"..." or $[[...]] f-string
                    if (*lexical_state).lexicalstate_current == Character::Dollar as i32
                        && FERRIGNO_EXTENSION_FSTRING.load(Ordering::Relaxed)
                    {
                        let next = (*(*lexical_state).lexicalstate_zio).peek_char();
                        if next == Character::Quote as i32 || next == Character::DoubleQuote as i32 {
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume $
                            let del = (*lexical_state).lexicalstate_current;
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume quote
                            if (*lexical_state).lexicalstate_current == del {
                                // $"" — empty string
                                (*semantic_info).value_tstring = luax_newstring(state, lexical_state, c"".as_ptr(), 0);
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                return Token::String as i32;
                            }
                            (*lexical_state).fstring_delimiter = del;
                            (*lexical_state).fstring_long_sep = 0;
                            (*lexical_state).fstring_phase = FSTRING_OPEN_PAREN;
                            (*lexical_state).fstring_brace_depth = 0;
                            return llex(state, lexical_state, semantic_info);
                        } else if next == Character::BracketLeft as i32 {
                            // $[[ or $[=[ long fstring
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume $
                            // now current is [, use skip_sep to determine separator
                            let sep = skip_sep(state, lexical_state);
                            if sep >= 2 {
                                // consume closing bracket of opener (skip_sep left current on it)
                                (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                // check for immediate ]] (empty long fstring)
                                if (*lexical_state).lexicalstate_current == Character::BracketRight as i32 {
                                    let saved_len = (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length();
                                    let close_sep = skip_sep(state, lexical_state);
                                    if close_sep == sep {
                                        // $[[]] — empty string
                                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                                        (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(saved_len as usize);
                                        (*semantic_info).value_tstring = luax_newstring(state, lexical_state, c"".as_ptr(), 0);
                                        return Token::String as i32;
                                    }
                                    (*(*lexical_state).lexicalstate_buffer).buffer_loads.set_length(saved_len as usize);
                                }
                                // valid long string opener — skip first newline like read_long_string
                                if (*lexical_state).lexicalstate_current == Character::LineFeed as i32
                                    || (*lexical_state).lexicalstate_current == Character::CarriageReturn as i32
                                {
                                    inclinenumber(state, lexical_state);
                                }
                                (*lexical_state).fstring_delimiter = FSTRING_LONG;
                                (*lexical_state).fstring_long_sep = sep;
                                (*lexical_state).fstring_phase = FSTRING_OPEN_PAREN;
                                (*lexical_state).fstring_brace_depth = 0;
                                (*(*lexical_state).lexicalstate_buffer).buffer_loads.zero_length();
                                return llex(state, lexical_state, semantic_info);
                            }
                            // not a valid long string, fall through
                        }
                    }
                    // `...` backtick command execution
                    if (*lexical_state).lexicalstate_current == Character::Grave as i32
                        && FERRIGNO_EXTENSION_BACKTICK.load(Ordering::Relaxed)
                    {
                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char(); // consume `
                        if (*lexical_state).lexicalstate_current == Character::Grave as i32 {
                            // `` — empty command, just return empty string
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            (*semantic_info).value_tstring = luax_newstring(state, lexical_state, c"".as_ptr(), 0);
                            return Token::String as i32;
                        }
                        // emit __ferrigno_backtick name token, then set up call + fstring
                        (*lexical_state).fstring_delimiter = Character::Grave as i32;
                        (*lexical_state).fstring_long_sep = 0;
                        (*lexical_state).fstring_phase = FSTRING_BACKTICK_CALL_OPEN;
                        (*lexical_state).fstring_brace_depth = 0;
                        // return the function name token
                        const BACKTICK_FN: &[u8] = b"__ferrigno_backtick";
                        let tstring = luax_newstring(
                            state,
                            lexical_state,
                            BACKTICK_FN.as_ptr() as *const i8,
                            BACKTICK_FN.len(),
                        );
                        (*semantic_info).value_tstring = tstring;
                        return Token::Name as i32;
                    }
                    if Character::from((*lexical_state).lexicalstate_current).is_identifier() {
                        loop {
                            save(state, lexical_state, (*lexical_state).lexicalstate_current);
                            (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                            if !Character::from((*lexical_state).lexicalstate_current).is_alphanumeric() {
                                break;
                            }
                        }
                        let tstring: *mut TString = luax_newstring(
                            state,
                            lexical_state,
                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.loads_pointer,
                            (*(*lexical_state).lexicalstate_buffer).buffer_loads.get_length() as usize,
                        );
                        (*semantic_info).value_tstring = tstring;
                        return (*tstring).somefunction();
                    } else {
                        let c: i32 = (*lexical_state).lexicalstate_current;
                        (*lexical_state).lexicalstate_current = (*(*lexical_state).lexicalstate_zio).get_char();
                        return c;
                    }
                },
            }
        }
    }
}
pub unsafe fn luax_next(state: *mut State, lexical_state: *mut LexicalState) {
    unsafe {
        (*lexical_state).lexicalstate_lastline = (*lexical_state).lexicalstate_linenumber;
        if (*lexical_state).lexicalstate_lookahead.token != Token::EndOfStream as i32 {
            (*lexical_state).lexicalstate_token = (*lexical_state).lexicalstate_lookahead;
            (*lexical_state).lexicalstate_lookahead.token = Token::EndOfStream as i32;
        } else {
            (*lexical_state).lexicalstate_token.token =
                llex(state, lexical_state, &mut (*lexical_state).lexicalstate_token.semantic_info);
        };
        // track brace nesting inside fstring expressions
        if (*lexical_state).fstring_phase == FSTRING_EXPR {
            if (*lexical_state).lexicalstate_token.token == Character::BraceLeft as i32 {
                (*lexical_state).fstring_brace_depth += 1;
            } else if (*lexical_state).lexicalstate_token.token == Character::BraceRight as i32 {
                (*lexical_state).fstring_brace_depth -= 1;
            }
        }
    }
}
pub unsafe fn luax_lookahead(state: *mut State, lexical_state: *mut LexicalState) -> i32 {
    unsafe {
        (*lexical_state).lexicalstate_lookahead.token =
            llex(state, lexical_state, &mut (*lexical_state).lexicalstate_lookahead.semantic_info);
        (*lexical_state).lexicalstate_lookahead.token
    }
}
pub unsafe fn luak_semerror(state: *mut State, lexical_state: *mut LexicalState, message: *const i8) -> ! {
    unsafe {
        (*lexical_state).lexicalstate_token.token = 0;
        (*lexical_state).lexicalstate_linenumber = (*lexical_state).lexicalstate_lastline;
        luax_syntaxerror(state, lexical_state, message);
    }
}
pub fn block_follow_without_until(token: i32) -> bool {
    const TK_ENDOFSTREAM: i32 = Token::EndOfStream as i32;
    const TK_END: i32 = Token::End as i32;
    const TK_ELSEIF: i32 = Token::Elseif as i32;
    const TK_ELSE: i32 = Token::Else as i32;
    const TK_BRACERIGHT: i32 = Character::BraceRight as i32;
    match token {
        | TK_ELSE | TK_ELSEIF | TK_END | TK_ENDOFSTREAM => true,
        | TK_BRACERIGHT if FERRIGNO_EXTENSION_BRACE.load(Ordering::Relaxed) => true,
        | _ => false,
    }
}
pub fn block_follow_with_until(token: i32) -> bool {
    const TK_ENDOFSTREAM: i32 = Token::EndOfStream as i32;
    const TK_END: i32 = Token::End as i32;
    const TK_UNTIL: i32 = Token::Until as i32;
    const TK_ELSEIF: i32 = Token::Elseif as i32;
    const TK_ELSE: i32 = Token::Else as i32;
    const TK_BRACERIGHT: i32 = Character::BraceRight as i32;
    match token {
        | TK_ELSE | TK_ELSEIF | TK_END | TK_ENDOFSTREAM | TK_UNTIL => true,
        | TK_BRACERIGHT if FERRIGNO_EXTENSION_BRACE.load(Ordering::Relaxed) => true,
        | _ => false,
    }
}
pub unsafe fn parse_statement(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
    unsafe {
        pub const TK_WHILE: i32 = Token::While as i32;
        let line: i32 = (*lexical_state).lexicalstate_linenumber;
        (*state).increment_c_stack();
        const TK_CHARACTER_SEMICOLON: i32 = Character::Semicolon as i32;
        const TK_DOUBLECOLON: i32 = Token::Dbcolon as i32;
        const TK_FUNCTION: i32 = Token::Function as i32;
        const TK_GLOBAL: i32 = Token::Global as i32;
        const TK_GOTO: i32 = Token::Goto as i32;
        const TK_BREAK: i32 = Token::Break as i32;
        const TK_RETURN: i32 = Token::Return as i32;
        const TK_LOCAL: i32 = Token::Local as i32;
        const TK_REPEAT: i32 = Token::Repeat as i32;
        const TK_FOR: i32 = Token::For as i32;
        const TK_DO: i32 = Token::Do as i32;
        const TK_IF: i32 = Token::If as i32;
        match (*lexical_state).lexicalstate_token.token {
            | TK_CHARACTER_SEMICOLON => {
                luax_next(state, lexical_state);
            },
            | TK_IF => {
                handle_if_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_WHILE => {
                handle_while_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_DO => {
                luax_next(state, lexical_state);
                handle_block(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
                check_match(state, lexical_state, function_state, Token::End as i32, Token::Do as i32, line);
            },
            | TK_FOR => {
                handle_for_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_REPEAT => {
                handle_repeat_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_FUNCTION => {
                handle_function_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_LOCAL => {
                luax_next(state, lexical_state);
                if testnext(state, lexical_state, function_state, TK_FUNCTION) != 0 {
                    handle_local_function(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
                } else {
                    handle_local_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
                }
            },
            | TK_GLOBAL => {
                globalstatfunc(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
            },
            | TK_DOUBLECOLON => {
                luax_next(state, lexical_state);
                handle_label_statement(
                    state,
                    lexical_state,
                    (*lexical_state).lexicalstate_functionstate,
                    str_checkname(state, lexical_state, function_state),
                    line,
                );
            },
            | TK_RETURN => {
                luax_next(state, lexical_state);
                handle_return_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
            },
            | TK_BREAK => {
                breakstat(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
            },
            | TK_GOTO => {
                luax_next(state, lexical_state);
                gotostat(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
            },
            | _ => {
                // LUA_COMPAT_GLOBAL: when "global" is not reserved, check
                // if this name token is "global" followed by '<', name, '*', or 'function'
                if (*lexical_state).lexicalstate_token.token == Token::Name as i32
                    && (*lexical_state).lexicalstate_token.semantic_info.value_object as *mut TString
                        == (*lexical_state).lexicalstate_glbn
                {
                    let lk = luax_lookahead(state, lexical_state);
                    if lk == Character::AngleLeft as i32
                        || lk == Token::Name as i32
                        || lk == Character::Asterisk as i32
                        || lk == Token::Function as i32
                    {
                        globalstatfunc(state, lexical_state, (*lexical_state).lexicalstate_functionstate, line);
                        (*state).decrement_c_stack();
                        return;
                    }
                }
                handle_expression_statement(state, lexical_state, (*lexical_state).lexicalstate_functionstate);
            },
        }
        (*(*lexical_state).lexicalstate_functionstate).functionstate_free_register =
            luay_nvarstack(lexical_state, (*lexical_state).lexicalstate_functionstate) as u8;
        (*state).decrement_c_stack();
    }
}

#![allow(
    static_mut_refs,
    unpredictable_function_pointer_comparisons,
    unsafe_code,
)]
use crate::coroutine::*;
use crate::object::*;
use crate::absolutelineinfo::*;
use crate::blockcontrol::*;
use crate::buffer::*;
use crate::tag::*;
use crate::bufffs::*;
use crate::c::*;
use crate::callinfo::*;
use crate::calls::*;
use crate::cclosure::*;
use crate::character::*;
use crate::closep::*;
use crate::closure::*;
use crate::constructorcontrol::*;
use crate::debug::*;
use crate::dumpstate::*;
use crate::dynamicdata::*;
use crate::expressiondescription::*;
use crate::f2i::*;
use crate::functions::*;
use crate::functionstate::*;
use crate::gcunion::*;
use crate::global::*;
use crate::gmatchstate::*;
use crate::header::*;
use crate::instruction::*;
use crate::k::*;
use crate::labeldescription::*;
use crate::labellist::*;
use crate::lclosure::*;
use crate::lexicalstate::*;
use crate::lg::*;
use crate::lhsassign::*;
use crate::loadf::*;
use crate::loads::*;
use crate::loadstate::*;
use crate::localvariable::*;
use crate::longjump::*;
use crate::lx::*;
use crate::matchstate::*;
use crate::math::*;
use crate::nativeendian::*;
use crate::new::*;
use crate::node::*;
use crate::operator_::*;
use crate::priority::*;
use crate::prototype::*;
use crate::rawvalue::*;
use crate::registeredfunction::*;
use crate::rn::*;
use crate::semanticinfo::*;
use crate::sparser::*;
use crate::stackvalue::*;
use crate::state::*;
use crate::stream::*;
use crate::streamwriter::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tm::*;
use crate::token::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::ubox::*;
use crate::user::*;
use crate::unary::*;
use crate::upvaldesc::*;
use crate::upvalue::*;
use crate::utility::*;
use crate::uvalue::*;
use crate::v::*;
use crate::value::*;
use crate::variabledescription::*;
use crate::zio::*;
use libc::{remove, rename, setlocale, system, tolower, toupper};
pub unsafe extern "C" fn luad_throw(state: *mut State, mut error_code: i32) -> ! { unsafe {
    if !((*state).error_jump).is_null() {
        ::core::ptr::write_volatile(&mut (*(*state).error_jump).status as *mut i32, error_code);
        _longjmp(((*(*state).error_jump).jbt).as_mut_ptr(), 1);
    } else {
        let g: *mut Global = (*state).global;
        error_code = luae_resetthread(state, error_code);
        (*state).status = error_code as u8;
        if !((*(*g).mainthread).error_jump).is_null() {
            let fresh0 = (*(*g).mainthread).top.p;
            (*(*g).mainthread).top.p = ((*(*g).mainthread).top.p).offset(1);
            let io1: *mut TValue = &mut (*fresh0).value;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            luad_throw((*g).mainthread, error_code);
        } else {
            if ((*g).panic).is_some() {
                ((*g).panic).expect("non-null function pointer")(state);
            }
            abort();
        }
    };
}}
pub unsafe extern "C" fn luad_rawrunprotected(
    state: *mut State,
    f: Pfunc,
    ud: *mut libc::c_void,
) -> i32 { unsafe {
    let old_count_c_calls: u32 = (*state).count_c_calls;
    let mut long_jump = LongJump::new ();
    ::core::ptr::write_volatile(&mut long_jump.status as *mut i32, 0);
    long_jump.previous = (*state).error_jump;
    (*state).error_jump = &mut long_jump;
    if _setjmp((long_jump.jbt).as_mut_ptr()) == 0 {
        (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(
            state, ud,
        );
    }
    (*state).error_jump = long_jump.previous;
    (*state).count_c_calls = old_count_c_calls;
    return long_jump.status;
}}
pub unsafe extern "C" fn relstack(state: *mut State) { unsafe {
    (*state).top.offset =
        ((*state).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
    (*state).tbc_list.offset =
        ((*state).tbc_list.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
    let mut up: *mut UpValue = (*state).open_upvalue;
    while !up.is_null() {
        (*up).v.offset =
            ((*up).v.p as StkId as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        up = (*up).u.open.next;
    }
    let mut call_info: *mut CallInfo = (*state).call_info;
    while !call_info.is_null() {
        (*call_info).top.offset = ((*call_info).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        (*call_info).function.offset =
            ((*call_info).function.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        call_info = (*call_info).previous;
    }
}}
pub unsafe extern "C" fn luad_errerr(state: *mut State) -> ! { unsafe {
    let message: *mut TString = luas_newlstr(
        state,
        b"error in error handling\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 24]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64),
    );
    let io: *mut TValue = &mut (*(*state).top.p).value;
    (*io).value.object = &mut (*(message as *mut GCUnion)).object;
    (*io).set_tag ((*message).get_tag());
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    luad_throw(state, 5);
}}
pub unsafe extern "C" fn luad_reallocstack(
    state: *mut State,
    new_size: i32,
    should_raise_error: bool,
) -> i32 { unsafe {
    let old_size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32;
    let oldgcstop: i32 = (*(*state).global).gcstopem as i32;
    relstack(state);
    (*(*state).global).gcstopem = 1;
    let newstack: StkId = luam_realloc_(
        state,
        (*state).stack.p as *mut libc::c_void,
        ((old_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
        ((new_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
    ) as *mut StackValue;
    (*(*state).global).gcstopem = oldgcstop as u8;
    if ((newstack == std::ptr::null_mut() as StkId) as i32 != 0) as i32 as i64 != 0 {
        (*state).correct_stack();
        if should_raise_error {
            luad_throw(state, 4);
        } else {
            return 0;
        }
    }
    (*state).stack.p = newstack;
    (*state).correct_stack();
    (*state).stack_last.p = ((*state).stack.p).offset(new_size as isize);
    let mut i: i32 = old_size + 5;
    while i < new_size + 5 {
        (*newstack.offset(i as isize)).value.set_tag(TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    return 1;
}}
pub unsafe extern "C" fn luad_growstack(
    state: *mut State,
    n: i32,
    should_raise_error: bool,
) -> i32 { unsafe {
    let size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32;
    if size > 1000000 {
        if should_raise_error {
            luad_errerr(state);
        }
        return 0;
    } else if n < 1000000 {
        let mut new_size: i32 = 2 * size;
        let needed: i32 = ((*state).top.p).offset_from((*state).stack.p) as i64 as i32 + n;
        if new_size > 1000000 {
            new_size = 1000000;
        }
        if new_size < needed {
            new_size = needed;
        }
        if new_size <= 1000000 {
            return luad_reallocstack(state, new_size, should_raise_error);
        }
    }
    luad_reallocstack(state, 1000000 + 200, should_raise_error);
    if should_raise_error {
        luag_runerror(state, b"stack overflow\0" as *const u8 as *const i8);
    }
    return 0;
}}
pub unsafe extern "C" fn stackinuse(state: *mut State) -> i32 { unsafe {
    let mut lim: StkId = (*state).top.p;
    let mut call_info: *mut CallInfo = (*state).call_info;
    while !call_info.is_null() {
        if lim < (*call_info).top.p {
            lim = (*call_info).top.p;
        }
        call_info = (*call_info).previous;
    }
    let mut res: i32 = lim.offset_from((*state).stack.p) as i64 as i32 + 1;
    if res < 20 as i32 {
        res = 20 as i32;
    }
    return res;
}}
pub unsafe extern "C" fn luad_shrinkstack(state: *mut State) { unsafe {
    let inuse: i32 = stackinuse(state);
    let max: i32 = if inuse > 1000000 / 3 {
        1000000
    } else {
        inuse * 3
    };
    if inuse <= 1000000
        && ((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32 > max
    {
        let new_size: i32 = if inuse > 1000000 / 2 {
            1000000
        } else {
            inuse * 2
        };
        luad_reallocstack(state, new_size, false);
    }
    luae_shrinkci(state);
}}
pub unsafe extern "C" fn luad_inctop(state: *mut State) { unsafe {
    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 1) as i32 != 0)
        as i32 as i64
        != 0
    {
        luad_growstack(state, 1, true);
    }
    (*state).top.p = (*state).top.p.offset(1);
}}
pub unsafe extern "C" fn luad_hook(
    state: *mut State,
    event: i32,
    line: i32,
    ftransfer: i32,
    ntransfer: i32,
) { unsafe {
    let hook: HookFunction = (*state).hook;
    if hook.is_some() && (*state).allow_hook as i32 != 0 {
        let mut mask: i32 = 1 << 3;
        let call_info: *mut CallInfo = (*state).call_info;
        let top: i64 =
            ((*state).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        let ci_top: i64 =
            ((*call_info).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        let mut ar: Debug = Debug {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = call_info;
        if ntransfer != 0 {
            mask |= 1 << 8;
            (*call_info).u2.transferinfo.ftransfer = ftransfer as u16;
            (*call_info).u2.transferinfo.ntransfer = ntransfer as u16;
        }
        if (*call_info).call_status as i32 & 1 << 1 == 0 && (*state).top.p < (*call_info).top.p {
            (*state).top.p = (*call_info).top.p;
        }
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i32 as i64) as i32
            != 0) as i32 as i64
            != 0
        {
            luad_growstack(state, 20 as i32, true);
        }
        if (*call_info).top.p < (*state).top.p.offset(20 as i32 as isize) {
            (*call_info).top.p = (*state).top.p.offset(20 as i32 as isize);
        }
        (*state).allow_hook = 0;
        (*call_info).call_status = ((*call_info).call_status as i32 | mask) as u16;
        (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(
            state, &mut ar,
        );
        (*state).allow_hook = 1;
        (*call_info).top.p = ((*state).stack.p as *mut i8).offset(ci_top as isize) as StkId;
        (*state).top.p = ((*state).stack.p as *mut i8).offset(top as isize) as StkId;
        (*call_info).call_status = ((*call_info).call_status as i32 & !mask) as u16;
    }
}}
pub unsafe extern "C" fn luad_hookcall(state: *mut State, call_info: *mut CallInfo) { unsafe {
    (*state).old_program_counter = 0;
    if (*state).hook_mask & (1 << 0) != 0 {
        let event: i32 = if ((*call_info).call_status & (1 << 5)) != 0 {
            4
        } else {
            0
        };
        let p: *mut Prototype = (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p;
        (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(1);
        (*call_info).u.l.saved_program_counter;
        luad_hook(state, event, -1, 1, (*p).count_parameters as i32);
        (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(-1);
        (*call_info).u.l.saved_program_counter;
    }
}}
pub unsafe extern "C" fn rethook(state: *mut State, mut call_info: *mut CallInfo, nres: i32) { unsafe {
    if (*state).hook_mask & 1 << 1 != 0 {
        let firstres: StkId = (*state).top.p.offset(-(nres as isize));
        let mut delta: i32 = 0;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            let p: *mut Prototype = (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p;
            if (*p).is_variable_arguments {
                delta = (*call_info).u.l.count_extra_arguments + (*p).count_parameters as i32 + 1;
            }
        }
        (*call_info).function.p = ((*call_info).function.p).offset(delta as isize);
        let ftransfer: i32 = firstres.offset_from((*call_info).function.p) as i64 as u16 as i32;
        luad_hook(state, 1, -1, ftransfer, nres);
        (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
    }
    call_info = (*call_info).previous;
    if (*call_info).call_status as i32 & 1 << 1 == 0 {
        (*state).old_program_counter = ((*call_info).u.l.saved_program_counter)
            .offset_from((*(*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p).code)
            as i64 as i32
            - 1;
    }
}}
pub unsafe extern "C" fn tryfunctm(state: *mut State, mut function: StkId) -> StkId { unsafe {
    let mut p: StkId;
    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 1) as i32 != 0)
        as i32 as i64
        != 0
    {
        let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        luad_growstack(state, 1, true);
        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
    }
    let tm: *const TValue = luat_gettmbyobj(state, &mut (*function).value, TM_CALL);
    if (*tm).get_tag_type() == TAG_TYPE_NIL {
        luag_callerror(state, &mut (*function).value);
    }
    p = (*state).top.p;
    while p > function {
        let io1: *mut TValue = &mut (*p).value;
        let io2: *const TValue = &mut (*p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        p = p.offset(-1);
    }
    (*state).top.p = (*state).top.p.offset(1);
    let io1_0: *mut TValue = &mut (*function).value;
    let io2_0: *const TValue = tm;
    (*io1_0).value = (*io2_0).value;
    (*io1_0).set_tag ((*io2_0).get_tag());
    return function;
}}
#[inline]
pub unsafe extern "C" fn moveresults(
    state: *mut State,
    mut res: StkId,
    mut nres: i32,
    mut wanted: i32,
) { unsafe {
    let firstresult: StkId;
    let mut i: i32;
    match wanted {
        0 => {
            (*state).top.p = res;
            return;
        }
        1 => {
            if nres == 0 {
                (*res).value.set_tag (TAG_VARIANT_NIL_NIL);
            } else {
                let io1: *mut TValue = &mut (*res).value;
                let io2: *const TValue = &mut (*(*state).top.p.offset(-(nres as isize))).value;
                (*io1).value = (*io2).value;
                (*io1).set_tag ((*io2).get_tag());
            }
            (*state).top.p = res.offset(1 as isize);
            return;
        }
        -1 => {
            wanted = nres;
        }
        _ => {
            if wanted < -1 {
                (*(*state).call_info).call_status =
                    ((*(*state).call_info).call_status as i32 | 1 << 9 as i32) as u16;
                (*(*state).call_info).u2.nres = nres;
                res = luaf_close(state, res, -1, 1);
                (*(*state).call_info).call_status =
                    ((*(*state).call_info).call_status as i32 & !(1 << 9 as i32)) as u16;
                if (*state).hook_mask != 0 {
                    let savedres: i64 =
                        (res as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                    rethook(state, (*state).call_info, nres);
                    res = ((*state).stack.p as *mut i8).offset(savedres as isize) as StkId;
                }
                wanted = -wanted - 3;
                if wanted == -1 {
                    wanted = nres;
                }
            }
        }
    }
    firstresult = (*state).top.p.offset(-(nres as isize));
    if nres > wanted {
        nres = wanted;
    }
    i = 0;
    while i < nres {
        let io1_0: *mut TValue = &mut (*res.offset(i as isize)).value;
        let io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).value;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag ((*io2_0).get_tag());
        i += 1;
    }
    while i < wanted {
        (*res.offset(i as isize)).value.set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    (*state).top.p = res.offset(wanted as isize);
}}
pub unsafe extern "C" fn luad_poscall(state: *mut State, call_info: *mut CallInfo, nres: i32) { unsafe {
    let wanted: i32 = (*call_info).count_results as i32;
    if (((*state).hook_mask != 0 && !(wanted < -1)) as i32 != 0) as i32 as i64 != 0 {
        rethook(state, call_info, nres);
    }
    moveresults(state, (*call_info).function.p, nres, wanted);
    (*state).call_info = (*call_info).previous;
}}
#[inline]
pub unsafe extern "C" fn prepcallinfo(
    state: *mut State,
    function: StkId,
    nret: i32,
    mask: i32,
    top: StkId,
) -> *mut CallInfo { unsafe {
    (*state).call_info = if !((*(*state).call_info).next).is_null() {
        (*(*state).call_info).next
    } else {
        luae_extendci(state)
    };
    let call_info: *mut CallInfo = (*state).call_info;
    (*call_info).function.p = function;
    (*call_info).count_results = nret as i16;
    (*call_info).call_status = mask as u16;
    (*call_info).top.p = top;
    return call_info;
}}
#[inline]
pub unsafe extern "C" fn precallc(
    state: *mut State,
    mut function: StkId,
    count_results: i32,
    f: CFunction,
) -> i32 { unsafe {
    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i32 as i64) as i32
    != 0) as i32 as i64
        != 0
    {
        let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        luad_growstack(state, 20 as i32, true);
        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
    }
    let call_info = prepcallinfo(
        state,
        function,
        count_results,
        1 << 1,
        (*state).top.p.offset(20 as i32 as isize),
    );
    (*state).call_info = call_info;
    if ((*state).hook_mask & 1 << 0 != 0) as i32 as i64 != 0 {
        let narg: i32 = ((*state).top.p).offset_from(function) as i64 as i32 - 1;
        luad_hook(state, 0, -1, 1, narg);
    }
    let n: i32 = (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(state);
    luad_poscall(state, call_info, n);
    return n;
}}
pub unsafe extern "C" fn luad_pretailcall(
    state: *mut State,
    call_info: *mut CallInfo,
    mut function: StkId,
    mut narg1: i32,
    delta: i32,
) -> i32 { unsafe {
    loop {
        match (*function).value.get_tag_variant() {
            TAG_VARIANT_CLOSURE_C => {
                return precallc(
                    state,
                    function,
                    -1,
                    (*((*function).value.value.object as *mut GCUnion)).ccl.f,
                );
            }
            TAG_VARIANT_CLOSURE_CFUNCTION => return precallc(state, function, -1, (*function).value.value.f),
            TAG_VARIANT_CLOSURE_L => {
                let p: *mut Prototype = (*((*function).value.value.object as *mut GCUnion)).lcl.p;
                let fsize: i32 = (*p).maximum_stack_size as i32;
                let nfixparams: i32 = (*p).count_parameters as i32;
                let mut i: i32;
                if ((((*state).stack_last.p).offset_from((*state).top.p) as i64
                    <= (fsize - delta) as i64) as i32
                    != 0) as i32 as i64
                    != 0
                {
                    let t__: i64 =
                        (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                    if (*(*state).global).gc_debt > 0 {
                        luac_step(state);
                    }
                    luad_growstack(state, fsize - delta, true);
                    function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
                }
                (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
                i = 0;
                while i < narg1 {
                    let io1: *mut TValue = &mut (*((*call_info).function.p).offset(i as isize)).value;
                    let io2: *const TValue = &mut (*function.offset(i as isize)).value;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag ((*io2).get_tag());
                    i += 1;
                }
                function = (*call_info).function.p;
                while narg1 <= nfixparams {
                    (*function.offset(narg1 as isize)).value.set_tag (TAG_VARIANT_NIL_NIL);
                    narg1 += 1;
                }
                (*call_info).top.p = function.offset(1 as isize).offset(fsize as isize);
                (*call_info).u.l.saved_program_counter = (*p).code;
                (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 5) as u16;
                (*state).top.p = function.offset(narg1 as isize);
                return -1;
            }
            _ => {
                function = tryfunctm(state, function);
                narg1 += 1;
            }
        }
    }
}}
pub unsafe extern "C" fn luad_precall(
    state: *mut State,
    mut function: StkId,
    count_results: i32,
) -> *mut CallInfo { unsafe {
    loop {
        match (*function).value.get_tag_variant() {
            TAG_VARIANT_CLOSURE_C => {
                precallc(
                    state,
                    function,
                    count_results,
                    (*((*function).value.value.object as *mut GCUnion)).ccl.f,
                );
                return std::ptr::null_mut();
            }
            TAG_VARIANT_CLOSURE_CFUNCTION => {
                precallc(state, function, count_results, (*function).value.value.f);
                return std::ptr::null_mut();
            }
            TAG_VARIANT_CLOSURE_L => {
                let call_info;
                let p: *mut Prototype = (*((*function).value.value.object as *mut GCUnion)).lcl.p;
                let mut narg: i32 = ((*state).top.p).offset_from(function) as i64 as i32 - 1;
                let nfixparams: i32 = (*p).count_parameters as i32;
                let fsize: i32 = (*p).maximum_stack_size as i32;
                if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= fsize as i64)
                    as i32
                    != 0) as i32 as i64
                    != 0
                {
                    let t__: i64 =
                        (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                    if (*(*state).global).gc_debt > 0 {
                        luac_step(state);
                    }
                    luad_growstack(state, fsize, true);
                    function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
                }
                call_info = prepcallinfo(
                    state,
                    function,
                    count_results,
                    0,
                    function.offset(1 as isize).offset(fsize as isize),
                );
                (*state).call_info = call_info;
                (*call_info).u.l.saved_program_counter = (*p).code;
                while narg < nfixparams {
                    let fresh1 = (*state).top.p;
                    (*state).top.p = (*state).top.p.offset(1);
                    (*fresh1).value.set_tag (TAG_VARIANT_NIL_NIL);
                    narg += 1;
                }
                return call_info;
            }
            _ => {
                function = tryfunctm(state, function);
            }
        }
    }
}}
#[inline]
pub unsafe extern "C" fn ccall(
    state: *mut State,
    mut function: StkId,
    count_results: i32,
    inc: u32,
) { unsafe {
    let call_info;
    (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_add(inc) as u32 as u32;
    if (((*state).count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32) as i32 != 0) as i32
        as i64
        != 0
    {
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 0) as i32
            != 0) as i32 as i64
            != 0
        {
            let t__: i64 =
                (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            luad_growstack(state, 0, true);
            function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
        }
        luae_checkcstack(state);
    }
    call_info = luad_precall(state, function, count_results);
    if !call_info.is_null() {
        (*call_info).call_status = (1 << 2) as u16;
        luav_execute(state, call_info);
    }
    (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_sub(inc) as u32 as u32;
}}
pub unsafe extern "C" fn luad_callnoyield(
    state: *mut State,
    function: StkId,
    count_results: i32,
) { unsafe {
    ccall(state, function, count_results, (0x10000 as i32 | 1) as u32);
}}
pub unsafe extern "C" fn finishpcallk(state: *mut State, call_info: *mut CallInfo) -> i32 { unsafe {
    let mut status: i32 = (*call_info).call_status as i32 >> 10 as i32 & 7;
    if ((status == 0) as i32 != 0) as i32 as i64 != 0 {
        status = 1;
    } else {
        let mut function: StkId =
            ((*state).stack.p as *mut i8).offset((*call_info).u2.funcidx as isize) as StkId;
        (*state).allow_hook = ((*call_info).call_status as i32 & 1 << 0) as u8;
        function = luaf_close(state, function, status, 1);
        (*state).set_error_object(status, function);
        luad_shrinkstack(state);
        (*call_info).call_status =
            ((*call_info).call_status as i32 & !((7) << 10 as i32) | 0 << 10 as i32) as u16;
    }
    (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
    (*state).error_function = (*call_info).u.c.old_error_function;
    return status;
}}
pub unsafe extern "C" fn finishccall(state: *mut State, call_info: *mut CallInfo) { unsafe {
    let n: i32;
    if (*call_info).call_status as i32 & 1 << 9 as i32 != 0 {
        n = (*call_info).u2.nres;
    } else {
        let mut status: i32 = 1;
        if (*call_info).call_status as i32 & 1 << 4 != 0 {
            status = finishpcallk(state, call_info);
        }
        if -1 <= -1 && (*(*state).call_info).top.p < (*state).top.p {
            (*(*state).call_info).top.p = (*state).top.p;
        }
        n = (Some(((*call_info).u.c.k).expect("non-null function pointer")))
            .expect("non-null function pointer")(state, status, (*call_info).u.c.ctx);
    }
    luad_poscall(state, call_info, n);
}}
pub unsafe extern "C" fn unroll(state: *mut State, mut _ud: *mut libc::c_void) { unsafe {
    let mut call_info;
    loop {
        call_info = (*state).call_info;
        if !(call_info != &mut (*state).base_callinfo as *mut CallInfo) {
            break;
        }
        if (*call_info).call_status as i32 & 1 << 1 != 0 {
            finishccall(state, call_info);
        } else {
            luav_finishop(state);
            luav_execute(state, call_info);
        }
    }
}}
pub unsafe extern "C" fn resume_error(
    state: *mut State,
    message: *const i8,
    narg: i32,
) -> i32 { unsafe {
    (*state).top.p = (*state).top.p.offset(-(narg as isize));
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut TString = luas_new(state, message);
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag ((*x_).get_tag());
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    return 2;
}}
pub unsafe extern "C" fn resume(state: *mut State, ud: *mut libc::c_void) { unsafe {
    let mut n: i32 = *(ud as *mut i32);
    let first_argument: StkId = (*state).top.p.offset(-(n as isize));
    let call_info: *mut CallInfo = (*state).call_info;
    if (*state).status as i32 == 0 {
        ccall(state, first_argument.offset(-(1 as isize)), -1, 0u32);
    } else {
        (*state).status = 0;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(-1);
            (*call_info).u.l.saved_program_counter;
            (*state).top.p = first_argument;
            luav_execute(state, call_info);
        } else {
            if ((*call_info).u.c.k).is_some() {
                n = (Some(((*call_info).u.c.k).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    state, 1, (*call_info).u.c.ctx
                );
            }
            luad_poscall(state, call_info, n);
        }
        unroll(state, std::ptr::null_mut());
    };
}}
pub unsafe extern "C" fn precover(state: *mut State, mut status: i32) -> i32 { unsafe {
    let mut call_info;
    while status > 1 && {
        call_info = (*state).find_pcall();
        !call_info.is_null()
    } {
        (*state).call_info = call_info;
        (*call_info).call_status =
            ((*call_info).call_status as i32 & !((7) << 10 as i32) | status << 10 as i32) as u16;
        status = luad_rawrunprotected(
            state,
            Some(unroll as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            std::ptr::null_mut(),
        );
    }
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_resume(
    state: *mut State,
    from: *mut State,
    mut nargs: i32,
    count_results: *mut i32,
) -> i32 { unsafe {
    let mut status;
    if (*state).status as i32 == 0 {
        if (*state).call_info != &mut (*state).base_callinfo as *mut CallInfo {
            return resume_error(
                state,
                b"cannot resume non-suspended coroutine\0" as *const u8 as *const i8,
                nargs,
            );
        } else if ((*state).top.p).offset_from(((*(*state).call_info).function.p).offset(1 as isize))
            as i64
            == nargs as i64
        {
            return resume_error(
                state,
                b"cannot resume dead coroutine\0" as *const u8 as *const i8,
                nargs,
            );
        }
    } else if (*state).status as i32 != 1 {
        return resume_error(
            state,
            b"cannot resume dead coroutine\0" as *const u8 as *const i8,
            nargs,
        );
    }
    (*state).count_c_calls = if !from.is_null() {
        (*from).count_c_calls & 0xffff as i32 as u32
    } else {
        0u32
    };
    if (*state).count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32 {
        return resume_error(
            state,
            b"C stack overflow\0" as *const u8 as *const i8,
            nargs,
        );
    }
    (*state).count_c_calls = ((*state).count_c_calls).wrapping_add(1);
    (*state).count_c_calls;
    status = luad_rawrunprotected(
        state,
        Some(resume as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
        &mut nargs as *mut i32 as *mut libc::c_void,
    );
    status = precover(state, status);
    if !((!(status > 1) as i32 != 0) as i32 as i64 != 0) {
        (*state).status = status as u8;
        (*state).set_error_object(status, (*state).top.p);
        (*(*state).call_info).top.p = (*state).top.p;
    }
    *count_results = if status == 1 {
        (*(*state).call_info).u2.nyield
    } else {
        ((*state).top.p).offset_from(((*(*state).call_info).function.p).offset(1 as isize)) as i64
            as i32
    };
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_yieldk(
    state: *mut State,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) -> i32 { unsafe {
    let call_info;
    call_info = (*state).call_info;
    if (!((*state).count_c_calls & 0xffff0000 as u32 == 0u32) as i32 != 0) as i32 as i64 != 0 {
        if state != (*(*state).global).mainthread {
            luag_runerror(
                state,
                b"attempt to yield across a C-call boundary\0" as *const u8 as *const i8,
            );
        } else {
            luag_runerror(
                state,
                b"attempt to yield from outside a coroutine\0" as *const u8 as *const i8,
            );
        }
    }
    (*state).status = 1;
    (*call_info).u2.nyield = count_results;
    if (*call_info).call_status as i32 & 1 << 1 == 0 {
    } else {
        (*call_info).u.c.k = k;
        if ((*call_info).u.c.k).is_some() {
            (*call_info).u.c.ctx = ctx;
        }
        luad_throw(state, 1);
    }
    return 0;
}}
pub unsafe extern "C" fn closepaux(state: *mut State, ud: *mut libc::c_void) { unsafe {
    let pcl: *mut CloseP = ud as *mut CloseP;
    luaf_close(state, (*pcl).level, (*pcl).status, 0);
}}
pub unsafe extern "C" fn luad_closeprotected(
    state: *mut State,
    level: i64,
    mut status: i32,
) -> i32 { unsafe {
    let old_ci: *mut CallInfo = (*state).call_info;
    let old_allowhooks: u8 = (*state).allow_hook;
    loop {
        let mut pcl = CloseP::new();
        pcl.level = ((*state).stack.p as *mut i8).offset(level as isize) as StkId;
        pcl.status = status;
        status = luad_rawrunprotected(
            state,
            Some(closepaux as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            &mut pcl as *mut CloseP as *mut libc::c_void,
        );
        if ((status == 0) as i32 != 0) as i32 as i64 != 0 {
            return pcl.status;
        } else {
            (*state).call_info = old_ci;
            (*state).allow_hook = old_allowhooks;
        }
    }
}}
pub unsafe extern "C" fn luad_pcall(
    state: *mut State,
    function: Pfunc,
    u: *mut libc::c_void,
    old_top: i64,
    ef: i64,
) -> i32 { unsafe {
    let mut status: i32;
    let old_ci: *mut CallInfo = (*state).call_info;
    let old_allowhooks: u8 = (*state).allow_hook;
    let old_error_function: i64 = (*state).error_function;
    (*state).error_function = ef;
    status = luad_rawrunprotected(state, function, u);
    if ((status != 0) as i32 != 0) as i32 as i64 != 0 {
        (*state).call_info = old_ci;
        (*state).allow_hook = old_allowhooks;
        status = luad_closeprotected(state, old_top, status);
        (*state).set_error_object(
            status,
            ((*state).stack.p as *mut i8).offset(old_top as isize) as StkId,
        );
        luad_shrinkstack(state);
    }
    (*state).error_function = old_error_function;
    return status;
}}
pub unsafe extern "C" fn checkmode(state: *mut State, mode: *const i8, x: *const i8) { unsafe {
    if !mode.is_null() && (strchr(mode, *x.offset(0) as i32)).is_null() {
        luao_pushfstring(
            state,
            b"attempt to load a %s chunk (mode is '%s')\0" as *const u8 as *const i8,
            x,
            mode,
        );
        luad_throw(state, 3);
    }
}}
pub unsafe extern "C" fn f_parser(state: *mut State, ud: *mut libc::c_void) { unsafe {
    let cl: *mut LClosure;
    let p: *mut SParser = ud as *mut SParser;
    let fresh2 = (*(*p).zio).n;
    (*(*p).zio).n = ((*(*p).zio).n).wrapping_sub(1);
    let c: i32 = if fresh2 > 0u64 {
        let fresh3 = (*(*p).zio).p;
        (*(*p).zio).p = ((*(*p).zio).p).offset(1);
        *fresh3 as u8 as i32
    } else {
        luaz_fill((*p).zio)
    };
    if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
        checkmode(state, (*p).mode, b"binary\0" as *const u8 as *const i8);
        cl = luau_undump(state, (*p).zio, (*p).name);
    } else {
        checkmode(state, (*p).mode, b"text\0" as *const u8 as *const i8);
        cl = luay_parser(
            state,
            (*p).zio,
            &mut (*p).buffer,
            &mut (*p).dynamic_data,
            (*p).name,
            c,
        );
    }
    luaf_initupvals(state, cl);
}}
pub unsafe extern "C" fn luad_protectedparser(
    state: *mut State,
    zio: *mut ZIO,
    name: *const i8,
    mode: *const i8,
) -> i32 { unsafe {
    let mut p: SParser = SParser {
        zio: std::ptr::null_mut(),
        buffer: Buffer::new(),
        dynamic_data: DynamicData {
            active_variable: DynamicDataActiveVariable {
                arr: std::ptr::null_mut(),
                n: 0,
                size: 0,
            },
            gt: Labellist {
                arr: std::ptr::null_mut(),
                n: 0,
                size: 0,
            },
            label: Labellist {
                arr: std::ptr::null_mut(),
                n: 0,
                size: 0,
            },
        },
        mode: std::ptr::null(),
        name: std::ptr::null(),
    };
    let status: i32;
    (*state).count_c_calls =
        ((*state).count_c_calls as u32).wrapping_add(0x10000 as i32 as u32) as u32 as u32;
    p.zio = zio;
    p.name = name;
    p.mode = mode;
    p.dynamic_data.active_variable.arr = std::ptr::null_mut();
    p.dynamic_data.active_variable.size = 0;
    p.dynamic_data.gt.arr = std::ptr::null_mut();
    p.dynamic_data.gt.size = 0;
    p.dynamic_data.label.arr = std::ptr::null_mut();
    p.dynamic_data.label.size = 0;
    p.buffer.pointer = std::ptr::null_mut();
    p.buffer.size = 0;
    status = luad_pcall(
        state,
        Some(f_parser as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
        &mut p as *mut SParser as *mut libc::c_void,
        ((*state).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64,
        (*state).error_function,
    );
    p.buffer.pointer = luam_saferealloc_(
        state,
        p.buffer.pointer as *mut libc::c_void,
        (p.buffer.size).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        (0u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    p.buffer.size = 0;
(*state).free_memory(
        p.dynamic_data.active_variable.arr as *mut libc::c_void,
        (p.dynamic_data.active_variable.size as u64)
            .wrapping_mul(::core::mem::size_of::<VariableDescription>() as u64),
    );
(*state).free_memory(
        p.dynamic_data.gt.arr as *mut libc::c_void,
        (p.dynamic_data.gt.size as u64)
            .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64),
    );
(*state).free_memory(
        p.dynamic_data.label.arr as *mut libc::c_void,
        (p.dynamic_data.label.size as u64)
            .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64),
    );
    (*state).count_c_calls =
        ((*state).count_c_calls as u32).wrapping_sub(0x10000 as i32 as u32) as u32 as u32;
    return status;
}}
pub unsafe extern "C" fn index2value(state: *mut State, mut index: i32) -> *mut TValue { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    if index > 0 {
        let o: StkId = ((*call_info).function.p).offset(index as isize);
        if o >= (*state).top.p {
            return &mut (*(*state).global).nilvalue;
        } else {
            return &mut (*o).value;
        }
    } else if !(index <= -(1000000 as i32) - 1000 as i32) {
        return &mut (*(*state).top.p.offset(index as isize)).value;
    } else if index == -(1000000 as i32) - 1000 as i32 {
        return &mut (*(*state).global).l_registry;
    } else {
        index = -(1000000 as i32) - 1000 as i32 - index;
        let value = (*(*call_info).function.p).value;
        if value.is_collectable() && value.get_tag_variant() == TAG_VARIANT_CLOSURE_C
        {
            let function: *mut CClosure =
                &mut (*(value.value.object as *mut GCUnion)).ccl;
            return if index <= (*function).count_upvalues as i32 {
                &mut *((*function).upvalue)
                    .as_mut_ptr()
                    .offset((index - 1) as isize) as *mut TValue
            } else {
                &mut (*(*state).global).nilvalue
            };
        } else {
            return &mut (*(*state).global).nilvalue;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn index2stack(state: *mut State, index: i32) -> StkId { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    if index > 0 {
        let o: StkId = ((*call_info).function.p).offset(index as isize);
        return o;
    } else {
        return (*state).top.p.offset(index as isize);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_checkstack(state: *mut State, n: i32) -> i32 { unsafe {
    let res: i32;
    let call_info;
    call_info = (*state).call_info;
    if ((*state).stack_last.p).offset_from((*state).top.p) as i64 > n as i64 {
        res = 1;
    } else {
        res = luad_growstack(state, n, false);
    }
    if res != 0 && (*call_info).top.p < (*state).top.p.offset(n as isize) {
        (*call_info).top.p = (*state).top.p.offset(n as isize);
    }
    return res;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_xmove(from: *mut State, to: *mut State, n: i32) { unsafe {
    let mut i: i32;
    if from == to {
        return;
    }
    (*from).top.p = ((*from).top.p).offset(-(n as isize));
    i = 0;
    while i < n {
        let io1: *mut TValue = &mut (*(*to).top.p).value;
        let io2: *const TValue = &mut (*((*from).top.p).offset(i as isize)).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*to).top.p = ((*to).top.p).offset(1);
        (*to).top.p;
        i += 1;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_atpanic(state: *mut State, panicf: CFunction) -> CFunction { unsafe {
    let old: CFunction = (*(*state).global).panic;
    (*(*state).global).panic = panicf;
    return old;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_absindex(state: *mut State, index: i32) -> i32 { unsafe {
    return if index > 0 || index <= -(1000000 as i32) - 1000 as i32 {
        index
    } else {
        ((*state).top.p).offset_from((*(*state).call_info).function.p) as i64 as i32 + index
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_settop(state: *mut State, index: i32) { unsafe {
    let call_info;
    let mut newtop;
    let mut diff;
    call_info = (*state).call_info;
    let function: StkId = (*call_info).function.p;
    if index >= 0 {
        diff = function
            .offset(1 as isize)
            .offset(index as isize)
            .offset_from((*state).top.p) as i64;
        while diff > 0 {
            let fresh4 = (*state).top.p;
            (*state).top.p = (*state).top.p.offset(1);
            (*fresh4).value.set_tag (TAG_VARIANT_NIL_NIL);
            diff -= 1;
        }
    } else {
        diff = (index + 1) as i64;
    }
    newtop = (*state).top.p.offset(diff as isize);
    if diff < 0 && (*state).tbc_list.p >= newtop {
        newtop = luaf_close(state, newtop, -1, 0);
    }
    (*state).top.p = newtop;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_closeslot(state: *mut State, index: i32) { unsafe {
    let mut level = index2stack(state, index);
    level = luaf_close(state, level, -1, 0);
    (*level).value.set_tag (TAG_VARIANT_NIL_NIL);
}}
#[inline]
pub unsafe extern "C" fn reverse(mut _state: *mut State, mut from: StkId, mut to: StkId) { unsafe {
    while from < to {
        let mut temp: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io1: *mut TValue = &mut temp;
        let io2: *const TValue = &mut (*from).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        let io1_0: *mut TValue = &mut (*from).value;
        let io2_0: *const TValue = &mut (*to).value;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        let io1_1: *mut TValue = &mut (*to).value;
        let io2_1: *const TValue = &mut temp;
        (*io1_1).value = (*io2_1).value;
        (*io1_1).set_tag((*io2_1).get_tag());
        from = from.offset(1);
        to = to.offset(-1);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rotate(state: *mut State, index: i32, n: i32) { unsafe {
    let high: StkId = (*state).top.p.offset(-(1 as isize));
    let low: StkId = index2stack(state, index);
    let middle: StkId = if n >= 0 {
        high.offset(-(n as isize))
    } else {
        low.offset(-(n as isize)).offset(-(1 as isize))
    };
    reverse(state, low, middle);
    reverse(state, middle.offset(1 as isize), high);
    reverse(state, low, high);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_copy(state: *mut State, fromidx: i32, toidx: i32) { unsafe {
    let fr: *mut TValue = index2value(state, fromidx);
    let to: *mut TValue = index2value(state, toidx);
    let io1: *mut TValue = to;
    let io2: *const TValue = fr;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    if toidx < -(1000000 as i32) - 1000 as i32 {
        if (*fr).is_collectable() {
            if (*((*(*(*state).call_info).function.p).value.value.object as *mut GCUnion))
                .ccl
                .get_marked()
                & 1 << 5
                != 0
                && (*(*fr).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    state,
                    &mut (*(&mut (*((*(*(*state).call_info).function.p).value.value.object as *mut GCUnion)).ccl
                        as *mut CClosure as *mut GCUnion))
                        .object,
                    &mut (*((*fr).value.object as *mut GCUnion)).object,
                );
            } else {
            };
        } else {
        };
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushvalue(state: *mut State, index: i32) { unsafe {
    let io1: *mut TValue = &mut (*(*state).top.p).value;
    let io2: *const TValue = index2value(state, index);
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    (*state).top.p = (*state).top.p.offset(1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_type(state: *mut State, index: i32) -> i32 { unsafe {
    let o: *const TValue = index2value(state, index);
    return if (get_tag_type((*o).get_tag()) != TAG_TYPE_NIL)
        || o != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue
    {
        (*o).get_tag_type() as i32
    } else {
        TAG_NONE_
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_typename(mut _state: *mut State, t: i32) -> *const i8 { unsafe {
    return TYPE_NAMES[(t + 1) as usize];
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_iscfunction(state: *mut State, index: i32) -> bool { unsafe {
    let o: *const TValue = index2value(state, index);
    match (*o).get_tag_variant() {
        TAG_VARIANT_CLOSURE_CFUNCTION => return true,
        TAG_VARIANT_CLOSURE_C => return true,
        _ => return false,
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_isinteger(state: *mut State, index: i32) -> bool { unsafe {
    return (*index2value(state, index)).get_tag() == TAG_VARIANT_NUMERIC_INTEGER;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_isnumber(state: *mut State, index: i32) -> bool { unsafe {
    let o: *const TValue = index2value(state, index);
    return if (*o).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
        true
    } else {
        let mut n: f64 = 0.0;
        luav_tonumber_(o, &mut n)
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_isstring(state: *mut State, index: i32) -> bool { unsafe {
    let o: *const TValue = index2value(state, index);
    return match get_tag_type((*o).get_tag()) {
        TAG_TYPE_NUMERIC => true,
        TAG_TYPE_STRING => true,
        _ => false,
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawequal(
    state: *mut State,
    index1: i32,
    index2: i32,
) -> bool { unsafe {
    let o1: *const TValue = index2value(state, index1);
    let o2: *const TValue = index2value(state, index2);
    return if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
        || o1 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
        && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
            || o2 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
    {
        0 != luav_equalobj(std::ptr::null_mut(), o1, o2)
    } else {
        false
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_arith(state: *mut State, op: i32) { unsafe {
    if !(op != 12 as i32 && op != 13 as i32) {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
        }
    luao_arith(
        state,
        op,
        &mut (*(*state).top.p.offset(-(2 as isize))).value,
        &mut (*(*state).top.p.offset(-(1 as isize))).value,
        (*state).top.p.offset(-(2 as isize)),
    );
    (*state).top.p = (*state).top.p.offset(-1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_compare(
    state: *mut State,
    index1: i32,
    index2: i32,
    op: i32,
) -> i32 { unsafe {
    let o1: *const TValue = index2value(state, index1);
    let o2: *const TValue = index2value(state, index2);
    let mut i: i32 = 0;
    if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
        || o1 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
        && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
            || o2 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
    {
        match op {
            0 => {
                i = luav_equalobj(state, o1, o2);
            }
            1 => {
                i = luav_lessthan(state, o1, o2);
            }
            2 => {
                i = luav_lessequal(state, o1, o2);
            }
            _ => {}
        }
    }
    return i;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_stringtonumber(state: *mut State, s: *const i8) -> u64 { unsafe {
    let size: u64 = luao_str2num(s, &mut (*(*state).top.p).value);
    if size != 0u64 {
        (*state).top.p = (*state).top.p.offset(1);
        }
    return size;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_tonumberx(
    state: *mut State,
    index: i32,
    is_number: *mut bool,
) -> f64 { unsafe {
    let mut n: f64 = 0.0;
    let o: *const TValue = index2value(state, index);
    let is_number_: bool = if (*o).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
        n = (*o).value.n;
        true
    } else {
        luav_tonumber_(o, &mut n)
    };
    if !is_number.is_null() {
        *is_number = is_number_;
    }
    return n;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_tointegerx(
    state: *mut State,
    index: i32,
    is_number: *mut bool,
) -> i64 { unsafe {
    let mut res: i64 = 0;
    let o: *const TValue = index2value(state, index);
    let is_number_: bool =
        if (((*o).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32 as i64 != 0 {
            res = (*o).value.i;
            true
        } else {
            luav_tointeger(o, &mut res, F2I::Equal) != 0
        };
    if !is_number.is_null() {
        *is_number = is_number_;
    }
    return res;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_toboolean(state: *mut State, index: i32) -> i32 { unsafe {
    let o: *const TValue = index2value(state, index);
    return !((*o).get_tag() == TAG_VARIANT_BOOLEAN_FALSE || get_tag_type((*o).get_tag()) == TAG_TYPE_NIL)
        as i32;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_tolstring(
    state: *mut State,
    index: i32,
    length: *mut u64,
) -> *const i8 { unsafe {
    let mut o: *mut TValue = index2value(state, index);
    if !(get_tag_type((*o).get_tag()) == TAG_TYPE_STRING) {
        if !(get_tag_type((*o).get_tag()) == TAG_TYPE_NUMERIC) {
            if !length.is_null() {
                *length = 0;
            }
            return std::ptr::null();
        }
        luao_tostring(state, o);
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        o = index2value(state, index);
    }
    if !length.is_null() {
        *length = if (*((*o).value.object as *mut GCUnion)).ts.short_length as i32 != 0xFF as i32 {
            (*((*o).value.object as *mut GCUnion)).ts.short_length as u64
        } else {
            (*((*o).value.object as *mut GCUnion)).ts.u.long_length
        };
    }
    return ((*((*o).value.object as *mut GCUnion)).ts.contents).as_mut_ptr();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawlen(state: *mut State, index: i32) -> u64 { unsafe {
    let o: *const TValue = index2value(state, index);
    match (*o).get_tag_variant() {
        TAG_VARIANT_STRING_SHORT => return (*((*o).value.object as *mut GCUnion)).ts.short_length as u64,
        TAG_VARIANT_STRING_LONG => return (*((*o).value.object as *mut GCUnion)).ts.u.long_length as u64,
        TAG_VARIANT_USER => return (*((*o).value.object as *mut GCUnion)).u.length as u64,
        TAG_VARIANT_TABLE => return luah_getn(&mut (*((*o).value.object as *mut GCUnion)).h),
        _ => return 0,
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_tocfunction(state: *mut State, index: i32) -> CFunction { unsafe {
    let o: *const TValue = index2value(state, index);
    match (*o).get_tag_variant() {
    TAG_VARIANT_CLOSURE_CFUNCTION => (*o).value.f,
    TAG_VARIANT_CLOSURE_C =>
        (*((*o).value.object as *mut GCUnion)).ccl.f,
        _ => None,
    }
}}
#[inline]
pub unsafe extern "C" fn touserdata(o: *const TValue) -> *mut libc::c_void { unsafe {
    match get_tag_type((*o).get_tag()) {
        7 => {
            return (&mut (*((*o).value.object as *mut GCUnion)).u as *mut User as *mut i8).offset(
                (if (*((*o).value.object as *mut GCUnion)).u.nuvalue as i32 == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<UValue>() as u64)
                            .wrapping_mul((*((*o).value.object as *mut GCUnion)).u.nuvalue as u64),
                    )
                }) as isize,
            ) as *mut libc::c_void;
        }
        2 => return (*o).value.p,
        _ => return std::ptr::null_mut(),
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_touserdata(state: *mut State, index: i32) -> *mut libc::c_void { unsafe {
    let o: *const TValue = index2value(state, index);
    return touserdata(o);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_tothread(state: *mut State, index: i32) -> *mut State { unsafe {
    let o: *const TValue = index2value(state, index);
    return if !((*o).get_tag_variant() == TAG_VARIANT_STATE) {
        std::ptr::null_mut()
    } else {
        &mut (*((*o).value.object as *mut GCUnion)).th
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_topointer(state: *mut State, index: i32) -> *const libc::c_void { unsafe {
    let o: *const TValue = index2value(state, index);
    match (*o).get_tag_variant() {
        TAG_VARIANT_CLOSURE_CFUNCTION => {
            return ::core::mem::transmute::<CFunction, u64>((*o).value.f) as *mut libc::c_void;
        }
        TAG_VARIANT_USER | TAG_VARIANT_POINTER => return touserdata(o),
        _ => {
            if (*o).is_collectable() {
                return (*o).value.object as *const libc::c_void;
            } else {
                return std::ptr::null();
            }
        }
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushlstring(
    state: *mut State,
    s: *const i8,
    length: u64,
) -> *const i8 { unsafe {
    let ts: *mut TString = if length == 0u64 {
        luas_new(state, b"\0" as *const u8 as *const i8)
    } else {
        luas_newlstr(state, s, length)
    };
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut TString = ts;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag ((*x_).get_tag());
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    return ((*ts).contents).as_mut_ptr();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushstring(state: *mut State, mut s: *const i8) -> *const i8 { unsafe {
    if s.is_null() {
        (*(*state).top.p).value.set_tag (TAG_VARIANT_NIL_NIL);
    } else {
        let ts: *mut TString = luas_new(state, s);
        let io: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut TString = ts;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        s = ((*ts).contents).as_mut_ptr();
    }
    (*state).top.p = (*state).top.p.offset(1);
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    return s;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushvfstring(
    state: *mut State,
    fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 { unsafe {
    let ret: *const i8 = luao_pushvfstring(state, fmt, argp.as_va_list());
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    return ret;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushfstring(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> *const i8 { unsafe {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    let ret: *const i8 = luao_pushvfstring(state, fmt, argp.as_va_list());
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    return ret;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushcclosure(state: *mut State, fn_0: CFunction, mut n: i32) { unsafe {
    if n == 0 {
        let io: *mut TValue = &mut (*(*state).top.p).value;
        (*io).value.f = fn_0;
        (*io).set_tag (TAG_VARIANT_CLOSURE_CFUNCTION);
        (*state).top.p = (*state).top.p.offset(1);
        } else {
        let cl: *mut CClosure = luaf_newcclosure(state, n);
        (*cl).f = fn_0;
        (*state).top.p = (*state).top.p.offset(-(n as isize));
        loop {
            let fresh5 = n;
            n = n - 1;
            if !(fresh5 != 0) {
                break;
            }
            let io1: *mut TValue =
                &mut *((*cl).upvalue).as_mut_ptr().offset(n as isize) as *mut TValue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(n as isize)).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
        }
        let io_0: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut CClosure = cl;
        (*io_0).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io_0).set_tag(TAG_VARIANT_CLOSURE_C);
        (*io_0).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
            if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pushlightuserdata(state: *mut State, p: *mut libc::c_void) { unsafe {
    let io: *mut TValue = &mut (*(*state).top.p).value;
    (*io).value.p = p;
    (*io).set_tag (TAG_TYPE_POINTER);
    (*state).top.p = (*state).top.p.offset(1);
}}
#[inline]
pub unsafe extern "C" fn auxgetstr(
    state: *mut State,
    t: *const TValue,
    k: *const i8,
) -> i32 { unsafe {
    let slot: *const TValue;
    let str: *mut TString = luas_new(state, k);
    if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
        slot = std::ptr::null();
        0
    } else {
        slot = luah_getstr(&mut (*((*t).value.object as *mut GCUnion)).h, str);
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = slot;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
        } else {
        let io: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut TString = str;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
            luav_finishget(
            state,
            t,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            (*state).top.p.offset(-(1 as isize)),
            slot,
        );
    }
    return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getglobal(state: *mut State, name: *const i8) -> i32 { unsafe {
    let global_table: *const TValue = &mut *((*((*(*state).global).l_registry.value.object as *mut GCUnion))
        .h
        .array)
        .offset((2 - 1) as isize) as *mut TValue;
    return auxgetstr(state, global_table, name);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gettable(state: *mut State, index: i32) -> i32 { unsafe {
    let slot;
    let t: *mut TValue = index2value(state, index);
    if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
        slot = std::ptr::null();
        0
    } else {
        slot = luah_get(
            &mut (*((*t).value.object as *mut GCUnion)).h,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
        );
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        let io2: *const TValue = slot;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
    } else {
        luav_finishget(
            state,
            t,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            (*state).top.p.offset(-(1 as isize)),
            slot,
        );
    }
    return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getfield(
    state: *mut State,
    index: i32,
    k: *const i8,
) -> i32 { unsafe {
    return auxgetstr(state, index2value(state, index), k);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_geti(state: *mut State, index: i32, n: i64) -> i32 { unsafe {
    let t: *mut TValue;
    let slot: *const TValue;
    t = index2value(state, index);
    if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
        slot = std::ptr::null();
        0
    } else {
        slot = if (n as u64).wrapping_sub(1 as u32 as u64)
            < (*((*t).value.object as *mut GCUnion)).h.array_limit as u64
        {
            &mut *((*((*t).value.object as *mut GCUnion)).h.array).offset((n - 1) as isize)
                as *mut TValue as *const TValue
        } else {
            luah_getint(&mut (*((*t).value.object as *mut GCUnion)).h, n)
        };
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = slot;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
    } else {
        let mut aux: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut aux;
        (*io).value.i = n;
        (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
        luav_finishget(state, t, &mut aux, (*state).top.p, slot);
    }
    (*state).top.p = (*state).top.p.offset(1);
    return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
}}
#[inline]
pub unsafe extern "C" fn finishrawget(state: *mut State, value: *const TValue) -> i32 { unsafe {
    if get_tag_type((*value).get_tag()) == TAG_TYPE_NIL {
        (*(*state).top.p).value.set_tag (TAG_VARIANT_NIL_NIL);
    } else {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
    }
    (*state).top.p = (*state).top.p.offset(1);
    return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
}}
pub unsafe extern "C" fn gettable(state: *mut State, index: i32) -> *mut Table { unsafe {
    let t: *mut TValue = index2value(state, index);
    return &mut (*((*t).value.object as *mut GCUnion)).h;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawget(state: *mut State, index: i32) -> i32 { unsafe {
    let table: *mut Table = gettable(state, index);
    let value: *const TValue = luah_get(table, &mut (*(*state).top.p.offset(-(1 as isize))).value);
    (*state).top.p = (*state).top.p.offset(-1);
    return finishrawget(state, value);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawgeti(state: *mut State, index: i32, n: i64) -> i32 { unsafe {
    let table: *mut Table = gettable(state, index);
    return finishrawget(state, luah_getint(table, n));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawgetp(
    state: *mut State,
    index: i32,
    p: *const libc::c_void,
) -> i32 { unsafe {
    let table: *mut Table;
    let mut k: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    table = gettable(state, index);
    let io: *mut TValue = &mut k;
    (*io).value.p = p as *mut libc::c_void;
    (*io).set_tag (TAG_TYPE_POINTER);
    return finishrawget(state, luah_get(table, &mut k));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_createtable(state: *mut State, narray: i32, nrec: i32) { unsafe {
    let table: *mut Table;
    table = luah_new(state);
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut Table = table;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_TABLE);
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    if narray > 0 || nrec > 0 {
        luah_resize(state, table, narray as u32, nrec as u32);
    }
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getmetatable(state: *mut State, objindex: i32) -> i32 { unsafe {
    let obj: *const TValue = index2value(state, objindex);
    let mt: *mut Table;
    match (*obj).get_tag_type() {
        TAG_TYPE_TABLE => {
            mt = (*((*obj).value.object as *mut GCUnion)).h.metatable;
        }
        TAG_TYPE_USER => {
            mt = (*((*obj).value.object as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*state).global).mt[(get_tag_type((*obj).get_tag())) as usize];
        }
    }
    if mt.is_null() {
        0
    } else {
        let io: *mut TValue = &mut (*(*state).top.p).value;
        (*io).value.object = &mut (*(mt as *mut GCUnion)).object;
        (*io).set_tag (TAG_VARIANT_TABLE);
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
        1
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getiuservalue(state: *mut State, index: i32, n: i32) -> i32 { unsafe {
    let t: i32;
    let o: *mut TValue = index2value(state, index);
    if n <= 0 || n > (*((*o).value.object as *mut GCUnion)).u.nuvalue as i32 {
        (*(*state).top.p).value.set_tag (TAG_VARIANT_NIL_NIL);
        t = -1;
    } else {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = &mut (*((*((*o).value.object as *mut GCUnion)).u.uv)
            .as_mut_ptr()
            .offset((n - 1) as isize))
        .uv;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        t = (get_tag_type((*(*state).top.p).value.get_tag())) as i32;
    }
    (*state).top.p = (*state).top.p.offset(1);
    return t;
}}
pub unsafe extern "C" fn auxsetstr(state: *mut State, t: *const TValue, k: *const i8) { unsafe {
    let slot: *const TValue;
    let str: *mut TString = luas_new(state, k);
    if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
        slot = std::ptr::null();
        0
    } else {
        slot = luah_getstr(&mut (*((*t).value.object as *mut GCUnion)).h, str);
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = slot as *mut TValue;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
            if (*(*t).value.object).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, (*t).value.object);
            } else {
            };
        } else {
        };
        (*state).top.p = (*state).top.p.offset(-1);
        } else {
        let io: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut TString = str;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
            luav_finishset(
            state,
            t,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            &mut (*(*state).top.p.offset(-(2 as isize))).value,
            slot,
        );
        (*state).top.p = (*state).top.p.offset(-(2 as isize));
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setglobal(state: *mut State, name: *const i8) { unsafe {
    let global_table: *const TValue = &mut *((*((*(*state).global).l_registry.value.object as *mut GCUnion))
        .h
        .array)
        .offset((2 - 1) as isize) as *mut TValue;
    auxsetstr(state, global_table, name);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_settable(state: *mut State, index: i32) { unsafe {
    let t: *mut TValue;
    let slot: *const TValue;
    t = index2value(state, index);
    if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
        slot = std::ptr::null();
        0
    } else {
        slot = luah_get(
            &mut (*((*t).value.object as *mut GCUnion)).h,
            &mut (*(*state).top.p.offset(-(2 as isize))).value,
        );
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = slot as *mut TValue;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
            if (*(*t).value.object).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, (*t).value.object);
            } else {
            };
        } else {
        };
    } else {
        luav_finishset(
            state,
            t,
            &mut (*(*state).top.p.offset(-(2 as isize))).value,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            slot,
        );
    }
    (*state).top.p = (*state).top.p.offset(-(2 as isize));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setfield(state: *mut State, index: i32, k: *const i8) { unsafe {
    auxsetstr(state, index2value(state, index), k);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_seti(state: *mut State, index: i32, n: i64) { unsafe {
    let t: *mut TValue;
    let slot: *const TValue;
    t = index2value(state, index);
    if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
        slot = std::ptr::null();
        0
    } else {
        slot = if (n as u64).wrapping_sub(1 as u32 as u64)
            < (*((*t).value.object as *mut GCUnion)).h.array_limit as u64
        {
            &mut *((*((*t).value.object as *mut GCUnion)).h.array).offset((n - 1) as isize)
                as *mut TValue as *const TValue
        } else {
            luah_getint(&mut (*((*t).value.object as *mut GCUnion)).h, n)
        };
        (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
    } != 0
    {
        let io1: *mut TValue = slot as *mut TValue;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
            if (*(*t).value.object).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, (*t).value.object);
            } else {
            };
        } else {
        };
    } else {
        let mut aux: TValue = TValue::new();
        aux.value.i = n;
        aux.set_tag (TAG_VARIANT_NUMERIC_INTEGER);
        luav_finishset(
            state,
            t,
            &mut aux,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            slot,
        );
    }
    (*state).top.p = (*state).top.p.offset(-1);
}}
pub unsafe extern "C" fn aux_rawset(
    state: *mut State,
    index: i32,
    key: *mut TValue,
    n: i32,
) { unsafe {
    let table: *mut Table = gettable(state, index);
    luah_set(
        state,
        table,
        key,
        &mut (*(*state).top.p.offset(-(1 as isize))).value,
    );
    (*table).flags = ((*table).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
    if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
        if (*(table as *mut GCUnion)).object.get_marked() & 1 << 5 != 0
            && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            luac_barrierback_(state, &mut (*(table as *mut GCUnion)).object);
        } else {
        };
    } else {
    };
    (*state).top.p = (*state).top.p.offset(-(n as isize));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawset(state: *mut State, index: i32) { unsafe {
    aux_rawset(
        state,
        index,
        &mut (*(*state).top.p.offset(-(2 as isize))).value,
        2,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawsetp(
    state: *mut State,
    index: i32,
    p: *const libc::c_void,
) { unsafe {
    let mut k: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io: *mut TValue = &mut k;
    (*io).value.p = p as *mut libc::c_void;
    (*io).set_tag (TAG_TYPE_POINTER);
    aux_rawset(state, index, &mut k, 1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_rawseti(state: *mut State, index: i32, n: i64) { unsafe {
    let table: *mut Table = gettable(state, index);
    luah_setint(
        state,
        table,
        n,
        &mut (*(*state).top.p.offset(-(1 as isize))).value,
    );
    if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
        if (*(table as *mut GCUnion)).object.get_marked() & 1 << 5 != 0
            && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            luac_barrierback_(state, &mut (*(table as *mut GCUnion)).object);
        } else {
        };
    } else {
    };
    (*state).top.p = (*state).top.p.offset(-1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setmetatable(state: *mut State, objindex: i32) -> i32 { unsafe {
    let mt: *mut Table;
    let obj: *mut TValue = index2value(state, objindex);
    if get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag()) == TAG_TYPE_NIL {
        mt = std::ptr::null_mut();
    } else {
        mt = &mut (*((*(*state).top.p.offset(-(1 as isize))).value.value.object as *mut GCUnion)).h;
    }
    match get_tag_type((*obj).get_tag()) {
        5 => {
            let ref mut fresh6 = (*((*obj).value.object as *mut GCUnion)).h.metatable;
            *fresh6 = mt;
            if !mt.is_null() {
                if (*(*obj).value.object).get_marked() & 1 << 5 != 0
                    && (*mt).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*((*obj).value.object as *mut GCUnion)).object,
                        &mut (*(mt as *mut GCUnion)).object,
                    );
                } else {
                };
                luac_checkfinalizer(state, (*obj).value.object, mt);
            }
        }
        7 => {
            let ref mut fresh7 = (*((*obj).value.object as *mut GCUnion)).u.metatable;
            *fresh7 = mt;
            if !mt.is_null() {
                if (*((*obj).value.object as *mut GCUnion)).u.get_marked() & 1 << 5 != 0
                    && (*mt).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(&mut (*((*obj).value.object as *mut GCUnion)).u as *mut User
                            as *mut GCUnion))
                            .object,
                        &mut (*(mt as *mut GCUnion)).object,
                    );
                } else {
                };
                luac_checkfinalizer(state, (*obj).value.object, mt);
            }
        }
        _ => {
            (*(*state).global).mt[(get_tag_type((*obj).get_tag())) as usize] = mt;
        }
    }
    (*state).top.p = (*state).top.p.offset(-1);
    return 1;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setiuservalue(state: *mut State, index: i32, n: i32) -> i32 { unsafe {
    let res: i32;
    let o: *mut TValue = index2value(state, index);
    if !((n as u32).wrapping_sub(1 as u32) < (*((*o).value.object as *mut GCUnion)).u.nuvalue as u32) {
        res = 0;
    } else {
        let io1: *mut TValue = &mut (*((*((*o).value.object as *mut GCUnion)).u.uv)
            .as_mut_ptr()
            .offset((n - 1) as isize))
        .uv;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        if (*(*state).top.p.offset(-(1 as isize))).value.is_collectable() {
            if (*(*o).value.object).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).value.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, (*o).value.object);
            } else {
            };
        } else {
        };
        res = 1;
    }
    (*state).top.p = (*state).top.p.offset(-1);
    return res;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_callk(
    state: *mut State,
    nargs: i32,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) { unsafe {
    let function: StkId = (*state).top.p.offset(-((nargs + 1) as isize));
    if k.is_some() && (*state).count_c_calls & 0xffff0000 as u32 == 0u32 {
        (*(*state).call_info).u.c.k = k;
        (*(*state).call_info).u.c.ctx = ctx;
        ccall(state, function, count_results, 1);
    } else {
        luad_callnoyield(state, function, count_results);
    }
    if count_results <= -1 && (*(*state).call_info).top.p < (*state).top.p {
        (*(*state).call_info).top.p = (*state).top.p;
    }
}}
pub unsafe extern "C" fn f_call(state: *mut State, ud: *mut libc::c_void) { unsafe {
    let c: *mut CallS = ud as *mut CallS;
    luad_callnoyield(state, (*c).function, (*c).count_results);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_pcallk(
    state: *mut State,
    nargs: i32,
    count_results: i32,
    error_function: i32,
    ctx: i64,
    k: ContextFunction,
) -> i32 { unsafe {
    let mut c: CallS = CallS {
        function: std::ptr::null_mut(),
        count_results: 0,
    };
    let status: i32;
    let function: i64;
    if error_function == 0 {
        function = 0;
    } else {
        let o: StkId = index2stack(state, error_function);
        function = (o as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
    }
    c.function = (*state).top.p.offset(-((nargs + 1) as isize));
    if k.is_none() || !((*state).count_c_calls & 0xffff0000 as u32 == 0u32) {
        c.count_results = count_results;
        status = luad_pcall(
            state,
            Some(f_call as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64,
            function,
        );
    } else {
        let call_info: *mut CallInfo = (*state).call_info;
        (*call_info).u.c.k = k;
        (*call_info).u.c.ctx = ctx;
        (*call_info).u2.funcidx =
            (c.function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64 as i32;
        (*call_info).u.c.old_error_function = (*state).error_function;
        (*state).error_function = function;
        (*call_info).call_status =
            ((*call_info).call_status as i32 & !(1 << 0) | (*state).allow_hook as i32) as u16;
        (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 4) as u16;
        ccall(state, c.function, count_results, 1);
        (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
        (*state).error_function = (*call_info).u.c.old_error_function;
        status = 0;
    }
    if count_results <= -1 && (*(*state).call_info).top.p < (*state).top.p {
        (*(*state).call_info).top.p = (*state).top.p;
    }
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_load(
    state: *mut State,
    reader: ReadFunction,
    data: *mut libc::c_void,
    mut chunkname: *const i8,
    mode: *const i8,
) -> i32 { unsafe {
    let mut zio: ZIO = ZIO {
        n: 0,
        p: std::ptr::null(),
        reader: None,
        data: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
    };
    let status: i32;
    if chunkname.is_null() {
        chunkname = b"?\0" as *const u8 as *const i8;
    }
    luaz_init(state, &mut zio, reader, data);
    status = luad_protectedparser(state, &mut zio, chunkname, mode);
    if status == 0 {
        let f: *mut LClosure =
            &mut (*((*(*state).top.p.offset(-(1 as isize))).value.value.object as *mut GCUnion)).lcl;
        if (*f).count_upvalues as i32 >= 1 {
            let gt: *const TValue =
                &mut *((*((*(*state).global).l_registry.value.object as *mut GCUnion))
                    .h
                    .array)
                    .offset((2 - 1) as isize) as *mut TValue;
            let io1: *mut TValue = (**((*f).upvalues).as_mut_ptr().offset(0 as isize)).v.p;
            let io2: *const TValue = gt;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            if (*gt).is_collectable() {
                if (**((*f).upvalues).as_mut_ptr().offset(0 as isize)).get_marked()
                    & 1 << 5
                    != 0
                    && (*(*gt).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(*((*f).upvalues).as_mut_ptr().offset(0 as isize)
                            as *mut GCUnion))
                            .object,
                        &mut (*((*gt).value.object as *mut GCUnion)).object,
                    );
                } else {
                };
            } else {
            };
        }
    }
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_dump(
    state: *mut State,
    writer_0: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 { unsafe {
    let status: i32;
    let o: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
    if (*o).get_tag_variant() == TAG_VARIANT_CLOSURE_L {
        status = luau_dump(
            state,
            (*((*o).value.object as *mut GCUnion)).lcl.p,
            writer_0,
            data,
            is_strip,
        );
    } else {
        status = 1;
    }
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_status(state: *mut State) -> i32 { unsafe {
    return (*state).status as i32;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gc(state: *mut State, what: i32, args: ...) -> i32 { unsafe {
    let mut argp: ::core::ffi::VaListImpl;
    let mut res: i32 = 0;
    let g: *mut Global = (*state).global;
    if (*g).gcstp as i32 & 2 != 0 {
        return -1;
    }
    argp = args.clone();
    match what {
        0 => {
            (*g).gcstp = 1;
        }
        1 => {
            (*g).set_debt(0);
            (*g).gcstp = 0;
        }
        2 => {
            luac_fullgc(state, false);
        }
        3 => {
            res = (((*g).totalbytes + (*g).gc_debt) as u64 >> 10 as i32) as i32;
        }
        4 => {
            res = (((*g).totalbytes + (*g).gc_debt) as u64 & 0x3ff as i32 as u64) as i32;
        }
        5 => {
            let data: i32 = argp.arg::<i32>();
            let mut debt: i64 = 1;
            let oldstp: u8 = (*g).gcstp;
            (*g).gcstp = 0;
            if data == 0 {
                (*g).set_debt(0);
                luac_step(state);
            } else {
                debt = data as i64 * 1024 as i32 as i64 + (*g).gc_debt;
                (*g).set_debt(debt);
                if (*(*state).global).gc_debt > 0 {
                    luac_step(state);
                }
            }
            (*g).gcstp = oldstp;
            if debt > 0 && (*g).gcstate as i32 == 8 {
                res = 1;
            }
        }
        6 => {
            let data_0: i32 = argp.arg::<i32>();
            res = (*g).gcpause as i32 * 4;
            (*g).gcpause = (data_0 / 4) as u8;
        }
        7 => {
            let data_1: i32 = argp.arg::<i32>();
            res = (*g).gcstepmul as i32 * 4;
            (*g).gcstepmul = (data_1 / 4) as u8;
        }
        9 => {
            res = ((*g).gcstp as i32 == 0) as i32;
        }
        10 => {
            let minormul: i32 = argp.arg::<i32>();
            let majormul: i32 = argp.arg::<i32>();
            res = if (*g).gckind as i32 == 1 || (*g).lastatomic != 0u64 {
                10 as i32
            } else {
                11 as i32
            };
            if minormul != 0 {
                (*g).genminormul = minormul as u64;
            }
            if majormul != 0 {
                (*g).genmajormul = (majormul / 4) as u64;
            }
            luac_changemode(state, 1);
        }
        11 => {
            let pause: i32 = argp.arg::<i32>();
            let stepmul: i32 = argp.arg::<i32>();
            let stepsize: i32 = argp.arg::<i32>();
            res = if (*g).gckind as i32 == 1 || (*g).lastatomic != 0u64 {
                10 as i32
            } else {
                11 as i32
            };
            if pause != 0 {
                (*g).gcpause = (pause / 4) as u8;
            }
            if stepmul != 0 {
                (*g).gcstepmul = (stepmul / 4) as u8;
            }
            if stepsize != 0 {
                (*g).gcstepsize = stepsize as u8;
            }
            luac_changemode(state, 0);
        }
        _ => {
            res = -1;
        }
    }
    return res;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_error(state: *mut State) -> i32 { unsafe {
    let errobj: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
    if (*errobj).get_tag_variant() == TAG_VARIANT_STRING_SHORT
        && &mut (*((*errobj).value.object as *mut GCUnion)).ts as *mut TString
            == (*(*state).global).memerrmsg
    {
        luad_throw(state, 4);
    } else {
        luag_errormsg(state);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_next(state: *mut State, index: i32) -> i32 { unsafe {
    let table: *mut Table = gettable(state, index);
    let more: i32 = luah_next(state, table, (*state).top.p.offset(-(1 as isize)));
    if more != 0 {
        (*state).top.p = (*state).top.p.offset(1);
    } else {
        (*state).top.p = (*state).top.p.offset(-(1 as isize));
    }
    return more;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_toclose(state: *mut State, index: i32) { unsafe {
    let o: StkId = index2stack(state, index);
    let count_results: i32 = (*(*state).call_info).count_results as i32;
    luaf_newtbcupval(state, o);
    if !(count_results < -1) {
        (*(*state).call_info).count_results = (-count_results - 3) as i16;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_concat(state: *mut State, n: i32) { unsafe {
    if n > 0 {
        luav_concat(state, n);
    } else {
        let io: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut TString =
            luas_newlstr(state, b"\0" as *const u8 as *const i8, 0u64);
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
        }
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_len(state: *mut State, index: i32) { unsafe {
    let t: *mut TValue = index2value(state, index);
    luav_objlen(state, (*state).top.p, t);
    (*state).top.p = (*state).top.p.offset(1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setwarnf(
    state: *mut State,
    f: WarnFunction,
    ud: *mut libc::c_void,
) { unsafe {
    (*(*state).global).ud_warn = ud;
    (*(*state).global).warnf = f;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_warning(state: *mut State, message: *const i8, tocont: i32) { unsafe {
    luae_warning(state, message, tocont);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_newuserdatauv(
    state: *mut State,
    size: u64,
    nuvalue: i32,
) -> *mut libc::c_void { unsafe {
    let u: *mut User = luas_newudata(state, size, nuvalue);
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut User = u;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_USER);
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    return (u as *mut i8).offset(
        (if (*u).nuvalue as i32 == 0 {
            32 as u64
        } else {
            (40 as u64).wrapping_add(
                (::core::mem::size_of::<UValue>() as u64).wrapping_mul((*u).nuvalue as u64),
            )
        }) as isize,
    ) as *mut libc::c_void;
}}
pub unsafe extern "C" fn aux_upvalue(
    fi: *mut TValue,
    n: i32,
    value: *mut *mut TValue,
    owner: *mut *mut Object,
) -> *const i8 { unsafe {
    match (*fi).get_tag_variant() {
        TAG_VARIANT_CLOSURE_C => {
            let f: *mut CClosure = &mut (*((*fi).value.object as *mut GCUnion)).ccl;
            if !((n as u32).wrapping_sub(1 as u32) < (*f).count_upvalues as u32) {
                return std::ptr::null();
            }
            *value = &mut *((*f).upvalue).as_mut_ptr().offset((n - 1) as isize) as *mut TValue;
            if !owner.is_null() {
                *owner = &mut (*(f as *mut GCUnion)).object;
            }
            return b"\0" as *const u8 as *const i8;
        }
        TAG_VARIANT_CLOSURE_L => {
            let f_0: *mut LClosure = &mut (*((*fi).value.object as *mut GCUnion)).lcl;
            let p: *mut Prototype = (*f_0).p;
            if !((n as u32).wrapping_sub(1 as u32) < (*p).size_upvalues as u32) {
                return std::ptr::null();
            }
            *value = (**((*f_0).upvalues).as_mut_ptr().offset((n - 1) as isize))
                .v
                .p;
            if !owner.is_null() {
                *owner = &mut (*(*((*f_0).upvalues).as_mut_ptr().offset((n - 1) as isize)
                    as *mut GCUnion))
                    .object;
            }
            let name: *mut TString = (*((*p).upvalues).offset((n - 1) as isize)).name;
            return if name.is_null() {
                b"(no name)\0" as *const u8 as *const i8
            } else {
                ((*name).contents).as_mut_ptr() as *const i8
            };
        }
        _ => return std::ptr::null(),
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getupvalue(
    state: *mut State,
    funcindex: i32,
    n: i32,
) -> *const i8 { unsafe {
    let mut value: *mut TValue = std::ptr::null_mut();
    let name: *const i8 = aux_upvalue(
        index2value(state, funcindex),
        n,
        &mut value,
        std::ptr::null_mut(),
    );
    if !name.is_null() {
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
        }
    return name;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setupvalue(
    state: *mut State,
    funcindex: i32,
    n: i32,
) -> *const i8 { unsafe {
    let mut value: *mut TValue = std::ptr::null_mut();
    let mut owner: *mut Object = std::ptr::null_mut();
    let fi: *mut TValue = index2value(state, funcindex);
    let name: *const i8 = aux_upvalue(fi, n, &mut value, &mut owner);
    if !name.is_null() {
        (*state).top.p = (*state).top.p.offset(-1);
            let io1: *mut TValue = value;
        let io2: *const TValue = &mut (*(*state).top.p).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        if (*value).is_collectable() {
            if (*owner).get_marked() & 1 << 5 != 0
                && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    state,
                    &mut (*(owner as *mut GCUnion)).object,
                    &mut (*((*value).value.object as *mut GCUnion)).object,
                );
            } else {
            };
        } else {
        };
    }
    return name;
}}
pub unsafe extern "C" fn getupvalref(
    state: *mut State,
    fidx: i32,
    n: i32,
    pf: *mut *mut LClosure,
) -> *mut *mut UpValue { unsafe {
    static mut NULLUP: *const UpValue = std::ptr::null();
    let fi: *mut TValue = index2value(state, fidx);
    let f: *mut LClosure = &mut (*((*fi).value.object as *mut GCUnion)).lcl;
    if !pf.is_null() {
        *pf = f;
    }
    if 1 <= n && n <= (*(*f).p).size_upvalues {
        return &mut *((*f).upvalues).as_mut_ptr().offset((n - 1) as isize) as *mut *mut UpValue;
    } else {
        return &NULLUP as *const *const UpValue as *mut *mut UpValue;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_upvalueid(
    state: *mut State,
    fidx: i32,
    n: i32,
) -> *mut libc::c_void { unsafe {
    let fi: *mut TValue = index2value(state, fidx);
    match (*fi).get_tag_variant() {
        TAG_VARIANT_CLOSURE_L => {
            return *getupvalref(state, fidx, n, std::ptr::null_mut()) as *mut libc::c_void;
        }
        TAG_VARIANT_CLOSURE_C => {
            let f: *mut CClosure = &mut (*((*fi).value.object as *mut GCUnion)).ccl;
            if 1 <= n && n <= (*f).count_upvalues as i32 {
                return &mut *((*f).upvalue).as_mut_ptr().offset((n - 1) as isize) as *mut TValue
                    as *mut libc::c_void;
            }
        }
        TAG_VARIANT_CLOSURE_CFUNCTION => {}
        _ => return std::ptr::null_mut(),
    }
    return std::ptr::null_mut();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_upvaluejoin(
    state: *mut State,
    fidx1: i32,
    n1: i32,
    fidx2: i32,
    n2: i32,
) { unsafe {
    let mut f1: *mut LClosure = std::ptr::null_mut();
    let up1: *mut *mut UpValue = getupvalref(state, fidx1, n1, &mut f1);
    let up2: *mut *mut UpValue = getupvalref(state, fidx2, n2, std::ptr::null_mut());
    *up1 = *up2;
    if (*f1).get_marked() & 1 << 5 != 0
        && (**up1).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            state,
            &mut (*(f1 as *mut GCUnion)).object,
            &mut (*(*up1 as *mut GCUnion)).object,
        );
    } else {
    };
}}
pub unsafe extern "C" fn luai_makeseed(state: *mut State) -> u32 { unsafe {
    let mut buffer: [i8; 24] = [0; 24];
    let mut h: u32 = time(std::ptr::null_mut()) as u32;
    let mut p: i32 = 0;
    let mut t: u64 = state as u64;
    memcpy(
        buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t as *mut u64 as *const libc::c_void,
        ::core::mem::size_of::<u64>() as u64,
    );
    p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32 as i32;
    let mut t_0: u64 = &mut h as *mut u32 as u64;
    memcpy(
        buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_0 as *mut u64 as *const libc::c_void,
        ::core::mem::size_of::<u64>() as u64,
    );
    p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32 as i32;
    let mut t_1: u64 = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> *mut State>,
        u64,
    >(Some(
        lua_newstate as unsafe extern "C" fn() -> *mut State,
    ));
    memcpy(
        buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_1 as *mut u64 as *const libc::c_void,
        ::core::mem::size_of::<u64>() as u64,
    );
    p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32 as i32;
    return luas_hash(buffer.as_mut_ptr(), p as u64, h);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setcstacklimit(mut _state: *mut State, mut _limit: u32) -> i32 {
    return 200 as i32;
}
pub unsafe extern "C" fn luae_extendci(state: *mut State) -> *mut CallInfo { unsafe {
    let call_info;
    call_info = luam_malloc_(state, ::core::mem::size_of::<CallInfo>() as u64) as *mut CallInfo;
    (*(*state).call_info).next = call_info;
    (*call_info).previous = (*state).call_info;
    (*call_info).next = std::ptr::null_mut();
    ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 0);
    (*state).count_call_info = ((*state).count_call_info).wrapping_add(1);
    (*state).count_call_info;
    return call_info;
}}
pub unsafe extern "C" fn freeci(state: *mut State) { unsafe {
    let mut call_info: *mut CallInfo = (*state).call_info;
    let mut next: *mut CallInfo = (*call_info).next;
    (*call_info).next = std::ptr::null_mut();
    loop {
        call_info = next;
        if call_info.is_null() {
            break;
        }
        next = (*call_info).next;
        (*state).free_memory(
            call_info as *mut libc::c_void,
            ::core::mem::size_of::<CallInfo>() as u64,
        );
        (*state).count_call_info = ((*state).count_call_info).wrapping_sub(1);
        (*state).count_call_info;
    }
}}
pub unsafe extern "C" fn luae_shrinkci(state: *mut State) { unsafe {
    let mut call_info: *mut CallInfo = (*(*state).call_info).next;
    if !call_info.is_null() {
        let mut next: *mut CallInfo;
        loop {
            next = (*call_info).next;
            if next.is_null() {
                break;
            }
            let next2: *mut CallInfo = (*next).next;
            (*call_info).next = next2;
            (*state).count_call_info = ((*state).count_call_info).wrapping_sub(1);
            (*state).count_call_info;
            (*state).free_memory(
                next as *mut libc::c_void,
                ::core::mem::size_of::<CallInfo>() as u64,
            );
            if next2.is_null() {
                break;
            }
            (*next2).previous = call_info;
            call_info = next2;
        }
    }
}}
pub unsafe extern "C" fn luae_checkcstack(state: *mut State) { unsafe {
    if (*state).count_c_calls & 0xffff as i32 as u32 == 200 as i32 as u32 {
        luag_runerror(state, b"C stack overflow\0" as *const u8 as *const i8);
    } else if (*state).count_c_calls & 0xffff as i32 as u32
        >= (200 as i32 / 10 as i32 * 11 as i32) as u32
    {
        luad_errerr(state);
    }
}}
pub unsafe extern "C" fn luae_inccstack(state: *mut State) { unsafe {
    (*state).count_c_calls = ((*state).count_c_calls).wrapping_add(1);
    (*state).count_c_calls;
    if (((*state).count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32) as i32 != 0) as i32
        as i64
        != 0
    {
        luae_checkcstack(state);
    }
}}
pub unsafe extern "C" fn stack_init(other_state: *mut State, state: *mut State) { unsafe {
    let mut i: i32;
    let call_info;
    (*other_state).stack.p = luam_malloc_(
        state,
        ((2 * 20 as i32 + 5) as u64)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
    ) as *mut StackValue;
    (*other_state).tbc_list.p = (*other_state).stack.p;
    i = 0;
    while i < 2 * 20 as i32 + 5 {
        (*((*other_state).stack.p).offset(i as isize)).value.set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    (*other_state).top.p = (*other_state).stack.p;
    (*other_state).stack_last.p = ((*other_state).stack.p).offset((2 * 20 as i32) as isize);
    call_info = &mut (*other_state).base_callinfo;
    (*call_info).previous = std::ptr::null_mut();
    (*call_info).next = (*call_info).previous;
    (*call_info).call_status = (1 << 1) as u16;
    (*call_info).function.p = (*other_state).top.p;
    (*call_info).u.c.k = None;
    (*call_info).count_results = 0 as i16;
    (*(*other_state).top.p).value.set_tag (TAG_VARIANT_NIL_NIL);
    (*other_state).top.p = ((*other_state).top.p).offset(1);
    (*other_state).top.p;
    (*call_info).top.p = ((*other_state).top.p).offset(20 as i32 as isize);
    (*other_state).call_info = call_info;
}}
pub unsafe extern "C" fn freestack(state: *mut State) { unsafe {
    if ((*state).stack.p).is_null() {
        return;
    }
    (*state).call_info = &mut (*state).base_callinfo;
    freeci(state);
(*state).free_memory(
        (*state).stack.p as *mut libc::c_void,
        ((((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32 + 5) as u64)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
    );
}}
pub unsafe extern "C" fn init_registry(state: *mut State, g: *mut Global) { unsafe {
    let registry: *mut Table = luah_new(state);
    let io: *mut TValue = &mut (*g).l_registry;
    let x_: *mut Table = registry;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_TABLE);
    (*io).set_collectable();
    luah_resize(state, registry, 2 as u32, 0u32);
    let io_0: *mut TValue =
        &mut *((*registry).array).offset((1 - 1) as isize) as *mut TValue;
    let x0: *mut State = state;
    (*io_0).value.object = &mut (*(x0 as *mut GCUnion)).object;
    (*io_0).set_tag(TAG_VARIANT_STATE);
    (*io_0).set_collectable();
    let io_1: *mut TValue =
        &mut *((*registry).array).offset((2 - 1) as isize) as *mut TValue;
    let x1: *mut Table = luah_new(state);
    (*io_1).value.object = &mut (*(x1 as *mut GCUnion)).object;
    (*io_1).set_tag(TAG_VARIANT_TABLE);
    (*io_1).set_collectable();
}}
pub unsafe extern "C" fn f_luaopen(state: *mut State, mut _ud: *mut libc::c_void) { unsafe {
    let g: *mut Global = (*state).global;
    stack_init(state, state);
    init_registry(state, g);
    luas_init(state);
    luat_init(state);
    luax_init(state);
    (*g).gcstp = 0;
    (*g).nilvalue.set_tag (TAG_VARIANT_NIL_NIL);
}}
pub unsafe extern "C" fn preinit_thread(state: *mut State, g: *mut Global) { unsafe {
    (*state).global = g;
    (*state).stack.p = std::ptr::null_mut();
    (*state).call_info = std::ptr::null_mut();
    (*state).count_call_info = 0;
    (*state).twups = state;
    (*state).count_c_calls = 0;
    (*state).error_jump = std::ptr::null_mut();
    ::core::ptr::write_volatile(&mut (*state).hook as *mut HookFunction, None);
    ::core::ptr::write_volatile(&mut (*state).hook_mask as *mut i32, 0);
    (*state).base_hook_count = 0;
    (*state).allow_hook = 1;
    (*state).hook_count = (*state).base_hook_count;
    (*state).open_upvalue = std::ptr::null_mut();
    (*state).status = 0;
    (*state).error_function = 0;
    (*state).old_program_counter = 0;
}}
pub unsafe extern "C" fn close_state(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    if !(get_tag_type((*g).nilvalue.get_tag()) == TAG_TYPE_NIL) {
        luac_freeallobjects(state);
    } else {
        (*state).call_info = &mut (*state).base_callinfo;
        (*state).error_function = 0;
        luad_closeprotected(state, 1 as i64, 0);
        (*state).top.p = ((*state).stack.p).offset(1 as isize);
        luac_freeallobjects(state);
    }
(*state).free_memory(
        (*(*state).global).string_table.hash as *mut libc::c_void,
        ((*(*state).global).string_table.size as u64)
            .wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
    );
    freestack(state);
    raw_allocate(
        (state as *mut u8).offset(-(8 as u64 as isize)) as *mut LX as *mut libc::c_void,
        ::core::mem::size_of::<LG>() as u64,
        0u64,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_newthread(state: *mut State) -> *mut State { unsafe {
    let g: *mut Global = (*state).global;
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    let o: *mut Object = luac_newobjdt(state, TAG_TYPE_STATE, ::core::mem::size_of::<LX>() as u64, 8 as u64);
    let other_state: *mut State = &mut (*(o as *mut GCUnion)).th;
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut State = other_state;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_STATE);
    (*io).set_collectable();
    (*state).top.p = (*state).top.p.offset(1);
    preinit_thread(other_state, g);
    ::core::ptr::write_volatile(&mut (*other_state).hook_mask as *mut i32, (*state).hook_mask);
    (*other_state).base_hook_count = (*state).base_hook_count;
    ::core::ptr::write_volatile(&mut (*other_state).hook as *mut HookFunction, (*state).hook);
    (*other_state).hook_count = (*other_state).base_hook_count;
    memcpy(
        (other_state as *mut i8).offset(-(::core::mem::size_of::<*mut libc::c_void>() as u64 as isize))
            as *mut libc::c_void,
        ((*g).mainthread as *mut i8)
            .offset(-(::core::mem::size_of::<*mut libc::c_void>() as u64 as isize))
            as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as u64,
    );
    stack_init(other_state, state);
    return other_state;
}}
pub unsafe extern "C" fn luae_freethread(state: *mut State, other_state: *mut State) { unsafe {
    let l: *mut LX = (other_state as *mut u8).offset(-(8 as u64 as isize)) as *mut LX;
    luaf_closeupval(other_state, (*other_state).stack.p);
    freestack(other_state);
(*state).free_memory(
        l as *mut libc::c_void,
        ::core::mem::size_of::<LX>() as u64,
    );
}}
pub unsafe extern "C" fn luae_resetthread(state: *mut State, mut status: i32) -> i32 { unsafe {
    (*state).call_info = &mut (*state).base_callinfo;
    let call_info: *mut CallInfo = (*state).call_info;
    (*(*state).stack.p).value.set_tag (TAG_VARIANT_NIL_NIL);
    (*call_info).function.p = (*state).stack.p;
    (*call_info).call_status = (1 << 1) as u16;
    if status == 1 {
        status = 0;
    }
    (*state).status = 0;
    (*state).error_function = 0;
    status = luad_closeprotected(state, 1 as i64, status);
    if status != 0 {
        (*state).set_error_object(status, ((*state).stack.p).offset(1 as isize));
    } else {
        (*state).top.p = ((*state).stack.p).offset(1 as isize);
    }
    (*call_info).top.p = (*state).top.p.offset(20 as i32 as isize);
    luad_reallocstack(
        state,
        ((*call_info).top.p).offset_from((*state).stack.p) as i64 as i32,
        false,
    );
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_closethread(state: *mut State, from: *mut State) -> i32 { unsafe {
    let status: i32;
    (*state).count_c_calls = if !from.is_null() {
        (*from).count_c_calls & 0xffff as i32 as u32
    } else {
        0u32
    };
    status = luae_resetthread(state, (*state).status as i32);
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_resetthread(state: *mut State) -> i32 { unsafe {
    return lua_closethread(state, std::ptr::null_mut());
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_newstate() -> *mut State { unsafe {
    let mut i: i32;
    let l: *mut LG = raw_allocate(
        std::ptr::null_mut(),
        8 as u64,
        ::core::mem::size_of::<LG>() as u64,
    ) as *mut LG;
    if l.is_null() {
        return std::ptr::null_mut();
    }
    let mut state: *mut State = &mut (*l).l.l;
    let g: *mut Global = &mut (*l).g;
    (*state).set_tag(TAG_TYPE_STATE);
    (*g).current_white = (1 << 3) as u8;
    (*state).set_marked((*g).current_white & (1 << 3 | 1 << 4));
    preinit_thread(state, g);
    (*g).allgc = &mut (*(state as *mut GCUnion)).object;
    (*state).next = std::ptr::null_mut();
    (*state).count_c_calls =
        ((*state).count_c_calls as u32).wrapping_add(0x10000 as i32 as u32) as u32 as u32;
    (*g).warnf = None;
    (*g).ud_warn = std::ptr::null_mut();
    (*g).mainthread = state;
    (*g).seed = luai_makeseed(state);
    (*g).gcstp = 2 as u8;
    (*g).string_table.length = 0;
    (*g).string_table.size = (*g).string_table.length;
    (*g).string_table.hash = std::ptr::null_mut();
    (*g).l_registry.set_tag (TAG_VARIANT_NIL_NIL);
    (*g).panic = None;
    (*g).gcstate = 8 as u8;
    (*g).gckind = 0;
    (*g).gcstopem = 0;
    (*g).is_emergency = false;
    (*g).fixedgc = std::ptr::null_mut();
    (*g).tobefnz = (*g).fixedgc;
    (*g).finobj = (*g).tobefnz;
    (*g).reallyold = std::ptr::null_mut();
    (*g).old1 = (*g).reallyold;
    (*g).survival = (*g).old1;
    (*g).firstold1 = (*g).survival;
    (*g).finobjrold = std::ptr::null_mut();
    (*g).finobjold1 = (*g).finobjrold;
    (*g).finobjsur = (*g).finobjold1;
    (*g).sweepgc = std::ptr::null_mut();
    (*g).grayagain = std::ptr::null_mut();
    (*g).gray = (*g).grayagain;
    (*g).allweak = std::ptr::null_mut();
    (*g).ephemeron = (*g).allweak;
    (*g).weak = (*g).ephemeron;
    (*g).twups = std::ptr::null_mut();
    (*g).totalbytes = ::core::mem::size_of::<LG>() as u64 as i64;
    (*g).gc_debt = 0;
    (*g).lastatomic = 0;
    let io: *mut TValue = &mut (*g).nilvalue;
    (*io).value.i = 0;
    (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
    (*g).gcpause = 200 / 4;
    (*g).gcstepmul = 100 / 4;
    (*g).gcstepsize = 13;
    (*g).genmajormul = 100 / 4;
    (*g).genminormul = 20;
    i = 0;
    while i < 9 as i32 {
        (*g).mt[i as usize] = std::ptr::null_mut();
        i += 1;
    }
    if luad_rawrunprotected(
        state,
        Some(f_luaopen as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
        std::ptr::null_mut(),
    ) != 0
    {
        close_state(state);
        state = std::ptr::null_mut();
    }
    return state;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_close(mut state: *mut State) { unsafe {
    state = (*(*state).global).mainthread;
    close_state(state);
}}
pub unsafe extern "C" fn luae_warning(state: *mut State, message: *const i8, tocont: i32) { unsafe {
    let wf: WarnFunction = (*(*state).global).warnf;
    if wf.is_some() {
        wf.expect("non-null function pointer")((*(*state).global).ud_warn, message, tocont);
    }
}}
pub unsafe extern "C" fn luae_warnerror(state: *mut State, where_0: *const i8) { unsafe {
    let errobj: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
    let message: *const i8 = if get_tag_type((*errobj).get_tag()) == TAG_TYPE_STRING {
        ((*((*errobj).value.object as *mut GCUnion)).ts.contents).as_mut_ptr() as *const i8
    } else {
        b"error object is not a string\0" as *const u8 as *const i8
    };
    luae_warning(state, b"error in \0" as *const u8 as *const i8, 1);
    luae_warning(state, where_0, 1);
    luae_warning(state, b" (\0" as *const u8 as *const i8, 1);
    luae_warning(state, message, 1);
    luae_warning(state, b")\0" as *const u8 as *const i8, 0);
}}
static mut STRING_LOCAL: [i8; 6] = unsafe { *::core::mem::transmute::<&[u8; 6], &[i8; 6]>(b"local\0") };
static mut STRING_UPVALUE: [i8; 8] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"upvalue\0") };
pub unsafe extern "C" fn currentpc(call_info: *mut CallInfo) -> i32 { unsafe {
    return ((*call_info).u.l.saved_program_counter)
        .offset_from((*(*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p).code)
        as i64 as i32
        - 1;
}}
pub unsafe extern "C" fn getbaseline(
    f: *const Prototype,
    program_counter: i32,
    basepc: *mut i32,
) -> i32 { unsafe {
    if (*f).size_absolute_line_info == 0
        || program_counter < (*((*f).absolute_line_info).offset(0 as isize)).program_counter
    {
        *basepc = -1;
        return (*f).line_defined;
    } else {
        let mut i: i32 = (program_counter as u32)
            .wrapping_div(128 as i32 as u32)
            .wrapping_sub(1 as u32) as i32;
        while (i + 1) < (*f).size_absolute_line_info
            && program_counter >= (*((*f).absolute_line_info).offset((i + 1) as isize)).program_counter
        {
            i += 1;
        }
        *basepc = (*((*f).absolute_line_info).offset(i as isize)).program_counter;
        return (*((*f).absolute_line_info).offset(i as isize)).line;
    };
}}
pub unsafe extern "C" fn luag_getfuncline(
    f: *const Prototype,
    program_counter: i32,
) -> i32 { unsafe {
    if ((*f).line_info).is_null() {
        return -1;
    } else {
        let mut basepc: i32 = 0;
        let mut baseline: i32 = getbaseline(f, program_counter, &mut basepc);
        loop {
            let fresh8 = basepc;
            basepc = basepc + 1;
            if !(fresh8 < program_counter) {
                break;
            }
            baseline += *((*f).line_info).offset(basepc as isize) as i32;
        }
        return baseline;
    };
}}
pub unsafe extern "C" fn getcurrentline(call_info: *mut CallInfo) -> i32 { unsafe {
    return luag_getfuncline(
        (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p,
        currentpc(call_info),
    );
}}
pub unsafe extern "C" fn settraps(mut call_info: *mut CallInfo) { unsafe {
    while !call_info.is_null() {
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
        }
        call_info = (*call_info).previous;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_sethook(
    state: *mut State,
    mut function: HookFunction,
    mut mask: i32,
    count: i32,
) { unsafe {
    if function.is_none() || mask == 0 {
        mask = 0;
        function = None;
    }
    ::core::ptr::write_volatile(&mut (*state).hook as *mut HookFunction, function);
    (*state).base_hook_count = count;
    (*state).hook_count = (*state).base_hook_count;
    ::core::ptr::write_volatile(&mut (*state).hook_mask as *mut i32, mask as u8 as i32);
    if mask != 0 {
        settraps((*state).call_info);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethook(state: *mut State) -> HookFunction { unsafe {
    return (*state).hook;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethookmask(state: *mut State) -> i32 { unsafe {
    return (*state).hook_mask;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethookcount(state: *mut State) -> i32 { unsafe {
    return (*state).base_hook_count;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getstack(
    state: *mut State,
    mut level: i32,
    ar: *mut Debug,
) -> i32 { unsafe {
    let status: i32;
    let mut call_info;
    if level < 0 {
        return 0;
    }
    call_info = (*state).call_info;
    while level > 0 && call_info != &mut (*state).base_callinfo as *mut CallInfo {
        level -= 1;
        call_info = (*call_info).previous;
    }
    if level == 0 && call_info != &mut (*state).base_callinfo as *mut CallInfo {
        status = 1;
        (*ar).i_ci = call_info;
    } else {
        status = 0;
    }
    return status;
}}
pub unsafe extern "C" fn upvalname(p: *const Prototype, uv: i32) -> *const i8 { unsafe {
    let s: *mut TString = (*((*p).upvalues).offset(uv as isize)).name;
    if s.is_null() {
        return b"?\0" as *const u8 as *const i8;
    } else {
        return ((*s).contents).as_mut_ptr();
    };
}}
pub unsafe extern "C" fn findvararg(
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut StkId,
) -> *const i8 { unsafe {
    if (*(*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p).is_variable_arguments {
        let nextra: i32 = (*call_info).u.l.count_extra_arguments;
        if n >= -nextra {
            *pos = ((*call_info).function.p)
                .offset(-(nextra as isize))
                .offset(-((n + 1) as isize));
            return b"(vararg)\0" as *const u8 as *const i8;
        }
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn luag_findlocal(
    state: *mut State,
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut StkId,
) -> *const i8 { unsafe {
    let base: StkId = ((*call_info).function.p).offset(1 as isize);
    let mut name: *const i8 = std::ptr::null();
    if (*call_info).call_status as i32 & 1 << 1 == 0 {
        if n < 0 {
            return findvararg(call_info, n, pos);
        } else {
            name = luaf_getlocalname(
                (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p,
                n,
                currentpc(call_info),
            );
        }
    }
    if name.is_null() {
        let limit: StkId = if call_info == (*state).call_info {
            (*state).top.p
        } else {
            (*(*call_info).next).function.p
        };
        if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
            name = if (*call_info).call_status as i32 & 1 << 1 == 0 {
                b"(temporary)\0" as *const u8 as *const i8
            } else {
                b"(C temporary)\0" as *const u8 as *const i8
            };
        } else {
            return std::ptr::null();
        }
    }
    if !pos.is_null() {
        *pos = base.offset((n - 1) as isize);
    }
    return name;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getlocal(
    state: *mut State,
    ar: *const Debug,
    n: i32,
) -> *const i8 { unsafe {
    let name;
    if ar.is_null() {
        if !((*(*state).top.p.offset(-(1 as isize))).value.get_tag_variant() == TAG_VARIANT_CLOSURE_L)
        {
            name = std::ptr::null();
        } else {
            name = luaf_getlocalname(
                (*((*(*state).top.p.offset(-(1 as isize))).value.value.object as *mut GCUnion))
                    .lcl
                    .p,
                n,
                0,
            );
        }
    } else {
        let mut pos: StkId = std::ptr::null_mut();
        name = luag_findlocal(state, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let io1: *mut TValue = &mut (*(*state).top.p).value;
            let io2: *const TValue = &mut (*pos).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
                }
    }
    return name;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setlocal(
    state: *mut State,
    ar: *const Debug,
    n: i32,
) -> *const i8 { unsafe {
    let mut pos: StkId = std::ptr::null_mut();
    let name: *const i8 = luag_findlocal(state, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let io1: *mut TValue = &mut (*pos).value;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(-1);
        }
    return name;
}}
pub unsafe extern "C" fn funcinfo(ar: *mut Debug, cl: *mut UClosure) { unsafe {
    if !(!cl.is_null() && (*cl).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
        (*ar).source = b"=[C]\0" as *const u8 as *const i8;
        (*ar).source_length = (::core::mem::size_of::<[i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64);
        (*ar).line_defined = -1;
        (*ar).last_line_defined = -1;
        (*ar).what = b"C\0" as *const u8 as *const i8;
    } else {
        let p: *const Prototype = (*cl).l.p;
        if !((*p).source).is_null() {
            (*ar).source = ((*(*p).source).contents).as_mut_ptr();
            (*ar).source_length = if (*(*p).source).short_length as i32 != 0xFF as i32 {
                (*(*p).source).short_length as u64
            } else {
                (*(*p).source).u.long_length
            };
        } else {
            (*ar).source = b"=?\0" as *const u8 as *const i8;
            (*ar).source_length = (::core::mem::size_of::<[i8; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64);
        }
        (*ar).line_defined = (*p).line_defined;
        (*ar).last_line_defined = (*p).last_line_defined;
        (*ar).what = if (*ar).line_defined == 0 {
            b"main\0" as *const u8 as *const i8
        } else {
            b"Lua\0" as *const u8 as *const i8
        };
    }
    luao_chunkid(
        ((*ar).short_src).as_mut_ptr(),
        (*ar).source,
        (*ar).source_length,
    );
}}
pub unsafe extern "C" fn nextline(
    p: *const Prototype,
    currentline: i32,
    program_counter: i32,
) -> i32 { unsafe {
    if *((*p).line_info).offset(program_counter as isize) as i32 != -(0x80 as i32) {
        return currentline + *((*p).line_info).offset(program_counter as isize) as i32;
    } else {
        return luag_getfuncline(p, program_counter);
    };
}}
pub unsafe extern "C" fn collectvalidlines(state: *mut State, f: *mut UClosure) { unsafe {
    if !(!f.is_null() && (*f).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
        (*(*state).top.p).value.set_tag (TAG_VARIANT_NIL_NIL);
        (*state).top.p = (*state).top.p.offset(1);
        } else {
        let p: *const Prototype = (*f).l.p;
        let mut currentline: i32 = (*p).line_defined;
        let table: *mut Table = luah_new(state);
        let io: *mut TValue = &mut (*(*state).top.p).value;
        let x_: *mut Table = table;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag (TAG_VARIANT_TABLE);
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
            if !((*p).line_info).is_null() {
            let mut i: i32;
            let mut v: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            v.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
            if !(*p).is_variable_arguments {
                i = 0;
            } else {
                currentline = nextline(p, currentline, 0);
                i = 1;
            }
            while i < (*p).size_line_info {
                currentline = nextline(p, currentline, i);
                luah_setint(state, table, currentline as i64, &mut v);
                i += 1;
            }
        }
    };
}}
pub unsafe extern "C" fn getfuncname(
    state: *mut State,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 5 == 0 {
        return funcnamefromcall(state, (*call_info).previous, name);
    } else {
        return std::ptr::null();
    };
}}
pub unsafe extern "C" fn auxgetinfo(
    state: *mut State,
    mut what: *const i8,
    ar: *mut Debug,
    f: *mut UClosure,
    call_info: *mut CallInfo,
) -> i32 { unsafe {
    let mut status: i32 = 1;
    while *what != 0 {
        match *what as i32 {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar).currentline =
                    if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 1 == 0 {
                        getcurrentline(call_info)
                    } else {
                        -1
                    };
            }
            117 => {
                (*ar).nups = (if f.is_null() {
                    0
                } else {
                    (*f).c.count_upvalues as i32
                }) as u8;
                if !(!f.is_null() && (*f).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
                    (*ar).is_variable_arguments = true;
                    (*ar).nparams = 0;
                } else {
                    (*ar).is_variable_arguments = (*(*f).l.p).is_variable_arguments;
                    (*ar).nparams = (*(*f).l.p).count_parameters;
                }
            }
            116 => {
                (*ar).is_tail_call = if !call_info.is_null() {
                    0 != ((*call_info).call_status as i32 & 1 << 5)
                } else {
                    false
                };
            }
            110 => {
                (*ar).namewhat = getfuncname(state, call_info, &mut (*ar).name);
                if ((*ar).namewhat).is_null() {
                    (*ar).namewhat = b"\0" as *const u8 as *const i8;
                    (*ar).name = std::ptr::null();
                }
            }
            114 => {
                if call_info.is_null() || (*call_info).call_status as i32 & 1 << 8 == 0 {
                    (*ar).ntransfer = 0;
                    (*ar).ftransfer = (*ar).ntransfer;
                } else {
                    (*ar).ftransfer = (*call_info).u2.transferinfo.ftransfer;
                    (*ar).ntransfer = (*call_info).u2.transferinfo.ntransfer;
                }
            }
            76 | 102 => {}
            _ => {
                status = 0;
            }
        }
        what = what.offset(1);
    }
    return status;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getinfo(
    state: *mut State,
    mut what: *const i8,
    ar: *mut Debug,
) -> i32 { unsafe {
    let status: i32;
    let function;
    let call_info;
    if *what as i32 == '>' as i32 {
        call_info = std::ptr::null_mut();
        function = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        what = what.offset(1);
        (*state).top.p = (*state).top.p.offset(-1);
        } else {
        call_info = (*ar).i_ci;
        function = &mut (*(*call_info).function.p).value;
    }
    match (*function).get_tag_variant() {
        TAG_VARIANT_CLOSURE_L => {
            let cl: *mut UClosure = &mut (*((*function).value.object as *mut GCUnion)).ucl;
            status = auxgetinfo(state, what, ar, cl, call_info);
            if !(strchr(what, 'f' as i32)).is_null() {
                let io1: *mut TValue = &mut (*(*state).top.p).value;
                let io2: *const TValue = function;
                (*io1).value = (*io2).value;
                (*io1).set_tag ((*io2).get_tag());
                (*state).top.p = (*state).top.p.offset(1);
                }
            if !(strchr(what, 'L' as i32)).is_null() {
                collectvalidlines(state, cl);
            }
            return status;
        },
        TAG_VARIANT_CLOSURE_C => {
            let cl: *mut UClosure = &mut (*((*function).value.object as *mut GCUnion)).ucl;
            status = auxgetinfo(state, what, ar, cl, call_info);
            if !(strchr(what, 'f' as i32)).is_null() {
                let io1: *mut TValue = &mut (*(*state).top.p).value;
                let io2: *const TValue = function;
                (*io1).value = (*io2).value;
                (*io1).set_tag ((*io2).get_tag());
                (*state).top.p = (*state).top.p.offset(1);
            }
            if !(strchr(what, 'L' as i32)).is_null() {
                collectvalidlines(state, cl);
            }
            return status;
        },
        _ => {
            let cl: *mut UClosure = std::ptr::null_mut();
            status = auxgetinfo(state, what, ar, cl, call_info);
            if !(strchr(what, 'f' as i32)).is_null() {
                let io1: *mut TValue = &mut (*(*state).top.p).value;
                let io2: *const TValue = function;
                (*io1).value = (*io2).value;
                (*io1).set_tag ((*io2).get_tag());
                (*state).top.p = (*state).top.p.offset(1);
                }
            if !(strchr(what, 'L' as i32)).is_null() {
                collectvalidlines(state, cl);
            }
            return status;
        }
    }
}}
pub unsafe extern "C" fn filterpc(program_counter: i32, jmptarget: i32) -> i32 {
    if program_counter < jmptarget {
        return -1;
    } else {
        return program_counter;
    };
}
pub unsafe extern "C" fn findsetreg(p: *const Prototype, mut lastpc: i32, reg: i32) -> i32 { unsafe {
    let mut setreg: i32 = -1;
    let mut jmptarget: i32 = 0;
    if OPMODES[(*((*p).code).offset(lastpc as isize) >> 0 & !(!(0u32) << 7) << 0)
        as u32 as usize] as i32
        & 1 << 7
        != 0
    {
        lastpc -= 1;
    }
    let mut program_counter: i32 = 0;
    while program_counter < lastpc {
        let i: u32 = *((*p).code).offset(program_counter as isize);
        let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
        let a: i32 = (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32;
        let change: i32;
        match op as u32 {
            8 => {
                let b: i32 =
                    (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                change = (a <= reg && reg <= a + b) as i32;
            }
            76 => {
                change = (reg >= a + 2) as i32;
            }
            68 | 69 => {
                change = (reg >= a) as i32;
            }
            56 => {
                let b_0: i32 =
                    (i >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
                let dest: i32 = program_counter + 1 + b_0;
                if dest <= lastpc && dest > jmptarget {
                    jmptarget = dest;
                }
                change = 0;
            }
            _ => {
                change =
                    (OPMODES[op as usize] as i32 & 1 << 3 != 0 && reg == a) as i32;
            }
        }
        if change != 0 {
            setreg = filterpc(program_counter, jmptarget);
        }
        program_counter += 1;
    }
    return setreg;
}}
pub unsafe extern "C" fn kname(
    p: *const Prototype,
    index: i32,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    let kvalue: *mut TValue = &mut *((*p).k).offset(index as isize) as *mut TValue;
    if get_tag_type((*kvalue).get_tag()) == TAG_TYPE_STRING {
        *name = ((*((*kvalue).value.object as *mut GCUnion)).ts.contents).as_mut_ptr();
        return b"constant\0" as *const u8 as *const i8;
    } else {
        *name = b"?\0" as *const u8 as *const i8;
        return std::ptr::null();
    };
}}
pub unsafe extern "C" fn basicgetobjname(
    p: *const Prototype,
    ppc: *mut i32,
    reg: i32,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    let mut program_counter: i32 = *ppc;
    *name = luaf_getlocalname(p, reg + 1, program_counter);
    if !(*name).is_null() {
        return STRING_LOCAL.as_ptr();
    }
    program_counter = findsetreg(p, program_counter, reg);
    *ppc = program_counter;
    if program_counter != -1 {
        let i: u32 = *((*p).code).offset(program_counter as isize);
        let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
        match op as u32 {
            0 => {
                let b: i32 =
                    (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                if b < (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 {
                    return basicgetobjname(p, ppc, b, name);
                }
            }
            9 => {
                *name = upvalname(
                    p,
                    (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32,
                );
                return STRING_UPVALUE.as_ptr();
            }
            3 => {
                return kname(
                    p,
                    (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32,
                    name,
                );
            }
            4 => {
                return kname(
                    p,
                    (*((*p).code).offset((program_counter + 1) as isize) >> 0 + 7
                        & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                        as i32,
                    name,
                );
            }
            _ => {}
        }
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn rname(
    p: *const Prototype,
    mut program_counter: i32,
    c: i32,
    name: *mut *const i8,
) { unsafe {
    let what: *const i8 = basicgetobjname(p, &mut program_counter, c, name);
    if !(!what.is_null() && *what as i32 == 'c' as i32) {
        *name = b"?\0" as *const u8 as *const i8;
    }
}}
pub unsafe extern "C" fn rkname(
    p: *const Prototype,
    program_counter: i32,
    i: u32,
    name: *mut *const i8,
) { unsafe {
    let c: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
    if (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 != 0 {
        kname(p, c, name);
    } else {
        rname(p, program_counter, c, name);
    };
}}
pub unsafe extern "C" fn is_environment(
    p: *const Prototype,
    mut program_counter: i32,
    i: u32,
    isup: i32,
) -> *const i8 { unsafe {
    let t: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
    let mut name: *const i8 = std::ptr::null();
    if isup != 0 {
        name = upvalname(p, t);
    } else {
        let what: *const i8 = basicgetobjname(p, &mut program_counter, t, &mut name);
        if what != STRING_LOCAL.as_ptr() && what != STRING_UPVALUE.as_ptr() {
            name = std::ptr::null();
        }
    }
    return if !name.is_null() && strcmp(name, b"_ENV\0" as *const u8 as *const i8) == 0 {
        b"global\0" as *const u8 as *const i8
    } else {
        b"field\0" as *const u8 as *const i8
    };
}}
pub unsafe extern "C" fn getobjname(
    p: *const Prototype,
    mut lastpc: i32,
    reg: i32,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    let kind: *const i8 = basicgetobjname(p, &mut lastpc, reg, name);
    if !kind.is_null() {
        return kind;
    } else if lastpc != -1 {
        let i: u32 = *((*p).code).offset(lastpc as isize);
        let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
        match op as u32 {
            11 => {
                let k: i32 =
                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                kname(p, k, name);
                return is_environment(p, lastpc, i, 1);
            }
            12 => {
                let k_0: i32 =
                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                rname(p, lastpc, k_0, name);
                return is_environment(p, lastpc, i, 0);
            }
            13 => {
                *name = b"integer index\0" as *const u8 as *const i8;
                return b"field\0" as *const u8 as *const i8;
            }
            14 => {
                let k_1: i32 =
                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                kname(p, k_1, name);
                return is_environment(p, lastpc, i, 0);
            }
            20 => {
                rkname(p, lastpc, i, name);
                return b"method\0" as *const u8 as *const i8;
            }
            _ => {}
        }
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn funcnamefromcode(
    state: *mut State,
    p: *const Prototype,
    program_counter: i32,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    let tm: u32;
    let i: u32 = *((*p).code).offset(program_counter as isize);
    match (i >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 {
        68 | 69 => {
            return getobjname(
                p,
                program_counter,
                (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32,
                name,
            );
        }
        76 => {
            *name = b"for iterator\0" as *const u8 as *const i8;
            return b"for iterator\0" as *const u8 as *const i8;
        }
        20 | 11 | 12 | 13 | 14 => {
            tm = TM_INDEX;
        }
        15 | 16 | 17 | 18 => {
            tm = TM_NEWINDEX;
        }
        46 | 47 | 48 => {
            tm = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32 as u32;
        }
        49 => {
            tm = TM_UNM;
        }
        OP_BNOT => {
            tm = TM_BNOT;
        }
        52 => {
            tm = TM_LEN;
        }
        53 => {
            tm = TM_CONCAT;
        }
        57 => {
            tm = TM_EQ;
        }
        58 | 62 | 64 => {
            tm = TM_LT;
        }
        59 | 63 | 65 => {
            tm = TM_LE;
        }
        54 | 70 => {
            tm = TM_CLOSE;
        }
        _ => return std::ptr::null(),
    }
    *name = ((*(*(*state).global).tmname[tm as usize]).contents)
        .as_mut_ptr()
        .offset(2 as isize);
    return b"metamethod\0" as *const u8 as *const i8;
}}
pub unsafe extern "C" fn funcnamefromcall(
    state: *mut State,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    if (*call_info).call_status as i32 & 1 << 3 != 0 {
        *name = b"?\0" as *const u8 as *const i8;
        return b"hook\0" as *const u8 as *const i8;
    } else if (*call_info).call_status as i32 & 1 << 7 != 0 {
        *name = b"__gc\0" as *const u8 as *const i8;
        return b"metamethod\0" as *const u8 as *const i8;
    } else if (*call_info).call_status as i32 & 1 << 1 == 0 {
        return funcnamefromcode(
            state,
            (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p,
            currentpc(call_info),
            name,
        );
    } else {
        return std::ptr::null();
    };
}}
pub unsafe extern "C" fn in_stack(call_info: *mut CallInfo, o: *const TValue) -> i32 { unsafe {
    let base: StkId = ((*call_info).function.p).offset(1 as isize);
    let mut pos: i32 = 0;
    while base.offset(pos as isize) < (*call_info).top.p {
        if o == &mut (*base.offset(pos as isize)).value as *mut TValue as *const TValue {
            return pos;
        }
        pos += 1;
    }
    return -1;
}}
pub unsafe extern "C" fn getupvalname(
    call_info: *mut CallInfo,
    o: *const TValue,
    name: *mut *const i8,
) -> *const i8 { unsafe {
    let c: *mut LClosure = &mut (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl;
    let mut i: i32;
    i = 0;
    while i < (*c).count_upvalues as i32 {
        if (**((*c).upvalues).as_mut_ptr().offset(i as isize)).v.p == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return STRING_UPVALUE.as_ptr();
        }
        i += 1;
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn formatvarinfo(
    state: *mut State,
    kind: *const i8,
    name: *const i8,
) -> *const i8 { unsafe {
    if kind.is_null() {
        return b"\0" as *const u8 as *const i8;
    } else {
        return luao_pushfstring(state, b" (%s '%s')\0" as *const u8 as *const i8, kind, name);
    };
}}
pub unsafe extern "C" fn varinfo(state: *mut State, o: *const TValue) -> *const i8 { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let mut name: *const i8 = std::ptr::null();
    let mut kind: *const i8 = std::ptr::null();
    if (*call_info).call_status as i32 & 1 << 1 == 0 {
        kind = getupvalname(call_info, o, &mut name);
        if kind.is_null() {
            let reg: i32 = in_stack(call_info, o);
            if reg >= 0 {
                kind = getobjname(
                    (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p,
                    currentpc(call_info),
                    reg,
                    &mut name,
                );
            }
        }
    }
    return formatvarinfo(state, kind, name);
}}
pub unsafe extern "C" fn typeerror(
    state: *mut State,
    o: *const TValue,
    op: *const i8,
    extra: *const i8,
) -> ! { unsafe {
    let t: *const i8 = luat_objtypename(state, o);
    luag_runerror(
        state,
        b"attempt to %s a %s value%s\0" as *const u8 as *const i8,
        op,
        t,
        extra,
    );
}}
pub unsafe extern "C" fn luag_typeerror(
    state: *mut State,
    o: *const TValue,
    op: *const i8,
) -> ! { unsafe {
    typeerror(state, o, op, varinfo(state, o));
}}
pub unsafe extern "C" fn luag_callerror(state: *mut State, o: *const TValue) -> ! { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let mut name: *const i8 = std::ptr::null();
    let kind: *const i8 = funcnamefromcall(state, call_info, &mut name);
    let extra: *const i8 = if !kind.is_null() {
        formatvarinfo(state, kind, name)
    } else {
        varinfo(state, o)
    };
    typeerror(state, o, b"call\0" as *const u8 as *const i8, extra);
}}
pub unsafe extern "C" fn luag_forerror(
    state: *mut State,
    o: *const TValue,
    what: *const i8,
) -> ! { unsafe {
    luag_runerror(
        state,
        b"bad 'for' %s (number expected, got %s)\0" as *const u8 as *const i8,
        what,
        luat_objtypename(state, o),
    );
}}
pub unsafe extern "C" fn luag_concaterror(
    state: *mut State,
    mut p1: *const TValue,
    p2: *const TValue,
) -> ! { unsafe {
    if get_tag_type((*p1).get_tag()) == TAG_TYPE_STRING || get_tag_type((*p1).get_tag()) == TAG_TYPE_NUMERIC {
        p1 = p2;
    }
    luag_typeerror(state, p1, b"concatenate\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn luag_opinterror(
    state: *mut State,
    p1: *const TValue,
    mut p2: *const TValue,
    message: *const i8,
) -> ! { unsafe {
    if get_tag_type((*p1).get_tag()) != 3 {
        p2 = p1;
    }
    luag_typeerror(state, p2, message);
}}
pub unsafe extern "C" fn luag_tointerror(
    state: *mut State,
    p1: *const TValue,
    mut p2: *const TValue,
) -> ! { unsafe {
    let mut temp: i64 = 0;
    if luav_tointegerns(p1, &mut temp, F2I::Equal) == 0 {
        p2 = p1;
    }
    luag_runerror(
        state,
        b"number%s has no integer representation\0" as *const u8 as *const i8,
        varinfo(state, p2),
    );
}}
pub unsafe extern "C" fn luag_ordererror(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
) -> ! { unsafe {
    let t1: *const i8 = luat_objtypename(state, p1);
    let t2: *const i8 = luat_objtypename(state, p2);
    if strcmp(t1, t2) == 0 {
        luag_runerror(
            state,
            b"attempt to compare two %s values\0" as *const u8 as *const i8,
            t1,
        );
    } else {
        luag_runerror(
            state,
            b"attempt to compare %s with %s\0" as *const u8 as *const i8,
            t1,
            t2,
        );
    };
}}
pub unsafe extern "C" fn luag_addinfo(
    state: *mut State,
    message: *const i8,
    src: *mut TString,
    line: i32,
) -> *const i8 { unsafe {
    let mut buffer: [i8; 60] = [0; 60];
    if !src.is_null() {
        luao_chunkid(
            buffer.as_mut_ptr(),
            ((*src).contents).as_mut_ptr(),
            if (*src).short_length as i32 != 0xFF as i32 {
                (*src).short_length as u64
            } else {
                (*src).u.long_length
            },
        );
    } else {
        buffer[0] = '?' as i8;
        buffer[1] = '\0' as i8;
    }
    return luao_pushfstring(
        state,
        b"%s:%d: %s\0" as *const u8 as *const i8,
        buffer.as_mut_ptr(),
        line,
        message,
    );
}}
pub unsafe extern "C" fn luag_errormsg(state: *mut State) -> ! { unsafe {
    if (*state).error_function != 0 {
        let error_function: StkId =
            ((*state).stack.p as *mut i8).offset((*state).error_function as isize) as StkId;
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        let io1_0: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        let io2_0: *const TValue = &mut (*error_function).value;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
            luad_callnoyield(state, (*state).top.p.offset(-(2 as isize)), 1);
    }
    luad_throw(state, 2);
}}
pub unsafe extern "C" fn luag_runerror(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> ! { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let message: *const i8;
    let mut argp: ::core::ffi::VaListImpl;
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
    argp = args.clone();
    message = luao_pushvfstring(state, fmt, argp.as_va_list());
    if (*call_info).call_status as i32 & 1 << 1 == 0 {
        luag_addinfo(
            state,
            message,
            (*(*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p).source,
            getcurrentline(call_info),
        );
        let io1: *mut TValue = &mut (*(*state).top.p.offset(-(2 as isize))).value;
        let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(-1);
        }
    luag_errormsg(state);
}}
pub unsafe extern "C" fn changedline(
    p: *const Prototype,
    old_program_counter: i32,
    newpc: i32,
) -> i32 { unsafe {
    if ((*p).line_info).is_null() {
        return 0;
    }
    if newpc - old_program_counter < 128 as i32 / 2 {
        let mut delta: i32 = 0;
        let mut program_counter: i32 = old_program_counter;
        loop {
            program_counter += 1;
            let line_info: i32 = *((*p).line_info).offset(program_counter as isize) as i32;
            if line_info == -(0x80 as i32) {
                break;
            }
            delta += line_info;
            if program_counter == newpc {
                return (delta != 0) as i32;
            }
        }
    }
    return (luag_getfuncline(p, old_program_counter) != luag_getfuncline(p, newpc)) as i32;
}}
pub unsafe extern "C" fn luag_tracecall(state: *mut State) -> i32 { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let p: *mut Prototype = (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p;
    ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
    if (*call_info).u.l.saved_program_counter == (*p).code as *const u32 {
        if (*p).is_variable_arguments {
            return 0;
        } else if (*call_info).call_status as i32 & 1 << 6 == 0 {
            luad_hookcall(state, call_info);
        }
    }
    return 1;
}}
pub unsafe extern "C" fn luag_traceexec(
    state: *mut State,
    mut program_counter: *const u32,
) -> i32 { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let mask: u8 = (*state).hook_mask as u8;
    let p: *const Prototype = (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl.p;
    if mask as i32 & (1 << 2 | 1 << 3) == 0 {
        ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 0);
        return 0;
    }
    program_counter = program_counter.offset(1);
    (*call_info).u.l.saved_program_counter = program_counter;
    let counthook: i32 = (mask as i32 & 1 << 3 != 0 && {
        (*state).hook_count -= 1;
        (*state).hook_count == 0
    }) as i32;
    if counthook != 0 {
        (*state).hook_count = (*state).base_hook_count;
    } else if mask as i32 & 1 << 2 == 0 {
        return 1;
    }
    if (*call_info).call_status as i32 & 1 << 6 != 0 {
        (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 6)) as u16;
        return 1;
    }
    if !(OPMODES[(*((*call_info).u.l.saved_program_counter).offset(-(1 as isize)) >> 0
        & !(!(0u32) << 7) << 0) as u32 as usize] as i32
        & 1 << 5
        != 0
        && (*((*call_info).u.l.saved_program_counter).offset(-(1 as isize))
            >> 0 + 7 + 8 + 1
            & !(!(0u32) << 8) << 0) as i32
            == 0)
    {
        (*state).top.p = (*call_info).top.p;
    }
    if counthook != 0 {
        luad_hook(state, 3, -1, 0, 0);
    }
    if mask as i32 & 1 << 2 != 0 {
        let old_program_counter: i32 = if (*state).old_program_counter < (*p).size_code {
            (*state).old_program_counter
        } else {
            0
        };
        let npci: i32 = program_counter.offset_from((*p).code) as i64 as i32 - 1;
        if npci <= old_program_counter || changedline(p, old_program_counter, npci) != 0 {
            let newline: i32 = luag_getfuncline(p, npci);
            luad_hook(state, 2, newline, 0, 0);
        }
        (*state).old_program_counter = npci;
    }
    if (*state).status as i32 == 1 {
        if counthook != 0 {
            (*state).hook_count = 1;
        }
        (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 6) as u16;
        luad_throw(state, 1);
    }
    return 1;
}}
pub unsafe extern "C" fn luam_growaux_(
    state: *mut State,
    block: *mut libc::c_void,
    count_elements: i32,
    total_size: *mut i32,
    element_size: i32,
    limit: i32,
    what: *const i8,
) -> *mut libc::c_void { unsafe {
    let mut size: i32 = *total_size;
    if count_elements + 1 <= size {
        return block;
    }
    if size >= limit / 2 {
        if ((size >= limit) as i32 != 0) as i32 as i64 != 0 {
            luag_runerror(
                state,
                b"too many %s (limit is %d)\0" as *const u8 as *const i8,
                what,
                limit,
            );
        }
        size = limit;
    } else {
        size *= 2;
        if size < 4 {
            size = 4;
        }
    }
    let new_block: *mut libc::c_void = luam_saferealloc_(
        state,
        block,
        (*total_size as u64).wrapping_mul(element_size as u64),
        (size as u64).wrapping_mul(element_size as u64),
    );
    *total_size = size;
    return new_block;
}}
pub unsafe extern "C" fn luam_shrinkvector_(
    state: *mut State,
    block: *mut libc::c_void,
    size: *mut i32,
    count_elements: i32,
    element_size: i32,
) -> *mut libc::c_void { unsafe {
    let old_size: u64 = (*size * element_size) as u64;
    let new_size: u64 = (count_elements * element_size) as u64;
    let new_block: *mut libc::c_void = luam_saferealloc_(state, block, old_size, new_size);
    *size = count_elements;
    return new_block;
}}
pub unsafe extern "C" fn tryagain(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void { unsafe {
    let g: *mut Global = (*state).global;
    if get_tag_type((*g).nilvalue.get_tag()) == TAG_TYPE_NIL && (*g).gcstopem == 0 {
        luac_fullgc(state, true);
        return raw_allocate(block, old_size, new_size);
    } else {
        return std::ptr::null_mut();
    };
}}
pub unsafe extern "C" fn luam_realloc_(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void { unsafe {
    let g: *mut Global = (*state).global;
    let mut new_block: *mut libc::c_void = raw_allocate(block, old_size, new_size);
    if ((new_block.is_null() && new_size > 0u64) as i32 != 0) as i32 as i64 != 0 {
        new_block = tryagain(state, block, old_size, new_size);
        if new_block.is_null() {
            return std::ptr::null_mut();
        }
    }
    (*g).gc_debt = ((*g).gc_debt as u64)
        .wrapping_add(new_size)
        .wrapping_sub(old_size) as i64;
    return new_block;
}}
pub unsafe extern "C" fn luam_saferealloc_(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void { unsafe {
    let new_block: *mut libc::c_void = luam_realloc_(state, block, old_size, new_size);
    if ((new_block.is_null() && new_size > 0u64) as i32 != 0) as i32 as i64 != 0 {
        luad_throw(state, 4);
    }
    return new_block;
}}
pub unsafe extern "C" fn luam_malloc_(
    state: *mut State,
    size: u64,
) -> *mut libc::c_void { unsafe {
    if size == 0u64 {
        return std::ptr::null_mut();
    } else {
        let g: *mut Global = (*state).global;
        let mut new_block: *mut libc::c_void =
            raw_allocate(
                std::ptr::null_mut(),
                0,
                size,
            );
        if ((new_block == std::ptr::null_mut()) as i32 != 0) as i32 as i64 != 0 {
            new_block = tryagain(state, std::ptr::null_mut(), 0, size);
            if new_block.is_null() {
                luad_throw(state, 4);
            }
        }
        (*g).gc_debt = ((*g).gc_debt as u64).wrapping_add(size) as i64 as i64;
        return new_block;
    };
}}
pub unsafe extern "C" fn intarith(
    state: *mut State,
    op: i32,
    v1: i64,
    v2: i64,
) -> i64 { unsafe {
    match op {
        0 => return (v1 as u64).wrapping_add(v2 as u64) as i64,
        1 => return (v1 as u64).wrapping_sub(v2 as u64) as i64,
        2 => return (v1 as u64).wrapping_mul(v2 as u64) as i64,
        3 => return luav_mod(state, v1, v2),
        6 => return luav_idiv(state, v1, v2),
        7 => return (v1 as u64 & v2 as u64) as i64,
        8 => return (v1 as u64 | v2 as u64) as i64,
        9 => return (v1 as u64 ^ v2 as u64) as i64,
        10 => return luav_shiftl(v1, v2),
        11 => {
            return luav_shiftl(v1, (0u64).wrapping_sub(v2 as u64) as i64);
        }
        12 => {
            return (0u64).wrapping_sub(v1 as u64) as i64;
        }
        13 => {
            return (!(0u64) ^ v1 as u64) as i64;
        }
        _ => return 0,
    };
}}
pub unsafe extern "C" fn numarith(
    state: *mut State,
    op: i32,
    v1: f64,
    v2: f64,
) -> f64 { unsafe {
    match op {
        0 => return v1 + v2,
        1 => return v1 - v2,
        2 => return v1 * v2,
        5 => return v1 / v2,
        4 => {
            return if v2 == 2.0 {
                v1 * v1
            } else {
                v1.powf(v2)
            };
        }
        6 => return (v1 / v2).floor(),
        12 => return -v1,
        3 => return luav_modf(state, v1, v2),
        _ => return 0.0,
    };
}}
pub unsafe extern "C" fn luao_rawarith(
    state: *mut State,
    op: i32,
    p1: *const TValue,
    p2: *const TValue,
    res: *mut TValue,
) -> i32 { unsafe {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i1: i64 = 0;
            let mut i2: i64 = 0;
            if (if (((*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32 as i64 != 0 {
                i1 = (*p1).value.i;
                1
            } else {
                luav_tointegerns(p1, &mut i1, F2I::Equal)
            }) != 0
                && (if (((*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32 as i64
                    != 0
                {
                    i2 = (*p2).value.i;
                    1
                } else {
                    luav_tointegerns(p2, &mut i2, F2I::Equal)
                }) != 0
            {
                (*res).value.i = intarith(state, op, i1, i2);
                (*res).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                return 1;
            } else {
                return 0;
            }
        }
        5 | 4 => {
            let mut n1: f64 = 0.0;
            let mut n2: f64 = 0.0;
            if (if (*p1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                n1 = (*p1).value.n;
                1
            } else {
                if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                    n1 = (*p1).value.i as f64;
                    1
                } else {
                    0
                }
            }) != 0
                && (if (*p2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                    n2 = (*p2).value.n;
                    1
                } else {
                    if (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                        n2 = (*p2).value.i as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
            {
                (*res).value.n = numarith(state, op, n1, n2);
                (*res).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                return 1;
            } else {
                return 0;
            }
        }
        _ => {
            let mut n1_0: f64 = 0.0;
            let mut n2_0: f64 = 0.0;
            if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                && (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
            {
                let io_1: *mut TValue = res;
                (*io_1).value.i = intarith(state, op, (*p1).value.i, (*p2).value.i);
                (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                return 1;
            } else if (if (*p1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                n1_0 = (*p1).value.n;
                1
            } else {
                if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                    n1_0 = (*p1).value.i as f64;
                    1
                } else {
                    0
                }
            }) != 0
                && (if (*p2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                    n2_0 = (*p2).value.n;
                    1
                } else {
                    if (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                        n2_0 = (*p2).value.i as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
            {
                let io_2: *mut TValue = res;
                (*io_2).value.n = numarith(state, op, n1_0, n2_0);
                (*io_2).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                return 1;
            } else {
                return 0;
            }
        }
    };
}}
pub unsafe extern "C" fn luao_arith(
    state: *mut State,
    op: i32,
    p1: *const TValue,
    p2: *const TValue,
    res: StkId,
) { unsafe {
    if luao_rawarith(state, op, p1, p2, &mut (*res).value) == 0 {
        luat_trybintm(state, p1, p2, res, (op - 0 + TM_ADD as i32) as u32);
    }
}}
pub unsafe extern "C" fn luao_hexavalue(c: i32) -> i32 { unsafe {
    if CHARACTER_TYPE[(c + 1) as usize] as i32 & 1 << 1 != 0 {
        return c - '0' as i32;
    } else {
        return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10 as i32;
    };
}}
pub unsafe extern "C" fn is_negative(s: *mut *const i8) -> bool { unsafe {
    if **s as i32 == '-' as i32 {
        *s = (*s).offset(1);
        return true;
    } else {
        if **s as i32 == '+' as i32 {
            *s = (*s).offset(1);
        }
        return false;
    }
}}
pub unsafe extern "C" fn l_str2dloc(
    s: *const i8,
    result: *mut f64,
    mode: i32,
) -> *const i8 { unsafe {
    let mut endptr: *mut i8 = std::ptr::null_mut();
    *result = if mode == 'x' as i32 {
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut i8 {
        return std::ptr::null();
    }
    while CHARACTER_TYPE[(*endptr as u8 as i32 + 1) as usize] as i32 & 1 << 3 != 0 {
        endptr = endptr.offset(1);
    }
    return if *endptr as i32 == '\0' as i32 {
        endptr
    } else {
        std::ptr::null_mut()
    };
}}
pub unsafe extern "C" fn l_str2d(s: *const i8, result: *mut f64) -> *const i8 { unsafe {
    let pmode: *const i8 = strpbrk(s, b".xXnN\0" as *const u8 as *const i8);
    let mode: i32 = if !pmode.is_null() {
        *pmode as u8 as i32 | 'A' as i32 ^ 'a' as i32
    } else {
        0
    };
    if mode == 'n' as i32 {
        return std::ptr::null();
    }
    let mut endptr: *const i8 = l_str2dloc(s, result, mode);
    if endptr.is_null() {
        let mut buffer: [i8; 201] = [0; 201];
        let pdot: *const i8 = strchr(s, '.' as i32);
        if pdot.is_null() || strlen(s) > 200 as i32 as u64 {
            return std::ptr::null();
        }
        strcpy(buffer.as_mut_ptr(), s);
        buffer[pdot.offset_from(s) as i64 as usize] = '.' as i8;
        endptr = l_str2dloc(buffer.as_mut_ptr(), result, mode);
        if !endptr.is_null() {
            endptr = s.offset(endptr.offset_from(buffer.as_mut_ptr()) as i64 as isize);
        }
    }
    return endptr;
}}
pub unsafe extern "C" fn l_str2int(mut s: *const i8, result: *mut i64) -> *const i8 { unsafe {
    let mut a: u64 = 0;
    let mut empty: i32 = 1;
    while CHARACTER_TYPE[(*s as u8 as i32 + 1) as usize] as i32 & 1 << 3 != 0 {
        s = s.offset(1);
    }
    let is_negative_: bool = is_negative(&mut s);
    if *s.offset(0 as isize) as i32 == '0' as i32
        && (*s.offset(1 as isize) as i32 == 'x' as i32
            || *s.offset(1 as isize) as i32 == 'X' as i32)
    {
        s = s.offset(2 as isize);
        while CHARACTER_TYPE[(*s as u8 as i32 + 1) as usize] as i32 & 1 << 4 != 0 {
            a = a
                .wrapping_mul(16 as i32 as u64)
                .wrapping_add(luao_hexavalue(*s as i32) as u64);
            empty = 0;
            s = s.offset(1);
        }
    } else {
        while CHARACTER_TYPE[(*s as u8 as i32 + 1) as usize] as i32 & 1 << 1 != 0 {
            let d: i32 = *s as i32 - '0' as i32;
            if a >= (9223372036854775807 as i64 / 10 as i32 as i64) as u64
                && (a > (9223372036854775807 as i64 / 10 as i32 as i64) as u64
                    || d > (9223372036854775807 as i64 % 10 as i32 as i64) as i32 + if is_negative_ { 1 } else { 0 })
            {
                return std::ptr::null();
            }
            a = a.wrapping_mul(10 as i32 as u64).wrapping_add(d as u64);
            empty = 0;
            s = s.offset(1);
        }
    }
    while CHARACTER_TYPE[(*s as u8 as i32 + 1) as usize] as i32 & 1 << 3 != 0 {
        s = s.offset(1);
    }
    if empty != 0 || *s as i32 != '\0' as i32 {
        return std::ptr::null();
    } else {
        *result = (if is_negative_ {
            (0u64).wrapping_sub(a)
        } else {
            a
        }) as i64;
        return s;
    };
}}
pub unsafe extern "C" fn luao_str2num(s: *const i8, o: *mut TValue) -> u64 { unsafe {
    let mut i: i64 = 0;
    let mut n: f64 = 0.0;
    let mut e: *const i8 = l_str2int(s, &mut i);
    if e.is_null() {
        e = l_str2d(s, &mut n);
        if e.is_null() {
            return 0u64;
        } else {
            (*o).value.n = n;
            (*o).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        }
    } else {
        (*o).value.i = i;
        (*o).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
    }
    return (e.offset_from(s) as i64 + 1) as u64;
}}
pub unsafe extern "C" fn luao_utf8esc(buffer: *mut i8, mut x: u64) -> i32 { unsafe {
    let mut n: i32 = 1;
    if x < 0x80 as i32 as u64 {
        *buffer.offset((8 - 1) as isize) = x as i8;
    } else {
        let mut mfb: u32 = 0x3f as u32;
        loop {
            let fresh9 = n;
            n = n + 1;
            *buffer.offset((8 - fresh9) as isize) =
                (0x80 as i32 as u64 | x & 0x3f as u64) as i8;
            x >>= 6;
            mfb >>= 1;
            if !(x > mfb as u64) {
                break;
            }
        }
        *buffer.offset((8 - n) as isize) = ((!mfb << 1) as u64 | x) as i8;
    }
    return n;
}}
pub unsafe extern "C" fn tostringbuff(obj: *mut TValue, buffer: *mut i8) -> u64 { unsafe {
    let mut length: u64;
    if (*obj).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        length = snprintf(
            buffer,
            44 as i32 as u64,
            b"%lld\0" as *const u8 as *const i8,
            (*obj).value.i,
        ) as u64;
    } else {
        length = snprintf(
            buffer,
            44 as i32 as u64,
            b"%.14g\0" as *const u8 as *const i8,
            (*obj).value.n,
        ) as u64;
        if *buffer.offset(strspn(buffer, b"-0123456789\0" as *const u8 as *const i8) as isize)
            as i32
            == '\0' as i32
        {
            let fresh10 = length;
            length = length + 1;
            *buffer.offset(fresh10 as isize) = '.' as i8;
            let fresh11 = length;
            length = length + 1;
            *buffer.offset(fresh11 as isize) = '0' as i8;
        }
    }
    return length;
}}
pub unsafe extern "C" fn luao_tostring(state: *mut State, obj: *mut TValue) { unsafe {
    let mut buffer: [i8; 44] = [0; 44];
    let length = tostringbuff(obj, buffer.as_mut_ptr());
    let io: *mut TValue = obj;
    let x_: *mut TString = luas_newlstr(state, buffer.as_mut_ptr(), length);
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag ((*x_).get_tag());
    (*io).set_collectable();
}}
pub unsafe extern "C" fn luao_pushvfstring(
    state: *mut State,
    mut fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 { unsafe {
    let mut buff_fs = BuffFS::new(state);
    let mut e: *const i8;
    loop {
        e = strchr(fmt, '%' as i32);
        if e.is_null() {
            break;
        }
        buff_fs.add_string(fmt, e.offset_from(fmt) as i64 as u64);
        match *e.offset(1 as isize) as i32 {
            115 => {
                let mut s: *const i8 = argp.arg::<*mut i8>();
                if s.is_null() {
                    s = b"(null)\0" as *const u8 as *const i8;
                }
                buff_fs.add_string(s, strlen(s));
            }
            99 => {
                let mut c: i8 = argp.arg::<i32>() as u8 as i8;
                buff_fs.add_string(&mut c, ::core::mem::size_of::<i8>() as u64);
            }
            100 => {
                let mut num: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                let io: *mut TValue = &mut num;
                (*io).value.i = argp.arg::<i32>() as i64;
                (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
                buff_fs.add_number(&mut num);
            }
            73 => {
                let mut num_0: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                let io_0: *mut TValue = &mut num_0;
                (*io_0).value.i = argp.arg::<i64>();
                (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                buff_fs.add_number(&mut num_0);
            }
            102 => {
                let mut num_1: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                let io_1: *mut TValue = &mut num_1;
                (*io_1).value.n = argp.arg::<f64>();
                (*io_1).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                buff_fs.add_number(&mut num_1);
            }
            112 => {
                let size = (3 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_add(8 as u64);
                let bf: *mut i8 = buff_fs.get_raw(size);
                let p: *mut libc::c_void = argp.arg::<*mut libc::c_void>();
                let length = snprintf(bf, size as u64, b"%p\0" as *const u8 as *const i8, p) as u64;
                buff_fs.add_length (length);
            }
            85 => {
                let mut bf_0: [i8; 8] = [0; 8];
                let length_0: i32 = luao_utf8esc(bf_0.as_mut_ptr(), argp.arg::<i64>() as u64);
                buff_fs.add_string(
                    bf_0.as_mut_ptr()
                        .offset(8 as isize)
                        .offset(-(length_0 as isize)),
                    length_0 as u64,
                );
            }
            37 => {
                buff_fs.add_string(b"%\0" as *const u8 as *const i8, 1 as u64);
            }
            _ => {
                luag_runerror(
                    state,
                    b"invalid option '%%%c' to 'lua_pushfstring'\0" as *const u8 as *const i8,
                    *e.offset(1 as isize) as i32,
                );
            }
        }
        fmt = e.offset(2 as isize);
    }
    buff_fs.add_string(fmt, strlen(fmt));
    buff_fs.clear();
    return ((*((*(*state).top.p.offset(-(1 as isize))).value.value.object as *mut GCUnion))
        .ts
        .contents)
        .as_mut_ptr();
}}
pub unsafe extern "C" fn luao_pushfstring(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> *const i8 { unsafe {
    let message: *const i8;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    message = luao_pushvfstring(state, fmt, argp.as_va_list());
    return message;
}}
pub unsafe extern "C" fn luao_chunkid(
    mut out: *mut i8,
    source: *const i8,
    mut source_length: u64,
) { unsafe {
    let mut bufflen: u64 = 60 as i32 as u64;
    if *source as i32 == '=' as i32 {
        if source_length <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1 as isize) as *const libc::c_void,
                source_length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
        } else {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1 as isize) as *const libc::c_void,
                bufflen
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            out = out.offset(bufflen.wrapping_sub(1 as u64) as isize);
            *out = '\0' as i8;
        }
    } else if *source as i32 == '@' as i32 {
        if source_length <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1 as isize) as *const libc::c_void,
                source_length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
        } else {
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            out = out.offset(
                (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64) as isize,
            );
            bufflen = (bufflen as u64).wrapping_sub(
                (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            ) as u64 as u64;
            memcpy(
                out as *mut libc::c_void,
                source
                    .offset(1 as isize)
                    .offset(source_length as isize)
                    .offset(-(bufflen as isize)) as *const libc::c_void,
                bufflen.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
        }
    } else {
        let nl: *const i8 = strchr(source, '\n' as i32);
        memcpy(
            out as *mut libc::c_void,
            b"[string \"\0" as *const u8 as *const i8 as *const libc::c_void,
            (::core::mem::size_of::<[i8; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        out = out.offset(
            (::core::mem::size_of::<[i8; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64) as isize,
        );
        bufflen = (bufflen as u64).wrapping_sub(
            (::core::mem::size_of::<[i8; 15]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64)
                .wrapping_add(1 as u64),
        ) as u64 as u64;
        if source_length < bufflen && nl.is_null() {
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                source_length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            out = out.offset(source_length as isize);
        } else {
            if !nl.is_null() {
                source_length = nl.offset_from(source) as i64 as u64;
            }
            if source_length > bufflen {
                source_length = bufflen;
            }
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                source_length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            out = out.offset(source_length as isize);
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            out = out.offset(
                (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64) as isize,
            );
        }
        memcpy(
            out as *mut libc::c_void,
            b"\"]\0" as *const u8 as *const i8 as *const libc::c_void,
            (::core::mem::size_of::<[i8; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64)
                .wrapping_add(1 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
    };
}}
static mut UDATA_TYPE_NAME: [i8; 9] =
    unsafe { *::core::mem::transmute::<&[u8; 9], &[i8; 9]>(b"userdata\0") };
static mut TYPE_NAMES: [*const i8; 12] = unsafe {
    [
        b"no value\0" as *const u8 as *const i8,
        b"nil\0" as *const u8 as *const i8,
        b"boolean\0" as *const u8 as *const i8,
        UDATA_TYPE_NAME.as_ptr(),
        b"number\0" as *const u8 as *const i8,
        b"string\0" as *const u8 as *const i8,
        b"table\0" as *const u8 as *const i8,
        b"function\0" as *const u8 as *const i8,
        UDATA_TYPE_NAME.as_ptr(),
        b"thread\0" as *const u8 as *const i8,
        b"upvalue\0" as *const u8 as *const i8,
        b"proto\0" as *const u8 as *const i8,
    ]
};
pub unsafe extern "C" fn luat_init(state: *mut State) { unsafe {
    static mut EVENT_NAMES: [*const i8; 25] = [
        b"__index\0" as *const u8 as *const i8,
        b"__newindex\0" as *const u8 as *const i8,
        b"__gc\0" as *const u8 as *const i8,
        b"__mode\0" as *const u8 as *const i8,
        b"__len\0" as *const u8 as *const i8,
        b"__eq\0" as *const u8 as *const i8,
        b"__add\0" as *const u8 as *const i8,
        b"__sub\0" as *const u8 as *const i8,
        b"__mul\0" as *const u8 as *const i8,
        b"__mod\0" as *const u8 as *const i8,
        b"__pow\0" as *const u8 as *const i8,
        b"__div\0" as *const u8 as *const i8,
        b"__idiv\0" as *const u8 as *const i8,
        b"__band\0" as *const u8 as *const i8,
        b"__bor\0" as *const u8 as *const i8,
        b"__bxor\0" as *const u8 as *const i8,
        b"__shl\0" as *const u8 as *const i8,
        b"__shr\0" as *const u8 as *const i8,
        b"__unm\0" as *const u8 as *const i8,
        b"__bnot\0" as *const u8 as *const i8,
        b"__lt\0" as *const u8 as *const i8,
        b"__le\0" as *const u8 as *const i8,
        b"__concat\0" as *const u8 as *const i8,
        b"__call\0" as *const u8 as *const i8,
        b"__close\0" as *const u8 as *const i8,
    ];
    let mut i: i32;
    i = 0;
    while i < TM_N as i32 {
        (*(*state).global).tmname[i as usize] = luas_new(state, EVENT_NAMES[i as usize]);
        luac_fix(
            state,
            &mut (*(*((*(*state).global).tmname).as_mut_ptr().offset(i as isize) as *mut GCUnion))
                .object,
        );
        i += 1;
    }
}}
pub unsafe extern "C" fn luat_gettm(
    events: *mut Table,
    event: u32,
    ename: *mut TString,
) -> *const TValue { unsafe {
    let tm: *const TValue = luah_getshortstr(events, ename);
    if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
        (*events).flags =
            ((*events).flags as i32 | ((1 as u32) << event as u32) as u8 as i32) as u8;
        return std::ptr::null();
    } else {
        return tm;
    };
}}
pub unsafe extern "C" fn luat_gettmbyobj(
    state: *mut State,
    o: *const TValue,
    event: u32,
) -> *const TValue { unsafe {
    let mt: *mut Table;
    match get_tag_type((*o).get_tag()) {
        5 => {
            mt = (*((*o).value.object as *mut GCUnion)).h.metatable;
        }
        7 => {
            mt = (*((*o).value.object as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*state).global).mt[(get_tag_type((*o).get_tag())) as usize];
        }
    }
    return if mt.is_null() {
        &mut (*(*state).global).nilvalue as *mut TValue as *const TValue
    } else {
        luah_getshortstr(mt, (*(*state).global).tmname[event as usize])
    };
}}
pub unsafe extern "C" fn luat_objtypename(
    state: *mut State,
    o: *const TValue,
) -> *const i8 { unsafe {
    let mut mt: *mut Table;
    if (*o).get_tag_variant() == TAG_VARIANT_TABLE && {
        mt = (*((*o).value.object as *mut GCUnion)).h.metatable;
        !mt.is_null()
    } || (*o).get_tag_variant() == TAG_VARIANT_USER && {
        mt = (*((*o).value.object as *mut GCUnion)).u.metatable;
        !mt.is_null()
    } {
        let name: *const TValue =
            luah_getshortstr(mt, luas_new(state, b"__name\0" as *const u8 as *const i8));
        if get_tag_type((*name).get_tag()) == TAG_TYPE_STRING {
            return ((*((*name).value.object as *mut GCUnion)).ts.contents).as_mut_ptr();
        }
    }
    return TYPE_NAMES[(((*o).get_tag_type()) + 1) as usize];
}}
pub unsafe extern "C" fn luat_calltm(
    state: *mut State,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    p3: *const TValue,
) { unsafe {
    let function: StkId = (*state).top.p;
    let io1: *mut TValue = &mut (*function).value;
    let io2: *const TValue = f;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).value;
    let io2_0: *const TValue = p1;
    (*io1_0).value = (*io2_0).value;
    (*io1_0).set_tag((*io2_0).get_tag());
    let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).value;
    let io2_1: *const TValue = p2;
    (*io1_1).value = (*io2_1).value;
    (*io1_1).set_tag((*io2_1).get_tag());
    let io1_2: *mut TValue = &mut (*function.offset(3 as isize)).value;
    let io2_2: *const TValue = p3;
    (*io1_2).value = (*io2_2).value;
    (*io1_2).set_tag((*io2_2).get_tag());
    (*state).top.p = function.offset(4 as isize);
    if (*(*state).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
        ccall(state, function, 0, 1);
    } else {
        luad_callnoyield(state, function, 0);
    };
}}
pub unsafe extern "C" fn luat_calltmres(
    state: *mut State,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    mut res: StkId,
) { unsafe {
    let result: i64 = (res as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
    let function: StkId = (*state).top.p;
    let io1: *mut TValue = &mut (*function).value;
    let io2: *const TValue = f;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).value;
    let io2_0: *const TValue = p1;
    (*io1_0).value = (*io2_0).value;
    (*io1_0).set_tag((*io2_0).get_tag());
    let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).value;
    let io2_1: *const TValue = p2;
    (*io1_1).value = (*io2_1).value;
    (*io1_1).set_tag((*io2_1).get_tag());
    (*state).top.p = (*state).top.p.offset(3 as isize);
    if (*(*state).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
        ccall(state, function, 1, 1);
    } else {
        luad_callnoyield(state, function, 1);
    }
    res = ((*state).stack.p as *mut i8).offset(result as isize) as StkId;
    let io1_2: *mut TValue = &mut (*res).value;
    (*state).top.p = (*state).top.p.offset(-1);
    let io2_2: *const TValue = &mut (*(*state).top.p).value;
    (*io1_2).value = (*io2_2).value;
    (*io1_2).set_tag((*io2_2).get_tag());
}}
pub unsafe extern "C" fn callbintm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    res: StkId,
    event: u32,
) -> i32 { unsafe {
    let mut tm: *const TValue = luat_gettmbyobj(state, p1, event);
    if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
        tm = luat_gettmbyobj(state, p2, event);
    }
    if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
        return 0;
    }
    luat_calltmres(state, tm, p1, p2, res);
    return 1;
}}
pub unsafe extern "C" fn luat_trybintm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    res: StkId,
    event: u32,
) { unsafe {
    if ((callbintm(state, p1, p2, res, event) == 0) as i32 != 0) as i32 as i64 != 0 {
        match event as u32 {
            13 | 14 | 15 | 16 | 17 | 19 => {
                if get_tag_type((*p1).get_tag()) == TAG_TYPE_NUMERIC && get_tag_type((*p2).get_tag()) == TAG_TYPE_NUMERIC {
                    luag_tointerror(state, p1, p2);
                } else {
                    luag_opinterror(
                        state,
                        p1,
                        p2,
                        b"perform bitwise operation on\0" as *const u8 as *const i8,
                    );
                }
            }
            _ => {
                luag_opinterror(
                    state,
                    p1,
                    p2,
                    b"perform arithmetic on\0" as *const u8 as *const i8,
                );
            }
        }
    }
}}
pub unsafe extern "C" fn luat_tryconcattm(state: *mut State) { unsafe {
    let top: StkId = (*state).top.p;
    if ((callbintm(
        state,
        &mut (*top.offset(-(2 as isize))).value,
        &mut (*top.offset(-(1 as isize))).value,
        top.offset(-(2 as isize)),
        TM_CONCAT,
    ) == 0) as i32
        != 0) as i32 as i64
        != 0
    {
        luag_concaterror(
            state,
            &mut (*top.offset(-(2 as isize))).value,
            &mut (*top.offset(-(1 as isize))).value,
        );
    }
}}
pub unsafe extern "C" fn luat_trybinassoctm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    flip: i32,
    res: StkId,
    event: u32,
) { unsafe {
    if flip != 0 {
        luat_trybintm(state, p2, p1, res, event);
    } else {
        luat_trybintm(state, p1, p2, res, event);
    };
}}
pub unsafe extern "C" fn luat_trybinitm(
    state: *mut State,
    p1: *const TValue,
    i2: i64,
    flip: i32,
    res: StkId,
    event: u32,
) { unsafe {
    let mut aux: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io: *mut TValue = &mut aux;
    (*io).value.i = i2;
    (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
    luat_trybinassoctm(state, p1, &mut aux, flip, res, event);
}}
pub unsafe extern "C" fn luat_callordertm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    event: u32,
) -> i32 { unsafe {
    if callbintm(state, p1, p2, (*state).top.p, event) != 0 {
        return !((*(*state).top.p).value.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
            || get_tag_type((*(*state).top.p).value.get_tag()) == TAG_TYPE_NIL) as i32;
    }
    luag_ordererror(state, p1, p2);
}}
pub unsafe extern "C" fn luat_callorderitm(
    state: *mut State,
    mut p1: *const TValue,
    v2: i32,
    flip: i32,
    is_float: bool,
    event: u32,
) -> i32 { unsafe {
    let mut aux: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let p2: *const TValue;
    if is_float {
        let io: *mut TValue = &mut aux;
        (*io).value.n = v2 as f64;
        (*io).set_tag (TAG_VARIANT_NUMERIC_NUMBER);
    } else {
        let io_0: *mut TValue = &mut aux;
        (*io_0).value.i = v2 as i64;
        (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
    }
    if flip != 0 {
        p2 = p1;
        p1 = &mut aux;
    } else {
        p2 = &mut aux;
    }
    return luat_callordertm(state, p1, p2, event);
}}
pub unsafe extern "C" fn luat_adjustvarargs(
    state: *mut State,
    nfixparams: i32,
    call_info: *mut CallInfo,
    p: *const Prototype,
) { unsafe {
    let mut i: i32;
    let actual: i32 = ((*state).top.p).offset_from((*call_info).function.p) as i64 as i32 - 1;
    let nextra: i32 = actual - nfixparams;
    (*call_info).u.l.count_extra_arguments = nextra;
    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64
        <= ((*p).maximum_stack_size as i32 + 1) as i64) as i32
        != 0) as i32 as i64
        != 0
    {
        luad_growstack(state, (*p).maximum_stack_size as i32 + 1, true);
    }
    let fresh12 = (*state).top.p;
    (*state).top.p = (*state).top.p.offset(1);
    let io1: *mut TValue = &mut (*fresh12).value;
    let io2: *const TValue = &mut (*(*call_info).function.p).value;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    i = 1;
    while i <= nfixparams {
        let fresh13 = (*state).top.p;
        (*state).top.p = (*state).top.p.offset(1);
        let io1_0: *mut TValue = &mut (*fresh13).value;
        let io2_0: *const TValue = &mut (*((*call_info).function.p).offset(i as isize)).value;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        (*((*call_info).function.p).offset(i as isize)).value.set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    (*call_info).function.p = ((*call_info).function.p).offset((actual + 1) as isize);
    (*call_info).top.p = ((*call_info).top.p).offset((actual + 1) as isize);
}}
pub unsafe extern "C" fn luat_getvarargs(
    state: *mut State,
    call_info: *mut CallInfo,
    mut where_0: StkId,
    mut wanted: i32,
) { unsafe {
    let mut i: i32;
    let nextra: i32 = (*call_info).u.l.count_extra_arguments;
    if wanted < 0 {
        wanted = nextra;
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= nextra as i64) as i32
            != 0) as i32 as i64
            != 0
        {
            let t__: i64 = (where_0 as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            luad_growstack(state, nextra, true);
            where_0 = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
        }
        (*state).top.p = where_0.offset(nextra as isize);
    }
    i = 0;
    while i < wanted && i < nextra {
        let io1: *mut TValue = &mut (*where_0.offset(i as isize)).value;
        let io2: *const TValue = &mut (*((*call_info).function.p)
            .offset(-(nextra as isize))
            .offset(i as isize))
        .value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        i += 1;
    }
    while i < wanted {
        (*where_0.offset(i as isize)).value.set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
}}
pub unsafe extern "C" fn luaz_fill(zio: *mut ZIO) -> i32 { unsafe {
    let mut size: u64 = 0;
    let state: *mut State = (*zio).state;
    let buffer: *const i8 = ((*zio).reader).expect("non-null function pointer")(state, (*zio).data, &mut size);
    if buffer.is_null() || size == 0u64 {
        return -1;
    }
    (*zio).n = size.wrapping_sub(1 as u64);
    (*zio).p = buffer;
    let fresh14 = (*zio).p;
    (*zio).p = ((*zio).p).offset(1);
    return *fresh14 as u8 as i32;
}}
pub unsafe extern "C" fn luaz_init(
    state: *mut State,
    zio: *mut ZIO,
    reader: ReadFunction,
    data: *mut libc::c_void,
) { unsafe {
    (*zio).state = state;
    (*zio).reader = reader;
    (*zio).data = data;
    (*zio).n = 0;
    (*zio).p = std::ptr::null();
}}
pub unsafe extern "C" fn luaz_read(zio: *mut ZIO, mut b: *mut libc::c_void, mut n: u64) -> u64 { unsafe {
    while n != 0 {
        if (*zio).n == 0u64 {
            if luaz_fill(zio) == -1 {
                return n;
            } else {
                (*zio).n = ((*zio).n).wrapping_add(1);
                (*zio).n;
                (*zio).p = ((*zio).p).offset(-1);
                (*zio).p;
            }
        }
        let m: u64 = if n <= (*zio).n { n } else { (*zio).n };
        memcpy(b, (*zio).p as *const libc::c_void, m);
        (*zio).n = ((*zio).n as u64).wrapping_sub(m) as u64 as u64;
        (*zio).p = ((*zio).p).offset(m as isize);
        b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
        n = (n as u64).wrapping_sub(m) as u64 as u64;
    }
    return 0u64;
}}
static mut CHARACTER_TYPE: [u8; 257] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x8 as i32 as u8,
    0x8 as i32 as u8,
    0x8 as i32 as u8,
    0x8 as i32 as u8,
    0x8 as i32 as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0xc as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x16 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x5 as i32 as u8,
    0x4 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x15 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x5 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0x4 as i32 as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut OPMODES: [u8; 83] = [
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IASBX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IASBX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (1 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (1 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (1 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | ISJ as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 1 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 1 << 6
        | 1 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 1 << 6
        | 1 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 1 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 0 << 6
        | 1 << 5
        | 0 << 4
        | 0 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABX as i32) as u8,
    (0 << 7
        | 1 << 6
        | 0 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 1 << 5
        | 0 << 4
        | 1 << 3
        | IABC as i32) as u8,
    (0 << 7
        | 0 << 6
        | 0 << 5
        | 0 << 4
        | 0 << 3
        | IAX as i32) as u8,
];
pub unsafe extern "C" fn getgclist(o: *mut Object) -> *mut *mut Object { unsafe {
    match (*o).get_tag() {
        TAG_VARIANT_TABLE => return &mut (*(o as *mut GCUnion)).h.gc_list,
        TAG_VARIANT_CLOSURE_L => return &mut (*(o as *mut GCUnion)).lcl.gc_list,
        TAG_VARIANT_CLOSURE_C => return &mut (*(o as *mut GCUnion)).ccl.gc_list,
        TAG_VARIANT_STATE => return &mut (*(o as *mut GCUnion)).th.gc_list,
        TAG_VARIANT_PROTOTYPE => return &mut (*(o as *mut GCUnion)).p.gc_list,
        TAG_VARIANT_USER => return &mut (*(o as *mut GCUnion)).u.gc_list,
        _ => return std::ptr::null_mut(),
    };
}}
pub unsafe extern "C" fn linkgclist_(
    o: *mut Object,
    pnext: *mut *mut Object,
    list: *mut *mut Object,
) { unsafe {
    *pnext = *list;
    *list = o;
    (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
}}
pub unsafe extern "C" fn clearkey(node: *mut Node) { unsafe {
    if is_collectable((*node).key.tag) {
        (*node).key.tag = (9 as i32 + 2) as u8;
    }
}}
pub unsafe extern "C" fn iscleared(g: *mut Global, o: *const Object) -> i32 { unsafe {
    if o.is_null() {
        return 0;
    } else if get_tag_type((*o).get_tag()) == TAG_TYPE_STRING {
        if (*o).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).object);
        }
        return 0;
    } else {
        return ((*o).get_marked() & (1 << 3 | 1 << 4)) as i32;
    };
}}
pub unsafe extern "C" fn luac_barrier_(
    state: *mut State,
    o: *mut Object,
    v: *mut Object,
) { unsafe {
    let g: *mut Global = (*state).global;
    if (*g).gcstate as i32 <= 2 {
        reallymarkobject(g, v);
        if (*o).get_marked() & 7 > 1 {
            (*v).set_marked((*v).get_marked() & !(7) | 2);
        }
    } else if (*g).gckind as i32 == 0 {
        (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
            | ((*g).current_white & (1 << 3 | 1 << 4)));
    }
}}
pub unsafe extern "C" fn luac_barrierback_(state: *mut State, o: *mut Object) { unsafe {
    let g: *mut Global = (*state).global;
    if (*o).get_marked() & 7 == 6 {
        (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
    } else {
        linkgclist_(
            &mut (*(o as *mut GCUnion)).object,
            getgclist(o),
            &mut (*g).grayagain,
        );
    }
    if (*o).get_marked() & 7 > 1 {
        (*o).set_marked((*o).get_marked() & !7 | 5);
    }
}}
pub unsafe extern "C" fn luac_fix(state: *mut State, o: *mut Object) { unsafe {
    let g: *mut Global = (*state).global;
    (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
    (*o).set_marked((*o).get_marked() & !(7) | 4);
    (*g).allgc = (*o).next;
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}}
pub unsafe extern "C" fn luac_newobjdt(
    state: *mut State,
    tag: u8,
    size: u64,
    offset: u64,
) -> *mut Object { unsafe {
    let g: *mut Global = (*state).global;
    let p: *mut i8 = luam_malloc_(state, size) as *mut i8;
    let o: *mut Object = p.offset(offset as isize) as *mut Object;
    (*o).set_marked((*g).current_white & (1 << 3 | 1 << 4));
    (*o).set_tag(tag);
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}}
pub unsafe extern "C" fn luac_newobj(
    state: *mut State,
    tag: u8,
    size: u64,
) -> *mut Object { unsafe {
    return luac_newobjdt(state, tag, size, 0u64);
}}
pub unsafe extern "C" fn reallymarkobject(g: *mut Global, o: *mut Object) { unsafe {
    let current_block_18: u64;
    match (*o).get_tag() {
        TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
            (*o).set_marked(
                (*o).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
            current_block_18 = 18317007320854588510;
        }
        TAG_VARIANT_UPVALUE => {
            let uv: *mut UpValue = &mut (*(o as *mut GCUnion)).upv;
            if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
                (*uv).set_marked((*uv).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
            } else {
                (*uv).set_marked(((*uv).get_marked() & !(1 << 3 | 1 << 4)
                    | 1 << 5) as u8);
            }
            if ((*(*uv).v.p).is_collectable())
                && (*(*(*uv).v.p).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                reallymarkobject(g, (*(*uv).v.p).value.object);
            }
            current_block_18 = 18317007320854588510;
        }
        TAG_VARIANT_USER => {
            let u: *mut User = &mut (*(o as *mut GCUnion)).u;
            if (*u).nuvalue as i32 == 0 {
                if !((*u).metatable).is_null() {
                    if (*(*u).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        reallymarkobject(g, &mut (*((*u).metatable as *mut GCUnion)).object);
                    }
                }
                (*u).set_marked((*u).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                current_block_18 = 18317007320854588510;
            } else {
                current_block_18 = 15904375183555213903;
            }
        }
        TAG_VARIANT_CLOSURE_L | TAG_VARIANT_CLOSURE_C | TAG_VARIANT_TABLE | TAG_VARIANT_STATE | TAG_VARIANT_PROTOTYPE => {
            current_block_18 = 15904375183555213903;
        }
        _ => {
            current_block_18 = 18317007320854588510;
        }
    }
    match current_block_18 {
        15904375183555213903 => {
            linkgclist_(&mut (*(o as *mut GCUnion)).object, getgclist(o), &mut (*g).gray);
        }
        _ => {}
    };
}}
pub unsafe extern "C" fn markmt(g: *mut Global) { unsafe {
    let mut i: i32;
    i = 0;
    while i < 9 as i32 {
        if !((*g).mt[i as usize]).is_null() {
            if (*(*g).mt[i as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(
                    g,
                    &mut (*(*((*g).mt).as_mut_ptr().offset(i as isize) as *mut GCUnion)).object,
                );
            }
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn markbeingfnz(g: *mut Global) -> u64 { unsafe {
    let mut count: u64 = 0;
    let mut o: *mut Object = (*g).tobefnz;
    while !o.is_null() {
        count = count.wrapping_add(1);
        if (*o).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).object);
        }
        o = (*o).next;
    }
    return count;
}}
pub unsafe extern "C" fn remarkupvals(g: *mut Global) -> i32 { unsafe {
    let mut p: *mut *mut State = &mut (*g).twups;
    let mut work: i32 = 0;
    loop {
        let thread: *mut State = *p;
        if thread.is_null() {
            break;
        }
        work += 1;
        if (*thread).get_marked() & (1 << 3 | 1 << 4) == 0
            && !((*thread).open_upvalue).is_null()
        {
            p = &mut (*thread).twups;
        } else {
            *p = (*thread).twups;
            (*thread).twups = thread;
            let mut uv: *mut UpValue = (*thread).open_upvalue;
            while !uv.is_null() {
                work += 1;
                if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
                    if ((*(*uv).v.p).is_collectable())
                        && (*(*(*uv).v.p).value.object).get_marked()
                            & (1 << 3 | 1 << 4)
                            != 0
                    {
                        reallymarkobject(g, (*(*uv).v.p).value.object);
                    }
                }
                uv = (*uv).u.open.next;
            }
        }
    }
    return work;
}}
pub unsafe extern "C" fn cleargraylists(g: *mut Global) { unsafe {
    (*g).grayagain = std::ptr::null_mut();
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = std::ptr::null_mut();
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
}}
pub unsafe extern "C" fn restartcollection(g: *mut Global) { unsafe {
    cleargraylists(g);
    if (*(*g).mainthread).get_marked() & (1 << 3 | 1 << 4) != 0 {
        reallymarkobject(g, &mut (*((*g).mainthread as *mut GCUnion)).object);
    }
    if ((*g).l_registry.is_collectable())
        && (*(*g).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        reallymarkobject(g, (*g).l_registry.value.object);
    }
    markmt(g);
    markbeingfnz(g);
}}
pub unsafe extern "C" fn genlink(g: *mut Global, o: *mut Object) { unsafe {
    if (*o).get_marked() & 7 == 5 {
        linkgclist_(
            &mut (*(o as *mut GCUnion)).object,
            getgclist(o),
            &mut (*g).grayagain,
        );
    } else if (*o).get_marked() & 7 == 6 {
        (*o).set_marked(((*o).get_marked() ^ (6 ^ 4)) as u8);
    }
}}
pub unsafe extern "C" fn traverseweakvalue(g: *mut Global, h: *mut Table) { unsafe {
    let limit: *mut Node =
    &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as u64 as isize) as *mut Node;
    let mut hasclears: i32 = ((*h).array_limit > 0u32) as i32;
    let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
    while node < limit {
        if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
            clearkey(node);
        } else {
            if is_collectable((*node).key.tag)
                && (*(*node).key.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                reallymarkobject(g, (*node).key.value.object);
            }
            if hasclears == 0
                && iscleared(
                    g,
                    if (*node).value.is_collectable() {
                        (*node).value.value.object
                    } else {
                        std::ptr::null_mut()
                    },
                ) != 0
            {
                hasclears = 1;
            }
        }
        node = node.offset(1);
    }
    if (*g).gcstate as i32 == 2 && hasclears != 0 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).object,
            &mut (*h).gc_list,
            &mut (*g).weak,
        );
    } else {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).object,
            &mut (*h).gc_list,
            &mut (*g).grayagain,
        );
    };
}}
pub unsafe extern "C" fn traverseephemeron(
    g: *mut Global,
    h: *mut Table,
    inv: i32,
) -> i32 { unsafe {
    let mut marked: i32 = 0;
    let mut hasclears: i32 = 0;
    let mut hasww: i32 = 0;
    let asize: u32 = luah_realasize(h);
    let new_size: u32 = (1 << (*h).log_size_node as i32) as u32;
    let mut i: u32 = 0;
    while i < asize {
        if ((*((*h).array).offset(i as isize)).is_collectable())
            && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            marked = 1;
            reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
        }
        i = i.wrapping_add(1);
    }
    i = 0u32;
    while i < new_size {
        let node: *mut Node = if inv != 0 {
            &mut *((*h).node).offset(new_size.wrapping_sub(1 as u32).wrapping_sub(i) as isize)
                as *mut Node
        } else {
            &mut *((*h).node).offset(i as isize) as *mut Node
        };
        if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
            clearkey(node);
        } else if iscleared(
            g,
            if is_collectable((*node).key.tag) {
                (*node).key.value.object
            } else {
                std::ptr::null_mut()
            },
        ) != 0
        {
            hasclears = 1;
            if ((*node).value.is_collectable())
                && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                hasww = 1;
            }
        } else if ((*node).value.is_collectable())
            && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            marked = 1;
            reallymarkobject(g, (*node).value.value.object);
        }
        i = i.wrapping_add(1);
    }
    if (*g).gcstate as i32 == 0 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).object,
            &mut (*h).gc_list,
            &mut (*g).grayagain,
        );
    } else if hasww != 0 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).object,
            &mut (*h).gc_list,
            &mut (*g).ephemeron,
        );
    } else if hasclears != 0 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).object,
            &mut (*h).gc_list,
            &mut (*g).allweak,
        );
    } else {
        genlink(g, &mut (*(h as *mut GCUnion)).object);
    }
    return marked;
}}
pub unsafe extern "C" fn traversestrongtable(g: *mut Global, h: *mut Table) { unsafe {
    let limit: *mut Node =
    &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as u64 as isize) as *mut Node;
    let asize: u32 = luah_realasize(h);
    let mut i: u32 = 0u32;
    while i < asize {
        if ((*((*h).array).offset(i as isize)).is_collectable())
            && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
        }
        i = i.wrapping_add(1);
    }
    let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
    while node < limit {
        if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
            clearkey(node);
        } else {
            if is_collectable((*node).key.tag)
                && (*(*node).key.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                reallymarkobject(g, (*node).key.value.object);
            }
            if ((*node).value.is_collectable())
                && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                reallymarkobject(g, (*node).value.value.object);
            }
        }
        node = node.offset(1);
    }
    genlink(g, &mut (*(h as *mut GCUnion)).object);
}}
pub unsafe extern "C" fn traversetable(g: *mut Global, h: *mut Table) -> u64 { unsafe {
    let mut weakkey: *const i8 = std::ptr::null();
    let mut weakvalue: *const i8 = std::ptr::null();
    let mode: *const TValue = if ((*h).metatable).is_null() {
        std::ptr::null()
    } else if (*(*h).metatable).flags as u32 & (1 as u32) << TM_MODE as i32 != 0 {
        std::ptr::null()
    } else {
        luat_gettm(
            (*h).metatable,
            TM_MODE,
            (*g).tmname[TM_MODE as i32 as usize],
        )
    };
    let smode: *mut TString;
    if !((*h).metatable).is_null() {
        if (*(*h).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).object);
        }
    }
    if !mode.is_null() && (*mode).get_tag_variant() == TAG_VARIANT_STRING_SHORT && {
        smode = &mut (*((*mode).value.object as *mut GCUnion)).ts as *mut TString;
        weakkey = strchr(((*smode).contents).as_mut_ptr(), 'k' as i32);
        weakvalue = strchr(((*smode).contents).as_mut_ptr(), 'v' as i32);
        !weakkey.is_null() || !weakvalue.is_null()
    } {
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h, 0);
        } else {
            linkgclist_(
                &mut (*(h as *mut GCUnion)).object,
                &mut (*h).gc_list,
                &mut (*g).allweak,
            );
        }
    } else {
        traversestrongtable(g, h);
    }
    return (1 as u32).wrapping_add((*h).array_limit).wrapping_add(
        (2
            * (if ((*h).last_free).is_null() {
                0
            } else {
                1 << (*h).log_size_node as i32
            })) as u32,
    ) as u64;
}}
pub unsafe extern "C" fn traverseudata(g: *mut Global, u: *mut User) -> i32 { unsafe {
    let mut i: i32;
    if !((*u).metatable).is_null() {
        if (*(*u).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*((*u).metatable as *mut GCUnion)).object);
        }
    }
    i = 0;
    while i < (*u).nuvalue as i32 {
        if ((*((*u).uv).as_mut_ptr().offset(i as isize)).uv.is_collectable())
            && (*(*((*u).uv).as_mut_ptr().offset(i as isize)).uv.value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            reallymarkobject(g, (*((*u).uv).as_mut_ptr().offset(i as isize)).uv.value.object);
        }
        i += 1;
    }
    genlink(g, &mut (*(u as *mut GCUnion)).object);
    return 1 + (*u).nuvalue as i32;
}}
pub unsafe extern "C" fn traverseproto(g: *mut Global, f: *mut Prototype) -> i32 { unsafe {
    let mut i: i32;
    if !((*f).source).is_null() {
        if (*(*f).source).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).object);
        }
    }
    i = 0;
    while i < (*f).size_k {
        if ((*((*f).k).offset(i as isize)).is_collectable())
            && (*(*((*f).k).offset(i as isize)).value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            reallymarkobject(g, (*((*f).k).offset(i as isize)).value.object);
        }
        i += 1;
    }
    i = 0;
    while i < (*f).size_upvalues {
        if !((*((*f).upvalues).offset(i as isize)).name).is_null() {
            if (*(*((*f).upvalues).offset(i as isize)).name).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).upvalues).offset(i as isize)).name as *mut GCUnion)).object,
                );
            }
        }
        i += 1;
    }
    i = 0;
    while i < (*f).size_p {
        if !(*((*f).p).offset(i as isize)).is_null() {
            if (**((*f).p).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4)
                != 0
            {
                reallymarkobject(g, &mut (*(*((*f).p).offset(i as isize) as *mut GCUnion)).object);
            }
        }
        i += 1;
    }
    i = 0;
    while i < (*f).size_local_variables {
        if !((*((*f).local_variables).offset(i as isize)).variable_name).is_null() {
            if (*(*((*f).local_variables).offset(i as isize)).variable_name).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).local_variables).offset(i as isize)).variable_name as *mut GCUnion)).object,
                );
            }
        }
        i += 1;
    }
    return 1 + (*f).size_k + (*f).size_upvalues + (*f).size_p + (*f).size_local_variables;
}}
pub unsafe extern "C" fn traversecclosure(g: *mut Global, cl: *mut CClosure) -> i32 { unsafe {
    let mut i: i32;
    i = 0;
    while i < (*cl).count_upvalues as i32 {
        if ((*((*cl).upvalue).as_mut_ptr().offset(i as isize)).is_collectable())
            && (*(*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value.object).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            reallymarkobject(
                g,
                (*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value.object,
            );
        }
        i += 1;
    }
    return 1 + (*cl).count_upvalues as i32;
}}
pub unsafe extern "C" fn traverselclosure(g: *mut Global, cl: *mut LClosure) -> i32 { unsafe {
    let mut i: i32;
    if !((*cl).p).is_null() {
        if (*(*cl).p).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).object);
        }
    }
    i = 0;
    while i < (*cl).count_upvalues as i32 {
        let uv: *mut UpValue = *((*cl).upvalues).as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*(uv as *mut GCUnion)).object);
            }
        }
        i += 1;
    }
    return 1 + (*cl).count_upvalues as i32;
}}
pub unsafe extern "C" fn traversethread(g: *mut Global, th: *mut State) -> i32 { unsafe {
    let mut o: StkId = (*th).stack.p;
    if (*th).get_marked() & 7 > 1 || (*g).gcstate as i32 == 0 {
        linkgclist_(
            &mut (*(th as *mut GCUnion)).object,
            &mut (*th).gc_list,
            &mut (*g).grayagain,
        );
    }
    if o.is_null() {
        return 1;
    }
    while o < (*th).top.p {
        if ((*o).value.is_collectable())
        && (*(*o).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            reallymarkobject(g, (*o).value.value.object);
        }
        o = o.offset(1);
    }
    let mut uv: *mut UpValue = (*th).open_upvalue;
    while !uv.is_null() {
        if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*(uv as *mut GCUnion)).object);
        }
        uv = (*uv).u.open.next;
    }
    if (*g).gcstate as i32 == 2 {
        if !(*g).is_emergency{
            luad_shrinkstack(th);
        }
        o = (*th).top.p;
        while o < ((*th).stack_last.p).offset(5 as isize) {
            (*o).value.set_tag (TAG_VARIANT_NIL_NIL);
            o = o.offset(1);
        }
        if !((*th).twups != th) && !((*th).open_upvalue).is_null() {
            (*th).twups = (*g).twups;
            (*g).twups = th;
        }
    }
    return 1 + ((*th).stack_last.p).offset_from((*th).stack.p) as i64 as i32;
}}
pub unsafe extern "C" fn propagateall(g: *mut Global) -> u64 { unsafe {
    let mut tot: u64 = 0;
    while !((*g).gray).is_null() {
        tot = (tot as u64).wrapping_add((*g).propagatemark()) as u64 as u64;
    }
    return tot;
}}
pub unsafe extern "C" fn convergeephemerons(g: *mut Global) { unsafe {
    let mut changed;
    let mut dir: i32 = 0;
    loop {
        let mut next: *mut Object = (*g).ephemeron;
        (*g).ephemeron = std::ptr::null_mut();
        changed = 0;
        loop {
            let w: *mut Object = next;
            if w.is_null() {
                break;
            }
            let h: *mut Table = &mut (*(w as *mut GCUnion)).h;
            next = (*h).gc_list;
            (*h).set_marked((*h).get_marked() | 1 << 5);
            if traverseephemeron(g, h, dir) != 0 {
                propagateall(g);
                changed = 1;
            }
        }
        dir = (dir == 0) as i32;
        if !(changed != 0) {
            break;
        }
    }
}}
pub unsafe extern "C" fn clearbykeys(g: *mut Global, mut l: *mut Object) { unsafe {
    while !l.is_null() {
        let h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let limit: *mut Node = &mut *((*h).node)
            .offset((1 << (*h).log_size_node as i32) as u64 as isize)
            as *mut Node;
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if iscleared(
                g,
                if is_collectable((*node).key.tag) {
                    (*node).key.value.object
                } else {
                    std::ptr::null_mut()
                },
            ) != 0
            {
                (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
            }
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                clearkey(node);
            }
            node = node.offset(1);
        }
        l = (*(l as *mut GCUnion)).h.gc_list;
    }
}}
pub unsafe extern "C" fn clearbyvalues(g: *mut Global, mut l: *mut Object, f: *mut Object) { unsafe {
    while l != f {
        let h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let limit: *mut Node = &mut *((*h).node)
        .offset((1 << (*h).log_size_node as i32) as u64 as isize)
            as *mut Node;
        let mut i: u32 = 0;
        let asize: u32 = luah_realasize(h);
        while i < asize {
            let o: *mut TValue = &mut *((*h).array).offset(i as isize) as *mut TValue;
            if iscleared(
                g,
                if (*o).is_collectable() {
                    (*o).value.object
                } else {
                    std::ptr::null_mut()
                },
            ) != 0
            {
                (*o).set_tag(TAG_VARIANT_NIL_EMPTY);
            }
            i = i.wrapping_add(1);
        }
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if iscleared(
                g,
                if (*node).value.is_collectable() {
                    (*node).value.value.object
                } else {
                    std::ptr::null_mut()
                },
            ) != 0
            {
                (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
            }
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                clearkey(node);
            }
            node = node.offset(1);
        }
        l = (*(l as *mut GCUnion)).h.gc_list;
    }
}}
pub unsafe extern "C" fn freeupval(state: *mut State, uv: *mut UpValue) { unsafe {
    if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
        luaf_unlinkupval(uv);
    }
(*state).free_memory(
        uv as *mut libc::c_void,
        ::core::mem::size_of::<UpValue>() as u64,
    );
}}
pub unsafe extern "C" fn freeobj(state: *mut State, o: *mut Object) { unsafe {
    match (*o).get_tag() {
        TAG_VARIANT_PROTOTYPE => {
            (*(&mut (*(o as *mut GCUnion)).p)).free_prototype(state);
        }
        TAG_VARIANT_UPVALUE => {
            freeupval(state, &mut (*(o as *mut GCUnion)).upv);
        }
        TAG_VARIANT_CLOSURE_L => {
            let cl: *mut LClosure = &mut (*(o as *mut GCUnion)).lcl;
            (*state).free_memory(
                cl as *mut libc::c_void,
                (32 as u64 as i32
                    + ::core::mem::size_of::<*mut TValue>() as u64 as i32
                        * (*cl).count_upvalues as i32) as u64,
            );
        }
        TAG_VARIANT_CLOSURE_C => {
            let cl_0: *mut CClosure = &mut (*(o as *mut GCUnion)).ccl;
            (*state).free_memory(
                cl_0 as *mut libc::c_void,
                (32 as u64 as i32
                    + ::core::mem::size_of::<TValue>() as u64 as i32
                        * (*cl_0).count_upvalues as i32) as u64,
            );
        }
        TAG_VARIANT_TABLE => {
            luah_free(state, &mut (*(o as *mut GCUnion)).h);
        }
        TAG_VARIANT_STATE => {
            luae_freethread(state, &mut (*(o as *mut GCUnion)).th);
        }
        TAG_VARIANT_USER => {
            let u: *mut User = &mut (*(o as *mut GCUnion)).u;
            (*state).free_memory(
                o as *mut libc::c_void,
                (if (*u).nuvalue as i32 == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<UValue>() as u64).wrapping_mul((*u).nuvalue as u64),
                    )
                })
                .wrapping_add((*u).length),
            );
        }
        TAG_VARIANT_STRING_SHORT => {
            let ts: *mut TString = &mut (*(o as *mut GCUnion)).ts;
            luas_remove(state, ts);
            (*state).free_memory(
                ts as *mut libc::c_void,
                (24 as u64).wrapping_add(
                    (((*ts).short_length as i32 + 1) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ),
            );
        }
        TAG_VARIANT_STRING_LONG => {
            let ts_0: *mut TString = &mut (*(o as *mut GCUnion)).ts;
            (*state).free_memory(
                ts_0 as *mut libc::c_void,
                (24 as u64).wrapping_add(
                    ((*ts_0).u.long_length)
                        .wrapping_add(1 as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ),
            );
        }
        _ => {}
    };
}}
pub unsafe extern "C" fn sweeptolive(
    state: *mut State,
    mut p: *mut *mut Object,
) -> *mut *mut Object { unsafe {
    let old: *mut *mut Object = p;
    loop {
        p = (*state).sweep_list(p, 1, std::ptr::null_mut());
        if !(p == old) {
            break;
        }
    }
    return p;
}}
pub unsafe extern "C" fn check_sizes(state: *mut State, g: *mut Global) { unsafe {
    if !(*g).is_emergency {
        if (*g).string_table.length < (*g).string_table.size / 4 {
            let olddebt: i64 = (*g).gc_debt;
            luas_resize(state, (*g).string_table.size / 2);
            (*g).gc_estimate = ((*g).gc_estimate as u64)
                .wrapping_add(((*g).gc_debt - olddebt) as u64) as u64
                as u64;
        }
    }
}}
pub unsafe extern "C" fn udata2finalize(g: *mut Global) -> *mut Object { unsafe {
    let o: *mut Object = (*g).tobefnz;
    (*g).tobefnz = (*o).next;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    (*o).set_marked((*o).get_marked() & !(1 << 6));
    if 3 <= (*g).gcstate as i32 && (*g).gcstate as i32 <= 6 {
        (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
            | ((*g).current_white & (1 << 3 | 1 << 4)));
    } else if (*o).get_marked() & 7 == 3 {
        (*g).firstold1 = o;
    }
    return o;
}}
pub unsafe extern "C" fn dothecall(state: *mut State, mut _ud: *mut libc::c_void) { unsafe {
    luad_callnoyield(state, (*state).top.p.offset(-(2 as isize)), 0);
}}
pub unsafe extern "C" fn gctm_function(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    let tm: *const TValue;
    let mut v: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io: *mut TValue = &mut v;
    let i_g: *mut Object = udata2finalize(g);
    (*io).value.object = i_g;
    (*io).set_tag ((*i_g).get_tag());
    (*io).set_collectable();
    tm = luat_gettmbyobj(state, &mut v, TM_GC);
    if !(get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) {
        let status: i32;
        let oldah: u8 = (*state).allow_hook;
        let oldgcstp: i32 = (*g).gcstp as i32;
        (*g).gcstp = ((*g).gcstp as i32 | 2) as u8;
        (*state).allow_hook = 0;
        let fresh15 = (*state).top.p;
        (*state).top.p = (*state).top.p.offset(1);
        let io1: *mut TValue = &mut (*fresh15).value;
        let io2: *const TValue = tm;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        let fresh16 = (*state).top.p;
        (*state).top.p = (*state).top.p.offset(1);
        let io1_0: *mut TValue = &mut (*fresh16).value;
        let io2_0: *const TValue = &mut v;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        (*(*state).call_info).call_status = ((*(*state).call_info).call_status as i32 | 1 << 7) as u16;
        status = luad_pcall(
            state,
            Some(dothecall as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            std::ptr::null_mut(),
            ((*state).top.p.offset(-(2 as isize)) as *mut i8)
                .offset_from((*state).stack.p as *mut i8) as i64,
            0,
        );
        (*(*state).call_info).call_status = ((*(*state).call_info).call_status as i32 & !(1 << 7)) as u16;
        (*state).allow_hook = oldah;
        (*g).gcstp = oldgcstp as u8;
        if ((status != 0) as i32 != 0) as i32 as i64 != 0 {
            luae_warnerror(state, b"__gc\0" as *const u8 as *const i8);
            (*state).top.p = (*state).top.p.offset(-1);
                }
    }
}}
pub unsafe extern "C" fn runafewfinalizers(state: *mut State, n: i32) -> i32 { unsafe {
    let g: *mut Global = (*state).global;
    let mut i: i32;
    i = 0;
    while i < n && !((*g).tobefnz).is_null() {
        gctm_function(state);
        i += 1;
    }
    return i;
}}
pub unsafe extern "C" fn callallpendingfinalizers(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    while !((*g).tobefnz).is_null() {
        gctm_function(state);
    }
}}
pub unsafe extern "C" fn findlast(mut p: *mut *mut Object) -> *mut *mut Object { unsafe {
    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    return p;
}}
pub unsafe extern "C" fn separatetobefnz(g: *mut Global, all: i32) { unsafe {
    let mut p: *mut *mut Object = &mut (*g).finobj;
    let mut lastnext: *mut *mut Object = findlast(&mut (*g).tobefnz);
    loop {
        let curr: *mut Object = *p;
        if !(curr != (*g).finobjold1) {
            break;
        }
        if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0 || all != 0) {
            p = &mut (*curr).next;
        } else {
            if curr == (*g).finobjsur {
                (*g).finobjsur = (*curr).next;
            }
            *p = (*curr).next;
            (*curr).next = *lastnext;
            *lastnext = curr;
            lastnext = &mut (*curr).next;
        }
    }
}}
pub unsafe extern "C" fn checkpointer(p: *mut *mut Object, o: *mut Object) { unsafe {
    if o == *p {
        *p = (*o).next;
    }
}}
pub unsafe extern "C" fn correctpointers(g: *mut Global, o: *mut Object) { unsafe {
    checkpointer(&mut (*g).survival, o);
    checkpointer(&mut (*g).old1, o);
    checkpointer(&mut (*g).reallyold, o);
    checkpointer(&mut (*g).firstold1, o);
}}
pub unsafe extern "C" fn luac_checkfinalizer(
    state: *mut State,
    o: *mut Object,
    mt: *mut Table,
) { unsafe {
    let g: *mut Global = (*state).global;
    if (*o).get_marked() & 1 << 6 != 0
        || (if mt.is_null() {
            std::ptr::null()
        } else {
            if (*mt).flags as u32 & (1 as u32) << TM_GC as i32 != 0 {
                std::ptr::null()
            } else {
                luat_gettm(mt, TM_GC, (*g).tmname[TM_GC as i32 as usize])
            }
        })
        .is_null()
        || (*g).gcstp as i32 & 4 != 0
    {
        return;
    } else {
        if 3 <= (*g).gcstate as i32 && (*g).gcstate as i32 <= 6 {
            (*o).set_marked((*o).get_marked()
                & !(1 << 5 | (1 << 3 | 1 << 4))
                | ((*g).current_white & (1 << 3 | 1 << 4)));
            if (*g).sweepgc == &mut (*o).next as *mut *mut Object {
                (*g).sweepgc = sweeptolive(state, (*g).sweepgc);
            }
        } else {
            correctpointers(g, o);
        }
        let mut p: *mut *mut Object = &mut (*g).allgc;
        while *p != o {
            p = &mut (**p).next;
        }
        *p = (*o).next;
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        (*o).set_marked(((*o).get_marked() | 1 << 6) as u8);
    };
}}
pub unsafe extern "C" fn setpause(g: *mut Global) { unsafe {
    let pause: i32 = (*g).gcpause as i32 * 4;
    let estimate: i64 = ((*g).gc_estimate).wrapping_div(100 as i32 as u64) as i64;
    let threshold: i64 = if (pause as i64) < (!(0u64) >> 1) as i64 / estimate {
        estimate * pause as i64
    } else {
        (!(0u64) >> 1) as i64
    };
    let mut debt: i64 = (((*g).totalbytes + (*g).gc_debt) as u64).wrapping_sub(threshold as u64) as i64;
    if debt > 0 {
        debt = 0;
    }
    (*g).set_debt(debt);
}}
pub unsafe extern "C" fn sweep2old(state: *mut State, mut p: *mut *mut Object) { unsafe {
    let g: *mut Global = (*state).global;
    loop {
        let curr: *mut Object = *p;
        if curr.is_null() {
            break;
        }
        if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
            *p = (*curr).next;
            freeobj(state, curr);
        } else {
            (*curr).set_marked((*curr).get_marked() & !(7) | 4);
            if (*curr).get_tag() == TAG_TYPE_STATE {
                let th: *mut State = &mut (*(curr as *mut GCUnion)).th;
                linkgclist_(
                    &mut (*(th as *mut GCUnion)).object,
                    &mut (*th).gc_list,
                    &mut (*g).grayagain,
                );
            } else if (*curr).get_tag() == TAG_TYPE_UPVALUE
                && (*(curr as *mut GCUnion)).upv.v.p
                    != &mut (*(curr as *mut GCUnion)).upv.u.value as *mut TValue
            {
                (*curr).set_marked((*curr).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
            } else {
                (*curr).set_marked((*curr).get_marked() | 1 << 5);
            }
            p = &mut (*curr).next;
        }
    }
}}
pub unsafe extern "C" fn sweepgen(
    state: *mut State,
    g: *mut Global,
    mut p: *mut *mut Object,
    limit: *mut Object,
    pfirstold1: *mut *mut Object,
) -> *mut *mut Object { unsafe {
    static mut NEXT_AGE: [u8; 7] = [
        1, 3 as u8, 3 as u8, 4 as u8, 4 as u8, 5 as u8, 6 as u8,
    ];
    let white =
        (*g).current_white & (1 << 3 | 1 << 4);
        loop {
        let curr: *mut Object = *p;
        if !(curr != limit) {
            break;
        }
        if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
            *p = (*curr).next;
            freeobj(state, curr);
        } else {
            if (*curr).get_marked() & 7 == 0 {
                let marked = (*curr).get_marked()
                    & !(1 << 5 | (1 << 3 | 1 << 4) | 7);
                (*curr).set_marked(marked | 1 | white);
            } else {
                (*curr).set_marked((*curr).get_marked() & !(7) | NEXT_AGE[((*curr).get_marked() & 7) as usize]);
                if (*curr).get_marked() & 7 == 3 && (*pfirstold1).is_null() {
                    *pfirstold1 = curr;
                }
            }
            p = &mut (*curr).next;
        }
    }
    return p;
}}
pub unsafe extern "C" fn correctgraylist(mut p: *mut *mut Object) -> *mut *mut Object { unsafe {
    let mut current_block: u64;
    loop {
        let curr: *mut Object = *p;
        if curr.is_null() {
            break;
        }
        let next: *mut *mut Object = getgclist(curr);
        if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0) {
            if (*curr).get_marked() & 7 == 5 {
                (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                (*curr).set_marked(((*curr).get_marked() ^ (5 ^ 6)) as u8);
                current_block = 11248371660297272285;
            } else if (*curr).get_tag() == TAG_TYPE_STATE {
                current_block = 11248371660297272285;
            } else {
                if (*curr).get_marked() & 7 == 6 {
                    (*curr).set_marked(((*curr).get_marked() ^ (6 ^ 4)) as u8);
                }
                (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                current_block = 6316553219439668466;
            }
            match current_block {
                6316553219439668466 => {}
                _ => {
                    p = next;
                    continue;
                }
            }
        }
        *p = *next;
    }
    return p;
}}
pub unsafe extern "C" fn correctgraylists(g: *mut Global) { unsafe {
    let mut list: *mut *mut Object = correctgraylist(&mut (*g).grayagain);
    *list = (*g).weak;
    (*g).weak = std::ptr::null_mut();
    list = correctgraylist(list);
    *list = (*g).allweak;
    (*g).allweak = std::ptr::null_mut();
    list = correctgraylist(list);
    *list = (*g).ephemeron;
    (*g).ephemeron = std::ptr::null_mut();
    correctgraylist(list);
}}
pub unsafe extern "C" fn markold(g: *mut Global, from: *mut Object, to: *mut Object) { unsafe {
    let mut p: *mut Object = from;
    while p != to {
        if (*p).get_marked() & 7 == 3 {
            (*p).set_marked((*p).get_marked() ^ (3 ^ 4));
            if (*p).get_marked() & 1 << 5 != 0 {
                reallymarkobject(g, p);
            }
        }
        p = (*p).next;
    }
}}
pub unsafe extern "C" fn finishgencycle(state: *mut State, g: *mut Global) { unsafe {
    correctgraylists(g);
    check_sizes(state, g);
    (*g).gcstate = 0;
    if !(*g).is_emergency {
        callallpendingfinalizers(state);
    }
}}
pub unsafe extern "C" fn youngcollection(state: *mut State, g: *mut Global) { unsafe {
    if !((*g).firstold1).is_null() {
        markold(g, (*g).firstold1, (*g).reallyold);
        (*g).firstold1 = std::ptr::null_mut();
    }
    markold(g, (*g).finobj, (*g).finobjrold);
    markold(g, (*g).tobefnz, std::ptr::null_mut());
    atomic(state);
    (*g).gcstate = 3 as u8;
    let mut psurvival: *mut *mut Object = sweepgen(
        state,
        g,
        &mut (*g).allgc,
        (*g).survival,
        &mut (*g).firstold1,
    );
    sweepgen(state, g, psurvival, (*g).old1, &mut (*g).firstold1);
    (*g).reallyold = (*g).old1;
    (*g).old1 = *psurvival;
    (*g).survival = (*g).allgc;
    let mut dummy: *mut Object = std::ptr::null_mut();
    psurvival = sweepgen(state, g, &mut (*g).finobj, (*g).finobjsur, &mut dummy);
    sweepgen(state, g, psurvival, (*g).finobjold1, &mut dummy);
    (*g).finobjrold = (*g).finobjold1;
    (*g).finobjold1 = *psurvival;
    (*g).finobjsur = (*g).finobj;
    sweepgen(state, g, &mut (*g).tobefnz, std::ptr::null_mut(), &mut dummy);
    finishgencycle(state, g);
}}
pub unsafe extern "C" fn atomic2gen(state: *mut State, g: *mut Global) { unsafe {
    cleargraylists(g);
    (*g).gcstate = 3 as u8;
    sweep2old(state, &mut (*g).allgc);
    (*g).survival = (*g).allgc;
    (*g).old1 = (*g).survival;
    (*g).reallyold = (*g).old1;
    (*g).firstold1 = std::ptr::null_mut();
    sweep2old(state, &mut (*g).finobj);
    (*g).finobjsur = (*g).finobj;
    (*g).finobjold1 = (*g).finobjsur;
    (*g).finobjrold = (*g).finobjold1;
    sweep2old(state, &mut (*g).tobefnz);
    (*g).gckind = 1;
    (*g).lastatomic = 0;
    (*g).gc_estimate = ((*g).totalbytes + (*g).gc_debt) as u64;
    finishgencycle(state, g);
}}
pub unsafe extern "C" fn entergen(state: *mut State, g: *mut Global) -> u64 { unsafe {
    luac_runtilstate(state, 1 << 8);
    luac_runtilstate(state, 1 << 0);
    let numobjs: u64 = atomic(state);
    atomic2gen(state, g);
    (*g).set_minor_debt();
    return numobjs;
}}
pub unsafe extern "C" fn luac_changemode(state: *mut State, newmode: i32) { unsafe {
    let g: *mut Global = (*state).global;
    if newmode != (*g).gckind as i32 {
        if newmode == 1 {
            entergen(state, g);
        } else {
            (*g).enter_incremental();
        }
    }
    (*g).lastatomic = 0;
}}
pub unsafe extern "C" fn fullgen(state: *mut State, g: *mut Global) -> u64 { unsafe {
    (*g).enter_incremental();
    return entergen(state, g);
}}
pub unsafe extern "C" fn stepgenfull(state: *mut State, g: *mut Global) { unsafe {
    let lastatomic: u64 = (*g).lastatomic;
    if (*g).gckind as i32 == 1 {
        (*g).enter_incremental();
    }
    luac_runtilstate(state, 1 << 0);
    let newatomic: u64 = atomic(state);
    if newatomic < lastatomic.wrapping_add(lastatomic >> 3) {
        atomic2gen(state, g);
        (*g).set_minor_debt();
    } else {
        (*g).gc_estimate = ((*g).totalbytes + (*g).gc_debt) as u64;
        entersweep(state);
        luac_runtilstate(state, 1 << 8);
        setpause(g);
        (*g).lastatomic = newatomic;
    };
}}
pub unsafe extern "C" fn genstep(state: *mut State, g: *mut Global) { unsafe {
    if (*g).lastatomic != 0u64 {
        stepgenfull(state, g);
    } else {
        let majorbase: u64 = (*g).gc_estimate;
        let majorinc: u64 = majorbase
            .wrapping_div(100 as i32 as u64)
            .wrapping_mul((*g).genmajormul * 4);
        if (*g).gc_debt > 0
            && ((*g).totalbytes + (*g).gc_debt) as u64 > majorbase.wrapping_add(majorinc)
        {
            let numobjs: u64 = fullgen(state, g);
            if !((((*g).totalbytes + (*g).gc_debt) as u64)
                < majorbase.wrapping_add(majorinc.wrapping_div(2 as u64)))
            {
                (*g).lastatomic = numobjs;
                setpause(g);
            }
        } else {
            youngcollection(state, g);
            (*g).set_minor_debt();
            (*g).gc_estimate = majorbase;
        }
    };
}}
pub unsafe extern "C" fn entersweep(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    (*g).gcstate = 3 as u8;
    (*g).sweepgc = sweeptolive(state, &mut (*g).allgc);
}}
pub unsafe extern "C" fn deletelist(
    state: *mut State,
    mut p: *mut Object,
    limit: *mut Object,
) { unsafe {
    while p != limit {
        let next: *mut Object = (*p).next;
        freeobj(state, p);
        p = next;
    }
}}
pub unsafe extern "C" fn luac_freeallobjects(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    (*g).gcstp = 4 as u8;
    luac_changemode(state, 0);
    separatetobefnz(g, 1);
    callallpendingfinalizers(state);
    deletelist(
        state,
        (*g).allgc,
        &mut (*((*g).mainthread as *mut GCUnion)).object,
    );
    deletelist(state, (*g).fixedgc, std::ptr::null_mut());
}}
pub unsafe extern "C" fn atomic(state: *mut State) -> u64 { unsafe {
    let g: *mut Global = (*state).global;
    let mut work: u64 = 0;
    let grayagain: *mut Object = (*g).grayagain;
    (*g).grayagain = std::ptr::null_mut();
    (*g).gcstate = 2 as u8;
    if (*state).get_marked() & (1 << 3 | 1 << 4) != 0 {
        reallymarkobject(g, &mut (*(state as *mut GCUnion)).object);
    }
    if ((*g).l_registry.is_collectable())
        && (*(*g).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        reallymarkobject(g, (*g).l_registry.value.object);
    }
    markmt(g);
    work = (work as u64).wrapping_add(propagateall(g)) as u64 as u64;
    work = (work as u64).wrapping_add(remarkupvals(g) as u64) as u64 as u64;
    work = (work as u64).wrapping_add(propagateall(g)) as u64 as u64;
    (*g).gray = grayagain;
    work = (work as u64).wrapping_add(propagateall(g)) as u64 as u64;
    convergeephemerons(g);
    clearbyvalues(g, (*g).weak, std::ptr::null_mut());
    clearbyvalues(g, (*g).allweak, std::ptr::null_mut());
    let origweak: *mut Object = (*g).weak;
    let origall: *mut Object = (*g).allweak;
    separatetobefnz(g, 0);
    work = (work as u64).wrapping_add(markbeingfnz(g)) as u64 as u64;
    work = (work as u64).wrapping_add(propagateall(g)) as u64 as u64;
    convergeephemerons(g);
    clearbykeys(g, (*g).ephemeron);
    clearbykeys(g, (*g).allweak);
    clearbyvalues(g, (*g).weak, origweak);
    clearbyvalues(g, (*g).allweak, origall);
    (*g).clear_cache();
    (*g).current_white = ((*g).current_white as i32 ^ (1 << 3 | 1 << 4)) as u8;
    return work;
}}
pub unsafe extern "C" fn sweepstep(
    state: *mut State,
    g: *mut Global,
    nextstate: i32,
    nextlist: *mut *mut Object,
) -> i32 { unsafe {
    if !((*g).sweepgc).is_null() {
        let olddebt: i64 = (*g).gc_debt;
        let mut count: i32 = 0;
        (*g).sweepgc = (*state).sweep_list((*g).sweepgc, 100 as i32, &mut count);
        (*g).gc_estimate =
            ((*g).gc_estimate as u64).wrapping_add(((*g).gc_debt - olddebt) as u64) as u64 as u64;
        return count;
    } else {
        (*g).gcstate = nextstate as u8;
        (*g).sweepgc = nextlist;
        return 0;
    };
}}
pub unsafe extern "C" fn singlestep(state: *mut State) -> u64 { unsafe {
    let g: *mut Global = (*state).global;
    let work: u64;
    (*g).gcstopem = 1;
    match (*g).gcstate as i32 {
        8 => {
            restartcollection(g);
            (*g).gcstate = 0;
            work = 1 as u64;
        }
        0 => {
            if ((*g).gray).is_null() {
                (*g).gcstate = 1;
                work = 0;
            } else {
                work = (*g).propagatemark();
            }
        }
        1 => {
            work = atomic(state);
            entersweep(state);
            (*g).gc_estimate = ((*g).totalbytes + (*g).gc_debt) as u64;
        }
        3 => {
            work = sweepstep(state, g, 4, &mut (*g).finobj) as u64;
        }
        4 => {
            work = sweepstep(state, g, 5, &mut (*g).tobefnz) as u64;
        }
        5 => {
            work = sweepstep(state, g, 6, std::ptr::null_mut()) as u64;
        }
        6 => {
            check_sizes(state, g);
            (*g).gcstate = 7 as u8;
            work = 0;
        }
        7 => {
            if !((*g).tobefnz).is_null() && !(*g).is_emergency {
                (*g).gcstopem = 0;
                work = (runafewfinalizers(state, 10 as i32) * 50 as i32) as u64;
            } else {
                (*g).gcstate = 8 as u8;
                work = 0;
            }
        }
        _ => return 0u64,
    }
    (*g).gcstopem = 0;
    return work;
}}
pub unsafe extern "C" fn luac_runtilstate(state: *mut State, statesmask: i32) { unsafe {
    let g: *mut Global = (*state).global;
    while statesmask & 1 << (*g).gcstate as i32 == 0 {
        singlestep(state);
    }
}}
pub unsafe extern "C" fn incstep(state: *mut State, g: *mut Global) { unsafe {
    let stepmul: i32 = (*g).gcstepmul as i32 * 4 | 1;
    let mut debt: i64 = ((*g).gc_debt as u64)
        .wrapping_div(::core::mem::size_of::<TValue>() as u64)
        .wrapping_mul(stepmul as u64) as i64;
    let stepsize: i64 = (if (*g).gcstepsize as u64
        <= (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(8 as u64)
            .wrapping_sub(2 as u64)
    {
        ((1 << (*g).gcstepsize as i32) as u64)
            .wrapping_div(::core::mem::size_of::<TValue>() as u64)
            .wrapping_mul(stepmul as u64)
    } else {
        (!(0u64) >> 1) as i64 as u64
    }) as i64;
    loop {
        let work: u64 = singlestep(state);
        debt = (debt as u64).wrapping_sub(work) as i64 as i64;
        if !(debt > -stepsize && (*g).gcstate as i32 != 8) {
            break;
        }
    }
    if (*g).gcstate as i32 == 8 {
        setpause(g);
    } else {
        debt = ((debt / stepmul as i64) as u64)
            .wrapping_mul(::core::mem::size_of::<TValue>() as u64) as i64;
        (*g).set_debt(debt);
    };
}}
pub unsafe extern "C" fn luac_step(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    if !((*g).gcstp as i32 == 0) {
        (*g).set_debt(-(2000 as i32) as i64);
    } else if (*g).gckind as i32 == 1 || (*g).lastatomic != 0u64 {
        genstep(state, g);
    } else {
        incstep(state, g);
    };
}}
pub unsafe extern "C" fn fullinc(state: *mut State, g: *mut Global) { unsafe {
    if (*g).gcstate as i32 <= 2 {
        entersweep(state);
    }
    luac_runtilstate(state, 1 << 8);
    luac_runtilstate(state, 1 << 0);
    (*g).gcstate = 1;
    luac_runtilstate(state, 1 << 7);
    luac_runtilstate(state, 1 << 8);
    setpause(g);
}}
pub unsafe extern "C" fn luac_fullgc(state: *mut State, is_emergency: bool) { unsafe {
    (*((*state).global)).is_emergency = is_emergency;
    if (*((*state).global)).gckind as i32 == 0 {
        fullinc(state, (*state).global);
    } else {
        fullgen(state, (*state).global);
    }
    (*((*state).global)).is_emergency = false;
}}
pub unsafe extern "C" fn luaf_newcclosure(
    state: *mut State,
    nupvals: i32,
) -> *mut CClosure { unsafe {
    let o: *mut Object = luac_newobj(
        state,
        TAG_VARIANT_CLOSURE_C,
        (32 as u64 as i32 + ::core::mem::size_of::<TValue>() as u64 as i32 * nupvals) as u64,
    );
    let c: *mut CClosure = &mut (*(o as *mut GCUnion)).ccl;
    (*c).count_upvalues = nupvals as u8;
    return c;
}}
pub unsafe extern "C" fn luaf_newlclosure(
    state: *mut State,
    mut nupvals: i32,
) -> *mut LClosure { unsafe {
    let o: *mut Object = luac_newobj(
        state,
        TAG_VARIANT_CLOSURE_L,
        (32 as u64 as i32 + ::core::mem::size_of::<*mut TValue>() as u64 as i32 * nupvals) as u64,
    );
    let c: *mut LClosure = &mut (*(o as *mut GCUnion)).lcl;
    (*c).p = std::ptr::null_mut();
    (*c).count_upvalues = nupvals as u8;
    loop {
        let fresh17 = nupvals;
        nupvals = nupvals - 1;
        if !(fresh17 != 0) {
            break;
        }
        let ref mut fresh18 = *((*c).upvalues).as_mut_ptr().offset(nupvals as isize);
        *fresh18 = std::ptr::null_mut();
    }
    return c;
}}
pub unsafe extern "C" fn luaf_initupvals(state: *mut State, cl: *mut LClosure) { unsafe {
    let mut i: i32;
    i = 0;
    while i < (*cl).count_upvalues as i32 {
        let o: *mut Object = luac_newobj(
            state,
            TAG_TYPE_UPVALUE,
            ::core::mem::size_of::<UpValue>() as u64,
        );
        let uv: *mut UpValue = &mut (*(o as *mut GCUnion)).upv;
        (*uv).v.p = &mut (*uv).u.value;
        (*(*uv).v.p).set_tag (TAG_VARIANT_NIL_NIL);
        let ref mut fresh19 = *((*cl).upvalues).as_mut_ptr().offset(i as isize);
        *fresh19 = uv;
        if (*cl).get_marked() & 1 << 5 != 0
            && (*uv).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                state,
                &mut (*(cl as *mut GCUnion)).object,
                &mut (*(uv as *mut GCUnion)).object,
            );
        } else {
        };
        i += 1;
    }
}}
pub unsafe extern "C" fn newupval(
    state: *mut State,
    level: StkId,
    previous: *mut *mut UpValue,
) -> *mut UpValue { unsafe {
    let o: *mut Object = luac_newobj(
        state,
        TAG_TYPE_UPVALUE,
        ::core::mem::size_of::<UpValue>() as u64,
    );
    let uv: *mut UpValue = &mut (*(o as *mut GCUnion)).upv;
    let next: *mut UpValue = *previous;
    (*uv).v.p = &mut (*level).value;
    (*uv).u.open.next = next;
    (*uv).u.open.previous = previous;
    if !next.is_null() {
        (*next).u.open.previous = &mut (*uv).u.open.next;
    }
    *previous = uv;
    if !((*state).twups != state) {
        (*state).twups = (*(*state).global).twups;
        (*(*state).global).twups = state;
    }
    return uv;
}}
pub unsafe extern "C" fn luaf_findupval(state: *mut State, level: StkId) -> *mut UpValue { unsafe {
    let mut pp: *mut *mut UpValue = &mut (*state).open_upvalue;
    loop {
        let p: *mut UpValue = *pp;
        if !(!p.is_null() && (*p).v.p as StkId >= level) {
            break;
        }
        if (*p).v.p as StkId == level {
            return p;
        }
        pp = &mut (*p).u.open.next;
    }
    return newupval(state, level, pp);
}}
pub unsafe extern "C" fn callclosemethod(
    state: *mut State,
    obj: *mut TValue,
    err: *mut TValue,
    yy: i32,
) { unsafe {
    let top: StkId = (*state).top.p;
    let tm: *const TValue = luat_gettmbyobj(state, obj, TM_CLOSE);
    let io1: *mut TValue = &mut (*top).value;
    let io2: *const TValue = tm;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    let io1_0: *mut TValue = &mut (*top.offset(1 as isize)).value;
    let io2_0: *const TValue = obj;
    (*io1_0).value = (*io2_0).value;
    (*io1_0).set_tag((*io2_0).get_tag());
    let io1_1: *mut TValue = &mut (*top.offset(2 as isize)).value;
    let io2_1: *const TValue = err;
    (*io1_1).value = (*io2_1).value;
    (*io1_1).set_tag((*io2_1).get_tag());
    (*state).top.p = top.offset(3 as isize);
    if yy != 0 {
        ccall(state, top, 0, 1);
    } else {
        luad_callnoyield(state, top, 0);
    };
}}
pub unsafe extern "C" fn checkclosemth(state: *mut State, level: StkId) { unsafe {
    let tm: *const TValue = luat_gettmbyobj(state, &mut (*level).value, TM_CLOSE);
    if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
        let index: i32 = level.offset_from((*(*state).call_info).function.p) as i64 as i32;
        let mut vname: *const i8 = luag_findlocal(state, (*state).call_info, index, std::ptr::null_mut());
        if vname.is_null() {
            vname = b"?\0" as *const u8 as *const i8;
        }
        luag_runerror(
            state,
            b"variable '%s' got a non-closable value\0" as *const u8 as *const i8,
            vname,
        );
    }
}}
pub unsafe extern "C" fn prepcallclosemth(
    state: *mut State,
    level: StkId,
    status: i32,
    yy: i32,
) { unsafe {
    let uv: *mut TValue = &mut (*level).value;
    let errobj: *mut TValue;
    if status == -1 {
        errobj = &mut (*(*state).global).nilvalue;
    } else {
        errobj = &mut (*level.offset(1 as isize)).value;
        (*state).set_error_object(status, level.offset(1 as isize));
    }
    callclosemethod(state, uv, errobj, yy);
}}
pub unsafe extern "C" fn luaf_newtbcupval(state: *mut State, level: StkId) { unsafe {
    if (*level).value.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
        || get_tag_type((*level).value.get_tag()) == TAG_TYPE_NIL
    {
        return;
    }
    checkclosemth(state, level);
    while level.offset_from((*state).tbc_list.p) as i64 as u32 as u64
        > ((256 as u64)
            << (::core::mem::size_of::<u16>() as u64)
                .wrapping_sub(1 as u64)
                .wrapping_mul(8 as u64))
        .wrapping_sub(1 as u64)
    {
        (*state).tbc_list.p = ((*state).tbc_list.p).offset(
            ((256 as u64)
                << (::core::mem::size_of::<u16>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(8 as u64))
            .wrapping_sub(1 as u64) as isize,
        );
        (*(*state).tbc_list.p).tbc_list.delta = 0;
    }
    (*level).tbc_list.delta = level.offset_from((*state).tbc_list.p) as i64 as u16;
    (*state).tbc_list.p = level;
}}
pub unsafe extern "C" fn luaf_unlinkupval(uv: *mut UpValue) { unsafe {
    *(*uv).u.open.previous = (*uv).u.open.next;
    if !((*uv).u.open.next).is_null() {
        (*(*uv).u.open.next).u.open.previous = (*uv).u.open.previous;
    }
}}
pub unsafe extern "C" fn luaf_closeupval(state: *mut State, level: StkId) { unsafe {
    loop {
        let uv: *mut UpValue = (*state).open_upvalue;
        let upl: StkId;
        if !(!uv.is_null() && {
            upl = (*uv).v.p as StkId;
            upl >= level
        }) {
            break;
        }
        let slot: *mut TValue = &mut (*uv).u.value;
        luaf_unlinkupval(uv);
        let io1: *mut TValue = slot;
        let io2: *const TValue = (*uv).v.p;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
        (*uv).v.p = slot;
        if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
            (*uv).set_marked((*uv).get_marked() | 1 << 5);
            if (*slot).is_collectable() {
                if (*uv).get_marked() & 1 << 5 != 0
                    && (*(*slot).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(uv as *mut GCUnion)).object,
                        &mut (*((*slot).value.object as *mut GCUnion)).object,
                    );
                } else {
                };
            } else {
            };
        }
    }
}}
pub unsafe extern "C" fn poptbclist(state: *mut State) { unsafe {
    let mut tbc: StkId = (*state).tbc_list.p;
    tbc = tbc.offset(-((*tbc).tbc_list.delta as i32 as isize));
    while tbc > (*state).stack.p && (*tbc).tbc_list.delta as i32 == 0 {
        tbc = tbc.offset(
            -(((256 as u64)
                << (::core::mem::size_of::<u16>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(8 as u64))
            .wrapping_sub(1 as u64) as isize),
        );
    }
    (*state).tbc_list.p = tbc;
}}
pub unsafe extern "C" fn luaf_close(
    state: *mut State,
    mut level: StkId,
    status: i32,
    yy: i32,
) -> StkId { unsafe {
    let levelrel: i64 = (level as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
    luaf_closeupval(state, level);
    while (*state).tbc_list.p >= level {
        let tbc: StkId = (*state).tbc_list.p;
        poptbclist(state);
        prepcallclosemth(state, tbc, status, yy);
        level = ((*state).stack.p as *mut i8).offset(levelrel as isize) as StkId;
    }
    return level;
}}
pub unsafe extern "C" fn luaf_newproto(state: *mut State) -> *mut Prototype { unsafe {
    let o: *mut Object = luac_newobj(
        state,
        TAG_TYPE_PROTOTYPE,
        ::core::mem::size_of::<Prototype>() as u64,
    );
    let f: *mut Prototype = &mut (*(o as *mut GCUnion)).p;
    (*f).k = std::ptr::null_mut();
    (*f).size_k = 0;
    (*f).p = std::ptr::null_mut();
    (*f).size_p = 0;
    (*f).code = std::ptr::null_mut();
    (*f).size_code = 0;
    (*f).line_info = std::ptr::null_mut();
    (*f).size_line_info = 0;
    (*f).absolute_line_info = std::ptr::null_mut();
    (*f).size_absolute_line_info = 0;
    (*f).upvalues = std::ptr::null_mut();
    (*f).size_upvalues = 0;
    (*f).count_parameters = 0;
    (*f).is_variable_arguments = false;
    (*f).maximum_stack_size = 0;
    (*f).local_variables = std::ptr::null_mut();
    (*f).size_local_variables = 0;
    (*f).line_defined = 0;
    (*f).last_line_defined = 0;
    (*f).source = std::ptr::null_mut();
    return f;
}}
pub unsafe extern "C" fn luaf_getlocalname(
    f: *const Prototype,
    mut local_number: i32,
    program_counter: i32,
) -> *const i8 { unsafe {
    let mut i: i32;
    i = 0;
    while i < (*f).size_local_variables && (*((*f).local_variables).offset(i as isize)).start_program_counter <= program_counter {
        if program_counter < (*((*f).local_variables).offset(i as isize)).end_program_counter {
            local_number -= 1;
            if local_number == 0 {
                return ((*(*((*f).local_variables).offset(i as isize)).variable_name).contents).as_mut_ptr();
            }
        }
        i += 1;
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn luas_eqlngstr(a: *mut TString, b: *mut TString) -> i32 { unsafe {
    let length: u64 = (*a).u.long_length;
    return (a == b
        || length == (*b).u.long_length
            && memcmp(
                ((*a).contents).as_mut_ptr() as *const libc::c_void,
                ((*b).contents).as_mut_ptr() as *const libc::c_void,
                length,
            ) == 0) as i32;
}}
pub unsafe extern "C" fn luas_hash(str: *const i8, mut l: u64, seed: u32) -> u32 { unsafe {
    let mut h: u32 = seed ^ l as u32;
    while l > 0u64 {
        h ^= (h << 5)
            .wrapping_add(h >> 2)
            .wrapping_add(*str.offset(l.wrapping_sub(1 as u64) as isize) as u8 as u32);
        l = l.wrapping_sub(1);
    }
    return h;
}}
pub unsafe extern "C" fn luas_hashlongstr(ts: *mut TString) -> u32 { unsafe {
    if (*ts).extra as i32 == 0 {
        let length: u64 = (*ts).u.long_length;
        (*ts).hash = luas_hash(((*ts).contents).as_mut_ptr(), length, (*ts).hash);
        (*ts).extra = 1;
    }
    return (*ts).hash;
}}
pub unsafe extern "C" fn tablerehash(vect: *mut *mut TString, old_size: i32, new_size: i32) { unsafe {
    let mut i: i32;
    i = old_size;
    while i < new_size {
        let ref mut fresh20 = *vect.offset(i as isize);
        *fresh20 = std::ptr::null_mut();
        i += 1;
    }
    i = 0;
    while i < old_size {
        let mut p: *mut TString = *vect.offset(i as isize);
        let ref mut fresh21 = *vect.offset(i as isize);
        *fresh21 = std::ptr::null_mut();
        while !p.is_null() {
            let hash_next: *mut TString = (*p).u.hash_next;
            let h: u32 = ((*p).hash & (new_size - 1) as u32) as i32 as u32;
            (*p).u.hash_next = *vect.offset(h as isize);
            let ref mut fresh22 = *vect.offset(h as isize);
            *fresh22 = p;
            p = hash_next;
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn luas_resize(state: *mut State, new_size: i32) { unsafe {
    let tb: *mut StringTable = &mut (*(*state).global).string_table;
    let old_size: i32 = (*tb).size;
    if new_size < old_size {
        tablerehash((*tb).hash, old_size, new_size);
    }
    let newvect: *mut *mut TString = luam_realloc_(
        state,
        (*tb).hash as *mut libc::c_void,
        (old_size as u64).wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
        (new_size as u64).wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
    ) as *mut *mut TString;
    if ((newvect == std::ptr::null_mut() as *mut *mut TString) as i32 != 0) as i32 as i64 != 0
    {
        if new_size < old_size {
            tablerehash((*tb).hash, new_size, old_size);
        }
    } else {
        (*tb).hash = newvect;
        (*tb).size = new_size;
        if new_size > old_size {
            tablerehash(newvect, old_size, new_size);
        }
    };
}}
pub unsafe extern "C" fn luas_init(state: *mut State) { unsafe {
    let g: *mut Global = (*state).global;
    let tb: *mut StringTable = &mut (*(*state).global).string_table;
    (*tb).hash = luam_malloc_(
        state,
        (128 as i32 as u64).wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
    ) as *mut *mut TString;
    tablerehash((*tb).hash, 0, 128 as i32);
    (*tb).size = 128 as i32;
    (*g).memerrmsg = luas_newlstr(
        state,
        b"not enough memory\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 18]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64),
    );
    luac_fix(state, &mut (*((*g).memerrmsg as *mut GCUnion)).object);
    let mut i: i32 = 0;
    while i < 53 as i32 {
        let mut j: i32 = 0;
        while j < 2 {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1;
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn createstrobj(
    state: *mut State,
    l: u64,
    tag: u8,
    h: u32,
) -> *mut TString { unsafe {
    let totalsize: u64 = (24 as u64).wrapping_add(
        l.wrapping_add(1 as u64)
        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    let o: *mut Object = luac_newobj(state, tag, totalsize);
    let ts: *mut TString = &mut (*(o as *mut GCUnion)).ts;
    (*ts).hash = h;
    (*ts).extra = 0;
    *((*ts).contents).as_mut_ptr().offset(l as isize) = '\0' as i8;
    return ts;
}}
pub unsafe extern "C" fn luas_createlngstrobj(state: *mut State, length: u64) -> *mut TString { unsafe {
    let ret: *mut TString =
        createstrobj(state, length, TAG_VARIANT_STRING_LONG, (*(*state).global).seed);
    (*ret).u.long_length = length;
    (*ret).short_length = 0xFF;
    return ret;
}}
pub unsafe extern "C" fn luas_remove(state: *mut State, ts: *mut TString) { unsafe {
    let tb: *mut StringTable = &mut (*(*state).global).string_table;
    let mut p: *mut *mut TString = &mut *((*tb).hash)
        .offset(((*ts).hash & ((*tb).size - 1) as u32) as i32 as isize)
        as *mut *mut TString;
    while *p != ts {
        p = &mut (**p).u.hash_next;
    }
    *p = (**p).u.hash_next;
    (*tb).length -= 1;
    (*tb).length;
}}
pub unsafe extern "C" fn growstrtab(state: *mut State, tb: *mut StringTable) { unsafe {
    if (*tb).length == 0x7FFFFFF {
        luac_fullgc(state, true);
        if (*tb).length == 0x7FFFFFF {
            luad_throw(state, 4);
        }
    }
    if (*tb).size
        <= (if 0x7FFFFFF <= (!(0u64)).wrapping_div(::core::mem::size_of::<*mut TString>() as u64)
        {
            0x7FFFFFF
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<*mut TString>() as u64) as u32
        }) as i32
            / 2
    {
        luas_resize(state, (*tb).size * 2);
    }
}}
pub unsafe extern "C" fn internshrstr(
    state: *mut State,
    str: *const i8,
    l: u64,
) -> *mut TString { unsafe {
    let g: *mut Global = (*state).global;
    let tb: *mut StringTable = &mut (*g).string_table;
    let h: u32 = luas_hash(str, l, (*g).seed);
    let mut list: *mut *mut TString = &mut *((*tb).hash)
    .offset((h & ((*tb).size - 1) as u32) as i32 as isize)
    as *mut *mut TString;
    let mut ts: *mut TString = *list;
    while !ts.is_null() {
        if l == (*ts).short_length as u64
            && memcmp(
                str as *const libc::c_void,
                ((*ts).contents).as_mut_ptr() as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            ) == 0
        {
            if (*ts).get_marked() & ((*g).current_white ^ (1 << 3 | 1 << 4))
                != 0
            {
                (*ts).set_marked((*ts).get_marked() ^ (1 << 3 | 1 << 4));
            }
            return ts;
        }
        ts = (*ts).u.hash_next;
    }
    if (*tb).length >= (*tb).size {
        growstrtab(state, tb);
        list = &mut *((*tb).hash).offset((h & ((*tb).size - 1) as u32) as i32 as isize)
            as *mut *mut TString;
    }
    ts = createstrobj(state, l, TAG_VARIANT_STRING_SHORT, h);
    (*ts).short_length = l as u8;
    memcpy(
        ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    (*ts).u.hash_next = *list;
    *list = ts;
    (*tb).length += 1;
    (*tb).length;
    return ts;
}}
pub unsafe extern "C" fn luas_newlstr(
    state: *mut State,
    str: *const i8,
    l: u64,
) -> *mut TString { unsafe {
    if l <= 40 as i32 as u64 {
        return internshrstr(state, str, l);
    } else {
        if ((l.wrapping_mul(::core::mem::size_of::<i8>() as u64)
        >= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
                !(0u64)
            } else {
                9223372036854775807 as i64 as u64
            })
            .wrapping_sub(::core::mem::size_of::<TString>() as u64)) as i32
            != 0) as i32 as i64
            != 0
        {
            (*state).too_big();
        }
        let ts: *mut TString = luas_createlngstrobj(state, l);
        memcpy(
            ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
            str as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        return ts;
    };
}}
pub unsafe extern "C" fn luas_new(state: *mut State, str: *const i8) -> *mut TString { unsafe {
    let i: u32 = ((str as u64
        & (2147483647 as i32 as u32)
            .wrapping_mul(2 as u32)
            .wrapping_add(1 as u32) as u64) as u32)
        .wrapping_rem(53 as i32 as u32);
    let p: *mut *mut TString = ((*(*state).global).strcache[i as usize]).as_mut_ptr();
    let mut j: i32 = 0;
    while j < 2 {
        if strcmp(str, ((**p.offset(j as isize)).contents).as_mut_ptr()) == 0 {
            return *p.offset(j as isize);
        }
        j += 1;
    }
    j = 2 - 1;
    while j > 0 {
        let ref mut fresh23 = *p.offset(j as isize);
        *fresh23 = *p.offset((j - 1) as isize);
        j -= 1;
    }
    let ref mut fresh24 = *p.offset(0 as isize);
    *fresh24 = luas_newlstr(state, str, strlen(str));
    return *p.offset(0 as isize);
}}
pub unsafe extern "C" fn luas_newudata(
    state: *mut State,
    s: u64,
    nuvalue: i32,
) -> *mut User { unsafe {
    let mut i: i32;
    if ((s
        > (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
            !(0u64)
        } else {
            9223372036854775807 as i64 as u64
        })
        .wrapping_sub(if nuvalue == 0 {
            32 as u64
        } else {
            (40 as u64).wrapping_add(
                (::core::mem::size_of::<UValue>() as u64).wrapping_mul(nuvalue as u64),
            )
        })) as i32
        != 0) as i32 as i64
        != 0
    {
        (*state).too_big();
    }
    let o: *mut Object = luac_newobj(
        state,
        TAG_TYPE_USER,
        (if nuvalue == 0 {
            32 as u64
        } else {
            (40 as u64).wrapping_add(
                (::core::mem::size_of::<UValue>() as u64).wrapping_mul(nuvalue as u64),
            )
        })
        .wrapping_add(s),
    );
    let u: *mut User = &mut (*(o as *mut GCUnion)).u;
    (*u).length = s;
    (*u).nuvalue = nuvalue as u16;
    (*u).metatable = std::ptr::null_mut();
    i = 0;
    while i < nuvalue {
        (*((*u).uv).as_mut_ptr().offset(i as isize)).uv.set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    return u;
}}
pub unsafe extern "C" fn error(load_state: *mut LoadState, why: *const i8) -> ! { unsafe {
    luao_pushfstring(
        (*load_state).state,
        b"%s: bad binary format (%s)\0" as *const u8 as *const i8,
        (*load_state).name,
        why,
    );
    luad_throw((*load_state).state, 3);
}}
pub unsafe extern "C" fn load_block(load_state: *mut LoadState, b: *mut libc::c_void, size: u64) { unsafe {
    if luaz_read((*load_state).zio, b, size) != 0u64 {
        error(load_state, b"truncated chunk\0" as *const u8 as *const i8);
    }
}}
pub unsafe extern "C" fn load_byte(load_state: *mut LoadState) -> u8 { unsafe {
    let fresh25 = (*(*load_state).zio).n;
    (*(*load_state).zio).n = ((*(*load_state).zio).n).wrapping_sub(1);
    let b: i32 = if fresh25 > 0u64 {
        let fresh26 = (*(*load_state).zio).p;
        (*(*load_state).zio).p = ((*(*load_state).zio).p).offset(1);
        *fresh26 as u8 as i32
    } else {
        luaz_fill((*load_state).zio)
    };
    if b == -1 {
        error(load_state, b"truncated chunk\0" as *const u8 as *const i8);
    }
    return b as u8;
}}
pub unsafe extern "C" fn load_unsigned(load_state: *mut LoadState, mut limit: u64) -> u64 { unsafe {
    let mut x: u64 = 0;
    limit >>= 7;
    loop {
        let b: i32 = load_byte(load_state) as i32;
        if x >= limit {
            error(load_state, b"integer overflow\0" as *const u8 as *const i8);
        }
        x = x << 7 | (b & 0x7f as i32) as u64;
        if !(b & 0x80 as i32 == 0) {
            break;
        }
    }
    return x;
}}
pub unsafe extern "C" fn load_size(load_state: *mut LoadState) -> u64 { unsafe {
    return load_unsigned(load_state, !(0u64));
}}
pub unsafe extern "C" fn load_int(load_state: *mut LoadState) -> i32 { unsafe {
    return load_unsigned(load_state, 2147483647 as i32 as u64) as i32;
}}
pub unsafe extern "C" fn load_number(load_state: *mut LoadState) -> f64 { unsafe {
    let mut x: f64 = 0.0;
    load_block(
        load_state,
        &mut x as *mut f64 as *mut libc::c_void,
        (1 as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
    );
    return x;
}}
pub unsafe extern "C" fn load_integer(load_state: *mut LoadState) -> i64 { unsafe {
    let mut x: i64 = 0;
    load_block(
        load_state,
        &mut x as *mut i64 as *mut libc::c_void,
        (1 as u64).wrapping_mul(::core::mem::size_of::<i64>() as u64),
    );
    return x;
}}
pub unsafe extern "C" fn load_string_n(load_state: *mut LoadState, p: *mut Prototype) -> *mut TString { unsafe {
    let state: *mut State = (*load_state).state;
    let ts: *mut TString;
    let mut size: u64 = load_size(load_state);
    if size == 0u64 {
        return std::ptr::null_mut();
    } else {
        size = size.wrapping_sub(1);
        if size <= 40 as i32 as u64 {
            let mut buffer: [i8; 40] = [0; 40];
            load_block(
                load_state,
                buffer.as_mut_ptr() as *mut libc::c_void,
                size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            ts = luas_newlstr(state, buffer.as_mut_ptr(), size);
        } else {
            ts = luas_createlngstrobj(state, size);
            let io: *mut TValue = &mut (*(*state).top.p).value;
            let x_: *mut TString = ts;
            (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
            (*io).set_tag ((*x_).get_tag());
            (*io).set_collectable();
            luad_inctop(state);
            load_block(
                load_state,
                ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
                size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            (*state).top.p = (*state).top.p.offset(-1);
                }
    }
    if (*p).get_marked() & 1 << 5 != 0
        && (*ts).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            state,
            &mut (*(p as *mut GCUnion)).object,
            &mut (*(ts as *mut GCUnion)).object,
        );
    } else {
    };
    return ts;
}}
pub unsafe extern "C" fn load_string(load_state: *mut LoadState, p: *mut Prototype) -> *mut TString { unsafe {
    let st: *mut TString = load_string_n(load_state, p);
    if st.is_null() {
        error(
            load_state,
            b"bad format for constant string\0" as *const u8 as *const i8,
        );
    }
    return st;
}}
pub unsafe extern "C" fn load_code(load_state: *mut LoadState, f: *mut Prototype) { unsafe {
    let n: i32 = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<u32>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).code = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
    ) as *mut u32;
    (*f).size_code = n;
    load_block(
        load_state,
        (*f).code as *mut libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
}}
pub unsafe extern "C" fn load_constants(load_state: *mut LoadState, f: *mut Prototype) { unsafe {
    let mut i: i32;
    let n: i32 = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).k = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
    ) as *mut TValue;
    (*f).size_k = n;
    i = 0;
    while i < n {
        (*((*f).k).offset(i as isize)).set_tag (TAG_VARIANT_NIL_NIL);
        i += 1;
    }
    i = 0;
    while i < n {
        let o: *mut TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let t: i32 = load_byte(load_state) as i32;
        match t {
            0 => {
                (*o).set_tag (TAG_VARIANT_NIL_NIL);
            }
            1 => {
                (*o).set_tag(TAG_VARIANT_BOOLEAN_FALSE);
            }
            17 => {
                (*o).set_tag(TAG_VARIANT_BOOLEAN_TRUE);
            }
            19 => {
                let io: *mut TValue = o;
                (*io).value.n = load_number(load_state);
                (*io).set_tag (TAG_VARIANT_NUMERIC_NUMBER);
            }
            3 => {
                let io_0: *mut TValue = o;
                (*io_0).value.i = load_integer(load_state);
                (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            }
            4 | 20 => {
                let io_1: *mut TValue = o;
                let x_: *mut TString = load_string(load_state, f);
                (*io_1).value.object = &mut (*(x_ as *mut GCUnion)).object;
                (*io_1).set_tag((*x_).get_tag());
                (*io_1).set_collectable();
            }
            _ => {}
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn load_prototypes(load_state: *mut LoadState, f: *mut Prototype) { unsafe {
    let mut i: i32;
    let n: i32 = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).p = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<*mut Prototype>() as u64),
    ) as *mut *mut Prototype;
    (*f).size_p = n;
    i = 0;
    while i < n {
        let ref mut fresh27 = *((*f).p).offset(i as isize);
        *fresh27 = std::ptr::null_mut();
        i += 1;
    }
    i = 0;
    while i < n {
        let ref mut fresh28 = *((*f).p).offset(i as isize);
        *fresh28 = luaf_newproto((*load_state).state);
        if (*f).get_marked() & 1 << 5 != 0
            && (**((*f).p).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4)
                != 0
        {
            luac_barrier_(
                (*load_state).state,
                &mut (*(f as *mut GCUnion)).object,
                &mut (*(*((*f).p).offset(i as isize) as *mut GCUnion)).object,
            );
        } else {
        };
        load_function(load_state, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
    }
}}
pub unsafe extern "C" fn load_upvalues(load_state: *mut LoadState, f: *mut Prototype) { unsafe {
    let mut i: i32;
    let n: i32;
    n = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<Upvaldesc>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).upvalues = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<Upvaldesc>() as u64),
    ) as *mut Upvaldesc;
    (*f).size_upvalues = n;
    i = 0;
    while i < n {
        let ref mut fresh29 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh29 = std::ptr::null_mut();
        i += 1;
    }
    i = 0;
    while i < n {
        (*((*f).upvalues).offset(i as isize)).is_in_stack = load_byte(load_state) != 0;
        (*((*f).upvalues).offset(i as isize)).index = load_byte(load_state);
        (*((*f).upvalues).offset(i as isize)).kind = load_byte(load_state);
        i += 1;
    }
}}
pub unsafe extern "C" fn load_debug(load_state: *mut LoadState, f: *mut Prototype) { unsafe {
    let mut i: i32;
    let mut n: i32;
    n = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).line_info = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    (*f).size_line_info = n;
    load_block(
        load_state,
        (*f).line_info as *mut libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    n = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).absolute_line_info = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>() as u64),
    ) as *mut AbsoluteLineInfo;
    (*f).size_absolute_line_info = n;
    i = 0;
    while i < n {
        (*((*f).absolute_line_info).offset(i as isize)).program_counter = load_int(load_state);
        (*((*f).absolute_line_info).offset(i as isize)).line = load_int(load_state);
        i += 1;
    }
    n = load_int(load_state);
    if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
        && (n as u64).wrapping_add(1 as u64)
            > (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64)
    {
        (*((*load_state).state)).too_big();
    } else {
    };
    (*f).local_variables = luam_malloc_(
        (*load_state).state,
        (n as u64).wrapping_mul(::core::mem::size_of::<LocalVariable>() as u64),
    ) as *mut LocalVariable;
    (*f).size_local_variables = n;
    i = 0;
    while i < n {
        let ref mut fresh30 = (*((*f).local_variables).offset(i as isize)).variable_name;
        *fresh30 = std::ptr::null_mut();
        i += 1;
    }
    i = 0;
    while i < n {
        let ref mut fresh31 = (*((*f).local_variables).offset(i as isize)).variable_name;
        *fresh31 = load_string_n(load_state, f);
        (*((*f).local_variables).offset(i as isize)).start_program_counter = load_int(load_state);
        (*((*f).local_variables).offset(i as isize)).end_program_counter = load_int(load_state);
        i += 1;
    }
    n = load_int(load_state);
    if n != 0 {
        n = (*f).size_upvalues;
    }
    i = 0;
    while i < n {
        let ref mut fresh32 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh32 = load_string_n(load_state, f);
        i += 1;
    }
}}
pub unsafe extern "C" fn load_function(
    load_state: *mut LoadState,
    f: *mut Prototype,
    psource: *mut TString,
) { unsafe {
    (*f).source = load_string_n(load_state, f);
    if ((*f).source).is_null() {
        (*f).source = psource;
    }
    (*f).line_defined = load_int(load_state);
    (*f).last_line_defined = load_int(load_state);
    (*f).count_parameters = load_byte(load_state);
    (*f).is_variable_arguments = 0 != load_byte(load_state);
    (*f).maximum_stack_size = load_byte(load_state);
    load_code(load_state, f);
    load_constants(load_state, f);
    load_upvalues(load_state, f);
    load_prototypes(load_state, f);
    load_debug(load_state, f);
}}
pub unsafe extern "C" fn check_literal(load_state: *mut LoadState, s: *const i8, message: *const i8) { unsafe {
    let mut buffer: [i8; 12] = [0; 12];
    let length: u64 = strlen(s);
    load_block(
        load_state,
        buffer.as_mut_ptr() as *mut libc::c_void,
        length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    if memcmp(
        s as *const libc::c_void,
        buffer.as_mut_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        error(load_state, message);
    }
}}
pub unsafe extern "C" fn f_check_size(load_state: *mut LoadState, size: u64, tname: *const i8) { unsafe {
    if load_byte(load_state) as u64 != size {
        error(
            load_state,
            luao_pushfstring(
                (*load_state).state,
                b"%s size mismatch\0" as *const u8 as *const i8,
                tname,
            ),
        );
    }
}}
pub unsafe extern "C" fn check_header(load_state: *mut LoadState) { unsafe {
    check_literal(
        load_state,
        &*(b"\x1BLua\0" as *const u8 as *const i8).offset(1 as isize),
        b"not a binary chunk\0" as *const u8 as *const i8,
    );
    if load_byte(load_state) as i32 != 504 as i32 / 100 as i32 * 16 as i32 + 504 as i32 % 100 as i32 {
        error(load_state, b"version mismatch\0" as *const u8 as *const i8);
    }
    if load_byte(load_state) as i32 != 0 {
        error(load_state, b"format mismatch\0" as *const u8 as *const i8);
    }
    check_literal(
        load_state,
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8,
        b"corrupted chunk\0" as *const u8 as *const i8,
    );
    f_check_size(
        load_state,
        ::core::mem::size_of::<u32>() as u64,
        b"u32\0" as *const u8 as *const i8,
    );
    f_check_size(
        load_state,
        ::core::mem::size_of::<i64>() as u64,
        b"i64\0" as *const u8 as *const i8,
    );
    f_check_size(
        load_state,
        ::core::mem::size_of::<f64>() as u64,
        b"f64\0" as *const u8 as *const i8,
    );
    if load_integer(load_state) != 0x5678 as i32 as i64 {
        error(load_state, b"integer format mismatch\0" as *const u8 as *const i8);
    }
    if load_number(load_state) != 370.5f64 {
        error(load_state, b"float format mismatch\0" as *const u8 as *const i8);
    }
}}
pub unsafe extern "C" fn luau_undump(
    state: *mut State,
    zio: *mut ZIO,
    name: *const i8,
) -> *mut LClosure { unsafe {
    let mut load_state: LoadState = LoadState {
        state: std::ptr::null_mut(),
        zio: std::ptr::null_mut(),
        name: std::ptr::null(),
    };
    let cl: *mut LClosure;
    if *name as i32 == '@' as i32 || *name as i32 == '=' as i32 {
        load_state.name = name.offset(1 as isize);
    } else if *name as i32
        == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32
    {
        load_state.name = b"binary string\0" as *const u8 as *const i8;
    } else {
        load_state.name = name;
    }
    load_state.state = state;
    load_state.zio = zio;
    check_header(&mut load_state);
    cl = luaf_newlclosure(state, load_byte(&mut load_state) as i32);
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut LClosure = cl;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_CLOSURE_L);
    (*io).set_collectable ();
    luad_inctop(state);
    (*cl).p = luaf_newproto(state);
    if (*cl).get_marked() & 1 << 5 != 0
        && (*(*cl).p).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            state,
            &mut (*(cl as *mut GCUnion)).object,
            &mut (*((*cl).p as *mut GCUnion)).object,
        );
    } else {
    };
    load_function(&mut load_state, (*cl).p, std::ptr::null_mut());
    return cl;
}}
pub unsafe extern "C" fn dump_block(
    dump_state: *mut DumpState,
    b: *const libc::c_void,
    size: u64,
) { unsafe {
    if (*dump_state).status == 0 && size > 0u64 {
        (*dump_state).status =
            (Some(((*dump_state).write_function).expect("non-null function pointer")))
                .expect("non-null function pointer")((*dump_state).state, b, size, (*dump_state).data);
    }
}}
pub unsafe extern "C" fn dump_byte(dump_state: *mut DumpState, y: u8) { unsafe {
    let mut x: u8 = y;
    dump_block(
        dump_state,
        &mut x as *mut u8 as *const libc::c_void,
        (1 as u64).wrapping_mul(::core::mem::size_of::<u8>() as u64),
    );
}}
pub unsafe extern "C" fn dump_size(dump_state: *mut DumpState, mut x: u64) { unsafe {
    let mut buffer: [u8; 10] = [0; 10];
    let mut n: i32 = 0;
    loop {
        n += 1;
        buffer[(::core::mem::size_of::<u64>() as u64)
            .wrapping_mul(8 as u64)
            .wrapping_add(6 as u64)
            .wrapping_div(7 as u64)
            .wrapping_sub(n as u64) as usize] = (x & 0x7f as i32 as u64) as u8;
        x >>= 7;
        if !(x != 0u64) {
            break;
        }
    }
    buffer[(::core::mem::size_of::<u64>() as u64)
        .wrapping_mul(8 as u64)
        .wrapping_add(6 as u64)
        .wrapping_div(7 as u64)
        .wrapping_sub(1 as u64) as usize] = (buffer[(::core::mem::size_of::<u64>() as u64)
        .wrapping_mul(8 as u64)
        .wrapping_add(6 as u64)
        .wrapping_div(7 as u64)
        .wrapping_sub(1 as u64) as usize] as i32
        | 0x80 as i32) as u8;
    dump_block(
        dump_state,
        buffer
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<u64>() as u64)
                    .wrapping_mul(8 as u64)
                    .wrapping_add(6 as u64)
                    .wrapping_div(7 as u64) as isize,
            )
            .offset(-(n as isize)) as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<u8>() as u64),
    );
}}
pub unsafe extern "C" fn dump_int(dump_state: *mut DumpState, x: i32) { unsafe {
    dump_size(dump_state, x as u64);
}}
pub unsafe extern "C" fn dump_number(dump_state: *mut DumpState, mut x: f64) { unsafe {
    dump_block(
        dump_state,
        &mut x as *mut f64 as *const libc::c_void,
        (1 as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
    );
}}
pub unsafe extern "C" fn dump_integer(dump_state: *mut DumpState, mut x: i64) { unsafe {
    dump_block(
        dump_state,
        &mut x as *mut i64 as *const libc::c_void,
        (1 as u64).wrapping_mul(::core::mem::size_of::<i64>() as u64),
    );
}}
pub unsafe extern "C" fn dump_string(dump_state: *mut DumpState, s: *const TString) { unsafe {
    if s.is_null() {
        dump_size(dump_state, 0u64);
    } else {
        let size: u64 = if (*s).short_length as i32 != 0xFF as i32 {
            (*s).short_length as u64
        } else {
            (*s).u.long_length
        };
        let str: *const i8 = ((*s).contents).as_ptr();
        dump_size(dump_state, size.wrapping_add(1 as u64));
        dump_block(
            dump_state,
            str as *const libc::c_void,
            size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
    };
}}
pub unsafe extern "C" fn dump_code(dump_state: *mut DumpState, f: *const Prototype) { unsafe {
    dump_int(dump_state, (*f).size_code);
    dump_block(
        dump_state,
        (*f).code as *const libc::c_void,
        ((*f).size_code as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
}}
pub unsafe extern "C" fn dump_constants(dump_state: *mut DumpState, f: *const Prototype) { unsafe {
    let mut i: i32;
    let n: i32 = (*f).size_k;
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        let o: *const TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let tag = (*o).get_tag_variant();
        dump_byte(dump_state, tag);
        match tag {
            19 => {
                dump_number(dump_state, (*o).value.n);
            }
            3 => {
                dump_integer(dump_state, (*o).value.i);
            }
            4 | 20 => {
                dump_string(dump_state, &mut (*((*o).value.object as *mut GCUnion)).ts);
            }
            _ => {}
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn dump_prototypes(dump_state: *mut DumpState, f: *const Prototype) { unsafe {
    let mut i: i32;
    let n: i32 = (*f).size_p;
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        dump_function(dump_state, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
    }
}}
pub unsafe extern "C" fn dump_upvalues(dump_state: *mut DumpState, f: *const Prototype) { unsafe {
    let mut i: i32;
    let n: i32 = (*f).size_upvalues;
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        dump_byte(dump_state, if (*((*f).upvalues).offset(i as isize)).is_in_stack { 1 } else { 0 });
        dump_byte(dump_state, (*((*f).upvalues).offset(i as isize)).index);
        dump_byte(dump_state, (*((*f).upvalues).offset(i as isize)).kind);
        i += 1;
    }
}}
pub unsafe extern "C" fn dump_debug(dump_state: *mut DumpState, f: *const Prototype) { unsafe {
    let mut i: i32;
    let mut n: i32;
    n = if (*dump_state).is_strip {
        0
    } else {
        (*f).size_line_info
    };
    dump_int(dump_state, n);
    dump_block(
        dump_state,
        (*f).line_info as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    n = if (*dump_state).is_strip {
        0
    } else {
        (*f).size_absolute_line_info
    };
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        dump_int(dump_state, (*((*f).absolute_line_info).offset(i as isize)).program_counter);
        dump_int(dump_state, (*((*f).absolute_line_info).offset(i as isize)).line);
        i += 1;
    }
    n = if (*dump_state).is_strip {
        0
    } else {
        (*f).size_local_variables
    };
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        dump_string(dump_state, (*((*f).local_variables).offset(i as isize)).variable_name);
        dump_int(dump_state, (*((*f).local_variables).offset(i as isize)).start_program_counter);
        dump_int(dump_state, (*((*f).local_variables).offset(i as isize)).end_program_counter);
        i += 1;
    }
    n = if (*dump_state).is_strip {
        0
    } else {
        (*f).size_upvalues
    };
    dump_int(dump_state, n);
    i = 0;
    while i < n {
        dump_string(dump_state, (*((*f).upvalues).offset(i as isize)).name);
        i += 1;
    }
}}
pub unsafe extern "C" fn dump_function(
    dump_state: *mut DumpState,
    f: *const Prototype,
    psource: *mut TString,
) { unsafe {
    if (*dump_state).is_strip || (*f).source == psource {
        dump_string(dump_state, std::ptr::null());
    } else {
        dump_string(dump_state, (*f).source);
    }
    dump_int(dump_state, (*f).line_defined);
    dump_int(dump_state, (*f).last_line_defined);
    dump_byte(dump_state, (*f).count_parameters);
    dump_byte(dump_state, if (*f).is_variable_arguments { 1 } else { 0 });
    dump_byte(dump_state, (*f).maximum_stack_size);
    dump_code(dump_state, f);
    dump_constants(dump_state, f);
    dump_upvalues(dump_state, f);
    dump_prototypes(dump_state, f);
    dump_debug(dump_state, f);
}}
pub unsafe extern "C" fn dump_header(dump_state: *mut DumpState) { unsafe {
    dump_block(
        dump_state,
        b"\x1BLua\0" as *const u8 as *const i8 as *const libc::c_void,
        (::core::mem::size_of::<[i8; 5]>() as u64)
            .wrapping_sub(::core::mem::size_of::<i8>() as u64),
    );
    dump_byte(
        dump_state,
        5 * 16 + 4,
    );
    dump_byte(dump_state, 0);
    dump_block(
        dump_state,
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8 as *const libc::c_void,
        (::core::mem::size_of::<[i8; 7]>() as u64)
            .wrapping_sub(::core::mem::size_of::<i8>() as u64),
    );
    dump_byte(dump_state, ::core::mem::size_of::<u32>() as u8);
    dump_byte(dump_state, ::core::mem::size_of::<i64>() as u8);
    dump_byte(dump_state, ::core::mem::size_of::<f64>() as u8);
    dump_integer(dump_state, 0x5678 as i32 as i64);
    dump_number(dump_state, 370.5f64);
}}
pub unsafe extern "C" fn luau_dump(
    state: *mut State,
    f: *const Prototype,
    w: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 { unsafe {
    let mut dump_state: DumpState = DumpState {
        state: std::ptr::null_mut(),
        write_function: None,
        data: std::ptr::null_mut(),
        is_strip: false,
        status: 0,
    };
    dump_state.state = state;
    dump_state.write_function = w;
    dump_state.data = data;
    dump_state.is_strip = is_strip;
    dump_state.status = 0;
    dump_header(&mut dump_state);
    dump_byte(&mut dump_state, (*f).size_upvalues as u8);
    dump_function(&mut dump_state, f, std::ptr::null_mut());
    return dump_state.status;
}}
pub unsafe extern "C" fn error_expected(lexical_state: *mut LexicalState, token: i32) -> ! { unsafe {
    luax_syntaxerror(
        lexical_state,
        luao_pushfstring(
            (*lexical_state).state,
            b"%s expected\0" as *const u8 as *const i8,
            luax_token2str(lexical_state, token),
        ),
    );
}}
pub unsafe extern "C" fn errorlimit(
    fs: *mut FunctionState,
    limit: i32,
    what: *const i8,
) -> ! { unsafe {
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
}}
pub unsafe extern "C" fn checklimit(
    fs: *mut FunctionState,
    v: i32,
    l: i32,
    what: *const i8,
) { unsafe {
    if v > l {
        errorlimit(fs, l, what);
    }
}}
pub unsafe extern "C" fn testnext(lexical_state: *mut LexicalState, c: i32) -> i32 { unsafe {
    if (*lexical_state).t.token == c {
        luax_next(lexical_state);
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn check(lexical_state: *mut LexicalState, c: i32) { unsafe {
    if (*lexical_state).t.token != c {
        error_expected(lexical_state, c);
    }
}}
pub unsafe extern "C" fn checknext(lexical_state: *mut LexicalState, c: i32) { unsafe {
    check(lexical_state, c);
    luax_next(lexical_state);
}}
pub unsafe extern "C" fn check_match(
    lexical_state: *mut LexicalState,
    what: i32,
    who: i32,
    where_0: i32,
) { unsafe {
    if ((testnext(lexical_state, what) == 0) as i32 != 0) as i32 as i64 != 0 {
        if where_0 == (*lexical_state).line_number {
            error_expected(lexical_state, what);
        } else {
            luax_syntaxerror(
                lexical_state,
                luao_pushfstring(
                    (*lexical_state).state,
                    b"%s expected (to close %s at line %d)\0" as *const u8 as *const i8,
                    luax_token2str(lexical_state, what),
                    luax_token2str(lexical_state, who),
                    where_0,
                ),
            );
        }
    }
}}
pub unsafe extern "C" fn str_checkname(lexical_state: *mut LexicalState) -> *mut TString { unsafe {
    check(lexical_state, TK_NAME as i32);
    let ts: *mut TString = (*lexical_state).t.semantic_info.ts;
    luax_next(lexical_state);
    return ts;
}}
pub unsafe extern "C" fn init_exp(e: *mut ExpressionDescription, k: u32, i: i32) { unsafe {
    (*e).t = -1;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}}
pub unsafe extern "C" fn codestring(e: *mut ExpressionDescription, s: *mut TString) { unsafe {
    (*e).t = -1;
    (*e).f = (*e).t;
    (*e).k = VKSTR;
    (*e).u.strval = s;
}}
pub unsafe extern "C" fn codename(lexical_state: *mut LexicalState, e: *mut ExpressionDescription) { unsafe {
    codestring(e, str_checkname(lexical_state));
}}
pub unsafe extern "C" fn registerlocalvar(
    lexical_state: *mut LexicalState,
    fs: *mut FunctionState,
    variable_name: *mut TString,
) -> i32 { unsafe {
    let f: *mut Prototype = (*fs).f;
    let mut old_size: i32 = (*f).size_local_variables;
    (*f).local_variables = luam_growaux_(
        (*lexical_state).state,
        (*f).local_variables as *mut libc::c_void,
        (*fs).ndebugvars as i32,
        &mut (*f).size_local_variables,
        ::core::mem::size_of::<LocalVariable>() as u64 as i32,
        (if 32767 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64)
        {
            32767 as i32 as u32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64) as u32
        }) as i32,
        b"local variables\0" as *const u8 as *const i8,
    ) as *mut LocalVariable;
    while old_size < (*f).size_local_variables {
        let fresh33 = old_size;
        old_size = old_size + 1;
        let ref mut fresh34 = (*((*f).local_variables).offset(fresh33 as isize)).variable_name;
        *fresh34 = std::ptr::null_mut();
    }
    let ref mut fresh35 = (*((*f).local_variables).offset((*fs).ndebugvars as isize)).variable_name;
    *fresh35 = variable_name;
    (*((*f).local_variables).offset((*fs).ndebugvars as isize)).start_program_counter = (*fs).program_counter;
    if (*f).get_marked() & 1 << 5 != 0
        && (*variable_name).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            (*lexical_state).state,
            &mut (*(f as *mut GCUnion)).object,
            &mut (*(variable_name as *mut GCUnion)).object,
        );
    } else {
    };
    let fresh36 = (*fs).ndebugvars;
    (*fs).ndebugvars = (*fs).ndebugvars + 1;
    return fresh36 as i32;
}}
pub unsafe extern "C" fn new_localvar(lexical_state: *mut LexicalState, name: *mut TString) -> i32 { unsafe {
    let state: *mut State = (*lexical_state).state;
    let fs: *mut FunctionState = (*lexical_state).fs;
    let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
    let var: *mut VariableDescription;
    checklimit(
        fs,
        (*dynamic_data).active_variable.n + 1 - (*fs).firstlocal,
        200 as i32,
        b"local variables\0" as *const u8 as *const i8,
    );
    (*dynamic_data).active_variable.arr = luam_growaux_(
        state,
        (*dynamic_data).active_variable.arr as *mut libc::c_void,
        (*dynamic_data).active_variable.n + 1,
        &mut (*dynamic_data).active_variable.size,
        ::core::mem::size_of::<VariableDescription>() as u64 as i32,
        (if 32767 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<VariableDescription>() as u64)
        {
            32767 as i32 as u32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<VariableDescription>() as u64)
                as u32
        }) as i32,
        b"local variables\0" as *const u8 as *const i8,
    ) as *mut VariableDescription;
    let fresh37 = (*dynamic_data).active_variable.n;
    (*dynamic_data).active_variable.n = (*dynamic_data).active_variable.n + 1;
    var = &mut *((*dynamic_data).active_variable.arr).offset(fresh37 as isize)
        as *mut VariableDescription;
    (*var).vd.kind = 0;
    (*var).vd.name = name;
    return (*dynamic_data).active_variable.n - 1 - (*fs).firstlocal;
}}
pub unsafe extern "C" fn getlocalvardesc(
    fs: *mut FunctionState,
    vidx: i32,
) -> *mut VariableDescription { unsafe {
    return &mut *((*(*(*fs).lexical_state).dynamic_data).active_variable.arr)
        .offset(((*fs).firstlocal + vidx) as isize) as *mut VariableDescription;
}}
pub unsafe extern "C" fn reglevel(fs: *mut FunctionState, mut nvar: i32) -> i32 { unsafe {
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
}}
pub unsafe extern "C" fn luay_nvarstack(fs: *mut FunctionState) -> i32 { unsafe {
    return reglevel(fs, (*fs).count_active_variables as i32);
}}
pub unsafe extern "C" fn localdebuginfo(
    fs: *mut FunctionState,
    vidx: i32,
) -> *mut LocalVariable { unsafe {
    let vd: *mut VariableDescription = getlocalvardesc(fs, vidx);
    if (*vd).vd.kind as i32 == 3 {
        return std::ptr::null_mut();
    } else {
        let index: i32 = (*vd).vd.pidx as i32;
        return &mut *((*(*fs).f).local_variables).offset(index as isize) as *mut LocalVariable;
    };
}}
pub unsafe extern "C" fn init_var(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    vidx: i32,
) { unsafe {
    (*e).t = -1;
    (*e).f = (*e).t;
    (*e).k = VLOCAL;
    (*e).u.var.vidx = vidx as u16;
    (*e).u.var.ridx = (*getlocalvardesc(fs, vidx)).vd.ridx;
}}
pub unsafe extern "C" fn check_readonly(lexical_state: *mut LexicalState, e: *mut ExpressionDescription) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut variable_name: *mut TString = std::ptr::null_mut();
    match (*e).k as u32 {
        11 => {
            variable_name = (*((*(*lexical_state).dynamic_data).active_variable.arr).offset((*e).u.info as isize))
                .vd
                .name;
        }
        9 => {
            let vardesc: *mut VariableDescription = getlocalvardesc(fs, (*e).u.var.vidx as i32);
            if (*vardesc).vd.kind as i32 != 0 {
                variable_name = (*vardesc).vd.name;
            }
        }
        10 => {
            let up: *mut Upvaldesc =
                &mut *((*(*fs).f).upvalues).offset((*e).u.info as isize) as *mut Upvaldesc;
            if (*up).kind as i32 != 0 {
                variable_name = (*up).name;
            }
        }
        _ => return,
    }
    if !variable_name.is_null() {
        let message: *const i8 = luao_pushfstring(
            (*lexical_state).state,
            b"attempt to assign to const variable '%s'\0" as *const u8 as *const i8,
            ((*variable_name).contents).as_mut_ptr(),
        );
        luak_semerror(lexical_state, message);
    }
}}
pub unsafe extern "C" fn adjustlocalvars(lexical_state: *mut LexicalState, nvars: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut reglevel_0: i32 = luay_nvarstack(fs);
    let mut i: i32;
    i = 0;
    while i < nvars {
        let fresh39 = (*fs).count_active_variables;
        (*fs).count_active_variables = ((*fs).count_active_variables).wrapping_add(1);
        let vidx: i32 = fresh39 as i32;
        let var: *mut VariableDescription = getlocalvardesc(fs, vidx);
        let fresh40 = reglevel_0;
        reglevel_0 = reglevel_0 + 1;
        (*var).vd.ridx = fresh40 as u8;
        (*var).vd.pidx = registerlocalvar(lexical_state, fs, (*var).vd.name) as i16;
        i += 1;
    }
}}
pub unsafe extern "C" fn removevars(fs: *mut FunctionState, tolevel: i32) { unsafe {
    (*(*(*fs).lexical_state).dynamic_data).active_variable.n -= (*fs).count_active_variables as i32 - tolevel;
    while (*fs).count_active_variables as i32 > tolevel {
        (*fs).count_active_variables = ((*fs).count_active_variables).wrapping_sub(1);
        let var: *mut LocalVariable = localdebuginfo(fs, (*fs).count_active_variables as i32);
        if !var.is_null() {
            (*var).end_program_counter = (*fs).program_counter;
        }
    }
}}
pub unsafe extern "C" fn searchupvalue(fs: *mut FunctionState, name: *mut TString) -> i32 { unsafe {
    let mut i: i32;
    let up: *mut Upvaldesc = (*(*fs).f).upvalues;
    i = 0;
    while i < (*fs).nups as i32 {
        if (*up.offset(i as isize)).name == name {
            return i;
        }
        i += 1;
    }
    return -1;
}}
pub unsafe extern "C" fn allocupvalue(fs: *mut FunctionState) -> *mut Upvaldesc { unsafe {
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
        ::core::mem::size_of::<Upvaldesc>() as u64 as i32,
        (if 255 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<Upvaldesc>() as u64)
        {
            255 as i32 as u32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<Upvaldesc>() as u64) as u32
        }) as i32,
        b"upvalues\0" as *const u8 as *const i8,
    ) as *mut Upvaldesc;
    while old_size < (*f).size_upvalues {
        let fresh41 = old_size;
        old_size = old_size + 1;
        let ref mut fresh42 = (*((*f).upvalues).offset(fresh41 as isize)).name;
        *fresh42 = std::ptr::null_mut();
    }
    let fresh43 = (*fs).nups;
    (*fs).nups = ((*fs).nups).wrapping_add(1);
    return &mut *((*f).upvalues).offset(fresh43 as isize) as *mut Upvaldesc;
}}
pub unsafe extern "C" fn newupvalue(
    fs: *mut FunctionState,
    name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 { unsafe {
    let up: *mut Upvaldesc = allocupvalue(fs);
    let previous: *mut FunctionState = (*fs).previous;
    if (*v).k as u32 == VLOCAL as i32 as u32 {
        (*up).is_in_stack = true;
        (*up).index = (*v).u.var.ridx;
        (*up).kind = (*getlocalvardesc(previous, (*v).u.var.vidx as i32)).vd.kind;
    } else {
        (*up).is_in_stack = false;
        (*up).index = (*v).u.info as u8;
        (*up).kind = (*((*(*previous).f).upvalues).offset((*v).u.info as isize)).kind;
    }
    (*up).name = name;
    if (*(*fs).f).get_marked() & 1 << 5 != 0
        && (*name).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            (*(*fs).lexical_state).state,
            &mut (*((*fs).f as *mut GCUnion)).object,
            &mut (*(name as *mut GCUnion)).object,
        );
    } else {
    };
    return (*fs).nups as i32 - 1;
}}
pub unsafe extern "C" fn searchvar(
    fs: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
) -> i32 { unsafe {
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
}}
pub unsafe extern "C" fn markupval(fs: *mut FunctionState, level: i32) { unsafe {
    let mut block_control: *mut BlockControl = (*fs).block_control;
    while (*block_control).count_active_variables as i32 > level {
        block_control = (*block_control).previous;
    }
    (*block_control).count_upvalues = 1;
    (*fs).needclose = 1;
}}
pub unsafe extern "C" fn marktobeclosed(fs: *mut FunctionState) { unsafe {
    let block_control: *mut BlockControl = (*fs).block_control;
    (*block_control).count_upvalues = 1;
    (*block_control).is_inside_tbc = true;
    (*fs).needclose = 1;
}}
pub unsafe extern "C" fn singlevaraux(
    fs: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
    base: i32,
) { unsafe {
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
                if (*var).k as u32 == VLOCAL as i32 as u32
                    || (*var).k as u32 == VUPVAL as i32 as u32
                {
                    index = newupvalue(fs, n, var);
                } else {
                    return;
                }
            }
            init_exp(var, VUPVAL, index);
        }
    };
}}
pub unsafe extern "C" fn singlevar(lexical_state: *mut LexicalState, var: *mut ExpressionDescription) { unsafe {
    let variable_name: *mut TString = str_checkname(lexical_state);
    let fs: *mut FunctionState = (*lexical_state).fs;
    singlevaraux(fs, variable_name, var, 1);
    if (*var).k as u32 == VVOID as i32 as u32 {
        let mut key: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        singlevaraux(fs, (*lexical_state).envn, var, 1);
        luak_exp2anyregup(fs, var);
        codestring(&mut key, variable_name);
        luak_indexed(fs, var, &mut key);
    }
}}
pub unsafe extern "C" fn adjust_assign(
    lexical_state: *mut LexicalState,
    nvars: i32,
    nexps: i32,
    e: *mut ExpressionDescription,
) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let needed: i32 = nvars - nexps;
    if (*e).k as u32 == VCALL as i32 as u32 || (*e).k as u32 == VVARARG as i32 as u32 {
        let mut extra: i32 = needed + 1;
        if extra < 0 {
            extra = 0;
        }
        luak_setreturns(fs, e, extra);
    } else {
        if (*e).k as u32 != VVOID as i32 as u32 {
            luak_exp2nextreg(fs, e);
        }
        if needed > 0 {
            luak_nil(fs, (*fs).freereg as i32, needed);
        }
    }
    if needed > 0 {
        luak_reserveregs(fs, needed);
    } else {
        (*fs).freereg = ((*fs).freereg as i32 + needed) as u8;
    };
}}
pub unsafe extern "C" fn jumpscopeerror(lexical_state: *mut LexicalState, gt: *mut LabelDescription) -> ! { unsafe {
    let variable_name: *const i8 =
        ((*(*getlocalvardesc((*lexical_state).fs, (*gt).count_active_variables as i32))
            .vd
            .name)
            .contents)
            .as_mut_ptr();
    let mut message: *const i8 =
        b"<goto %s> at line %d jumps into the scope of local '%s'\0" as *const u8 as *const i8;
    message = luao_pushfstring(
        (*lexical_state).state,
        message,
        ((*(*gt).name).contents).as_mut_ptr(),
        (*gt).line,
        variable_name,
    );
    luak_semerror(lexical_state, message);
}}
pub unsafe extern "C" fn solvegoto(
    lexical_state: *mut LexicalState,
    g: i32,
    label: *mut LabelDescription,
) { unsafe {
    let mut i: i32;
    let gl: *mut Labellist = &mut (*(*lexical_state).dynamic_data).gt;
    let gt: *mut LabelDescription =
        &mut *((*gl).arr).offset(g as isize) as *mut LabelDescription;
    if ((((*gt).count_active_variables as i32) < (*label).count_active_variables as i32) as i32
        != 0) as i32 as i64
        != 0
    {
        jumpscopeerror(lexical_state, gt);
    }
    luak_patchlist((*lexical_state).fs, (*gt).program_counter, (*label).program_counter);
    i = g;
    while i < (*gl).n - 1 {
        *((*gl).arr).offset(i as isize) = *((*gl).arr).offset((i + 1) as isize);
        i += 1;
    }
    (*gl).n -= 1;
    (*gl).n;
}}
pub unsafe extern "C" fn findlabel(
    lexical_state: *mut LexicalState,
    name: *mut TString,
) -> *mut LabelDescription { unsafe {
    let mut i: i32;
    let dynamic_data: *mut DynamicData = (*lexical_state).dynamic_data;
    i = (*(*lexical_state).fs).first_label;
    while i < (*dynamic_data).label.n {
        let lb: *mut LabelDescription =
            &mut *((*dynamic_data).label.arr).offset(i as isize) as *mut LabelDescription;
        if (*lb).name == name {
            return lb;
        }
        i += 1;
    }
    return std::ptr::null_mut();
}}
pub unsafe extern "C" fn newlabelentry(
    lexical_state: *mut LexicalState,
    l: *mut Labellist,
    name: *mut TString,
    line: i32,
    program_counter: i32,
) -> i32 { unsafe {
    let n: i32 = (*l).n;
    (*l).arr = luam_growaux_(
        (*lexical_state).state,
        (*l).arr as *mut libc::c_void,
        n,
        &mut (*l).size,
        ::core::mem::size_of::<LabelDescription>() as u64 as i32,
        (if 32767 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<LabelDescription>() as u64)
        {
            32767 as i32 as u32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<LabelDescription>() as u64) as u32
        }) as i32,
        b"labels/gotos\0" as *const u8 as *const i8,
    ) as *mut LabelDescription;
    let ref mut fresh44 = (*((*l).arr).offset(n as isize)).name;
    *fresh44 = name;
    (*((*l).arr).offset(n as isize)).line = line;
    (*((*l).arr).offset(n as isize)).count_active_variables = (*(*lexical_state).fs).count_active_variables;
    (*((*l).arr).offset(n as isize)).close = 0;
    (*((*l).arr).offset(n as isize)).program_counter = program_counter;
    (*l).n = n + 1;
    return n;
}}
pub unsafe extern "C" fn newgotoentry(
    lexical_state: *mut LexicalState,
    name: *mut TString,
    line: i32,
    program_counter: i32,
) -> i32 { unsafe {
    return newlabelentry(
        lexical_state,
        &mut (*(*lexical_state).dynamic_data).gt,
        name,
        line,
        program_counter,
    );
}}
pub unsafe extern "C" fn solvegotos(lexical_state: *mut LexicalState, lb: *mut LabelDescription) -> bool { unsafe {
    let gl: *mut Labellist = &mut (*(*lexical_state).dynamic_data).gt;
    let mut i: i32 = (*(*(*lexical_state).fs).block_control).first_goto;
    let mut needsclose = false;
    while i < (*gl).n {
        if (*((*gl).arr).offset(i as isize)).name == (*lb).name {
            needsclose = needsclose || (0 != (*((*gl).arr).offset(i as isize)).close);
            solvegoto(lexical_state, i, lb);
        } else {
            i += 1;
        }
    }
    return needsclose;
}}
pub unsafe extern "C" fn createlabel(
    lexical_state: *mut LexicalState,
    name: *mut TString,
    line: i32,
    last: i32,
) -> i32 { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let ll: *mut Labellist = &mut (*(*lexical_state).dynamic_data).label;
    let l: i32 = newlabelentry(lexical_state, ll, name, line, luak_getlabel(fs));
    if last != 0 {
        (*((*ll).arr).offset(l as isize)).count_active_variables =
            (*(*fs).block_control).count_active_variables;
    }
    if solvegotos(lexical_state, &mut *((*ll).arr).offset(l as isize)) {
        luak_code_abck(fs, OP_CLOSE, luay_nvarstack(fs), 0, 0, 0);
        return 1;
    }
    return 0;
}}
pub unsafe extern "C" fn movegotosout(
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
) { unsafe {
    let mut i: i32;
    let gl: *mut Labellist = &mut (*(*(*fs).lexical_state).dynamic_data).gt;
    i = (*block_control).first_goto;
    while i < (*gl).n {
        let gt: *mut LabelDescription =
            &mut *((*gl).arr).offset(i as isize) as *mut LabelDescription;
        if reglevel(fs, (*gt).count_active_variables as i32)
            > reglevel(fs, (*block_control).count_active_variables as i32)
        {
            (*gt).close = ((*gt).close as i32 | (*block_control).count_upvalues as i32) as u8;
        }
        (*gt).count_active_variables = (*block_control).count_active_variables;
        i += 1;
    }
}}
pub unsafe extern "C" fn enterblock(
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
    is_loop: bool,
) { unsafe {
    (*block_control).is_loop = is_loop;
    (*block_control).count_active_variables = (*fs).count_active_variables;
    (*block_control).first_label = (*(*(*fs).lexical_state).dynamic_data).label.n;
    (*block_control).first_goto = (*(*(*fs).lexical_state).dynamic_data).gt.n;
    (*block_control).count_upvalues = 0;
    (*block_control).is_inside_tbc =
        !((*fs).block_control).is_null() && (*(*fs).block_control).is_inside_tbc as i32 != 0;
    (*block_control).previous = (*fs).block_control;
    (*fs).block_control = block_control;
}}
pub unsafe extern "C" fn undefgoto(lexical_state: *mut LexicalState, gt: *mut LabelDescription) -> ! { unsafe {
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
            ((*(*gt).name).contents).as_mut_ptr(),
            (*gt).line,
        );
    }
    luak_semerror(lexical_state, message);
}}
pub unsafe extern "C" fn leaveblock(fs: *mut FunctionState) { unsafe {
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
            &mut *((*(*lexical_state).dynamic_data).gt.arr).offset((*block_control).first_goto as isize),
        );
    }
}}
pub unsafe extern "C" fn codeclosure(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
    let fs: *mut FunctionState = (*(*lexical_state).fs).previous;
    init_exp(
        v,
        VRELOC,
        luak_codeabx(fs, OP_CLOSURE, 0, ((*fs).np - 1) as u32),
    );
    luak_exp2nextreg(fs, v);
}}
pub unsafe extern "C" fn open_func(
    lexical_state: *mut LexicalState,
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
) { unsafe {
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
    if (*f).get_marked() & 1 << 5 != 0
        && (*(*f).source).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            (*lexical_state).state,
            &mut (*(f as *mut GCUnion)).object,
            &mut (*((*f).source as *mut GCUnion)).object,
        );
    } else {
    };
    (*f).maximum_stack_size = 2 as u8;
    enterblock(fs, block_control, false);
}}
pub unsafe extern "C" fn close_func(lexical_state: *mut LexicalState) { unsafe {
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
        ::core::mem::size_of::<u32>() as u64 as i32,
    ) as *mut u32;
    (*f).line_info = luam_shrinkvector_(
        state,
        (*f).line_info as *mut libc::c_void,
        &mut (*f).size_line_info,
        (*fs).program_counter,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    (*f).absolute_line_info = luam_shrinkvector_(
        state,
        (*f).absolute_line_info as *mut libc::c_void,
        &mut (*f).size_absolute_line_info,
        (*fs).nabslineinfo,
        ::core::mem::size_of::<AbsoluteLineInfo>() as u64 as i32,
    ) as *mut AbsoluteLineInfo;
    (*f).k = luam_shrinkvector_(
        state,
        (*f).k as *mut libc::c_void,
        &mut (*f).size_k,
        (*fs).nk,
        ::core::mem::size_of::<TValue>() as u64 as i32,
    ) as *mut TValue;
    (*f).p = luam_shrinkvector_(
        state,
        (*f).p as *mut libc::c_void,
        &mut (*f).size_p,
        (*fs).np,
        ::core::mem::size_of::<*mut Prototype>() as u64 as i32,
    ) as *mut *mut Prototype;
    (*f).local_variables = luam_shrinkvector_(
        state,
        (*f).local_variables as *mut libc::c_void,
        &mut (*f).size_local_variables,
        (*fs).ndebugvars as i32,
        ::core::mem::size_of::<LocalVariable>() as u64 as i32,
    ) as *mut LocalVariable;
    (*f).upvalues = luam_shrinkvector_(
        state,
        (*f).upvalues as *mut libc::c_void,
        &mut (*f).size_upvalues,
        (*fs).nups as i32,
        ::core::mem::size_of::<Upvaldesc>() as u64 as i32,
    ) as *mut Upvaldesc;
    (*lexical_state).fs = (*fs).previous;
    if (*(*state).global).gc_debt > 0 {
        luac_step(state);
    }
}}
pub unsafe extern "C" fn block_follow(lexical_state: *mut LexicalState, withuntil: i32) -> i32 { unsafe {
    match (*lexical_state).t.token {
        259 | 260 | 261 | 288 => return 1,
        276 => return withuntil,
        _ => return 0,
    };
}}
pub unsafe extern "C" fn statlist(lexical_state: *mut LexicalState) { unsafe {
    while block_follow(lexical_state, 1) == 0 {
        if (*lexical_state).t.token == TK_RETURN as i32 {
            statement(lexical_state);
            return;
        }
        statement(lexical_state);
    }
}}
pub unsafe extern "C" fn fieldsel(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
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
}}
pub unsafe extern "C" fn yindex(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
    luax_next(lexical_state);
    expr(lexical_state, v);
    luak_exp2val((*lexical_state).fs, v);
    checknext(lexical_state, ']' as i32);
}}
pub unsafe extern "C" fn recfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) { unsafe {
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
        2147483647 as i32,
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
}}
pub unsafe extern "C" fn closelistfield(
    fs: *mut FunctionState,
    cc: *mut ConstructorControl,
) { unsafe {
    if (*cc).v.k as u32 == VVOID as i32 as u32 {
        return;
    }
    luak_exp2nextreg(fs, &mut (*cc).v);
    (*cc).v.k = VVOID;
    if (*cc).tostore == 50 as i32 {
        luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        (*cc).na += (*cc).tostore;
        (*cc).tostore = 0;
    }
}}
pub unsafe extern "C" fn lastlistfield(
    fs: *mut FunctionState,
    cc: *mut ConstructorControl,
) { unsafe {
    if (*cc).tostore == 0 {
        return;
    }
    if (*cc).v.k as u32 == VCALL as i32 as u32 || (*cc).v.k as u32 == VVARARG as i32 as u32 {
        luak_setreturns(fs, &mut (*cc).v, -1);
        luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, -1);
        (*cc).na -= 1;
        (*cc).na;
    } else {
        if (*cc).v.k as u32 != VVOID as i32 as u32 {
            luak_exp2nextreg(fs, &mut (*cc).v);
        }
        luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
    }
    (*cc).na += (*cc).tostore;
}}
pub unsafe extern "C" fn listfield(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) { unsafe {
    expr(lexical_state, &mut (*cc).v);
    (*cc).tostore += 1;
    (*cc).tostore;
}}
pub unsafe extern "C" fn field(lexical_state: *mut LexicalState, cc: *mut ConstructorControl) { unsafe {
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
}}
pub unsafe extern "C" fn constructor(lexical_state: *mut LexicalState, t: *mut ExpressionDescription) { unsafe {
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
        if !(testnext(lexical_state, ',' as i32) != 0 || testnext(lexical_state, ';' as i32) != 0) {
            break;
        }
    }
    check_match(lexical_state, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    luak_settablesize(fs, program_counter, (*t).u.info, cc.na, cc.nh);
}}
pub unsafe extern "C" fn setvararg(fs: *mut FunctionState, nparams: i32) { unsafe {
    (*(*fs).f).is_variable_arguments = true;
    luak_code_abck(fs, OP_VARARGPREP, nparams, 0, 0, 0);
}}
pub unsafe extern "C" fn parlist(lexical_state: *mut LexicalState) { unsafe {
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
                    luax_syntaxerror(lexical_state, b"<name> or '...' expected\0" as *const u8 as *const i8);
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
}}
pub unsafe extern "C" fn body(
    lexical_state: *mut LexicalState,
    e: *mut ExpressionDescription,
    ismethod: i32,
    line: i32,
) { unsafe {
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
}}
pub unsafe extern "C" fn explist(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) -> i32 { unsafe {
    let mut n: i32 = 1;
    expr(lexical_state, v);
    while testnext(lexical_state, ',' as i32) != 0 {
        luak_exp2nextreg((*lexical_state).fs, v);
        expr(lexical_state, v);
        n += 1;
    }
    return n;
}}
pub unsafe extern "C" fn funcargs(lexical_state: *mut LexicalState, f: *mut ExpressionDescription) { unsafe {
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
                if args.k as u32 == VCALL as i32 as u32 || args.k as u32 == VVARARG as i32 as u32 {
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
    if args.k as u32 == VCALL as i32 as u32 || args.k as u32 == VVARARG as i32 as u32 {
        nparams = -1;
    } else {
        if args.k as u32 != VVOID as i32 as u32 {
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
}}
pub unsafe extern "C" fn primaryexp(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
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
            luax_syntaxerror(lexical_state, b"unexpected symbol\0" as *const u8 as *const i8);
        }
    };
}}
pub unsafe extern "C" fn suffixedexp(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
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
}}
pub unsafe extern "C" fn simpleexp(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
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
}}
pub unsafe extern "C" fn getunopr(op: i32) -> Unary {
    match op {
        TK_NOT => return Unary::Not,
        45 => return Unary::Minus,
        126 => return Unary::BitwiseNot,
        35 => return Unary::Length,
        _ => return Unary::None_,
    };
}
pub unsafe extern "C" fn getbinopr(op: i32) -> u32 {
    match op {
        43 => return OPR_ADD,
        45 => return OPR_SUB,
        42 => return OPR_MUL,
        37 => return OPR_MOD,
        94 => return OPR_POW,
        47 => return OPR_DIV,
        278 => return OPR_IDIV,
        38 => return OPR_BAND,
        124 => return OPR_BOR,
        126 => return OPR_BXOR,
        285 => return OPR_SHL,
        286 => return OPR_SHR,
        279 => return OPR_CONCAT,
        284 => return OPR_NE,
        281 => return OPR_EQ,
        60 => return OPR_LT,
        283 => return OPR_LE,
        62 => return OPR_GT,
        TK_GE => return OPR_GE,
        TK_AND => return OPR_AND,
        TK_OR => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}
pub unsafe extern "C" fn subexpr(
    lexical_state: *mut LexicalState,
    v: *mut ExpressionDescription,
    limit: i32,
) -> u32 { unsafe {
    luae_inccstack((*lexical_state).state);
    let uop = getunopr((*lexical_state).t.token);
    if uop as u32 != Unary::None_ as i32 as u32 {
        let line: i32 = (*lexical_state).line_number;
        luax_next(lexical_state);
        subexpr(lexical_state, v, 12 as i32);
        luak_prefix((*lexical_state).fs, uop, v, line);
    } else {
        simpleexp(lexical_state, v);
    }
    let mut op: u32 = getbinopr((*lexical_state).t.token);
    while op as u32 != OPR_NOBINOPR as i32 as u32 && PRIORITY[op as usize].left as i32 > limit {
        let mut v2: ExpressionDescription = ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        };
        let line_0: i32 = (*lexical_state).line_number;
        luax_next(lexical_state);
        luak_infix((*lexical_state).fs, op, v);
        let nextop: u32 = subexpr(lexical_state, &mut v2, PRIORITY[op as usize].right as i32);
        luak_posfix((*lexical_state).fs, op, v, &mut v2, line_0);
        op = nextop;
    }
    (*(*lexical_state).state).count_c_calls = ((*(*lexical_state).state).count_c_calls).wrapping_sub(1);
    (*(*lexical_state).state).count_c_calls;
    return op;
}}
pub unsafe extern "C" fn expr(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) { unsafe {
    subexpr(lexical_state, v, 0);
}}
pub unsafe extern "C" fn block(lexical_state: *mut LexicalState) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut block_control: BlockControl = BlockControl::new();
    enterblock(fs, &mut block_control, false);
    statlist(lexical_state);
    leaveblock(fs);
}}
pub unsafe extern "C" fn check_conflict(
    lexical_state: *mut LexicalState,
    mut lh: *mut LHSAssign,
    v: *mut ExpressionDescription,
) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let extra: i32 = (*fs).freereg as i32;
    let mut conflict: i32 = 0;
    while !lh.is_null() {
        if VINDEXED as i32 as u32 <= (*lh).v.k as u32 && (*lh).v.k as u32 <= VINDEXSTR as i32 as u32
        {
            if (*lh).v.k as u32 == VINDEXUP as i32 as u32 {
                if (*v).k as u32 == VUPVAL as i32 as u32 && (*lh).v.u.ind.t as i32 == (*v).u.info {
                    conflict = 1;
                    (*lh).v.k = VINDEXSTR;
                    (*lh).v.u.ind.t = extra as u8;
                }
            } else {
                if (*v).k as u32 == VLOCAL as i32 as u32
                    && (*lh).v.u.ind.t as i32 == (*v).u.var.ridx as i32
                {
                    conflict = 1;
                    (*lh).v.u.ind.t = extra as u8;
                }
                if (*lh).v.k as u32 == VINDEXED as i32 as u32
                    && (*v).k as u32 == VLOCAL as i32 as u32
                    && (*lh).v.u.ind.index as i32 == (*v).u.var.ridx as i32
                {
                    conflict = 1;
                    (*lh).v.u.ind.index = extra as i16;
                }
            }
        }
        lh = (*lh).previous;
    }
    if conflict != 0 {
        if (*v).k as u32 == VLOCAL as i32 as u32 {
            luak_code_abck(fs, OP_MOVE, extra, (*v).u.var.ridx as i32, 0, 0);
        } else {
            luak_code_abck(fs, OP_GETUPVAL, extra, (*v).u.info, 0, 0);
        }
        luak_reserveregs(fs, 1);
    }
}}
pub unsafe extern "C" fn restassign(lexical_state: *mut LexicalState, lh: *mut LHSAssign, nvars: i32) { unsafe {
    let mut e: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    if !(VLOCAL as i32 as u32 <= (*lh).v.k as u32 && (*lh).v.k as u32 <= VINDEXSTR as i32 as u32) {
        luax_syntaxerror(lexical_state, b"syntax error\0" as *const u8 as *const i8);
    }
    check_readonly(lexical_state, &mut (*lh).v);
    if testnext(lexical_state, ',' as i32) != 0 {
        let mut nv: LHSAssign = LHSAssign {
            previous: std::ptr::null_mut(),
            v: ExpressionDescription {
                k: VVOID,
                u: RawValue { ival: 0 },
                t: 0,
                f: 0,
            },
        };
        nv.previous = lh;
        suffixedexp(lexical_state, &mut nv.v);
        if !(VINDEXED as i32 as u32 <= nv.v.k as u32 && nv.v.k as u32 <= VINDEXSTR as i32 as u32) {
            check_conflict(lexical_state, lh, &mut nv.v);
        }
        luae_inccstack((*lexical_state).state);
        restassign(lexical_state, &mut nv, nvars + 1);
        (*(*lexical_state).state).count_c_calls = ((*(*lexical_state).state).count_c_calls).wrapping_sub(1);
        (*(*lexical_state).state).count_c_calls;
    } else {
        checknext(lexical_state, '=' as i32);
        let nexps: i32 = explist(lexical_state, &mut e);
        if nexps != nvars {
            adjust_assign(lexical_state, nvars, nexps, &mut e);
        } else {
            luak_setoneret((*lexical_state).fs, &mut e);
            luak_storevar((*lexical_state).fs, &mut (*lh).v, &mut e);
            return;
        }
    }
    init_exp(&mut e, VNONRELOC, (*(*lexical_state).fs).freereg as i32 - 1);
    luak_storevar((*lexical_state).fs, &mut (*lh).v, &mut e);
}}
pub unsafe extern "C" fn cond(lexical_state: *mut LexicalState) -> i32 { unsafe {
    let mut v: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(lexical_state, &mut v);
    if v.k as u32 == VNIL as i32 as u32 {
        v.k = VFALSE;
    }
    luak_goiftrue((*lexical_state).fs, &mut v);
    return v.f;
}}
pub unsafe extern "C" fn gotostat(lexical_state: *mut LexicalState) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let line: i32 = (*lexical_state).line_number;
    let name: *mut TString = str_checkname(lexical_state);
    let lb: *mut LabelDescription = findlabel(lexical_state, name);
    if lb.is_null() {
        newgotoentry(lexical_state, name, line, luak_jump(fs));
    } else {
        let lblevel: i32 = reglevel(fs, (*lb).count_active_variables as i32);
        if luay_nvarstack(fs) > lblevel {
            luak_code_abck(fs, OP_CLOSE, lblevel, 0, 0, 0);
        }
        luak_patchlist(fs, luak_jump(fs), (*lb).program_counter);
    };
}}
pub unsafe extern "C" fn breakstat(lexical_state: *mut LexicalState) { unsafe {
    let line: i32 = (*lexical_state).line_number;
    luax_next(lexical_state);
    newgotoentry(
        lexical_state,
        luas_newlstr(
            (*lexical_state).state,
            b"break\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
        line,
        luak_jump((*lexical_state).fs),
    );
}}
pub unsafe extern "C" fn checkrepeated(lexical_state: *mut LexicalState, name: *mut TString) { unsafe {
    let lb: *mut LabelDescription = findlabel(lexical_state, name);
    if ((lb != std::ptr::null_mut() as *mut LabelDescription) as i32 != 0) as i32 as i64 != 0 {
        let mut message: *const i8 =
            b"label '%s' already defined on line %d\0" as *const u8 as *const i8;
        message = luao_pushfstring(
            (*lexical_state).state,
            message,
            ((*name).contents).as_mut_ptr(),
            (*lb).line,
        );
        luak_semerror(lexical_state, message);
    }
}}
pub unsafe extern "C" fn labelstat(lexical_state: *mut LexicalState, name: *mut TString, line: i32) { unsafe {
    checknext(lexical_state, TK_DBCOLON as i32);
    while (*lexical_state).t.token == ';' as i32 || (*lexical_state).t.token == TK_DBCOLON as i32 {
        statement(lexical_state);
    }
    checkrepeated(lexical_state, name);
    createlabel(lexical_state, name, line, block_follow(lexical_state, 0));
}}
pub unsafe extern "C" fn whilestat(lexical_state: *mut LexicalState, line: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut block_control: BlockControl = BlockControl::new();
    luax_next(lexical_state);
    let whileinit: i32 = luak_getlabel(fs);
    let condexit: i32 = cond(lexical_state);
    enterblock(fs, &mut block_control, true);
    checknext(lexical_state, TK_DO as i32);
    block(lexical_state);
    luak_patchlist(fs, luak_jump(fs), whileinit);
    check_match(lexical_state, TK_END as i32, TK_WHILE as i32, line);
    leaveblock(fs);
    luak_patchtohere(fs, condexit);
}}
pub unsafe extern "C" fn repeatstat(lexical_state: *mut LexicalState, line: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let repeat_init: i32 = luak_getlabel(fs);
    let mut bl1: BlockControl = BlockControl::new();
    let mut bl2: BlockControl = BlockControl::new();
    enterblock(fs, &mut bl1, true);
    enterblock(fs, &mut bl2, false);
    luax_next(lexical_state);
    statlist(lexical_state);
    check_match(lexical_state, TK_UNTIL as i32, TK_REPEAT as i32, line);
    let mut condexit: i32 = cond(lexical_state);
    leaveblock(fs);
    if bl2.count_upvalues != 0 {
        let exit_0: i32 = luak_jump(fs);
        luak_patchtohere(fs, condexit);
        luak_code_abck(
            fs,
            OP_CLOSE,
            reglevel(fs, bl2.count_active_variables as i32),
            0,
            0,
            0,
        );
        condexit = luak_jump(fs);
        luak_patchtohere(fs, exit_0);
    }
    luak_patchlist(fs, condexit, repeat_init);
    leaveblock(fs);
}}
pub unsafe extern "C" fn exp1(lexical_state: *mut LexicalState) { unsafe {
    let mut e: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(lexical_state, &mut e);
    luak_exp2nextreg((*lexical_state).fs, &mut e);
}}
pub unsafe extern "C" fn fixforjump(
    fs: *mut FunctionState,
    program_counter: i32,
    dest: i32,
    back: i32,
) { unsafe {
    let jmp: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
    let mut offset: i32 = dest - (program_counter + 1);
    if back != 0 {
        offset = -offset;
    }
    if ((offset > (1 << 8 + 8 + 1) - 1) as i32 != 0) as i32 as i64 != 0 {
        luax_syntaxerror(
            (*fs).lexical_state,
            b"control structure too long\0" as *const u8 as *const i8,
        );
    }
    *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1) << 0 + 7 + 8)
        | (offset as u32) << 0 + 7 + 8
            & !(!(0u32) << 8 + 8 + 1) << 0 + 7 + 8;
}}
pub unsafe extern "C" fn forbody(
    lexical_state: *mut LexicalState,
    base: i32,
    line: i32,
    nvars: i32,
    isgen: i32,
) { unsafe {
    static mut FOR_PREP: [u32; 2] = [OP_FORPREP, OP_TFORPREP];
    static mut FOR_LOOP: [u32; 2] = [OP_FORLOOP, OP_TFORLOOP];
    let mut block_control: BlockControl = BlockControl::new();
    let fs: *mut FunctionState = (*lexical_state).fs;
    checknext(lexical_state, TK_DO as i32);
    let prep: i32 = luak_codeabx(fs, FOR_PREP[isgen as usize], base, 0u32);
    enterblock(fs, &mut block_control, false);
    adjustlocalvars(lexical_state, nvars);
    luak_reserveregs(fs, nvars);
    block(lexical_state);
    leaveblock(fs);
    fixforjump(fs, prep, luak_getlabel(fs), 0);
    if isgen != 0 {
        luak_code_abck(fs, OP_TFORCALL, base, 0, nvars, 0);
        luak_fixline(fs, line);
    }
    let endfor: i32 = luak_codeabx(fs, FOR_LOOP[isgen as usize], base, 0u32);
    fixforjump(fs, endfor, prep + 1, 1);
    luak_fixline(fs, line);
}}
pub unsafe extern "C" fn fornum(lexical_state: *mut LexicalState, variable_name: *mut TString, line: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let base: i32 = (*fs).freereg as i32;
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(lexical_state, variable_name);
    checknext(lexical_state, '=' as i32);
    exp1(lexical_state);
    checknext(lexical_state, ',' as i32);
    exp1(lexical_state);
    if testnext(lexical_state, ',' as i32) != 0 {
        exp1(lexical_state);
    } else {
        luak_int(fs, (*fs).freereg as i32, 1 as i64);
        luak_reserveregs(fs, 1);
    }
    adjustlocalvars(lexical_state, 3);
    forbody(lexical_state, base, line, 1, 0);
}}
pub unsafe extern "C" fn forlist(lexical_state: *mut LexicalState, indexname: *mut TString) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut e: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nvars: i32 = 5;
    let base: i32 = (*fs).freereg as i32;
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(
        lexical_state,
        luax_newstring(
            lexical_state,
            b"(for state)\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        ),
    );
    new_localvar(lexical_state, indexname);
    while testnext(lexical_state, ',' as i32) != 0 {
        new_localvar(lexical_state, str_checkname(lexical_state));
        nvars += 1;
    }
    checknext(lexical_state, TK_IN as i32);
    let line: i32 = (*lexical_state).line_number;
    adjust_assign(lexical_state, 4, explist(lexical_state, &mut e), &mut e);
    adjustlocalvars(lexical_state, 4);
    marktobeclosed(fs);
    luak_checkstack(fs, 3);
    forbody(lexical_state, base, line, nvars - 4, 1);
}}
pub unsafe extern "C" fn forstat(lexical_state: *mut LexicalState, line: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut block_control: BlockControl = BlockControl::new();
    enterblock(fs, &mut block_control, true);
    luax_next(lexical_state);
    let variable_name: *mut TString = str_checkname(lexical_state);
    match (*lexical_state).t.token {
        61 => {
            fornum(lexical_state, variable_name, line);
        }
        44 | 267 => {
            forlist(lexical_state, variable_name);
        }
        _ => {
            luax_syntaxerror(lexical_state, b"'=' or 'in' expected\0" as *const u8 as *const i8);
        }
    }
    check_match(lexical_state, TK_END as i32, TK_FOR as i32, line);
    leaveblock(fs);
}}
pub unsafe extern "C" fn test_then_block(lexical_state: *mut LexicalState, escapelist: *mut i32) { unsafe {
    let mut block_control: BlockControl = BlockControl::new();
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut v: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    let jf;
    luax_next(lexical_state);
    expr(lexical_state, &mut v);
    checknext(lexical_state, TK_THEN as i32);
    if (*lexical_state).t.token == TK_BREAK as i32 {
        let line: i32 = (*lexical_state).line_number;
        luak_goiffalse((*lexical_state).fs, &mut v);
        luax_next(lexical_state);
        enterblock(fs, &mut block_control, false);
        newgotoentry(
            lexical_state,
            luas_newlstr(
                (*lexical_state).state,
                b"break\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            ),
            line,
            v.t,
        );
        while testnext(lexical_state, ';' as i32) != 0 {}
        if block_follow(lexical_state, 0) != 0 {
            leaveblock(fs);
            return;
        } else {
            jf = luak_jump(fs);
        }
    } else {
        luak_goiftrue((*lexical_state).fs, &mut v);
        enterblock(fs, &mut block_control, false);
        jf = v.f;
    }
    statlist(lexical_state);
    leaveblock(fs);
    if (*lexical_state).t.token == TK_ELSE as i32 || (*lexical_state).t.token == TK_ELSEIF as i32 {
        luak_concat(fs, escapelist, luak_jump(fs));
    }
    luak_patchtohere(fs, jf);
}}
pub unsafe extern "C" fn ifstat(lexical_state: *mut LexicalState, line: i32) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut escapelist: i32 = -1;
    test_then_block(lexical_state, &mut escapelist);
    while (*lexical_state).t.token == TK_ELSEIF as i32 {
        test_then_block(lexical_state, &mut escapelist);
    }
    if testnext(lexical_state, TK_ELSE as i32) != 0 {
        block(lexical_state);
    }
    check_match(lexical_state, TK_END as i32, TK_IF as i32, line);
    luak_patchtohere(fs, escapelist);
}}
pub unsafe extern "C" fn localfunc(lexical_state: *mut LexicalState) { unsafe {
    let mut b: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    let fs: *mut FunctionState = (*lexical_state).fs;
    let fvar: i32 = (*fs).count_active_variables as i32;
    new_localvar(lexical_state, str_checkname(lexical_state));
    adjustlocalvars(lexical_state, 1);
    body(lexical_state, &mut b, 0, (*lexical_state).line_number);
    (*localdebuginfo(fs, fvar)).start_program_counter = (*fs).program_counter;
}}
pub unsafe extern "C" fn getlocalattribute(lexical_state: *mut LexicalState) -> i32 { unsafe {
    if testnext(lexical_state, '<' as i32) != 0 {
        let attr: *const i8 = ((*str_checkname(lexical_state)).contents).as_mut_ptr();
        checknext(lexical_state, '>' as i32);
        if strcmp(attr, b"const\0" as *const u8 as *const i8) == 0 {
            return 1;
        } else if strcmp(attr, b"close\0" as *const u8 as *const i8) == 0 {
            return 2;
        } else {
            luak_semerror(
                lexical_state,
                luao_pushfstring(
                    (*lexical_state).state,
                    b"unknown attribute '%s'\0" as *const u8 as *const i8,
                    attr,
                ),
            );
        }
    }
    return 0;
}}
pub unsafe extern "C" fn checktoclose(fs: *mut FunctionState, level: i32) { unsafe {
    if level != -1 {
        marktobeclosed(fs);
        luak_code_abck(fs, OP_TBC, reglevel(fs, level), 0, 0, 0);
    }
}}
pub unsafe extern "C" fn localstat(lexical_state: *mut LexicalState) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut toclose: i32 = -1;
    let var: *mut VariableDescription;
    let mut vidx: i32;
    let mut kind: i32;
    let mut nvars: i32 = 0;
    let nexps: i32;
    let mut e: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        vidx = new_localvar(lexical_state, str_checkname(lexical_state));
        kind = getlocalattribute(lexical_state);
        (*getlocalvardesc(fs, vidx)).vd.kind = kind as u8;
        if kind == 2 {
            if toclose != -1 {
                luak_semerror(
                    lexical_state,
                    b"multiple to-be-closed variables in local list\0" as *const u8 as *const i8,
                );
            }
            toclose = (*fs).count_active_variables as i32 + nvars;
        }
        nvars += 1;
        if !(testnext(lexical_state, ',' as i32) != 0) {
            break;
        }
    }
    if testnext(lexical_state, '=' as i32) != 0 {
        nexps = explist(lexical_state, &mut e);
    } else {
        e.k = VVOID;
        nexps = 0;
    }
    var = getlocalvardesc(fs, vidx);
    if nvars == nexps
        && (*var).vd.kind as i32 == 1
        && luak_exp2const(fs, &mut e, &mut (*var).k) != 0
    {
        (*var).vd.kind = 3 as u8;
        adjustlocalvars(lexical_state, nvars - 1);
        (*fs).count_active_variables = ((*fs).count_active_variables).wrapping_add(1);
        (*fs).count_active_variables;
    } else {
        adjust_assign(lexical_state, nvars, nexps, &mut e);
        adjustlocalvars(lexical_state, nvars);
    }
    checktoclose(fs, toclose);
}}
pub unsafe extern "C" fn funcname(lexical_state: *mut LexicalState, v: *mut ExpressionDescription) -> i32 { unsafe {
    let mut ismethod: i32 = 0;
    singlevar(lexical_state, v);
    while (*lexical_state).t.token == '.' as i32 {
        fieldsel(lexical_state, v);
    }
    if (*lexical_state).t.token == ':' as i32 {
        ismethod = 1;
        fieldsel(lexical_state, v);
    }
    return ismethod;
}}
pub unsafe extern "C" fn funcstat(lexical_state: *mut LexicalState, line: i32) { unsafe {
    let mut v: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    luax_next(lexical_state);
    let ismethod: i32 = funcname(lexical_state, &mut v);
    body(lexical_state, &mut b, ismethod, line);
    check_readonly(lexical_state, &mut v);
    luak_storevar((*lexical_state).fs, &mut v, &mut b);
    luak_fixline((*lexical_state).fs, line);
}}
pub unsafe extern "C" fn exprstat(lexical_state: *mut LexicalState) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut v: LHSAssign = LHSAssign {
        previous: std::ptr::null_mut(),
        v: ExpressionDescription {
            k: VVOID,
            u: RawValue { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(lexical_state, &mut v.v);
    if (*lexical_state).t.token == '=' as i32 || (*lexical_state).t.token == ',' as i32 {
        v.previous = std::ptr::null_mut();
        restassign(lexical_state, &mut v, 1);
    } else {
        if !(v.v.k as u32 == VCALL as i32 as u32) {
            luax_syntaxerror(lexical_state, b"syntax error\0" as *const u8 as *const i8);
        }
        let inst: *mut u32 = &mut *((*(*fs).f).code).offset(v.v.u.info as isize) as *mut u32;
        *inst = *inst & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
            | (1 as u32) << 0 + 7 + 8 + 1 + 8
                & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
    };
}}
pub unsafe extern "C" fn retstat(lexical_state: *mut LexicalState) { unsafe {
    let fs: *mut FunctionState = (*lexical_state).fs;
    let mut e: ExpressionDescription = ExpressionDescription {
        k: VVOID,
        u: RawValue { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nret: i32;
    let mut first: i32 = luay_nvarstack(fs);
    if block_follow(lexical_state, 1) != 0 || (*lexical_state).t.token == ';' as i32 {
        nret = 0;
    } else {
        nret = explist(lexical_state, &mut e);
        if e.k as u32 == VCALL as i32 as u32 || e.k as u32 == VVARARG as i32 as u32 {
            luak_setreturns(fs, &mut e, -1);
            if e.k as u32 == VCALL as i32 as u32
                && nret == 1
                && !(*(*fs).block_control).is_inside_tbc
            {
                *((*(*fs).f).code).offset(e.u.info as isize) =
                    *((*(*fs).f).code).offset(e.u.info as isize) & !(!(!(0u32) << 7) << 0)
                        | (OP_TAILCALL as i32 as u32) << 0 & !(!(0u32) << 7) << 0;
            }
            nret = -1;
        } else if nret == 1 {
            first = luak_exp2anyreg(fs, &mut e);
        } else {
            luak_exp2nextreg(fs, &mut e);
        }
    }
    luak_ret(fs, first, nret);
    testnext(lexical_state, ';' as i32);
}}
pub unsafe extern "C" fn statement(lexical_state: *mut LexicalState) { unsafe {
    let line: i32 = (*lexical_state).line_number;
    luae_inccstack((*lexical_state).state);
    match (*lexical_state).t.token {
        CHARACTER_SEMICOLON => {
            luax_next(lexical_state);
        }
        TK_IF => {
            ifstat(lexical_state, line);
        }
        TK_WHILE => {
            whilestat(lexical_state, line);
        }
        TK_DO => {
            luax_next(lexical_state);
            block(lexical_state);
            check_match(lexical_state, TK_END as i32, TK_DO as i32, line);
        }
        TK_FOR => {
            forstat(lexical_state, line);
        }
        TK_REPEAT => {
            repeatstat(lexical_state, line);
        }
        TK_FUNCTION => {
            funcstat(lexical_state, line);
        }
        TK_LOCAL => {
            luax_next(lexical_state);
            if testnext(lexical_state, TK_FUNCTION as i32) != 0 {
                localfunc(lexical_state);
            } else {
                localstat(lexical_state);
            }
        }
        TK_DBCOLON => {
            luax_next(lexical_state);
            labelstat(lexical_state, str_checkname(lexical_state), line);
        }
        TK_RETURN => {
            luax_next(lexical_state);
            retstat(lexical_state);
        }
        TK_BREAK => {
            breakstat(lexical_state);
        }
        TK_GOTO => {
            luax_next(lexical_state);
            gotostat(lexical_state);
        }
        _ => {
            exprstat(lexical_state);
        }
    }
    (*(*lexical_state).fs).freereg = luay_nvarstack((*lexical_state).fs) as u8;
    (*(*lexical_state).state).count_c_calls = ((*(*lexical_state).state).count_c_calls).wrapping_sub(1);
    (*(*lexical_state).state).count_c_calls;
}}
pub unsafe extern "C" fn mainfunc(lexical_state: *mut LexicalState, fs: *mut FunctionState) { unsafe {
    let mut block_control: BlockControl = BlockControl::new();
    let env: *mut Upvaldesc;
    open_func(lexical_state, fs, &mut block_control);
    setvararg(fs, 0);
    env = allocupvalue(fs);
    (*env).is_in_stack = true;
    (*env).index = 0;
    (*env).kind = 0;
    (*env).name = (*lexical_state).envn;
    if (*(*fs).f).get_marked() & 1 << 5 != 0
        && (*(*env).name).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            (*lexical_state).state,
            &mut (*((*fs).f as *mut GCUnion)).object,
            &mut (*((*env).name as *mut GCUnion)).object,
        );
    } else {
    };
    luax_next(lexical_state);
    statlist(lexical_state);
    check(lexical_state, TK_EOS as i32);
    close_func(lexical_state);
}}
pub unsafe extern "C" fn luay_parser(
    state: *mut State,
    zio: *mut ZIO,
    buffer: *mut Buffer,
    dynamic_data: *mut DynamicData,
    name: *const i8,
    firstchar: i32,
) -> *mut LClosure { unsafe {
    let mut lexstate: LexicalState = LexicalState::new();
    let mut funcstate: FunctionState = FunctionState::new();
    let cl: *mut LClosure = luaf_newlclosure(state, 1);
    let io: *mut TValue = &mut (*(*state).top.p).value;
    let x_: *mut LClosure = cl;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_CLOSURE_L);
    (*io).set_collectable ();
    luad_inctop(state);
    lexstate.h = luah_new(state);
    let io_0: *mut TValue = &mut (*(*state).top.p).value;
    let x0: *mut Table = lexstate.h;
    (*io_0).value.object = &mut (*(x0 as *mut GCUnion)).object;
    (*io_0).set_tag(TAG_VARIANT_TABLE);
    (*io_0).set_collectable();
    luad_inctop(state);
    (*cl).p = luaf_newproto(state);
    funcstate.f = (*cl).p;
    if (*cl).get_marked() & 1 << 5 != 0
        && (*(*cl).p).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            state,
            &mut (*(cl as *mut GCUnion)).object,
            &mut (*((*cl).p as *mut GCUnion)).object,
        );
    } else {
    };
    (*funcstate.f).source = luas_new(state, name);
    if (*funcstate.f).get_marked() & 1 << 5 != 0
        && (*(*funcstate.f).source).get_marked() & (1 << 3 | 1 << 4) != 0
    {
        luac_barrier_(
            state,
            &mut (*(funcstate.f as *mut GCUnion)).object,
            &mut (*((*funcstate.f).source as *mut GCUnion)).object,
        );
    } else {
    };
    lexstate.buffer = buffer;
    lexstate.dynamic_data = dynamic_data;
    (*dynamic_data).label.n = 0;
    (*dynamic_data).gt.n = (*dynamic_data).label.n;
    (*dynamic_data).active_variable.n = (*dynamic_data).gt.n;
    luax_setinput(state, &mut lexstate, zio, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    (*state).top.p = (*state).top.p.offset(-1);
    return cl;
}}
pub unsafe extern "C" fn save(lexical_state: *mut LexicalState, c: i32) { unsafe {
    let b: *mut Buffer = (*lexical_state).buffer;
    if ((*b).length).wrapping_add(1 as u64) > (*b).size {
        if (*b).size
        >= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
                !(0u64)
            } else {
                9223372036854775807 as i64 as u64
            })
            .wrapping_div(2 as u64)
        {
            lexerror(
                lexical_state,
                b"lexical element too long\0" as *const u8 as *const i8,
                0,
            );
        }
        let new_size: u64 = ((*b).size).wrapping_mul(2 as u64);
        (*b).pointer = luam_saferealloc_(
            (*lexical_state).state,
            (*b).pointer as *mut libc::c_void,
            ((*b).size).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            new_size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ) as *mut i8;
        (*b).size = new_size;
    }
    let fresh49 = (*b).length;
    (*b).length = ((*b).length).wrapping_add(1);
    *((*b).pointer).offset(fresh49 as isize) = c as i8;
}}
pub unsafe extern "C" fn luax_init(state: *mut State) { unsafe {
    let mut i: i32;
    let e: *mut TString = luas_newlstr(
        state,
        b"_ENV\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64),
    );
    luac_fix(state, &mut (*(e as *mut GCUnion)).object);
    i = 0;
    while i < TK_WHILE as i32 - (127 as i32 * 2 + 1 + 1) + 1 {
        let ts: *mut TString = luas_new(state, TOKENS[i as usize]);
        luac_fix(state, &mut (*(ts as *mut GCUnion)).object);
        (*ts).extra = (i + 1) as u8;
        i += 1;
    }
}}
pub unsafe extern "C" fn luax_token2str(lexical_state: *mut LexicalState, token: i32) -> *const i8 { unsafe {
    if token < 127 as i32 * 2 + 1 + 1 {
        if CHARACTER_TYPE[(token + 1) as usize] as i32 & 1 << 2 != 0 {
            return luao_pushfstring((*lexical_state).state, b"'%c'\0" as *const u8 as *const i8, token);
        } else {
            return luao_pushfstring((*lexical_state).state, b"'<\\%d>'\0" as *const u8 as *const i8, token);
        }
    } else {
        let s: *const i8 = TOKENS[(token - (127 as i32 * 2 + 1 + 1)) as usize];
        if token < TK_EOS as i32 {
            return luao_pushfstring((*lexical_state).state, b"'%s'\0" as *const u8 as *const i8, s);
        } else {
            return s;
        }
    };
}}
pub unsafe extern "C" fn text_token(lexical_state: *mut LexicalState, token: i32) -> *const i8 { unsafe {
    match token {
        291 | 292 | 289 | 290 => {
            save(lexical_state, '\0' as i32);
            return luao_pushfstring(
                (*lexical_state).state,
                b"'%s'\0" as *const u8 as *const i8,
                (*(*lexical_state).buffer).pointer,
            );
        }
        _ => return luax_token2str(lexical_state, token),
    };
}}
pub unsafe extern "C" fn lexerror(lexical_state: *mut LexicalState, mut message: *const i8, token: i32) -> ! { unsafe {
    message = luag_addinfo((*lexical_state).state, message, (*lexical_state).source, (*lexical_state).line_number);
    if token != 0 {
        luao_pushfstring(
            (*lexical_state).state,
            b"%s near %s\0" as *const u8 as *const i8,
            message,
            text_token(lexical_state, token),
        );
    }
    luad_throw((*lexical_state).state, 3);
}}
pub unsafe extern "C" fn luax_syntaxerror(lexical_state: *mut LexicalState, message: *const i8) -> ! { unsafe {
    lexerror(lexical_state, message, (*lexical_state).t.token);
}}
pub unsafe extern "C" fn luax_newstring(
    lexical_state: *mut LexicalState,
    str: *const i8,
    l: u64,
) -> *mut TString { unsafe {
    let state: *mut State = (*lexical_state).state;
    let mut ts: *mut TString = luas_newlstr(state, str, l);
    let o: *const TValue = luah_getstr((*lexical_state).h, ts);
    if !(get_tag_type((*o).get_tag()) == TAG_TYPE_NIL) {
        ts = &mut (*((*(o as *mut Node)).key.value.object as *mut GCUnion)).ts;
    } else {
        let fresh50 = (*state).top.p;
        (*state).top.p = (*state).top.p.offset(1);
        let stv: *mut TValue = &mut (*fresh50).value;
        let io: *mut TValue = stv;
        let x_: *mut TString = ts;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        luah_finishset(state, (*lexical_state).h, stv, o, stv);
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        (*state).top.p = (*state).top.p.offset(-1);
        }
    return ts;
}}
pub unsafe extern "C" fn inclinenumber(lexical_state: *mut LexicalState) { unsafe {
    let old: i32 = (*lexical_state).current;
    let fresh51 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh51 > 0u64 {
        let fresh52 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh52 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    if ((*lexical_state).current == '\n' as i32 || (*lexical_state).current == '\r' as i32) && (*lexical_state).current != old {
        let fresh53 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh53 > 0u64 {
            let fresh54 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh54 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
    }
    (*lexical_state).line_number += 1;
    if (*lexical_state).line_number >= 2147483647 as i32 {
        lexerror(
            lexical_state,
            b"chunk has too many lines\0" as *const u8 as *const i8,
            0,
        );
    }
}}
pub unsafe extern "C" fn luax_setinput(
    state: *mut State,
    lexical_state: *mut LexicalState,
    zio: *mut ZIO,
    source: *mut TString,
    firstchar: i32,
) { unsafe {
    (*lexical_state).t.token = 0;
    (*lexical_state).state = state;
    (*lexical_state).current = firstchar;
    (*lexical_state).look_ahead.token = TK_EOS as i32;
    (*lexical_state).zio = zio;
    (*lexical_state).fs = std::ptr::null_mut();
    (*lexical_state).line_number = 1;
    (*lexical_state).last_line = 1;
    (*lexical_state).source = source;
    (*lexical_state).envn = luas_newlstr(
        state,
        b"_ENV\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64),
    );
    (*(*lexical_state).buffer).pointer = luam_saferealloc_(
        (*lexical_state).state,
        (*(*lexical_state).buffer).pointer as *mut libc::c_void,
        ((*(*lexical_state).buffer).size).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        (32 as i32 as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    (*(*lexical_state).buffer).size = 32 as i32 as u64;
}}
pub unsafe extern "C" fn check_next1(lexical_state: *mut LexicalState, c: i32) -> i32 { unsafe {
    if (*lexical_state).current == c {
        let fresh55 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh55 > 0u64 {
            let fresh56 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh56 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn check_next2(lexical_state: *mut LexicalState, set: *const i8) -> i32 { unsafe {
    if (*lexical_state).current == *set.offset(0 as isize) as i32
        || (*lexical_state).current == *set.offset(1 as isize) as i32
    {
        save(lexical_state, (*lexical_state).current);
        let fresh57 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh57 > 0u64 {
            let fresh58 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh58 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn read_numeral(
    lexical_state: *mut LexicalState,
    semantic_info: *mut SemanticInfo,
) -> i32 { unsafe {
    let mut obj: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let mut expo: *const i8 = b"Ee\0" as *const u8 as *const i8;
    let first: i32 = (*lexical_state).current;
    save(lexical_state, (*lexical_state).current);
    let fresh59 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh59 > 0u64 {
        let fresh60 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh60 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    if first == '0' as i32 && check_next2(lexical_state, b"xX\0" as *const u8 as *const i8) != 0 {
        expo = b"Pp\0" as *const u8 as *const i8;
    }
    loop {
        if check_next2(lexical_state, expo) != 0 {
            check_next2(lexical_state, b"-+\0" as *const u8 as *const i8);
        } else {
            if !(CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 4 != 0
                || (*lexical_state).current == '.' as i32)
            {
                break;
            }
            save(lexical_state, (*lexical_state).current);
            let fresh61 = (*(*lexical_state).zio).n;
            (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
            (*lexical_state).current = if fresh61 > 0u64 {
                let fresh62 = (*(*lexical_state).zio).p;
                (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                *fresh62 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
        }
    }
    if CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 0 != 0 {
        save(lexical_state, (*lexical_state).current);
        let fresh63 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh63 > 0u64 {
            let fresh64 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh64 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
    }
    save(lexical_state, '\0' as i32);
    if luao_str2num((*(*lexical_state).buffer).pointer, &mut obj) == 0u64 {
        lexerror(
            lexical_state,
            b"malformed number\0" as *const u8 as *const i8,
            TK_FLT as i32,
        );
    }
    if obj.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        (*semantic_info).i = obj.value.i;
        return TK_INT as i32;
    } else {
        (*semantic_info).r = obj.value.n;
        return TK_FLT as i32;
    };
}}
pub unsafe extern "C" fn skip_sep(lexical_state: *mut LexicalState) -> u64 { unsafe {
    let mut count: u64 = 0;
    let s: i32 = (*lexical_state).current;
    save(lexical_state, (*lexical_state).current);
    let fresh65 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh65 > 0u64 {
        let fresh66 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh66 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    while (*lexical_state).current == '=' as i32 {
        save(lexical_state, (*lexical_state).current);
        let fresh67 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh67 > 0u64 {
            let fresh68 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh68 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        count = count.wrapping_add(1);
    }
    return if (*lexical_state).current == s {
        count.wrapping_add(2 as u64)
    } else {
        (if count == 0u64 { 1 } else { 0 }) as u64
    };
}}
pub unsafe extern "C" fn read_long_string(
    lexical_state: *mut LexicalState,
    semantic_info: *mut SemanticInfo,
    sep: u64,
) { unsafe {
    let line: i32 = (*lexical_state).line_number;
    save(lexical_state, (*lexical_state).current);
    let fresh69 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh69 > 0u64 {
        let fresh70 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh70 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    if (*lexical_state).current == '\n' as i32 || (*lexical_state).current == '\r' as i32 {
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
                    (*lexical_state).state,
                    b"unfinished long %s (starting at line %d)\0" as *const u8 as *const i8,
                    what,
                    line,
                );
                lexerror(lexical_state, message, TK_EOS as i32);
            }
            93 => {
                if !(skip_sep(lexical_state) == sep) {
                    continue;
                }
                save(lexical_state, (*lexical_state).current);
                let fresh71 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh71 > 0u64 {
                    let fresh72 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh72 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                break;
            }
            10 | 13 => {
                save(lexical_state, '\n' as i32);
                inclinenumber(lexical_state);
                if semantic_info.is_null() {
                    (*(*lexical_state).buffer).length = 0;
                }
            }
            _ => {
                if !semantic_info.is_null() {
                    save(lexical_state, (*lexical_state).current);
                    let fresh73 = (*(*lexical_state).zio).n;
                    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                    (*lexical_state).current = if fresh73 > 0u64 {
                        let fresh74 = (*(*lexical_state).zio).p;
                        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                        *fresh74 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                } else {
                    let fresh75 = (*(*lexical_state).zio).n;
                    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                    (*lexical_state).current = if fresh75 > 0u64 {
                        let fresh76 = (*(*lexical_state).zio).p;
                        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                        *fresh76 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                }
            }
        }
    }
    if !semantic_info.is_null() {
        (*semantic_info).ts = luax_newstring(
            lexical_state,
            ((*(*lexical_state).buffer).pointer).offset(sep as isize),
            ((*(*lexical_state).buffer).length).wrapping_sub((2 as u64).wrapping_mul(sep)),
        );
    }
}}
pub unsafe extern "C" fn esccheck(lexical_state: *mut LexicalState, c: i32, message: *const i8) { unsafe {
    if c == 0 {
        if (*lexical_state).current != -1 {
            save(lexical_state, (*lexical_state).current);
            let fresh77 = (*(*lexical_state).zio).n;
            (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
            (*lexical_state).current = if fresh77 > 0u64 {
                let fresh78 = (*(*lexical_state).zio).p;
                (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                *fresh78 as u8 as i32
            } else {
                luaz_fill((*lexical_state).zio)
            };
        }
        lexerror(lexical_state, message, TK_STRING as i32);
    }
}}
pub unsafe extern "C" fn gethexa(lexical_state: *mut LexicalState) -> i32 { unsafe {
    save(lexical_state, (*lexical_state).current);
    let fresh79 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh79 > 0u64 {
        let fresh80 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh80 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    esccheck(
        lexical_state,
        CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 4,
        b"hexadecimal digit expected\0" as *const u8 as *const i8,
    );
    return luao_hexavalue((*lexical_state).current);
}}
pub unsafe extern "C" fn readhexaesc(lexical_state: *mut LexicalState) -> i32 { unsafe {
    let mut r: i32 = gethexa(lexical_state);
    r = (r << 4) + gethexa(lexical_state);
    (*(*lexical_state).buffer).length =
        ((*(*lexical_state).buffer).length as u64).wrapping_sub(2 as u64) as u64 as u64;
    return r;
}}
pub unsafe extern "C" fn readutf8esc(lexical_state: *mut LexicalState) -> u64 { unsafe {
    let mut i: i32 = 4;
    save(lexical_state, (*lexical_state).current);
    let fresh81 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh81 > 0u64 {
        let fresh82 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh82 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    esccheck(
        lexical_state,
        ((*lexical_state).current == '{' as i32) as i32,
        b"missing '{'\0" as *const u8 as *const i8,
    );
    let mut r: u64 = gethexa(lexical_state) as u64;
    loop {
        save(lexical_state, (*lexical_state).current);
        let fresh83 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh83 > 0u64 {
            let fresh84 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh84 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        if !(CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 4 != 0) {
            break;
        }
        i += 1;
        esccheck(
            lexical_state,
            (r <= (0x7fffffff as u32 >> 4) as u64) as i32,
            b"UTF-8 value too large\0" as *const u8 as *const i8,
        );
        r = (r << 4).wrapping_add(luao_hexavalue((*lexical_state).current) as u64);
    }
    esccheck(
        lexical_state,
        ((*lexical_state).current == '}' as i32) as i32,
        b"missing '}'\0" as *const u8 as *const i8,
    );
    let fresh85 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh85 > 0u64 {
        let fresh86 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh86 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    (*(*lexical_state).buffer).length = ((*(*lexical_state).buffer).length as u64).wrapping_sub(i as u64) as u64 as u64;
    return r;
}}
pub unsafe extern "C" fn utf8esc(lexical_state: *mut LexicalState) { unsafe {
    let mut buffer: [i8; 8] = [0; 8];
    let mut n: i32 = luao_utf8esc(buffer.as_mut_ptr(), readutf8esc(lexical_state));
    while n > 0 {
        save(lexical_state, buffer[(8 - n) as usize] as i32);
        n -= 1;
    }
}}
pub unsafe extern "C" fn readdecesc(lexical_state: *mut LexicalState) -> i32 { unsafe {
    let mut i: i32;
    let mut r: i32 = 0;
    i = 0;
    while i < 3 && CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 1 != 0 {
        r = 10 as i32 * r + (*lexical_state).current - '0' as i32;
        save(lexical_state, (*lexical_state).current);
        let fresh87 = (*(*lexical_state).zio).n;
        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
        (*lexical_state).current = if fresh87 > 0u64 {
            let fresh88 = (*(*lexical_state).zio).p;
            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
            *fresh88 as u8 as i32
        } else {
            luaz_fill((*lexical_state).zio)
        };
        i += 1;
    }
    esccheck(
        lexical_state,
        (r <= 127 as i32 * 2 + 1) as i32,
        b"decimal escape too large\0" as *const u8 as *const i8,
    );
    (*(*lexical_state).buffer).length = ((*(*lexical_state).buffer).length as u64).wrapping_sub(i as u64) as u64 as u64;
    return r;
}}
pub unsafe extern "C" fn read_string(
    lexical_state: *mut LexicalState,
    del: i32,
    semantic_info: *mut SemanticInfo,
) { unsafe {
    let mut current_block: u64;
    save(lexical_state, (*lexical_state).current);
    let fresh89 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh89 > 0u64 {
        let fresh90 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
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
            10 | 13 => {
                lexerror(
                    lexical_state,
                    b"unfinished string\0" as *const u8 as *const i8,
                    TK_STRING as i32,
                );
            }
            92 => {
                let c: i32;
                save(lexical_state, (*lexical_state).current);
                let fresh91 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh91 > 0u64 {
                    let fresh92 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh92 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                match (*lexical_state).current {
                    97 => {
                        c = '\u{7}' as i32;
                        current_block = 15029063370732930705;
                    }
                    98 => {
                        c = '\u{8}' as i32;
                        current_block = 15029063370732930705;
                    }
                    102 => {
                        c = '\u{c}' as i32;
                        current_block = 15029063370732930705;
                    }
                    110 => {
                        c = '\n' as i32;
                        current_block = 15029063370732930705;
                    }
                    114 => {
                        c = '\r' as i32;
                        current_block = 15029063370732930705;
                    }
                    116 => {
                        c = '\t' as i32;
                        current_block = 15029063370732930705;
                    }
                    118 => {
                        c = '\u{b}' as i32;
                        current_block = 15029063370732930705;
                    }
                    120 => {
                        c = readhexaesc(lexical_state);
                        current_block = 15029063370732930705;
                    }
                    117 => {
                        utf8esc(lexical_state);
                        continue;
                    }
                    10 | 13 => {
                        inclinenumber(lexical_state);
                        c = '\n' as i32;
                        current_block = 7010296663004816197;
                    }
                    92 | 34 | 39 => {
                        c = (*lexical_state).current;
                        current_block = 15029063370732930705;
                    }
                    -1 => {
                        continue;
                    }
                    122 => {
                        (*(*lexical_state).buffer).length =
                            ((*(*lexical_state).buffer).length as u64).wrapping_sub(1 as u64) as u64 as u64;
                        let fresh93 = (*(*lexical_state).zio).n;
                        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                        (*lexical_state).current = if fresh93 > 0u64 {
                            let fresh94 = (*(*lexical_state).zio).p;
                            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                            *fresh94 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                        while CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 3
                            != 0
                        {
                            if (*lexical_state).current == '\n' as i32 || (*lexical_state).current == '\r' as i32 {
                                inclinenumber(lexical_state);
                            } else {
                                let fresh95 = (*(*lexical_state).zio).n;
                                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                                (*lexical_state).current = if fresh95 > 0u64 {
                                    let fresh96 = (*(*lexical_state).zio).p;
                                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
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
                            CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 1,
                            b"invalid escape sequence\0" as *const u8 as *const i8,
                        );
                        c = readdecesc(lexical_state);
                        current_block = 7010296663004816197;
                    }
                }
                match current_block {
                    15029063370732930705 => {
                        let fresh97 = (*(*lexical_state).zio).n;
                        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                        (*lexical_state).current = if fresh97 > 0u64 {
                            let fresh98 = (*(*lexical_state).zio).p;
                            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                            *fresh98 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                    }
                    _ => {}
                }
                (*(*lexical_state).buffer).length =
                    ((*(*lexical_state).buffer).length as u64).wrapping_sub(1 as u64) as u64 as u64;
                save(lexical_state, c);
            }
            _ => {
                save(lexical_state, (*lexical_state).current);
                let fresh99 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh99 > 0u64 {
                    let fresh100 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh100 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
            }
        }
    }
    save(lexical_state, (*lexical_state).current);
    let fresh101 = (*(*lexical_state).zio).n;
    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
    (*lexical_state).current = if fresh101 > 0u64 {
        let fresh102 = (*(*lexical_state).zio).p;
        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
        *fresh102 as u8 as i32
    } else {
        luaz_fill((*lexical_state).zio)
    };
    (*semantic_info).ts = luax_newstring(
        lexical_state,
        ((*(*lexical_state).buffer).pointer).offset(1 as isize),
        ((*(*lexical_state).buffer).length).wrapping_sub(2 as u64),
    );
}}
pub unsafe extern "C" fn llex(lexical_state: *mut LexicalState, semantic_info: *mut SemanticInfo) -> i32 { unsafe {
    (*(*lexical_state).buffer).length = 0;
    loop {
        let current_block_85: u64;
        match (*lexical_state).current {
            10 | 13 => {
                inclinenumber(lexical_state);
            }
            32 | 12 | 9 | 11 => {
                let fresh103 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh103 > 0u64 {
                    let fresh104 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh104 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
            }
            45 => {
                let fresh105 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh105 > 0u64 {
                    let fresh106 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh106 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if (*lexical_state).current != '-' as i32 {
                    return '-' as i32;
                }
                let fresh107 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh107 > 0u64 {
                    let fresh108 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh108 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if (*lexical_state).current == '[' as i32 {
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
                        while !((*lexical_state).current == '\n' as i32 || (*lexical_state).current == '\r' as i32)
                            && (*lexical_state).current != -1
                        {
                            let fresh109 = (*(*lexical_state).zio).n;
                            (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                            (*lexical_state).current = if fresh109 > 0u64 {
                                let fresh110 = (*(*lexical_state).zio).p;
                                (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                                *fresh110 as u8 as i32
                            } else {
                                luaz_fill((*lexical_state).zio)
                            };
                        }
                    }
                }
            }
            91 => {
                let sep_0: u64 = skip_sep(lexical_state);
                if sep_0 >= 2 as u64 {
                    read_long_string(lexical_state, semantic_info, sep_0);
                    return TK_STRING as i32;
                } else if sep_0 == 0u64 {
                    lexerror(
                        lexical_state,
                        b"invalid long string delimiter\0" as *const u8 as *const i8,
                        TK_STRING as i32,
                    );
                }
                return '[' as i32;
            }
            61 => {
                let fresh111 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh111 > 0u64 {
                    let fresh112 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh112 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '=' as i32) != 0 {
                    return TK_EQ as i32;
                } else {
                    return '=' as i32;
                }
            }
            60 => {
                let fresh113 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh113 > 0u64 {
                    let fresh114 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh114 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '=' as i32) != 0 {
                    return TK_LE as i32;
                } else if check_next1(lexical_state, '<' as i32) != 0 {
                    return TK_SHL as i32;
                } else {
                    return '<' as i32;
                }
            }
            62 => {
                let fresh115 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh115 > 0u64 {
                    let fresh116 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh116 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '=' as i32) != 0 {
                    return TK_GE as i32;
                } else if check_next1(lexical_state, '>' as i32) != 0 {
                    return TK_SHR as i32;
                } else {
                    return '>' as i32;
                }
            }
            47 => {
                let fresh117 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh117 > 0u64 {
                    let fresh118 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh118 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '/' as i32) != 0 {
                    return TK_IDIV as i32;
                } else {
                    return '/' as i32;
                }
            }
            126 => {
                let fresh119 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh119 > 0u64 {
                    let fresh120 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh120 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '=' as i32) != 0 {
                    return TK_NE as i32;
                } else {
                    return '~' as i32;
                }
            }
            CHARACTER_COLON => {
                let fresh121 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh121 > 0u64 {
                    let fresh122 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh122 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, ':' as i32) != 0 {
                    return TK_DBCOLON as i32;
                } else {
                    return ':' as i32;
                }
            }
            34 | 39 => {
                read_string(lexical_state, (*lexical_state).current, semantic_info);
                return TK_STRING as i32;
            }
            46 => {
                save(lexical_state, (*lexical_state).current);
                let fresh123 = (*(*lexical_state).zio).n;
                (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                (*lexical_state).current = if fresh123 > 0u64 {
                    let fresh124 = (*(*lexical_state).zio).p;
                    (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                    *fresh124 as u8 as i32
                } else {
                    luaz_fill((*lexical_state).zio)
                };
                if check_next1(lexical_state, '.' as i32) != 0 {
                    if check_next1(lexical_state, '.' as i32) != 0 {
                        return TK_DOTS as i32;
                    } else {
                        return TK_CONCAT as i32;
                    }
                } else if CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 1 == 0
                {
                    return '.' as i32;
                } else {
                    return read_numeral(lexical_state, semantic_info);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return read_numeral(lexical_state, semantic_info);
            }
            -1 => return TK_EOS as i32,
            _ => {
                if CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32 & 1 << 0 != 0 {
                    loop {
                        save(lexical_state, (*lexical_state).current);
                        let fresh125 = (*(*lexical_state).zio).n;
                        (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                        (*lexical_state).current = if fresh125 > 0u64 {
                            let fresh126 = (*(*lexical_state).zio).p;
                            (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                            *fresh126 as u8 as i32
                        } else {
                            luaz_fill((*lexical_state).zio)
                        };
                        if !(CHARACTER_TYPE[((*lexical_state).current + 1) as usize] as i32
                            & (1 << 0 | 1 << 1)
                            != 0)
                        {
                            break;
                        }
                    }
                    let ts: *mut TString = luax_newstring(lexical_state, (*(*lexical_state).buffer).pointer, (*(*lexical_state).buffer).length);
                    (*semantic_info).ts = ts;
                    if (*ts).get_tag() == TAG_VARIANT_STRING_SHORT && (*ts).extra as i32 > 0 {
                        return (*ts).extra as i32 - 1 + (127 as i32 * 2 + 1 + 1);
                    } else {
                        return TK_NAME as i32;
                    }
                } else {
                    let c: i32 = (*lexical_state).current;
                    let fresh127 = (*(*lexical_state).zio).n;
                    (*(*lexical_state).zio).n = ((*(*lexical_state).zio).n).wrapping_sub(1);
                    (*lexical_state).current = if fresh127 > 0u64 {
                        let fresh128 = (*(*lexical_state).zio).p;
                        (*(*lexical_state).zio).p = ((*(*lexical_state).zio).p).offset(1);
                        *fresh128 as u8 as i32
                    } else {
                        luaz_fill((*lexical_state).zio)
                    };
                    return c;
                }
            }
        }
    }
}}
pub unsafe extern "C" fn luax_next(lexical_state: *mut LexicalState) { unsafe {
    (*lexical_state).last_line = (*lexical_state).line_number;
    if (*lexical_state).look_ahead.token != TK_EOS as i32 {
        (*lexical_state).t = (*lexical_state).look_ahead;
        (*lexical_state).look_ahead.token = TK_EOS as i32;
    } else {
        (*lexical_state).t.token = llex(lexical_state, &mut (*lexical_state).t.semantic_info);
    };
}}
pub unsafe extern "C" fn luax_lookahead(lexical_state: *mut LexicalState) -> i32 { unsafe {
    (*lexical_state).look_ahead.token = llex(lexical_state, &mut (*lexical_state).look_ahead.semantic_info);
    return (*lexical_state).look_ahead.token;
}}
static mut DUMMY_NODE: Node = Node {
    value: TValue {
        tag: TAG_VARIANT_NIL_EMPTY,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    key: TValue {
        tag: TAG_VARIANT_NIL_NIL,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    next: 0,
};
static mut ABSENT_KEY: TValue = {
    let init = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: TAG_VARIANT_NIL_ABSENTKEY,
    };
    init
};
pub unsafe extern "C" fn hashint(t: *const Table, i: i64) -> *mut Node { unsafe {
    let ui: u64 = i as u64;
    if ui <= 2147483647 as i32 as u32 as u64 {
        return &mut *((*t).node)
            .offset((ui as i32 % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize)
            as *mut Node;
    } else {
        return &mut *((*t).node).offset(
            ui.wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u64) as isize,
        ) as *mut Node;
    };
}}
fn frexp_(x: f64) -> (f64, i32) {
    if x == 0.0 {
        return (0.0, 0);
    } else {
        let bits = x.to_bits();
        let sign = if (bits >> 63) != 0 { -1.0 } else { 1.0 };
        let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
        let mantissa = sign * f64::from_bits((bits & 0xfffffffffffff) | 0x3fe0000000000000);
        return (mantissa, exponent + 1);
    }
}
fn ldexp_(x: f64, exp: i32) -> f64 {
    if x == 0.0 || exp == 0 {
        return x;
    } else {
        let bits = x.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let new_exponent = exponent + exp;
        if !(0..=0x7ff).contains(&new_exponent) {
            return if (bits >> 63) != 0 {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            };
        } else {
            let result_bits = (bits & 0x800fffffffffffff) | ((new_exponent as u64) << 52);
            return f64::from_bits(result_bits);
        }
    }
}
pub unsafe extern "C" fn l_hashfloat(mut n: f64) -> i32 {
    let i: i32;
    let mut ni: i64 = 0;
    (n, i) = frexp_(n);
    n = n * -((-(2147483647 as i32) - 1) as f64);
    if !(n >= (-(9223372036854775807 as i64) - 1 as i64) as f64
        && n < -((-(9223372036854775807 as i64) - 1 as i64) as f64)
        && {
            ni = n as i64;
            1 != 0
        })
    {
        return 0;
    } else {
        let u: u32 = (i as u32).wrapping_add(ni as u32);
        return (if u <= 2147483647 as i32 as u32 { u } else { !u }) as i32;
    };
}
pub unsafe extern "C" fn mainpositiontv(t: *const Table, key: *const TValue) -> *mut Node { unsafe {
    match (*key).get_tag_variant() {
        TAG_VARIANT_NUMERIC_INTEGER => {
            let i: i64 = (*key).value.i;
            return hashint(t, i);
        }
        TAG_VARIANT_NUMERIC_NUMBER => {
            let n: f64 = (*key).value.n;
            return &mut *((*t).node).offset(
                ((l_hashfloat as unsafe extern "C" fn(f64) -> i32)(n)
                    % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize,
            ) as *mut Node;
        }
        TAG_VARIANT_STRING_SHORT => {
            let ts: *mut TString = &mut (*((*key).value.object as *mut GCUnion)).ts;
            return &mut *((*t).node).offset(
                ((*ts).hash & ((1 << (*t).log_size_node as i32) - 1) as u32) as i32 as isize,
            ) as *mut Node;
        }
        TAG_VARIANT_STRING_LONG => {
            let ts_0: *mut TString = &mut (*((*key).value.object as *mut GCUnion)).ts;
            return &mut *((*t).node).offset(
                ((luas_hashlongstr as unsafe extern "C" fn(*mut TString) -> u32)(ts_0)
                    & ((1 << (*t).log_size_node as i32) - 1) as u32) as i32
                    as isize,
            ) as *mut Node;
        }
        TAG_VARIANT_BOOLEAN_FALSE => {
            return &mut *((*t).node)
                .offset((0 & (1 << (*t).log_size_node as i32) - 1) as isize)
                as *mut Node;
        }
        TAG_VARIANT_BOOLEAN_TRUE => {
            return &mut *((*t).node)
                .offset((1 & (1 << (*t).log_size_node as i32) - 1) as isize)
                as *mut Node;
        }
        TAG_VARIANT_POINTER => {
            let p: *mut libc::c_void = (*key).value.p;
            return &mut *((*t).node).offset(
                ((p as u64
                    & (2147483647 as i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as u64) as u32)
                    .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                    as isize,
            ) as *mut Node;
        }
        TAG_VARIANT_CLOSURE_CFUNCTION => {
            let f: CFunction = (*key).value.f;
            return &mut *((*t).node).offset(
                ((::core::mem::transmute::<CFunction, u64>(f)
                    & (2147483647 as i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as u64) as u32)
                    .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                    as isize,
            ) as *mut Node;
        }
        _ => {
            let o: *mut Object = (*key).value.object;
            return &mut *((*t).node).offset(
                ((o as u64
                    & (2147483647 as i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as u64) as u32)
                    .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                    as isize,
            ) as *mut Node;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn mainpositionfromnode(t: *const Table, nd: *mut Node) -> *mut Node { unsafe {
    let mut key: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io_: *mut TValue = &mut key;
    let node: *const Node = nd;
    (*io_).value = (*node).key.value;
    (*io_).set_tag((*node).key.tag);
    return mainpositiontv(t, &mut key);
}}
pub unsafe extern "C" fn equalkey(
    k1: *const TValue,
    node: *const Node,
    deadok: i32,
) -> i32 { unsafe {
    if (*k1).get_tag() != (*node).key.tag
        && !(deadok != 0
            && (*node).key.tag == 9 + 2
            && ((*k1).is_collectable()))
    {
        return 0;
    }
    match get_tag_variant((*node).key.tag) {
        TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => return 1,
        TAG_VARIANT_NUMERIC_INTEGER => return ((*k1).value.i == (*node).key.value.i) as i32,
        TAG_VARIANT_NUMERIC_NUMBER => return ((*k1).value.n == (*node).key.value.n) as i32,
        TAG_VARIANT_POINTER => return ((*k1).value.p == (*node).key.value.p) as i32,
        TAG_VARIANT_CLOSURE_CFUNCTION => return ((*k1).value.f == (*node).key.value.f) as i32,
        TAG_VARIANT_STRING_LONG => {
            return luas_eqlngstr(
                &mut (*((*k1).value.object as *mut GCUnion)).ts,
                &mut (*((*node).key.value.object as *mut GCUnion)).ts,
            );
        }
        _ => return ((*k1).value.object == (*node).key.value.object) as i32,
    };
}}
pub unsafe extern "C" fn luah_realasize(t: *const Table) -> u32 { unsafe {
    if (*t).flags as i32 & 1 << 7 == 0
        || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0u32
    {
        return (*t).array_limit;
    } else {
        let mut size: u32 = (*t).array_limit;
        size |= size >> 1;
        size |= size >> 2;
        size |= size >> 4;
        size |= size >> 8;
        size |= size >> 16 as i32;
        size = size.wrapping_add(1);
        return size;
    };
}}
pub unsafe extern "C" fn ispow2realasize(t: *const Table) -> i32 { unsafe {
    return ((*t).flags as i32 & 1 << 7 != 0
        || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0u32) as i32;
}}
pub unsafe extern "C" fn setlimittosize(table: *mut Table) -> u32 { unsafe {
    (*table).array_limit = luah_realasize(table);
    (*table).flags = ((*table).flags as i32 & !(1 << 7) as u8 as i32) as u8;
    return (*table).array_limit;
}}
pub unsafe extern "C" fn getgeneric(
    table: *mut Table,
    key: *const TValue,
    deadok: i32,
) -> *const TValue { unsafe {
    let mut node: *mut Node = mainpositiontv(table, key);
    loop {
        if equalkey(key, node, deadok) != 0 {
            return &mut (*node).value;
        } else {
            let nx: i32 = (*node).next;
            if nx == 0 {
                return &ABSENT_KEY;
            }
            node = node.offset(nx as isize);
        }
    }
}}
pub unsafe extern "C" fn arrayindex(k: i64) -> u32 {
    if (k as u64).wrapping_sub(1 as u32 as u64)
        < (if ((1 as u32)
            << (::core::mem::size_of::<i32>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(1 as u64) as i32) as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64)
        {
            (1 as u32)
                << (::core::mem::size_of::<i32>() as u64)
                    .wrapping_mul(8 as u64)
                    .wrapping_sub(1 as u64) as i32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64) as u32
        }) as u64
    {
        return k as u32;
    } else {
        return 0u32;
    };
}
pub unsafe extern "C" fn findindex(
    state: *mut State,
    table: *mut Table,
    key: *mut TValue,
    asize: u32,
) -> u32 { unsafe {
    let mut i: u32;
    if get_tag_type((*key).get_tag()) == TAG_TYPE_NIL {
        return 0u32;
    }
    i = if (*key).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        arrayindex((*key).value.i)
    } else {
        0u32
    };
    if i.wrapping_sub(1 as u32) < asize {
        return i;
    } else {
        let n_value: *const TValue = getgeneric(table, key, 1);
        if (((*n_value).get_tag() == TAG_VARIANT_NIL_ABSENTKEY) as i32 != 0) as i32 as i64 != 0 {
            luag_runerror(state, b"invalid key to 'next'\0" as *const u8 as *const i8);
        }
        i = (n_value as *mut Node).offset_from(&mut *((*table).node).offset(0 as isize) as *mut Node)
            as i64 as i32 as u32;
        return i.wrapping_add(1 as u32).wrapping_add(asize);
    };
}}
pub unsafe extern "C" fn luah_next(
    state: *mut State,
    table: *mut Table,
    key: StkId,
) -> i32 { unsafe {
    let asize: u32 = luah_realasize(table);
    let mut i: u32 = findindex(state, table, &mut (*key).value, asize);
    while i < asize {
        if get_tag_type ((*((*table).array).offset(i as isize)).get_tag()) != TAG_TYPE_NIL {
            let io: *mut TValue = &mut (*key).value;
            (*io).value.i = i.wrapping_add(1 as u32) as i64;
            (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
            let io1: *mut TValue = &mut (*key.offset(1 as isize)).value;
            let io2: *const TValue = &mut *((*table).array).offset(i as isize) as *mut TValue;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            return 1;
        }
        i = i.wrapping_add(1);
    }
    i = i.wrapping_sub(asize);
    while (i as i32) < 1 << (*table).log_size_node as i32 {
        if !(get_tag_type ((*((*table).node).offset(i as isize)).value.get_tag()) == TAG_TYPE_NIL) {
            let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
            let io_: *mut TValue = &mut (*key).value;
            (*io_).value = (*node).key.value;
            (*io_).set_tag((*node).key.tag);
            let io1_0: *mut TValue = &mut (*key.offset(1 as isize)).value;
            let io2_0: *const TValue = &mut (*node).value;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            return 1;
        }
        i = i.wrapping_add(1);
    }
    return 0;
}}
pub unsafe extern "C" fn freehash(state: *mut State, table: *mut Table) { unsafe {
    if !((*table).last_free).is_null() {
        (*state).free_memory(
            (*table).node as *mut libc::c_void,
            ((1 << (*table).log_size_node as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<Node>() as u64),
        );
    }
}}
pub unsafe extern "C" fn computesizes(nums: *mut u32, pna: *mut u32) -> u32 { unsafe {
    let mut i: i32;
    let mut twotoi: u32;
    let mut a: u32 = 0u32;
    let mut na: u32 = 0u32;
    let mut optimal: u32 = 0u32;
    i = 0;
    twotoi = 1 as u32;
    while twotoi > 0u32 && *pna > twotoi.wrapping_div(2 as u32) {
        a = a.wrapping_add(*nums.offset(i as isize));
        if a > twotoi.wrapping_div(2 as u32) {
            optimal = twotoi;
            na = a;
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2 as u32);
    }
    *pna = na;
    return optimal;
}}
pub unsafe extern "C" fn countint(key: i64, nums: *mut u32) -> i32 { unsafe {
    let k: u32 = arrayindex(key);
    if k != 0u32 {
        let ref mut fresh129 = *nums.offset(ceiling_log2(k as u64) as isize);
        *fresh129 = (*fresh129).wrapping_add(1);
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn numusearray(t: *const Table, nums: *mut u32) -> u32 { unsafe {
    let mut lg: i32;
    let mut ttlg: u32;
    let mut ause: u32 = 0u32;
    let mut i: u32 = 1 as u32;
    let asize: u32 = (*t).array_limit;
    lg = 0;
    ttlg = 1 as u32;
    while lg
        <= (::core::mem::size_of::<i32>() as u64)
            .wrapping_mul(8 as u64)
            .wrapping_sub(1 as u64) as i32
    {
        let mut lc: u32 = 0u32;
        let mut lim: u32 = ttlg;
        if lim > asize {
            lim = asize;
            if i > lim {
                break;
            }
        }
        while i <= lim {
            if get_tag_type ((*((*t).array).offset(i.wrapping_sub(1 as u32) as isize)).get_tag())
                != 0
            {
                lc = lc.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        let ref mut fresh130 = *nums.offset(lg as isize);
        *fresh130 = (*fresh130).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        ttlg = ttlg.wrapping_mul(2 as u32);
    }
    return ause;
}}
pub unsafe extern "C" fn numusehash(
    t: *const Table,
    nums: *mut u32,
    pna: *mut u32,
) -> i32 { unsafe {
    let mut totaluse: i32 = 0;
    let mut ause: i32 = 0;
    let mut i: i32 = 1 << (*t).log_size_node as i32;
    loop {
        let fresh131 = i;
        i = i - 1;
        if !(fresh131 != 0) {
            break;
        }
        let node: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
        if !(get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL) {
            if (*node).key.tag == TAG_VARIANT_NUMERIC_INTEGER {
                ause += countint((*node).key.value.i, nums);
            }
            totaluse += 1;
        }
    }
    *pna = (*pna).wrapping_add(ause as u32);
    return totaluse;
}}
pub unsafe extern "C" fn setnodevector(state: *mut State, table: *mut Table, mut size: u32) { unsafe {
    if size == 0u32 {
        (*table).node = &DUMMY_NODE as *const Node as *mut Node;
        (*table).log_size_node = 0;
        (*table).last_free = std::ptr::null_mut();
    } else {
        let mut i: i32;
        let lsize: i32 = ceiling_log2(size as u64) as i32;
        if lsize
            > (::core::mem::size_of::<i32>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(1 as u64) as i32
                - 1
            || (1 as u32) << lsize
                > (if ((1 as u32)
                    << (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(8 as u64)
                        .wrapping_sub(1 as u64) as i32
                        - 1) as u64
                    <= (!(0u64)).wrapping_div(::core::mem::size_of::<Node>() as u64)
                {
                    (1 as u32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as u64)
                            .wrapping_sub(1 as u64) as i32
                            - 1
                } else {
                    (!(0u64)).wrapping_div(::core::mem::size_of::<Node>() as u64) as u32
                })
        {
            luag_runerror(state, b"table overflow\0" as *const u8 as *const i8);
        }
        size = (1 << lsize) as u32;
        (*table).node = luam_malloc_(
            state,
            (size as u64).wrapping_mul(::core::mem::size_of::<Node>() as u64),
        ) as *mut Node;
        i = 0;
        while i < size as i32 {
            let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
            (*node).next = 0;
            (*node).key.tag = 0;
            (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
            i += 1;
        }
        (*table).log_size_node = lsize as u8;
        (*table).last_free = &mut *((*table).node).offset(size as isize) as *mut Node;
    };
}}
pub unsafe extern "C" fn reinsert(state: *mut State, ot: *mut Table, table: *mut Table) { unsafe {
    let mut j: i32;
    let size: i32 = 1 << (*ot).log_size_node as i32;
    j = 0;
    while j < size {
        let old: *mut Node = &mut *((*ot).node).offset(j as isize) as *mut Node;
        if !(get_tag_type((*old).value.get_tag()) == TAG_TYPE_NIL) {
            let mut k: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io_: *mut TValue = &mut k;
            let node: *const Node = old;
            (*io_).value = (*node).key.value;
            (*io_).set_tag((*node).key.tag);
            luah_set(state, table, &mut k, &mut (*old).value);
        }
        j += 1;
    }
}}
pub unsafe extern "C" fn luah_resize(
    state: *mut State,
    table: *mut Table,
    new_array_size: u32,
    nhsize: u32,
) { unsafe {
    let mut i: u32;
    let mut new_table: Table = Table::new();
    let old_array_size: u32 = setlimittosize(table);
    let new_array: *mut TValue;
    setnodevector(state, &mut new_table, nhsize);
    if new_array_size < old_array_size {
        (*table).array_limit = new_array_size;
        Table::exchange_hash_part(table, &mut new_table);
        i = new_array_size;
        while i < old_array_size {
            if get_tag_type ((*((*table).array).offset(i as isize)).get_tag()) != TAG_TYPE_NIL {
                luah_setint(
                    state,
                    table,
                    i.wrapping_add(1 as u32) as i64,
                    &mut *((*table).array).offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
        }
        (*table).array_limit = old_array_size;
        Table::exchange_hash_part(table, &mut new_table);
    }
    new_array = luam_realloc_(
        state,
        (*table).array as *mut libc::c_void,
        (old_array_size as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
        (new_array_size as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
    ) as *mut TValue;
    if ((new_array.is_null() && new_array_size > 0u32) as i32 != 0) as i32 as i64 != 0 {
        freehash(state, &mut new_table);
        luad_throw(state, 4);
    }
    Table::exchange_hash_part(table, &mut new_table);
    (*table).array = new_array;
    (*table).array_limit = new_array_size;
    i = old_array_size;
    while i < new_array_size {
        (*((*table).array).offset(i as isize)).set_tag(TAG_VARIANT_NIL_EMPTY);
        i = i.wrapping_add(1);
    }
    reinsert(state, &mut new_table, table);
    freehash(state, &mut new_table);
}}
pub unsafe extern "C" fn luah_resizearray(
    state: *mut State,
    table: *mut Table,
    new_array_size: u32,
) { unsafe {
    let new_size: i32 = if ((*table).last_free).is_null() {
        0
    } else {
        1 << (*table).log_size_node as i32
    };
    luah_resize(state, table, new_array_size, new_size as u32);
}}
pub unsafe extern "C" fn rehash(state: *mut State, table: *mut Table, ek: *const TValue) { unsafe {
    let asize: u32;
    let mut na: u32;
    let mut nums: [u32; 32] = [0; 32];
    let mut i: i32;
    let mut totaluse: i32;
    i = 0;
    while i
        <= (::core::mem::size_of::<i32>() as u64)
            .wrapping_mul(8 as u64)
            .wrapping_sub(1 as u64) as i32
    {
        nums[i as usize] = 0u32;
        i += 1;
    }
    setlimittosize(table);
    na = numusearray(table, nums.as_mut_ptr());
    totaluse = na as i32;
    totaluse += numusehash(table, nums.as_mut_ptr(), &mut na);
    if (*ek).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        na = na.wrapping_add(countint((*ek).value.i, nums.as_mut_ptr()) as u32);
    }
    totaluse += 1;
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    luah_resize(state, table, asize, (totaluse as u32).wrapping_sub(na));
}}
pub unsafe extern "C" fn luah_new(state: *mut State) -> *mut Table { unsafe {
    let o: *mut Object = luac_newobj(
        state,
        TAG_TYPE_TABLE,
        ::core::mem::size_of::<Table>() as u64,
    );
    let new_table: *mut Table = &mut (*(o as *mut GCUnion)).h;
    (*new_table).metatable = std::ptr::null_mut();
    (*new_table).flags = !(!0 << TM_EQ as i32 + 1) as u8;
    (*new_table).array = std::ptr::null_mut();
    (*new_table).array_limit = 0u32;
    setnodevector(state, new_table, 0u32);
    return new_table;
}}
pub unsafe extern "C" fn luah_free(state: *mut State, table: *mut Table) { unsafe {
    freehash(state, table);
(*state).free_memory(
        (*table).array as *mut libc::c_void,
        (luah_realasize(table) as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
    );
(*state).free_memory(
        table as *mut libc::c_void,
        ::core::mem::size_of::<Table>() as u64,
    );
}}
pub unsafe extern "C" fn luah_newkey(
    state: *mut State,
    table: *mut Table,
    mut key: *const TValue,
    value: *mut TValue,
) { unsafe {
    let mut mp;
    let mut aux: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    if ((get_tag_type((*key).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i32 as i64 != 0 {
        luag_runerror(state, b"table index is nil\0" as *const u8 as *const i8);
    } else if (*key).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
        let f: f64 = (*key).value.n;
        let mut k: i64 = 0;
        if luav_flttointeger(f, &mut k, F2I::Equal) != 0 {
            let io: *mut TValue = &mut aux;
            (*io).value.i = k;
            (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
            key = &mut aux;
        } else if (!(f == f) as i32 != 0) as i32 as i64 != 0 {
            luag_runerror(state, b"table index is NaN\0" as *const u8 as *const i8);
        }
    }
    if get_tag_type ((*value).get_tag()) == TAG_TYPE_NIL {
        return;
    }
    mp = mainpositiontv(table, key);
    if (get_tag_type((*mp).value.get_tag()) != TAG_TYPE_NIL) || ((*table).last_free).is_null() {
        let mut other_node: *mut Node;
        let f_0: *mut Node = (*table).get_free_position();
        if f_0.is_null() {
            rehash(state, table, key);
            luah_set(state, table, key, value);
            return;
        }
        other_node = mainpositionfromnode(table, mp);
        if other_node != mp {
            while other_node.offset((*other_node).next as isize) != mp {
                other_node = other_node.offset((*other_node).next as isize);
            }
            (*other_node).next = f_0.offset_from(other_node) as i64 as i32;
            *f_0 = *mp;
            if (*mp).next != 0 {
                (*f_0).next += mp.offset_from(f_0) as i64 as i32;
                (*mp).next = 0;
            }
            (*mp).value.set_tag(TAG_VARIANT_NIL_EMPTY);
        } else {
            if (*mp).next != 0 {
                (*f_0).next = mp.offset((*mp).next as isize).offset_from(f_0) as i64 as i32;
            }
            (*mp).next = f_0.offset_from(mp) as i64 as i32;
            mp = f_0;
        }
    }
    let node: *mut Node = mp;
    let io_: *const TValue = key;
    (*node).key.value = (*io_).value;
    (*node).key.tag = (*io_).get_tag();
    if (*key).is_collectable() {
        if (*(table as *mut GCUnion)).object.get_marked() & 1 << 5 != 0
            && (*(*key).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrierback_(state, &mut (*(table as *mut GCUnion)).object);
        } else {
        };
    } else {
    };
    let io1: *mut TValue = &mut (*mp).value;
    let io2: *const TValue = value;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
}}
pub unsafe extern "C" fn luah_getint(table: *mut Table, key: i64) -> *const TValue { unsafe {
    let array_limit: u64 = (*table).array_limit as u64;
    if (key as u64).wrapping_sub(1 as u32 as u64) < array_limit {
        return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
    } else if (*table).flags as i32 & 1 << 7 != 0
        && (key as u64).wrapping_sub(1 as u32 as u64) & !array_limit.wrapping_sub(1 as u32 as u64)
            < array_limit
    {
        (*table).array_limit = key as u32;
        return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
    } else {
        let mut node: *mut Node = hashint(table, key);
        loop {
            if (*node).key.tag == TAG_VARIANT_NUMERIC_INTEGER && (*node).key.value.i == key {
                return &mut (*node).value;
            } else {
                let nx: i32 = (*node).next;
                if nx == 0 {
                    break;
                }
                node = node.offset(nx as isize);
            }
        }
        return &ABSENT_KEY;
    };
}}
pub unsafe extern "C" fn luah_getshortstr(
    table: *mut Table,
    key: *mut TString,
) -> *const TValue { unsafe {
    let mut node: *mut Node = &mut *((*table).node)
        .offset(((*key).hash & ((1 << (*table).log_size_node as i32) - 1) as u32) as i32 as isize)
        as *mut Node;
    loop {
        if get_tag_variant((*node).key.tag) == TAG_VARIANT_STRING_SHORT
            && &mut (*((*node).key.value.object as *mut GCUnion)).ts as *mut TString == key
        {
            return &mut (*node).value;
        } else {
            let nx: i32 = (*node).next;
            if nx == 0 {
                return &ABSENT_KEY;
            }
            node = node.offset(nx as isize);
        }
    }
}}
pub unsafe extern "C" fn luah_getstr(table: *mut Table, key: *mut TString) -> *const TValue { unsafe {
    if (*key).get_tag() == TAG_VARIANT_STRING_SHORT {
        return luah_getshortstr(table, key);
    } else {
        let mut ko: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut ko;
        let x_: *mut TString = key;
        (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
        (*io).set_tag ((*x_).get_tag());
        (*io).set_collectable();
        return getgeneric(table, &mut ko, 0);
    };
}}
pub unsafe extern "C" fn luah_get(table: *mut Table, key: *const TValue) -> *const TValue { unsafe {
    match (*key).get_tag_variant() {
        4 => return luah_getshortstr(table, &mut (*((*key).value.object as *mut GCUnion)).ts),
        3 => return luah_getint(table, (*key).value.i),
        0 => return &ABSENT_KEY,
        19 => {
            let mut k: i64 = 0;
            if luav_flttointeger((*key).value.n, &mut k, F2I::Equal) != 0 {
                return luah_getint(table, k);
            }
        }
        _ => {}
    }
    return getgeneric(table, key, 0);
}}
pub unsafe extern "C" fn luah_finishset(
    state: *mut State,
    table: *mut Table,
    key: *const TValue,
    slot: *const TValue,
    value: *mut TValue,
) { unsafe {
    if (*slot).get_tag() == TAG_VARIANT_NIL_ABSENTKEY {
        luah_newkey(state, table, key, value);
    } else {
        let io1: *mut TValue = slot as *mut TValue;
        let io2: *const TValue = value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
    };
}}
pub unsafe extern "C" fn luah_set(
    state: *mut State,
    table: *mut Table,
    key: *const TValue,
    value: *mut TValue,
) { unsafe {
    let slot: *const TValue = luah_get(table, key);
    luah_finishset(state, table, key, slot, value);
}}
pub unsafe extern "C" fn luah_setint(
    state: *mut State,
    table: *mut Table,
    key: i64,
    value: *mut TValue,
) { unsafe {
    let p: *const TValue = luah_getint(table, key);
    if (*p).get_tag() == TAG_VARIANT_NIL_ABSENTKEY {
        let mut k: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut k;
        (*io).value.i = key;
        (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
        luah_newkey(state, table, &mut k, value);
    } else {
        let io1: *mut TValue = p as *mut TValue;
        let io2: *const TValue = value;
        (*io1).value = (*io2).value;
        (*io1).set_tag ((*io2).get_tag());
    };
}}
pub unsafe extern "C" fn hash_search(table: *mut Table, mut j: u64) -> u64 { unsafe {
    let mut i: u64;
    if j == 0u64 {
        j = j.wrapping_add(1);
    }
    loop {
        i = j;
        if j <= (9223372036854775807 as i64 as u64).wrapping_div(2 as u64) {
            j = (j as u64).wrapping_mul(2 as u64) as u64 as u64;
            if get_tag_type((*luah_getint(table, j as i64)).get_tag()) == TAG_TYPE_NIL {
                break;
            }
        } else {
            j = 9223372036854775807 as i64 as u64;
            if get_tag_type((*luah_getint(table, j as i64)).get_tag()) == TAG_TYPE_NIL {
                break;
            }
            return j;
        }
    }
    while j.wrapping_sub(i) > 1 as u32 as u64 {
        let m: u64 = i.wrapping_add(j).wrapping_div(2 as u64);
        if get_tag_type ((*luah_getint(table, m as i64)).get_tag()) == TAG_TYPE_NIL {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}}
pub unsafe extern "C" fn binsearch(array: *const TValue, mut i: u32, mut j: u32) -> u32 { unsafe {
    while j.wrapping_sub(i) > 1 as u32 {
        let m: u32 = i.wrapping_add(j).wrapping_div(2 as u32);
        if get_tag_type ((*array.offset(m.wrapping_sub(1 as u32) as isize)).get_tag()) == TAG_TYPE_NIL {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}}
pub unsafe extern "C" fn luah_getn(table: *mut Table) -> u64 { unsafe {
    let mut limit: u32 = (*table).array_limit;
    if limit > 0u32
        && get_tag_type ((*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).get_tag()) == TAG_TYPE_NIL
    {
        if limit >= 2 as u32
            && !get_tag_type ((*((*table).array).offset(limit.wrapping_sub(2 as u32) as isize)).get_tag()) == TAG_TYPE_NIL
        {
            if ispow2realasize(table) != 0
                && !(limit.wrapping_sub(1 as u32)
                    & limit.wrapping_sub(1 as u32).wrapping_sub(1 as u32)
                    == 0u32)
            {
                (*table).array_limit = limit.wrapping_sub(1 as u32);
                (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
            }
            return limit.wrapping_sub(1 as u32) as u64;
        } else {
            let boundary: u32 = binsearch((*table).array, 0u32, limit);
            if ispow2realasize(table) != 0 && boundary > (luah_realasize(table)).wrapping_div(2 as u32) {
                (*table).array_limit = boundary;
                (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
            }
            return boundary as u64;
        }
    }
    if !((*table).flags as i32 & 1 << 7 == 0
        || (*table).array_limit & ((*table).array_limit).wrapping_sub(1 as u32) == 0u32)
    {
        if get_tag_type ((*((*table).array).offset(limit as isize)).get_tag()) == TAG_TYPE_NIL {
            return limit as u64;
        }
        limit = luah_realasize(table);
        if get_tag_type ((*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).get_tag()) == TAG_TYPE_NIL
        {
            let boundary_0: u32 = binsearch((*table).array, (*table).array_limit, limit);
            (*table).array_limit = boundary_0;
            return boundary_0 as u64;
        }
    }
    if ((*table).last_free).is_null()
        || get_tag_type ((*luah_getint(table, limit.wrapping_add(1 as u32) as i64)).get_tag()) == TAG_TYPE_NIL
    {
        return limit as u64;
    } else {
        return hash_search(table, limit as u64);
    };
}}
pub unsafe extern "C" fn luak_semerror(lexical_state: *mut LexicalState, message: *const i8) -> ! { unsafe {
    (*lexical_state).t.token = 0;
    luax_syntaxerror(lexical_state, message);
}}
pub unsafe extern "C" fn tonumeral(e: *const ExpressionDescription, v: *mut TValue) -> i32 { unsafe {
    if (*e).t == (*e).f {
        match (*e).k as u32 {
            6 => {
                if !v.is_null() {
                    (*v).value.i = (*e).u.ival;
                    (*v).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                }
                return 1;
            }
            5 => {
                if !v.is_null() {
                    (*v).value.n = (*e).u.nval;
                    (*v).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                }
                return 1;
            }
            _ => return 0,
        };
    } else {
        return 0;
    }
}}
pub unsafe extern "C" fn const2val(
    fs: *mut FunctionState,
    e: *const ExpressionDescription,
) -> *mut TValue { unsafe {
    return &mut (*((*(*(*fs).lexical_state).dynamic_data).active_variable.arr).offset((*e).u.info as isize))
        .k;
}}
pub unsafe extern "C" fn luak_exp2const(
    fs: *mut FunctionState,
    e: *const ExpressionDescription,
    v: *mut TValue,
) -> i32 { unsafe {
    if (*e).t != (*e).f {
        return 0;
    }
    match (*e).k as u32 {
        3 => {
            (*v).set_tag(TAG_VARIANT_BOOLEAN_FALSE);
            return 1;
        }
        2 => {
            (*v).set_tag(TAG_VARIANT_BOOLEAN_TRUE);
            return 1;
        }
        1 => {
            (*v).set_tag (TAG_VARIANT_NIL_NIL);
            return 1;
        }
        7 => {
            let io: *mut TValue = v;
            let x_: *mut TString = (*e).u.strval;
            (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
            (*io).set_tag ((*x_).get_tag());
            (*io).set_collectable();
            return 1;
        }
        11 => {
            let io1: *mut TValue = v;
            let io2: *const TValue = const2val(fs, e);
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            return 1;
        }
        _ => return tonumeral(e, v),
    };
}}
pub unsafe extern "C" fn previousinstruction(fs: *mut FunctionState) -> *mut u32 { unsafe {
    static mut INVALID_INSTRUCTION: u32 = !(0u32);
    if (*fs).program_counter > (*fs).lasttarget {
        return &mut *((*(*fs).f).code).offset(((*fs).program_counter - 1) as isize) as *mut u32;
    } else {
        return &INVALID_INSTRUCTION as *const u32 as *mut u32;
    };
}}
pub unsafe extern "C" fn luak_nil(fs: *mut FunctionState, mut from: i32, n: i32) { unsafe {
    let mut l: i32 = from + n - 1;
    let previous: *mut u32 = previousinstruction(fs);
    if (*previous >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 == OP_LOADNIL as i32 as u32 {
        let pfrom: i32 = (*previous >> 0 + 7 & !(!(0u32) << 8) << 0) as i32;
        let pl: i32 =
            pfrom + (*previous >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
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
                | ((l - from) as u32) << 0 + 7 + 8 + 1
                    & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
            return;
        }
    }
    luak_code_abck(fs, OP_LOADNIL, from, n - 1, 0, 0);
}}
pub unsafe extern "C" fn getjump(fs: *mut FunctionState, program_counter: i32) -> i32 { unsafe {
    let offset: i32 = (*((*(*fs).f).code).offset(program_counter as isize) >> 0 + 7
        & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
    if offset == -1 {
        return -1;
    } else {
        return program_counter + 1 + offset;
    };
}}
pub unsafe extern "C" fn fixjump(
    fs: *mut FunctionState,
    program_counter: i32,
    dest: i32,
) { unsafe {
    let jmp: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
    let offset: i32 = dest - (program_counter + 1);
    if !(-((1 << 8 + 8 + 1 + 8) - 1 >> 1) <= offset
        && offset
            <= (1 << 8 + 8 + 1 + 8)
                - 1
                - ((1 << 8 + 8 + 1 + 8) - 1 >> 1))
    {
        luax_syntaxerror(
            (*fs).lexical_state,
            b"control structure too long\0" as *const u8 as *const i8,
        );
    }
    *jmp = *jmp & !(!(!(0u32) << 8 + 8 + 1 + 8) << 0 + 7)
        | ((offset + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32) << 0 + 7
            & !(!(0u32) << 8 + 8 + 1 + 8) << 0 + 7;
}}
pub unsafe extern "C" fn luak_concat(fs: *mut FunctionState, l1: *mut i32, l2: i32) { unsafe {
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
}}
pub unsafe extern "C" fn luak_jump(fs: *mut FunctionState) -> i32 { unsafe {
    return codesj(fs, OP_JMP, -1, 0);
}}
pub unsafe extern "C" fn luak_ret(fs: *mut FunctionState, first: i32, nret: i32) { unsafe {
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
}}
pub unsafe extern "C" fn condjump(
    fs: *mut FunctionState,
    op: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 { unsafe {
    luak_code_abck(fs, op, a, b, c, k);
    return luak_jump(fs);
}}
pub unsafe extern "C" fn luak_getlabel(fs: *mut FunctionState) -> i32 { unsafe {
    (*fs).lasttarget = (*fs).program_counter;
    return (*fs).program_counter;
}}
pub unsafe extern "C" fn getjumpcontrol(
    fs: *mut FunctionState,
    program_counter: i32,
) -> *mut u32 { unsafe {
    let pi: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
    if program_counter >= 1
        && OPMODES
            [(*pi.offset(-(1 as isize)) >> 0 & !(!(0u32) << 7) << 0) as u32 as usize]
            as i32
            & 1 << 4
            != 0
    {
        return pi.offset(-(1 as isize));
    } else {
        return pi;
    };
}}
pub unsafe extern "C" fn patchtestreg(
    fs: *mut FunctionState,
    node: i32,
    reg: i32,
) -> i32 { unsafe {
    let i: *mut u32 = getjumpcontrol(fs, node);
    if (*i >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 != OP_TESTSET as i32 as u32 {
        return 0;
    }
    if reg != (1 << 8) - 1
        && reg != (*i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
    {
        *i = *i & !(!(!(0u32) << 8) << 0 + 7)
            | (reg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
    } else {
        *i = (OP_TEST as i32 as u32) << 0
            | ((*i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32 as u32)
                << 0 + 7
            | (0u32) << 0 + 7 + 8 + 1
            | (0u32) << 0 + 7 + 8 + 1 + 8
            | ((*i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 as u32)
                << 0 + 7 + 8;
    }
    return 1;
}}
pub unsafe extern "C" fn removevalues(fs: *mut FunctionState, mut list: i32) { unsafe {
    while list != -1 {
        patchtestreg(fs, list, (1 << 8) - 1);
        list = getjump(fs, list);
    }
}}
pub unsafe extern "C" fn patchlistaux(
    fs: *mut FunctionState,
    mut list: i32,
    vtarget: i32,
    reg: i32,
    dtarget: i32,
) { unsafe {
    while list != -1 {
        let next: i32 = getjump(fs, list);
        if patchtestreg(fs, list, reg) != 0 {
            fixjump(fs, list, vtarget);
        } else {
            fixjump(fs, list, dtarget);
        }
        list = next;
    }
}}
pub unsafe extern "C" fn luak_patchlist(
    fs: *mut FunctionState,
    list: i32,
    target: i32,
) { unsafe {
    patchlistaux(fs, list, target, (1 << 8) - 1, target);
}}
pub unsafe extern "C" fn luak_patchtohere(fs: *mut FunctionState, list: i32) { unsafe {
    let hr: i32 = luak_getlabel(fs);
    luak_patchlist(fs, list, hr);
}}
pub unsafe extern "C" fn savelineinfo(
    fs: *mut FunctionState,
    f: *mut Prototype,
    line: i32,
) { unsafe {
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
            ::core::mem::size_of::<AbsoluteLineInfo>() as u64 as i32,
            (if 2147483647 as i32 as u64
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64)
            {
                2147483647 as i32 as u32
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64)
                    as u32
            }) as i32,
            b"lines\0" as *const u8 as *const i8,
        ) as *mut AbsoluteLineInfo;
        (*((*f).absolute_line_info).offset((*fs).nabslineinfo as isize)).program_counter = program_counter;
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
        ::core::mem::size_of::<i8>() as u64 as i32,
        (if 2147483647 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64)
        {
            2147483647 as i32 as u32
        } else {
            (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64) as u32
        }) as i32,
        b"opcodes\0" as *const u8 as *const i8,
    ) as *mut i8;
    *((*f).line_info).offset(program_counter as isize) = linedif as i8;
    (*fs).previousline = line;
}}
pub unsafe extern "C" fn removelastlineinfo(fs: *mut FunctionState) { unsafe {
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
}}
pub unsafe extern "C" fn removelastinstruction(fs: *mut FunctionState) { unsafe {
    removelastlineinfo(fs);
    (*fs).program_counter -= 1;
    (*fs).program_counter;
}}
pub unsafe extern "C" fn luak_code(fs: *mut FunctionState, i: u32) -> i32 { unsafe {
    let f: *mut Prototype = (*fs).f;
    (*f).code = luam_growaux_(
        (*(*fs).lexical_state).state,
        (*f).code as *mut libc::c_void,
        (*fs).program_counter,
        &mut (*f).size_code,
        ::core::mem::size_of::<u32>() as u64 as i32,
        (if 2147483647 as i32 as u64
            <= (!(0u64)).wrapping_div(::core::mem::size_of::<u32>() as u64)
        {
            2147483647 as i32 as u32
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
}}
pub unsafe extern "C" fn luak_code_abck(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 { unsafe {
    return luak_code(
        fs,
        (o as u32) << 0
            | (a as u32) << 0 + 7
            | (b as u32) << 0 + 7 + 8 + 1
            | (c as u32) << 0 + 7 + 8 + 1 + 8
            | (k as u32) << 0 + 7 + 8,
    );
}}
pub unsafe extern "C" fn luak_codeabx(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    bc: u32,
) -> i32 { unsafe {
    return luak_code(
        fs,
        (o as u32) << 0 | (a as u32) << 0 + 7 | bc << 0 + 7 + 8,
    );
}}
pub unsafe extern "C" fn codeasbx(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    bc: i32,
) -> i32 { unsafe {
    let b: u32 = (bc + ((1 << 8 + 8 + 1) - 1 >> 1)) as u32;
    return luak_code(
        fs,
        (o as u32) << 0 | (a as u32) << 0 + 7 | b << 0 + 7 + 8,
    );
}}
pub unsafe extern "C" fn codesj(
    fs: *mut FunctionState,
    o: u32,
    sj: i32,
    k: i32,
) -> i32 { unsafe {
    let j: u32 = (sj + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32;
    return luak_code(
        fs,
        (o as u32) << 0 | j << 0 + 7 | (k as u32) << 0 + 7 + 8,
    );
}}
pub unsafe extern "C" fn codeextraarg(fs: *mut FunctionState, a: i32) -> i32 { unsafe {
    return luak_code(
        fs,
        (OP_EXTRAARG as i32 as u32) << 0 | (a as u32) << 0 + 7,
    );
}}
pub unsafe extern "C" fn luak_codek(fs: *mut FunctionState, reg: i32, k: i32) -> i32 { unsafe {
    if k <= (1 << 8 + 8 + 1) - 1 {
        return luak_codeabx(fs, OP_LOADK, reg, k as u32);
    } else {
        let p: i32 = luak_codeabx(fs, OP_LOADKX, reg, 0u32);
        codeextraarg(fs, k);
        return p;
    };
}}
pub unsafe extern "C" fn luak_checkstack(fs: *mut FunctionState, n: i32) { unsafe {
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
}}
pub unsafe extern "C" fn luak_reserveregs(fs: *mut FunctionState, n: i32) { unsafe {
    luak_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as i32 + n) as u8;
}}
pub unsafe extern "C" fn freereg(fs: *mut FunctionState, reg: i32) { unsafe {
    if reg >= luay_nvarstack(fs) {
        (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
        (*fs).freereg;
    }
}}
pub unsafe extern "C" fn freeregs(fs: *mut FunctionState, r1: i32, r2: i32) { unsafe {
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}}
pub unsafe extern "C" fn freeexp(fs: *mut FunctionState, e: *mut ExpressionDescription) { unsafe {
    if (*e).k as u32 == VNONRELOC as i32 as u32 {
        freereg(fs, (*e).u.info);
    }
}}
pub unsafe extern "C" fn freeexps(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) { unsafe {
    let r1: i32 = if (*e1).k as u32 == VNONRELOC as i32 as u32 {
        (*e1).u.info
    } else {
        -1
    };
    let r2: i32 = if (*e2).k as u32 == VNONRELOC as i32 as u32 {
        (*e2).u.info
    } else {
        -1
    };
    freeregs(fs, r1, r2);
}}
pub unsafe extern "C" fn addk(
    fs: *mut FunctionState,
    key: *mut TValue,
    v: *mut TValue,
) -> i32 { unsafe {
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
            && (*((*f).k).offset(k as isize)).get_tag_variant() == (*v).get_tag_variant() && luav_equalobj(std::ptr::null_mut(), &mut *((*f).k).offset(k as isize), v) != 0
        {
            return k;
        }
    }
    let mut old_size: i32 = (*f).size_k;
    k = (*fs).nk;
    let io: *mut TValue = &mut value;
    (*io).value.i = k as i64;
    (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
    luah_finishset(state, (*(*fs).lexical_state).h, key, index, &mut value);
    (*f).k = luam_growaux_(
        state,
        (*f).k as *mut libc::c_void,
        k,
        &mut (*f).size_k,
        ::core::mem::size_of::<TValue>() as u64 as i32,
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
        (*((*f).k).offset(fresh135 as isize)).set_tag (TAG_VARIANT_NIL_NIL);
    }
    let io1: *mut TValue = &mut *((*f).k).offset(k as isize) as *mut TValue;
    let io2: *const TValue = v;
    (*io1).value = (*io2).value;
    (*io1).set_tag ((*io2).get_tag());
    (*fs).nk += 1;
    (*fs).nk;
    if (*v).is_collectable() {
        if (*f).get_marked() & 1 << 5 != 0
            && (*(*v).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                state,
                &mut (*(f as *mut GCUnion)).object,
                &mut (*((*v).value.object as *mut GCUnion)).object,
            );
        } else {
        };
    } else {
    };
    return k;
}}
pub unsafe extern "C" fn string_k(fs: *mut FunctionState, s: *mut TString) -> i32 { unsafe {
    let mut o: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io: *mut TValue = &mut o;
    let x_: *mut TString = s;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag ((*x_).get_tag());
    (*io).set_collectable();
    return addk(fs, &mut o, &mut o);
}}
pub unsafe extern "C" fn luak_int_k(fs: *mut FunctionState, n: i64) -> i32 { unsafe {
    let mut o: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let io: *mut TValue = &mut o;
    (*io).value.i = n;
    (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
    return addk(fs, &mut o, &mut o);
}}
pub unsafe extern "C" fn luak_number_k(fs: *mut FunctionState, r: f64) -> i32 { unsafe {
    let mut o: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    let mut ik: i64 = 0;
    let io: *mut TValue = &mut o;
    (*io).value.n = r;
    (*io).set_tag (TAG_VARIANT_NUMERIC_NUMBER);
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
}}
pub unsafe extern "C" fn bool_false(fs: *mut FunctionState) -> i32 { unsafe {
    let mut o: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    o.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
    return addk(fs, &mut o, &mut o);
}}
pub unsafe extern "C" fn bool_true(fs: *mut FunctionState) -> i32 { unsafe {
    let mut o: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    o.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
    return addk(fs, &mut o, &mut o);
}}
pub unsafe extern "C" fn nil_k(fs: *mut FunctionState) -> i32 { unsafe {
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
    v.set_tag (TAG_VARIANT_NIL_NIL);
    let io: *mut TValue = &mut k;
    let x_: *mut Table = (*(*fs).lexical_state).h;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_TABLE);
    (*io).set_collectable();
    return addk(fs, &mut k, &mut v);
}}
pub unsafe extern "C" fn fits_c(i: i64) -> i32 {
    return ((i as u64).wrapping_add(((1 << 8) - 1 >> 1) as u64)
        <= ((1 << 8) - 1) as u32 as u64) as i32;
}
pub unsafe extern "C" fn fits_bx(i: i64) -> i32 {
    return (-((1 << 8 + 8 + 1) - 1 >> 1) as i64 <= i
        && i <= ((1 << 8 + 8 + 1)
            - 1
            - ((1 << 8 + 8 + 1) - 1 >> 1)) as i64) as i32;
}
pub unsafe extern "C" fn luak_int(fs: *mut FunctionState, reg: i32, i: i64) { unsafe {
    if fits_bx(i) != 0 {
        codeasbx(fs, OP_LOADI, reg, i as i32);
    } else {
        luak_codek(fs, reg, luak_int_k(fs, i));
    };
}}
pub unsafe extern "C" fn luak_float(fs: *mut FunctionState, reg: i32, f: f64) { unsafe {
    let mut fi: i64 = 0;
    if luav_flttointeger(f, &mut fi, F2I::Equal) != 0 && fits_bx(fi) != 0 {
        codeasbx(fs, OP_LOADF, reg, fi as i32);
    } else {
        luak_codek(fs, reg, luak_number_k(fs, f));
    };
}}
pub unsafe extern "C" fn const2exp(v: *mut TValue, e: *mut ExpressionDescription) { unsafe {
    match (*v).get_tag_variant() {
        3 => {
            (*e).k = VKINT;
            (*e).u.ival = (*v).value.i;
        }
        19 => {
            (*e).k = VKFLT;
            (*e).u.nval = (*v).value.n;
        }
        1 => {
            (*e).k = VFALSE;
        }
        17 => {
            (*e).k = VTRUE;
        }
        0 => {
            (*e).k = VNIL;
        }
        4 | 20 => {
            (*e).k = VKSTR;
            (*e).u.strval = &mut (*((*v).value.object as *mut GCUnion)).ts;
        }
        _ => {}
    };
}}
pub unsafe extern "C" fn luak_setreturns(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    count_results: i32,
) { unsafe {
    let program_counter: *mut u32 =
        &mut *((*(*fs).f).code).offset((*e).u.info as isize) as *mut u32;
    if (*e).k as u32 == VCALL as i32 as u32 {
        *program_counter = *program_counter
            & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
            | ((count_results + 1) as u32) << 0 + 7 + 8 + 1 + 8
                & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
    } else {
        *program_counter = *program_counter
            & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
            | ((count_results + 1) as u32) << 0 + 7 + 8 + 1 + 8
                & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
        *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7)
            | ((*fs).freereg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
        luak_reserveregs(fs, 1);
    };
}}
pub unsafe extern "C" fn str_to_k(fs: *mut FunctionState, e: *mut ExpressionDescription) { unsafe {
    (*e).u.info = string_k(fs, (*e).u.strval);
    (*e).k = VK;
}}
pub unsafe extern "C" fn luak_setoneret(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    if (*e).k as u32 == VCALL as i32 as u32 {
        (*e).k = VNONRELOC;
        (*e).u.info = (*((*(*fs).f).code).offset((*e).u.info as isize) >> 0 + 7
            & !(!(0u32) << 8) << 0) as i32;
    } else if (*e).k as u32 == VVARARG as i32 as u32 {
        *((*(*fs).f).code).offset((*e).u.info as isize) = *((*(*fs).f).code)
            .offset((*e).u.info as isize)
            & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
            | (2 as u32) << 0 + 7 + 8 + 1 + 8
                & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
        (*e).k = VRELOC;
    }
}}
pub unsafe extern "C" fn luak_dischargevars(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    match (*e).k as u32 {
        11 => {
            const2exp(const2val(fs, e), e);
        }
        9 => {
            let temp: i32 = (*e).u.var.ridx as i32;
            (*e).u.info = temp;
            (*e).k = VNONRELOC;
        }
        10 => {
            (*e).u.info = luak_code_abck(fs, OP_GETUPVAL, 0, (*e).u.info, 0, 0);
            (*e).k = VRELOC;
        }
        13 => {
            (*e).u.info = luak_code_abck(
                fs,
                OP_GETTABUP,
                0,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0,
            );
            (*e).k = VRELOC;
        }
        14 => {
            freereg(fs, (*e).u.ind.t as i32);
            (*e).u.info = luak_code_abck(
                fs,
                OP_GETI,
                0,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0,
            );
            (*e).k = VRELOC;
        }
        15 => {
            freereg(fs, (*e).u.ind.t as i32);
            (*e).u.info = luak_code_abck(
                fs,
                OP_GETFIELD,
                0,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0,
            );
            (*e).k = VRELOC;
        }
        12 => {
            freeregs(fs, (*e).u.ind.t as i32, (*e).u.ind.index as i32);
            (*e).u.info = luak_code_abck(
                fs,
                OP_GETTABLE,
                0,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0,
            );
            (*e).k = VRELOC;
        }
        19 | 18 => {
            luak_setoneret(fs, e);
        }
        _ => {}
    };
}}
pub unsafe extern "C" fn discharge2reg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    reg: i32,
) { unsafe {
    luak_dischargevars(fs, e);
    let current_block_14: u64;
    match (*e).k as u32 {
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
            luak_float(fs, reg, (*e).u.nval);
            current_block_14 = 13242334135786603907;
        }
        6 => {
            luak_int(fs, reg, (*e).u.ival);
            current_block_14 = 13242334135786603907;
        }
        17 => {
            let program_counter: *mut u32 =
                &mut *((*(*fs).f).code).offset((*e).u.info as isize) as *mut u32;
            *program_counter = *program_counter & !(!(!(0u32) << 8) << 0 + 7)
                | (reg as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
            current_block_14 = 13242334135786603907;
        }
        8 => {
            if reg != (*e).u.info {
                luak_code_abck(fs, OP_MOVE, reg, (*e).u.info, 0, 0);
            }
            current_block_14 = 13242334135786603907;
        }
        _ => return,
    }
    match current_block_14 {
        6937071982253665452 => {
            luak_codek(fs, reg, (*e).u.info);
        }
        _ => {}
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}}
pub unsafe extern "C" fn discharge2anyreg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    if (*e).k as u32 != VNONRELOC as i32 as u32 {
        luak_reserveregs(fs, 1);
        discharge2reg(fs, e, (*fs).freereg as i32 - 1);
    }
}}
pub unsafe extern "C" fn code_loadbool(fs: *mut FunctionState, a: i32, op: u32) -> i32 { unsafe {
    luak_getlabel(fs);
    return luak_code_abck(fs, op, a, 0, 0, 0);
}}
pub unsafe extern "C" fn need_value(fs: *mut FunctionState, mut list: i32) -> i32 { unsafe {
    while list != -1 {
        let i: u32 = *getjumpcontrol(fs, list);
        if (i >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 != OP_TESTSET as i32 as u32 {
            return 1;
        }
        list = getjump(fs, list);
    }
    return 0;
}}
pub unsafe extern "C" fn exp2reg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    reg: i32,
) { unsafe {
    discharge2reg(fs, e, reg);
    if (*e).k as u32 == VJMP as i32 as u32 {
        luak_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if (*e).t != (*e).f {
        let mut p_f: i32 = -1;
        let mut p_t: i32 = -1;
        if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
            let fj: i32 = if (*e).k as u32 == VJMP as i32 as u32 {
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
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}}
pub unsafe extern "C" fn luak_exp2nextreg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    luak_dischargevars(fs, e);
    freeexp(fs, e);
    luak_reserveregs(fs, 1);
    exp2reg(fs, e, (*fs).freereg as i32 - 1);
}}
pub unsafe extern "C" fn luak_exp2anyreg(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 { unsafe {
    luak_dischargevars(fs, e);
    if (*e).k as u32 == VNONRELOC as i32 as u32 {
        if !((*e).t != (*e).f) {
            return (*e).u.info;
        }
        if (*e).u.info >= luay_nvarstack(fs) {
            exp2reg(fs, e, (*e).u.info);
            return (*e).u.info;
        }
    }
    luak_exp2nextreg(fs, e);
    return (*e).u.info;
}}
pub unsafe extern "C" fn luak_exp2anyregup(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    if (*e).k as u32 != VUPVAL as i32 as u32 || (*e).t != (*e).f {
        luak_exp2anyreg(fs, e);
    }
}}
pub unsafe extern "C" fn luak_exp2val(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    if (*e).k as u32 == VJMP as i32 as u32 || (*e).t != (*e).f {
        luak_exp2anyreg(fs, e);
    } else {
        luak_dischargevars(fs, e);
    };
}}
pub unsafe extern "C" fn luak_exp2k(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 { unsafe {
    if !((*e).t != (*e).f) {
        let info: i32;
        match (*e).k as u32 {
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
                info = luak_int_k(fs, (*e).u.ival);
            }
            5 => {
                info = luak_number_k(fs, (*e).u.nval);
            }
            7 => {
                info = string_k(fs, (*e).u.strval);
            }
            4 => {
                info = (*e).u.info;
            }
            _ => return 0,
        }
        if info <= (1 << 8) - 1 {
            (*e).k = VK;
            (*e).u.info = info;
            return 1;
        }
    }
    return 0;
}}
pub unsafe extern "C" fn exp2rk(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 { unsafe {
    if luak_exp2k(fs, e) != 0 {
        return 1;
    } else {
        luak_exp2anyreg(fs, e);
        return 0;
    };
}}
pub unsafe extern "C" fn codeabrk(
    fs: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    ec: *mut ExpressionDescription,
) { unsafe {
    let k: i32 = exp2rk(fs, ec);
    luak_code_abck(fs, o, a, b, (*ec).u.info, k);
}}
pub unsafe extern "C" fn luak_storevar(
    fs: *mut FunctionState,
    var: *mut ExpressionDescription,
    ex: *mut ExpressionDescription,
) { unsafe {
    match (*var).k as u32 {
        9 => {
            freeexp(fs, ex);
            exp2reg(fs, ex, (*var).u.var.ridx as i32);
            return;
        }
        10 => {
            let e: i32 = luak_exp2anyreg(fs, ex);
            luak_code_abck(fs, OP_SETUPVAL, e, (*var).u.info, 0, 0);
        }
        13 => {
            codeabrk(
                fs,
                OP_SETTABUP,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        14 => {
            codeabrk(
                fs,
                OP_SETI,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        15 => {
            codeabrk(
                fs,
                OP_SETFIELD,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        12 => {
            codeabrk(
                fs,
                OP_SETTABLE,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        _ => {}
    }
    freeexp(fs, ex);
}}
pub unsafe extern "C" fn luak_self(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    key: *mut ExpressionDescription,
) { unsafe {
    luak_exp2anyreg(fs, e);
    let ereg: i32 = (*e).u.info;
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as i32;
    (*e).k = VNONRELOC;
    luak_reserveregs(fs, 2);
    codeabrk(fs, OP_SELF, (*e).u.info, ereg, key);
    freeexp(fs, key);
}}
pub unsafe extern "C" fn negatecondition(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    let program_counter: *mut u32 = getjumpcontrol(fs, (*e).u.info);
    *program_counter = *program_counter & !(!(!(0u32) << 1) << 0 + 7 + 8)
        | (((*program_counter >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 ^ 1)
            as u32)
            << 0 + 7 + 8
            & !(!(0u32) << 1) << 0 + 7 + 8;
}}
pub unsafe extern "C" fn jumponcond(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
    cond_0: i32,
) -> i32 { unsafe {
    if (*e).k as u32 == VRELOC as i32 as u32 {
        let ie: u32 = *((*(*fs).f).code).offset((*e).u.info as isize);
        if (ie >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 == OP_NOT as i32 as u32 {
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
    return condjump(
        fs,
        OP_TESTSET,
        (1 << 8) - 1,
        (*e).u.info,
        0,
        cond_0,
    );
}}
pub unsafe extern "C" fn luak_goiftrue(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    let program_counter: i32;
    luak_dischargevars(fs, e);
    match (*e).k as u32 {
        16 => {
            negatecondition(fs, e);
            program_counter = (*e).u.info;
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
}}
pub unsafe extern "C" fn luak_goiffalse(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) { unsafe {
    let program_counter: i32;
    luak_dischargevars(fs, e);
    match (*e).k as u32 {
        16 => {
            program_counter = (*e).u.info;
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
}}
pub unsafe extern "C" fn codenot(fs: *mut FunctionState, e: *mut ExpressionDescription) { unsafe {
    match (*e).k as u32 {
        1 | 3 => {
            (*e).k = VTRUE;
        }
        4 | 5 | 6 | 7 | 2 => {
            (*e).k = VFALSE;
        }
        16 => {
            negatecondition(fs, e);
        }
        17 | 8 => {
            discharge2anyreg(fs, e);
            freeexp(fs, e);
            (*e).u.info = luak_code_abck(fs, OP_NOT, 0, (*e).u.info, 0, 0);
            (*e).k = VRELOC;
        }
        _ => {}
    }
    let temp: i32 = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}}
pub unsafe extern "C" fn is_k_string(
    fs: *mut FunctionState,
    e: *mut ExpressionDescription,
) -> i32 { unsafe {
    return ((*e).k as u32 == VK as i32 as u32
        && !((*e).t != (*e).f)
        && (*e).u.info <= (1 << 8) - 1
        && (*((*(*fs).f).k).offset((*e).u.info as isize)).get_tag_variant() == TAG_VARIANT_STRING_SHORT) as i32;
}}
pub unsafe extern "C" fn is_k_int(e: *mut ExpressionDescription) -> i32 { unsafe {
    return ((*e).k as u32 == VKINT as i32 as u32 && !((*e).t != (*e).f)) as i32;
}}
pub unsafe extern "C" fn is_c_int(e: *mut ExpressionDescription) -> i32 { unsafe {
    return (is_k_int(e) != 0 && (*e).u.ival as u64 <= ((1 << 8) - 1) as u64) as i32;
}}
pub unsafe extern "C" fn is_sc_int(e: *mut ExpressionDescription) -> i32 { unsafe {
    return (is_k_int(e) != 0 && fits_c((*e).u.ival) != 0) as i32;
}}
pub unsafe extern "C" fn is_sc_number(
    e: *mut ExpressionDescription,
    pi: *mut i32,
    is_float: *mut bool,
) -> i32 { unsafe {
    let mut i: i64 = 0;
    if (*e).k as u32 == VKINT as i32 as u32 {
        i = (*e).u.ival;
    } else if (*e).k as u32 == VKFLT as i32 as u32
        && luav_flttointeger((*e).u.nval, &mut i, F2I::Equal) != 0
    {
        *is_float = true;
    } else {
        return 0;
    }
    if !((*e).t != (*e).f) && fits_c(i) != 0 {
        *pi = i as i32 + ((1 << 8) - 1 >> 1);
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn luak_indexed(
    fs: *mut FunctionState,
    t: *mut ExpressionDescription,
    k: *mut ExpressionDescription,
) { unsafe {
    if (*k).k as u32 == VKSTR as i32 as u32 {
        str_to_k(fs, k);
    }
    if (*t).k as u32 == VUPVAL as i32 as u32 && is_k_string(fs, k) == 0 {
        luak_exp2anyreg(fs, t);
    }
    if (*t).k as u32 == VUPVAL as i32 as u32 {
        let temp: i32 = (*t).u.info;
        (*t).u.ind.t = temp as u8;
        (*t).u.ind.index = (*k).u.info as i16;
        (*t).k = VINDEXUP;
    } else {
        (*t).u.ind.t = (if (*t).k as u32 == VLOCAL as i32 as u32 {
            (*t).u.var.ridx as i32
        } else {
            (*t).u.info
        }) as u8;
        if is_k_string(fs, k) != 0 {
            (*t).u.ind.index = (*k).u.info as i16;
            (*t).k = VINDEXSTR;
        } else if is_c_int(k) != 0 {
            (*t).u.ind.index = (*k).u.ival as i32 as i16;
            (*t).k = VINDEXI;
        } else {
            (*t).u.ind.index = luak_exp2anyreg(fs, k) as i16;
            (*t).k = VINDEXED;
        }
    };
}}
pub unsafe extern "C" fn validop(op: i32, v1: *mut TValue, v2: *mut TValue) -> i32 { unsafe {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i: i64 = 0;
            return (luav_tointegerns(v1, &mut i, F2I::Equal) != 0
                && luav_tointegerns(v2, &mut i, F2I::Equal) != 0) as i32;
        }
        5 | 6 | 3 => {
            return ((if (*v2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                (*v2).value.i as f64
            } else {
                (*v2).value.n
            }) != 0.0) as i32;
        }
        _ => return 1,
    };
}}
pub unsafe extern "C" fn constfolding(
    fs: *mut FunctionState,
    op: i32,
    e1: *mut ExpressionDescription,
    e2: *const ExpressionDescription,
) -> i32 { unsafe {
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
    if tonumeral(e1, &mut v1) == 0
        || tonumeral(e2, &mut v2) == 0
        || validop(op, &mut v1, &mut v2) == 0
    {
        return 0;
    }
    luao_rawarith((*(*fs).lexical_state).state, op, &mut v1, &mut v2, &mut res);
    if res.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        (*e1).k = VKINT;
        (*e1).u.ival = res.value.i;
    } else {
        let n: f64 = res.value.n;
        if !(n == n) || n == 0.0 {
            return 0;
        }
        (*e1).k = VKFLT;
        (*e1).u.nval = n;
    }
    return 1;
}}
#[inline]
pub unsafe extern "C" fn binopr2op(opr: u32, baser: u32, base: u32) -> u32 {
    return (opr as i32 - baser as i32 + base as i32) as u32;
}
#[inline]
pub unsafe extern "C" fn unopr2op(unary: Unary) -> u32 {
    return (unary as i32 - Unary::Minus as i32 + OP_UNM as i32) as u32;
}
#[inline]
pub unsafe extern "C" fn binopr2tm(opr: u32) -> u32 {
    return (opr as i32 - OPR_ADD as i32 + TM_ADD as i32) as u32;
}
pub unsafe extern "C" fn codeunexpval(
    fs: *mut FunctionState,
    op: u32,
    e: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    let r: i32 = luak_exp2anyreg(fs, e);
    freeexp(fs, e);
    (*e).u.info = luak_code_abck(fs, op, 0, r, 0, 0);
    (*e).k = VRELOC;
    luak_fixline(fs, line);
}}
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
) { unsafe {
    let v1: i32 = luak_exp2anyreg(fs, e1);
    let program_counter: i32 = luak_code_abck(fs, op, 0, v1, v2, 0);
    freeexps(fs, e1, e2);
    (*e1).u.info = program_counter;
    (*e1).k = VRELOC;
    luak_fixline(fs, line);
    luak_code_abck(fs, mmop, v1, v2, event as i32, flip);
    luak_fixline(fs, line);
}}
pub unsafe extern "C" fn codebinexpval(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    let op: u32 = binopr2op(opr, OPR_ADD, OP_ADD);
    let v2: i32 = luak_exp2anyreg(fs, e2);
    finishbinexpval(fs, e1, e2, op, v2, 0, line, OP_MMBIN, binopr2tm(opr));
}}
pub unsafe extern "C" fn codebini(
    fs: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
    event: u32,
) { unsafe {
    let v2: i32 = (*e2).u.ival as i32 + ((1 << 8) - 1 >> 1);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINI, event);
}}
pub unsafe extern "C" fn codebink(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) { unsafe {
    let event: u32 = binopr2tm(opr);
    let v2: i32 = (*e2).u.info;
    let op: u32 = binopr2op(opr, OPR_ADD, OP_ADDK);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINK, event);
}}
pub unsafe extern "C" fn finishbinexpneg(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    op: u32,
    line: i32,
    event: u32,
) -> i32 { unsafe {
    if is_k_int(e2) == 0 {
        return 0;
    } else {
        let i2: i64 = (*e2).u.ival;
        if !(fits_c(i2) != 0 && fits_c(-i2) != 0) {
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
            *((*(*fs).f).code).offset(((*fs).program_counter - 1) as isize) = *((*(*fs).f).code)
                .offset(((*fs).program_counter - 1) as isize)
                & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1)
                | ((v2 + ((1 << 8) - 1 >> 1)) as u32) << 0 + 7 + 8 + 1
                    & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
            return 1;
        }
    };
}}
pub unsafe extern "C" fn swapexps(
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) { unsafe {
    let temp: ExpressionDescription = *e1;
    *e1 = *e2;
    *e2 = temp;
}}
pub unsafe extern "C" fn codebinnok(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) { unsafe {
    if flip != 0 {
        swapexps(e1, e2);
    }
    codebinexpval(fs, opr, e1, e2, line);
}}
pub unsafe extern "C" fn codearith(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) { unsafe {
    if tonumeral(e2, std::ptr::null_mut()) != 0 && luak_exp2k(fs, e2) != 0 {
        codebink(fs, opr, e1, e2, flip, line);
    } else {
        codebinnok(fs, opr, e1, e2, flip, line);
    };
}}
pub unsafe extern "C" fn codecommutative(
    fs: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    let mut flip: i32 = 0;
    if tonumeral(e1, std::ptr::null_mut()) != 0 {
        swapexps(e1, e2);
        flip = 1;
    }
    if op as u32 == OPR_ADD as i32 as u32 && is_sc_int(e2) != 0 {
        codebini(fs, OP_ADDI, e1, e2, flip, line, TM_ADD);
    } else {
        codearith(fs, op, e1, e2, flip, line);
    };
}}
pub unsafe extern "C" fn codebitwise(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    let mut flip: i32 = 0;
    if (*e1).k as u32 == VKINT as i32 as u32 {
        swapexps(e1, e2);
        flip = 1;
    }
    if (*e2).k as u32 == VKINT as i32 as u32 && luak_exp2k(fs, e2) != 0 {
        codebink(fs, opr, e1, e2, flip, line);
    } else {
        codebinnok(fs, opr, e1, e2, flip, line);
    };
}}
pub unsafe extern "C" fn codeorder(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) { unsafe {
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
    (*e1).u.info = condjump(fs, op, r1, r2, is_float as i32, 1);
    (*e1).k = VJMP;
}}
pub unsafe extern "C" fn codeeq(
    fs: *mut FunctionState,
    opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) { unsafe {
    let r1: i32;
    let r2: i32;
    let mut im: i32 = 0;
    let mut is_float: bool = false;
    let op: u32;
    if (*e1).k as u32 != VNONRELOC as i32 as u32 {
        swapexps(e1, e2);
    }
    r1 = luak_exp2anyreg(fs, e1);
    if is_sc_number(e2, &mut im, &mut is_float) != 0 {
        op = OP_EQI;
        r2 = im;
    } else if exp2rk(fs, e2) != 0 {
        op = OP_EQK;
        r2 = (*e2).u.info;
    } else {
        op = OP_EQ;
        r2 = luak_exp2anyreg(fs, e2);
    }
    freeexps(fs, e1, e2);
    (*e1).u.info = condjump(
        fs,
        op,
        r1,
        r2,
        is_float as i32,
        (opr as u32 == OPR_EQ as i32 as u32) as i32,
    );
    (*e1).k = VJMP;
}}
pub unsafe extern "C" fn luak_prefix(
    fs: *mut FunctionState,
    opr: Unary,
    e: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    static mut EF: ExpressionDescription = {
        let init = ExpressionDescription {
            k: VKINT,
            u: RawValue { ival: 0 },
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
                (opr as u32).wrapping_add(12 as i32 as u32) as i32,
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
}}
pub unsafe extern "C" fn luak_infix(
    fs: *mut FunctionState,
    op: u32,
    v: *mut ExpressionDescription,
) { unsafe {
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
            if tonumeral(v, std::ptr::null_mut()) == 0 {
                luak_exp2anyreg(fs, v);
            }
        }
        13 | 16 => {
            if tonumeral(v, std::ptr::null_mut()) == 0 {
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
}}
pub unsafe extern "C" fn codeconcat(
    fs: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    let ie2: *mut u32 = previousinstruction(fs);
    if (*ie2 >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 == OP_CONCAT as i32 as u32 {
        let n: i32 = (*ie2 >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
        freeexp(fs, e2);
        *ie2 = *ie2 & !(!(!(0u32) << 8) << 0 + 7)
            | ((*e1).u.info as u32) << 0 + 7 & !(!(0u32) << 8) << 0 + 7;
        *ie2 = *ie2 & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1)
            | ((n + 1) as u32) << 0 + 7 + 8 + 1
                & !(!(0u32) << 8) << 0 + 7 + 8 + 1;
    } else {
        luak_code_abck(fs, OP_CONCAT, (*e1).u.info, 2, 0, 0);
        freeexp(fs, e2);
        luak_fixline(fs, line);
    };
}}
pub unsafe extern "C" fn luak_posfix(
    fs: *mut FunctionState,
    mut opr: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) { unsafe {
    luak_dischargevars(fs, e2);
    if opr as u32 <= OPR_SHR as i32 as u32
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
            if is_sc_int(e1) != 0 {
                swapexps(e1, e2);
                codebini(fs, OP_SHLI, e1, e2, 1, line, TM_SHL);
            } else if !(finishbinexpneg(fs, e1, e2, OP_SHRI, line, TM_SHL) != 0) {
                codebinexpval(fs, opr, e1, e2, line);
            }
            current_block_30 = 8180496224585318153;
        }
        11 => {
            if is_sc_int(e2) != 0 {
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
                .wrapping_sub(OPR_GT as i32 as u32)
                .wrapping_add(OPR_LT as i32 as u32) as u32;
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
}}
pub unsafe extern "C" fn luak_fixline(fs: *mut FunctionState, line: i32) { unsafe {
    removelastlineinfo(fs);
    savelineinfo(fs, (*fs).f, line);
}}
pub unsafe extern "C" fn luak_settablesize(
    fs: *mut FunctionState,
    program_counter: i32,
    ra: i32,
    asize: i32,
    hsize: i32,
) { unsafe {
    let inst: *mut u32 = &mut *((*(*fs).f).code).offset(program_counter as isize) as *mut u32;
    let rb: i32 = if hsize != 0 {
        ceiling_log2(hsize as u64) as i32 + 1
    } else {
        0
    };
    let extra: i32 = asize / ((1 << 8) - 1 + 1);
    let rc: i32 = asize % ((1 << 8) - 1 + 1);
    let k: i32 = (extra > 0) as i32;
    *inst = (OP_NEWTABLE as i32 as u32) << 0
        | (ra as u32) << 0 + 7
        | (rb as u32) << 0 + 7 + 8 + 1
        | (rc as u32) << 0 + 7 + 8 + 1 + 8
        | (k as u32) << 0 + 7 + 8;
    *inst.offset(1 as isize) =
        (OP_EXTRAARG as i32 as u32) << 0 | (extra as u32) << 0 + 7;
}}
pub unsafe extern "C" fn luak_setlist(
    fs: *mut FunctionState,
    base: i32,
    mut count_elements: i32,
    mut tostore: i32,
) { unsafe {
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
}}
pub unsafe extern "C" fn finaltarget(code: *mut u32, mut i: i32) -> i32 { unsafe {
    let mut count: i32 = 0;
    while count < 100 as i32 {
        let program_counter: u32 = *code.offset(i as isize);
        if (program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 as u32
            != OP_JMP as i32 as u32
        {
            break;
        }
        i += (program_counter >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
            as i32
            - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
            + 1;
        count += 1;
    }
    return i;
}}
pub unsafe extern "C" fn luak_finish(fs: *mut FunctionState) { unsafe {
    let mut i: i32;
    let p: *mut Prototype = (*fs).f;
    i = 0;
    while i < (*fs).program_counter {
        let program_counter: *mut u32 = &mut *((*p).code).offset(i as isize) as *mut u32;
        let current_block_7: u64;
        match (*program_counter >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 {
            71 | 72 => {
                if !((*fs).needclose as i32 != 0 || (*p).is_variable_arguments as i32 != 0) {
                    current_block_7 = 12599329904712511516;
                } else {
                    *program_counter = *program_counter & !(!(!(0u32) << 7) << 0)
                        | (OP_RETURN as i32 as u32) << 0 & !(!(0u32) << 7) << 0;
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
                if (*fs).needclose != 0 {
                    *program_counter = *program_counter
                        & !(!(!(0u32) << 1) << 0 + 7 + 8)
                        | (1 as u32) << 0 + 7 + 8
                            & !(!(0u32) << 1) << 0 + 7 + 8;
                }
                if (*p).is_variable_arguments {
                    *program_counter = *program_counter
                        & !(!(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8)
                        | (((*p).count_parameters as i32 + 1) as u32)
                            << 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0 + 7 + 8 + 1 + 8;
                }
            }
            _ => {}
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn l_strton(obj: *const TValue, result: *mut TValue) -> i32 { unsafe {
    if !(get_tag_type((*obj).get_tag()) == TAG_TYPE_STRING) {
        return 0;
    } else {
        let st: *mut TString = &mut (*((*obj).value.object as *mut GCUnion)).ts;
        return (luao_str2num(((*st).contents).as_mut_ptr(), result)
            == (if (*st).short_length as i32 != 0xFF as i32 {
                (*st).short_length as u64
            } else {
                (*st).u.long_length
            })
            .wrapping_add(1 as u64)) as i32;
    };
}}
pub unsafe extern "C" fn luav_tonumber_(obj: *const TValue, n: *mut f64) -> bool { unsafe {
    let mut v: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    if (*obj).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        *n = (*obj).value.i as f64;
        return true;
    } else if l_strton(obj, &mut v) != 0 {
        *n = if v.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            v.value.i as f64
        } else {
            v.value.n
        };
        return true;
    } else {
        return false;
    };
}}
pub unsafe extern "C" fn luav_flttointeger(n: f64, p: *mut i64, mode: F2I) -> i32 { unsafe {
    let mut f: f64 = n.floor();
    if n != f {
        if mode == F2I::Equal {
            return 0;
        } else if mode == F2I::Ceiling {
            f += 1.0;
        }
    }
    return (f >= (-(9223372036854775807 as i64) - 1 as i64) as f64
        && f < -((-(9223372036854775807 as i64) - 1 as i64) as f64)
        && {
            *p = f as i64;
            1 != 0
        }) as i32;
}}
pub unsafe extern "C" fn luav_tointegerns(
    obj: *const TValue,
    p: *mut i64,
    mode: F2I,
) -> i32 { unsafe {
    if (*obj).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
        return luav_flttointeger((*obj).value.n, p, mode);
    } else if (*obj).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        *p = (*obj).value.i;
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn luav_tointeger(
    mut obj: *const TValue,
    p: *mut i64,
    mode: F2I,
) -> i32 { unsafe {
    let mut v: TValue = TValue {
        value: Value {
            object: std::ptr::null_mut(),
        },
        tag: 0,
    };
    if l_strton(obj, &mut v) != 0 {
        obj = &mut v;
    }
    return luav_tointegerns(obj, p, mode);
}}
pub unsafe extern "C" fn forlimit(
    state: *mut State,
    init: i64,
    lim: *const TValue,
    p: *mut i64,
    step: i64,
) -> i32 { unsafe {
    if luav_tointeger(
        lim,
        p,
        if step < 0 {
            F2I::Ceiling
        } else {
            F2I::Floor
        },
    ) == 0
    {
        let mut flim: f64 = 0.0;
        if if (*lim).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            flim = (*lim).value.n;
            1
        } else {
            if luav_tonumber_(lim, &mut flim) { 1 } else { 0 }
        } == 0
        {
            luag_forerror(state, lim, b"limit\0" as *const u8 as *const i8);
        }
        if (0.0) < flim {
            if step < 0 {
                return 1;
            }
            *p = 9223372036854775807 as i64;
        } else {
            if step > 0 {
                return 1;
            }
            *p = -(9223372036854775807 as i64) - 1 as i64;
        }
    }
    return if step > 0 {
        (init > *p) as i32
    } else {
        (init < *p) as i32
    };
}}
pub unsafe extern "C" fn forprep(state: *mut State, ra: StkId) -> i32 { unsafe {
    let pinit: *mut TValue = &mut (*ra).value;
    let plimit: *mut TValue = &mut (*ra.offset(1 as isize)).value;
    let pstep: *mut TValue = &mut (*ra.offset(2 as isize)).value;
    if (*pinit).get_tag() == TAG_VARIANT_NUMERIC_INTEGER && (*pstep).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
    {
        let init: i64 = (*pinit).value.i;
        let step: i64 = (*pstep).value.i;
        let mut limit: i64 = 0;
        if step == 0 {
            luag_runerror(state, b"'for' step is zero\0" as *const u8 as *const i8);
        }
        let io: *mut TValue = &mut (*ra.offset(3 as isize)).value;
        (*io).value.i = init;
        (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
        if forlimit(state, init, plimit, &mut limit, step) != 0 {
            return 1;
        } else {
            let mut count: u64;
            if step > 0 {
                count = (limit as u64).wrapping_sub(init as u64);
                if step != 1 {
                    count = (count as u64).wrapping_div(step as u64) as u64 as u64;
                }
            } else {
                count = (init as u64).wrapping_sub(limit as u64);
                count = (count as u64)
                    .wrapping_div((-(step + 1) as u64).wrapping_add(1 as u32 as u64))
                    as u64 as u64;
            }
            let io_0: *mut TValue = plimit;
            (*io_0).value.i = count as i64;
            (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        }
    } else {
        let mut init_0: f64 = 0.0;
        let mut limit_0: f64 = 0.0;
        let mut step_0: f64 = 0.0;
        if (((if (*plimit).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            limit_0 = (*plimit).value.n;
            1
        } else {
            if luav_tonumber_(plimit, &mut limit_0) { 1 } else { 0 }
        }) == 0) as i32
            != 0) as i32 as i64
            != 0
        {
            luag_forerror(state, plimit, b"limit\0" as *const u8 as *const i8);
        }
        if (((if (*pstep).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            step_0 = (*pstep).value.n;
            1
        } else {
            if luav_tonumber_(pstep, &mut step_0) { 1 } else { 0 }
        }) == 0) as i32
            != 0) as i32 as i64
            != 0
        {
            luag_forerror(state, pstep, b"step\0" as *const u8 as *const i8);
        }
        if (((if (*pinit).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            init_0 = (*pinit).value.n;
            1
        } else {
            if luav_tonumber_(pinit, &mut init_0) { 1 } else { 0 }
        }) == 0) as i32
            != 0) as i32 as i64
            != 0
        {
            luag_forerror(state, pinit, b"initial value\0" as *const u8 as *const i8);
        }
        if step_0 == 0.0 {
            luag_runerror(state, b"'for' step is zero\0" as *const u8 as *const i8);
        }
        if if (0.0) < step_0 {
            (limit_0 < init_0) as i32
        } else {
            (init_0 < limit_0) as i32
        } != 0
        {
            return 1;
        } else {
            let io_1: *mut TValue = plimit;
            (*io_1).value.n = limit_0;
            (*io_1).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            let io_2: *mut TValue = pstep;
            (*io_2).value.n = step_0;
            (*io_2).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            let io_3: *mut TValue = &mut (*ra).value;
            (*io_3).value.n = init_0;
            (*io_3).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            let io_4: *mut TValue = &mut (*ra.offset(3 as isize)).value;
            (*io_4).value.n = init_0;
            (*io_4).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        }
    }
    return 0;
}}
pub unsafe extern "C" fn floatforloop(ra: StkId) -> i32 { unsafe {
    let step: f64 = (*ra.offset(2 as isize)).value.value.n;
    let limit: f64 = (*ra.offset(1 as isize)).value.value.n;
    let mut index: f64 = (*ra).value.value.n;
    index = index + step;
    if if (0.0) < step {
        (index <= limit) as i32
    } else {
        (limit <= index) as i32
    } != 0
    {
        let io: *mut TValue = &mut (*ra).value;
        (*io).value.n = index;
        let io_0: *mut TValue = &mut (*ra.offset(3 as isize)).value;
        (*io_0).value.n = index;
        (*io_0).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        return 1;
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn luav_finishget(
    state: *mut State,
    mut t: *const TValue,
    key: *mut TValue,
    value: StkId,
    mut slot: *const TValue,
) { unsafe {
    let mut loop_0: i32 = 0;
    let mut tm: *const TValue;
    while loop_0 < 2000 as i32 {
        if slot.is_null() {
            tm = luat_gettmbyobj(state, t, TM_INDEX);
            if ((get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i32 as i64 != 0 {
                luag_typeerror(state, t, b"index\0" as *const u8 as *const i8);
            }
        } else {
            tm = if ((*((*t).value.object as *mut GCUnion)).h.metatable).is_null() {
                std::ptr::null()
            } else if (*(*((*t).value.object as *mut GCUnion)).h.metatable).flags as u32
                & (1 as u32) << TM_INDEX as i32
                != 0
            {
                std::ptr::null()
            } else {
                luat_gettm(
                    (*((*t).value.object as *mut GCUnion)).h.metatable,
                    TM_INDEX,
                    (*(*state).global).tmname[TM_INDEX as i32 as usize],
                )
            };
            if tm.is_null() {
                (*value).value.set_tag (TAG_VARIANT_NIL_NIL);
                return;
            }
        }
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_CLOSURE {
            luat_calltmres(state, tm, t, key, value);
            return;
        }
        t = tm;
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_get(&mut (*((*t).value.object as *mut GCUnion)).h, key);
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*value).value;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            return;
        }
        loop_0 += 1;
    }
    luag_runerror(
        state,
        b"'__index' chain too long; possible loop\0" as *const u8 as *const i8,
    );
}}
pub unsafe extern "C" fn luav_finishset(
    state: *mut State,
    mut t: *const TValue,
    key: *mut TValue,
    value: *mut TValue,
    mut slot: *const TValue,
) { unsafe {
    let mut loop_0: i32 = 0;
    while loop_0 < 2000 as i32 {
        let tm: *const TValue;
        if !slot.is_null() {
            let h: *mut Table = &mut (*((*t).value.object as *mut GCUnion)).h;
            tm = if ((*h).metatable).is_null() {
                std::ptr::null()
            } else if (*(*h).metatable).flags as u32 & (1 as u32) << TM_NEWINDEX as i32 != 0 {
                std::ptr::null()
            } else {
                luat_gettm(
                    (*h).metatable,
                    TM_NEWINDEX,
                    (*(*state).global).tmname[TM_NEWINDEX as i32 as usize],
                )
            };
            if tm.is_null() {
                let io: *mut TValue = &mut (*(*state).top.p).value;
                let x_: *mut Table = h;
                (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
                (*io).set_tag (TAG_VARIANT_TABLE);
                (*io).set_collectable();
                (*state).top.p = (*state).top.p.offset(1);
                            luah_finishset(state, h, key, slot, value);
                (*state).top.p = (*state).top.p.offset(-1);
                            (*h).flags = ((*h).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
                if (*value).is_collectable() {
                    if (*(h as *mut GCUnion)).object.get_marked() & 1 << 5 != 0
                        && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrierback_(state, &mut (*(h as *mut GCUnion)).object);
                    } else {
                    };
                } else {
                };
                return;
            }
        } else {
            tm = luat_gettmbyobj(state, t, TM_NEWINDEX);
            if ((get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i32 as i64 != 0 {
                luag_typeerror(state, t, b"index\0" as *const u8 as *const i8);
            }
        }
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_CLOSURE {
            luat_calltm(state, tm, t, key, value);
            return;
        }
        t = tm;
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_get(&mut (*((*t).value.object as *mut GCUnion)).h, key);
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
            if (*value).is_collectable() {
                if (*(*t).value.object).get_marked() & 1 << 5 != 0
                    && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrierback_(state, (*t).value.object);
                } else {
                };
            } else {
            };
            return;
        }
        loop_0 += 1;
    }
    luag_runerror(
        state,
        b"'__newindex' chain too long; possible loop\0" as *const u8 as *const i8,
    );
}}
pub unsafe extern "C" fn l_strcmp(ts1: *const TString, ts2: *const TString) -> i32 { unsafe {
    let mut s1: *const i8 = ((*ts1).contents).as_ptr();
    let mut rl1: u64 = if (*ts1).short_length as i32 != 0xFF as i32 {
        (*ts1).short_length as u64
    } else {
        (*ts1).u.long_length
    };
    let mut s2: *const i8 = ((*ts2).contents).as_ptr();
    let mut rl2: u64 = if (*ts2).short_length as i32 != 0xFF as i32 {
        (*ts2).short_length as u64
    } else {
        (*ts2).u.long_length
    };
    loop {
        let temp: i32 = strcoll(s1, s2);
        if temp != 0 {
            return temp;
        } else {
            let mut zl1: u64 = strlen(s1);
            let mut zl2: u64 = strlen(s2);
            if zl2 == rl2 {
                return if zl1 == rl1 { 0 } else { 1 };
            } else if zl1 == rl1 {
                return -1;
            }
            zl1 = zl1.wrapping_add(1);
            zl2 = zl2.wrapping_add(1);
            s1 = s1.offset(zl1 as isize);
            rl1 = (rl1 as u64).wrapping_sub(zl1) as u64 as u64;
            s2 = s2.offset(zl2 as isize);
            rl2 = (rl2 as u64).wrapping_sub(zl2) as u64 as u64;
        }
    }
}}
#[inline]
pub unsafe extern "C" fn ltintfloat(i: i64, f: f64) -> i32 { unsafe {
    if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
        <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
    {
        return ((i as f64) < f) as i32;
    } else {
        let mut fi: i64 = 0;
        if luav_flttointeger(f, &mut fi, F2I::Ceiling) != 0 {
            return (i < fi) as i32;
        } else {
            return (f > 0.0) as i32;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn leintfloat(i: i64, f: f64) -> i32 { unsafe {
    if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
        <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
    {
        return (i as f64 <= f) as i32;
    } else {
        let mut fi: i64 = 0;
        if luav_flttointeger(f, &mut fi, F2I::Floor) != 0 {
            return (i <= fi) as i32;
        } else {
            return (f > 0.0) as i32;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn ltfloatint(f: f64, i: i64) -> i32 { unsafe {
    if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
        <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
    {
        return (f < i as f64) as i32;
    } else {
        let mut fi: i64 = 0;
        if luav_flttointeger(f, &mut fi, F2I::Floor) != 0 {
            return (fi < i) as i32;
        } else {
            return (f < 0.0) as i32;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn lefloatint(f: f64, i: i64) -> i32 { unsafe {
    if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
        <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
    {
        return (f <= i as f64) as i32;
    } else {
        let mut fi: i64 = 0;
        if luav_flttointeger(f, &mut fi, F2I::Ceiling) != 0 {
            return (fi <= i) as i32;
        } else {
            return (f < 0.0) as i32;
        }
    };
}}
#[inline]
pub unsafe extern "C" fn ltnum(l: *const TValue, r: *const TValue) -> i32 { unsafe {
    if (*l).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        let li: i64 = (*l).value.i;
        if (*r).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            return (li < (*r).value.i) as i32;
        } else {
            return ltintfloat(li, (*r).value.n);
        }
    } else {
        let lf: f64 = (*l).value.n;
        if (*r).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            return (lf < (*r).value.n) as i32;
        } else {
            return ltfloatint(lf, (*r).value.i);
        }
    };
}}
#[inline]
pub unsafe extern "C" fn lenum(l: *const TValue, r: *const TValue) -> i32 { unsafe {
    if (*l).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
        let li: i64 = (*l).value.i;
        if (*r).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            return (li <= (*r).value.i) as i32;
        } else {
            return leintfloat(li, (*r).value.n);
        }
    } else {
        let lf: f64 = (*l).value.n;
        if (*r).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            return (lf <= (*r).value.n) as i32;
        } else {
            return lefloatint(lf, (*r).value.i);
        }
    };
}}
pub unsafe extern "C" fn lessthanothers(
    state: *mut State,
    l: *const TValue,
    r: *const TValue,
) -> i32 { unsafe {
    if get_tag_type ((*l).get_tag()) == TAG_TYPE_STRING && get_tag_type ((*r).get_tag()) == TAG_TYPE_STRING {
        return (l_strcmp(
            &mut (*((*l).value.object as *mut GCUnion)).ts,
            &mut (*((*r).value.object as *mut GCUnion)).ts,
        ) < 0) as i32;
    } else {
        return luat_callordertm(state, l, r, TM_LT);
    };
}}
pub unsafe extern "C" fn luav_lessthan(
    state: *mut State,
    l: *const TValue,
    r: *const TValue,
) -> i32 { unsafe {
    if get_tag_type ((*l).get_tag()) == TAG_TYPE_NUMERIC && get_tag_type ((*r).get_tag()) == TAG_TYPE_NUMERIC {
        return ltnum(l, r);
    } else {
        return lessthanothers(state, l, r);
    };
}}
pub unsafe extern "C" fn lessequalothers(
    state: *mut State,
    l: *const TValue,
    r: *const TValue,
) -> i32 { unsafe {
    if get_tag_type ((*l).get_tag()) == TAG_TYPE_STRING && get_tag_type ((*r).get_tag()) == TAG_TYPE_STRING {
        return (l_strcmp(
            &mut (*((*l).value.object as *mut GCUnion)).ts,
            &mut (*((*r).value.object as *mut GCUnion)).ts,
        ) <= 0) as i32;
    } else {
        return luat_callordertm(state, l, r, TM_LE);
    }
}}
pub unsafe extern "C" fn luav_lessequal(
    state: *mut State,
    l: *const TValue,
    r: *const TValue,
) -> i32 { unsafe {
    if get_tag_type ((*l).get_tag()) == TAG_TYPE_NUMERIC && get_tag_type ((*r).get_tag()) == TAG_TYPE_NUMERIC {
        return lenum(l, r);
    } else {
        return lessequalothers(state, l, r);
    };
}}
pub unsafe extern "C" fn luav_equalobj(
    state: *mut State,
    t1: *const TValue,
    t2: *const TValue,
) -> i32 { unsafe {
    let mut tm: *const TValue;
    if (*t1).get_tag_variant() != (*t2).get_tag_variant() {
        if (*t1).get_tag_type() != (*t2).get_tag_type()
            || (*t1).get_tag_type() != TAG_TYPE_NUMERIC
        {
            return 0;
        } else {
            let mut i1: i64 = 0;
            let mut i2: i64 = 0;
            return (luav_tointegerns(t1, &mut i1, F2I::Equal) != 0
                && luav_tointegerns(t2, &mut i2, F2I::Equal) != 0
                && i1 == i2) as i32;
        }
    }
    match (*t1).get_tag_variant() {
        TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => return 1,
        TAG_VARIANT_NUMERIC_INTEGER => return ((*t1).value.i == (*t2).value.i) as i32,
        TAG_VARIANT_NUMERIC_NUMBER => return ((*t1).value.n == (*t2).value.n) as i32,
        TAG_VARIANT_POINTER => return ((*t1).value.p == (*t2).value.p) as i32,
        TAG_VARIANT_CLOSURE_CFUNCTION => return ((*t1).value.f == (*t2).value.f) as i32,
        TAG_VARIANT_STRING_SHORT => {
            return (&mut (*((*t1).value.object as *mut GCUnion)).ts as *mut TString
                == &mut (*((*t2).value.object as *mut GCUnion)).ts as *mut TString)
                as i32;
        }
        TAG_VARIANT_STRING_LONG => {
            return luas_eqlngstr(
                &mut (*((*t1).value.object as *mut GCUnion)).ts,
                &mut (*((*t2).value.object as *mut GCUnion)).ts,
            );
        }
        TAG_VARIANT_USER => {
            if &mut (*((*t1).value.object as *mut GCUnion)).u as *mut User
                == &mut (*((*t2).value.object as *mut GCUnion)).u as *mut User
            {
                return 1;
            } else if state.is_null() {
                return 0;
            }
            tm = if ((*((*t1).value.object as *mut GCUnion)).u.metatable).is_null() {
                std::ptr::null()
            } else if (*(*((*t1).value.object as *mut GCUnion)).u.metatable).flags as u32
                & (1 as u32) << TM_EQ as i32
                != 0
            {
                std::ptr::null()
            } else {
                luat_gettm(
                    (*((*t1).value.object as *mut GCUnion)).u.metatable,
                    TM_EQ,
                    (*(*state).global).tmname[TM_EQ as i32 as usize],
                )
            };
            if tm.is_null() {
                tm = if ((*((*t2).value.object as *mut GCUnion)).u.metatable).is_null() {
                    std::ptr::null()
                } else if (*(*((*t2).value.object as *mut GCUnion)).u.metatable).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*((*t2).value.object as *mut GCUnion)).u.metatable,
                        TM_EQ,
                        (*(*state).global).tmname[TM_EQ as i32 as usize],
                    )
                };
            }
        }
        TAG_VARIANT_TABLE => {
            if &mut (*((*t1).value.object as *mut GCUnion)).h as *mut Table
                == &mut (*((*t2).value.object as *mut GCUnion)).h as *mut Table
            {
                return 1;
            } else if state.is_null() {
                return 0;
            }
            tm = if ((*((*t1).value.object as *mut GCUnion)).h.metatable).is_null() {
                std::ptr::null()
            } else if (*(*((*t1).value.object as *mut GCUnion)).h.metatable).flags as u32
                & (1 as u32) << TM_EQ as i32
                != 0
            {
                std::ptr::null()
            } else {
                luat_gettm(
                    (*((*t1).value.object as *mut GCUnion)).h.metatable,
                    TM_EQ,
                    (*(*state).global).tmname[TM_EQ as i32 as usize],
                )
            };
            if tm.is_null() {
                tm = if ((*((*t2).value.object as *mut GCUnion)).h.metatable).is_null() {
                    std::ptr::null()
                } else if (*(*((*t2).value.object as *mut GCUnion)).h.metatable).flags as u32
                    & (1 as u32) << TM_EQ as i32
                    != 0
                {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*((*t2).value.object as *mut GCUnion)).h.metatable,
                        TM_EQ,
                        (*(*state).global).tmname[TM_EQ as i32 as usize],
                    )
                };
            }
        }
        _ => return ((*t1).value.object == (*t2).value.object) as i32,
    }
    if tm.is_null() {
        return 0;
    } else {
        luat_calltmres(state, tm, t1, t2, (*state).top.p);
        return !((*(*state).top.p).value.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
            || get_tag_type((*(*state).top.p).value.get_tag()) == TAG_TYPE_NIL) as i32;
    };
}}
pub unsafe extern "C" fn copy2buff(top: StkId, mut n: i32, buffer: *mut i8) { unsafe {
    let mut tl: u64 = 0;
    loop {
        let st: *mut TString =
            &mut (*((*top.offset(-(n as isize))).value.value.object as *mut GCUnion)).ts;
        let l: u64 = if (*st).short_length as i32 != 0xFF as i32 {
            (*st).short_length as u64
        } else {
            (*st).u.long_length
        };
        memcpy(
            buffer.offset(tl as isize) as *mut libc::c_void,
            ((*st).contents).as_mut_ptr() as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        tl = (tl as u64).wrapping_add(l) as u64 as u64;
        n -= 1;
        if !(n > 0) {
            break;
        }
    }
}}
pub unsafe extern "C" fn luav_concat(state: *mut State, mut total: i32) { unsafe {
    if total == 1 {
        return;
    }
    loop {
        let top: StkId = (*state).top.p;
        let mut n: i32 = 2;
        if !(get_tag_type ((*top.offset(-(2 as isize))).value.get_tag()) == TAG_TYPE_STRING
            || get_tag_type ((*top.offset(-(2 as isize))).value.get_tag()) == TAG_TYPE_NUMERIC)
            || !(get_tag_type ((*top.offset(-(1 as isize))).value.get_tag()) == TAG_TYPE_STRING
                || get_tag_type ((*top.offset(-(1 as isize))).value.get_tag()) == TAG_TYPE_NUMERIC && {
                    luao_tostring(state, &mut (*top.offset(-(1 as isize))).value);
                    1 != 0
                })
        {
            luat_tryconcattm(state);
        } else if (*top.offset(-(1 as isize))).value.get_tag_variant() == TAG_VARIANT_STRING_SHORT
            && (*((*top.offset(-(1 as isize))).value.value.object as *mut GCUnion))
                .ts
                .short_length as i32
                == 0
        {
            (get_tag_type((*top.offset(-(2 as isize))).value.get_tag()) == TAG_TYPE_STRING
                || get_tag_type((*top.offset(-(2 as isize))).value.get_tag()) == TAG_TYPE_NUMERIC && {
                    luao_tostring(state, &mut (*top.offset(-(2 as isize))).value);
                    1 != 0
                }) as i32;
        } else if (*top.offset(-(2 as isize))).value.get_tag_variant() == TAG_VARIANT_STRING_SHORT
            && (*((*top.offset(-(2 as isize))).value.value.object as *mut GCUnion))
                .ts
                .short_length as i32
                == 0
        {
            let io1: *mut TValue = &mut (*top.offset(-(2 as isize))).value;
            let io2: *const TValue = &mut (*top.offset(-(1 as isize))).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
        } else {
            let mut tl: u64 = if (*((*top.offset(-(1 as isize))).value.value.object as *mut GCUnion))
                .ts
                .short_length as i32
                != 0xFF as i32
            {
                (*((*top.offset(-(1 as isize))).value.value.object as *mut GCUnion))
                    .ts
                    .short_length as u64
            } else {
                (*((*top.offset(-(1 as isize))).value.value.object as *mut GCUnion))
                    .ts
                    .u
                    .long_length
            };
            let ts: *mut TString;
            n = 1;
            while n < total
                && (get_tag_type((*top.offset(-(n as isize)).offset(-(1 as isize)))
                    .value
                    .get_tag())
                    == 4
                    || get_tag_type((*top.offset(-(n as isize)).offset(-(1 as isize)))
                        .value
                        .get_tag())
                        == 3
                        && {
                            luao_tostring(
                                state,
                                &mut (*top.offset(-(n as isize)).offset(-(1 as isize))).value,
                            );
                            1 != 0
                        })
            {
                let l: u64 = if (*((*top.offset(-(n as isize)).offset(-(1 as isize)))
                    .value
                    .value
                    .object as *mut GCUnion))
                    .ts
                    .short_length as i32
                    != 0xFF as i32
                {
                    (*((*top.offset(-(n as isize)).offset(-(1 as isize)))
                        .value
                        .value
                        .object as *mut GCUnion))
                        .ts
                        .short_length as u64
                } else {
                    (*((*top.offset(-(n as isize)).offset(-(1 as isize)))
                        .value
                        .value
                        .object as *mut GCUnion))
                        .ts
                        .u
                        .long_length
                };
                if ((l
                    >= (if (::core::mem::size_of::<u64>() as u64)
                        < ::core::mem::size_of::<i64>() as u64
                    {
                        !(0u64)
                    } else {
                        9223372036854775807 as i64 as u64
                    })
                    .wrapping_sub(::core::mem::size_of::<TString>() as u64)
                    .wrapping_sub(tl)) as i32
                    != 0) as i32 as i64
                    != 0
                {
                    (*state).top.p = top.offset(-(total as isize));
                    luag_runerror(state, b"string length overflow\0" as *const u8 as *const i8);
                }
                tl = (tl as u64).wrapping_add(l) as u64 as u64;
                n += 1;
            }
            if tl <= 40 as i32 as u64 {
                let mut buffer: [i8; 40] = [0; 40];
                copy2buff(top, n, buffer.as_mut_ptr());
                ts = luas_newlstr(state, buffer.as_mut_ptr(), tl);
            } else {
                ts = luas_createlngstrobj(state, tl);
                copy2buff(top, n, ((*ts).contents).as_mut_ptr());
            }
            let io: *mut TValue = &mut (*top.offset(-(n as isize))).value;
            let x_: *mut TString = ts;
            (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
            (*io).set_tag ((*x_).get_tag());
            (*io).set_collectable();
        }
        total -= n - 1;
        (*state).top.p = (*state).top.p.offset(-((n - 1) as isize));
        if !(total > 1) {
            break;
        }
    }
}}
pub unsafe extern "C" fn luav_objlen(state: *mut State, ra: StkId, rb: *const TValue) { unsafe {
    let tm: *const TValue;
    match (*rb).get_tag_variant() {
        5 => {
            let h: *mut Table = &mut (*((*rb).value.object as *mut GCUnion)).h;
            tm = if ((*h).metatable).is_null() {
                std::ptr::null()
            } else if (*(*h).metatable).flags as u32 & (1 as u32) << TM_LEN as i32 != 0 {
                std::ptr::null()
            } else {
                luat_gettm(
                    (*h).metatable,
                    TM_LEN,
                    (*(*state).global).tmname[TM_LEN as i32 as usize],
                )
            };
            if tm.is_null() {
                let io: *mut TValue = &mut (*ra).value;
                (*io).value.i = luah_getn(h) as i64;
                (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
                return;
            }
        }
        4 => {
            let io_0: *mut TValue = &mut (*ra).value;
            (*io_0).value.i = (*((*rb).value.object as *mut GCUnion)).ts.short_length as i64;
            (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            return;
        }
        20 => {
            let io_1: *mut TValue = &mut (*ra).value;
            (*io_1).value.i = (*((*rb).value.object as *mut GCUnion)).ts.u.long_length as i64;
            (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            return;
        }
        _ => {
            tm = luat_gettmbyobj(state, rb, TM_LEN);
            if ((get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i32 as i64 != 0 {
                luag_typeerror(state, rb, b"get length of\0" as *const u8 as *const i8);
            }
        }
    }
    luat_calltmres(state, tm, rb, rb, ra);
}}
pub unsafe extern "C" fn luav_idiv(state: *mut State, m: i64, n: i64) -> i64 { unsafe {
    if (((n as u64).wrapping_add(1 as u32 as u64) <= 1 as u32 as u64) as i32 != 0) as i32 as i64
        != 0
    {
        if n == 0 {
            luag_runerror(
                state,
                b"attempt to divide by zero\0" as *const u8 as *const i8,
            );
        }
        return (0u64).wrapping_sub(m as u64) as i64;
    } else {
        let mut q: i64 = m / n;
        if m ^ n < 0 && m % n != 0 {
            q -= 1;
        }
        return q;
    };
}}
pub unsafe extern "C" fn luav_mod(state: *mut State, m: i64, n: i64) -> i64 { unsafe {
    if (((n as u64).wrapping_add(1 as u32 as u64) <= 1 as u32 as u64) as i32 != 0) as i32 as i64
        != 0
    {
        if n == 0 {
            luag_runerror(
                state,
                b"attempt to perform 'n%%0'\0" as *const u8 as *const i8,
            );
        }
        return 0;
    } else {
        let mut r: i64 = m % n;
        if r != 0 && r ^ n < 0 {
            r += n;
        }
        return r;
    };
}}
pub unsafe extern "C" fn luav_modf(mut _state: *mut State, m: f64, n: f64) -> f64 { unsafe {
    let mut r: f64 = fmod(m, n);
    if if r > 0.0 {
        (n < 0.0) as i32
    } else {
        (r < 0.0 && n > 0.0) as i32
    } != 0
    {
        r += n;
    }
    return r;
}}
pub unsafe extern "C" fn luav_shiftl(x: i64, y: i64) -> i64 {
    if y < 0 {
        if y <= -((::core::mem::size_of::<i64>() as u64).wrapping_mul(8 as u64) as i32) as i64 {
            return 0;
        } else {
            return (x as u64 >> -y as u64) as i64;
        }
    } else if y >= (::core::mem::size_of::<i64>() as u64).wrapping_mul(8 as u64) as i32 as i64 {
        return 0;
    } else {
        return ((x as u64) << y as u64) as i64;
    };
}
pub unsafe extern "C" fn pushclosure(
    state: *mut State,
    p: *mut Prototype,
    encup: *mut *mut UpValue,
    base: StkId,
    ra: StkId,
) { unsafe {
    let nup: i32 = (*p).size_upvalues;
    let uv: *mut Upvaldesc = (*p).upvalues;
    let mut i: i32;
    let ncl: *mut LClosure = luaf_newlclosure(state, nup);
    (*ncl).p = p;
    let io: *mut TValue = &mut (*ra).value;
    let x_: *mut LClosure = ncl;
    (*io).value.object = &mut (*(x_ as *mut GCUnion)).object;
    (*io).set_tag (TAG_VARIANT_CLOSURE_L);
    (*io).set_collectable();
    i = 0;
    while i < nup {
        if (*uv.offset(i as isize)).is_in_stack {
            let ref mut fresh136 = *((*ncl).upvalues).as_mut_ptr().offset(i as isize);
            *fresh136 = luaf_findupval(
                state,
                base.offset((*uv.offset(i as isize)).index as i32 as isize),
            );
        } else {
            let ref mut fresh137 = *((*ncl).upvalues).as_mut_ptr().offset(i as isize);
            *fresh137 = *encup.offset((*uv.offset(i as isize)).index as isize);
        }
        if (*ncl).get_marked() & 1 << 5 != 0
            && (**((*ncl).upvalues).as_mut_ptr().offset(i as isize)).get_marked()
                & (1 << 3 | 1 << 4)
                != 0
        {
            luac_barrier_(
                state,
                &mut (*(ncl as *mut GCUnion)).object,
                &mut (*(*((*ncl).upvalues).as_mut_ptr().offset(i as isize) as *mut GCUnion)).object,
            );
        } else {
        };
        i += 1;
    }
}}
pub unsafe extern "C" fn luav_finishop(state: *mut State) { unsafe {
    let call_info: *mut CallInfo = (*state).call_info;
    let base: StkId = ((*call_info).function.p).offset(1 as isize);
    let inst: u32 = *((*call_info).u.l.saved_program_counter).offset(-(1 as isize));
    let op: u32 = (inst >> 0 & !(!(0u32) << 7) << 0) as u32;
    match op as u32 {
        46 | 47 | 48 => {
            let io1: *mut TValue = &mut (*base.offset(
                (*((*call_info).u.l.saved_program_counter).offset(-(2 as isize)) >> 0 + 7
                    & !(!(0u32) << 8) << 0) as i32 as isize,
            ))
            .value;
            (*state).top.p = (*state).top.p.offset(-1);
            let io2: *const TValue = &mut (*(*state).top.p).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag ((*io2).get_tag());
        }
        49 | 50 | 52 | 11 | 12 | 13 | 14 | 20 => {
            let io1_0: *mut TValue = &mut (*base
                .offset((inst >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize))
            .value;
            (*state).top.p = (*state).top.p.offset(-1);
            let io2_0: *const TValue = &mut (*(*state).top.p).value;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
        }
        58 | 59 | 62 | 63 | 64 | 65 | 57 => {
            let res: i32 = !((*(*state).top.p.offset(-(1 as isize))).value.get_tag()                 == TAG_VARIANT_BOOLEAN_FALSE
                || get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag()) == TAG_TYPE_NIL)
                as i32;
            (*state).top.p = (*state).top.p.offset(-1);
                    if res != (inst >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(1);
                (*call_info).u.l.saved_program_counter;
            }
        }
        53 => {
            let top: StkId = (*state).top.p.offset(-(1 as isize));
            let a: i32 = (inst >> 0 + 7 & !(!(0u32) << 8) << 0) as i32;
            let total: i32 =
                top.offset(-(1 as isize))
                    .offset_from(base.offset(a as isize)) as i64 as i32;
            let io1_1: *mut TValue = &mut (*top.offset(-(2 as isize))).value;
            let io2_1: *const TValue = &mut (*top).value;
            (*io1_1).value = (*io2_1).value;
            (*io1_1).set_tag((*io2_1).get_tag());
            (*state).top.p = top.offset(-(1 as isize));
            luav_concat(state, total);
        }
        54 => {
            (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(-1);
            (*call_info).u.l.saved_program_counter;
        }
        70 => {
            let ra: StkId =
                base.offset((inst >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize);
            (*state).top.p = ra.offset((*call_info).u2.nres as isize);
            (*call_info).u.l.saved_program_counter = ((*call_info).u.l.saved_program_counter).offset(-1);
            (*call_info).u.l.saved_program_counter;
        }
        _ => {}
    };
}}
pub unsafe extern "C" fn luav_execute(state: *mut State, mut call_info: *mut CallInfo) { unsafe {
    let mut i: u32;
    let mut ra_65: StkId;
    let mut newci: *mut CallInfo;
    let mut b_4: i32;
    let mut count_results: i32;
    let mut current_block: u64;
    let mut cl: *mut LClosure;
    let mut k: *mut TValue;
    let mut base: StkId;
    let mut program_counter: *const u32;
    let mut trap: i32;
    '_startfunc: loop {
        trap = (*state).hook_mask;
        '_returning: loop {
            cl = &mut (*((*(*call_info).function.p).value.value.object as *mut GCUnion)).lcl;
            k = (*(*cl).p).k;
            program_counter = (*call_info).u.l.saved_program_counter;
            if (trap != 0) as i32 as i64 != 0 {
                trap = luag_tracecall(state);
            }
            base = ((*call_info).function.p).offset(1 as isize);
            loop {
                if (trap != 0) as i32 as i64 != 0 {
                    trap = luag_traceexec(state, program_counter);
                    base = ((*call_info).function.p).offset(1 as isize);
                }
                let fresh138 = program_counter;
                program_counter = program_counter.offset(1);
                i = *fresh138;
                match (i >> 0 & !(!(0u32) << 7) << 0) as u32 as u32 {
                    0 => {
                        let ra: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let io1: *mut TValue = &mut (*ra).value;
                        let io2: *const TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        (*io1).value = (*io2).value;
                        (*io1).set_tag ((*io2).get_tag());
                        continue;
                    }
                    1 => {
                        let ra_0: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let b: i64 = ((i >> 0 + 7 + 8
                            & !(!(0u32) << 8 + 8 + 1) << 0)
                            as i32
                            - ((1 << 8 + 8 + 1) - 1 >> 1))
                            as i64;
                        let io: *mut TValue = &mut (*ra_0).value;
                        (*io).value.i = b;
                        (*io).set_tag (TAG_VARIANT_NUMERIC_INTEGER);
                        continue;
                    }
                    2 => {
                        let ra_1: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let b_0: i32 = (i >> 0 + 7 + 8
                            & !(!(0u32) << 8 + 8 + 1) << 0)
                            as i32
                            - ((1 << 8 + 8 + 1) - 1 >> 1);
                        let io_0: *mut TValue = &mut (*ra_1).value;
                        (*io_0).value.n = b_0 as f64;
                        (*io_0).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        continue;
                    }
                    3 => {
                        let ra_2: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0)
                                as i32 as isize,
                        );
                        let io1_0: *mut TValue = &mut (*ra_2).value;
                        let io2_0: *const TValue = rb;
                        (*io1_0).value = (*io2_0).value;
                        (*io1_0).set_tag((*io2_0).get_tag());
                        continue;
                    }
                    4 => {
                        let ra_3: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_0: *mut TValue = k.offset(
                            (*program_counter >> 0 + 7
                                & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                as i32 as isize,
                        );
                        program_counter = program_counter.offset(1);
                        let io1_1: *mut TValue = &mut (*ra_3).value;
                        let io2_1: *const TValue = rb_0;
                        (*io1_1).value = (*io2_1).value;
                        (*io1_1).set_tag((*io2_1).get_tag());
                        continue;
                    }
                    5 => {
                        let ra_4: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*ra_4).value.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                        continue;
                    }
                    6 => {
                        let ra_5: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*ra_5).value.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                        program_counter = program_counter.offset(1);
                        continue;
                    }
                    7 => {
                        let ra_6: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*ra_6).value.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                        continue;
                    }
                    8 => {
                        let mut ra_7: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let mut b_1: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        loop {
                            let fresh139 = ra_7;
                            ra_7 = ra_7.offset(1);
                            (*fresh139).value.set_tag (TAG_VARIANT_NIL_NIL);
                            let fresh140 = b_1;
                            b_1 = b_1 - 1;
                            if !(fresh140 != 0) {
                                break;
                            }
                        }
                        continue;
                    }
                    9 => {
                        let ra_8: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let b_2: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        let io1_2: *mut TValue = &mut (*ra_8).value;
                        let io2_2: *const TValue =
                            (**((*cl).upvalues).as_mut_ptr().offset(b_2 as isize)).v.p;
                        (*io1_2).value = (*io2_2).value;
                        (*io1_2).set_tag((*io2_2).get_tag());
                        continue;
                    }
                    10 => {
                        let ra_9: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let uv: *mut UpValue = *((*cl).upvalues).as_mut_ptr().offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        );
                        let io1_3: *mut TValue = (*uv).v.p;
                        let io2_3: *const TValue = &mut (*ra_9).value;
                        (*io1_3).value = (*io2_3).value;
                        (*io1_3).set_tag((*io2_3).get_tag());
                        if (*ra_9).value.is_collectable() {
                            if (*uv).get_marked() & 1 << 5 != 0
                                && (*(*ra_9).value.value.object).get_marked()
                                    & (1 << 3 | 1 << 4)
                                    != 0
                            {
                                luac_barrier_(
                                    state,
                                    &mut (*(uv as *mut GCUnion)).object,
                                    &mut (*((*ra_9).value.value.object as *mut GCUnion)).object,
                                );
                            } else {
                            };
                        } else {
                        };
                        continue;
                    }
                    11 => {
                        let ra_10: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot: *const TValue;
                        let count_upvalues: *mut TValue =
                            (**((*cl).upvalues).as_mut_ptr().offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                    as isize,
                            ))
                            .v
                            .p;
                        let rc: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let key: *mut TString = &mut (*((*rc).value.object as *mut GCUnion)).ts;
                        if if !((*count_upvalues).get_tag_variant() == TAG_VARIANT_TABLE)
                        {
                            slot = std::ptr::null();
                            0
                        } else {
                            slot = luah_getshortstr(
                                &mut (*((*count_upvalues).value.object as *mut GCUnion)).h,
                                key,
                            );
                            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_4: *mut TValue = &mut (*ra_10).value;
                            let io2_4: *const TValue = slot;
                            (*io1_4).value = (*io2_4).value;
                            (*io1_4).set_tag((*io2_4).get_tag());
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishget(state, count_upvalues, rc, ra_10, slot);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    12 => {
                        let ra_11: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_0: *const TValue;
                        let rb_1: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let rc_0: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let n: u64;
                        if if (*rc_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            n = (*rc_0).value.i as u64;
                            if !((*rb_1).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_0 = std::ptr::null();
                                0
                            } else {
                                slot_0 = if n.wrapping_sub(1 as u32 as u64)
                                    < (*((*rb_1).value.object as *mut GCUnion)).h.array_limit as u64
                                {
                                    &mut *((*((*rb_1).value.object as *mut GCUnion)).h.array)
                                        .offset(n.wrapping_sub(1 as u64) as isize)
                                        as *mut TValue
                                        as *const TValue
                                } else {
                                    luah_getint(
                                        &mut (*((*rb_1).value.object as *mut GCUnion)).h,
                                        n as i64,
                                    )
                                };
                                !(get_tag_type ((*slot_0).get_tag()) == TAG_TYPE_NIL) as i32
                            }
                        } else if !((*rb_1).get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_0 = std::ptr::null();
                            0
                        } else {
                            slot_0 = luah_get(&mut (*((*rb_1).value.object as *mut GCUnion)).h, rc_0);
                            !(get_tag_type ((*slot_0).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_5: *mut TValue = &mut (*ra_11).value;
                            let io2_5: *const TValue = slot_0;
                            (*io1_5).value = (*io2_5).value;
                            (*io1_5).set_tag((*io2_5).get_tag());
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishget(state, rb_1, rc_0, ra_11, slot_0);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    13 => {
                        let ra_12: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_1: *const TValue;
                        let rb_2: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let c: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32;
                        if if !((*rb_2).get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_1 = std::ptr::null();
                            0
                        } else {
                            slot_1 = if (c as u64).wrapping_sub(1 as u32 as u64)
                                < (*((*rb_2).value.object as *mut GCUnion)).h.array_limit as u64
                            {
                                &mut *((*((*rb_2).value.object as *mut GCUnion)).h.array)
                                    .offset((c - 1) as isize)
                                    as *mut TValue as *const TValue
                            } else {
                                luah_getint(&mut (*((*rb_2).value.object as *mut GCUnion)).h, c as i64)
                            };
                            !(get_tag_type ((*slot_1).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_6: *mut TValue = &mut (*ra_12).value;
                            let io2_6: *const TValue = slot_1;
                            (*io1_6).value = (*io2_6).value;
                            (*io1_6).set_tag((*io2_6).get_tag());
                        } else {
                            let mut key_0: TValue = TValue {
                                value: Value {
                                    object: std::ptr::null_mut(),
                                },
                                tag: 0,
                            };
                            let io_1: *mut TValue = &mut key_0;
                            (*io_1).value.i = c as i64;
                            (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishget(state, rb_2, &mut key_0, ra_12, slot_1);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    14 => {
                        let ra_13: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_2: *const TValue;
                        let rb_3: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let rc_1: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let key_1: *mut TString = &mut (*((*rc_1).value.object as *mut GCUnion)).ts;
                        if if !((*rb_3).get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_2 = std::ptr::null();
                            0
                        } else {
                            slot_2 = luah_getshortstr(
                                &mut (*((*rb_3).value.object as *mut GCUnion)).h,
                                key_1,
                            );
                            !(get_tag_type ((*slot_2).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_7: *mut TValue = &mut (*ra_13).value;
                            let io2_7: *const TValue = slot_2;
                            (*io1_7).value = (*io2_7).value;
                            (*io1_7).set_tag((*io2_7).get_tag());
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishget(state, rb_3, rc_1, ra_13, slot_2);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    15 => {
                        let slot_3: *const TValue;
                        let upval_0: *mut TValue = (**((*cl).upvalues).as_mut_ptr().offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        ))
                        .v
                        .p;
                        let rb_4: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        );
                        let rc_2: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32
                            != 0
                        {
                            k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            )
                        } else {
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            ))
                            .value
                        };
                        let key_2: *mut TString = &mut (*((*rb_4).value.object as *mut GCUnion)).ts;
                        if if !((*upval_0).get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_3 = std::ptr::null();
                            0
                        } else {
                            slot_3 = luah_getshortstr(
                                &mut (*((*upval_0).value.object as *mut GCUnion)).h,
                                key_2,
                            );
                            !(get_tag_type ((*slot_3).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_8: *mut TValue = slot_3 as *mut TValue;
                            let io2_8: *const TValue = rc_2;
                            (*io1_8).value = (*io2_8).value;
                            (*io1_8).set_tag((*io2_8).get_tag());
                            if (*rc_2).is_collectable() {
                                if (*(*upval_0).value.object).get_marked() & 1 << 5 != 0
                                    && (*(*rc_2).value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrierback_(state, (*upval_0).value.object);
                                } else {
                                };
                            } else {
                            };
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishset(state, upval_0, rb_4, rc_2, slot_3);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    16 => {
                        let ra_14: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_4: *const TValue;
                        let rb_5: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let rc_3: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32
                            != 0
                        {
                            k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            )
                        } else {
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            ))
                            .value
                        };
                        let n_0: u64;
                        if if (*rb_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            n_0 = (*rb_5).value.i as u64;
                            if !((*ra_14).value.get_tag_variant() == TAG_VARIANT_TABLE)
                            {
                                slot_4 = std::ptr::null();
                                0
                            } else {
                                slot_4 = if n_0.wrapping_sub(1 as u32 as u64)
                                    < (*((*ra_14).value.value.object as *mut GCUnion)).h.array_limit as u64
                                {
                                    &mut *((*((*ra_14).value.value.object as *mut GCUnion)).h.array)
                                        .offset(n_0.wrapping_sub(1 as u64) as isize)
                                        as *mut TValue
                                        as *const TValue
                                } else {
                                    luah_getint(
                                        &mut (*((*ra_14).value.value.object as *mut GCUnion)).h,
                                        n_0 as i64,
                                    )
                                };
                                (get_tag_type ((*slot_4).get_tag()) != TAG_TYPE_NIL) as i32
                            }
                        } else if !((*ra_14).value.get_tag_variant() == TAG_VARIANT_TABLE)
                        {
                            slot_4 = std::ptr::null();
                            0
                        } else {
                            slot_4 =
                                luah_get(&mut (*((*ra_14).value.value.object as *mut GCUnion)).h, rb_5);
                            !(get_tag_type ((*slot_4).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_9: *mut TValue = slot_4 as *mut TValue;
                            let io2_9: *const TValue = rc_3;
                            (*io1_9).value = (*io2_9).value;
                            (*io1_9).set_tag((*io2_9).get_tag());
                            if (*rc_3).is_collectable() {
                                if (*(*ra_14).value.value.object).get_marked() & 1 << 5 != 0
                                    && (*(*rc_3).value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrierback_(state, (*ra_14).value.value.object);
                                } else {
                                };
                            } else {
                            };
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishset(state, &mut (*ra_14).value, rb_5, rc_3, slot_4);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    17 => {
                        let ra_15: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_5: *const TValue;
                        let c_0: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        let rc_4: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32
                            != 0
                        {
                            k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            )
                        } else {
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            ))
                            .value
                        };
                        if if !((*ra_15).value.get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_5 = std::ptr::null();
                            0
                        } else {
                            slot_5 = if (c_0 as u64).wrapping_sub(1 as u32 as u64)
                                < (*((*ra_15).value.value.object as *mut GCUnion)).h.array_limit as u64
                            {
                                &mut *((*((*ra_15).value.value.object as *mut GCUnion)).h.array)
                                    .offset((c_0 - 1) as isize)
                                    as *mut TValue as *const TValue
                            } else {
                                luah_getint(
                                    &mut (*((*ra_15).value.value.object as *mut GCUnion)).h,
                                    c_0 as i64,
                                )
                            };
                            !(get_tag_type ((*slot_5).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_10: *mut TValue = slot_5 as *mut TValue;
                            let io2_10: *const TValue = rc_4;
                            (*io1_10).value = (*io2_10).value;
                            (*io1_10).set_tag((*io2_10).get_tag());
                            if (*rc_4).is_collectable() {
                                if (*(*ra_15).value.value.object).get_marked() & 1 << 5 != 0
                                    && (*(*rc_4).value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrierback_(state, (*ra_15).value.value.object);
                                } else {
                                };
                            } else {
                            };
                        } else {
                            let mut key_3: TValue = TValue {
                                value: Value {
                                    object: std::ptr::null_mut(),
                                },
                                tag: 0,
                            };
                            let io_2: *mut TValue = &mut key_3;
                            (*io_2).value.i = c_0 as i64;
                            (*io_2).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishset(state, &mut (*ra_15).value, &mut key_3, rc_4, slot_5);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    18 => {
                        let ra_16: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_6: *const TValue;
                        let rb_6: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        );
                        let rc_5: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32
                            != 0
                        {
                            k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            )
                        } else {
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            ))
                            .value
                        };
                        let key_4: *mut TString = &mut (*((*rb_6).value.object as *mut GCUnion)).ts;
                        if if !((*ra_16).value.get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_6 = std::ptr::null();
                            0
                        } else {
                            slot_6 = luah_getshortstr(
                                &mut (*((*ra_16).value.value.object as *mut GCUnion)).h,
                                key_4,
                            );
                            !(get_tag_type ((*slot_6).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_11: *mut TValue = slot_6 as *mut TValue;
                            let io2_11: *const TValue = rc_5;
                            (*io1_11).value = (*io2_11).value;
                            (*io1_11).set_tag((*io2_11).get_tag());
                            if (*rc_5).is_collectable() {
                                if (*(*ra_16).value.value.object).get_marked() & 1 << 5 != 0
                                    && (*(*rc_5).value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrierback_(state, (*ra_16).value.value.object);
                                } else {
                                };
                            } else {
                            };
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishset(state, &mut (*ra_16).value, rb_6, rc_5, slot_6);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    19 => {
                        let ra_17: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let mut b_3: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        let mut c_1: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32;
                        let table: *mut Table;
                        if b_3 > 0 {
                            b_3 = 1 << b_3 - 1;
                        }
                        if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                            c_1 += (*program_counter >> 0 + 7
                                & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                as i32
                                * ((1 << 8) - 1 + 1);
                        }
                        program_counter = program_counter.offset(1);
                        (*state).top.p = ra_17.offset(1 as isize);
                        table = luah_new(state);
                        let io_3: *mut TValue = &mut (*ra_17).value;
                        let x_: *mut Table = table;
                        (*io_3).value.object = &mut (*(x_ as *mut GCUnion)).object;
                        (*io_3).set_tag(TAG_VARIANT_TABLE);
                        (*io_3).set_collectable();
                        if b_3 != 0 || c_1 != 0 {
                            luah_resize(state, table, c_1 as u32, b_3 as u32);
                        }
                        if (*(*state).global).gc_debt > 0 {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = ra_17.offset(1 as isize);
                            luac_step(state);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    20 => {
                        let ra_18: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let slot_7: *const TValue;
                        let rb_7: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let rc_6: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32
                            != 0
                        {
                            k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            )
                        } else {
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                    as i32 as isize,
                            ))
                            .value
                        };
                        let key_5: *mut TString = &mut (*((*rc_6).value.object as *mut GCUnion)).ts;
                        let io1_12: *mut TValue = &mut (*ra_18.offset(1 as isize)).value;
                        let io2_12: *const TValue = rb_7;
                        (*io1_12).value = (*io2_12).value;
                        (*io1_12).set_tag((*io2_12).get_tag());
                        if if !((*rb_7).get_tag_variant() == TAG_VARIANT_TABLE) {
                            slot_7 = std::ptr::null();
                            0
                        } else {
                            slot_7 =
                                luah_getstr(&mut (*((*rb_7).value.object as *mut GCUnion)).h, key_5);
                            !(get_tag_type ((*slot_7).get_tag()) == TAG_TYPE_NIL) as i32
                        } != 0
                        {
                            let io1_13: *mut TValue = &mut (*ra_18).value;
                            let io2_13: *const TValue = slot_7;
                            (*io1_13).value = (*io2_13).value;
                            (*io1_13).set_tag((*io2_13).get_tag());
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_finishget(state, rb_7, rc_6, ra_18, slot_7);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    21 => {
                        let ra_19: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let imm: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32
                            - ((1 << 8) - 1 >> 1);
                        if (*v1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            let iv1: i64 = (*v1).value.i;
                            program_counter = program_counter.offset(1);
                            let io_4: *mut TValue = &mut (*ra_19).value;
                            (*io_4).value.i = (iv1 as u64).wrapping_add(imm as u64) as i64;
                            (*io_4).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else if (*v1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            let nb: f64 = (*v1).value.n;
                            let fimm: f64 = imm as f64;
                            program_counter = program_counter.offset(1);
                            let io_5: *mut TValue = &mut (*ra_19).value;
                            (*io_5).value.n = nb + fimm;
                            (*io_5).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        }
                        continue;
                    }
                    22 => {
                        let v1_0: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let ra_20: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1: i64 = (*v1_0).value.i;
                            let i2: i64 = (*v2).value.i;
                            program_counter = program_counter.offset(1);
                            let io_6: *mut TValue = &mut (*ra_20).value;
                            (*io_6).value.i = (i1 as u64).wrapping_add(i2 as u64) as i64;
                            (*io_6).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1: f64 = 0.0;
                            let mut n2: f64 = 0.0;
                            if (if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1 = (*v1_0).value.n;
                                1
                            } else {
                                if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1 = (*v1_0).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2 = (*v2).value.n;
                                    1
                                } else {
                                    if (*v2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2 = (*v2).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_7: *mut TValue = &mut (*ra_20).value;
                                (*io_7).value.n = n1 + n2;
                                (*io_7).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_SUBK => {
                        let v1_1: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_0: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let ra_21: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_0: i64 = (*v1_1).value.i;
                            let i2_0: i64 = (*v2_0).value.i;
                            program_counter = program_counter.offset(1);
                            let io_8: *mut TValue = &mut (*ra_21).value;
                            (*io_8).value.i = (i1_0 as u64).wrapping_sub(i2_0 as u64) as i64;
                            (*io_8).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_0: f64 = 0.0;
                            let mut n2_0: f64 = 0.0;
                            if (if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_0 = (*v1_1).value.n;
                                1
                            } else {
                                if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_0 = (*v1_1).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_0 = (*v2_0).value.n;
                                    1
                                } else {
                                    if (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_0 = (*v2_0).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_9: *mut TValue = &mut (*ra_21).value;
                                (*io_9).value.n = n1_0 - n2_0;
                                (*io_9).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_MULK => {
                        let v1_2: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_1: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let ra_22: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_1: i64 = (*v1_2).value.i;
                            let i2_1: i64 = (*v2_1).value.i;
                            program_counter = program_counter.offset(1);
                            let io_10: *mut TValue = &mut (*ra_22).value;
                            (*io_10).value.i = (i1_1 as u64).wrapping_mul(i2_1 as u64) as i64;
                            (*io_10).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_1: f64 = 0.0;
                            let mut n2_1: f64 = 0.0;
                            if (if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_1 = (*v1_2).value.n;
                                1
                            } else {
                                if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_1 = (*v1_2).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_1 = (*v2_1).value.n;
                                    1
                                } else {
                                    if (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_1 = (*v2_1).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_11: *mut TValue = &mut (*ra_22).value;
                                (*io_11).value.n = n1_1 * n2_1;
                                (*io_11).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_MODK => {
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        let v1_3: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_2: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let ra_23: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_2: i64 = (*v1_3).value.i;
                            let i2_2: i64 = (*v2_2).value.i;
                            program_counter = program_counter.offset(1);
                            let io_12: *mut TValue = &mut (*ra_23).value;
                            (*io_12).value.i = luav_mod(state, i1_2, i2_2);
                            (*io_12).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_2: f64 = 0.0;
                            let mut n2_2: f64 = 0.0;
                            if (if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_2 = (*v1_3).value.n;
                                1
                            } else {
                                if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_2 = (*v1_3).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_2 = (*v2_2).value.n;
                                    1
                                } else {
                                    if (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_2 = (*v2_2).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_13: *mut TValue = &mut (*ra_23).value;
                                (*io_13).value.n = luav_modf(state, n1_2, n2_2);
                                (*io_13).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_POWK => {
                        let ra_24: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_4: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_3: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let mut n1_3: f64 = 0.0;
                        let mut n2_3: f64 = 0.0;
                        if (if (*v1_4).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            n1_3 = (*v1_4).value.n;
                            1
                        } else {
                            if (*v1_4).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n1_3 = (*v1_4).value.i as f64;
                                1
                            } else {
                                0
                            }
                        }) != 0
                            && (if (*v2_3).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n2_3 = (*v2_3).value.n;
                                1
                            } else {
                                if (*v2_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n2_3 = (*v2_3).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_14: *mut TValue = &mut (*ra_24).value;
                            (*io_14).value.n = if n2_3 == 2.0 {
                                n1_3 * n1_3
                            } else {
                                n1_3.powf(n2_3)
                            };
                            (*io_14).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        }
                        continue;
                    }
                    OP_DIVK => {
                        let ra_25: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_5: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_4: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let mut n1_4: f64 = 0.0;
                        let mut n2_4: f64 = 0.0;
                        if (if (*v1_5).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            n1_4 = (*v1_5).value.n;
                            1
                        } else {
                            if (*v1_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n1_4 = (*v1_5).value.i as f64;
                                1
                            } else {
                                0
                            }
                        }) != 0
                            && (if (*v2_4).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n2_4 = (*v2_4).value.n;
                                1
                            } else {
                                if (*v2_4).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n2_4 = (*v2_4).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_15: *mut TValue = &mut (*ra_25).value;
                            (*io_15).value.n = n1_4 / n2_4;
                            (*io_15).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        }
                        continue;
                    }
                    OP_IDIVK => {
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        let v1_6: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_5: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let ra_26: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_3: i64 = (*v1_6).value.i;
                            let i2_3: i64 = (*v2_5).value.i;
                            program_counter = program_counter.offset(1);
                            let io_16: *mut TValue = &mut (*ra_26).value;
                            (*io_16).value.i = luav_idiv(state, i1_3, i2_3);
                            (*io_16).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_5: f64 = 0.0;
                            let mut n2_5: f64 = 0.0;
                            if (if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_5 = (*v1_6).value.n;
                                1
                            } else {
                                if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_5 = (*v1_6).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_5 = (*v2_5).value.n;
                                    1
                                } else {
                                    if (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_5 = (*v2_5).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_17: *mut TValue = &mut (*ra_26).value;
                                (*io_17).value.n = (n1_5 / n2_5).floor();
                                (*io_17).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_BANDK => {
                        let ra_27: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_7: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_6: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let mut i1_4: i64 = 0;
                        let i2_4: i64 = (*v2_6).value.i;
                        if if (((*v1_7).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            i1_4 = (*v1_7).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_7, &mut i1_4, F2I::Equal)
                        } != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_18: *mut TValue = &mut (*ra_27).value;
                            (*io_18).value.i = (i1_4 as u64 & i2_4 as u64) as i64;
                            (*io_18).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_BORK => {
                        let ra_28: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_8: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_7: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let mut i1_5: i64 = 0;
                        let i2_5: i64 = (*v2_7).value.i;
                        if if (((*v1_8).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            i1_5 = (*v1_8).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_8, &mut i1_5, F2I::Equal)
                        } != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_19: *mut TValue = &mut (*ra_28).value;
                            (*io_19).value.i = (i1_5 as u64 | i2_5 as u64) as i64;
                            (*io_19).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_BXORK => {
                        let ra_29: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_9: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_8: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        );
                        let mut i1_6: i64 = 0;
                        let i2_6: i64 = (*v2_8).value.i;
                        if if (((*v1_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            i1_6 = (*v1_9).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_9, &mut i1_6, F2I::Equal)
                        } != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_20: *mut TValue = &mut (*ra_29).value;
                            (*io_20).value.i = (i1_6 as u64 ^ i2_6 as u64) as i64;
                            (*io_20).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    32 => {
                        let ra_30: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_8: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let ic: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32
                            - ((1 << 8) - 1 >> 1);
                        let mut ib: i64 = 0;
                        if if (((*rb_8).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            ib = (*rb_8).value.i;
                            1
                        } else {
                            luav_tointegerns(rb_8, &mut ib, F2I::Equal)
                        } != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_21: *mut TValue = &mut (*ra_30).value;
                            (*io_21).value.i = luav_shiftl(ib, -ic as i64);
                            (*io_21).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    33 => {
                        let ra_31: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_9: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let ic_0: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32
                            - ((1 << 8) - 1 >> 1);
                        let mut ib_0: i64 = 0;
                        if if (((*rb_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            ib_0 = (*rb_9).value.i;
                            1
                        } else {
                            luav_tointegerns(rb_9, &mut ib_0, F2I::Equal)
                        } != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_22: *mut TValue = &mut (*ra_31).value;
                            (*io_22).value.i = luav_shiftl(ic_0 as i64, ib_0);
                            (*io_22).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    34 => {
                        let v1_10: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_9: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let ra_32: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_7: i64 = (*v1_10).value.i;
                            let i2_7: i64 = (*v2_9).value.i;
                            program_counter = program_counter.offset(1);
                            let io_23: *mut TValue = &mut (*ra_32).value;
                            (*io_23).value.i = (i1_7 as u64).wrapping_add(i2_7 as u64) as i64;
                            (*io_23).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_6: f64 = 0.0;
                            let mut n2_6: f64 = 0.0;
                            if (if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_6 = (*v1_10).value.n;
                                1
                            } else {
                                if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_6 = (*v1_10).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_6 = (*v2_9).value.n;
                                    1
                                } else {
                                    if (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_6 = (*v2_9).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_24: *mut TValue = &mut (*ra_32).value;
                                (*io_24).value.n = n1_6 + n2_6;
                                (*io_24).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_SUB => {
                        let v1_11: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_10: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let ra_33: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_8: i64 = (*v1_11).value.i;
                            let i2_8: i64 = (*v2_10).value.i;
                            program_counter = program_counter.offset(1);
                            let io_25: *mut TValue = &mut (*ra_33).value;
                            (*io_25).value.i = (i1_8 as u64).wrapping_sub(i2_8 as u64) as i64;
                            (*io_25).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_7: f64 = 0.0;
                            let mut n2_7: f64 = 0.0;
                            if (if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_7 = (*v1_11).value.n;
                                1
                            } else {
                                if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_7 = (*v1_11).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_7 = (*v2_10).value.n;
                                    1
                                } else {
                                    if (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_7 = (*v2_10).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_26: *mut TValue = &mut (*ra_33).value;
                                (*io_26).value.n = n1_7 - n2_7;
                                (*io_26).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_MUL => {
                        let v1_12: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_11: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let ra_34: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_9: i64 = (*v1_12).value.i;
                            let i2_9: i64 = (*v2_11).value.i;
                            program_counter = program_counter.offset(1);
                            let io_27: *mut TValue = &mut (*ra_34).value;
                            (*io_27).value.i = (i1_9 as u64).wrapping_mul(i2_9 as u64) as i64;
                            (*io_27).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_8: f64 = 0.0;
                            let mut n2_8: f64 = 0.0;
                            if (if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_8 = (*v1_12).value.n;
                                1
                            } else {
                                if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_8 = (*v1_12).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_8 = (*v2_11).value.n;
                                    1
                                } else {
                                    if (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_8 = (*v2_11).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_28: *mut TValue = &mut (*ra_34).value;
                                (*io_28).value.n = n1_8 * n2_8;
                                (*io_28).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_MOD => {
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        let v1_13: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_12: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let ra_35: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_10: i64 = (*v1_13).value.i;
                            let i2_10: i64 = (*v2_12).value.i;
                            program_counter = program_counter.offset(1);
                            let io_29: *mut TValue = &mut (*ra_35).value;
                            (*io_29).value.i = luav_mod(state, i1_10, i2_10);
                            (*io_29).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_9: f64 = 0.0;
                            let mut n2_9: f64 = 0.0;
                            if (if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_9 = (*v1_13).value.n;
                                1
                            } else {
                                if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_9 = (*v1_13).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_9 = (*v2_12).value.n;
                                    1
                                } else {
                                    if (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_9 = (*v2_12).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_30: *mut TValue = &mut (*ra_35).value;
                                (*io_30).value.n = luav_modf(state, n1_9, n2_9);
                                (*io_30).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_POW => {
                        let ra_36: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_14: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_13: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut n1_10: f64 = 0.0;
                        let mut n2_10: f64 = 0.0;
                        if (if (*v1_14).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            n1_10 = (*v1_14).value.n;
                            1
                        } else {
                            if (*v1_14).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n1_10 = (*v1_14).value.i as f64;
                                1
                            } else {
                                0
                            }
                        }) != 0
                            && (if (*v2_13).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n2_10 = (*v2_13).value.n;
                                1
                            } else {
                                if (*v2_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n2_10 = (*v2_13).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_31: *mut TValue = &mut (*ra_36).value;
                            (*io_31).value.n = if n2_10 == 2.0 {
                                n1_10 * n1_10
                            } else {
                                n1_10.powf(n2_10)
                            };
                            (*io_31).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        }
                        continue;
                    }
                    OP_DIV => {
                        let ra_37: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_15: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_14: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut n1_11: f64 = 0.0;
                        let mut n2_11: f64 = 0.0;
                        if (if (*v1_15).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            n1_11 = (*v1_15).value.n;
                            1
                        } else {
                            if (*v1_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n1_11 = (*v1_15).value.i as f64;
                                1
                            } else {
                                0
                            }
                        }) != 0
                            && (if (*v2_14).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n2_11 = (*v2_14).value.n;
                                1
                            } else {
                                if (*v2_14).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n2_11 = (*v2_14).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_32: *mut TValue = &mut (*ra_37).value;
                            (*io_32).value.n = n1_11 / n2_11;
                            (*io_32).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        }
                        continue;
                    }
                    OP_IDIV => {
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        let v1_16: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_15: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let ra_38: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let i1_11: i64 = (*v1_16).value.i;
                            let i2_11: i64 = (*v2_15).value.i;
                            program_counter = program_counter.offset(1);
                            let io_33: *mut TValue = &mut (*ra_38).value;
                            (*io_33).value.i = luav_idiv(state, i1_11, i2_11);
                            (*io_33).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            let mut n1_12: f64 = 0.0;
                            let mut n2_12: f64 = 0.0;
                            if (if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_12 = (*v1_16).value.n;
                                1
                            } else {
                                if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_12 = (*v1_16).value.i as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_12 = (*v2_15).value.n;
                                    1
                                } else {
                                    if (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_12 = (*v2_15).value.i as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_34: *mut TValue = &mut (*ra_38).value;
                                (*io_34).value.n = (n1_12 / n2_12).floor();
                                (*io_34).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                        }
                        continue;
                    }
                    OP_BAND => {
                        let ra_39: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_17: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_16: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut i1_12: i64 = 0;
                        let mut i2_12: i64 = 0;
                        if (if (((*v1_17).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                            as i32 as i64
                            != 0
                        {
                            i1_12 = (*v1_17).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_17, &mut i1_12, F2I::Equal)
                        }) != 0
                            && (if (((*v2_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i32 as i64
                                != 0
                            {
                                i2_12 = (*v2_16).value.i;
                                1
                            } else {
                                luav_tointegerns(v2_16, &mut i2_12, F2I::Equal)
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_35: *mut TValue = &mut (*ra_39).value;
                            (*io_35).value.i = (i1_12 as u64 & i2_12 as u64) as i64;
                            (*io_35).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_BOR => {
                        let ra_40: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_18: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_17: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut i1_13: i64 = 0;
                        let mut i2_13: i64 = 0;
                        if (if (((*v1_18).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                            as i32 as i64
                            != 0
                        {
                            i1_13 = (*v1_18).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_18, &mut i1_13, F2I::Equal)
                        }) != 0
                            && (if (((*v2_17).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i32 as i64
                                != 0
                            {
                                i2_13 = (*v2_17).value.i;
                                1
                            } else {
                                luav_tointegerns(v2_17, &mut i2_13, F2I::Equal)
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_36: *mut TValue = &mut (*ra_40).value;
                            (*io_36).value.i = (i1_13 as u64 | i2_13 as u64) as i64;
                            (*io_36).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_BXOR => {
                        let ra_41: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_19: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_18: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut i1_14: i64 = 0;
                        let mut i2_14: i64 = 0;
                        if (if (((*v1_19).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                            as i32 as i64
                            != 0
                        {
                            i1_14 = (*v1_19).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_19, &mut i1_14, F2I::Equal)
                        }) != 0
                            && (if (((*v2_18).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i32 as i64
                                != 0
                            {
                                i2_14 = (*v2_18).value.i;
                                1
                            } else {
                                luav_tointegerns(v2_18, &mut i2_14, F2I::Equal)
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_37: *mut TValue = &mut (*ra_41).value;
                            (*io_37).value.i = (i1_14 as u64 ^ i2_14 as u64) as i64;
                            (*io_37).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_SHR => {
                        let ra_42: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_20: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_19: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut i1_15: i64 = 0;
                        let mut i2_15: i64 = 0;
                        if (if (((*v1_20).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                            as i32 as i64
                            != 0
                        {
                            i1_15 = (*v1_20).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_20, &mut i1_15, F2I::Equal)
                        }) != 0
                            && (if (((*v2_19).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i32 as i64
                                != 0
                            {
                                i2_15 = (*v2_19).value.i;
                                1
                            } else {
                                luav_tointegerns(v2_19, &mut i2_15, F2I::Equal)
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_38: *mut TValue = &mut (*ra_42).value;
                            (*io_38).value.i =
                                luav_shiftl(i1_15, (0u64).wrapping_sub(i2_15 as u64) as i64);
                            (*io_38).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    OP_SHL => {
                        let ra_43: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let v1_21: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let v2_20: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32 as isize,
                        ))
                        .value;
                        let mut i1_16: i64 = 0;
                        let mut i2_16: i64 = 0;
                        if (if (((*v1_21).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                            as i32 as i64
                            != 0
                        {
                            i1_16 = (*v1_21).value.i;
                            1
                        } else {
                            luav_tointegerns(v1_21, &mut i1_16, F2I::Equal)
                        }) != 0
                            && (if (((*v2_20).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i32 as i64
                                != 0
                            {
                                i2_16 = (*v2_20).value.i;
                                1
                            } else {
                                luav_tointegerns(v2_20, &mut i2_16, F2I::Equal)
                            }) != 0
                        {
                            program_counter = program_counter.offset(1);
                            let io_39: *mut TValue = &mut (*ra_43).value;
                            (*io_39).value.i = luav_shiftl(i1_16, i2_16);
                            (*io_39).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        }
                        continue;
                    }
                    46 => {
                        let ra_44: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let pi: u32 = *program_counter.offset(-(2 as isize));
                        let rb_10: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let tm: u32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32 as u32;
                        let result: StkId = base.offset(
                            (pi >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luat_trybintm(state, &mut (*ra_44).value, rb_10, result, tm);
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    47 => {
                        let ra_45: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let pi_0: u32 = *program_counter.offset(-(2 as isize));
                        let imm_0: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        let tm_0: u32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32 as u32;
                        let flip: i32 =
                            (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32;
                        let result_0: StkId = base.offset(
                            (pi_0 >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luat_trybinitm(
                            state,
                            &mut (*ra_45).value,
                            imm_0 as i64,
                            flip,
                            result_0,
                            tm_0,
                        );
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    48 => {
                        let ra_46: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let pi_1: u32 = *program_counter.offset(-(2 as isize));
                        let imm_1: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        );
                        let tm_1: u32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32 as u32;
                        let flip_0: i32 =
                            (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32;
                        let result_1: StkId = base.offset(
                            (pi_1 >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luat_trybinassoctm(state, &mut (*ra_46).value, imm_1, flip_0, result_1, tm_1);
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    49 => {
                        let ra_47: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_11: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let mut nb_0: f64 = 0.0;
                        if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            let ib_1: i64 = (*rb_11).value.i;
                            let io_40: *mut TValue = &mut (*ra_47).value;
                            (*io_40).value.i = (0u64).wrapping_sub(ib_1 as u64) as i64;
                            (*io_40).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else if if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            nb_0 = (*rb_11).value.n;
                            1
                        } else if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            nb_0 = (*rb_11).value.i as f64;
                            1
                        } else {
                            0
                        } != 0
                        {
                            let io_41: *mut TValue = &mut (*ra_47).value;
                            (*io_41).value.n = -nb_0;
                            (*io_41).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_trybintm(state, rb_11, rb_11, ra_47, TM_UNM);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    50 => {
                        let ra_48: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_12: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        let mut ib_2: i64 = 0;
                        if if (((*rb_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                            as i64
                            != 0
                        {
                            ib_2 = (*rb_12).value.i;
                            1
                        } else {
                            luav_tointegerns(rb_12, &mut ib_2, F2I::Equal)
                        } != 0
                        {
                            let io_42: *mut TValue = &mut (*ra_48).value;
                            (*io_42).value.i = (!(0u64) ^ ib_2 as u64) as i64;
                            (*io_42).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_trybintm(state, rb_12, rb_12, ra_48, TM_BNOT);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    51 => {
                        let ra_49: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_13: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        if (*rb_13).get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                            || get_tag_type ((*rb_13).get_tag()) == TAG_TYPE_NIL
                        {
                            (*ra_49).value.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                        } else {
                            (*ra_49).value.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                        }
                        continue;
                    }
                    OP_LEN => {
                        let ra_50: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luav_objlen(
                            state,
                            ra_50,
                            &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                    as isize,
                            ))
                            .value,
                        );
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    53 => {
                        let ra_51: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let n_1: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        (*state).top.p = ra_51.offset(n_1 as isize);
                        (*call_info).u.l.saved_program_counter = program_counter;
                        luav_concat(state, n_1);
                        trap = (*call_info).u.l.trap;
                        if (*(*state).global).gc_debt > 0 {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*state).top.p;
                            luac_step(state);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    54 => {
                        let ra_52: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luaf_close(state, ra_52, 0, 1);
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    55 => {
                        let ra_53: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luaf_newtbcupval(state, ra_53);
                        continue;
                    }
                    56 => {
                        program_counter = program_counter.offset(
                            ((i >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                as i32
                                - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                + 0) as isize,
                        );
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    57 => {
                        let ra_54: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_0: i32;
                        let rb_14: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        cond_0 = luav_equalobj(state, &mut (*ra_54).value, rb_14);
                        trap = (*call_info).u.l.trap;
                        if cond_0 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    58 => {
                        let ra_55: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_1: i32;
                        let rb_15: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        if (*ra_55).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*rb_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let ia: i64 = (*ra_55).value.value.i;
                            let ib_3: i64 = (*rb_15).value.i;
                            cond_1 = (ia < ib_3) as i32;
                        } else if get_tag_type ((*ra_55).value.get_tag()) == TAG_TYPE_NUMERIC
                            && get_tag_type ((*rb_15).get_tag()) == TAG_TYPE_NUMERIC
                        {
                            cond_1 = ltnum(&mut (*ra_55).value, rb_15);
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_1 = lessthanothers(state, &mut (*ra_55).value, rb_15);
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_1 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_0: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_0 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    OP_LE => {
                        let ra_56: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_2: i32;
                        let rb_16: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        if (*ra_56).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            && (*rb_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                        {
                            let ia_0: i64 = (*ra_56).value.value.i;
                            let ib_4: i64 = (*rb_16).value.i;
                            cond_2 = (ia_0 <= ib_4) as i32;
                        } else if get_tag_type((*ra_56).value.get_tag()) == TAG_TYPE_NUMERIC
                            && get_tag_type((*rb_16).get_tag()) == TAG_TYPE_NUMERIC
                        {
                            cond_2 = lenum(&mut (*ra_56).value, rb_16);
                        } else {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_2 = lessequalothers(state, &mut (*ra_56).value, rb_16);
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_2 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_1: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_1 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    60 => {
                        let ra_57: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_17: *mut TValue = k.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        );
                        let cond_3: i32 =
                            luav_equalobj(std::ptr::null_mut(), &mut (*ra_57).value, rb_17);
                        if cond_3 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_2: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_2 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    61 => {
                        let ra_58: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_4: i32;
                        let im: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        if (*ra_58).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            cond_4 = ((*ra_58).value.value.i == im as i64) as i32;
                        } else if (*ra_58).value.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            cond_4 = ((*ra_58).value.value.n == im as f64) as i32;
                        } else {
                            cond_4 = 0;
                        }
                        if cond_4 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_3: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_3 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    62 => {
                        let ra_59: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_5: i32;
                        let im_0: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        if (*ra_59).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            cond_5 = ((*ra_59).value.value.i < im_0 as i64) as i32;
                        } else if (*ra_59).value.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            let fa: f64 = (*ra_59).value.value.n;
                            let fim: f64 = im_0 as f64;
                            cond_5 = (fa < fim) as i32;
                        } else {
                            let isf: bool = (i >> 0 + 7 + 8 + 1 + 8
                                & !(!(0u32) << 8) << 0) != 0;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_5 =
                                luat_callorderitm(state, &mut (*ra_59).value, im_0, 0, isf, TM_LT);
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_5 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_4: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_4 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    OP_LEI => {
                        let ra_60: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_6: i32;
                        let im_1: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        if (*ra_60).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            cond_6 = ((*ra_60).value.value.i <= im_1 as i64) as i32;
                        } else if (*ra_60).value.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            let fa_0: f64 = (*ra_60).value.value.n;
                            let fim_0: f64 = im_1 as f64;
                            cond_6 = (fa_0 <= fim_0) as i32;
                        } else {
                            let isf_0: bool = (i >> 0 + 7 + 8 + 1 + 8
                                & !(!(0u32) << 8) << 0) != 0;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_6 =
                                luat_callorderitm(state, &mut (*ra_60).value, im_1, 0, isf_0, TM_LE);
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_6 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_5: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_5 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    64 => {
                        let ra_61: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_7: i32;
                        let im_2: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        if (*ra_61).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            cond_7 = ((*ra_61).value.value.i > im_2 as i64) as i32;
                        } else if (*ra_61).value.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            let fa_1: f64 = (*ra_61).value.value.n;
                            let fim_1: f64 = im_2 as f64;
                            cond_7 = (fa_1 > fim_1) as i32;
                        } else {
                            let isf_1: bool = (i >> 0 + 7 + 8 + 1 + 8
                                & !(!(0u32) << 8) << 0) != 0;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_7 = luat_callorderitm(
                                state,
                                &mut (*ra_61).value,
                                im_2,
                                1,
                                isf_1,
                                TM_LT,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_7 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_6: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_6 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    OP_GEI => {
                        let ra_62: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_8: i32;
                        let im_3: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                        if (*ra_62).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            cond_8 = ((*ra_62).value.value.i >= im_3 as i64) as i32;
                        } else if (*ra_62).value.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                            let fa_2: f64 = (*ra_62).value.value.n;
                            let fim_2: f64 = im_3 as f64;
                            cond_8 = (fa_2 >= fim_2) as i32;
                        } else {
                            let isf_2: bool = (i >> 0 + 7 + 8 + 1 + 8
                                & !(!(0u32) << 8) << 0) != 0;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_8 = luat_callorderitm(
                                state,
                                &mut (*ra_62).value,
                                im_3,
                                1,
                                isf_2,
                                TM_LE,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        if cond_8 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_7: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_7 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    66 => {
                        let ra_63: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let cond_9: i32 = !((*ra_63).value.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                            || get_tag_type((*ra_63).value.get_tag()) == TAG_TYPE_NIL)
                            as i32;
                        if cond_9 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                            program_counter = program_counter.offset(1);
                        } else {
                            let ni_8: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_8 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    67 => {
                        let ra_64: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let rb_18: *mut TValue = &mut (*base.offset(
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                as isize,
                        ))
                        .value;
                        if ((*rb_18).get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                            || get_tag_type ((*rb_18).get_tag()) == TAG_TYPE_NIL)
                            as i32
                            == (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32
                        {
                            program_counter = program_counter.offset(1);
                        } else {
                            let io1_14: *mut TValue = &mut (*ra_64).value;
                            let io2_14: *const TValue = rb_18;
                            (*io1_14).value = (*io2_14).value;
                            (*io1_14).set_tag((*io2_14).get_tag());
                            let ni_9: u32 = *program_counter;
                            program_counter = program_counter.offset(
                                ((ni_9 >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 1) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    68 => {
                        ra_65 = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        b_4 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        count_results = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32
                            - 1;
                        if b_4 != 0 {
                            (*state).top.p = ra_65.offset(b_4 as isize);
                        }
                        (*call_info).u.l.saved_program_counter = program_counter;
                        newci = luad_precall(state, ra_65, count_results);
                        if !newci.is_null() {
                            break '_returning;
                        }
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    69 => {
                        let ra_66: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let mut b_5: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        let n_2: i32;
                        let nparams1: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32;
                        let delta: i32 = if nparams1 != 0 {
                            (*call_info).u.l.count_extra_arguments + nparams1
                        } else {
                            0
                        };
                        if b_5 != 0 {
                            (*state).top.p = ra_66.offset(b_5 as isize);
                        } else {
                            b_5 = ((*state).top.p).offset_from(ra_66) as i64 as i32;
                        }
                        (*call_info).u.l.saved_program_counter = program_counter;
                        if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                            luaf_closeupval(state, base);
                        }
                        n_2 = luad_pretailcall(state, call_info, ra_66, b_5, delta);
                        if n_2 < 0 {
                            continue '_startfunc;
                        }
                        (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
                        luad_poscall(state, call_info, n_2);
                        trap = (*call_info).u.l.trap;
                        break;
                    }
                    70 => {
                        let mut ra_67: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let mut n_3: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - 1;
                        let nparams1_0: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32;
                        if n_3 < 0 {
                            n_3 = ((*state).top.p).offset_from(ra_67) as i64 as i32;
                        }
                        (*call_info).u.l.saved_program_counter = program_counter;
                        if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                            (*call_info).u2.nres = n_3;
                            if (*state).top.p < (*call_info).top.p {
                                (*state).top.p = (*call_info).top.p;
                            }
                            luaf_close(state, base, -1, 1);
                            trap = (*call_info).u.l.trap;
                            if (trap != 0) as i32 as i64 != 0 {
                                base = ((*call_info).function.p).offset(1 as isize);
                                ra_67 = base.offset(
                                    (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                                );
                            }
                        }
                        if nparams1_0 != 0 {
                            (*call_info).function.p = ((*call_info).function.p)
                                .offset(-(((*call_info).u.l.count_extra_arguments + nparams1_0) as isize));
                        }
                        (*state).top.p = ra_67.offset(n_3 as isize);
                        luad_poscall(state, call_info, n_3);
                        trap = (*call_info).u.l.trap;
                        break;
                    }
                    71 => {
                        if ((*state).hook_mask != 0) as i32 as i64 != 0 {
                            let ra_68: StkId = base.offset(
                                (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                            );
                            (*state).top.p = ra_68;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            luad_poscall(state, call_info, 0);
                            trap = 1;
                        } else {
                            let mut nres: i32;
                            (*state).call_info = (*call_info).previous;
                            (*state).top.p = base.offset(-(1 as isize));
                            nres = (*call_info).count_results as i32;
                            while ((nres > 0) as i32 != 0) as i32 as i64 != 0 {
                                let fresh141 = (*state).top.p;
                                (*state).top.p = (*state).top.p.offset(1);
                                (*fresh141).value.set_tag (TAG_VARIANT_NIL_NIL);
                                nres -= 1;
                            }
                        }
                        break;
                    }
                    72 => {
                        if ((*state).hook_mask != 0) as i32 as i64 != 0 {
                            let ra_69: StkId = base.offset(
                                (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                            );
                            (*state).top.p = ra_69.offset(1 as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            luad_poscall(state, call_info, 1);
                            trap = 1;
                        } else {
                            let mut nres_0: i32 = (*call_info).count_results as i32;
                            (*state).call_info = (*call_info).previous;
                            if nres_0 == 0 {
                                (*state).top.p = base.offset(-(1 as isize));
                            } else {
                                let ra_70: StkId = base.offset(
                                    (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                                );
                                let io1_15: *mut TValue =
                                    &mut (*base.offset(-(1 as isize))).value;
                                let io2_15: *const TValue = &mut (*ra_70).value;
                                (*io1_15).value = (*io2_15).value;
                                (*io1_15).set_tag((*io2_15).get_tag());
                                (*state).top.p = base;
                                while ((nres_0 > 1) as i32 != 0) as i32 as i64 != 0 {
                                    let fresh142 = (*state).top.p;
                                    (*state).top.p = (*state).top.p.offset(1);
                                    (*fresh142).value.set_tag (TAG_VARIANT_NIL_NIL);
                                    nres_0 -= 1;
                                }
                            }
                        }
                        break;
                    }
                    73 => {
                        let ra_71: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        if (*ra_71.offset(2 as isize)).value.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            let count: u64 = (*ra_71.offset(1 as isize)).value.value.i as u64;
                            if count > 0u64 {
                                let step: i64 = (*ra_71.offset(2 as isize)).value.value.i;
                                let mut index: i64 = (*ra_71).value.value.i;
                                let io_43: *mut TValue =
                                    &mut (*ra_71.offset(1 as isize)).value;
                                (*io_43).value.i = count.wrapping_sub(1 as u64) as i64;
                                index = (index as u64).wrapping_add(step as u64) as i64;
                                let io_44: *mut TValue = &mut (*ra_71).value;
                                (*io_44).value.i = index;
                                let io_45: *mut TValue =
                                    &mut (*ra_71.offset(3 as isize)).value;
                                (*io_45).value.i = index;
                                (*io_45).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                                program_counter = program_counter.offset(
                                    -((i >> 0 + 7 + 8
                                        & !(!(0u32) << 8 + 8 + 1) << 0)
                                        as i32 as isize),
                                );
                            }
                        } else if floatforloop(ra_71) != 0 {
                            program_counter = program_counter.offset(
                                -((i >> 0 + 7 + 8
                                    & !(!(0u32) << 8 + 8 + 1) << 0)
                                    as i32 as isize),
                            );
                        }
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    74 => {
                        let ra_72: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        if forprep(state, ra_72) != 0 {
                            program_counter = program_counter.offset(
                                ((i >> 0 + 7 + 8
                                    & !(!(0u32) << 8 + 8 + 1) << 0)
                                    as i32
                                    + 1) as isize,
                            );
                        }
                        continue;
                    }
                    75 => {
                        let ra_73: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luaf_newtbcupval(state, ra_73.offset(3 as isize));
                        program_counter = program_counter.offset(
                            (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0)
                                as i32 as isize,
                        );
                        let fresh143 = program_counter;
                        program_counter = program_counter.offset(1);
                        i = *fresh143;
                        current_block = 13973394567113199817;
                    }
                    76 => {
                        current_block = 13973394567113199817;
                    }
                    77 => {
                        current_block = 15611964311717037170;
                    }
                    78 => {
                        let ra_76: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let mut n_4: i32 =
                            (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                        let mut last: u32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32 as u32;
                        let h: *mut Table = &mut (*((*ra_76).value.value.object as *mut GCUnion)).h;
                        if n_4 == 0 {
                            n_4 = ((*state).top.p).offset_from(ra_76) as i64 as i32 - 1;
                        } else {
                            (*state).top.p = (*call_info).top.p;
                        }
                        last = last.wrapping_add(n_4 as u32);
                        if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                            last = last.wrapping_add(
                                ((*program_counter >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    * ((1 << 8) - 1 + 1))
                                    as u32,
                            );
                            program_counter = program_counter.offset(1);
                        }
                        if last > luah_realasize(h) {
                            luah_resizearray(state, h, last);
                        }
                        while n_4 > 0 {
                            let value: *mut TValue = &mut (*ra_76.offset(n_4 as isize)).value;
                            let io1_17: *mut TValue = &mut *((*h).array)
                                .offset(last.wrapping_sub(1 as u32) as isize)
                                as *mut TValue;
                            let io2_17: *const TValue = value;
                            (*io1_17).value = (*io2_17).value;
                            (*io1_17).set_tag((*io2_17).get_tag());
                            last = last.wrapping_sub(1);
                            if (*value).is_collectable() {
                                if (*(h as *mut GCUnion)).object.get_marked() & 1 << 5 != 0
                                    && (*(*value).value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrierback_(state, &mut (*(h as *mut GCUnion)).object);
                                } else {
                                };
                            } else {
                            };
                            n_4 -= 1;
                        }
                        continue;
                    }
                    79 => {
                        let ra_77: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let p: *mut Prototype = *((*(*cl).p).p).offset(
                            (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0)
                                as i32 as isize,
                        );
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        pushclosure(state, p, ((*cl).upvalues).as_mut_ptr(), base, ra_77);
                        if (*(*state).global).gc_debt > 0 {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = ra_77.offset(1 as isize);
                            luac_step(state);
                            trap = (*call_info).u.l.trap;
                        }
                        continue;
                    }
                    80 => {
                        let ra_78: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        let n_5: i32 = (i >> 0 + 7 + 8 + 1 + 8
                            & !(!(0u32) << 8) << 0)
                            as i32
                            - 1;
                        (*call_info).u.l.saved_program_counter = program_counter;
                        (*state).top.p = (*call_info).top.p;
                        luat_getvarargs(state, call_info, ra_78, n_5);
                        trap = (*call_info).u.l.trap;
                        continue;
                    }
                    81 => {
                        (*call_info).u.l.saved_program_counter = program_counter;
                        luat_adjustvarargs(
                            state,
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32,
                            call_info,
                            (*cl).p,
                        );
                        trap = (*call_info).u.l.trap;
                        if (trap != 0) as i32 as i64 != 0 {
                            luad_hookcall(state, call_info);
                            (*state).old_program_counter = 1;
                        }
                        base = ((*call_info).function.p).offset(1 as isize);
                        continue;
                    }
                    82 | _ => {
                        continue;
                    }
                }
                match current_block {
                    13973394567113199817 => {
                        let ra_74: StkId = base.offset(
                            (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize,
                        );
                        memcpy(
                            ra_74.offset(4 as isize) as *mut libc::c_void,
                            ra_74 as *const libc::c_void,
                            (3 as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
                        );
                        (*state).top.p = ra_74.offset(4 as isize).offset(3 as isize);
                        (*call_info).u.l.saved_program_counter = program_counter;
                        ccall(
                            state,
                            ra_74.offset(4 as isize),
                            (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0)
                                as i32,
                            1
                        );
                        trap = (*call_info).u.l.trap;
                        if (trap != 0) as i32 as i64 != 0 {
                            base = ((*call_info).function.p).offset(1 as isize);
                        }
                        let fresh144 = program_counter;
                        program_counter = program_counter.offset(1);
                        i = *fresh144;
                    }
                    _ => {}
                }
                let ra_75: StkId =
                    base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 as isize);
                if get_tag_type ((*ra_75.offset(4 as isize)).value.get_tag()) != TAG_TYPE_NIL {
                    let io1_16: *mut TValue = &mut (*ra_75.offset(2 as isize)).value;
                    let io2_16: *const TValue = &mut (*ra_75.offset(4 as isize)).value;
                    (*io1_16).value = (*io2_16).value;
                    (*io1_16).set_tag((*io2_16).get_tag());
                    program_counter = program_counter.offset(
                        -((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0)
                            as i32 as isize),
                    );
                }
            }
            if (*call_info).call_status as i32 & 1 << 2 != 0 {
                break '_startfunc;
            }
            call_info = (*call_info).previous;
        }
        call_info = newci;
    }
}}
pub unsafe extern "C" fn findfield(state: *mut State, objidx: i32, level: i32) -> bool { unsafe {
    if level == 0 || !(lua_type(state, -1) == 5) {
        return false;
    }
    (*state).push_nil();
    while lua_next(state, -(2)) != 0 {
        if lua_type(state, -(2)) == 4 {
            if lua_rawequal(state, objidx, -1) {
                lua_settop(state, -1 - 1);
                return true;
            } else if findfield(state, objidx, level - 1) {
                lua_pushstring(state, b".\0" as *const u8 as *const i8);
                lua_copy(state, -1, -(3));
                lua_settop(state, -1 - 1);
                lua_concat(state, 3);
                return true;
            }
        }
        lua_settop(state, -1 - 1);
    }
    return false;
}}
pub unsafe extern "C" fn pushglobalfuncname(state: *mut State, ar: *mut Debug) -> i32 { unsafe {
    let top: i32 = (*state).get_top();
    lua_getinfo(state, b"f\0" as *const u8 as *const i8, ar);
    lua_getfield(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const i8,
    );
    lual_checkstack(state, 6, b"not enough stack\0" as *const u8 as *const i8);
    if findfield(state, top + 1, 2) {
        let name: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
        if strncmp(name, b"_G.\0" as *const u8 as *const i8, 3 as u64) == 0 {
            lua_pushstring(state, name.offset(3 as isize));
            lua_rotate(state, -(2), -1);
            lua_settop(state, -1 - 1);
        }
        lua_copy(state, -1, top + 1);
        lua_settop(state, top + 1);
        return 1;
    } else {
        lua_settop(state, top);
        return 0;
    };
}}
pub unsafe extern "C" fn pushfuncname(state: *mut State, ar: *mut Debug) { unsafe {
    if pushglobalfuncname(state, ar) != 0 {
        lua_pushfstring(
            state,
            b"function '%s'\0" as *const u8 as *const i8,
            lua_tolstring(state, -1, std::ptr::null_mut()),
        );
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
    } else if *(*ar).namewhat as i32 != '\0' as i32 {
        lua_pushfstring(
            state,
            b"%s '%s'\0" as *const u8 as *const i8,
            (*ar).namewhat,
            (*ar).name,
        );
    } else if *(*ar).what as i32 == 'm' as i32 {
        lua_pushstring(state, b"main chunk\0" as *const u8 as *const i8);
    } else if *(*ar).what as i32 != 'C' as i32 {
        lua_pushfstring(
            state,
            b"function <%s:%d>\0" as *const u8 as *const i8,
            ((*ar).short_src).as_mut_ptr(),
            (*ar).line_defined,
        );
    } else {
        lua_pushstring(state, b"?\0" as *const u8 as *const i8);
    };
}}
pub unsafe extern "C" fn lastlevel(state: *mut State) -> i32 { unsafe {
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    let mut li: i32 = 1;
    let mut le: i32 = 1;
    while lua_getstack(state, le, &mut ar) != 0 {
        li = le;
        le *= 2;
    }
    while li < le {
        let m: i32 = (li + le) / 2;
        if lua_getstack(state, m, &mut ar) != 0 {
            li = m + 1;
        } else {
            le = m;
        }
    }
    return le - 1;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_traceback(
    state: *mut State,
    other_state: *mut State,
    message: *const i8,
    mut level: i32,
) { unsafe {
    let mut b = Buffer::new();
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    let last: i32 = lastlevel(other_state);
    let mut limit2show: i32 = if last - level > 10 as i32 + 11 as i32 {
        10 as i32
    } else {
        -1
    };
    b.lual_buffinit(state);
    if !message.is_null() {
        b.lual_addstring(message);
        (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh145 = b.length;
        b.length = (b.length).wrapping_add(1);
        *(b.pointer).offset(fresh145 as isize) = '\n' as i8;
    }
    b.lual_addstring(b"stack traceback:\0" as *const u8 as *const i8);
    loop {
        let fresh146 = level;
        level = level + 1;
        if !(lua_getstack(other_state, fresh146, &mut ar) != 0) {
            break;
        }
        let fresh147 = limit2show;
        limit2show = limit2show - 1;
        if fresh147 == 0 {
            let n: i32 = last - level - 11 as i32 + 1;
            lua_pushfstring(
                state,
                b"\n\t...\t(skipping %d levels)\0" as *const u8 as *const i8,
                n,
            );
            b.lual_addvalue();
            level += n;
        } else {
            lua_getinfo(other_state, b"Slnt\0" as *const u8 as *const i8, &mut ar);
            if ar.currentline <= 0 {
                lua_pushfstring(
                    state,
                    b"\n\t%s: in \0" as *const u8 as *const i8,
                    (ar.short_src).as_mut_ptr(),
                );
            } else {
                lua_pushfstring(
                    state,
                    b"\n\t%s:%d: in \0" as *const u8 as *const i8,
                    (ar.short_src).as_mut_ptr(),
                    ar.currentline,
                );
            }
            b.lual_addvalue();
            pushfuncname(state, &mut ar);
            b.lual_addvalue();
            if ar.is_tail_call {
                b.lual_addstring(
                    b"\n\t(...tail calls...)\0" as *const u8 as *const i8,
                );
            }
        }
    }
    b.lual_pushresult();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_argerror(
    state: *mut State,
    mut arg: i32,
    extramsg: *const i8,
) -> i32 { unsafe {
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    if lua_getstack(state, 0, &mut ar) == 0 {
        return lual_error(
            state,
            b"bad argument #%d (%s)\0" as *const u8 as *const i8,
            arg,
            extramsg,
        );
    }
    lua_getinfo(state, b"n\0" as *const u8 as *const i8, &mut ar);
    if strcmp(ar.namewhat, b"method\0" as *const u8 as *const i8) == 0 {
        arg -= 1;
        if arg == 0 {
            return lual_error(
                state,
                b"calling '%s' on bad self (%s)\0" as *const u8 as *const i8,
                ar.name,
                extramsg,
            );
        }
    }
    if (ar.name).is_null() {
        ar.name = if pushglobalfuncname(state, &mut ar) != 0 {
            lua_tolstring(state, -1, std::ptr::null_mut())
        } else {
            b"?\0" as *const u8 as *const i8
        };
    }
    return lual_error(
        state,
        b"bad argument #%d to '%s' (%s)\0" as *const u8 as *const i8,
        arg,
        ar.name,
        extramsg,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_typeerror(
    state: *mut State,
    arg: i32,
    tname: *const i8,
) -> i32 { unsafe {
    let message: *const i8;
    let typearg: *const i8;
    if lual_getmetafield(state, arg, b"__name\0" as *const u8 as *const i8) == 4 {
        typearg = lua_tolstring(state, -1, std::ptr::null_mut());
    } else if lua_type(state, arg) == 2 {
        typearg = b"light userdata\0" as *const u8 as *const i8;
    } else {
        typearg = lua_typename(state, lua_type(state, arg));
    }
    message = lua_pushfstring(
        state,
        b"%s expected, got %s\0" as *const u8 as *const i8,
        tname,
        typearg,
    );
    return lual_argerror(state, arg, message);
}}
pub unsafe extern "C" fn tag_error(state: *mut State, arg: i32, tag: i32) { unsafe {
    lual_typeerror(state, arg, lua_typename(state, tag));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_where(state: *mut State, level: i32) { unsafe {
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    if lua_getstack(state, level, &mut ar) != 0 {
        lua_getinfo(state, b"Sl\0" as *const u8 as *const i8, &mut ar);
        if ar.currentline > 0 {
            lua_pushfstring(
                state,
                b"%s:%d: \0" as *const u8 as *const i8,
                (ar.short_src).as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    lua_pushfstring(state, b"\0" as *const u8 as *const i8);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_error(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> i32 { unsafe {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    lual_where(state, 1);
    lua_pushvfstring(state, fmt, argp.as_va_list());
    lua_concat(state, 2);
    return lua_error(state);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_fileresult(
    state: *mut State,
    stat: i32,
    fname: *const i8,
) -> i32 { unsafe {
    let en: i32 = *__errno_location();
    if stat != 0 {
        (*state).push_boolean(true);
        return 1;
    } else {
        let message: *const i8;
        (*state).push_nil();
        message = if en != 0 {
            strerror(en) as *const i8
        } else {
            b"(no extra info)\0" as *const u8 as *const i8
        };
        if !fname.is_null() {
            lua_pushfstring(state, b"%s: %s\0" as *const u8 as *const i8, fname, message);
        } else {
            lua_pushstring(state, message);
        }
        (*state).push_integer(en as i64);
        return 3;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_execresult(state: *mut State, mut stat: i32) -> i32 { unsafe {
    if stat != 0 && *__errno_location() != 0 {
        return lual_fileresult(state, 0, std::ptr::null());
    } else {
        let mut what: *const i8 = b"exit\0" as *const u8 as *const i8;
        if stat & 0x7f as i32 == 0 {
            stat = (stat & 0xff00 as i32) >> 8;
        } else if ((stat & 0x7f as i32) + 1) as i8 as i32 >> 1 > 0 {
            stat = stat & 0x7f as i32;
            what = b"signal\0" as *const u8 as *const i8;
        }
        if *what as i32 == 'e' as i32 && stat == 0 {
            (*state).push_boolean(true);
        } else {
            (*state).push_nil();
        }
        lua_pushstring(state, what);
        (*state).push_integer(stat as i64);
        return 3;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_newmetatable(state: *mut State, tname: *const i8) -> i32 { unsafe {
    if lua_getfield(state, -(1000000 as i32) - 1000 as i32, tname) != 0 {
        return 0;
    }
    lua_settop(state, -1 - 1);
    lua_createtable(state, 0, 2);
    lua_pushstring(state, tname);
    lua_setfield(state, -(2), b"__name\0" as *const u8 as *const i8);
    lua_pushvalue(state, -1);
    lua_setfield(state, -(1000000 as i32) - 1000 as i32, tname);
    return 1;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_setmetatable(state: *mut State, tname: *const i8) { unsafe {
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, tname);
    lua_setmetatable(state, -(2));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_testudata(
    state: *mut State,
    ud: i32,
    tname: *const i8,
) -> *mut libc::c_void { unsafe {
    let mut p: *mut libc::c_void = lua_touserdata(state, ud);
    if !p.is_null() {
        if lua_getmetatable(state, ud) != 0 {
            lua_getfield(state, -(1000000 as i32) - 1000 as i32, tname);
            if !lua_rawequal(state, -1, -(2)) {
                p = std::ptr::null_mut();
            }
            lua_settop(state, -(2) - 1);
            return p;
        }
    }
    return std::ptr::null_mut();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkudata(
    state: *mut State,
    ud: i32,
    tname: *const i8,
) -> *mut libc::c_void { unsafe {
    let p: *mut libc::c_void = lual_testudata(state, ud, tname);
    (((p != std::ptr::null_mut()) as i32 != 0) as i32 as i64 != 0
        || lual_typeerror(state, ud, tname) != 0) as i32;
    return p;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkoption(
    state: *mut State,
    arg: i32,
    def: *const i8,
    lst: *const *const i8,
) -> i32 { unsafe {
    let name: *const i8 = if !def.is_null() {
        lual_optlstring(state, arg, def, std::ptr::null_mut())
    } else {
        lual_checklstring(state, arg, std::ptr::null_mut())
    };
    let mut i: i32;
    i = 0;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0 {
            return i;
        }
        i += 1;
    }
    return lual_argerror(
        state,
        arg,
        lua_pushfstring(
            state,
            b"invalid option '%s'\0" as *const u8 as *const i8,
            name,
        ),
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkstack(
    state: *mut State,
    space: i32,
    message: *const i8,
) { unsafe {
    if ((lua_checkstack(state, space) == 0) as i32 != 0) as i32 as i64 != 0 {
        if !message.is_null() {
            lual_error(
                state,
                b"stack overflow (%s)\0" as *const u8 as *const i8,
                message,
            );
        } else {
            lual_error(state, b"stack overflow\0" as *const u8 as *const i8);
        }
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checktype(state: *mut State, arg: i32, t: i32) { unsafe {
    if ((lua_type(state, arg) != t) as i32 != 0) as i32 as i64 != 0 {
        tag_error(state, arg, t);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkany(state: *mut State, arg: i32) { unsafe {
    if ((lua_type(state, arg) == -1) as i32 != 0) as i32 as i64 != 0 {
        lual_argerror(state, arg, b"value expected\0" as *const u8 as *const i8);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checklstring(
    state: *mut State,
    arg: i32,
    length: *mut u64,
) -> *const i8 { unsafe {
    let s: *const i8 = lua_tolstring(state, arg, length);
    if (s.is_null() as i32 != 0) as i32 as i64 != 0 {
        tag_error(state, arg, 4);
    }
    return s;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_optlstring(
    state: *mut State,
    arg: i32,
    def: *const i8,
    length: *mut u64,
) -> *const i8 { unsafe {
    if lua_type(state, arg) <= 0 {
        if !length.is_null() {
            *length = if !def.is_null() {
                strlen(def)
            } else {
                0u64
            };
        }
        return def;
    } else {
        return lual_checklstring(state, arg, length);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checknumber(state: *mut State, arg: i32) -> f64 { unsafe {
    let mut is_number: bool = false;
    let d: f64 = lua_tonumberx(state, arg, &mut is_number);
    if !is_number {
        tag_error(state, arg, 3);
    }
    return d;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_optnumber(state: *mut State, arg: i32, def: f64) -> f64 { unsafe {
    return if lua_type(state, arg) <= 0 {
        def
    } else {
        lual_checknumber(state, arg)
    };
}}
pub unsafe extern "C" fn interror(state: *mut State, arg: i32) { unsafe {
    if lua_isnumber(state, arg) {
        lual_argerror(
            state,
            arg,
            b"number has no integer representation\0" as *const u8 as *const i8,
        );
    } else {
        tag_error(state, arg, 3);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkinteger(state: *mut State, arg: i32) -> i64 { unsafe {
    let mut is_number: bool = false;
    let ret: i64 = lua_tointegerx(state, arg, &mut is_number);
    if !is_number {
        interror(state, arg);
    }
    return ret;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_optinteger(state: *mut State, arg: i32, def: i64) -> i64 { unsafe {
    return if lua_type(state, arg) <= 0 {
        def
    } else {
        lual_checkinteger(state, arg)
    };
}}
pub unsafe extern "C" fn resizebox(
    state: *mut State,
    index: i32,
    new_size: u64,
) -> *mut libc::c_void { unsafe {
    let box_0: *mut UBox = lua_touserdata(state, index) as *mut UBox;
    let temp: *mut libc::c_void =
        raw_allocate((*box_0).box_0, (*box_0).bsize, new_size);
    if ((temp.is_null() && new_size > 0u64) as i32 != 0) as i32 as i64 != 0 {
        lua_pushstring(state, b"not enough memory\0" as *const u8 as *const i8);
        lua_error(state);
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = new_size;
    return temp;
}}
pub unsafe extern "C" fn boxgc(state: *mut State) -> i32 { unsafe {
    resizebox(state, 1, 0u64);
    return 0;
}}
static mut BOX_METATABLE: [RegisteredFunction; 3] = {
    [
        {
            let init = RegisteredFunction {
                name: b"__gc\0" as *const u8 as *const i8,
                function: Some(boxgc as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__close\0" as *const u8 as *const i8,
                function: Some(boxgc as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn newbox(state: *mut State) { unsafe {
    let box_0: *mut UBox =
        lua_newuserdatauv(state, ::core::mem::size_of::<UBox>() as u64, 0) as *mut UBox;
    (*box_0).box_0 = std::ptr::null_mut();
    (*box_0).bsize = 0;
    if lual_newmetatable(state, b"_UBOX*\0" as *const u8 as *const i8) != 0 {
        lual_setfuncs(state, BOX_METATABLE.as_ptr(), 0);
    }
    lua_setmetatable(state, -(2));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_ref(state: *mut State, mut t: i32) -> i32 { unsafe {
    let mut ref_0: i32;
    if lua_type(state, -1) == 0 {
        lua_settop(state, -1 - 1);
        return -1;
    }
    t = lua_absindex(state, t);
    if lua_rawgeti(state, t, (2 + 1) as i64) == 0 {
        ref_0 = 0;
        (*state).push_integer(0);
        lua_rawseti(state, t, (2 + 1) as i64);
    } else {
        ref_0 = lua_tointegerx(state, -1, std::ptr::null_mut()) as i32;
    }
    lua_settop(state, -1 - 1);
    if ref_0 != 0 {
        lua_rawgeti(state, t, ref_0 as i64);
        lua_rawseti(state, t, (2 + 1) as i64);
    } else {
        ref_0 = lua_rawlen(state, t) as i32 + 1;
    }
    lua_rawseti(state, t, ref_0 as i64);
    return ref_0;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_unref(state: *mut State, mut t: i32, ref_0: i32) { unsafe {
    if ref_0 >= 0 {
        t = lua_absindex(state, t);
        lua_rawgeti(state, t, (2 + 1) as i64);
        lua_rawseti(state, t, ref_0 as i64);
        (*state).push_integer(ref_0 as i64);
        lua_rawseti(state, t, (2 + 1) as i64);
    }
}}
pub unsafe extern "C" fn get_f(
    mut _state: *mut State,
    ud: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 { unsafe {
    let lf: *mut LoadF = ud as *mut LoadF;
    if (*lf).n > 0 {
        *size = (*lf).n as u64;
        (*lf).n = 0;
    } else {
        if feof((*lf).f) != 0 {
            return std::ptr::null();
        }
        *size = fread(
            ((*lf).buffer).as_mut_ptr() as *mut libc::c_void,
            1 as u64,
            ::core::mem::size_of::<[i8; 8192]>() as u64,
            (*lf).f,
        );
    }
    return ((*lf).buffer).as_mut_ptr();
}}
pub unsafe extern "C" fn errfile(
    state: *mut State,
    what: *const i8,
    fnameindex: i32,
) -> i32 { unsafe {
    let err: i32 = *__errno_location();
    let filename: *const i8 =
        (lua_tolstring(state, fnameindex, std::ptr::null_mut())).offset(1 as isize);
    if err != 0 {
        lua_pushfstring(
            state,
            b"cannot %s %s: %s\0" as *const u8 as *const i8,
            what,
            filename,
            strerror(err),
        );
    } else {
        lua_pushfstring(
            state,
            b"cannot %s %s\0" as *const u8 as *const i8,
            what,
            filename,
        );
    }
    lua_rotate(state, fnameindex, -1);
    lua_settop(state, -1 - 1);
    return 5 + 1;
}}
pub unsafe extern "C" fn skip_bom(f: *mut FILE) -> i32 { unsafe {
    let c: i32 = getc(f);
    if c == 0xef as i32 && getc(f) == 0xbb as i32 && getc(f) == 0xbf as i32 {
        return getc(f);
    } else {
        return c;
    };
}}
pub unsafe extern "C" fn skipcomment(f: *mut FILE, cp: *mut i32) -> i32 { unsafe {
    *cp = skip_bom(f);
    let mut c: i32 = *cp;
    if c == '#' as i32 {
        loop {
            c = getc(f);
            if !(c != -1 && c != '\n' as i32) {
                break;
            }
        }
        *cp = getc(f);
        return 1;
    } else {
        return 0;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_loadfilex(
    state: *mut State,
    filename: *const i8,
    mode: *const i8,
) -> i32 { unsafe {
    let mut lf: LoadF = LoadF {
        n: 0,
        f: std::ptr::null_mut(),
        buffer: [0; 8192],
    };
    let status: i32;
    let readstatus: i32;
    let mut c: i32 = 0;
    let fnameindex: i32 = (*state).get_top() + 1;
    if filename.is_null() {
        lua_pushstring(state, b"=stdin\0" as *const u8 as *const i8);
        lf.f = stdin;
    } else {
        lua_pushfstring(state, b"@%s\0" as *const u8 as *const i8, filename);
        *__errno_location() = 0;
        lf.f = fopen(filename, b"r\0" as *const u8 as *const i8);
        if (lf.f).is_null() {
            return errfile(state, b"open\0" as *const u8 as *const i8, fnameindex);
        }
    }
    lf.n = 0;
    if skipcomment(lf.f, &mut c) != 0 {
        let fresh148 = lf.n;
        lf.n = lf.n + 1;
        lf.buffer[fresh148 as usize] = '\n' as i8;
    }
    if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
        lf.n = 0;
        if !filename.is_null() {
            *__errno_location() = 0;
            lf.f = freopen(filename, b"rb\0" as *const u8 as *const i8, lf.f);
            if (lf.f).is_null() {
                return errfile(state, b"reopen\0" as *const u8 as *const i8, fnameindex);
            }
            skipcomment(lf.f, &mut c);
        }
    }
    if c != -1 {
        let fresh149 = lf.n;
        lf.n = lf.n + 1;
        lf.buffer[fresh149 as usize] = c as i8;
    }
    *__errno_location() = 0;
    status = lua_load(
        state,
        Some(get_f as unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8),
        &mut lf as *mut LoadF as *mut libc::c_void,
        lua_tolstring(state, -1, std::ptr::null_mut()),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        fclose(lf.f);
    }
    if readstatus != 0 {
        lua_settop(state, fnameindex);
        return errfile(state, b"read\0" as *const u8 as *const i8, fnameindex);
    }
    lua_rotate(state, fnameindex, -1);
    lua_settop(state, -1 - 1);
    return status;
}}
pub unsafe extern "C" fn get_s(
    mut _state: *mut State,
    ud: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 { unsafe {
    let lexical_state: *mut LoadS = ud as *mut LoadS;
    if (*lexical_state).size == 0u64 {
        return std::ptr::null();
    }
    *size = (*lexical_state).size;
    (*lexical_state).size = 0;
    return (*lexical_state).s;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_loadbufferx(
    state: *mut State,
    buffer: *const i8,
    size: u64,
    name: *const i8,
    mode: *const i8,
) -> i32 { unsafe {
    let mut lexical_state: LoadS = LoadS {
        s: std::ptr::null(),
        size: 0,
    };
    lexical_state.s = buffer;
    lexical_state.size = size;
    return lua_load(
        state,
        Some(get_s as unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8),
        &mut lexical_state as *mut LoadS as *mut libc::c_void,
        name,
        mode,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_loadstring(state: *mut State, s: *const i8) -> i32 { unsafe {
    return lual_loadbufferx(state, s, strlen(s), s, std::ptr::null());
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_getmetafield(
    state: *mut State,
    obj: i32,
    event: *const i8,
) -> i32 { unsafe {
    if lua_getmetatable(state, obj) == 0 {
        return 0;
    } else {
        let tag: i32;
        lua_pushstring(state, event);
        tag = lua_rawget(state, -(2));
        if tag == 0 {
            lua_settop(state, -(2) - 1);
        } else {
            lua_rotate(state, -(2), -1);
            lua_settop(state, -1 - 1);
        }
        return tag;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_callmeta(
    state: *mut State,
    mut obj: i32,
    event: *const i8,
) -> i32 { unsafe {
    obj = lua_absindex(state, obj);
    if lual_getmetafield(state, obj, event) == 0 {
        return 0;
    }
    lua_pushvalue(state, obj);
    lua_callk(state, 1, 1, 0, None);
    return 1;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_len(state: *mut State, index: i32) -> i64 { unsafe {
    let l: i64;
    let mut is_number: bool = false;
    lua_len(state, index);
    l = lua_tointegerx(state, -1, &mut is_number);
    if !is_number {
        lual_error(state, b"object length is not an integer\0" as *const u8 as *const i8);
    }
    lua_settop(state, -1 - 1);
    return l;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_tolstring(
    state: *mut State,
    mut index: i32,
    length: *mut u64,
) -> *const i8 { unsafe {
    index = lua_absindex(state, index);
    if lual_callmeta(state, index, b"__tostring\0" as *const u8 as *const i8) != 0 {
        if !lua_isstring(state, -1) {
            lual_error(
                state,
                b"'__tostring' must return a string\0" as *const u8 as *const i8,
            );
        }
    } else {
        match lua_type(state, index) {
            3 => {
                if lua_isinteger(state, index) {
                    lua_pushfstring(
                        state,
                        b"%I\0" as *const u8 as *const i8,
                        lua_tointegerx(state, index, std::ptr::null_mut()),
                    );
                } else {
                    lua_pushfstring(
                        state,
                        b"%f\0" as *const u8 as *const i8,
                        lua_tonumberx(state, index, std::ptr::null_mut()),
                    );
                }
            }
            4 => {
                lua_pushvalue(state, index);
            }
            1 => {
                lua_pushstring(
                    state,
                    if lua_toboolean(state, index) != 0 {
                        b"true\0" as *const u8 as *const i8
                    } else {
                        b"false\0" as *const u8 as *const i8
                    },
                );
            }
            0 => {
                lua_pushstring(state, b"nil\0" as *const u8 as *const i8);
            }
            _ => {
                let tag: i32 =
                    lual_getmetafield(state, index, b"__name\0" as *const u8 as *const i8);
                let kind: *const i8 = if tag == 4 {
                    lua_tolstring(state, -1, std::ptr::null_mut())
                } else {
                    lua_typename(state, lua_type(state, index))
                };
                lua_pushfstring(
                    state,
                    b"%s: %p\0" as *const u8 as *const i8,
                    kind,
                    lua_topointer(state, index),
                );
                if tag != 0 {
                    lua_rotate(state, -(2), -1);
                    lua_settop(state, -1 - 1);
                }
            }
        }
    }
    return lua_tolstring(state, -1, length);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_setfuncs(
    state: *mut State,
    mut l: *const RegisteredFunction,
    nup: i32,
) { unsafe {
    lual_checkstack(state, nup, b"too many upvalues\0" as *const u8 as *const i8);
    while !((*l).name).is_null() {
        if ((*l).function).is_none() {
            (*state).push_boolean(false);
        } else {
            let mut i: i32;
            i = 0;
            while i < nup {
                lua_pushvalue(state, -nup);
                i += 1;
            }
            lua_pushcclosure(state, (*l).function, nup);
        }
        lua_setfield(state, -(nup + 2), (*l).name);
        l = l.offset(1);
    }
    lua_settop(state, -nup - 1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_getsubtable(
    state: *mut State,
    mut index: i32,
    fname: *const i8,
) -> i32 { unsafe {
    if lua_getfield(state, index, fname) == 5 {
        return 1;
    } else {
        lua_settop(state, -1 - 1);
        index = lua_absindex(state, index);
        lua_createtable(state, 0, 0);
        lua_pushvalue(state, -1);
        lua_setfield(state, index, fname);
        return 0;
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_requiref(
    state: *mut State,
    modname: *const i8,
    openf: CFunction,
    glb: i32,
) { unsafe {
    lual_getsubtable(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const i8,
    );
    lua_getfield(state, -1, modname);
    if lua_toboolean(state, -1) == 0 {
        lua_settop(state, -1 - 1);
        lua_pushcclosure(state, openf, 0);
        lua_pushstring(state, modname);
        lua_callk(state, 1, 1, 0, None);
        lua_pushvalue(state, -1);
        lua_setfield(state, -(3), modname);
    }
    lua_rotate(state, -(2), -1);
    lua_settop(state, -1 - 1);
    if glb != 0 {
        lua_pushvalue(state, -1);
        lua_setglobal(state, modname);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_addgsub(
    b: *mut Buffer,
    mut s: *const i8,
    p: *const i8,
    r: *const i8,
) { unsafe {
    let mut wild: *const i8;
    let l: u64 = strlen(p);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        (*b).lual_addlstring(s, wild.offset_from(s) as i64 as u64);
        (*b).lual_addstring(r);
        s = wild.offset(l as isize);
    }
    (*b).lual_addstring(s);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_gsub(
    state: *mut State,
    s: *const i8,
    p: *const i8,
    r: *const i8,
) -> *const i8 { unsafe {
    let mut b = Buffer::new();
    b.lual_buffinit(state);
    lual_addgsub(&mut b, s, p, r);
    b.lual_pushresult();
    return lua_tolstring(state, -1, std::ptr::null_mut());
}}
pub unsafe extern "C" fn raw_allocate(
    ptr: *mut libc::c_void,
    mut _osize: u64,
    new_size: u64,
) -> *mut libc::c_void { unsafe {
    if new_size == 0u64 {
        free(ptr);
        return std::ptr::null_mut();
    } else {
        return realloc(ptr, new_size);
    };
}}
pub unsafe extern "C" fn panic(state: *mut State) -> i32 { unsafe {
    let message: *const i8 = if lua_type(state, -1) == 4 {
        lua_tolstring(state, -1, std::ptr::null_mut())
    } else {
        b"error object is not a string\0" as *const u8 as *const i8
    };
    fprintf(
        stderr,
        b"PANIC: unprotected error in call to Lua API (%s)\n\0" as *const u8 as *const i8,
        message,
    );
    fflush(stderr);
    return 0;
}}
pub unsafe extern "C" fn checkcontrol(
    state: *mut State,
    mut message: *const i8,
    tocont: i32,
) -> i32 { unsafe {
    if tocont != 0 || {
        let fresh150 = message;
        message = message.offset(1);
        *fresh150 as i32 != '@' as i32
    } {
        return 0;
    } else {
        if strcmp(message, b"off\0" as *const u8 as *const i8) == 0 {
            lua_setwarnf(
                state,
                Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                state as *mut libc::c_void,
            );
        } else if strcmp(message, b"on\0" as *const u8 as *const i8) == 0 {
            lua_setwarnf(
                state,
                Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                state as *mut libc::c_void,
            );
        }
        return 1;
    };
}}
pub unsafe extern "C" fn warnfoff(
    ud: *mut libc::c_void,
    message: *const i8,
    tocont: i32,
) { unsafe {
    checkcontrol(ud as *mut State, message, tocont);
}}
pub unsafe extern "C" fn warnfcont(
    ud: *mut libc::c_void,
    message: *const i8,
    tocont: i32,
) { unsafe {
    let state: *mut State = ud as *mut State;
    fprintf(stderr, b"%s\0" as *const u8 as *const i8, message);
    fflush(stderr);
    if tocont != 0 {
        lua_setwarnf(
            state,
            Some(warnfcont as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
            state as *mut libc::c_void,
        );
    } else {
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const i8,
            b"\n\0" as *const u8 as *const i8,
        );
        fflush(stderr);
        lua_setwarnf(
            state,
            Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
            state as *mut libc::c_void,
        );
    };
}}
pub unsafe extern "C" fn warnfon(
    ud: *mut libc::c_void,
    message: *const i8,
    tocont: i32,
) { unsafe {
    if checkcontrol(ud as *mut State, message, tocont) != 0 {
        return;
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const i8,
        b"Lua warning: \0" as *const u8 as *const i8,
    );
    fflush(stderr);
    warnfcont(ud, message, tocont);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_newstate() -> *mut State { unsafe {
    let state: *mut State = lua_newstate();
    if (state != std::ptr::null_mut()) as i32 as i64 != 0 {
        lua_atpanic(
            state,
            Some(panic as unsafe extern "C" fn(*mut State) -> i32),
        );
        lua_setwarnf(
            state,
            Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
            state as *mut libc::c_void,
        );
    }
    return state;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_checkversion_(state: *mut State, version: f64, size: u64) { unsafe {
    let v: f64 = 504.0;
    if size
        != (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64)
    {
        lual_error(
            state,
            b"core and library have incompatible numeric types\0" as *const u8 as *const i8,
        );
    } else if v != version {
        lual_error(
            state,
            b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8 as *const i8,
            version,
            v,
        );
    }
}}
pub unsafe extern "C" fn luab_print(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    let mut i: i32;
    i = 1;
    while i <= n {
        let mut l: u64 = 0;
        let s: *const i8 = lual_tolstring(state, i, &mut l);
        if i > 1 {
            fwrite(
                b"\t\0" as *const u8 as *const i8 as *const libc::c_void,
                ::core::mem::size_of::<i8>() as u64,
                1 as u64,
                stdout,
            );
        }
        fwrite(
            s as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            l,
            stdout,
        );
        lua_settop(state, -1 - 1);
        i += 1;
    }
    fwrite(
        b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        1 as u64,
        stdout,
    );
    fflush(stdout);
    return 0;
}}
pub unsafe extern "C" fn luab_warn(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    let mut i: i32;
    lual_checklstring(state, 1, std::ptr::null_mut());
    i = 2;
    while i <= n {
        lual_checklstring(state, i, std::ptr::null_mut());
        i += 1;
    }
    i = 1;
    while i < n {
        lua_warning(state, lua_tolstring(state, i, std::ptr::null_mut()), 1);
        i += 1;
    }
    lua_warning(state, lua_tolstring(state, n, std::ptr::null_mut()), 0);
    return 0;
}}
pub unsafe extern "C" fn b_str2int(mut s: *const i8, base: i32, pn: *mut i64) -> *const i8 { unsafe {
    let mut n: u64 = 0;
    let mut is_negative_: i32 = 0;
    s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8) as isize);
    if *s as i32 == '-' as i32 {
        s = s.offset(1);
        is_negative_ = 1;
    } else if *s as i32 == '+' as i32 {
        s = s.offset(1);
    }
    if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32 & _ISALPHANUMERIC as i32 as u16 as i32
        == 0
    {
        return std::ptr::null();
    }
    loop {
        let digit_0: i32 = if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISDIGIT as i32 as u16 as i32
            != 0
        {
            *s as i32 - '0' as i32
        } else {
            toupper(*s as u8 as i32) - 'A' as i32 + 10 as i32
        };
        if digit_0 >= base {
            return std::ptr::null();
        }
        n = n.wrapping_mul(base as u64).wrapping_add(digit_0 as u64);
        s = s.offset(1);
        if !(*(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISALPHANUMERIC as i32 as u16 as i32
            != 0)
        {
            break;
        }
    }
    s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8) as isize);
    *pn = (if is_negative_ != 0 {
        (0u64).wrapping_sub(n)
    } else {
        n
    }) as i64;
    return s;
}}
pub unsafe extern "C" fn luab_tonumber(state: *mut State) -> i32 { unsafe {
    if lua_type(state, 2) <= 0 {
        if lua_type(state, 1) == 3 {
            lua_settop(state, 1);
            return 1;
        } else {
            let mut l: u64 = 0;
            let s: *const i8 = lua_tolstring(state, 1, &mut l);
            if !s.is_null() && lua_stringtonumber(state, s) == l.wrapping_add(1 as u64) {
                return 1;
            }
            lual_checkany(state, 1);
        }
    } else {
        let mut l_0: u64 = 0;
        let s_0: *const i8;
        let mut n: i64 = 0;
        let base: i64 = lual_checkinteger(state, 2);
        lual_checktype(state, 1, 4);
        s_0 = lua_tolstring(state, 1, &mut l_0);
        (((2 as i64 <= base && base <= 36 as i32 as i64) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(
                state,
                2,
                b"base out of range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if b_str2int(s_0, base as i32, &mut n) == s_0.offset(l_0 as isize) {
            (*state).push_integer(n);
            return 1;
        }
    }
    (*state).push_nil();
    return 1;
}}
pub unsafe extern "C" fn luab_error(state: *mut State) -> i32 { unsafe {
    let level: i32 = lual_optinteger(state, 2, 1) as i32;
    lua_settop(state, 1);
    if lua_type(state, 1) == 4 && level > 0 {
        lual_where(state, level);
        lua_pushvalue(state, 1);
        lua_concat(state, 2);
    }
    return lua_error(state);
}}
pub unsafe extern "C" fn luab_getmetatable(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    if lua_getmetatable(state, 1) == 0 {
        (*state).push_nil();
        return 1;
    }
    lual_getmetafield(state, 1, b"__metatable\0" as *const u8 as *const i8);
    return 1;
}}
pub unsafe extern "C" fn luab_setmetatable(state: *mut State) -> i32 { unsafe {
    let t: i32 = lua_type(state, 2);
    lual_checktype(state, 1, 5);
    (((t == 0 || t == 5) as i32 != 0) as i32 as i64 != 0
        || lual_typeerror(state, 2, b"nil or table\0" as *const u8 as *const i8) != 0)
        as i32;
    if ((lual_getmetafield(state, 1, b"__metatable\0" as *const u8 as *const i8) != 0) as i32
        != 0) as i32 as i64
        != 0
    {
        return lual_error(
            state,
            b"cannot change a protected metatable\0" as *const u8 as *const i8,
        );
    }
    lua_settop(state, 2);
    lua_setmetatable(state, 1);
    return 1;
}}
pub unsafe extern "C" fn luab_rawequal(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    lual_checkany(state, 2);
    (*state).push_boolean(lua_rawequal(state, 1, 2));
    return 1;
}}
pub unsafe extern "C" fn luab_rawlen(state: *mut State) -> i32 { unsafe {
    let t: i32 = lua_type(state, 1);
    (((t == 5 || t == 4) as i32 != 0) as i32 as i64 != 0
        || lual_typeerror(state, 1, b"table or string\0" as *const u8 as *const i8) != 0)
        as i32;
    (*state).push_integer(lua_rawlen(state, 1) as i64);
    return 1;
}}
pub unsafe extern "C" fn luab_rawget(state: *mut State) -> i32 { unsafe {
    lual_checktype(state, 1, 5);
    lual_checkany(state, 2);
    lua_settop(state, 2);
    lua_rawget(state, 1);
    return 1;
}}
pub unsafe extern "C" fn luab_rawset(state: *mut State) -> i32 { unsafe {
    lual_checktype(state, 1, 5);
    lual_checkany(state, 2);
    lual_checkany(state, 3);
    lua_settop(state, 3);
    lua_rawset(state, 1);
    return 1;
}}
pub unsafe extern "C" fn pushmode(state: *mut State, oldmode: i32) -> i32 { unsafe {
    if oldmode == -1 {
        (*state).push_nil();
    } else {
        lua_pushstring(
            state,
            if oldmode == 11 as i32 {
                b"incremental\0" as *const u8 as *const i8
            } else {
                b"generational\0" as *const u8 as *const i8
            },
        );
    }
    return 1;
}}
pub unsafe extern "C" fn luab_collectgarbage(state: *mut State) -> i32 { unsafe {
    static mut OPTS: [*const i8; 11] = [
        b"stop\0" as *const u8 as *const i8,
        b"restart\0" as *const u8 as *const i8,
        b"collect\0" as *const u8 as *const i8,
        b"count\0" as *const u8 as *const i8,
        b"step\0" as *const u8 as *const i8,
        b"setpause\0" as *const u8 as *const i8,
        b"setstepmul\0" as *const u8 as *const i8,
        b"isrunning\0" as *const u8 as *const i8,
        b"generational\0" as *const u8 as *const i8,
        b"incremental\0" as *const u8 as *const i8,
        std::ptr::null(),
    ];
    static mut OPTS_NUMBERS: [i32; 10] = [
        0, 1, 2, 3, 5, 6, 7, 9 as i32, 10 as i32, 11 as i32,
    ];
    let o: i32 = OPTS_NUMBERS[lual_checkoption(
        state,
        1,
        b"collect\0" as *const u8 as *const i8,
        OPTS.as_ptr(),
    ) as usize];
    match o {
        3 => {
            let k: i32 = lua_gc(state, o);
            let b: i32 = lua_gc(state, 4);
            if !(k == -1) {
                (*state).push_number(k as f64 + b as f64 / 1024.0);
                return 1;
            }
        }
        5 => {
            let step: i32 = lual_optinteger(state, 2, 0) as i32;
            let res: i32 = lua_gc(state, o, step);
            if !(res == -1) {
                (*state).push_boolean(0 != res);
                return 1;
            }
        }
        6 | 7 => {
            let p: i32 = lual_optinteger(state, 2, 0) as i32;
            let previous: i32 = lua_gc(state, o, p);
            if !(previous == -1) {
                (*state).push_integer(previous as i64);
                return 1;
            }
        }
        9 => {
            let res_0: i32 = lua_gc(state, o);
            if !(res_0 == -1) {
                (*state).push_boolean(0 != res_0);
                return 1;
            }
        }
        10 => {
            let minormul: i32 = lual_optinteger(state, 2, 0) as i32;
            let majormul: i32 = lual_optinteger(state, 3, 0) as i32;
            return pushmode(state, lua_gc(state, o, minormul, majormul));
        }
        11 => {
            let pause: i32 = lual_optinteger(state, 2, 0) as i32;
            let stepmul: i32 = lual_optinteger(state, 3, 0) as i32;
            let stepsize: i32 = lual_optinteger(state, 4, 0) as i32;
            return pushmode(state, lua_gc(state, o, pause, stepmul, stepsize));
        }
        _ => {
            let res_1: i32 = lua_gc(state, o);
            if !(res_1 == -1) {
                (*state).push_integer(res_1 as i64);
                return 1;
            }
        }
    }
    (*state).push_nil();
    return 1;
}}
pub unsafe extern "C" fn luab_type(state: *mut State) -> i32 { unsafe {
    let t: i32 = lua_type(state, 1);
    (((t != -1) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(state, 1, b"value expected\0" as *const u8 as *const i8) != 0)
        as i32;
    lua_pushstring(state, lua_typename(state, t));
    return 1;
}}
pub unsafe extern "C" fn luab_next(state: *mut State) -> i32 { unsafe {
    lual_checktype(state, 1, 5);
    lua_settop(state, 2);
    if lua_next(state, 1) != 0 {
        return 2;
    } else {
        (*state).push_nil();
        return 1;
    };
}}
pub unsafe extern "C" fn pairscont(mut _state: *mut State, mut _status: i32, mut _k: i64) -> i32 {
    return 3;
}
pub unsafe extern "C" fn luab_pairs(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    if lual_getmetafield(state, 1, b"__pairs\0" as *const u8 as *const i8) == 0 {
        lua_pushcclosure(
            state,
            Some(luab_next as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        lua_pushvalue(state, 1);
        (*state).push_nil();
    } else {
        lua_pushvalue(state, 1);
        lua_callk(
            state,
            1,
            3,
            0,
            Some(pairscont as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
        );
    }
    return 3;
}}
pub unsafe extern "C" fn ipairsaux(state: *mut State) -> i32 { unsafe {
    let mut i: i64 = lual_checkinteger(state, 2);
    i = (i as u64).wrapping_add(1 as u64) as i64;
    (*state).push_integer(i);
    return if lua_geti(state, 1, i) == 0 {
        1
    } else {
        2
    };
}}
pub unsafe extern "C" fn luab_ipairs(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    lua_pushcclosure(
        state,
        Some(ipairsaux as unsafe extern "C" fn(*mut State) -> i32),
        0,
    );
    lua_pushvalue(state, 1);
    (*state).push_integer(0);
    return 3;
}}
pub unsafe extern "C" fn load_aux(state: *mut State, status: i32, envidx: i32) -> i32 { unsafe {
    if ((status == 0) as i32 != 0) as i32 as i64 != 0 {
        if envidx != 0 {
            lua_pushvalue(state, envidx);
            if (lua_setupvalue(state, -(2), 1)).is_null() {
                lua_settop(state, -1 - 1);
            }
        }
        return 1;
    } else {
        (*state).push_nil();
        lua_rotate(state, -(2), 1);
        return 2;
    };
}}
pub unsafe extern "C" fn luab_loadfile(state: *mut State) -> i32 { unsafe {
    let fname: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
    let mode: *const i8 = lual_optlstring(state, 2, std::ptr::null(), std::ptr::null_mut());
    let env: i32 = if !(lua_type(state, 3) == -1) {
        3
    } else {
        0
    };
    let status: i32 = lual_loadfilex(state, fname, mode);
    return load_aux(state, status, env);
}}
pub unsafe extern "C" fn generic_reader(
    state: *mut State,
    mut _ud: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 { unsafe {
    lual_checkstack(
        state,
        2,
        b"too many nested functions\0" as *const u8 as *const i8,
    );
    lua_pushvalue(state, 1);
    lua_callk(state, 0, 1, 0, None);
    if lua_type(state, -1) == 0 {
        lua_settop(state, -1 - 1);
        *size = 0;
        return std::ptr::null();
    } else if !lua_isstring(state, -1) {
        lual_error(
            state,
            b"reader function must return a string\0" as *const u8 as *const i8,
        );
    }
    lua_copy(state, -1, 5);
    lua_settop(state, -1 - 1);
    return lua_tolstring(state, 5, size);
}}
pub unsafe extern "C" fn luab_load(state: *mut State) -> i32 { unsafe {
    let status: i32;
    let mut l: u64 = 0;
    let s: *const i8 = lua_tolstring(state, 1, &mut l);
    let mode: *const i8 = lual_optlstring(
        state,
        3,
        b"bt\0" as *const u8 as *const i8,
        std::ptr::null_mut(),
    );
    let env: i32 = if !(lua_type(state, 4) == -1) {
        4
    } else {
        0
    };
    if !s.is_null() {
        let chunkname: *const i8 = lual_optlstring(state, 2, s, std::ptr::null_mut());
        status = lual_loadbufferx(state, s, l, chunkname, mode);
    } else {
        let chunkname_0: *const i8 = lual_optlstring(
            state,
            2,
            b"=(load)\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        lual_checktype(state, 1, 6);
        lua_settop(state, 5);
        status = lua_load(
            state,
            Some(
                generic_reader
                    as unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8,
            ),
            std::ptr::null_mut(),
            chunkname_0,
            mode,
        );
    }
    return load_aux(state, status, env);
}}
pub unsafe extern "C" fn dofilecont(state: *mut State, mut _d1: i32, mut _d2: i64) -> i32 { unsafe {
    return (*state).get_top() - 1;
}}
pub unsafe extern "C" fn luab_dofile(state: *mut State) -> i32 { unsafe {
    let fname: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
    lua_settop(state, 1);
    if ((lual_loadfilex(state, fname, std::ptr::null()) != 0) as i32 != 0) as i32 as i64 != 0 {
        return lua_error(state);
    }
    lua_callk(
        state,
        0,
        -1,
        0,
        Some(dofilecont as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
    );
    return dofilecont(state, 0, 0);
}}
pub unsafe extern "C" fn luab_assert(state: *mut State) -> i32 { unsafe {
    if (lua_toboolean(state, 1) != 0) as i32 as i64 != 0 {
        return (*state).get_top();
    } else {
        lual_checkany(state, 1);
        lua_rotate(state, 1, -1);
        lua_settop(state, -1 - 1);
        lua_pushstring(state, b"assertion failed!\0" as *const u8 as *const i8);
        lua_settop(state, 1);
        return luab_error(state);
    };
}}
pub unsafe extern "C" fn luab_select(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    if lua_type(state, 1) == 4 && *lua_tolstring(state, 1, std::ptr::null_mut()) as i32 == '#' as i32 {
        (*state).push_integer((n - 1) as i64);
        return 1;
    } else {
        let mut i: i64 = lual_checkinteger(state, 1);
        if i < 0 {
            i = n as i64 + i;
        } else if i > n as i64 {
            i = n as i64;
        }
        (((1 <= i) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(
                state,
                1,
                b"index out of range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        return n - i as i32;
    };
}}
pub unsafe extern "C" fn finishpcall(
    state: *mut State,
    status: i32,
    extra: i64,
) -> i32 { unsafe {
    if ((status != 0 && status != 1) as i32 != 0) as i32 as i64 != 0 {
        (*state).push_boolean(false);
        lua_pushvalue(state, -(2));
        return 2;
    } else {
        return (*state).get_top() - extra as i32;
    };
}}
pub unsafe extern "C" fn luab_pcall(state: *mut State) -> i32 { unsafe {
    let status: i32;
    lual_checkany(state, 1);
    (*state).push_boolean(true);
    lua_rotate(state, 1, 1);
    status = lua_pcallk(
        state,
        (*state).get_top() - 2,
        -1,
        0,
        0,
        Some(finishpcall as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
    );
    return finishpcall(state, status, 0);
}}
pub unsafe extern "C" fn luab_xpcall(state: *mut State) -> i32 { unsafe {
    let status: i32;
    let n: i32 = (*state).get_top();
    lual_checktype(state, 2, 6);
    (*state).push_boolean(true);
    lua_pushvalue(state, 1);
    lua_rotate(state, 3, 2);
    status = lua_pcallk(
        state,
        n - 2,
        -1,
        2,
        2 as i64,
        Some(finishpcall as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
    );
    return finishpcall(state, status, 2 as i64);
}}
pub unsafe extern "C" fn luab_tostring(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    lual_tolstring(state, 1, std::ptr::null_mut());
    return 1;
}}
static mut BASE_FUNCTIONS: [RegisteredFunction; 26] = {
    [
        {
            RegisteredFunction {
                name: b"assert\0" as *const u8 as *const i8,
                function: Some(luab_assert as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"collectgarbage\0" as *const u8 as *const i8,
                function: Some(luab_collectgarbage as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"dofile\0" as *const u8 as *const i8,
                function: Some(luab_dofile as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"error\0" as *const u8 as *const i8,
                function: Some(luab_error as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(luab_getmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ipairs\0" as *const u8 as *const i8,
                function: Some(luab_ipairs as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"loadfile\0" as *const u8 as *const i8,
                function: Some(luab_loadfile as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"load\0" as *const u8 as *const i8,
                function: Some(luab_load as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"next\0" as *const u8 as *const i8,
                function: Some(luab_next as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pairs\0" as *const u8 as *const i8,
                function: Some(luab_pairs as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pcall\0" as *const u8 as *const i8,
                function: Some(luab_pcall as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"print\0" as *const u8 as *const i8,
                function: Some(luab_print as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"warn\0" as *const u8 as *const i8,
                function: Some(luab_warn as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawequal\0" as *const u8 as *const i8,
                function: Some(luab_rawequal as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawlen\0" as *const u8 as *const i8,
                function: Some(luab_rawlen as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawget\0" as *const u8 as *const i8,
                function: Some(luab_rawget as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawset\0" as *const u8 as *const i8,
                function: Some(luab_rawset as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"select\0" as *const u8 as *const i8,
                function: Some(luab_select as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(luab_setmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tonumber\0" as *const u8 as *const i8,
                function: Some(luab_tonumber as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tostring\0" as *const u8 as *const i8,
                function: Some(luab_tostring as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(luab_type as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"xpcall\0" as *const u8 as *const i8,
                function: Some(luab_xpcall as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"_G\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"_VERSION\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            }
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_base(state: *mut State) -> i32 { unsafe {
    lua_rawgeti(state, -(1000000 as i32) - 1000 as i32, 2 as i64);
    lual_setfuncs(state, BASE_FUNCTIONS.as_ptr(), 0);
    lua_pushvalue(state, -1);
    lua_setfield(state, -(2), b"_G\0" as *const u8 as *const i8);
    lua_pushstring(state, b"Lua 5.4\0" as *const u8 as *const i8);
    lua_setfield(state, -(2), b"_VERSION\0" as *const u8 as *const i8);
    return 1;
}}
pub unsafe extern "C" fn getco(state: *mut State) -> *mut State { unsafe {
    let co: *mut State = lua_tothread(state, 1);
    ((co != std::ptr::null_mut()) as i32 as i64 != 0
        || lual_typeerror(state, 1, b"thread\0" as *const u8 as *const i8) != 0) as i32;
    return co;
}}
pub unsafe extern "C" fn auxresume(
    state: *mut State,
    co: *mut State,
    narg: i32,
) -> i32 { unsafe {
    let status: i32;
    let mut nres: i32 = 0;
    if ((lua_checkstack(co, narg) == 0) as i32 != 0) as i32 as i64 != 0 {
        lua_pushstring(
            state,
            b"too many arguments to resume\0" as *const u8 as *const i8,
        );
        return -1;
    }
    lua_xmove(state, co, narg);
    status = lua_resume(co, state, narg, &mut nres);
    if ((status == 0 || status == 1) as i32 != 0) as i32 as i64 != 0 {
        if ((lua_checkstack(state, nres + 1) == 0) as i32 != 0) as i32 as i64 != 0 {
            lua_settop(co, -nres - 1);
            lua_pushstring(
                state,
                b"too many results to resume\0" as *const u8 as *const i8,
            );
            return -1;
        }
        lua_xmove(co, state, nres);
        return nres;
    } else {
        lua_xmove(co, state, 1);
        return -1;
    };
}}
pub unsafe extern "C" fn luab_auxwrap(state: *mut State) -> i32 { unsafe {
    let co: *mut State = lua_tothread(state, -(1000000 as i32) - 1000 as i32 - 1);
    let r: i32 = auxresume(state, co, (*state).get_top());
    if ((r < 0) as i32 != 0) as i32 as i64 != 0 {
        let mut stat: i32 = lua_status(co);
        if stat != 0 && stat != 1 {
            stat = lua_closethread(co, state);
            lua_xmove(co, state, 1);
        }
        if stat != 4 && lua_type(state, -1) == 4 {
            lual_where(state, 1);
            lua_rotate(state, -(2), 1);
            lua_concat(state, 2);
        }
        return lua_error(state);
    }
    return r;
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn checkfield(state: *mut State, key: *const i8, n: i32) -> i32 { unsafe {
    lua_pushstring(state, key);
    return (lua_rawget(state, -n) != 0) as i32;
}}
pub unsafe extern "C" fn checktab(state: *mut State, arg: i32, what: i32) { unsafe {
    if lua_type(state, arg) != 5 {
        let mut n: i32 = 1;
        if lua_getmetatable(state, arg) != 0
            && (what & 1 == 0 || {
                n += 1;
                checkfield(state, b"__index\0" as *const u8 as *const i8, n) != 0
            })
            && (what & 2 == 0 || {
                n += 1;
                checkfield(state, b"__newindex\0" as *const u8 as *const i8, n) != 0
            })
            && (what & 4 == 0 || {
                n += 1;
                checkfield(state, b"__len\0" as *const u8 as *const i8, n) != 0
            })
        {
            lua_settop(state, -n - 1);
        } else {
            lual_checktype(state, arg, 5);
        }
    }
}}
pub unsafe extern "C" fn tinsert(state: *mut State) -> i32 { unsafe {
    let pos: i64;
    checktab(state, 1, 1 | 2 | 4);
    let mut e: i64 = lual_len(state, 1);
    e = (e as u64).wrapping_add(1 as u64) as i64;
    match (*state).get_top() {
        2 => {
            pos = e;
        }
        3 => {
            let mut i: i64;
            pos = lual_checkinteger(state, 2);
            ((((pos as u64).wrapping_sub(1 as u32 as u64) < e as u64) as i32 != 0) as i32 as i64
                != 0
                || lual_argerror(
                    state,
                    2,
                    b"position out of bounds\0" as *const u8 as *const i8,
                ) != 0) as i32;
            i = e;
            while i > pos {
                lua_geti(state, 1, i - 1);
                lua_seti(state, 1, i);
                i -= 1;
            }
        }
        _ => {
            return lual_error(
                state,
                b"wrong number of arguments to 'insert'\0" as *const u8 as *const i8,
            );
        }
    }
    lua_seti(state, 1, pos);
    return 0;
}}
pub unsafe extern "C" fn tremove(state: *mut State) -> i32 { unsafe {
    checktab(state, 1, 1 | 2 | 4);
    let size: i64 = lual_len(state, 1);
    let mut pos: i64 = lual_optinteger(state, 2, size);
    if pos != size {
        ((((pos as u64).wrapping_sub(1 as u32 as u64) <= size as u64) as i32 != 0) as i32 as i64
            != 0
            || lual_argerror(
                state,
                2,
                b"position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
    }
    lua_geti(state, 1, pos);
    while pos < size {
        lua_geti(state, 1, pos + 1);
        lua_seti(state, 1, pos);
        pos += 1;
    }
    (*state).push_nil();
    lua_seti(state, 1, pos);
    return 1;
}}
pub unsafe extern "C" fn tmove(state: *mut State) -> i32 { unsafe {
    let f: i64 = lual_checkinteger(state, 2);
    let e: i64 = lual_checkinteger(state, 3);
    let t: i64 = lual_checkinteger(state, 4);
    let tag: i32 = if !(lua_type(state, 5) <= 0) {
        5
    } else {
        1
    };
    checktab(state, 1, 1);
    checktab(state, tag, 2);
    if e >= f {
        let n: i64;
        let mut i: i64;
        (((f > 0 || e < 9223372036854775807 as i64 + f) as i32 != 0) as i32 as i64
            != 0
            || lual_argerror(
                state,
                3,
                b"too many elements to move\0" as *const u8 as *const i8,
            ) != 0) as i32;
        n = e - f + 1;
        (((t <= 9223372036854775807 as i64 - n + 1) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(
                state,
                4,
                b"destination wrap around\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if t > e || t <= f || tag != 1 && lua_compare(state, 1, tag, 0) == 0 {
            i = 0;
            while i < n {
                lua_geti(state, 1, f + i);
                lua_seti(state, tag, t + i);
                i += 1;
            }
        } else {
            i = n - 1;
            while i >= 0 {
                lua_geti(state, 1, f + i);
                lua_seti(state, tag, t + i);
                i -= 1;
            }
        }
    }
    lua_pushvalue(state, tag);
    return 1;
}}
pub unsafe extern "C" fn addfield(state: *mut State, b: *mut Buffer, i: i64) { unsafe {
    lua_geti(state, 1, i);
    if !lua_isstring(state, -1) {
        lual_error(
            state,
            b"invalid value (%s) at index %I in table for 'concat'\0" as *const u8 as *const i8,
            lua_typename(state, lua_type(state, -1)),
            i,
        );
    }
    (*b).lual_addvalue();
}}
pub unsafe extern "C" fn tconcat(state: *mut State) -> i32 { unsafe {
    let mut b = Buffer::new();
    checktab(state, 1, 1 | 4);
    let mut last: i64 = lual_len(state, 1);
    let mut lsep: u64 = 0;
    let sep: *const i8 =
        lual_optlstring(state, 2, b"\0" as *const u8 as *const i8, &mut lsep);
    let mut i: i64 = lual_optinteger(state, 3, 1);
    last = lual_optinteger(state, 4, last);
    b.lual_buffinit(state);
    while i < last {
        addfield(state, &mut b, i);
        b.lual_addlstring(sep, lsep);
        i += 1;
    }
    if i == last {
        addfield(state, &mut b, i);
    }
    b.lual_pushresult();
    return 1;
}}
pub unsafe extern "C" fn tpack(state: *mut State) -> i32 { unsafe {
    let mut i: i32;
    let n: i32 = (*state).get_top();
    lua_createtable(state, n, 1);
    lua_rotate(state, 1, 1);
    i = n;
    while i >= 1 {
        lua_seti(state, 1, i as i64);
        i -= 1;
    }
    (*state).push_integer(n as i64);
    lua_setfield(state, 1, b"n\0" as *const u8 as *const i8);
    return 1;
}}
pub unsafe extern "C" fn tunpack(state: *mut State) -> i32 { unsafe {
    let mut n: u64;
    let mut i: i64 = lual_optinteger(state, 2, 1);
    let e: i64 = if lua_type(state, 3) <= 0 {
        lual_len(state, 1)
    } else {
        lual_checkinteger(state, 3)
    };
    if i > e {
        return 0;
    }
    n = (e as u64).wrapping_sub(i as u64);
    if ((n >= 2147483647 as i32 as u32 as u64 || {
        n = n.wrapping_add(1);
        lua_checkstack(state, n as i32) == 0
    }) as i32
        != 0) as i32 as i64
        != 0
    {
        return lual_error(
            state,
            b"too many results to unpack\0" as *const u8 as *const i8,
        );
    }
    while i < e {
        lua_geti(state, 1, i);
        i += 1;
    }
    lua_geti(state, 1, e);
    return n as i32;
}}
pub unsafe extern "C" fn l_randomizepivot() -> u32 { unsafe {
    let mut c: i64 = clock();
    let mut t: i64 = time(std::ptr::null_mut());
    let mut buffer: [u32; 4] = [0; 4];
    let mut i: u32;
    let mut rnd: u32 = 0u32;
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        &mut c as *mut i64 as *const libc::c_void,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_div(::core::mem::size_of::<u32>() as u64)
            .wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
    memcpy(
        buffer.as_mut_ptr().offset(
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_div(::core::mem::size_of::<u32>() as u64) as isize,
        ) as *mut libc::c_void,
        &mut t as *mut i64 as *const libc::c_void,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_div(::core::mem::size_of::<u32>() as u64)
            .wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
    i = 0u32;
    while (i as u64)
        < (::core::mem::size_of::<[u32; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<u32>() as u64)
    {
        rnd = rnd.wrapping_add(buffer[i as usize]);
        i = i.wrapping_add(1);
    }
    return rnd;
}}
pub unsafe extern "C" fn set2(state: *mut State, i: u32, j: u32) { unsafe {
    lua_seti(state, 1, i as i64);
    lua_seti(state, 1, j as i64);
}}
pub unsafe extern "C" fn sort_comp(state: *mut State, a: i32, b: i32) -> i32 { unsafe {
    if lua_type(state, 2) == 0 {
        return lua_compare(state, a, b, 1);
    } else {
        let res: i32;
        lua_pushvalue(state, 2);
        lua_pushvalue(state, a - 1);
        lua_pushvalue(state, b - 2);
        lua_callk(state, 2, 1, 0, None);
        res = lua_toboolean(state, -1);
        lua_settop(state, -1 - 1);
        return res;
    };
}}
pub unsafe extern "C" fn partition(state: *mut State, lo: u32, up: u32) -> u32 { unsafe {
    let mut i: u32 = lo;
    let mut j: u32 = up.wrapping_sub(1 as u32);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(state, 1, i as i64);
            if !(sort_comp(state, -1, -(2)) != 0) {
                break;
            }
            if ((i == up.wrapping_sub(1 as u32)) as i32 != 0) as i32 as i64 != 0 {
                lual_error(
                    state,
                    b"invalid order function for sorting\0" as *const u8 as *const i8,
                );
            }
            lua_settop(state, -1 - 1);
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(state, 1, j as i64);
            if !(sort_comp(state, -(3), -1) != 0) {
                break;
            }
            if ((j < i) as i32 != 0) as i32 as i64 != 0 {
                lual_error(
                    state,
                    b"invalid order function for sorting\0" as *const u8 as *const i8,
                );
            }
            lua_settop(state, -1 - 1);
        }
        if j < i {
            lua_settop(state, -1 - 1);
            set2(state, up.wrapping_sub(1 as u32), i);
            return i;
        }
        set2(state, i, j);
    }
}}
pub unsafe extern "C" fn choose_pivot(lo: u32, up: u32, rnd: u32) -> u32 {
    let r4: u32 = up.wrapping_sub(lo).wrapping_div(4 as u32);
    let p: u32 = rnd
        .wrapping_rem(r4.wrapping_mul(2 as u32))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
pub unsafe extern "C" fn auxsort(state: *mut State, mut lo: u32, mut up: u32, mut rnd: u32) { unsafe {
    while lo < up {
        let mut p: u32;
        let n: u32;
        lua_geti(state, 1, lo as i64);
        lua_geti(state, 1, up as i64);
        if sort_comp(state, -1, -(2)) != 0 {
            set2(state, lo, up);
        } else {
            lua_settop(state, -(2) - 1);
        }
        if up.wrapping_sub(lo) == 1 as u32 {
            return;
        }
        if up.wrapping_sub(lo) < 100 as u32 || rnd == 0u32 {
            p = lo.wrapping_add(up).wrapping_div(2 as u32);
        } else {
            p = choose_pivot(lo, up, rnd);
        }
        lua_geti(state, 1, p as i64);
        lua_geti(state, 1, lo as i64);
        if sort_comp(state, -(2), -1) != 0 {
            set2(state, p, lo);
        } else {
            lua_settop(state, -1 - 1);
            lua_geti(state, 1, up as i64);
            if sort_comp(state, -1, -(2)) != 0 {
                set2(state, p, up);
            } else {
                lua_settop(state, -(2) - 1);
            }
        }
        if up.wrapping_sub(lo) == 2 as u32 {
            return;
        }
        lua_geti(state, 1, p as i64);
        lua_pushvalue(state, -1);
        lua_geti(state, 1, up.wrapping_sub(1 as u32) as i64);
        set2(state, p, up.wrapping_sub(1 as u32));
        p = partition(state, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(state, lo, p.wrapping_sub(1 as u32), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1 as u32);
        } else {
            auxsort(state, p.wrapping_add(1 as u32), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1 as u32);
        }
        if up.wrapping_sub(lo).wrapping_div(128 as i32 as u32) > n {
            rnd = l_randomizepivot();
        }
    }
}}
pub unsafe extern "C" fn sort(state: *mut State) -> i32 { unsafe {
    checktab(state, 1, 1 | 2 | 4);
    let n: i64 = lual_len(state, 1);
    if n > 1 {
        (((n < 2147483647 as i32 as i64) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(state, 1, b"array too big\0" as *const u8 as *const i8) != 0)
            as i32;
        if !(lua_type(state, 2) <= 0) {
            lual_checktype(state, 2, 6);
        }
        lua_settop(state, 2);
        auxsort(state, 1 as u32, n as u32, 0u32);
    }
    return 0;
}}
static mut TABLE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            let init = RegisteredFunction {
                name: b"concat\0" as *const u8 as *const i8,
                function: Some(tconcat as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"insert\0" as *const u8 as *const i8,
                function: Some(tinsert as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(tpack as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(tunpack as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(tremove as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"move\0" as *const u8 as *const i8,
                function: Some(tmove as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"sort\0" as *const u8 as *const i8,
                function: Some(sort as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_table(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, TABLE_FUNCTIONS.as_ptr(), 0);
    return 1;
}}
pub unsafe extern "C" fn l_checkmode(mut mode: *const i8) -> i32 { unsafe {
    return (*mode as i32 != '\0' as i32
        && {
            let fresh151 = mode;
            mode = mode.offset(1);
            !(strchr(b"rwa\0" as *const u8 as *const i8, *fresh151 as i32)).is_null()
        }
        && (*mode as i32 != '+' as i32 || {
            mode = mode.offset(1);
            1 != 0
        })
        && strspn(mode, b"b\0" as *const u8 as *const i8) == strlen(mode)) as i32;
}}
pub unsafe extern "C" fn io_type(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream;
    lual_checkany(state, 1);
    p = lual_testudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    if p.is_null() {
        (*state).push_nil();
    } else if ((*p).closef).is_none() {
        lua_pushstring(state, b"closed file\0" as *const u8 as *const i8);
    } else {
        lua_pushstring(state, b"file\0" as *const u8 as *const i8);
    }
    return 1;
}}
pub unsafe extern "C" fn f_tostring(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    if ((*p).closef).is_none() {
        lua_pushstring(state, b"file (closed)\0" as *const u8 as *const i8);
    } else {
        lua_pushfstring(state, b"file (%p)\0" as *const u8 as *const i8, (*p).f);
    }
    return 1;
}}
pub unsafe extern "C" fn tofile(state: *mut State) -> *mut FILE { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    if (((*p).closef).is_none() as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            state,
            b"attempt to use a closed file\0" as *const u8 as *const i8,
        );
    }
    return (*p).f;
}}
pub unsafe extern "C" fn newprefile(state: *mut State) -> *mut Stream { unsafe {
    let p: *mut Stream =
        lua_newuserdatauv(state, ::core::mem::size_of::<Stream>() as u64, 0) as *mut Stream;
    (*p).closef = None;
    lual_setmetatable(state, b"FILE*\0" as *const u8 as *const i8);
    return p;
}}
pub unsafe extern "C" fn aux_close(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    let cf: CFunction = (*p).closef;
    (*p).closef = None;
    return (Some(cf.expect("non-null function pointer"))).expect("non-null function pointer")(
        state,
    );
}}
pub unsafe extern "C" fn f_close(state: *mut State) -> i32 { unsafe {
    tofile(state);
    return aux_close(state);
}}
pub unsafe extern "C" fn io_close(state: *mut State) -> i32 { unsafe {
    if lua_type(state, 1) == -1 {
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_IO_output\0" as *const u8 as *const i8,
        );
    }
    return f_close(state);
}}
pub unsafe extern "C" fn f_gc(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    if ((*p).closef).is_some() && !((*p).f).is_null() {
        aux_close(state);
    }
    return 0;
}}
pub unsafe extern "C" fn io_fclose(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    *__errno_location() = 0;
    return lual_fileresult(state, (fclose((*p).f) == 0) as i32, std::ptr::null());
}}
pub unsafe extern "C" fn newfile(state: *mut State) -> *mut Stream { unsafe {
    let p: *mut Stream = newprefile(state);
    (*p).f = std::ptr::null_mut();
    (*p).closef = Some(io_fclose as unsafe extern "C" fn(*mut State) -> i32);
    return p;
}}
pub unsafe extern "C" fn opencheck(
    state: *mut State,
    fname: *const i8,
    mode: *const i8,
) { unsafe {
    let p: *mut Stream = newfile(state);
    (*p).f = fopen(fname, mode);
    if (((*p).f == std::ptr::null_mut() as *mut FILE) as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            state,
            b"cannot open file '%s' (%s)\0" as *const u8 as *const i8,
            fname,
            strerror(*__errno_location()),
        );
    }
}}
pub unsafe extern "C" fn io_open(state: *mut State) -> i32 { unsafe {
    let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let mode: *const i8 =
        lual_optlstring(state, 2, b"r\0" as *const u8 as *const i8, std::ptr::null_mut());
    let p: *mut Stream = newfile(state);
    let md: *const i8 = mode;
    ((l_checkmode(md) != 0) as i32 as i64 != 0
        || lual_argerror(state, 2, b"invalid mode\0" as *const u8 as *const i8) != 0) as i32;
    *__errno_location() = 0;
    (*p).f = fopen(filename, mode);
    return if ((*p).f).is_null() {
        lual_fileresult(state, 0, filename)
    } else {
        1
    };
}}
pub unsafe extern "C" fn io_pclose(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    *__errno_location() = 0;
    return lual_execresult(state, pclose((*p).f));
}}
pub unsafe extern "C" fn io_popen(state: *mut State) -> i32 { unsafe {
    let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let mode: *const i8 =
        lual_optlstring(state, 2, b"r\0" as *const u8 as *const i8, std::ptr::null_mut());
    let p: *mut Stream = newprefile(state);
    ((((*mode.offset(0 as isize) as i32 == 'r' as i32
        || *mode.offset(0 as isize) as i32 == 'w' as i32)
        && *mode.offset(1 as isize) as i32 == '\0' as i32) as i32
        != 0) as i32 as i64
        != 0
        || lual_argerror(state, 2, b"invalid mode\0" as *const u8 as *const i8) != 0) as i32;
    *__errno_location() = 0;
    fflush(std::ptr::null_mut());
    (*p).f = popen(filename, mode);
    (*p).closef = Some(io_pclose as unsafe extern "C" fn(*mut State) -> i32);
    return if ((*p).f).is_null() {
        lual_fileresult(state, 0, filename)
    } else {
        1
    };
}}
pub unsafe extern "C" fn io_tmpfile(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream = newfile(state);
    *__errno_location() = 0;
    (*p).f = tmpfile();
    return if ((*p).f).is_null() {
        lual_fileresult(state, 0, std::ptr::null())
    } else {
        1
    };
}}
pub unsafe extern "C" fn getiofile(state: *mut State, findex: *const i8) -> *mut FILE { unsafe {
    let p: *mut Stream;
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, findex);
    p = lua_touserdata(state, -1) as *mut Stream;
    if (((*p).closef).is_none() as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            state,
            b"default %s file is closed\0" as *const u8 as *const i8,
            findex.offset(
                (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64) as isize,
            ),
        );
    }
    return (*p).f;
}}
pub unsafe extern "C" fn g_iofile(
    state: *mut State,
    f: *const i8,
    mode: *const i8,
) -> i32 { unsafe {
    if !(lua_type(state, 1) <= 0) {
        let filename: *const i8 = lua_tolstring(state, 1, std::ptr::null_mut());
        if !filename.is_null() {
            opencheck(state, filename, mode);
        } else {
            tofile(state);
            lua_pushvalue(state, 1);
        }
        lua_setfield(state, -(1000000 as i32) - 1000 as i32, f);
    }
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, f);
    return 1;
}}
pub unsafe extern "C" fn io_input(state: *mut State) -> i32 { unsafe {
    return g_iofile(
        state,
        b"_IO_input\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
    );
}}
pub unsafe extern "C" fn io_output(state: *mut State) -> i32 { unsafe {
    return g_iofile(
        state,
        b"_IO_output\0" as *const u8 as *const i8,
        b"w\0" as *const u8 as *const i8,
    );
}}
pub unsafe extern "C" fn aux_lines(state: *mut State, to_close: bool) { unsafe {
    let n: i32 = (*state).get_top() - 1;
    (((n <= 250 as i32) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            250 as i32 + 2,
            b"too many arguments\0" as *const u8 as *const i8,
        ) != 0) as i32;
    lua_pushvalue(state, 1);
    (*state).push_integer(n as i64);
    (*state).push_boolean(to_close);
    lua_rotate(state, 2, 3);
    lua_pushcclosure(
        state,
        Some(io_readline as unsafe extern "C" fn(*mut State) -> i32),
        3 + n,
    );
}}
pub unsafe extern "C" fn f_lines(state: *mut State) -> i32 { unsafe {
    tofile(state);
    aux_lines(state, false);
    return 1;
}}
pub unsafe extern "C" fn io_lines(state: *mut State) -> i32 { unsafe {
    let to_close: bool;
    if lua_type(state, 1) == -1 {
        (*state).push_nil();
    }
    if lua_type(state, 1) == 0 {
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_IO_input\0" as *const u8 as *const i8,
        );
        lua_copy(state, -1, 1);
        lua_settop(state, -1 - 1);
        tofile(state);
        to_close = false;
    } else {
        let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        opencheck(state, filename, b"r\0" as *const u8 as *const i8);
        lua_copy(state, -1, 1);
        lua_settop(state, -1 - 1);
        to_close = true;
    }
    aux_lines(state, to_close);
    if to_close {
        (*state).push_nil();
        (*state).push_nil();
        lua_pushvalue(state, 1);
        return 4;
    } else {
        return 1;
    };
}}
pub unsafe extern "C" fn nextc(rn: *mut RN) -> i32 { unsafe {
    if (((*rn).n >= 200 as i32) as i32 != 0) as i32 as i64 != 0 {
        (*rn).buffer[0] = '\0' as i8;
        return 0;
    } else {
        let fresh152 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buffer[fresh152 as usize] = (*rn).c as i8;
        (*rn).c = getc_unlocked((*rn).f);
        return 1;
    };
}}
pub unsafe extern "C" fn test2(rn: *mut RN, set: *const i8) -> i32 { unsafe {
    if (*rn).c == *set.offset(0 as isize) as i32 || (*rn).c == *set.offset(1 as isize) as i32
    {
        return nextc(rn);
    } else {
        return 0;
    };
}}
pub unsafe extern "C" fn readdigits(rn: *mut RN, hex: i32) -> i32 { unsafe {
    let mut count: i32 = 0;
    while (if hex != 0 {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as i32 & _ISXDIGIT as i32 as u16 as i32
    } else {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as i32 & _ISDIGIT as i32 as u16 as i32
    }) != 0
        && nextc(rn) != 0
    {
        count += 1;
    }
    return count;
}}
pub unsafe extern "C" fn read_number(state: *mut State, f: *mut FILE) -> i32 { unsafe {
    let mut rn: RN = RN {
        f: std::ptr::null_mut(),
        c: 0,
        n: 0,
        buffer: [0; 201],
    };
    let mut count: i32 = 0;
    let mut hex: i32 = 0;
    let mut decp: [i8; 2] = [0; 2];
    rn.f = f;
    rn.n = 0;
    decp[0] = '.' as i8;
    decp[1] = '.' as i8;
    flockfile(rn.f);
    loop {
        rn.c = getc_unlocked(rn.f);
        if !(*(*__ctype_b_loc()).offset(rn.c as isize) as i32 & _ISSPACE as i32 as u16 as i32 != 0)
        {
            break;
        }
    }
    test2(&mut rn, b"-+\0" as *const u8 as *const i8);
    if test2(&mut rn, b"00\0" as *const u8 as *const i8) != 0 {
        if test2(&mut rn, b"xX\0" as *const u8 as *const i8) != 0 {
            hex = 1;
        } else {
            count = 1;
        }
    }
    count += readdigits(&mut rn, hex);
    if test2(&mut rn, decp.as_mut_ptr()) != 0 {
        count += readdigits(&mut rn, hex);
    }
    if count > 0
        && test2(
            &mut rn,
            if hex != 0 {
                b"pP\0" as *const u8 as *const i8
            } else {
                b"eE\0" as *const u8 as *const i8
            },
        ) != 0
    {
        test2(&mut rn, b"-+\0" as *const u8 as *const i8);
        readdigits(&mut rn, 0);
    }
    ungetc(rn.c, rn.f);
    funlockfile(rn.f);
    rn.buffer[rn.n as usize] = '\0' as i8;
    if (lua_stringtonumber(state, (rn.buffer).as_mut_ptr()) != 0u64) as i32 as i64 != 0 {
        return 1;
    } else {
        (*state).push_nil();
        return 0;
    };
}}
pub unsafe extern "C" fn test_eof(state: *mut State, f: *mut FILE) -> i32 { unsafe {
    let c: i32 = getc(f);
    ungetc(c, f);
    lua_pushstring(state, b"\0" as *const u8 as *const i8);
    return (c != -1) as i32;
}}
pub unsafe extern "C" fn read_line(state: *mut State, f: *mut FILE, chop: i32) -> i32 { unsafe {
    let mut b = Buffer::new();
    let mut c: i32 = 0;
    b.lual_buffinit(state);
    loop {
        let buffer: *mut i8 = b.lual_prepbuffsize((16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as u64,
        );
        let mut i: i32 = 0;
        flockfile(f);
        while i
            < (16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
            && {
                c = getc_unlocked(f);
                c != -1
            }
            && c != '\n' as i32
        {
            let fresh153 = i;
            i = i + 1;
            *buffer.offset(fresh153 as isize) = c as i8;
        }
        funlockfile(f);
        b.length = (b.length as u64).wrapping_add(i as u64) as u64 as u64;
        if !(c != -1 && c != '\n' as i32) {
            break;
        }
    }
    if chop == 0 && c == '\n' as i32 {
        (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh154 = b.length;
        b.length = (b.length).wrapping_add(1);
        *(b.pointer).offset(fresh154 as isize) = c as i8;
    }
    b.lual_pushresult();
    return (c == '\n' as i32 || lua_rawlen(state, -1) > 0u64) as i32;
}}
pub unsafe extern "C" fn read_all(state: *mut State, f: *mut FILE) { unsafe {
    let mut nr: u64;
    let mut b = Buffer::new();
    b.lual_buffinit(state);
    loop {
        let p: *mut i8 = b.lual_prepbuffsize(
            (16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as u64,
        );
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            (16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as u64,
            f,
        );
        b.length = (b.length as u64).wrapping_add(nr) as u64 as u64;
        if !(nr
            == (16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as u64)
        {
            break;
        }
    }
    b.lual_pushresult();
}}
pub unsafe extern "C" fn read_chars(state: *mut State, f: *mut FILE, n: u64) -> i32 { unsafe {
    let nr: u64;
    let p: *mut i8;
    let mut b = Buffer::new();
    b.lual_buffinit(state);
    p = b.lual_prepbuffsize(n);
    nr = fread(
        p as *mut libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        n,
        f,
    );
    b.length = (b.length as u64).wrapping_add(nr) as u64 as u64;
    b.lual_pushresult();
    return (nr > 0u64) as i32;
}}
pub unsafe extern "C" fn g_read(state: *mut State, f: *mut FILE, first: i32) -> i32 { unsafe {
    let mut nargs: i32 = (*state).get_top() - 1;
    let mut n: i32;
    let mut success: i32;
    clearerr(f);
    *__errno_location() = 0;
    if nargs == 0 {
        success = read_line(state, f, 1);
        n = first + 1;
    } else {
        lual_checkstack(
            state,
            nargs + 20 as i32,
            b"too many arguments\0" as *const u8 as *const i8,
        );
        success = 1;
        n = first;
        loop {
            let fresh155 = nargs;
            nargs = nargs - 1;
            if !(fresh155 != 0 && success != 0) {
                break;
            }
            if lua_type(state, n) == 3 {
                let l: u64 = lual_checkinteger(state, n) as u64;
                success = if l == 0u64 {
                    test_eof(state, f)
                } else {
                    read_chars(state, f, l)
                };
            } else {
                let mut p: *const i8 = lual_checklstring(state, n, std::ptr::null_mut());
                if *p as i32 == '*' as i32 {
                    p = p.offset(1);
                }
                match *p as i32 {
                    110 => {
                        success = read_number(state, f);
                    }
                    108 => {
                        success = read_line(state, f, 1);
                    }
                    76 => {
                        success = read_line(state, f, 0);
                    }
                    97 => {
                        read_all(state, f);
                        success = 1;
                    }
                    _ => {
                        return lual_argerror(
                            state,
                            n,
                            b"invalid format\0" as *const u8 as *const i8,
                        );
                    }
                }
            }
            n += 1;
        }
    }
    if ferror(f) != 0 {
        return lual_fileresult(state, 0, std::ptr::null());
    }
    if success == 0 {
        lua_settop(state, -1 - 1);
        (*state).push_nil();
    }
    return n - first;
}}
pub unsafe extern "C" fn io_read(state: *mut State) -> i32 { unsafe {
    return g_read(
        state,
        getiofile(state, b"_IO_input\0" as *const u8 as *const i8),
        1,
    );
}}
pub unsafe extern "C" fn f_read(state: *mut State) -> i32 { unsafe {
    return g_read(state, tofile(state), 2);
}}
pub unsafe extern "C" fn io_readline(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 1) as *mut Stream;
    let mut i: i32;
    let mut n: i32 =
        lua_tointegerx(state, -(1000000 as i32) - 1000 as i32 - 2, std::ptr::null_mut()) as i32;
    if ((*p).closef).is_none() {
        return lual_error(state, b"file is already closed\0" as *const u8 as *const i8);
    }
    lua_settop(state, 1);
    lual_checkstack(state, n, b"too many arguments\0" as *const u8 as *const i8);
    i = 1;
    while i <= n {
        lua_pushvalue(state, -(1000000 as i32) - 1000 as i32 - (3 + i));
        i += 1;
    }
    n = g_read(state, (*p).f, 2);
    if lua_toboolean(state, -n) != 0 {
        return n;
    } else {
        if n > 1 {
            return lual_error(
                state,
                b"%s\0" as *const u8 as *const i8,
                lua_tolstring(state, -n + 1, std::ptr::null_mut()),
            );
        }
        if lua_toboolean(state, -(1000000 as i32) - 1000 as i32 - 3) != 0 {
            lua_settop(state, 0);
            lua_pushvalue(state, -(1000000 as i32) - 1000 as i32 - 1);
            aux_close(state);
        }
        return 0;
    };
}}
pub unsafe extern "C" fn g_write(state: *mut State, f: *mut FILE, mut arg: i32) -> i32 { unsafe {
    let mut nargs: i32 = (*state).get_top() - arg;
    let mut status: i32 = 1;
    *__errno_location() = 0;
    loop {
        let fresh156 = nargs;
        nargs = nargs - 1;
        if !(fresh156 != 0) {
            break;
        }
        if lua_type(state, arg) == 3 {
            let length: i32 = if lua_isinteger(state, arg) {
                fprintf(
                    f,
                    b"%lld\0" as *const u8 as *const i8,
                    lua_tointegerx(state, arg, std::ptr::null_mut()),
                )
            } else {
                fprintf(
                    f,
                    b"%.14g\0" as *const u8 as *const i8,
                    lua_tonumberx(state, arg, std::ptr::null_mut()),
                )
            };
            status = (status != 0 && length > 0) as i32;
        } else {
            let mut l: u64 = 0;
            let s: *const i8 = lual_checklstring(state, arg, &mut l);
            status = (status != 0
                && fwrite(
                    s as *const libc::c_void,
                    ::core::mem::size_of::<i8>() as u64,
                    l,
                    f,
                ) == l) as i32;
        }
        arg += 1;
    }
    if (status != 0) as i32 as i64 != 0 {
        return 1;
    } else {
        return lual_fileresult(state, status, std::ptr::null());
    };
}}
pub unsafe extern "C" fn io_write(state: *mut State) -> i32 { unsafe {
    return g_write(
        state,
        getiofile(state, b"_IO_output\0" as *const u8 as *const i8),
        1,
    );
}}
pub unsafe extern "C" fn f_write(state: *mut State) -> i32 { unsafe {
    let f: *mut FILE = tofile(state);
    lua_pushvalue(state, 1);
    return g_write(state, f, 2);
}}
pub unsafe extern "C" fn f_seek(state: *mut State) -> i32 { unsafe {
    static mut MODE: [i32; 3] = [0, 1, 2];
    static mut MODE_NAMES: [*const i8; 4] = [
        b"set\0" as *const u8 as *const i8,
        b"cur\0" as *const u8 as *const i8,
        b"end\0" as *const u8 as *const i8,
        std::ptr::null(),
    ];
    let f: *mut FILE = tofile(state);
    let mut op: i32 = lual_checkoption(
        state,
        2,
        b"cur\0" as *const u8 as *const i8,
        MODE_NAMES.as_ptr(),
    );
    let p3: i64 = lual_optinteger(state, 3, 0);
    let offset: i64 = p3 as i64;
    (((offset as i64 == p3) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            3,
            b"not an integer in proper range\0" as *const u8 as *const i8,
        ) != 0) as i32;
    *__errno_location() = 0;
    op = fseeko(f, offset, MODE[op as usize]);
    if (op != 0) as i32 as i64 != 0 {
        return lual_fileresult(state, 0, std::ptr::null());
    } else {
        (*state).push_integer(ftello(f) as i64);
        return 1;
    };
}}
pub unsafe extern "C" fn f_setvbuf(state: *mut State) -> i32 { unsafe {
    static mut MODE: [i32; 3] = [2, 0, 1];
    static mut MODE_NAMES: [*const i8; 4] = [
        b"no\0" as *const u8 as *const i8,
        b"full\0" as *const u8 as *const i8,
        b"line\0" as *const u8 as *const i8,
        std::ptr::null(),
    ];
    let f: *mut FILE = tofile(state);
    let op: i32 = lual_checkoption(state, 2, std::ptr::null(), MODE_NAMES.as_ptr());
    let size: i64 = lual_optinteger(
        state,
        3,
        (16 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
            .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32 as i64,
    );
    let res: i32;
    *__errno_location() = 0;
    res = setvbuf(f, std::ptr::null_mut(), MODE[op as usize], size as u64);
    return lual_fileresult(state, (res == 0) as i32, std::ptr::null());
}}
pub unsafe extern "C" fn io_flush(state: *mut State) -> i32 { unsafe {
    let f: *mut FILE = getiofile(state, b"_IO_output\0" as *const u8 as *const i8);
    *__errno_location() = 0;
    return lual_fileresult(state, (fflush(f) == 0) as i32, std::ptr::null());
}}
pub unsafe extern "C" fn f_flush(state: *mut State) -> i32 { unsafe {
    let f: *mut FILE = tofile(state);
    *__errno_location() = 0;
    return lual_fileresult(state, (fflush(f) == 0) as i32, std::ptr::null());
}}
static mut IO_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            let init = RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(io_close as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(io_flush as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"input\0" as *const u8 as *const i8,
                function: Some(io_input as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(io_lines as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"open\0" as *const u8 as *const i8,
                function: Some(io_open as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"output\0" as *const u8 as *const i8,
                function: Some(io_output as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"popen\0" as *const u8 as *const i8,
                function: Some(io_popen as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"read\0" as *const u8 as *const i8,
                function: Some(io_read as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"tmpfile\0" as *const u8 as *const i8,
                function: Some(io_tmpfile as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(io_type as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(io_write as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
static mut IO_METHODS: [RegisteredFunction; 8] = {
    [
        {
            let init = RegisteredFunction {
                name: b"read\0" as *const u8 as *const i8,
                function: Some(f_read as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(f_write as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(f_lines as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(f_flush as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"seek\0" as *const u8 as *const i8,
                function: Some(f_seek as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(f_close as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setvbuf\0" as *const u8 as *const i8,
                function: Some(f_setvbuf as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
static mut IO_METAMETHODS: [RegisteredFunction; 5] = {
    [
        {
            let init = RegisteredFunction {
                name: b"__index\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__gc\0" as *const u8 as *const i8,
                function: Some(f_gc as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__close\0" as *const u8 as *const i8,
                function: Some(f_gc as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__tostring\0" as *const u8 as *const i8,
                function: Some(f_tostring as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn createmeta(state: *mut State) { unsafe {
    lual_newmetatable(state, b"FILE*\0" as *const u8 as *const i8);
    lual_setfuncs(state, IO_METAMETHODS.as_ptr(), 0);
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, IO_METHODS.as_ptr(), 0);
    lua_setfield(state, -(2), b"__index\0" as *const u8 as *const i8);
    lua_settop(state, -1 - 1);
}}
pub unsafe extern "C" fn io_noclose(state: *mut State) -> i32 { unsafe {
    let p: *mut Stream =
        lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
    (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
    (*state).push_nil();
    lua_pushstring(
        state,
        b"cannot close standard file\0" as *const u8 as *const i8,
    );
    return 2;
}}
pub unsafe extern "C" fn createstdfile(
    state: *mut State,
    f: *mut FILE,
    k: *const i8,
    fname: *const i8,
) { unsafe {
    let p: *mut Stream = newprefile(state);
    (*p).f = f;
    (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
    if !k.is_null() {
        lua_pushvalue(state, -1);
        lua_setfield(state, -(1000000 as i32) - 1000 as i32, k);
    }
    lua_setfield(state, -(2), fname);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_io(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, IO_FUNCTIONS.as_ptr(), 0);
    createmeta(state);
    createstdfile(
        state,
        stdin,
        b"_IO_input\0" as *const u8 as *const i8,
        b"stdin\0" as *const u8 as *const i8,
    );
    createstdfile(
        state,
        stdout,
        b"_IO_output\0" as *const u8 as *const i8,
        b"stdout\0" as *const u8 as *const i8,
    );
    createstdfile(
        state,
        stderr,
        std::ptr::null(),
        b"stderr\0" as *const u8 as *const i8,
    );
    return 1;
}}
pub unsafe extern "C" fn os_execute(state: *mut State) -> i32 { unsafe {
    let cmd: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
    let stat: i32;
    *__errno_location() = 0;
    stat = system(cmd);
    if !cmd.is_null() {
        return lual_execresult(state, stat);
    } else {
        (*state).push_boolean(0 != stat);
        return 1;
    };
}}
pub unsafe extern "C" fn os_remove(state: *mut State) -> i32 { unsafe {
    let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    *__errno_location() = 0;
    return lual_fileresult(state, (remove(filename) == 0) as i32, filename);
}}
pub unsafe extern "C" fn os_rename(state: *mut State) -> i32 { unsafe {
    let fromname: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let toname: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
    *__errno_location() = 0;
    return lual_fileresult(
        state,
        (rename(fromname, toname) == 0) as i32,
        std::ptr::null(),
    );
}}
pub unsafe extern "C" fn os_tmpname(state: *mut State) -> i32 { unsafe {
    let mut buffer: [i8; 32] = [0; 32];
    let mut err: i32;
    strcpy(
        buffer.as_mut_ptr(),
        b"/tmp/lua_XXXXXX\0" as *const u8 as *const i8,
    );
    err = mkstemp(buffer.as_mut_ptr());
    if err != -1 {
        close(err);
    }
    err = (err == -1) as i32;
    if (err != 0) as i32 as i64 != 0 {
        return lual_error(
            state,
            b"unable to generate a unique filename\0" as *const u8 as *const i8,
        );
    }
    lua_pushstring(state, buffer.as_mut_ptr());
    return 1;
}}
pub unsafe extern "C" fn os_getenv(state: *mut State) -> i32 { unsafe {
    lua_pushstring(state, getenv(lual_checklstring(state, 1, std::ptr::null_mut())));
    return 1;
}}
pub unsafe extern "C" fn os_clock(state: *mut State) -> i32 { unsafe {
    (*state).push_number(clock() as f64 / 1000000 as i32 as i64 as f64);
    return 1;
}}
pub unsafe extern "C" fn setfield(
    state: *mut State,
    key: *const i8,
    value: i32,
    delta: i32,
) { unsafe {
    (*state).push_integer(value as i64 + delta as i64);
    lua_setfield(state, -(2), key);
}}
pub unsafe extern "C" fn setboolfield(state: *mut State, key: *const i8, value: bool) { unsafe {
    (*state).push_boolean(value);
    lua_setfield(state, -(2), key);
}}
pub unsafe extern "C" fn setallfields(state: *mut State, stm: *mut TM) { unsafe {
    setfield(
        state,
        b"year\0" as *const u8 as *const i8,
        (*stm).tm_year,
        1900 as i32,
    );
    setfield(
        state,
        b"month\0" as *const u8 as *const i8,
        (*stm).tm_mon,
        1,
    );
    setfield(state, b"day\0" as *const u8 as *const i8, (*stm).tm_mday, 0);
    setfield(
        state,
        b"hour\0" as *const u8 as *const i8,
        (*stm).tm_hour,
        0,
    );
    setfield(state, b"min\0" as *const u8 as *const i8, (*stm).tm_min, 0);
    setfield(state, b"sec\0" as *const u8 as *const i8, (*stm).tm_sec, 0);
    setfield(
        state,
        b"yday\0" as *const u8 as *const i8,
        (*stm).tm_yday,
        1,
    );
    setfield(
        state,
        b"wday\0" as *const u8 as *const i8,
        (*stm).tm_wday,
        1,
    );
    setboolfield(state, b"isdst\0" as *const u8 as *const i8, 0 != (*stm).tm_isdst);
}}
pub unsafe extern "C" fn getboolfield(state: *mut State, key: *const i8) -> i32 { unsafe {
    let res: i32;
    res = if lua_getfield(state, -1, key) == 0 {
        -1
    } else {
        lua_toboolean(state, -1)
    };
    lua_settop(state, -1 - 1);
    return res;
}}
pub unsafe extern "C" fn getfield(
    state: *mut State,
    key: *const i8,
    d: i32,
    delta: i32,
) -> i32 { unsafe {
    let mut is_number: bool = false;
    let t: i32 = lua_getfield(state, -1, key);
    let mut res: i64 = lua_tointegerx(state, -1, &mut is_number);
    if !is_number {
        if ((t != 0) as i32 != 0) as i32 as i64 != 0 {
            return lual_error(
                state,
                b"field '%s' is not an integer\0" as *const u8 as *const i8,
                key,
            );
        } else if ((d < 0) as i32 != 0) as i32 as i64 != 0 {
            return lual_error(
                state,
                b"field '%s' missing in date table\0" as *const u8 as *const i8,
                key,
            );
        }
        res = d as i64;
    } else {
        if if res >= 0 {
            (res - delta as i64 <= 2147483647 as i32 as i64) as i32
        } else {
            ((-(2147483647 as i32) - 1 + delta) as i64 <= res) as i32
        } == 0
        {
            return lual_error(
                state,
                b"field '%s' is out-of-bound\0" as *const u8 as *const i8,
                key,
            );
        }
        res -= delta as i64;
    }
    lua_settop(state, -1 - 1);
    return res as i32;
}}
pub unsafe extern "C" fn checkoption(
    state: *mut State,
    conv: *const i8,
    convlen: i64,
    buffer: *mut i8,
) -> *const i8 { unsafe {
    let mut option: *const i8 =
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
            as *const u8 as *const i8;
    let mut oplen: i32 = 1;
    while *option as i32 != '\0' as i32 && oplen as i64 <= convlen {
        if *option as i32 == '|' as i32 {
            oplen += 1;
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as u64,
        ) == 0
        {
            memcpy(
                buffer as *mut libc::c_void,
                conv as *const libc::c_void,
                oplen as u64,
            );
            *buffer.offset(oplen as isize) = '\0' as i8;
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize);
    }
    lual_argerror(
        state,
        1,
        lua_pushfstring(
            state,
            b"invalid conversion specifier '%%%s'\0" as *const u8 as *const i8,
            conv,
        ),
    );
    return conv;
}}
pub unsafe extern "C" fn l_checktime(state: *mut State, arg: i32) -> i64 { unsafe {
    let t: i64 = lual_checkinteger(state, arg);
    (((t as i64 as i64 == t) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            arg,
            b"time out-of-bounds\0" as *const u8 as *const i8,
        ) != 0) as i32;
    return t as i64;
}}
pub unsafe extern "C" fn os_date(state: *mut State) -> i32 { unsafe {
    let mut slen: u64 = 0;
    let mut s: *const i8 =
        lual_optlstring(state, 1, b"%c\0" as *const u8 as *const i8, &mut slen);
    let mut t: i64 = if lua_type(state, 2) <= 0 {
        time(std::ptr::null_mut())
    } else {
        l_checktime(state, 2)
    };
    let se: *const i8 = s.offset(slen as isize);
    let mut tmr: TM = TM {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: std::ptr::null(),
    };
    let stm: *mut TM;
    if *s as i32 == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1);
    } else {
        stm = localtime_r(&mut t, &mut tmr);
    }
    if stm.is_null() {
        return lual_error(
            state,
            b"date result cannot be represented in this installation\0" as *const u8 as *const i8,
        );
    }
    if strcmp(s, b"*t\0" as *const u8 as *const i8) == 0 {
        lua_createtable(state, 0, 9 as i32);
        setallfields(state, stm);
    } else {
        let mut cc: [i8; 4] = [0; 4];
        let mut b = Buffer::new();
        cc[0] = '%' as i8;
        b.lual_buffinit(state);
        while s < se {
            if *s as i32 != '%' as i32 {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh157 = s;
                s = s.offset(1);
                let fresh158 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh158 as isize) = *fresh157;
            } else {
                let reslen: u64;
                let buffer: *mut i8 = b.lual_prepbuffsize(250 as i32 as u64);
                s = s.offset(1);
                s = checkoption(
                    state,
                    s,
                    se.offset_from(s) as i64,
                    cc.as_mut_ptr().offset(1 as isize),
                );
                reslen = strftime(buffer, 250 as i32 as u64, cc.as_mut_ptr(), stm);
                b.length = (b.length as u64).wrapping_add(reslen) as u64 as u64;
            }
        }
        b.lual_pushresult();
    }
    return 1;
}}
pub unsafe extern "C" fn os_time(state: *mut State) -> i32 { unsafe {
    let t: i64;
    if lua_type(state, 1) <= 0 {
        t = time(std::ptr::null_mut());
    } else {
        let mut ts: TM = TM {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: std::ptr::null(),
        };
        lual_checktype(state, 1, 5);
        lua_settop(state, 1);
        ts.tm_year = getfield(state, b"year\0" as *const u8 as *const i8, -1, 1900 as i32);
        ts.tm_mon = getfield(state, b"month\0" as *const u8 as *const i8, -1, 1);
        ts.tm_mday = getfield(state, b"day\0" as *const u8 as *const i8, -1, 0);
        ts.tm_hour = getfield(state, b"hour\0" as *const u8 as *const i8, 12 as i32, 0);
        ts.tm_min = getfield(state, b"min\0" as *const u8 as *const i8, 0, 0);
        ts.tm_sec = getfield(state, b"sec\0" as *const u8 as *const i8, 0, 0);
        ts.tm_isdst = getboolfield(state, b"isdst\0" as *const u8 as *const i8);
        t = mktime(&mut ts);
        setallfields(state, &mut ts);
    }
    if t != t as i64 as i64 || t == -1 as i64 {
        return lual_error(
            state,
            b"time result cannot be represented in this installation\0" as *const u8 as *const i8,
        );
    }
    (*state).push_integer(t as i64);
    return 1;
}}
pub unsafe extern "C" fn os_difftime(state: *mut State) -> i32 { unsafe {
    let t1: i64 = l_checktime(state, 1);
    let t2: i64 = l_checktime(state, 2);
    (*state).push_number(difftime(t1, t2));
    return 1;
}}
pub unsafe extern "C" fn os_setlocale(state: *mut State) -> i32 { unsafe {
    static mut CATEGORY: [i32; 6] = [6, 3, 0, 4, 1, 2];
    static mut CATEGORY_NAMES: [*const i8; 7] = [
        b"all\0" as *const u8 as *const i8,
        b"collate\0" as *const u8 as *const i8,
        b"ctype\0" as *const u8 as *const i8,
        b"monetary\0" as *const u8 as *const i8,
        b"numeric\0" as *const u8 as *const i8,
        b"time\0" as *const u8 as *const i8,
        std::ptr::null(),
    ];
    let l: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
    let op: i32 = lual_checkoption(
        state,
        2,
        b"all\0" as *const u8 as *const i8,
        CATEGORY_NAMES.as_ptr(),
    );
    lua_pushstring(state, setlocale(CATEGORY[op as usize], l));
    return 1;
}}
pub unsafe extern "C" fn os_exit(state: *mut State) -> i32 { unsafe {
    let status: i32;
    if lua_type(state, 1) == 1 {
        status = if lua_toboolean(state, 1) != 0 {
            0
        } else {
            1
        };
    } else {
        status = lual_optinteger(state, 1, 0) as i32;
    }
    if lua_toboolean(state, 2) != 0 {
        lua_close(state);
    }
    if !state.is_null() {
        exit(status);
    }
    return 0;
}}
static mut SYSTEM_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            let init = RegisteredFunction {
                name: b"clock\0" as *const u8 as *const i8,
                function: Some(os_clock as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"date\0" as *const u8 as *const i8,
                function: Some(os_date as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"difftime\0" as *const u8 as *const i8,
                function: Some(os_difftime as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"execute\0" as *const u8 as *const i8,
                function: Some(os_execute as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"exit\0" as *const u8 as *const i8,
                function: Some(os_exit as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getenv\0" as *const u8 as *const i8,
                function: Some(os_getenv as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(os_remove as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"rename\0" as *const u8 as *const i8,
                function: Some(os_rename as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setlocale\0" as *const u8 as *const i8,
                function: Some(os_setlocale as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"time\0" as *const u8 as *const i8,
                function: Some(os_time as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"tmpname\0" as *const u8 as *const i8,
                function: Some(os_tmpname as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_os(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, SYSTEM_FUNCTIONS.as_ptr(), 0);
    return 1;
}}
pub unsafe extern "C" fn str_len(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    lual_checklstring(state, 1, &mut l);
    (*state).push_integer(l as i64);
    return 1;
}}
pub unsafe extern "C" fn posrelati(pos: i64, length: u64) -> u64 {
    if pos > 0 {
        return pos as u64;
    } else if pos == 0 {
        return 1 as u64;
    } else if pos < -(length as i64) {
        return 1 as u64;
    } else {
        return length.wrapping_add(pos as u64).wrapping_add(1 as u64);
    };
}
pub unsafe extern "C" fn getendpos(
    state: *mut State,
    arg: i32,
    def: i64,
    length: u64,
) -> u64 { unsafe {
    let pos: i64 = lual_optinteger(state, arg, def);
    if pos > length as i64 {
        return length;
    } else if pos >= 0 {
        return pos as u64;
    } else if pos < -(length as i64) {
        return 0u64;
    } else {
        return length.wrapping_add(pos as u64).wrapping_add(1 as u64);
    };
}}
pub unsafe extern "C" fn str_sub(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let start: u64 = posrelati(lual_checkinteger(state, 2), l);
    let end: u64 = getendpos(state, 3, -1 as i64, l);
    if start <= end {
        lua_pushlstring(
            state,
            s.offset(start as isize).offset(-(1 as isize)),
            end.wrapping_sub(start).wrapping_add(1 as u64),
        );
    } else {
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
    }
    return 1;
}}
pub unsafe extern "C" fn str_reverse(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let mut i: u64;
    let mut b = Buffer::new();
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let p: *mut i8 = b.lual_buffinitsize(state, l);
    i = 0;
    while i < l {
        *p.offset(i as isize) = *s.offset(l.wrapping_sub(i).wrapping_sub(1 as u64) as isize);
        i = i.wrapping_add(1);
    }
    b.lual_pushresultsize(l);
    return 1;
}}
pub unsafe extern "C" fn str_lower(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let mut i: u64;
    let mut b = Buffer::new();
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let p: *mut i8 = b.lual_buffinitsize(state, l);
    i = 0;
    while i < l {
        *p.offset(i as isize) = tolower(*s.offset(i as isize) as u8 as i32) as i8;
        i = i.wrapping_add(1);
    }
    b.lual_pushresultsize(l);
    return 1;
}}
pub unsafe extern "C" fn str_upper(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let mut i: u64;
    let mut b = Buffer::new();
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let p: *mut i8 = b.lual_buffinitsize(state, l);
    i = 0;
    while i < l {
        *p.offset(i as isize) = toupper(*s.offset(i as isize) as u8 as i32) as i8;
        i = i.wrapping_add(1);
    }
    b.lual_pushresultsize(l);
    return 1;
}}
pub unsafe extern "C" fn str_rep(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let mut lsep: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let mut n: i64 = lual_checkinteger(state, 2);
    let sep: *const i8 =
        lual_optlstring(state, 3, b"\0" as *const u8 as *const i8, &mut lsep);
    if n <= 0 {
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
    } else if ((l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as u64
            > ((if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                2147483647 as i32 as u64
            }) as u64)
                .wrapping_div(n as u64)) as i32
        != 0) as i32 as i64
        != 0
    {
        return lual_error(
            state,
            b"resulting string too large\0" as *const u8 as *const i8,
        );
    } else {
        let totallen: u64 = (n as u64)
            .wrapping_mul(l)
            .wrapping_add(((n - 1) as u64).wrapping_mul(lsep));
        let mut b = Buffer::new();
        let mut p: *mut i8 = b.lual_buffinitsize(state, totallen);
        loop {
            let fresh159 = n;
            n = n - 1;
            if !(fresh159 > 1) {
                break;
            }
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            p = p.offset(l as isize);
            if lsep > 0u64 {
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    lsep.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                p = p.offset(lsep as isize);
            }
        }
        memcpy(
            p as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        b.lual_pushresultsize(totallen);
    }
    return 1;
}}
pub unsafe extern "C" fn str_byte(state: *mut State) -> i32 { unsafe {
    let mut l: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut l);
    let pi: i64 = lual_optinteger(state, 2, 1);
    let posi: u64 = posrelati(pi, l);
    let pose: u64 = getendpos(state, 3, pi, l);
    let n: i32;
    let mut i: i32;
    if posi > pose {
        return 0;
    }
    if ((pose.wrapping_sub(posi) >= 2147483647 as i32 as u64) as i32 != 0) as i32 as i64 != 0 {
        return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
    }
    n = pose.wrapping_sub(posi) as i32 + 1;
    lual_checkstack(
        state,
        n,
        b"string slice too long\0" as *const u8 as *const i8,
    );
    i = 0;
    while i < n {
        (*state).push_integer(
            *s.offset(posi.wrapping_add(i as u64).wrapping_sub(1 as u64) as isize) as u8 as i64,
        );
        i += 1;
    }
    return n;
}}
pub unsafe extern "C" fn str_char(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    let mut i: i32;
    let mut b = Buffer::new();
    let p: *mut i8 = b.lual_buffinitsize(state, n as u64);
    i = 1;
    while i <= n {
        let c: u64 = lual_checkinteger(state, i) as u64;
        (((c <= (127 as i32 * 2 + 1) as u64) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(state, i, b"value out of range\0" as *const u8 as *const i8) != 0)
            as i32;
        *p.offset((i - 1) as isize) = c as u8 as i8;
        i += 1;
    }
    b.lual_pushresultsize(n as u64);
    return 1;
}}
pub unsafe extern "C" fn writer(
    state: *mut State,
    b: *const libc::c_void,
    size: u64,
    ud: *mut libc::c_void,
) -> i32 { unsafe {
    let stream_writer: *mut StreamWriter = ud as *mut StreamWriter;
    if (*stream_writer).init == 0 {
        (*stream_writer).init = 1;
        (*stream_writer).buffer.lual_buffinit(state);
    }
    (*stream_writer).buffer.lual_addlstring(b as *const i8, size);
    return 0;
}}
pub unsafe extern "C" fn str_dump(state: *mut State) -> i32 { unsafe {
    let mut stream_writer: StreamWriter = StreamWriter {
        init: 0,
        buffer: Buffer::new(),
    };
    let is_strip = 0 != lua_toboolean(state, 2);
    lual_checktype(state, 1, 6);
    lua_settop(state, 1);
    stream_writer.init = 0;
    if ((lua_dump(
        state,
        Some(
            writer
                as unsafe extern "C" fn(
                    *mut State,
                    *const libc::c_void,
                    u64,
                    *mut libc::c_void,
                ) -> i32,
        ),
        &mut stream_writer as *mut StreamWriter as *mut libc::c_void,
        is_strip,
    ) != 0) as i32
        != 0) as i32 as i64
        != 0
    {
        return lual_error(
            state,
            b"unable to dump given function\0" as *const u8 as *const i8,
        );
    }
    stream_writer.buffer.lual_pushresult();
    return 1;
}}
pub unsafe extern "C" fn tonum(state: *mut State, arg: i32) -> i32 { unsafe {
    if lua_type(state, arg) == 3 {
        lua_pushvalue(state, arg);
        return 1;
    } else {
        let mut length: u64 = 0;
        let s: *const i8 = lua_tolstring(state, arg, &mut length);
        return (!s.is_null() && lua_stringtonumber(state, s) == length.wrapping_add(1 as u64))
            as i32;
    };
}}
pub unsafe extern "C" fn trymt(state: *mut State, mtname: *const i8) { unsafe {
    lua_settop(state, 2);
    if ((lua_type(state, 2) == 4 || lual_getmetafield(state, 2, mtname) == 0) as i32
        != 0) as i32 as i64
        != 0
    {
        lual_error(
            state,
            b"attempt to %s a '%s' with a '%s'\0" as *const u8 as *const i8,
            mtname.offset(2 as isize),
            lua_typename(state, lua_type(state, -(2))),
            lua_typename(state, lua_type(state, -1)),
        );
    }
    lua_rotate(state, -(3), 1);
    lua_callk(state, 2, 1, 0, None);
}}
pub unsafe extern "C" fn arith(state: *mut State, op: i32, mtname: *const i8) -> i32 { unsafe {
    if tonum(state, 1) != 0 && tonum(state, 2) != 0 {
        lua_arith(state, op);
    } else {
        trymt(state, mtname);
    }
    return 1;
}}
pub unsafe extern "C" fn arith_add(state: *mut State) -> i32 { unsafe {
    return arith(state, 0, b"__add\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_sub(state: *mut State) -> i32 { unsafe {
    return arith(state, 1, b"__sub\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_mul(state: *mut State) -> i32 { unsafe {
    return arith(state, 2, b"__mul\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_mod(state: *mut State) -> i32 { unsafe {
    return arith(state, 3, b"__mod\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_pow(state: *mut State) -> i32 { unsafe {
    return arith(state, 4, b"__pow\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_div(state: *mut State) -> i32 { unsafe {
    return arith(state, 5, b"__div\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_idiv(state: *mut State) -> i32 { unsafe {
    return arith(state, 6, b"__idiv\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn arith_unm(state: *mut State) -> i32 { unsafe {
    return arith(state, 12 as i32, b"__unm\0" as *const u8 as *const i8);
}}
static mut STRING_METAMETHODS: [RegisteredFunction; 10] = {
    [
        {
            let init = RegisteredFunction {
                name: b"__add\0" as *const u8 as *const i8,
                function: Some(arith_add as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__sub\0" as *const u8 as *const i8,
                function: Some(arith_sub as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__mul\0" as *const u8 as *const i8,
                function: Some(arith_mul as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__mod\0" as *const u8 as *const i8,
                function: Some(arith_mod as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__pow\0" as *const u8 as *const i8,
                function: Some(arith_pow as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__div\0" as *const u8 as *const i8,
                function: Some(arith_div as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__idiv\0" as *const u8 as *const i8,
                function: Some(arith_idiv as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__unm\0" as *const u8 as *const i8,
                function: Some(arith_unm as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"__index\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn check_capture(ms: *mut MatchState, mut l: i32) -> i32 { unsafe {
    l -= '1' as i32;
    if ((l < 0 || l >= (*ms).level as i32 || (*ms).capture[l as usize].length == -1 as i64) as i32
        != 0) as i32 as i64
        != 0
    {
        return lual_error(
            (*ms).state,
            b"invalid capture index %%%d\0" as *const u8 as *const i8,
            l + 1,
        );
    }
    return l;
}}
pub unsafe extern "C" fn capture_to_close(ms: *mut MatchState) -> i32 { unsafe {
    let mut level: i32 = (*ms).level as i32;
    level -= 1;
    while level >= 0 {
        if (*ms).capture[level as usize].length == -1 as i64 {
            return level;
        }
        level -= 1;
    }
    return lual_error(
        (*ms).state,
        b"invalid pattern capture\0" as *const u8 as *const i8,
    );
}}
pub unsafe extern "C" fn classend(ms: *mut MatchState, mut p: *const i8) -> *const i8 { unsafe {
    let fresh160 = p;
    p = p.offset(1);
    match *fresh160 as i32 {
        37 => {
            if ((p == (*ms).p_end) as i32 != 0) as i32 as i64 != 0 {
                lual_error(
                    (*ms).state,
                    b"malformed pattern (ends with '%%')\0" as *const u8 as *const i8,
                );
            }
            return p.offset(1 as isize);
        }
        91 => {
            if *p as i32 == '^' as i32 {
                p = p.offset(1);
            }
            loop {
                if ((p == (*ms).p_end) as i32 != 0) as i32 as i64 != 0 {
                    lual_error(
                        (*ms).state,
                        b"malformed pattern (missing ']')\0" as *const u8 as *const i8,
                    );
                }
                let fresh161 = p;
                p = p.offset(1);
                if *fresh161 as i32 == '%' as i32 && p < (*ms).p_end {
                    p = p.offset(1);
                }
                if !(*p as i32 != ']' as i32) {
                    break;
                }
            }
            return p.offset(1 as isize);
        }
        _ => return p,
    };
}}
pub unsafe extern "C" fn match_class(c: i32, cl: i32) -> i32 { unsafe {
    let res: i32;
    match tolower(cl) {
        97 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISALPHA as i32 as u16 as i32;
        }
        99 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISCONTROL as i32 as u16 as i32;
        }
        100 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISDIGIT as i32 as u16 as i32;
        }
        103 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISGRAPH as i32 as u16 as i32;
        }
        108 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISLOWER as i32 as u16 as i32;
        }
        112 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISPUNCTUATION as i32 as u16 as i32;
        }
        115 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISSPACE as i32 as u16 as i32;
        }
        117 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISUPPER as i32 as u16 as i32;
        }
        119 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISALPHANUMERIC as i32 as u16 as i32;
        }
        120 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISXDIGIT as i32 as u16 as i32;
        }
        122 => {
            res = (c == 0) as i32;
        }
        _ => return (cl == c) as i32,
    }
    return if *(*__ctype_b_loc()).offset(cl as isize) as i32 & _ISLOWER as i32 as u16 as i32 != 0 {
        res
    } else {
        (res == 0) as i32
    };
}}
pub unsafe extern "C" fn matchbracketclass(c: i32, mut p: *const i8, ec: *const i8) -> i32 { unsafe {
    let mut sig: i32 = 1;
    if *p.offset(1 as isize) as i32 == '^' as i32 {
        sig = 0;
        p = p.offset(1);
    }
    loop {
        p = p.offset(1);
        if !(p < ec) {
            break;
        }
        if *p as i32 == '%' as i32 {
            p = p.offset(1);
            if match_class(c, *p as u8 as i32) != 0 {
                return sig;
            }
        } else if *p.offset(1 as isize) as i32 == '-' as i32 && p.offset(2 as isize) < ec {
            p = p.offset(2 as isize);
            if *p.offset(-(2 as isize)) as u8 as i32 <= c && c <= *p as u8 as i32 {
                return sig;
            }
        } else if *p as u8 as i32 == c {
            return sig;
        }
    }
    return (sig == 0) as i32;
}}
pub unsafe extern "C" fn singlematch(
    ms: *mut MatchState,
    s: *const i8,
    p: *const i8,
    ep: *const i8,
) -> i32 { unsafe {
    if s >= (*ms).src_end {
        return 0;
    } else {
        let c: i32 = *s as u8 as i32;
        match *p as i32 {
            46 => return 1,
            37 => {
                return match_class(c, *p.offset(1 as isize) as u8 as i32);
            }
            91 => return matchbracketclass(c, p, ep.offset(-(1 as isize))),
            _ => return (*p as u8 as i32 == c) as i32,
        }
    };
}}
pub unsafe extern "C" fn matchbalance(
    ms: *mut MatchState,
    mut s: *const i8,
    p: *const i8,
) -> *const i8 { unsafe {
    if ((p >= ((*ms).p_end).offset(-(1 as isize))) as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            (*ms).state,
            b"malformed pattern (missing arguments to '%%b')\0" as *const u8 as *const i8,
        );
    }
    if *s as i32 != *p as i32 {
        return std::ptr::null();
    } else {
        let b: i32 = *p as i32;
        let e: i32 = *p.offset(1 as isize) as i32;
        let mut cont: i32 = 1;
        loop {
            s = s.offset(1);
            if !(s < (*ms).src_end) {
                break;
            }
            if *s as i32 == e {
                cont -= 1;
                if cont == 0 {
                    return s.offset(1 as isize);
                }
            } else if *s as i32 == b {
                cont += 1;
            }
        }
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn max_expand(
    ms: *mut MatchState,
    s: *const i8,
    p: *const i8,
    ep: *const i8,
) -> *const i8 { unsafe {
    let mut i: i64 = 0;
    while singlematch(ms, s.offset(i as isize), p, ep) != 0 {
        i += 1;
    }
    while i >= 0 {
        let res: *const i8 = match_0(ms, s.offset(i as isize), ep.offset(1 as isize));
        if !res.is_null() {
            return res;
        }
        i -= 1;
    }
    return std::ptr::null();
}}
pub unsafe extern "C" fn min_expand(
    ms: *mut MatchState,
    mut s: *const i8,
    p: *const i8,
    ep: *const i8,
) -> *const i8 { unsafe {
    loop {
        let res: *const i8 = match_0(ms, s, ep.offset(1 as isize));
        if !res.is_null() {
            return res;
        } else if singlematch(ms, s, p, ep) != 0 {
            s = s.offset(1);
        } else {
            return std::ptr::null();
        }
    }
}}
pub unsafe extern "C" fn start_capture(
    ms: *mut MatchState,
    s: *const i8,
    p: *const i8,
    what: i32,
) -> *const i8 { unsafe {
    let res: *const i8;
    let level: usize = (*ms).level;
    if level >= MAX_CAPTURES {
        lual_error(
            (*ms).state,
            b"too many captures\0" as *const u8 as *const i8,
        );
    }
    (*ms).capture[level].init = s;
    (*ms).capture[level].length = what as i64;
    (*ms).level = level + 1;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).level = ((*ms).level).wrapping_sub(1);
        (*ms).level;
    }
    return res;
}}
pub unsafe extern "C" fn end_capture(
    ms: *mut MatchState,
    s: *const i8,
    p: *const i8,
) -> *const i8 { unsafe {
    let l: i32 = capture_to_close(ms);
    let res: *const i8;
    (*ms).capture[l as usize].length = s.offset_from((*ms).capture[l as usize].init) as i64;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).capture[l as usize].length = -1 as i64;
    }
    return res;
}}
pub unsafe extern "C" fn match_capture(
    ms: *mut MatchState,
    s: *const i8,
    mut l: i32,
) -> *const i8 { unsafe {
    let length: u64;
    l = check_capture(ms, l);
    length = (*ms).capture[l as usize].length as u64;
    if ((*ms).src_end).offset_from(s) as i64 as u64 >= length
        && memcmp(
            (*ms).capture[l as usize].init as *const libc::c_void,
            s as *const libc::c_void,
            length,
        ) == 0
    {
        return s.offset(length as isize);
    } else {
        return std::ptr::null();
    };
}}
pub unsafe extern "C" fn match_0(
    ms: *mut MatchState,
    mut s: *const i8,
    mut p: *const i8,
) -> *const i8 { unsafe {
    let mut ep_0: *const i8 = std::ptr::null();
    let mut current_block: u64;
    let fresh162 = (*ms).matchdepth;
    (*ms).matchdepth = (*ms).matchdepth - 1;
    if ((fresh162 == 0) as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            (*ms).state,
            b"pattern too complex\0" as *const u8 as *const i8,
        );
    }
    loop {
        if !(p != (*ms).p_end) {
            current_block = 6476622998065200121;
            break;
        }
        match *p as i32 {
            CHARACTER_PARENTHESIS_LEFT => {
                if *p.offset(1 as isize) as i32 == ')' as i32 {
                    s = start_capture(ms, s, p.offset(2 as isize), -(2));
                } else {
                    s = start_capture(ms, s, p.offset(1 as isize), -1);
                }
                current_block = 6476622998065200121;
                break;
            }
            CHARACTER_PARENTHESIS_RIGHT => {
                s = end_capture(ms, s, p.offset(1 as isize));
                current_block = 6476622998065200121;
                break;
            }
            36 => {
                if !(p.offset(1 as isize) != (*ms).p_end) {
                    s = if s == (*ms).src_end {
                        s
                    } else {
                        std::ptr::null()
                    };
                    current_block = 6476622998065200121;
                    break;
                }
            }
            37 => match *p.offset(1 as isize) as i32 {
                98 => {
                    current_block = 17965632435239708295;
                    match current_block {
                        17965632435239708295 => {
                            s = matchbalance(ms, s, p.offset(2 as isize));
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(4 as isize);
                            continue;
                        }
                        8236137900636309791 => {
                            let ep: *const i8;
                            let previous: i8;
                            p = p.offset(2 as isize);
                            if ((*p as i32 != '[' as i32) as i32 != 0) as i32 as i64 != 0 {
                                lual_error(
                                    (*ms).state,
                                    b"missing '[' after '%%f' in pattern\0" as *const u8
                                        as *const i8,
                                );
                            }
                            ep = classend(ms, p);
                            previous = (if s == (*ms).src_init {
                                '\0' as i32
                            } else {
                                *s.offset(-(1 as isize)) as i32
                            }) as i8;
                            if matchbracketclass(
                                previous as u8 as i32,
                                p,
                                ep.offset(-(1 as isize)),
                            ) == 0
                                && matchbracketclass(
                                    *s as u8 as i32,
                                    p,
                                    ep.offset(-(1 as isize)),
                                ) != 0
                            {
                                p = ep;
                                continue;
                            } else {
                                s = std::ptr::null();
                                current_block = 6476622998065200121;
                                break;
                            }
                        }
                        _ => {
                            s = match_capture(ms, s, *p.offset(1 as isize) as u8 as i32);
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(2 as isize);
                            continue;
                        }
                    }
                }
                102 => {
                    current_block = 8236137900636309791;
                    match current_block {
                        17965632435239708295 => {
                            s = matchbalance(ms, s, p.offset(2 as isize));
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(4 as isize);
                            continue;
                        }
                        8236137900636309791 => {
                            let ep: *const i8;
                            let previous: i8;
                            p = p.offset(2 as isize);
                            if ((*p as i32 != '[' as i32) as i32 != 0) as i32 as i64 != 0 {
                                lual_error(
                                    (*ms).state,
                                    b"missing '[' after '%%f' in pattern\0" as *const u8
                                        as *const i8,
                                );
                            }
                            ep = classend(ms, p);
                            previous = (if s == (*ms).src_init {
                                '\0' as i32
                            } else {
                                *s.offset(-(1 as isize)) as i32
                            }) as i8;
                            if matchbracketclass(
                                previous as u8 as i32,
                                p,
                                ep.offset(-(1 as isize)),
                            ) == 0
                                && matchbracketclass(
                                    *s as u8 as i32,
                                    p,
                                    ep.offset(-(1 as isize)),
                                ) != 0
                            {
                                p = ep;
                                continue;
                            } else {
                                s = std::ptr::null();
                                current_block = 6476622998065200121;
                                break;
                            }
                        }
                        _ => {
                            s = match_capture(ms, s, *p.offset(1 as isize) as u8 as i32);
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(2 as isize);
                            continue;
                        }
                    }
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 14576567515993809846;
                    match current_block {
                        17965632435239708295 => {
                            s = matchbalance(ms, s, p.offset(2 as isize));
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(4 as isize);
                            continue;
                        }
                        8236137900636309791 => {
                            let ep: *const i8;
                            let previous: i8;
                            p = p.offset(2 as isize);
                            if ((*p as i32 != '[' as i32) as i32 != 0) as i32 as i64 != 0 {
                                lual_error(
                                    (*ms).state,
                                    b"missing '[' after '%%f' in pattern\0" as *const u8
                                        as *const i8,
                                );
                            }
                            ep = classend(ms, p);
                            previous = (if s == (*ms).src_init {
                                '\0' as i32
                            } else {
                                *s.offset(-(1 as isize)) as i32
                            }) as i8;
                            if matchbracketclass(
                                previous as u8 as i32,
                                p,
                                ep.offset(-(1 as isize)),
                            ) == 0
                                && matchbracketclass(
                                    *s as u8 as i32,
                                    p,
                                    ep.offset(-(1 as isize)),
                                ) != 0
                            {
                                p = ep;
                                continue;
                            } else {
                                s = std::ptr::null();
                                current_block = 6476622998065200121;
                                break;
                            }
                        }
                        _ => {
                            s = match_capture(ms, s, *p.offset(1 as isize) as u8 as i32);
                            if s.is_null() {
                                current_block = 6476622998065200121;
                                break;
                            }
                            p = p.offset(2 as isize);
                            continue;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        ep_0 = classend(ms, p);
        if singlematch(ms, s, p, ep_0) == 0 {
            if *ep_0 as i32 == '*' as i32
                || *ep_0 as i32 == '?' as i32
                || *ep_0 as i32 == '-' as i32
            {
                p = ep_0.offset(1 as isize);
            } else {
                s = std::ptr::null();
                current_block = 6476622998065200121;
                break;
            }
        } else {
            match *ep_0 as i32 {
                63 => {
                    let res: *const i8;
                    res = match_0(ms, s.offset(1 as isize), ep_0.offset(1 as isize));
                    if !res.is_null() {
                        s = res;
                        current_block = 6476622998065200121;
                        break;
                    } else {
                        p = ep_0.offset(1 as isize);
                    }
                }
                43 => {
                    s = s.offset(1);
                    current_block = 13376797365003376294;
                    break;
                }
                42 => {
                    current_block = 13376797365003376294;
                    break;
                }
                45 => {
                    s = min_expand(ms, s, p, ep_0);
                    current_block = 6476622998065200121;
                    break;
                }
                _ => {
                    s = s.offset(1);
                    p = ep_0;
                }
            }
        }
    }
    match current_block {
        13376797365003376294 => {
            s = max_expand(ms, s, p, ep_0);
        }
        _ => {}
    }
    (*ms).matchdepth += 1;
    (*ms).matchdepth;
    return s;
}}
pub unsafe extern "C" fn lmemfind(
    mut s1: *const i8,
    mut l1: u64,
    s2: *const i8,
    mut l2: u64,
) -> *const i8 { unsafe {
    if l2 == 0u64 {
        return s1;
    } else if l2 > l1 {
        return std::ptr::null();
    } else {
        let mut init: *const i8 = std::ptr::null();
        l2 = l2.wrapping_sub(1);
        l1 = l1.wrapping_sub(l2);
        while l1 > 0u64 && {
            init = memchr(s1 as *const libc::c_void, *s2 as i32, l1) as *const i8;
            !init.is_null()
        } {
            init = init.offset(1);
            if memcmp(
                init as *const libc::c_void,
                s2.offset(1 as isize) as *const libc::c_void,
                l2,
            ) == 0
            {
                return init.offset(-(1 as isize));
            } else {
                l1 = (l1 as u64).wrapping_sub(init.offset_from(s1) as i64 as u64) as u64 as u64;
                s1 = init;
            }
        }
        return std::ptr::null();
    };
}}
pub unsafe extern "C" fn get_onecapture(
    ms: *mut MatchState,
    i: i32,
    s: *const i8,
    e: *const i8,
    cap: *mut *const i8,
) -> u64 { unsafe {
    if i >= (*ms).level as i32 {
        if ((i != 0) as i32 != 0) as i32 as i64 != 0 {
            lual_error(
                (*ms).state,
                b"invalid capture index %%%d\0" as *const u8 as *const i8,
                i + 1,
            );
        }
        *cap = s;
        return e.offset_from(s) as i64 as u64;
    } else {
        let capl: i64 = (*ms).capture[i as usize].length;
        *cap = (*ms).capture[i as usize].init;
        if ((capl == -1 as i64) as i32 != 0) as i32 as i64 != 0 {
            lual_error(
                (*ms).state,
                b"unfinished capture\0" as *const u8 as *const i8,
            );
        } else if capl == -(2) as i64 {
            (*((*ms).state)).push_integer(
                (((*ms).capture[i as usize].init).offset_from((*ms).src_init) as i64 + 1)
                    as i64,
            );
        }
        return capl as u64;
    };
}}
pub unsafe extern "C" fn push_onecapture(
    ms: *mut MatchState,
    i: i32,
    s: *const i8,
    e: *const i8,
) { unsafe {
    let mut cap: *const i8 = std::ptr::null();
    let l: i64 = get_onecapture(ms, i, s, e, &mut cap) as i64;
    if l != -(2) as i64 {
        lua_pushlstring((*ms).state, cap, l as u64);
    }
}}
pub unsafe extern "C" fn push_captures(
    ms: *mut MatchState,
    s: *const i8,
    e: *const i8,
) -> i32 { unsafe {
    let mut i: i32;
    let nlevels: i32 = if (*ms).level as i32 == 0 && !s.is_null() {
        1
    } else {
        (*ms).level as i32
    };
    lual_checkstack(
        (*ms).state,
        nlevels,
        b"too many captures\0" as *const u8 as *const i8,
    );
    i = 0;
    while i < nlevels {
        push_onecapture(ms, i, s, e);
        i += 1;
    }
    return nlevels;
}}
pub unsafe extern "C" fn nospecials(p: *const i8, l: u64) -> i32 { unsafe {
    let mut upto: u64 = 0;
    loop {
        if !(strpbrk(
            p.offset(upto as isize),
            b"^$*+?.([%-\0" as *const u8 as *const i8,
        ))
        .is_null()
        {
            return 0;
        }
        upto = (upto as u64)
            .wrapping_add((strlen(p.offset(upto as isize))).wrapping_add(1 as u64))
            as u64 as u64;
        if !(upto <= l) {
            break;
        }
    }
    return 1;
}}
pub unsafe extern "C" fn prepstate(
    ms: *mut MatchState,
    state: *mut State,
    s: *const i8,
    lexical_state: u64,
    p: *const i8,
    lp: u64,
) { unsafe {
    (*ms).state = state;
    (*ms).matchdepth = 200 as i32;
    (*ms).src_init = s;
    (*ms).src_end = s.offset(lexical_state as isize);
    (*ms).p_end = p.offset(lp as isize);
}}
pub unsafe extern "C" fn reprepstate(ms: *mut MatchState) { unsafe {
    (*ms).level = 0;
}}
pub unsafe extern "C" fn str_find_aux(state: *mut State, find: i32) -> i32 { unsafe {
    let mut lexical_state: u64 = 0;
    let mut lp: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
    let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
    let init: u64 =
        (posrelati(lual_optinteger(state, 3, 1 as i64), lexical_state)).wrapping_sub(1 as u64);
    if init > lexical_state {
        (*state).push_nil();
        return 1;
    }
    if find != 0 && (lua_toboolean(state, 4) != 0 || nospecials(p, lp) != 0) {
        let s2: *const i8 = lmemfind(s.offset(init as isize), lexical_state.wrapping_sub(init), p, lp);
        if !s2.is_null() {
            (*state).push_integer((s2.offset_from(s) as i64 + 1) as i64);
            (*state).push_integer(
                (s2.offset_from(s) as i64 as u64).wrapping_add(lp) as i64,
            );
            return 2;
        }
    } else {
        let mut ms: MatchState = MatchState {
            src_init: std::ptr::null(),
            src_end: std::ptr::null(),
            p_end: std::ptr::null(),
            state: std::ptr::null_mut(),
            matchdepth: 0,
            level: 0,
            capture: [MatchStateCapture {
                init: std::ptr::null(),
                length: 0,
            }; 32],
        };
        let mut s1: *const i8 = s.offset(init as isize);
        let anchor: i32 = (*p as i32 == '^' as i32) as i32;
        if anchor != 0 {
            p = p.offset(1);
            lp = lp.wrapping_sub(1);
        }
        prepstate(&mut ms, state, s, lexical_state, p, lp);
        loop {
            let res: *const i8;
            reprepstate(&mut ms);
            res = match_0(&mut ms, s1, p);
            if !res.is_null() {
                if find != 0 {
                    (*state).push_integer((s1.offset_from(s) as i64 + 1) as i64);
                    (*state).push_integer(res.offset_from(s) as i64 as i64);
                    return push_captures(&mut ms, std::ptr::null(), std::ptr::null()) + 2;
                } else {
                    return push_captures(&mut ms, s1, res);
                }
            }
            let fresh163 = s1;
            s1 = s1.offset(1);
            if !(fresh163 < ms.src_end && anchor == 0) {
                break;
            }
        }
    }
    (*state).push_nil();
    return 1;
}}
pub unsafe extern "C" fn str_find(state: *mut State) -> i32 { unsafe {
    return str_find_aux(state, 1);
}}
pub unsafe extern "C" fn str_match(state: *mut State) -> i32 { unsafe {
    return str_find_aux(state, 0);
}}
pub unsafe extern "C" fn gmatch_aux(state: *mut State) -> i32 { unsafe {
    let gm: *mut GMatchState =
        lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 3) as *mut GMatchState;
    let mut src: *const i8;
    (*gm).ms.state = state;
    src = (*gm).src;
    while src <= (*gm).ms.src_end {
        let e: *const i8;
        reprepstate(&mut (*gm).ms);
        e = match_0(&mut (*gm).ms, src, (*gm).p);
        if !e.is_null() && e != (*gm).lastmatch {
            (*gm).lastmatch = e;
            (*gm).src = (*gm).lastmatch;
            return push_captures(&mut (*gm).ms, src, e);
        }
        src = src.offset(1);
    }
    return 0;
}}
pub unsafe extern "C" fn gmatch(state: *mut State) -> i32 { unsafe {
    let mut lexical_state: u64 = 0;
    let mut lp: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
    let p: *const i8 = lual_checklstring(state, 2, &mut lp);
    let mut init: u64 =
        (posrelati(lual_optinteger(state, 3, 1 as i64), lexical_state)).wrapping_sub(1 as u64);
    let gm: *mut GMatchState;
    lua_settop(state, 2);
    gm = lua_newuserdatauv(state, ::core::mem::size_of::<GMatchState>() as u64, 0)
        as *mut GMatchState;
    if init > lexical_state {
        init = lexical_state.wrapping_add(1 as u64);
    }
    prepstate(&mut (*gm).ms, state, s, lexical_state, p, lp);
    (*gm).src = s.offset(init as isize);
    (*gm).p = p;
    (*gm).lastmatch = std::ptr::null();
    lua_pushcclosure(
        state,
        Some(gmatch_aux as unsafe extern "C" fn(*mut State) -> i32),
        3,
    );
    return 1;
}}
pub unsafe extern "C" fn add_s(
    ms: *mut MatchState,
    b: *mut Buffer,
    s: *const i8,
    e: *const i8,
) { unsafe {
    let mut l: u64 = 0;
    let state: *mut State = (*ms).state;
    let mut news: *const i8 = lua_tolstring(state, 3, &mut l);
    let mut p: *const i8;
    loop {
        p = memchr(news as *const libc::c_void, '%' as i32, l) as *mut i8;
        if p.is_null() {
            break;
        }
        (*b).lual_addlstring(news, p.offset_from(news) as i64 as u64);
        p = p.offset(1);
        if *p as i32 == '%' as i32 {
            ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh164 = (*b).length;
            (*b).length = ((*b).length).wrapping_add(1);
            *((*b).pointer).offset(fresh164 as isize) = *p;
        } else if *p as i32 == '0' as i32 {
            (*b).lual_addlstring(s, e.offset_from(s) as i64 as u64);
        } else if *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
            & _ISDIGIT as i32 as u16 as i32
            != 0
        {
            let mut cap: *const i8 = std::ptr::null();
            let resl: i64 = get_onecapture(ms, *p as i32 - '1' as i32, s, e, &mut cap) as i64;
            if resl == -(2) as i64 {
                (*b).lual_addvalue();
            } else {
                (*b).lual_addlstring(cap, resl as u64);
            }
        } else {
            lual_error(
                state,
                b"invalid use of '%c' in replacement string\0" as *const u8 as *const i8,
                '%' as i32,
            );
        }
        l = (l as u64).wrapping_sub(p.offset(1 as isize).offset_from(news) as i64 as u64) as u64
            as u64;
        news = p.offset(1 as isize);
    }
    (*b).lual_addlstring(news, l);
}}
pub unsafe extern "C" fn add_value(
    ms: *mut MatchState,
    b: *mut Buffer,
    s: *const i8,
    e: *const i8,
    tr: i32,
) -> i32 { unsafe {
    let state: *mut State = (*ms).state;
    match tr {
        6 => {
            let n: i32;
            lua_pushvalue(state, 3);
            n = push_captures(ms, s, e);
            lua_callk(state, n, 1, 0, None);
        }
        5 => {
            push_onecapture(ms, 0, s, e);
            lua_gettable(state, 3);
        }
        _ => {
            add_s(ms, b, s, e);
            return 1;
        }
    }
    if lua_toboolean(state, -1) == 0 {
        lua_settop(state, -1 - 1);
        (*b).lual_addlstring(s, e.offset_from(s) as i64 as u64);
        return 0;
    } else if ((!lua_isstring(state, -1)) as i32 != 0) as i32 as i64 != 0 {
        return lual_error(
            state,
            b"invalid replacement value (a %s)\0" as *const u8 as *const i8,
            lua_typename(state, lua_type(state, -1)),
        );
    } else {
        (*b).lual_addvalue();
        return 1;
    };
}}
pub unsafe extern "C" fn str_gsub(state: *mut State) -> i32 { unsafe {
    let mut srcl: u64 = 0;
    let mut lp: u64 = 0;
    let mut src: *const i8 = lual_checklstring(state, 1, &mut srcl);
    let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
    let mut lastmatch: *const i8 = std::ptr::null();
    let tr: i32 = lua_type(state, 3);
    let max_s: i64 = lual_optinteger(state, 4, srcl.wrapping_add(1 as u64) as i64);
    let anchor: i32 = (*p as i32 == '^' as i32) as i32;
    let mut n: i64 = 0;
    let mut changed: i32 = 0;
    let mut ms: MatchState = MatchState {
        src_init: std::ptr::null(),
        src_end: std::ptr::null(),
        p_end: std::ptr::null(),
        state: std::ptr::null_mut(),
        matchdepth: 0,
        level: 0,
        capture: [MatchStateCapture {
            init: std::ptr::null(),
            length: 0,
        }; 32],
    };
    let mut b = Buffer::new();
    (((tr == 3 || tr == 4 || tr == 6 || tr == 5) as i32 != 0) as i32 as i64 != 0
        || lual_typeerror(
            state,
            3,
            b"string/function/table\0" as *const u8 as *const i8,
        ) != 0) as i32;
    b.lual_buffinit(state);
    if anchor != 0 {
        p = p.offset(1);
        lp = lp.wrapping_sub(1);
    }
    prepstate(&mut ms, state, src, srcl, p, lp);
    while n < max_s {
        let e: *const i8;
        reprepstate(&mut ms);
        e = match_0(&mut ms, src, p);
        if !e.is_null() && e != lastmatch {
            n += 1;
            changed = add_value(&mut ms, &mut b, src, e, tr) | changed;
            lastmatch = e;
            src = lastmatch;
        } else {
            if !(src < ms.src_end) {
                break;
            }
            (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh165 = src;
            src = src.offset(1);
            let fresh166 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh166 as isize) = *fresh165;
        }
        if anchor != 0 {
            break;
        }
    }
    if changed == 0 {
        lua_pushvalue(state, 1);
    } else {
        b.lual_addlstring(src, (ms.src_end).offset_from(src) as i64 as u64);
        b.lual_pushresult();
    }
    (*state).push_integer(n);
    return 2;
}}
pub unsafe extern "C" fn addquoted(b: *mut Buffer, mut s: *const i8, mut length: u64) { unsafe {
    ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
    let fresh167 = (*b).length;
    (*b).length = ((*b).length).wrapping_add(1);
    *((*b).pointer).offset(fresh167 as isize) = '"' as i8;
    loop {
        let fresh168 = length;
        length = length.wrapping_sub(1);
        if !(fresh168 != 0) {
            break;
        }
        if *s as i32 == '"' as i32 || *s as i32 == '\\' as i32 || *s as i32 == '\n' as i32 {
            ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh169 = (*b).length;
            (*b).length = ((*b).length).wrapping_add(1);
            *((*b).pointer).offset(fresh169 as isize) = '\\' as i8;
            ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh170 = (*b).length;
            (*b).length = ((*b).length).wrapping_add(1);
            *((*b).pointer).offset(fresh170 as isize) = *s;
        } else if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISCONTROL as i32 as u16 as i32
            != 0
        {
            let mut buffer: [i8; 10] = [0; 10];
            if *(*__ctype_b_loc()).offset(*s.offset(1 as isize) as u8 as i32 as isize) as i32
                & _ISDIGIT as i32 as u16 as i32
                == 0
            {
                snprintf(
                    buffer.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 10]>() as u64,
                    b"\\%d\0" as *const u8 as *const i8,
                    *s as u8 as i32,
                );
            } else {
                snprintf(
                    buffer.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 10]>() as u64,
                    b"\\%03d\0" as *const u8 as *const i8,
                    *s as u8 as i32,
                );
            }
            (*b).lual_addstring(buffer.as_mut_ptr());
        } else {
            ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh171 = (*b).length;
            (*b).length = ((*b).length).wrapping_add(1);
            *((*b).pointer).offset(fresh171 as isize) = *s;
        }
        s = s.offset(1);
    }
    ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
    let fresh172 = (*b).length;
    (*b).length = ((*b).length).wrapping_add(1);
    *((*b).pointer).offset(fresh172 as isize) = '"' as i8;
}}
pub unsafe extern "C" fn quotefloat(mut _state: *mut State, buffer: *mut i8, n: f64) -> i32 { unsafe {
    let s: *const i8;
    if n == ::core::f64::INFINITY {
        s = b"1e9999\0" as *const u8 as *const i8;
    } else if n == -::core::f64::INFINITY {
        s = b"-1e9999\0" as *const u8 as *const i8;
    } else if n != n {
        s = b"(0/0)\0" as *const u8 as *const i8;
    } else {
        let nb: i32 = snprintf(
            buffer,
            120 as i32 as u64,
            b"%a\0" as *const u8 as *const i8,
            n,
        );
        if (memchr(buffer as *const libc::c_void, '.' as i32, nb as u64)).is_null() {
            let point: i8 = '.' as i8;
            let ppoint: *mut i8 =
                memchr(buffer as *const libc::c_void, point as i32, nb as u64) as *mut i8;
            if !ppoint.is_null() {
                *ppoint = '.' as i8;
            }
        }
        return nb;
    }
    return snprintf(
        buffer,
        120 as i32 as u64,
        b"%s\0" as *const u8 as *const i8,
        s,
    );
}}
pub unsafe extern "C" fn addliteral(state: *mut State, b: *mut Buffer, arg: i32) { unsafe {
    match lua_type(state, arg) {
        4 => {
            let mut length: u64 = 0;
            let s: *const i8 = lua_tolstring(state, arg, &mut length);
            addquoted(b, s, length);
        }
        3 => {
            let buffer: *mut i8 = (*b).lual_prepbuffsize(120 as i32 as u64);
            let nb: i32;
            if lua_isinteger(state, arg) {
                let n: i64 = lua_tointegerx(state, arg, std::ptr::null_mut());
                let format: *const i8 = if n == -(9223372036854775807 as i64) - 1 as i64 {
                    b"0x%llx\0" as *const u8 as *const i8
                } else {
                    b"%lld\0" as *const u8 as *const i8
                };
                nb = snprintf(buffer, 120 as i32 as u64, format, n);
            } else {
                nb = quotefloat(state, buffer, lua_tonumberx(state, arg, std::ptr::null_mut()));
            }
            (*b).length = ((*b).length as u64).wrapping_add(nb as u64) as u64 as u64;
        }
        0 | 1 => {
            lual_tolstring(state, arg, std::ptr::null_mut());
            (*b).lual_addvalue();
        }
        _ => {
            lual_argerror(
                state,
                arg,
                b"value has no literal form\0" as *const u8 as *const i8,
            );
        }
    };
}}
pub unsafe extern "C" fn get2digits(mut s: *const i8) -> *const i8 { unsafe {
    if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32 & _ISDIGIT as i32 as u16 as i32
        != 0
    {
        s = s.offset(1);
        if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISDIGIT as i32 as u16 as i32
            != 0
        {
            s = s.offset(1);
        }
    }
    return s;
}}
pub unsafe extern "C" fn checkformat(
    state: *mut State,
    form: *const i8,
    flags: *const i8,
    precision: i32,
) { unsafe {
    let mut spec: *const i8 = form.offset(1 as isize);
    spec = spec.offset(strspn(spec, flags) as isize);
    if *spec as i32 != '0' as i32 {
        spec = get2digits(spec);
        if *spec as i32 == '.' as i32 && precision != 0 {
            spec = spec.offset(1);
            spec = get2digits(spec);
        }
    }
    if *(*__ctype_b_loc()).offset(*spec as u8 as i32 as isize) as i32
        & _ISALPHA as i32 as u16 as i32
        == 0
    {
        lual_error(
            state,
            b"invalid conversion specification: '%s'\0" as *const u8 as *const i8,
            form,
        );
    }
}}
pub unsafe extern "C" fn getformat(
    state: *mut State,
    strfrmt: *const i8,
    mut form: *mut i8,
) -> *const i8 { unsafe {
    let mut length: u64 = strspn(strfrmt, b"-+#0 123456789.\0" as *const u8 as *const i8);
    length = length.wrapping_add(1);
    if length >= (32 as i32 - 10 as i32) as u64 {
        lual_error(
            state,
            b"invalid format (too long)\0" as *const u8 as *const i8,
        );
    }
    let fresh173 = form;
    form = form.offset(1);
    *fresh173 = '%' as i8;
    memcpy(
        form as *mut libc::c_void,
        strfrmt as *const libc::c_void,
        length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    *form.offset(length as isize) = '\0' as i8;
    return strfrmt.offset(length as isize).offset(-(1 as isize));
}}
pub unsafe extern "C" fn addlenmod(form: *mut i8, lenmod: *const i8) { unsafe {
    let l: u64 = strlen(form);
    let lm: u64 = strlen(lenmod);
    let spec: i8 = *form.offset(l.wrapping_sub(1 as u64) as isize);
    strcpy(form.offset(l as isize).offset(-(1 as isize)), lenmod);
    *form.offset(l.wrapping_add(lm).wrapping_sub(1 as u64) as isize) = spec;
    *form.offset(l.wrapping_add(lm) as isize) = '\0' as i8;
}}
pub unsafe extern "C" fn str_format(state: *mut State) -> i32 { unsafe {
    let mut current_block: u64;
    let top: i32 = (*state).get_top();
    let mut arg: i32 = 1;
    let mut sfl: u64 = 0;
    let mut strfrmt: *const i8 = lual_checklstring(state, arg, &mut sfl);
    let strfrmt_end: *const i8 = strfrmt.offset(sfl as isize);
    let mut flags: *const i8 = std::ptr::null();
    let mut b = Buffer::new();
    b.lual_buffinit(state);
    while strfrmt < strfrmt_end {
        if *strfrmt as i32 != '%' as i32 {
            (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh174 = strfrmt;
            strfrmt = strfrmt.offset(1);
            let fresh175 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh175 as isize) = *fresh174;
        } else {
            strfrmt = strfrmt.offset(1);
            if *strfrmt as i32 == '%' as i32 {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh176 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh177 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh177 as isize) = *fresh176;
            } else {
                let mut form: [i8; 32] = [0; 32];
                let mut maxitem: i32 = 120 as i32;
                let mut buffer: *mut i8 = b.lual_prepbuffsize(maxitem as u64);
                let mut nb: i32 = 0;
                arg += 1;
                if arg > top {
                    return lual_argerror(state, arg, b"no value\0" as *const u8 as *const i8);
                }
                strfrmt = getformat(state, strfrmt, form.as_mut_ptr());
                let fresh178 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh178 as i32 {
                    99 => {
                        checkformat(
                            state,
                            form.as_mut_ptr(),
                            b"-\0" as *const u8 as *const i8,
                            0,
                        );
                        nb = snprintf(
                            buffer,
                            maxitem as u64,
                            form.as_mut_ptr(),
                            lual_checkinteger(state, arg) as i32,
                        );
                        current_block = 11793792312832361944;
                    }
                    100 | 105 => {
                        flags = b"-+0 \0" as *const u8 as *const i8;
                        current_block = 5689001924483802034;
                    }
                    117 => {
                        flags = b"-0\0" as *const u8 as *const i8;
                        current_block = 5689001924483802034;
                    }
                    111 | 120 | 88 => {
                        flags = b"-#0\0" as *const u8 as *const i8;
                        current_block = 5689001924483802034;
                    }
                    97 | 65 => {
                        checkformat(
                            state,
                            form.as_mut_ptr(),
                            b"-+#0 \0" as *const u8 as *const i8,
                            1,
                        );
                        addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                        nb = snprintf(
                            buffer,
                            maxitem as u64,
                            form.as_mut_ptr(),
                            lual_checknumber(state, arg),
                        );
                        current_block = 11793792312832361944;
                    }
                    102 => {
                        maxitem = 110 as i32 + 308 as i32;
                        buffer = b.lual_prepbuffsize(maxitem as u64);
                        current_block = 6669252993407410313;
                    }
                    101 | 69 | 103 | 71 => {
                        current_block = 6669252993407410313;
                    }
                    112 => {
                        let mut p: *const libc::c_void = lua_topointer(state, arg);
                        checkformat(
                            state,
                            form.as_mut_ptr(),
                            b"-\0" as *const u8 as *const i8,
                            0,
                        );
                        if p.is_null() {
                            p = b"(null)\0" as *const u8 as *const i8 as *const libc::c_void;
                            form[(strlen(form.as_mut_ptr())).wrapping_sub(1 as u64) as usize] =
                                's' as i8;
                        }
                        nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), p);
                        current_block = 11793792312832361944;
                    }
                    113 => {
                        if form[2 as usize] as i32 != '\0' as i32 {
                            return lual_error(
                                state,
                                b"specifier '%%q' cannot have modifiers\0" as *const u8
                                    as *const i8,
                            );
                        }
                        addliteral(state, &mut b, arg);
                        current_block = 11793792312832361944;
                    }
                    115 => {
                        let mut l: u64 = 0;
                        let s: *const i8 = lual_tolstring(state, arg, &mut l);
                        if form[2 as usize] as i32 == '\0' as i32 {
                            b.lual_addvalue();
                        } else {
                            (((l == strlen(s)) as i32 != 0) as i32 as i64 != 0
                                || lual_argerror(
                                    state,
                                    arg,
                                    b"string contains zeros\0" as *const u8 as *const i8,
                                ) != 0) as i32;
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const i8,
                                1,
                            );
                            if (strchr(form.as_mut_ptr(), '.' as i32)).is_null()
                                && l >= 100 as i32 as u64
                            {
                                b.lual_addvalue();
                            } else {
                                nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), s);
                                lua_settop(state, -1 - 1);
                            }
                        }
                        current_block = 11793792312832361944;
                    }
                    _ => {
                        return lual_error(
                            state,
                            b"invalid conversion '%s' to 'format'\0" as *const u8 as *const i8,
                            form.as_mut_ptr(),
                        );
                    }
                }
                match current_block {
                    5689001924483802034 => {
                        let n: i64 = lual_checkinteger(state, arg);
                        checkformat(state, form.as_mut_ptr(), flags, 1);
                        addlenmod(form.as_mut_ptr(), b"ll\0" as *const u8 as *const i8);
                        nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n);
                    }
                    6669252993407410313 => {
                        let n_0: f64 = lual_checknumber(state, arg);
                        checkformat(
                            state,
                            form.as_mut_ptr(),
                            b"-+#0 \0" as *const u8 as *const i8,
                            1,
                        );
                        addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                        nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n_0);
                    }
                    _ => {}
                }
                b.length = (b.length as u64).wrapping_add(nb as u64) as u64 as u64;
            }
        }
    }
    b.lual_pushresult();
    return 1;
}}
static mut NATIVE_ENDIAN: NativeEndian = NativeEndian { dummy: 1 };
pub unsafe extern "C" fn digit(c: i32) -> i32 {
    return ('0' as i32 <= c && c <= '9' as i32) as i32;
}
pub unsafe extern "C" fn getnum(fmt: *mut *const i8, df: i32) -> i32 { unsafe {
    if digit(**fmt as i32) == 0 {
        return df;
    } else {
        let mut a: i32 = 0;
        loop {
            let fresh179 = *fmt;
            *fmt = (*fmt).offset(1);
            a = a * 10 as i32 + (*fresh179 as i32 - '0' as i32);
            if !(digit(**fmt as i32) != 0
                && a <= ((if (::core::mem::size_of::<u64>() as u64)
                    < ::core::mem::size_of::<i32>() as u64
                {
                    !(0u64)
                } else {
                    2147483647 as i32 as u64
                }) as i32
                    - 9 as i32)
                    / 10 as i32)
            {
                break;
            }
        }
        return a;
    };
}}
pub unsafe extern "C" fn getnumlimit(
    h: *mut Header,
    fmt: *mut *const i8,
    df: i32,
) -> i32 { unsafe {
    let size: i32 = getnum(fmt, df);
    if ((size > 16 as i32 || size <= 0) as i32 != 0) as i32 as i64 != 0 {
        return lual_error(
            (*h).state,
            b"integral size (%d) out of limits [1,%d]\0" as *const u8 as *const i8,
            size,
            16 as i32,
        );
    }
    return size;
}}
pub unsafe extern "C" fn initheader(state: *mut State, h: *mut Header) { unsafe {
    (*h).state = state;
    (*h).islittle = NATIVE_ENDIAN.little as i32;
    (*h).maxalign = 1;
}}
pub unsafe extern "C" fn getoption(
    h: *mut Header,
    fmt: *mut *const i8,
    size: *mut i32,
) -> K { unsafe {
    let fresh180 = *fmt;
    *fmt = (*fmt).offset(1);
    let opt: i32 = *fresh180 as i32;
    *size = 0;
    match opt {
        98 => {
            *size = ::core::mem::size_of::<i8>() as u64 as i32;
            return K::Integer;
        }
        66 => {
            *size = ::core::mem::size_of::<i8>() as u64 as i32;
            return K::Unsigned;
        }
        104 => {
            *size = ::core::mem::size_of::<i16>() as u64 as i32;
            return K::Integer;
        }
        72 => {
            *size = ::core::mem::size_of::<i16>() as u64 as i32;
            return K::Unsigned;
        }
        108 => {
            *size = ::core::mem::size_of::<i64>() as u64 as i32;
            return K::Integer;
        }
        76 => {
            *size = ::core::mem::size_of::<i64>() as u64 as i32;
            return K::Unsigned;
        }
        106 => {
            *size = ::core::mem::size_of::<i64>() as u64 as i32;
            return K::Integer;
        }
        74 => {
            *size = ::core::mem::size_of::<i64>() as u64 as i32;
            return K::Unsigned;
        }
        84 => {
            *size = ::core::mem::size_of::<u64>() as u64 as i32;
            return K::Unsigned;
        }
        102 => {
            *size = ::core::mem::size_of::<libc::c_float>() as u64 as i32;
            return K::Float;
        }
        110 => {
            *size = ::core::mem::size_of::<f64>() as u64 as i32;
            return K::Number;
        }
        100 => {
            *size = ::core::mem::size_of::<f64>() as u64 as i32;
            return K::Double;
        }
        105 => {
            *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as u64 as i32);
            return K::Integer;
        }
        73 => {
            *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as u64 as i32);
            return K::Unsigned;
        }
        115 => {
            *size = getnumlimit(h, fmt, ::core::mem::size_of::<u64>() as u64 as i32);
            return K::String;
        }
        99 => {
            *size = getnum(fmt, -1);
            if ((*size == -1) as i32 != 0) as i32 as i64 != 0 {
                lual_error(
                    (*h).state,
                    b"missing size for format option 'c'\0" as *const u8 as *const i8,
                );
            }
            return K::Character;
        }
        122 => return K::ZString,
        120 => {
            *size = 1;
            return K::Padding;
        }
        88 => return K::PaddingAlignment,
        32 => {}
        60 => {
            (*h).islittle = 1;
        }
        62 => {
            (*h).islittle = 0;
        }
        61 => {
            (*h).islittle = NATIVE_ENDIAN.little as i32;
        }
        33 => {
            let maxalign: i32 = 8;
            (*h).maxalign = getnumlimit(h, fmt, maxalign);
        }
        _ => {
            lual_error(
                (*h).state,
                b"invalid format option '%c'\0" as *const u8 as *const i8,
                opt,
            );
        }
    }
    return K::NoOperator;
}}
pub unsafe extern "C" fn getdetails(
    h: *mut Header,
    totalsize: u64,
    fmt: *mut *const i8,
    total_size: *mut i32,
    ntoalign: *mut i32,
) -> K { unsafe {
    let opt: K = getoption(h, fmt, total_size);
    let mut align: i32 = *total_size;
    if opt as u32 == K::PaddingAlignment as i32 as u32 {
        if **fmt as i32 == '\0' as i32
            || getoption(h, fmt, &mut align) as u32 == K::Character as i32 as u32
            || align == 0
        {
            lual_argerror(
                (*h).state,
                1,
                b"invalid next option for option 'X'\0" as *const u8 as *const i8,
            );
        }
    }
    if align <= 1 || opt as u32 == K::Character as i32 as u32 {
        *ntoalign = 0;
    } else {
        if align > (*h).maxalign {
            align = (*h).maxalign;
        }
        if ((align & align - 1 != 0) as i32 != 0) as i32 as i64 != 0 {
            lual_argerror(
                (*h).state,
                1,
                b"format asks for alignment not power of 2\0" as *const u8 as *const i8,
            );
        }
        *ntoalign = align - (totalsize & (align - 1) as u64) as i32 & align - 1;
    }
    return opt;
}}
pub unsafe extern "C" fn packint(
    b: *mut Buffer,
    mut n: u64,
    islittle: i32,
    size: i32,
    is_negative_: i32,
) { unsafe {
    let buffer: *mut i8 = (*b).lual_prepbuffsize(size as u64);
    let mut i: i32;
    *buffer.offset((if islittle != 0 { 0 } else { size - 1 }) as isize) =
        (n & ((1 << 8) - 1) as u64) as i8;
    i = 1;
    while i < size {
        n >>= 8;
        *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
            (n & ((1 << 8) - 1) as u64) as i8;
        i += 1;
    }
    if is_negative_ != 0 && size > ::core::mem::size_of::<i64>() as u64 as i32 {
        i = ::core::mem::size_of::<i64>() as u64 as i32;
        while i < size {
            *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
                ((1 << 8) - 1) as i8;
            i += 1;
        }
    }
    (*b).length = ((*b).length as u64).wrapping_add(size as u64) as u64 as u64;
}}
pub unsafe extern "C" fn copywithendian(
    mut dest: *mut i8,
    mut src: *const i8,
    mut size: i32,
    islittle: i32,
) { unsafe {
    if islittle == NATIVE_ENDIAN.little as i32 {
        memcpy(
            dest as *mut libc::c_void,
            src as *const libc::c_void,
            size as u64,
        );
    } else {
        dest = dest.offset((size - 1) as isize);
        loop {
            let fresh181 = size;
            size = size - 1;
            if !(fresh181 != 0) {
                break;
            }
            let fresh182 = src;
            src = src.offset(1);
            let fresh183 = dest;
            dest = dest.offset(-1);
            *fresh183 = *fresh182;
        }
    };
}}
pub unsafe extern "C" fn str_pack(state: *mut State) -> i32 { unsafe {
    let mut b = Buffer::new();
    let mut h: Header = Header {
        state: std::ptr::null_mut(),
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let mut arg: i32 = 1;
    let mut totalsize: u64 = 0;
    initheader(state, &mut h);
    (*state).push_nil();
    b.lual_buffinit(state);
    while *fmt as i32 != '\0' as i32 {
        let mut size: i32 = 0;
        let mut ntoalign: i32 = 0;
        let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        totalsize = (totalsize as u64).wrapping_add((ntoalign + size) as u64) as u64 as u64;
        loop {
            let fresh184 = ntoalign;
            ntoalign = ntoalign - 1;
            if !(fresh184 > 0) {
                break;
            }
            (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh185 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh185 as isize) = 0 as i8;
        }
        arg += 1;
        let current_block_33: u64;
        match opt as u32 {
            0 => {
                let n: i64 = lual_checkinteger(state, arg);
                if size < ::core::mem::size_of::<i64>() as u64 as i32 {
                    let lim: i64 = 1 << size * 8 - 1;
                    (((-lim <= n && n < lim) as i32 != 0) as i32 as i64 != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"integer overflow\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                }
                packint(&mut b, n as u64, h.islittle, size, (n < 0) as i32);
                current_block_33 = 3222590281903869779;
            }
            1 => {
                let n_0: i64 = lual_checkinteger(state, arg);
                if size < ::core::mem::size_of::<i64>() as u64 as i32 {
                    ((((n_0 as u64) < (1 as u64) << size * 8) as i32 != 0) as i32 as i64
                        != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"unsigned overflow\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                }
                packint(&mut b, n_0 as u64, h.islittle, size, 0);
                current_block_33 = 3222590281903869779;
            }
            2 => {
                let mut f: libc::c_float = lual_checknumber(state, arg) as libc::c_float;
                let buffer: *mut i8 =
                    b.lual_prepbuffsize(::core::mem::size_of::<libc::c_float>() as u64);
                copywithendian(
                    buffer,
                    &mut f as *mut libc::c_float as *mut i8,
                    ::core::mem::size_of::<libc::c_float>() as u64 as i32,
                    h.islittle,
                );
                b.length = (b.length as u64).wrapping_add(size as u64) as u64 as u64;
                current_block_33 = 3222590281903869779;
            }
            3 => {
                let mut f_0: f64 = lual_checknumber(state, arg);
                let buff_0: *mut i8 =
                    b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                copywithendian(
                    buff_0,
                    &mut f_0 as *mut f64 as *mut i8,
                    ::core::mem::size_of::<f64>() as u64 as i32,
                    h.islittle,
                );
                b.length = (b.length as u64).wrapping_add(size as u64) as u64 as u64;
                current_block_33 = 3222590281903869779;
            }
            4 => {
                let mut f_1: f64 = lual_checknumber(state, arg);
                let buff_1: *mut i8 =
                    b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                copywithendian(
                    buff_1,
                    &mut f_1 as *mut f64 as *mut i8,
                    ::core::mem::size_of::<f64>() as u64 as i32,
                    h.islittle,
                );
                b.length = (b.length as u64).wrapping_add(size as u64) as u64 as u64;
                current_block_33 = 3222590281903869779;
            }
            5 => {
                let mut length: u64 = 0;
                let s: *const i8 = lual_checklstring(state, arg, &mut length);
                (((length <= size as u64) as i32 != 0) as i32 as i64 != 0
                    || lual_argerror(
                        state,
                        arg,
                        b"string longer than given size\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                b.lual_addlstring(s, length);
                loop {
                    let fresh186 = length;
                    length = length.wrapping_add(1);
                    if !(fresh186 < size as u64) {
                        break;
                    }
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh187 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh187 as isize) = 0 as i8;
                }
                current_block_33 = 3222590281903869779;
            }
            6 => {
                let mut length_0: u64 = 0;
                let s_0: *const i8 = lual_checklstring(state, arg, &mut length_0);
                (((size >= ::core::mem::size_of::<u64>() as u64 as i32
                    || length_0 < (1 as u64) << size * 8) as i32
                    != 0) as i32 as i64
                    != 0
                    || lual_argerror(
                        state,
                        arg,
                        b"string length does not fit in given size\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                packint(&mut b, length_0 as u64, h.islittle, size, 0);
                b.lual_addlstring(s_0, length_0);
                totalsize = (totalsize as u64).wrapping_add(length_0) as u64 as u64;
                current_block_33 = 3222590281903869779;
            }
            7 => {
                let mut length_1: u64 = 0;
                let s_1: *const i8 = lual_checklstring(state, arg, &mut length_1);
                (((strlen(s_1) == length_1) as i32 != 0) as i32 as i64 != 0
                    || lual_argerror(
                        state,
                        arg,
                        b"string contains zeros\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                b.lual_addlstring(s_1, length_1);
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh188 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh188 as isize) = '\0' as i8;
                totalsize =
                    (totalsize as u64).wrapping_add(length_1.wrapping_add(1 as u64)) as u64 as u64;
                current_block_33 = 3222590281903869779;
            }
            8 => {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh189 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh189 as isize) = 0 as i8;
                current_block_33 = 7383952003695197780;
            }
            9 | 10 => {
                current_block_33 = 7383952003695197780;
            }
            _ => {
                current_block_33 = 3222590281903869779;
            }
        }
        match current_block_33 {
            7383952003695197780 => {
                arg -= 1;
            }
            _ => {}
        }
    }
    b.lual_pushresult();
    return 1;
}}
pub unsafe extern "C" fn str_packsize(state: *mut State) -> i32 { unsafe {
    let mut h: Header = Header {
        state: std::ptr::null_mut(),
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let mut totalsize: u64 = 0;
    initheader(state, &mut h);
    while *fmt as i32 != '\0' as i32 {
        let mut size: i32 = 0;
        let mut ntoalign: i32 = 0;
        let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        (((opt as u32 != K::String as i32 as u32 && opt as u32 != K::ZString as i32 as u32) as i32
            != 0) as i32 as i64
            != 0
            || lual_argerror(
                state,
                1,
                b"variable-length format\0" as *const u8 as *const i8,
            ) != 0) as i32;
        size += ntoalign;
        (((totalsize
            <= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                2147483647 as i32 as u64
            })
            .wrapping_sub(size as u64)) as i32
            != 0) as i32 as i64
            != 0
            || lual_argerror(
                state,
                1,
                b"format result too large\0" as *const u8 as *const i8,
            ) != 0) as i32;
        totalsize = (totalsize as u64).wrapping_add(size as u64) as u64 as u64;
    }
    (*state).push_integer(totalsize as i64);
    return 1;
}}
pub unsafe extern "C" fn unpackint(
    state: *mut State,
    str: *const i8,
    islittle: i32,
    size: i32,
    issigned: i32,
) -> i64 { unsafe {
    let mut res: u64 = 0;
    let mut i: i32;
    let limit: i32 = if size <= ::core::mem::size_of::<i64>() as u64 as i32 {
        size
    } else {
        ::core::mem::size_of::<i64>() as u64 as i32
    };
    i = limit - 1;
    while i >= 0 {
        res <<= 8;
        res |= *str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8 as u64;
        i -= 1;
    }
    if size < ::core::mem::size_of::<i64>() as u64 as i32 {
        if issigned != 0 {
            let mask: u64 = (1 as u64) << size * 8 - 1;
            res = (res ^ mask).wrapping_sub(mask);
        }
    } else if size > ::core::mem::size_of::<i64>() as u64 as i32 {
        let mask_0: i32 = if issigned == 0 || res as i64 >= 0 {
            0
        } else {
            (1 << 8) - 1
        };
        i = limit;
        while i < size {
            if ((*str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8
                as i32
                != mask_0) as i32
                != 0) as i32 as i64
                != 0
            {
                lual_error(
                    state,
                    b"%d-byte integer does not fit into Lua Integer\0" as *const u8 as *const i8,
                    size,
                );
            }
            i += 1;
        }
    }
    return res as i64;
}}
pub unsafe extern "C" fn str_unpack(state: *mut State) -> i32 { unsafe {
    let mut h: Header = Header {
        state: std::ptr::null_mut(),
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let mut ld: u64 = 0;
    let data: *const i8 = lual_checklstring(state, 2, &mut ld);
    let mut pos: u64 =
        (posrelati(lual_optinteger(state, 3, 1 as i64), ld)).wrapping_sub(1 as u64);
    let mut n: i32 = 0;
    (((pos <= ld) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            3,
            b"initial position out of string\0" as *const u8 as *const i8,
        ) != 0) as i32;
    initheader(state, &mut h);
    while *fmt as i32 != '\0' as i32 {
        let mut size: i32 = 0;
        let mut ntoalign: i32 = 0;
        let opt: K = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
        ((((ntoalign as u64).wrapping_add(size as u64) <= ld.wrapping_sub(pos)) as i32 != 0)
            as i32 as i64
            != 0
            || lual_argerror(
                state,
                2,
                b"data string too short\0" as *const u8 as *const i8,
            ) != 0) as i32;
        pos = (pos as u64).wrapping_add(ntoalign as u64) as u64 as u64;
        lual_checkstack(state, 2, b"too many results\0" as *const u8 as *const i8);
        n += 1;
        match opt as u32 {
            0 | 1 => {
                let res: i64 = unpackint(
                    state,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    (opt as u32 == K::Integer as i32 as u32) as i32,
                );
                (*state).push_integer(res);
            }
            2 => {
                let mut f: libc::c_float = 0.0;
                copywithendian(
                    &mut f as *mut libc::c_float as *mut i8,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<libc::c_float>() as u64 as i32,
                    h.islittle,
                );
                (*state).push_number(f as f64);
            }
            3 => {
                let mut f_0: f64 = 0.0;
                copywithendian(
                    &mut f_0 as *mut f64 as *mut i8,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<f64>() as u64 as i32,
                    h.islittle,
                );
                (*state).push_number(f_0);
            }
            4 => {
                let mut f_1: f64 = 0.0;
                copywithendian(
                    &mut f_1 as *mut f64 as *mut i8,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<f64>() as u64 as i32,
                    h.islittle,
                );
                (*state).push_number(f_1);
            }
            5 => {
                lua_pushlstring(state, data.offset(pos as isize), size as u64);
            }
            6 => {
                let length: u64 =
                    unpackint(state, data.offset(pos as isize), h.islittle, size, 0) as u64;
                (((length <= ld.wrapping_sub(pos).wrapping_sub(size as u64)) as i32 != 0) as i32
                    as i64
                    != 0
                    || lual_argerror(
                        state,
                        2,
                        b"data string too short\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                lua_pushlstring(state, data.offset(pos as isize).offset(size as isize), length);
                pos = (pos as u64).wrapping_add(length) as u64 as u64;
            }
            7 => {
                let length_0: u64 = strlen(data.offset(pos as isize));
                (((pos.wrapping_add(length_0) < ld) as i32 != 0) as i32 as i64 != 0
                    || lual_argerror(
                        state,
                        2,
                        b"unfinished string for format 'zio'\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                lua_pushlstring(state, data.offset(pos as isize), length_0);
                pos = (pos as u64).wrapping_add(length_0.wrapping_add(1 as u64)) as u64 as u64;
            }
            9 | 8 | 10 => {
                n -= 1;
            }
            _ => {}
        }
        pos = (pos as u64).wrapping_add(size as u64) as u64 as u64;
    }
    (*state).push_integer(pos.wrapping_add(1 as u64) as i64);
    return n + 1;
}}
static mut STRING_FUNCTIONS: [RegisteredFunction; 18] = {
    [
        {
            let init = RegisteredFunction {
                name: b"byte\0" as *const u8 as *const i8,
                function: Some(str_byte as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(str_char as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"dump\0" as *const u8 as *const i8,
                function: Some(str_dump as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"find\0" as *const u8 as *const i8,
                function: Some(str_find as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"format\0" as *const u8 as *const i8,
                function: Some(str_format as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"gmatch\0" as *const u8 as *const i8,
                function: Some(gmatch as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"gsub\0" as *const u8 as *const i8,
                function: Some(str_gsub as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(str_len as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"lower\0" as *const u8 as *const i8,
                function: Some(str_lower as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"match\0" as *const u8 as *const i8,
                function: Some(str_match as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"rep\0" as *const u8 as *const i8,
                function: Some(str_rep as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"reverse\0" as *const u8 as *const i8,
                function: Some(str_reverse as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"sub\0" as *const u8 as *const i8,
                function: Some(str_sub as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"upper\0" as *const u8 as *const i8,
                function: Some(str_upper as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(str_pack as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"packsize\0" as *const u8 as *const i8,
                function: Some(str_packsize as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(str_unpack as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn createmetatable(state: *mut State) { unsafe {
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 10]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, STRING_METAMETHODS.as_ptr(), 0);
    lua_pushstring(state, b"\0" as *const u8 as *const i8);
    lua_pushvalue(state, -(2));
    lua_setmetatable(state, -(2));
    lua_settop(state, -1 - 1);
    lua_pushvalue(state, -(2));
    lua_setfield(state, -(2), b"__index\0" as *const u8 as *const i8);
    lua_settop(state, -1 - 1);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_string(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 18]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, STRING_FUNCTIONS.as_ptr(), 0);
    createmetatable(state);
    return 1;
}}
pub unsafe extern "C" fn u_posrelat(pos: i64, length: u64) -> i64 {
    if pos >= 0 {
        return pos;
    } else if (0u64).wrapping_sub(pos as u64) > length {
        return 0;
    } else {
        return length as i64 + pos + 1;
    };
}
pub unsafe extern "C" fn utf8_decode(
    mut s: *const i8,
    value: *mut u32,
    strict: i32,
) -> *const i8 { unsafe {
    static mut LIMITS: [u32; 6] = [
        !(0u32),
        0x80 as i32 as u32,
        0x800 as i32 as u32,
        0x10000 as u32,
        0x200000 as u32,
        0x4000000 as u32,
    ];
    let mut c: u32 = *s.offset(0 as isize) as u8 as u32;
    let mut res: u32 = 0u32;
    if c < 0x80 as i32 as u32 {
        res = c;
    } else {
        let mut count: i32 = 0;
        while c & 0x40 as i32 as u32 != 0 {
            count += 1;
            let cc: u32 = *s.offset(count as isize) as u8 as u32;
            if !(cc & 0xc0 as i32 as u32 == 0x80 as i32 as u32) {
                return std::ptr::null();
            }
            res = res << 6 | cc & 0x3f as u32;
            c <<= 1;
        }
        res |= (c & 0x7f as i32 as u32) << count * 5;
        if count > 5 || res > 0x7fffffff as u32 || res < LIMITS[count as usize] {
            return std::ptr::null();
        }
        s = s.offset(count as isize);
    }
    if strict != 0 {
        if res > 0x10ffff as u32 || 0xd800 as u32 <= res && res <= 0xdfff as u32 {
            return std::ptr::null();
        }
    }
    if !value.is_null() {
        *value = res;
    }
    return s.offset(1 as isize);
}}
pub unsafe extern "C" fn utflen(state: *mut State) -> i32 { unsafe {
    let mut n: i64 = 0;
    let mut length: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut length);
    let mut posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
    let mut posj: i64 = u_posrelat(lual_optinteger(state, 3, -1 as i64), length);
    let lax: i32 = lua_toboolean(state, 4);
    (((1 <= posi && {
        posi -= 1;
        posi <= length as i64
    }) as i32
        != 0) as i32 as i64
        != 0
        || lual_argerror(
            state,
            2,
            b"initial position out of bounds\0" as *const u8 as *const i8,
        ) != 0) as i32;
    posj -= 1;
    (((posj < length as i64) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            3,
            b"final position out of bounds\0" as *const u8 as *const i8,
        ) != 0) as i32;
    while posi <= posj {
        let s1: *const i8 =
            utf8_decode(s.offset(posi as isize), std::ptr::null_mut(), (lax == 0) as i32);
        if s1.is_null() {
            (*state).push_nil();
            (*state).push_integer(posi + 1);
            return 2;
        }
        posi = s1.offset_from(s) as i64 as i64;
        n += 1;
    }
    (*state).push_integer(n);
    return 1;
}}
pub unsafe extern "C" fn codepoint(state: *mut State) -> i32 { unsafe {
    let mut length: u64 = 0;
    let mut s: *const i8 = lual_checklstring(state, 1, &mut length);
    let posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
    let pose: i64 = u_posrelat(lual_optinteger(state, 3, posi), length);
    let lax: i32 = lua_toboolean(state, 4);
    let mut n: i32;
    let se: *const i8;
    (((posi >= 1) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(state, 2, b"out of bounds\0" as *const u8 as *const i8) != 0)
        as i32;
    (((pose <= length as i64) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(state, 3, b"out of bounds\0" as *const u8 as *const i8) != 0)
        as i32;
    if posi > pose {
        return 0;
    }
    if pose - posi >= 2147483647 as i32 as i64 {
        return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
    }
    n = (pose - posi) as i32 + 1;
    lual_checkstack(
        state,
        n,
        b"string slice too long\0" as *const u8 as *const i8,
    );
    n = 0;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1) as isize);
    while s < se {
        let mut code: u32 = 0;
        s = utf8_decode(s, &mut code, (lax == 0) as i32);
        if s.is_null() {
            return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
        }
        (*state).push_integer(code as i64);
        n += 1;
    }
    return n;
}}
pub unsafe extern "C" fn pushutfchar(state: *mut State, arg: i32) { unsafe {
    let code: u64 = lual_checkinteger(state, arg) as u64;
    (((code <= 0x7fffffff as u32 as u64) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            arg,
            b"value out of range\0" as *const u8 as *const i8,
        ) != 0) as i32;
    lua_pushfstring(state, b"%U\0" as *const u8 as *const i8, code as i64);
}}
pub unsafe extern "C" fn utfchar(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    if n == 1 {
        pushutfchar(state, 1);
    } else {
        let mut i: i32;
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        i = 1;
        while i <= n {
            pushutfchar(state, i);
            b.lual_addvalue();
            i += 1;
        }
        b.lual_pushresult();
    }
    return 1;
}}
pub unsafe extern "C" fn byteoffset(state: *mut State) -> i32 { unsafe {
    let mut length: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut length);
    let mut n: i64 = lual_checkinteger(state, 2);
    let mut posi: i64 = (if n >= 0 {
        1 as u64
    } else {
        length.wrapping_add(1 as u64)
    }) as i64;
    posi = u_posrelat(lual_optinteger(state, 3, posi), length);
    (((1 <= posi && {
        posi -= 1;
        posi <= length as i64
    }) as i32
        != 0) as i32 as i64
        != 0
        || lual_argerror(
            state,
            3,
            b"position out of bounds\0" as *const u8 as *const i8,
        ) != 0) as i32;
    if n == 0 {
        while posi > 0 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            posi -= 1;
        }
    } else {
        if *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            return lual_error(
                state,
                b"initial position is a continuation byte\0" as *const u8 as *const i8,
            );
        }
        if n < 0 {
            while n < 0 && posi > 0 {
                loop {
                    posi -= 1;
                    if !(posi > 0
                        && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32)
                    {
                        break;
                    }
                }
                n += 1;
            }
        } else {
            n -= 1;
            while n > 0 && posi < length as i64 {
                loop {
                    posi += 1;
                    if !(*s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32) {
                        break;
                    }
                }
                n -= 1;
            }
        }
    }
    if n == 0 {
        (*state).push_integer(posi + 1);
    } else {
        (*state).push_nil();
    }
    return 1;
}}
pub unsafe extern "C" fn iter_aux(state: *mut State, strict: i32) -> i32 { unsafe {
    let mut length: u64 = 0;
    let s: *const i8 = lual_checklstring(state, 1, &mut length);
    let mut n: u64 = lua_tointegerx(state, 2, std::ptr::null_mut()) as u64;
    if n < length as u64 {
        while *s.offset(n as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            n = n.wrapping_add(1);
        }
    }
    if n >= length as u64 {
        return 0;
    } else {
        let mut code: u32 = 0;
        let next: *const i8 = utf8_decode(s.offset(n as isize), &mut code, strict);
        if next.is_null() || *next as i32 & 0xc0 as i32 == 0x80 as i32 {
            return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
        }
        (*state).push_integer(n.wrapping_add(1 as u64) as i64);
        (*state).push_integer(code as i64);
        return 2;
    };
}}
pub unsafe extern "C" fn iter_auxstrict(state: *mut State) -> i32 { unsafe {
    return iter_aux(state, 1);
}}
pub unsafe extern "C" fn iter_auxlax(state: *mut State) -> i32 { unsafe {
    return iter_aux(state, 0);
}}
pub unsafe extern "C" fn iter_codes(state: *mut State) -> i32 { unsafe {
    let lax: i32 = lua_toboolean(state, 2);
    let s: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    ((!(*s as i32 & 0xc0 as i32 == 0x80 as i32) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            1,
            b"invalid UTF-8 code\0" as *const u8 as *const i8,
        ) != 0) as i32;
    lua_pushcclosure(
        state,
        if lax != 0 {
            Some(iter_auxlax as unsafe extern "C" fn(*mut State) -> i32)
        } else {
            Some(iter_auxstrict as unsafe extern "C" fn(*mut State) -> i32)
        },
        0,
    );
    lua_pushvalue(state, 1);
    (*state).push_integer(0);
    return 3;
}}
static mut UTF8_FUNCTIONS: [RegisteredFunction; 7] = {
    [
        {
            let init = RegisteredFunction {
                name: b"offset\0" as *const u8 as *const i8,
                function: Some(byteoffset as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"codepoint\0" as *const u8 as *const i8,
                function: Some(codepoint as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(utfchar as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(utflen as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"codes\0" as *const u8 as *const i8,
                function: Some(iter_codes as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"charpattern\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_utf8(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, UTF8_FUNCTIONS.as_ptr(), 0);
    lua_pushlstring(
        state,
        b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 15]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(1 as u64),
    );
    lua_setfield(state, -(2), b"charpattern\0" as *const u8 as *const i8);
    return 1;
}}
static mut HOOKKEY: *const i8 = b"_HOOKKEY\0" as *const u8 as *const i8;
pub unsafe extern "C" fn checkstack(state: *mut State, other_state: *mut State, n: i32) { unsafe {
    if ((state != other_state && lua_checkstack(other_state, n) == 0) as i32 != 0) as i32 as i64 != 0 {
        lual_error(state, b"stack overflow\0" as *const u8 as *const i8);
    }
}}
pub unsafe extern "C" fn db_getregistry(state: *mut State) -> i32 { unsafe {
    lua_pushvalue(state, -(1000000 as i32) - 1000 as i32);
    return 1;
}}
pub unsafe extern "C" fn db_getmetatable(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 1);
    if lua_getmetatable(state, 1) == 0 {
        (*state).push_nil();
    }
    return 1;
}}
pub unsafe extern "C" fn db_setmetatable(state: *mut State) -> i32 { unsafe {
    let t: i32 = lua_type(state, 2);
    (((t == 0 || t == 5) as i32 != 0) as i32 as i64 != 0
        || lual_typeerror(state, 2, b"nil or table\0" as *const u8 as *const i8) != 0)
        as i32;
    lua_settop(state, 2);
    lua_setmetatable(state, 1);
    return 1;
}}
pub unsafe extern "C" fn db_getuservalue(state: *mut State) -> i32 { unsafe {
    let n: i32 = lual_optinteger(state, 2, 1) as i32;
    if lua_type(state, 1) != 7 {
        (*state).push_nil();
    } else if lua_getiuservalue(state, 1, n) != -1 {
        (*state).push_boolean(true);
        return 2;
    }
    return 1;
}}
pub unsafe extern "C" fn db_setuservalue(state: *mut State) -> i32 { unsafe {
    let n: i32 = lual_optinteger(state, 3, 1) as i32;
    lual_checktype(state, 1, 7);
    lual_checkany(state, 2);
    lua_settop(state, 2);
    if lua_setiuservalue(state, 1, n) == 0 {
        (*state).push_nil();
    }
    return 1;
}}
pub unsafe extern "C" fn getthread(state: *mut State, arg: *mut i32) -> *mut State { unsafe {
    if lua_type(state, 1) == 8 {
        *arg = 1;
        return lua_tothread(state, 1);
    } else {
        *arg = 0;
        return state;
    };
}}
pub unsafe extern "C" fn settabss(state: *mut State, k: *const i8, v: *const i8) { unsafe {
    lua_pushstring(state, v);
    lua_setfield(state, -(2), k);
}}
pub unsafe extern "C" fn settabsi(state: *mut State, k: *const i8, v: i32) { unsafe {
    (*state).push_integer(v as i64);
    lua_setfield(state, -(2), k);
}}
pub unsafe extern "C" fn settabsb(state: *mut State, k: *const i8, v: i32) { unsafe {
    (*state).push_boolean(v != 0);
    lua_setfield(state, -(2), k);
}}
pub unsafe extern "C" fn treatstackoption(
    state: *mut State,
    other_state: *mut State,
    fname: *const i8,
) { unsafe {
    if state == other_state {
        lua_rotate(state, -(2), 1);
    } else {
        lua_xmove(other_state, state, 1);
    }
    lua_setfield(state, -(2), fname);
}}
pub unsafe extern "C" fn db_getinfo(state: *mut State) -> i32 { unsafe {
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    let mut arg: i32 = 0;
    let other_state: *mut State = getthread(state, &mut arg);
    let mut options: *const i8 = lual_optlstring(
        state,
        arg + 2,
        b"flnSrtu\0" as *const u8 as *const i8,
        std::ptr::null_mut(),
    );
    checkstack(state, other_state, 3);
    (((*options.offset(0 as isize) as i32 != '>' as i32) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            arg + 2,
            b"invalid option '>'\0" as *const u8 as *const i8,
        ) != 0) as i32;
    if lua_type(state, arg + 1) == 6 {
        options = lua_pushfstring(state, b">%s\0" as *const u8 as *const i8, options);
        lua_pushvalue(state, arg + 1);
        lua_xmove(state, other_state, 1);
    } else if lua_getstack(other_state, lual_checkinteger(state, arg + 1) as i32, &mut ar) == 0 {
        (*state).push_nil();
        return 1;
    }
    if lua_getinfo(other_state, options, &mut ar) == 0 {
        return lual_argerror(
            state,
            arg + 2,
            b"invalid option\0" as *const u8 as *const i8,
        );
    }
    lua_createtable(state, 0, 0);
    if !(strchr(options, 'S' as i32)).is_null() {
        lua_pushlstring(state, ar.source, ar.source_length);
        lua_setfield(state, -(2), b"source\0" as *const u8 as *const i8);
        settabss(
            state,
            b"short_src\0" as *const u8 as *const i8,
            (ar.short_src).as_mut_ptr(),
        );
        settabsi(
            state,
            b"linedefined\0" as *const u8 as *const i8,
            ar.line_defined,
        );
        settabsi(
            state,
            b"lastlinedefined\0" as *const u8 as *const i8,
            ar.last_line_defined,
        );
        settabss(state, b"what\0" as *const u8 as *const i8, ar.what);
    }
    if !(strchr(options, 'l' as i32)).is_null() {
        settabsi(
            state,
            b"currentline\0" as *const u8 as *const i8,
            ar.currentline,
        );
    }
    if !(strchr(options, 'u' as i32)).is_null() {
        settabsi(state, b"nups\0" as *const u8 as *const i8, ar.nups as i32);
        settabsi(
            state,
            b"nparams\0" as *const u8 as *const i8,
            ar.nparams as i32,
        );
        settabsb(
            state,
            b"isvararg\0" as *const u8 as *const i8,
            ar.is_variable_arguments as i32,
        );
    }
    if !(strchr(options, 'n' as i32)).is_null() {
        settabss(state, b"name\0" as *const u8 as *const i8, ar.name);
        settabss(state, b"namewhat\0" as *const u8 as *const i8, ar.namewhat);
    }
    if !(strchr(options, 'r' as i32)).is_null() {
        settabsi(
            state,
            b"ftransfer\0" as *const u8 as *const i8,
            ar.ftransfer as i32,
        );
        settabsi(
            state,
            b"ntransfer\0" as *const u8 as *const i8,
            ar.ntransfer as i32,
        );
    }
    if !(strchr(options, 't' as i32)).is_null() {
        settabsb(
            state,
            b"istailcall\0" as *const u8 as *const i8,
            if ar.is_tail_call { 1 } else { 0 },
        );
    }
    if !(strchr(options, 'L' as i32)).is_null() {
        treatstackoption(state, other_state, b"activelines\0" as *const u8 as *const i8);
    }
    if !(strchr(options, 'f' as i32)).is_null() {
        treatstackoption(state, other_state, b"func\0" as *const u8 as *const i8);
    }
    return 1;
}}
pub unsafe extern "C" fn db_getlocal(state: *mut State) -> i32 { unsafe {
    let mut arg: i32 = 0;
    let other_state: *mut State = getthread(state, &mut arg);
    let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
    if lua_type(state, arg + 1) == 6 {
        lua_pushvalue(state, arg + 1);
        lua_pushstring(state, lua_getlocal(state, std::ptr::null(), nvar));
        return 1;
    } else {
        let mut ar: Debug = Debug {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        let level: i32 = lual_checkinteger(state, arg + 1) as i32;
        if ((lua_getstack(other_state, level, &mut ar) == 0) as i32 != 0) as i32 as i64 != 0 {
            return lual_argerror(
                state,
                arg + 1,
                b"level out of range\0" as *const u8 as *const i8,
            );
        }
        checkstack(state, other_state, 1);
        let name: *const i8 = lua_getlocal(other_state, &mut ar, nvar);
        if !name.is_null() {
            lua_xmove(other_state, state, 1);
            lua_pushstring(state, name);
            lua_rotate(state, -(2), 1);
            return 2;
        } else {
            (*state).push_nil();
            return 1;
        }
    };
}}
pub unsafe extern "C" fn db_setlocal(state: *mut State) -> i32 { unsafe {
    let mut arg: i32 = 0;
    let name: *const i8;
    let other_state: *mut State = getthread(state, &mut arg);
    let mut ar: Debug = Debug {
        event: 0,
        name: std::ptr::null(),
        namewhat: std::ptr::null(),
        what: std::ptr::null(),
        source: std::ptr::null(),
        source_length: 0,
        currentline: 0,
        line_defined: 0,
        last_line_defined: 0,
        nups: 0,
        nparams: 0,
        is_variable_arguments: false,
        is_tail_call: false,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: std::ptr::null_mut(),
    };
    let level: i32 = lual_checkinteger(state, arg + 1) as i32;
    let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
    if ((lua_getstack(other_state, level, &mut ar) == 0) as i32 != 0) as i32 as i64 != 0 {
        return lual_argerror(
            state,
            arg + 1,
            b"level out of range\0" as *const u8 as *const i8,
        );
    }
    lual_checkany(state, arg + 3);
    lua_settop(state, arg + 3);
    checkstack(state, other_state, 1);
    lua_xmove(state, other_state, 1);
    name = lua_setlocal(other_state, &mut ar, nvar);
    if name.is_null() {
        lua_settop(other_state, -1 - 1);
    }
    lua_pushstring(state, name);
    return 1;
}}
pub unsafe extern "C" fn auxupvalue(state: *mut State, get: i32) -> i32 { unsafe {
    let n: i32 = lual_checkinteger(state, 2) as i32;
    lual_checktype(state, 1, 6);
    let name: *const i8 = if get != 0 {
        lua_getupvalue(state, 1, n)
    } else {
        lua_setupvalue(state, 1, n)
    };
    if name.is_null() {
        return 0;
    }
    lua_pushstring(state, name);
    lua_rotate(state, -(get + 1), 1);
    return get + 1;
}}
pub unsafe extern "C" fn db_getupvalue(state: *mut State) -> i32 { unsafe {
    return auxupvalue(state, 1);
}}
pub unsafe extern "C" fn db_setupvalue(state: *mut State) -> i32 { unsafe {
    lual_checkany(state, 3);
    return auxupvalue(state, 0);
}}
pub unsafe extern "C" fn checkupval(
    state: *mut State,
    argf: i32,
    argnup: i32,
    pnup: *mut i32,
) -> *mut libc::c_void { unsafe {
    let id: *mut libc::c_void;
    let nup: i32 = lual_checkinteger(state, argnup) as i32;
    lual_checktype(state, argf, 6);
    id = lua_upvalueid(state, argf, nup);
    if !pnup.is_null() {
        (((id != std::ptr::null_mut()) as i32 != 0) as i32 as i64 != 0
            || lual_argerror(
                state,
                argnup,
                b"invalid upvalue index\0" as *const u8 as *const i8,
            ) != 0) as i32;
        *pnup = nup;
    }
    return id;
}}
pub unsafe extern "C" fn db_upvalueid(state: *mut State) -> i32 { unsafe {
    let id: *mut libc::c_void = checkupval(state, 1, 2, std::ptr::null_mut());
    if !id.is_null() {
        lua_pushlightuserdata(state, id);
    } else {
        (*state).push_nil();
    }
    return 1;
}}
pub unsafe extern "C" fn db_upvaluejoin(state: *mut State) -> i32 { unsafe {
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    checkupval(state, 1, 2, &mut n1);
    checkupval(state, 3, 4, &mut n2);
    ((!lua_iscfunction(state, 1))
        || lual_argerror(
            state,
            1,
            b"Lua function expected\0" as *const u8 as *const i8,
        ) != 0) as i32;
    ((!lua_iscfunction(state, 3))
        || lual_argerror(
            state,
            3,
            b"Lua function expected\0" as *const u8 as *const i8,
        ) != 0) as i32;
    lua_upvaluejoin(state, 1, n1, 3, n2);
    return 0;
}}
pub unsafe extern "C" fn hookf(state: *mut State, ar: *mut Debug) { unsafe {
    static mut HOOK_NAMES: [*const i8; 5] = [
        b"call\0" as *const u8 as *const i8,
        b"return\0" as *const u8 as *const i8,
        b"line\0" as *const u8 as *const i8,
        b"count\0" as *const u8 as *const i8,
        b"tail call\0" as *const u8 as *const i8,
    ];
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, HOOKKEY);
    (*state).push_state();
    if lua_rawget(state, -(2)) == 6 {
        lua_pushstring(state, HOOK_NAMES[(*ar).event as usize]);
        if (*ar).currentline >= 0 {
            (*state).push_integer((*ar).currentline as i64);
        } else {
            (*state).push_nil();
        }
        lua_callk(state, 2, 0, 0, None);
    }
}}
pub unsafe extern "C" fn makemask(smask: *const i8, count: i32) -> i32 { unsafe {
    let mut mask: i32 = 0;
    if !(strchr(smask, 'c' as i32)).is_null() {
        mask |= 1 << 0;
    }
    if !(strchr(smask, 'r' as i32)).is_null() {
        mask |= 1 << 1;
    }
    if !(strchr(smask, 'l' as i32)).is_null() {
        mask |= 1 << 2;
    }
    if count > 0 {
        mask |= 1 << 3;
    }
    return mask;
}}
pub unsafe extern "C" fn unmakemask(mask: i32, smask: *mut i8) -> *mut i8 { unsafe {
    let mut i: i32 = 0;
    if mask & 1 << 0 != 0 {
        let fresh190 = i;
        i = i + 1;
        *smask.offset(fresh190 as isize) = 'c' as i8;
    }
    if mask & 1 << 1 != 0 {
        let fresh191 = i;
        i = i + 1;
        *smask.offset(fresh191 as isize) = 'r' as i8;
    }
    if mask & 1 << 2 != 0 {
        let fresh192 = i;
        i = i + 1;
        *smask.offset(fresh192 as isize) = 'l' as i8;
    }
    *smask.offset(i as isize) = '\0' as i8;
    return smask;
}}
pub unsafe extern "C" fn db_sethook(state: *mut State) -> i32 { unsafe {
    let mut arg: i32 = 0;
    let mask: i32;
    let count: i32;
    let function: HookFunction;
    let other_state: *mut State = getthread(state, &mut arg);
    if lua_type(state, arg + 1) <= 0 {
        lua_settop(state, arg + 1);
        function = None;
        mask = 0;
        count = 0;
    } else {
        let smask: *const i8 = lual_checklstring(state, arg + 2, std::ptr::null_mut());
        lual_checktype(state, arg + 1, 6);
        count = lual_optinteger(state, arg + 3, 0) as i32;
        function = Some(hookf as unsafe extern "C" fn(*mut State, *mut Debug) -> ());
        mask = makemask(smask, count);
    }
    if lual_getsubtable(state, -(1000000 as i32) - 1000 as i32, HOOKKEY) == 0 {
        lua_pushstring(state, b"k\0" as *const u8 as *const i8);
        lua_setfield(state, -(2), b"__mode\0" as *const u8 as *const i8);
        lua_pushvalue(state, -1);
        lua_setmetatable(state, -(2));
    }
    checkstack(state, other_state, 1);
    (*other_state).push_state();
    lua_xmove(other_state, state, 1);
    lua_pushvalue(state, arg + 1);
    lua_rawset(state, -(3));
    lua_sethook(other_state, function, mask, count);
    return 0;
}}
pub unsafe extern "C" fn db_gethook(state: *mut State) -> i32 { unsafe {
    let mut arg: i32 = 0;
    let other_state: *mut State = getthread(state, &mut arg);
    let mut buffer: [i8; 5] = [0; 5];
    let mask: i32 = lua_gethookmask(other_state);
    let hook: HookFunction = lua_gethook(other_state);
    if hook.is_none() {
        (*state).push_nil();
        return 1;
    } else if hook != Some(hookf as unsafe extern "C" fn(*mut State, *mut Debug) -> ()) {
        lua_pushstring(state, b"external hook\0" as *const u8 as *const i8);
    } else {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, HOOKKEY);
        checkstack(state, other_state, 1);
        (*other_state).push_state();
        lua_xmove(other_state, state, 1);
        lua_rawget(state, -(2));
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
    }
    lua_pushstring(state, unmakemask(mask, buffer.as_mut_ptr()));
    (*state).push_integer(lua_gethookcount(other_state) as i64);
    return 3;
}}
pub unsafe extern "C" fn db_debug(state: *mut State) -> i32 { unsafe {
    loop {
        let mut buffer: [i8; 250] = [0; 250];
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const i8,
            b"lua_debug> \0" as *const u8 as *const i8,
        );
        fflush(stderr);
        if (fgets(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 250]>() as u64 as i32,
            stdin,
        ))
        .is_null()
            || strcmp(buffer.as_mut_ptr(), b"cont\n\0" as *const u8 as *const i8) == 0
        {
            return 0;
        }
        if lual_loadbufferx(
            state,
            buffer.as_mut_ptr(),
            strlen(buffer.as_mut_ptr()),
            b"=(debug command)\0" as *const u8 as *const i8,
            std::ptr::null(),
        ) != 0
            || lua_pcallk(state, 0, 0, 0, 0, None) != 0
        {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const i8,
                lual_tolstring(state, -1, std::ptr::null_mut()),
            );
            fflush(stderr);
        }
        lua_settop(state, 0);
    }
}}
pub unsafe extern "C" fn db_traceback(state: *mut State) -> i32 { unsafe {
    let mut arg: i32 = 0;
    let other_state: *mut State = getthread(state, &mut arg);
    let message: *const i8 = lua_tolstring(state, arg + 1, std::ptr::null_mut());
    if message.is_null() && !(lua_type(state, arg + 1) <= 0) {
        lua_pushvalue(state, arg + 1);
    } else {
        let level: i32 = lual_optinteger(
            state,
            arg + 2,
            (if state == other_state { 1 } else { 0 }) as i64,
        ) as i32;
        lual_traceback(state, other_state, message, level);
    }
    return 1;
}}
pub unsafe extern "C" fn db_setcstacklimit(state: *mut State) -> i32 { unsafe {
    let limit: i32 = lual_checkinteger(state, 1) as i32;
    let res: i32 = lua_setcstacklimit(state, limit as u32);
    (*state).push_integer(res as i64);
    return 1;
}}
static mut DEBUG_FUNCTIONS: [RegisteredFunction; 18] = {
    [
        {
            let init = RegisteredFunction {
                name: b"debug\0" as *const u8 as *const i8,
                function: Some(db_debug as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getuservalue\0" as *const u8 as *const i8,
                function: Some(db_getuservalue as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"gethook\0" as *const u8 as *const i8,
                function: Some(db_gethook as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getinfo\0" as *const u8 as *const i8,
                function: Some(db_getinfo as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getlocal\0" as *const u8 as *const i8,
                function: Some(db_getlocal as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getregistry\0" as *const u8 as *const i8,
                function: Some(db_getregistry as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(db_getmetatable as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"getupvalue\0" as *const u8 as *const i8,
                function: Some(db_getupvalue as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"upvaluejoin\0" as *const u8 as *const i8,
                function: Some(db_upvaluejoin as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"upvalueid\0" as *const u8 as *const i8,
                function: Some(db_upvalueid as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setuservalue\0" as *const u8 as *const i8,
                function: Some(db_setuservalue as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"sethook\0" as *const u8 as *const i8,
                function: Some(db_sethook as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setlocal\0" as *const u8 as *const i8,
                function: Some(db_setlocal as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(db_setmetatable as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setupvalue\0" as *const u8 as *const i8,
                function: Some(db_setupvalue as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"traceback\0" as *const u8 as *const i8,
                function: Some(db_traceback as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"setcstacklimit\0" as *const u8 as *const i8,
                function: Some(db_setcstacklimit as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_debug(state: *mut State) -> i32 { unsafe {
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 18]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, DEBUG_FUNCTIONS.as_ptr(), 0);
    return 1;
}}
static mut CLIBS: *const i8 = b"_CLIBS\0" as *const u8 as *const i8;
pub unsafe extern "C" fn lsys_unloadlib(lib: *mut libc::c_void) { unsafe {
    dlclose(lib);
}}
pub unsafe extern "C" fn lsys_load(
    state: *mut State,
    path: *const i8,
    seeglb: i32,
) -> *mut libc::c_void { unsafe {
    let lib: *mut libc::c_void = dlopen(
        path,
        0x2 as i32 | (if seeglb != 0 { 0x100 as i32 } else { 0 }),
    );
    if ((lib == std::ptr::null_mut()) as i32 != 0) as i32 as i64 != 0 {
        lua_pushstring(state, dlerror());
    }
    return lib;
}}
pub unsafe extern "C" fn lsys_sym(
    state: *mut State,
    lib: *mut libc::c_void,
    sym: *const i8,
) -> CFunction { unsafe {
    let f: CFunction = ::core::mem::transmute::<*mut libc::c_void, CFunction>(dlsym(lib, sym));
    if (f.is_none() as i32 != 0) as i32 as i64 != 0 {
        lua_pushstring(state, dlerror());
    }
    return f;
}}
pub unsafe extern "C" fn noenv(state: *mut State) -> i32 { unsafe {
    let b: i32;
    lua_getfield(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"LUA_NOENV\0" as *const u8 as *const i8,
    );
    b = lua_toboolean(state, -1);
    lua_settop(state, -1 - 1);
    return b;
}}
pub unsafe extern "C" fn setpath(
    state: *mut State,
    fieldname: *const i8,
    envname: *const i8,
    dft: *const i8,
) { unsafe {
    let dftmark: *const i8;
    let nver: *const i8 = lua_pushfstring(
        state,
        b"%s%s\0" as *const u8 as *const i8,
        envname,
        b"_5_4\0" as *const u8 as *const i8,
    );
    let mut path: *const i8 = getenv(nver);
    if path.is_null() {
        path = getenv(envname);
    }
    if path.is_null() || noenv(state) != 0 {
        lua_pushstring(state, dft);
    } else {
        dftmark = strstr(path, b";;\0" as *const u8 as *const i8);
        if dftmark.is_null() {
            lua_pushstring(state, path);
        } else {
            let length: u64 = strlen(path);
            let mut b = Buffer::new();
            b.lual_buffinit(state);
            if path < dftmark {
                b.lual_addlstring(path, dftmark.offset_from(path) as i64 as u64);
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh193 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh193 as isize) = *(b";\0" as *const u8 as *const i8);
            }
            b.lual_addstring(dft);
            if dftmark < path.offset(length as isize).offset(-(2 as isize)) {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh194 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh194 as isize) = *(b";\0" as *const u8 as *const i8);
                b.lual_addlstring(
                    dftmark.offset(2 as isize),
                    path.offset(length as isize)
                        .offset(-(2 as isize))
                        .offset_from(dftmark) as i64 as u64,
                );
            }
            b.lual_pushresult();
        }
    }
    lua_setfield(state, -(3), fieldname);
    lua_settop(state, -1 - 1);
}}
pub unsafe extern "C" fn checkclib(
    state: *mut State,
    path: *const i8,
) -> *mut libc::c_void { unsafe {
    let plib: *mut libc::c_void;
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_getfield(state, -1, path);
    plib = lua_touserdata(state, -1);
    lua_settop(state, -(2) - 1);
    return plib;
}}
pub unsafe extern "C" fn addtoclib(
    state: *mut State,
    path: *const i8,
    plib: *mut libc::c_void,
) { unsafe {
    lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_pushlightuserdata(state, plib);
    lua_pushvalue(state, -1);
    lua_setfield(state, -(3), path);
    lua_rawseti(state, -(2), lual_len(state, -(2)) + 1);
    lua_settop(state, -1 - 1);
}}
pub unsafe extern "C" fn gctm(state: *mut State) -> i32 { unsafe {
    let mut n: i64 = lual_len(state, 1);
    while n >= 1 {
        lua_rawgeti(state, 1, n);
        lsys_unloadlib(lua_touserdata(state, -1));
        lua_settop(state, -1 - 1);
        n -= 1;
    }
    return 0;
}}
pub unsafe extern "C" fn lookforfunc(
    state: *mut State,
    path: *const i8,
    sym: *const i8,
) -> i32 { unsafe {
    let mut reg: *mut libc::c_void = checkclib(state, path);
    if reg.is_null() {
        reg = lsys_load(state, path, (*sym as i32 == '*' as i32) as i32);
        if reg.is_null() {
            return 1;
        }
        addtoclib(state, path, reg);
    }
    if *sym as i32 == '*' as i32 {
        (*state).push_boolean(true);
        return 0;
    } else {
        let f: CFunction = lsys_sym(state, reg, sym);
        if f.is_none() {
            return 2;
        }
        lua_pushcclosure(state, f, 0);
        return 0;
    };
}}
pub unsafe extern "C" fn ll_loadlib(state: *mut State) -> i32 { unsafe {
    let path: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let init: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
    let stat: i32 = lookforfunc(state, path, init);
    if ((stat == 0) as i32 != 0) as i32 as i64 != 0 {
        return 1;
    } else {
        (*state).push_nil();
        lua_rotate(state, -(2), 1);
        lua_pushstring(
            state,
            if stat == 1 {
                b"open\0" as *const u8 as *const i8
            } else {
                b"init\0" as *const u8 as *const i8
            },
        );
        return 3;
    };
}}
pub unsafe extern "C" fn readable(filename: *const i8) -> i32 { unsafe {
    let f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const i8);
    if f.is_null() {
        return 0;
    }
    fclose(f);
    return 1;
}}
pub unsafe extern "C" fn getnextfilename(path: *mut *mut i8, end: *mut i8) -> *const i8 { unsafe {
    let mut name: *mut i8 = *path;
    if name == end {
        return std::ptr::null();
    } else if *name as i32 == '\0' as i32 {
        *name = *(b";\0" as *const u8 as *const i8);
        name = name.offset(1);
    }
    let mut sep: *mut i8 = strchr(name, *(b";\0" as *const u8 as *const i8) as i32);
    if sep.is_null() {
        sep = end;
    }
    *sep = '\0' as i8;
    *path = sep;
    return name;
}}
pub unsafe extern "C" fn pusherrornotfound(state: *mut State, path: *const i8) { unsafe {
    let mut b = Buffer::new();
    b.lual_buffinit(state);
    b.lual_addstring(b"no file '\0" as *const u8 as *const i8);
    lual_addgsub(
        &mut b,
        path,
        b";\0" as *const u8 as *const i8,
        b"'\n\tno file '\0" as *const u8 as *const i8,
    );
    b.lual_addstring(b"'\0" as *const u8 as *const i8);
    b.lual_pushresult();
}}
pub unsafe extern "C" fn searchpath(
    state: *mut State,
    mut name: *const i8,
    path: *const i8,
    sep: *const i8,
    dirsep: *const i8,
) -> *const i8 { unsafe {
    let mut pathname;
    let endpathname;
    let mut filename;
    if *sep as i32 != '\0' as i32 && !(strchr(name, *sep as i32)).is_null() {
        name = lual_gsub(state, name, sep, dirsep);
    }
    let mut buffer = Buffer::new ();
    buffer.lual_buffinit(state);
    lual_addgsub(&mut buffer, path, b"?\0" as *const u8 as *const i8, name);
    (buffer.length < buffer.size || !(buffer.lual_prepbuffsize(1 as u64)).is_null()) as i32;
    let fresh195 = buffer.length;
    buffer.length = (buffer.length).wrapping_add(1);
    *(buffer.pointer).offset(fresh195 as isize) = '\0' as i8;
    pathname = buffer.pointer;
    endpathname = pathname
        .offset(buffer.length as isize)
        .offset(-(1 as isize));
    loop {
        filename = getnextfilename(&mut pathname, endpathname);
        if filename.is_null() {
            break;
        }
        if readable(filename) != 0 {
            return lua_pushstring(state, filename);
        }
    }
    buffer.lual_pushresult();
    pusherrornotfound(state, lua_tolstring(state, -1, std::ptr::null_mut()));
    return std::ptr::null();
}}
pub unsafe extern "C" fn ll_searchpath(state: *mut State) -> i32 { unsafe {
    let f: *const i8 = searchpath(
        state,
        lual_checklstring(state, 1, std::ptr::null_mut()),
        lual_checklstring(state, 2, std::ptr::null_mut()),
        lual_optlstring(state, 3, b".\0" as *const u8 as *const i8, std::ptr::null_mut()),
        lual_optlstring(state, 4, b"/\0" as *const u8 as *const i8, std::ptr::null_mut()),
    );
    if !f.is_null() {
        return 1;
    } else {
        (*state).push_nil();
        lua_rotate(state, -(2), 1);
        return 2;
    };
}}
pub unsafe extern "C" fn findfile(
    state: *mut State,
    name: *const i8,
    pname: *const i8,
    dirsep: *const i8,
) -> *const i8 { unsafe {
    lua_getfield(state, -(1000000 as i32) - 1000 as i32 - 1, pname);
    let path: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
    if ((path == std::ptr::null_mut() as *const i8) as i32 != 0) as i32 as i64 != 0 {
        lual_error(
            state,
            b"'package.%s' must be a string\0" as *const u8 as *const i8,
            pname,
        );
    }
    return searchpath(state, name, path, b".\0" as *const u8 as *const i8, dirsep);
}}
pub unsafe extern "C" fn checkload(
    state: *mut State,
    stat: i32,
    filename: *const i8,
) -> i32 { unsafe {
    if (stat != 0) as i32 as i64 != 0 {
        lua_pushstring(state, filename);
        return 2;
    } else {
        return lual_error(
            state,
            b"error loading module '%s' from file '%s':\n\t%s\0" as *const u8 as *const i8,
            lua_tolstring(state, 1, std::ptr::null_mut()),
            filename,
            lua_tolstring(state, -1, std::ptr::null_mut()),
        );
    };
}}
pub unsafe extern "C" fn searcher_lua(state: *mut State) -> i32 { unsafe {
    let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let filename: *const i8 = findfile(
        state,
        name,
        b"path\0" as *const u8 as *const i8,
        b"/\0" as *const u8 as *const i8,
    );
    if filename.is_null() {
        return 1;
    }
    return checkload(
        state,
        (lual_loadfilex(state, filename, std::ptr::null()) == 0) as i32,
        filename,
    );
}}
pub unsafe extern "C" fn loadfunc(
    state: *mut State,
    filename: *const i8,
    mut modname: *const i8,
) -> i32 { unsafe {
    modname = lual_gsub(
        state,
        modname,
        b".\0" as *const u8 as *const i8,
        b"_\0" as *const u8 as *const i8,
    );
    let mut openfunc: *const i8;
    let mark: *const i8 = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
    if !mark.is_null() {
        openfunc = lua_pushlstring(state, modname, mark.offset_from(modname) as i64 as u64);
        openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, openfunc);
        let stat: i32 = lookforfunc(state, filename, openfunc);
        if stat != 2 {
            return stat;
        }
        modname = mark.offset(1 as isize);
    }
    openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, modname);
    return lookforfunc(state, filename, openfunc);
}}
pub unsafe extern "C" fn searcher_c(state: *mut State) -> i32 { unsafe {
    let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let filename: *const i8 = findfile(
        state,
        name,
        b"cpath\0" as *const u8 as *const i8,
        b"/\0" as *const u8 as *const i8,
    );
    if filename.is_null() {
        return 1;
    }
    return checkload(
        state,
        (loadfunc(state, filename, name) == 0) as i32,
        filename,
    );
}}
pub unsafe extern "C" fn searcher_croot(state: *mut State) -> i32 { unsafe {
    let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    let p: *const i8 = strchr(name, '.' as i32);
    if p.is_null() {
        return 0;
    }
    lua_pushlstring(state, name, p.offset_from(name) as i64 as u64);
    let filename: *const i8 = findfile(
        state,
        lua_tolstring(state, -1, std::ptr::null_mut()),
        b"cpath\0" as *const u8 as *const i8,
        b"/\0" as *const u8 as *const i8,
    );
    if filename.is_null() {
        return 1;
    }
    let stat: i32 = loadfunc(state, filename, name);
    if stat != 0 {
        if stat != 2 {
            return checkload(state, 0, filename);
        } else {
            lua_pushfstring(
                state,
                b"no module '%s' in file '%s'\0" as *const u8 as *const i8,
                name,
                filename,
            );
            return 1;
        }
    }
    lua_pushstring(state, filename);
    return 2;
}}
pub unsafe extern "C" fn searcher_preload(state: *mut State) -> i32 { unsafe {
    let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    lua_getfield(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_PRELOAD\0" as *const u8 as *const i8,
    );
    if lua_getfield(state, -1, name) == 0 {
        lua_pushfstring(
            state,
            b"no field package.preload['%s']\0" as *const u8 as *const i8,
            name,
        );
        return 1;
    } else {
        lua_pushstring(state, b":preload:\0" as *const u8 as *const i8);
        return 2;
    };
}}
pub unsafe extern "C" fn findloader(state: *mut State, name: *const i8) { unsafe {
    let mut i: i32;
    let mut message = Buffer::new();
    if ((lua_getfield(
        state,
        -(1000000 as i32) - 1000 as i32 - 1,
        b"searchers\0" as *const u8 as *const i8,
    ) != 5) as i32
        != 0) as i32 as i64
        != 0
    {
        lual_error(
            state,
            b"'package.searchers' must be a table\0" as *const u8 as *const i8,
        );
    }
    message.lual_buffinit(state);
    i = 1;
    loop {
        message.lual_addstring(b"\n\t\0" as *const u8 as *const i8);
        if ((lua_rawgeti(state, 3, i as i64) == 0) as i32 != 0) as i32 as i64 != 0 {
            lua_settop(state, -1 - 1);
            message.length = (message.length as u64).wrapping_sub(2 as u64) as u64 as u64;
            message.lual_pushresult();
            lual_error(
                state,
                b"module '%s' not found:%s\0" as *const u8 as *const i8,
                name,
                lua_tolstring(state, -1, std::ptr::null_mut()),
            );
        }
        lua_pushstring(state, name);
        lua_callk(state, 1, 2, 0, None);
        if lua_type(state, -(2)) == 6 {
            return;
        } else if lua_isstring(state, -(2)) {
            lua_settop(state, -1 - 1);
            message.lual_addvalue();
        } else {
            lua_settop(state, -(2) - 1);
            message.length = (message.length as u64).wrapping_sub(2 as u64) as u64 as u64;
        }
        i += 1;
    }
}}
pub unsafe extern "C" fn ll_require(state: *mut State) -> i32 { unsafe {
    let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
    lua_settop(state, 1);
    lua_getfield(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const i8,
    );
    lua_getfield(state, 2, name);
    if lua_toboolean(state, -1) != 0 {
        return 1;
    }
    lua_settop(state, -1 - 1);
    findloader(state, name);
    lua_rotate(state, -(2), 1);
    lua_pushvalue(state, 1);
    lua_pushvalue(state, -(3));
    lua_callk(state, 2, 1, 0, None);
    if !(lua_type(state, -1) == 0) {
        lua_setfield(state, 2, name);
    } else {
        lua_settop(state, -1 - 1);
    }
    if lua_getfield(state, 2, name) == 0 {
        (*state).push_boolean(true);
        lua_copy(state, -1, -(2));
        lua_setfield(state, 2, name);
    }
    lua_rotate(state, -(2), 1);
    return 2;
}}
static mut PACKAGE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            let init = RegisteredFunction {
                name: b"loadlib\0" as *const u8 as *const i8,
                function: Some(ll_loadlib as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"searchpath\0" as *const u8 as *const i8,
                function: Some(ll_searchpath as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"preload\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"cpath\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"path\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"searchers\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"loaded\0" as *const u8 as *const i8,
                function: None,
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
static mut LL_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            let init = RegisteredFunction {
                name: b"require\0" as *const u8 as *const i8,
                function: Some(ll_require as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn createsearcherstable(state: *mut State) { unsafe {
    static mut SEARCHERS: [CFunction; 5] = {
        [
            Some(searcher_preload as unsafe extern "C" fn(*mut State) -> i32),
            Some(searcher_lua as unsafe extern "C" fn(*mut State) -> i32),
            Some(searcher_c as unsafe extern "C" fn(*mut State) -> i32),
            Some(searcher_croot as unsafe extern "C" fn(*mut State) -> i32),
            None,
        ]
    };
    let mut i: i32;
    lua_createtable(
        state,
        (::core::mem::size_of::<[CFunction; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<CFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
        0,
    );
    i = 0;
    while (SEARCHERS[i as usize]).is_some() {
        lua_pushvalue(state, -(2));
        lua_pushcclosure(state, SEARCHERS[i as usize], 1);
        lua_rawseti(state, -(2), (i + 1) as i64);
        i += 1;
    }
    lua_setfield(state, -(2), b"searchers\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn createclibstable(state: *mut State) { unsafe {
    lual_getsubtable(state, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_createtable(state, 0, 1);
    lua_pushcclosure(
        state,
        Some(gctm as unsafe extern "C" fn(*mut State) -> i32),
        0,
    );
    lua_setfield(state, -(2), b"__gc\0" as *const u8 as *const i8);
    lua_setmetatable(state, -(2));
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_package(state: *mut State) -> i32 { unsafe {
    createclibstable(state);
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    lua_createtable(
        state,
        0,
        (::core::mem::size_of::<[RegisteredFunction; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<RegisteredFunction>() as u64)
            .wrapping_sub(1 as u64) as i32,
    );
    lual_setfuncs(state, PACKAGE_FUNCTIONS.as_ptr(), 0);
    createsearcherstable(state);
    setpath(
        state,
        b"path\0" as *const u8 as *const i8,
        b"LUA_PATH\0" as *const u8 as *const i8,
        b"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const i8,
    );
    setpath(
        state,
        b"cpath\0" as *const u8 as *const i8,
        b"LUA_CPATH\0" as *const u8 as *const i8,
        b"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0" as *const u8
            as *const i8,
    );
    lua_pushstring(state, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const i8);
    lua_setfield(state, -(2), b"config\0" as *const u8 as *const i8);
    lual_getsubtable(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const i8,
    );
    lua_setfield(state, -(2), b"loaded\0" as *const u8 as *const i8);
    lual_getsubtable(
        state,
        -(1000000 as i32) - 1000 as i32,
        b"_PRELOAD\0" as *const u8 as *const i8,
    );
    lua_setfield(state, -(2), b"preload\0" as *const u8 as *const i8);
    lua_rawgeti(state, -(1000000 as i32) - 1000 as i32, 2 as i64);
    lua_pushvalue(state, -(2));
    lual_setfuncs(state, LL_FUNCTIONS.as_ptr(), 1);
    lua_settop(state, -1 - 1);
    return 1;
}}
static mut LOADED_FUNCTIONS: [RegisteredFunction; 11] = {
    [
        {
            let init = RegisteredFunction {
                name: b"_G\0" as *const u8 as *const i8,
                function: Some(luaopen_base as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"package\0" as *const u8 as *const i8,
                function: Some(luaopen_package as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"coroutine\0" as *const u8 as *const i8,
                function: Some(luaopen_coroutine as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"table\0" as *const u8 as *const i8,
                function: Some(luaopen_table as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"io\0" as *const u8 as *const i8,
                function: Some(luaopen_io as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"os\0" as *const u8 as *const i8,
                function: Some(luaopen_os as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"string\0" as *const u8 as *const i8,
                function: Some(luaopen_string as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"math\0" as *const u8 as *const i8,
                function: Some(luaopen_math as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"utf8\0" as *const u8 as *const i8,
                function: Some(luaopen_utf8 as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: b"debug\0" as *const u8 as *const i8,
                function: Some(luaopen_debug as unsafe extern "C" fn(*mut State) -> i32),
            };
            init
        },
        {
            let init = RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lual_openlibs(state: *mut State) { unsafe {
    let mut lib: *const RegisteredFunction = LOADED_FUNCTIONS.as_ptr();
    while ((*lib).function).is_some() {
        lual_requiref(state, (*lib).name, (*lib).function, 1);
        lua_settop(state, -1 - 1);
        lib = lib.offset(1);
    }
}}
static mut GLOBAL_STATE: *mut State = std::ptr::null_mut();
static mut PROGRAM_NAME: *const i8 = b"lua\0" as *const u8 as *const i8;
pub unsafe extern "C" fn setsignal(
    sig: i32,
    handler: Option<unsafe extern "C" fn(i32) -> ()>,
) { unsafe {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: SigActionA { sa_handler: None },
        sa_mask: SIgnalSet { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sa.__sigaction_handler.sa_handler = handler;
    sa.sa_flags = 0;
    sigemptyset(&mut sa.sa_mask);
    sigaction(sig, &mut sa, std::ptr::null_mut());
}}
pub unsafe extern "C" fn lstop(state: *mut State, mut _ar: *mut Debug) { unsafe {
    lua_sethook(state, None, 0, 0);
    lual_error(state, b"interrupted!\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn laction(i: i32) { unsafe {
    let flag: i32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
    setsignal(i, None);
    lua_sethook(
        GLOBAL_STATE,
        Some(lstop as unsafe extern "C" fn(*mut State, *mut Debug) -> ()),
        flag,
        1,
    );
}}
pub unsafe extern "C" fn print_usage(badoption: *const i8) { unsafe {
    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, PROGRAM_NAME);
    fflush(stderr);
    if *badoption.offset(1 as isize) as i32 == 'e' as i32
        || *badoption.offset(1 as isize) as i32 == 'l' as i32
    {
        fprintf(
            stderr,
            b"'%s' needs argument\n\0" as *const u8 as *const i8,
            badoption,
        );
        fflush(stderr);
    } else {
        fprintf(
            stderr,
            b"unrecognized option '%s'\n\0" as *const u8 as *const i8,
            badoption,
        );
        fflush(stderr);
    }
    fprintf(
        stderr,
        b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l g=mod  require library 'mod' into global 'g'\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  -         stop handling options and execute stdin\n\0"
            as *const u8 as *const i8,
        PROGRAM_NAME,
    );
    fflush(stderr);
}}
pub unsafe extern "C" fn l_message(pname: *const i8, message: *const i8) { unsafe {
    if !pname.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, pname);
        fflush(stderr);
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, message);
    fflush(stderr);
}}
pub unsafe extern "C" fn report(state: *mut State, status: i32) -> i32 { unsafe {
    if status != 0 {
        let mut message: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
        if message.is_null() {
            message = b"(error message not a string)\0" as *const u8 as *const i8;
        }
        l_message(PROGRAM_NAME, message);
        lua_settop(state, -1 - 1);
    }
    return status;
}}
pub unsafe extern "C" fn msghandler(state: *mut State) -> i32 { unsafe {
    let mut message: *const i8 = lua_tolstring(state, 1, std::ptr::null_mut());
    if message.is_null() {
        if lual_callmeta(state, 1, b"__tostring\0" as *const u8 as *const i8) != 0
            && lua_type(state, -1) == 4
        {
            return 1;
        } else {
            message = lua_pushfstring(
                state,
                b"(error object is a %s value)\0" as *const u8 as *const i8,
                lua_typename(state, lua_type(state, 1)),
            );
        }
    }
    lual_traceback(state, state, message, 1);
    return 1;
}}
pub unsafe extern "C" fn docall(state: *mut State, narg: i32, nres: i32) -> i32 { unsafe {
    let status: i32;
    let base: i32 = (*state).get_top() - narg;
    lua_pushcclosure(
        state,
        Some(msghandler as unsafe extern "C" fn(*mut State) -> i32),
        0,
    );
    lua_rotate(state, base, 1);
    GLOBAL_STATE = state;
    setsignal(2, Some(laction as unsafe extern "C" fn(i32) -> ()));
    status = lua_pcallk(state, narg, nres, base, 0, None);
    setsignal(2, None);
    lua_rotate(state, base, -1);
    lua_settop(state, -1 - 1);
    return status;
}}
pub unsafe extern "C" fn print_version() { unsafe {
    fwrite(
        b"Lua 5.4.8  Copyright (C) 1994-2025 Lua.org, PUC-Rio\0" as *const u8 as *const i8
            as *const libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        strlen(b"Lua 5.4.8  Copyright (C) 1994-2025 Lua.org, PUC-Rio\0" as *const u8 as *const i8),
        stdout,
    );
    fwrite(
        b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        1 as u64,
        stdout,
    );
    fflush(stdout);
}}
pub unsafe extern "C" fn createargtable(
    state: *mut State,
    argv: *mut *mut i8,
    argc: i32,
    script: i32,
) { unsafe {
    let mut i: i32;
    let narg: i32 = argc - (script + 1);
    lua_createtable(state, narg, script + 1);
    i = 0;
    while i < argc {
        lua_pushstring(state, *argv.offset(i as isize));
        lua_rawseti(state, -(2), (i - script) as i64);
        i += 1;
    }
    lua_setglobal(state, b"arg\0" as *const u8 as *const i8);
}}
pub unsafe extern "C" fn dochunk(state: *mut State, mut status: i32) -> i32 { unsafe {
    if status == 0 {
        status = docall(state, 0, 0);
    }
    return report(state, status);
}}
pub unsafe extern "C" fn dofile(state: *mut State, name: *const i8) -> i32 { unsafe {
    return dochunk(state, lual_loadfilex(state, name, std::ptr::null()));
}}
pub unsafe extern "C" fn dostring(
    state: *mut State,
    s: *const i8,
    name: *const i8,
) -> i32 { unsafe {
    return dochunk(
        state,
        lual_loadbufferx(state, s, strlen(s), name, std::ptr::null()),
    );
}}
pub unsafe extern "C" fn dolibrary(state: *mut State, globname: *mut i8) -> i32 { unsafe {
    let status: i32;
    let mut suffix: *mut i8 = std::ptr::null_mut();
    let mut modname: *mut i8 = strchr(globname, '=' as i32);
    if modname.is_null() {
        modname = globname;
        suffix = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
    } else {
        *modname = '\0' as i8;
        modname = modname.offset(1);
    }
    lua_getglobal(state, b"require\0" as *const u8 as *const i8);
    lua_pushstring(state, modname);
    status = docall(state, 1, 1);
    if status == 0 {
        if !suffix.is_null() {
            *suffix = '\0' as i8;
        }
        lua_setglobal(state, globname);
    }
    return report(state, status);
}}
pub unsafe extern "C" fn pushargs(state: *mut State) -> i32 { unsafe {
    let mut i: i32;
    let n: i32;
    if lua_getglobal(state, b"arg\0" as *const u8 as *const i8) != 5 {
        lual_error(state, b"'arg' is not a table\0" as *const u8 as *const i8);
    }
    n = lual_len(state, -1) as i32;
    lual_checkstack(
        state,
        n + 3,
        b"too many arguments to script\0" as *const u8 as *const i8,
    );
    i = 1;
    while i <= n {
        lua_rawgeti(state, -i, i as i64);
        i += 1;
    }
    lua_rotate(state, -i, -1);
    lua_settop(state, -1 - 1);
    return n;
}}
pub unsafe extern "C" fn handle_script(state: *mut State, argv: *mut *mut i8) -> i32 { unsafe {
    let mut status: i32;
    let mut fname: *const i8 = *argv.offset(0 as isize);
    if strcmp(fname, b"-\0" as *const u8 as *const i8) == 0
        && strcmp(*argv.offset(-1 as isize), b"--\0" as *const u8 as *const i8) != 0
    {
        fname = std::ptr::null();
    }
    status = lual_loadfilex(state, fname, std::ptr::null());
    if status == 0 {
        let n: i32 = pushargs(state);
        status = docall(state, n, -1);
    }
    return report(state, status);
}}
pub unsafe extern "C" fn collectargs(argv: *mut *mut i8, first: *mut i32) -> i32 { unsafe {
    let mut args: i32 = 0;
    let mut i: i32;
    if !(*argv.offset(0 as isize)).is_null() {
        if *(*argv.offset(0 as isize)).offset(0 as isize) != 0 {
            PROGRAM_NAME = *argv.offset(0 as isize);
        }
    } else {
        *first = -1;
        return 0;
    }
    i = 1;
    while !(*argv.offset(i as isize)).is_null() {
        *first = i;
        if *(*argv.offset(i as isize)).offset(0 as isize) as i32 != '-' as i32 {
            return args;
        }
        let current_block_31: u64;
        match *(*argv.offset(i as isize)).offset(1 as isize) as i32 {
            45 => {
                if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                    return 1;
                }
                *first = i + 1;
                return args;
            }
            0 => return args,
            69 => {
                if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                    return 1;
                }
                args |= 16 as i32;
                current_block_31 = 4761528863920922185;
            }
            87 => {
                if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                    return 1;
                }
                current_block_31 = 4761528863920922185;
            }
            105 => {
                args |= 2;
                current_block_31 = 6636775023221328366;
            }
            118 => {
                current_block_31 = 6636775023221328366;
            }
            101 => {
                args |= 8;
                current_block_31 = 15172496195422792753;
            }
            108 => {
                current_block_31 = 15172496195422792753;
            }
            _ => return 1,
        }
        match current_block_31 {
            6636775023221328366 => {
                if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                    return 1;
                }
                args |= 4;
            }
            15172496195422792753 => {
                if *(*argv.offset(i as isize)).offset(2 as isize) as i32 == '\0' as i32 {
                    i += 1;
                    if (*argv.offset(i as isize)).is_null()
                        || *(*argv.offset(i as isize)).offset(0 as isize) as i32 == '-' as i32
                    {
                        return 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    *first = 0;
    return args;
}}
pub unsafe extern "C" fn runargs(state: *mut State, argv: *mut *mut i8, n: i32) -> i32 { unsafe {
    let mut i: i32;
    i = 1;
    while i < n {
        let option: i32 = *(*argv.offset(i as isize)).offset(1 as isize) as i32;
        match option {
            101 | 108 => {
                let status: i32;
                let mut extra: *mut i8 = (*argv.offset(i as isize)).offset(2 as isize);
                if *extra as i32 == '\0' as i32 {
                    i += 1;
                    extra = *argv.offset(i as isize);
                }
                status = if option == 'e' as i32 {
                    dostring(state, extra, b"=(command line)\0" as *const u8 as *const i8)
                } else {
                    dolibrary(state, extra)
                };
                if status != 0 {
                    return 0;
                }
            }
            87 => {
                lua_warning(state, b"@on\0" as *const u8 as *const i8, 0);
            }
            _ => {}
        }
        i += 1;
    }
    return 1;
}}
pub unsafe extern "C" fn handle_luainit(state: *mut State) -> i32 { unsafe {
    let mut name: *const i8 = b"=LUA_INIT_5_4\0" as *const u8 as *const i8;
    let mut init: *const i8 = getenv(name.offset(1 as isize));
    if init.is_null() {
        name = b"=LUA_INIT\0" as *const u8 as *const i8;
        init = getenv(name.offset(1 as isize));
    }
    if init.is_null() {
        return 0;
    } else if *init.offset(0 as isize) as i32 == '@' as i32 {
        return dofile(state, init.offset(1 as isize));
    } else {
        return dostring(state, init, name);
    };
}}
pub unsafe extern "C" fn get_prompt(state: *mut State, firstline: i32) -> *const i8 { unsafe {
    if lua_getglobal(
        state,
        if firstline != 0 {
            b"_PROMPT\0" as *const u8 as *const i8
        } else {
            b"_PROMPT2\0" as *const u8 as *const i8
        },
    ) == 0
    {
        return if firstline != 0 {
            b"> \0" as *const u8 as *const i8
        } else {
            b">> \0" as *const u8 as *const i8
        };
    } else {
        let p: *const i8 = lual_tolstring(state, -1, std::ptr::null_mut());
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
        return p;
    };
}}
pub unsafe extern "C" fn incomplete(state: *mut State, status: i32) -> i32 { unsafe {
    if status == 3 {
        let mut lmsg: u64 = 0;
        let message: *const i8 = lua_tolstring(state, -1, &mut lmsg);
        if lmsg
            >= (::core::mem::size_of::<[i8; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64)
            && strcmp(
                message.offset(lmsg as isize).offset(
                    -((::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64) as isize),
                ),
                b"<eof>\0" as *const u8 as *const i8,
            ) == 0
        {
            return 1;
        }
    }
    return 0;
}}
pub unsafe extern "C" fn pushline(state: *mut State, firstline: i32) -> i32 { unsafe {
    let mut buffer: [i8; 512] = [0; 512];
    let b: *mut i8 = buffer.as_mut_ptr();
    let prmt: *const i8 = get_prompt(state, firstline);
    fputs(prmt, stdout);
    fflush(stdout);
    let readstatus: i32 =
        (fgets(b, 512 as i32, stdin) != std::ptr::null_mut() as *mut i8) as i32;
    lua_settop(state, 0);
    if readstatus == 0 {
        return 0;
    }
    let mut l: u64 = strlen(b);
    if l > 0 && *b.offset(l.wrapping_sub(1 as u64) as isize) as i32 == '\n' as i32 {
        l = l.wrapping_sub(1);
        *b.offset(l as isize) = '\0' as i8;
    }
    if firstline != 0 && *b.offset(0 as isize) as i32 == '=' as i32 {
        lua_pushfstring(
            state,
            b"return %s\0" as *const u8 as *const i8,
            b.offset(1 as isize),
        );
    } else {
        lua_pushlstring(state, b, l);
    }
    return 1;
}}
pub unsafe extern "C" fn addreturn(state: *mut State) -> i32 { unsafe {
    let line: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
    let retline: *const i8 =
        lua_pushfstring(state, b"return %s;\0" as *const u8 as *const i8, line);
    let status: i32 = lual_loadbufferx(
        state,
        retline,
        strlen(retline),
        b"=stdin\0" as *const u8 as *const i8,
        std::ptr::null(),
    );
    if status == 0 {
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
    } else {
        lua_settop(state, -(2) - 1);
    }
    return status;
}}
pub unsafe extern "C" fn multiline(state: *mut State) -> i32 { unsafe {
    loop {
        let mut length: u64 = 0;
        let line: *const i8 = lua_tolstring(state, 1, &mut length);
        let status: i32 = lual_loadbufferx(
            state,
            line,
            length,
            b"=stdin\0" as *const u8 as *const i8,
            std::ptr::null(),
        );
        if incomplete(state, status) == 0 || pushline(state, 0) == 0 {
            return status;
        }
        lua_rotate(state, -(2), -1);
        lua_settop(state, -1 - 1);
        lua_pushstring(state, b"\n\0" as *const u8 as *const i8);
        lua_rotate(state, -(2), 1);
        lua_concat(state, 3);
    }
}}
pub unsafe extern "C" fn loadline(state: *mut State) -> i32 { unsafe {
    lua_settop(state, 0);
    if pushline(state, 1) == 0 {
        return -1;
    }
    let mut status: i32 = addreturn(state);
    if status != 0 {
        status = multiline(state);
    }
    lua_rotate(state, 1, -1);
    lua_settop(state, -1 - 1);
    return status;
}}
pub unsafe extern "C" fn l_print(state: *mut State) { unsafe {
    let n: i32 = (*state).get_top();
    if n > 0 {
        lual_checkstack(
            state,
            20 as i32,
            b"too many results to print\0" as *const u8 as *const i8,
        );
        lua_getglobal(state, b"print\0" as *const u8 as *const i8);
        lua_rotate(state, 1, 1);
        if lua_pcallk(state, n, 0, 0, 0, None) != 0 {
            l_message(
                PROGRAM_NAME,
                lua_pushfstring(
                    state,
                    b"error calling 'print' (%s)\0" as *const u8 as *const i8,
                    lua_tolstring(state, -1, std::ptr::null_mut()),
                ),
            );
        }
    }
}}
pub unsafe extern "C" fn do_repl(state: *mut State) { unsafe {
    let mut status: i32;
    let oldprogname: *const i8 = PROGRAM_NAME;
    PROGRAM_NAME = std::ptr::null();
    loop {
        status = loadline(state);
        if !(status != -1) {
            break;
        }
        if status == 0 {
            status = docall(state, 0, -1);
        }
        if status == 0 {
            l_print(state);
        } else {
            report(state, status);
        }
    }
    lua_settop(state, 0);
    fwrite(
        b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        1 as u64,
        stdout,
    );
    fflush(stdout);
    PROGRAM_NAME = oldprogname;
}}
pub unsafe extern "C" fn pmain(state: *mut State) -> i32 { unsafe {
    let argc: i32 = lua_tointegerx(state, 1, std::ptr::null_mut()) as i32;
    let argv: *mut *mut i8 = lua_touserdata(state, 2) as *mut *mut i8;
    let mut script: i32 = 0;
    let args: i32 = collectargs(argv, &mut script);
    let optlim: i32 = if script > 0 { script } else { argc };
    lual_checkversion_(
        state,
        504.0,
        (::core::mem::size_of::<i64>() as u64)
            .wrapping_mul(16 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<f64>() as u64),
    );
    if args == 1 {
        print_usage(*argv.offset(script as isize));
        return 0;
    }
    if args & 4 != 0 {
        print_version();
    }
    if args & 16 as i32 != 0 {
        (*state).push_boolean(true);
        lua_setfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"LUA_NOENV\0" as *const u8 as *const i8,
        );
    }
    lual_openlibs(state);
    createargtable(state, argv, argc, script);
    lua_gc(state, 1);
    lua_gc(state, 10 as i32, 0, 0);
    if args & 16 as i32 == 0 {
        if handle_luainit(state) != 0 {
            return 0;
        }
    }
    if runargs(state, argv, optlim) == 0 {
        return 0;
    }
    if script > 0 {
        if handle_script(state, argv.offset(script as isize)) != 0 {
            return 0;
        }
    }
    if args & 2 != 0 {
        do_repl(state);
    } else if script < 1 && args & (8 | 4) == 0 {
        if isatty(0) != 0 {
            print_version();
            do_repl(state);
        } else {
            dofile(state, std::ptr::null());
        }
    }
    (*state).push_boolean(true);
    return 1;
}}
pub unsafe fn main_0(argc: i32, argv: *mut *mut i8) -> i32 { unsafe {
    let state: *mut State = lual_newstate();
    if state.is_null() {
        l_message(
            *argv.offset(0),
            b"cannot create state: not enough memory\0" as *const u8 as *const i8,
        );
        return 1;
    } else {
        lua_gc(state, 0);
        lua_pushcclosure(
            state,
            Some(pmain as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        (*state).push_integer(argc as i64);
        lua_pushlightuserdata(state, argv as *mut libc::c_void);
        let status: i32 = lua_pcallk(state, 2, 1, 0, 0, None);
        let result: i32 = lua_toboolean(state, -1);
        report(state, status);
        lua_close(state);
        return if result != 0 && status == 0 { 0 } else { 1 };
    }
}}

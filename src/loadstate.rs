use crate::state::*;
use crate::zio::*;
use crate::lclosure::*;
use crate::prototype::*;
use crate::tstring::*;
use crate::object::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::localvariable::*;
use crate::upvaluedescription::*;
use crate::debugger::absolutelineinfo::*;
use crate::utility::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub state: *mut State,
    pub zio: *mut ZIO,
    pub name: *const i8,
}
pub unsafe extern "C" fn error(load_state: *mut LoadState, why: *const i8) -> ! {
    unsafe {
        luao_pushfstring(
            (*load_state).state,
            b"%s: bad binary format (%s)\0" as *const u8 as *const i8,
            (*load_state).name,
            why,
        );
        luad_throw((*load_state).state, 3);
    }
}
pub unsafe extern "C" fn load_block(load_state: *mut LoadState, b: *mut libc::c_void, size: u64) {
    unsafe {
        if luaz_read((*load_state).zio, b, size) != 0u64 {
            error(load_state, b"truncated chunk\0" as *const u8 as *const i8);
        }
    }
}
pub unsafe extern "C" fn load_byte(load_state: *mut LoadState) -> u8 {
    unsafe {
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
    }
}
pub unsafe extern "C" fn load_unsigned(load_state: *mut LoadState, mut limit: u64) -> u64 {
    unsafe {
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
    }
}
pub unsafe extern "C" fn load_size(load_state: *mut LoadState) -> u64 {
    unsafe {
        return load_unsigned(load_state, !(0u64));
    }
}
pub unsafe extern "C" fn load_int(load_state: *mut LoadState) -> i32 {
    unsafe {
        return load_unsigned(load_state, 0x7FFFFFFF as u64) as i32;
    }
}
pub unsafe extern "C" fn load_number(load_state: *mut LoadState) -> f64 {
    unsafe {
        let mut x: f64 = 0.0;
        load_block(
            load_state,
            &mut x as *mut f64 as *mut libc::c_void,
            (1 as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
        );
        return x;
    }
}
pub unsafe extern "C" fn load_integer(load_state: *mut LoadState) -> i64 {
    unsafe {
        let mut x: i64 = 0;
        load_block(
            load_state,
            &mut x as *mut i64 as *mut libc::c_void,
            (1 as u64).wrapping_mul(::core::mem::size_of::<i64>() as u64),
        );
        return x;
    }
}
pub unsafe extern "C" fn load_string_n(
    load_state: *mut LoadState,
    p: *mut Prototype,
) -> *mut TString {
    unsafe {
        let state: *mut State = (*load_state).state;
        let ts: *mut TString;
        let mut size: u64 = load_size(load_state);
        if size == 0u64 {
            return std::ptr::null_mut();
        } else {
            size = size.wrapping_sub(1);
            if size <= 40 as u64 {
                let mut buffer: [i8; 40] = [0; 40];
                load_block(
                    load_state,
                    buffer.as_mut_ptr() as *mut libc::c_void,
                    size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                ts = luas_newlstr(state, buffer.as_mut_ptr(), size);
            } else {
                ts = TString::create_long(state, size);
                let io: *mut TValue = &mut (*(*state).top.p).value;
                let x_: *mut TString = ts;
                (*io).value.object = &mut (*(x_ as *mut Object));
                (*io).set_tag((*x_).get_tag());
                (*io).set_collectable();
                (*state).luad_inctop();
                load_block(
                    load_state,
                    ((*ts).get_contents()) as *mut libc::c_void,
                    size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                (*state).top.p = (*state).top.p.offset(-1);
            }
        }
        if (*p).get_marked() & 1 << 5 != 0 && (*ts).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                state,
                &mut (*(p as *mut Object)),
                &mut (*(ts as *mut Object)),
            );
        } else {
        };
        return ts;
    }
}
pub unsafe extern "C" fn load_string(
    load_state: *mut LoadState,
    p: *mut Prototype,
) -> *mut TString {
    unsafe {
        let st: *mut TString = load_string_n(load_state, p);
        if st.is_null() {
            error(
                load_state,
                b"bad format for constant string\0" as *const u8 as *const i8,
            );
        }
        return st;
    }
}
pub unsafe extern "C" fn load_code(load_state: *mut LoadState, f: *mut Prototype) {
    unsafe {
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
    }
}
pub unsafe extern "C" fn load_constants(load_state: *mut LoadState, f: *mut Prototype) {
    unsafe {
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
            (*((*f).k).offset(i as isize)).set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        i = 0;
        while i < n {
            let o: *mut TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
            let t: i32 = load_byte(load_state) as i32;
            match t {
                0 => {
                    (*o).set_tag(TAG_VARIANT_NIL_NIL);
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
                    (*io).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                }
                3 => {
                    let io_0: *mut TValue = o;
                    (*io_0).value.i = load_integer(load_state);
                    (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                }
                4 | 20 => {
                    let io_1: *mut TValue = o;
                    let x_: *mut TString = load_string(load_state, f);
                    (*io_1).value.object = &mut (*(x_ as *mut Object));
                    (*io_1).set_tag((*x_).get_tag());
                    (*io_1).set_collectable();
                }
                _ => {}
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn load_prototypes(load_state: *mut LoadState, f: *mut Prototype) {
    unsafe {
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
                && (**((*f).p).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrier_(
                    (*load_state).state,
                    &mut (*(f as *mut Object)),
                    &mut (*(*((*f).p).offset(i as isize) as *mut Object)),
                );
            } else {
            };
            load_function(load_state, *((*f).p).offset(i as isize), (*f).source);
            i += 1;
        }
    }
}
pub unsafe extern "C" fn load_upvalues(load_state: *mut LoadState, f: *mut Prototype) {
    unsafe {
        let mut i: i32;
        let n: i32;
        n = load_int(load_state);
        if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
            && (n as u64).wrapping_add(1 as u64)
                > (!(0u64)).wrapping_div(::core::mem::size_of::<UpValueDescription>() as u64)
        {
            (*((*load_state).state)).too_big();
        } else {
        };
        (*f).upvalues = luam_malloc_(
            (*load_state).state,
            (n as u64).wrapping_mul(::core::mem::size_of::<UpValueDescription>() as u64),
        ) as *mut UpValueDescription;
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
    }
}
pub unsafe extern "C" fn load_debug(load_state: *mut LoadState, f: *mut Prototype) {
    unsafe {
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
            (*((*f).local_variables).offset(i as isize)).start_program_counter =
                load_int(load_state);
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
    }
}
pub unsafe extern "C" fn load_function(
    load_state: *mut LoadState,
    f: *mut Prototype,
    psource: *mut TString,
) {
    unsafe {
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
    }
}
pub unsafe extern "C" fn check_literal(
    load_state: *mut LoadState,
    s: *const i8,
    message: *const i8,
) {
    unsafe {
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
    }
}
pub unsafe extern "C" fn f_check_size(load_state: *mut LoadState, size: u64, tname: *const i8) {
    unsafe {
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
    }
}
pub unsafe extern "C" fn check_header(load_state: *mut LoadState) {
    unsafe {
        check_literal(
            load_state,
            &*(b"\x1BLua\0" as *const u8 as *const i8).offset(1 as isize),
            b"not a binary chunk\0" as *const u8 as *const i8,
        );
        if load_byte(load_state) as i32
            != 504 as i32 / 100 as i32 * 16 as i32 + 504 as i32 % 100 as i32
        {
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
        if load_integer(load_state) != 0x5678 as i64 {
            error(
                load_state,
                b"integer format mismatch\0" as *const u8 as *const i8,
            );
        }
        if load_number(load_state) != 370.5f64 {
            error(
                load_state,
                b"float format mismatch\0" as *const u8 as *const i8,
            );
        }
    }
}
pub unsafe extern "C" fn luau_undump(
    state: *mut State,
    zio: *mut ZIO,
    name: *const i8,
) -> *mut LClosure {
    unsafe {
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
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable();
        (*state).luad_inctop();
        (*cl).p = luaf_newproto(state);
        if (*cl).get_marked() & 1 << 5 != 0 && (*(*cl).p).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                state,
                &mut (*(cl as *mut Object)),
                &mut (*((*cl).p as *mut Object)),
            );
        } else {
        };
        load_function(&mut load_state, (*cl).p, std::ptr::null_mut());
        return cl;
    }
}

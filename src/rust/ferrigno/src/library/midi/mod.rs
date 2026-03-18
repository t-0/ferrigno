#[cfg(target_os = "linux")]
mod alsa;
mod backend;
#[cfg(target_os = "macos")]
mod coremidi;

use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::user::*;
use backend::*;
use std::mem::size_of;
use std::ptr::*;

// ─── Platform backend selection ──────────────────────────────────────────────

#[cfg(target_os = "macos")]
type PlatformBackend = coremidi::CoreMidiBackend;
#[cfg(target_os = "macos")]
fn platform_backend() -> PlatformBackend {
    coremidi::CoreMidiBackend
}

#[cfg(target_os = "linux")]
type PlatformBackend = alsa::AlsaBackend;
#[cfg(target_os = "linux")]
fn platform_backend() -> PlatformBackend {
    alsa::AlsaBackend
}

type PlatformOutput = <PlatformBackend as MidiBackend>::Output;
type PlatformInput = <PlatformBackend as MidiBackend>::Input;

// ─── Metatable names ────────────────────────────────────────────────────────

const OUT_META: *const i8 = c"midi_output*".as_ptr();
const IN_META: *const i8 = c"midi_input*".as_ptr();

// ─── Userdata wrappers ──────────────────────────────────────────────────────

// We store a Box pointer in the Lua userdata so we can support the platform
// output/input types which are not necessarily repr(C) or fixed-size.

#[repr(C)]
struct MidiOutputData {
    inner: *mut PlatformOutput,
    closed: bool,
}

#[repr(C)]
struct MidiInputData {
    inner: *mut PlatformInput,
    closed: bool,
}

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn check_out(state: *mut State) -> *mut MidiOutputData {
    unsafe {
        let p = lual_checkudata(state, 1, OUT_META) as *mut MidiOutputData;
        if (*p).closed {
            lual_error(state, c"attempt to use a closed MIDI output port".as_ptr(), &[]);
            unreachable!()
        }
        p
    }
}

unsafe fn check_in(state: *mut State) -> *mut MidiInputData {
    unsafe {
        let p = lual_checkudata(state, 1, IN_META) as *mut MidiInputData;
        if (*p).closed {
            lual_error(state, c"attempt to use a closed MIDI input port".as_ptr(), &[]);
            unreachable!()
        }
        p
    }
}

unsafe fn send_data(state: *mut State, out: &mut PlatformOutput) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        let data: Vec<u8> = if lua_type(state, 2) == Some(TagType::String) {
            let mut slen = 0usize;
            let sptr = lua_tolstring(state, 2, &mut slen);
            std::slice::from_raw_parts(sptr as *const u8, slen).to_vec()
        } else {
            let mut v = Vec::new();
            for i in 2..=nargs {
                v.push((lual_checkinteger(state, i) & 0xFF) as u8);
            }
            v
        };
        if data.is_empty() {
            return 0;
        }
        if let Err(e) = out.send(&data) {
            let msg = format!("midi:send failed: {}\0", e);
            return lual_error(state, msg.as_ptr() as *const i8, &[]);
        }
        0
    }
}

unsafe fn send_bytes_helper(out: &mut PlatformOutput, data: &[u8]) {
    let _ = out.send(data);
}

// ─── Output port __gc / __close ─────────────────────────────────────────────

pub unsafe fn midi_out_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, OUT_META) as *mut MidiOutputData;
        if !(*p).closed {
            let inner = Box::from_raw((*p).inner);
            drop(inner);
            (*p).closed = true;
        }
        0
    }
}

// ─── Input port __gc / __close ──────────────────────────────────────────────

pub unsafe fn midi_in_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, IN_META) as *mut MidiInputData;
        if !(*p).closed {
            let inner = Box::from_raw((*p).inner);
            drop(inner);
            (*p).closed = true;
        }
        0
    }
}

// ─── Output port methods ────────────────────────────────────────────────────

pub unsafe fn midi_out_send(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        send_data(state, out)
    }
}

pub unsafe fn midi_out_note_on(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(state, 3) & 0x7F) as u8;
        let vel = (lual_checkinteger(state, 4) & 0x7F) as u8;
        send_bytes_helper(out, &[0x90 | ch, note, vel]);
        0
    }
}

pub unsafe fn midi_out_note_off(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(state, 3) & 0x7F) as u8;
        let vel = if lua_type(state, 4) == Some(TagType::Numeric) {
            (lua_tointegerx(state, 4, null_mut()) & 0x7F) as u8
        } else {
            64
        };
        send_bytes_helper(out, &[0x80 | ch, note, vel]);
        0
    }
}

pub unsafe fn midi_out_cc(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let cc = (lual_checkinteger(state, 3) & 0x7F) as u8;
        let val = (lual_checkinteger(state, 4) & 0x7F) as u8;
        send_bytes_helper(out, &[0xB0 | ch, cc, val]);
        0
    }
}

pub unsafe fn midi_out_program_change(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let pgm = (lual_checkinteger(state, 3) & 0x7F) as u8;
        send_bytes_helper(out, &[0xC0 | ch, pgm]);
        0
    }
}

pub unsafe fn midi_out_pitch_bend(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let val = lual_checkinteger(state, 3).clamp(-8192, 8191);
        let bent = (val + 8192) as u16;
        let lsb = (bent & 0x7F) as u8;
        let msb = ((bent >> 7) & 0x7F) as u8;
        send_bytes_helper(out, &[0xE0 | ch, lsb, msb]);
        0
    }
}

pub unsafe fn midi_out_aftertouch(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        let ch = ((lual_checkinteger(state, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(state, 3) & 0x7F) as u8;
        let pres = (lual_checkinteger(state, 4) & 0x7F) as u8;
        send_bytes_helper(out, &[0xA0 | ch, note, pres]);
        0
    }
}

pub unsafe fn midi_out_clock(state: *mut State) -> i32 {
    unsafe {
        let p = check_out(state);
        let out = &mut *(*p).inner;
        send_bytes_helper(out, &[0xF8]);
        0
    }
}

pub unsafe fn midi_out_close(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, OUT_META) as *mut MidiOutputData;
        if !(*p).closed {
            let inner = Box::from_raw((*p).inner);
            drop(inner);
            (*p).closed = true;
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── Input port methods ─────────────────────────────────────────────────────

pub unsafe fn midi_in_recv(state: *mut State) -> i32 {
    unsafe {
        let p = check_in(state);
        let inp = &*(*p).inner;
        let timeout_ms: i64 = if lua_type(state, 2) == Some(TagType::Numeric) {
            lua_tointegerx(state, 2, null_mut())
        } else {
            0
        };

        let buf = inp.buffer();

        let deadline = if timeout_ms > 0 {
            Some(std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms as u64))
        } else {
            None
        };

        loop {
            {
                let mut q = buf.lock().unwrap();
                if let Some(msg) = q.pop_front() {
                    (*state).lua_createtable();

                    lua_pushlstring(state, b"data".as_ptr() as *const i8, 4);
                    lua_pushlstring(state, msg.data.as_ptr() as *const i8, msg.data.len());
                    lua_rawset(state, -3);

                    lua_pushlstring(state, b"time".as_ptr() as *const i8, 4);
                    (*state).push_integer(msg.timestamp as i64);
                    lua_rawset(state, -3);

                    return 1;
                }
            }

            if timeout_ms == 0 {
                break;
            }

            match deadline {
                | Some(dl) if std::time::Instant::now() >= dl => break,
                | _ => {},
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        (*state).push_nil();
        1
    }
}

pub unsafe fn midi_in_pending(state: *mut State) -> i32 {
    unsafe {
        let p = check_in(state);
        let inp = &*(*p).inner;
        let count = inp.buffer().lock().unwrap().len();
        (*state).push_integer(count as i64);
        1
    }
}

pub unsafe fn midi_in_flush(state: *mut State) -> i32 {
    unsafe {
        let p = check_in(state);
        let inp = &*(*p).inner;
        inp.buffer().lock().unwrap().clear();
        0
    }
}

pub unsafe fn midi_in_close(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, IN_META) as *mut MidiInputData;
        if !(*p).closed {
            let inner = Box::from_raw((*p).inner);
            drop(inner);
            (*p).closed = true;
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── Module-level functions ─────────────────────────────────────────────────

pub unsafe fn midi_sources(state: *mut State) -> i32 {
    unsafe {
        let backend = platform_backend();
        let sources = backend.sources();
        (*state).lua_createtable();
        for (i, src) in sources.iter().enumerate() {
            (*state).push_integer((i + 1) as i64);
            (*state).lua_createtable();

            lua_pushlstring(state, b"name".as_ptr() as *const i8, 4);
            lua_pushlstring(state, src.name.as_ptr() as *const i8, src.name.len());
            lua_rawset(state, -3);

            lua_pushlstring(state, b"index".as_ptr() as *const i8, 5);
            (*state).push_integer(src.index as i64);
            lua_rawset(state, -3);

            lua_rawset(state, -3);
        }
        1
    }
}

pub unsafe fn midi_destinations(state: *mut State) -> i32 {
    unsafe {
        let backend = platform_backend();
        let dests = backend.destinations();
        (*state).lua_createtable();
        for (i, dest) in dests.iter().enumerate() {
            (*state).push_integer((i + 1) as i64);
            (*state).lua_createtable();

            lua_pushlstring(state, b"name".as_ptr() as *const i8, 4);
            lua_pushlstring(state, dest.name.as_ptr() as *const i8, dest.name.len());
            lua_rawset(state, -3);

            lua_pushlstring(state, b"index".as_ptr() as *const i8, 5);
            (*state).push_integer(dest.index as i64);
            lua_rawset(state, -3);

            lua_rawset(state, -3);
        }
        1
    }
}

unsafe fn parse_endpoint_selector(state: *mut State, arg: i32) -> EndpointSelector<'static> {
    unsafe {
        match lua_type(state, arg) {
            | Some(TagType::Numeric) => {
                let idx = lua_tointegerx(state, arg, null_mut());
                EndpointSelector::Index(idx as usize)
            },
            | Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(state, arg, &mut slen);
                let s = std::slice::from_raw_parts(sptr as *const u8, slen);
                // Leak the string so it has 'static lifetime — it's short-lived anyway
                let owned = String::from_utf8_lossy(s).into_owned();
                EndpointSelector::Name(Box::leak(owned.into_boxed_str()))
            },
            | _ => EndpointSelector::Index(0), // will fail resolution
        }
    }
}

pub unsafe fn midi_open_output(state: *mut State) -> i32 {
    unsafe {
        let sel = parse_endpoint_selector(state, 1);
        let backend = platform_backend();
        match backend.open_output(sel) {
            | Ok(output) => {
                let boxed = Box::new(output);
                let user_data = User::lua_newuserdatauv(state, size_of::<MidiOutputData>(), 0) as *mut MidiOutputData;
                (*user_data).inner = Box::into_raw(boxed);
                (*user_data).closed = false;
                lual_setmetatable(state, OUT_META);
                1
            },
            | Err(e) => {
                (*state).push_nil();
                let msg = format!("{}\0", e);
                lua_pushstring(state, msg.as_ptr() as *const std::ffi::c_char);
                2
            },
        }
    }
}

pub unsafe fn midi_open_input(state: *mut State) -> i32 {
    unsafe {
        let sel = parse_endpoint_selector(state, 1);
        let backend = platform_backend();
        match backend.open_input(sel) {
            | Ok(input) => {
                let boxed = Box::new(input);
                let user_data = User::lua_newuserdatauv(state, size_of::<MidiInputData>(), 0) as *mut MidiInputData;
                (*user_data).inner = Box::into_raw(boxed);
                (*user_data).closed = false;
                lual_setmetatable(state, IN_META);
                1
            },
            | Err(e) => {
                (*state).push_nil();
                let msg = format!("{}\0", e);
                lua_pushstring(state, msg.as_ptr() as *const std::ffi::c_char);
                2
            },
        }
    }
}

// ─── Registration ───────────────────────────────────────────────────────────

pub const MIDI_FUNCS: [RegisteredFunction; 4] = [
    RegisteredFunction {
        registeredfunction_name: c"sources".as_ptr(),
        registeredfunction_function: Some(midi_sources as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"destinations".as_ptr(),
        registeredfunction_function: Some(midi_destinations as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"open_output".as_ptr(),
        registeredfunction_function: Some(midi_open_output as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"open_input".as_ptr(),
        registeredfunction_function: Some(midi_open_input as unsafe fn(*mut State) -> i32),
    },
];

pub const MIDI_OUT_METHODS: [RegisteredFunction; 9] = [
    RegisteredFunction {
        registeredfunction_name: c"send".as_ptr(),
        registeredfunction_function: Some(midi_out_send as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"note_on".as_ptr(),
        registeredfunction_function: Some(midi_out_note_on as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"note_off".as_ptr(),
        registeredfunction_function: Some(midi_out_note_off as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"cc".as_ptr(),
        registeredfunction_function: Some(midi_out_cc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"program_change".as_ptr(),
        registeredfunction_function: Some(midi_out_program_change as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"pitch_bend".as_ptr(),
        registeredfunction_function: Some(midi_out_pitch_bend as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"aftertouch".as_ptr(),
        registeredfunction_function: Some(midi_out_aftertouch as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"clock".as_ptr(),
        registeredfunction_function: Some(midi_out_clock as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(midi_out_close as unsafe fn(*mut State) -> i32),
    },
];

pub const MIDI_OUT_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(midi_out_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(midi_out_gc as unsafe fn(*mut State) -> i32),
    },
];

pub const MIDI_IN_METHODS: [RegisteredFunction; 4] = [
    RegisteredFunction {
        registeredfunction_name: c"recv".as_ptr(),
        registeredfunction_function: Some(midi_in_recv as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"pending".as_ptr(),
        registeredfunction_function: Some(midi_in_pending as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(midi_in_flush as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(midi_in_close as unsafe fn(*mut State) -> i32),
    },
];

pub const MIDI_IN_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(midi_in_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(midi_in_gc as unsafe fn(*mut State) -> i32),
    },
];

// ─── luaopen_midi ───────────────────────────────────────────────────────────

pub unsafe fn luaopen_midi(state: *mut State) -> i32 {
    unsafe {
        // Output-port metatable
        lual_newmetatable(state, OUT_META);
        lual_setfuncs(state, MIDI_OUT_META.as_ptr(), MIDI_OUT_META.len(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, MIDI_OUT_METHODS.as_ptr(), MIDI_OUT_METHODS.len(), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2);

        // Input-port metatable
        lual_newmetatable(state, IN_META);
        lual_setfuncs(state, MIDI_IN_META.as_ptr(), MIDI_IN_META.len(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, MIDI_IN_METHODS.as_ptr(), MIDI_IN_METHODS.len(), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2);

        // Library table
        (*state).lua_createtable();
        lual_setfuncs(state, MIDI_FUNCS.as_ptr(), MIDI_FUNCS.len(), 0);

        // MIDI status byte constants
        macro_rules! set_int {
            ($name:expr, $val:expr) => {
                (*state).push_integer($val);
                lua_setfield(state, -2, $name);
            };
        }

        set_int!(c"NOTE_OFF".as_ptr(), 0x80);
        set_int!(c"NOTE_ON".as_ptr(), 0x90);
        set_int!(c"AFTERTOUCH".as_ptr(), 0xA0);
        set_int!(c"CC".as_ptr(), 0xB0);
        set_int!(c"PROGRAM_CHANGE".as_ptr(), 0xC0);
        set_int!(c"CHANNEL_PRESSURE".as_ptr(), 0xD0);
        set_int!(c"PITCH_BEND".as_ptr(), 0xE0);
        set_int!(c"SYSEX".as_ptr(), 0xF0);
        set_int!(c"QUARTER_FRAME".as_ptr(), 0xF1);
        set_int!(c"SONG_POSITION".as_ptr(), 0xF2);
        set_int!(c"SONG_SELECT".as_ptr(), 0xF3);
        set_int!(c"TUNE_REQUEST".as_ptr(), 0xF6);
        set_int!(c"SYSEX_END".as_ptr(), 0xF7);
        set_int!(c"CLOCK".as_ptr(), 0xF8);
        set_int!(c"START".as_ptr(), 0xFA);
        set_int!(c"CONTINUE".as_ptr(), 0xFB);
        set_int!(c"STOP".as_ptr(), 0xFC);
        set_int!(c"ACTIVE_SENSING".as_ptr(), 0xFE);
        set_int!(c"RESET".as_ptr(), 0xFF);

        1
    }
}

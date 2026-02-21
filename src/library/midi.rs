use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tagtype::*;
use crate::user::*;
use std::collections::VecDeque;
use std::mem::size_of;
use std::ptr::*;
use std::sync::{Arc, Mutex};

// ─── CoreMIDI FFI ─────────────────────────────────────────────────────────────

type MidiObjectRef  = u32;
type MidiClientRef  = MidiObjectRef;
type MidiPortRef    = MidiObjectRef;
type MidiEndpointRef = MidiObjectRef;
type OsStatus       = i32;
type MidiTimeStamp  = u64;
type ItemCount      = libc::c_ulong;
type ByteCount      = libc::c_ulong;

#[link(name = "CoreMIDI", kind = "framework")]
unsafe extern "C" {
    fn MIDIClientCreate(
        name:            *const libc::c_void, // CFStringRef
        notify_proc:     *mut   libc::c_void,
        notify_ref_con:  *mut   libc::c_void,
        out_client:      *mut   MidiClientRef,
    ) -> OsStatus;

    fn MIDIOutputPortCreate(
        client:    MidiClientRef,
        port_name: *const libc::c_void, // CFStringRef
        out_port:  *mut   MidiPortRef,
    ) -> OsStatus;

    fn MIDIInputPortCreate(
        client:    MidiClientRef,
        port_name: *const libc::c_void, // CFStringRef
        read_proc: unsafe extern "C" fn(
            pktlist:        *const libc::c_void,
            read_proc_ref:  *mut   libc::c_void,
            src_conn_ref:   *mut   libc::c_void,
        ),
        ref_con:   *mut   libc::c_void,
        out_port:  *mut   MidiPortRef,
    ) -> OsStatus;

    fn MIDIPortConnectSource(
        port:         MidiPortRef,
        source:       MidiEndpointRef,
        conn_ref_con: *mut libc::c_void,
    ) -> OsStatus;

    fn MIDIPortDisconnectSource(
        port:   MidiPortRef,
        source: MidiEndpointRef,
    ) -> OsStatus;

    fn MIDIPortDispose(port: MidiPortRef) -> OsStatus;

    fn MIDIGetNumberOfSources() -> ItemCount;
    fn MIDIGetSource(src_index0: ItemCount) -> MidiEndpointRef;

    fn MIDIGetNumberOfDestinations() -> ItemCount;
    fn MIDIGetDestination(dest_index0: ItemCount) -> MidiEndpointRef;

    fn MIDIObjectGetStringProperty(
        obj:     MidiObjectRef,
        prop_id: *const libc::c_void, // CFStringRef
        str_out: *mut   *mut libc::c_void, // CFStringRef *
    ) -> OsStatus;

    fn MIDISend(
        port:    MidiPortRef,
        dest:    MidiEndpointRef,
        pktlist: *const libc::c_void, // MIDIPacketList *
    ) -> OsStatus;

    fn MIDIPacketListInit(pktlist: *mut libc::c_void) -> *mut libc::c_void;

    fn MIDIPacketListAdd(
        pktlist:    *mut   libc::c_void,
        list_size:  ByteCount,
        cur_packet: *mut   libc::c_void,
        time:       MidiTimeStamp,
        n_data:     ByteCount,
        data:       *const u8,
    ) -> *mut libc::c_void;

    // kMIDIPropertyName is a CFStringRef stored at this symbol address.
    static kMIDIPropertyName: *const libc::c_void;
}

// ─── CoreFoundation FFI ───────────────────────────────────────────────────────

const CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(
        alloc:    *const libc::c_void, // pass null for default allocator
        c_str:    *const libc::c_char,
        encoding: u32,
    ) -> *mut libc::c_void; // CFStringRef

    fn CFStringGetCString(
        the_string:  *const libc::c_void, // CFStringRef
        buffer:      *mut   libc::c_char,
        buffer_size: libc::c_long,
        encoding:    u32,
    ) -> i32; // Boolean (1 = success)

    fn CFRelease(cf: *const libc::c_void);
}

// ─── CFString helpers ─────────────────────────────────────────────────────────

unsafe fn cf_str_from_bytes(s: &[u8]) -> *mut libc::c_void {
    unsafe {
        let mut buf = s.to_vec();
        buf.push(0); // null-terminate
        CFStringCreateWithCString(null(), buf.as_ptr() as *const libc::c_char, CF_STRING_ENCODING_UTF8)
    }
}

unsafe fn cf_str_to_string(cfstr: *const libc::c_void) -> String {
    unsafe {
        if cfstr.is_null() {
            return String::new();
        }
        let mut buf = vec![0u8; 512];
        let ok = CFStringGetCString(cfstr, buf.as_mut_ptr() as *mut libc::c_char, 512, CF_STRING_ENCODING_UTF8);
        if ok != 0 {
            let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            String::from_utf8_lossy(&buf[..end]).into_owned()
        } else {
            String::new()
        }
    }
}

unsafe fn endpoint_name(endpoint: MidiEndpointRef) -> String {
    unsafe {
        let name_prop = kMIDIPropertyName;
        let mut cfstr: *mut libc::c_void = null_mut();
        MIDIObjectGetStringProperty(endpoint, name_prop, &mut cfstr);
        if cfstr.is_null() {
            return format!("endpoint-{}", endpoint);
        }
        let name = cf_str_to_string(cfstr);
        CFRelease(cfstr);
        name
    }
}

// ─── Global MIDI client ───────────────────────────────────────────────────────

static MIDI_CLIENT: Mutex<MidiClientRef> = Mutex::new(0);

unsafe fn get_or_create_client() -> Result<MidiClientRef, String> {
    unsafe {
        let mut guard = MIDI_CLIENT.lock().map_err(|e| format!("lock: {}", e))?;
        if *guard != 0 {
            return Ok(*guard);
        }
        let mut client: MidiClientRef = 0;
        let name_cf = cf_str_from_bytes(b"rlua-midi");
        let rc = MIDIClientCreate(name_cf, null_mut(), null_mut(), &mut client);
        CFRelease(name_cf);
        if rc != 0 {
            return Err(format!("MIDIClientCreate failed: {}", rc));
        }
        *guard = client;
        Ok(client)
    }
}

// ─── MIDI message ring buffer ─────────────────────────────────────────────────

struct MidiMsg {
    timestamp: u64,
    data: Vec<u8>,
}

type SharedBuf = Arc<Mutex<VecDeque<MidiMsg>>>;

// Maximum messages to buffer per input port
const MIDI_BUF_CAP: usize = 4096;

// ─── CoreMIDI read callback ───────────────────────────────────────────────────
//
// Called from CoreMIDI's internal thread.  The refcon is a *raw* Arc pointer
// (obtained via Arc::into_raw); we reconstruct it, use it, then mem::forget to
// avoid incorrectly decrementing the reference count.

unsafe extern "C" fn midi_read_proc(
    pktlist:   *const libc::c_void,
    refcon:    *mut   libc::c_void,
    _src_conn: *mut   libc::c_void,
) {
    unsafe {
        // Reconstruct Arc from raw pointer — does NOT change refcount.
        let arc = Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>);
        if let Ok(mut q) = arc.lock() {
            // MIDIPacketList layout (packed 4):
            //   [0..4]  numPackets: u32
            //   [4..]   packets (variable length)
            // MIDIPacket layout:
            //   [0..8]  timeStamp: u64
            //   [8..10] length: u16
            //   [10..]  data[length]
            //   (next packet 4-byte aligned on ARM64)
            let num_pkts = std::ptr::read_unaligned(pktlist as *const u32);
            let mut pkt_ptr = (pktlist as *const u8).add(4);

            for _ in 0..num_pkts {
                let timestamp = std::ptr::read_unaligned(pkt_ptr as *const u64);
                let length    = std::ptr::read_unaligned(pkt_ptr.add(8) as *const u16) as usize;
                let data_ptr  = pkt_ptr.add(10);
                let safe_len  = length.min(65535);
                let data = std::slice::from_raw_parts(data_ptr, safe_len).to_vec();

                q.push_back(MidiMsg { timestamp, data });
                while q.len() > MIDI_BUF_CAP { q.pop_front(); }

                // Advance to next packet (4-byte aligned on ARM64)
                let advance = (10 + safe_len + 3) & !3;
                pkt_ptr = pkt_ptr.add(advance);
            }
        }
        std::mem::forget(arc); // don't decrement refcount
    }
}

// ─── Packet list builder ──────────────────────────────────────────────────────
//
// We call the real MIDIPacketListInit / MIDIPacketListAdd from the framework.
// These are deprecated in favour of MIDIEventList but still fully functional.

const SEND_BUF: usize = 65536 + 64; // max packet list + header

unsafe fn send_bytes(port: MidiPortRef, dest: MidiEndpointRef, data: &[u8]) -> OsStatus {
    unsafe {
        if data.is_empty() || data.len() > 65535 {
            return -1;
        }
        let mut buf = vec![0u8; SEND_BUF];
        let pktlist = buf.as_mut_ptr() as *mut libc::c_void;
        let cur = MIDIPacketListInit(pktlist);
        let cur = MIDIPacketListAdd(
            pktlist,
            SEND_BUF as ByteCount,
            cur,
            0, // timestamp 0 = "now"
            data.len() as ByteCount,
            data.as_ptr(),
        );
        if cur.is_null() {
            return -2;
        }
        MIDISend(port, dest, pktlist)
    }
}

// ─── Metatable names ──────────────────────────────────────────────────────────

const OUT_META: *const i8 = c"midi_output*".as_ptr();
const IN_META:  *const i8 = c"midi_input*".as_ptr();

// ─── Output-port userdata ─────────────────────────────────────────────────────

#[repr(C)]
struct MidiOutputData {
    port:     MidiPortRef,
    endpoint: MidiEndpointRef,
    closed:   bool,
}

// ─── Input-port userdata ──────────────────────────────────────────────────────
//
// Two raw pointers for the ring buffer:
//   buf_box:  Box<SharedBuf> — the Lua side's Arc reference (Arc refcount +=1)
//   refcon:   *const Mutex<...> from Arc::into_raw() — the callback's reference

#[repr(C)]
struct MidiInputData {
    port:      MidiPortRef,
    endpoint:  MidiEndpointRef,
    buf_box:   *mut libc::c_void, // *mut Box<SharedBuf>
    refcon:    *mut libc::c_void, // raw ptr from Arc::into_raw for callback
    closed:    bool,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

unsafe fn check_out(interpreter: *mut Interpreter) -> *mut MidiOutputData {
    unsafe {
        let p = lual_checkudata(interpreter, 1, OUT_META) as *mut MidiOutputData;
        if (*p).closed {
            lual_error(interpreter, c"attempt to use a closed MIDI output port".as_ptr());
            unreachable!()
        }
        p
    }
}

unsafe fn check_in(interpreter: *mut Interpreter) -> *mut MidiInputData {
    unsafe {
        let p = lual_checkudata(interpreter, 1, IN_META) as *mut MidiInputData;
        if (*p).closed {
            lual_error(interpreter, c"attempt to use a closed MIDI input port".as_ptr());
            unreachable!()
        }
        p
    }
}

// ─── Output port __gc / __close ───────────────────────────────────────────────

pub unsafe fn midi_out_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, OUT_META) as *mut MidiOutputData;
        if !(*p).closed {
            MIDIPortDispose((*p).port);
            (*p).closed = true;
        }
        0
    }
}

// ─── Input port __gc / __close ────────────────────────────────────────────────

pub unsafe fn midi_in_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, IN_META) as *mut MidiInputData;
        if !(*p).closed {
            MIDIPortDisconnectSource((*p).port, (*p).endpoint);
            MIDIPortDispose((*p).port); // waits for in-flight callbacks to complete
            // Release the callback's Arc ref
            drop(Arc::from_raw((*p).refcon as *const Mutex<VecDeque<MidiMsg>>));
            // Release the Lua-side Box<SharedBuf>
            drop(Box::from_raw((*p).buf_box as *mut SharedBuf));
            (*p).closed = true;
        }
        0
    }
}

// ─── Output port methods ──────────────────────────────────────────────────────

/// port:send(byte, ...) or port:send(string_of_bytes)
pub unsafe fn midi_out_send(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_out(interpreter);
        let nargs = (*interpreter).get_top();

        // If arg 2 is a string, send its raw bytes; otherwise collect integer args.
        let data: Vec<u8> = if lua_type(interpreter, 2) == Some(TagType::String) {
            let mut slen = 0usize;
            let sptr = lua_tolstring(interpreter, 2, &mut slen);
            std::slice::from_raw_parts(sptr as *const u8, slen).to_vec()
        } else {
            let mut v = Vec::new();
            for i in 2..=nargs {
                v.push((lual_checkinteger(interpreter, i) & 0xFF) as u8);
            }
            v
        };

        if data.is_empty() {
            return 0;
        }
        let rc = send_bytes((*p).port, (*p).endpoint, &data);
        if rc != 0 {
            let msg = format!("midi:send failed: {}\0", rc);
            return lual_error(interpreter, msg.as_ptr() as *const i8);
        }
        0
    }
}

/// port:note_on(channel, note, velocity)  — channel 1-16, note/velocity 0-127
pub unsafe fn midi_out_note_on(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_out(interpreter);
        let ch  = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(interpreter, 3) & 0x7F) as u8;
        let vel  = (lual_checkinteger(interpreter, 4) & 0x7F) as u8;
        send_bytes((*p).port, (*p).endpoint, &[0x90 | ch, note, vel]);
        0
    }
}

/// port:note_off(channel, note [, velocity])
pub unsafe fn midi_out_note_off(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_out(interpreter);
        let ch   = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(interpreter, 3) & 0x7F) as u8;
        let vel  = if lua_type(interpreter, 4) == Some(TagType::Numeric) {
            (lua_tointegerx(interpreter, 4, null_mut()) & 0x7F) as u8
        } else { 64 };
        send_bytes((*p).port, (*p).endpoint, &[0x80 | ch, note, vel]);
        0
    }
}

/// port:cc(channel, controller, value)  — control change
pub unsafe fn midi_out_cc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p  = check_out(interpreter);
        let ch  = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let cc  = (lual_checkinteger(interpreter, 3) & 0x7F) as u8;
        let val = (lual_checkinteger(interpreter, 4) & 0x7F) as u8;
        send_bytes((*p).port, (*p).endpoint, &[0xB0 | ch, cc, val]);
        0
    }
}

/// port:program_change(channel, program)  — program 0-127
pub unsafe fn midi_out_program_change(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p   = check_out(interpreter);
        let ch  = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let pgm = (lual_checkinteger(interpreter, 3) & 0x7F) as u8;
        send_bytes((*p).port, (*p).endpoint, &[0xC0 | ch, pgm]);
        0
    }
}

/// port:pitch_bend(channel, value)  — value -8192..8191
pub unsafe fn midi_out_pitch_bend(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p    = check_out(interpreter);
        let ch   = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let val  = lual_checkinteger(interpreter, 3).clamp(-8192, 8191);
        let bent = (val + 8192) as u16; // 0..16383
        let lsb  = (bent & 0x7F) as u8;
        let msb  = ((bent >> 7) & 0x7F) as u8;
        send_bytes((*p).port, (*p).endpoint, &[0xE0 | ch, lsb, msb]);
        0
    }
}

/// port:aftertouch(channel, note, pressure)  — polyphonic key pressure
pub unsafe fn midi_out_aftertouch(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p    = check_out(interpreter);
        let ch   = ((lual_checkinteger(interpreter, 2) - 1) & 0xF) as u8;
        let note = (lual_checkinteger(interpreter, 3) & 0x7F) as u8;
        let pres = (lual_checkinteger(interpreter, 4) & 0x7F) as u8;
        send_bytes((*p).port, (*p).endpoint, &[0xA0 | ch, note, pres]);
        0
    }
}

/// port:clock()  — send MIDI timing clock (0xF8)
pub unsafe fn midi_out_clock(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_out(interpreter);
        send_bytes((*p).port, (*p).endpoint, &[0xF8]);
        0
    }
}

/// port:close()
pub unsafe fn midi_out_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, OUT_META) as *mut MidiOutputData;
        if !(*p).closed {
            MIDIPortDispose((*p).port);
            (*p).closed = true;
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── Input port methods ───────────────────────────────────────────────────────

/// port:recv([timeout_ms]) → {data=string, time=integer} | nil
/// timeout_ms < 0: block indefinitely; 0 (default): non-blocking poll
pub unsafe fn midi_in_recv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_in(interpreter);
        let timeout_ms: i64 = if lua_type(interpreter, 2) == Some(TagType::Numeric) {
            lua_tointegerx(interpreter, 2, null_mut())
        } else {
            0
        };

        let buf_box = &*((*p).buf_box as *const SharedBuf);

        let deadline = if timeout_ms > 0 {
            Some(std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms as u64))
        } else {
            None
        };

        loop {
            // Try to pop a message
            {
                let mut q = buf_box.lock().unwrap();
                if let Some(msg) = q.pop_front() {
                    (*interpreter).lua_createtable();

                    lua_pushlstring(interpreter, b"data".as_ptr() as *const i8, 4);
                    lua_pushlstring(interpreter, msg.data.as_ptr() as *const i8, msg.data.len());
                    lua_rawset(interpreter, -3);

                    lua_pushlstring(interpreter, b"time".as_ptr() as *const i8, 4);
                    (*interpreter).push_integer(msg.timestamp as i64);
                    lua_rawset(interpreter, -3);

                    return 1;
                }
            }

            if timeout_ms == 0 { break; }

            match deadline {
                Some(dl) if std::time::Instant::now() >= dl => break,
                _ => {}
            }

            // Sleep 1 ms to avoid spinning
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        (*interpreter).push_nil();
        1
    }
}

/// port:pending() → number of messages in buffer
pub unsafe fn midi_in_pending(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_in(interpreter);
        let buf_box = &*((*p).buf_box as *const SharedBuf);
        let count = buf_box.lock().unwrap().len();
        (*interpreter).push_integer(count as i64);
        1
    }
}

/// port:flush() — discard all buffered messages
pub unsafe fn midi_in_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = check_in(interpreter);
        let buf_box = &*((*p).buf_box as *const SharedBuf);
        buf_box.lock().unwrap().clear();
        0
    }
}

/// port:close()
pub unsafe fn midi_in_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, IN_META) as *mut MidiInputData;
        if !(*p).closed {
            MIDIPortDisconnectSource((*p).port, (*p).endpoint);
            MIDIPortDispose((*p).port);
            drop(Arc::from_raw((*p).refcon as *const Mutex<VecDeque<MidiMsg>>));
            drop(Box::from_raw((*p).buf_box as *mut SharedBuf));
            (*p).closed = true;
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── Module-level functions ───────────────────────────────────────────────────

/// midi.sources() → [{name=..., index=...}, ...]
pub unsafe fn midi_sources(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n = MIDIGetNumberOfSources();
        (*interpreter).lua_createtable();
        for i in 0..n {
            let endpoint = MIDIGetSource(i);
            let name     = endpoint_name(endpoint);

            (*interpreter).push_integer((i + 1) as i64); // key (1-based)
            (*interpreter).lua_createtable();

            lua_pushlstring(interpreter, b"name".as_ptr() as *const i8, 4);
            lua_pushlstring(interpreter, name.as_ptr() as *const i8, name.len());
            lua_rawset(interpreter, -3);

            lua_pushlstring(interpreter, b"index".as_ptr() as *const i8, 5);
            (*interpreter).push_integer((i + 1) as i64);
            lua_rawset(interpreter, -3);

            lua_rawset(interpreter, -3);
        }
        1
    }
}

/// midi.destinations() → [{name=..., index=...}, ...]
pub unsafe fn midi_destinations(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n = MIDIGetNumberOfDestinations();
        (*interpreter).lua_createtable();
        for i in 0..n {
            let endpoint = MIDIGetDestination(i);
            let name     = endpoint_name(endpoint);

            (*interpreter).push_integer((i + 1) as i64);
            (*interpreter).lua_createtable();

            lua_pushlstring(interpreter, b"name".as_ptr() as *const i8, 4);
            lua_pushlstring(interpreter, name.as_ptr() as *const i8, name.len());
            lua_rawset(interpreter, -3);

            lua_pushlstring(interpreter, b"index".as_ptr() as *const i8, 5);
            (*interpreter).push_integer((i + 1) as i64);
            lua_rawset(interpreter, -3);

            lua_rawset(interpreter, -3);
        }
        1
    }
}

/// Resolve a destination by 1-based index (integer) or name (string).
/// Returns MidiEndpointRef 0 on failure.
unsafe fn resolve_destination(interpreter: *mut Interpreter, arg: i32) -> MidiEndpointRef {
    unsafe {
        let n = MIDIGetNumberOfDestinations();
        match lua_type(interpreter, arg) {
            Some(TagType::Numeric) => {
                let idx = lua_tointegerx(interpreter, arg, null_mut());
                if idx < 1 || idx as libc::c_ulong > n { return 0; }
                MIDIGetDestination((idx - 1) as ItemCount)
            }
            Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(interpreter, arg, &mut slen);
                let want = std::slice::from_raw_parts(sptr as *const u8, slen);
                for i in 0..n {
                    let ep = MIDIGetDestination(i);
                    let name = endpoint_name(ep);
                    if name.as_bytes() == want { return ep; }
                }
                0
            }
            _ => 0,
        }
    }
}

/// Resolve a source by 1-based index or name.
unsafe fn resolve_source(interpreter: *mut Interpreter, arg: i32) -> MidiEndpointRef {
    unsafe {
        let n = MIDIGetNumberOfSources();
        match lua_type(interpreter, arg) {
            Some(TagType::Numeric) => {
                let idx = lua_tointegerx(interpreter, arg, null_mut());
                if idx < 1 || idx as libc::c_ulong > n { return 0; }
                MIDIGetSource((idx - 1) as ItemCount)
            }
            Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(interpreter, arg, &mut slen);
                let want = std::slice::from_raw_parts(sptr as *const u8, slen);
                for i in 0..n {
                    let ep = MIDIGetSource(i);
                    let name = endpoint_name(ep);
                    if name.as_bytes() == want { return ep; }
                }
                0
            }
            _ => 0,
        }
    }
}

/// midi.open_output(index_or_name) → port | nil, errmsg
pub unsafe fn midi_open_output(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let client = match get_or_create_client() {
            Ok(c)  => c,
            Err(e) => {
                (*interpreter).push_nil();
                let s = format!("{}\0", e);
                return lual_error(interpreter, s.as_ptr() as *const i8);
            }
        };

        let endpoint = resolve_destination(interpreter, 1);
        if endpoint == 0 {
            (*interpreter).push_nil();
            lua_pushstring(interpreter, c"MIDI destination not found".as_ptr());
            return 2;
        }

        let port_name_cf = cf_str_from_bytes(b"rlua-output");
        let mut port: MidiPortRef = 0;
        let rc = MIDIOutputPortCreate(client, port_name_cf, &mut port);
        CFRelease(port_name_cf);

        if rc != 0 {
            (*interpreter).push_nil();
            let msg = format!("MIDIOutputPortCreate failed: {}\0", rc);
            lua_pushstring(interpreter, msg.as_ptr() as *const libc::c_char);
            return 2;
        }

        let ud = User::lua_newuserdatauv(interpreter, size_of::<MidiOutputData>(), 0)
            as *mut MidiOutputData;
        (*ud).port     = port;
        (*ud).endpoint = endpoint;
        (*ud).closed   = false;
        lual_setmetatable(interpreter, OUT_META);
        1
    }
}

/// midi.open_input(index_or_name) → port | nil, errmsg
pub unsafe fn midi_open_input(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let client = match get_or_create_client() {
            Ok(c) => c,
            Err(e) => {
                (*interpreter).push_nil();
                let s = format!("{}\0", e);
                return lual_error(interpreter, s.as_ptr() as *const i8);
            }
        };

        let endpoint = resolve_source(interpreter, 1);
        if endpoint == 0 {
            (*interpreter).push_nil();
            lua_pushstring(interpreter, c"MIDI source not found".as_ptr());
            return 2;
        }

        // Create shared ring buffer.
        let buf: SharedBuf = Arc::new(Mutex::new(VecDeque::new()));
        let callback_arc   = buf.clone(); // Arc refcount = 2
        let refcon         = Arc::into_raw(callback_arc) as *mut libc::c_void;
        let buf_box        = Box::into_raw(Box::new(buf)) as *mut libc::c_void;

        let port_name_cf = cf_str_from_bytes(b"rlua-input");
        let mut port: MidiPortRef = 0;
        let rc = MIDIInputPortCreate(client, port_name_cf, midi_read_proc, refcon, &mut port);
        CFRelease(port_name_cf);

        if rc != 0 {
            // Clean up the arcs we allocated
            drop(Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>));
            drop(Box::from_raw(buf_box as *mut SharedBuf));
            (*interpreter).push_nil();
            let msg = format!("MIDIInputPortCreate failed: {}\0", rc);
            lua_pushstring(interpreter, msg.as_ptr() as *const libc::c_char);
            return 2;
        }

        let rc2 = MIDIPortConnectSource(port, endpoint, null_mut());
        if rc2 != 0 {
            MIDIPortDispose(port);
            drop(Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>));
            drop(Box::from_raw(buf_box as *mut SharedBuf));
            (*interpreter).push_nil();
            let msg = format!("MIDIPortConnectSource failed: {}\0", rc2);
            lua_pushstring(interpreter, msg.as_ptr() as *const libc::c_char);
            return 2;
        }

        let ud = User::lua_newuserdatauv(interpreter, size_of::<MidiInputData>(), 0)
            as *mut MidiInputData;
        (*ud).port     = port;
        (*ud).endpoint = endpoint;
        (*ud).buf_box  = buf_box;
        (*ud).refcon   = refcon;
        (*ud).closed   = false;
        lual_setmetatable(interpreter, IN_META);
        1
    }
}

// ─── Registration ─────────────────────────────────────────────────────────────

pub const MIDI_FUNCS: [RegisteredFunction; 4] = [
    RegisteredFunction {
        registeredfunction_name:     c"sources".as_ptr(),
        registeredfunction_function: Some(midi_sources as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"destinations".as_ptr(),
        registeredfunction_function: Some(midi_destinations as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"open_output".as_ptr(),
        registeredfunction_function: Some(midi_open_output as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"open_input".as_ptr(),
        registeredfunction_function: Some(midi_open_input as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const MIDI_OUT_METHODS: [RegisteredFunction; 9] = [
    RegisteredFunction {
        registeredfunction_name:     c"send".as_ptr(),
        registeredfunction_function: Some(midi_out_send as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"note_on".as_ptr(),
        registeredfunction_function: Some(midi_out_note_on as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"note_off".as_ptr(),
        registeredfunction_function: Some(midi_out_note_off as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"cc".as_ptr(),
        registeredfunction_function: Some(midi_out_cc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"program_change".as_ptr(),
        registeredfunction_function: Some(midi_out_program_change as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"pitch_bend".as_ptr(),
        registeredfunction_function: Some(midi_out_pitch_bend as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"aftertouch".as_ptr(),
        registeredfunction_function: Some(midi_out_aftertouch as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"clock".as_ptr(),
        registeredfunction_function: Some(midi_out_clock as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"close".as_ptr(),
        registeredfunction_function: Some(midi_out_close as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const MIDI_OUT_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name:     c"__gc".as_ptr(),
        registeredfunction_function: Some(midi_out_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"__close".as_ptr(),
        registeredfunction_function: Some(midi_out_gc as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const MIDI_IN_METHODS: [RegisteredFunction; 4] = [
    RegisteredFunction {
        registeredfunction_name:     c"recv".as_ptr(),
        registeredfunction_function: Some(midi_in_recv as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"pending".as_ptr(),
        registeredfunction_function: Some(midi_in_pending as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"flush".as_ptr(),
        registeredfunction_function: Some(midi_in_flush as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"close".as_ptr(),
        registeredfunction_function: Some(midi_in_close as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const MIDI_IN_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name:     c"__gc".as_ptr(),
        registeredfunction_function: Some(midi_in_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name:     c"__close".as_ptr(),
        registeredfunction_function: Some(midi_in_gc as unsafe fn(*mut Interpreter) -> i32),
    },
];

// ─── luaopen_midi ─────────────────────────────────────────────────────────────

pub unsafe fn luaopen_midi(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        // Output-port metatable
        lual_newmetatable(interpreter, OUT_META);
        lual_setfuncs(interpreter, MIDI_OUT_META.as_ptr(), MIDI_OUT_META.len(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, MIDI_OUT_METHODS.as_ptr(), MIDI_OUT_METHODS.len(), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2); // pop out metatable

        // Input-port metatable
        lual_newmetatable(interpreter, IN_META);
        lual_setfuncs(interpreter, MIDI_IN_META.as_ptr(), MIDI_IN_META.len(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, MIDI_IN_METHODS.as_ptr(), MIDI_IN_METHODS.len(), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2); // pop in metatable

        // Library table
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, MIDI_FUNCS.as_ptr(), MIDI_FUNCS.len(), 0);

        // ── MIDI status byte constants ────────────────────────────────────────
        macro_rules! set_int {
            ($name:expr, $val:expr) => {
                (*interpreter).push_integer($val);
                lua_setfield(interpreter, -2, $name);
            };
        }

        set_int!(c"NOTE_OFF".as_ptr(),          0x80);
        set_int!(c"NOTE_ON".as_ptr(),           0x90);
        set_int!(c"AFTERTOUCH".as_ptr(),        0xA0);
        set_int!(c"CC".as_ptr(),                0xB0);
        set_int!(c"PROGRAM_CHANGE".as_ptr(),    0xC0);
        set_int!(c"CHANNEL_PRESSURE".as_ptr(),  0xD0);
        set_int!(c"PITCH_BEND".as_ptr(),        0xE0);
        set_int!(c"SYSEX".as_ptr(),             0xF0);
        set_int!(c"QUARTER_FRAME".as_ptr(),     0xF1);
        set_int!(c"SONG_POSITION".as_ptr(),     0xF2);
        set_int!(c"SONG_SELECT".as_ptr(),       0xF3);
        set_int!(c"TUNE_REQUEST".as_ptr(),      0xF6);
        set_int!(c"SYSEX_END".as_ptr(),         0xF7);
        set_int!(c"CLOCK".as_ptr(),             0xF8);
        set_int!(c"START".as_ptr(),             0xFA);
        set_int!(c"CONTINUE".as_ptr(),          0xFB);
        set_int!(c"STOP".as_ptr(),              0xFC);
        set_int!(c"ACTIVE_SENSING".as_ptr(),    0xFE);
        set_int!(c"RESET".as_ptr(),             0xFF);

        1
    }
}

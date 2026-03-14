use super::backend::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const POLLIN: i16 = 0x0001;

#[repr(C)]
#[derive(Clone)]
struct Pollfd {
    fd: i32,
    events: i16,
    revents: i16,
}

unsafe extern "C" {
    fn poll(fds: *mut Pollfd, nfds: u64, timeout: i32) -> i32;
}

// ─── ALSA sequencer FFI ──────────────────────────────────────────────────────

const SND_SEQ_OPEN_OUTPUT: i32 = 1;
const SND_SEQ_OPEN_INPUT: i32 = 2;
const SND_SEQ_OPEN_DUPLEX: i32 = SND_SEQ_OPEN_OUTPUT | SND_SEQ_OPEN_INPUT;
const SND_SEQ_PORT_CAP_READ: u32 = 1 << 0;
const SND_SEQ_PORT_CAP_WRITE: u32 = 1 << 1;
const SND_SEQ_PORT_CAP_SUBS_READ: u32 = 1 << 5;
const SND_SEQ_PORT_CAP_SUBS_WRITE: u32 = 1 << 6;
const SND_SEQ_PORT_TYPE_MIDI_GENERIC: u32 = 1 << 1;
const SND_SEQ_PORT_TYPE_APPLICATION: u32 = 1 << 20;

// Event types we care about
const SND_SEQ_EVENT_NOTEON: u8 = 6;
const SND_SEQ_EVENT_NOTEOFF: u8 = 7;
const SND_SEQ_EVENT_KEYPRESS: u8 = 8;
const SND_SEQ_EVENT_CONTROLLER: u8 = 10;
const SND_SEQ_EVENT_PGMCHANGE: u8 = 11;
const SND_SEQ_EVENT_CHANPRESS: u8 = 12;
const SND_SEQ_EVENT_PITCHBEND: u8 = 13;
const SND_SEQ_EVENT_SYSEX: u8 = 130;
const SND_SEQ_EVENT_CLOCK: u8 = 36;
const SND_SEQ_EVENT_START: u8 = 38;
const SND_SEQ_EVENT_CONTINUE: u8 = 39;
const SND_SEQ_EVENT_STOP: u8 = 40;
const SND_SEQ_EVENT_SENSING: u8 = 42;
const SND_SEQ_EVENT_RESET: u8 = 43;
const SND_SEQ_EVENT_QFRAME: u8 = 34;
const SND_SEQ_EVENT_SONGPOS: u8 = 35;
const SND_SEQ_EVENT_SONGSEL: u8 = 37;
const SND_SEQ_EVENT_TUNE_REQUEST: u8 = 41;

#[allow(unused)]
const SND_SEQ_ADDRESS_SUBSCRIBERS: u8 = 254;
const SND_SEQ_QUEUE_DIRECT: u8 = 253;

// snd_seq_event_t is 28 bytes on 64-bit Linux.
// We use a raw byte buffer and read fields at known offsets.
const SEQ_EVENT_SIZE: usize = 28;

#[allow(unused)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
struct SndSeqAddr {
    client: u8,
    port: u8,
}

#[allow(unused)]
#[repr(C)]
struct SndSeqPortSubscribe {
    _opaque: [u8; 0],
}

#[allow(unused)]
#[repr(C)]
struct SndSeqPortInfo {
    _opaque: [u8; 0],
}

#[allow(unused)]
#[repr(C)]
struct SndSeqClientInfo {
    _opaque: [u8; 0],
}

#[link(name = "asound")]
unsafe extern "C" {
    fn snd_seq_open(handle: *mut *mut std::ffi::c_void, name: *const std::ffi::c_char, streams: i32, mode: i32) -> i32;
    fn snd_seq_close(handle: *mut std::ffi::c_void) -> i32;
    fn snd_seq_set_client_name(handle: *mut std::ffi::c_void, name: *const std::ffi::c_char) -> i32;
    #[allow(unused)]
    fn snd_seq_client_id(handle: *mut std::ffi::c_void) -> i32;
    fn snd_seq_create_simple_port(handle: *mut std::ffi::c_void, name: *const std::ffi::c_char, caps: u32, port_type: u32) -> i32;
    fn snd_seq_delete_simple_port(handle: *mut std::ffi::c_void, port: i32) -> i32;

    fn snd_seq_connect_to(handle: *mut std::ffi::c_void, my_port: i32, dest_client: i32, dest_port: i32) -> i32;
    fn snd_seq_connect_from(handle: *mut std::ffi::c_void, my_port: i32, src_client: i32, src_port: i32) -> i32;
    fn snd_seq_disconnect_to(handle: *mut std::ffi::c_void, my_port: i32, dest_client: i32, dest_port: i32) -> i32;
    fn snd_seq_disconnect_from(handle: *mut std::ffi::c_void, my_port: i32, src_client: i32, src_port: i32) -> i32;

    fn snd_seq_event_output_direct(handle: *mut std::ffi::c_void, ev: *mut std::ffi::c_void) -> i32;
    fn snd_seq_event_input(handle: *mut std::ffi::c_void, ev: *mut *mut std::ffi::c_void) -> i32;
    #[allow(unused)]
    fn snd_seq_event_input_pending(handle: *mut std::ffi::c_void, fetch_sequencer: i32) -> i32;

    fn snd_seq_client_info_sizeof() -> usize;
    fn snd_seq_port_info_sizeof() -> usize;
    fn snd_seq_client_info_set_client(info: *mut std::ffi::c_void, client: i32);
    fn snd_seq_port_info_set_client(info: *mut std::ffi::c_void, client: i32);
    fn snd_seq_port_info_set_port(info: *mut std::ffi::c_void, port: i32);
    fn snd_seq_query_next_client(handle: *mut std::ffi::c_void, info: *mut std::ffi::c_void) -> i32;
    fn snd_seq_query_next_port(handle: *mut std::ffi::c_void, info: *mut std::ffi::c_void) -> i32;

    fn snd_seq_client_info_get_client(info: *const std::ffi::c_void) -> i32;
    fn snd_seq_client_info_get_name(info: *const std::ffi::c_void) -> *const std::ffi::c_char;
    fn snd_seq_port_info_get_port(info: *const std::ffi::c_void) -> i32;
    fn snd_seq_port_info_get_name(info: *const std::ffi::c_void) -> *const std::ffi::c_char;
    fn snd_seq_port_info_get_capability(info: *const std::ffi::c_void) -> u32;
    fn snd_seq_port_info_get_type(info: *const std::ffi::c_void) -> u32;

    fn snd_seq_nonblock(handle: *mut std::ffi::c_void, nonblock: i32) -> i32;
    fn snd_seq_poll_descriptors_count(handle: *mut std::ffi::c_void, events: i16) -> i32;
    fn snd_seq_poll_descriptors(handle: *mut std::ffi::c_void, pfds: *mut Pollfd, space: u32, events: i16) -> i32;

    fn snd_strerror(errnum: i32) -> *const std::ffi::c_char;
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

unsafe fn alsa_error(rc: i32) -> String {
    unsafe {
        let p = snd_strerror(rc);
        if p.is_null() {
            format!("ALSA error {}", rc)
        } else {
            std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned()
        }
    }
}

struct SeqHandle(*mut std::ffi::c_void);
unsafe impl Send for SeqHandle {}

impl SeqHandle {
    fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.0
    }
}

impl Drop for SeqHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                snd_seq_close(self.0);
            }
            self.0 = std::ptr::null_mut();
        }
    }
}

// ─── Port enumeration ────────────────────────────────────────────────────────

#[derive(Clone)]
struct AlsaPort {
    client_id: i32,
    port_id: i32,
    client_name: String,
    port_name: String,
}

impl AlsaPort {
    fn display_name(&self) -> String {
        format!("{}:{}", self.client_name, self.port_name)
    }
}

unsafe fn enumerate_ports(cap_mask: u32) -> Vec<AlsaPort> {
    unsafe {
        let mut handle: *mut std::ffi::c_void = std::ptr::null_mut();
        let rc = snd_seq_open(&mut handle, c"default".as_ptr(), SND_SEQ_OPEN_DUPLEX, 0);
        if rc < 0 {
            return Vec::new();
        }

        let mut result = Vec::new();

        let ci_size = snd_seq_client_info_sizeof();
        let pi_size = snd_seq_port_info_sizeof();
        let mut ci_buf = vec![0u8; ci_size];
        let mut pi_buf = vec![0u8; pi_size];
        let ci = ci_buf.as_mut_ptr() as *mut std::ffi::c_void;
        let pi = pi_buf.as_mut_ptr() as *mut std::ffi::c_void;

        snd_seq_client_info_set_client(ci, -1);
        while snd_seq_query_next_client(handle, ci) >= 0 {
            let cid = snd_seq_client_info_get_client(ci);
            let cname_ptr = snd_seq_client_info_get_name(ci);
            let cname = if cname_ptr.is_null() {
                format!("client-{}", cid)
            } else {
                std::ffi::CStr::from_ptr(cname_ptr).to_string_lossy().into_owned()
            };

            snd_seq_port_info_set_client(pi, cid);
            snd_seq_port_info_set_port(pi, -1);
            while snd_seq_query_next_port(handle, pi) >= 0 {
                let caps = snd_seq_port_info_get_capability(pi);
                let ptype = snd_seq_port_info_get_type(pi);
                if (caps & cap_mask) != cap_mask {
                    continue;
                }
                if (ptype & SND_SEQ_PORT_TYPE_MIDI_GENERIC) == 0 {
                    continue;
                }

                let pid = snd_seq_port_info_get_port(pi);
                let pname_ptr = snd_seq_port_info_get_name(pi);
                let pname = if pname_ptr.is_null() {
                    format!("port-{}", pid)
                } else {
                    std::ffi::CStr::from_ptr(pname_ptr).to_string_lossy().into_owned()
                };

                result.push(AlsaPort { client_id: cid, port_id: pid, client_name: cname.clone(), port_name: pname });
            }
        }

        snd_seq_close(handle);
        result
    }
}

fn resolve_port(ports: &[AlsaPort], sel: &EndpointSelector) -> Option<AlsaPort> {
    match sel {
        | EndpointSelector::Index(idx) => {
            if *idx < 1 || *idx > ports.len() {
                return None;
            }
            Some(ports[*idx - 1].clone())
        },
        | EndpointSelector::Name(want) => ports
            .iter()
            .find(|p| p.display_name() == *want || p.port_name == *want)
            .cloned(),
    }
}

// ─── ALSA sequencer event helpers ────────────────────────────────────────────

/// Convert an ALSA sequencer event to raw MIDI bytes.
unsafe fn event_to_bytes(ev: *const std::ffi::c_void) -> Option<Vec<u8>> {
    unsafe {
        let ev_type = *(ev as *const u8);
        // Data fields start at offset 12 in snd_seq_event_t.
        // For note/controller events the data union is snd_seq_ev_note or snd_seq_ev_ctrl,
        // laid out as: channel(u8), note/param(u8), velocity/value(u8/u32), ...
        // We read the fields from the raw pointer at the data offset.
        let data_ptr = (ev as *const u8).add(12);

        match ev_type {
            | SND_SEQ_EVENT_NOTEON => {
                let channel = *data_ptr;
                let note = *data_ptr.add(1);
                let velocity = *data_ptr.add(2);
                Some(vec![0x90 | (channel & 0xF), note & 0x7F, velocity & 0x7F])
            },
            | SND_SEQ_EVENT_NOTEOFF => {
                let channel = *data_ptr;
                let note = *data_ptr.add(1);
                let velocity = *data_ptr.add(2);
                Some(vec![0x80 | (channel & 0xF), note & 0x7F, velocity & 0x7F])
            },
            | SND_SEQ_EVENT_KEYPRESS => {
                let channel = *data_ptr;
                let note = *data_ptr.add(1);
                let velocity = *data_ptr.add(2);
                Some(vec![0xA0 | (channel & 0xF), note & 0x7F, velocity & 0x7F])
            },
            | SND_SEQ_EVENT_CONTROLLER => {
                let channel = *data_ptr;
                // snd_seq_ev_ctrl: channel(u8), pad[3], param(u32), value(i32)
                let param = std::ptr::read_unaligned(data_ptr.add(4) as *const u32);
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                Some(vec![0xB0 | (channel & 0xF), (param & 0x7F) as u8, (value & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_PGMCHANGE => {
                let channel = *data_ptr;
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                Some(vec![0xC0 | (channel & 0xF), (value & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_CHANPRESS => {
                let channel = *data_ptr;
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                Some(vec![0xD0 | (channel & 0xF), (value & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_PITCHBEND => {
                let channel = *data_ptr;
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                // ALSA pitch bend: -8192..8191, MIDI wire: 0..16383
                let bent = (value + 8192).clamp(0, 16383) as u16;
                let lsb = (bent & 0x7F) as u8;
                let msb = ((bent >> 7) & 0x7F) as u8;
                Some(vec![0xE0 | (channel & 0xF), lsb, msb])
            },
            | SND_SEQ_EVENT_SYSEX => {
                // snd_seq_ev_ext: flags(u32), len(u32), ptr(*void)
                let len = std::ptr::read_unaligned(data_ptr.add(4) as *const u32) as usize;
                let pointer = std::ptr::read_unaligned(data_ptr.add(8) as *const *const u8);
                if pointer.is_null() || len == 0 {
                    return None;
                }
                Some(std::slice::from_raw_parts(pointer, len).to_vec())
            },
            | SND_SEQ_EVENT_CLOCK => Some(vec![0xF8]),
            | SND_SEQ_EVENT_START => Some(vec![0xFA]),
            | SND_SEQ_EVENT_CONTINUE => Some(vec![0xFB]),
            | SND_SEQ_EVENT_STOP => Some(vec![0xFC]),
            | SND_SEQ_EVENT_SENSING => Some(vec![0xFE]),
            | SND_SEQ_EVENT_RESET => Some(vec![0xFF]),
            | SND_SEQ_EVENT_QFRAME => {
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                Some(vec![0xF1, (value & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_SONGPOS => {
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                let v = value.clamp(0, 16383) as u16;
                Some(vec![0xF2, (v & 0x7F) as u8, ((v >> 7) & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_SONGSEL => {
                let value = std::ptr::read_unaligned(data_ptr.add(8) as *const i32);
                Some(vec![0xF3, (value & 0x7F) as u8])
            },
            | SND_SEQ_EVENT_TUNE_REQUEST => Some(vec![0xF6]),
            | _ => None,
        }
    }
}

/// Build and send an ALSA sequencer event from raw MIDI bytes.
unsafe fn send_midi_event(
    handle: *mut std::ffi::c_void, port: i32, dest_client: i32, dest_port: i32, data: &[u8],
) -> Result<(), String> {
    unsafe {
        if data.is_empty() {
            return Err("empty data".into());
        }

        // Zero-initialize event buffer
        let mut ev_buf = [0u8; SEQ_EVENT_SIZE];
        let ev = ev_buf.as_mut_ptr() as *mut std::ffi::c_void;

        let status = data[0];
        let data_ptr = (ev as *mut u8).add(12);

        // Set source port — offset 2
        *((ev as *mut u8).add(2)) = port as u8;
        // Set dest — offset 4 (client), offset 5 (port)
        *((ev as *mut u8).add(4)) = dest_client as u8;
        *((ev as *mut u8).add(5)) = dest_port as u8;
        // Set queue = DIRECT — offset 6
        *((ev as *mut u8).add(6)) = SND_SEQ_QUEUE_DIRECT;

        match status & 0xF0 {
            | 0x90 if data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_NOTEON;
                // flags byte at offset 1: set length mode to fixed (0)
                *data_ptr = status & 0x0F; // channel
                *data_ptr.add(1) = data[1]; // note
                *data_ptr.add(2) = data[2]; // velocity
            },
            | 0x80 if data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_NOTEOFF;
                *data_ptr = status & 0x0F;
                *data_ptr.add(1) = data[1];
                *data_ptr.add(2) = data[2];
            },
            | 0xA0 if data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_KEYPRESS;
                *data_ptr = status & 0x0F;
                *data_ptr.add(1) = data[1];
                *data_ptr.add(2) = data[2];
            },
            | 0xB0 if data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_CONTROLLER;
                *data_ptr = status & 0x0F; // channel
                // param at offset +4 (u32)
                std::ptr::write_unaligned(data_ptr.add(4) as *mut u32, data[1] as u32);
                // value at offset +8 (i32)
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, data[2] as i32);
            },
            | 0xC0 if data.len() >= 2 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_PGMCHANGE;
                *data_ptr = status & 0x0F;
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, data[1] as i32);
            },
            | 0xD0 if data.len() >= 2 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_CHANPRESS;
                *data_ptr = status & 0x0F;
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, data[1] as i32);
            },
            | 0xE0 if data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_PITCHBEND;
                *data_ptr = status & 0x0F;
                let lsb = data[1] as i32;
                let msb = data[2] as i32;
                let value = ((msb << 7) | lsb) - 8192;
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, value);
            },
            | _ if status == 0xF0 => {
                // SysEx: use snd_seq_ev_ext with variable-length data
                *(ev as *mut u8) = SND_SEQ_EVENT_SYSEX;
                // flags: SND_SEQ_EVENT_LENGTH_VARIABLE = (1 << 2)
                *((ev as *mut u8).add(1)) = 1 << 2;
                std::ptr::write_unaligned(data_ptr.add(4) as *mut u32, data.len() as u32);
                std::ptr::write_unaligned(data_ptr.add(8) as *mut *const u8, data.as_ptr());
            },
            | _ if status == 0xF8 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_CLOCK;
            },
            | _ if status == 0xFA => {
                *(ev as *mut u8) = SND_SEQ_EVENT_START;
            },
            | _ if status == 0xFB => {
                *(ev as *mut u8) = SND_SEQ_EVENT_CONTINUE;
            },
            | _ if status == 0xFC => {
                *(ev as *mut u8) = SND_SEQ_EVENT_STOP;
            },
            | _ if status == 0xFE => {
                *(ev as *mut u8) = SND_SEQ_EVENT_SENSING;
            },
            | _ if status == 0xFF => {
                *(ev as *mut u8) = SND_SEQ_EVENT_RESET;
            },
            | _ if status == 0xF6 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_TUNE_REQUEST;
            },
            | _ if status == 0xF1 && data.len() >= 2 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_QFRAME;
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, data[1] as i32);
            },
            | _ if status == 0xF2 && data.len() >= 3 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_SONGPOS;
                let v = (data[1] as i32) | ((data[2] as i32) << 7);
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, v);
            },
            | _ if status == 0xF3 && data.len() >= 2 => {
                *(ev as *mut u8) = SND_SEQ_EVENT_SONGSEL;
                std::ptr::write_unaligned(data_ptr.add(8) as *mut i32, data[1] as i32);
            },
            | _ => return Err(format!("unsupported MIDI status 0x{:02X}", status)),
        }

        let rc = snd_seq_event_output_direct(handle, ev);
        if rc < 0 {
            Err(format!("snd_seq_event_output_direct: {}", alsa_error(rc)))
        } else {
            Ok(())
        }
    }
}

// ─── Output port ─────────────────────────────────────────────────────────────

pub struct AlsaOutput {
    handle: SeqHandle,
    port_id: i32,
    dest_client: i32,
    dest_port: i32,
    closed: bool,
}

impl MidiOutputPort for AlsaOutput {
    fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if self.closed {
            return Err("port closed".into());
        }
        unsafe { send_midi_event(self.handle.as_ptr(), self.port_id, self.dest_client, self.dest_port, data) }
    }

    fn close(&mut self) {
        if !self.closed {
            unsafe {
                snd_seq_disconnect_to(self.handle.as_ptr(), self.port_id, self.dest_client, self.dest_port);
                snd_seq_delete_simple_port(self.handle.as_ptr(), self.port_id);
            }
            self.closed = true;
        }
    }
}

impl Drop for AlsaOutput {
    fn drop(&mut self) {
        self.close();
    }
}

// ─── Input port ──────────────────────────────────────────────────────────────

pub struct AlsaInput {
    buf: SharedBuf,
    #[allow(unused)]
    handle: SeqHandle,
    #[allow(unused)]
    port_id: i32,
    #[allow(unused)]
    src_client: i32,
    #[allow(unused)]
    src_port: i32,
    closed: bool,
    thread: Option<std::thread::JoinHandle<()>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MidiInputPort for AlsaInput {
    fn buffer(&self) -> &SharedBuf {
        &self.buf
    }

    fn close(&mut self) {
        if !self.closed {
            self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
            if let Some(th) = self.thread.take() {
                let _ = th.join();
            }
            // handle/port cleanup happens via SeqHandle Drop in the reader thread's handle
            self.closed = true;
        }
    }
}

impl Drop for AlsaInput {
    fn drop(&mut self) {
        self.close();
    }
}

// ─── Input reader thread ─────────────────────────────────────────────────────

fn input_reader_thread(
    handle: SeqHandle, port_id: i32, src_client: i32, src_port: i32, buf: SharedBuf, stop: Arc<std::sync::atomic::AtomicBool>,
) {
    unsafe {
        // Set non-blocking so we can poll
        snd_seq_nonblock(handle.as_ptr(), 1);

        let nfds = snd_seq_poll_descriptors_count(handle.as_ptr(), POLLIN as i16);
        let mut pfds = vec![Pollfd { fd: 0, events: 0, revents: 0 }; nfds as usize];

        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            snd_seq_poll_descriptors(handle.as_ptr(), pfds.as_mut_ptr(), nfds as u32, POLLIN as i16);
            let poll_rc = poll(pfds.as_mut_ptr(), nfds as u64, 10); // 10ms timeout
            if poll_rc <= 0 {
                continue;
            }

            loop {
                let mut ev_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
                let rc = snd_seq_event_input(handle.as_ptr(), &mut ev_ptr);
                if rc < 0 || ev_ptr.is_null() {
                    break;
                }

                if let Some(bytes) = event_to_bytes(ev_ptr) {
                    if let Ok(mut q) = buf.lock() {
                        q.push_back(MidiMsg { timestamp: 0, data: bytes });
                        while q.len() > MIDI_BUF_CAP {
                            q.pop_front();
                        }
                    }
                }
            }
        }

        snd_seq_disconnect_from(handle.as_ptr(), port_id, src_client, src_port);
        snd_seq_delete_simple_port(handle.as_ptr(), port_id);
        // SeqHandle dropped here, closing the sequencer
    }
}

// ─── Backend ─────────────────────────────────────────────────────────────────

pub struct AlsaBackend;

impl MidiBackend for AlsaBackend {
    type Output = AlsaOutput;
    type Input = AlsaInput;

    fn sources(&self) -> Vec<MidiEndpointInfo> {
        let ports = unsafe { enumerate_ports(SND_SEQ_PORT_CAP_READ | SND_SEQ_PORT_CAP_SUBS_READ) };
        ports
            .iter()
            .enumerate()
            .map(|(i, p)| MidiEndpointInfo { name: p.display_name(), index: i + 1 })
            .collect()
    }

    fn destinations(&self) -> Vec<MidiEndpointInfo> {
        let ports = unsafe { enumerate_ports(SND_SEQ_PORT_CAP_WRITE | SND_SEQ_PORT_CAP_SUBS_WRITE) };
        ports
            .iter()
            .enumerate()
            .map(|(i, p)| MidiEndpointInfo { name: p.display_name(), index: i + 1 })
            .collect()
    }

    fn open_output(&self, sel: EndpointSelector) -> Result<AlsaOutput, String> {
        let ports = unsafe { enumerate_ports(SND_SEQ_PORT_CAP_WRITE | SND_SEQ_PORT_CAP_SUBS_WRITE) };
        let dest = resolve_port(&ports, &sel).ok_or("MIDI destination not found")?;

        unsafe {
            let mut handle: *mut std::ffi::c_void = std::ptr::null_mut();
            let rc = snd_seq_open(&mut handle, c"default".as_ptr(), SND_SEQ_OPEN_OUTPUT, 0);
            if rc < 0 {
                return Err(format!("snd_seq_open: {}", alsa_error(rc)));
            }
            snd_seq_set_client_name(handle, c"ferrigno-midi".as_ptr());

            let port_id = snd_seq_create_simple_port(
                handle,
                c"ferrigno-output".as_ptr(),
                SND_SEQ_PORT_CAP_READ,
                SND_SEQ_PORT_TYPE_MIDI_GENERIC | SND_SEQ_PORT_TYPE_APPLICATION,
            );
            if port_id < 0 {
                snd_seq_close(handle);
                return Err(format!("snd_seq_create_simple_port: {}", alsa_error(port_id)));
            }

            let rc = snd_seq_connect_to(handle, port_id, dest.client_id, dest.port_id);
            if rc < 0 {
                snd_seq_delete_simple_port(handle, port_id);
                snd_seq_close(handle);
                return Err(format!("snd_seq_connect_to: {}", alsa_error(rc)));
            }

            Ok(AlsaOutput {
                handle: SeqHandle(handle),
                port_id,
                dest_client: dest.client_id,
                dest_port: dest.port_id,
                closed: false,
            })
        }
    }

    fn open_input(&self, sel: EndpointSelector) -> Result<AlsaInput, String> {
        let ports = unsafe { enumerate_ports(SND_SEQ_PORT_CAP_READ | SND_SEQ_PORT_CAP_SUBS_READ) };
        let src = resolve_port(&ports, &sel).ok_or("MIDI source not found")?;

        unsafe {
            let mut handle: *mut std::ffi::c_void = std::ptr::null_mut();
            let rc = snd_seq_open(&mut handle, c"default".as_ptr(), SND_SEQ_OPEN_INPUT, 0);
            if rc < 0 {
                return Err(format!("snd_seq_open: {}", alsa_error(rc)));
            }
            snd_seq_set_client_name(handle, c"ferrigno-midi".as_ptr());

            let port_id = snd_seq_create_simple_port(
                handle,
                c"ferrigno-input".as_ptr(),
                SND_SEQ_PORT_CAP_WRITE | SND_SEQ_PORT_CAP_SUBS_WRITE,
                SND_SEQ_PORT_TYPE_MIDI_GENERIC | SND_SEQ_PORT_TYPE_APPLICATION,
            );
            if port_id < 0 {
                snd_seq_close(handle);
                return Err(format!("snd_seq_create_simple_port: {}", alsa_error(port_id)));
            }

            let rc = snd_seq_connect_from(handle, port_id, src.client_id, src.port_id);
            if rc < 0 {
                snd_seq_delete_simple_port(handle, port_id);
                snd_seq_close(handle);
                return Err(format!("snd_seq_connect_from: {}", alsa_error(rc)));
            }

            let buf: SharedBuf = Arc::new(Mutex::new(VecDeque::new()));
            let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

            // Spawn reader thread with its own seq handle
            let thread_buf = buf.clone();
            let thread_stop = stop.clone();
            let thread_src_client = src.client_id;
            let thread_src_port = src.port_id;

            // Transfer the handle to the reader thread — open a new one for the Input struct
            // Actually, ALSA seq handles can't be shared; the thread owns it.
            let thread_handle = SeqHandle(handle);

            let thread = std::thread::spawn(move || {
                input_reader_thread(
                    thread_handle, port_id, thread_src_client, thread_src_port, thread_buf, thread_stop,
                );
            });

            Ok(AlsaInput {
                buf,
                handle: SeqHandle(std::ptr::null_mut()), // thread owns the handle
                port_id,
                src_client: src.client_id,
                src_port: src.port_id,
                closed: false,
                thread: Some(thread),
                stop,
            })
        }
    }
}

use super::backend::*;
use std::collections::VecDeque;
use std::ptr::*;
use std::sync::{Arc, Mutex};

// ─── CoreMIDI FFI ────────────────────────────────────────────────────────────

type MidiObjectRef = u32;
type MidiClientRef = MidiObjectRef;
type MidiPortRef = MidiObjectRef;
type MidiEndpointRef = MidiObjectRef;
type OsStatus = i32;
type MidiTimeStamp = u64;
type ItemCount = u64;
type ByteCount = u64;

#[link(name = "CoreMIDI", kind = "framework")]
unsafe extern "C" {
    fn MIDIClientCreate(
        name: *const std::ffi::c_void,
        notify_proc: *mut std::ffi::c_void,
        notify_ref_con: *mut std::ffi::c_void,
        out_client: *mut MidiClientRef,
    ) -> OsStatus;

    fn MIDIOutputPortCreate(client: MidiClientRef, port_name: *const std::ffi::c_void, out_port: *mut MidiPortRef) -> OsStatus;

    fn MIDIInputPortCreate(
        client: MidiClientRef,
        port_name: *const std::ffi::c_void,
        read_proc: unsafe extern "C" fn(
            pktlist: *const std::ffi::c_void,
            read_proc_ref: *mut std::ffi::c_void,
            src_conn_ref: *mut std::ffi::c_void,
        ),
        ref_con: *mut std::ffi::c_void,
        out_port: *mut MidiPortRef,
    ) -> OsStatus;

    fn MIDIPortConnectSource(port: MidiPortRef, source: MidiEndpointRef, conn_ref_con: *mut std::ffi::c_void) -> OsStatus;

    fn MIDIPortDisconnectSource(port: MidiPortRef, source: MidiEndpointRef) -> OsStatus;

    fn MIDIPortDispose(port: MidiPortRef) -> OsStatus;

    fn MIDIGetNumberOfSources() -> ItemCount;
    fn MIDIGetSource(src_index0: ItemCount) -> MidiEndpointRef;

    fn MIDIGetNumberOfDestinations() -> ItemCount;
    fn MIDIGetDestination(dest_index0: ItemCount) -> MidiEndpointRef;

    fn MIDIObjectGetStringProperty(
        obj: MidiObjectRef,
        prop_id: *const std::ffi::c_void,
        str_out: *mut *mut std::ffi::c_void,
    ) -> OsStatus;

    fn MIDISend(port: MidiPortRef, dest: MidiEndpointRef, pktlist: *const std::ffi::c_void) -> OsStatus;

    fn MIDIPacketListInit(pktlist: *mut std::ffi::c_void) -> *mut std::ffi::c_void;

    fn MIDIPacketListAdd(
        pktlist: *mut std::ffi::c_void,
        list_size: ByteCount,
        cur_packet: *mut std::ffi::c_void,
        time: MidiTimeStamp,
        n_data: ByteCount,
        data: *const u8,
    ) -> *mut std::ffi::c_void;

    static kMIDIPropertyName: *const std::ffi::c_void;
}

// ─── CoreFoundation FFI ──────────────────────────────────────────────────────

const CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(
        alloc: *const std::ffi::c_void,
        c_str: *const std::ffi::c_char,
        encoding: u32,
    ) -> *mut std::ffi::c_void;

    fn CFStringGetCString(
        the_string: *const std::ffi::c_void,
        buffer: *mut std::ffi::c_char,
        buffer_size: i64,
        encoding: u32,
    ) -> i32;

    fn CFRelease(cf: *const std::ffi::c_void);
}

// ─── CFString helpers ────────────────────────────────────────────────────────

unsafe fn cf_str_from_bytes(s: &[u8]) -> *mut std::ffi::c_void {
    unsafe {
        let mut buf = s.to_vec();
        buf.push(0);
        CFStringCreateWithCString(
            null(),
            buf.as_ptr() as *const std::ffi::c_char,
            CF_STRING_ENCODING_UTF8,
        )
    }
}

unsafe fn cf_str_to_string(cfstr: *const std::ffi::c_void) -> String {
    unsafe {
        if cfstr.is_null() {
            return String::new();
        }
        let mut buf = vec![0u8; 512];
        let ok = CFStringGetCString(
            cfstr,
            buf.as_mut_ptr() as *mut std::ffi::c_char,
            512,
            CF_STRING_ENCODING_UTF8,
        );
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
        let mut cfstr: *mut std::ffi::c_void = null_mut();
        MIDIObjectGetStringProperty(endpoint, name_prop, &mut cfstr);
        if cfstr.is_null() {
            return format!("endpoint-{}", endpoint);
        }
        let name = cf_str_to_string(cfstr);
        CFRelease(cfstr);
        name
    }
}

// ─── Global MIDI client ──────────────────────────────────────────────────────

static MIDI_CLIENT: Mutex<MidiClientRef> = Mutex::new(0);

unsafe fn get_or_create_client() -> Result<MidiClientRef, String> {
    unsafe {
        let mut guard = MIDI_CLIENT.lock().map_err(|e| format!("lock: {}", e))?;
        if *guard != 0 {
            return Ok(*guard);
        }
        let mut client: MidiClientRef = 0;
        let name_cf = cf_str_from_bytes(b"ferrigno-midi");
        let rc = MIDIClientCreate(name_cf, null_mut(), null_mut(), &mut client);
        CFRelease(name_cf);
        if rc != 0 {
            return Err(format!("MIDIClientCreate failed: {}", rc));
        }
        *guard = client;
        Ok(client)
    }
}

// ─── CoreMIDI read callback ──────────────────────────────────────────────────

unsafe extern "C" fn midi_read_proc(
    pktlist: *const std::ffi::c_void,
    refcon: *mut std::ffi::c_void,
    _src_conn: *mut std::ffi::c_void,
) {
    unsafe {
        let arc = Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>);
        if let Ok(mut q) = arc.lock() {
            let num_pkts = std::ptr::read_unaligned(pktlist as *const u32);
            let mut pkt_ptr = (pktlist as *const u8).add(4);

            for _ in 0..num_pkts {
                let timestamp = std::ptr::read_unaligned(pkt_ptr as *const u64);
                let length = std::ptr::read_unaligned(pkt_ptr.add(8) as *const u16) as usize;
                let data_ptr = pkt_ptr.add(10);
                let safe_len = length.min(65535);
                let data = std::slice::from_raw_parts(data_ptr, safe_len).to_vec();

                q.push_back(MidiMsg { timestamp, data });
                while q.len() > MIDI_BUF_CAP {
                    q.pop_front();
                }

                let advance = (10 + safe_len + 3) & !3;
                pkt_ptr = pkt_ptr.add(advance);
            }
        }
        std::mem::forget(arc);
    }
}

// ─── Packet list builder ─────────────────────────────────────────────────────

const SEND_BUF: usize = 65536 + 64;

unsafe fn send_bytes(port: MidiPortRef, dest: MidiEndpointRef, data: &[u8]) -> OsStatus {
    unsafe {
        if data.is_empty() || data.len() > 65535 {
            return -1;
        }
        let mut buf = vec![0u8; SEND_BUF];
        let pktlist = buf.as_mut_ptr() as *mut std::ffi::c_void;
        let cur = MIDIPacketListInit(pktlist);
        let cur = MIDIPacketListAdd(
            pktlist,
            SEND_BUF as ByteCount,
            cur,
            0,
            data.len() as ByteCount,
            data.as_ptr(),
        );
        if cur.is_null() {
            return -2;
        }
        MIDISend(port, dest, pktlist)
    }
}

// ─── Output port ─────────────────────────────────────────────────────────────

pub struct CoreMidiOutput {
    port: MidiPortRef,
    endpoint: MidiEndpointRef,
    closed: bool,
}

impl MidiOutputPort for CoreMidiOutput {
    fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if self.closed {
            return Err("port closed".into());
        }
        let rc = unsafe { send_bytes(self.port, self.endpoint, data) };
        if rc != 0 {
            Err(format!("MIDISend failed: {}", rc))
        } else {
            Ok(())
        }
    }

    fn close(&mut self) {
        if !self.closed {
            unsafe {
                MIDIPortDispose(self.port);
            }
            self.closed = true;
        }
    }
}

impl Drop for CoreMidiOutput {
    fn drop(&mut self) {
        self.close();
    }
}

// ─── Input port ──────────────────────────────────────────────────────────────

pub struct CoreMidiInput {
    port: MidiPortRef,
    endpoint: MidiEndpointRef,
    buf: SharedBuf,
    refcon: *mut std::ffi::c_void,
    closed: bool,
}

// Safety: the raw pointer (refcon) points to an Arc-managed allocation that is
// only accessed behind a Mutex, so sending across threads is safe.
unsafe impl Send for CoreMidiInput {}

impl MidiInputPort for CoreMidiInput {
    fn buffer(&self) -> &SharedBuf {
        &self.buf
    }

    fn close(&mut self) {
        if !self.closed {
            unsafe {
                MIDIPortDisconnectSource(self.port, self.endpoint);
                MIDIPortDispose(self.port);
                drop(Arc::from_raw(
                    self.refcon as *const Mutex<VecDeque<MidiMsg>>,
                ));
            }
            self.closed = true;
        }
    }
}

impl Drop for CoreMidiInput {
    fn drop(&mut self) {
        self.close();
    }
}

// ─── Backend ─────────────────────────────────────────────────────────────────

pub struct CoreMidiBackend;

impl CoreMidiBackend {
    unsafe fn resolve_destination(sel: &EndpointSelector) -> Option<MidiEndpointRef> {
        unsafe {
            let n = MIDIGetNumberOfDestinations();
            match sel {
                EndpointSelector::Index(idx) => {
                    if *idx < 1 || *idx as u64 > n {
                        return None;
                    }
                    Some(MIDIGetDestination((*idx - 1) as ItemCount))
                }
                EndpointSelector::Name(want) => {
                    for i in 0..n {
                        let ep = MIDIGetDestination(i);
                        if endpoint_name(ep) == *want {
                            return Some(ep);
                        }
                    }
                    None
                }
            }
        }
    }

    unsafe fn resolve_source(sel: &EndpointSelector) -> Option<MidiEndpointRef> {
        unsafe {
            let n = MIDIGetNumberOfSources();
            match sel {
                EndpointSelector::Index(idx) => {
                    if *idx < 1 || *idx as u64 > n {
                        return None;
                    }
                    Some(MIDIGetSource((*idx - 1) as ItemCount))
                }
                EndpointSelector::Name(want) => {
                    for i in 0..n {
                        let ep = MIDIGetSource(i);
                        if endpoint_name(ep) == *want {
                            return Some(ep);
                        }
                    }
                    None
                }
            }
        }
    }
}

impl MidiBackend for CoreMidiBackend {
    type Output = CoreMidiOutput;
    type Input = CoreMidiInput;

    fn sources(&self) -> Vec<MidiEndpointInfo> {
        unsafe {
            let n = MIDIGetNumberOfSources();
            (0..n)
                .map(|i| MidiEndpointInfo {
                    name: endpoint_name(MIDIGetSource(i)),
                    index: (i + 1) as usize,
                })
                .collect()
        }
    }

    fn destinations(&self) -> Vec<MidiEndpointInfo> {
        unsafe {
            let n = MIDIGetNumberOfDestinations();
            (0..n)
                .map(|i| MidiEndpointInfo {
                    name: endpoint_name(MIDIGetDestination(i)),
                    index: (i + 1) as usize,
                })
                .collect()
        }
    }

    fn open_output(&self, sel: EndpointSelector) -> Result<CoreMidiOutput, String> {
        unsafe {
            let client = get_or_create_client()?;
            let endpoint = Self::resolve_destination(&sel).ok_or("MIDI destination not found")?;

            let port_name_cf = cf_str_from_bytes(b"ferrigno-output");
            let mut port: MidiPortRef = 0;
            let rc = MIDIOutputPortCreate(client, port_name_cf, &mut port);
            CFRelease(port_name_cf);

            if rc != 0 {
                return Err(format!("MIDIOutputPortCreate failed: {}", rc));
            }
            Ok(CoreMidiOutput {
                port,
                endpoint,
                closed: false,
            })
        }
    }

    fn open_input(&self, sel: EndpointSelector) -> Result<CoreMidiInput, String> {
        unsafe {
            let client = get_or_create_client()?;
            let endpoint = Self::resolve_source(&sel).ok_or("MIDI source not found")?;

            let buf: SharedBuf = Arc::new(Mutex::new(VecDeque::new()));
            let callback_arc = buf.clone();
            let refcon = Arc::into_raw(callback_arc) as *mut std::ffi::c_void;

            let port_name_cf = cf_str_from_bytes(b"ferrigno-input");
            let mut port: MidiPortRef = 0;
            let rc = MIDIInputPortCreate(client, port_name_cf, midi_read_proc, refcon, &mut port);
            CFRelease(port_name_cf);

            if rc != 0 {
                drop(Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>));
                return Err(format!("MIDIInputPortCreate failed: {}", rc));
            }

            let rc2 = MIDIPortConnectSource(port, endpoint, null_mut());
            if rc2 != 0 {
                MIDIPortDispose(port);
                drop(Arc::from_raw(refcon as *const Mutex<VecDeque<MidiMsg>>));
                return Err(format!("MIDIPortConnectSource failed: {}", rc2));
            }

            Ok(CoreMidiInput {
                port,
                endpoint,
                buf,
                refcon,
                closed: false,
            })
        }
    }
}

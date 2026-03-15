use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// ─── Platform-independent MIDI message ───────────────────────────────────────

pub struct MidiMsg {
    pub timestamp: u64,
    pub data: Vec<u8>,
}

pub type SharedBuf = Arc<Mutex<VecDeque<MidiMsg>>>;

pub const MIDI_BUF_CAP: usize = 4096;

// ─── Endpoint descriptor ─────────────────────────────────────────────────────

pub struct MidiEndpointInfo {
    pub name: String,
    pub index: usize, // 1-based
}

// ─── Backend trait ───────────────────────────────────────────────────────────

pub trait MidiOutputPort: Send {
    fn send(&mut self, data: &[u8]) -> Result<(), String>;
    fn close(&mut self);
}

pub trait MidiInputPort: Send {
    fn buffer(&self) -> &SharedBuf;
    fn close(&mut self);
}

pub trait MidiBackend {
    type Output: MidiOutputPort;
    type Input: MidiInputPort;

    fn sources(&self) -> Vec<MidiEndpointInfo>;
    fn destinations(&self) -> Vec<MidiEndpointInfo>;
    fn open_output(&self, index_or_name: EndpointSelector) -> Result<Self::Output, String>;
    fn open_input(&self, index_or_name: EndpointSelector) -> Result<Self::Input, String>;
}

pub enum EndpointSelector<'a> {
    Index(usize), // 1-based
    Name(&'a str),
}

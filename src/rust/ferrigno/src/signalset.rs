#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalSet {
    signalset_value: [usize; 16],
}
impl Default for SignalSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalSet {
    pub fn new() -> Self {
        SignalSet {
            signalset_value: [0; 16],
        }
    }
}

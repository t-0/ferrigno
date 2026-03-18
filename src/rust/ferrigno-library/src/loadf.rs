use crate::functionstate::LUAL_BUFFERSIZE;

pub enum LoadSource {
    File(std::fs::File),
    Stdin,
}

impl LoadSource {
    pub fn read_byte(&mut self) -> Option<u8> {
        use std::io::Read;
        let mut b = [0u8; 1];
        let n = match self {
            | LoadSource::File(f) => f.read(&mut b).unwrap_or(0),
            | LoadSource::Stdin => std::io::stdin().lock().read(&mut b).unwrap_or(0),
        };
        if n == 0 { None } else { Some(b[0]) }
    }

    pub fn read_chunk(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        use std::io::Read;
        match self {
            | LoadSource::File(f) => f.read(buf),
            | LoadSource::Stdin => std::io::stdin().lock().read(buf),
        }
    }
}

pub struct LoadF {
    pub loadf_n: i32,
    pub loadf_source: Option<LoadSource>,
    pub loadf_had_error: bool,
    pub loadf_buffer: [i8; LUAL_BUFFERSIZE],
}

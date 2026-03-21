use crate::functionstate::LUAL_BUFFERSIZE;
use crate::writebuffermode::*;
use std::fs::File;
use std::io::{Read, Write};

pub struct FileHandle {
    pub filehandle_file: File,
    pub filehandle_pushback: Option<u8>,
    pub filehandle_had_error: bool,
    pub filehandle_write_buffer: Vec<u8>,
    pub filehandle_write_buffer_mode: WriteBufferMode,
}

impl FileHandle {
    pub fn new(file: File) -> Self {
        FileHandle {
            filehandle_file: file,
            filehandle_pushback: None,
            filehandle_had_error: false,
            filehandle_write_buffer: Vec::new(),
            filehandle_write_buffer_mode: WriteBufferMode::No,
        }
    }
    pub fn new_buffered(file: File) -> Self {
        FileHandle {
            filehandle_file: file,
            filehandle_pushback: None,
            filehandle_had_error: false,
            filehandle_write_buffer: Vec::new(),
            filehandle_write_buffer_mode: WriteBufferMode::Full(LUAL_BUFFERSIZE),
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if let Some(b) = self.filehandle_pushback.take() {
            return Some(b);
        }
        let mut buf = [0u8; 1];
        match self.filehandle_file.read(&mut buf) {
            Ok(1) => Some(buf[0]),
            Ok(_) => None,
            Err(_) => {
                self.filehandle_had_error = true;
                None
            }
        }
    }

    pub fn unread_byte(&mut self, b: u8) {
        self.filehandle_pushback = Some(b);
    }

    /// Write data through the user-space write buffer.
    pub fn write_buffered(&mut self, data: &[u8]) -> bool {
        match &self.filehandle_write_buffer_mode {
            WriteBufferMode::No => self.filehandle_file.write_all(data).is_ok(),
            WriteBufferMode::Full(cap) => {
                let cap = *cap;
                self.filehandle_write_buffer.extend_from_slice(data);
                if self.filehandle_write_buffer.len() >= cap {
                    let buf = std::mem::take(&mut self.filehandle_write_buffer);
                    self.filehandle_file.write_all(&buf).is_ok()
                } else {
                    true
                }
            }
            WriteBufferMode::Line => {
                self.filehandle_write_buffer.extend_from_slice(data);
                if self.filehandle_write_buffer.contains(&b'\n') {
                    let buf = std::mem::take(&mut self.filehandle_write_buffer);
                    self.filehandle_file.write_all(&buf).is_ok()
                } else {
                    true
                }
            }
        }
    }

    /// Flush any pending write buffer to the underlying file.
    pub fn flush_write_buf(&mut self) -> bool {
        if self.filehandle_write_buffer.is_empty() {
            return true;
        }
        let buf = std::mem::take(&mut self.filehandle_write_buffer);
        self.filehandle_file.write_all(&buf).is_ok()
    }
}

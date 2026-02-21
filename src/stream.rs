use crate::functions::*;
use std::fs::File;
use std::io::{Read, Write};

pub enum WriteBufMode {
    No,
    Full(usize),
    Line,
}

pub struct FileHandle {
    pub file: File,
    pub pushback: Option<u8>,
    pub had_error: bool,
    pub write_buf: Vec<u8>,
    pub write_buf_mode: WriteBufMode,
}

impl FileHandle {
    pub fn new(file: File) -> Self {
        FileHandle {
            file,
            pushback: None,
            had_error: false,
            write_buf: Vec::new(),
            write_buf_mode: WriteBufMode::No,
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if let Some(b) = self.pushback.take() {
            return Some(b);
        }
        let mut buf = [0u8; 1];
        match self.file.read(&mut buf) {
            Ok(1) => Some(buf[0]),
            Ok(_) => None,
            Err(_) => {
                self.had_error = true;
                None
            }
        }
    }

    pub fn unread_byte(&mut self, b: u8) {
        self.pushback = Some(b);
    }

    /// Write data through the user-space write buffer.
    pub fn write_buffered(&mut self, data: &[u8]) -> bool {
        match &self.write_buf_mode {
            WriteBufMode::No => self.file.write_all(data).is_ok(),
            WriteBufMode::Full(cap) => {
                let cap = *cap;
                self.write_buf.extend_from_slice(data);
                if self.write_buf.len() >= cap {
                    let buf = std::mem::take(&mut self.write_buf);
                    self.file.write_all(&buf).is_ok()
                } else {
                    true
                }
            }
            WriteBufMode::Line => {
                self.write_buf.extend_from_slice(data);
                if self.write_buf.contains(&b'\n') {
                    let buf = std::mem::take(&mut self.write_buf);
                    self.file.write_all(&buf).is_ok()
                } else {
                    true
                }
            }
        }
    }

    /// Flush any pending write buffer to the underlying file.
    pub fn flush_write_buf(&mut self) -> bool {
        if self.write_buf.is_empty() {
            return true;
        }
        let buf = std::mem::take(&mut self.write_buf);
        self.file.write_all(&buf).is_ok()
    }
}

pub enum IoHandle {
    /// A regular file opened with io.open / io.tmpfile
    File(FileHandle),
    /// A pipe opened with io.popen (uses libc popen/pclose)
    Pipe(*mut libc::FILE),
    /// Standard input
    Stdin { pushback: Option<u8> },
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
}

impl IoHandle {
    pub fn read_byte(&mut self) -> Option<u8> {
        match self {
            IoHandle::File(fh) => fh.read_byte(),
            IoHandle::Pipe(f) => {
                let c = unsafe { libc::fgetc(*f) };
                if c == libc::EOF { None } else { Some(c as u8) }
            }
            IoHandle::Stdin { pushback } => {
                if let Some(b) = pushback.take() {
                    return Some(b);
                }
                let mut buf = [0u8; 1];
                match std::io::stdin().lock().read(&mut buf) {
                    Ok(1) => Some(buf[0]),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn unread_byte(&mut self, b: u8) {
        match self {
            IoHandle::File(fh) => fh.unread_byte(b),
            IoHandle::Pipe(f) => unsafe { libc::ungetc(b as i32, *f); }
            IoHandle::Stdin { pushback } => *pushback = Some(b),
            _ => {}
        }
    }

    pub fn had_error(&self) -> bool {
        match self {
            IoHandle::File(fh) => fh.had_error,
            IoHandle::Pipe(f) => unsafe { libc::ferror(*f) != 0 },
            _ => false,
        }
    }

    pub fn clear_error(&mut self) {
        match self {
            IoHandle::File(fh) => fh.had_error = false,
            IoHandle::Pipe(f) => unsafe { libc::clearerr(*f) },
            _ => {}
        }
    }

    /// Flush any pending write buffer (files only).
    pub fn flush(&mut self) -> bool {
        match self {
            IoHandle::File(fh) => fh.flush_write_buf(),
            IoHandle::Pipe(f) => unsafe { libc::fflush(*f) == 0 },
            IoHandle::Stdout => std::io::stdout().flush().is_ok(),
            IoHandle::Stderr => std::io::stderr().flush().is_ok(),
            _ => true,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub stream_handle: *mut IoHandle,
    pub stream_cfunctionclose: CFunction,
}

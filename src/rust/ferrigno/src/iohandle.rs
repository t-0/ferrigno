use crate::filehandle::*;
use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout};

pub struct PipeHandle {
    pub pipe_child: Child,
    pub pipe_reader: Option<ChildStdout>,
    pub pipe_writer: Option<ChildStdin>,
    pub pipe_pushback: Option<u8>,
    pub pipe_had_error: bool,
}

impl PipeHandle {
    pub fn new_read(mut child: Child) -> Self {
        let reader = child.stdout.take();
        PipeHandle {
            pipe_child: child,
            pipe_reader: reader,
            pipe_writer: None,
            pipe_pushback: None,
            pipe_had_error: false,
        }
    }

    pub fn new_write(mut child: Child) -> Self {
        let writer = child.stdin.take();
        PipeHandle {
            pipe_child: child,
            pipe_reader: None,
            pipe_writer: writer,
            pipe_pushback: None,
            pipe_had_error: false,
        }
    }
}

pub enum IoHandle {
    /// A regular file opened with io.open / io.tmpfile
    File(FileHandle),
    /// A pipe opened with io.popen
    Pipe(PipeHandle),
    /// Standard input
    Stdin { filehandle_pushback: Option<u8> },
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
}

impl IoHandle {
    pub fn read_byte(&mut self) -> Option<u8> {
        match self {
            IoHandle::File(fh) => fh.read_byte(),
            IoHandle::Pipe(ph) => {
                if let Some(b) = ph.pipe_pushback.take() {
                    return Some(b);
                }
                if let Some(ref mut reader) = ph.pipe_reader {
                    let mut buf = [0u8; 1];
                    match reader.read(&mut buf) {
                        Ok(1) => Some(buf[0]),
                        Ok(_) => None,
                        Err(_) => {
                            ph.pipe_had_error = true;
                            None
                        }
                    }
                } else {
                    None
                }
            }
            IoHandle::Stdin {
                filehandle_pushback,
            } => {
                if let Some(b) = filehandle_pushback.take() {
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
            IoHandle::Pipe(ph) => ph.pipe_pushback = Some(b),
            IoHandle::Stdin {
                filehandle_pushback,
            } => *filehandle_pushback = Some(b),
            _ => {}
        }
    }

    pub fn filehandle_had_error(&self) -> bool {
        match self {
            IoHandle::File(fh) => fh.filehandle_had_error,
            IoHandle::Pipe(ph) => ph.pipe_had_error,
            _ => false,
        }
    }

    pub fn clear_error(&mut self) {
        match self {
            IoHandle::File(fh) => fh.filehandle_had_error = false,
            IoHandle::Pipe(ph) => ph.pipe_had_error = false,
            _ => {}
        }
    }

    /// Flush any pending write buffer.
    pub fn flush(&mut self) -> bool {
        match self {
            IoHandle::File(fh) => fh.flush_write_buf(),
            IoHandle::Pipe(ph) => {
                if let Some(ref mut writer) = ph.pipe_writer {
                    writer.flush().is_ok()
                } else {
                    true
                }
            }
            IoHandle::Stdout => std::io::stdout().flush().is_ok(),
            IoHandle::Stderr => std::io::stderr().flush().is_ok(),
            _ => true,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    OK = 0,
    Yield = 1,
    RuntimeError = 2,
    SyntaxError = 3,
    MemoryError = 4,
    GenericError = 5,
    FileError = 6,
    Closing = 7,
    Unknown = 8,
}
impl Status {
    pub fn from(input: i32) -> Self {
        match input {
            | 0 => Status::OK,
            | 1 => Status::Yield,
            | 2 => Status::RuntimeError,
            | 3 => Status::SyntaxError,
            | 4 => Status::MemoryError,
            | 5 => Status::GenericError,
            | 6 => Status::FileError,
            | 7 => Status::Closing,
            | _ => Status::Unknown,
        }
    }
    pub fn is_error(&self) -> bool {
        match *self {
            | Status::OK | Status::Yield | Status::Closing => false,
            | Status::RuntimeError | Status::SyntaxError | Status::MemoryError | Status::GenericError | Status::FileError | _ => {
                true
            },
        }
    }
}

#[derive(Copy,Clone,PartialEq, PartialOrd, Eq, Ord)]
#[repr(i32)]
pub enum Status {
    OK = 0,
    Yield = 1,
    RuntimeError = 2,
    SyntaxError = 3,
    MemoryError = 4,
    GenericError = 5,
    FileError = 6,
    Closing = -1,
}
impl Status {
    pub fn from (input: i32) -> Self {
        match input {
            0 => Status::OK,
            1 => Status::Yield,
            2 => Status::RuntimeError,
            3 => Status::SyntaxError,
            4 => Status::MemoryError,
            5 => Status::GenericError,
            6 => Status::FileError,
            -1 => Status::Closing,
            _ => Status::Closing,
        }
    }
}

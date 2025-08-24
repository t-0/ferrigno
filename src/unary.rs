#[derive(Copy, Clone)]
#[repr(C)]
pub enum Unary {
    Minus,
    BitwiseNot,
    Not,
    Length,
    None_,
}

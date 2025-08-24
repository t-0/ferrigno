use crate::value::*;
#[derive(Copy, Clone)]
pub struct TValue {
    pub value: Value,
    pub tag: u8,
}

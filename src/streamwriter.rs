#![allow (
    non_snake_case,
)]
use crate::buffer::*;
#[derive(Copy, Clone)]
pub struct StreamWriter {
    pub init: i32,
    pub B: Buffer,
}

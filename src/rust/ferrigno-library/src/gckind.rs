#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum GCKind {
    Incremental = 0,
    GenerationalMinor = 1,
    GenerationalMajor = 2,
}

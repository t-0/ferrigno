#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum K {
    Integer,
    Unsigned,
    Float,
    Number,
    Double,
    Character,
    String,
    ZString,
    Padding,
    PaddingAlignment,
    NoOperator,
}

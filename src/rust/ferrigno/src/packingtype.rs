#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum PackingType {
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

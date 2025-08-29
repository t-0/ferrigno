#![allow(unused)]
enum TagType {
    Nil = 0x00,
    Boolean = 0x01,
    Pointer = 0x02,
    Numeric = 0x03,
    String = 0x04,
    Table = 0x05,
    Closure = 0x06,
    User = 0x07,
    State = 0x08,
    UpValue = 0x09,
    Prototype = 0x0A,
}
pub const TAG_TYPE_NIL: u8 = TagType::Nil as u8;
pub const TAG_VARIANT_NIL_NIL: u8 = TAG_TYPE_NIL | (0x00 << 0x04);
pub const TAG_VARIANT_NIL_EMPTY: u8 = TAG_TYPE_NIL | (0x01 << 0x04);
pub const TAG_VARIANT_NIL_ABSENTKEY: u8 = TAG_TYPE_NIL | (0x02 << 0x04);
pub const TAG_TYPE_BOOLEAN: u8 = TagType::Boolean as u8;
pub const TAG_VARIANT_BOOLEAN_FALSE: u8 = TAG_TYPE_BOOLEAN | (0x00 << 0x04);
pub const TAG_VARIANT_BOOLEAN_TRUE: u8 = TAG_TYPE_BOOLEAN | (0x01 << 0x04);
pub const TAG_TYPE_POINTER: u8 = TagType::Pointer as u8;
pub const TAG_VARIANT_POINTER: u8 = TAG_TYPE_POINTER;
pub const TAG_TYPE_NUMERIC: u8 = TagType::Numeric as u8;
pub const TAG_VARIANT_NUMERIC_INTEGER: u8 = TAG_TYPE_NUMERIC | (0x00 << 0x04);
pub const TAG_VARIANT_NUMERIC_NUMBER: u8 = TAG_TYPE_NUMERIC | (0x01 << 0x04);
pub const TAG_TYPE_STRING: u8 = TagType::String as u8;
pub const TAG_VARIANT_STRING_SHORT: u8 = TAG_TYPE_STRING | (0x00 << 0x04);
pub const TAG_VARIANT_STRING_LONG: u8 = TAG_TYPE_STRING | (0x01 << 0x04);
pub const TAG_TYPE_TABLE: u8 = TagType::Table as u8;
pub const TAG_VARIANT_TABLE: u8 = TAG_TYPE_TABLE;
pub const TAG_TYPE_CLOSURE: u8 = TagType::Closure as u8;
pub const TAG_VARIANT_CLOSURE_L: u8 = TAG_TYPE_CLOSURE | (0x00 << 0x04);
pub const TAG_VARIANT_CLOSURE_CFUNCTION: u8 = TAG_TYPE_CLOSURE | (0x01 << 0x04);
pub const TAG_VARIANT_CLOSURE_C: u8 = TAG_TYPE_CLOSURE | (0x02 << 0x04);
pub const TAG_TYPE_USER: u8 = TagType::User as u8;
pub const TAG_VARIANT_USER: u8 = TAG_TYPE_USER;
pub const TAG_TYPE_STATE: u8 = TagType::State as u8;
pub const TAG_VARIANT_STATE: u8 = TAG_TYPE_STATE;
pub const TAG_TYPE_UPVALUE: u8 = TagType::UpValue as u8;
pub const TAG_VARIANT_UPVALUE: u8 = TAG_TYPE_UPVALUE;
pub const TAG_TYPE_PROTOTYPE: u8 = TagType::Prototype as u8;
pub const TAG_VARIANT_PROTOTYPE: u8 = TAG_TYPE_PROTOTYPE;
const TAG_TYPE_MASK_: u8 = 0x0F;
const TAG_VARIANT_MASK_: u8 = 0x3F;
pub const fn get_tag_type(tag: u8) -> u8 {
    TAG_TYPE_MASK_ & tag
}
pub const fn get_tag_variant(tag: u8) -> u8 {
    TAG_VARIANT_MASK_ & tag
}
const TAG_COLLECTABLE: u8 = 0x40;
pub fn is_none_or_nil(tag: Option<u8>) -> bool {
    match tag {
        None | Some(TAG_TYPE_NIL) => true,
        _ => false,
    }
}
pub const fn set_collectable(tag: u8) -> u8 {
    tag | TAG_COLLECTABLE
}
pub const fn is_collectable(tag: u8) -> bool {
    0 != (TAG_COLLECTABLE & tag)
}

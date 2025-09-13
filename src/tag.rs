use rlua::*;
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TagType {
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
    DeadKey = 0x0B,
}
pub const TAGTYPE_SIMPLE_: [TagType; 9] = [
    TagType::Nil,
    TagType::Boolean,
    TagType::Pointer,
    TagType::Numeric,
    TagType::String,
    TagType::Table,
    TagType::Closure,
    TagType::User,
    TagType::State,
];
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TagVariantRaw {
    Alpha = 0x00 << 0x04,
    Beta = 0x01 << 0x04,
    Gamma = 0x02 << 0x04,
}
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TagVariant {
    NilNil = TagType::Nil as u8 | TagVariantRaw::Alpha as u8,
    NilEmpty = TagType::Nil as u8 | TagVariantRaw::Beta as u8,
    NilAbsentKey = TagType::Nil as u8 | TagVariantRaw::Gamma as u8,
    BooleanFalse = TagType::Boolean as u8 | TagVariantRaw::Alpha as u8,
    BooleanTrue = TagType::Boolean as u8 | TagVariantRaw::Beta as u8,
    Pointer = TagType::Pointer as u8 | TagVariantRaw::Alpha as u8,
    NumericInteger = TagType::Numeric as u8 | TagVariantRaw::Alpha as u8,
    NumericNumber = TagType::Numeric as u8 | TagVariantRaw::Beta as u8,
    StringShort = TagType::String as u8 | TagVariantRaw::Alpha as u8,
    StringLong = TagType::String as u8 | TagVariantRaw::Beta as u8,
    Table = TagType::Table as u8 | TagVariantRaw::Alpha as u8,
    ClosureL = TagType::Closure as u8 | TagVariantRaw::Alpha as u8,
    ClosureCFunction = TagType::Closure as u8 | TagVariantRaw::Beta as u8,
    ClosureC = TagType::Closure as u8 | TagVariantRaw::Gamma as u8,
    User = TagType::User as u8 | TagVariantRaw::Alpha as u8,
    State = TagType::State as u8 | TagVariantRaw::Alpha as u8,
    UpValue = TagType::UpValue as u8 | TagVariantRaw::Alpha as u8,
    Prototype = TagType::Prototype as u8 | TagVariantRaw::Alpha as u8,
    DeadKey = TagType::DeadKey as u8 | TagVariantRaw::Alpha as u8,
}
pub const TAG_VARIANT_NIL_NIL: u8 = TagVariant::NilNil as u8;
pub const TAG_VARIANT_NIL_EMPTY: u8 = TagVariant::NilEmpty as u8;
pub const TAG_VARIANT_NIL_ABSENTKEY: u8 = TagVariant::NilAbsentKey as u8;
pub const TAG_VARIANT_BOOLEAN_FALSE: u8 = TagVariant::BooleanFalse as u8;
pub const TAG_VARIANT_BOOLEAN_TRUE: u8 = TagVariant::BooleanTrue as u8;
pub const TAG_VARIANT_POINTER: u8 = TagVariant::Pointer as u8;
pub const TAG_VARIANT_NUMERIC_INTEGER: u8 = TagVariant::NumericInteger as u8;
pub const TAG_VARIANT_NUMERIC_NUMBER: u8 = TagVariant::NumericNumber as u8;
pub const TAG_VARIANT_STRING_SHORT: u8 = TagVariant::StringShort as u8;
pub const TAG_VARIANT_STRING_LONG: u8 = TagVariant::StringLong as u8;
pub const TAG_VARIANT_TABLE: u8 = TagVariant::Table as u8;
pub const TAG_VARIANT_CLOSURE_L: u8 = TagVariant::ClosureL as u8;
pub const TAG_VARIANT_CLOSURE_CFUNCTION: u8 = TagVariant::ClosureCFunction as u8;
pub const TAG_VARIANT_CLOSURE_C: u8 = TagVariant::ClosureC as u8;
pub const TAG_VARIANT_USER: u8 = TagVariant::User as u8;
pub const TAG_VARIANT_STATE: u8 = TagVariant::State as u8;
pub const TAG_VARIANT_UPVALUE: u8 = TagVariant::UpValue as u8;
pub const TAG_VARIANT_PROTOTYPE: u8 = TagVariant::Prototype as u8;
pub const TAG_VARIANT_DEADKEY: u8 = TagVariant::DeadKey as u8;
const TAG_TYPE_MASK_: u8 = 0x0F;
const TAG_VARIANT_MASK_: u8 = 0x3F;
pub const fn get_tag_type(tag: u8) -> TagType {
    match TAG_TYPE_MASK_ & tag {
        0 => TagType::Nil,
        1 => TagType::Boolean,
        2 => TagType::Pointer,
        3 => TagType::Numeric,
        4 => TagType::String,
        5 => TagType::Table,
        6 => TagType::Closure,
        7 => TagType::User,
        8 => TagType::State,
        9 => TagType::UpValue,
        10 => TagType::Prototype,
        11 => TagType::DeadKey,
        _ => TagType::Nil,
    }
}
pub const fn get_tag_variant(tag: u8) -> u8 {
    TAG_VARIANT_MASK_ & tag
}
pub const TAG_COLLECTABLE: u8 = 0x40;
pub fn is_none_or_nil(tag: Option<TagType>) -> bool {
    match tag {
        None | Some(TagType::Nil) => true,
        _ => false,
    }
}
pub const fn set_collectable(tag: u8) -> u8 {
    tag | TAG_COLLECTABLE
}
pub const STRING_LOCAL: *const i8 = make_cstring!("local");
pub const STRING_UPVALUE: *const i8 = make_cstring!("upvalue");
pub const TYPE_NAMES: [*const i8; 12] = [
    make_cstring!("no value"),
    make_cstring!("nil"),
    make_cstring!("boolean"),
    make_cstring!("userdata"),
    make_cstring!("number"),
    make_cstring!("string"),
    make_cstring!("table"),
    make_cstring!("function"),
    make_cstring!("userdata"),
    make_cstring!("thread"),
    make_cstring!("upvalue"),
    make_cstring!("proto"),
];

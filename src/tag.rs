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
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TagVariantRaw {
    Alpha = 0x00 << 0x04,
    Beta = 0x01 << 0x04,
    Gamma = 0x02 << 0x04,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
impl TagVariant {
    pub const fn from (value: u8) -> Self {
        if value == TagType::Nil as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::NilNil;
        } else if value == TagType::Nil as u8 | TagVariantRaw::Beta as u8 {
            return TagVariant::NilEmpty;
        } else if value == TagType::Nil as u8 | TagVariantRaw::Gamma as u8 {
            return TagVariant::NilAbsentKey;
        } else if value == TagType::Boolean as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::BooleanFalse;
        } else if value == TagType::Boolean as u8 | TagVariantRaw::Beta as u8 {
            return TagVariant::BooleanTrue;
        } else if value == TagType::Pointer as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::Pointer;
        } else if value == TagType::Numeric as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::NumericInteger;
        } else if value == TagType::Numeric as u8 | TagVariantRaw::Beta as u8 {
            return TagVariant::NumericNumber;
        } else if value == TagType::String as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::StringShort;
        } else if value == TagType::String as u8 | TagVariantRaw::Beta as u8 {
            return TagVariant::StringLong;
        } else if value == TagType::Table as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::Table;
        } else if value == TagType::Closure as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::ClosureL;
        } else if value == TagType::Closure as u8 | TagVariantRaw::Beta as u8 {
            return TagVariant::ClosureCFunction;
        } else if value == TagType::Closure as u8 | TagVariantRaw::Gamma as u8 {
            return TagVariant::ClosureC;
        } else if value == TagType::User as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::User;
        } else if value == TagType::State as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::State;
        } else if value == TagType::UpValue as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::UpValue;
        } else if value == TagType::Prototype as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::Prototype;
        } else if value == TagType::DeadKey as u8 | TagVariantRaw::Alpha as u8 {
            return TagVariant::DeadKey;
        } else {
            return TagVariant::DeadKey;
        }
    }
}
pub const fn get_tag_type(tagvariant: TagVariant) -> TagType {
    match tagvariant {
        TagVariant::NilNil => TagType::Nil,
        TagVariant::NilEmpty => TagType::Nil,
        TagVariant::NilAbsentKey => TagType::Nil,
        TagVariant::BooleanFalse => TagType::Boolean,
        TagVariant::BooleanTrue => TagType::Boolean,
        TagVariant::Pointer => TagType::Pointer,
        TagVariant::NumericInteger => TagType::Numeric,
        TagVariant::NumericNumber => TagType::Numeric,
        TagVariant::StringShort => TagType::String,
        TagVariant::StringLong => TagType::String,
        TagVariant::Table => TagType::Table,
        TagVariant::ClosureL => TagType::Closure,
        TagVariant::ClosureCFunction => TagType::Closure,
        TagVariant::ClosureC => TagType::Closure,
        TagVariant::User => TagType::User,
        TagVariant::State => TagType::State,
        TagVariant::UpValue => TagType::UpValue,
        TagVariant::Prototype => TagType::Prototype,
        TagVariant::DeadKey => TagType::DeadKey,
    }
}
pub fn is_none_or_nil(tagtype: Option<TagType>) -> bool {
    match tagtype {
        None | Some(TagType::Nil) => true,
        _ => false,
    }
}
pub const STRING_LOCAL: *const i8 = c"local".as_ptr();
pub const STRING_UPVALUE: *const i8 = c"upvalue".as_ptr();
pub const TYPE_NAMES: [*const i8; 12] = [
    c"no value".as_ptr(),
    c"nil".as_ptr(),
    c"boolean".as_ptr(),
    c"userdata".as_ptr(),
    c"number".as_ptr(),
    c"string".as_ptr(),
    c"table".as_ptr(),
    c"function".as_ptr(),
    c"userdata".as_ptr(),
    c"thread".as_ptr(),
    c"upvalue".as_ptr(),
    c"proto".as_ptr(),
];

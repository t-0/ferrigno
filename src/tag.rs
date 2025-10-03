#![allow(unused)]
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
    Interpreter = 0x08,
    UpValue = 0x09,
    Prototype = 0x0A,
    DeadKey = 0x0B,
}
impl TagType {
    pub fn is_none_or_nil(tagtype: Option<TagType>) -> bool {
        match tagtype {
            | None | Some(TagType::Nil) => true,
            | _ => false,
        }
    }
    pub fn is_nil(&self) -> bool {
        *self == TagType::Nil
    }
    pub fn is_string(&self) -> bool {
        *self == TagType::String
    }
    pub fn is_numeric(&self) -> bool {
        *self == TagType::Numeric
    }
    pub fn is_boolean(&self) -> bool {
        *self == TagType::Boolean
    }
    pub fn is_closure(&self) -> bool {
        *self == TagType::Closure
    }
    pub fn is_user(&self) -> bool {
        *self == TagType::User
    }
    pub fn is_table(&self) -> bool {
        *self == TagType::Table
    }
    pub fn is_pointer(&self) -> bool {
        *self == TagType::Pointer
    }
    pub fn is_interpreter(&self) -> bool {
        *self == TagType::Interpreter
    }
    pub fn is_upvalue(&self) -> bool {
        *self == TagType::UpValue
    }
    pub fn is_prototype(&self) -> bool {
        *self == TagType::Prototype
    }
    pub fn is_deadkey(&self) -> bool {
        *self == TagType::DeadKey
    }
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
    Interpreter = TagType::Interpreter as u8 | TagVariantRaw::Alpha as u8,
    UpValue = TagType::UpValue as u8 | TagVariantRaw::Alpha as u8,
    Prototype = TagType::Prototype as u8 | TagVariantRaw::Alpha as u8,
    DeadKey = TagType::DeadKey as u8 | TagVariantRaw::Alpha as u8,
}
impl TagVariant {
    pub const fn from(value: u8) -> Self {
        if value == TagType::Nil as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::NilNil
        } else if value == TagType::Nil as u8 | TagVariantRaw::Beta as u8 {
            TagVariant::NilEmpty
        } else if value == TagType::Nil as u8 | TagVariantRaw::Gamma as u8 {
            TagVariant::NilAbsentKey
        } else if value == TagType::Boolean as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::BooleanFalse
        } else if value == TagType::Boolean as u8 | TagVariantRaw::Beta as u8 {
            TagVariant::BooleanTrue
        } else if value == TagType::Pointer as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::Pointer
        } else if value == TagType::Numeric as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::NumericInteger
        } else if value == TagType::Numeric as u8 | TagVariantRaw::Beta as u8 {
            TagVariant::NumericNumber
        } else if value == TagType::String as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::StringShort
        } else if value == TagType::String as u8 | TagVariantRaw::Beta as u8 {
            TagVariant::StringLong
        } else if value == TagType::Table as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::Table
        } else if value == TagType::Closure as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::ClosureL
        } else if value == TagType::Closure as u8 | TagVariantRaw::Beta as u8 {
            TagVariant::ClosureCFunction
        } else if value == TagType::Closure as u8 | TagVariantRaw::Gamma as u8 {
            TagVariant::ClosureC
        } else if value == TagType::User as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::User
        } else if value == TagType::Interpreter as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::Interpreter
        } else if value == TagType::UpValue as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::UpValue
        } else if value == TagType::Prototype as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::Prototype
        } else if value == TagType::DeadKey as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::DeadKey
        } else {
            TagVariant::DeadKey
        }
    }
    pub const fn to_tag_type(&self) -> TagType {
        match self {
            | TagVariant::NilNil => TagType::Nil,
            | TagVariant::NilEmpty => TagType::Nil,
            | TagVariant::NilAbsentKey => TagType::Nil,
            | TagVariant::BooleanFalse => TagType::Boolean,
            | TagVariant::BooleanTrue => TagType::Boolean,
            | TagVariant::Pointer => TagType::Pointer,
            | TagVariant::NumericInteger => TagType::Numeric,
            | TagVariant::NumericNumber => TagType::Numeric,
            | TagVariant::StringShort => TagType::String,
            | TagVariant::StringLong => TagType::String,
            | TagVariant::Table => TagType::Table,
            | TagVariant::ClosureL => TagType::Closure,
            | TagVariant::ClosureCFunction => TagType::Closure,
            | TagVariant::ClosureC => TagType::Closure,
            | TagVariant::User => TagType::User,
            | TagVariant::Interpreter => TagType::Interpreter,
            | TagVariant::UpValue => TagType::UpValue,
            | TagVariant::Prototype => TagType::Prototype,
            | TagVariant::DeadKey => TagType::DeadKey,
        }
    }
}
pub const STRING_LOCAL: *const i8 = c"local".as_ptr();
pub const STRING_UPVALUE: *const i8 = c"upvalue".as_ptr();
pub const STRING_USERDATA: *const i8 = c"userdata".as_ptr();
pub const TYPE_NAMES: [*const i8; 12] = [
    c"no value".as_ptr(),
    c"nil".as_ptr(),
    c"boolean".as_ptr(),
    STRING_USERDATA,
    c"number".as_ptr(),
    c"string".as_ptr(),
    c"table".as_ptr(),
    c"function".as_ptr(),
    STRING_USERDATA,
    c"thread".as_ptr(),
    STRING_UPVALUE,
    c"proto".as_ptr(),
];

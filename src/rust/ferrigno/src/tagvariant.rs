#![allow(unused)]
use crate::tagtype::*;
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum TagVariantRaw {
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
        } else if value == TagType::State as u8 | TagVariantRaw::Alpha as u8 {
            TagVariant::State
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
        match *self as u8 & 0x0F {
            0x00 => TagType::Nil,
            0x01 => TagType::Boolean,
            0x02 => TagType::Pointer,
            0x03 => TagType::Numeric,
            0x04 => TagType::String,
            0x05 => TagType::Table,
            0x06 => TagType::Closure,
            0x07 => TagType::User,
            0x08 => TagType::State,
            0x09 => TagType::UpValue,
            0x0A => TagType::Prototype,
            _ => TagType::DeadKey,
        }
    }
}

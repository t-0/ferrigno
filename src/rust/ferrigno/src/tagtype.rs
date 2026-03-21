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
    State = 0x08,
    UpValue = 0x09,
    Prototype = 0x0A,
    DeadKey = 0x0B,
}
impl TagType {
    pub fn is_none_or_nil(tagtype: Option<TagType>) -> bool {
        match tagtype {
            None | Some(TagType::Nil) => true,
            _ => false,
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
        *self == TagType::State
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
    pub const STRING_UPVALUE: *const i8 = c"upvalue".as_ptr();
    pub const STRING_USERDATA: *const i8 = c"userdata".as_ptr();
    pub const TYPE_NAMES: [*const i8; 12] = [
        c"no value".as_ptr(),
        c"nil".as_ptr(),
        c"boolean".as_ptr(),
        TagType::STRING_USERDATA,
        c"number".as_ptr(),
        c"string".as_ptr(),
        c"table".as_ptr(),
        c"function".as_ptr(),
        TagType::STRING_USERDATA,
        c"thread".as_ptr(),
        TagType::STRING_UPVALUE,
        c"proto".as_ptr(),
    ];
}

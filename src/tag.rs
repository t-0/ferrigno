pub const TAG_TYPE_NIL: u8 = 0x00;
pub const TAG_TYPE_NIL_NIL: u8 = TAG_TYPE_NIL | (0x00 << 0x04);
pub const TAG_TYPE_NIL_EMPTY: u8 = TAG_TYPE_NIL | (0x01 << 0x04);
pub const TAG_TYPE_NIL_ABSENTKEY: u8 = TAG_TYPE_NIL | (0x02 << 0x04);
pub const TAG_TYPE_BOOLEAN: u8 = 0x01;
pub const TAG_TYPE_BOOLEAN_FALSE: u8 = TAG_TYPE_BOOLEAN | (0x00 << 0x04);
pub const TAG_TYPE_BOOLEAN_TRUE: u8 = TAG_TYPE_BOOLEAN | (0x01 << 0x04);
pub const TAG_TYPE_POINTER: u8 = 0x02;
pub const TAG_TYPE_NUMERIC: u8 = 0x03;
pub const TAG_TYPE_NUMERIC_INTEGER: u8 = TAG_TYPE_NUMERIC | (0x00 << 0x04);
pub const TAG_TYPE_NUMERIC_NUMBER: u8 = TAG_TYPE_NUMERIC | (0x01 << 0x04);
pub const TAG_TYPE_STRING: u8 = 0x4;
pub const TAG_TYPE_STRING_SHORT: u8 = TAG_TYPE_STRING | (0x00 << 0x04);
pub const TAG_TYPE_STRING_LONG: u8 = TAG_TYPE_STRING | (0x01 << 0x04);
pub const TAG_TYPE_TABLE: u8 = 0x5;
pub const TAG_TYPE_CLOSURE: u8 = 0x06;
pub const TAG_TYPE_CLOSURE_L: u8 = TAG_TYPE_CLOSURE | (0x00 << 0x04);
pub const TAG_TYPE_CLOSURE_CFUNCTION: u8 = TAG_TYPE_CLOSURE | (0x01 << 0x04);
pub const TAG_TYPE_CLOSURE_C: u8 = TAG_TYPE_CLOSURE | (0x02 << 0x04);
pub const TAG_TYPE_USER: u8 = 0x7;
pub const TAG_TYPE_STATE: u8 = 0x8;
pub const TAG_TYPE_UPVALUE: u8 = 0x9;
pub const TAG_TYPE_PROTOTYPE: u8 = 0xA;
pub const TAG_TYPE_MASK_: u8 = 0xF;
pub const fn get_tag_type(tag: u8) -> u8 {
    return TAG_TYPE_MASK_ & tag;
}
pub const TAG_NONE_: i32 = -1;
const TAG_COLLECTABLE: u8 = 0x40;
pub const fn set_collectable(tag: u8) -> u8 {
    return tag | TAG_COLLECTABLE;
}
pub const fn is_collectable(tag: u8) -> bool {
    return 0 != (TAG_COLLECTABLE & tag);
}
pub const COLLECTABLE_TAG_TYPE_TABLE: u8 = set_collectable(TAG_TYPE_TABLE);
pub const COLLECTABLE_TAG_TYPE_CLOSURE_C: u8 = set_collectable(TAG_TYPE_CLOSURE_C);
// pub const COLLECTABLE_TAG_TYPE_CLOSURE_CFUNCTION: u8 = set_collectable(TAG_TYPE_CLOSURE_CFUNCTION);
pub const COLLECTABLE_TAG_TYPE_CLOSURE_L: u8 = set_collectable(TAG_TYPE_CLOSURE_L);
pub const COLLECTABLE_TAG_TYPE_STATE: u8 = set_collectable(TAG_TYPE_STATE);
pub const COLLECTABLE_TAG_TYPE_STRING_SHORT: u8 = set_collectable(TAG_TYPE_STRING_SHORT);
pub const COLLECTABLE_TAG_TYPE_USER: u8 = set_collectable(TAG_TYPE_USER);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockControl {
    pub previous: *mut BlockControl,
    pub firstlabel: i32,
    pub firstgoto: i32,
    pub nactvar: u8,
    pub upval: u8,
    pub isloop: u8,
    pub insidetbc: u8,
}

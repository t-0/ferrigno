pub struct BlockControl {
    pub previous: *mut BlockControl,
    pub first_label: i32,
    pub first_goto: i32,
    pub count_active_variables: u8,
    pub count_upvalues: u8,
    pub is_loop: bool,
    pub is_inside_tbc: bool,
}

#[derive(Clone, Copy)]
pub enum FmtArg {
    Str(*const i8),
    Int(i32),
    Long(i64),
    Float(f64),
    Ptr(*mut std::ffi::c_void),
}

impl FmtArg {
    pub fn as_str(self) -> *const i8 {
        match self {
            FmtArg::Str(v) => v,
            _ => panic!("FmtArg: expected Str"),
        }
    }
    pub fn as_int(self) -> i32 {
        match self {
            FmtArg::Int(v) => v,
            FmtArg::Long(v) => v as i32,
            _ => panic!("FmtArg: expected Int"),
        }
    }
    pub fn as_long(self) -> i64 {
        match self {
            FmtArg::Long(v) => v,
            FmtArg::Int(v) => v as i64,
            _ => panic!("FmtArg: expected Long"),
        }
    }
    pub fn as_float(self) -> f64 {
        match self {
            FmtArg::Float(v) => v,
            _ => panic!("FmtArg: expected Float"),
        }
    }
    pub fn as_ptr(self) -> *mut std::ffi::c_void {
        match self {
            FmtArg::Ptr(v) => v,
            _ => panic!("FmtArg: expected Ptr"),
        }
    }
}

impl From<*const i8> for FmtArg {
    fn from(v: *const i8) -> Self {
        FmtArg::Str(v)
    }
}
impl From<*mut i8> for FmtArg {
    fn from(v: *mut i8) -> Self {
        FmtArg::Str(v as *const i8)
    }
}
impl From<i32> for FmtArg {
    fn from(v: i32) -> Self {
        FmtArg::Int(v)
    }
}
impl From<i64> for FmtArg {
    fn from(v: i64) -> Self {
        FmtArg::Long(v)
    }
}
impl From<f64> for FmtArg {
    fn from(v: f64) -> Self {
        FmtArg::Float(v)
    }
}
impl From<*mut std::ffi::c_void> for FmtArg {
    fn from(v: *mut std::ffi::c_void) -> Self {
        FmtArg::Ptr(v)
    }
}
impl From<usize> for FmtArg {
    fn from(v: usize) -> Self {
        FmtArg::Long(v as i64)
    }
}

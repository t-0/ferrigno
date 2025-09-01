#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum V{
    VVOID = 0,
    VNIL = 1,
    VTRUE = 2,
    VFALSE = 3,
    VK = 4,
    VKFLT = 5,
    VKINT = 6,
    VKSTR = 7,
    VNONRELOC = 8,
    VLOCAL = 9,
    VUPVAL = 10,
    VCONST = 11,
    VINDEXED = 12,
    VINDEXUP = 13,
    VINDEXI = 14,
    VINDEXSTR = 15,
    VJMP = 16,
    VRELOC = 17,
    VCALL = 18,
    VVARARG = 19,
}

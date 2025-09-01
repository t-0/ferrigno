#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum V{
    VVARARG = 19,
    VCALL = 18,
    VRELOC = 17,
    VJMP = 16,
    VINDEXSTR = 15,
    VINDEXI = 14,
    VINDEXUP = 13,
    VINDEXED = 12,
    VCONST = 11,
    VUPVAL = 10,
    VLOCAL = 9,
    VNONRELOC = 8,
    VKSTR = 7,
    VKINT = 6,
    VKFLT = 5,
    VK = 4,
    VFALSE = 3,
    VTRUE = 2,
    VNIL = 1,
    VVOID = 0,
}

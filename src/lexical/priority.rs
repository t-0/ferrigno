#[derive(Copy, Clone)]
#[repr(C)]
pub struct Priority {
    pub left: u8,
    pub right: u8,
}
pub const PRIORITY: [Priority; 21] = [
    {
        Priority {
            left: 10,
            right: 10,
        }
    },
    {
        Priority {
            left: 10,
            right: 10,
        }
    },
    {
        Priority {
            left: 11,
            right: 11,
        }
    },
    {
        Priority {
            left: 11,
            right: 11,
        }
    },
    {
        Priority {
            left: 14,
            right: 13,
        }
    },
    {
        Priority {
            left: 11,
            right: 11,
        }
    },
    {
        Priority {
            left: 11,
            right: 11,
        }
    },
    { Priority { left: 6, right: 6 } },
    { Priority { left: 4, right: 4 } },
    { Priority { left: 5, right: 5 } },
    { Priority { left: 7, right: 7 } },
    { Priority { left: 7, right: 7 } },
    { Priority { left: 9, right: 8 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 3, right: 3 } },
    { Priority { left: 2, right: 2 } },
    { Priority { left: 1, right: 1 } },
];

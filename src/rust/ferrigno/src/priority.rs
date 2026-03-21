#[derive(Copy, Clone)]
#[repr(C)]
pub struct Priority {
    pub priority_left: u8,
    pub priority_right: u8,
}
pub const PRIORITY: [Priority; 21] = [
    {
        Priority {
            priority_left: 10,
            priority_right: 10,
        }
    },
    {
        Priority {
            priority_left: 10,
            priority_right: 10,
        }
    },
    {
        Priority {
            priority_left: 11,
            priority_right: 11,
        }
    },
    {
        Priority {
            priority_left: 11,
            priority_right: 11,
        }
    },
    {
        Priority {
            priority_left: 14,
            priority_right: 13,
        }
    },
    {
        Priority {
            priority_left: 11,
            priority_right: 11,
        }
    },
    {
        Priority {
            priority_left: 11,
            priority_right: 11,
        }
    },
    {
        Priority {
            priority_left: 6,
            priority_right: 6,
        }
    },
    {
        Priority {
            priority_left: 4,
            priority_right: 4,
        }
    },
    {
        Priority {
            priority_left: 5,
            priority_right: 5,
        }
    },
    {
        Priority {
            priority_left: 7,
            priority_right: 7,
        }
    },
    {
        Priority {
            priority_left: 7,
            priority_right: 7,
        }
    },
    {
        Priority {
            priority_left: 9,
            priority_right: 8,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 3,
            priority_right: 3,
        }
    },
    {
        Priority {
            priority_left: 2,
            priority_right: 2,
        }
    },
    {
        Priority {
            priority_left: 1,
            priority_right: 1,
        }
    },
];

-- models/roland/d110.lua
-- Roland D-110 / MT-32
-- SysEx: F0 41 <dev> 16 12 <addr_h> <addr_m> <addr_l> <data...> <checksum> F7

return {
    name = "Roland D-110",

    sysex = {
        { param_index=0,  name="Master Volume",      template="F0 41 10 16 12 10 00 16 {v} {cs} F7", min_val=0, max_val=100, default_val=100 },
        { param_index=1,  name="Reverb Mode",        template="F0 41 10 16 12 10 00 01 {v} {cs} F7", min_val=0, max_val=3, default_val=0 },
        { param_index=2,  name="Reverb Time",        template="F0 41 10 16 12 10 00 02 {v} {cs} F7", min_val=0, max_val=7, default_val=5 },
        { param_index=3,  name="Reverb Level",       template="F0 41 10 16 12 10 00 03 {v} {cs} F7", min_val=0, max_val=7, default_val=3 },
        { param_index=4,  name="Part 1 Volume",      template="F0 41 10 16 12 03 00 06 {v} {cs} F7", min_val=0, max_val=100, default_val=100 },
        { param_index=5,  name="Part 1 Pan",         template="F0 41 10 16 12 03 00 07 {v} {cs} F7", min_val=0, max_val=14, default_val=7 },
        { param_index=6,  name="Part 1 Reverb",      template="F0 41 10 16 12 03 00 08 {v} {cs} F7", min_val=0, max_val=1, default_val=1 },
        { param_index=7,  name="Part 2 Volume",      template="F0 41 10 16 12 03 10 06 {v} {cs} F7", min_val=0, max_val=100, default_val=100 },
        { param_index=8,  name="Part 2 Pan",         template="F0 41 10 16 12 03 10 07 {v} {cs} F7", min_val=0, max_val=14, default_val=7 },
    },

    cc = {
        { cc_number=1,   note=-1, name="Modulation" },
        { cc_number=7,   note=-1, name="Volume" },
        { cc_number=10,  note=-1, name="Pan" },
        { cc_number=11,  note=-1, name="Expression" },
        { cc_number=64,  note=-1, name="Sustain" },
        { cc_number=91,  note=-1, name="Reverb Send" },
    },
}

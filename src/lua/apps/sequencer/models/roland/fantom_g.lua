-- models/roland/fantom_g.lua
-- Roland Fantom-G (G6/G7/G8 Workstation)
-- 128-voice polyphonic workstation with SuperNATURAL engine, ARX expansion,
-- multi-effects, and 16-part multitimbral capability.
-- Standard GS/GM2 CC assignments plus Fantom-specific extensions.

local cc = {
    -- Standard controllers
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=2,   note=-1, name="Breath Controller" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    -- Sound control (GS/GM2 standard)
    { cc_number=71,  note=-1, name="Resonance" },
    { cc_number=72,  note=-1, name="Release Time" },
    { cc_number=73,  note=-1, name="Attack Time" },
    { cc_number=74,  note=-1, name="Cutoff" },
    { cc_number=75,  note=-1, name="Decay Time" },
    { cc_number=76,  note=-1, name="Vibrato Rate" },
    { cc_number=77,  note=-1, name="Vibrato Depth" },
    { cc_number=78,  note=-1, name="Vibrato Delay" },
    -- Pedals / switches
    { cc_number=64,  note=-1, name="Hold (Damper)" },
    { cc_number=65,  note=-1, name="Portamento On/Off" },
    { cc_number=66,  note=-1, name="Sostenuto" },
    { cc_number=67,  note=-1, name="Soft Pedal" },
    { cc_number=84,  note=-1, name="Portamento Control" },
    -- Effects sends
    { cc_number=91,  note=-1, name="Reverb Send" },
    { cc_number=93,  note=-1, name="Chorus Send" },
    -- General purpose (assignable)
    { cc_number=16,  note=-1, name="General Purpose 1" },
    { cc_number=17,  note=-1, name="General Purpose 2" },
    { cc_number=18,  note=-1, name="General Purpose 3" },
    { cc_number=19,  note=-1, name="General Purpose 4" },
    { cc_number=80,  note=-1, name="General Purpose 5" },
    { cc_number=81,  note=-1, name="General Purpose 6" },
    { cc_number=82,  note=-1, name="General Purpose 7" },
    { cc_number=83,  note=-1, name="General Purpose 8" },
}

return {
    name = "Roland Fantom-G",
    cc   = cc,
}

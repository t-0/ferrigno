-- models/roland/mc101.lua
-- Roland MC-101 Groovebox
-- 4-track groovebox with ZEN-Core engine, drum kits, looper, and scatter.
-- CC 80-83 correspond to the four front-panel knobs (assignable per track).

local cc = {
    -- Knobs (transmit and receive, assignable per track)
    { cc_number=80,  note=-1, name="Filter Knob" },
    { cc_number=81,  note=-1, name="Mod Knob" },
    { cc_number=82,  note=-1, name="FX Knob" },
    { cc_number=83,  note=-1, name="Sound Knob" },
    -- Sound control
    { cc_number=74,  note=-1, name="Cutoff" },
    { cc_number=71,  note=-1, name="Resonance" },
    { cc_number=73,  note=-1, name="Attack" },
    { cc_number=75,  note=-1, name="Decay" },
    { cc_number=72,  note=-1, name="Release" },
    { cc_number=76,  note=-1, name="Vibrato Rate" },
    { cc_number=77,  note=-1, name="Vibrato Depth" },
    { cc_number=78,  note=-1, name="Vibrato Delay" },
    -- Mixer / performance
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=84,  note=-1, name="Portamento Control" },
    -- Pedals / switches
    { cc_number=64,  note=-1, name="Hold" },
    { cc_number=65,  note=-1, name="Glide" },
    { cc_number=66,  note=-1, name="Sostenuto" },
    { cc_number=67,  note=-1, name="Soft Pedal" },
    { cc_number=68,  note=-1, name="Legato Foot Switch" },
    -- Effects sends
    { cc_number=91,  note=-1, name="Reverb Send" },
    { cc_number=92,  note=-1, name="Chorus Send" },
    { cc_number=93,  note=-1, name="Delay Send" },
}

return {
    name = "Roland MC-101",
    cc   = cc,
}

-- models/korg/multipoly.lua
-- Korg multi/poly (Analog Modeling Synthesizer; also multi/poly module)
-- Mono/Poly-inspired: 4 VCOs per voice, 60-voice polyphony, 2 Layers,
-- Kaoss Physics, 4 Mod Knobs, mod matrix, motion sequencing.
-- No dedicated CCs for synth params; panel knobs edit the sound directly and
-- are NOT transmitted as CCs. Mod Knobs and Scale Select send/receive the CCs
-- below (defaults, reassignable in UTILITY > MIDI CC Assign). Program Mod Knobs
-- use each Layer's MIDI channel, so the CC numbers are shared across Layers.
-- Kaoss Physics default set is 18-20, 105-110 (alternatives: 18-20, 114-119;
-- 102-110; 111-119). Any CC 1-119 can also be a mod-matrix source.
-- No NRPN or parameter SysEx; supports universal SysEx (device inquiry, master
-- volume/tuning) and MIDI-CI Property Exchange. Set List slots A-D (64) map to
-- Program Change 1-64.

local cc = {
    -- Fixed (cannot be reassigned)
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=12,  note=-1, name="Kaoss Button" },
    { cc_number=64,  note=-1, name="Damper" },
    { cc_number=66,  note=-1, name="Sostenuto" },
    { cc_number=67,  note=-1, name="Soft" },
    -- Scale Select (default CC)
    { cc_number=9,   note=-1, name="Scale Select" },
    -- Mod Knobs 1-4 (default CCs)
    { cc_number=24,  note=-1, name="Mod Knob 1" },
    { cc_number=25,  note=-1, name="Mod Knob 2" },
    { cc_number=26,  note=-1, name="Mod Knob 3" },
    { cc_number=27,  note=-1, name="Mod Knob 4" },
    -- Kaoss Physics (default set 18-20, 105-110)
    { cc_number=18,  note=-1, name="Kaoss X Position MSB" },
    { cc_number=19,  note=-1, name="Kaoss Y Position MSB" },
    { cc_number=20,  note=-1, name="Kaoss Touched On/Off" },
    { cc_number=105, note=-1, name="Kaoss X Position LSB" },
    { cc_number=106, note=-1, name="Kaoss Y Position LSB" },
    { cc_number=107, note=-1, name="Kaoss X Velocity MSB" },
    { cc_number=108, note=-1, name="Kaoss X Velocity LSB" },
    { cc_number=109, note=-1, name="Kaoss Y Velocity MSB" },
    { cc_number=110, note=-1, name="Kaoss Y Velocity LSB" },
    -- Channel mode
    { cc_number=120, note=-1, name="All Sound Off" },
    { cc_number=121, note=-1, name="Reset All Controllers" },
}

return {
    name = "Korg multi/poly",
    cc   = cc,
}

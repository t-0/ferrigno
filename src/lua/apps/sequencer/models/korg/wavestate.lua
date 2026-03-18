-- models/korg/wavestate.lua
-- Korg wavestate (Wave Sequencing Synthesizer)
-- 37-key, 64-voice polyphony, 4 layers, Wave Sequencing 2.0, vector synthesis.
-- No dedicated CCs for synth params; uses 40 assignable Mod Knobs routed via
-- the modulation matrix. CC assignments below are factory defaults, reassignable
-- in UTILITY > MIDI CC Assign. No NRPN or traditional SysEx; uses MIDI-CI
-- Property Exchange for deep editing. Patch transfer via USB Editor/Librarian.

local cc = {
    -- Fixed (cannot be reassigned)
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=16,  note=-1, name="Vector Joystick X" },
    { cc_number=17,  note=-1, name="Vector Joystick Y" },
    { cc_number=64,  note=-1, name="Damper" },
    { cc_number=66,  note=-1, name="Sostenuto" },
    { cc_number=67,  note=-1, name="Soft" },
    -- Performance Mod Knobs 1-8 (default CCs)
    { cc_number=24,  note=-1, name="Perf Mod Knob 1" },
    { cc_number=25,  note=-1, name="Perf Mod Knob 2" },
    { cc_number=26,  note=-1, name="Perf Mod Knob 3" },
    { cc_number=27,  note=-1, name="Perf Mod Knob 4" },
    { cc_number=28,  note=-1, name="Perf Mod Knob 5" },
    { cc_number=29,  note=-1, name="Perf Mod Knob 6" },
    { cc_number=30,  note=-1, name="Perf Mod Knob 7" },
    { cc_number=31,  note=-1, name="Perf Mod Knob 8" },
    -- Layer A Mod Knobs 1-8
    { cc_number=80,  note=-1, name="Layer A Knob 1" },
    { cc_number=81,  note=-1, name="Layer A Knob 2" },
    { cc_number=82,  note=-1, name="Layer A Knob 3" },
    { cc_number=83,  note=-1, name="Layer A Knob 4" },
    { cc_number=84,  note=-1, name="Layer A Knob 5" },
    { cc_number=85,  note=-1, name="Layer A Knob 6" },
    { cc_number=86,  note=-1, name="Layer A Knob 7" },
    { cc_number=87,  note=-1, name="Layer A Knob 8" },
    -- Layer B Mod Knobs 1-8
    { cc_number=88,  note=-1, name="Layer B Knob 1" },
    { cc_number=89,  note=-1, name="Layer B Knob 2" },
    { cc_number=90,  note=-1, name="Layer B Knob 3" },
    { cc_number=91,  note=-1, name="Layer B Knob 4" },
    { cc_number=92,  note=-1, name="Layer B Knob 5" },
    { cc_number=93,  note=-1, name="Layer B Knob 6" },
    { cc_number=94,  note=-1, name="Layer B Knob 7" },
    { cc_number=95,  note=-1, name="Layer B Knob 8" },
    -- Layer C Mod Knobs 1-8
    { cc_number=102, note=-1, name="Layer C Knob 1" },
    { cc_number=103, note=-1, name="Layer C Knob 2" },
    { cc_number=104, note=-1, name="Layer C Knob 3" },
    { cc_number=105, note=-1, name="Layer C Knob 4" },
    { cc_number=106, note=-1, name="Layer C Knob 5" },
    { cc_number=107, note=-1, name="Layer C Knob 6" },
    { cc_number=108, note=-1, name="Layer C Knob 7" },
    { cc_number=109, note=-1, name="Layer C Knob 8" },
    -- Layer D Mod Knobs 1-8
    { cc_number=110, note=-1, name="Layer D Knob 1" },
    { cc_number=111, note=-1, name="Layer D Knob 2" },
    { cc_number=112, note=-1, name="Layer D Knob 3" },
    { cc_number=113, note=-1, name="Layer D Knob 4" },
    { cc_number=114, note=-1, name="Layer D Knob 5" },
    { cc_number=115, note=-1, name="Layer D Knob 6" },
    { cc_number=116, note=-1, name="Layer D Knob 7" },
    { cc_number=117, note=-1, name="Layer D Knob 8" },
}

return {
    name = "Korg wavestate",
    cc   = cc,
}

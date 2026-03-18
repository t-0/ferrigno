-- models/behringer/rd6.lua
-- Behringer RD-6 (Analog Drum Machine, TR-606 clone)
-- 8 drum sounds: BD, SD, LT, HT, CP, CH, OH, CY.
-- MIDI note triggering only; no documented CC support for knob parameters
-- (Level, Tone, Decay, Snap/Attack are front-panel only).
-- Accent is triggered by velocity exceeding the accent threshold.

local drum_map = {
    { note=36, name="BD" },   -- Bass Drum
    { note=39, name="CP" },   -- Clap
    { note=40, name="SD" },   -- Snare Drum
    { note=42, name="CH" },   -- Closed Hi-Hat
    { note=45, name="LT" },   -- Low Tom
    { note=46, name="OH" },   -- Open Hi-Hat
    { note=50, name="HT" },   -- High Tom
    { note=51, name="CY" },   -- Cymbal
}

return {
    name     = "Behringer RD-6",
    drum_map = drum_map,
}

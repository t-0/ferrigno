-- models/behringer/monopoly.lua
-- Behringer Monopoly (4-Voice Analog Polyphonic Synthesizer, Korg Mono/Poly clone)
-- 4 VCOs, 24dB LPF, LFO, arpeggiator, unison/poly/share voice modes.
-- Very limited MIDI: responds to note number, pitch bend, and modulation only.
-- No CC output from knobs/switches. No NRPN/CC parameter control.
-- Global settings transferable via SysEx (SynthTool app).

local cc = {
    { cc_number=1,  note=-1, name="Mod Wheel" },
}

return {
    name = "Behringer Monopoly",
    cc   = cc,
}

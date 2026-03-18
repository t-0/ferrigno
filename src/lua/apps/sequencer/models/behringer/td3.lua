-- models/behringer/td3.lua
-- Behringer TD-3 (Monophonic Analog Bass Synthesizer, TB-303 clone)
-- Single VCO (saw/square), 24dB lowpass filter, envelope mod, decay, accent.
-- Very limited MIDI CC: original TD-3 has no CC support at all; the TD-3-MO
-- adds CC 74 (Filter Cutoff). Resonance, Env Mod, Decay, Accent level, and
-- Waveform are not controllable via MIDI. Accent is triggered by velocity
-- exceeding a threshold (default 96, configurable via SysEx). Slides are
-- triggered by overlapping (legato) notes.
-- SysEx: F0 00 20 32 00 01 0A ... F7 (config/pattern management only, not
-- real-time parameter control).

local cc = {
    { cc_number=74,  note=-1, name="Filter Cutoff" },
}

return {
    name = "Behringer TD-3",
    cc   = cc,
}

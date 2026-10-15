-- models/clavia/nord_drum.lua
-- Clavia Nord Drum (original 2012 model, 4-channel drum synthesizer)
-- 4 channels, each a blend of Noise, Tone and Click components with a
-- dedicated filter, EQ, bend and decay. 99 programs, 6 trigger inputs.
-- Minimal MIDI: all four channels receive Note On/Off with velocity on a
-- single global MIDI channel, plus Program Change (1-99). No CC, NRPN,
-- aftertouch or pitch bend response; sound editing is front-panel only.
-- Factory default notes below are as printed in the manual (which describes
-- them as GM Bass Drum 1, Snare Drum 1, Hi Tom 1 and Mid Tom 1); each
-- channel's note is reassignable via MIDI Note / MIDI Learn.
-- Program dumps (one or all 99) are available via SysEx (Shift + Prog Dump).

local drum_map = {
    { note=36, name="Ch 1" },   -- Channel 1
    { note=38, name="Ch 2" },   -- Channel 2
    { note=59, name="Ch 3" },   -- Channel 3
    { note=47, name="Ch 4" },   -- Channel 4
}

return {
    name     = "Clavia Nord Drum",
    drum_map = drum_map,
}

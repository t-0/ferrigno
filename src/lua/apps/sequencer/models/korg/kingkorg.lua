-- models/korg/kingkorg.lua
-- Korg KingKORG (61-key analog modeling synthesizer)
-- 24-voice polyphony, 3 oscillators, 2 filters, vocoder, virtual tube circuit.
-- CC assignments below are factory defaults; reassignable via GLOBAL g28 CC#Map.
-- OSC3, Filter Type, Noise Level, EG3, and Vocoder params are SysEx-only.
-- SysEx header: F0 42 3n 00 01 18 <func> ... F7 (n = MIDI channel 0-F).

local cc = {
    -- Performance / standard
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=2,   note=-1, name="Filter Mod (Joy -Y)" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=7,   note=-1, name="Amp Level" },
    { cc_number=10,  note=-1, name="Amp Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=64,  note=-1, name="Damper" },
    { cc_number=65,  note=-1, name="Portamento Switch" },
    -- Oscillator
    { cc_number=3,   note=-1, name="Unison Voice" },
    { cc_number=8,   note=-1, name="OSC1 Type" },
    { cc_number=15,  note=-1, name="OSC1 Control 1" },
    { cc_number=17,  note=-1, name="OSC1 Control 2" },
    { cc_number=18,  note=-1, name="OSC2 Type" },
    { cc_number=19,  note=-1, name="OSC2 Control 1" },
    { cc_number=20,  note=-1, name="OSC2 Control 2" },
    -- Mixer
    { cc_number=23,  note=-1, name="OSC1 Level" },
    { cc_number=24,  note=-1, name="OSC2 Level" },
    { cc_number=25,  note=-1, name="OSC3 Level" },
    -- Filter
    { cc_number=74,  note=-1, name="Filter Cutoff" },
    { cc_number=71,  note=-1, name="Filter Resonance" },
    { cc_number=79,  note=-1, name="Filter EG Intensity" },
    { cc_number=28,  note=-1, name="Filter KeyTrack" },
    { cc_number=89,  note=-1, name="Filter LFO1 Intensity" },
    -- Filter EG (EG1)
    { cc_number=85,  note=-1, name="Filter EG Attack" },
    { cc_number=86,  note=-1, name="Filter EG Decay" },
    { cc_number=87,  note=-1, name="Filter EG Sustain" },
    { cc_number=88,  note=-1, name="Filter EG Release" },
    -- Amp EG (EG2)
    { cc_number=73,  note=-1, name="Amp EG Attack" },
    { cc_number=75,  note=-1, name="Amp EG Decay" },
    { cc_number=70,  note=-1, name="Amp EG Sustain" },
    { cc_number=72,  note=-1, name="Amp EG Release" },
    -- LFO
    { cc_number=90,  note=-1, name="LFO1 Frequency" },
    { cc_number=76,  note=-1, name="LFO2 Frequency" },
    { cc_number=77,  note=-1, name="LFO2 Pitch Intensity" },
    -- Virtual Patch (receive-only)
    { cc_number=103, note=-1, name="V.Patch 1 Intensity" },
    { cc_number=104, note=-1, name="V.Patch 2 Intensity" },
    { cc_number=105, note=-1, name="V.Patch 3 Intensity" },
    { cc_number=106, note=-1, name="V.Patch 4 Intensity" },
    { cc_number=107, note=-1, name="V.Patch 5 Intensity" },
    { cc_number=108, note=-1, name="V.Patch 6 Intensity" },
    -- Effects
    { cc_number=12,  note=-1, name="PreFX Drive" },
    { cc_number=115, note=-1, name="PreFX Switch" },
    { cc_number=91,  note=-1, name="ModFX Depth" },
    { cc_number=113, note=-1, name="ModFX Speed" },
    { cc_number=94,  note=-1, name="ModFX Switch" },
    { cc_number=93,  note=-1, name="Rev/Dly Depth" },
    { cc_number=114, note=-1, name="Rev/Dly Time" },
    { cc_number=95,  note=-1, name="Rev/Dly Switch" },
    -- EQ
    { cc_number=109, note=-1, name="EQ Hi Gain" },
    { cc_number=110, note=-1, name="EQ Lo Gain" },
}

local nrpn = {
    { nrpn_number=2,    note=-1, name="Arp Switch" },
    { nrpn_number=1280, note=-1, name="Voice Mode" },
    { nrpn_number=1284, note=-1, name="Vocoder Switch" },
}

return {
    name = "Korg KingKORG",
    cc   = cc,
    nrpn = nrpn,
}

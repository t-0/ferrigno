-- models/sonicware/liven_8bit_warps.lua
-- Sonicware LIVEN 8bit warps (8-bit Wave Memory Synthesizer, LVN-010)
-- 6-voice polyphony, 4 synth engines (WARP, ATTACK, MORPH, FM),
-- 63 preset + user waveforms, filter (LPF/HPF/BPF), sweep, 10 FX + 6 reverbs.
-- No NRPN support. SysEx used for patch/pattern export only.

local cc = {
    -- Performance
    { cc_number=5,   note=-1, name="Glide / Arp Type" },
    { cc_number=52,  note=-1, name="Voice Mode" },
    -- Synth engine
    { cc_number=20,  note=-1, name="Synth Engine" },
    { cc_number=21,  note=-1, name="Octave" },
    { cc_number=22,  note=-1, name="Velocity" },
    { cc_number=23,  note=-1, name="Detune" },
    -- Engine parameters (meaning varies by engine)
    { cc_number=24,  note=-1, name="Synth Param 1" },
    { cc_number=25,  note=-1, name="Synth Param 2" },
    { cc_number=26,  note=-1, name="Synth Param 3" },
    { cc_number=27,  note=-1, name="Synth Param 4" },
    { cc_number=28,  note=-1, name="Synth Param 5" },
    { cc_number=29,  note=-1, name="Synth Param 6" },
    { cc_number=30,  note=-1, name="Alias Noise" },
    -- Filter
    { cc_number=31,  note=-1, name="Filter Active" },
    { cc_number=32,  note=-1, name="Filter Type" },
    { cc_number=33,  note=-1, name="Filter Cutoff" },
    { cc_number=34,  note=-1, name="Filter Resonance" },
    { cc_number=35,  note=-1, name="Filter CO" },
    -- LFO
    { cc_number=36,  note=-1, name="LFO Rate" },
    { cc_number=37,  note=-1, name="LFO Pitch" },
    -- Envelope
    { cc_number=38,  note=-1, name="EG Attack" },
    { cc_number=39,  note=-1, name="EG Decay" },
    { cc_number=40,  note=-1, name="EG Sustain" },
    { cc_number=41,  note=-1, name="EG Release" },
    -- Sweep
    { cc_number=42,  note=-1, name="Sweep" },
    { cc_number=43,  note=-1, name="Sweep Speed" },
    { cc_number=44,  note=-1, name="Sweep Shift" },
    -- Effects
    { cc_number=45,  note=-1, name="FX Type" },
    { cc_number=46,  note=-1, name="FX Speed" },
    { cc_number=47,  note=-1, name="FX Amount" },
    { cc_number=48,  note=-1, name="Reverb Type" },
    { cc_number=49,  note=-1, name="Reverb" },
    -- Sequencer
    { cc_number=50,  note=-1, name="Gate Time" },
    { cc_number=51,  note=-1, name="Swing" },
    { cc_number=53,  note=-1, name="Seq Mode" },
    { cc_number=54,  note=-1, name="Mem Level" },
    { cc_number=55,  note=-1, name="Param Lock" },
}

return {
    name = "LIVEN 8bit warps",
    cc   = cc,
}

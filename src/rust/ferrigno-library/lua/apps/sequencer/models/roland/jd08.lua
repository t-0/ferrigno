-- models/roland/jd08.lua
-- Roland JD-08 (Boutique JD-800 sound module)
-- 4-tone polyphonic synth with per-tone WG/TVF/TVA sections, dual LFOs,
-- multi-stage envelopes, and built-in effects.
-- No SysEx support; all parameters via CC only.

local cc = {
    -- LFO 1
    { cc_number=14,  note=-1, name="LFO1 Rate" },
    { cc_number=96,  note=-1, name="LFO1 Rate (Sync)" },
    { cc_number=15,  note=-1, name="LFO1 Delay" },
    { cc_number=16,  note=-1, name="LFO1 Fade" },
    { cc_number=17,  note=-1, name="LFO1 Offset" },
    { cc_number=18,  note=-1, name="LFO1 Key Trigger" },
    { cc_number=19,  note=-1, name="LFO1 Waveform" },
    -- LFO 2
    { cc_number=20,  note=-1, name="LFO2 Rate" },
    { cc_number=97,  note=-1, name="LFO2 Rate (Sync)" },
    { cc_number=21,  note=-1, name="LFO2 Delay" },
    { cc_number=22,  note=-1, name="LFO2 Fade" },
    { cc_number=23,  note=-1, name="LFO2 Waveform" },
    { cc_number=24,  note=-1, name="LFO2 Offset" },
    { cc_number=25,  note=-1, name="LFO2 Key Trigger" },
    -- WG (Wave Generator)
    { cc_number=72,  note=-1, name="Waveform" },           -- 0-108
    { cc_number=79,  note=-1, name="Pitch Coarse" },
    { cc_number=80,  note=-1, name="Pitch Fine" },
    { cc_number=81,  note=-1, name="Pitch Random" },
    { cc_number=82,  note=-1, name="Pitch Key Follow" },
    { cc_number=83,  note=-1, name="WG LFO1 Depth" },
    { cc_number=85,  note=-1, name="WG LFO2 Depth" },
    -- Pitch Envelope
    { cc_number=26,  note=-1, name="Pitch Env Time KF" },
    { cc_number=27,  note=-1, name="Pitch Env L0" },
    { cc_number=28,  note=-1, name="Pitch Env T1" },
    { cc_number=29,  note=-1, name="Pitch Env L1" },
    { cc_number=30,  note=-1, name="Pitch Env T2" },
    { cc_number=35,  note=-1, name="Pitch Env L2" },
    { cc_number=31,  note=-1, name="Pitch Env T3" },
    -- TVF (Time-Variant Filter)
    { cc_number=3,   note=-1, name="TVF Cutoff" },
    { cc_number=9,   note=-1, name="TVF Resonance" },
    { cc_number=86,  note=-1, name="TVF Filter Mode" },
    { cc_number=87,  note=-1, name="TVF Env Depth" },
    { cc_number=89,  note=-1, name="TVF Key Follow" },
    { cc_number=90,  note=-1, name="TVF LFO Select" },
    { cc_number=102, note=-1, name="TVF LFO Depth" },
    -- Filter Envelope
    { cc_number=46,  note=-1, name="Filter Env Time KF" },
    { cc_number=47,  note=-1, name="Filter Env T1" },
    { cc_number=48,  note=-1, name="Filter Env L1" },
    { cc_number=50,  note=-1, name="Filter Env T2" },
    { cc_number=51,  note=-1, name="Filter Env L2" },
    { cc_number=52,  note=-1, name="Filter Env T3" },
    { cc_number=53,  note=-1, name="Filter Env Sustain" },
    { cc_number=54,  note=-1, name="Filter Env T4" },
    { cc_number=56,  note=-1, name="Filter Env L4" },
    -- TVA (Time-Variant Amplifier)
    { cc_number=103, note=-1, name="TVA Level" },
    { cc_number=104, note=-1, name="TVA Bias Point" },
    { cc_number=105, note=-1, name="TVA Bias Level" },
    { cc_number=106, note=-1, name="TVA Bias Direction" },
    { cc_number=107, note=-1, name="TVA LFO Select" },
    { cc_number=108, note=-1, name="TVA LFO Depth" },
    -- Amp Envelope
    { cc_number=55,  note=-1, name="Amp Env Time KF" },
    { cc_number=57,  note=-1, name="Amp Env T1" },
    { cc_number=58,  note=-1, name="Amp Env L1" },
    { cc_number=59,  note=-1, name="Amp Env T2" },
    { cc_number=60,  note=-1, name="Amp Env L2" },
    { cc_number=61,  note=-1, name="Amp Env T3" },
    { cc_number=62,  note=-1, name="Amp Env Sustain" },
    { cc_number=63,  note=-1, name="Amp Env T4" },
    -- Effects
    { cc_number=12,  note=-1, name="FX B Level" },
    { cc_number=13,  note=-1, name="FX B Reverb Time" },
    { cc_number=91,  note=-1, name="FX B Reverb Level" },
    { cc_number=92,  note=-1, name="FX B Delay L Level" },
    { cc_number=93,  note=-1, name="FX B Chorus Level" },
    { cc_number=94,  note=-1, name="FX B Delay C Level" },
    { cc_number=95,  note=-1, name="FX B Delay R Level" },
    -- Tone
    { cc_number=110, note=-1, name="Tone Level" },
    { cc_number=111, note=-1, name="Layer" },
    { cc_number=112, note=-1, name="Active" },
    -- Performance
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=7,   note=-1, name="Part Level" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=41,  note=-1, name="Bend Range Up" },
    { cc_number=49,  note=-1, name="Bend Range Down" },
    { cc_number=64,  note=-1, name="Hold Pedal" },
    { cc_number=66,  note=-1, name="Sostenuto Pedal" },
    { cc_number=115, note=-1, name="Solo" },
    { cc_number=116, note=-1, name="Legato" },
    { cc_number=117, note=-1, name="Portamento" },
    { cc_number=118, note=-1, name="Portamento Mode" },
    { cc_number=119, note=-1, name="Unison" },
}

return {
    name = "Roland JD-08",
    cc   = cc,
}

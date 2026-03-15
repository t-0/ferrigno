-- models/behringer/jt4000.lua
-- Behringer JT-4000 Micro (4-voice polyphonic analog/digital hybrid synth)
-- 2 oscillators (tri/square/PWM/saw/supersaw/FM/noise), analog VCF (LPF/HPF/BPF),
-- 2 ADSR envelopes, 2 LFOs, ring modulator. No built-in effects.
-- No NRPN support. SysEx for patch management only.
-- Requires firmware v1.1.5+ for full CC coverage.

local cc = {
    -- Performance
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    -- Oscillator 1
    { cc_number=24,  note=-1, name="Osc 1 Wave" },
    { cc_number=115, note=-1, name="Osc 1 Coarse Tune" },
    { cc_number=111, note=-1, name="Osc 1 Fine Tune" },
    { cc_number=113, note=-1, name="Osc 1 PWM/Detune/FB" },
    -- Oscillator 2
    { cc_number=25,  note=-1, name="Osc 2 Wave" },
    { cc_number=116, note=-1, name="Osc 2 Coarse Tune" },
    { cc_number=112, note=-1, name="Osc 2 Fine Tune" },
    { cc_number=114, note=-1, name="Osc 2 PWM" },
    -- Mixer
    { cc_number=29,  note=-1, name="Osc Balance" },
    -- Filter
    { cc_number=74,  note=-1, name="VCF Cutoff" },
    { cc_number=71,  note=-1, name="VCF Resonance" },
    { cc_number=47,  note=-1, name="VCF Env Amount" },
    -- Filter envelope
    { cc_number=85,  note=-1, name="VCF EG Attack" },
    { cc_number=86,  note=-1, name="VCF EG Decay" },
    { cc_number=87,  note=-1, name="VCF EG Sustain" },
    { cc_number=88,  note=-1, name="VCF EG Release" },
    -- Amp envelope
    { cc_number=81,  note=-1, name="VCA EG Attack" },
    { cc_number=82,  note=-1, name="VCA EG Decay" },
    { cc_number=83,  note=-1, name="VCA EG Sustain" },
    { cc_number=84,  note=-1, name="VCA EG Release" },
    -- LFO 1
    { cc_number=54,  note=-1, name="LFO 1 Waveform" },
    { cc_number=72,  note=-1, name="LFO 1 Rate" },
    { cc_number=70,  note=-1, name="LFO 1 Amount" },
    { cc_number=56,  note=-1, name="LFO 1 Destination" },
    -- LFO 2
    { cc_number=55,  note=-1, name="LFO 2 Waveform" },
    { cc_number=73,  note=-1, name="LFO 2 Rate" },
    { cc_number=28,  note=-1, name="LFO 2 Amount" },
    -- Ring Modulator
    { cc_number=96,  note=-1, name="Ring Mod Toggle" },
    { cc_number=95,  note=-1, name="Ring Mod Amount" },
}

return {
    name = "Behringer JT-4000",
    cc   = cc,
}

-- models/sequential/evolver.lua
-- DSI / Sequential Evolver Desktop (Mono/4-voice hybrid analog/digital synth)
-- 2 analog oscillators + 2 digital oscillators, Curtis lowpass filter, 3 envelopes,
-- 4 LFOs, 16-step x 4-track sequencer, feedback path, 3 delays, distortion.
-- Parameters beyond CCs use proprietary SysEx (not standard NRPNs).
-- SysEx: F0 01 20 01 <cmd> ... F7 (DSI mfr ID 01, Evolver device 20h).

local cc = {
    -- Standard controllers
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=2,   note=-1, name="Breath Controller" },
    { cc_number=4,   note=-1, name="Foot Controller" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=32,  note=-1, name="Bank Select" },
    { cc_number=64,  note=-1, name="Sustain" },
    { cc_number=74,  note=-1, name="Brightness" },
    -- Oscillators
    { cc_number=20,  note=-1, name="Osc 1 Frequency" },
    { cc_number=21,  note=-1, name="Osc 2 Frequency" },
    { cc_number=22,  note=-1, name="Osc 3 Frequency" },
    { cc_number=23,  note=-1, name="Osc 4 Frequency" },
    { cc_number=24,  note=-1, name="Osc 1 Level" },
    { cc_number=25,  note=-1, name="Osc 2 Level" },
    { cc_number=26,  note=-1, name="Osc 3 Level" },
    { cc_number=27,  note=-1, name="Osc 4 Level" },
    { cc_number=28,  note=-1, name="Osc 1 Shape" },
    { cc_number=29,  note=-1, name="Osc 2 Shape" },
    { cc_number=30,  note=-1, name="Osc 3 Shape" },
    { cc_number=31,  note=-1, name="Osc 4 Shape" },
    -- Cross-modulation
    { cc_number=40,  note=-1, name="FM Osc 4->3" },
    { cc_number=41,  note=-1, name="FM Osc 3->4" },
    { cc_number=42,  note=-1, name="Ring Mod 4->3" },
    { cc_number=43,  note=-1, name="Ring Mod 3->4" },
    -- Filter
    { cc_number=52,  note=-1, name="Filter Frequency" },
    { cc_number=53,  note=-1, name="Filter Resonance" },
    { cc_number=54,  note=-1, name="Filter Env Amount" },
    { cc_number=55,  note=-1, name="Filter Env Attack" },
    { cc_number=56,  note=-1, name="Filter Env Decay" },
    { cc_number=57,  note=-1, name="Filter Env Sustain" },
    { cc_number=58,  note=-1, name="Filter Env Release" },
    { cc_number=59,  note=-1, name="Filter Audio Mod" },
    { cc_number=60,  note=-1, name="Filter Split" },
    { cc_number=61,  note=-1, name="Filter Key Amount" },
    { cc_number=62,  note=-1, name="Noise Level" },
    -- Amp envelope
    { cc_number=75,  note=-1, name="Amp Env Attack" },
    { cc_number=76,  note=-1, name="Amp Env Decay" },
    { cc_number=77,  note=-1, name="Amp Env Sustain" },
    { cc_number=78,  note=-1, name="Amp Env Release" },
    -- Feedback
    { cc_number=85,  note=-1, name="Feedback Frequency" },
    { cc_number=86,  note=-1, name="Feedback Level" },
    -- Distortion / Highpass
    { cc_number=12,  note=-1, name="Distortion" },
    { cc_number=13,  note=-1, name="Highpass Cutoff" },
    -- Delays
    { cc_number=102, note=-1, name="Delay 1 Time" },
    { cc_number=103, note=-1, name="Delay 2 Time" },
    { cc_number=104, note=-1, name="Delay 3 Time" },
    { cc_number=105, note=-1, name="Delay 1 Level" },
    { cc_number=106, note=-1, name="Delay 2 Level" },
    { cc_number=107, note=-1, name="Delay 3 Level" },
    { cc_number=108, note=-1, name="Delay Feedback 1" },
    { cc_number=109, note=-1, name="Delay Feedback 2" },
}

return {
    name = "DSI Evolver",
    cc   = cc,
}

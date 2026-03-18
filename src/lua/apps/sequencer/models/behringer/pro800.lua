-- models/behringer/pro800.lua
-- Behringer Pro-800 (8-Voice Analog Polyphonic Synthesizer, Prophet-600 inspired)
-- 2 DCOs per voice (saw/triangle/square/PWM), 4-pole LPF, 2 ADSR envelopes,
-- LFO, poly mod, arpeggiator, unison mode. Full CC support.

local cc = {
    -- Performance
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=2,   note=-1, name="Breath" },
    { cc_number=3,   note=-1, name="Master Tune" },
    { cc_number=7,   note=-1, name="Main Volume" },
    { cc_number=64,  note=-1, name="Sustain Pedal" },
    -- Oscillator A
    { cc_number=49,  note=-1, name="Osc A Saw" },
    { cc_number=50,  note=-1, name="Osc A Square" },
    { cc_number=80,  note=-1, name="Osc A Freq" },
    { cc_number=81,  note=-1, name="Osc A Volume" },
    { cc_number=82,  note=-1, name="Osc A PW" },
    { cc_number=68,  note=-1, name="Osc A Chromatic Pitch" },
    -- Oscillator B
    { cc_number=51,  note=-1, name="Osc B Saw" },
    { cc_number=52,  note=-1, name="Osc B Triangle" },
    { cc_number=53,  note=-1, name="Osc B Square" },
    { cc_number=54,  note=-1, name="Osc B Sync" },
    { cc_number=83,  note=-1, name="Osc B Freq" },
    { cc_number=84,  note=-1, name="Osc B Volume" },
    { cc_number=85,  note=-1, name="Osc B PW" },
    { cc_number=86,  note=-1, name="Osc B Fine" },
    { cc_number=69,  note=-1, name="Osc B Chromatic Pitch" },
    -- Mixer
    { cc_number=113, note=-1, name="Noise" },
    -- Poly Mod
    { cc_number=55,  note=-1, name="Poly Mod Freq A" },
    { cc_number=56,  note=-1, name="Poly Mod VCF" },
    { cc_number=102, note=-1, name="Poly Mod Filter Env Amount" },
    { cc_number=103, note=-1, name="Poly Mod Osc B Amount" },
    -- Filter
    { cc_number=87,  note=-1, name="VCF Freq" },
    { cc_number=88,  note=-1, name="VCF Resonance" },
    { cc_number=89,  note=-1, name="VCF Env Amount" },
    { cc_number=60,  note=-1, name="VCF Keyboard" },
    { cc_number=108, note=-1, name="VCF Velocity" },
    { cc_number=115, note=-1, name="VCF Aftertouch" },
    -- Filter envelope
    { cc_number=61,  note=-1, name="VCF Env Exponential" },
    { cc_number=62,  note=-1, name="VCF Env Speed" },
    { cc_number=93,  note=-1, name="VCF Attack" },
    { cc_number=92,  note=-1, name="VCF Decay" },
    { cc_number=91,  note=-1, name="VCF Sustain" },
    { cc_number=90,  note=-1, name="VCF Release" },
    -- Amp envelope
    { cc_number=63,  note=-1, name="VCA Env Exponential" },
    { cc_number=72,  note=-1, name="VCA Env Speed" },
    { cc_number=101, note=-1, name="VCA Attack" },
    { cc_number=100, note=-1, name="VCA Decay" },
    { cc_number=95,  note=-1, name="VCA Sustain" },
    { cc_number=94,  note=-1, name="VCA Release" },
    { cc_number=107, note=-1, name="VCA Velocity" },
    { cc_number=114, note=-1, name="VCA Aftertouch" },
    -- LFO
    { cc_number=57,  note=-1, name="LFO Shape" },
    { cc_number=58,  note=-1, name="LFO Speed" },
    { cc_number=59,  note=-1, name="LFO Targets" },
    { cc_number=74,  note=-1, name="LFO Dest Freq" },
    { cc_number=75,  note=-1, name="LFO Dest Filter" },
    { cc_number=76,  note=-1, name="LFO Dest PWM" },
    { cc_number=104, note=-1, name="LFO Freq" },
    { cc_number=105, note=-1, name="LFO Amount" },
    { cc_number=116, note=-1, name="LFO Aftertouch" },
    -- Vibrato
    { cc_number=71,  note=-1, name="Vibrato Target" },
    { cc_number=110, note=-1, name="Vibrato Freq" },
    { cc_number=111, note=-1, name="Vibrato Amount" },
    -- Mod Wheel routing
    { cc_number=67,  note=-1, name="Mod Wheel Amount" },
    { cc_number=70,  note=-1, name="Mod Wheel Target" },
    { cc_number=109, note=-1, name="Mod Delay" },
    -- Performance
    { cc_number=65,  note=-1, name="Unison" },
    { cc_number=112, note=-1, name="Unison Detune" },
    { cc_number=66,  note=-1, name="Bender Target" },
    { cc_number=106, note=-1, name="Glide" },
    { cc_number=73,  note=-1, name="Arp Mode" },
}

return {
    name = "Behringer Pro-800",
    cc   = cc,
}

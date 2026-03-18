-- models/moog/sub37.lua
-- Moog Sub 37 / Subsequent 37 (2-oscillator paraphonic analog synthesizer)
-- Moog ladder filter (6/12/18/24 dB), 2 DAHDSR envelopes, 2 LFOs, 2 mod buses,
-- arpeggiator/sequencer, duo mode. 14-bit CC supported (LSB = MSB + 32).
-- Full NRPN table (150+ params) available in manual pp.56-59 but could not be
-- fully extracted; CC coverage below is comprehensive for panel control.
-- SysEx: F0 04 ... F7 (Moog mfr ID 04).

local cc = {
    -- Standard / global
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=7,   note=-1, name="Master Volume" },
    { cc_number=64,  note=-1, name="Hold Pedal" },
    -- Oscillator 1
    { cc_number=74,  note=-1, name="Osc 1 Octave" },
    { cc_number=9,   note=-1, name="Osc 1 Wave" },
    { cc_number=114, note=-1, name="Osc 1 Level" },
    { cc_number=115, note=-1, name="Osc 1 Sub Level" },
    -- Oscillator 2
    { cc_number=75,  note=-1, name="Osc 2 Octave" },
    { cc_number=14,  note=-1, name="Osc 2 Wave" },
    { cc_number=12,  note=-1, name="Osc 2 Frequency" },
    { cc_number=13,  note=-1, name="Osc 2 Beat Freq" },
    { cc_number=116, note=-1, name="Osc 2 Level" },
    { cc_number=77,  note=-1, name="Osc 2 Hard Sync" },
    { cc_number=110, note=-1, name="Osc Duo Mode" },
    { cc_number=81,  note=-1, name="Osc KB Reset" },
    -- Mixer
    { cc_number=117, note=-1, name="Noise Level" },
    { cc_number=118, note=-1, name="Feedback/Ext Level" },
    -- Glide
    { cc_number=5,   note=-1, name="Glide Time" },
    { cc_number=65,  note=-1, name="Glide On/Off" },
    { cc_number=85,  note=-1, name="Glide Type" },
    { cc_number=94,  note=-1, name="Glide Legato" },
    { cc_number=102, note=-1, name="Glide Dest" },
    -- Filter
    { cc_number=19,  note=-1, name="Filter Cutoff" },
    { cc_number=21,  note=-1, name="Filter Resonance" },
    { cc_number=18,  note=-1, name="Filter MultiDrive" },
    { cc_number=22,  note=-1, name="Filter KB Amount" },
    { cc_number=109, note=-1, name="Filter Slope" },
    -- Filter envelope (DAHDSR)
    { cc_number=27,  note=-1, name="Filter EG Amount" },
    { cc_number=23,  note=-1, name="Filter EG Attack" },
    { cc_number=103, note=-1, name="Filter EG Delay" },
    { cc_number=105, note=-1, name="Filter EG Hold" },
    { cc_number=24,  note=-1, name="Filter EG Decay" },
    { cc_number=25,  note=-1, name="Filter EG Sustain" },
    { cc_number=26,  note=-1, name="Filter EG Release" },
    { cc_number=79,  note=-1, name="Filter EG KB Amount" },
    { cc_number=82,  note=-1, name="Filter EG Reset" },
    { cc_number=86,  note=-1, name="Filter EG Velocity" },
    { cc_number=112, note=-1, name="Filter EG Multi Trig" },
    -- Amp envelope (DAHDSR)
    { cc_number=28,  note=-1, name="Amp EG Attack" },
    { cc_number=104, note=-1, name="Amp EG Delay" },
    { cc_number=106, note=-1, name="Amp EG Hold" },
    { cc_number=29,  note=-1, name="Amp EG Decay" },
    { cc_number=30,  note=-1, name="Amp EG Sustain" },
    { cc_number=31,  note=-1, name="Amp EG Release" },
    { cc_number=80,  note=-1, name="Amp EG KB Amount" },
    { cc_number=83,  note=-1, name="Amp EG Reset" },
    { cc_number=87,  note=-1, name="Amp EG Velocity" },
    { cc_number=113, note=-1, name="Amp EG Multi Trig" },
    -- LFO 1
    { cc_number=3,   note=-1, name="LFO 1 Rate" },
    { cc_number=76,  note=-1, name="LFO 1 Range" },
    { cc_number=93,  note=-1, name="LFO 1 KB Reset" },
    -- LFO 2
    { cc_number=8,   note=-1, name="LFO 2 Rate" },
    { cc_number=78,  note=-1, name="LFO 2 Range" },
    { cc_number=95,  note=-1, name="LFO 2 KB Reset" },
    -- Mod Bus 1
    { cc_number=71,  note=-1, name="Mod 1 Source" },
    { cc_number=4,   note=-1, name="Mod 1 Pitch Amount" },
    { cc_number=11,  note=-1, name="Mod 1 Filter Amount" },
    { cc_number=20,  note=-1, name="Mod 1 PGM Dest Amt" },
    { cc_number=70,  note=-1, name="Mod 1 Osc Select" },
    { cc_number=91,  note=-1, name="Mod 1 Destination" },
    -- Mod Bus 2
    { cc_number=72,  note=-1, name="Mod 2 Source" },
    { cc_number=15,  note=-1, name="Mod 2 Pitch Amount" },
    { cc_number=16,  note=-1, name="Mod 2 Filter Amount" },
    { cc_number=17,  note=-1, name="Mod 2 PGM Dest Amt" },
    { cc_number=88,  note=-1, name="Mod 2 Osc Select" },
    { cc_number=92,  note=-1, name="Mod 2 Destination" },
    -- Arpeggiator
    { cc_number=73,  note=-1, name="Arp On/Off" },
    { cc_number=69,  note=-1, name="Arp Latch" },
    -- Performance
    { cc_number=89,  note=-1, name="KB Octave" },
    { cc_number=119, note=-1, name="KB Transpose" },
    { cc_number=111, note=-1, name="KB Control Lo/Hi" },
    { cc_number=107, note=-1, name="Pitch Bend Up" },
    { cc_number=108, note=-1, name="Pitch Bend Down" },
}

return {
    name = "Moog Sub 37",
    cc   = cc,
}

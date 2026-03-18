-- models/roland/jp08.lua
-- Roland JP-08 (Boutique Jupiter-8 sound module)
-- 4-voice polyphonic synth with dual VCOs, multimode filter, dual envelopes.
-- No SysEx support; all parameters via CC only.
-- In Dual mode, lower-part CCs transmit/receive on channel + 1.

local cc = {
    -- LFO
    { cc_number=3,   note=-1, name="LFO Rate" },
    { cc_number=9,   note=-1, name="LFO Delay Time" },
    { cc_number=12,  note=-1, name="LFO Wave" },           -- 0=Sin,1=Tri,2=Saw,3=Pulse,4=Sqr,5=Noise
    -- VCO Modulation
    { cc_number=13,  note=-1, name="VCO Mod LFO" },
    { cc_number=14,  note=-1, name="VCO Mod Env" },
    { cc_number=15,  note=-1, name="VCO Mod Freq" },       -- 0=VCO2,1=VCO1+2,2=VCO1
    { cc_number=16,  note=-1, name="VCO Mod PWM" },
    { cc_number=17,  note=-1, name="VCO Mod PWM Src" },    -- 0=Env-1,1=Manual,2=LFO
    -- VCO-1
    { cc_number=18,  note=-1, name="VCO-1 Cross Mod" },
    { cc_number=19,  note=-1, name="VCO-1 Range" },        -- 0=64',1=32',2=16',3=8',4=4',5=2'
    { cc_number=20,  note=-1, name="VCO-1 Wave" },         -- 0=Sin,1=Tri,2=Saw,3=Pulse,4=Sqr,5=Noise
    -- VCO-2
    { cc_number=21,  note=-1, name="VCO-2 Sync" },         -- 0=Off,1=On
    { cc_number=22,  note=-1, name="VCO-2 Range" },        -- 0=64',10=32',30=16',50=8',70=4',7F=2'
    { cc_number=23,  note=-1, name="VCO-2 Tune" },         -- 00=-,40=center,7F=+
    { cc_number=24,  note=-1, name="VCO-2 Wave" },         -- 0=Sin,1=Saw,2=Pulse,3=Lo-Sin,4=Lo-Saw,5=Lo-Pulse
    { cc_number=25,  note=-1, name="Source Mix" },          -- 00=VCO1,40=VCO1+2,7F=VCO2
    -- Filter
    { cc_number=26,  note=-1, name="HPF Cutoff" },
    { cc_number=27,  note=-1, name="VCF Slope" },          -- 0=-24dB,1=-12dB
    { cc_number=74,  note=-1, name="VCF Cutoff" },
    { cc_number=71,  note=-1, name="VCF Resonance" },
    { cc_number=28,  note=-1, name="VCF Env Mod" },
    { cc_number=29,  note=-1, name="VCF Env Mod Src" },    -- 0=Env2,1=Env1
    { cc_number=30,  note=-1, name="VCF LFO Mod" },
    { cc_number=31,  note=-1, name="VCF Key Follow" },
    -- VCA
    { cc_number=35,  note=-1, name="VCA Level" },
    { cc_number=46,  note=-1, name="VCA LFO Mod" },        -- 0-3
    -- Envelope 1
    { cc_number=47,  note=-1, name="Env-1 Attack" },
    { cc_number=52,  note=-1, name="Env-1 Decay" },
    { cc_number=53,  note=-1, name="Env-1 Sustain" },
    { cc_number=54,  note=-1, name="Env-1 Release" },
    { cc_number=55,  note=-1, name="Env-1 Polarity" },     -- 0=Inv,1=Normal
    -- Envelope 2
    { cc_number=73,  note=-1, name="Env-2 Attack" },
    { cc_number=75,  note=-1, name="Env-2 Decay" },
    { cc_number=56,  note=-1, name="Env-2 Sustain" },
    { cc_number=72,  note=-1, name="Env-2 Release" },
    { cc_number=57,  note=-1, name="Env-2 Key Follow" },   -- 0=Off,1=Env1,2=Env2,3=Env1+2
    -- Effects
    { cc_number=93,  note=-1, name="Chorus" },              -- 0=Off,1-3=Type
    { cc_number=82,  note=-1, name="Delay Time" },          -- 0-15
    { cc_number=83,  note=-1, name="Delay Feedback" },      -- 0=Off,1-15=Level
    { cc_number=91,  note=-1, name="Delay Level" },         -- 0=Off,1-15=Level
    -- Performance
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=64,  note=-1, name="Hold" },
    { cc_number=65,  note=-1, name="Portamento Switch" },   -- 0=Off,7F=On
    { cc_number=80,  note=-1, name="Dual Switch" },
    { cc_number=81,  note=-1, name="Current Part" },
    { cc_number=86,  note=-1, name="Assign Mode" },         -- 0=Poly,2=Solo,3=Unison
    { cc_number=87,  note=-1, name="Bend Range" },          -- 0=Off,1-12=semi,18=2oct
}

return {
    name = "Roland JP-08",
    cc   = cc,
}

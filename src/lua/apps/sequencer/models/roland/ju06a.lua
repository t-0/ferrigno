-- models/roland/ju06a.lua
-- Roland JU-06A (Boutique Juno-106/Juno-60 sound module)
-- SysEx: F0 41 <dev> 00 00 00 1D 12 03 00 <addr_m> <addr_l> {vh} {vl} {cs} F7
-- Values are 8-bit, split across two SysEx bytes (high nibble, low nibble).

local function tpl(addr_m, addr_l)
    return string.format("F0 41 10 00 00 00 1D 12 03 00 %02X %02X {vh} {vl} {cs} F7", addr_m, addr_l)
end

-- ── SysEx parameters ────────────────────────────────────────────────────────

local sysex = {}
local idx = 0

local function add(addr_m, addr_l, name, max, def)
    sysex[#sysex+1] = { param_index=idx, name=name,
        template=tpl(addr_m, addr_l), min_val=0, max_val=max, default_val=def or 0 }
    idx = idx + 1
end

-- LFO
add(0x06, 0x00, "LFO Rate",         255, 0)
add(0x06, 0x02, "LFO Delay Time",   255, 0)

-- DCO
add(0x07, 0x00, "DCO Range",          2, 1)   -- 0=4', 1=8', 2=16'
add(0x07, 0x02, "DCO LFO Depth",    255, 0)
add(0x07, 0x04, "DCO PWM",          255, 0)
add(0x07, 0x06, "DCO PWM Src",        1, 0)   -- 0=Manual, 1=LFO
add(0x07, 0x08, "DCO Wave Pulse",     1, 1)   -- 0=off, 1=on
add(0x07, 0x0A, "DCO Wave Saw",       1, 1)   -- 0=off, 1=on
add(0x07, 0x0C, "DCO Sub Level",    255, 0)
add(0x07, 0x0E, "DCO Noise Level",  255, 0)

-- VCF
add(0x08, 0x00, "HPF Freq",         255, 0)
add(0x08, 0x02, "VCF Freq",         255, 255)
add(0x08, 0x04, "VCF Resonance",    255, 0)
add(0x08, 0x06, "VCF ENV Polarity",   1, 0)   -- 0=positive, 1=negative
add(0x08, 0x08, "VCF ENV Depth",    255, 0)
add(0x08, 0x0A, "VCF LFO Depth",    255, 0)
add(0x08, 0x0C, "VCF Keytrack",     255, 0)

-- VCA
add(0x09, 0x00, "VCA Mode",           1, 1)   -- 0=Gate, 1=ENV
add(0x09, 0x02, "VCA Level",        255, 200)

-- ENV
add(0x0A, 0x00, "ENV Attack",       255, 0)
add(0x0A, 0x02, "ENV Decay",        255, 0)
add(0x0A, 0x04, "ENV Sustain",      255, 255)
add(0x0A, 0x06, "ENV Release",      255, 0)

-- Effects
add(0x10, 0x00, "Chorus",             3, 0)   -- 0=Off, 1=I, 2=II, 3=I+II
add(0x10, 0x02, "Delay Level",       15, 0)
add(0x10, 0x04, "Delay Time",        15, 0)
add(0x10, 0x06, "Delay Feedback",    15, 0)

-- Controller
add(0x11, 0x08, "Bend Range",       255, 24)

-- ── CC names ────────────────────────────────────────────────────────────────

local cc = {
    -- LFO
    { cc_number=3,   note=-1, name="LFO Rate" },
    { cc_number=9,   note=-1, name="LFO Delay" },
    { cc_number=15,  note=-1, name="LFO Target" },
    { cc_number=29,  note=-1, name="LFO Wave" },
    { cc_number=30,  note=-1, name="LFO Trig" },
    -- DCO
    { cc_number=12,  note=-1, name="DCO Range" },
    { cc_number=13,  note=-1, name="DCO LFO Depth" },
    { cc_number=14,  note=-1, name="DCO PWM" },
    { cc_number=16,  note=-1, name="DCO SQR Switch" },
    { cc_number=17,  note=-1, name="DCO SAW Switch" },
    { cc_number=18,  note=-1, name="DCO SUB Level" },
    { cc_number=28,  note=-1, name="DCO SUB Switch" },
    { cc_number=19,  note=-1, name="DCO Noise Level" },
    -- VCF
    { cc_number=20,  note=-1, name="HPF Freq" },
    { cc_number=74,  note=-1, name="VCF Freq" },
    { cc_number=71,  note=-1, name="VCF Resonance" },
    { cc_number=21,  note=-1, name="VCF ENV Polarity" },
    { cc_number=22,  note=-1, name="VCF ENV Depth" },
    { cc_number=23,  note=-1, name="VCF LFO Depth" },
    { cc_number=24,  note=-1, name="VCF Keytrack" },
    -- VCA
    { cc_number=25,  note=-1, name="VCA ENV Mode" },
    { cc_number=26,  note=-1, name="VCA Level" },
    -- ENV
    { cc_number=73,  note=-1, name="ENV Attack" },
    { cc_number=75,  note=-1, name="ENV Decay" },
    { cc_number=27,  note=-1, name="ENV Sustain" },
    { cc_number=72,  note=-1, name="ENV Release" },
    -- Effects
    { cc_number=93,  note=-1, name="Chorus Type" },
    { cc_number=82,  note=-1, name="Delay Time" },
    { cc_number=83,  note=-1, name="Delay Feedback" },
    { cc_number=89,  note=-1, name="Delay Switch" },
    { cc_number=91,  note=-1, name="Delay Level" },
    -- Performance
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=64,  note=-1, name="Sustain Pedal" },
    { cc_number=65,  note=-1, name="Portamento Switch" },
    { cc_number=86,  note=-1, name="Poly/Mono/Unison" },
    { cc_number=87,  note=-1, name="Bend Range" },
    { cc_number=88,  note=-1, name="Clock Sync" },
}

return {
    name  = "Roland JU-06A",
    sysex = sysex,
    cc    = cc,
}

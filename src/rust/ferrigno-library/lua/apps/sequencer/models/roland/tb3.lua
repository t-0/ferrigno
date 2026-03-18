-- models/roland/tb3.lua
-- Roland TB-3 (AIRA Touch Bassline)
-- Virtual analog bass synth with touch pad, effects, and pattern sequencer.
-- SysEx: F0 41 10 00 00 7B 12 <addr×4> <data> {cs} F7
-- Based on unofficial MIDI implementation by mugenkidou.

-- ── SysEx helpers ─────────────────────────────────────────────────────────────

local function tpl1(a, b, c, d)
    return string.format("F0 41 10 00 00 7B 12 %02X %02X %02X %02X {v} {cs} F7", a, b, c, d)
end

local function tpl2(a, b, c, d)
    return string.format("F0 41 10 00 00 7B 12 %02X %02X %02X %02X {vh} {vl} {cs} F7", a, b, c, d)
end

-- ── SysEx parameters ──────────────────────────────────────────────────────────

local sysex = {}
local idx = 0

local function add1(a, b, c, d, name, max, def)
    sysex[#sysex+1] = { param_index=idx, name=name,
        template=tpl1(a, b, c, d), min_val=0, max_val=max, default_val=def or 0 }
    idx = idx + 1
end

local function add2(a, b, c, d, name, max, def)
    sysex[#sysex+1] = { param_index=idx, name=name,
        template=tpl2(a, b, c, d), min_val=0, max_val=max, default_val=def or 0 }
    idx = idx + 1
end

-- LFO
add1(0x10, 0x00, 0x00, 0x00, "LFO Rate",          127, 0)
add1(0x10, 0x00, 0x00, 0x01, "LFO Delay",          127, 0)
add1(0x10, 0x00, 0x00, 0x02, "LFO Wave SAW",       127, 0)
add1(0x10, 0x00, 0x00, 0x03, "LFO Wave SQR",       127, 0)
add1(0x10, 0x00, 0x00, 0x04, "LFO Wave TRI",       127, 0)
add1(0x10, 0x00, 0x00, 0x05, "LFO Wave SIN",       127, 0)
add1(0x10, 0x00, 0x00, 0x06, "CV Offset (LFO)",    127, 0)
add1(0x10, 0x00, 0x00, 0x07, "LFO Wave S&H",       127, 0)
add1(0x10, 0x00, 0x00, 0x08, "LFO Depth VCO",      127, 64)  -- signed: -64..+63
add1(0x10, 0x00, 0x00, 0x09, "LFO Depth VCF",      127, 64)
add1(0x10, 0x00, 0x00, 0x0A, "LFO Depth VCA",      127, 64)
add1(0x10, 0x00, 0x00, 0x0B, "BPM Sync",             1, 0)   -- 0=Off, 1=On
add1(0x10, 0x00, 0x00, 0x0C, "LFO Retrigger",        1, 0)   -- 0=Off, 1=On

-- CV Offset (pitch per oscillator, 16-bit signed -128..+127)
add2(0x10, 0x00, 0x02, 0x00, "CV Offset SQR Pitch",  255, 128)
add2(0x10, 0x00, 0x02, 0x02, "CV Offset SAW Pitch",  255, 128)
add2(0x10, 0x00, 0x02, 0x04, "CV Offset Ring Pitch", 255, 128)

-- Cross Modulation
add1(0x10, 0x00, 0x04, 0x00, "XMod SQR>SAW",       127, 0)
add1(0x10, 0x00, 0x04, 0x02, "XMod SAW>SAW",       127, 0)
add1(0x10, 0x00, 0x04, 0x03, "XMod White>SAW",     127, 0)
add1(0x10, 0x00, 0x04, 0x04, "XMod Pink>SAW",      127, 0)
add1(0x10, 0x00, 0x04, 0x05, "XMod SQR>SQR",       127, 0)
add1(0x10, 0x00, 0x04, 0x07, "XMod SAW>SQR",       127, 0)
add1(0x10, 0x00, 0x04, 0x08, "XMod White>SQR",     127, 0)
add1(0x10, 0x00, 0x04, 0x09, "XMod Pink>SQR",      127, 0)

-- Ring Modulation
add1(0x10, 0x00, 0x06, 0x00, "Ring Mod SAW",        127, 0)
add1(0x10, 0x00, 0x06, 0x03, "Ring Mod Ring",       127, 0)
add1(0x10, 0x00, 0x06, 0x04, "Ring Mod White",      127, 0)
add1(0x10, 0x00, 0x06, 0x05, "Ring Mod Pink",       127, 0)

-- VCO
add1(0x10, 0x00, 0x08, 0x00, "VCO SAW Level",      127, 0)
add1(0x10, 0x00, 0x08, 0x01, "VCO SQR Level",      127, 0)
add1(0x10, 0x00, 0x08, 0x04, "VCO SIN Level",      127, 0)
add1(0x10, 0x00, 0x08, 0x05, "VCO White Noise",    127, 0)
add1(0x10, 0x00, 0x08, 0x06, "VCO Pink Noise",     127, 0)
add1(0x10, 0x00, 0x08, 0x07, "VCO Ring Level",     127, 0)
add1(0x10, 0x00, 0x08, 0x08, "VCO SAW Switch",       1, 1)   -- 0=Off, 1=On
add1(0x10, 0x00, 0x08, 0x09, "VCO SQR Switch",       1, 0)
add1(0x10, 0x00, 0x08, 0x0A, "VCO SIN Switch",       1, 0)
add1(0x10, 0x00, 0x08, 0x0B, "VCO White Noise SW",   1, 0)
add1(0x10, 0x00, 0x08, 0x0C, "VCO Pink Noise SW",    1, 0)
add1(0x10, 0x00, 0x08, 0x0D, "VCO Ring Switch",      1, 0)

-- VCF
add2(0x10, 0x00, 0x0A, 0x00, "VCF Cutoff",         255, 255)
add2(0x10, 0x00, 0x0A, 0x02, "VCF Resonance",      255, 0)
add2(0x10, 0x00, 0x0A, 0x04, "VCF Env Depth",      255, 0)
add1(0x10, 0x00, 0x0A, 0x06, "VCF Env Attack",     127, 0)
add1(0x10, 0x00, 0x0A, 0x07, "VCF Env Decay",      127, 0)
add1(0x10, 0x00, 0x0A, 0x08, "VCF Env Sustain",    127, 0)
add1(0x10, 0x00, 0x0A, 0x09, "VCF Env Release",    127, 0)
add1(0x10, 0x00, 0x0A, 0x0A, "VCF Key Follow",     127, 0)

-- VCA
add1(0x10, 0x00, 0x0C, 0x00, "VCA Env Attack",     127, 0)
add1(0x10, 0x00, 0x0C, 0x01, "VCA Env Decay",      127, 0)
add1(0x10, 0x00, 0x0C, 0x02, "VCA Env Sustain",    127, 127)
add1(0x10, 0x00, 0x0C, 0x03, "VCA Env Release",    127, 0)
add1(0x10, 0x00, 0x0C, 0x04, "Master Volume",      127, 100)

-- Distortion
add1(0x10, 0x00, 0x0E, 0x00, "Distortion Switch",    1, 0)   -- 0=Off, 1=On
add1(0x10, 0x00, 0x0E, 0x01, "Distortion Type",      24, 0)  -- 25 types
add1(0x10, 0x00, 0x0E, 0x02, "Drive",               120, 0)
add1(0x10, 0x00, 0x0E, 0x03, "Bottom",              100, 50)  -- signed: -50..+50
add1(0x10, 0x00, 0x0E, 0x04, "Tone",                100, 50)  -- signed: -50..+50
add1(0x10, 0x00, 0x0E, 0x05, "Dist Effect Level",   100, 0)
add1(0x10, 0x00, 0x0E, 0x06, "Dist Dry Level",      100, 100)
add1(0x10, 0x00, 0x0E, 0x07, "Dist Color",            1, 0)

-- Controller
add1(0x10, 0x00, 0x14, 0x00, "Portamento Switch",     1, 0)  -- 0=Off, 1=On
add1(0x10, 0x00, 0x14, 0x01, "Portamento Time",     127, 0)
add1(0x10, 0x00, 0x14, 0x02, "Portamento Mode",       1, 0)  -- 0=Legato, 1=Always
add1(0x10, 0x00, 0x14, 0x03, "Bend Range",           17, 2)
add2(0x10, 0x00, 0x14, 0x0E, "Accent",              255, 0)

-- ── CC names ──────────────────────────────────────────────────────────────────

local cc = {
    { cc_number=1,   note=-1, name="Pad Z (Mod)" },
    { cc_number=11,  note=-1, name="XY Pad Y" },
    { cc_number=12,  note=-1, name="Env Mod (Pad X)" },
    { cc_number=13,  note=-1, name="Env Mod (Pad Y)" },
    { cc_number=16,  note=-1, name="Accent" },
    { cc_number=17,  note=-1, name="Effect" },
    { cc_number=68,  note=-1, name="Scatter Type" },
    { cc_number=69,  note=-1, name="Scatter Depth" },
    { cc_number=71,  note=-1, name="Resonance" },
    { cc_number=74,  note=-1, name="Cutoff" },
    { cc_number=102, note=-1, name="Pattern Slide" },
    { cc_number=103, note=-1, name="Pattern Accent" },
    { cc_number=104, note=-1, name="Tuning" },
}

return {
    name  = "Roland TB-3",
    sysex = sysex,
    cc    = cc,
}

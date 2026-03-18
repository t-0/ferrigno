-- models/roland/d20.lua
-- Roland D-20 (Multi-Timbral Linear Synthesizer with Sequencer)
-- Same LA synthesis engine as D-10/D-110/MT-32 (model ID 16H).
-- SysEx: F0 41 <dev> 16 12 <addr_h> <addr_m> <addr_l> <data> {cs} F7
-- 6 multitimbral parts + rhythm, 4 partials per tone.

local sysex = {}
local idx = 0

local function add(addr_h, addr_m, addr_l, name, max, def)
    sysex[#sysex+1] = {
        param_index = idx,
        name        = name,
        template    = string.format("F0 41 10 16 12 %02X %02X %02X {v} {cs} F7",
                                    addr_h, addr_m, addr_l),
        min_val     = 0,
        max_val     = max,
        default_val = def or 0,
    }
    idx = idx + 1
end

-- ── System (10 00 xx) ─────────────────────────────────────────────────────────
add(0x10, 0x00, 0x16, "Master Volume",      100, 100)
add(0x10, 0x00, 0x01, "Reverb Mode",          3, 0)   -- 0=Room,1=Hall,2=Plate,3=Tap Delay
add(0x10, 0x00, 0x02, "Reverb Time",          7, 5)
add(0x10, 0x00, 0x03, "Reverb Level",         7, 3)

-- ── Part Mixer (03 xx yy) ────────────────────────────────────────────────────
-- Parts 1-6 at offsets 00,10,20,30,40,50 from 03 00 00
for p = 1, 6 do
    local off = (p - 1) * 0x10
    add(0x03, off, 0x06, string.format("Part %d Volume", p),  100, 100)
    add(0x03, off, 0x07, string.format("Part %d Pan", p),      14, 7)  -- 0=L..7=C..14=R
    add(0x03, off, 0x08, string.format("Part %d Reverb", p),    1, 1)  -- 0=Off,1=On
end

-- ── Tone Temporary — Common (04 00 xx) ────────────────────────────────────────
add(0x04, 0x00, 0x0A, "Structure 1/2",       12, 0)
add(0x04, 0x00, 0x0B, "Structure 3/4",       12, 0)

-- ── Tone Temporary — Partial 1 (base 04 00 0E) ───────────────────────────────
-- WG (Wave Generator)
add(0x04, 0x00, 0x0E, "P1 WG Pitch Coarse",  96, 36)  -- 0-96, middle=36
add(0x04, 0x00, 0x0F, "P1 WG Pitch Fine",   100, 50)  -- 0-100, center=50
add(0x04, 0x00, 0x12, "P1 WG Waveform",       1, 1)   -- 0=SQR,1=SAW
add(0x04, 0x00, 0x13, "P1 WG PCM Wave",     127, 0)
add(0x04, 0x00, 0x14, "P1 WG Pulse Width",  100, 50)
-- TVF (Time-Variant Filter)
add(0x04, 0x00, 0x25, "P1 TVF Cutoff",      100, 50)
add(0x04, 0x00, 0x26, "P1 TVF Resonance",    30, 0)
add(0x04, 0x00, 0x27, "P1 TVF Keyfollow",    14, 0)
-- TVA (Time-Variant Amplifier)
add(0x04, 0x00, 0x37, "P1 TVA Level",       100, 100)

-- ── Tone Temporary — Partial 2 (base 04 00 48) ───────────────────────────────
add(0x04, 0x00, 0x48, "P2 WG Pitch Coarse",  96, 36)
add(0x04, 0x00, 0x49, "P2 WG Pitch Fine",   100, 50)
add(0x04, 0x00, 0x4C, "P2 WG Waveform",       1, 1)
add(0x04, 0x00, 0x4D, "P2 WG PCM Wave",     127, 0)
add(0x04, 0x00, 0x4E, "P2 WG Pulse Width",  100, 50)
add(0x04, 0x00, 0x5F, "P2 TVF Cutoff",      100, 50)
add(0x04, 0x00, 0x60, "P2 TVF Resonance",    30, 0)
add(0x04, 0x00, 0x61, "P2 TVF Keyfollow",    14, 0)
add(0x04, 0x00, 0x71, "P2 TVA Level",       100, 100)

-- ── CC names ──────────────────────────────────────────────────────────────────

local cc = {
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=64,  note=-1, name="Sustain" },
    { cc_number=91,  note=-1, name="Reverb Send" },
}

return {
    name  = "Roland D-20",
    sysex = sysex,
    cc    = cc,
}

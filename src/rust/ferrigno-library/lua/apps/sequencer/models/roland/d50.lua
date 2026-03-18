-- models/roland/d50.lua
-- Roland D-50 / D-550 / D-05
-- SysEx: F0 41 <dev> 14 12 <addr_h> <addr_m> <addr_l> <data> <checksum> F7
-- Temporary patch area address map:
--   Upper Partial 1:  00 00 00   Upper Partial 2:  00 00 40
--   Upper Common:     00 01 00   Lower Partial 1:  00 01 40
--   Lower Partial 2:  00 02 00   Lower Common:     00 02 40
--   Patch:            00 03 00

local function tpl(addr_m, addr_l)
    return string.format("F0 41 10 14 12 00 %02X %02X {v} {cs} F7", addr_m, addr_l)
end

-- Partial parameters (shared structure for all 4 partials)
local partial = {
    -- WG (Wave Generator)
    { off=0x00, name="WG Pitch Coarse",     min=0, max=72,  def=36 },
    { off=0x01, name="WG Pitch Fine",       min=0, max=100, def=50 },
    { off=0x02, name="WG Pitch KF",         min=0, max=16,  def=11 },
    { off=0x03, name="WG Mod LFO Mode",     min=0, max=3,   def=0 },
    { off=0x04, name="WG Mod P-ENV Mode",   min=0, max=2,   def=0 },
    { off=0x05, name="WG Mod Bender",       min=0, max=2,   def=2 },
    { off=0x06, name="WG Waveform",         min=0, max=1,   def=1 },
    { off=0x07, name="WG PCM Wave No",      min=0, max=99,  def=0 },
    { off=0x08, name="WG Pulse Width",      min=0, max=100, def=50 },
    { off=0x09, name="WG PW Vel Range",     min=0, max=14,  def=7 },
    { off=0x0A, name="WG PW LFO Sel",       min=0, max=5,   def=0 },
    { off=0x0B, name="WG PW LFO Depth",     min=0, max=100, def=0 },
    { off=0x0C, name="WG PW AT Range",      min=0, max=14,  def=7 },
    -- TVF (Time-Variant Filter)
    { off=0x0D, name="TVF Cutoff",          min=0, max=100, def=50 },
    { off=0x0E, name="TVF Resonance",       min=0, max=30,  def=0 },
    { off=0x0F, name="TVF Keyfollow",       min=0, max=14,  def=7 },
    { off=0x10, name="TVF Bias Pt/Dir",     min=0, max=127, def=64 },
    { off=0x11, name="TVF Bias Level",      min=0, max=14,  def=7 },
    { off=0x12, name="TVF ENV Depth",       min=0, max=100, def=50 },
    { off=0x13, name="TVF ENV Vel Range",   min=0, max=100, def=50 },
    { off=0x14, name="TVF ENV Depth KF",    min=0, max=4,   def=0 },
    { off=0x15, name="TVF ENV Time KF",     min=0, max=4,   def=0 },
    { off=0x16, name="TVF ENV T1",          min=0, max=100, def=50 },
    { off=0x17, name="TVF ENV T2",          min=0, max=100, def=50 },
    { off=0x18, name="TVF ENV T3",          min=0, max=100, def=50 },
    { off=0x19, name="TVF ENV T4",          min=0, max=100, def=50 },
    { off=0x1A, name="TVF ENV T5",          min=0, max=100, def=0 },
    { off=0x1B, name="TVF ENV L1",          min=0, max=100, def=100 },
    { off=0x1C, name="TVF ENV L2",          min=0, max=100, def=100 },
    { off=0x1D, name="TVF ENV L3",          min=0, max=100, def=100 },
    { off=0x1E, name="TVF ENV Sustain",     min=0, max=100, def=100 },
    { off=0x1F, name="TVF ENV End Lv",      min=0, max=1,   def=0 },
    { off=0x20, name="TVF Mod LFO Sel",     min=0, max=5,   def=0 },
    { off=0x21, name="TVF Mod LFO Depth",   min=0, max=100, def=0 },
    { off=0x22, name="TVF Mod AT Range",    min=0, max=14,  def=7 },
    -- TVA (Time-Variant Amplifier)
    { off=0x23, name="TVA Level",           min=0, max=100, def=100 },
    { off=0x24, name="TVA Vel Range",       min=0, max=100, def=50 },
    { off=0x25, name="TVA Bias Pt/Dir",     min=0, max=127, def=64 },
    { off=0x26, name="TVA Bias Level",      min=0, max=12,  def=0 },
    { off=0x27, name="TVA ENV T1",          min=0, max=100, def=50 },
    { off=0x28, name="TVA ENV T2",          min=0, max=100, def=50 },
    { off=0x29, name="TVA ENV T3",          min=0, max=100, def=50 },
    { off=0x2A, name="TVA ENV T4",          min=0, max=100, def=50 },
    { off=0x2B, name="TVA ENV T5",          min=0, max=100, def=0 },
    { off=0x2C, name="TVA ENV L1",          min=0, max=100, def=100 },
    { off=0x2D, name="TVA ENV L2",          min=0, max=100, def=100 },
    { off=0x2E, name="TVA ENV L3",          min=0, max=100, def=100 },
    { off=0x2F, name="TVA ENV Sustain",     min=0, max=100, def=100 },
    { off=0x30, name="TVA ENV End Lv",      min=0, max=1,   def=0 },
    { off=0x31, name="TVA ENV Vel Follow",  min=0, max=4,   def=0 },
    { off=0x32, name="TVA ENV Time KF",     min=0, max=4,   def=0 },
    { off=0x33, name="TVA Mod LFO Sel",     min=0, max=5,   def=0 },
    { off=0x34, name="TVA Mod LFO Depth",   min=0, max=100, def=0 },
    { off=0x35, name="TVA Mod AT Range",    min=0, max=14,  def=7 },
}

-- Tone common parameters (shared for Upper/Lower)
local common = {
    { off=0x0A, name="Structure",           min=0, max=6,   def=0 },
    { off=0x0B, name="P-ENV Vel Range",     min=0, max=2,   def=0 },
    { off=0x0C, name="P-ENV Time KF",       min=0, max=4,   def=0 },
    { off=0x0D, name="P-ENV Time 1",        min=0, max=50,  def=0 },
    { off=0x0E, name="P-ENV Time 2",        min=0, max=50,  def=0 },
    { off=0x0F, name="P-ENV Time 3",        min=0, max=50,  def=0 },
    { off=0x10, name="P-ENV Time 4",        min=0, max=50,  def=0 },
    { off=0x11, name="P-ENV Level 0",       min=0, max=100, def=50 },
    { off=0x12, name="P-ENV Level 1",       min=0, max=100, def=50 },
    { off=0x13, name="P-ENV Level 2",       min=0, max=100, def=50 },
    { off=0x14, name="P-ENV Sustain Lv",    min=0, max=100, def=50 },
    { off=0x15, name="P-ENV End Level",     min=0, max=100, def=50 },
    { off=0x16, name="Pitch Mod LFO Dep",   min=0, max=100, def=0 },
    { off=0x17, name="Pitch Mod Lever",     min=0, max=100, def=0 },
    { off=0x18, name="Pitch Mod AT",        min=0, max=100, def=0 },
    { off=0x19, name="LFO-1 Waveform",     min=0, max=3,   def=0 },
    { off=0x1A, name="LFO-1 Rate",         min=0, max=100, def=50 },
    { off=0x1B, name="LFO-1 Delay",        min=0, max=100, def=0 },
    { off=0x1C, name="LFO-1 Sync",         min=0, max=2,   def=0 },
    { off=0x1D, name="LFO-2 Waveform",     min=0, max=3,   def=0 },
    { off=0x1E, name="LFO-2 Rate",         min=0, max=100, def=50 },
    { off=0x1F, name="LFO-2 Delay",        min=0, max=100, def=0 },
    { off=0x20, name="LFO-2 Sync",         min=0, max=1,   def=0 },
    { off=0x21, name="LFO-3 Waveform",     min=0, max=3,   def=0 },
    { off=0x22, name="LFO-3 Rate",         min=0, max=100, def=50 },
    { off=0x23, name="LFO-3 Delay",        min=0, max=100, def=0 },
    { off=0x24, name="LFO-3 Sync",         min=0, max=1,   def=0 },
    { off=0x25, name="EQ Low Freq",         min=0, max=15,  def=4 },
    { off=0x26, name="EQ Low Gain",         min=0, max=24,  def=12 },
    { off=0x27, name="EQ High Freq",        min=0, max=21,  def=16 },
    { off=0x28, name="EQ High Q",           min=0, max=8,   def=3 },
    { off=0x29, name="EQ High Gain",        min=0, max=24,  def=12 },
    { off=0x2A, name="Chorus Type",         min=0, max=7,   def=0 },
    { off=0x2B, name="Chorus Rate",         min=0, max=100, def=50 },
    { off=0x2C, name="Chorus Depth",        min=0, max=100, def=50 },
    { off=0x2D, name="Chorus Balance",      min=0, max=100, def=0 },
    { off=0x2E, name="Partial Mute",        min=0, max=3,   def=3 },
    { off=0x2F, name="Partial Balance",     min=0, max=100, def=50 },
}

-- Patch parameters
local patch = {
    { off=0x12, name="Key Mode",            min=0, max=8,   def=0 },
    { off=0x13, name="Split Point",         min=0, max=60,  def=30 },
    { off=0x14, name="Portamento Mode",     min=0, max=2,   def=0 },
    { off=0x15, name="Hold Mode",           min=0, max=2,   def=0 },
    { off=0x16, name="U-Tone Key Shift",    min=0, max=48,  def=24 },
    { off=0x17, name="L-Tone Key Shift",    min=0, max=48,  def=24 },
    { off=0x18, name="U-Tone Fine Tune",    min=0, max=100, def=50 },
    { off=0x19, name="L-Tone Fine Tune",    min=0, max=100, def=50 },
    { off=0x1A, name="Bender Range",        min=0, max=12,  def=2 },
    { off=0x1B, name="AT Bend Range",       min=0, max=24,  def=12 },
    { off=0x1C, name="Portamento Time",     min=0, max=100, def=0 },
    { off=0x1D, name="Output Mode",         min=0, max=3,   def=0 },
    { off=0x1E, name="Reverb Type",         min=0, max=31,  def=4 },
    { off=0x1F, name="Reverb Balance",      min=0, max=100, def=50 },
    { off=0x20, name="Total Volume",        min=0, max=100, def=100 },
    { off=0x21, name="Tone Balance",        min=0, max=100, def=50 },
    { off=0x22, name="Chase Mode",          min=0, max=2,   def=0 },
    { off=0x23, name="Chase Level",         min=0, max=100, def=50 },
    { off=0x24, name="Chase Time",          min=0, max=100, def=50 },
}

-- Build complete preset table
local params = {}
local idx = 0

-- Patch (base 00 03 00)
for _, p in ipairs(patch) do
    params[#params+1] = { param_index=idx, name="Pat "..p.name,
        template=tpl(0x03, p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Upper Common (base 00 01 00)
for _, p in ipairs(common) do
    params[#params+1] = { param_index=idx, name="UC "..p.name,
        template=tpl(0x01, p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Lower Common (base 00 02 40)
for _, p in ipairs(common) do
    params[#params+1] = { param_index=idx, name="LC "..p.name,
        template=tpl(0x02, 0x40 + p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Upper Partial 1 (base 00 00 00)
for _, p in ipairs(partial) do
    params[#params+1] = { param_index=idx, name="UP1 "..p.name,
        template=tpl(0x00, p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Upper Partial 2 (base 00 00 40)
for _, p in ipairs(partial) do
    params[#params+1] = { param_index=idx, name="UP2 "..p.name,
        template=tpl(0x00, 0x40 + p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Lower Partial 1 (base 00 01 40)
for _, p in ipairs(partial) do
    params[#params+1] = { param_index=idx, name="LP1 "..p.name,
        template=tpl(0x01, 0x40 + p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

-- Lower Partial 2 (base 00 02 00)
for _, p in ipairs(partial) do
    params[#params+1] = { param_index=idx, name="LP2 "..p.name,
        template=tpl(0x02, p.off), min_val=p.min, max_val=p.max, default_val=p.def }
    idx = idx + 1
end

return { name = "Roland D-50", params = params }

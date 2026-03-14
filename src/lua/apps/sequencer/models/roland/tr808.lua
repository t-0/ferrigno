-- models/roland/tr808.lua
-- Roland TR-808 (and software emulations)
-- GM-compatible drum mapping on channel 10.
-- Demonstrates all model capabilities: drum_map, global CC, per-note CC,
-- per-note NRPN, and global SysEx.

-- ── Drum map (GM-standard note assignments) ─────────────────────────────────

local drum_map = {
    { note=35, name="Kick 2" },
    { note=36, name="Kick 1" },
    { note=37, name="Rimshot" },
    { note=38, name="Snare" },
    { note=39, name="Clap" },
    { note=40, name="Snare 2" },
    { note=41, name="Lo Tom" },
    { note=42, name="Closed HH" },
    { note=43, name="Lo Tom 2" },
    { note=44, name="Pedal HH" },
    { note=45, name="Mid Tom" },
    { note=46, name="Open HH" },
    { note=47, name="Mid Tom 2" },
    { note=48, name="Hi Tom" },
    { note=49, name="Crash" },
    { note=50, name="Hi Tom 2" },
    { note=51, name="Ride" },
    { note=53, name="Ride Bell" },
    { note=56, name="Cowbell" },
    { note=62, name="Hi Conga" },
    { note=63, name="Lo Conga" },
    { note=70, name="Maracas" },
    { note=75, name="Claves" },
}

-- ── Global CC (channel-wide) ────────────────────────────────────────────────

local cc = {
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=91,  note=-1, name="Reverb Send" },
    { cc_number=93,  note=-1, name="Chorus Send" },
}

-- ── Per-note CC (individual drum sound control) ─────────────────────────────
-- Many TR-808 emulations support per-note CC for tuning and decay.

-- Per-note CC: Decay
for _, d in ipairs(drum_map) do
    cc[#cc+1] = { cc_number=71, note=d.note, name=d.name .. " Decay" }
end

-- Per-note CC: Tuning
for _, d in ipairs(drum_map) do
    cc[#cc+1] = { cc_number=74, note=d.note, name=d.name .. " Tune" }
end

-- Per-note CC: Pan
for _, d in ipairs(drum_map) do
    cc[#cc+1] = { cc_number=10, note=d.note, name=d.name .. " Pan" }
end

-- ── Per-note NRPN (fine-grained drum control) ───────────────────────────────
-- GM2/GS per-note NRPNs: MSB=0x18..0x1D, LSB=note number.
-- Here we register the common ones.

local nrpn = {}

-- NRPN 0x1800+note = Pitch Coarse
for _, d in ipairs(drum_map) do
    nrpn[#nrpn+1] = { nrpn_number=0x1800 + d.note, note=d.note, name=d.name .. " Pitch" }
end

-- NRPN 0x1A00+note = Level
for _, d in ipairs(drum_map) do
    nrpn[#nrpn+1] = { nrpn_number=0x1A00 + d.note, note=d.note, name=d.name .. " Level" }
end

-- NRPN 0x1C00+note = Pan
for _, d in ipairs(drum_map) do
    nrpn[#nrpn+1] = { nrpn_number=0x1C00 + d.note, note=d.note, name=d.name .. " Pan" }
end

-- NRPN 0x1D00+note = Reverb Send
for _, d in ipairs(drum_map) do
    nrpn[#nrpn+1] = { nrpn_number=0x1D00 + d.note, note=d.note, name=d.name .. " Reverb" }
end

-- ── Global SysEx (Roland GS drum map select) ────────────────────────────────
-- GS SysEx to switch the drum map on part 10 (address 40 1A 15).

local sysex = {
    { param_index=0, name="Drum Map Number",
      template="F0 41 10 42 12 40 1A 15 {v} {cs} F7",
      min_val=0, max_val=3, default_val=0 },
}

return {
    name     = "Roland TR-808",
    drum_map = drum_map,
    cc       = cc,
    nrpn     = nrpn,
    sysex    = sysex,
}

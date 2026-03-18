-- models/roland/tr8s.lua
-- Roland TR-8S Rhythm Performer
-- 11 instrument drum machine with per-instrument Tune/Decay/Ctrl/Level CCs.

-- ── Drum map (default note assignments, configurable via UTILITY:MIDI) ──────

local drum_map = {
    { note=36, name="BD" },   -- Bass Drum
    { note=37, name="RS" },   -- Rim Shot
    { note=38, name="SD" },   -- Snare Drum
    { note=39, name="HC" },   -- Hand Clap
    { note=42, name="CH" },   -- Closed Hi-Hat
    { note=43, name="LT" },   -- Low Tom
    { note=46, name="OH" },   -- Open Hi-Hat
    { note=47, name="MT" },   -- Mid Tom
    { note=49, name="CC" },   -- Crash Cymbal
    { note=50, name="HT" },   -- High Tom
    { note=51, name="RC" },   -- Ride Cymbal
}

-- ── Instrument CC definitions ───────────────────────────────────────────────
-- Each instrument has Tune, Decay, Level, and Ctrl knobs mapped to CCs.

local instruments = {
    { name="BD", tune=20, decay=23, level=24, ctrl=96,  note=36 },
    { name="SD", tune=25, decay=28, level=29, ctrl=97,  note=38 },
    { name="LT", tune=46, decay=47, level=48, ctrl=102, note=43 },
    { name="MT", tune=49, decay=50, level=51, ctrl=103, note=47 },
    { name="HT", tune=52, decay=53, level=54, ctrl=104, note=50 },
    { name="RS", tune=55, decay=56, level=57, ctrl=105, note=37 },
    { name="HC", tune=58, decay=59, level=60, ctrl=106, note=39 },
    { name="CH", tune=61, decay=62, level=63, ctrl=107, note=42 },
    { name="OH", tune=80, decay=81, level=82, ctrl=108, note=46 },
    { name="CC", tune=83, decay=84, level=85, ctrl=109, note=49 },
    { name="RC", tune=86, decay=87, level=88, ctrl=110, note=51 },
}

-- Build CC list: global CCs + per-note instrument CCs
local cc = {
    -- Global
    { cc_number=9,   note=-1, name="Shuffle" },
    { cc_number=12,  note=-1, name="External IN Level" },
    { cc_number=14,  note=-1, name="Auto Fill IN" },
    { cc_number=15,  note=-1, name="Master FX On" },
    { cc_number=16,  note=-1, name="Delay Level" },
    { cc_number=17,  note=-1, name="Delay Time" },
    { cc_number=18,  note=-1, name="Delay Feedback" },
    { cc_number=19,  note=-1, name="Master FX Ctrl" },
    { cc_number=70,  note=-1, name="Auto Fill IN Trig" },
    { cc_number=71,  note=-1, name="Accent" },
    { cc_number=91,  note=-1, name="Reverb Level" },
}

-- Per-instrument CCs (global, note=-1 for channel-wide control,
-- and per-note for drum-mode instruments)
for _, inst in ipairs(instruments) do
    -- Global CC entries (work on the channel regardless of note)
    cc[#cc+1] = { cc_number=inst.tune,  note=-1, name=inst.name .. " Tune" }
    cc[#cc+1] = { cc_number=inst.decay, note=-1, name=inst.name .. " Decay" }
    cc[#cc+1] = { cc_number=inst.level, note=-1, name=inst.name .. " Level" }
    cc[#cc+1] = { cc_number=inst.ctrl,  note=-1, name=inst.name .. " Ctrl" }
    -- Per-note CC entries (for drum-type instrument display)
    cc[#cc+1] = { cc_number=inst.tune,  note=inst.note, name="Tune" }
    cc[#cc+1] = { cc_number=inst.decay, note=inst.note, name="Decay" }
    cc[#cc+1] = { cc_number=inst.level, note=inst.note, name="Level" }
    cc[#cc+1] = { cc_number=inst.ctrl,  note=inst.note, name="Ctrl" }
end

return {
    name     = "Roland TR-8S",
    drum_map = drum_map,
    cc       = cc,
}

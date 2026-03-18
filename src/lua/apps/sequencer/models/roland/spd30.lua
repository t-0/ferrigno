-- models/roland/spd30.lua
-- Roland OCTAPAD SPD-30 (Digital Percussion Pad)
-- 8 built-in pads + 4 external trigger inputs, 670+ onboard instruments.
-- Note assignments are per-kit and configurable; these are the factory defaults.
-- Default MIDI channel: 10.

-- ── Drum map (factory defaults for Kit 1 + external trigger inputs) ───────────

local drum_map = {
    -- External trigger inputs (GM-standard drum notes)
    { note=35, name="Kick Alt" },       -- KICK input (secondary)
    { note=36, name="Kick" },           -- KICK input (primary)
    { note=26, name="HiHat Alt" },      -- HIHAT input (secondary)
    { note=38, name="Snare" },          -- SNARE input (primary)
    { note=40, name="Snare Alt" },      -- SNARE input (secondary)
    { note=46, name="Open HiHat" },     -- HIHAT input (primary)
    { note=51, name="Ride" },           -- RIDE input (primary)
    { note=53, name="Ride Bell" },      -- RIDE input (secondary)
    -- Built-in pads (Kit 1 factory defaults)
    { note=60, name="Pad 1" },
    { note=61, name="Pad 2" },
    { note=62, name="Pad 3" },
    { note=63, name="Pad 4" },
    { note=64, name="Pad 5" },
    { note=65, name="Pad 6" },
    { note=66, name="Pad 7" },
    { note=67, name="Pad 8" },
}

-- ── CC names ──────────────────────────────────────────────────────────────────
-- The SPD-30 recognizes standard GM CCs for its internal sound engine.
-- FX knob CC numbers are user-configurable via the FX CONTROL screen.

local cc = {
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=91,  note=-1, name="Reverb Send" },
}

return {
    name     = "Roland SPD-30",
    drum_map = drum_map,
    cc       = cc,
}

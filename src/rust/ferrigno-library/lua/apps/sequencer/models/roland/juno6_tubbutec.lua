-- models/roland/juno6_tubbutec.lua
-- Roland Juno-6 with Tubbutec Juno-66 MIDI retrofit (firmware v1.29)
-- Adds MIDI in/out, 2nd filter envelope (ADSR), triangular filter LFO,
-- S/H filter LFO, portamento, Powerarp sequencer, and chord memory.
-- No SysEx; all parameters via CC. Full CC chart in manual appendix §13.1.
-- Manual: https://tubbutec.de/files/Juno-66-User-Manual.pdf

-- ── CC names ──────────────────────────────────────────────────────────────────
-- CC 17 (Filter) confirmed from Tubbutec documentation.
-- TODO: Envelope, LFO, and portamento CCs are documented in the manual appendix

local cc = {
    -- Tubbutec Juno-66 specific
    { cc_number=17,  note=-1, name="Filter" },
    -- Standard MIDI (supported by the mod)
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=5,   note=-1, name="Portamento Time" },
    { cc_number=64,  note=-1, name="Sustain" },
    { cc_number=65,  note=-1, name="Portamento Switch" },
}

return {
    name = "Juno-6 (Tubbutec)",
    cc   = cc,
}

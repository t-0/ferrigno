-- models/yamaha/dsr2000.lua
-- Yamaha DSR-2000 (Portatone series, 1987)
-- 61-key consumer FM keyboard, 4-operator OPZ (YM2414B) chip.
-- 100 voices (60 preset + 40 user), 8-note polyphony, 8 algorithms, 8 waveforms.
-- Minimal MIDI: notes with velocity, pitch bend, mod wheel, program change (0-99).
-- Voice editing is SysEx bulk dump only (80-byte voice format, nibble-encoded);
-- no individual parameter CC or SysEx addressing.
-- SysEx reverse-engineered by Matt Montag: https://www.mattmontag.com/development/unlocking-the-yamaha-dsr-2000

local cc = {
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=64,  note=-1, name="Sustain" },
}

return {
    name = "Yamaha DSR-2000",
    cc   = cc,
}

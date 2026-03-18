-- sequencer/models.lua
-- Instrument model definitions: SysEx parameters, CC names, and NRPN names.
-- Each model lives in its own file under models/<manufacturer>/<model>.lua.
--
-- Model files return:
--   { name = "Display Name",
--     sysex    = { ... },   -- sysex parameter templates (optional)
--     cc       = { ... },   -- CC name definitions (optional)
--     nrpn     = { ... },   -- NRPN name definitions (optional)
--     drum_map = { ... },   -- drum note name mappings (optional)
--   }
--
-- "params" is accepted as an alias for "sysex" for backward compatibility.
--
-- SysEx template format: hex string with placeholders:
--   {v}  = single 7-bit value (0-127)
--   {vh} / {vl} = 14-bit value as MSB / LSB (each 0-127)
--   {cs} = Roland checksum (sum bytes after 0x12 command to before {cs}, mod 128)
--
-- CC entry format:       { cc_number=N, name="...", note=-1 }
--   (note=-1 = global, note=0..127 = per-note for drum instruments)
-- NRPN entry format:     { nrpn_number=N, name="...", note=-1 }
-- Drum map entry format: { note=N, name="..." }

local M = {}

-- Determine the directory containing this file (works both embedded and bare).
local this_dir = (arg and arg[0]) and arg[0]:match("(.*/)") or "./"

-- Model file list (embedded at compile time, no filesystem scan needed).
-- Paths are relative to the models/ directory.
local model_files = {
    "roland/d50",
    "roland/d20",
    "roland/d110",
    "roland/fantom_g",
    "roland/jd08",
    "roland/jp08",
    "roland/mc101",
    "roland/ju06a",
    "roland/juno6_tubbutec",
    "roland/spd30",
    "roland/tr808",
    "roland/tb03",
    "roland/tb3",
    "roland/tr8s",
    "yamaha/dsr2000",
    "korg/kingkorg",
    "korg/opsix",
    "korg/wavestate",
    "waldorf/streichfett",
    "sequential/prophet08",
    "sequential/evolver",
    "sonicware/liven_8bit_warps",
    "moog/sub37",
    "behringer/td3",
    "behringer/jt4000",
    "behringer/rd6",
    "behringer/monopoly",
    "behringer/pro800",
}

for _, name in ipairs(model_files) do
    local ok, model = pcall(dofile, this_dir .. "models/" .. name .. ".lua")
    if ok and type(model) == "table" and model.name then
        -- Normalize: accept "params" as alias for "sysex".
        if model.params and not model.sysex then
            model.sysex = model.params
        end
        M[model.name] = model
    end
end

function M.list()
    local names = {}
    for k, v in pairs(M) do
        if type(v) == "table" and v.name then
            names[#names+1] = k
        end
    end
    table.sort(names)
    return names
end

return M

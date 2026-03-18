-- sequencer/chain.lua
-- Device chain architecture: ordered pipeline of MIDI processors.
-- Each track owns a chain of devices; events flow through them in order.

local M = {}

-- ── Device type registry ────────────────────────────────────────────────────
-- M.processors[type_name] = { params_schema, init, process, tick, feed, cleanup }
M.processors = {}

-- Lazy reference to arp module (set via M.set_arp_module).
local arp_mod = nil
function M.set_arp_module(mod) arp_mod = mod end

-- SysEx parameter templates, populated by main.lua.
-- M.sysex_templates[inst_id][param_index] = { template, min_val, max_val }
M.sysex_templates = {}

-- Roland checksum: sum bytes from start to end_idx, result = (128 - sum%128) % 128.
local function roland_checksum(bytes, start_idx, end_idx)
    local sum = 0
    for i = start_idx, end_idx do
        sum = sum + bytes[i]
    end
    return (128 - sum % 128) % 128
end

-- Expand a sysex template string, substituting {v}, {vh}, {vl}, {cs}.
local function expand_sysex_template(template_str, value)
    local hex = template_str
    -- 14-bit split
    local vh = (value >> 7) & 0x7F
    local vl = value & 0x7F
    hex = hex:gsub("{vh}", string.format("%02X", vh))
    hex = hex:gsub("{vl}", string.format("%02X", vl))
    hex = hex:gsub("{v}",  string.format("%02X", value & 0x7F))
    -- Compute Roland checksum if {cs} is present
    if hex:find("{cs}") then
        -- Parse all hex bytes so far (except {cs} placeholder)
        local bytes = {}
        for b in hex:gmatch("%x%x") do
            bytes[#bytes+1] = tonumber(b, 16)
        end
        -- Find DT1 command byte 0x12; checksum covers bytes after it
        local dt1_pos = nil
        for i, b in ipairs(bytes) do
            if b == 0x12 then dt1_pos = i; break end
        end
        local cs = 0
        if dt1_pos and dt1_pos + 1 <= #bytes then
            cs = roland_checksum(bytes, dt1_pos + 1, #bytes)
        end
        hex = hex:gsub("{cs}", string.format("%02X", cs))
    end
    return hex
end

-- Expose expand_sysex_template for use by main.lua's resolve_patch.
M.expand_sysex_template = expand_sysex_template

-- ── Built-in: instrument ────────────────────────────────────────────────────

M.processors["instrument"] = {
    params_schema = {},
    init = function(params)
        return { held_notes = {} }
    end,
    process = function(events, context, params, state)
        local inst_id = tonumber(params.instrument_id)
        if not inst_id then return events end
        local p = context.engine.output_ports[inst_id]
        if not p then return events end
        local ch = (p.channel - 1) & 0x0F
        for _, ev in ipairs(events) do
            if ev.status == 0xF2 then
                -- Synthetic SysEx parameter event: expand template and send
                local tmpl = M.sysex_templates[inst_id] and M.sysex_templates[inst_id][ev.data1]
                if tmpl then
                    local hex = expand_sysex_template(tmpl.template, ev.data2 or 0)
                    local binary = context.engine.parse_hex(hex)
                    if #binary > 0 then pcall(function() p.port:send(binary) end) end
                end
            elseif ev.status == 0xF1 then
                -- Synthetic NRPN event: expand to 4 CC messages
                local nrpn = ev.data1 or 0
                local val   = ev.data2 or 0
                local nrpn_msb = (nrpn >> 7) & 0x7F
                local nrpn_lsb = nrpn & 0x7F
                local val_msb  = (val >> 7) & 0x7F
                local val_lsb  = val & 0x7F
                pcall(function()
                    p.port:send(string.char(0xB0 | ch, 99, nrpn_msb))
                    p.port:send(string.char(0xB0 | ch, 98, nrpn_lsb))
                    p.port:send(string.char(0xB0 | ch, 6,  val_msb))
                    p.port:send(string.char(0xB0 | ch, 38, val_lsb))
                end)
            else
                local msg_type = ev.status & 0xF0
                local b0 = msg_type | ch
                pcall(function()
                    p.port:send(string.char(b0, ev.data1 or 0, ev.data2 or 0))
                end)
                if msg_type == 0x90 and (ev.data2 or 0) > 0 then
                    state.held_notes[ev.data1] = true
                elseif msg_type == 0x80 or (msg_type == 0x90 and (ev.data2 or 0) == 0) then
                    state.held_notes[ev.data1] = nil
                end
            end
        end
        return events
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params)
        local inst_id = tonumber(params.instrument_id)
        if not inst_id then return end
        local p = context.engine.output_ports[inst_id]
        if not p then return end
        local ch = (p.channel - 1) & 0x0F
        for note in pairs(state.held_notes) do
            pcall(function()
                p.port:send(string.char(0x80 | ch, note, 0))
            end)
        end
        state.held_notes = {}
    end,
}

-- ── Built-in: transpose ─────────────────────────────────────────────────────

M.processors["transpose"] = {
    params_schema = {
        { key = "semitones", label = "Semitones", type = "int", default = "0" },
    },
    init = function(params) return {} end,
    process = function(events, context, params, state)
        local semitones = tonumber(params.semitones) or 0
        if semitones == 0 then return events end
        local out = {}
        for _, ev in ipairs(events) do
            local msg_type = ev.status & 0xF0
            if msg_type == 0x90 or msg_type == 0x80 then
                local note = math.max(0, math.min(127, ev.data1 + semitones))
                out[#out+1] = { status=ev.status, data1=note, data2=ev.data2, beat_offset=ev.beat_offset }
            else
                out[#out+1] = ev
            end
        end
        return out
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: velocity_curve ────────────────────────────────────────────────

M.processors["velocity_curve"] = {
    params_schema = {
        { key = "curve",       label = "Curve",       type = "picker", opts = {"linear","compress","expand","fixed"}, default = "linear" },
        { key = "min",         label = "Min",         type = "int", default = "0" },
        { key = "max",         label = "Max",         type = "int", default = "127" },
        { key = "fixed_value", label = "Fixed Value",  type = "int", default = "100" },
    },
    init = function(params) return {} end,
    process = function(events, context, params, state)
        local curve = params.curve or "linear"
        local mn    = tonumber(params.min) or 0
        local mx    = tonumber(params.max) or 127
        local fixed = tonumber(params.fixed_value) or 100
        local out   = {}
        for _, ev in ipairs(events) do
            local msg_type = ev.status & 0xF0
            if msg_type == 0x90 and (ev.data2 or 0) > 0 then
                local vel = ev.data2
                if curve == "fixed" then
                    vel = fixed
                elseif curve == "compress" then
                    local r = vel / 127
                    vel = math.floor(mn + (mx - mn) * (r * 0.5 + 0.5))
                elseif curve == "expand" then
                    local r = vel / 127
                    vel = math.floor(mn + (mx - mn) * r * r)
                else -- linear
                    local r = vel / 127
                    vel = math.floor(mn + (mx - mn) * r)
                end
                vel = math.max(1, math.min(127, vel))
                out[#out+1] = { status=ev.status, data1=ev.data1, data2=vel, beat_offset=ev.beat_offset }
            else
                out[#out+1] = ev
            end
        end
        return out
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: channel_remap ─────────────────────────────────────────────────

M.processors["channel_remap"] = {
    params_schema = {
        { key = "channel", label = "Channel", type = "int", default = "1" },
    },
    init = function(params) return {} end,
    process = function(events, context, params, state)
        local ch = (tonumber(params.channel) or 1) - 1
        ch = math.max(0, math.min(15, ch))
        local out = {}
        for _, ev in ipairs(events) do
            local msg_type = ev.status & 0xF0
            if msg_type >= 0x80 and msg_type <= 0xE0 then
                out[#out+1] = { status=(msg_type | ch), data1=ev.data1, data2=ev.data2, beat_offset=ev.beat_offset }
            else
                out[#out+1] = ev
            end
        end
        return out
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: cc_filter ─────────────────────────────────────────────────────

M.processors["cc_filter"] = {
    params_schema = {
        { key = "mode",    label = "Mode",    type = "picker", opts = {"allow","block"}, default = "block" },
        { key = "cc_list", label = "CC List", type = "text", default = "" },
    },
    init = function(params)
        local cc_set = {}
        for s in (params.cc_list or ''):gmatch("%d+") do
            cc_set[tonumber(s)] = true
        end
        return { cc_set = cc_set }
    end,
    process = function(events, context, params, state)
        local mode = params.mode or "block"
        local out  = {}
        for _, ev in ipairs(events) do
            if ev.status == 0xF1 or ev.status == 0xF2 then
                -- NRPN and SysEx param events always pass through cc_filter
                out[#out+1] = ev
            elseif (ev.status & 0xF0) == 0xB0 then
                local in_set = state.cc_set[ev.data1]
                if mode == "allow" then
                    if in_set then out[#out+1] = ev end
                else
                    if not in_set then out[#out+1] = ev end
                end
            else
                out[#out+1] = ev
            end
        end
        return out
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: arp ───────────────────────────────────────────────────────────

M.processors["arp"] = {
    params_schema = {
        { key = "mode",      label = "Mode",      type = "picker", opts = {"up","down","updown","downup","random","played","outside_in","inside_out"}, default = "up" },
        { key = "octaves",   label = "Octaves",   type = "int", default = "1" },
        { key = "rate",      label = "Rate",       type = "float", default = "0.25" },
        { key = "gate",      label = "Gate",       type = "float", default = "0.8" },
        { key = "hold_mode", label = "Hold",       type = "int", default = "0" },
    },
    init = function(params)
        if not arp_mod then return {} end
        return arp_mod.create_state({
            mode      = params.mode or "up",
            octaves   = tonumber(params.octaves) or 1,
            rate      = tonumber(params.rate)    or 0.25,
            gate      = tonumber(params.gate)    or 0.8,
            hold_mode = tonumber(params.hold_mode) or 0,
        })
    end,
    process = function(events, context, params, state)
        if not arp_mod then return events end
        local out = {}
        for _, ev in ipairs(events) do
            local consumed = arp_mod.feed_event(state, ev.status, ev.data1, ev.data2, context.cur_beat)
            if not consumed then
                out[#out+1] = ev
            end
        end
        return out
    end,
    tick = function(state, context, params, output_fn)
        if not arp_mod then return end
        local generated = arp_mod.tick_generate(state, context.cur_beat)
        if generated and #generated > 0 then
            output_fn(generated)
        end
    end,
    feed = function(state, status, d1, d2, beat, params)
        if not arp_mod then return false end
        return arp_mod.feed_event(state, status, d1, d2, beat)
    end,
    cleanup = function(state, context, params)
        if not arp_mod then return end
        return arp_mod.cleanup_state(state)
    end,
}

-- ── Built-in: splitter ──────────────────────────────────────────────────────

M.processors["splitter"] = {
    params_schema = {
        { key = "output_count", label = "Outputs", type = "int", default = "2" },
    },
    init = function(params) return {} end,
    -- process is handled at chain level (forks to sub-chains)
    process = function(events, context, params, state) return events end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: lua_script ────────────────────────────────────────────────────

M.processors["lua_script"] = {
    params_schema = {
        { key = "code",        label = "Code",        type = "text", default = "" },
        { key = "script_path", label = "Script Path", type = "text", default = "" },
    },
    init = function(params)
        local fn = nil
        if params.code and params.code ~= '' then
            local chunk, err = load("return " .. params.code)
            if not chunk then chunk = load(params.code) end
            if chunk then
                local ok2, result = pcall(chunk)
                if ok2 then
                    if type(result) == "table" and result.process then
                        fn = result.process
                    elseif type(result) == "function" then
                        fn = result
                    end
                end
            end
        elseif params.script_path and params.script_path ~= '' then
            local ok, mod = pcall(dofile, params.script_path)
            if ok and type(mod) == "table" and mod.process then
                fn = mod.process
            end
        end
        return { process_fn = fn }
    end,
    process = function(events, context, params, state)
        if state.process_fn then
            local ok, result = pcall(state.process_fn, events, context)
            if ok and type(result) == "table" then return result end
        end
        return events
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Built-in: monitor ───────────────────────────────────────────────────────

M.processors["monitor"] = {
    params_schema = {},
    init = function(params) return { log = {} } end,
    process = function(events, context, params, state)
        for _, ev in ipairs(events) do
            state.log[#state.log + 1] = string.format(
                "beat=%.3f st=%02X d1=%d d2=%d",
                ev.beat_offset or 0, ev.status, ev.data1 or 0, ev.data2 or 0)
            if #state.log > 100 then table.remove(state.log, 1) end
        end
        return events
    end,
    tick = function(state, context, params, output_fn) end,
    feed = function(state, status, d1, d2, beat, params) return false end,
    cleanup = function(state, context, params) end,
}

-- ── Device type names (for UI pickers) ──────────────────────────────────────

M.DEVICE_TYPES = {
    "instrument", "transpose", "velocity_curve", "channel_remap",
    "cc_filter", "arp", "splitter", "lua_script", "monitor",
}

-- ── Chain resolution ────────────────────────────────────────────────────────

function M.resolve_chain(db, chain_id)
    if not chain_id then return nil end
    local rows = db:query("SELECT * FROM device_chains WHERE id=?", chain_id) or {}
    if #rows == 0 then return nil end
    local chain_row = rows[1]

    -- Load top-level devices (parent_device_id IS NULL).
    local dev_rows = db:query(
        "SELECT * FROM devices WHERE chain_id=? AND parent_device_id IS NULL ORDER BY position",
        chain_id) or {}

    local devices = {}
    for _, dr in ipairs(dev_rows) do
        devices[#devices + 1] = M._build_device(db, chain_id, dr)
    end

    return {
        id      = chain_row.id,
        name    = chain_row.name or '',
        devices = devices,
    }
end

function M._build_device(db, chain_id, dr)
    -- Load params.
    local param_rows = db:query(
        "SELECT key, value FROM device_params WHERE device_id=?", dr.id) or {}
    local params = {}
    for _, pr in ipairs(param_rows) do
        params[pr.key] = pr.value
    end

    -- For instrument devices, set instrument_id from the FK column.
    if dr.device_type == "instrument" and dr.instrument_id then
        params.instrument_id = tostring(dr.instrument_id)
    end

    -- Initialize state.
    local processor = M.processors[dr.device_type]
    local state = processor and processor.init(params) or {}

    -- Load sub-chains for splitter devices.
    local sub_chains = nil
    if dr.device_type == "splitter" then
        sub_chains = M._resolve_sub_chains(db, chain_id, dr.id)
    end

    return {
        id         = dr.id,
        type       = dr.device_type,
        params     = params,
        state      = state,
        enabled    = (dr.enabled ~= 0),
        name       = dr.name or '',
        position   = dr.position,
        sub_chains = sub_chains,
    }
end

function M._resolve_sub_chains(db, chain_id, parent_device_id)
    local dev_rows = db:query(
        "SELECT * FROM devices WHERE chain_id=? AND parent_device_id=? ORDER BY output_index, position",
        chain_id, parent_device_id) or {}

    local by_output = {}
    for _, dr in ipairs(dev_rows) do
        local oi = dr.output_index or 0
        if not by_output[oi] then by_output[oi] = {} end
        by_output[oi][#by_output[oi] + 1] = M._build_device(db, chain_id, dr)
    end
    return by_output
end

-- ── Processing pipeline ─────────────────────────────────────────────────────

function M.process(chain_def, events, context)
    if not chain_def or not events or #events == 0 then return events end

    local current = events
    for _, dev in ipairs(chain_def.devices) do
        if dev.enabled then
            if dev.type == "splitter" and dev.sub_chains then
                M._process_splitter(dev, current, context)
                return {}
            end
            local proc = M.processors[dev.type]
            if proc and proc.process then
                current = proc.process(current, context, dev.params, dev.state)
            end
        end
    end
    return current
end

function M._process_splitter(dev, events, context)
    if not dev.sub_chains then return end
    for _, sub_devices in pairs(dev.sub_chains) do
        -- Copy events for each output.
        local copy = {}
        for _, ev in ipairs(events) do
            copy[#copy+1] = { status=ev.status, data1=ev.data1, data2=ev.data2, beat_offset=ev.beat_offset }
        end
        M._process_device_list(sub_devices, copy, context)
    end
end

function M._process_device_list(devices, events, context)
    local current = events
    for _, dev in ipairs(devices) do
        if dev.enabled then
            if dev.type == "splitter" and dev.sub_chains then
                M._process_splitter(dev, current, context)
                return {}
            end
            local proc = M.processors[dev.type]
            if proc and proc.process then
                current = proc.process(current, context, dev.params, dev.state)
            end
        end
    end
    return current
end

-- ── Tick (for time-based devices like arp) ──────────────────────────────────

function M.tick_devices(chain_def, context)
    if not chain_def then return end
    for i, dev in ipairs(chain_def.devices) do
        if dev.enabled then
            local proc = M.processors[dev.type]
            if proc and proc.tick then
                proc.tick(dev.state, context, dev.params, function(generated)
                    M._route_after(chain_def.devices, i, generated, context)
                end)
            end
            if dev.sub_chains then
                for _, sub_devices in pairs(dev.sub_chains) do
                    M._tick_device_list(sub_devices, context)
                end
            end
        end
    end
end

function M._tick_device_list(devices, context)
    for i, dev in ipairs(devices) do
        if dev.enabled then
            local proc = M.processors[dev.type]
            if proc and proc.tick then
                proc.tick(dev.state, context, dev.params, function(generated)
                    M._route_after_list(devices, i, generated, context)
                end)
            end
            if dev.sub_chains then
                for _, sub_devices in pairs(dev.sub_chains) do
                    M._tick_device_list(sub_devices, context)
                end
            end
        end
    end
end

-- Route events through devices after position `after_idx` in the list.
function M._route_after(devices, after_idx, events, context)
    if not events or #events == 0 then return end
    local current = events
    for i = after_idx + 1, #devices do
        local dev = devices[i]
        if dev.enabled then
            if dev.type == "splitter" and dev.sub_chains then
                M._process_splitter(dev, current, context)
                return
            end
            local proc = M.processors[dev.type]
            if proc and proc.process then
                current = proc.process(current, context, dev.params, dev.state)
            end
        end
    end
end

M._route_after_list = M._route_after

-- ── Cleanup ─────────────────────────────────────────────────────────────────

function M.cleanup(chain_def, context)
    if not chain_def then return end
    for i, dev in ipairs(chain_def.devices) do
        local proc = M.processors[dev.type]
        if proc and proc.cleanup then
            local pending = proc.cleanup(dev.state, context, dev.params)
            if pending and #pending > 0 then
                M._route_after(chain_def.devices, i, pending, context)
            end
        end
        if dev.sub_chains then
            for _, sub_devices in pairs(dev.sub_chains) do
                M._cleanup_list(sub_devices, context)
            end
        end
    end
end

function M._cleanup_list(devices, context)
    for i, dev in ipairs(devices) do
        local proc = M.processors[dev.type]
        if proc and proc.cleanup then
            local pending = proc.cleanup(dev.state, context, dev.params)
            if pending and #pending > 0 then
                M._route_after_list(devices, i, pending, context)
            end
        end
        if dev.sub_chains then
            for _, sub_devices in pairs(dev.sub_chains) do
                M._cleanup_list(sub_devices, context)
            end
        end
    end
end

-- ── Feed MIDI input ─────────────────────────────────────────────────────────

function M.feed_input(chain_def, status, d1, d2, beat, context)
    if not chain_def then return false end
    for _, dev in ipairs(chain_def.devices) do
        if dev.enabled then
            local proc = M.processors[dev.type]
            if proc and proc.feed then
                if proc.feed(dev.state, status, d1, d2, beat, dev.params) then
                    return true
                end
            end
            if dev.sub_chains then
                for _, sub_devices in pairs(dev.sub_chains) do
                    if M._feed_list(sub_devices, status, d1, d2, beat, context) then
                        return true
                    end
                end
            end
        end
    end
    return false
end

function M._feed_list(devices, status, d1, d2, beat, context)
    for _, dev in ipairs(devices) do
        if dev.enabled then
            local proc = M.processors[dev.type]
            if proc and proc.feed then
                if proc.feed(dev.state, status, d1, d2, beat, dev.params) then
                    return true
                end
            end
        end
    end
    return false
end

-- ── Utility: check if chain has a device of a given type ────────────────────

function M.has_device_type(chain_def, device_type)
    if not chain_def then return false end
    for _, dev in ipairs(chain_def.devices) do
        if dev.type == device_type and dev.enabled then return true end
    end
    return false
end

-- Find first device of given type.
function M.find_device(chain_def, device_type)
    if not chain_def then return nil end
    for _, dev in ipairs(chain_def.devices) do
        if dev.type == device_type then return dev end
    end
    return nil
end

return M

-- sequencer/engine.lua
-- MIDI playback and recording engine.
-- Uses os.monotime() for timing (seconds, high precision on macOS).

local M = {}

M.bpm       = 120.0
M.playing   = false
M.beat_pos  = 0.0    -- beat position when transport last stopped/started
M.start_time = 0.0   -- os.monotime() when transport started

-- active_clips[track_idx] = {
--   chain_def, clip_id, events, event_idx,
--   loop_start_beat, length_beats, is_looping
-- }
M.active_clips = {}

-- output_ports[inst_id] = { port=<midi port userdata>, channel=<1..16> }
M.output_ports = {}

-- track_chains[track_idx] = chain_def (set by main.lua)
M.track_chains = {}

-- chain module reference (set by main.lua)
M.chain = nil

-- Recording state
M.recording     = false
M.rec_track_idx = nil
M.rec_inst_id   = nil
M.rec_events    = {}
M.rec_start_beat = 0.0
M.input_port    = nil

M.midi_monitor_hook = nil  -- function(status, data1, data2, raw) called for each input message

-- ── Timing ────────────────────────────────────────────────────────────────────

function M.beats_per_sec()
    return M.bpm / 60.0
end

function M.cur_beat()
    if not M.playing then return M.beat_pos end
    return M.beat_pos + (os.monotime() - M.start_time) * M.beats_per_sec()
end

-- ── Transport ─────────────────────────────────────────────────────────────────

function M.start_transport()
    if M.playing then return end
    M.start_time = os.monotime()
    M.playing    = true
end

function M.stop_transport()
    if not M.playing then return end
    M.beat_pos = M.cur_beat()
    M.playing  = false
    -- Cleanup all active chains.
    local ctx = { engine = M, cur_beat = M.beat_pos }
    for _, ac in pairs(M.active_clips) do
        if ac.chain_def and M.chain then
            M.chain.cleanup(ac.chain_def, ctx)
        end
    end
    -- Safety net: all-notes-off on every active output port.
    for _, p in pairs(M.output_ports) do
        local ch = (p.channel - 1) & 0x0F
        pcall(function()
            p.port:send(string.char(0xB0 | ch, 123, 0))
            p.port:send(string.char(0xB0 | ch, 64,  0))
        end)
    end
end

function M.set_bpm(bpm)
    bpm = math.max(20, math.min(300, bpm))
    if M.playing then
        local cur   = M.cur_beat()
        M.bpm       = bpm
        M.beat_pos  = cur
        M.start_time = os.monotime()
    else
        M.bpm = bpm
    end
end

-- ── MIDI port management ──────────────────────────────────────────────────────

function M.open_output(inst)
    if M.output_ports[inst.id] then return true end
    if not inst.midi_output or inst.midi_output == '' then return false, "no device" end
    local ok, port = pcall(midi.open_output, inst.midi_output)
    if ok and port then
        M.output_ports[inst.id] = { port = port, channel = inst.midi_channel or 1 }
        return true
    end
    return false, tostring(port)
end

function M.open_input(device_name)
    if M.input_port then
        pcall(function() M.input_port:close() end)
        M.input_port = nil
    end
    if not device_name or device_name == '' then return false, "no device" end
    local ok, port = pcall(midi.open_input, device_name)
    if ok and port then
        M.input_port = port
        return true
    end
    return false, tostring(port)
end

-- Parse a space-separated hex string (e.g. "F0 41 10 42 F7") into a binary string.
function M.parse_hex(hex_str)
    local bytes = {}
    for b in (hex_str or ''):gmatch("%x%x") do
        bytes[#bytes + 1] = string.char(tonumber(b, 16))
    end
    return table.concat(bytes)
end

-- Send a raw SysEx (or any MIDI) message given as a hex string.
function M.send_sysex(inst_id, hex_data)
    local p = M.output_ports[inst_id]
    if not p or not hex_data or hex_data == '' then return end
    local data = M.parse_hex(hex_data)
    if #data > 0 then
        pcall(function() p.port:send(data) end)
    end
end

-- Send a resolved patch table in strict order:
-- 1. bank_sysex  2. bank MSB/LSB  3. program change  4. program_sysex  5. overrides
function M.send_patch(inst_id, patch)
    local p = M.output_ports[inst_id]
    if not p then return end
    local ch = (p.channel - 1) & 0x0F

    -- 1. Bank SysEx dump
    if patch.bank_sysex and patch.bank_sysex ~= '' then
        local data = M.parse_hex(patch.bank_sysex)
        if #data > 0 then pcall(function() p.port:send(data) end) end
    end

    -- 2. Bank Select CC#0 MSB + CC#32 LSB
    if patch.bank_msb then
        pcall(function() p.port:send(string.char(0xB0 | ch, 0,  patch.bank_msb)) end)
    end
    if patch.bank_lsb then
        pcall(function() p.port:send(string.char(0xB0 | ch, 32, patch.bank_lsb)) end)
    end

    -- 3. Program Change
    if patch.program then
        pcall(function() p.port:send(string.char(0xC0 | ch, patch.program)) end)
    end

    -- 4. Program SysEx dump
    if patch.program_sysex and patch.program_sysex ~= '' then
        local data = M.parse_hex(patch.program_sysex)
        if #data > 0 then pcall(function() p.port:send(data) end) end
    end

    -- 5. Overrides (already sorted: sysex, nrpn, cc)
    if patch.overrides then
        for _, ov in ipairs(patch.overrides) do
            if ov.type == 'sysex' then
                if ov.hex and ov.hex ~= '' then
                    local data = M.parse_hex(ov.hex)
                    if #data > 0 then pcall(function() p.port:send(data) end) end
                end
            elseif ov.type == 'nrpn' then
                local nrpn = ov.param or 0
                local val  = ov.value or 0
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
            elseif ov.type == 'cc' then
                local cc  = ov.param or 0
                local val = ov.value or 0
                pcall(function() p.port:send(string.char(0xB0 | ch, cc, val)) end)
            end
        end
    end
end

function M.close_all()
    for _, p in pairs(M.output_ports) do
        pcall(function() p.port:close() end)
    end
    M.output_ports = {}
    if M.input_port then
        pcall(function() M.input_port:close() end)
        M.input_port = nil
    end
end

-- ── Clip launch / stop ────────────────────────────────────────────────────────

-- Launch a clip on a track, quantised to the next 4-beat bar boundary.
function M.launch(track_idx, chain_def, clip, events)
    M.stop_track(track_idx)

    local cur      = M.cur_beat()
    local next_bar = math.ceil(cur / 4) * 4
    if next_bar - cur < 0.05 then next_bar = next_bar + 4 end
    if not M.playing then
        next_bar = 0
    else
        local cur_bar = math.floor(cur / 4) * 4
        if cur - cur_bar < 1.0 then next_bar = cur_bar end
    end

    local len        = clip.length_beats or 4.0
    local start_off  = clip.start_offset or 0.0
    local ls         = clip.loop_start   or 0.0
    local ll         = clip.loop_length
    local loop_end   = ll and (ls + math.max(0.001, ll)) or len

    -- Advance event_idx past events before start_offset.
    local event_idx = 1
    while event_idx <= #events and events[event_idx].beat_offset < start_off do
        event_idx = event_idx + 1
    end

    M.active_clips[track_idx] = {
        chain_def       = chain_def,
        clip_id         = clip.id,
        events          = events,
        event_idx       = event_idx,
        loop_start_beat = next_bar,
        length_beats    = len,
        start_offset    = start_off,
        loop_start_off  = ls,
        loop_end        = loop_end,
        is_looping      = (clip.is_looping == 1 or clip.is_looping == true),
    }
end

-- Close a single instrument's output port.
function M.close_output(inst_id)
    local p = M.output_ports[inst_id]
    if p then
        pcall(function() p.port:close() end)
        M.output_ports[inst_id] = nil
    end
end

-- Stop a single track and cleanup its chain.
function M.stop_track(track_idx)
    local ac = M.active_clips[track_idx]
    if not ac then return end
    if ac.chain_def and M.chain then
        M.chain.cleanup(ac.chain_def, { engine = M, cur_beat = M.cur_beat() })
    end
    M.active_clips[track_idx] = nil
end

-- ── Recording ─────────────────────────────────────────────────────────────────

function M.start_record(track_idx, inst_id)
    M.rec_track_idx  = track_idx
    M.rec_inst_id    = inst_id
    M.rec_events     = {}
    M.rec_start_beat = M.cur_beat()
    M.recording      = true
end

-- Stop recording. Returns track_idx, events, length_beats.
function M.stop_record()
    M.recording    = false
    local track    = M.rec_track_idx
    local evts     = M.rec_events
    local length   = M.cur_beat() - M.rec_start_beat
    length = math.ceil(length / 4) * 4
    if length < 4 then length = 4 end
    M.rec_events    = {}
    M.rec_track_idx = nil
    return track, evts, length
end

-- Poll MIDI input, decode raw bytes, route to chain feed, then record.
function M.process_input()
    if not M.input_port then return end
    local cur = M.cur_beat()
    while true do
        local ok, msg = pcall(function() return M.input_port:recv(0) end)
        if not ok or not msg then break end

        local raw    = msg.data or ''
        local status = raw:byte(1) or 0
        local data1  = raw:byte(2) or 0
        local data2  = raw:byte(3) or 0

        -- Forward to MIDI monitor if hooked.
        if M.midi_monitor_hook then
            M.midi_monitor_hook(status, data1, data2, raw)
        end

        -- Route through chain feed (for arp device consumption).
        local consumed = false
        if M.rec_track_idx and M.chain then
            local chain_def = M.track_chains[M.rec_track_idx]
            if chain_def then
                consumed = M.chain.feed_input(chain_def, status, data1, data2, cur,
                    { engine = M, cur_beat = cur })
            end
        end

        if not consumed and M.recording then
            local beat = cur - M.rec_start_beat
            table.insert(M.rec_events, {
                beat_offset = beat,
                status      = status,
                data1       = data1,
                data2       = data2,
            })
        end
    end
end

-- ── Main tick ─────────────────────────────────────────────────────────────────

function M.tick()
    if not M.playing then return end
    local now = M.cur_beat()
    local ctx = { engine = M, cur_beat = now }

    local to_stop = {}

    for track_idx, ac in pairs(M.active_clips) do
        local clip_beat_raw = now - ac.loop_start_beat
        local clip_beat     = clip_beat_raw + (ac.start_offset or 0)
        local ls            = ac.loop_start_off or 0
        local loop_end      = ac.loop_end or ac.length_beats

        if clip_beat_raw < 0 then
            -- Waiting for launch quantisation.
        elseif clip_beat >= loop_end then
            if ac.is_looping then
                local period = math.max(0.001, loop_end - ls)
                local loops  = math.max(1, math.floor((clip_beat - ls) / period))
                ac.loop_start_beat = ac.loop_start_beat + loops * period
                clip_beat = clip_beat - loops * period
                -- Release held notes via chain cleanup.
                if ac.chain_def and M.chain then
                    M.chain.cleanup(ac.chain_def, ctx)
                end
                -- Reset event index to loop start.
                local idx = 1
                while idx <= #ac.events and ac.events[idx].beat_offset < ls do
                    idx = idx + 1
                end
                ac.event_idx = idx
            else
                table.insert(to_stop, track_idx)
            end
        end

        if clip_beat_raw >= 0 then
            -- Collect due events.
            local due = {}
            while ac.event_idx <= #ac.events do
                local ev = ac.events[ac.event_idx]
                if ev.beat_offset > clip_beat then break end
                due[#due + 1] = ev
                ac.event_idx = ac.event_idx + 1
            end
            -- Process through chain.
            if #due > 0 and ac.chain_def and M.chain then
                M.chain.process(ac.chain_def, due, ctx)
            end
        end
    end

    -- Tick time-based chain devices (arp).
    for track_idx, ac in pairs(M.active_clips) do
        if ac.chain_def and M.chain then
            M.chain.tick_devices(ac.chain_def, ctx)
        end
    end

    for _, ti in ipairs(to_stop) do
        M.stop_track(ti)
    end
end

return M

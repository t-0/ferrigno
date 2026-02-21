-- sequencer/engine.lua
-- MIDI playback and recording engine.
-- Uses os.monotime() for timing (seconds, high precision on macOS).

local M = {}

M.bpm       = 120.0
M.playing   = false
M.beat_pos  = 0.0    -- beat position when transport last stopped/started
M.start_time = 0.0   -- os.monotime() when transport started

-- active_clips[track_idx] = {
--   inst_id, clip_id, events, event_idx,
--   loop_start_beat, length_beats, is_looping, held_notes={note->true}
-- }
M.active_clips = {}

-- output_ports[inst_id] = { port=<midi port userdata>, channel=<1..16> }
M.output_ports = {}

-- Recording state
M.recording     = false
M.rec_track_idx = nil
M.rec_inst_id   = nil
M.rec_events    = {}
M.rec_start_beat = 0.0
M.input_port    = nil

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
    -- All-notes-off on every active output port.
    for _, p in pairs(M.output_ports) do
        for n = 0, 127 do
            pcall(function() p.port:note_off(p.channel, n, 0) end)
        end
    end
end

function M.set_bpm(bpm)
    bpm = math.max(20, math.min(300, bpm))
    if M.playing then
        -- Preserve current beat position, adjust reference time.
        local cur   = M.cur_beat()
        M.bpm       = bpm
        M.beat_pos  = cur
        M.start_time = os.monotime()
    else
        M.bpm = bpm
    end
end

-- ── MIDI port management ──────────────────────────────────────────────────────

-- Returns true on success, false + errmsg on failure.
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
function M.launch(track_idx, inst_id, clip, events)
    M.stop_track(track_idx)

    local cur      = M.cur_beat()
    local next_bar = math.ceil(cur / 4) * 4
    -- If we're already very close to the bar line, skip to the next one.
    if next_bar - cur < 0.05 then next_bar = next_bar + 4 end
    if not M.playing then next_bar = 0 end

    M.active_clips[track_idx] = {
        inst_id         = inst_id,
        clip_id         = clip.id,
        events          = events,
        event_idx       = 1,
        loop_start_beat = next_bar,
        length_beats    = clip.length_beats or 4.0,
        is_looping      = (clip.is_looping == 1 or clip.is_looping == true),
        held_notes      = {},
    }
end

-- Stop a single track and release any held notes.
function M.stop_track(track_idx)
    local ac = M.active_clips[track_idx]
    if not ac then return end
    local p = M.output_ports[ac.inst_id]
    if p then
        for note in pairs(ac.held_notes) do
            pcall(function() p.port:note_off(p.channel, note, 0) end)
        end
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
    -- Round up to the nearest 4-beat bar.
    length = math.ceil(length / 4) * 4
    if length < 4 then length = 4 end
    M.rec_events    = {}
    M.rec_track_idx = nil
    return track, evts, length
end

-- Optional callback: on_midi_input(track_idx, status, data1, data2, cur_beat) → bool consumed
-- Set by main.lua to route input to the arpeggiator.
M.on_midi_input = nil

-- Poll MIDI input, decode raw bytes, route to arp callback, then record.
function M.process_input()
    if not M.input_port then return end
    local cur = M.cur_beat()
    while true do
        local ok, msg = pcall(function() return M.input_port:recv(0) end)
        if not ok or not msg then break end

        -- recv() returns {data=rawstring, time=integer} — decode bytes.
        local raw    = msg.data or ''
        local status = raw:byte(1) or 0
        local data1  = raw:byte(2) or 0
        local data2  = raw:byte(3) or 0

        -- Offer to arp callback first; record only if not consumed.
        local consumed = false
        if M.on_midi_input and M.rec_track_idx then
            consumed = M.on_midi_input(M.rec_track_idx, status, data1, data2, cur)
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

    -- Collect tracks to stop (avoid modifying active_clips while iterating).
    local to_stop = {}

    for track_idx, ac in pairs(M.active_clips) do
        local clip_beat = now - ac.loop_start_beat

        if clip_beat < 0 then
            -- Waiting for launch quantisation — nothing to do yet.
        elseif clip_beat >= ac.length_beats then
            if ac.is_looping then
                -- Advance loop: release held notes, reset events.
                local loops = math.floor(clip_beat / ac.length_beats)
                ac.loop_start_beat = ac.loop_start_beat + loops * ac.length_beats
                clip_beat = clip_beat - loops * ac.length_beats
                local p = M.output_ports[ac.inst_id]
                if p then
                    for note in pairs(ac.held_notes) do
                        pcall(function() p.port:note_off(p.channel, note, 0) end)
                    end
                end
                ac.held_notes = {}
                ac.event_idx  = 1
            else
                table.insert(to_stop, track_idx)
            end
        end

        if clip_beat >= 0 then
            local p = M.output_ports[ac.inst_id]
            -- Dispatch all events whose beat_offset <= clip_beat.
            while ac.event_idx <= #ac.events do
                local ev = ac.events[ac.event_idx]
                if ev.beat_offset > clip_beat then break end

                local msg_type = ev.status & 0xF0
                local ch       = p and ((p.channel - 1) & 0x0F) or 0
                local b0       = msg_type | ch

                if p then
                    pcall(function()
                        p.port:send(string.char(b0, ev.data1 or 0, ev.data2 or 0))
                    end)
                    if msg_type == 0x90 and (ev.data2 or 0) > 0 then
                        ac.held_notes[ev.data1] = true
                    elseif msg_type == 0x80 or (msg_type == 0x90 and (ev.data2 or 0) == 0) then
                        ac.held_notes[ev.data1] = nil
                    end
                end

                ac.event_idx = ac.event_idx + 1
            end
        end
    end

    for _, ti in ipairs(to_stop) do
        M.stop_track(ti)
    end
end

return M

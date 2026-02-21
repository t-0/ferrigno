-- sequencer/main.lua
-- Entry point: rlua sequencer/main.lua [song.db]
--
-- Keyboard controls
--   Arrow keys       Navigate the session grid
--   SPACE            Start / stop transport
--   ENTER            Trigger (launch) the clip under the cursor
--   R                Start / stop recording on the current track
--   E                Open piano roll editor for clip under cursor
--   C                Clip settings (name, bank/program patch override)
--   N                New empty clip in current slot
--   D                Delete clip in current slot
--   I                Edit current instrument (name, MIDI device, channel)
--   B                Set BPM
--   F                Rename song
--   P                Toggle arpeggiator on/off for current track
--   A                Cycle arp mode  (when arp is on)
--   O                Cycle arp rate  (when arp is on)
--   +  (or =)        Add a new track
--   -                Remove the rightmost track (if empty)
--   S                Save to database
--   ESC              Quit (auto-saves first)

local script_dir = (arg and arg[0]) and arg[0]:match("(.*/)") or "./"
local function req(name)
    return dofile(script_dir .. name .. ".lua")
end

local db_mod     = req("db")
local engine     = req("engine")
local ui         = req("ui")
local piano_roll = req("piano_roll")
local arp        = req("arp")

-- ── Open song ─────────────────────────────────────────────────────────────────

local song_path = (arg and arg[1]) or "song.db"
local db        = db_mod.open(song_path)
local song      = db_mod.get_or_create_song(db, "Untitled")

local NUM_SLOTS = 8

-- Load instruments, creating two defaults if the song is brand-new.
local instruments = db_mod.get_instruments(db, song.id)
if #instruments == 0 then
    local dests       = midi.destinations() or {}
    local default_out = dests[1] or ''
    local srcs        = midi.sources()      or {}
    local default_in  = srcs[1]  or ''

    local t1 = db_mod.upsert_instrument(db, {
        name = "Synth", midi_output = default_out, midi_input = default_in,
        midi_channel = 1, track_index = 0, color = 2,
    }, song.id)
    local t2 = db_mod.upsert_instrument(db, {
        name = "Bass", midi_output = default_out, midi_input = default_in,
        midi_channel = 2, track_index = 1, color = 4,
    }, song.id)
    instruments = { t1, t2 }
end

-- Load clips and their MIDI events into memory.
-- clips[inst_id][slot_idx] = clip_table
-- events[clip_id] = { {beat_offset,status,data1,data2}, ... }
local clips  = {}
local events = {}
for _, inst in ipairs(instruments) do
    clips[inst.id] = {}
    for _, clip in ipairs(db_mod.get_clips(db, inst.id)) do
        clips[inst.id][clip.slot_index] = clip
        events[clip.id] = db_mod.get_events(db, clip.id)
    end
end

-- Load SysEx dumps per instrument.
local sysex_dumps = {}
for _, inst in ipairs(instruments) do
    sysex_dumps[inst.id] = db_mod.get_sysex_dumps(db, inst.id)
end

-- Ensure scene rows exist.
local scenes = db_mod.ensure_scenes(db, song.id, NUM_SLOTS)

-- ── Open MIDI outputs ─────────────────────────────────────────────────────────

-- Open output port, then send sysex dumps and instrument-level program change.
local function open_and_configure_output(inst)
    local ok, err = engine.open_output(inst)
    if not ok then return false, err end
    for _, dump in ipairs(sysex_dumps[inst.id] or {}) do
        engine.send_sysex(inst.id, dump.data)
    end
    engine.send_program_change(inst)
    return true
end

engine.bpm = song.bpm
for _, inst in ipairs(instruments) do
    open_and_configure_output(inst)
end

-- ── Wire arp callback into engine ─────────────────────────────────────────────

engine.on_midi_input = function(track_idx, status, data1, data2, cur)
    if arp.states[track_idx] then
        return arp.feed(track_idx, status, data1, data2, cur)
    end
    return false
end

-- ── Wire UI state ─────────────────────────────────────────────────────────────

ui.song        = song
ui.instruments = instruments
ui.clips       = clips
ui.scenes      = scenes
ui.num_slots   = NUM_SLOTS
ui.bpm         = song.bpm
ui.arp_states  = arp.states  -- live reference so ui.draw sees arp status

-- ── TUI setup ─────────────────────────────────────────────────────────────────

tui.init()
tui.enter_alt()
tui.raw()
tui.hide_cursor()
tui.clear()

-- ── Helpers ───────────────────────────────────────────────────────────────────

local function current_inst()
    return instruments[ui.cursor.track]
end

local function current_clip()
    local inst = current_inst()
    if not inst then return nil end
    return clips[inst.id] and clips[inst.id][ui.cursor.slot]
end

local function save_all()
    db_mod.save_song(db, song)
    for _, inst in ipairs(instruments) do
        db_mod.upsert_instrument(db, inst, song.id)
    end
    for _, inst in ipairs(instruments) do
        local ic = clips[inst.id] or {}
        for _, clip in pairs(ic) do
            db_mod.upsert_clip(db, clip)
            if events[clip.id] then
                db_mod.save_events(db, clip.id, events[clip.id])
            end
        end
    end
    ui.set_status("Saved → " .. song_path, tui.GREEN)
end

-- ── Action handlers ───────────────────────────────────────────────────────────

local function trigger_clip()
    local inst = current_inst()
    if not inst then return end
    local clip = current_clip()
    if not clip then
        ui.set_status("Empty slot — press N to create a clip", tui.YELLOW)
        return
    end
    local evts = events[clip.id] or {}
    -- Send clip-level patch override before launch so the synth is ready.
    if clip.bank_msb or clip.bank_lsb or clip.program then
        engine.send_clip_patch(inst, clip)
    end
    engine.launch(ui.cursor.track, inst.id, clip, evts)
    if not engine.playing then
        engine.start_transport()
    end
    ui.set_status("► " .. (clip.name ~= '' and clip.name or "Clip " .. ui.cursor.slot))
end

local function new_clip()
    local inst = current_inst()
    if not inst then return end
    local si = ui.cursor.slot
    if not clips[inst.id] then clips[inst.id] = {} end
    if clips[inst.id][si] then
        ui.set_status("Slot occupied — press D to delete first", tui.YELLOW)
        return
    end
    local clip = { instrument_id = inst.id, slot_index = si,
                   name = '', length_beats = 4.0, is_looping = true }
    db_mod.upsert_clip(db, clip)
    clips[inst.id][si] = clip
    events[clip.id] = {}
    ui.set_status("New clip created in slot " .. si)
end

local function delete_clip()
    local inst = current_inst()
    if not inst then return end
    local clip = current_clip()
    if not clip then
        ui.set_status("No clip here", tui.YELLOW)
        return
    end
    engine.stop_track(ui.cursor.track)
    db_mod.delete_clip(db, clip.id)
    events[clip.id] = nil
    clips[inst.id][ui.cursor.slot] = nil
    ui.set_status("Clip deleted")
end

local function toggle_record()
    if engine.recording then
        -- ── Stop recording ────────────────────────────────────────────────────
        local track_idx, rec_evts, length = engine.stop_record()
        ui.recording = false
        local inst = instruments[track_idx]
        if not inst then return end
        local si = ui.cursor.slot
        if not clips[inst.id] then clips[inst.id] = {} end
        local clip = clips[inst.id][si]
        if not clip then
            clip = { instrument_id = inst.id, slot_index = si,
                     name = '', length_beats = length, is_looping = true }
            db_mod.upsert_clip(db, clip)
            clips[inst.id][si] = clip
        else
            clip.length_beats = length
            db_mod.upsert_clip(db, clip)
        end
        events[clip.id] = rec_evts
        db_mod.save_events(db, clip.id, rec_evts)
        ui.set_status(string.format("Recorded %d events (%.1f beats)", #rec_evts, length), tui.GREEN)
    else
        -- ── Start recording ───────────────────────────────────────────────────
        local inst = current_inst()
        if not inst then return end
        if inst.midi_input and inst.midi_input ~= '' then
            local ok, err = engine.open_input(inst.midi_input)
            if not ok then
                ui.set_status("MIDI input error: " .. tostring(err), tui.RED)
                return
            end
        else
            ui.set_status("No MIDI input configured for this instrument (press I)", tui.YELLOW)
            return
        end
        engine.start_record(ui.cursor.track, inst.id)
        if not engine.playing then engine.start_transport() end
        ui.recording = true
        ui.set_status("● Recording — press R to stop", tui.RED)
    end
end

local function add_track()
    local dests       = midi.destinations() or {}
    local default_out = dests[1] or ''
    local inst = {
        name = "Track " .. (#instruments + 1),
        midi_output  = default_out,
        midi_input   = '',
        midi_channel = math.min(16, #instruments + 1),
        track_index  = #instruments,
        color = 0,
    }
    db_mod.upsert_instrument(db, inst, song.id)
    clips[inst.id] = {}
    sysex_dumps[inst.id] = {}
    table.insert(instruments, inst)
    ui.cursor.track = #instruments
    open_and_configure_output(inst)
    ui.set_status("Added track: " .. inst.name)
end

local function remove_track()
    if #instruments <= 1 then
        ui.set_status("Cannot remove the last track", tui.YELLOW)
        return
    end
    local inst = instruments[#instruments]
    -- Only remove if the track has no clips.
    local has_clip = false
    for _ in pairs(clips[inst.id] or {}) do has_clip = true; break end
    if has_clip then
        ui.set_status("Track has clips — delete clips first", tui.YELLOW)
        return
    end
    engine.stop_track(#instruments)
    db_mod.delete_instrument(db, inst.id)
    clips[inst.id] = nil
    table.remove(instruments)
    ui.cursor.track = math.min(ui.cursor.track, #instruments)
    ui.set_status("Track removed")
end

local function edit_clip_settings()
    local clip = current_clip()
    if not clip then
        ui.set_status("No clip here — press N to create one first", tui.YELLOW)
        return
    end
    local w = tui.size()
    tui.clear()
    tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
    tui.print_at(1, 2, "Clip Settings", tui.WHITE, tui.BLUE, tui.BOLD)

    local name_val = ui.read_line(3, "Clip name: ", clip.name or '')
    if name_val ~= nil then clip.name = name_val end

    tui.clear()
    local bank_msb_val = ui.read_line(3, "Bank MSB override (0-127, blank=none): ",
        clip.bank_msb ~= nil and tostring(clip.bank_msb) or '')
    if bank_msb_val ~= nil then
        local n = tonumber(bank_msb_val)
        clip.bank_msb = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    tui.clear()
    local bank_lsb_val = ui.read_line(3, "Bank LSB override (0-127, blank=none): ",
        clip.bank_lsb ~= nil and tostring(clip.bank_lsb) or '')
    if bank_lsb_val ~= nil then
        local n = tonumber(bank_lsb_val)
        clip.bank_lsb = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    tui.clear()
    local prog_val = ui.read_line(3, "Program Change override (0-127, blank=none): ",
        clip.program ~= nil and tostring(clip.program) or '')
    if prog_val ~= nil then
        local n = tonumber(prog_val)
        clip.program = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    db_mod.upsert_clip(db, clip)
    tui.clear()
    ui.set_status("Clip settings saved")
end

-- SysEx dump manager for an instrument.
-- Called from edit_instrument(); edits sysex_dumps[inst.id] in-place.
local function edit_sysex(inst)
    sysex_dumps[inst.id] = sysex_dumps[inst.id] or {}
    local dumps = sysex_dumps[inst.id]
    local sel   = math.max(1, #dumps)

    local function draw()
        local w, h = tui.size()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "SysEx Dumps — " .. inst.name, tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  D=delete  ESC=done",
            tui.BRIGHT_BLACK, tui.BLACK, 0)
        if #dumps == 0 then
            tui.print_at(5, 2, "(no sysex dumps)", tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            for i, d in ipairs(dumps) do
                local label = string.format(" %-18s  %s", d.name or '', d.data or '')
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLACK, 0)
                end
            end
        end
        tui.flush()
    end

    draw()
    while true do
        local key = tui.read_key(5000)
        if not key then goto sysex_again end

        if key == 'esc' then
            break
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #dumps), sel + 1)
        elseif key == 'd' or key == 'D' then
            if #dumps > 0 then
                local d = table.remove(dumps, sel)
                db_mod.delete_sysex_dump(db, d.id)
                sel = math.max(1, math.min(sel, #dumps))
            end
        elseif key == 'a' or key == 'A' then
            tui.clear()
            local name_val = ui.read_line(3, "Dump name: ", '')
            if name_val then
                tui.clear()
                local data_val = ui.read_line(3, "Hex bytes (e.g. F0 41 10 42 F7): ", '')
                if data_val and data_val ~= '' then
                    local dump = {
                        instrument_id = inst.id,
                        name    = name_val,
                        data    = data_val:upper(),
                        send_on = 'connect',
                    }
                    db_mod.upsert_sysex_dump(db, dump)
                    table.insert(dumps, dump)
                    sel = #dumps
                end
            end
        end
        draw()
        ::sysex_again::
    end
end

local function edit_instrument()
    local inst = current_inst()
    if not inst then return end

    local dests = midi.destinations() or {}
    local srcs  = midi.sources()      or {}

    -- Pick from an enumerated list. items = {"None", name1, name2, ...}
    -- Returns the selected name, or nil if ESC pressed.
    local function pick_port(label, items, current)
        local w, h = tui.size()
        local sel = 1  -- default to "None"
        for i, v in ipairs(items) do
            if v == current then sel = i; break end
        end

        local function draw_picker()
            tui.clear()
            tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
            tui.print_at(1, 2, label, tui.WHITE, tui.BLUE, tui.BOLD)
            tui.print_at(3, 2, "Use ↑↓ to select, ENTER to confirm, ESC to cancel.",
                tui.BRIGHT_BLACK, tui.BLACK, 0)
            for i, v in ipairs(items) do
                if i == sel then
                    tui.print_at(3 + i, 2, string.format(" ► %s", v), tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(3 + i, 2, string.format("   %s", v), tui.WHITE, tui.BLACK, 0)
                end
            end
            tui.flush()
        end

        draw_picker()
        while true do
            local key = tui.read_key(5000)
            if not key then goto again end
            if key == 'up'   then sel = math.max(1, sel - 1)
            elseif key == 'down' then sel = math.min(#items, sel + 1)
            elseif key == 'enter' or key == 'return' then return items[sel]
            elseif key == 'esc' then return nil
            end
            draw_picker()
            ::again::
        end
    end

    -- Build option lists with "None" prepended.
    local dest_items = {"None"}
    for _, d in ipairs(dests) do dest_items[#dest_items+1] = d end

    local src_items = {"None"}
    for _, s in ipairs(srcs) do src_items[#src_items+1] = s end

    -- Name (free text, same as before).
    local w, h = tui.size()
    tui.clear()
    tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
    tui.print_at(1, 2, "Instrument Settings", tui.WHITE, tui.BLUE, tui.BOLD)
    local name_val = ui.read_line(3, "Name: ", inst.name)
    if name_val and name_val ~= '' then inst.name = name_val end

    -- MIDI Output picker.
    local out_val = pick_port("MIDI Output", dest_items, inst.midi_output or "None")
    if out_val ~= nil then
        inst.midi_output = (out_val == "None") and '' or out_val
    end

    -- MIDI Input picker.
    local in_val = pick_port("MIDI Input", src_items, inst.midi_input or "None")
    if in_val ~= nil then
        inst.midi_input = (in_val == "None") and '' or in_val
    end

    -- Channel (free text, small range).
    tui.clear()
    local ch_val = ui.read_line(3, "Channel (1-16): ", tostring(inst.midi_channel or 1))
    if ch_val then
        inst.midi_channel = math.max(1, math.min(16, tonumber(ch_val) or inst.midi_channel))
    end

    -- Bank MSB (CC 0), optional 0-127.
    tui.clear()
    local bank_msb_val = ui.read_line(3, "Bank MSB (0-127, blank=none): ",
        inst.bank_msb ~= nil and tostring(inst.bank_msb) or '')
    if bank_msb_val ~= nil then
        local n = tonumber(bank_msb_val)
        inst.bank_msb = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    -- Bank LSB (CC 32), optional 0-127.
    tui.clear()
    local bank_lsb_val = ui.read_line(3, "Bank LSB (0-127, blank=none): ",
        inst.bank_lsb ~= nil and tostring(inst.bank_lsb) or '')
    if bank_lsb_val ~= nil then
        local n = tonumber(bank_lsb_val)
        inst.bank_lsb = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    -- Program Change, optional 0-127.
    tui.clear()
    local prog_val = ui.read_line(3, "Program Change (0-127, blank=none): ",
        inst.program ~= nil and tostring(inst.program) or '')
    if prog_val ~= nil then
        local n = tonumber(prog_val)
        inst.program = n and math.max(0, math.min(127, math.floor(n))) or nil
    end

    db_mod.upsert_instrument(db, inst, song.id)
    engine.output_ports[inst.id] = nil
    open_and_configure_output(inst)

    -- SysEx dump manager.
    edit_sysex(inst)

    tui.clear()
    ui.set_status("Instrument updated: " .. inst.name)
end

local function edit_bpm()
    local w, h = tui.size()
    local val = ui.read_line(h - 1, "BPM: ", string.format("%.1f", engine.bpm))
    if val then
        local n = tonumber(val)
        if n then
            engine.set_bpm(n)
            song.bpm = engine.bpm
            ui.bpm   = engine.bpm
            ui.set_status(string.format("BPM → %.1f", engine.bpm))
        else
            ui.set_status("Invalid BPM value", tui.RED)
        end
    end
end

local function rename_song()
    local w, h = tui.size()
    local val = ui.read_line(h - 1, "Song name: ", song.name)
    if val and val ~= '' then
        song.name = val
        ui.song.name = val
        ui.set_status("Song renamed: " .. val)
    end
end

local function edit_clip_piano_roll()
    local inst = current_inst()
    local clip = current_clip()
    if not clip then
        ui.set_status("No clip here — press N to create one first", tui.YELLOW)
        return
    end
    local was_playing = engine.playing
    if was_playing then engine.stop_transport() end

    local new_evts, new_len, new_loop = piano_roll.open(clip, events[clip.id] or {})
    tui.clear()

    if new_evts then
        events[clip.id]   = new_evts
        clip.length_beats = new_len
        clip.is_looping   = new_loop and 1 or 0
        db_mod.upsert_clip(db, clip)
        db_mod.save_events(db, clip.id, new_evts)
        ui.set_status(string.format("Clip saved: %d events, %.1f beats", #new_evts, new_len), tui.GREEN)
    else
        ui.set_status("Piano roll closed")
    end

    if was_playing then
        engine.start_transport()
        -- Re-launch clip if it was active.
        if inst and engine.active_clips[ui.cursor.track] then
            engine.launch(ui.cursor.track, inst.id, clip, events[clip.id] or {})
        end
    end
end

local function toggle_arp()
    local ti   = ui.cursor.track
    local inst = current_inst()
    if not inst then return end
    if arp.states[ti] then
        arp.disable(ti, engine.output_ports[inst.id])
        db_mod.save_arp_settings(db, inst.id, { mode="up", octaves=1, rate=0.25, gate=0.8, hold=false })
        ui.set_status("Arp OFF — track " .. ti)
    else
        local settings = db_mod.get_arp_settings(db, inst.id)
        arp.enable(ti, settings, engine.cur_beat())
        -- Open MIDI input if needed.
        if inst.midi_input and inst.midi_input ~= '' then
            engine.open_input(inst.midi_input)
        end
        if not engine.playing then engine.start_transport() end
        ui.set_status(string.format("Arp ON — track %d [%s]", ti, settings.mode or "up"))
    end
end

local function cycle_arp_mode()
    local ti   = ui.cursor.track
    local inst = current_inst()
    if not arp.states[ti] then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    local next_mode = arp.cycle(ti, "mode", arp.MODES)
    if inst then db_mod.save_arp_settings(db, inst.id, arp.get_state(ti)) end
    ui.set_status("Arp mode: " .. next_mode)
end

local function cycle_arp_rate()
    local ti   = ui.cursor.track
    local inst = current_inst()
    if not arp.states[ti] then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    local next_rate = arp.cycle(ti, "rate", arp.RATES)
    if inst then db_mod.save_arp_settings(db, inst.id, arp.get_state(ti)) end
    ui.set_status("Arp rate: " .. (arp.RATE_NAMES[next_rate] or tostring(next_rate)))
end

-- ── Main loop ─────────────────────────────────────────────────────────────────

local running       = true
local DRAW_INTERVAL = 1.0 / 30   -- target ~30 FPS
-- Initialise to a negative value so the very first iteration always draws.
local last_draw     = -1

while running do
    -- 1. Engine tick (dispatch scheduled MIDI events).
    engine.tick()
    engine.process_input()

    -- 2. Arp output tick (generates arpeggiated notes per active track).
    if engine.playing then
        local cur = engine.cur_beat()
        for ti, inst in ipairs(instruments) do
            if arp.states[ti] then
                local port_rec = engine.output_ports[inst.id]
                if port_rec then arp.tick(ti, cur, port_rec) end
            end
        end
    end

    -- 3. Sync UI state from engine.
    ui.playing   = engine.playing
    ui.recording = engine.recording
    ui.bpm       = engine.bpm

    -- 4. Non-blocking key read (10 ms timeout keeps timing granularity tight).
    local key = tui.read_key(10)
    if key then
        if key == 'esc' then
            running = false

        elseif key == ' ' then
            if engine.playing then
                engine.stop_transport()
                ui.set_status("Stopped")
            else
                engine.start_transport()
                ui.set_status("Playing")
            end

        elseif key == 'enter' or key == 'return' then
            trigger_clip()

        elseif key == 'up'    then ui.move_cursor(0, -1)
        elseif key == 'down'  then ui.move_cursor(0,  1)
        elseif key == 'left'  then ui.move_cursor(-1, 0)
        elseif key == 'right' then ui.move_cursor( 1, 0)

        elseif key == 'r' or key == 'R' then toggle_record()
        elseif key == 'e' or key == 'E' then edit_clip_piano_roll()
        elseif key == 'c' or key == 'C' then edit_clip_settings()
        elseif key == 'n' or key == 'N' then new_clip()
        elseif key == 'd' or key == 'D' then delete_clip()
        elseif key == 'i' or key == 'I' then edit_instrument()
        elseif key == 'b' or key == 'B' then edit_bpm()
        elseif key == 'p' or key == 'P' then toggle_arp()
        elseif key == 'a' or key == 'A' then cycle_arp_mode()
        elseif key == 'o' or key == 'O' then cycle_arp_rate()
        elseif key == 's' or key == 'S' then save_all()
        elseif key == 'f' or key == 'F' then rename_song()
        elseif key == '+' or key == '=' then add_track()
        elseif key == '-'               then remove_track()
        end
    end

    -- 4. Redraw at ~30 FPS.
    local now = os.monotime()
    if (now - last_draw) >= DRAW_INTERVAL then
        ui.draw(engine)
        last_draw = now
    end
end

-- ── Cleanup ───────────────────────────────────────────────────────────────────

-- Disable all arps (sends note-off for any active arp note).
for ti, inst in ipairs(instruments) do
    if arp.states[ti] then
        arp.disable(ti, engine.output_ports[inst.id])
    end
end

engine.stop_transport()
engine.close_all()
save_all()
db:close()
tui.exit_alt()
tui.show_cursor()
tui.cleanup()
print("Session saved to " .. song_path .. ". Goodbye!")

-- sequencer/main.lua
-- Entry point: rlua sequencer/main.lua [song.db]
--
-- Keyboard controls
--   Arrow keys       Navigate the session grid
--   SPACE            Start / stop transport
--   ENTER            Trigger (launch) the clip under the cursor
--   ESC              Stop the clip on the current track
--   R                Start / stop recording on the current track
--   N                New empty clip in current slot
--   D                Delete clip in current slot
--   I                Edit current instrument (name, MIDI device, channel)
--   B                Set BPM
--   +  (or =)        Add a new track
--   -                Remove the rightmost track (if empty)
--   S                Save to database
--   Q                Quit (auto-saves first)

local script_dir = (arg and arg[0]) and arg[0]:match("(.*/)") or "./"
local function req(name)
    return dofile(script_dir .. name .. ".lua")
end

local db_mod = req("db")
local engine = req("engine")
local ui     = req("ui")

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

-- Ensure scene rows exist.
local scenes = db_mod.ensure_scenes(db, song.id, NUM_SLOTS)

-- ── Open MIDI outputs ─────────────────────────────────────────────────────────

engine.bpm = song.bpm
for _, inst in ipairs(instruments) do
    engine.open_output(inst)
end

-- ── Wire UI state ─────────────────────────────────────────────────────────────

ui.song        = song
ui.instruments = instruments
ui.clips       = clips
ui.scenes      = scenes
ui.num_slots   = NUM_SLOTS
ui.bpm         = song.bpm

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
    table.insert(instruments, inst)
    ui.cursor.track = #instruments
    engine.open_output(inst)
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

local function edit_instrument()
    local inst = current_inst()
    if not inst then return end
    local h = select(2, tui.size())

    tui.clear()

    -- Show available MIDI devices.
    local dests = midi.destinations() or {}
    local srcs  = midi.sources()      or {}
    tui.color(tui.BRIGHT_YELLOW, tui.BLACK, tui.BOLD)
    tui.print_at(1, 1, " Instrument Settings — ESC to cancel each field ")
    tui.color(tui.YELLOW, tui.BLACK, 0)
    tui.print_at(3, 2, "MIDI Outputs:")
    for i, d in ipairs(dests) do
        tui.color(tui.WHITE, tui.BLACK, 0)
        tui.print_at(3 + i, 4, string.format("%d: %s", i, d))
    end
    local src_top = 3 + #dests + 1
    tui.color(tui.YELLOW, tui.BLACK, 0)
    tui.print_at(src_top, 2, "MIDI Inputs:")
    for i, s in ipairs(srcs) do
        tui.color(tui.WHITE, tui.BLACK, 0)
        tui.print_at(src_top + i, 4, string.format("%d: %s", i, s))
    end
    tui.flush()

    local edit_row = src_top + #srcs + 2

    local function field(label, key, current)
        local val = ui.read_line(edit_row, label, tostring(current or ''))
        edit_row = edit_row + 1
        if val == nil then return end  -- ESC = keep current
        if key == 'midi_channel' then
            inst[key] = math.max(1, math.min(16, tonumber(val) or inst[key]))
        else
            inst[key] = val
        end
    end

    field("Name          ", "name",         inst.name)
    field("MIDI Output   ", "midi_output",  inst.midi_output  or '')
    field("MIDI Input    ", "midi_input",   inst.midi_input   or '')
    field("Channel (1-16)", "midi_channel", inst.midi_channel or 1)

    db_mod.upsert_instrument(db, inst, song.id)
    -- Re-open output with updated settings.
    engine.output_ports[inst.id] = nil
    engine.open_output(inst)
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

-- ── Main loop ─────────────────────────────────────────────────────────────────

local running       = true
local DRAW_INTERVAL = 1.0 / 30   -- target ~30 FPS
local last_draw     = 0

while running do
    -- 1. Engine tick (dispatch scheduled MIDI events).
    engine.tick()
    engine.process_input()

    -- 2. Sync UI state from engine.
    ui.playing   = engine.playing
    ui.recording = engine.recording
    ui.bpm       = engine.bpm

    -- 3. Non-blocking key read (10 ms timeout keeps timing granularity tight).
    local key = tui.read_key(10)
    if key then
        if key == 'q' or key == 'Q' then
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

        elseif key == 'escape' then
            engine.stop_track(ui.cursor.track)
            ui.set_status("Track " .. ui.cursor.track .. " stopped")

        elseif key == 'up'    then ui.move_cursor(0, -1)
        elseif key == 'down'  then ui.move_cursor(0,  1)
        elseif key == 'left'  then ui.move_cursor(-1, 0)
        elseif key == 'right' then ui.move_cursor( 1, 0)

        elseif key == 'r' or key == 'R' then toggle_record()
        elseif key == 'n' or key == 'N' then new_clip()
        elseif key == 'd' or key == 'D' then delete_clip()
        elseif key == 'i' or key == 'I' then edit_instrument()
        elseif key == 'b' or key == 'B' then edit_bpm()
        elseif key == 's' or key == 'S' then save_all()
        elseif key == 'f' or key == 'F' then rename_song()
        elseif key == '+' or key == '=' then add_track()
        elseif key == '-'               then remove_track()
        end
    end

    -- 4. Redraw at ~30 FPS.
    local now = os.clock()
    if (now - last_draw) >= DRAW_INTERVAL then
        ui.draw(engine)
        last_draw = now
    end
end

-- ── Cleanup ───────────────────────────────────────────────────────────────────

engine.stop_transport()
engine.close_all()
save_all()
db:close()
tui.exit_alt()
tui.show_cursor()
tui.cleanup()
print("Session saved to " .. song_path .. ". Goodbye!")

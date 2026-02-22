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
--   L                Launch all clips in the current scene row
--   [                Insert a scene row at the cursor position
--   ]                Delete the current scene row
--   I                Instruments page (list, add, edit, delete instruments)
--   W                Studios page (named MIDI routing profiles)
--   T                Edit current track (name + instrument assignment)
--   B                Set BPM
--   F                Rename song
--   P                Toggle arpeggiator on/off for current track
--   A                Cycle arp mode  (when arp is on)
--   O                Cycle arp rate  (when arp is on)
--   +  (or =)        Add a new sequencer track
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

-- ── Load instruments (standalone MIDI device/patch definitions) ───────────────

local instruments = db_mod.get_instruments(db, song.id)
local instruments_by_name = {}
for _, inst in ipairs(instruments) do
    instruments_by_name[inst.name] = inst
end

-- ── Load tracks (sequencer lanes that reference an instrument by name) ────────

local tracks = db_mod.get_tracks(db, song.id)

-- Fresh song: create 2 default instruments + 2 tracks.
if #instruments == 0 and #tracks == 0 then
    local dests       = midi.destinations() or {}
    local default_out = (dests[1] and dests[1].name) or ''
    local srcs        = midi.sources()      or {}
    local default_in  = (srcs[1]  and srcs[1].name)  or ''

    local i1 = db_mod.upsert_instrument(db, {
        name = "Synth", midi_output = default_out, midi_input = default_in,
        midi_channel = 1, color = 2,
    }, song.id)
    local i2 = db_mod.upsert_instrument(db, {
        name = "Bass", midi_output = default_out, midi_input = default_in,
        midi_channel = 2, color = 4,
    }, song.id)
    instruments = { i1, i2 }
    instruments_by_name = { ["Synth"] = i1, ["Bass"] = i2 }

    local t1 = db_mod.upsert_track(db, {
        name = "Synth", track_index = 0, instrument_name = "Synth", color = 2,
    }, song.id)
    local t2 = db_mod.upsert_track(db, {
        name = "Bass", track_index = 1, instrument_name = "Bass", color = 4,
    }, song.id)
    tracks = { t1, t2 }
end

-- ── Load clips and MIDI events ────────────────────────────────────────────────
-- clips[track.id][slot_idx] = clip_table
-- events[clip_id] = { {beat_offset,status,data1,data2}, ... }
local clips  = {}
local events = {}
for _, track in ipairs(tracks) do
    clips[track.id] = {}
    for _, clip in ipairs(db_mod.get_clips(db, track.id)) do
        clips[track.id][clip.slot_index] = clip
        events[clip.id] = db_mod.get_events(db, clip.id)
    end
end

-- ── Load SysEx dumps per instrument ──────────────────────────────────────────
local sysex_dumps = {}
for _, inst in ipairs(instruments) do
    sysex_dumps[inst.id] = db_mod.get_sysex_dumps(db, inst.id)
end

-- ── Load drum maps per instrument ────────────────────────────────────────────
-- drum_maps[inst.id] = {[note_number] = name_string}
local drum_maps = {}
local function reload_drum_map(inst)
    local entries = db_mod.get_drum_map(db, inst.id)
    local dm = {}
    for _, e in ipairs(entries) do dm[e.note] = e.name end
    drum_maps[inst.id] = dm
end
for _, inst in ipairs(instruments) do
    reload_drum_map(inst)
end

-- Ensure at least 8 scene rows exist; NUM_SLOTS is dynamic after that.
local scenes    = db_mod.ensure_scenes(db, song.id, 8)
local NUM_SLOTS = #scenes

-- ── Load studios ──────────────────────────────────────────────────────────────

local studios        = db_mod.get_studios(db)
local studios_by_id  = {}
local default_studio
for _, s in ipairs(studios) do
    studios_by_id[s.id] = s
    if s.name == "all" then default_studio = s end
end
local current_studio = studios_by_id[song.studio_id] or default_studio

local function load_studio_entries(studio)
    local entries = {}
    for _, e in ipairs(db_mod.get_studio_instruments(db, studio.id)) do
        entries[e.instrument_name] = e
    end
    studio.entries = entries
end
if current_studio then load_studio_entries(current_studio) end

-- ── Routing helpers ───────────────────────────────────────────────────────────

local function effective_routing(inst)
    local ov = current_studio and current_studio.entries
               and current_studio.entries[inst.name]
    if ov then
        return ov.port_override or inst.midi_output,
               ov.channel_override or inst.midi_channel,
               ov.is_live ~= 0
    end
    return inst.midi_output, inst.midi_channel, true
end

-- ── Open MIDI outputs ─────────────────────────────────────────────────────────

-- Open output port and send any 'connect' sysex dumps.
local function open_and_configure_output(inst)
    local port, ch, live = effective_routing(inst)
    if not live then return true end
    local resolved = { id = inst.id, midi_output = port, midi_channel = ch }
    local ok, err = engine.open_output(resolved)
    if not ok then return false, err end
    for _, dump in ipairs(sysex_dumps[inst.id] or {}) do
        engine.send_sysex(inst.id, dump.data)
    end
    return true
end

local function apply_studio()
    for _, inst in ipairs(instruments) do
        engine.close_output(inst.id)
        open_and_configure_output(inst)
    end
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
ui.tracks      = tracks
ui.clips       = clips
ui.scenes      = scenes
ui.num_slots   = NUM_SLOTS
ui.bpm         = song.bpm
ui.arp_states  = arp.states  -- live reference so ui.draw sees arp status
ui.studio_name = current_studio and current_studio.name or "all"

-- ── TUI setup ─────────────────────────────────────────────────────────────────

tui.init()
tui.enter_alt()
tui.raw()
tui.hide_cursor()
tui.clear()

-- ── Splash screen ─────────────────────────────────────────────────────────────

do
    local VERSION   = "v0.1.0"
    local COPYRIGHT = "© 2026 Rhodium Audio"
    local LINES = {
        "",
        "  Sequencer Sharp Rhodium  ",
        "",
        "  " .. VERSION .. "  ",
        "",
        "  " .. COPYRIGHT .. "  ",
        "",
    }

    local w, h   = tui.size()
    local bw     = math.floor(w * 0.5)
    local bh     = math.floor(h * 0.5)
    local bx     = math.floor((w - bw) / 2) + 1
    local by     = math.floor((h - bh) / 2) + 1

    -- Draw box background.
    for row = 0, bh - 1 do
        tui.print_at(by + row, bx, string.rep(' ', bw), tui.WHITE, tui.BRIGHT_BLACK, 0)
    end

    -- Top and bottom border.
    tui.print_at(by,          bx, string.rep('─', bw), tui.BRIGHT_WHITE, tui.BRIGHT_BLACK, tui.BOLD)
    tui.print_at(by + bh - 1, bx, string.rep('─', bw), tui.BRIGHT_WHITE, tui.BRIGHT_BLACK, tui.BOLD)

    -- Content lines, centred vertically within the box.
    local content_start = by + math.floor((bh - #LINES) / 2)
    for i, line in ipairs(LINES) do
        local row = content_start + i - 1
        if row > by and row < by + bh - 1 then
            -- Centre the text horizontally within the box.
            local text = line
            local pad_l = math.floor((bw - #text) / 2)
            local pad_r = bw - #text - pad_l
            local display = string.rep(' ', pad_l) .. text .. string.rep(' ', pad_r)
            local is_title = (i == 2)
            tui.print_at(row, bx, display,
                is_title and tui.WHITE or tui.BRIGHT_WHITE,
                tui.BRIGHT_BLACK,
                is_title and tui.BOLD or 0)
        end
    end

    tui.flush()
    tui.read_key(2000)
    tui.clear()
end

-- ── Helpers ───────────────────────────────────────────────────────────────────

-- Tick engine + arp while a sub-page is open so MIDI keeps playing.
local function tick_all()
    engine.tick()
    engine.process_input()
    if engine.playing then
        local cur = engine.cur_beat()
        for ti, track in ipairs(tracks) do
            if arp.states[ti] then
                local inst     = instruments_by_name[track.instrument_name]
                local port_rec = inst and engine.output_ports[inst.id]
                if port_rec then arp.tick(ti, cur, port_rec) end
            end
        end
    end
end

-- Drop-in replacement for tui.read_key() that keeps the engine ticking.
-- Returns a key string when one arrives, nil is never returned to callers.
local function read_key_live()
    while true do
        local key = tui.read_key(10)
        if key then return key end
        tick_all()
    end
end

ui.on_idle         = tick_all
piano_roll.on_idle = tick_all

local function current_track()
    return tracks[ui.cursor.track]
end

local function current_inst()
    local track = current_track()
    if not track then return nil end
    return instruments_by_name[track.instrument_name]
end

local function current_clip()
    local track = current_track()
    if not track then return nil end
    return clips[track.id] and clips[track.id][ui.cursor.slot]
end

local function save_all()
    db_mod.save_song(db, song)
    for _, inst in ipairs(instruments) do
        db_mod.upsert_instrument(db, inst, song.id)
    end
    for _, track in ipairs(tracks) do
        db_mod.upsert_track(db, track, song.id)
    end
    for _, track in ipairs(tracks) do
        local tc = clips[track.id] or {}
        for _, clip in pairs(tc) do
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
    local track = current_track()
    if not track then return end
    local inst = current_inst()
    local clip = current_clip()
    if not clip then
        ui.set_status("Empty slot — press N to create a clip", tui.YELLOW)
        return
    end
    local evts = events[clip.id] or {}
    if clip.bank_msb or clip.bank_lsb or clip.program then
        if inst then engine.send_clip_patch(inst, clip) end
    end
    engine.launch(ui.cursor.track, inst and inst.id, clip, evts)
    if not engine.playing then
        engine.start_transport()
    end
    ui.set_status("► " .. (clip.name ~= '' and clip.name or "Clip " .. ui.cursor.slot))
end

local function new_clip()
    local track = current_track()
    if not track then return end
    local si = ui.cursor.slot
    if not clips[track.id] then clips[track.id] = {} end
    if clips[track.id][si] then
        ui.set_status("Slot occupied — press D to delete first", tui.YELLOW)
        return
    end
    local clip = { instrument_id = track.id, slot_index = si,
                   name = '', length_beats = 4.0, is_looping = true }
    db_mod.upsert_clip(db, clip)
    clips[track.id][si] = clip
    events[clip.id] = {}
    ui.set_status("New clip created in slot " .. si)
end

local function delete_clip()
    local track = current_track()
    if not track then return end
    local clip = current_clip()
    if not clip then
        ui.set_status("No clip here", tui.YELLOW)
        return
    end
    engine.stop_track(ui.cursor.track)
    db_mod.delete_clip(db, clip.id)
    events[clip.id] = nil
    clips[track.id][ui.cursor.slot] = nil
    ui.set_status("Clip deleted")
end

local function toggle_record()
    if engine.recording then
        -- ── Stop recording ────────────────────────────────────────────────────
        local track_idx, rec_evts, length = engine.stop_record()
        ui.recording = false
        local track = tracks[track_idx]
        if not track then return end
        local si = ui.cursor.slot
        if not clips[track.id] then clips[track.id] = {} end
        local clip = clips[track.id][si]
        if not clip then
            clip = { instrument_id = track.id, slot_index = si,
                     name = '', length_beats = length, is_looping = true }
            db_mod.upsert_clip(db, clip)
            clips[track.id][si] = clip
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
        if not inst then
            ui.set_status("No instrument assigned to this track — press T to assign", tui.YELLOW)
            return
        end
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
    local default_inst_name = (#instruments > 0) and instruments[1].name or ''
    local track = {
        name            = "Track " .. (#tracks + 1),
        track_index     = #tracks,
        instrument_name = default_inst_name,
        color           = 0,
    }
    db_mod.upsert_track(db, track, song.id)
    clips[track.id] = {}
    table.insert(tracks, track)
    ui.cursor.track = #tracks
    ui.set_status("Added track: " .. track.name)
end

local function remove_track()
    if #tracks <= 1 then
        ui.set_status("Cannot remove the last track", tui.YELLOW)
        return
    end
    local track = tracks[#tracks]
    local has_clip = false
    for _ in pairs(clips[track.id] or {}) do has_clip = true; break end
    if has_clip then
        ui.set_status("Track has clips — delete clips first", tui.YELLOW)
        return
    end
    engine.stop_track(#tracks)
    db_mod.delete_track(db, track.id)
    clips[track.id] = nil
    table.remove(tracks)
    ui.cursor.track = math.min(ui.cursor.track, #tracks)
    ui.set_status("Track removed")
end

local function edit_clip_settings()
    local clip = current_clip()
    if not clip then
        ui.set_status("No clip here — press N to create one first", tui.YELLOW)
        return
    end

    local function fmt_f(v)
        if v == nil then return '' end
        local s = string.format("%.3f", v)
        return s:gsub("%.?0+$", "")
    end

    local loop_opts = {"No", "Yes"}
    local fields = {
        { label="Name",        type="text"                                        },
        { label="Length",      type="float", min=0.01, nullable=false             },
        { label="Loopable",    type="toggle", opts=loop_opts                      },
        { label="Start",       type="float", min=0,    nullable=false, hint="beats" },
        { label="Loop Start",  type="float", min=0,    nullable=false, hint="beats" },
        { label="Loop Length", type="float", min=0.01, nullable=true,  hint="blank=full" },
        { label="Bank MSB",    type="int",   min=0,    max=127, nullable=true     },
        { label="Bank LSB",    type="int",   min=0,    max=127, nullable=true     },
        { label="Program",     type="int",   min=0,    max=127, nullable=true     },
    }

    local is_looping = (clip.is_looping ~= 0 and clip.is_looping ~= false)
    local vals = {
        clip.name or '',
        fmt_f(clip.length_beats or 4.0),
        is_looping and 2 or 1,
        fmt_f(clip.start_offset or 0),
        fmt_f(clip.loop_start   or 0),
        clip.loop_length ~= nil and fmt_f(clip.loop_length) or '',
        clip.bank_msb ~= nil and tostring(clip.bank_msb) or '',
        clip.bank_lsb ~= nil and tostring(clip.bank_lsb) or '',
        clip.program  ~= nil and tostring(clip.program)  or '',
    }

    local LABEL_W  = 14
    local VAL_COL  = LABEL_W + 5
    local sel      = 1
    local editing  = false
    local edit_buf = ''

    local function pad(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function draw()
        local w, h = tui.size()
        local val_w = math.max(10, w - VAL_COL - 4)
        tui.hide_cursor()
        tui.clear()

        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Clip Settings", tui.WHITE, tui.BLUE, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.WHITE, tui.BRIGHT_BLACK, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLACK, 0)
            end

            if f.type == "toggle" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                end
            else
                local hint = f.hint and ("  " .. f.hint) or ""
                if is_sel and editing then
                    local display = pad(edit_buf, val_w)
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                    tui.move(row, VAL_COL + 2 + math.min(#edit_buf, val_w))
                    tui.show_cursor()
                else
                    local display = pad(vals[i] .. hint, val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.CYAN, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  S=save  ESC=cancel", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto clip_again end

        if editing then
            if key == 'enter' or key == 'return' then
                local f = fields[sel]
                if f.type == "float" then
                    local n = tonumber(edit_buf)
                    if edit_buf == '' and f.nullable then
                        vals[sel] = ''
                    elseif n then
                        local clamped = math.max(f.min, n)
                        vals[sel] = (clamped == math.floor(clamped))
                            and tostring(math.floor(clamped)) or fmt_f(clamped)
                    end
                elseif f.type == "int" then
                    local n = tonumber(edit_buf)
                    if edit_buf == '' and f.nullable then
                        vals[sel] = ''
                    elseif n then
                        vals[sel] = tostring(math.max(f.min, math.min(f.max, math.floor(n))))
                    end
                else
                    vals[sel] = edit_buf
                end
                editing = false
                sel = (sel % #fields) + 1
            elseif key == 'esc' then
                editing = false
            elseif key == 'backspace' then
                if #edit_buf > 0 then edit_buf = edit_buf:sub(1, -2) end
            elseif #key == 1 then
                edit_buf = edit_buf .. key
            end
        else
            if key == 'esc' then
                tui.clear()
                return
            elseif key == 's' or key == 'S' then
                break
            elseif key == 'tab' or key == 'down' then
                sel = (sel % #fields) + 1
            elseif key == 'up' then
                sel = ((sel - 2 + #fields) % #fields) + 1
            elseif key == 'left' or key == 'right' then
                local f = fields[sel]
                if f.type == "toggle" then
                    local n = #f.opts
                    vals[sel] = key == 'right' and (vals[sel] % n) + 1
                                                or ((vals[sel] - 2 + n) % n) + 1
                end
            elseif key == 'enter' or key == 'return' then
                if fields[sel].type ~= "toggle" then
                    editing  = true
                    edit_buf = vals[sel]
                end
            end
        end
        draw()
        ::clip_again::
    end

    local function parse_float(s, default, mn)
        local n = tonumber(s)
        if n then return math.max(mn or 0, n) end
        return default
    end
    local function parse_opt_int(s, mn, mx)
        if s == '' then return nil end
        local n = tonumber(s)
        return n and math.max(mn, math.min(mx, math.floor(n))) or nil
    end

    if vals[1] ~= '' then clip.name = vals[1] end
    clip.length_beats  = parse_float(vals[2], clip.length_beats or 4.0, 0.01)
    clip.is_looping    = (loop_opts[vals[3]] == "Yes") and 1 or 0
    clip.start_offset  = parse_float(vals[4], 0, 0)
    clip.loop_start    = parse_float(vals[5], 0, 0)
    clip.loop_length   = vals[6] ~= '' and parse_float(vals[6], nil, 0.01) or nil
    clip.bank_msb      = parse_opt_int(vals[7], 0, 127)
    clip.bank_lsb      = parse_opt_int(vals[8], 0, 127)
    clip.program       = parse_opt_int(vals[9], 0, 127)

    db_mod.upsert_clip(db, clip)
    tui.clear()
    ui.set_status("Clip settings saved")
end

-- SysEx dump manager for an instrument.
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
        local key = read_key_live()
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

-- Drum map editor: note → name label assignments for a drum instrument.
local function edit_drum_map(inst)
    local function load_entries()
        local entries = {}
        for note, name in pairs(drum_maps[inst.id] or {}) do
            entries[#entries+1] = { note = note, name = name }
        end
        table.sort(entries, function(a, b) return a.note < b.note end)
        return entries
    end

    local function save_entries(entries)
        db_mod.save_drum_map(db, inst.id, entries)
        reload_drum_map(inst)
    end

    local function parse_note_input(s)
        s = (s or ''):match("^%s*(.-)%s*$")
        local n = tonumber(s)
        if n then return math.floor(math.max(0, math.min(127, n))) end
        local note_vals = {C=0,D=2,E=4,F=5,G=7,A=9,B=11}
        local letter, mod, oct = s:upper():match("^([A-G])([#B]?)(-?%d+)$")
        if letter and note_vals[letter] then
            local semi = note_vals[letter]
            if mod == '#' then semi = semi + 1 elseif mod == 'B' then semi = semi - 1 end
            return math.max(0, math.min(127, (tonumber(oct) + 1) * 12 + semi))
        end
        return nil
    end

    local entries = load_entries()
    local sel = math.max(1, #entries)

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Drum Map — " .. inst.name, tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  E/Enter=rename  D=delete  ESC=done",
            tui.BRIGHT_BLACK, tui.BLACK, 0)
        if #entries == 0 then
            tui.print_at(5, 2, "(no entries — press A to add)", tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            for i, e in ipairs(entries) do
                local nn = ({"C","C#","D","D#","E","F","F#","G","G#","A","A#","B"})[(e.note % 12) + 1]
                local note_s = string.format("%-3d  %-3s%-2d",
                    e.note, nn, math.floor(e.note / 12) - 1)
                local label = string.format(" %s  %s", note_s, e.name)
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
        local key = read_key_live()
        if key == 'esc' then
            break
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #entries), sel + 1)
        elseif key == 'a' or key == 'A' then
            tui.clear()
            local note_s = ui.read_line(3, "Note (0-127 or name e.g. C2): ", '')
            if note_s then
                local note = parse_note_input(note_s)
                if note then
                    tui.clear()
                    local name_s = ui.read_line(3, "Name: ", '')
                    if name_s and name_s ~= '' then
                        for i = #entries, 1, -1 do
                            if entries[i].note == note then table.remove(entries, i) end
                        end
                        entries[#entries+1] = { note = note, name = name_s }
                        table.sort(entries, function(a, b) return a.note < b.note end)
                        for i, e in ipairs(entries) do
                            if e.note == note then sel = i; break end
                        end
                        save_entries(entries)
                    end
                else
                    ui.set_status("Invalid note — use 0-127 or name like C2", tui.YELLOW)
                end
            end
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #entries > 0 then
            tui.clear()
            local e = entries[sel]
            local name_s = ui.read_line(3, "Name: ", e.name)
            if name_s and name_s ~= '' then
                e.name = name_s
                save_entries(entries)
            end
        elseif (key == 'd' or key == 'D') and #entries > 0 then
            table.remove(entries, sel)
            sel = math.max(1, math.min(sel, #entries))
            save_entries(entries)
        end
        draw()
    end
    tui.clear()
end

-- Edit an instrument's definition (MIDI settings, name, patch).
-- Accepts the instrument object directly.
local function edit_instrument_def(inst)
    local old_name = inst.name

    local dests = midi.destinations() or {}
    local srcs  = midi.sources()      or {}

    local dest_opts = {"None"}
    for _, d in ipairs(dests) do dest_opts[#dest_opts+1] = d.name end

    local src_opts = {"None"}
    for _, s in ipairs(srcs) do src_opts[#src_opts+1] = s.name end

    local function opt_index(opts, val)
        for i, v in ipairs(opts) do
            if v == (val or '') then return i end
        end
        return 1
    end

    local type_opts = {"keyboard", "drum"}
    local function type_index(t)
        for i, v in ipairs(type_opts) do if v == (t or 'keyboard') then return i end end
        return 1
    end

    local fields = {
        { label="Name",        type="text"                                     },
        { label="Type",        type="picker", opts=type_opts                   },
        { label="MIDI Output", type="picker", opts=dest_opts                   },
        { label="MIDI Input",  type="picker", opts=src_opts                    },
        { label="Channel",     type="number", min=1,   max=16,  nullable=false },
        { label="Bank MSB",    type="number", min=0,   max=127, nullable=true  },
        { label="Bank LSB",    type="number", min=0,   max=127, nullable=true  },
        { label="Program",     type="number", min=0,   max=127, nullable=true  },
    }

    local vals = {
        inst.name or '',
        type_index(inst.type),
        opt_index(dest_opts, inst.midi_output or ''),
        opt_index(src_opts,  inst.midi_input  or ''),
        tostring(inst.midi_channel or 1),
        inst.bank_msb ~= nil and tostring(inst.bank_msb) or '',
        inst.bank_lsb ~= nil and tostring(inst.bank_lsb) or '',
        inst.program  ~= nil and tostring(inst.program)  or '',
    }

    local LABEL_W    = 14
    local VAL_COL    = LABEL_W + 5
    local sel        = 1
    local editing    = false
    local edit_buf   = ''
    local do_sysex   = false
    local do_drummap = false

    local function pad(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function draw()
        local w, h = tui.size()
        local val_w = math.max(10, w - VAL_COL - 4)
        tui.hide_cursor()
        tui.clear()

        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Instrument: " .. inst.name, tui.WHITE, tui.BLUE, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.WHITE, tui.BRIGHT_BLACK, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLACK, 0)
            end

            if f.type == "picker" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                end
            else
                if is_sel and editing then
                    local display = pad(edit_buf, val_w)
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                    tui.move(row, VAL_COL + 2 + math.min(#edit_buf, val_w))
                    tui.show_cursor()
                else
                    local display = pad(vals[i], val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.CYAN, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            local drum_hint = (type_opts[vals[2]] == 'drum') and "  M=drum map" or ""
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle options  Enter=edit  S=save  X=sysex" .. drum_hint .. "  ESC=cancel", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto inst_def_again end

        if editing then
            if key == 'enter' or key == 'return' then
                local f = fields[sel]
                if f.type == "number" then
                    local n = tonumber(edit_buf)
                    if edit_buf == '' and f.nullable then
                        vals[sel] = ''
                    elseif n then
                        vals[sel] = tostring(math.max(f.min, math.min(f.max, math.floor(n))))
                    end
                else
                    vals[sel] = edit_buf
                end
                editing = false
                sel = (sel % #fields) + 1
            elseif key == 'esc' then
                editing = false
            elseif key == 'backspace' then
                if #edit_buf > 0 then edit_buf = edit_buf:sub(1, -2) end
            elseif #key == 1 then
                edit_buf = edit_buf .. key
            end
        else
            if key == 'esc' then
                tui.clear()
                return
            elseif key == 's' or key == 'S' then
                break
            elseif key == 'x' or key == 'X' then
                do_sysex = true
                break
            elseif (key == 'm' or key == 'M') and type_opts[vals[2]] == 'drum' then
                do_drummap = true
                break
            elseif key == 'tab' or key == 'down' then
                sel = (sel % #fields) + 1
            elseif key == 'up' then
                sel = ((sel - 2 + #fields) % #fields) + 1
            elseif key == 'left' then
                if fields[sel].type == "picker" then
                    local n = #fields[sel].opts
                    vals[sel] = ((vals[sel] - 2 + n) % n) + 1
                end
            elseif key == 'right' then
                if fields[sel].type == "picker" then
                    local n = #fields[sel].opts
                    vals[sel] = (vals[sel] % n) + 1
                end
            elseif key == 'enter' or key == 'return' then
                if fields[sel].type ~= "picker" then
                    editing  = true
                    edit_buf = vals[sel]
                end
            end
        end
        draw()
        ::inst_def_again::
    end

    local function parse_opt_num(s, mn, mx)
        if s == '' then return nil end
        local n = tonumber(s)
        return n and math.max(mn, math.min(mx, math.floor(n))) or nil
    end

    local new_name        = vals[1] ~= '' and vals[1] or inst.name
    inst.type             = type_opts[vals[2]] or 'keyboard'
    inst.midi_output      = (dest_opts[vals[3]] == "None") and '' or (dest_opts[vals[3]] or '')
    inst.midi_input       = (src_opts[vals[4]]  == "None") and '' or (src_opts[vals[4]]  or '')
    inst.midi_channel     = math.max(1, math.min(16, tonumber(vals[5]) or 1))
    inst.bank_msb         = parse_opt_num(vals[6], 0, 127)
    inst.bank_lsb         = parse_opt_num(vals[7], 0, 127)
    inst.program          = parse_opt_num(vals[8], 0, 127)

    -- Handle name change: update instruments_by_name and all track references.
    if new_name ~= old_name then
        instruments_by_name[old_name] = nil
        inst.name = new_name
        instruments_by_name[new_name] = inst
        for _, track in ipairs(tracks) do
            if track.instrument_name == old_name then
                track.instrument_name = new_name
                db_mod.upsert_track(db, track, song.id)
            end
        end
    end

    db_mod.upsert_instrument(db, inst, song.id)
    engine.output_ports[inst.id] = nil
    open_and_configure_output(inst)

    if do_sysex then
        edit_sysex(inst)
    end
    if do_drummap then
        edit_drum_map(inst)
    end

    tui.clear()
    ui.set_status("Instrument updated: " .. inst.name)
end

-- Full-screen instruments list page.
local function instruments_page()
    local sel = math.max(1, #instruments > 0 and 1 or 0)

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()

        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Instruments", tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  E/Enter=edit  A=add  D=delete  X=sysex  ESC=close",
            tui.BRIGHT_BLACK, tui.BLACK, 0)

        if #instruments == 0 then
            tui.print_at(5, 2, "(no instruments — press A to add one)", tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            for i, inst in ipairs(instruments) do
                local label = string.format(" %-18s  ch:%-2d  out: %s",
                    inst.name or '',
                    inst.midi_channel or 1,
                    inst.midi_output or '')
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
        local key = read_key_live()
        if not key then goto inst_page_again end

        if key == 'esc' then
            break
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #instruments), sel + 1)
        elseif key == 'e' or key == 'E' or key == 'enter' or key == 'return' then
            if instruments[sel] then
                tui.clear()
                edit_instrument_def(instruments[sel])
                sel = math.max(1, math.min(sel, #instruments))
            end
        elseif key == 'a' or key == 'A' then
            local dests       = midi.destinations() or {}
            local default_out = (dests[1] and dests[1].name) or ''
            local srcs        = midi.sources()      or {}
            local default_in  = (srcs[1]  and srcs[1].name)  or ''
            local inst = {
                name         = "Inst " .. (#instruments + 1),
                midi_output  = default_out,
                midi_input   = default_in,
                midi_channel = math.min(16, #instruments + 1),
                color        = 0,
            }
            db_mod.upsert_instrument(db, inst, song.id)
            sysex_dumps[inst.id] = {}
            instruments_by_name[inst.name] = inst
            table.insert(instruments, inst)
            open_and_configure_output(inst)
            sel = #instruments
        elseif key == 'd' or key == 'D' then
            if #instruments > 0 then
                local inst = instruments[sel]
                -- Guard: check no track currently references this instrument.
                local in_use = false
                for _, track in ipairs(tracks) do
                    if track.instrument_name == inst.name then
                        in_use = true; break
                    end
                end
                if in_use then
                    ui.set_status("Instrument in use by a track — reassign tracks first (T)", tui.YELLOW)
                else
                    db_mod.delete_instrument(db, inst.id)
                    instruments_by_name[inst.name] = nil
                    sysex_dumps[inst.id] = nil
                    engine.output_ports[inst.id] = nil
                    table.remove(instruments, sel)
                    sel = math.max(1, math.min(sel, #instruments))
                    ui.set_status("Instrument deleted")
                end
            end
        elseif key == 'x' or key == 'X' then
            if instruments[sel] then
                tui.clear()
                edit_sysex(instruments[sel])
            end
        end
        draw()
        ::inst_page_again::
    end
    tui.clear()
end

-- Edit a studio's name and per-instrument routing overrides.
local function edit_studio(studio)
    local is_all = (studio.name == "all")

    -- Build fields: Name field + 3 fields per instrument.
    local fields = {
        { label = "Name", type = "text" },
    }
    for _, inst in ipairs(instruments) do
        fields[#fields+1] = { label = inst.name .. " Live", type = "toggle",
                              opts = {"yes","no"}, inst_name = inst.name, key = "live" }
        fields[#fields+1] = { label = inst.name .. " Port", type = "text",
                              inst_name = inst.name, key = "port" }
        fields[#fields+1] = { label = inst.name .. " Chan", type = "int",
                              min = 0, max = 16, inst_name = inst.name, key = "chan" }
    end

    -- Initial values.
    local vals = { studio.name }
    for _, inst in ipairs(instruments) do
        local ov  = studio.entries and studio.entries[inst.name]
        local live = ov and (ov.is_live ~= 0) or true
        vals[#vals+1] = live and 1 or 2           -- toggle: 1=yes, 2=no
        vals[#vals+1] = (ov and ov.port_override) or ''
        vals[#vals+1] = tostring((ov and ov.channel_override) or 0)
    end

    local LABEL_W = 22
    local VAL_COL = LABEL_W + 5
    local sel     = 1
    local editing = false
    local edit_buf = ''

    local function pad(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function draw()
        local w, h = tui.size()
        local val_w = math.max(10, w - VAL_COL - 4)
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Studio: " .. studio.name, tui.WHITE, tui.BLUE, tui.BOLD)

        local max_row = h - 3
        for i, f in ipairs(fields) do
            local row = i + 2
            if row > max_row then break end
            local is_sel = (i == sel)
            local lbl = string.format("  %-" .. LABEL_W .. "s  ", f.label)
            if is_sel then
                tui.print_at(row, 1, lbl, tui.WHITE, tui.BRIGHT_BLACK, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLACK, 0)
            end

            if f.type == "toggle" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                end
            else
                -- field 1 (Name) is read-only for "all"
                local locked = (i == 1 and is_all)
                if locked then
                    local display = pad(vals[i] .. "  (default, cannot rename)", val_w)
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_BLACK, tui.BLACK, 0)
                elseif is_sel and editing then
                    local display = pad(edit_buf, val_w)
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                    tui.move(row, VAL_COL + 2 + math.min(#edit_buf, val_w))
                    tui.show_cursor()
                else
                    local hint = (f.key == "chan") and "  (0=inst default)" or ""
                    local display = pad(vals[i] .. hint, val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.CYAN, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                    end
                end
            end
        end

        local hint_row = h - 1
        if editing then
            tui.print_at(hint_row, 1, pad("  Enter=confirm  ESC=cancel edit", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  S=save  ESC=cancel", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto studio_edit_again end

        if editing then
            if key == 'enter' or key == 'return' then
                local f = fields[sel]
                if f.type == "int" then
                    local n = tonumber(edit_buf)
                    if n then
                        vals[sel] = tostring(math.max(f.min, math.min(f.max, math.floor(n))))
                    end
                else
                    vals[sel] = edit_buf
                end
                editing = false
                sel = (sel % #fields) + 1
            elseif key == 'esc' then
                editing = false
            elseif key == 'backspace' then
                if #edit_buf > 0 then edit_buf = edit_buf:sub(1, -2) end
            elseif #key == 1 then
                edit_buf = edit_buf .. key
            end
        else
            if key == 'esc' then
                tui.clear()
                return
            elseif key == 's' or key == 'S' then
                break
            elseif key == 'tab' or key == 'down' then
                sel = (sel % #fields) + 1
            elseif key == 'up' then
                sel = ((sel - 2 + #fields) % #fields) + 1
            elseif key == 'left' or key == 'right' then
                local f = fields[sel]
                if f.type == "toggle" then
                    local n = #f.opts
                    vals[sel] = key == 'right' and (vals[sel] % n) + 1
                                                or ((vals[sel] - 2 + n) % n) + 1
                end
            elseif key == 'enter' or key == 'return' then
                local f = fields[sel]
                if f.type ~= "toggle" and not (sel == 1 and is_all) then
                    editing  = true
                    edit_buf = vals[sel]
                end
            end
        end
        draw()
        ::studio_edit_again::
    end

    -- Save name (unless it's "all").
    if not is_all and vals[1] ~= '' then
        studio.name = vals[1]
        db_mod.upsert_studio(db, studio)
    end

    -- Build entries list from fields (fields 2+ correspond to instruments, 3 fields each).
    local entries_list = {}
    for idx, inst in ipairs(instruments) do
        local base  = 1 + (idx - 1) * 3 + 1   -- index of the "Live" toggle for this inst
        local live_v = vals[base]               -- 1=yes, 2=no
        local port_v = vals[base + 1]
        local chan_v = tonumber(vals[base + 2]) or 0
        local is_live = (live_v == 1 or live_v == nil)
        local port_ov = (port_v ~= '') and port_v or nil
        local chan_ov = (chan_v > 0) and chan_v or nil
        -- Only persist entries that differ from defaults.
        if not is_live or port_ov or chan_ov then
            entries_list[#entries_list+1] = {
                instrument_name  = inst.name,
                is_live          = is_live and 1 or 0,
                port_override    = port_ov,
                channel_override = chan_ov,
            }
        end
    end

    db_mod.save_studio_instruments(db, studio.id, entries_list)
    load_studio_entries(studio)
    if studio.id == (current_studio and current_studio.id) then
        apply_studio()
    end
    tui.clear()
    ui.set_status("Studio saved: " .. studio.name)
end

-- Full-screen studios list page.
local function studios_page()
    local sel = 1

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        local cur_name = current_studio and current_studio.name or "all"
        tui.print_at(1, 2, "STUDIOS  (current: " .. cur_name .. ")", tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  ENTER=use for song  E=edit  A=add  D=delete  ESC=close",
            tui.BRIGHT_BLACK, tui.BLACK, 0)
        for i, s in ipairs(studios) do
            local is_cur = current_studio and (s.id == current_studio.id)
            local marker = is_cur and "►" or " "
            local label  = marker .. " " .. (s.name or '')
            if i == sel then
                tui.print_at(4 + i, 2, label, tui.BLACK, tui.CYAN, tui.BOLD)
            elseif is_cur then
                tui.print_at(4 + i, 2, label, tui.GREEN, tui.BLACK, tui.BOLD)
            else
                tui.print_at(4 + i, 2, label, tui.WHITE, tui.BLACK, 0)
            end
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto studios_again end

        if key == 'esc' then
            break
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #studios), sel + 1)
        elseif key == 'enter' or key == 'return' then
            -- Use selected studio for this song.
            local s = studios[sel]
            if s then
                current_studio   = s
                song.studio_id   = s.id
                load_studio_entries(current_studio)
                db_mod.save_song(db, song)
                apply_studio()
                ui.studio_name = current_studio.name
                ui.set_status("Studio → " .. s.name, tui.GREEN)
            end
        elseif key == 'e' or key == 'E' then
            if studios[sel] then
                tui.clear()
                edit_studio(studios[sel])
                -- Refresh name in case it changed.
                ui.studio_name = current_studio and current_studio.name or "all"
            end
        elseif key == 'a' or key == 'A' then
            tui.clear()
            local name_val = ui.read_line(3, "Studio name: ", '')
            if name_val and name_val ~= '' then
                local ns = { name = name_val }
                db_mod.upsert_studio(db, ns)
                load_studio_entries(ns)
                studios_by_id[ns.id] = ns
                table.insert(studios, ns)
                sel = #studios
                ui.set_status("Added studio: " .. name_val)
            end
        elseif key == 'd' or key == 'D' then
            local s = studios[sel]
            if s then
                if s.name == "all" then
                    ui.set_status("Cannot delete the default studio", tui.YELLOW)
                elseif current_studio and s.id == current_studio.id then
                    ui.set_status("Cannot delete the current studio — switch first", tui.YELLOW)
                else
                    db_mod.delete_studio(db, s.id)
                    studios_by_id[s.id] = nil
                    table.remove(studios, sel)
                    sel = math.max(1, math.min(sel, #studios))
                    ui.set_status("Studio deleted")
                end
            end
        end
        draw()
        ::studios_again::
    end
    tui.clear()
end

-- Edit the current track's name and instrument assignment.
local function edit_track()
    local track = current_track()
    if not track then return end

    -- Build picker list from current instruments.
    local inst_names = {}
    for _, inst in ipairs(instruments) do
        inst_names[#inst_names+1] = inst.name
    end
    if #inst_names == 0 then inst_names = {"(none)"} end

    local function name_index(name)
        for i, n in ipairs(inst_names) do
            if n == (name or '') then return i end
        end
        return 1
    end

    local fields = {
        { label="Track Name",  type="text"                       },
        { label="Instrument",  type="picker", opts=inst_names    },
    }

    local vals = {
        track.name or '',
        name_index(track.instrument_name),
    }

    local LABEL_W  = 14
    local VAL_COL  = LABEL_W + 5
    local sel      = 1
    local editing  = false
    local edit_buf = ''

    local function pad(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function draw()
        local w, h = tui.size()
        local val_w = math.max(10, w - VAL_COL - 4)
        tui.hide_cursor()
        tui.clear()

        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Track: " .. track.name, tui.WHITE, tui.BLUE, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.WHITE, tui.BRIGHT_BLACK, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLACK, 0)
            end

            if f.type == "picker" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.CYAN, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                end
            else
                if is_sel and editing then
                    local display = pad(edit_buf, val_w)
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                    tui.move(row, VAL_COL + 2 + math.min(#edit_buf, val_w))
                    tui.show_cursor()
                else
                    local display = pad(vals[i], val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.CYAN, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLACK, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  S=save  ESC=cancel", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto track_edit_again end

        if editing then
            if key == 'enter' or key == 'return' then
                vals[sel] = edit_buf
                editing = false
                sel = (sel % #fields) + 1
            elseif key == 'esc' then
                editing = false
            elseif key == 'backspace' then
                if #edit_buf > 0 then edit_buf = edit_buf:sub(1, -2) end
            elseif #key == 1 then
                edit_buf = edit_buf .. key
            end
        else
            if key == 'esc' then
                tui.clear()
                return
            elseif key == 's' or key == 'S' then
                break
            elseif key == 'tab' or key == 'down' then
                sel = (sel % #fields) + 1
            elseif key == 'up' then
                sel = ((sel - 2 + #fields) % #fields) + 1
            elseif key == 'left' then
                if fields[sel].type == "picker" then
                    local n = #fields[sel].opts
                    vals[sel] = ((vals[sel] - 2 + n) % n) + 1
                end
            elseif key == 'right' then
                if fields[sel].type == "picker" then
                    local n = #fields[sel].opts
                    vals[sel] = (vals[sel] % n) + 1
                end
            elseif key == 'enter' or key == 'return' then
                if fields[sel].type ~= "picker" then
                    editing  = true
                    edit_buf = vals[sel]
                end
            end
        end
        draw()
        ::track_edit_again::
    end

    if vals[1] ~= '' then track.name = vals[1] end
    track.instrument_name = inst_names[vals[2]] or track.instrument_name
    db_mod.upsert_track(db, track, song.id)
    tui.clear()
    ui.set_status("Track updated: " .. track.name)
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
    local track = current_track()
    local inst  = current_inst()
    local clip  = current_clip()
    if not clip then
        ui.set_status("No clip here — press N to create one first", tui.YELLOW)
        return
    end
    local dm = (inst and inst.type == 'drum') and drum_maps[inst.id] or nil
    local new_evts, new_len, new_loop = piano_roll.open(clip, events[clip.id] or {}, dm)
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

end

local function toggle_arp()
    local ti    = ui.cursor.track
    local track = current_track()
    local inst  = current_inst()
    if not track then return end
    if arp.states[ti] then
        arp.disable(ti, inst and engine.output_ports[inst.id])
        db_mod.save_arp_settings(db, track.id, { mode="up", octaves=1, rate=0.25, gate=0.8, hold=false })
        ui.set_status("Arp OFF — track " .. ti)
    else
        local settings = db_mod.get_arp_settings(db, track.id)
        arp.enable(ti, settings, engine.cur_beat())
        if inst and inst.midi_input and inst.midi_input ~= '' then
            engine.open_input(inst.midi_input)
        end
        if not engine.playing then engine.start_transport() end
        ui.set_status(string.format("Arp ON — track %d [%s]", ti, settings.mode or "up"))
    end
end

local function cycle_arp_mode()
    local ti    = ui.cursor.track
    local track = current_track()
    if not arp.states[ti] then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    local next_mode = arp.cycle(ti, "mode", arp.MODES)
    if track then db_mod.save_arp_settings(db, track.id, arp.get_state(ti)) end
    ui.set_status("Arp mode: " .. next_mode)
end

local function cycle_arp_rate()
    local ti    = ui.cursor.track
    local track = current_track()
    if not arp.states[ti] then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    local next_rate = arp.cycle(ti, "rate", arp.RATES)
    if track then db_mod.save_arp_settings(db, track.id, arp.get_state(ti)) end
    ui.set_status("Arp rate: " .. (arp.RATE_NAMES[next_rate] or tostring(next_rate)))
end

-- ── Scene (row) operations ────────────────────────────────────────────────────

-- Launch all clips in the current scene row across every track.
local function launch_scene()
    local si       = ui.cursor.slot
    local launched = 0
    for ti, track in ipairs(tracks) do
        local clip = clips[track.id] and clips[track.id][si]
        if clip then
            local inst = instruments_by_name[track.instrument_name]
            local evts = events[clip.id] or {}
            if clip.bank_msb or clip.bank_lsb or clip.program then
                if inst then engine.send_clip_patch(inst, clip) end
            end
            engine.launch(ti, inst and inst.id, clip, evts)
            launched = launched + 1
        end
    end
    if launched > 0 then
        if not engine.playing then engine.start_transport() end
        ui.set_status(string.format("► Scene %d (%d clips)", si, launched))
    else
        ui.set_status("Scene " .. si .. " is empty", tui.YELLOW)
    end
end

-- Insert an empty scene row at the cursor position, pushing rows below down.
local function insert_scene_row()
    local si = ui.cursor.slot

    -- Shift clips in memory (high-to-low to avoid self-overwrite).
    for _, track in ipairs(tracks) do
        local tc = clips[track.id]
        if tc then
            for slot = NUM_SLOTS, si, -1 do
                if tc[slot] then
                    tc[slot + 1]            = tc[slot]
                    tc[slot + 1].slot_index = slot + 1
                    tc[slot]                = nil
                end
            end
        end
    end

    -- Shift scenes in memory (scene_index is 0-based; si is 1-based).
    for _, scene in ipairs(scenes) do
        if scene.scene_index >= si - 1 then
            scene.scene_index = scene.scene_index + 1
        end
    end

    -- Persist to DB.
    db_mod.shift_clips(db, song.id, si, 1)
    db_mod.shift_scenes(db, song.id, si - 1, 1)
    local new_scene = db_mod.insert_scene_at(db, song.id, si - 1, '')
    table.insert(scenes, si, new_scene)

    NUM_SLOTS       = #scenes
    ui.num_slots    = NUM_SLOTS
    ui.set_status("Inserted scene row at " .. si)
end

-- Delete the current scene row, shifting rows below up.
local function delete_scene_row()
    if NUM_SLOTS <= 1 then
        ui.set_status("Cannot delete the last scene row", tui.YELLOW)
        return
    end
    local si = ui.cursor.slot

    -- Stop any clips playing on this row and clear in-memory data.
    for ti, track in ipairs(tracks) do
        local tc   = clips[track.id]
        local clip = tc and tc[si]
        if clip then
            local ac = engine.active_clips[ti]
            if ac and ac.clip_id == clip.id then engine.stop_track(ti) end
            events[clip.id] = nil
            tc[si] = nil
        end
    end

    -- Persist deletions to DB, then shift remaining rows up.
    db_mod.delete_clips_at_slot(db, song.id, si)
    db_mod.delete_scene_at(db, song.id, si - 1)
    db_mod.shift_clips(db, song.id, si + 1, -1)
    db_mod.shift_scenes(db, song.id, si, -1)

    -- Shift clips in memory (low-to-high).
    for _, track in ipairs(tracks) do
        local tc = clips[track.id]
        if tc then
            for slot = si + 1, NUM_SLOTS do
                tc[slot - 1] = tc[slot]
                if tc[slot - 1] then tc[slot - 1].slot_index = slot - 1 end
                tc[slot] = nil
            end
        end
    end

    -- Remove scene from memory.
    table.remove(scenes, si)

    NUM_SLOTS    = #scenes
    ui.num_slots = NUM_SLOTS
    if ui.cursor.slot > NUM_SLOTS then ui.cursor.slot = NUM_SLOTS end
    ui.set_status("Deleted scene row " .. si)
end

-- ── Main loop ─────────────────────────────────────────────────────────────────

local running       = true
local DRAW_INTERVAL = 1.0 / 30   -- target ~30 FPS
local last_draw     = -1

while running do
    -- 1. Engine tick (dispatch scheduled MIDI events).
    engine.tick()
    engine.process_input()

    -- 2. Arp output tick (generates arpeggiated notes per active track).
    if engine.playing then
        local cur = engine.cur_beat()
        for ti, track in ipairs(tracks) do
            if arp.states[ti] then
                local inst     = instruments_by_name[track.instrument_name]
                local port_rec = inst and engine.output_ports[inst.id]
                if port_rec then arp.tick(ti, cur, port_rec) end
            end
        end
    end

    -- 3. Sync UI state from engine.
    ui.playing      = engine.playing
    ui.recording    = engine.recording
    ui.bpm          = engine.bpm
    ui.studio_name  = current_studio and current_studio.name or "all"

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
            if ui.cursor.track == 0 then launch_scene() else trigger_clip() end

        elseif key == 'up'    then ui.move_cursor(0, -1)
        elseif key == 'down'  then ui.move_cursor(0,  1)
        elseif key == 'left'  then ui.move_cursor(-1, 0)
        elseif key == 'right' then ui.move_cursor( 1, 0)

        elseif key == 'r' or key == 'R' then toggle_record()
        elseif key == 'e' or key == 'E' then edit_clip_piano_roll()
        elseif key == 'c' or key == 'C' then edit_clip_settings()
        elseif key == 'n' or key == 'N' then new_clip()
        elseif key == 'd' or key == 'D' then delete_clip()
        elseif key == 'l' or key == 'L' then launch_scene()
        elseif key == '['               then insert_scene_row()
        elseif key == ']'               then delete_scene_row()
        elseif key == 'i' or key == 'I' then instruments_page()
        elseif key == 'w' or key == 'W' then studios_page()
        elseif key == 't' or key == 'T' then edit_track()
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

    -- 5. Redraw at ~30 FPS.
    local now = os.monotime()
    if (now - last_draw) >= DRAW_INTERVAL then
        ui.draw(engine)
        last_draw = now
    end
end

-- ── Cleanup ───────────────────────────────────────────────────────────────────

-- Disable all arps (sends note-off for any active arp note).
for ti, track in ipairs(tracks) do
    if arp.states[ti] then
        local inst = instruments_by_name[track.instrument_name]
        arp.disable(ti, inst and engine.output_ports[inst.id])
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

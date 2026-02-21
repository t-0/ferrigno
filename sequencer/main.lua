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
    local default_out = (dests[1] and dests[1].name) or ''
    local srcs        = midi.sources()      or {}
    local default_in  = (srcs[1]  and srcs[1].name)  or ''

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

-- Open output port and send any 'connect' sysex dumps.
-- Program change is NOT sent here — only when explicitly edited via I.
local function open_and_configure_output(inst)
    local ok, err = engine.open_output(inst)
    if not ok then return false, err end
    for _, dump in ipairs(sysex_dumps[inst.id] or {}) do
        engine.send_sysex(inst.id, dump.data)
    end
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
    local default_out = (dests[1] and dests[1].name) or ''
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

    -- Format a float for display: trim trailing zeros after decimal point.
    local function fmt_f(v)
        if v == nil then return '' end
        local s = string.format("%.3f", v)
        return s:gsub("%.?0+$", "")
    end

    -- Field definitions.
    -- type "text": free string
    -- type "float": real number, min/max, nullable (blank → nil or default)
    -- type "int":   integer, min/max, nullable
    -- type "toggle": cycles through opts with ←→ (stored as index)
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
        is_looping and 2 or 1,                -- index into loop_opts
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
                -- Append hint in dim after the value.
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
        local key = tui.read_key(5000)
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

    -- Apply values back to clip.
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

    -- Extract name strings (midi.destinations/sources return {name=..., index=...} tables).
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

    -- Field definitions: text, number (with range + optional nil), picker (cycle with ←→).
    local fields = {
        { label="Name",        type="text"                                     },
        { label="MIDI Output", type="picker", opts=dest_opts                   },
        { label="MIDI Input",  type="picker", opts=src_opts                    },
        { label="Channel",     type="number", min=1,   max=16,  nullable=false },
        { label="Bank MSB",    type="number", min=0,   max=127, nullable=true  },
        { label="Bank LSB",    type="number", min=0,   max=127, nullable=true  },
        { label="Program",     type="number", min=0,   max=127, nullable=true  },
    }

    -- Working values: pickers store index into opts; text/number store string.
    local vals = {
        inst.name or '',
        opt_index(dest_opts, inst.midi_output or ''),
        opt_index(src_opts,  inst.midi_input  or ''),
        tostring(inst.midi_channel or 1),
        inst.bank_msb ~= nil and tostring(inst.bank_msb) or '',
        inst.bank_lsb ~= nil and tostring(inst.bank_lsb) or '',
        inst.program  ~= nil and tostring(inst.program)  or '',
    }

    local LABEL_W  = 14
    local VAL_COL  = LABEL_W + 5   -- column where value field starts
    local sel      = 1
    local editing  = false
    local edit_buf = ''
    local do_sysex = false

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

        -- Header
        tui.print_at(1, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, tui.BOLD)
        tui.print_at(1, 2, "Instrument Settings", tui.WHITE, tui.BLUE, tui.BOLD)

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
                pad("  ↑↓/Tab=navigate  ←→=cycle options  Enter=edit  S=save  X=sysex  ESC=cancel", w),
                tui.BRIGHT_BLACK, tui.BLACK, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = tui.read_key(5000)
        if not key then goto inst_again end

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
                sel = (sel % #fields) + 1  -- advance to next field on confirm
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
        ::inst_again::
    end

    -- Apply values back to inst.
    local function parse_opt_num(s, mn, mx)
        if s == '' then return nil end
        local n = tonumber(s)
        return n and math.max(mn, math.min(mx, math.floor(n))) or nil
    end

    if vals[1] ~= '' then inst.name = vals[1] end
    inst.midi_output  = (dest_opts[vals[2]] == "None") and '' or (dest_opts[vals[2]] or '')
    inst.midi_input   = (src_opts[vals[3]]  == "None") and '' or (src_opts[vals[3]]  or '')
    inst.midi_channel = math.max(1, math.min(16, tonumber(vals[4]) or 1))
    inst.bank_msb     = parse_opt_num(vals[5], 0, 127)
    inst.bank_lsb     = parse_opt_num(vals[6], 0, 127)
    inst.program      = parse_opt_num(vals[7], 0, 127)

    db_mod.upsert_instrument(db, inst, song.id)
    engine.output_ports[inst.id] = nil
    open_and_configure_output(inst)

    if do_sysex then
        edit_sysex(inst)
    end

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

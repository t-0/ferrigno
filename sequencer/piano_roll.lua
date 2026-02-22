-- sequencer/piano_roll.lua
-- Full-screen piano roll editor.
-- Entry point: piano_roll.open(clip, raw_events, drum_map, cc_names) → events, loop_len, is_looping | nil
--   drum_map  : {[note_number]=name} or nil
--   cc_names  : {[cc_number]=name}   or nil  (global CC name overrides)

local M = {}

M.on_idle = nil   -- set by main.lua; called each 10 ms tick when no key is available

-- ── Constants ─────────────────────────────────────────────────────────────────

local PIANO_W = 4                    -- chars for pitch label column
local NOTE_NAMES = {"C","C#","D","D#","E","F","F#","G","G#","A","A#","B"}
local BLACK_KEYS = {[1]=true,[3]=true,[6]=true,[8]=true,[10]=true}

local ZOOM_VALS = {1, 2, 4, 8, 16, 32}   -- chars per beat
local Q_VALS   = {4.0, 2.0, 1.0, 0.5, 0.25, 0.125, 0.0625}
local Q_NAMES  = {"1/1","1/2","1/4","1/8","1/16","1/32","1/64"}

-- ── Helpers ───────────────────────────────────────────────────────────────────

local function note_name(midi)
    local oct  = math.floor(midi / 12) - 1
    return string.format("%-2s%d", NOTE_NAMES[(midi % 12) + 1], oct)
end

local function is_black(midi)
    return BLACK_KEYS[midi % 12] == true
end

local function snap(v, q)
    return math.floor(v / q + 0.5) * q
end

local function pad(s, n)
    s = tostring(s or '')
    if #s >= n then return s:sub(1, n) end
    return s .. string.rep(' ', n - #s)
end

-- ── Event ↔ Note conversion ───────────────────────────────────────────────────

local function events_to_notes(evts)
    local notes, open = {}, {}
    for _, ev in ipairs(evts) do
        local t = ev.status & 0xF0
        local p = ev.data1 or 0
        if t == 0x90 and (ev.data2 or 0) > 0 then
            open[p] = { start = ev.beat_offset, vel = ev.data2 }
        elseif t == 0x80 or (t == 0x90 and (ev.data2 or 0) == 0) then
            if open[p] then
                notes[#notes+1] = {
                    pitch    = p,
                    start    = open[p].start,
                    duration = math.max(0.0625, ev.beat_offset - open[p].start),
                    velocity = open[p].vel,
                    sel      = false,
                }
                open[p] = nil
            end
        end
    end
    -- Close any still-open notes with a minimum duration.
    for p, o in pairs(open) do
        notes[#notes+1] = { pitch=p, start=o.start, duration=0.25, velocity=o.vel, sel=false }
    end
    table.sort(notes, function(a,b)
        if a.start ~= b.start then return a.start < b.start end
        return a.pitch < b.pitch
    end)
    return notes
end

local function notes_to_events(notes)
    local evts = {}
    for _, n in ipairs(notes) do
        local d = math.max(0.0625, n.duration)
        evts[#evts+1] = { beat_offset=n.start,   status=0x90, data1=n.pitch, data2=n.velocity or 100 }
        evts[#evts+1] = { beat_offset=n.start+d, status=0x80, data1=n.pitch, data2=0 }
    end
    table.sort(evts, function(a, b)
        if a.beat_offset ~= b.beat_offset then return a.beat_offset < b.beat_offset end
        return (a.status & 0xF0) > (b.status & 0xF0)  -- note-off before note-on at same time
    end)
    return evts
end

-- ── Note lookup ───────────────────────────────────────────────────────────────

-- Find the first note that covers (pitch, beat). Returns index or nil.
local function note_at(notes, pitch, beat)
    for i, n in ipairs(notes) do
        if n.pitch == pitch
           and beat >= n.start
           and beat <  n.start + math.max(0.0001, n.duration) then
            return i, n
        end
    end
    return nil, nil
end

local function sel_count(notes)
    local c = 0
    for _, n in ipairs(notes) do if n.sel then c = c + 1 end end
    return c
end

-- ── Self-contained line input ─────────────────────────────────────────────────
-- (mirrors ui.read_line without requiring ui.lua)

local function read_line(prompt_row, label, default)
    local w = tui.size()
    local buf = default or ''
    local prompt = ' ' .. label

    local function redraw()
        tui.print_at(prompt_row, 1, pad(prompt, #prompt + 1), tui.BLACK, tui.CYAN, tui.BOLD)
        tui.print(tui.color(tui.WHITE, tui.BLACK, 0))
        tui.print(pad(buf, w - #prompt - 1))
        tui.print(tui.reset())
        tui.move(prompt_row, #prompt + 2 + #buf)
        tui.show_cursor()
    end

    redraw()
    while true do
        local key = tui.read_key(10)
        if not key then if M.on_idle then M.on_idle() end; goto again end
        if     key == 'enter' or key == 'return' then tui.hide_cursor(); return buf
        elseif key == 'esc'                   then tui.hide_cursor(); return nil
        elseif key == 'backspace' then if #buf > 0 then buf = buf:sub(1,-2) end
        elseif #key == 1          then buf = buf .. key
        end
        redraw()
        ::again::
    end
end

-- ── Renderer ─────────────────────────────────────────────────────────────────

-- Emit an ANSI color code only when the color changes (run-length encode).
local function make_emitter(parts)
    local last = nil
    return function(fg, bg, attrs)
        local k = fg * 10000 + bg * 100 + (attrs or 0)
        if k ~= last then
            parts[#parts+1] = tui.color(fg, bg, attrs or 0)
            last = k
        end
    end
end

local RST = nil  -- initialised lazily in open()

local function draw(st, w, h)
    local z    = ZOOM_VALS[st.zoom_idx]
    local Q    = Q_VALS[st.q_idx]
    local gw   = w - PIANO_W          -- grid width in chars
    local gh   = h - 4                -- grid height in pitch rows

    local function pitch_label(pitch)
        if st.drum_map then
            local name = st.drum_map[pitch]
            if name then
                if #name >= PIANO_W then return name:sub(1, PIANO_W) end
                return name .. string.rep(' ', PIANO_W - #name)
            end
            local s = tostring(pitch)
            return string.rep(' ', PIANO_W - #s) .. s
        end
        return note_name(pitch)
    end

    tui.hide_cursor()

    -- ── Row 1: header ─────────────────────────────────────────────────────────
    local sel = sel_count(st.notes)
    local roll_type = st.drum_map and "Drum Roll" or "Piano Roll"
    local hdr = string.format(
        " %s | %s | Loop:%.2f %s | Q:%s | Zoom:%d | Sel:%d", roll_type,
        pad(st.clip.name ~= '' and st.clip.name or "Clip", 10),
        st.loop_len,
        st.looping and "[LOOP]" or "[ ONE]",
        Q_NAMES[st.q_idx], z, sel)
    tui.print_at(1, 1, pad(hdr, w), tui.WHITE, tui.BLUE, tui.BOLD)

    -- ── Row 2: beat ruler ─────────────────────────────────────────────────────
    local parts = {}
    local emit  = make_emitter(parts)

    emit(tui.BRIGHT_BLACK, tui.BLACK, 0)
    parts[#parts+1] = string.rep(' ', PIANO_W)

    -- Build ruler char-by-char into a flat table, tracking a "skip" counter
    -- for multi-char bar numbers (can't modify for-var in Lua).
    local ruler_chars = {}   -- [col+1] = char string
    local skip_until  = -1
    for col = 0, gw - 1 do
        if col <= skip_until then
            ruler_chars[col+1] = ""   -- already emitted as part of bar number
        else
            local beat      = st.view_left + col / z
            local at_cursor = (math.abs(beat - snap(st.cursor_beat, 1/z)) < 0.5/z)
            local at_loop   = (beat >= st.loop_len - 0.5/z and beat < st.loop_len + 0.5/z)
            if at_loop then
                ruler_chars[col+1] = "\1║"         -- \1 = marker for bold-white
            elseif at_cursor then
                ruler_chars[col+1] = "\2▼"         -- \2 = cursor colour
            elseif math.abs(beat % 1.0) < 0.5/z then
                local s = tostring(math.floor(beat) + 1)
                if col + #s <= gw then
                    ruler_chars[col+1] = "\3" .. s   -- \3 = bar-number colour
                    skip_until = col + #s - 1
                else
                    ruler_chars[col+1] = "\3|"
                end
            elseif math.abs(beat % 0.5) < 0.5/z then
                ruler_chars[col+1] = "\4·"
            else
                ruler_chars[col+1] = " "
            end
        end
    end
    -- Second pass: emit with colour transitions.
    for _, rc in ipairs(ruler_chars) do
        if rc == "" then
            -- skip (already part of a multi-char number)
        elseif rc:sub(1,1) == "\1" then
            emit(tui.BRIGHT_WHITE, tui.BLACK, tui.BOLD); parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\2" then
            emit(tui.BLACK, tui.CYAN, tui.BOLD);         parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\3" then
            emit(tui.BRIGHT_WHITE, tui.BLACK, 0);        parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\4" then
            emit(tui.BRIGHT_BLACK, tui.BLACK, 0);        parts[#parts+1] = rc:sub(2)
        else
            emit(tui.BRIGHT_BLACK, tui.BLACK, 0);        parts[#parts+1] = rc
        end
    end
    parts[#parts+1] = RST
    tui.move(2, 1); tui.print(table.concat(parts))

    -- ── Rows 3…3+gh-1: pitch grid ─────────────────────────────────────────────
    for row = 0, gh - 1 do
        local pitch = st.view_top - row
        if pitch < 0 or pitch > 127 then
            tui.print_at(3 + row, 1, string.rep(' ', w), tui.BLACK, tui.BLACK, 0)
        else
            local black  = is_black(pitch)
            local is_c   = (pitch % 12 == 0)
            local at_cur_pitch = (pitch == st.cursor_pitch)

            parts = {}
            emit  = make_emitter(parts)

            -- Pitch label
            if at_cur_pitch then
                emit(tui.BLACK, tui.CYAN, tui.BOLD)
            elseif black then
                emit(tui.BRIGHT_WHITE, tui.BLACK, tui.BOLD)
            else
                emit(tui.BLACK, is_c and tui.BRIGHT_WHITE or tui.WHITE, 0)
            end
            parts[#parts+1] = pitch_label(pitch)

            -- Grid columns
            for col = 0, gw - 1 do
                local beat = st.view_left + col / z
                local at_cursor = at_cur_pitch and (math.abs(beat - snap(st.cursor_beat, 1/z)) < 0.5/z)
                local at_loop   = (beat >= st.loop_len - 0.5/z and beat < st.loop_len + 0.5/z)
                local on_beat   = math.abs(beat % 1.0) < 0.5/z

                local ni, n = note_at(st.notes, pitch, beat)
                local ch, fg, bg, attrs = " ", tui.BRIGHT_BLACK, tui.BLACK, 0

                if at_loop then
                    ch = "│"; fg = tui.BRIGHT_WHITE; bg = tui.BLACK; attrs = tui.BOLD
                elseif ni then
                    -- Detect note head vs. body: first char of the note
                    local is_head = (beat < n.start + 1/z)
                    ch = is_head and "▐" or "█"
                    if at_cursor then
                        fg, bg, attrs = tui.BLACK, tui.CYAN, tui.BOLD
                    elseif n.sel then
                        fg, bg, attrs = tui.BLACK, tui.YELLOW, tui.BOLD
                    else
                        fg, bg, attrs = tui.BLACK, tui.GREEN, 0
                    end
                elseif at_cursor then
                    ch = "│"; fg = tui.CYAN; bg = tui.BLACK; attrs = tui.BOLD
                elseif black then
                    ch = " "; fg = tui.BRIGHT_BLACK; bg = tui.BRIGHT_BLACK
                else
                    ch = on_beat and "·" or " "
                    fg = tui.BRIGHT_BLACK; bg = tui.BLACK
                end
                emit(fg, bg, attrs)
                parts[#parts+1] = ch
            end

            parts[#parts+1] = RST
            tui.move(3 + row, 1)
            tui.print(table.concat(parts))
        end
    end

    -- ── Footer rows ───────────────────────────────────────────────────────────
    tui.print_at(h - 1, 1, pad(st.status, w), tui.BRIGHT_WHITE, tui.BLACK, 0)
    tui.print_at(h, 1,
        pad(" ←→↑↓=navigate  Shift+←→↑↓=sel-range  SPC=sel  A=sel-all" ..
            "  +/-=transpose  ,/.=length  z/x=position  ENTER=add/del  DEL=delete" ..
            "  []={Q}  Q=quant  R=rev  I=inv  C/V/^X=copy/paste/cut  L=loop  O=loop-tog  S=save  ESC=cancel",
            w),
        tui.BRIGHT_BLACK, tui.BLACK, 0)
end

-- ── Operations ────────────────────────────────────────────────────────────────

local function sort_notes(notes)
    table.sort(notes, function(a, b)
        if a.start ~= b.start then return a.start < b.start end
        return a.pitch < b.pitch
    end)
end

local function op_add_or_delete(st)
    local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
    local ni   = note_at(st.notes, st.cursor_pitch, beat)
    if ni then
        table.remove(st.notes, ni)
        st.status = "Note deleted"
    else
        st.notes[#st.notes+1] = {
            pitch    = st.cursor_pitch,
            start    = beat,
            duration = st.note_dur,
            velocity = 100,
            sel      = false,
        }
        sort_notes(st.notes)
        st.status = string.format("Note added: %s @ beat %.3f",
            note_name(st.cursor_pitch), beat)
    end
    st.dirty = true
end

local function op_delete_sel_or_cursor(st)
    local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
    local ni   = note_at(st.notes, st.cursor_pitch, beat)
    if ni then
        table.remove(st.notes, ni); st.dirty = true
        st.status = "Note deleted"
        return
    end
    local new, cnt = {}, 0
    for _, n in ipairs(st.notes) do
        if n.sel then cnt = cnt + 1 else new[#new+1] = n end
    end
    if cnt > 0 then
        st.notes = new; st.dirty = true
        st.status = "Deleted " .. cnt .. " notes"
    end
end

local function op_select_toggle(st)
    local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
    local _, n = note_at(st.notes, st.cursor_pitch, beat)
    if n then
        n.sel = not n.sel
        st.status = n.sel and "Note selected" or "Note deselected"
    end
end

local function op_select_all(st)
    local any = sel_count(st.notes) < #st.notes
    for _, n in ipairs(st.notes) do n.sel = any end
    st.status = any and "All selected" or "All deselected"
end

local function op_quantize(st)
    local Q   = Q_VALS[st.q_idx]
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            n.start    = snap(n.start,    Q)
            n.duration = math.max(Q, snap(n.duration, Q))
            cnt = cnt + 1
        end
    end
    sort_notes(st.notes); st.dirty = true
    st.status = string.format("Quantized %d notes to %s", cnt, Q_NAMES[st.q_idx])
end

local function op_reverse(st)
    local L   = st.loop_len
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            n.start = math.max(0, L - n.start - n.duration)
            cnt = cnt + 1
        end
    end
    sort_notes(st.notes); st.dirty = true
    st.status = "Reversed " .. cnt .. " notes"
end

local function op_invert(st)
    local pitches = {}
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            pitches[#pitches+1] = n.pitch
        end
    end
    if #pitches == 0 then return end
    local lo = math.min(table.unpack(pitches))
    local hi = math.max(table.unpack(pitches))
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            n.pitch = lo + hi - n.pitch; cnt = cnt + 1
        end
    end
    st.dirty = true
    st.status = "Inverted " .. cnt .. " notes"
end

local function op_transpose(st, semitones)
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            n.pitch = math.max(0, math.min(127, n.pitch + semitones))
            cnt = cnt + 1
        end
    end
    st.dirty = true
    st.status = string.format("Transposed %d notes by %+d", cnt, semitones)
end

local function op_copy(st)
    local min_start = math.huge
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            if n.start < min_start then min_start = n.start end
            cnt = cnt + 1
        end
    end
    st.clipboard = {}
    for _, n in ipairs(st.notes) do
        if sel_count(st.notes) == 0 or n.sel then
            st.clipboard[#st.clipboard+1] = {
                pitch=n.pitch, start=n.start - min_start,
                duration=n.duration, velocity=n.velocity, sel=false,
            }
        end
    end
    st.status = "Copied " .. cnt .. " notes"
end

local function op_paste(st)
    if #st.clipboard == 0 then st.status = "Clipboard empty"; return end
    for _, n in ipairs(st.notes) do n.sel = false end
    local base = snap(st.cursor_beat, Q_VALS[st.q_idx])
    for _, n in ipairs(st.clipboard) do
        st.notes[#st.notes+1] = {
            pitch=n.pitch, start=base+n.start,
            duration=n.duration, velocity=n.velocity, sel=true,
        }
    end
    sort_notes(st.notes); st.dirty = true
    st.status = "Pasted " .. #st.clipboard .. " notes"
end

local function op_move_sel(st, dt, dp)
    local Q  = Q_VALS[st.q_idx]
    local s  = sel_count(st.notes)
    local cnt = 0
    for _, n in ipairs(st.notes) do
        if s == 0 or n.sel then
            n.start = math.max(0, n.start + dt * Q)
            n.pitch = math.max(0, math.min(127, n.pitch + dp))
            cnt = cnt + 1
        end
    end
    if cnt > 0 then sort_notes(st.notes); st.dirty = true end
end

local function op_resize_sel(st, dir)
    local Q = Q_VALS[st.q_idx]
    local s = sel_count(st.notes)
    for _, n in ipairs(st.notes) do
        if s == 0 or n.sel then
            n.duration = math.max(Q, n.duration + dir * Q)
        end
    end
    st.dirty = true
end

local function scroll_to_cursor(st, w, h)
    local z  = ZOOM_VALS[st.zoom_idx]
    local gw = w - PIANO_W
    local gh = h - 4
    local vb = gw / z  -- visible beats

    if st.cursor_beat < st.view_left then
        st.view_left = snap(st.cursor_beat, Q_VALS[st.q_idx])
    elseif st.cursor_beat >= st.view_left + vb then
        st.view_left = snap(st.cursor_beat - vb + Q_VALS[st.q_idx], Q_VALS[st.q_idx])
    end
    if st.cursor_pitch > st.view_top then
        st.view_top = st.cursor_pitch
    elseif st.cursor_pitch < st.view_top - gh + 1 then
        st.view_top = st.cursor_pitch + gh - 1
    end
end

-- ── Entry point ───────────────────────────────────────────────────────────────

function M.open(clip, raw_events, drum_map, cc_names)
    RST = tui.reset()

    local w, h = tui.size()
    local st = {
        clip       = clip,
        notes      = events_to_notes(raw_events or {}),
        loop_len   = clip.length_beats or 4.0,
        looping    = (clip.is_looping == 1 or clip.is_looping == true),
        cursor_beat  = 0.0,
        cursor_pitch = 60,
        view_left    = 0.0,
        view_top     = 72,   -- C5 at top
        zoom_idx     = 3,    -- zoom = 4 chars/beat
        q_idx        = 5,    -- 1/16th note
        note_dur     = 0.25, -- default added-note duration (tracks Q)
        clipboard    = {},
        status       = "Ready. ENTER=add/del  S=save  ESC=cancel",
        dirty        = false,
        drum_map     = drum_map,  -- {[note]=name} or nil
        cc_names     = cc_names,  -- {[cc_number]=name} or nil
    }

    tui.clear()
    draw(st, w, h)

    local result = nil

    while true do
        local key = tui.read_key(10)
        if not key then
            if M.on_idle then M.on_idle() end
            w, h = tui.size()
            draw(st, w, h)
            goto next
        end
        w, h = tui.size()

            -- Navigation: plain arrows move cursor; shift-arrows extend selection.
        if key == 'right' or key == 'shift-right' then
            st.cursor_beat = math.max(0, st.cursor_beat + Q_VALS[st.q_idx])
            if key == 'shift-right' then
                -- Select every note under/at the new cursor beat on this pitch row.
                local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
                local _, n = note_at(st.notes, st.cursor_pitch, beat)
                if n then n.sel = true end
            end
        elseif key == 'left' or key == 'shift-left' then
            st.cursor_beat = math.max(0, st.cursor_beat - Q_VALS[st.q_idx])
            if key == 'shift-left' then
                local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
                local _, n = note_at(st.notes, st.cursor_pitch, beat)
                if n then n.sel = true end
            end
        elseif key == 'up' or key == 'shift-up' then
            st.cursor_pitch = math.min(127, st.cursor_pitch + 1)
            if key == 'shift-up' then
                -- Select all notes at the new pitch row that overlap the cursor beat.
                local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
                local _, n = note_at(st.notes, st.cursor_pitch, beat)
                if n then n.sel = true end
            end
        elseif key == 'down' or key == 'shift-down' then
            st.cursor_pitch = math.max(0, st.cursor_pitch - 1)
            if key == 'shift-down' then
                local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
                local _, n = note_at(st.notes, st.cursor_pitch, beat)
                if n then n.sel = true end
            end
        elseif key == 'pgup'  then st.view_top = math.min(127, st.view_top + 12)
        elseif key == 'pgdn'  then st.view_top = math.max(12,  st.view_top - 12)
        elseif key == 'home'  then st.cursor_beat = 0; st.view_left = 0

        -- Edit
        elseif key == 'enter' or key == 'return' then op_add_or_delete(st)
        elseif key == 'backspace' or key == 'del' then op_delete_sel_or_cursor(st)
        elseif key == ' '         then op_select_toggle(st)
        elseif key == 'a' or key == 'A' then op_select_all(st)

        -- Transpose: = / - → ±1 semitone;  + (shift+=) / _ (shift+-) → ±1 octave
        elseif key == '='  then op_transpose(st,   1)
        elseif key == '+'  then op_transpose(st,  12)
        elseif key == '-'  then op_transpose(st,  -1)
        elseif key == '_'  then op_transpose(st, -12)

        -- Resize selected notes with , / .
        elseif key == '.' or key == '>' then op_resize_sel(st,  1)
        elseif key == ',' or key == '<' then op_resize_sel(st, -1)

        -- Move selected notes with z / x
        elseif key == 'z' or key == 'Z' then op_move_sel(st, -1,  0)
        elseif key == 'x' or key == 'X' then op_move_sel(st,  1,  0)

        -- Zoom with ] / [
        elseif key == ']' then
            st.zoom_idx = math.min(#ZOOM_VALS, st.zoom_idx + 1)
            st.status = "Zoom: " .. ZOOM_VALS[st.zoom_idx]
        elseif key == '[' then
            st.zoom_idx = math.max(1, st.zoom_idx - 1)
            st.status = "Zoom: " .. ZOOM_VALS[st.zoom_idx]
        elseif key == '{' then
            st.q_idx = math.max(1, st.q_idx - 1)
            st.note_dur = Q_VALS[st.q_idx]
            st.status = "Quantize: " .. Q_NAMES[st.q_idx]
        elseif key == '}' then
            st.q_idx = math.min(#Q_VALS, st.q_idx + 1)
            st.note_dur = Q_VALS[st.q_idx]
            st.status = "Quantize: " .. Q_NAMES[st.q_idx]

        -- Operations
        elseif key == 'q' or key == 'Q' then op_quantize(st)
        elseif key == 'r' or key == 'R' then op_reverse(st)
        elseif key == 'i' or key == 'I' then op_invert(st)
        elseif key == 'c' or key == 'C' then op_copy(st)
        elseif key == 'v' or key == 'V' then op_paste(st)
        elseif key == 'ctrl-x' then
            op_copy(st)
            local cnt = sel_count(st.notes)
            if cnt > 0 then
                local new = {}
                for _, n in ipairs(st.notes) do
                    if not n.sel then new[#new+1] = n end
                end
                st.notes = new; st.dirty = true
                st.status = "Cut " .. cnt .. " notes"
            end

        elseif key == 'h' or key == 'H' then
            -- HJKL still available for moving selection (alternative to z/x/+/-)
            op_move_sel(st, -1,  0)
        elseif key == 'l' or key == 'L' then
            local val = read_line(h - 1, "Loop length (beats): ",
                                  string.format("%.2f", st.loop_len))
            tui.clear()
            if val then
                local n = tonumber(val)
                if n and n > 0 then
                    st.loop_len = n; st.dirty = true
                    st.status = string.format("Loop length: %.2f beats", n)
                end
            end
        elseif key == 'k' or key == 'K' then op_move_sel(st,  0,  1)
        elseif key == 'j' or key == 'J' then op_move_sel(st,  0, -1)

        elseif key == 't' or key == 'T' then
            local val = read_line(h - 1, "Transpose semitones: ", "0")
            tui.clear()
            if val then
                local n = tonumber(val)
                if n then op_transpose(st, math.floor(n)) end
            end

        elseif key == 'o' or key == 'O' then
            st.looping = not st.looping; st.dirty = true
            st.status = "Looping: " .. (st.looping and "ON" or "OFF")

        -- Save / cancel
        elseif key == 's' or key == 'S' then
            result = notes_to_events(st.notes)
            break
        elseif key == 'esc' then
            if st.dirty then
                st.status = "Unsaved changes! Press S to save or ESC again to discard."
                draw(st, w, h)
                local k2
                repeat
                    k2 = tui.read_key(10)
                    if not k2 and M.on_idle then M.on_idle() end
                until k2
                if k2 == 'esc' then break end
            else
                break
            end
        end

        scroll_to_cursor(st, w, h)
        draw(st, w, h)
        ::next::
    end

    tui.clear()

    if result then
        return result, st.loop_len, st.looping
    end
    return nil
end

return M

-- sequencer/piano_roll.lua
-- Full-screen piano roll editor.
-- Entry point: piano_roll.open(clip, raw_events, drum_map, cc_names, nrpn_names) → events, loop_len, is_looping | nil
--   drum_map    : {[note_number]=name}   or nil
--   cc_names    : {[cc_number]=name}     or nil  (global CC name overrides)
--   nrpn_names  : {[nrpn_number]=name}   or nil  (global NRPN name overrides)

local M = {}

M.on_idle = nil   -- set by main.lua; called each 10 ms tick when no key is available

-- ── Constants ─────────────────────────────────────────────────────────────────

local PIANO_W  = 4                    -- chars for pitch label column
local NOTE_NAMES = {"C","C#","D","D#","E","F","F#","G","G#","A","A#","B"}
local BLACK_KEYS = {[1]=true,[3]=true,[6]=true,[8]=true,[10]=true}

local ZOOM_VALS = {1, 2, 4, 8, 16, 32}   -- chars per beat
local Q_VALS   = {4.0, 2.0, 1.0, 0.5, 0.25, 0.125, 0.0625}
local Q_NAMES  = {"1/1","1/2","1/4","1/8","1/16","1/32","1/64"}

local CC_PB     = 128   -- sentinel: pitch-bend lane
local NRPN_BASE = 256   -- sentinel offset: NRPN N is lane 256 + N
local CC_LANE_H = 4     -- bar-chart rows per lane (+ 1 header row = 5 rows total per lane)

local SYSEX_BASE = 32768  -- sentinel offset: SysEx param N is lane 32768 + N

local function is_sysex_lane(num) return num >= SYSEX_BASE end
local function sysex_param_index(lane) return lane - SYSEX_BASE end
local function is_nrpn_lane(num)  return num >= NRPN_BASE and num < SYSEX_BASE end
local function nrpn_number(lane)  return lane - NRPN_BASE end

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

local function events_to_notes_and_cc(evts)
    local notes, open = {}, {}
    local cc_extras = {}
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
        else
            cc_extras[#cc_extras+1] = ev
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
    table.sort(cc_extras, function(a, b) return a.beat_offset < b.beat_offset end)
    return notes, cc_extras
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

local function notes_and_cc_to_events(notes, cc_extras)
    local evts = notes_to_events(notes)
    for _, ev in ipairs(cc_extras) do
        evts[#evts+1] = ev
    end
    table.sort(evts, function(a, b)
        if a.beat_offset ~= b.beat_offset then return a.beat_offset < b.beat_offset end
        return (a.status & 0xF0) > (b.status & 0xF0)
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

-- ── CC Helpers ────────────────────────────────────────────────────────────────

local function lane_label(cc_num, cc_names, nrpn_names, sysex_names)
    if cc_num == CC_PB then
        return "Pitch Bend"
    end
    if is_sysex_lane(cc_num) then
        local n = sysex_param_index(cc_num)
        local name = sysex_names and sysex_names[n]
        if name then
            return string.format("SX %d %s", n, name)
        end
        return string.format("SX %d", n)
    end
    if is_nrpn_lane(cc_num) then
        local n = nrpn_number(cc_num)
        local name = nrpn_names and nrpn_names[n]
        if name then
            return string.format("NRPN %d %s", n, name)
        end
        return string.format("NRPN %d", n)
    end
    local name = cc_names and cc_names[cc_num]
    if name then
        return string.format("CC%3d %s", cc_num, name)
    end
    return string.format("CC%3d", cc_num)
end

-- Returns the last held value at or before `beat`, or nil if none.
local function cc_value_before_beat(cc_extras, cc_num, beat)
    local val = nil
    for _, ev in ipairs(cc_extras) do  -- sorted by beat_offset
        if ev.beat_offset > beat then break end
        local is_match
        if cc_num == CC_PB then
            is_match = (ev.status & 0xF0) == 0xE0
        elseif is_sysex_lane(cc_num) then
            is_match = ev.status == 0xF2 and ev.data1 == sysex_param_index(cc_num)
        elseif is_nrpn_lane(cc_num) then
            is_match = ev.status == 0xF1 and ev.data1 == nrpn_number(cc_num)
        else
            is_match = (ev.status & 0xF0) == 0xB0 and ev.data1 == cc_num
        end
        if is_match then
            if cc_num == CC_PB then
                val = (ev.data1 or 0) + ((ev.data2 or 0) * 128) - 8192
            elseif is_sysex_lane(cc_num) then
                val = ev.data2 or 0
            elseif is_nrpn_lane(cc_num) then
                val = ev.data2 or 0
            else
                val = ev.data2 or 0
            end
        end
    end
    return val
end

-- Returns the value of a CC event at snap(beat, q) ±q/2, or nil.
local function cc_event_at_beat(cc_extras, cc_num, beat, q)
    local sb = snap(beat, q)
    for _, ev in ipairs(cc_extras) do
        if math.abs(ev.beat_offset - sb) <= q / 2 then
            local is_match
            if cc_num == CC_PB then
                is_match = (ev.status & 0xF0) == 0xE0
            elseif is_sysex_lane(cc_num) then
                is_match = ev.status == 0xF2 and ev.data1 == sysex_param_index(cc_num)
            elseif is_nrpn_lane(cc_num) then
                is_match = ev.status == 0xF1 and ev.data1 == nrpn_number(cc_num)
            else
                is_match = (ev.status & 0xF0) == 0xB0 and ev.data1 == cc_num
            end
            if is_match then
                if cc_num == CC_PB then
                    return (ev.data1 or 0) + ((ev.data2 or 0) * 128) - 8192
                elseif is_sysex_lane(cc_num) then
                    return ev.data2 or 0
                elseif is_nrpn_lane(cc_num) then
                    return ev.data2 or 0
                else
                    return ev.data2 or 0
                end
            end
        end
    end
    return nil
end

-- Returns true if there's a CC event near `beat` (within hw half-window).
local function cc_has_event_near(cc_extras, cc_num, beat, hw)
    for _, ev in ipairs(cc_extras) do
        if math.abs(ev.beat_offset - beat) <= hw then
            local is_match
            if cc_num == CC_PB then
                is_match = (ev.status & 0xF0) == 0xE0
            elseif is_sysex_lane(cc_num) then
                is_match = ev.status == 0xF2 and ev.data1 == sysex_param_index(cc_num)
            elseif is_nrpn_lane(cc_num) then
                is_match = ev.status == 0xF1 and ev.data1 == nrpn_number(cc_num)
            else
                is_match = (ev.status & 0xF0) == 0xB0 and ev.data1 == cc_num
            end
            if is_match then return true end
        end
    end
    return false
end

-- Place a CC event (remove exact-beat duplicate first, insert new, re-sort).
local function cc_place(cc_extras, cc_num, beat, value)
    local new = {}
    for _, ev in ipairs(cc_extras) do
        local is_match
        if cc_num == CC_PB then
            is_match = (ev.status & 0xF0) == 0xE0 and ev.beat_offset == beat
        elseif is_sysex_lane(cc_num) then
            is_match = ev.status == 0xF2 and ev.data1 == sysex_param_index(cc_num) and ev.beat_offset == beat
        elseif is_nrpn_lane(cc_num) then
            is_match = ev.status == 0xF1 and ev.data1 == nrpn_number(cc_num) and ev.beat_offset == beat
        else
            is_match = (ev.status & 0xF0) == 0xB0 and ev.data1 == cc_num and ev.beat_offset == beat
        end
        if not is_match then new[#new+1] = ev end
    end
    local ev
    if cc_num == CC_PB then
        local pb = math.max(0, math.min(16383, value + 8192))
        ev = { beat_offset=beat, status=0xE0, data1=pb & 0x7F, data2=(pb >> 7) & 0x7F }
    elseif is_sysex_lane(cc_num) then
        local v = math.max(0, math.min(127, value))
        ev = { beat_offset=beat, status=0xF2, data1=sysex_param_index(cc_num), data2=v }
    elseif is_nrpn_lane(cc_num) then
        local v = math.max(0, math.min(16383, value))
        ev = { beat_offset=beat, status=0xF1, data1=nrpn_number(cc_num), data2=v }
    else
        local v = math.max(0, math.min(127, value))
        ev = { beat_offset=beat, status=0xB0, data1=cc_num, data2=v }
    end
    new[#new+1] = ev
    table.sort(new, function(a, b) return a.beat_offset < b.beat_offset end)
    return new
end

-- Erase CC events within q/2 of snap(beat, q). Returns new list and count erased.
local function cc_erase(cc_extras, cc_num, beat, q)
    local sb = snap(beat, q)
    local new, cnt = {}, 0
    for _, ev in ipairs(cc_extras) do
        local is_match
        if cc_num == CC_PB then
            is_match = (ev.status & 0xF0) == 0xE0 and math.abs(ev.beat_offset - sb) <= q / 2
        elseif is_sysex_lane(cc_num) then
            is_match = ev.status == 0xF2 and ev.data1 == sysex_param_index(cc_num)
                       and math.abs(ev.beat_offset - sb) <= q / 2
        elseif is_nrpn_lane(cc_num) then
            is_match = ev.status == 0xF1 and ev.data1 == nrpn_number(cc_num)
                       and math.abs(ev.beat_offset - sb) <= q / 2
        else
            is_match = (ev.status & 0xF0) == 0xB0 and ev.data1 == cc_num
                       and math.abs(ev.beat_offset - sb) <= q / 2
        end
        if is_match then
            cnt = cnt + 1
        else
            new[#new+1] = ev
        end
    end
    return new, cnt
end

-- ── Self-contained line input ─────────────────────────────────────────────────
-- (mirrors ui.read_line without requiring ui.lua)

local function read_line(prompt_row, label, default)
    local w = tui.size()
    local buf = default or ''
    local prompt = ' ' .. label

    local function redraw()
        tui.print_at(prompt_row, 1, pad(prompt, #prompt + 1), tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
        tui.print(tui.color(tui.WHITE, tui.BLUE, 0))
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
        elseif key == 'esc'                      then tui.hide_cursor(); return nil
        elseif key == 'backspace' then if #buf > 0 then buf = buf:sub(1,-2) end
        elseif #key == 1          then buf = buf .. key
        end
        redraw()
        ::again::
    end
end

-- ── Help overlay ─────────────────────────────────────────────────────────────

local HELP_NOTE = {
    { "title",   "Piano Roll — Note Mode" },
    { "section", "Navigation" },
    { "entry",   "←  /  →",              "Move cursor by Q step" },
    { "entry",   "↑  /  ↓",              "Move cursor pitch up / down" },
    { "entry",   "Shift ← → ↑ ↓",        "Extend selection range" },
    { "entry",   "PgUp  /  PgDn",         "Scroll view ± 1 octave" },
    { "entry",   "Home",                  "Jump to beat 0" },
    { "blank" },
    { "section", "Editing" },
    { "entry",   "Enter",                 "Add note / delete note at cursor" },
    { "entry",   "Del  /  Backspace",     "Delete note or selected notes" },
    { "entry",   "Space",                 "Toggle note selection" },
    { "entry",   "A",                     "Select all / deselect all" },
    { "blank" },
    { "section", "Transpose & Length" },
    { "entry",   "=  /  -",              "±1 semitone" },
    { "entry",   "+  /  _",              "±1 octave" },
    { "entry",   "T",                     "Prompt exact transpose amount" },
    { "entry",   ".  /  >",              "Lengthen note(s) by Q" },
    { "entry",   ",  /  <",              "Shorten note(s) by Q" },
    { "blank" },
    { "section", "Position (selection)" },
    { "entry",   "Z  /  X",              "Move selection left / right by Q" },
    { "entry",   "H  /  L",              "Move selection left / right (HJKL)" },
    { "entry",   "J  /  K",              "Move selection down / up" },
    { "blank" },
    { "section", "Operations" },
    { "entry",   "Q",                     "Quantize to grid" },
    { "entry",   "R",                     "Reverse notes in loop" },
    { "entry",   "I",                     "Invert pitches" },
    { "entry",   "C  /  Ctrl-V  /  Ctrl-X", "Copy / Paste / Cut" },
    { "entry",   "V",                     "Set velocity for note at cursor / selection" },
    { "blank" },
    { "section", "View" },
    { "entry",   "[  /  ]",              "Zoom out / in" },
    { "entry",   "{  /  }",              "Coarser / finer Q grid" },
    { "blank" },
    { "section", "Loop & Clip" },
    { "entry",   "L",                     "Set loop length (prompt)" },
    { "entry",   "O",                     "Toggle loop on / off" },
    { "blank" },
    { "section", "CC Lanes" },
    { "entry",   "F",                     "Add CC or Pitch Bend lane (0–128)" },
    { "entry",   "Tab",                   "Enter CC lane (if any)" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                     "Save and exit" },
    { "entry",   "ESC",                   "Cancel (confirm if unsaved)" },
    { "entry",   "?",                     "Show this help" },
}

local HELP_CC = {
    { "title",   "Piano Roll — CC Lane Mode" },
    { "section", "Navigation" },
    { "entry",   "←  /  →",              "Move cursor by Q step" },
    { "entry",   "Home",                  "Jump to beat 0" },
    { "entry",   "Tab",                   "Cycle to next lane / back to note mode" },
    { "blank" },
    { "section", "Value" },
    { "entry",   "↑  /  ↓",              "+1 / −1  (CC);  +64 / −64  (Pitch Bend)" },
    { "entry",   "Shift ↑  /  ↓",        "+10 / −10  (CC);  +512 / −512  (PB)" },
    { "entry",   "V",                     "Prompt exact value" },
    { "blank" },
    { "section", "Editing" },
    { "entry",   "Enter",                 "Place event at cursor with staged value" },
    { "entry",   "Del  /  Backspace",     "Erase event(s) at cursor" },
    { "blank" },
    { "section", "Lane" },
    { "entry",   "-",                     "Remove this CC lane" },
    { "entry",   "F  (in note mode)",     "Add a new CC or Pitch Bend lane" },
    { "blank" },
    { "section", "View" },
    { "entry",   "[  /  ]",              "Zoom out / in" },
    { "entry",   "{  /  }",              "Coarser / finer Q grid" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                     "Save and exit" },
    { "entry",   "ESC",                   "Cancel (confirm if unsaved)" },
    { "entry",   "?",                     "Show this help" },
}

local HELP_KEY_W = 24   -- chars reserved for the key column

local function show_help(mode, w, h)
    local items = (mode == "cc") and HELP_CC or HELP_NOTE
    tui.hide_cursor()
    tui.clear()

    -- Header
    tui.print_at(1, 1, pad(" Piano Roll Help", w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
    -- Footer
    tui.print_at(h, 1, pad(" ESC or ? = close help", w), tui.MAGENTA, tui.BLUE, 0)

    local row = 3
    for _, item in ipairs(items) do
        if row > h - 1 then break end
        local kind = item[1]
        if kind == "blank" then
            row = row + 1
        elseif kind == "title" then
            -- already shown in header; skip
        elseif kind == "section" then
            tui.print_at(row, 1,
                pad("  " .. item[2], w),
                tui.CYAN, tui.BLUE, tui.BOLD)
            row = row + 1
        elseif kind == "entry" then
            local key_str  = "    " .. item[2]
            local desc_str = item[3]
            -- Key column: bright white
            tui.print_at(row, 1,
                pad(key_str, HELP_KEY_W),
                tui.BRIGHT_WHITE, tui.BLUE, 0)
            -- Desc column: normal white
            tui.move(row, HELP_KEY_W + 1)
            tui.print(tui.color(tui.WHITE, tui.BLUE, 0))
            tui.print(pad(desc_str, w - HELP_KEY_W))
            tui.print(tui.reset())
            row = row + 1
        end
    end

    -- Block until ESC or ?
    while true do
        local key = tui.read_key(10)
        if not key then
            if M.on_idle then M.on_idle() end
        elseif key == 'esc' or key == '?' then
            break
        end
    end
    tui.clear()
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
    local total_lane_rows = #st.cc_lanes * (CC_LANE_H + 1)
    local gh = math.max(4, h - 4 - total_lane_rows)   -- grid height in pitch rows

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
    local hdr
    if st.active_lane > 0 then
        local cc_num = st.cc_lanes[st.active_lane]
        local lbl = lane_label(cc_num, st.cc_names, st.nrpn_names, st.sysex_names)
        hdr = string.format(
            " %s | %s | Loop:%.2f %s | CC: %s  val:%d",
            roll_type,
            pad(st.clip.name ~= '' and st.clip.name or "Clip", 10),
            st.loop_len,
            st.looping and "[LOOP]" or "[ ONE]",
            lbl, st.cc_cur_val)
    else
        hdr = string.format(
            " %s | %s | Loop:%.2f %s | Q:%s | Zoom:%d | Sel:%d", roll_type,
            pad(st.clip.name ~= '' and st.clip.name or "Clip", 10),
            st.loop_len,
            st.looping and "[LOOP]" or "[ ONE]",
            Q_NAMES[st.q_idx], z, sel)
    end
    tui.print_at(1, 1, pad(hdr, w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

    -- ── Row 2: beat ruler ─────────────────────────────────────────────────────
    local parts = {}
    local emit  = make_emitter(parts)

    emit(tui.MAGENTA, tui.BLUE, 0)
    parts[#parts+1] = string.rep(' ', PIANO_W)

    local ruler_chars = {}
    local skip_until  = -1
    for col = 0, gw - 1 do
        if col <= skip_until then
            ruler_chars[col+1] = ""
        else
            local beat      = st.view_left + col / z
            local at_cursor = (math.abs(beat - snap(st.cursor_beat, 1/z)) < 0.5/z)
            local at_loop   = (beat >= st.loop_len - 0.5/z and beat < st.loop_len + 0.5/z)
            if at_loop then
                ruler_chars[col+1] = "\1║"
            elseif at_cursor then
                ruler_chars[col+1] = "\2▼"
            elseif math.abs(beat % 1.0) < 0.5/z then
                local s = tostring(math.floor(beat) + 1)
                if col + #s <= gw then
                    ruler_chars[col+1] = "\3" .. s
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
    for _, rc in ipairs(ruler_chars) do
        if rc == "" then
            -- skip (already part of a multi-char number)
        elseif rc:sub(1,1) == "\1" then
            emit(tui.BRIGHT_WHITE, tui.BLUE, tui.BOLD); parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\2" then
            emit(tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD);         parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\3" then
            emit(tui.BRIGHT_WHITE, tui.BLUE, 0);        parts[#parts+1] = rc:sub(2)
        elseif rc:sub(1,1) == "\4" then
            emit(tui.MAGENTA, tui.BLUE, 0);        parts[#parts+1] = rc:sub(2)
        else
            emit(tui.MAGENTA, tui.BLUE, 0);        parts[#parts+1] = rc
        end
    end
    parts[#parts+1] = RST
    tui.move(2, 1); tui.print(table.concat(parts))

    -- ── Rows 3…3+gh-1: pitch grid ─────────────────────────────────────────────
    for row = 0, gh - 1 do
        local pitch = st.view_top - row
        if pitch < 0 or pitch > 127 then
            tui.print_at(3 + row, 1, string.rep(' ', w), tui.BLUE, tui.BLUE, 0)
        else
            local black  = is_black(pitch)
            local is_c   = (pitch % 12 == 0)
            -- Cursor highlight only when in note mode
            local at_cur_pitch = (pitch == st.cursor_pitch) and (st.active_lane == 0)

            parts = {}
            emit  = make_emitter(parts)

            -- Pitch label
            if at_cur_pitch then
                emit(tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
            elseif black then
                emit(tui.BRIGHT_WHITE, tui.BLUE, tui.BOLD)
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
                local ch, fg, bg, attrs = " ", tui.MAGENTA, tui.BLUE, 0

                if at_loop then
                    ch = "│"; fg = tui.BRIGHT_WHITE; bg = tui.BLUE; attrs = tui.BOLD
                elseif ni then
                    local is_head = (beat < n.start + 1/z)
                    ch = is_head and "▐" or "█"
                    if at_cursor then
                        fg, bg, attrs = tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD
                    elseif n.sel then
                        fg, bg, attrs = tui.BLACK, tui.YELLOW, tui.BOLD
                    else
                        -- Shade by velocity: loud=bright, soft=dim
                        local vel = n.velocity or 100
                        if vel >= 100 then
                            fg, bg, attrs = tui.BLACK, tui.BRIGHT_GREEN, 0
                        elseif vel >= 50 then
                            fg, bg, attrs = tui.BLACK, tui.GREEN, 0
                        else
                            fg, bg, attrs = tui.GREEN, tui.BLUE, 0
                        end
                    end
                elseif at_cursor then
                    ch = "│"; fg = tui.CYAN; bg = tui.BLUE; attrs = tui.BOLD
                elseif black then
                    ch = " "; fg = tui.BRIGHT_BLACK; bg = tui.BRIGHT_BLACK -- playhead
                else
                    ch = on_beat and "·" or " "
                    fg = tui.BRIGHT_BLACK; bg = tui.BLUE
                end
                emit(fg, bg, attrs)
                parts[#parts+1] = ch
            end

            parts[#parts+1] = RST
            tui.move(3 + row, 1)
            tui.print(table.concat(parts))
        end
    end

    -- ── CC Lanes ──────────────────────────────────────────────────────────────
    for li = 1, #st.cc_lanes do
        local cc_num    = st.cc_lanes[li]
        local lane_row  = 3 + gh + (li - 1) * (CC_LANE_H + 1)
        local is_active = (st.active_lane == li)

        -- Header row
        local lbl     = lane_label(cc_num, st.cc_names, st.nrpn_names, st.sysex_names)
        local cur_str = string.format("%.3f", st.cursor_beat)
        local hdr_str = string.format(" %s %s  val:%-4d  cursor:%s",
            is_active and "▶" or " ", lbl, st.cc_cur_val, cur_str)
        if lane_row <= h - 2 then
            tui.print_at(lane_row, 1, pad(hdr_str, w),
                is_active and tui.BLACK    or tui.BRIGHT_BLACK,
                is_active and tui.MAGENTA  or tui.BLUE,
                is_active and tui.BOLD     or 0)
        end

        -- Bar rows
        for bar_row = 1, CC_LANE_H do
            local scr_row = lane_row + bar_row
            if scr_row > h - 2 then break end  -- protect footer

            parts = {}
            emit  = make_emitter(parts)

            -- Left label column (same width as PIANO_W)
            emit(tui.MAGENTA, tui.BLUE, 0)
            parts[#parts+1] = string.rep(' ', PIANO_W)

            for col = 0, gw - 1 do
                local beat = st.view_left + col / z
                local held = cc_value_before_beat(st.cc_extras, cc_num, beat)
                if held == nil then held = 0 end

                -- Normalize value to 0..1
                local norm
                if cc_num == CC_PB then
                    norm = (held + 8192) / 16383
                elseif is_sysex_lane(cc_num) then
                    local sr = st.sysex_ranges and st.sysex_ranges[sysex_param_index(cc_num)]
                    local max_v = sr and sr.max_val or 127
                    norm = max_v > 0 and (held / max_v) or 0
                elseif is_nrpn_lane(cc_num) then
                    norm = held / 16383
                else
                    norm = held / 127
                end

                local bar_h = math.floor(norm * CC_LANE_H + 0.5)
                -- row_from_bottom: 1 = bottom row, CC_LANE_H = top row
                local row_from_bottom = CC_LANE_H - bar_row + 1
                local inside_bar  = (row_from_bottom <= bar_h)
                local is_top_of_bar = (row_from_bottom == bar_h) or (bar_h == 0 and row_from_bottom == 1)

                local hw = 0.5 / z
                local at_cursor_col = (math.abs(beat - snap(st.cursor_beat, 1/z)) < hw)
                local has_ev  = cc_has_event_near(st.cc_extras, cc_num, beat, hw)
                local on_beat_col = math.abs(beat % 1.0) < hw

                local ch, fg, bg, attrs = " ", tui.MAGENTA, tui.BLUE, 0

                if inside_bar then
                    if at_cursor_col and is_active then
                        ch = "█"; fg = tui.BLACK; bg = tui.MAGENTA; attrs = tui.BOLD
                    elseif has_ev and is_top_of_bar then
                        ch = "▐"; fg = tui.MAGENTA; bg = tui.BLUE; attrs = 0
                    elseif has_ev then
                        ch = "█"; fg = tui.MAGENTA; bg = tui.BLUE; attrs = 0
                    else
                        ch = "▒"; fg = tui.BRIGHT_BLACK; bg = tui.BLUE; attrs = 0
                    end
                else
                    -- Above bar
                    if at_cursor_col and is_active then
                        ch = "│"; fg = tui.MAGENTA; bg = tui.BLUE; attrs = tui.BOLD
                    elseif on_beat_col and row_from_bottom == CC_LANE_H then
                        ch = "·"; fg = tui.BRIGHT_BLACK; bg = tui.BLUE; attrs = 0
                    else
                        ch = " "; fg = tui.BRIGHT_BLACK; bg = tui.BLUE; attrs = 0
                    end
                end
                emit(fg, bg, attrs)
                parts[#parts+1] = ch
            end

            parts[#parts+1] = RST
            tui.move(scr_row, 1)
            tui.print(table.concat(parts))
        end
    end

    -- ── Footer rows ───────────────────────────────────────────────────────────
    tui.print_at(h - 1, 1, pad(st.status, w), tui.BRIGHT_WHITE, tui.BLUE, 0)
    local hint
    if st.active_lane > 0 then
        hint = " ←→=move  ↑↓=±1 val  Shift↑↓=±10  Enter=place  Del=erase" ..
               "  v=set val  -=rm lane  Tab=note mode  S=save  ESC=cancel"
    else
        hint = " ←→↑↓=navigate  Shift+←→↑↓=sel-range  SPC=sel  A=sel-all" ..
               "  +/-=transpose  ,/.=length  z/x=position  ENTER=add/del  DEL=delete" ..
               "  []={Q}  Q=quant  R=rev  I=inv  C/^V/^X=copy/paste/cut  V=vel  L=loop  O=loop-tog" ..
               "  F=+CC  Tab=CC  S=save  ESC=cancel"
    end
    tui.print_at(h, 1, pad(hint, w), tui.MAGENTA, tui.BLUE, 0)
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
    local total_lane_rows = #st.cc_lanes * (CC_LANE_H + 1)
    local gh = math.max(4, h - 4 - total_lane_rows)
    local vb = gw / z  -- visible beats

    if st.cursor_beat < st.view_left then
        st.view_left = snap(st.cursor_beat, Q_VALS[st.q_idx])
    elseif st.cursor_beat >= st.view_left + vb then
        st.view_left = snap(st.cursor_beat - vb + Q_VALS[st.q_idx], Q_VALS[st.q_idx])
    end
    -- Only pitch-scroll in note mode
    if st.active_lane == 0 then
        if st.cursor_pitch > st.view_top then
            st.view_top = st.cursor_pitch
        elseif st.cursor_pitch < st.view_top - gh + 1 then
            st.view_top = st.cursor_pitch + gh - 1
        end
    end
end

-- ── Entry point ───────────────────────────────────────────────────────────────

function M.open(clip, raw_events, drum_map, cc_names, nrpn_names, sysex_names, sysex_ranges)
    RST = tui.reset()

    local w, h = tui.size()
    local notes, cc_extras = events_to_notes_and_cc(raw_events or {})
    local st = {
        clip        = clip,
        notes       = notes,
        cc_extras   = cc_extras,   -- non-note events (CC, PB) from input + edits
        cc_lanes    = {},          -- list of active CC numbers (128=pitch bend)
        active_lane = 0,           -- 0=note mode, 1..n=CC lane index
        cc_cur_val  = 64,          -- value staged for placement with Enter
        loop_len    = clip.length_beats or 4.0,
        looping     = (clip.is_looping == 1 or clip.is_looping == true),
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
        drum_map      = drum_map,      -- {[note]=name} or nil
        cc_names      = cc_names,      -- {[cc_number]=name} or nil
        nrpn_names    = nrpn_names,    -- {[nrpn_number]=name} or nil
        sysex_names   = sysex_names,   -- {[param_index]=name} or nil
        sysex_ranges  = sysex_ranges,  -- {[param_index]={min_val,max_val,default_val}} or nil
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

        -- ── Shared keys (both note mode and CC mode) ──────────────────────────
        if key == 'home' then
            st.cursor_beat = 0; st.view_left = 0

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

        elseif key == 's' or key == 'S' then
            result = notes_and_cc_to_events(st.notes, st.cc_extras)
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

        elseif key == 'tab' then
            -- Cycle: note mode (0) → CC lane 1 → CC lane 2 → … → note mode (0)
            if #st.cc_lanes == 0 then
                st.active_lane = 0
                st.status = "No CC lanes — press F to add one"
            else
                st.active_lane = (st.active_lane + 1) % (#st.cc_lanes + 1)
                if st.active_lane > 0 then
                    local cc_num = st.cc_lanes[st.active_lane]
                    local Q = Q_VALS[st.q_idx]
                    local ev_val = cc_event_at_beat(st.cc_extras, cc_num, st.cursor_beat, Q)
                    if ev_val ~= nil then
                        st.cc_cur_val = ev_val
                    else
                        local held = cc_value_before_beat(st.cc_extras, cc_num, st.cursor_beat)
                        if is_sysex_lane(cc_num) then
                            local sr = st.sysex_ranges and st.sysex_ranges[sysex_param_index(cc_num)]
                            st.cc_cur_val = held or (sr and sr.default_val or 0)
                        else
                            st.cc_cur_val = held or ((cc_num == CC_PB or is_nrpn_lane(cc_num)) and 0 or 64)
                        end
                    end
                    local lbl = lane_label(cc_num, st.cc_names, st.nrpn_names, st.sysex_names)
                    st.status = string.format("CC lane: %s  val:%d", lbl, st.cc_cur_val)
                else
                    st.status = "Note mode"
                end
            end

        -- ── CC mode keys ──────────────────────────────────────────────────────
        elseif st.active_lane > 0 then
            local cc_num = st.cc_lanes[st.active_lane]
            local Q = Q_VALS[st.q_idx]
            local pb_step = 64
            local pb_big  = 512

            local is_sysex = is_sysex_lane(cc_num)
            local is_14bit = (cc_num == CC_PB or is_nrpn_lane(cc_num))
            local sx_range = is_sysex and st.sysex_ranges and st.sysex_ranges[sysex_param_index(cc_num)]
            local sx_min = sx_range and sx_range.min_val or 0
            local sx_max = sx_range and sx_range.max_val or 127

            if key == 'right' then
                st.cursor_beat = math.max(0, st.cursor_beat + Q)
                -- Update cc_cur_val to event at new position or held
                local ev_val = cc_event_at_beat(st.cc_extras, cc_num, st.cursor_beat, Q)
                if ev_val ~= nil then st.cc_cur_val = ev_val end

            elseif key == 'left' then
                st.cursor_beat = math.max(0, st.cursor_beat - Q)
                local ev_val = cc_event_at_beat(st.cc_extras, cc_num, st.cursor_beat, Q)
                if ev_val ~= nil then st.cc_cur_val = ev_val end

            elseif key == 'up' then
                if is_sysex then
                    st.cc_cur_val = math.min(sx_max, st.cc_cur_val + 1)
                elseif is_14bit then
                    st.cc_cur_val = math.min(cc_num == CC_PB and 8191 or 16383, st.cc_cur_val + pb_step)
                else
                    st.cc_cur_val = math.min(127, st.cc_cur_val + 1)
                end
            elseif key == 'down' then
                if is_sysex then
                    st.cc_cur_val = math.max(sx_min, st.cc_cur_val - 1)
                elseif is_14bit then
                    st.cc_cur_val = math.max(cc_num == CC_PB and -8192 or 0, st.cc_cur_val - pb_step)
                else
                    st.cc_cur_val = math.max(0, st.cc_cur_val - 1)
                end
            elseif key == 'shift-up' then
                if is_sysex then
                    st.cc_cur_val = math.min(sx_max, st.cc_cur_val + 10)
                elseif is_14bit then
                    st.cc_cur_val = math.min(cc_num == CC_PB and 8191 or 16383, st.cc_cur_val + pb_big)
                else
                    st.cc_cur_val = math.min(127, st.cc_cur_val + 10)
                end
            elseif key == 'shift-down' then
                if is_sysex then
                    st.cc_cur_val = math.max(sx_min, st.cc_cur_val - 10)
                elseif is_14bit then
                    st.cc_cur_val = math.max(cc_num == CC_PB and -8192 or 0, st.cc_cur_val - pb_big)
                else
                    st.cc_cur_val = math.max(0, st.cc_cur_val - 10)
                end

            elseif key == 'enter' or key == 'return' then
                local beat = snap(st.cursor_beat, Q)
                st.cc_extras = cc_place(st.cc_extras, cc_num, beat, st.cc_cur_val)
                st.dirty = true
                local lbl = lane_label(cc_num, st.cc_names, st.nrpn_names, st.sysex_names)
                st.status = string.format("CC placed: %s = %d @ %.3f", lbl, st.cc_cur_val, beat)

            elseif key == 'del' or key == 'backspace' then
                local new_extras, cnt = cc_erase(st.cc_extras, cc_num, st.cursor_beat, Q)
                if cnt > 0 then
                    st.cc_extras = new_extras; st.dirty = true
                    st.status = string.format("Erased %d CC event(s)", cnt)
                else
                    st.status = "No CC event at cursor"
                end

            elseif key == 'v' or key == 'V' then
                local range_str
                if cc_num == CC_PB then range_str = "-8192..8191"
                elseif is_sysex then range_str = sx_min .. ".." .. sx_max
                elseif is_nrpn_lane(cc_num) then range_str = "0..16383"
                else range_str = "0..127"
                end
                local val_str = read_line(h - 1, "CC value (" .. range_str .. "): ",
                                          tostring(st.cc_cur_val))
                tui.clear()
                if val_str then
                    local n = tonumber(val_str)
                    if n then
                        if cc_num == CC_PB then
                            st.cc_cur_val = math.max(-8192, math.min(8191, math.floor(n)))
                        elseif is_sysex then
                            st.cc_cur_val = math.max(sx_min, math.min(sx_max, math.floor(n)))
                        elseif is_nrpn_lane(cc_num) then
                            st.cc_cur_val = math.max(0, math.min(16383, math.floor(n)))
                        else
                            st.cc_cur_val = math.max(0, math.min(127, math.floor(n)))
                        end
                        st.status = string.format("CC value set to %d", st.cc_cur_val)
                    end
                end

            elseif key == '-' then
                -- Remove current CC lane
                table.remove(st.cc_lanes, st.active_lane)
                st.active_lane = math.min(st.active_lane, #st.cc_lanes)
                -- active_lane=0 means note mode now if no more lanes, or stayed on previous lane
                if #st.cc_lanes == 0 then
                    st.active_lane = 0
                    st.status = "CC lane removed; back to note mode"
                else
                    -- If we removed the last lane, go to note mode; otherwise stay
                    if st.active_lane == 0 then
                        st.status = "CC lane removed; back to note mode"
                    else
                        local lbl = lane_label(st.cc_lanes[st.active_lane], st.cc_names, st.nrpn_names, st.sysex_names)
                        st.status = string.format("CC lane removed; now on %s", lbl)
                    end
                end

            elseif key == '?' then
                show_help("cc", w, h)
            end

        -- ── Note mode keys ────────────────────────────────────────────────────
        else
            if key == 'right' or key == 'shift-right' then
                st.cursor_beat = math.max(0, st.cursor_beat + Q_VALS[st.q_idx])
                if key == 'shift-right' then
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

            -- Operations
            elseif key == 'q' or key == 'Q' then op_quantize(st)
            elseif key == 'r' or key == 'R' then op_reverse(st)
            elseif key == 'i' or key == 'I' then op_invert(st)
            elseif key == 'c' or key == 'C' then op_copy(st)
            elseif key == 'v' or key == 'V' then
                -- Set velocity for note at cursor, or all selected notes
                local beat = snap(st.cursor_beat, Q_VALS[st.q_idx])
                local ni, n = note_at(st.notes, st.cursor_pitch, beat)
                local sc = sel_count(st.notes)
                local default = n and tostring(n.velocity) or "100"
                local val_str = read_line(h - 1, "Velocity (0-127): ", default)
                tui.clear()
                if val_str then
                    local vel = tonumber(val_str)
                    if vel then
                        vel = math.max(0, math.min(127, math.floor(vel)))
                        if sc > 0 then
                            for _, sn in ipairs(st.notes) do
                                if sn.sel then sn.velocity = vel end
                            end
                            st.status = string.format("Velocity → %d (%d notes)", vel, sc)
                            st.dirty = true
                        elseif ni then
                            n.velocity = vel
                            st.status = string.format("Velocity → %d  (%s @ %.3f)",
                                vel, note_name(n.pitch), n.start)
                            st.dirty = true
                        else
                            st.status = "No note at cursor"
                        end
                    end
                end
            elseif key == 'ctrl-v' then op_paste(st)
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

            -- Help
            elseif key == '?' then
                show_help("note", w, h)

            -- Add CC lane
            elseif key == 'f' or key == 'F' then
                local val = read_line(h - 1, "CC (0-128), N=NRPN, S=SysEx: ", "")
                tui.clear()
                if val then
                    local cc_num = nil
                    local sysex_input = val:match("^[sS](%d+)$")
                    local nrpn_input = val:match("^[nN](%d+)$")
                    if sysex_input then
                        local sn = tonumber(sysex_input)
                        if sn and sn >= 0 then
                            cc_num = SYSEX_BASE + math.floor(sn)
                        end
                    elseif val:match("^[sS]$") then
                        local sval = read_line(h - 1, "SysEx param index: ", "")
                        tui.clear()
                        if sval then
                            local sn = tonumber(sval)
                            if sn and sn >= 0 then
                                cc_num = SYSEX_BASE + math.floor(sn)
                            end
                        end
                    elseif nrpn_input then
                        local nn = tonumber(nrpn_input)
                        if nn and nn >= 0 and nn <= 16383 then
                            cc_num = NRPN_BASE + math.floor(nn)
                        end
                    elseif val:match("^[nN]$") then
                        -- User typed just 'n', prompt for NRPN number
                        local nval = read_line(h - 1, "NRPN number (0-16383): ", "")
                        tui.clear()
                        if nval then
                            local nn = tonumber(nval)
                            if nn and nn >= 0 and nn <= 16383 then
                                cc_num = NRPN_BASE + math.floor(nn)
                            end
                        end
                    else
                        local n = tonumber(val)
                        if n and math.floor(n) == n and n >= 0 and n <= 128 then
                            cc_num = math.floor(n)
                        end
                    end
                    if cc_num then
                        -- Check for duplicate
                        local dup = false
                        for _, existing in ipairs(st.cc_lanes) do
                            if existing == cc_num then dup = true; break end
                        end
                        if dup then
                            st.status = "Lane already exists"
                        else
                            st.cc_lanes[#st.cc_lanes+1] = cc_num
                            st.active_lane = #st.cc_lanes
                            -- Sync cc_cur_val
                            local Q = Q_VALS[st.q_idx]
                            local ev_val = cc_event_at_beat(st.cc_extras, cc_num, st.cursor_beat, Q)
                            if ev_val ~= nil then
                                st.cc_cur_val = ev_val
                            else
                                local held = cc_value_before_beat(st.cc_extras, cc_num, st.cursor_beat)
                                if is_sysex_lane(cc_num) then
                                    local sr = st.sysex_ranges and st.sysex_ranges[sysex_param_index(cc_num)]
                                    st.cc_cur_val = held or (sr and sr.default_val or 0)
                                else
                                    st.cc_cur_val = held or ((cc_num == CC_PB or is_nrpn_lane(cc_num)) and 0 or 64)
                                end
                            end
                            local lbl = lane_label(cc_num, st.cc_names, st.nrpn_names, st.sysex_names)
                            st.status = string.format("Added lane: %s", lbl)
                        end
                    else
                        st.status = "Invalid input — CC 0-128, N0-N16383, or S0-Sn"
                    end
                end
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

M.NOTE_NAMES = NOTE_NAMES

return M

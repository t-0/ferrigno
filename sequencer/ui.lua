-- sequencer/ui.lua
-- TUI rendering for the Ableton-style session view.
--
-- NOTE: tui.color() returns an ANSI escape string (does NOT write to terminal).
--       Use tui.print_at(row, col, text, fg, bg, attrs) for coloured output, or
--       io.write(tui.color(...)) + io.write(text) to maintain state across writes.

local M = {}

-- ── Layout constants ──────────────────────────────────────────────────────────

local COL_W       = 14   -- characters per track column
local SCENE_W     = 9    -- characters for the scene-label column
local HEADER_ROWS = 3    -- [1] status bar  [2] track headers  [3] separator
local FOOTER_ROWS = 2    -- controls hint + blank/recording line

-- Track accent colours (cycles per track index).
local TRACK_COLORS = {
    tui.GREEN, tui.CYAN, tui.MAGENTA, tui.YELLOW,
    tui.BRIGHT_GREEN, tui.BRIGHT_CYAN, tui.BRIGHT_MAGENTA, tui.BRIGHT_YELLOW,
}

-- ── Public state (set by main before first draw) ──────────────────────────────

M.song        = nil
M.instruments = {}
M.clips       = {}    -- clips[inst_id][slot_idx] = clip_table
M.scenes      = {}    -- indexed 1..n, each has .scene_index (0-based) and .name
M.num_slots   = 8

M.cursor    = { track = 1, slot = 1 }
M.scroll_x  = 0   -- first visible track index offset (0-based)
M.scroll_y  = 0   -- first visible slot  index offset (0-based)

M.status_msg   = "Ready"
M.status_color = tui.BRIGHT_WHITE
M.playing      = false
M.recording    = false
M.bpm          = 120.0
M.arp_states   = nil   -- reference to arp.states (set by main)

-- ── Internal helpers ──────────────────────────────────────────────────────────

local function pad(s, n)
    s = tostring(s or '')
    if #s >= n then return s:sub(1, n) end
    return s .. string.rep(' ', n - #s)
end

local function center(s, n)
    s = tostring(s or '')
    if #s >= n then return s:sub(1, n) end
    local l = math.floor((n - #s) / 2)
    return string.rep(' ', l) .. s .. string.rep(' ', n - #s - l)
end

-- tui.print_at with colour; shorthand used throughout draw().
local function put(row, col, text, fg, bg, attrs)
    tui.print_at(row, col, text, fg, bg, attrs or 0)
end

local function visible_tracks(w)
    return math.max(1, math.floor((w - SCENE_W) / COL_W))
end

local function visible_rows(h)
    return math.max(1, h - HEADER_ROWS - FOOTER_ROWS)
end

-- ── Draw ──────────────────────────────────────────────────────────────────────

function M.draw(engine)
    local w, h = tui.size()
    local vt   = visible_tracks(w)
    local vr   = visible_rows(h)

    tui.hide_cursor()

    -- ── Row 1: transport / BPM / song name ───────────────────────────────────
    local tfg, tbg, tbold
    local transport_label
    if M.recording then
        transport_label = " ● REC     "
        tfg, tbg, tbold = tui.WHITE,         tui.RED,         tui.BOLD
    elseif M.playing then
        transport_label = " ► PLAYING "
        tfg, tbg, tbold = tui.BLACK,         tui.GREEN,       tui.BOLD
    else
        transport_label = " ■ STOPPED "
        tfg, tbg, tbold = tui.BRIGHT_WHITE,  tui.BRIGHT_BLACK, tui.BOLD
    end
    local bpm_s  = string.format(" BPM:%.1f", M.bpm)
    local name_s = "  Song: " .. (M.song and M.song.name or "Untitled")
    put(1, 1, pad(transport_label .. bpm_s .. name_s, w), tfg, tbg, tbold)

    -- ── Row 2: track name headers ─────────────────────────────────────────────
    put(2, 1, pad("", SCENE_W), tui.WHITE, tui.BRIGHT_BLACK, tui.BOLD)
    for i = 1, vt do
        local ti   = i + M.scroll_x
        local inst = M.instruments[ti]
        local name = inst and inst.name or ""
        local col  = TRACK_COLORS[((ti - 1) % #TRACK_COLORS) + 1]
        local arp_on = M.arp_states and M.arp_states[ti] ~= nil
        local display_name = arp_on and ("♩" .. name) or name
        if ti == M.cursor.track then
            put(2, SCENE_W + (i-1)*COL_W + 1, center(display_name, COL_W), tui.BLACK, col, tui.BOLD)
        else
            put(2, SCENE_W + (i-1)*COL_W + 1, center(display_name, COL_W), col, tui.BRIGHT_BLACK, tui.BOLD)
        end
    end
    -- Blank out remainder of track-header row.
    local used2 = SCENE_W + vt * COL_W
    if used2 < w then
        put(2, used2 + 1, string.rep(' ', w - used2), tui.WHITE, tui.BRIGHT_BLACK, 0)
    end

    -- ── Row 3: separator ──────────────────────────────────────────────────────
    local sep = string.rep('─', SCENE_W)
    for i = 1, vt do sep = sep .. '┬' .. string.rep('─', COL_W - 1) end
    put(3, 1, pad(sep, w), tui.BRIGHT_BLACK, tui.BLACK, 0)

    -- ── Rows 4+: clip grid ────────────────────────────────────────────────────
    for row = 1, vr do
        local si  = row + M.scroll_y     -- 1-based slot index
        local scn = M.scenes[si]
        local sc_name = scn and (scn.name ~= '' and scn.name or ("Scene " .. si)) or ("Scene " .. si)

        -- Scene label
        if si == M.cursor.slot then
            put(HEADER_ROWS + row, 1, pad(sc_name:sub(1, SCENE_W-1), SCENE_W), tui.BLACK, tui.CYAN, tui.BOLD)
        else
            put(HEADER_ROWS + row, 1, pad(sc_name:sub(1, SCENE_W-1), SCENE_W), tui.BRIGHT_WHITE, tui.BRIGHT_BLACK, 0)
        end

        -- Clip cells
        for i = 1, vt do
            local ti      = i + M.scroll_x
            local inst    = M.instruments[ti]
            local inst_id = inst and inst.id
            local clip    = inst_id and M.clips[inst_id] and M.clips[inst_id][si]
            local ac      = engine and engine.active_clips[ti]
            local playing = ac and clip and (ac.clip_id == clip.id)
            local is_cur  = (ti == M.cursor.track and si == M.cursor.slot)
            local tcol    = TRACK_COLORS[((ti - 1) % #TRACK_COLORS) + 1]

            local label, fg, bg, attrs
            if clip then
                local cname  = (clip.name and clip.name ~= '') and clip.name or ("Clip " .. si)
                local marker = playing and "►" or " "
                label = pad(" " .. marker .. " " .. cname, COL_W)
            else
                label = pad("  [ empty ]", COL_W)
            end

            if is_cur and clip then
                fg, bg, attrs = tui.BLACK, tui.CYAN, tui.BOLD
            elseif is_cur then
                fg, bg, attrs = tui.CYAN, tui.BLACK, tui.BOLD
            elseif playing then
                fg, bg, attrs = tui.BLACK, tcol, tui.BOLD
            elseif clip then
                fg, bg, attrs = tcol, tui.BLACK, 0
            else
                fg, bg, attrs = tui.BRIGHT_BLACK, tui.BLACK, 0
            end

            put(HEADER_ROWS + row, SCENE_W + (i-1)*COL_W + 1, label, fg, bg, attrs)
        end

        -- Clear trailing columns on this row.
        local ru = SCENE_W + vt * COL_W
        if ru < w then
            put(HEADER_ROWS + row, ru + 1, string.rep(' ', w - ru), tui.WHITE, tui.BLACK, 0)
        end
    end

    -- Clear rows between grid and footer.
    for row = vr + 1, h - FOOTER_ROWS do
        put(HEADER_ROWS + row, 1, string.rep(' ', w), tui.WHITE, tui.BLACK, 0)
    end

    -- ── Footer row 1: status message ──────────────────────────────────────────
    put(h - 1, 1, pad(M.status_msg, w), M.status_color, tui.BLACK, 0)

    -- ── Footer row 2: key hints ────────────────────────────────────────────────
    put(h, 1,
        pad(" SPC=play  ENTER=trigger  R=rec  E=edit  N=new  D=del  I=inst  B=bpm  P=arp  A=arp-mode  O=arp-rate  +=track  S=save  Q=quit", w),
        tui.BRIGHT_BLACK, tui.BLACK, 0)

    tui.flush()
end

-- ── Text input ────────────────────────────────────────────────────────────────

-- Read a line of text from the user while staying in raw mode.
-- Draws the prompt at (prompt_row, 1) and edits it in-place.
-- Returns the entered string, or nil if the user pressed Escape.
function M.read_line(prompt_row, label, default)
    local w = tui.size()
    local buf = default or ''

    local function redraw()
        local prompt_text = ' ' .. label
        put(prompt_row, 1, pad(prompt_text, #prompt_text + 1), tui.BLACK, tui.CYAN, tui.BOLD)
        -- Write the input buffer in default colours after the prompt.
        tui.print(tui.color(tui.WHITE, tui.BLACK, 0))
        tui.print(pad(buf, w - #prompt_text - 1))
        tui.print(tui.reset())
        -- Position cursor at end of input.
        tui.move(prompt_row, #prompt_text + 2 + #buf)
        tui.show_cursor()
    end

    redraw()

    while true do
        local key = tui.read_key(10000)
        if not key then goto again end

        if key == 'enter' or key == 'return' then
            tui.hide_cursor()
            tui.print(tui.reset())
            return buf
        elseif key == 'esc' then
            tui.hide_cursor()
            tui.print(tui.reset())
            return nil
        elseif key == 'backspace' then
            if #buf > 0 then buf = buf:sub(1, -2) end
        elseif #key == 1 then
            buf = buf .. key
        end
        redraw()
        ::again::
    end
end

-- ── Cursor movement ───────────────────────────────────────────────────────────

function M.move_cursor(dt, ds)
    local w, h  = tui.size()
    local vt    = visible_tracks(w)
    local vr    = visible_rows(h)
    local max_t = math.max(1, #M.instruments)
    local max_s = M.num_slots

    M.cursor.track = math.max(1, math.min(max_t, M.cursor.track + dt))
    M.cursor.slot  = math.max(1, math.min(max_s, M.cursor.slot  + ds))

    if M.cursor.track <= M.scroll_x then
        M.scroll_x = M.cursor.track - 1
    elseif M.cursor.track > M.scroll_x + vt then
        M.scroll_x = M.cursor.track - vt
    end

    if M.cursor.slot <= M.scroll_y then
        M.scroll_y = M.cursor.slot - 1
    elseif M.cursor.slot > M.scroll_y + vr then
        M.scroll_y = M.cursor.slot - vr
    end
end

-- ── Status ────────────────────────────────────────────────────────────────────

function M.set_status(msg, color)
    M.status_msg   = tostring(msg or '')
    M.status_color = color or tui.BRIGHT_WHITE
end

return M

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
M.tracks      = {}
M.clips       = {}    -- clips[track_id][slot_idx] = clip_table
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
M.track_has_arp = nil   -- {[track_idx]=true} for tracks with active arp
M.studio_name  = "all"
M.on_idle      = nil   -- called each 10 ms tick when no key is available

-- Chain panel state (bottom panel showing device chain for selected track).
M.focus        = "grid"   -- "grid" or "chain"
M.chain_def    = nil       -- chain_def for bottom panel (set by main each frame)
M.chain_label  = ""        -- "Track: Synth" or "Master"
M.chain_cursor = 1         -- selected device index (1-based)

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

-- Chain panel sizing: 2 rows when visible (separator + device row),
-- 3 rows when focused (+ key hints row).
local CHAIN_PANEL_MIN = 2
local CHAIN_PANEL_MAX = 3

local function chain_panel_rows()
    if not M.chain_def then return 0 end
    return M.focus == "chain" and CHAIN_PANEL_MAX or CHAIN_PANEL_MIN
end

local function visible_rows(h)
    return math.max(1, h - HEADER_ROWS - FOOTER_ROWS - chain_panel_rows())
end

-- ── Compact device label for chain panel ─────────────────────────────────────

local function compact_device_label(dev)
    local name = dev.name or dev.type or "?"
    local p = dev.params or {}
    local extra
    if dev.type == "transpose" then
        local s = tonumber(p.semitones or 0) or 0
        extra = (s >= 0 and "+" or "") .. tostring(s)
    elseif dev.type == "velocity_curve" then
        extra = p.curve or "linear"
    elseif dev.type == "channel_remap" then
        extra = "ch:" .. (p.channel or "1")
    elseif dev.type == "cc_filter" then
        extra = (p.mode or "block") .. " " .. (p.cc_list or "")
    elseif dev.type == "arp" then
        extra = p.mode or "up"
    elseif dev.type == "instrument" then
        return name
    elseif dev.type == "lua_script" then
        extra = (p.script_path and p.script_path ~= '') and p.script_path or "inline"
    end
    if extra then
        return name .. " " .. extra
    end
    return name
end

-- ── Chain panel drawing ──────────────────────────────────────────────────────

local function draw_chain_panel(w, h)
    if not M.chain_def then return end
    local cp_rows = chain_panel_rows()
    local panel_top = h - FOOTER_ROWS - cp_rows + 1
    local focused = (M.focus == "chain")

    -- Row 1: separator with chain label.
    local label = " Chain: " .. (M.chain_label or "") .. " "
    local sep_fg = focused and tui.CYAN or tui.MAGENTA
    local left_bar = string.rep('─', 2)
    local right_len = math.max(0, w - #left_bar - #label - 12)
    local tab_hint = focused and " [TAB=grid]" or " [TAB=chain]"
    local right_bar = string.rep('─', math.max(0, right_len - #tab_hint)) .. tab_hint
    put(panel_top, 1, pad(left_bar .. label .. right_bar, w), sep_fg, tui.BLUE, 0)

    -- Row 2: devices laid out horizontally.
    local devs = M.chain_def.devices or {}
    local parts = {}
    for i, dev in ipairs(devs) do
        local lbl = compact_device_label(dev)
        local disabled = (dev.enabled == false or dev.enabled == 0)
        if disabled then lbl = "~" .. lbl .. "~" end
        parts[#parts+1] = { idx = i, label = lbl, disabled = disabled }
    end

    local dev_row = panel_top + 1
    if #parts == 0 then
        put(dev_row, 1, pad("  (empty chain)", w), tui.MAGENTA, tui.BLUE, 0)
    else
        -- Build the line: [Dev1] -> [Dev2] -> [Dev3]
        local col = 2
        for pi, part in ipairs(parts) do
            if pi > 1 then
                -- Arrow separator.
                if col + 4 <= w then
                    put(dev_row, col, " -> ", tui.MAGENTA, tui.BLUE, 0)
                    col = col + 4
                end
            end
            local bracket_label = "[" .. part.label .. "]"
            if col + #bracket_label > w then
                -- Truncate with ellipsis.
                put(dev_row, col, "...", tui.MAGENTA, tui.BLUE, 0)
                col = col + 3
                break
            end
            local is_sel = focused and (pi == M.chain_cursor)
            if is_sel then
                put(dev_row, col, bracket_label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
            elseif part.disabled then
                put(dev_row, col, bracket_label, tui.MAGENTA, tui.BLUE, 0)
            else
                put(dev_row, col, bracket_label, tui.WHITE, tui.BLUE, 0)
            end
            col = col + #bracket_label
        end
        -- Clear remainder of the device row.
        if col <= w then
            put(dev_row, col, string.rep(' ', w - col + 1), tui.WHITE, tui.BLUE, 0)
        end
    end

    -- Row 3: key hints (only when focused).
    if focused and cp_rows >= 3 then
        put(panel_top + 2, 1,
            pad(" <-/-> select  X=toggle  Enter=edit  A=add  D=del  U/J=move  Tab=grid", w),
            tui.MAGENTA, tui.BLUE, 0)
    end
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
        tfg, tbg, tbold = tui.BRIGHT_WHITE,  tui.BLUE, tui.BOLD
    end
    local bpm_s     = string.format(" BPM:%.1f", M.bpm)
    local name_s    = "  Song: " .. (M.song and M.song.name or "Untitled")
    local studio_s  = "  Studio: " .. (M.studio_name or "all")
    put(1, 1, pad(transport_label .. bpm_s .. name_s .. studio_s, w), tfg, tbg, tbold)

    -- ── Row 2: track name headers ─────────────────────────────────────────────
    put(2, 1, pad("", SCENE_W), tui.CYAN, tui.BLUE, tui.BOLD)
    for i = 1, vt do
        local ti    = i + M.scroll_x
        local track = M.tracks[ti]
        local name  = track and track.name or ""
        local col   = TRACK_COLORS[((ti - 1) % #TRACK_COLORS) + 1]
        local arp_on = M.track_has_arp and M.track_has_arp[ti]
        local display_name = arp_on and ("♩" .. name) or name
        if ti == M.cursor.track then
            put(2, SCENE_W + (i-1)*COL_W + 1, center(display_name, COL_W), tui.BLACK, col, tui.BOLD)
        else
            put(2, SCENE_W + (i-1)*COL_W + 1, center(display_name, COL_W), col, tui.BLUE, tui.BOLD)
        end
    end
    -- Blank out remainder of track-header row.
    local used2 = SCENE_W + vt * COL_W
    if used2 < w then
        put(2, used2 + 1, string.rep(' ', w - used2), tui.WHITE, tui.BLUE, 0)
    end

    -- ── Row 3: separator ──────────────────────────────────────────────────────
    local sep = string.rep('─', SCENE_W)
    for i = 1, vt do sep = sep .. '┬' .. string.rep('─', COL_W - 1) end
    put(3, 1, pad(sep, w), tui.MAGENTA, tui.BLUE, 0)

    -- ── Rows 4+: clip grid ────────────────────────────────────────────────────
    for row = 1, vr do
        local si  = row + M.scroll_y     -- 1-based slot index
        local scn = M.scenes[si]
        local sc_name = scn and (scn.name ~= '' and scn.name or ("Scene " .. si)) or ("Scene " .. si)

        -- Scene label — ► prefix when the scene column itself is selected.
        local scene_selected = (M.cursor.track == 0 and si == M.cursor.slot)
        if scene_selected then
            put(HEADER_ROWS + row, 1, pad("►" .. sc_name:sub(1, SCENE_W-2), SCENE_W), tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
        elseif si == M.cursor.slot then
            put(HEADER_ROWS + row, 1, pad(sc_name:sub(1, SCENE_W-1), SCENE_W), tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
        else
            put(HEADER_ROWS + row, 1, pad(sc_name:sub(1, SCENE_W-1), SCENE_W), tui.BRIGHT_WHITE, tui.BLUE, 0)
        end

        -- Clip cells
        local row_sel = (M.cursor.track == 0 and si == M.cursor.slot)
        for i = 1, vt do
            local ti       = i + M.scroll_x
            local track    = M.tracks[ti]
            local track_id = track and track.id
            local clip     = track_id and M.clips[track_id] and M.clips[track_id][si]
            local ac      = engine and engine.active_clips[ti]
            local ac_hit  = ac and clip and (ac.clip_id == clip.id)
            local now_b   = engine and engine.cur_beat() or 0
            local queued  = ac_hit and (now_b < (ac.loop_start_beat or 0))
            local playing = ac_hit and not queued
            local is_cur  = (ti == M.cursor.track and si == M.cursor.slot)
            local tcol    = TRACK_COLORS[((ti - 1) % #TRACK_COLORS) + 1]

            local label, fg, bg, attrs
            if clip then
                local cname  = (clip.name and clip.name ~= '') and clip.name or ("Clip " .. si)
                local marker = playing and "►" or (queued and "○" or " ")
                label = pad(" " .. marker .. " " .. cname, COL_W)
            elseif track then
                label = pad("  [ empty ]", COL_W)
            else
                label = pad("", COL_W)
            end

            if is_cur and clip then
                fg, bg, attrs = tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD
            elseif is_cur then
                fg, bg, attrs = tui.CYAN, tui.BLUE, tui.BOLD
            elseif playing then
                fg, bg, attrs = tui.BLACK, tcol, tui.BOLD
            elseif queued then
                fg, bg, attrs = tcol, tui.BLUE, 0
            elseif row_sel and clip then
                fg, bg, attrs = tui.BLACK, tui.BRIGHT_MAGENTA, 0
            elseif row_sel then
                fg, bg, attrs = tui.BRIGHT_MAGENTA, tui.BLUE, 0
            elseif clip then
                fg, bg, attrs = tcol, tui.BLUE, 0
            else
                fg, bg, attrs = tui.MAGENTA, tui.BLUE, 0
            end

            put(HEADER_ROWS + row, SCENE_W + (i-1)*COL_W + 1, label, fg, bg, attrs)
        end

        -- Clear trailing columns on this row.
        local ru = SCENE_W + vt * COL_W
        if ru < w then
            put(HEADER_ROWS + row, ru + 1, string.rep(' ', w - ru), tui.WHITE, tui.BLUE, 0)
        end
    end

    -- Clear rows between grid and chain panel / footer.
    local cp_rows = chain_panel_rows()
    for row = vr + 1, h - FOOTER_ROWS - cp_rows do
        put(HEADER_ROWS + row, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, 0)
    end

    -- ── Chain panel (between grid and footer) ────────────────────────────────
    draw_chain_panel(w, h)

    -- ── Footer row 1: status message ──────────────────────────────────────────
    put(h - 1, 1, pad(M.status_msg, w), M.status_color, tui.BLUE, 0)

    -- ── Footer row 2: key hints ────────────────────────────────────────────────
    put(h, 1,
        pad(" SPC=play  ENTER=trigger  L=scene  R=rec  E=roll  C=clip  N=new  D=del  [=ins-row  ]=del-row  I=inst  T=track  B=bpm  W=studio  P=arp  Tab=chain  +=add-trk  -=del-trk  S=save  ESC=quit  ?=help", w),
        tui.MAGENTA, tui.BLUE, 0)

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
        put(prompt_row, 1, pad(prompt_text, #prompt_text + 1), tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
        -- Write the input buffer in default colours after the prompt.
        tui.print(tui.color(tui.WHITE, tui.BLUE, 0))
        tui.print(pad(buf, w - #prompt_text - 1))
        tui.print(tui.reset())
        -- Position cursor at end of input.
        tui.move(prompt_row, #prompt_text + 2 + #buf)
        tui.show_cursor()
    end

    redraw()

    while true do
        local key = tui.read_key(10)
        if not key then if M.on_idle then M.on_idle() end; goto again end

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
    local max_t = math.max(1, #M.tracks)
    local max_s = M.num_slots

    M.cursor.track = math.max(0, math.min(max_t, M.cursor.track + dt))
    M.cursor.slot  = math.max(1, math.min(max_s, M.cursor.slot  + ds))

    -- Scene column (track == 0) is always visible; only scroll for actual tracks.
    if M.cursor.track > 0 then
        if M.cursor.track <= M.scroll_x then
            M.scroll_x = M.cursor.track - 1
        elseif M.cursor.track > M.scroll_x + vt then
            M.scroll_x = M.cursor.track - vt
        end
    end

    if M.cursor.slot <= M.scroll_y then
        M.scroll_y = M.cursor.slot - 1
    elseif M.cursor.slot > M.scroll_y + vr then
        M.scroll_y = M.cursor.slot - vr
    end
end

-- ── Focus toggle ─────────────────────────────────────────────────────────────

function M.toggle_focus()
    M.focus = M.focus == "grid" and "chain" or "grid"
end

-- ── Status ────────────────────────────────────────────────────────────────────

function M.set_status(msg, color)
    M.status_msg   = tostring(msg or '')
    M.status_color = color or tui.BRIGHT_WHITE
end

return M

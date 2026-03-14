-- sequencer/midi_monitor.lua
-- Full-screen MIDI monitor view.  Shows decoded incoming MIDI messages
-- in real-time, color-coded by type.  Opened with F12, dismissed with ESC.

local M = {}

M.on_idle = nil  -- set by main.lua; called each 10 ms tick

-- Ring buffer of decoded messages
local MAX_MSGS = 500
local messages = {}
local paused = false
local scroll = 0  -- lines scrolled up from bottom

-- Called by engine hook for each decoded MIDI message
function M.push(status, data1, data2, raw)
    if paused then return end
    local t = os.monotime()
    local msg_type = status & 0xF0
    local channel  = (status & 0x0F) + 1
    local decoded = {
        time    = t,
        status  = status,
        data1   = data1,
        data2   = data2,
        channel = channel,
        raw     = raw,
        type    = msg_type,
    }
    messages[#messages + 1] = decoded
    -- Trim to ring buffer size
    while #messages > MAX_MSGS do
        table.remove(messages, 1)
    end
end

-- Format a single message for display
local NOTE_NAMES = {"C","C#","D","D#","E","F","F#","G","G#","A","A#","B"}
local function note_name(n)
    return string.format("%-2s%d", NOTE_NAMES[(n % 12) + 1], math.floor(n / 12) - 1)
end

local function format_msg(m)
    local t = m.type
    local ch = string.format("Ch%2d", m.channel)
    if t == 0x90 and m.data2 > 0 then
        return ch, "Note On ", string.format("%-5s vel=%3d", note_name(m.data1), m.data2)
    elseif t == 0x80 or (t == 0x90 and m.data2 == 0) then
        return ch, "Note Off", string.format("%-5s", note_name(m.data1))
    elseif t == 0xB0 then
        return ch, "CC      ", string.format("cc=%3d val=%3d", m.data1, m.data2)
    elseif t == 0xC0 then
        return ch, "PrgChg  ", string.format("pgm=%3d", m.data1)
    elseif t == 0xE0 then
        local bend = m.data1 | (m.data2 << 7)
        return ch, "PitchBnd", string.format("val=%5d", bend - 8192)
    elseif t == 0xD0 then
        return ch, "ChanPres", string.format("val=%3d", m.data1)
    elseif t == 0xA0 then
        return ch, "PolyPres", string.format("%-5s val=%3d", note_name(m.data1), m.data2)
    elseif t == 0xF0 then
        return "Sys", "System  ", string.format("0x%02X", m.status)
    else
        return ch, "Unknown ", string.format("0x%02X %02X %02X", m.status, m.data1, m.data2)
    end
end

-- Color for message type: returns fg color constant
local function msg_color(msg_type)
    if msg_type == 0x90 then return tui.GREEN end         -- Note On
    if msg_type == 0x80 then return tui.BRIGHT_BLACK end  -- Note Off
    if msg_type == 0xB0 then return tui.YELLOW end        -- CC
    if msg_type == 0xC0 then return tui.MAGENTA end       -- Program Change
    if msg_type == 0xE0 then return tui.CYAN end          -- Pitch Bend
    return tui.WHITE
end

local function draw()
    local w, h = tui.size()
    tui.hide_cursor()
    tui.clear()
    -- Title bar
    local title = string.format(" MIDI Monitor — %d messages%s",
        #messages, paused and " [PAUSED]" or "")
    local pad_s = title .. string.rep(' ', math.max(0, w - #title))
    tui.print_at(1, 1, pad_s, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
    -- Status bar
    local status = " ESC=close  C=clear  P=pause/resume  ↑↓=scroll"
    tui.print_at(h, 1, status .. string.rep(' ', math.max(0, w - #status)),
        tui.MAGENTA, tui.BLUE, 0)
    -- Message list (bottom-up, newest at bottom)
    local avail = h - 2  -- rows between title and status bar
    local total = #messages
    local start_idx = math.max(1, total - avail - scroll + 1)
    local end_idx   = math.max(0, total - scroll)
    local row = 2
    for i = start_idx, end_idx do
        if row > h - 1 then break end
        local m = messages[i]
        local ch, kind, detail = format_msg(m)
        local line = string.format(" %s  %s  %s", ch, kind, detail)
        if #line < w then line = line .. string.rep(' ', w - #line) end
        tui.print_at(row, 1, line:sub(1, w), msg_color(m.type), tui.BLUE, 0)
        row = row + 1
    end
    -- Fill remaining rows
    while row <= h - 1 do
        tui.print_at(row, 1, string.rep(' ', w), tui.WHITE, tui.BLUE, 0)
        row = row + 1
    end
    tui.flush()
end

function M.open()
    scroll = 0
    draw()
    while true do
        local key = tui.read_key(10)
        if not key then
            if M.on_idle then M.on_idle() end
            draw()
        elseif key == 'esc' then
            break
        elseif key == 'c' or key == 'C' then
            messages = {}
            scroll = 0
        elseif key == 'p' or key == 'P' then
            paused = not paused
        elseif key == 'up' then
            scroll = math.min(scroll + 1, math.max(0, #messages - 1))
        elseif key == 'down' then
            scroll = math.max(0, scroll - 1)
        elseif key == 'pageup' then
            local _, h = tui.size()
            scroll = math.min(scroll + (h - 2), math.max(0, #messages - 1))
        elseif key == 'pagedown' then
            local _, h = tui.size()
            scroll = math.max(0, scroll - (h - 2))
        end
        if key then draw() end
    end
    tui.clear()
end

return M

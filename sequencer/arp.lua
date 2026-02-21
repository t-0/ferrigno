-- sequencer/arp.lua
-- Arpeggiator engine — one state per track, ticked from the main loop.

local M = {}

M.states = {}  -- [track_idx] -> state table | nil

M.MODES = {"up","down","updown","downup","random","played","outside_in","inside_out"}
M.RATES = {4.0, 2.0, 1.0, 0.5, 0.25, 0.125, 0.0625}
M.RATE_NAMES = {
    [4.0]="1/1", [2.0]="1/2", [1.0]="1/4", [0.5]="1/8",
    [0.25]="1/16", [0.125]="1/32", [0.0625]="1/64",
}

-- ── Sequence builder ──────────────────────────────────────────────────────────

local function build_sequence(st)
    local notes = {}
    for n in pairs(st.held) do table.insert(notes, n) end
    if #notes == 0 then st.sequence = {}; return end
    table.sort(notes)

    -- Expand across octaves.
    local exp = {}
    for oct = 0, st.octaves - 1 do
        for _, n in ipairs(notes) do
            local s = n + oct * 12
            if s <= 127 then exp[#exp+1] = s end
        end
    end

    local m = st.mode
    if m == "up" then
        st.sequence = exp

    elseif m == "down" then
        local r = {}; for i = #exp, 1, -1 do r[#r+1] = exp[i] end
        st.sequence = r

    elseif m == "updown" then
        -- C E G E  (no endpoint repeat)
        local s = {}
        for _, n in ipairs(exp) do s[#s+1] = n end
        for i = #exp - 1, 2, -1 do s[#s+1] = exp[i] end
        st.sequence = s

    elseif m == "downup" then
        local s = {}
        for i = #exp, 1, -1 do s[#s+1] = exp[i] end
        for i = 2, #exp - 1     do s[#s+1] = exp[i] end
        st.sequence = s

    elseif m == "random" then
        -- Fixed random permutation; reshuffled whenever notes change.
        local s = {}; for _, n in ipairs(exp) do s[#s+1] = n end
        for i = #s, 2, -1 do
            local j = math.random(1, i); s[i], s[j] = s[j], s[i]
        end
        st.sequence = s

    elseif m == "played" then
        -- Insertion-order. played_order tracks note-on sequence.
        local s = {}
        for _, n in ipairs(st.played_order or {}) do
            for oct = 0, st.octaves - 1 do
                local sh = n + oct * 12
                if sh <= 127 then s[#s+1] = sh end
            end
        end
        st.sequence = #s > 0 and s or exp

    elseif m == "outside_in" then
        -- lowest, highest, 2nd-lowest, 2nd-highest, …
        local s = {}; local lo, hi = 1, #exp
        while lo <= hi do
            s[#s+1] = exp[lo]; lo = lo + 1
            if lo <= hi then s[#s+1] = exp[hi]; hi = hi - 1 end
        end
        st.sequence = s

    elseif m == "inside_out" then
        -- From centre outward.
        local s = {}
        local mid = math.ceil(#exp / 2)
        local lo, hi = mid, mid + (#exp % 2 == 0 and 1 or 0)
        while lo >= 1 or hi <= #exp do
            if lo >= 1  then s[#s+1] = exp[lo]; lo = lo - 1 end
            if hi <= #exp then s[#s+1] = exp[hi]; hi = hi + 1 end
        end
        st.sequence = s
    else
        st.sequence = exp
    end
end

-- ── Lifecycle ─────────────────────────────────────────────────────────────────

local function new_state(settings, cur_beat)
    local s = settings or {}
    return {
        mode         = s.mode   or "up",
        octaves      = tonumber(s.octaves) or 1,
        rate         = tonumber(s.rate)    or 0.25,
        gate         = tonumber(s.gate)    or 0.8,
        hold         = (s.hold_mode == 1 or s.hold == true),
        -- Runtime (not persisted):
        held         = {},
        played_order = {},
        sequence     = {},
        seq_idx      = 1,
        next_beat    = cur_beat or 0.0,
        active_note  = nil,
        active_off   = nil,
    }
end

function M.enable(track_idx, settings, cur_beat)
    M.states[track_idx] = new_state(settings, cur_beat)
end

function M.disable(track_idx, port_rec)
    local st = M.states[track_idx]
    if not st then return end
    if st.active_note and port_rec then
        local ch = (port_rec.channel - 1) & 0x0F
        pcall(function()
            port_rec.port:send(string.char(0x80 | ch, st.active_note, 0))
        end)
    end
    M.states[track_idx] = nil
end

-- ── MIDI input ────────────────────────────────────────────────────────────────

-- Feed a decoded MIDI event to the arp for `track_idx`.
-- Returns true if consumed (caller should not record it as a raw event).
function M.feed(track_idx, status, data1, data2, cur_beat)
    local st = M.states[track_idx]
    if not st then return false end

    local t = status & 0xF0
    if t == 0x90 and data2 > 0 then
        if not st.hold then
            -- Track play order (deduplicate).
            local found = false
            for _, n in ipairs(st.played_order) do
                if n == data1 then found = true; break end
            end
            if not found then st.played_order[#st.played_order+1] = data1 end
        end
        local was_empty = #st.sequence == 0
        st.held[data1] = data2
        build_sequence(st)
        if was_empty then
            st.next_beat = cur_beat
            st.seq_idx   = 1
        end
        return true

    elseif t == 0x80 or (t == 0x90 and data2 == 0) then
        if not st.hold then
            st.held[data1] = nil
            for i, n in ipairs(st.played_order) do
                if n == data1 then table.remove(st.played_order, i); break end
            end
            build_sequence(st)
        end
        return true
    end
    return false
end

-- ── Output tick ───────────────────────────────────────────────────────────────

-- Call every engine frame while transport is running.
function M.tick(track_idx, cur_beat, port_rec)
    local st = M.states[track_idx]
    if not st or #st.sequence == 0 then return end

    -- Send pending note-off.
    if st.active_note ~= nil and st.active_off ~= nil and cur_beat >= st.active_off then
        local ch = (port_rec.channel - 1) & 0x0F
        pcall(function()
            port_rec.port:send(string.char(0x80 | ch, st.active_note, 0))
        end)
        st.active_note = nil
        st.active_off  = nil
    end

    -- Fire next arp step.
    if cur_beat >= st.next_beat then
        if #st.sequence == 0 then return end
        local idx  = ((st.seq_idx - 1) % #st.sequence) + 1
        local note = st.sequence[idx]
        -- Resolve velocity: find the base note (same pitch class) in held.
        local vel = 100
        for hn, hv in pairs(st.held) do
            if hn % 12 == note % 12 then vel = hv; break end
        end

        local ch = (port_rec.channel - 1) & 0x0F
        -- Terminate previous note early if still sounding.
        if st.active_note ~= nil then
            pcall(function()
                port_rec.port:send(string.char(0x80 | ch, st.active_note, 0))
            end)
            st.active_note = nil
        end
        pcall(function()
            port_rec.port:send(string.char(0x90 | ch, note, vel))
        end)
        st.active_note = note
        st.active_off  = st.next_beat + st.rate * st.gate
        st.next_beat   = st.next_beat + st.rate
        st.seq_idx     = idx + 1
        if st.seq_idx > #st.sequence then st.seq_idx = 1 end
    end
end

-- ── Helpers ───────────────────────────────────────────────────────────────────

function M.set_param(track_idx, key, value)
    local st = M.states[track_idx]
    if not st then return end
    st[key] = value
    if key == "mode" or key == "octaves" then
        build_sequence(st)
        st.seq_idx = 1
    end
end

function M.get_state(track_idx)
    return M.states[track_idx]
end

-- Cycle a parameter forward through a values list. Returns the new value.
function M.cycle(track_idx, key, values)
    local st = M.states[track_idx]
    if not st then return nil end
    local cur = st[key]
    local idx = 1
    for i, v in ipairs(values) do if v == cur then idx = i; break end end
    local next_val = values[(idx % #values) + 1]
    M.set_param(track_idx, key, next_val)
    return next_val
end

return M

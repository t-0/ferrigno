-- sequencer/arp.lua
-- Arpeggiator engine — pure-function library for use as a chain device.

local M = {}

M.MODES = {"up","down","updown","downup","random","played","outside_in","inside_out"}
M.RATES = {4.0, 2.0, 1.0, 0.5, 0.25, 0.125, 0.0625}
M.RATE_NAMES = {
    [4.0]="1/1", [2.0]="1/2", [1.0]="1/4", [0.5]="1/8",
    [0.25]="1/16", [0.125]="1/32", [0.0625]="1/64",
}

-- ── Sequence builder ──────────────────────────────────────────────────────

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
        local s = {}; for _, n in ipairs(exp) do s[#s+1] = n end
        for i = #s, 2, -1 do
            local j = math.random(1, i); s[i], s[j] = s[j], s[i]
        end
        st.sequence = s

    elseif m == "played" then
        local s = {}
        for _, n in ipairs(st.played_order or {}) do
            for oct = 0, st.octaves - 1 do
                local sh = n + oct * 12
                if sh <= 127 then s[#s+1] = sh end
            end
        end
        st.sequence = #s > 0 and s or exp

    elseif m == "outside_in" then
        local s = {}; local lo, hi = 1, #exp
        while lo <= hi do
            s[#s+1] = exp[lo]; lo = lo + 1
            if lo <= hi then s[#s+1] = exp[hi]; hi = hi - 1 end
        end
        st.sequence = s

    elseif m == "inside_out" then
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

-- ── State creation ──────────────────────────────────────────────────────────

function M.create_state(settings)
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
        next_beat    = 0.0,
        active_note  = nil,
        active_off   = nil,
    }
end

-- ── Feed MIDI input ─────────────────────────────────────────────────────────

-- Feed a decoded MIDI event to an arp state.
-- Returns true if consumed (caller should not record it as a raw event).
function M.feed_event(state, status, data1, data2, cur_beat)
    if not state then return false end

    local t = status & 0xF0
    if t == 0x90 and data2 > 0 then
        if not state.hold then
            local found = false
            for _, n in ipairs(state.played_order) do
                if n == data1 then found = true; break end
            end
            if not found then state.played_order[#state.played_order+1] = data1 end
        end
        local was_empty = #state.sequence == 0
        state.held[data1] = data2
        build_sequence(state)
        if was_empty then
            state.next_beat = cur_beat
            state.seq_idx   = 1
        end
        return true

    elseif t == 0x80 or (t == 0x90 and data2 == 0) then
        if not state.hold then
            state.held[data1] = nil
            for i, n in ipairs(state.played_order) do
                if n == data1 then table.remove(state.played_order, i); break end
            end
            build_sequence(state)
        end
        return true
    end
    return false
end

-- ── Output tick ─────────────────────────────────────────────────────────────

-- Generate arpeggiated events. Returns array of event tables.
function M.tick_generate(state, cur_beat)
    if not state then return {} end
    local out = {}

    -- Send pending note-off first.
    if state.active_note ~= nil then
        local gate_done = state.active_off ~= nil and cur_beat >= state.active_off
        if gate_done or #state.sequence == 0 then
            out[#out+1] = { status = 0x80, data1 = state.active_note, data2 = 0 }
            state.active_note = nil
            state.active_off  = nil
        end
    end

    if #state.sequence == 0 then return out end

    -- Fire next arp step.
    if cur_beat >= state.next_beat then
        local idx  = ((state.seq_idx - 1) % #state.sequence) + 1
        local note = state.sequence[idx]
        local vel  = 100
        for hn, hv in pairs(state.held) do
            if hn % 12 == note % 12 then vel = hv; break end
        end

        -- Terminate previous note early if still sounding.
        if state.active_note ~= nil then
            out[#out+1] = { status = 0x80, data1 = state.active_note, data2 = 0 }
            state.active_note = nil
        end

        out[#out+1] = { status = 0x90, data1 = note, data2 = vel }
        state.active_note = note
        state.active_off  = state.next_beat + state.rate * state.gate
        state.next_beat   = state.next_beat + state.rate
        state.seq_idx     = idx + 1
        if state.seq_idx > #state.sequence then state.seq_idx = 1 end
    end

    return out
end

-- ── Cleanup ─────────────────────────────────────────────────────────────────

-- Return pending note-off events and reset state.
function M.cleanup_state(state)
    if not state then return {} end
    local out = {}
    if state.active_note ~= nil then
        out[#out+1] = { status = 0x80, data1 = state.active_note, data2 = 0 }
        state.active_note = nil
        state.active_off  = nil
    end
    state.held         = {}
    state.played_order = {}
    state.sequence     = {}
    state.seq_idx      = 1
    return out
end

-- ── Param helpers ───────────────────────────────────────────────────────────

function M.set_param(state, key, value)
    if not state then return end
    state[key] = value
    if key == "mode" or key == "octaves" then
        build_sequence(state)
        state.seq_idx = 1
    end
end

-- Cycle a parameter forward through a values list. Returns the new value.
function M.cycle_param(state, key, values)
    if not state then return nil end
    local cur = state[key]
    local idx = 1
    for i, v in ipairs(values) do if v == cur then idx = i; break end end
    local next_val = values[(idx % #values) + 1]
    M.set_param(state, key, next_val)
    return next_val
end

return M

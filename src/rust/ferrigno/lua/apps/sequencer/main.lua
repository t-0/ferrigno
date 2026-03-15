-- sequencer/main.lua
-- Entry point: ferrigno sequencer/main.lua [song.db]
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
--   T                Edit current track (name, instrument, chain)
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

local db_mod        = req("db")
local engine        = req("engine")
local ui            = req("ui")
local piano_roll    = req("piano_roll")
local arp           = req("arp")
local chain         = req("chain")
local models = req("models")
local midi_monitor  = req("midi_monitor")

-- Wire arp module into chain system.
chain.set_arp_module(arp)
engine.chain = chain

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

-- ── Auto-migrate tracks to chains ─────────────────────────────────────────────

db_mod.migrate_tracks_to_chains(db, song.id, instruments_by_name)
-- Reload tracks to pick up chain_id.
tracks = db_mod.get_tracks(db, song.id)

-- ── Load chains ───────────────────────────────────────────────────────────────

local track_chains = {}
for ti, track in ipairs(tracks) do
    track_chains[ti] = chain.resolve_chain(db, track.chain_id)
end
engine.track_chains = track_chains

-- ── Load master chain ───────────────────────────────────────────────────────

local master_chain_row = db_mod.get_or_create_master_chain(db, song)
local master_chain_def = chain.resolve_chain(db, master_chain_row.id)

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
    local dm = {}
    -- Populate from model first (if linked).
    local model = inst.instrument_model and models[inst.instrument_model]
    if model and model.drum_map then
        for _, e in ipairs(model.drum_map) do dm[e.note] = e.name end
    end
    -- Custom per-instrument DB entries override model entries.
    local entries = db_mod.get_drum_map(db, inst.id)
    for _, e in ipairs(entries) do dm[e.note] = e.name end
    drum_maps[inst.id] = dm
end
for _, inst in ipairs(instruments) do
    reload_drum_map(inst)
end

-- ── Load CC name maps per instrument ─────────────────────────────────────────
-- cc_name_maps[inst.id] = {
--   global  = {[cc_number] = row_table},   -- note == -1 entries
--   by_note = {[note] = {[cc_number] = row_table}},
-- }
local cc_name_maps = {}
local function reload_cc_names(inst)
    local m = { ["global"] = {}, by_note = {} }
    -- Populate from model first (if linked).
    local model = inst.instrument_model and models[inst.instrument_model]
    if model and model.cc then
        for _, row in ipairs(model.cc) do
            if (row.note or -1) == -1 then
                m["global"][row.cc_number] = row
            else
                if not m.by_note[row.note] then m.by_note[row.note] = {} end
                m.by_note[row.note][row.cc_number] = row
            end
        end
    end
    -- Custom per-instrument DB entries override model entries.
    local rows = db_mod.get_cc_names(db, inst.id)
    for _, row in ipairs(rows) do
        if row.note == -1 then
            m["global"][row.cc_number] = row
        else
            if not m.by_note[row.note] then m.by_note[row.note] = {} end
            m.by_note[row.note][row.cc_number] = row
        end
    end
    cc_name_maps[inst.id] = m
end
for _, inst in ipairs(instruments) do
    reload_cc_names(inst)
end

-- ── Load NRPN name maps per instrument ──────────────────────────────────────
-- nrpn_name_maps[inst.id] = {
--   global  = {[nrpn_number] = row_table},   -- note == -1 entries
--   by_note = {[note] = {[nrpn_number] = row_table}},
-- }
local nrpn_name_maps = {}
local function reload_nrpn_names(inst)
    local m = { ["global"] = {}, by_note = {} }
    -- Populate from model first (if linked).
    local model = inst.instrument_model and models[inst.instrument_model]
    if model and model.nrpn then
        for _, row in ipairs(model.nrpn) do
            if (row.note or -1) == -1 then
                m["global"][row.nrpn_number] = row
            else
                if not m.by_note[row.note] then m.by_note[row.note] = {} end
                m.by_note[row.note][row.nrpn_number] = row
            end
        end
    end
    -- Custom per-instrument DB entries override model entries.
    local rows = db_mod.get_nrpn_names(db, inst.id)
    for _, row in ipairs(rows) do
        if row.note == -1 then
            m["global"][row.nrpn_number] = row
        else
            if not m.by_note[row.note] then m.by_note[row.note] = {} end
            m.by_note[row.note][row.nrpn_number] = row
        end
    end
    nrpn_name_maps[inst.id] = m
end
for _, inst in ipairs(instruments) do
    reload_nrpn_names(inst)
end

-- ── Load SysEx param maps per instrument ─────────────────────────────────────
-- sysex_param_maps[inst.id] = { [param_index] = row_table }
local sysex_param_maps = {}
local function reload_sysex_params(inst)
    local m = {}
    -- If a model is linked, populate from model first.
    local model = inst.instrument_model and models[inst.instrument_model]
    if model and model.sysex then
        for _, p in ipairs(model.sysex) do
            m[p.param_index] = p
        end
    end
    -- Custom per-instrument DB params override preset entries.
    local rows = db_mod.get_sysex_params(db, inst.id)
    for _, row in ipairs(rows) do
        m[row.param_index] = row
    end
    sysex_param_maps[inst.id] = m
end
for _, inst in ipairs(instruments) do
    reload_sysex_params(inst)
end

local function rebuild_sysex_templates()
    chain.sysex_templates = {}
    for inst_id, pmap in pairs(sysex_param_maps) do
        chain.sysex_templates[inst_id] = {}
        for pi, row in pairs(pmap) do
            chain.sysex_templates[inst_id][pi] = {
                template = row.template,
                min_val  = row.min_val,
                max_val  = row.max_val,
            }
        end
    end
end
rebuild_sysex_templates()

-- ── Patch resolution ────────────────────────────────────────────────────────
-- Merge track + clip patch fields; clip overrides track per-field.
-- Pre-expand SysEx overrides via chain.expand_sysex_template.
-- Returns nil if nothing to send.

local function resolve_patch(track, clip, inst)
    local bank_sysex    = (clip and clip.bank_sysex)    or track.bank_sysex
    local bank_msb      = (clip and clip.bank_msb)      or track.bank_msb
    local bank_lsb      = (clip and clip.bank_lsb)      or track.bank_lsb
    local program       = (clip and clip.program)        or track.program
    local program_sysex = (clip and clip.program_sysex)  or track.program_sysex

    -- Merge overrides: start with track's, then clip's replace matching (type, param) pairs.
    local track_ovs = track.overrides or {}
    local clip_ovs  = (clip and clip.overrides) or {}
    if type(track_ovs) == "string" then track_ovs = json.decode(track_ovs) or {} end
    if type(clip_ovs)  == "string" then clip_ovs  = json.decode(clip_ovs)  or {} end

    local merged = {}
    local seen   = {}  -- key = "type:param"
    -- Clip overrides first (they win).
    for _, ov in ipairs(clip_ovs) do
        local key = ov.type .. ":" .. tostring(ov.param or '')
        if not seen[key] then
            seen[key] = true
            merged[#merged+1] = { type=ov.type, param=ov.param, value=ov.value }
        end
    end
    -- Then track overrides (only if not already overridden by clip).
    for _, ov in ipairs(track_ovs) do
        local key = ov.type .. ":" .. tostring(ov.param or '')
        if not seen[key] then
            seen[key] = true
            merged[#merged+1] = { type=ov.type, param=ov.param, value=ov.value }
        end
    end

    -- Sort: sysex first, then nrpn, then cc.
    local type_order = { sysex=1, nrpn=2, cc=3 }
    table.sort(merged, function(a, b)
        local oa = type_order[a.type] or 9
        local ob = type_order[b.type] or 9
        if oa ~= ob then return oa < ob end
        return (a.param or 0) < (b.param or 0)
    end)

    -- Pre-expand SysEx overrides.
    if inst then
        local pmap = sysex_param_maps[inst.id] or {}
        for _, ov in ipairs(merged) do
            if ov.type == 'sysex' then
                local tmpl = pmap[ov.param]
                if tmpl and tmpl.template then
                    ov.hex = chain.expand_sysex_template(tmpl.template, ov.value or 0)
                end
            end
        end
    end

    -- Return nil if everything is empty.
    if not bank_sysex and not bank_msb and not bank_lsb and not program
       and not program_sysex and #merged == 0 then
        return nil
    end

    return {
        bank_sysex    = bank_sysex,
        bank_msb      = bank_msb,
        bank_lsb      = bank_lsb,
        program       = program,
        program_sysex = program_sysex,
        overrides     = #merged > 0 and merged or nil,
    }
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

-- ── Build track_has_arp table ───────────────────────────────────────────────

local track_has_arp = {}
local function rebuild_track_has_arp()
    track_has_arp = {}
    for ti, cd in pairs(track_chains) do
        if cd then
            local dev = chain.find_device(cd, "arp")
            if dev and dev.enabled then
                track_has_arp[ti] = true
            end
        end
    end
end
rebuild_track_has_arp()

-- ── Wire UI state ─────────────────────────────────────────────────────────────

ui.song           = song
ui.tracks         = tracks
ui.clips          = clips
ui.scenes         = scenes
ui.num_slots      = NUM_SLOTS
ui.bpm            = song.bpm
ui.track_has_arp  = track_has_arp
ui.studio_name    = current_studio and current_studio.name or "all"

-- ── TUI setup ─────────────────────────────────────────────────────────────────

tui.init()
tui.enter_alt()
tui.raw()
tui.hide_cursor()
tui.clear()

-- ── Splash screen ─────────────────────────────────────────────────────────────

do
    local VERSION   = "v0.1.0"
    local COPYRIGHT = "©2026 github/t-0"
    local LINES = {
        "",
        "  t-0 Sequencer  ",
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
        tui.print_at(by + row, bx, string.rep(' ', bw), tui.WHITE, tui.BLUE, 0)
    end

    -- Top and bottom border.
    tui.print_at(by,          bx, string.rep('─', bw), tui.BRIGHT_WHITE, tui.BLUE, tui.BOLD)
    tui.print_at(by + bh - 1, bx, string.rep('─', bw), tui.BRIGHT_WHITE, tui.BLUE, tui.BOLD)

    -- Content lines, centred vertically within the box.
    local content_start = by + math.floor((bh - #LINES) / 2)
    for i, line in ipairs(LINES) do
        local row = content_start + i - 1
        if row > by and row < by + bh - 1 then
            local text = line
            local pad_l = math.floor((bw - #text) / 2)
            local pad_r = bw - #text - pad_l
            local display = string.rep(' ', pad_l) .. text .. string.rep(' ', pad_r)
            local is_title = (i == 2)
            tui.print_at(row, bx, display,
                is_title and tui.BRIGHT_YELLOW or tui.BRIGHT_WHITE,
                is_title and tui.RED or tui.BLUE,
                is_title and tui.BOLD or 0)
        end
    end

    tui.flush()
    tui.read_key(2000)
    tui.clear()
end

-- ── Helpers ───────────────────────────────────────────────────────────────────

-- Tick engine while a sub-page is open so MIDI keeps playing.
local function tick_all()
    engine.tick()
    engine.process_input()
    -- Tick chains for tracks without active clips (arp on input-only tracks).
    if engine.playing then
        local ctx = { engine = engine, cur_beat = engine.cur_beat() }
        for ti, cd in pairs(track_chains) do
            if cd and not engine.active_clips[ti] then
                chain.tick_devices(cd, ctx)
            end
        end
    end
end

-- Drop-in replacement for tui.read_key() that keeps the engine ticking.
local function read_key_live()
    while true do
        local key = tui.read_key(10)
        if key then return key end
        tick_all()
    end
end

ui.on_idle         = tick_all
piano_roll.on_idle = tick_all
midi_monitor.on_idle = tick_all

engine.midi_monitor_hook = function(status, data1, data2, raw)
    midi_monitor.push(status, data1, data2, raw)
end

-- ── Help overlay ─────────────────────────────────────────────────────────────

local function pad_h(s, n)
    s = tostring(s or '')
    if #s >= n then return s:sub(1, n) end
    return s .. string.rep(' ', n - #s)
end

local HELP_KEY_W = 24

-- show_help(items) — full-screen help overlay, dismiss with ESC or ?.
local function show_help(items)
    local w, h = tui.size()
    tui.hide_cursor()
    tui.clear()
    local title = "Help"
    for _, item in ipairs(items) do
        if item[1] == "title" then title = item[2]; break end
    end
    tui.print_at(1, 1, pad_h(" " .. title, w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
    tui.print_at(h, 1, pad_h(" ESC or ? = close help", w), tui.MAGENTA, tui.BLUE, 0)
    local row = 3
    for _, item in ipairs(items) do
        if row >= h then break end
        local kind = item[1]
        if kind == "blank" then
            row = row + 1
        elseif kind == "section" then
            tui.print_at(row, 1, pad_h("  " .. item[2], w), tui.CYAN, tui.BLUE, tui.BOLD)
            row = row + 1
        elseif kind == "entry" then
            tui.print_at(row, 1, pad_h("    " .. item[2], HELP_KEY_W),
                tui.BRIGHT_WHITE, tui.BLUE, 0)
            tui.move(row, HELP_KEY_W + 1)
            tui.print(tui.color(tui.WHITE, tui.BLUE, 0))
            tui.print(pad_h(item[3], w - HELP_KEY_W))
            tui.print(tui.reset())
            row = row + 1
        end
    end
    while true do
        local key = tui.read_key(10)
        if not key then tick_all()
        elseif key == 'esc' or key == '?' then break
        end
    end
    tui.clear()
end

local HELP_MAIN = {
    { "title",   "Session View" },
    { "section", "Navigation" },
    { "entry",   "↑↓←→",                  "Navigate the clip grid" },
    { "blank" },
    { "section", "Transport" },
    { "entry",   "Space",                  "Play / stop" },
    { "entry",   "R",                      "Start / stop recording on current track" },
    { "blank" },
    { "section", "Clips" },
    { "entry",   "Enter",                  "Trigger (launch) clip; or launch scene row" },
    { "entry",   "N",                      "New empty clip in current slot" },
    { "entry",   "D",                      "Delete clip in current slot" },
    { "entry",   "E",                      "Open piano roll editor" },
    { "entry",   "C",                      "Clip settings (name, length, loop, patch)" },
    { "blank" },
    { "section", "Scenes" },
    { "entry",   "L",                      "Launch all clips in current scene row" },
    { "entry",   "[",                      "Insert scene row at cursor position" },
    { "entry",   "]",                      "Delete current scene row" },
    { "blank" },
    { "section", "Tracks" },
    { "entry",   "+  /  =",               "Add a new track" },
    { "entry",   "-",                      "Remove the current track (if empty)" },
    { "entry",   "T",                      "Edit track name, instrument, and chain" },
    { "blank" },
    { "section", "Arpeggiator" },
    { "entry",   "P",                      "Toggle arpeggiator on / off" },
    { "entry",   "A",                      "Cycle arp mode  (when arp is on)" },
    { "entry",   "O",                      "Cycle arp rate  (when arp is on)" },
    { "blank" },
    { "section", "Pages" },
    { "entry",   "I",                      "Instruments page" },
    { "entry",   "W",                      "Studios page (MIDI routing profiles)" },
    { "entry",   "B",                      "Set BPM" },
    { "entry",   "F",                      "Rename song" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                      "Save to database" },
    { "entry",   "ESC",                    "Quit (auto-saves)" },
    { "entry",   "F12",                    "MIDI monitor (live input view)" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_INSTRUMENTS = {
    { "title",   "Instruments" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select instrument" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "Enter  /  E",           "Edit selected instrument" },
    { "entry",   "A",                      "Add new instrument" },
    { "entry",   "D",                      "Delete selected instrument" },
    { "entry",   "X",                      "Edit SysEx dumps" },
    { "entry",   "N",                      "Edit CC names" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "ESC",                    "Close instruments page" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_INST_DEF = {
    { "title",   "Instrument Editor" },
    { "section", "Navigation" },
    { "entry",   "↑↓  /  Tab",            "Move between fields" },
    { "entry",   "←→",                    "Cycle picker options" },
    { "entry",   "Enter",                  "Edit text / number field" },
    { "blank" },
    { "section", "While editing a field" },
    { "entry",   "Enter",                  "Confirm value" },
    { "entry",   "ESC",                    "Cancel edit" },
    { "blank" },
    { "section", "Sub-pages" },
    { "entry",   "X",                      "SysEx dumps" },
    { "entry",   "N",                      "CC names" },
    { "entry",   "M",                      "Drum map  (drum instruments only)" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                      "Save instrument" },
    { "entry",   "ESC",                    "Cancel changes" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_SYSEX = {
    { "title",   "SysEx Dumps" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select dump" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "A",                      "Add new SysEx dump" },
    { "entry",   "D",                      "Delete selected dump" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "ESC",                    "Done" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_DRUM_MAP = {
    { "title",   "Drum Map" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select entry" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "A",                      "Add entry  (note → name)" },
    { "entry",   "E  /  Enter",           "Rename selected entry" },
    { "entry",   "D",                      "Delete selected entry" },
    { "blank" },
    { "section", "Note format" },
    { "entry",   "0–127",                  "MIDI note number" },
    { "entry",   "C2, D#3 …",             "Note name with octave" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "ESC",                    "Done" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_CC_NAMES = {
    { "title",   "CC Names" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select entry" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "A",                      "Add CC name  (CC number → name)" },
    { "entry",   "E  /  Enter",           "Rename selected entry" },
    { "entry",   "D",                      "Delete selected entry" },
    { "blank" },
    { "section", "CC numbers" },
    { "entry",   "0–127",                  "Standard MIDI control change" },
    { "entry",   "128",                    "Pitch Bend" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "ESC",                    "Done" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_CLIP_SETTINGS = {
    { "title",   "Clip Settings" },
    { "section", "Navigation" },
    { "entry",   "↑↓  /  Tab",            "Move between fields" },
    { "entry",   "←→",                    "Cycle toggle field" },
    { "entry",   "Enter",                  "Edit text / number field" },
    { "blank" },
    { "section", "Fields" },
    { "entry",   "Name",                   "Clip display name" },
    { "entry",   "Length",                 "Clip length in beats" },
    { "entry",   "Loop",               "Whether clip loops" },
    { "entry",   "Start",                  "Playback start offset (beats)" },
    { "entry",   "Loop Start",             "Loop region start (beats)" },
    { "entry",   "Loop Length",            "Loop region length  (blank=full)" },
    { "entry",   "Bank MSB / LSB",         "MIDI bank select override" },
    { "entry",   "Program",                "MIDI program change override" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                      "Save clip settings" },
    { "entry",   "ESC",                    "Cancel changes" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_TRACK_EDIT = {
    { "title",   "Track Editor" },
    { "section", "Navigation" },
    { "entry",   "↑↓  /  Tab",            "Move between fields" },
    { "entry",   "←→",                    "Cycle instrument selection" },
    { "entry",   "Enter",                  "Edit track name field" },
    { "blank" },
    { "section", "Fields" },
    { "entry",   "Track Name",             "Display name for the track" },
    { "entry",   "Instrument",             "Instrument assignment" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "C",                      "Open device chain editor" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                      "Save track" },
    { "entry",   "ESC",                    "Cancel changes" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_STUDIOS = {
    { "title",   "Studios" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select studio" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "Enter",                  "Use selected studio for this song" },
    { "entry",   "E",                      "Edit studio routing" },
    { "entry",   "A",                      "Add new studio" },
    { "entry",   "D",                      "Delete selected studio" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "ESC",                    "Close studios page" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_STUDIO_EDIT = {
    { "title",   "Studio Editor" },
    { "section", "Navigation" },
    { "entry",   "↑↓  /  Tab",            "Move between fields" },
    { "entry",   "←→",                    "Cycle live toggle" },
    { "entry",   "Enter",                  "Edit port / channel field" },
    { "blank" },
    { "section", "Fields (per instrument)" },
    { "entry",   "Live",                   "Whether instrument is active in this studio" },
    { "entry",   "Port",                   "Override MIDI output port  (blank=default)" },
    { "entry",   "Chan",                   "Override MIDI channel  (0=instrument default)" },
    { "blank" },
    { "section", "Save / Exit" },
    { "entry",   "S",                      "Save studio" },
    { "entry",   "ESC",                    "Cancel changes" },
    { "entry",   "?",                      "Show this help" },
}

local HELP_CHAIN_EDIT = {
    { "title",   "Chain Editor" },
    { "section", "Navigation" },
    { "entry",   "↑↓",                    "Select device" },
    { "blank" },
    { "section", "Actions" },
    { "entry",   "A",                      "Add device (type picker)" },
    { "entry",   "D",                      "Delete selected device" },
    { "entry",   "E  /  Enter",           "Edit device params" },
    { "entry",   "U",                      "Move device up" },
    { "entry",   "J",                      "Move device down" },
    { "entry",   "X",                      "Toggle device enabled/disabled" },
    { "blank" },
    { "section", "Exit" },
    { "entry",   "S",                      "Save and close" },
    { "entry",   "ESC",                    "Close chain editor" },
    { "entry",   "?",                      "Show this help" },
}

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

-- Reload a track's chain_def after DB changes.
local function reload_chain(ti)
    local track = tracks[ti]
    if not track or not track.chain_id then return end
    track_chains[ti] = chain.resolve_chain(db, track.chain_id)
    engine.track_chains = track_chains
    rebuild_track_has_arp()
    ui.track_has_arp = track_has_arp
end

-- Reload whichever chain the UI is currently showing (master or track).
local function reload_current_chain()
    if ui.cursor.track == 0 then
        master_chain_def = chain.resolve_chain(db, master_chain_row.id)
    else
        reload_chain(ui.cursor.track)
    end
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
        engine.stop_track(ui.cursor.track)
        ui.set_status("Track stopped")
        return
    end
    local evts = events[clip.id] or {}
    if inst then
        local patch = resolve_patch(track, clip, inst)
        if patch then engine.send_patch(inst.id, patch) end
    end
    engine.launch(ui.cursor.track, track_chains[ui.cursor.track], clip, evts)
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

    -- Create a default chain with a single instrument device.
    local inst = instruments_by_name[default_inst_name]
    local chain_row = db_mod.upsert_device_chain(db,
        { track_id = track.id, name = track.name }, song.id)
    db_mod.upsert_device(db, {
        chain_id      = chain_row.id,
        device_type   = "instrument",
        position      = 0,
        instrument_id = inst and inst.id or nil,
        name          = default_inst_name,
    })
    track.chain_id = chain_row.id
    db_mod.upsert_track(db, track, song.id)

    local ti = #tracks
    track_chains[ti] = chain.resolve_chain(db, track.chain_id)
    engine.track_chains = track_chains
    rebuild_track_has_arp()
    ui.track_has_arp = track_has_arp

    ui.cursor.track = ti
    ui.set_status("Added track: " .. track.name)
end

local function remove_track()
    local ti = ui.cursor.track
    if ti < 1 then
        ui.set_status("No track selected", tui.YELLOW)
        return
    end
    if #tracks <= 1 then
        ui.set_status("Cannot remove the last track", tui.YELLOW)
        return
    end
    local track = tracks[ti]
    local has_clip = false
    for _ in pairs(clips[track.id] or {}) do has_clip = true; break end
    if has_clip then
        ui.set_status("Track has clips — delete clips first", tui.YELLOW)
        return
    end
    engine.stop_track(ti)
    -- Delete chain.
    if track.chain_id then
        db_mod.delete_device_chain(db, track.chain_id)
    end
    db_mod.delete_track(db, track.id)
    clips[track.id] = nil
    table.remove(tracks, ti)
    -- Shift chain references down.
    for i = ti, #tracks do
        track_chains[i] = track_chains[i + 1]
    end
    track_chains[#tracks + 1] = nil
    engine.track_chains = track_chains
    rebuild_track_has_arp()
    ui.track_has_arp = track_has_arp
    ui.cursor.track = math.min(ti, #tracks)
    ui.set_status("Track removed: " .. track.name)
end

-- Standard MIDI CC numbers and their conventional names.
local STANDARD_CC_NAMES = {
    [0]  = "Bank MSB",     [1]  = "Mod Wheel",    [2]  = "Breath",
    [4]  = "Foot Ctrl",    [5]  = "Portamento T",  [6]  = "Data Entry",
    [7]  = "Volume",       [8]  = "Balance",        [10] = "Pan",
    [11] = "Expression",   [12] = "Effect 1",       [13] = "Effect 2",
    [32] = "Bank LSB",     [64] = "Sustain",        [65] = "Portamento",
    [66] = "Sostenuto",    [67] = "Soft Pedal",     [68] = "Legato",
    [71] = "Resonance",    [72] = "Release",        [73] = "Attack",
    [74] = "Cutoff",       [91] = "Reverb",         [93] = "Chorus",
    [128] = "Pitch Bend",
}

-- Read SysEx data from hex input or a .syx file path.
-- Returns a hex string (e.g. "F0 41 10 42 F7") or nil on cancel/error.
local function read_sysex_data(row, prompt, default)
    local val = ui.read_line(row, prompt or "Hex or .syx path: ", default or '')
    if not val or val == '' then return val end
    -- If it looks like a file path, try to read it.
    if val:match("%.[Ss][Yy][Xx]$") or val:match("^/") or val:match("^%./") or val:match("^~/") then
        -- Expand ~ to HOME.
        local path = val:gsub("^~", os.getenv("HOME") or "~")
        local f = io.open(path, "rb")
        if not f then
            ui.set_status("Cannot open file: " .. val, tui.RED)
            return nil
        end
        local data = f:read("*a")
        f:close()
        if not data or #data == 0 then
            ui.set_status("File is empty: " .. val, tui.YELLOW)
            return nil
        end
        -- Convert binary to hex string.
        local hex = {}
        for i = 1, #data do
            hex[#hex+1] = string.format("%02X", data:byte(i))
        end
        return table.concat(hex, " ")
    end
    -- Otherwise treat as raw hex input.
    return val:upper()
end

-- ── Overrides sub-editor (shared by track and clip editors) ─────────────────
-- overrides_list = array of {type="cc"|"nrpn"|"sysex", param=N, value=N}
-- inst = instrument table (for name lookups)
-- Returns the (possibly modified) overrides list.

local function edit_overrides(overrides_list, inst)
    local ovs = {}
    for _, ov in ipairs(overrides_list or {}) do
        ovs[#ovs+1] = { type=ov.type, param=ov.param, value=ov.value }
    end
    local sel = math.max(1, #ovs)

    local function ov_name(ov)
        if not inst then return '' end
        if ov.type == 'cc' then
            local m = cc_name_maps[inst.id]
            if m and m["global"][ov.param] then return m["global"][ov.param].name or '' end
            return STANDARD_CC_NAMES[ov.param] or ''
        elseif ov.type == 'nrpn' then
            local m = nrpn_name_maps[inst.id]
            if m and m["global"][ov.param] then return m["global"][ov.param].name or '' end
            return ''
        elseif ov.type == 'sysex' then
            local m = sysex_param_maps[inst.id]
            if m and m[ov.param] then return m[ov.param].name or '' end
            return ''
        end
        return ''
    end

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Overrides" .. (inst and (" — " .. inst.name) or ""), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "A=add  E=edit value  D=delete  ↑↓=navigate  ESC=done",
            tui.MAGENTA, tui.BLUE, 0)
        if #ovs == 0 then
            tui.print_at(5, 2, "(no overrides — press A to add)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, ov in ipairs(ovs) do
                local name = ov_name(ov)
                local label = string.format(" %-5s  #%-5d  val:%-5d  %s",
                    ov.type, ov.param or 0, ov.value or 0, name)
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
                end
            end
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto overrides_again end

        if key == 'esc' then
            break
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #ovs), sel + 1)
        elseif key == 'a' or key == 'A' then
            tui.clear()
            local type_s = ui.read_line(3, "Type (cc/nrpn/sysex): ", 'cc')
            if type_s then
                type_s = type_s:lower()
                if type_s == 'cc' or type_s == 'nrpn' or type_s == 'sysex' then
                    tui.clear()
                    local param_s = ui.read_line(3, "Param number: ", '')
                    if param_s then
                        local param = tonumber(param_s)
                        if param then
                            param = math.floor(param)
                            tui.clear()
                            local val_s = ui.read_line(3, "Value: ", '0')
                            if val_s then
                                local val = tonumber(val_s)
                                if val then
                                    ovs[#ovs+1] = { type=type_s, param=param, value=math.floor(val) }
                                    sel = #ovs
                                end
                            end
                        end
                    end
                else
                    ui.set_status("Invalid type — use cc, nrpn, or sysex", tui.YELLOW)
                end
            end
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #ovs > 0 then
            tui.clear()
            local ov = ovs[sel]
            local val_s = ui.read_line(3, "Value: ", tostring(ov.value or 0))
            if val_s then
                local val = tonumber(val_s)
                if val then ov.value = math.floor(val) end
            end
        elseif (key == 'd' or key == 'D') and #ovs > 0 then
            table.remove(ovs, sel)
            sel = math.max(1, math.min(sel, #ovs))
        end
        draw()
        ::overrides_again::
    end
    tui.clear()
    return ovs
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
        { label="Name",         type="text"                                        },
        { label="Length",       type="float", min=0.01, nullable=false             },
        { label="Loop",         type="toggle", opts=loop_opts                      },
        { label="Start",        type="float", min=0,    nullable=false, hint="beats" },
        { label="Loop Start",   type="float", min=0,    nullable=false, hint="beats" },
        { label="Loop Length",  type="float", min=0.01, nullable=true,  hint="blank=full" },
        { label="Bank SysEx",   type="sysex", hint="Enter=load"                   },
        { label="Bank MSB",     type="int",   min=0,    max=127, nullable=true     },
        { label="Bank LSB",     type="int",   min=0,    max=127, nullable=true     },
        { label="Program",      type="int",   min=0,    max=127, nullable=true     },
        { label="Program SysEx",type="sysex", hint="Enter=load"                   },
    }

    local is_looping = (clip.is_looping ~= 0 and clip.is_looping ~= false)
    local clip_ovs = clip.overrides or {}
    if type(clip_ovs) == "string" then clip_ovs = json.decode(clip_ovs) or {} end
    local vals = {
        clip.name or '',
        fmt_f(clip.length_beats or 4.0),
        is_looping and 2 or 1,
        fmt_f(clip.start_offset or 0),
        fmt_f(clip.loop_start   or 0),
        clip.loop_length ~= nil and fmt_f(clip.loop_length) or '',
        clip.bank_sysex or '',
        clip.bank_msb ~= nil and tostring(clip.bank_msb) or '',
        clip.bank_lsb ~= nil and tostring(clip.bank_lsb) or '',
        clip.program  ~= nil and tostring(clip.program)  or '',
        clip.program_sysex or '',
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

        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Clip Settings", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.CYAN, tui.BLUE, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLUE, 0)
            end

            if f.type == "toggle" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                end
            elseif f.type == "sysex" then
                local v = vals[i] or ''
                local preview = v ~= '' and (v:sub(1, val_w) .. (#v > val_w and "..." or "")) or "(none)"
                local hint = f.hint and ("  " .. f.hint) or ""
                local display = pad(preview .. hint, val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
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
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.MAGENTA, tui.BLUE, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  O=overrides  S=save  ESC=cancel  ?=help", w),
                tui.MAGENTA, tui.BLUE, 0)
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
            elseif key == 'o' or key == 'O' then
                local inst = current_inst()
                clip_ovs = edit_overrides(clip_ovs, inst)
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
                if f.type == "sysex" then
                    tui.clear()
                    local result = read_sysex_data(3, "Hex or .syx path (blank=clear): ", vals[sel])
                    if result ~= nil then vals[sel] = result end
                elseif f.type ~= "toggle" then
                    editing  = true
                    edit_buf = vals[sel]
                end
            elseif key == '?' then
                show_help(HELP_CLIP_SETTINGS)
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
    clip.bank_sysex    = vals[7] ~= '' and vals[7] or nil
    clip.bank_msb      = parse_opt_int(vals[8], 0, 127)
    clip.bank_lsb      = parse_opt_int(vals[9], 0, 127)
    clip.program       = parse_opt_int(vals[10], 0, 127)
    clip.program_sysex = vals[11] ~= '' and vals[11] or nil
    clip.overrides     = #clip_ovs > 0 and json.encode(clip_ovs) or nil

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
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "SysEx Dumps — " .. inst.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  D=delete  ESC=done  ?=help",
            tui.MAGENTA, tui.BLUE, 0)
        if #dumps == 0 then
            tui.print_at(5, 2, "(no sysex dumps)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, d in ipairs(dumps) do
                local label = string.format(" %-18s  %s", d.name or '', d.data or '')
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
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
        elseif key == '?' then
            show_help(HELP_SYSEX)
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
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Drum Map — " .. inst.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  E/Enter=rename  D=delete  ESC=done  ?=help",
            tui.MAGENTA, tui.BLUE, 0)
        if #entries == 0 then
            tui.print_at(5, 2, "(no entries — press A to add)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, e in ipairs(entries) do
                local nn = piano_roll.NOTE_NAMES[(e.note % 12) + 1]
                local note_s = string.format("%-3d  %-3s%-2d",
                    e.note, nn, math.floor(e.note / 12) - 1)
                local label = string.format(" %s  %s", note_s, e.name)
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
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
        elseif key == '?' then
            show_help(HELP_DRUM_MAP)
        end
        draw()
    end
    tui.clear()
end

-- CC name editor for an instrument.
local function edit_cc_names(inst)
    local is_drum = (inst.type == 'drum')

    local function load_entries()
        local rows = db_mod.get_cc_names(db, inst.id)
        table.sort(rows, function(a, b)
            if a.cc_number ~= b.cc_number then return a.cc_number < b.cc_number end
            return a.note < b.note
        end)
        return rows
    end

    local function cc_label(cc)
        if cc == 128 then return " PB" end
        return string.format("%3d", cc)
    end

    local function note_label(note)
        if note == -1 then return "[global  ]" end
        local dm   = drum_maps[inst.id]
        local name = dm and dm[note]
        if name then
            return string.format("[%3d %-4s]", note, name:sub(1, 4))
        end
        return string.format("[%3d     ]", note)
    end

    local entries = load_entries()
    local sel     = math.max(1, #entries)

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "CC Names — " .. inst.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  E/Enter=rename  D=delete  ESC=done  ?=help",
            tui.MAGENTA, tui.BLUE, 0)
        if #entries == 0 then
            tui.print_at(5, 2, "(no CC names — press A to add)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, e in ipairs(entries) do
                local note_part = is_drum and ("  " .. note_label(e.note)) or ""
                local label = string.format(" CC%s%s  %s",
                    cc_label(e.cc_number), note_part, e.name or '')
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
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
            local cc_s = ui.read_line(3, "CC number (0-127, 128=pitch bend): ", '')
            if cc_s then
                local cc = tonumber(cc_s)
                if cc and cc >= 0 and cc <= 128 then
                    cc = math.floor(cc)
                    local note  = -1
                    local valid = true
                    if is_drum then
                        tui.clear()
                        local note_s = ui.read_line(3, "Note (0-127, or blank=global): ", '')
                        if note_s == nil then
                            valid = false
                        elseif note_s ~= '' then
                            local n = tonumber(note_s)
                            if n then note = math.floor(math.max(0, math.min(127, n))) end
                        end
                    end
                    if valid then
                        tui.clear()
                        local default_name = STANDARD_CC_NAMES[cc] or ''
                        local name_s = ui.read_line(3, "Name: ", default_name)
                        if name_s and name_s ~= '' then
                            local entry = {
                                instrument_id = inst.id,
                                cc_number     = cc,
                                note          = note,
                                name          = name_s,
                            }
                            db_mod.upsert_cc_name(db, entry)
                            entries = load_entries()
                            for i, e in ipairs(entries) do
                                if e.cc_number == cc and e.note == note then
                                    sel = i; break
                                end
                            end
                            reload_cc_names(inst)
                        end
                    end
                else
                    ui.set_status("Invalid CC number — use 0-128", tui.YELLOW)
                end
            end
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #entries > 0 then
            tui.clear()
            local e      = entries[sel]
            local name_s = ui.read_line(3, "Name: ", e.name)
            if name_s and name_s ~= '' then
                e.name = name_s
                db_mod.upsert_cc_name(db, e)
                reload_cc_names(inst)
            end
        elseif (key == 'd' or key == 'D') and #entries > 0 then
            local e = entries[sel]
            if e.id then db_mod.delete_cc_name(db, e.id) end
            table.remove(entries, sel)
            sel = math.max(1, math.min(sel, #entries))
            reload_cc_names(inst)
        elseif key == '?' then
            show_help(HELP_CC_NAMES)
        end
        draw()
    end
    tui.clear()
end

-- NRPN name editor for an instrument.
local function edit_nrpn_names(inst)
    local is_drum = (inst.type == 'drum')

    local function load_entries()
        local rows = db_mod.get_nrpn_names(db, inst.id)
        table.sort(rows, function(a, b)
            if a.nrpn_number ~= b.nrpn_number then return a.nrpn_number < b.nrpn_number end
            return a.note < b.note
        end)
        return rows
    end

    local function note_label(note)
        if note == -1 then return "[global  ]" end
        local dm   = drum_maps[inst.id]
        local name = dm and dm[note]
        if name then
            return string.format("[%3d %-4s]", note, name:sub(1, 4))
        end
        return string.format("[%3d     ]", note)
    end

    local entries = load_entries()
    local sel     = math.max(1, #entries)

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "NRPN Names — " .. inst.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  E/Enter=rename  D=delete  ESC=done",
            tui.MAGENTA, tui.BLUE, 0)
        if #entries == 0 then
            tui.print_at(5, 2, "(no NRPN names — press A to add)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, e in ipairs(entries) do
                local note_part = is_drum and ("  " .. note_label(e.note)) or ""
                local label = string.format(" NRPN %5d%s  %s",
                    e.nrpn_number, note_part, e.name or '')
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
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
            local nrpn_s = ui.read_line(3, "NRPN number (0-16383): ", '')
            if nrpn_s then
                local nrpn = tonumber(nrpn_s)
                if nrpn and nrpn >= 0 and nrpn <= 16383 then
                    nrpn = math.floor(nrpn)
                    local note  = -1
                    local valid = true
                    if is_drum then
                        tui.clear()
                        local note_s = ui.read_line(3, "Note (0-127, or blank=global): ", '')
                        if note_s == nil then
                            valid = false
                        elseif note_s ~= '' then
                            local n = tonumber(note_s)
                            if n then note = math.floor(math.max(0, math.min(127, n))) end
                        end
                    end
                    if valid then
                        tui.clear()
                        local name_s = ui.read_line(3, "Name: ", '')
                        if name_s and name_s ~= '' then
                            local entry = {
                                instrument_id = inst.id,
                                nrpn_number   = nrpn,
                                note          = note,
                                name          = name_s,
                            }
                            db_mod.upsert_nrpn_name(db, entry)
                            entries = load_entries()
                            for i, e in ipairs(entries) do
                                if e.nrpn_number == nrpn and e.note == note then
                                    sel = i; break
                                end
                            end
                            reload_nrpn_names(inst)
                        end
                    end
                else
                    ui.set_status("Invalid NRPN number — use 0-16383", tui.YELLOW)
                end
            end
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #entries > 0 then
            tui.clear()
            local e      = entries[sel]
            local name_s = ui.read_line(3, "Name: ", e.name)
            if name_s and name_s ~= '' then
                e.name = name_s
                db_mod.upsert_nrpn_name(db, e)
                reload_nrpn_names(inst)
            end
        elseif (key == 'd' or key == 'D') and #entries > 0 then
            local e = entries[sel]
            if e.id then db_mod.delete_nrpn_name(db, e.id) end
            table.remove(entries, sel)
            sel = math.max(1, math.min(sel, #entries))
            reload_nrpn_names(inst)
        end
        draw()
    end
    tui.clear()
end

-- SysEx parameter editor for an instrument.
local function edit_sysex_params(inst)
    -- Build merged list: preset params (if linked) + custom DB params.
    -- Entries from the preset are shown but not editable inline.
    local function load_entries()
        local merged = {}
        local preset_indices = {}
        local model = inst.instrument_model and models[inst.instrument_model]
        if model and model.sysex then
            for _, p in ipairs(model.sysex) do
                merged[#merged+1] = { param_index=p.param_index, name=p.name,
                    template=p.template, min_val=p.min_val, max_val=p.max_val,
                    default_val=p.default_val, from_preset=true }
                preset_indices[p.param_index] = #merged
            end
        end
        local rows = db_mod.get_sysex_params(db, inst.id)
        for _, row in ipairs(rows) do
            if preset_indices[row.param_index] then
                -- Custom DB row overrides preset entry.
                merged[preset_indices[row.param_index]] = row
                merged[preset_indices[row.param_index]].from_preset = false
            else
                row.from_preset = false
                merged[#merged+1] = row
            end
        end
        table.sort(merged, function(a, b) return a.param_index < b.param_index end)
        return merged
    end

    local entries = load_entries()
    local sel     = math.max(1, #entries)

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        local title = "SysEx Params — " .. inst.name
        if inst.instrument_model then title = title .. " [" .. inst.instrument_model .. "]" end
        tui.print_at(1, 2, title, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  E/Enter=edit  D=delete  P=link/unlink model  ESC=done",
            tui.MAGENTA, tui.BLUE, 0)
        if #entries == 0 then
            tui.print_at(5, 2, "(no sysex params — press A to add or P to link model)",
                tui.MAGENTA, tui.BLUE, 0)
        else
            for i, e in ipairs(entries) do
                local tag = e.from_preset and "·" or " "
                local label = string.format("%sSX %3d  %-20s  [%d..%d] def:%d",
                    tag, e.param_index, e.name or '', e.min_val or 0, e.max_val or 127, e.default_val or 0)
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    local fg = e.from_preset and tui.BRIGHT_BLACK or tui.WHITE
                    tui.print_at(4 + i, 2, " " .. label, fg, tui.BLUE, 0)
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
            local idx_s = ui.read_line(3, "Param index: ", '')
            if idx_s then
                local idx = tonumber(idx_s)
                if idx and idx >= 0 then
                    idx = math.floor(idx)
                    tui.clear()
                    local name_s = ui.read_line(3, "Name: ", '')
                    if name_s and name_s ~= '' then
                        tui.clear()
                        local tmpl_s = ui.read_line(3, "Template (hex): ", '')
                        if tmpl_s and tmpl_s ~= '' then
                            tui.clear()
                            local max_s = ui.read_line(3, "Max value (default 127): ", '127')
                            local max_v = tonumber(max_s) or 127
                            local entry = {
                                instrument_id = inst.id,
                                param_index   = idx,
                                name          = name_s,
                                template      = tmpl_s,
                                min_val       = 0,
                                max_val       = math.floor(max_v),
                                default_val   = 0,
                            }
                            db_mod.upsert_sysex_param(db, entry)
                            entries = load_entries()
                            for i, e in ipairs(entries) do
                                if e.param_index == idx then sel = i; break end
                            end
                            reload_sysex_params(inst)
                            rebuild_sysex_templates()
                        end
                    end
                end
            end
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #entries > 0 then
            tui.clear()
            local e = entries[sel]
            local name_s = ui.read_line(3, "Name: ", e.name)
            if name_s and name_s ~= '' then
                e.name = name_s
                tui.clear()
                local tmpl_s = ui.read_line(3, "Template: ", e.template)
                if tmpl_s and tmpl_s ~= '' then
                    e.template = tmpl_s
                end
                tui.clear()
                local max_s = ui.read_line(3, "Max value: ", tostring(e.max_val))
                if max_s then
                    local mv = tonumber(max_s)
                    if mv then e.max_val = math.floor(mv) end
                end
                db_mod.upsert_sysex_param(db, e)
                reload_sysex_params(inst)
                rebuild_sysex_templates()
            end
        elseif (key == 'd' or key == 'D') and #entries > 0 then
            local e = entries[sel]
            if e.id then db_mod.delete_sysex_param(db, e.id) end
            table.remove(entries, sel)
            sel = math.max(1, math.min(sel, #entries))
            reload_sysex_params(inst)
            rebuild_sysex_templates()
        elseif key == 'p' or key == 'P' then
            -- Link / unlink instrument model
            tui.clear()
            local model_names = models.list()
            if #model_names == 0 then
                ui.set_status("No presets available", tui.YELLOW)
            else
                tui.print_at(1, 1, " Link Instrument Model", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
                local row_off = 2
                if inst.instrument_model then
                    tui.print_at(row_off, 2, string.format("0. (unlink %s)", inst.instrument_model),
                        tui.YELLOW, tui.BLUE, 0)
                    row_off = row_off + 1
                end
                for i, name in ipairs(model_names) do
                    local cur = (name == inst.instrument_model) and " *" or ""
                    tui.print_at(row_off + i, 2, string.format("%d. %s%s", i, name, cur),
                        tui.WHITE, tui.BLUE, 0)
                end
                local pick_s = ui.read_line(row_off + #model_names + 2, "Model number (0=unlink): ", '')
                tui.clear()
                if pick_s then
                    local pick = tonumber(pick_s)
                    if pick == 0 and inst.instrument_model then
                        inst.instrument_model = nil
                        db_mod.upsert_instrument(db, inst, song.id)
                        entries = load_entries()
                        sel = math.max(1, math.min(sel, #entries))
                        reload_drum_map(inst)
                        reload_cc_names(inst)
                        reload_nrpn_names(inst)
                        reload_sysex_params(inst)
                        rebuild_sysex_templates()
                        ui.set_status("Model unlinked", tui.YELLOW)
                    elseif pick and pick >= 1 and pick <= #model_names then
                        inst.instrument_model = model_names[math.floor(pick)]
                        db_mod.upsert_instrument(db, inst, song.id)
                        entries = load_entries()
                        sel = math.max(1, #entries)
                        reload_drum_map(inst)
                        reload_cc_names(inst)
                        reload_nrpn_names(inst)
                        reload_sysex_params(inst)
                        rebuild_sysex_templates()
                        ui.set_status("Linked model: " .. inst.instrument_model, tui.GREEN)
                    end
                end
            end
        end
        draw()
    end
    tui.clear()
end

-- Edit an instrument's definition (MIDI settings, name, patch).
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

    local model_opts = {"None"}
    for _, pn in ipairs(models.list()) do model_opts[#model_opts+1] = pn end

    local fields = {
        { label="Name",        type="text"                                     },
        { label="Type",        type="picker", opts=type_opts                   },
        { label="MIDI Output", type="picker", opts=dest_opts                   },
        { label="MIDI Input",  type="picker", opts=src_opts                    },
        { label="Channel",     type="number", min=1,   max=16,  nullable=false },
        { label="Model",        type="picker", opts=model_opts                },
    }

    local vals = {
        inst.name or '',
        type_index(inst.type),
        opt_index(dest_opts, inst.midi_output or ''),
        opt_index(src_opts,  inst.midi_input  or ''),
        tostring(inst.midi_channel or 1),
        opt_index(model_opts, inst.instrument_model or ''),
    }

    local LABEL_W    = 14
    local VAL_COL    = LABEL_W + 5
    local sel        = 1
    local editing    = false
    local edit_buf   = ''
    local do_sysex       = false
    local do_drummap     = false
    local do_ccnames     = false
    local do_nrpnnames   = false
    local do_sysexparams = false

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

        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Instrument: " .. inst.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.CYAN, tui.BLUE, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLUE, 0)
            end

            if f.type == "picker" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
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
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.MAGENTA, tui.BLUE, 0)
        else
            local drum_hint = (type_opts[vals[2]] == 'drum') and "  M=drum map" or ""
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle options  Enter=edit  S=save  X=sysex  P=sysex params  N=cc names  R=nrpn names" .. drum_hint .. "  ESC=cancel  ?=help", w),
                tui.MAGENTA, tui.BLUE, 0)
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
            elseif key == 'n' or key == 'N' then
                do_ccnames = true
                break
            elseif key == 'r' or key == 'R' then
                do_nrpnnames = true
                break
            elseif key == 'p' or key == 'P' then
                do_sysexparams = true
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
            elseif key == '?' then
                show_help(HELP_INST_DEF)
            end
        end
        draw()
        ::inst_def_again::
    end

    local new_name        = vals[1] ~= '' and vals[1] or inst.name
    inst.type             = type_opts[vals[2]] or 'keyboard'
    inst.midi_output      = (dest_opts[vals[3]] == "None") and '' or (dest_opts[vals[3]] or '')
    inst.midi_input       = (src_opts[vals[4]]  == "None") and '' or (src_opts[vals[4]]  or '')
    inst.midi_channel     = math.max(1, math.min(16, tonumber(vals[5]) or 1))
    local old_model      = inst.instrument_model
    inst.instrument_model     = (model_opts[vals[6]] == "None") and nil or (model_opts[vals[6]] or nil)

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

    -- Refresh model-derived data if model link changed.
    if inst.instrument_model ~= old_model then
        reload_drum_map(inst)
        reload_cc_names(inst)
        reload_nrpn_names(inst)
        reload_sysex_params(inst)
        rebuild_sysex_templates()
    end

    if do_sysex then
        edit_sysex(inst)
    end
    if do_drummap then
        edit_drum_map(inst)
    end
    if do_ccnames then
        edit_cc_names(inst)
    end
    if do_nrpnnames then
        edit_nrpn_names(inst)
    end
    if do_sysexparams then
        edit_sysex_params(inst)
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

        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Instruments", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  E/Enter=edit  A=add  D=delete  X=sysex  P=sysex params  N=cc names  R=nrpn names  ESC=close  ?=help",
            tui.MAGENTA, tui.BLUE, 0)

        if #instruments == 0 then
            tui.print_at(5, 2, "(no instruments — press A to add one)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, inst in ipairs(instruments) do
                local model_tag = inst.instrument_model and ("  [" .. inst.instrument_model .. "]") or ""
                local label = string.format(" %-18s  ch:%-2d  out: %s%s",
                    inst.name or '',
                    inst.midi_channel or 1,
                    inst.midi_output or '',
                    model_tag)
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(4 + i, 2, " " .. label, tui.WHITE, tui.BLUE, 0)
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
                    sysex_param_maps[inst.id] = nil
                    engine.output_ports[inst.id] = nil
                    table.remove(instruments, sel)
                    sel = math.max(1, math.min(sel, #instruments))
                    rebuild_sysex_templates()
                    ui.set_status("Instrument deleted")
                end
            end
        elseif key == 'x' or key == 'X' then
            if instruments[sel] then
                tui.clear()
                edit_sysex(instruments[sel])
            end
        elseif key == 'n' or key == 'N' then
            if instruments[sel] then
                tui.clear()
                edit_cc_names(instruments[sel])
            end
        elseif key == 'r' or key == 'R' then
            if instruments[sel] then
                tui.clear()
                edit_nrpn_names(instruments[sel])
            end
        elseif key == 'p' or key == 'P' then
            if instruments[sel] then
                tui.clear()
                edit_sysex_params(instruments[sel])
            end
        elseif key == '?' then
            show_help(HELP_INSTRUMENTS)
        end
        draw()
        ::inst_page_again::
    end
    tui.clear()
end

-- Edit a studio's name and per-instrument routing overrides.
local function edit_studio(studio)
    local is_all = (studio.name == "all")

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

    local vals = { studio.name }
    for _, inst in ipairs(instruments) do
        local ov  = studio.entries and studio.entries[inst.name]
        local live = ov and (ov.is_live ~= 0) or true
        vals[#vals+1] = live and 1 or 2
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
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Studio: " .. studio.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

        local max_row = h - 3
        for i, f in ipairs(fields) do
            local row = i + 2
            if row > max_row then break end
            local is_sel = (i == sel)
            local lbl = string.format("  %-" .. LABEL_W .. "s  ", f.label)
            if is_sel then
                tui.print_at(row, 1, lbl, tui.CYAN, tui.BLUE, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLUE, 0)
            end

            if f.type == "toggle" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                end
            else
                local locked = (i == 1 and is_all)
                if locked then
                    local display = pad(vals[i] .. "  (default, cannot rename)", val_w)
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_BLACK, tui.BLUE, 0)
                elseif is_sel and editing then
                    local display = pad(edit_buf, val_w)
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                    tui.move(row, VAL_COL + 2 + math.min(#edit_buf, val_w))
                    tui.show_cursor()
                else
                    local hint = (f.key == "chan") and "  (0=inst default)" or ""
                    local display = pad(vals[i] .. hint, val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                end
            end
        end

        local hint_row = h - 1
        if editing then
            tui.print_at(hint_row, 1, pad("  Enter=confirm  ESC=cancel edit", w),
                tui.MAGENTA, tui.BLUE, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  S=save  ESC=cancel  ?=help", w),
                tui.MAGENTA, tui.BLUE, 0)
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
            elseif key == '?' then
                show_help(HELP_STUDIO_EDIT)
            end
        end
        draw()
        ::studio_edit_again::
    end

    if not is_all and vals[1] ~= '' then
        studio.name = vals[1]
        db_mod.upsert_studio(db, studio)
    end

    local entries_list = {}
    for idx, inst in ipairs(instruments) do
        local base  = 1 + (idx - 1) * 3 + 1
        local live_v = vals[base]
        local port_v = vals[base + 1]
        local chan_v = tonumber(vals[base + 2]) or 0
        local is_live = (live_v == 1 or live_v == nil)
        local port_ov = (port_v ~= '') and port_v or nil
        local chan_ov = (chan_v > 0) and chan_v or nil
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
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        local cur_name = current_studio and current_studio.name or "all"
        tui.print_at(1, 2, "STUDIOS  (current: " .. cur_name .. ")", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  ENTER=use for song  E=edit  A=add  D=delete  ESC=close  ?=help",
            tui.MAGENTA, tui.BLUE, 0)
        for i, s in ipairs(studios) do
            local is_cur = current_studio and (s.id == current_studio.id)
            local marker = is_cur and "►" or " "
            local label  = marker .. " " .. (s.name or '')
            if i == sel then
                tui.print_at(4 + i, 2, label, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
            elseif is_cur then
                tui.print_at(4 + i, 2, label, tui.GREEN, tui.BLUE, tui.BOLD)
            else
                tui.print_at(4 + i, 2, label, tui.WHITE, tui.BLUE, 0)
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
        elseif key == '?' then
            show_help(HELP_STUDIOS)
        end
        draw()
        ::studios_again::
    end
    tui.clear()
end

-- ── Chain editor ────────────────────────────────────────────────────────────

local function edit_chain_by_id(chain_id, label)
    if not chain_id then return end

    -- Work with DB devices directly for editing.
    local devices = db_mod.get_devices(db, chain_id)
    local sel = 1

    local function pad(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function device_summary(dev_row)
        local params = {}
        for _, p in ipairs(db_mod.get_device_params(db, dev_row.id)) do
            params[p.key] = p.value
        end
        local parts = {}
        if dev_row.device_type == "instrument" then
            parts[#parts+1] = dev_row.name or ''
        elseif dev_row.device_type == "transpose" then
            parts[#parts+1] = "semitones:" .. (params.semitones or "0")
        elseif dev_row.device_type == "velocity_curve" then
            parts[#parts+1] = "curve:" .. (params.curve or "linear")
        elseif dev_row.device_type == "channel_remap" then
            parts[#parts+1] = "ch:" .. (params.channel or "1")
        elseif dev_row.device_type == "cc_filter" then
            parts[#parts+1] = (params.mode or "block") .. " " .. (params.cc_list or "")
        elseif dev_row.device_type == "arp" then
            parts[#parts+1] = "mode:" .. (params.mode or "up")
            parts[#parts+1] = "rate:" .. (arp.RATE_NAMES[tonumber(params.rate) or 0.25] or params.rate or "1/16")
        elseif dev_row.device_type == "splitter" then
            parts[#parts+1] = "outputs:" .. (params.output_count or "2")
        elseif dev_row.device_type == "lua_script" then
            if params.script_path and params.script_path ~= '' then
                parts[#parts+1] = params.script_path
            else
                parts[#parts+1] = "(inline)"
            end
        end
        return table.concat(parts, " ")
    end

    local function draw()
        local w, h = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Chain — " .. (label or ''), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(3, 2, "↑↓=select  A=add  D=delete  E/Enter=params  U=up  J=down  X=toggle  S=save  ESC=close  ?=help",
            tui.MAGENTA, tui.BLUE, 0)

        if #devices == 0 then
            tui.print_at(5, 2, "(empty chain — press A to add a device)", tui.MAGENTA, tui.BLUE, 0)
        else
            for i, dev in ipairs(devices) do
                local enabled = (dev.enabled ~= 0) and "ON " or "OFF"
                local summary = device_summary(dev)
                local dev_label = string.format(" [%s]  %-16s  %s", enabled, dev.device_type, summary)
                if i == sel then
                    tui.print_at(4 + i, 2, "►" .. pad(dev_label, w - 3), tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    local fg = (dev.enabled ~= 0) and tui.WHITE or tui.BRIGHT_BLACK
                    tui.print_at(4 + i, 2, " " .. pad(dev_label, w - 3), fg, tui.BLUE, 0)
                end
            end
        end
        tui.flush()
    end

    local function edit_device_params(dev_row)
        local proc = chain.processors[dev_row.device_type]
        if not proc or not proc.params_schema or #proc.params_schema == 0 then
            -- Instrument devices: nothing to edit here.
            if dev_row.device_type == "instrument" then
                ui.set_status("Instrument configured via I (instruments page)", tui.YELLOW)
            else
                ui.set_status("No editable params for " .. dev_row.device_type, tui.YELLOW)
            end
            return
        end

        local schema = proc.params_schema
        local params = {}
        for _, p in ipairs(db_mod.get_device_params(db, dev_row.id)) do
            params[p.key] = p.value
        end

        local vals = {}
        for _, s in ipairs(schema) do
            vals[#vals+1] = params[s.key] or s.default or ''
        end

        local LABEL_W = 16
        local VAL_COL = LABEL_W + 5
        local psel = 1
        local pediting = false
        local pedit_buf = ''

        local function pdraw()
            local w, h = tui.size()
            local val_w = math.max(10, w - VAL_COL - 4)
            tui.hide_cursor()
            tui.clear()
            tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
            tui.print_at(1, 2, "Device Params — " .. dev_row.device_type, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

            for i, s in ipairs(schema) do
                local row = i + 2
                local is_sel = (i == psel)
                local lbl = string.format("  %-" .. LABEL_W .. "s  ", s.label)
                if is_sel then
                    tui.print_at(row, 1, lbl, tui.CYAN, tui.BLUE, tui.BOLD)
                else
                    tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLUE, 0)
                end

                if s.type == "picker" then
                    -- Find index of current value.
                    local cur_idx = 1
                    for j, o in ipairs(s.opts) do
                        if o == vals[i] then cur_idx = j; break end
                    end
                    local display = pad(s.opts[cur_idx] or vals[i], val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                elseif s.type == "int" or s.type == "float" then
                    local show_val = vals[i] == '' and '—' or vals[i]
                    local display = pad(show_val, val_w)
                    if is_sel then
                        tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                else
                    if is_sel and pediting then
                        local display = pad(pedit_buf, val_w)
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.YELLOW, tui.BOLD)
                        tui.move(row, VAL_COL + 2 + math.min(#pedit_buf, val_w))
                        tui.show_cursor()
                    else
                        local display = pad(vals[i], val_w)
                        if is_sel then
                            tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                        else
                            tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                        end
                    end
                end
            end

            local hint_row = #schema + 4
            if pediting then
                tui.print_at(hint_row, 1, pad("  Enter=confirm  ESC=cancel", w),
                    tui.MAGENTA, tui.BLUE, 0)
            else
                tui.print_at(hint_row, 1,
                    pad("  ↑↓/Tab=navigate  ←→=adjust  Enter=edit  Bksp=default  S=save  ESC=cancel", w),
                    tui.MAGENTA, tui.BLUE, 0)
            end
            tui.flush()
        end

        pdraw()
        while true do
            local key = read_key_live()

            if pediting then
                if key == 'enter' or key == 'return' then
                    vals[psel] = pedit_buf
                    pediting = false
                    psel = (psel % #schema) + 1
                elseif key == 'esc' then
                    pediting = false
                elseif key == 'backspace' then
                    if #pedit_buf > 0 then pedit_buf = pedit_buf:sub(1, -2) end
                elseif #key == 1 then
                    pedit_buf = pedit_buf .. key
                end
            else
                if key == 'esc' then
                    tui.clear()
                    return
                elseif key == 's' or key == 'S' then
                    -- Save params.
                    for i, s in ipairs(schema) do
                        db_mod.upsert_device_param(db, dev_row.id, s.key, vals[i])
                    end
                    tui.clear()
                    return
                elseif key == 'tab' or key == 'down' then
                    psel = (psel % #schema) + 1
                elseif key == 'up' then
                    psel = ((psel - 2 + #schema) % #schema) + 1
                elseif key == 'left' or key == 'right' then
                    local s = schema[psel]
                    if s.type == "picker" then
                        local cur_idx = 1
                        for j, o in ipairs(s.opts) do
                            if o == vals[psel] then cur_idx = j; break end
                        end
                        local n = #s.opts
                        if key == 'right' then
                            vals[psel] = s.opts[(cur_idx % n) + 1]
                        else
                            vals[psel] = s.opts[((cur_idx - 2 + n) % n) + 1]
                        end
                    elseif s.type == "int" or s.type == "float" then
                        local step = s.type == "float" and 0.1 or 1
                        local cur = tonumber(vals[psel])
                        if key == 'right' then
                            vals[psel] = tostring(cur and (cur + step) or 0)
                        else
                            if cur == nil or cur == '' then
                                -- already blank, stay blank
                            elseif (s.type == "int" and cur <= 0) or (s.type == "float" and cur <= 0) then
                                vals[psel] = ''
                            else
                                vals[psel] = tostring(cur - step)
                            end
                        end
                        if s.type == "float" and vals[psel] ~= '' then
                            local n = tonumber(vals[psel])
                            if n then vals[psel] = string.format("%.2f", n):gsub("%.?0+$", "") end
                        end
                    end
                elseif key == 'backspace' then
                    local s = schema[psel]
                    if s.type == "int" or s.type == "float" or s.type == "text" then
                        vals[psel] = s.default or ''
                    end
                elseif key == 'enter' or key == 'return' then
                    local s = schema[psel]
                    if s.type ~= "picker" then
                        pediting = true
                        pedit_buf = vals[psel]
                    end
                end
            end
            pdraw()
        end
    end

    draw()
    while true do
        local key = read_key_live()

        if key == 'esc' or key == 's' or key == 'S' then
            break
        elseif key == '?' then
            show_help(HELP_CHAIN_EDIT)
        elseif key == 'up' then
            sel = math.max(1, sel - 1)
        elseif key == 'down' then
            sel = math.min(math.max(1, #devices), sel + 1)
        elseif key == 'a' or key == 'A' then
            -- Type picker.
            tui.clear()
            local type_sel = 1
            local types = chain.DEVICE_TYPES

            local function tdraw()
                local w2, h2 = tui.size()
                tui.hide_cursor()
                tui.clear()
                tui.print_at(1, 1, string.rep(' ', w2), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
                tui.print_at(1, 2, "Add Device — select type", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
                for i, t in ipairs(types) do
                    if i == type_sel then
                        tui.print_at(2 + i, 2, "► " .. t, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(2 + i, 2, "  " .. t, tui.WHITE, tui.BLUE, 0)
                    end
                end
                tui.print_at(#types + 4, 2, "  Enter=select  ESC=cancel", tui.MAGENTA, tui.BLUE, 0)
                tui.flush()
            end

            tdraw()
            local chosen = nil
            while true do
                local tkey = read_key_live()
                if tkey == 'esc' then break
                elseif tkey == 'up' then type_sel = math.max(1, type_sel - 1)
                elseif tkey == 'down' then type_sel = math.min(#types, type_sel + 1)
                elseif tkey == 'enter' or tkey == 'return' then
                    chosen = types[type_sel]
                    break
                end
                tdraw()
            end

            if chosen then
                local pos = #devices
                local new_dev = {
                    chain_id    = chain_id,
                    device_type = chosen,
                    position    = pos,
                    enabled     = 1,
                    name        = chosen,
                }
                -- For instrument type, pick an instrument.
                if chosen == "instrument" then
                    local inst_name = (#instruments > 0) and instruments[1].name or ''
                    local inst = instruments_by_name[inst_name]
                    new_dev.instrument_id = inst and inst.id or nil
                    new_dev.name = inst_name
                end
                db_mod.upsert_device(db, new_dev)
                -- Set default params.
                local proc = chain.processors[chosen]
                if proc and proc.params_schema then
                    for _, s in ipairs(proc.params_schema) do
                        if s.default and s.default ~= '' then
                            db_mod.upsert_device_param(db, new_dev.id, s.key, s.default)
                        end
                    end
                end
                devices = db_mod.get_devices(db, chain_id)
                sel = #devices
            end
        elseif (key == 'd' or key == 'D') and #devices > 0 then
            local dev = devices[sel]
            db_mod.delete_device(db, dev.id)
            db_mod.reorder_devices(db, chain_id)
            devices = db_mod.get_devices(db, chain_id)
            sel = math.max(1, math.min(sel, #devices))
        elseif (key == 'e' or key == 'E' or key == 'enter' or key == 'return') and #devices > 0 then
            tui.clear()
            edit_device_params(devices[sel])
            devices = db_mod.get_devices(db, chain_id)
        elseif (key == 'u' or key == 'U') and sel > 1 then
            -- Swap position with previous device.
            local a, b = devices[sel - 1], devices[sel]
            a.position, b.position = b.position, a.position
            db_mod.upsert_device(db, a)
            db_mod.upsert_device(db, b)
            devices = db_mod.get_devices(db, chain_id)
            sel = sel - 1
        elseif (key == 'j' or key == 'J') and sel < #devices then
            local a, b = devices[sel], devices[sel + 1]
            a.position, b.position = b.position, a.position
            db_mod.upsert_device(db, a)
            db_mod.upsert_device(db, b)
            devices = db_mod.get_devices(db, chain_id)
            sel = sel + 1
        elseif (key == 'x' or key == 'X') and #devices > 0 then
            local dev = devices[sel]
            dev.enabled = (dev.enabled ~= 0) and 0 or 1
            db_mod.upsert_device(db, dev)
            devices = db_mod.get_devices(db, chain_id)
        end
        draw()
    end

    reload_current_chain()
    tui.clear()
    ui.set_status("Chain updated: " .. (label or ''))
end

local function edit_chain(ti)
    local track = tracks[ti]
    if not track or not track.chain_id then return end
    edit_chain_by_id(track.chain_id, track.name or '')
end

-- Edit the current track's name and instrument assignment.
local function edit_track()
    local track = current_track()
    if not track then return end

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
        { label="Track Name",   type="text"                                    },
        { label="Instrument",   type="picker", opts=inst_names                 },
        { label="Bank SysEx",   type="sysex", hint="Enter=load"               },
        { label="Bank MSB",     type="int",   min=0, max=127, nullable=true    },
        { label="Bank LSB",     type="int",   min=0, max=127, nullable=true    },
        { label="Program",      type="int",   min=0, max=127, nullable=true    },
        { label="Program SysEx",type="sysex", hint="Enter=load"               },
    }

    local track_ovs = track.overrides or {}
    if type(track_ovs) == "string" then track_ovs = json.decode(track_ovs) or {} end
    local vals = {
        track.name or '',
        name_index(track.instrument_name),
        track.bank_sysex or '',
        track.bank_msb ~= nil and tostring(track.bank_msb) or '',
        track.bank_lsb ~= nil and tostring(track.bank_lsb) or '',
        track.program  ~= nil and tostring(track.program)  or '',
        track.program_sysex or '',
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

        tui.print_at(1, 1, string.rep(' ', w), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Track: " .. track.name, tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)

        for i, f in ipairs(fields) do
            local row    = i + 2
            local is_sel = (i == sel)
            local lbl    = string.format("  %-" .. LABEL_W .. "s  ", f.label)

            if is_sel then
                tui.print_at(row, 1, lbl, tui.CYAN, tui.BLUE, tui.BOLD)
            else
                tui.print_at(row, 1, lbl, tui.BRIGHT_WHITE, tui.BLUE, 0)
            end

            if f.type == "picker" then
                local display = pad(f.opts[vals[i]] or '', val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "◄ " .. display .. " ►", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                end
            elseif f.type == "sysex" then
                local v = vals[i] or ''
                local preview = v ~= '' and (v:sub(1, val_w) .. (#v > val_w and "..." or "")) or "(none)"
                local hint = f.hint and ("  " .. f.hint) or ""
                local display = pad(preview .. hint, val_w)
                if is_sel then
                    tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                else
                    tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
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
                        tui.print_at(row, VAL_COL, "[ " .. display .. " ]", tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
                    else
                        tui.print_at(row, VAL_COL, "  " .. display .. "  ", tui.BRIGHT_WHITE, tui.BLUE, 0)
                    end
                end
            end
        end

        local hint_row = #fields + 4
        if editing then
            tui.print_at(hint_row, 1,
                pad("  Enter=confirm  ESC=cancel edit", w),
                tui.MAGENTA, tui.BLUE, 0)
        else
            tui.print_at(hint_row, 1,
                pad("  ↑↓/Tab=navigate  ←→=cycle  Enter=edit  O=overrides  C=chain  S=save  ESC=cancel  ?=help", w),
                tui.MAGENTA, tui.BLUE, 0)
        end
        tui.flush()
    end

    draw()
    while true do
        local key = read_key_live()
        if not key then goto track_edit_again end

        if editing then
            if key == 'enter' or key == 'return' then
                local f = fields[sel]
                if f.type == "int" then
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
            elseif key == 'o' or key == 'O' then
                local inst = instruments_by_name[track.instrument_name]
                track_ovs = edit_overrides(track_ovs, inst)
            elseif key == 'c' or key == 'C' then
                -- Save current values first, then open chain editor.
                local function _parse_opt_int(s, mn, mx)
                    if s == '' then return nil end
                    local n = tonumber(s)
                    return n and math.max(mn, math.min(mx, math.floor(n))) or nil
                end
                if vals[1] ~= '' then track.name = vals[1] end
                track.instrument_name = inst_names[vals[2]] or track.instrument_name
                track.bank_sysex    = vals[3] ~= '' and vals[3] or nil
                track.bank_msb      = _parse_opt_int(vals[4], 0, 127)
                track.bank_lsb      = _parse_opt_int(vals[5], 0, 127)
                track.program       = _parse_opt_int(vals[6], 0, 127)
                track.program_sysex = vals[7] ~= '' and vals[7] or nil
                track.overrides     = #track_ovs > 0 and json.encode(track_ovs) or nil
                db_mod.upsert_track(db, track, song.id)
                -- Update instrument device in chain if instrument changed.
                local cd = track_chains[ui.cursor.track]
                if cd then
                    local idev = chain.find_device(cd, "instrument")
                    if idev then
                        local new_inst = instruments_by_name[track.instrument_name]
                        if new_inst then
                            -- Update device row.
                            local dev_rows = db_mod.get_devices(db, track.chain_id)
                            for _, dr in ipairs(dev_rows) do
                                if dr.id == idev.id then
                                    dr.instrument_id = new_inst.id
                                    dr.name = new_inst.name
                                    db_mod.upsert_device(db, dr)
                                    break
                                end
                            end
                        end
                    end
                end
                tui.clear()
                edit_chain(ui.cursor.track)
                tui.clear()
                return
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
                local f = fields[sel]
                if f.type == "sysex" then
                    tui.clear()
                    local result = read_sysex_data(3, "Hex or .syx path (blank=clear): ", vals[sel])
                    if result ~= nil then vals[sel] = result end
                elseif f.type ~= "picker" then
                    editing  = true
                    edit_buf = vals[sel]
                end
            elseif key == '?' then
                show_help(HELP_TRACK_EDIT)
            end
        end
        draw()
        ::track_edit_again::
    end

    local function parse_opt_int(s, mn, mx)
        if s == '' then return nil end
        local n = tonumber(s)
        return n and math.max(mn, math.min(mx, math.floor(n))) or nil
    end

    if vals[1] ~= '' then track.name = vals[1] end
    local old_inst_name = track.instrument_name
    track.instrument_name = inst_names[vals[2]] or track.instrument_name
    track.bank_sysex    = vals[3] ~= '' and vals[3] or nil
    track.bank_msb      = parse_opt_int(vals[4], 0, 127)
    track.bank_lsb      = parse_opt_int(vals[5], 0, 127)
    track.program       = parse_opt_int(vals[6], 0, 127)
    track.program_sysex = vals[7] ~= '' and vals[7] or nil
    track.overrides     = #track_ovs > 0 and json.encode(track_ovs) or nil
    db_mod.upsert_track(db, track, song.id)

    -- Update instrument device in chain if instrument changed.
    if track.instrument_name ~= old_inst_name then
        local cd = track_chains[ui.cursor.track]
        if cd then
            local idev = chain.find_device(cd, "instrument")
            if idev then
                local new_inst = instruments_by_name[track.instrument_name]
                if new_inst then
                    local dev_rows = db_mod.get_devices(db, track.chain_id)
                    for _, dr in ipairs(dev_rows) do
                        if dr.id == idev.id then
                            dr.instrument_id = new_inst.id
                            dr.name = new_inst.name
                            db_mod.upsert_device(db, dr)
                            break
                        end
                    end
                end
            end
        end
        reload_chain(ui.cursor.track)
    end

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

local function edit_scene()
    if ui.cursor.track ~= 0 then return end
    local si    = ui.cursor.slot
    local scene = scenes[si]
    if not scene then return end
    local w, h = tui.size()
    local cur_name = scene.name ~= '' and scene.name or ('Scene ' .. si)
    local val = ui.read_line(h - 1, "Scene name: ", cur_name)
    if val then
        scene.name = val
        db_mod.update_scene_name(db, scene.id, val)
        ui.set_status("Scene " .. si .. " renamed: " .. (val ~= '' and val or '(empty)'))
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
    local cc_names_flat = {}
    local ccm = inst and cc_name_maps[inst.id]
    if ccm and ccm["global"] then
        for cc, row in pairs(ccm["global"]) do
            cc_names_flat[cc] = row.name
        end
    end
    local nrpn_names_flat = {}
    local nrm = inst and nrpn_name_maps[inst.id]
    if nrm and nrm["global"] then
        for nrpn, row in pairs(nrm["global"]) do
            nrpn_names_flat[nrpn] = row.name
        end
    end
    local sysex_names_flat = {}
    local sysex_ranges_flat = {}
    local spm = inst and sysex_param_maps[inst.id]
    if spm then
        for pi, row in pairs(spm) do
            sysex_names_flat[pi] = row.name
            sysex_ranges_flat[pi] = { min_val = row.min_val, max_val = row.max_val, default_val = row.default_val }
        end
    end
    local new_evts, new_len, new_loop = piano_roll.open(clip, events[clip.id] or {}, dm, cc_names_flat, nrpn_names_flat, sysex_names_flat, sysex_ranges_flat)
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
    if not track.chain_id then return end

    local cd = track_chains[ti]
    if not cd then return end

    local arp_dev = chain.find_device(cd, "arp")
    if arp_dev and arp_dev.enabled then
        -- Disable arp device.
        local dev_rows = db_mod.get_devices(db, track.chain_id)
        for _, dr in ipairs(dev_rows) do
            if dr.id == arp_dev.id then
                dr.enabled = 0
                db_mod.upsert_device(db, dr)
                break
            end
        end
        reload_chain(ti)
        ui.set_status("Arp OFF — track " .. ti)
    elseif arp_dev then
        -- Re-enable arp device.
        local dev_rows = db_mod.get_devices(db, track.chain_id)
        for _, dr in ipairs(dev_rows) do
            if dr.id == arp_dev.id then
                dr.enabled = 1
                db_mod.upsert_device(db, dr)
                break
            end
        end
        reload_chain(ti)
        if inst and inst.midi_input and inst.midi_input ~= '' then
            engine.open_input(inst.midi_input)
        end
        if not engine.playing then engine.start_transport() end
        ui.set_status("Arp ON — track " .. ti)
    else
        -- No arp device — insert one at position 0.
        -- Shift existing devices up.
        local dev_rows = db_mod.get_devices(db, track.chain_id)
        for _, dr in ipairs(dev_rows) do
            dr.position = dr.position + 1
            db_mod.upsert_device(db, dr)
        end
        -- Load saved settings or use defaults.
        local settings = db_mod.get_arp_settings(db, track.id)
        local new_dev = db_mod.upsert_device(db, {
            chain_id    = track.chain_id,
            device_type = "arp",
            position    = 0,
            enabled     = 1,
            name        = "Arp",
        })
        db_mod.upsert_device_param(db, new_dev.id, "mode",      settings.mode or "up")
        db_mod.upsert_device_param(db, new_dev.id, "octaves",   tostring(settings.octaves or 1))
        db_mod.upsert_device_param(db, new_dev.id, "rate",      tostring(settings.rate or 0.25))
        db_mod.upsert_device_param(db, new_dev.id, "gate",      tostring(settings.gate or 0.8))
        db_mod.upsert_device_param(db, new_dev.id, "hold_mode", tostring(settings.hold_mode or 0))
        reload_chain(ti)
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
    if not track then return end
    local cd = track_chains[ti]
    if not cd then return end
    local arp_dev = chain.find_device(cd, "arp")
    if not arp_dev or not arp_dev.enabled then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    -- Cycle mode on the runtime state.
    local next_mode = arp.cycle_param(arp_dev.state, "mode", arp.MODES)
    -- Persist to DB.
    db_mod.upsert_device_param(db, arp_dev.id, "mode", next_mode)
    ui.set_status("Arp mode: " .. next_mode)
end

local function cycle_arp_rate()
    local ti    = ui.cursor.track
    local track = current_track()
    if not track then return end
    local cd = track_chains[ti]
    if not cd then return end
    local arp_dev = chain.find_device(cd, "arp")
    if not arp_dev or not arp_dev.enabled then
        ui.set_status("Arp not active on this track (press P to enable)", tui.YELLOW)
        return
    end
    local next_rate = arp.cycle_param(arp_dev.state, "rate", arp.RATES)
    db_mod.upsert_device_param(db, arp_dev.id, "rate", tostring(next_rate))
    ui.set_status("Arp rate: " .. (arp.RATE_NAMES[next_rate] or tostring(next_rate)))
end

-- ── Scene (row) operations ────────────────────────────────────────────────────

local function launch_scene()
    local si       = ui.cursor.slot
    local launched = 0
    for ti, track in ipairs(tracks) do
        local clip = clips[track.id] and clips[track.id][si]
        if clip then
            local inst = instruments_by_name[track.instrument_name]
            local evts = events[clip.id] or {}
            if inst then
                local patch = resolve_patch(track, clip, inst)
                if patch then engine.send_patch(inst.id, patch) end
            end
            engine.launch(ti, track_chains[ti], clip, evts)
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

local function insert_scene_row()
    local si = ui.cursor.slot

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

    for _, scene in ipairs(scenes) do
        if scene.scene_index >= si - 1 then
            scene.scene_index = scene.scene_index + 1
        end
    end

    db_mod.shift_clips(db, song.id, si, 1)
    db_mod.shift_scenes(db, song.id, si - 1, 1)
    local new_scene = db_mod.insert_scene_at(db, song.id, si - 1, '')
    table.insert(scenes, si, new_scene)

    NUM_SLOTS       = #scenes
    ui.num_slots    = NUM_SLOTS
    ui.set_status("Inserted scene row at " .. si)
end

local function delete_scene_row()
    if NUM_SLOTS <= 1 then
        ui.set_status("Cannot delete the last scene row", tui.YELLOW)
        return
    end
    local si = ui.cursor.slot

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

    db_mod.delete_clips_at_slot(db, song.id, si)
    db_mod.delete_scene_at(db, song.id, si - 1)
    db_mod.shift_clips(db, song.id, si + 1, -1)
    db_mod.shift_scenes(db, song.id, si, -1)

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

    table.remove(scenes, si)

    NUM_SLOTS    = #scenes
    ui.num_slots = NUM_SLOTS
    if ui.cursor.slot > NUM_SLOTS then ui.cursor.slot = NUM_SLOTS end
    ui.set_status("Deleted scene row " .. si)
end

-- ── Chain panel quick-add device ─────────────────────────────────────────────

local function quick_add_device(chain_id)
    if not chain_id then return end
    tui.clear()
    local type_sel = 1
    local types = chain.DEVICE_TYPES

    local function pad_s(s, n)
        s = tostring(s or '')
        if #s >= n then return s:sub(1, n) end
        return s .. string.rep(' ', n - #s)
    end

    local function tdraw()
        local w2, h2 = tui.size()
        tui.hide_cursor()
        tui.clear()
        tui.print_at(1, 1, string.rep(' ', w2), tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        tui.print_at(1, 2, "Add Device — select type", tui.BRIGHT_YELLOW, tui.RED, tui.BOLD)
        for i, t in ipairs(types) do
            if i == type_sel then
                tui.print_at(2 + i, 2, "► " .. t, tui.BLACK, tui.BRIGHT_MAGENTA, tui.BOLD)
            else
                tui.print_at(2 + i, 2, "  " .. t, tui.WHITE, tui.BLUE, 0)
            end
        end
        tui.print_at(#types + 4, 2, "  Enter=select  ESC=cancel", tui.MAGENTA, tui.BLUE, 0)
        tui.flush()
    end

    tdraw()
    local chosen = nil
    while true do
        local tkey = read_key_live()
        if tkey == 'esc' then break
        elseif tkey == 'up' then type_sel = math.max(1, type_sel - 1)
        elseif tkey == 'down' then type_sel = math.min(#types, type_sel + 1)
        elseif tkey == 'enter' or tkey == 'return' then
            chosen = types[type_sel]
            break
        end
        tdraw()
    end

    if chosen then
        local devices = db_mod.get_devices(db, chain_id)
        local pos = #devices
        local new_dev = {
            chain_id    = chain_id,
            device_type = chosen,
            position    = pos,
            enabled     = 1,
            name        = chosen,
        }
        if chosen == "instrument" then
            local inst_name = (#instruments > 0) and instruments[1].name or ''
            local inst = instruments_by_name[inst_name]
            new_dev.instrument_id = inst and inst.id or nil
            new_dev.name = inst_name
        end
        db_mod.upsert_device(db, new_dev)
        local proc = chain.processors[chosen]
        if proc and proc.params_schema then
            for _, s in ipairs(proc.params_schema) do
                if s.default and s.default ~= '' then
                    db_mod.upsert_device_param(db, new_dev.id, s.key, s.default)
                end
            end
        end
        reload_current_chain()
        ui.set_status("Added device: " .. chosen)
    end
    tui.clear()
end

-- ── Chain panel key handler ──────────────────────────────────────────────────

local function get_current_chain_id()
    if ui.cursor.track == 0 then
        return master_chain_row and master_chain_row.id
    else
        local track = tracks[ui.cursor.track]
        return track and track.chain_id
    end
end

local function handle_chain_key(key)
    local cd = ui.chain_def
    if not cd then return end
    local devs = cd.devices or {}
    local chain_id = get_current_chain_id()
    if not chain_id then return end

    if key == 'left' then
        ui.chain_cursor = math.max(1, ui.chain_cursor - 1)
    elseif key == 'right' then
        ui.chain_cursor = math.min(math.max(1, #devs), ui.chain_cursor + 1)
    elseif key == 'up' then
        ui.move_cursor(0, -1)
    elseif key == 'down' then
        ui.move_cursor(0, 1)
    elseif (key == 'x' or key == 'X') and #devs > 0 then
        -- Toggle device enabled/disabled.
        local dev = devs[ui.chain_cursor]
        if dev then
            local db_devs = db_mod.get_devices(db, chain_id)
            for _, dr in ipairs(db_devs) do
                if dr.id == dev.id then
                    dr.enabled = (dr.enabled ~= 0) and 0 or 1
                    db_mod.upsert_device(db, dr)
                    break
                end
            end
            reload_current_chain()
            ui.set_status((dev.enabled and not (dev.enabled == false or dev.enabled == 0)) and "Disabled" or "Enabled")
        end
    elseif key == 'enter' or key == 'return' then
        -- Open full chain editor.
        local lbl = ui.chain_label or ''
        edit_chain_by_id(chain_id, lbl)
    elseif key == 'a' or key == 'A' then
        quick_add_device(chain_id)
    elseif (key == 'd' or key == 'D') and #devs > 0 then
        local dev = devs[ui.chain_cursor]
        if dev then
            db_mod.delete_device(db, dev.id)
            db_mod.reorder_devices(db, chain_id)
            reload_current_chain()
            ui.chain_cursor = math.max(1, math.min(ui.chain_cursor, #((ui.chain_def or {}).devices or {})))
            ui.set_status("Deleted device")
        end
    elseif (key == 'u' or key == 'U') and ui.chain_cursor > 1 and #devs >= 2 then
        -- Swap with previous device.
        local db_devs = db_mod.get_devices(db, chain_id)
        if ui.chain_cursor <= #db_devs then
            local a, b = db_devs[ui.chain_cursor - 1], db_devs[ui.chain_cursor]
            a.position, b.position = b.position, a.position
            db_mod.upsert_device(db, a)
            db_mod.upsert_device(db, b)
            ui.chain_cursor = ui.chain_cursor - 1
            reload_current_chain()
        end
    elseif (key == 'j' or key == 'J') and ui.chain_cursor < #devs then
        local db_devs = db_mod.get_devices(db, chain_id)
        if ui.chain_cursor < #db_devs then
            local a, b = db_devs[ui.chain_cursor], db_devs[ui.chain_cursor + 1]
            a.position, b.position = b.position, a.position
            db_mod.upsert_device(db, a)
            db_mod.upsert_device(db, b)
            ui.chain_cursor = ui.chain_cursor + 1
            reload_current_chain()
        end
    end
end

-- ── Main loop ─────────────────────────────────────────────────────────────────

local running       = true
local DRAW_INTERVAL = 1.0 / 30
local last_draw     = -1

while running do
    -- 1. Engine tick (dispatch scheduled MIDI events through chains).
    engine.tick()
    engine.process_input()

    -- 2. Tick chains for tracks without active clips (input-only arp).
    if engine.playing then
        local ctx = { engine = engine, cur_beat = engine.cur_beat() }
        for ti, cd in pairs(track_chains) do
            if cd and not engine.active_clips[ti] then
                chain.tick_devices(cd, ctx)
            end
        end
    end

    -- 3. Sync UI state from engine.
    ui.playing      = engine.playing
    ui.recording    = engine.recording
    ui.bpm          = engine.bpm
    ui.studio_name  = current_studio and current_studio.name or "all"

    -- 3b. Wire chain data to UI for the bottom panel.
    if ui.cursor.track == 0 then
        ui.chain_def   = master_chain_def
        ui.chain_label = "Master"
    elseif ui.cursor.track >= 1 and ui.cursor.track <= #tracks then
        ui.chain_def   = track_chains[ui.cursor.track]
        ui.chain_label = tracks[ui.cursor.track] and tracks[ui.cursor.track].name or ""
    else
        ui.chain_def   = nil
        ui.chain_label = ""
    end
    -- Clamp chain_cursor to device count.
    local cd_devs = ui.chain_def and ui.chain_def.devices or {}
    if #cd_devs > 0 then
        ui.chain_cursor = math.max(1, math.min(ui.chain_cursor, #cd_devs))
    else
        ui.chain_cursor = 1
    end

    -- 4. Non-blocking key read.
    local key = tui.read_key(10)
    if key then
        -- Global keys (work in both focus modes).
        if key == 'esc' then
            if ui.focus == "chain" then
                ui.focus = "grid"
            else
                running = false
            end
        elseif key == 'tab' then
            ui.toggle_focus()
        elseif key == ' ' then
            if engine.playing then
                engine.stop_transport()
                ui.set_status("Stopped")
            else
                engine.start_transport()
                ui.set_status("Playing")
            end
        elseif key == 's' or key == 'S' then
            save_all()
        elseif key == 'f12' then
            midi_monitor.open()
            tui.clear()
        elseif ui.focus == "chain" then
            -- Chain panel key dispatch.
            handle_chain_key(key)
        else
            -- Grid key dispatch (original behaviour).
            if key == 'enter' or key == 'return' then
                if ui.cursor.track == 0 then launch_scene() else trigger_clip() end

            elseif key == 'up'    then ui.move_cursor(0, -1)
            elseif key == 'down'  then ui.move_cursor(0,  1)
            elseif key == 'left'  then ui.move_cursor(-1, 0)
            elseif key == 'right' then ui.move_cursor( 1, 0)

            elseif key == 'r' or key == 'R' then toggle_record()
            elseif key == 'e' or key == 'E' then
                if ui.cursor.track == 0 then edit_scene() else edit_clip_piano_roll() end
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
            elseif key == 'f' or key == 'F' then rename_song()
            elseif key == '+' or key == '=' then add_track()
            elseif key == '-'               then remove_track()
            elseif key == '?'               then show_help(HELP_MAIN); tui.clear()
            end
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

-- Cleanup all active chains.
local ctx = { engine = engine, cur_beat = engine.cur_beat() }
for _, ac in pairs(engine.active_clips) do
    if ac.chain_def then
        chain.cleanup(ac.chain_def, ctx)
    end
end
-- Also cleanup input-only chains (arp state).
for _, cd in pairs(track_chains) do
    if cd then chain.cleanup(cd, ctx) end
end

engine.stop_transport()
engine.close_all()
save_all()
db:close()
tui.exit_alt()
tui.show_cursor()
tui.cleanup()
print("Session saved to " .. song_path .. ". Goodbye!")

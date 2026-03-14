-- sequencer/db.lua
-- Database layer for the MIDI sequencer.
-- SQLite API: sqlite.open(path), db:exec(sql), db:query(sql,...), db:rows(sql,...),
--             db:prepare(sql), db:last_insert_rowid(), db:changes()
--             stmt:bind_values(...), stmt:step(), stmt:finalize()

local M = {}

-- Helper: prepare + bind + step + finalize (for INSERT/UPDATE/DELETE with params).
local function run(db, sql, ...)
    local stmt, err = db:prepare(sql)
    if not stmt then
        error("DB prepare failed: " .. tostring(err) .. "\nSQL: " .. sql, 2)
    end
    local n = select('#', ...)
    if n > 0 then
        stmt:bind_values(...)
    end
    stmt:step()
    stmt:finalize()
end

-- Open (or create) a database and initialise the schema.
function M.open(path)
    local db, err = sqlite.open(path)
    if not db then error("Cannot open database '" .. path .. "': " .. tostring(err)) end
    M.init_schema(db)
    return db
end

function M.init_schema(db)
    -- Use exec for DDL (no parameter binding needed).
    local ok, err = db:exec([[
        CREATE TABLE IF NOT EXISTS song (
            id           INTEGER PRIMARY KEY,
            name         TEXT    NOT NULL DEFAULT 'Untitled',
            bpm          REAL    NOT NULL DEFAULT 120.0,
            time_sig_num INTEGER NOT NULL DEFAULT 4,
            time_sig_den INTEGER NOT NULL DEFAULT 4,
            created_at   TEXT             DEFAULT (datetime('now')),
            updated_at   TEXT             DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS instruments (
            id           INTEGER PRIMARY KEY,
            song_id      INTEGER NOT NULL,
            name         TEXT    NOT NULL DEFAULT 'Instrument',
            midi_output  TEXT             DEFAULT '',
            midi_input   TEXT             DEFAULT '',
            midi_channel INTEGER NOT NULL DEFAULT 1,
            track_index  INTEGER NOT NULL DEFAULT 0,
            color        INTEGER NOT NULL DEFAULT 0,
            bank_msb     INTEGER          DEFAULT NULL,
            bank_lsb     INTEGER          DEFAULT NULL,
            program      INTEGER          DEFAULT NULL
        );
        CREATE TABLE IF NOT EXISTS tracks (
            id              INTEGER PRIMARY KEY,
            song_id         INTEGER NOT NULL,
            name            TEXT    NOT NULL DEFAULT 'Track',
            track_index     INTEGER NOT NULL DEFAULT 0,
            instrument_name TEXT    NOT NULL DEFAULT '',
            color           INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS clips (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            slot_index    INTEGER NOT NULL,
            name          TEXT             DEFAULT '',
            length_beats  REAL    NOT NULL DEFAULT 4.0,
            is_looping    INTEGER NOT NULL DEFAULT 1,
            start_offset  REAL    NOT NULL DEFAULT 0.0,
            loop_start    REAL    NOT NULL DEFAULT 0.0,
            loop_length   REAL             DEFAULT NULL,
            bank_msb      INTEGER          DEFAULT NULL,
            bank_lsb      INTEGER          DEFAULT NULL,
            program       INTEGER          DEFAULT NULL
        );
        CREATE TABLE IF NOT EXISTS sysex_dumps (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            name          TEXT             DEFAULT '',
            data          TEXT    NOT NULL DEFAULT '',
            send_on       TEXT    NOT NULL DEFAULT 'connect'
        );
        CREATE TABLE IF NOT EXISTS midi_events (
            id          INTEGER PRIMARY KEY,
            clip_id     INTEGER NOT NULL,
            beat_offset REAL    NOT NULL,
            status      INTEGER NOT NULL,
            data1       INTEGER NOT NULL DEFAULT 0,
            data2       INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS scenes (
            id          INTEGER PRIMARY KEY,
            song_id     INTEGER NOT NULL,
            scene_index INTEGER NOT NULL,
            name        TEXT             DEFAULT ''
        );
        CREATE TABLE IF NOT EXISTS arp_settings (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL UNIQUE,
            mode          TEXT    NOT NULL DEFAULT 'up',
            octaves       INTEGER NOT NULL DEFAULT 1,
            rate          REAL    NOT NULL DEFAULT 0.25,
            gate          REAL    NOT NULL DEFAULT 0.8,
            hold_mode     INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS drum_map (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            note          INTEGER NOT NULL,
            name          TEXT    NOT NULL DEFAULT '',
            UNIQUE(instrument_id, note)
        );
        CREATE TABLE IF NOT EXISTS studios (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS studio_instruments (
            id               INTEGER PRIMARY KEY,
            studio_id        INTEGER NOT NULL,
            instrument_name  TEXT    NOT NULL,
            is_live          INTEGER NOT NULL DEFAULT 1,
            port_override    TEXT             DEFAULT NULL,
            channel_override INTEGER          DEFAULT NULL,
            UNIQUE(studio_id, instrument_name)
        );
        CREATE TABLE IF NOT EXISTS cc_names (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            cc_number     INTEGER NOT NULL,
            note          INTEGER NOT NULL DEFAULT -1,
            name          TEXT    NOT NULL DEFAULT '',
            UNIQUE(instrument_id, cc_number, note)
        );
        CREATE TABLE IF NOT EXISTS nrpn_names (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            nrpn_number   INTEGER NOT NULL,
            note          INTEGER NOT NULL DEFAULT -1,
            name          TEXT    NOT NULL DEFAULT '',
            UNIQUE(instrument_id, nrpn_number, note)
        );
        CREATE TABLE IF NOT EXISTS device_chains (
            id             INTEGER PRIMARY KEY,
            song_id        INTEGER NOT NULL,
            track_id       INTEGER DEFAULT NULL,
            instrument_id  INTEGER DEFAULT NULL,
            name           TEXT    NOT NULL DEFAULT '',
            UNIQUE(track_id),
            UNIQUE(instrument_id)
        );
        CREATE TABLE IF NOT EXISTS devices (
            id                INTEGER PRIMARY KEY,
            chain_id          INTEGER NOT NULL,
            device_type       TEXT    NOT NULL,
            position          INTEGER NOT NULL DEFAULT 0,
            enabled           INTEGER NOT NULL DEFAULT 1,
            instrument_id     INTEGER DEFAULT NULL,
            parent_device_id  INTEGER DEFAULT NULL,
            output_index      INTEGER NOT NULL DEFAULT 0,
            name              TEXT    NOT NULL DEFAULT ''
        );
        CREATE TABLE IF NOT EXISTS device_params (
            id         INTEGER PRIMARY KEY,
            device_id  INTEGER NOT NULL,
            key        TEXT    NOT NULL,
            value      TEXT    NOT NULL DEFAULT '',
            UNIQUE(device_id, key)
        );
        CREATE TABLE IF NOT EXISTS sysex_params (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            param_index   INTEGER NOT NULL,
            name          TEXT    NOT NULL DEFAULT '',
            template      TEXT    NOT NULL DEFAULT '',
            min_val       INTEGER NOT NULL DEFAULT 0,
            max_val       INTEGER NOT NULL DEFAULT 127,
            default_val   INTEGER NOT NULL DEFAULT 0,
            UNIQUE(instrument_id, param_index)
        );
    ]])
    if not ok then error("Schema init failed: " .. tostring(err)) end
    -- Migrate existing databases: add columns that may be absent.
    -- ALTER TABLE ADD COLUMN fails if the column exists; wrap in pcall to ignore.
    for _, col in ipairs({
        "instruments ADD COLUMN bank_msb INTEGER DEFAULT NULL",
        "instruments ADD COLUMN bank_lsb INTEGER DEFAULT NULL",
        "instruments ADD COLUMN program  INTEGER DEFAULT NULL",
        "clips        ADD COLUMN bank_msb      INTEGER DEFAULT NULL",
        "clips        ADD COLUMN bank_lsb      INTEGER DEFAULT NULL",
        "clips        ADD COLUMN program       INTEGER DEFAULT NULL",
        "clips        ADD COLUMN start_offset  REAL    NOT NULL DEFAULT 0.0",
        "clips        ADD COLUMN loop_start    REAL    NOT NULL DEFAULT 0.0",
        "clips        ADD COLUMN loop_length   REAL    DEFAULT NULL",
        "song         ADD COLUMN studio_id     INTEGER DEFAULT NULL",
        "instruments  ADD COLUMN type          TEXT    NOT NULL DEFAULT 'keyboard'",
        "instruments  ADD COLUMN instrument_model TEXT    DEFAULT NULL",
        "tracks       ADD COLUMN chain_id     INTEGER DEFAULT NULL",
        "tracks       ADD COLUMN bank_sysex    TEXT    DEFAULT NULL",
        "tracks       ADD COLUMN bank_msb      INTEGER DEFAULT NULL",
        "tracks       ADD COLUMN bank_lsb      INTEGER DEFAULT NULL",
        "tracks       ADD COLUMN program       INTEGER DEFAULT NULL",
        "tracks       ADD COLUMN program_sysex TEXT    DEFAULT NULL",
        "tracks       ADD COLUMN overrides     TEXT    DEFAULT NULL",
        "clips        ADD COLUMN bank_sysex    TEXT    DEFAULT NULL",
        "clips        ADD COLUMN program_sysex TEXT    DEFAULT NULL",
        "clips        ADD COLUMN overrides     TEXT    DEFAULT NULL",
        "song         ADD COLUMN master_chain_id INTEGER DEFAULT NULL",
    }) do
        pcall(function() db:exec("ALTER TABLE " .. col) end)
    end
    -- Seed the default studio.
    db:exec("INSERT OR IGNORE INTO studios (id, name) VALUES (1, 'all')")
    -- Migrate instruments → tracks: if tracks is empty but instruments has rows,
    -- populate tracks from instruments so existing clip references (instrument_id)
    -- remain valid (track.id == old instrument.id).
    local tc = db:query("SELECT COUNT(*) as n FROM tracks") or {}
    local ic = db:query("SELECT COUNT(*) as n FROM instruments") or {}
    if (tc[1] and tc[1].n == 0) and (ic[1] and ic[1].n > 0) then
        db:exec([[
            INSERT OR IGNORE INTO tracks (id, song_id, name, track_index, instrument_name, color)
            SELECT id, song_id, name, track_index, name, color FROM instruments
        ]])
    end
end

function M.get_or_create_song(db, name)
    local rows = db:query("SELECT * FROM song ORDER BY id LIMIT 1")
    if rows and #rows > 0 then return rows[1] end
    run(db, "INSERT INTO song (name) VALUES (?)", name or "Untitled")
    local rows2 = db:query("SELECT * FROM song ORDER BY id DESC LIMIT 1")
    return rows2[1]
end

function M.save_song(db, song)
    run(db,
        "UPDATE song SET name=?, bpm=?, time_sig_num=?, time_sig_den=?, studio_id=?, master_chain_id=?, updated_at=datetime('now') WHERE id=?",
        song.name, song.bpm, song.time_sig_num, song.time_sig_den, song.studio_id, song.master_chain_id, song.id)
end

function M.get_instruments(db, song_id)
    return db:query("SELECT * FROM instruments WHERE song_id=? ORDER BY track_index", song_id) or {}
end

-- Insert or update an instrument. Sets inst.id on insert.
-- instruments are standalone patch/device definitions; track_index is not used.
function M.upsert_instrument(db, inst, song_id)
    if inst.id then
        run(db,
            "UPDATE instruments SET name=?, midi_output=?, midi_input=?, midi_channel=?, color=?, bank_msb=?, bank_lsb=?, program=?, type=?, instrument_model=? WHERE id=?",
            inst.name,
            inst.midi_output  or '',
            inst.midi_input   or '',
            inst.midi_channel or 1,
            inst.color        or 0,
            inst.bank_msb,
            inst.bank_lsb,
            inst.program,
            inst.type         or 'keyboard',
            inst.instrument_model,
            inst.id)
    else
        run(db,
            "INSERT INTO instruments (song_id,name,midi_output,midi_input,midi_channel,color,bank_msb,bank_lsb,program,type,instrument_model) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
            song_id,
            inst.name,
            inst.midi_output  or '',
            inst.midi_input   or '',
            inst.midi_channel or 1,
            inst.color        or 0,
            inst.bank_msb,
            inst.bank_lsb,
            inst.program,
            inst.type         or 'keyboard',
            inst.instrument_model)
        inst.id = db:last_insert_rowid()
    end
    return inst
end

function M.delete_instrument(db, inst_id)
    -- Clips belong to tracks (not instruments), so only remove sysex + instrument row.
    run(db, "DELETE FROM sysex_dumps WHERE instrument_id=?", inst_id)
    run(db, "DELETE FROM sysex_params WHERE instrument_id=?", inst_id)
    run(db, "DELETE FROM instruments WHERE id=?", inst_id)
end

-- ── Tracks ────────────────────────────────────────────────────────────────────

function M.get_tracks(db, song_id)
    return db:query("SELECT * FROM tracks WHERE song_id=? ORDER BY track_index", song_id) or {}
end

-- Insert or update a track. Sets track.id on insert.
function M.upsert_track(db, track, song_id)
    if track.id then
        run(db,
            "UPDATE tracks SET name=?, track_index=?, instrument_name=?, color=?, chain_id=?, bank_sysex=?, bank_msb=?, bank_lsb=?, program=?, program_sysex=?, overrides=? WHERE id=?",
            track.name,
            track.track_index     or 0,
            track.instrument_name or '',
            track.color           or 0,
            track.chain_id,
            track.bank_sysex,
            track.bank_msb,
            track.bank_lsb,
            track.program,
            track.program_sysex,
            track.overrides,
            track.id)
    else
        run(db,
            "INSERT INTO tracks (song_id,name,track_index,instrument_name,color,chain_id,bank_sysex,bank_msb,bank_lsb,program,program_sysex,overrides) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",
            song_id,
            track.name,
            track.track_index     or 0,
            track.instrument_name or '',
            track.color           or 0,
            track.chain_id,
            track.bank_sysex,
            track.bank_msb,
            track.bank_lsb,
            track.program,
            track.program_sysex,
            track.overrides)
        track.id = db:last_insert_rowid()
    end
    return track
end

function M.delete_track(db, track_id)
    -- Cascade: midi_events → clips → arp_settings → track row.
    run(db, "DELETE FROM midi_events WHERE clip_id IN (SELECT id FROM clips WHERE instrument_id=?)", track_id)
    run(db, "DELETE FROM clips WHERE instrument_id=?", track_id)
    run(db, "DELETE FROM arp_settings WHERE instrument_id=?", track_id)
    run(db, "DELETE FROM tracks WHERE id=?", track_id)
end

function M.get_clips(db, instrument_id)
    return db:query("SELECT * FROM clips WHERE instrument_id=? ORDER BY slot_index", instrument_id) or {}
end

function M.get_clip(db, instrument_id, slot_index)
    local rows = db:query(
        "SELECT * FROM clips WHERE instrument_id=? AND slot_index=?",
        instrument_id, slot_index) or {}
    return rows[1]
end

-- Insert or update a clip. Sets clip.id on insert.
function M.upsert_clip(db, clip)
    if clip.id then
        run(db,
            "UPDATE clips SET name=?, length_beats=?, is_looping=?, start_offset=?, loop_start=?, loop_length=?, bank_msb=?, bank_lsb=?, program=?, bank_sysex=?, program_sysex=?, overrides=? WHERE id=?",
            clip.name or '',
            clip.length_beats or 4.0,
            clip.is_looping ~= false and 1 or 0,
            clip.start_offset or 0.0,
            clip.loop_start   or 0.0,
            clip.loop_length,
            clip.bank_msb,
            clip.bank_lsb,
            clip.program,
            clip.bank_sysex,
            clip.program_sysex,
            clip.overrides,
            clip.id)
    else
        run(db,
            "INSERT INTO clips (instrument_id,slot_index,name,length_beats,is_looping,start_offset,loop_start,loop_length,bank_msb,bank_lsb,program,bank_sysex,program_sysex,overrides) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
            clip.instrument_id,
            clip.slot_index,
            clip.name or '',
            clip.length_beats or 4.0,
            clip.is_looping ~= false and 1 or 0,
            clip.start_offset or 0.0,
            clip.loop_start   or 0.0,
            clip.loop_length,
            clip.bank_msb,
            clip.bank_lsb,
            clip.program,
            clip.bank_sysex,
            clip.program_sysex,
            clip.overrides)
        clip.id = db:last_insert_rowid()
    end
    return clip
end

function M.delete_clip(db, clip_id)
    run(db, "DELETE FROM midi_events WHERE clip_id=?", clip_id)
    run(db, "DELETE FROM clips WHERE id=?", clip_id)
end

function M.get_events(db, clip_id)
    return db:query(
        "SELECT * FROM midi_events WHERE clip_id=? ORDER BY beat_offset",
        clip_id) or {}
end

-- Replace all MIDI events for a clip.
function M.save_events(db, clip_id, evts)
    run(db, "DELETE FROM midi_events WHERE clip_id=?", clip_id)
    for _, ev in ipairs(evts) do
        run(db,
            "INSERT INTO midi_events (clip_id,beat_offset,status,data1,data2) VALUES (?,?,?,?,?)",
            clip_id, ev.beat_offset, ev.status, ev.data1 or 0, ev.data2 or 0)
    end
end

function M.get_scenes(db, song_id)
    return db:query("SELECT * FROM scenes WHERE song_id=? ORDER BY scene_index", song_id) or {}
end

-- Shift slot_index by delta for all clips in this song where slot_index >= from_slot.
function M.shift_clips(db, song_id, from_slot, delta)
    run(db, [[
        UPDATE clips SET slot_index = slot_index + ?
        WHERE slot_index >= ?
          AND instrument_id IN (SELECT id FROM tracks WHERE song_id = ?)
    ]], delta, from_slot, song_id)
end

-- Delete all clips (and their events) at a specific slot across all tracks in the song.
function M.delete_clips_at_slot(db, song_id, slot)
    run(db, [[
        DELETE FROM midi_events WHERE clip_id IN (
            SELECT c.id FROM clips c
            WHERE c.slot_index = ?
              AND c.instrument_id IN (SELECT id FROM tracks WHERE song_id = ?)
        )
    ]], slot, song_id)
    run(db, [[
        DELETE FROM clips
        WHERE slot_index = ?
          AND instrument_id IN (SELECT id FROM tracks WHERE song_id = ?)
    ]], slot, song_id)
end

-- Shift scene_index by delta for all scenes in this song where scene_index >= from_idx.
function M.shift_scenes(db, song_id, from_idx, delta)
    run(db, [[
        UPDATE scenes SET scene_index = scene_index + ?
        WHERE scene_index >= ? AND song_id = ?
    ]], delta, from_idx, song_id)
end

-- Insert a new scene row and return a Lua table representing it.
function M.insert_scene_at(db, song_id, scene_index, name)
    run(db,
        "INSERT INTO scenes (song_id, scene_index, name) VALUES (?, ?, ?)",
        song_id, scene_index, name or '')
    return { id = db:last_insert_rowid(), song_id = song_id,
             scene_index = scene_index, name = name or '' }
end

-- Delete the scene row at the given scene_index.
function M.delete_scene_at(db, song_id, scene_index)
    run(db, "DELETE FROM scenes WHERE song_id = ? AND scene_index = ?",
        song_id, scene_index)
end

-- Ensure at least `count` scene rows exist, creating missing ones.
function M.ensure_scenes(db, song_id, count)
    local scenes = M.get_scenes(db, song_id)
    for i = #scenes + 1, count do
        run(db,
            "INSERT INTO scenes (song_id, scene_index, name) VALUES (?,?,?)",
            song_id, i - 1, "Scene " .. i)
    end
    if #scenes < count then
        return M.get_scenes(db, song_id)
    end
    return scenes
end

function M.update_scene_name(db, scene_id, name)
    run(db, "UPDATE scenes SET name=? WHERE id=?", name or '', scene_id)
end

-- ── SysEx dumps ───────────────────────────────────────────────────────────────

function M.get_sysex_dumps(db, instrument_id)
    return db:query(
        "SELECT * FROM sysex_dumps WHERE instrument_id=? ORDER BY id",
        instrument_id) or {}
end

function M.upsert_sysex_dump(db, dump)
    if dump.id then
        run(db,
            "UPDATE sysex_dumps SET name=?, data=?, send_on=? WHERE id=?",
            dump.name or '', dump.data or '', dump.send_on or 'connect', dump.id)
    else
        run(db,
            "INSERT INTO sysex_dumps (instrument_id, name, data, send_on) VALUES (?,?,?,?)",
            dump.instrument_id, dump.name or '', dump.data or '', dump.send_on or 'connect')
        dump.id = db:last_insert_rowid()
    end
    return dump
end

function M.delete_sysex_dump(db, dump_id)
    run(db, "DELETE FROM sysex_dumps WHERE id=?", dump_id)
end

-- ── Arp settings ──────────────────────────────────────────────────────────────

function M.get_arp_settings(db, instrument_id)
    local rows = db:query(
        "SELECT * FROM arp_settings WHERE instrument_id=?", instrument_id) or {}
    if rows[1] then return rows[1] end
    return { instrument_id=instrument_id, mode="up", octaves=1, rate=0.25, gate=0.8, hold_mode=0 }
end

function M.save_arp_settings(db, instrument_id, state)
    run(db,
        [[INSERT INTO arp_settings (instrument_id, mode, octaves, rate, gate, hold_mode)
          VALUES (?,?,?,?,?,?)
          ON CONFLICT(instrument_id) DO UPDATE SET
            mode=excluded.mode, octaves=excluded.octaves,
            rate=excluded.rate, gate=excluded.gate, hold_mode=excluded.hold_mode]],
        instrument_id,
        state.mode    or "up",
        state.octaves or 1,
        state.rate    or 0.25,
        state.gate    or 0.8,
        state.hold    and 1 or 0)
end

-- ── Drum maps ─────────────────────────────────────────────────────────────────

function M.get_drum_map(db, instrument_id)
    return db:query(
        "SELECT * FROM drum_map WHERE instrument_id=? ORDER BY note",
        instrument_id) or {}
end

-- Replace all drum map entries for an instrument.
function M.save_drum_map(db, instrument_id, entries)
    run(db, "DELETE FROM drum_map WHERE instrument_id=?", instrument_id)
    for _, e in ipairs(entries) do
        run(db,
            "INSERT INTO drum_map (instrument_id, note, name) VALUES (?,?,?)",
            instrument_id, e.note, e.name or '')
    end
end

-- ── Studios ───────────────────────────────────────────────────────────────────

function M.get_studios(db)
    return db:query("SELECT * FROM studios ORDER BY id") or {}
end

-- Insert or update a studio. Sets studio.id on insert.
function M.upsert_studio(db, studio)
    if studio.id then
        run(db, "UPDATE studios SET name=? WHERE id=?", studio.name, studio.id)
    else
        run(db, "INSERT INTO studios (name) VALUES (?)", studio.name)
        studio.id = db:last_insert_rowid()
    end
    return studio
end

function M.delete_studio(db, studio_id)
    run(db, "DELETE FROM studio_instruments WHERE studio_id=?", studio_id)
    run(db, "DELETE FROM studios WHERE id=?", studio_id)
end

function M.get_studio_instruments(db, studio_id)
    return db:query(
        "SELECT * FROM studio_instruments WHERE studio_id=? ORDER BY id",
        studio_id) or {}
end

-- Replace all studio_instruments for a studio.
function M.save_studio_instruments(db, studio_id, list)
    run(db, "DELETE FROM studio_instruments WHERE studio_id=?", studio_id)
    for _, e in ipairs(list) do
        run(db,
            "INSERT INTO studio_instruments (studio_id, instrument_name, is_live, port_override, channel_override) VALUES (?,?,?,?,?)",
            studio_id,
            e.instrument_name,
            e.is_live ~= nil and e.is_live or 1,
            e.port_override    or nil,
            e.channel_override or nil)
    end
end

-- ── CC name maps ──────────────────────────────────────────────────────────────

-- Returns all CC name rows for an instrument, ordered by cc_number then note.
function M.get_cc_names(db, instrument_id)
    return db:query(
        "SELECT * FROM cc_names WHERE instrument_id=? ORDER BY cc_number, note",
        instrument_id) or {}
end

-- Insert or update a CC name entry.
-- For new entries (no id) uses ON CONFLICT upsert; sets entry.id on insert.
function M.upsert_cc_name(db, entry)
    if entry.id then
        run(db, "UPDATE cc_names SET name=? WHERE id=?",
            entry.name or '', entry.id)
    else
        run(db, [[
            INSERT INTO cc_names (instrument_id, cc_number, note, name)
            VALUES (?,?,?,?)
            ON CONFLICT(instrument_id, cc_number, note) DO UPDATE SET name=excluded.name
        ]], entry.instrument_id, entry.cc_number, entry.note or -1, entry.name or '')
        entry.id = db:last_insert_rowid()
    end
    return entry
end

function M.delete_cc_name(db, id)
    run(db, "DELETE FROM cc_names WHERE id=?", id)
end

-- ── NRPN name maps ──────────────────────────────────────────────────────────

function M.get_nrpn_names(db, instrument_id)
    return db:query(
        "SELECT * FROM nrpn_names WHERE instrument_id=? ORDER BY nrpn_number, note",
        instrument_id) or {}
end

function M.upsert_nrpn_name(db, entry)
    if entry.id then
        run(db, "UPDATE nrpn_names SET name=? WHERE id=?",
            entry.name or '', entry.id)
    else
        run(db, [[
            INSERT INTO nrpn_names (instrument_id, nrpn_number, note, name)
            VALUES (?,?,?,?)
            ON CONFLICT(instrument_id, nrpn_number, note) DO UPDATE SET name=excluded.name
        ]], entry.instrument_id, entry.nrpn_number, entry.note or -1, entry.name or '')
        entry.id = db:last_insert_rowid()
    end
    return entry
end

function M.delete_nrpn_name(db, id)
    run(db, "DELETE FROM nrpn_names WHERE id=?", id)
end

-- ── SysEx params (sequenceable sysex parameter templates) ────────────────────

function M.get_sysex_params(db, instrument_id)
    return db:query(
        "SELECT * FROM sysex_params WHERE instrument_id=? ORDER BY param_index",
        instrument_id) or {}
end

function M.upsert_sysex_param(db, entry)
    if entry.id then
        run(db,
            "UPDATE sysex_params SET name=?, template=?, min_val=?, max_val=?, default_val=? WHERE id=?",
            entry.name or '', entry.template or '', entry.min_val or 0,
            entry.max_val or 127, entry.default_val or 0, entry.id)
    else
        run(db, [[
            INSERT INTO sysex_params (instrument_id, param_index, name, template, min_val, max_val, default_val)
            VALUES (?,?,?,?,?,?,?)
            ON CONFLICT(instrument_id, param_index) DO UPDATE SET
                name=excluded.name, template=excluded.template,
                min_val=excluded.min_val, max_val=excluded.max_val, default_val=excluded.default_val
        ]], entry.instrument_id, entry.param_index, entry.name or '', entry.template or '',
            entry.min_val or 0, entry.max_val or 127, entry.default_val or 0)
        entry.id = db:last_insert_rowid()
    end
    return entry
end

function M.delete_sysex_param(db, id)
    run(db, "DELETE FROM sysex_params WHERE id=?", id)
end

function M.replace_sysex_params(db, instrument_id, params_list)
    run(db, "DELETE FROM sysex_params WHERE instrument_id=?", instrument_id)
    for _, p in ipairs(params_list) do
        run(db, [[
            INSERT INTO sysex_params (instrument_id, param_index, name, template, min_val, max_val, default_val)
            VALUES (?,?,?,?,?,?,?)
        ]], instrument_id, p.param_index, p.name or '', p.template or '',
            p.min_val or 0, p.max_val or 127, p.default_val or 0)
    end
end

-- ── Device chains ───────────────────────────────────────────────────────────

function M.get_device_chain(db, chain_id)
    local rows = db:query("SELECT * FROM device_chains WHERE id=?", chain_id) or {}
    return rows[1]
end

function M.get_track_chain(db, track_id)
    local rows = db:query("SELECT * FROM device_chains WHERE track_id=?", track_id) or {}
    return rows[1]
end

function M.get_instrument_chain(db, instrument_id)
    local rows = db:query("SELECT * FROM device_chains WHERE instrument_id=?", instrument_id) or {}
    return rows[1]
end

function M.upsert_device_chain(db, chain, song_id)
    if chain.id then
        run(db,
            "UPDATE device_chains SET name=?, track_id=?, instrument_id=? WHERE id=?",
            chain.name or '', chain.track_id, chain.instrument_id, chain.id)
    else
        run(db,
            "INSERT INTO device_chains (song_id, track_id, instrument_id, name) VALUES (?,?,?,?)",
            song_id, chain.track_id, chain.instrument_id, chain.name or '')
        chain.id = db:last_insert_rowid()
    end
    return chain
end

function M.delete_device_chain(db, chain_id)
    -- Cascade: device_params → devices → device_chains.
    run(db, "DELETE FROM device_params WHERE device_id IN (SELECT id FROM devices WHERE chain_id=?)", chain_id)
    run(db, "DELETE FROM devices WHERE chain_id=?", chain_id)
    run(db, "DELETE FROM device_chains WHERE id=?", chain_id)
end

-- Get (or lazily create) the master device chain for a song.
-- The chain has track_id=NULL, instrument_id=NULL.  Its id is stored on the song row.
function M.get_or_create_master_chain(db, song)
    if song.master_chain_id then
        local rows = db:query("SELECT * FROM device_chains WHERE id=?", song.master_chain_id) or {}
        if rows[1] then return rows[1] end
    end
    -- Create a new chain for the master bus.
    run(db,
        "INSERT INTO device_chains (song_id, track_id, instrument_id, name) VALUES (?,NULL,NULL,?)",
        song.id, "Master")
    local chain_id = db:last_insert_rowid()
    song.master_chain_id = chain_id
    run(db, "UPDATE song SET master_chain_id=? WHERE id=?", chain_id, song.id)
    local rows = db:query("SELECT * FROM device_chains WHERE id=?", chain_id) or {}
    return rows[1]
end

-- ── Devices ─────────────────────────────────────────────────────────────────

function M.get_devices(db, chain_id)
    return db:query(
        "SELECT * FROM devices WHERE chain_id=? AND parent_device_id IS NULL ORDER BY position",
        chain_id) or {}
end

function M.upsert_device(db, dev)
    if dev.id then
        run(db,
            "UPDATE devices SET device_type=?, position=?, enabled=?, instrument_id=?, parent_device_id=?, output_index=?, name=? WHERE id=?",
            dev.device_type,
            dev.position        or 0,
            dev.enabled ~= nil and dev.enabled or 1,
            dev.instrument_id,
            dev.parent_device_id,
            dev.output_index    or 0,
            dev.name            or '',
            dev.id)
    else
        run(db,
            "INSERT INTO devices (chain_id, device_type, position, enabled, instrument_id, parent_device_id, output_index, name) VALUES (?,?,?,?,?,?,?,?)",
            dev.chain_id,
            dev.device_type,
            dev.position        or 0,
            dev.enabled ~= nil and dev.enabled or 1,
            dev.instrument_id,
            dev.parent_device_id,
            dev.output_index    or 0,
            dev.name            or '')
        dev.id = db:last_insert_rowid()
    end
    return dev
end

function M.delete_device(db, device_id)
    run(db, "DELETE FROM device_params WHERE device_id=?", device_id)
    -- Also delete child devices (splitter sub-chains).
    local children = db:query("SELECT id FROM devices WHERE parent_device_id=?", device_id) or {}
    for _, child in ipairs(children) do
        M.delete_device(db, child.id)
    end
    run(db, "DELETE FROM devices WHERE id=?", device_id)
end

function M.reorder_devices(db, chain_id)
    local devs = M.get_devices(db, chain_id)
    for i, dev in ipairs(devs) do
        run(db, "UPDATE devices SET position=? WHERE id=?", i - 1, dev.id)
    end
end

-- ── Device params ───────────────────────────────────────────────────────────

function M.get_device_params(db, device_id)
    return db:query(
        "SELECT * FROM device_params WHERE device_id=? ORDER BY key",
        device_id) or {}
end

function M.upsert_device_param(db, device_id, key, value)
    run(db,
        [[INSERT INTO device_params (device_id, key, value) VALUES (?,?,?)
          ON CONFLICT(device_id, key) DO UPDATE SET value=excluded.value]],
        device_id, key, value or '')
end

function M.delete_device_param(db, device_id, key)
    run(db, "DELETE FROM device_params WHERE device_id=? AND key=?", device_id, key)
end

-- ── Auto-migration: tracks → chains ─────────────────────────────────────────

-- For tracks with instrument_name but no chain_id, create a default chain
-- containing an instrument device (and optionally an arp device).
function M.migrate_tracks_to_chains(db, song_id, instruments_by_name)
    local unmigrated = db:query(
        "SELECT * FROM tracks WHERE song_id=? AND chain_id IS NULL", song_id) or {}
    for _, track in ipairs(unmigrated) do
        if track.instrument_name and track.instrument_name ~= '' then
            -- Create chain.
            run(db, "INSERT INTO device_chains (song_id, track_id, name) VALUES (?,?,?)",
                song_id, track.id, track.name or '')
            local chain_id = db:last_insert_rowid()

            local position = 0

            -- Check for persisted arp settings (id present means a DB row exists).
            local arp_s = M.get_arp_settings(db, track.id)
            if arp_s.id then
                run(db,
                    "INSERT INTO devices (chain_id, device_type, position, enabled, name) VALUES (?,?,?,?,?)",
                    chain_id, "arp", position, 0, "Arp")
                local arp_dev_id = db:last_insert_rowid()
                -- Copy arp params.
                for _, kv in ipairs({
                    {"mode",      arp_s.mode or "up"},
                    {"octaves",   tostring(arp_s.octaves or 1)},
                    {"rate",      tostring(arp_s.rate or 0.25)},
                    {"gate",      tostring(arp_s.gate or 0.8)},
                    {"hold_mode", tostring(arp_s.hold_mode or 0)},
                }) do
                    run(db,
                        "INSERT INTO device_params (device_id, key, value) VALUES (?,?,?)",
                        arp_dev_id, kv[1], kv[2])
                end
                position = position + 1
            end

            -- Create instrument device.
            local inst = instruments_by_name[track.instrument_name]
            local inst_id = inst and inst.id or nil
            run(db,
                "INSERT INTO devices (chain_id, device_type, position, instrument_id, name) VALUES (?,?,?,?,?)",
                chain_id, "instrument", position, inst_id, track.instrument_name)

            -- Update track.
            run(db, "UPDATE tracks SET chain_id=? WHERE id=?", chain_id, track.id)
        end
    end
end

return M

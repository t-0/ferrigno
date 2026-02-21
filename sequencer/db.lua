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
            color        INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS clips (
            id            INTEGER PRIMARY KEY,
            instrument_id INTEGER NOT NULL,
            slot_index    INTEGER NOT NULL,
            name          TEXT             DEFAULT '',
            length_beats  REAL    NOT NULL DEFAULT 4.0,
            is_looping    INTEGER NOT NULL DEFAULT 1
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
    ]])
    if not ok then error("Schema init failed: " .. tostring(err)) end
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
        "UPDATE song SET name=?, bpm=?, time_sig_num=?, time_sig_den=?, updated_at=datetime('now') WHERE id=?",
        song.name, song.bpm, song.time_sig_num, song.time_sig_den, song.id)
end

function M.get_instruments(db, song_id)
    return db:query("SELECT * FROM instruments WHERE song_id=? ORDER BY track_index", song_id) or {}
end

-- Insert or update an instrument. Sets inst.id on insert.
function M.upsert_instrument(db, inst, song_id)
    if inst.id then
        run(db,
            "UPDATE instruments SET name=?, midi_output=?, midi_input=?, midi_channel=?, track_index=?, color=? WHERE id=?",
            inst.name,
            inst.midi_output  or '',
            inst.midi_input   or '',
            inst.midi_channel or 1,
            inst.track_index,
            inst.color        or 0,
            inst.id)
    else
        run(db,
            "INSERT INTO instruments (song_id,name,midi_output,midi_input,midi_channel,track_index,color) VALUES (?,?,?,?,?,?,?)",
            song_id,
            inst.name,
            inst.midi_output  or '',
            inst.midi_input   or '',
            inst.midi_channel or 1,
            inst.track_index,
            inst.color        or 0)
        inst.id = db:last_insert_rowid()
    end
    return inst
end

function M.delete_instrument(db, inst_id)
    -- Cascade manually (no FK enforcement guarantee without PRAGMA).
    run(db, "DELETE FROM midi_events WHERE clip_id IN (SELECT id FROM clips WHERE instrument_id=?)", inst_id)
    run(db, "DELETE FROM clips WHERE instrument_id=?", inst_id)
    run(db, "DELETE FROM instruments WHERE id=?", inst_id)
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
            "UPDATE clips SET name=?, length_beats=?, is_looping=? WHERE id=?",
            clip.name or '',
            clip.length_beats or 4.0,
            clip.is_looping ~= false and 1 or 0,
            clip.id)
    else
        run(db,
            "INSERT INTO clips (instrument_id,slot_index,name,length_beats,is_looping) VALUES (?,?,?,?,?)",
            clip.instrument_id,
            clip.slot_index,
            clip.name or '',
            clip.length_beats or 4.0,
            clip.is_looping ~= false and 1 or 0)
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

return M

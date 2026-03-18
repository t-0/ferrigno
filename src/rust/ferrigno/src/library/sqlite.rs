use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::user::*;
use crate::utility::*;
use std::mem::size_of;
use std::ptr::*;

// lua_upvalueindex(1)
const UPVALUE1: i32 = LUA_REGISTRYINDEX - 1;

// ─── SQLite3 FFI ──────────────────────────────────────────────────────────────

#[link(name = "sqlite3")]
unsafe extern "C" {
    fn sqlite3_open_v2(
        filename: *const std::ffi::c_char, ppdb: *mut *mut std::ffi::c_void, flags: std::ffi::c_int, zvfs: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn sqlite3_close(db: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn sqlite3_errmsg(db: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn sqlite3_exec(
        db: *mut std::ffi::c_void, sql: *const std::ffi::c_char, callback: *mut std::ffi::c_void, arg: *mut std::ffi::c_void,
        errmsg: *mut *mut std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn sqlite3_free(p: *mut std::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut std::ffi::c_void, zsql: *const std::ffi::c_char, nbyte: std::ffi::c_int, ppstmt: *mut *mut std::ffi::c_void,
        pztail: *mut *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn sqlite3_step(pstmt: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn sqlite3_reset(pstmt: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn sqlite3_finalize(pstmt: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn sqlite3_column_count(pstmt: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn sqlite3_column_type(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> std::ffi::c_int;
    fn sqlite3_column_name(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> *const std::ffi::c_char;
    fn sqlite3_column_int64(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> i64;
    fn sqlite3_column_double(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> f64;
    fn sqlite3_column_text(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> *const u8;
    fn sqlite3_column_bytes(pstmt: *mut std::ffi::c_void, icol: std::ffi::c_int) -> std::ffi::c_int;
    fn sqlite3_bind_null(pstmt: *mut std::ffi::c_void, i: std::ffi::c_int) -> std::ffi::c_int;
    fn sqlite3_bind_int64(pstmt: *mut std::ffi::c_void, i: std::ffi::c_int, v: i64) -> std::ffi::c_int;
    fn sqlite3_bind_double(pstmt: *mut std::ffi::c_void, i: std::ffi::c_int, v: f64) -> std::ffi::c_int;
    fn sqlite3_bind_text(
        pstmt: *mut std::ffi::c_void, i: std::ffi::c_int, z: *const std::ffi::c_char, n: std::ffi::c_int,
        destructor: *const std::ffi::c_void,
    ) -> std::ffi::c_int;
    fn sqlite3_last_insert_rowid(db: *mut std::ffi::c_void) -> i64;
    fn sqlite3_changes(db: *mut std::ffi::c_void) -> std::ffi::c_int;
}

const SQLITE_OK: std::ffi::c_int = 0;
const SQLITE_ROW: std::ffi::c_int = 100;
const SQLITE_DONE: std::ffi::c_int = 101;

const SQLITE_OPEN_READONLY: std::ffi::c_int = 0x00000001;
const SQLITE_OPEN_READWRITE: std::ffi::c_int = 0x00000002;
const SQLITE_OPEN_CREATE: std::ffi::c_int = 0x00000004;
const SQLITE_OPEN_URI: std::ffi::c_int = 0x00000040;

const SQLITE3_COL_INTEGER: std::ffi::c_int = 1;
const SQLITE3_COL_FLOAT: std::ffi::c_int = 2;
const SQLITE3_COL_TEXT: std::ffi::c_int = 3;
const SQLITE3_COL_BLOB: std::ffi::c_int = 4;
// const SQLITE3_COL_NULL: std::ffi::c_int  = 5;

// SQLITE_TRANSIENT: SQLite will copy the data before sqlite3_bind_text returns.
const SQLITE_TRANSIENT: *const std::ffi::c_void = usize::MAX as *const std::ffi::c_void;

const DB_META: *const i8 = c"sqlite3*".as_ptr();
const STMT_META: *const i8 = c"sqlite3_stmt*".as_ptr();
const ROWS_META: *const i8 = c"sqlite3_rows*".as_ptr();

// ─── Userdata structs ─────────────────────────────────────────────────────────

#[repr(C)]
struct SqliteDb {
    db: *mut std::ffi::c_void,
    closed: bool,
}

#[repr(C)]
struct SqliteStmt {
    stmt: *mut std::ffi::c_void,
    db: *mut std::ffi::c_void, // borrowed ref to db — for error messages
    closed: bool,
}

#[repr(C)]
struct RowsState {
    stmt: *mut std::ffi::c_void,
    done: bool,
}

// ─── Helper functions ─────────────────────────────────────────────────────────

/// Validate that the value at stack index 1 is an open database.  Errors on
/// misuse (wrong type or closed); returns the sqlite3* on success.
unsafe fn checkdb(state: *mut State) -> *mut std::ffi::c_void {
    unsafe {
        let p = lual_checkudata(state, 1, DB_META) as *mut SqliteDb;
        if (*p).closed {
            lual_error(state, c"attempt to use a closed database".as_ptr(), &[]);
            unreachable!()
        }
        (*p).db
    }
}

/// Same for a statement at index 1.  Returns (stmt*, db*).
unsafe fn checkstmt(state: *mut State) -> (*mut std::ffi::c_void, *mut std::ffi::c_void) {
    unsafe {
        let p = lual_checkudata(state, 1, STMT_META) as *mut SqliteStmt;
        if (*p).closed {
            lual_error(state, c"attempt to use a finalized statement".as_ptr(), &[]);
            unreachable!()
        }
        ((*p).stmt, (*p).db)
    }
}

/// Push the sqlite3_errmsg string for `db` onto the Lua stack.
unsafe fn push_db_errmsg(state: *mut State, db: *mut std::ffi::c_void) {
    unsafe {
        let msg = sqlite3_errmsg(db);
        if msg.is_null() {
            lua_pushstring(state, c"unknown sqlite3 error".as_ptr());
        } else {
            lua_pushstring(state, msg);
        }
    }
}

/// Bind the Lua value at stack position `arg` to parameter `i` (1-based).
unsafe fn bind_value(state: *mut State, stmt: *mut std::ffi::c_void, i: std::ffi::c_int, arg: i32) -> std::ffi::c_int {
    unsafe {
        match lua_type(state, arg) {
            | None | Some(TagType::Nil) => sqlite3_bind_null(stmt, i),
            | Some(TagType::Boolean) => sqlite3_bind_int64(stmt, i, lua_toboolean(state, arg) as i64),
            | Some(TagType::Numeric) => {
                if lua_isinteger(state, arg) {
                    let v = lua_tointegerx(state, arg, null_mut());
                    sqlite3_bind_int64(stmt, i, v)
                } else {
                    let v = lua_tonumberx(state, arg, null_mut());
                    sqlite3_bind_double(stmt, i, v)
                }
            },
            | Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(state, arg, &mut slen);
                sqlite3_bind_text(stmt, i, sptr, slen as std::ffi::c_int, SQLITE_TRANSIENT)
            },
            | _ => sqlite3_bind_null(stmt, i),
        }
    }
}

/// Push the value of column `icol` (0-based) of the current row onto the stack.
unsafe fn push_column(state: *mut State, stmt: *mut std::ffi::c_void, icol: std::ffi::c_int) {
    unsafe {
        match sqlite3_column_type(stmt, icol) {
            SQLITE3_COL_INTEGER => {
                (*state).push_integer(sqlite3_column_int64(stmt, icol));
            }
            SQLITE3_COL_FLOAT => {
                (*state).push_number(sqlite3_column_double(stmt, icol));
            }
            SQLITE3_COL_TEXT | SQLITE3_COL_BLOB => {
                let ptr = sqlite3_column_text(stmt, icol);
                let len = sqlite3_column_bytes(stmt, icol) as usize;
                lua_pushlstring(state, ptr as *const i8, len);
            }
            _ /* NULL */ => {
                (*state).push_nil();
            }
        }
    }
}

/// Push a table `{colname = value, ...}` for the current statement row.
unsafe fn push_row_table(state: *mut State, stmt: *mut std::ffi::c_void) {
    unsafe {
        let ncols = sqlite3_column_count(stmt);
        (*state).lua_createtable();
        for i in 0..ncols {
            let name = sqlite3_column_name(stmt, i);
            let name_len = cstr_len(name);
            lua_pushlstring(state, name, name_len); // key
            push_column(state, stmt, i); // value
            lua_rawset(state, -3);
        }
    }
}

// ─── Database __gc / __close ──────────────────────────────────────────────────

pub unsafe fn sqlite_db_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, DB_META) as *mut SqliteDb;
        if !(*p).closed && !(*p).db.is_null() {
            sqlite3_close((*p).db);
            (*p).db = null_mut();
            (*p).closed = true;
        }
        0
    }
}

// ─── Statement __gc / __close ─────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, STMT_META) as *mut SqliteStmt;
        if !(*p).closed && !(*p).stmt.is_null() {
            sqlite3_finalize((*p).stmt);
            (*p).stmt = null_mut();
            (*p).closed = true;
        }
        0
    }
}

// ─── Rows-state __gc ──────────────────────────────────────────────────────────

pub unsafe fn rows_state_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, ROWS_META) as *mut RowsState;
        if !(*p).done && !(*p).stmt.is_null() {
            sqlite3_finalize((*p).stmt);
            (*p).stmt = null_mut();
            (*p).done = true;
        }
        0
    }
}

// ─── Rows iterator function ───────────────────────────────────────────────────

// Upvalue 1: RowsState userdata
pub unsafe fn rows_iter(state: *mut State) -> i32 {
    unsafe {
        lua_pushvalue(state, UPVALUE1);
        let p = lual_checkudata(state, -1, ROWS_META) as *mut RowsState;
        lua_settop(state, -2); // pop state copy

        if (*p).done || (*p).stmt.is_null() {
            (*state).push_nil();
            return 1;
        }

        match sqlite3_step((*p).stmt) {
            | SQLITE_ROW => {
                push_row_table(state, (*p).stmt);
                1
            },
            | _ => {
                // SQLITE_DONE or an error — either way we are finished
                sqlite3_finalize((*p).stmt);
                (*p).stmt = null_mut();
                (*p).done = true;
                (*state).push_nil();
                1
            },
        }
    }
}

// ─── sqlite.open(path [, mode]) ───────────────────────────────────────────────
// mode: "ro" | "rw" | "rwc" (default "rwc")

pub unsafe fn sqlite_open(state: *mut State) -> i32 {
    unsafe {
        let pathptr = lual_checklstring(state, 1, null_mut());

        let flags = if lua_type(state, 2) == Some(TagType::String) {
            let mut mlen = 0usize;
            let mptr = lua_tolstring(state, 2, &mut mlen);
            let mode = std::slice::from_raw_parts(mptr as *const u8, mlen);
            match mode {
                | b"ro" => SQLITE_OPEN_READONLY | SQLITE_OPEN_URI,
                | b"rw" => SQLITE_OPEN_READWRITE | SQLITE_OPEN_URI,
                | b"rwc" => SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI,
                | _ => SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI,
            }
        } else {
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI
        };

        let mut db: *mut std::ffi::c_void = null_mut();
        let rc = sqlite3_open_v2(pathptr, &mut db, flags, null());

        if rc != SQLITE_OK {
            (*state).push_nil();
            if !db.is_null() {
                push_db_errmsg(state, db);
                sqlite3_close(db);
            } else {
                lua_pushstring(state, c"sqlite3_open failed".as_ptr());
            }
            return 2;
        }

        let user_data = User::lua_newuserdatauv(state, size_of::<SqliteDb>(), 0) as *mut SqliteDb;
        (*user_data).db = db;
        (*user_data).closed = false;
        lual_setmetatable(state, DB_META);
        1
    }
}

// ─── db:close() ───────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_close(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, DB_META) as *mut SqliteDb;
        if !(*p).closed && !(*p).db.is_null() {
            let rc = sqlite3_close((*p).db);
            if rc != SQLITE_OK {
                (*state).push_nil();
                push_db_errmsg(state, (*p).db);
                return 2;
            }
            (*p).db = null_mut();
            (*p).closed = true;
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── db:exec(sql) ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_exec(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);
        let sql = lual_checklstring(state, 2, null_mut());

        let mut errmsg: *mut std::ffi::c_char = null_mut();
        let rc = sqlite3_exec(db, sql, null_mut(), null_mut(), &mut errmsg);

        if rc != SQLITE_OK {
            (*state).push_nil();
            if !errmsg.is_null() {
                lua_pushstring(state, errmsg);
                sqlite3_free(errmsg as *mut std::ffi::c_void);
            } else {
                push_db_errmsg(state, db);
            }
            return 2;
        }

        (*state).push_boolean(true);
        1
    }
}

// ─── db:query(sql [, ...]) ────────────────────────────────────────────────────
// Returns an array of row tables: { {col=val, ...}, ... }

pub unsafe fn sqlite_db_query(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);

        let mut slen = 0usize;
        let sptr = lual_checklstring(state, 2, &mut slen);

        let mut stmt: *mut std::ffi::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as std::ffi::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*state).push_nil();
            push_db_errmsg(state, db);
            return 2;
        }

        // Bind optional extra args (positions 3, 4, …)
        let nargs = (*state).get_top();
        for i in 3..=nargs {
            bind_value(state, stmt, (i - 2) as std::ffi::c_int, i);
        }

        // Collect all rows into a Lua array
        (*state).lua_createtable();
        let mut row_idx = 1i64;
        loop {
            match sqlite3_step(stmt) {
                | SQLITE_ROW => {
                    (*state).push_integer(row_idx); // key
                    push_row_table(state, stmt); // value
                    lua_rawset(state, -3);
                    row_idx += 1;
                },
                | SQLITE_DONE => break,
                | _ => {
                    // Error mid-iteration: pop table, return nil + errmsg
                    lua_settop(state, -2);
                    sqlite3_finalize(stmt);
                    (*state).push_nil();
                    push_db_errmsg(state, db);
                    return 2;
                },
            }
        }

        sqlite3_finalize(stmt);
        1
    }
}

// ─── db:rows(sql [, ...]) ─────────────────────────────────────────────────────
// Returns an iterator function that yields one row table per call, then nil.

pub unsafe fn sqlite_db_rows(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);

        let mut slen = 0usize;
        let sptr = lual_checklstring(state, 2, &mut slen);

        let mut stmt: *mut std::ffi::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as std::ffi::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*state).push_nil();
            push_db_errmsg(state, db);
            return 2;
        }

        // Bind optional extra args
        let nargs = (*state).get_top();
        for i in 3..=nargs {
            bind_value(state, stmt, (i - 2) as std::ffi::c_int, i);
        }

        // Allocate a RowsState userdata as the closure upvalue
        let state2 = User::lua_newuserdatauv(state, size_of::<RowsState>(), 0) as *mut RowsState;
        (*state2).stmt = stmt;
        (*state2).done = false;
        lual_setmetatable(state, ROWS_META);

        // Push iterator closure with the state as upvalue 1
        lua_pushcclosure(state, Some(rows_iter as unsafe fn(*mut State) -> i32), 1);
        1
    }
}

// ─── db:prepare(sql) ──────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_prepare(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);

        let mut slen = 0usize;
        let sptr = lual_checklstring(state, 2, &mut slen);

        let mut stmt: *mut std::ffi::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as std::ffi::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*state).push_nil();
            push_db_errmsg(state, db);
            return 2;
        }

        let user_data = User::lua_newuserdatauv(state, size_of::<SqliteStmt>(), 0) as *mut SqliteStmt;
        (*user_data).stmt = stmt;
        (*user_data).db = db; // borrowed — must not outlive the db
        (*user_data).closed = false;
        lual_setmetatable(state, STMT_META);
        1
    }
}

// ─── db:last_insert_rowid() ───────────────────────────────────────────────────

pub unsafe fn sqlite_db_last_insert_rowid(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);
        (*state).push_integer(sqlite3_last_insert_rowid(db));
        1
    }
}

// ─── db:changes() ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_changes(state: *mut State) -> i32 {
    unsafe {
        let db = checkdb(state);
        (*state).push_integer(sqlite3_changes(db) as i64);
        1
    }
}

// ─── stmt:bind(i, value) ──────────────────────────────────────────────────────
// Bind parameter i (1-based) to a Lua value.

pub unsafe fn sqlite_stmt_bind(state: *mut State) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(state);
        let idx = lual_checkinteger(state, 2) as std::ffi::c_int;
        let rc = bind_value(state, stmt, idx, 3);
        if rc != SQLITE_OK {
            (*state).push_nil();
            push_db_errmsg(state, db);
            return 2;
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── stmt:bind_values(...) ────────────────────────────────────────────────────
// Bind all parameters in order from arg 2 onward.

pub unsafe fn sqlite_stmt_bind_values(state: *mut State) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(state);
        let nargs = (*state).get_top();
        for i in 2..=nargs {
            let param = (i - 1) as std::ffi::c_int;
            let rc = bind_value(state, stmt, param, i);
            if rc != SQLITE_OK {
                (*state).push_nil();
                push_db_errmsg(state, db);
                return 2;
            }
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── stmt:step() ──────────────────────────────────────────────────────────────
// Returns "row" if a row is available, "done" when finished, or nil + errmsg.

pub unsafe fn sqlite_stmt_step(state: *mut State) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(state);
        match sqlite3_step(stmt) {
            | SQLITE_ROW => {
                lua_pushstring(state, c"row".as_ptr());
                1
            },
            | SQLITE_DONE => {
                lua_pushstring(state, c"done".as_ptr());
                1
            },
            | _ => {
                (*state).push_nil();
                push_db_errmsg(state, db);
                2
            },
        }
    }
}

// ─── stmt:get_row() ───────────────────────────────────────────────────────────
// Returns the current row as a table {colname = value, ...}.
// Call after step() returns "row".

pub unsafe fn sqlite_stmt_get_row(state: *mut State) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(state);
        push_row_table(state, stmt);
        1
    }
}

// ─── stmt:columns() ───────────────────────────────────────────────────────────
// Returns an array of column names: {"col1", "col2", ...}

pub unsafe fn sqlite_stmt_columns(state: *mut State) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(state);
        let ncols = sqlite3_column_count(stmt);
        (*state).lua_createtable();
        for i in 0..ncols {
            let name = sqlite3_column_name(stmt, i);
            let name_len = cstr_len(name);
            (*state).push_integer((i + 1) as i64); // 1-based key
            lua_pushlstring(state, name, name_len); // value
            lua_rawset(state, -3);
        }
        1
    }
}

// ─── stmt:reset() ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_reset(state: *mut State) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(state);
        sqlite3_reset(stmt);
        (*state).push_boolean(true);
        1
    }
}

// ─── stmt:finalize() ──────────────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_finalize(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, STMT_META) as *mut SqliteStmt;
        if !(*p).closed && !(*p).stmt.is_null() {
            sqlite3_finalize((*p).stmt);
            (*p).stmt = null_mut();
            (*p).closed = true;
        }
        (*state).push_boolean(true);
        1
    }
}

// ─── Registration tables ──────────────────────────────────────────────────────

pub const SQLITE_DB_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(sqlite_db_close as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"exec".as_ptr(),
        registeredfunction_function: Some(sqlite_db_exec as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"query".as_ptr(),
        registeredfunction_function: Some(sqlite_db_query as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"rows".as_ptr(),
        registeredfunction_function: Some(sqlite_db_rows as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"prepare".as_ptr(),
        registeredfunction_function: Some(sqlite_db_prepare as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"last_insert_rowid".as_ptr(),
        registeredfunction_function: Some(sqlite_db_last_insert_rowid as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"changes".as_ptr(),
        registeredfunction_function: Some(sqlite_db_changes as unsafe fn(*mut State) -> i32),
    },
];

pub const SQLITE_DB_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(sqlite_db_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(sqlite_db_gc as unsafe fn(*mut State) -> i32),
    },
];

pub const SQLITE_STMT_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"bind".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_bind as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"bind_values".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_bind_values as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"step".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_step as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"get_row".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_get_row as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"columns".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_columns as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"reset".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_reset as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"finalize".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_finalize as unsafe fn(*mut State) -> i32),
    },
];

pub const SQLITE_STMT_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_gc as unsafe fn(*mut State) -> i32),
    },
];

pub const SQLITE_ROWS_META: [RegisteredFunction; 1] = [RegisteredFunction {
    registeredfunction_name: c"__gc".as_ptr(),
    registeredfunction_function: Some(rows_state_gc as unsafe fn(*mut State) -> i32),
}];

// ─── Library open ─────────────────────────────────────────────────────────────

pub unsafe fn luaopen_sqlite(state: *mut State) -> i32 {
    unsafe {
        // Database metatable
        lual_newmetatable(state, DB_META);
        lual_setfuncs(state, SQLITE_DB_META.as_ptr(), SQLITE_DB_META.len(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, SQLITE_DB_METHODS.as_ptr(), SQLITE_DB_METHODS.len(), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2); // pop db metatable

        // Statement metatable
        lual_newmetatable(state, STMT_META);
        lual_setfuncs(state, SQLITE_STMT_META.as_ptr(), SQLITE_STMT_META.len(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, SQLITE_STMT_METHODS.as_ptr(), SQLITE_STMT_METHODS.len(), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2); // pop stmt metatable

        // Rows-state metatable (only needs __gc for statement cleanup on GC)
        lual_newmetatable(state, ROWS_META);
        lual_setfuncs(state, SQLITE_ROWS_META.as_ptr(), SQLITE_ROWS_META.len(), 0);
        lua_settop(state, -2); // pop rows metatable

        // Library table: { open = sqlite_open }
        (*state).lua_createtable();
        lua_pushcclosure(state, Some(sqlite_open as unsafe fn(*mut State) -> i32), 0);
        lua_setfield(state, -2, c"open".as_ptr());
        1
    }
}

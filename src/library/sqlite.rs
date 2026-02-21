use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tagtype::*;
use crate::user::*;
use std::mem::size_of;
use std::ptr::*;

// lua_upvalueindex(1)
const UPVALUE1: i32 = -(1000000 + 1000) - 1;

// ─── SQLite3 FFI ──────────────────────────────────────────────────────────────

#[link(name = "sqlite3")]
unsafe extern "C" {
    fn sqlite3_open_v2(
        filename: *const libc::c_char,
        ppdb: *mut *mut libc::c_void,
        flags: libc::c_int,
        zvfs: *const libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_close(db: *mut libc::c_void) -> libc::c_int;
    fn sqlite3_errmsg(db: *mut libc::c_void) -> *const libc::c_char;
    fn sqlite3_exec(
        db: *mut libc::c_void,
        sql: *const libc::c_char,
        callback: *mut libc::c_void,
        arg: *mut libc::c_void,
        errmsg: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_free(p: *mut libc::c_void);
    fn sqlite3_prepare_v2(
        db: *mut libc::c_void,
        zsql: *const libc::c_char,
        nbyte: libc::c_int,
        ppstmt: *mut *mut libc::c_void,
        pztail: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_step(pstmt: *mut libc::c_void) -> libc::c_int;
    fn sqlite3_reset(pstmt: *mut libc::c_void) -> libc::c_int;
    fn sqlite3_finalize(pstmt: *mut libc::c_void) -> libc::c_int;
    fn sqlite3_column_count(pstmt: *mut libc::c_void) -> libc::c_int;
    fn sqlite3_column_type(pstmt: *mut libc::c_void, icol: libc::c_int) -> libc::c_int;
    fn sqlite3_column_name(pstmt: *mut libc::c_void, icol: libc::c_int) -> *const libc::c_char;
    fn sqlite3_column_int64(pstmt: *mut libc::c_void, icol: libc::c_int) -> i64;
    fn sqlite3_column_double(pstmt: *mut libc::c_void, icol: libc::c_int) -> f64;
    fn sqlite3_column_text(pstmt: *mut libc::c_void, icol: libc::c_int) -> *const libc::c_uchar;
    fn sqlite3_column_bytes(pstmt: *mut libc::c_void, icol: libc::c_int) -> libc::c_int;
    fn sqlite3_bind_null(pstmt: *mut libc::c_void, i: libc::c_int) -> libc::c_int;
    fn sqlite3_bind_int64(pstmt: *mut libc::c_void, i: libc::c_int, v: i64) -> libc::c_int;
    fn sqlite3_bind_double(pstmt: *mut libc::c_void, i: libc::c_int, v: f64) -> libc::c_int;
    fn sqlite3_bind_text(
        pstmt: *mut libc::c_void,
        i: libc::c_int,
        z: *const libc::c_char,
        n: libc::c_int,
        destructor: *const libc::c_void,
    ) -> libc::c_int;
    fn sqlite3_last_insert_rowid(db: *mut libc::c_void) -> i64;
    fn sqlite3_changes(db: *mut libc::c_void) -> libc::c_int;
}

const SQLITE_OK: libc::c_int = 0;
const SQLITE_ROW: libc::c_int = 100;
const SQLITE_DONE: libc::c_int = 101;

const SQLITE_OPEN_READONLY: libc::c_int  = 0x00000001;
const SQLITE_OPEN_READWRITE: libc::c_int = 0x00000002;
const SQLITE_OPEN_CREATE: libc::c_int    = 0x00000004;
const SQLITE_OPEN_URI: libc::c_int       = 0x00000040;

const SQLITE3_COL_INTEGER: libc::c_int = 1;
const SQLITE3_COL_FLOAT: libc::c_int   = 2;
const SQLITE3_COL_TEXT: libc::c_int    = 3;
const SQLITE3_COL_BLOB: libc::c_int    = 4;
// const SQLITE3_COL_NULL: libc::c_int  = 5;

// SQLITE_TRANSIENT: SQLite will copy the data before sqlite3_bind_text returns.
const SQLITE_TRANSIENT: *const libc::c_void = usize::MAX as *const libc::c_void;

const DB_META: *const i8   = c"sqlite3*".as_ptr();
const STMT_META: *const i8 = c"sqlite3_stmt*".as_ptr();
const ROWS_META: *const i8 = c"sqlite3_rows*".as_ptr();

// ─── Userdata structs ─────────────────────────────────────────────────────────

#[repr(C)]
struct SqliteDb {
    db: *mut libc::c_void,
    closed: bool,
}

#[repr(C)]
struct SqliteStmt {
    stmt: *mut libc::c_void,
    db: *mut libc::c_void, // borrowed ref to db — for error messages
    closed: bool,
}

#[repr(C)]
struct RowsState {
    stmt: *mut libc::c_void,
    done: bool,
}

// ─── Helper functions ─────────────────────────────────────────────────────────

/// Validate that the value at stack index 1 is an open database.  Errors on
/// misuse (wrong type or closed); returns the sqlite3* on success.
unsafe fn checkdb(interpreter: *mut Interpreter) -> *mut libc::c_void {
    unsafe {
        let p = lual_checkudata(interpreter, 1, DB_META) as *mut SqliteDb;
        if (*p).closed {
            lual_error(interpreter, c"attempt to use a closed database".as_ptr());
            unreachable!()
        }
        (*p).db
    }
}

/// Same for a statement at index 1.  Returns (stmt*, db*).
unsafe fn checkstmt(interpreter: *mut Interpreter) -> (*mut libc::c_void, *mut libc::c_void) {
    unsafe {
        let p = lual_checkudata(interpreter, 1, STMT_META) as *mut SqliteStmt;
        if (*p).closed {
            lual_error(interpreter, c"attempt to use a finalized statement".as_ptr());
            unreachable!()
        }
        ((*p).stmt, (*p).db)
    }
}

/// Push the sqlite3_errmsg string for `db` onto the Lua stack.
unsafe fn push_db_errmsg(interpreter: *mut Interpreter, db: *mut libc::c_void) {
    unsafe {
        let msg = sqlite3_errmsg(db);
        if msg.is_null() {
            lua_pushstring(interpreter, c"unknown sqlite3 error".as_ptr());
        } else {
            lua_pushstring(interpreter, msg);
        }
    }
}

/// Bind the Lua value at stack position `arg` to parameter `i` (1-based).
unsafe fn bind_value(
    interpreter: *mut Interpreter,
    stmt: *mut libc::c_void,
    i: libc::c_int,
    arg: i32,
) -> libc::c_int {
    unsafe {
        match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => sqlite3_bind_null(stmt, i),
            Some(TagType::Boolean) => {
                sqlite3_bind_int64(stmt, i, lua_toboolean(interpreter, arg) as i64)
            }
            Some(TagType::Numeric) => {
                if lua_isinteger(interpreter, arg) {
                    let v = lua_tointegerx(interpreter, arg, null_mut());
                    sqlite3_bind_int64(stmt, i, v)
                } else {
                    let v = lua_tonumberx(interpreter, arg, null_mut());
                    sqlite3_bind_double(stmt, i, v)
                }
            }
            Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(interpreter, arg, &mut slen);
                sqlite3_bind_text(stmt, i, sptr, slen as libc::c_int, SQLITE_TRANSIENT)
            }
            _ => sqlite3_bind_null(stmt, i),
        }
    }
}

/// Push the value of column `icol` (0-based) of the current row onto the stack.
unsafe fn push_column(interpreter: *mut Interpreter, stmt: *mut libc::c_void, icol: libc::c_int) {
    unsafe {
        match sqlite3_column_type(stmt, icol) {
            SQLITE3_COL_INTEGER => {
                (*interpreter).push_integer(sqlite3_column_int64(stmt, icol));
            }
            SQLITE3_COL_FLOAT => {
                (*interpreter).push_number(sqlite3_column_double(stmt, icol));
            }
            SQLITE3_COL_TEXT | SQLITE3_COL_BLOB => {
                let ptr = sqlite3_column_text(stmt, icol);
                let len = sqlite3_column_bytes(stmt, icol) as usize;
                lua_pushlstring(interpreter, ptr as *const i8, len);
            }
            _ /* NULL */ => {
                (*interpreter).push_nil();
            }
        }
    }
}

/// Push a table `{colname = value, ...}` for the current statement row.
unsafe fn push_row_table(interpreter: *mut Interpreter, stmt: *mut libc::c_void) {
    unsafe {
        let ncols = sqlite3_column_count(stmt);
        (*interpreter).lua_createtable();
        for i in 0..ncols {
            let name = sqlite3_column_name(stmt, i);
            let name_len = libc::strlen(name);
            lua_pushlstring(interpreter, name, name_len); // key
            push_column(interpreter, stmt, i);            // value
            lua_rawset(interpreter, -3);
        }
    }
}

// ─── Database __gc / __close ──────────────────────────────────────────────────

pub unsafe fn sqlite_db_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, DB_META) as *mut SqliteDb;
        if !(*p).closed && !(*p).db.is_null() {
            sqlite3_close((*p).db);
            (*p).db = null_mut();
            (*p).closed = true;
        }
        0
    }
}

// ─── Statement __gc / __close ─────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, STMT_META) as *mut SqliteStmt;
        if !(*p).closed && !(*p).stmt.is_null() {
            sqlite3_finalize((*p).stmt);
            (*p).stmt = null_mut();
            (*p).closed = true;
        }
        0
    }
}

// ─── Rows-state __gc ──────────────────────────────────────────────────────────

pub unsafe fn rows_state_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, ROWS_META) as *mut RowsState;
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
pub unsafe fn rows_iter(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushvalue(interpreter, UPVALUE1);
        let p = lual_checkudata(interpreter, -1, ROWS_META) as *mut RowsState;
        lua_settop(interpreter, -2); // pop state copy

        if (*p).done || (*p).stmt.is_null() {
            (*interpreter).push_nil();
            return 1;
        }

        match sqlite3_step((*p).stmt) {
            SQLITE_ROW => {
                push_row_table(interpreter, (*p).stmt);
                1
            }
            _ => {
                // SQLITE_DONE or an error — either way we are finished
                sqlite3_finalize((*p).stmt);
                (*p).stmt = null_mut();
                (*p).done = true;
                (*interpreter).push_nil();
                1
            }
        }
    }
}

// ─── sqlite.open(path [, mode]) ───────────────────────────────────────────────
// mode: "ro" | "rw" | "rwc" (default "rwc")

pub unsafe fn sqlite_open(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let pathptr = lual_checklstring(interpreter, 1, null_mut());

        let flags = if lua_type(interpreter, 2) == Some(TagType::String) {
            let mut mlen = 0usize;
            let mptr = lua_tolstring(interpreter, 2, &mut mlen);
            let mode = std::slice::from_raw_parts(mptr as *const u8, mlen);
            match mode {
                b"ro"  => SQLITE_OPEN_READONLY | SQLITE_OPEN_URI,
                b"rw"  => SQLITE_OPEN_READWRITE | SQLITE_OPEN_URI,
                b"rwc" => SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI,
                _      => SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI,
            }
        } else {
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI
        };

        let mut db: *mut libc::c_void = null_mut();
        let rc = sqlite3_open_v2(pathptr, &mut db, flags, null());

        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            if !db.is_null() {
                push_db_errmsg(interpreter, db);
                sqlite3_close(db);
            } else {
                lua_pushstring(interpreter, c"sqlite3_open failed".as_ptr());
            }
            return 2;
        }

        let ud = User::lua_newuserdatauv(interpreter, size_of::<SqliteDb>(), 0) as *mut SqliteDb;
        (*ud).db = db;
        (*ud).closed = false;
        lual_setmetatable(interpreter, DB_META);
        1
    }
}

// ─── db:close() ───────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, DB_META) as *mut SqliteDb;
        if !(*p).closed && !(*p).db.is_null() {
            let rc = sqlite3_close((*p).db);
            if rc != SQLITE_OK {
                (*interpreter).push_nil();
                push_db_errmsg(interpreter, (*p).db);
                return 2;
            }
            (*p).db = null_mut();
            (*p).closed = true;
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── db:exec(sql) ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_exec(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);
        let sql = lual_checklstring(interpreter, 2, null_mut());

        let mut errmsg: *mut libc::c_char = null_mut();
        let rc = sqlite3_exec(db, sql, null_mut(), null_mut(), &mut errmsg);

        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            if !errmsg.is_null() {
                lua_pushstring(interpreter, errmsg);
                sqlite3_free(errmsg as *mut libc::c_void);
            } else {
                push_db_errmsg(interpreter, db);
            }
            return 2;
        }

        (*interpreter).push_boolean(true);
        1
    }
}

// ─── db:query(sql [, ...]) ────────────────────────────────────────────────────
// Returns an array of row tables: { {col=val, ...}, ... }

pub unsafe fn sqlite_db_query(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);

        let mut slen = 0usize;
        let sptr = lual_checklstring(interpreter, 2, &mut slen);

        let mut stmt: *mut libc::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as libc::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            push_db_errmsg(interpreter, db);
            return 2;
        }

        // Bind optional extra args (positions 3, 4, …)
        let nargs = (*interpreter).get_top();
        for i in 3..=nargs {
            bind_value(interpreter, stmt, (i - 2) as libc::c_int, i);
        }

        // Collect all rows into a Lua array
        (*interpreter).lua_createtable();
        let mut row_idx = 1i64;
        loop {
            match sqlite3_step(stmt) {
                SQLITE_ROW => {
                    (*interpreter).push_integer(row_idx); // key
                    push_row_table(interpreter, stmt);    // value
                    lua_rawset(interpreter, -3);
                    row_idx += 1;
                }
                SQLITE_DONE => break,
                _ => {
                    // Error mid-iteration: pop table, return nil + errmsg
                    lua_settop(interpreter, -2);
                    sqlite3_finalize(stmt);
                    (*interpreter).push_nil();
                    push_db_errmsg(interpreter, db);
                    return 2;
                }
            }
        }

        sqlite3_finalize(stmt);
        1
    }
}

// ─── db:rows(sql [, ...]) ─────────────────────────────────────────────────────
// Returns an iterator function that yields one row table per call, then nil.

pub unsafe fn sqlite_db_rows(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);

        let mut slen = 0usize;
        let sptr = lual_checklstring(interpreter, 2, &mut slen);

        let mut stmt: *mut libc::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as libc::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            push_db_errmsg(interpreter, db);
            return 2;
        }

        // Bind optional extra args
        let nargs = (*interpreter).get_top();
        for i in 3..=nargs {
            bind_value(interpreter, stmt, (i - 2) as libc::c_int, i);
        }

        // Allocate a RowsState userdata as the closure upvalue
        let state = User::lua_newuserdatauv(interpreter, size_of::<RowsState>(), 0) as *mut RowsState;
        (*state).stmt = stmt;
        (*state).done = false;
        lual_setmetatable(interpreter, ROWS_META);

        // Push iterator closure with the state as upvalue 1
        lua_pushcclosure(interpreter, Some(rows_iter as unsafe fn(*mut Interpreter) -> i32), 1);
        1
    }
}

// ─── db:prepare(sql) ──────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_prepare(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);

        let mut slen = 0usize;
        let sptr = lual_checklstring(interpreter, 2, &mut slen);

        let mut stmt: *mut libc::c_void = null_mut();
        let rc = sqlite3_prepare_v2(db, sptr, slen as libc::c_int, &mut stmt, null_mut());
        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            push_db_errmsg(interpreter, db);
            return 2;
        }

        let ud = User::lua_newuserdatauv(interpreter, size_of::<SqliteStmt>(), 0) as *mut SqliteStmt;
        (*ud).stmt = stmt;
        (*ud).db = db; // borrowed — must not outlive the db
        (*ud).closed = false;
        lual_setmetatable(interpreter, STMT_META);
        1
    }
}

// ─── db:last_insert_rowid() ───────────────────────────────────────────────────

pub unsafe fn sqlite_db_last_insert_rowid(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);
        (*interpreter).push_integer(sqlite3_last_insert_rowid(db));
        1
    }
}

// ─── db:changes() ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_db_changes(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let db = checkdb(interpreter);
        (*interpreter).push_integer(sqlite3_changes(db) as i64);
        1
    }
}

// ─── stmt:bind(i, value) ──────────────────────────────────────────────────────
// Bind parameter i (1-based) to a Lua value.

pub unsafe fn sqlite_stmt_bind(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(interpreter);
        let idx = lual_checkinteger(interpreter, 2) as libc::c_int;
        let rc = bind_value(interpreter, stmt, idx, 3);
        if rc != SQLITE_OK {
            (*interpreter).push_nil();
            push_db_errmsg(interpreter, db);
            return 2;
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── stmt:bind_values(...) ────────────────────────────────────────────────────
// Bind all parameters in order from arg 2 onward.

pub unsafe fn sqlite_stmt_bind_values(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(interpreter);
        let nargs = (*interpreter).get_top();
        for i in 2..=nargs {
            let param = (i - 1) as libc::c_int;
            let rc = bind_value(interpreter, stmt, param, i);
            if rc != SQLITE_OK {
                (*interpreter).push_nil();
                push_db_errmsg(interpreter, db);
                return 2;
            }
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── stmt:step() ──────────────────────────────────────────────────────────────
// Returns "row" if a row is available, "done" when finished, or nil + errmsg.

pub unsafe fn sqlite_stmt_step(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, db) = checkstmt(interpreter);
        match sqlite3_step(stmt) {
            SQLITE_ROW  => { lua_pushstring(interpreter, c"row".as_ptr());  1 }
            SQLITE_DONE => { lua_pushstring(interpreter, c"done".as_ptr()); 1 }
            _ => {
                (*interpreter).push_nil();
                push_db_errmsg(interpreter, db);
                2
            }
        }
    }
}

// ─── stmt:get_row() ───────────────────────────────────────────────────────────
// Returns the current row as a table {colname = value, ...}.
// Call after step() returns "row".

pub unsafe fn sqlite_stmt_get_row(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(interpreter);
        push_row_table(interpreter, stmt);
        1
    }
}

// ─── stmt:columns() ───────────────────────────────────────────────────────────
// Returns an array of column names: {"col1", "col2", ...}

pub unsafe fn sqlite_stmt_columns(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(interpreter);
        let ncols = sqlite3_column_count(stmt);
        (*interpreter).lua_createtable();
        for i in 0..ncols {
            let name = sqlite3_column_name(stmt, i);
            let name_len = libc::strlen(name);
            (*interpreter).push_integer((i + 1) as i64);  // 1-based key
            lua_pushlstring(interpreter, name, name_len);  // value
            lua_rawset(interpreter, -3);
        }
        1
    }
}

// ─── stmt:reset() ─────────────────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_reset(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let (stmt, _db) = checkstmt(interpreter);
        sqlite3_reset(stmt);
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── stmt:finalize() ──────────────────────────────────────────────────────────

pub unsafe fn sqlite_stmt_finalize(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, STMT_META) as *mut SqliteStmt;
        if !(*p).closed && !(*p).stmt.is_null() {
            sqlite3_finalize((*p).stmt);
            (*p).stmt = null_mut();
            (*p).closed = true;
        }
        (*interpreter).push_boolean(true);
        1
    }
}

// ─── Registration tables ──────────────────────────────────────────────────────

pub const SQLITE_DB_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(sqlite_db_close as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"exec".as_ptr(),
        registeredfunction_function: Some(sqlite_db_exec as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"query".as_ptr(),
        registeredfunction_function: Some(sqlite_db_query as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"rows".as_ptr(),
        registeredfunction_function: Some(sqlite_db_rows as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"prepare".as_ptr(),
        registeredfunction_function: Some(sqlite_db_prepare as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"last_insert_rowid".as_ptr(),
        registeredfunction_function: Some(sqlite_db_last_insert_rowid as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"changes".as_ptr(),
        registeredfunction_function: Some(sqlite_db_changes as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const SQLITE_DB_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(sqlite_db_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(sqlite_db_gc as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const SQLITE_STMT_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"bind".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_bind as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"bind_values".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_bind_values as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"step".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_step as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"get_row".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_get_row as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"columns".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_columns as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"reset".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_reset as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"finalize".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_finalize as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const SQLITE_STMT_META: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(sqlite_stmt_gc as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const SQLITE_ROWS_META: [RegisteredFunction; 1] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(rows_state_gc as unsafe fn(*mut Interpreter) -> i32),
    },
];

// ─── Library open ─────────────────────────────────────────────────────────────

pub unsafe fn luaopen_sqlite(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        // Database metatable
        lual_newmetatable(interpreter, DB_META);
        lual_setfuncs(interpreter, SQLITE_DB_META.as_ptr(), SQLITE_DB_META.len(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, SQLITE_DB_METHODS.as_ptr(), SQLITE_DB_METHODS.len(), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2); // pop db metatable

        // Statement metatable
        lual_newmetatable(interpreter, STMT_META);
        lual_setfuncs(interpreter, SQLITE_STMT_META.as_ptr(), SQLITE_STMT_META.len(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, SQLITE_STMT_METHODS.as_ptr(), SQLITE_STMT_METHODS.len(), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2); // pop stmt metatable

        // Rows-state metatable (only needs __gc for statement cleanup on GC)
        lual_newmetatable(interpreter, ROWS_META);
        lual_setfuncs(interpreter, SQLITE_ROWS_META.as_ptr(), SQLITE_ROWS_META.len(), 0);
        lua_settop(interpreter, -2); // pop rows metatable

        // Library table: { open = sqlite_open }
        (*interpreter).lua_createtable();
        lua_pushcclosure(interpreter, Some(sqlite_open as unsafe fn(*mut Interpreter) -> i32), 0);
        lua_setfield(interpreter, -2, c"open".as_ptr());
        1
    }
}

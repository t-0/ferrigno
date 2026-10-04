//! Safe embedding API.
//!
//! [`Lua`] owns an interpreter state and exposes a small set of operations
//! that need no `unsafe` code on the caller's side: run source text or a
//! file, read and write globals, call Lua functions, and register Rust
//! closures as Lua functions. Values cross the boundary as owned
//! [`Value`]s, and every failure is reported as an [`Error`].
//!
//! ```
//! use ferrigno::api::{Lua, Value};
//!
//! let mut lua = Lua::new().unwrap();
//! lua.exec("greeting = 'hello'").unwrap();
//! assert_eq!(lua.get_global("greeting").unwrap(), Value::from("hello"));
//!
//! lua.register("double", |args| {
//!     let n = args.first().and_then(Value::as_integer).ok_or("expected an integer")?;
//!     Ok(vec![Value::Integer(n * 2)])
//! }).unwrap();
//! assert_eq!(lua.eval("return double(21)").unwrap(), vec![Value::Integer(42)]);
//! ```
//!
//! Every operation that can raise a Lua error runs under a protected call.
//! The interpreter aborts the process on an unprotected error, so this
//! module never touches the stack in a way that could raise outside one.

use crate::calls::CallS;
use crate::functionstate::LUA_REGISTRYINDEX;
use crate::library::lual_openlibs;
use crate::luastate::LuaState;
use crate::registeredfunction::RegisteredFunction;
use crate::state::*;
use crate::status::Status;
use crate::tagtype::TagType;
use std::ffi::{c_void, CString};
use std::fmt;
use std::ptr::{null, null_mut};

/// Index of the first upvalue of a C closure.
const UPVALUE1: i32 = LUA_REGISTRYINDEX - 1;
/// `LUA_MULTRET`: accept every result of a call.
const MULTIPLE_RESULTS: i32 = -1;
/// Metatable name for userdata that owns a boxed Rust closure.
const RUST_FUNCTION_METATABLE: &std::ffi::CStr = c"_FERRIGNO_RUST_FUNCTION";

// ─── Value ──────────────────────────────────────────────────────────────────

/// An owned snapshot of a Lua value.
///
/// Tables are copied eagerly as key/value pairs. Functions, userdata and
/// threads cannot be moved out of the interpreter and are reported as
/// [`Value::Opaque`] with their Lua type name; pushing an `Opaque` back into
/// Lua yields `nil`.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    /// A Lua string holding valid UTF-8.
    String(String),
    /// A Lua string holding bytes that are not valid UTF-8.
    Bytes(Vec<u8>),
    /// A table, as key/value pairs in iteration order.
    Table(Vec<(Value, Value)>),
    /// A function, userdata or thread, identified by its Lua type name.
    Opaque(String),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// The value as a float. Integers convert; nothing else does.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// The raw bytes of a `String` or `Bytes` value.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::String(s) => Some(s.as_bytes()),
            Value::Bytes(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn as_table(&self) -> Option<&[(Value, Value)]> {
        match self {
            Value::Table(t) => Some(t.as_slice()),
            _ => None,
        }
    }

    /// Looks a key up in a `Table` value.
    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.as_table()?.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Builds a `String` or `Bytes` value depending on whether the bytes are UTF-8.
    pub fn from_bytes(bytes: Vec<u8>) -> Value {
        match String::from_utf8(bytes) {
            Ok(s) => Value::String(s),
            Err(e) => Value::Bytes(e.into_bytes()),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value::Nil
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Boolean(b)
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Value {
        Value::Integer(i)
    }
}
impl From<i32> for Value {
    fn from(i: i32) -> Value {
        Value::Integer(i as i64)
    }
}
impl From<f64> for Value {
    fn from(n: f64) -> Value {
        Value::Number(n)
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::String(s.to_string())
    }
}
impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}
impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Value {
        Value::from_bytes(b)
    }
}
impl<T: Into<Value>> From<Vec<T>> for Value {
    /// A sequence: elements become values at integer keys starting at 1.
    fn from(items: Vec<T>) -> Value {
        Value::Table(
            items
                .into_iter()
                .enumerate()
                .map(|(i, v)| (Value::Integer(i as i64 + 1), v.into()))
                .collect(),
        )
    }
}

// ─── Error ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// An error raised while running Lua code.
    Runtime,
    /// The source text did not parse.
    Syntax,
    /// The interpreter ran out of memory.
    Memory,
    /// A file could not be opened or read.
    File,
    /// A value could not cross the Rust/Lua boundary.
    Conversion,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    error_kind: ErrorKind,
    error_message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Error {
        Error {
            error_kind: kind,
            error_message: message.into(),
        }
    }

    pub fn get_kind(&self) -> ErrorKind {
        self.error_kind
    }

    pub fn get_message(&self) -> &str {
        &self.error_message
    }

    fn from_status(status: Status, message: String) -> Error {
        let kind = match status {
            Status::RuntimeError => ErrorKind::Runtime,
            Status::SyntaxError => ErrorKind::Syntax,
            Status::MemoryError => ErrorKind::Memory,
            Status::FileError => ErrorKind::File,
            _ => ErrorKind::Other,
        };
        Error::new(kind, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// A Rust function callable from Lua.
///
/// It receives the call's arguments and returns the values to hand back, or
/// a message that is raised as a Lua error at the call site.
pub type Function = Box<dyn Fn(&[Value]) -> std::result::Result<Vec<Value>, String>>;

// ─── Lua ────────────────────────────────────────────────────────────────────

/// An owned Lua interpreter.
///
/// Dropping it closes the state, which runs every pending `__gc` finalizer,
/// including those that free registered Rust closures. It is neither `Send`
/// nor `Sync`.
pub struct Lua {
    lua_state: LuaState,
}

impl Lua {
    /// Creates an interpreter with every compiled-in library opened.
    pub fn new() -> Result<Lua> {
        crate::lexicalstate::ferrigno_extensions_init();
        // SAFETY: a fresh state is created and only this struct holds it.
        let lua_state = unsafe { LuaState::new() }.ok_or_else(|| Error::new(ErrorKind::Memory, "cannot create state: not enough memory"))?;
        let lua = Lua { lua_state };
        let state = lua.get_state();
        // SAFETY: opening libraries may raise, so it runs under pcall.
        unsafe {
            lua.protected(0, 0, |state| {
                lua_pushcclosure(state, Some(open_libraries as unsafe fn(*mut State) -> i32), 0);
            })?;
            lua_settop(state, 0);
        }
        Ok(lua)
    }

    /// The raw state, for callers who need the low-level API.
    pub fn get_state(&self) -> *mut State {
        self.lua_state.state()
    }

    /// Runs `source` as a chunk and discards its results.
    pub fn exec(&mut self, source: &str) -> Result<()> {
        self.eval(source).map(|_| ())
    }

    /// Runs `source` as a chunk and returns the values it returns.
    pub fn eval(&mut self, source: &str) -> Result<Vec<Value>> {
        let state = self.get_state();
        // SAFETY: the buffer outlives the load; the stack is reset afterwards.
        unsafe {
            let status = lual_loadbufferx(
                state,
                source.as_ptr() as *const i8,
                source.len(),
                c"=(eval)".as_ptr(),
                null(),
            );
            self.finish_load(status)
        }
    }

    /// Loads and runs a Lua source file.
    pub fn exec_file(&mut self, path: &str) -> Result<Vec<Value>> {
        let state = self.get_state();
        let path = c_string(path)?;
        // SAFETY: the path outlives the load; the stack is reset afterwards.
        unsafe {
            let status = lual_loadfilex(state, path.as_ptr(), null());
            self.finish_load(status)
        }
    }

    /// Reads a global variable.
    pub fn get_global(&mut self, name: &str) -> Result<Value> {
        let state = self.get_state();
        let name = c_string(name)?;
        // SAFETY: the name outlives the call, which runs under pcall.
        unsafe {
            self.protected(1, 1, |state| {
                lua_pushcclosure(state, Some(protected_getglobal as unsafe fn(*mut State) -> i32), 0);
                lua_pushlightuserdata(state, name.as_ptr() as *mut c_void);
            })?;
            let value = value_from_stack(state, -1);
            lua_settop(state, 0);
            value
        }
    }

    /// Assigns a global variable.
    pub fn set_global(&mut self, name: &str, value: impl Into<Value>) -> Result<()> {
        let state = self.get_state();
        let name = c_string(name)?;
        let value = value.into();
        // SAFETY: the name outlives the call, which runs under pcall.
        unsafe {
            lua_pushcclosure(state, Some(protected_setglobal as unsafe fn(*mut State) -> i32), 0);
            lua_pushlightuserdata(state, name.as_ptr() as *mut c_void);
            if let Err(e) = push_value(state, &value) {
                lua_settop(state, 0);
                return Err(e);
            }
            let status = self.pcall(2, 0);
            let result = self.finish_call(status).map(|_| ());
            lua_settop(state, 0);
            result
        }
    }

    /// Calls the global function `name` with `args` and returns its results.
    pub fn call(&mut self, name: &str, args: &[Value]) -> Result<Vec<Value>> {
        let state = self.get_state();
        let name = c_string(name)?;
        // SAFETY: the name outlives the call, which runs under pcall.
        unsafe {
            lua_pushcclosure(state, Some(protected_call as unsafe fn(*mut State) -> i32), 0);
            lua_pushlightuserdata(state, name.as_ptr() as *mut c_void);
            if lua_checkstack(state, args.len() as i32 + 2) == 0 {
                lua_settop(state, 0);
                return Err(Error::new(ErrorKind::Conversion, "too many arguments"));
            }
            for arg in args {
                if let Err(e) = push_value(state, arg) {
                    lua_settop(state, 0);
                    return Err(e);
                }
            }
            let status = self.pcall(args.len() as i32 + 1, MULTIPLE_RESULTS);
            let results = self.finish_call(status);
            lua_settop(state, 0);
            results
        }
    }

    /// Registers `function` as the global `name`.
    ///
    /// The closure is owned by the interpreter and freed when the function
    /// value is collected or the interpreter is dropped.
    pub fn register<F>(&mut self, name: &str, function: F) -> Result<()>
    where
        F: Fn(&[Value]) -> std::result::Result<Vec<Value>, String> + 'static,
    {
        let state = self.get_state();
        let name = c_string(name)?;
        let boxed: Function = Box::new(function);
        let raw: *mut Function = Box::into_raw(Box::new(boxed));
        // SAFETY: userdata creation and setglobal may raise, so both run
        // under pcall; the box is reclaimed by the userdata's `__gc`.
        unsafe {
            let result = self.protected(2, 0, |state| {
                lua_pushcclosure(state, Some(protected_register as unsafe fn(*mut State) -> i32), 0);
                lua_pushlightuserdata(state, name.as_ptr() as *mut c_void);
                lua_pushlightuserdata(state, raw as *mut c_void);
            });
            lua_settop(state, 0);
            if result.is_err() {
                // The userdata never took ownership, so drop the box here.
                drop(Box::from_raw(raw));
            }
            result
        }
    }

    // ─── internals ──────────────────────────────────────────────────────

    /// Pushes a function and its arguments via `setup`, then calls it under
    /// a message handler. On success the results are left on the stack.
    unsafe fn protected(&self, nargs: i32, nresults: i32, setup: impl FnOnce(*mut State)) -> Result<()> {
        unsafe {
            let state = self.get_state();
            setup(state);
            let status = self.pcall(nargs, nresults);
            if status == Status::OK {
                Ok(())
            } else {
                let error = pop_error(state, status);
                lua_settop(state, 0);
                Err(error)
            }
        }
    }

    /// `lua_pcall` with a traceback-producing message handler. Expects the
    /// function and `nargs` arguments on top of the stack.
    unsafe fn pcall(&self, nargs: i32, nresults: i32) -> Status {
        unsafe {
            let state = self.get_state();
            let base = (*state).get_top() - nargs;
            lua_pushcclosure(state, Some(msghandler as unsafe fn(*mut State) -> i32), 0);
            lua_rotate(state, base, 1);
            let status = CallS::api_call(state, nargs, nresults, base, 0, None);
            lua_rotate(state, base, -1);
            lua_settop(state, -2);
            status
        }
    }

    /// After a load: on success, runs the chunk; either way returns the
    /// outcome and resets the stack.
    unsafe fn finish_load(&self, status: Status) -> Result<Vec<Value>> {
        unsafe {
            let state = self.get_state();
            let result = if status == Status::OK {
                let status = self.pcall(0, MULTIPLE_RESULTS);
                self.finish_call(status)
            } else {
                Err(pop_error(state, status))
            };
            lua_settop(state, 0);
            result
        }
    }

    /// Converts everything on the stack to results, or the error on top to
    /// an `Error`. Does not reset the stack.
    unsafe fn finish_call(&self, status: Status) -> Result<Vec<Value>> {
        unsafe {
            let state = self.get_state();
            if status != Status::OK {
                return Err(pop_error(state, status));
            }
            let count = (*state).get_top();
            let mut results = Vec::with_capacity(count as usize);
            for index in 1..=count {
                results.push(value_from_stack(state, index)?);
            }
            Ok(results)
        }
    }
}

// ─── protected helpers (run inside pcall) ───────────────────────────────────

unsafe fn open_libraries(state: *mut State) -> i32 {
    unsafe {
        lual_openlibs(state);
        0
    }
}

/// arg 1: light userdata holding a C string name. Returns the global.
unsafe fn protected_getglobal(state: *mut State) -> i32 {
    unsafe {
        let name = (*state).to_pointer(1) as *const i8;
        lua_getglobal(state, name);
        1
    }
}

/// arg 1: light userdata holding a C string name; arg 2: the value.
unsafe fn protected_setglobal(state: *mut State) -> i32 {
    unsafe {
        let name = (*state).to_pointer(1) as *const i8;
        lua_pushvalue(state, 2);
        lua_setglobal(state, name);
        0
    }
}

/// arg 1: light userdata holding a C string name; args 2..: call arguments.
unsafe fn protected_call(state: *mut State) -> i32 {
    unsafe {
        let name = (*state).to_pointer(1) as *const i8;
        let nargs = (*state).get_top() - 1;
        // [name, args...] -> [name, args..., f] -> [f, name, args...]
        lua_getglobal(state, name);
        lua_rotate(state, 1, 1);
        // -> [f, args..., name] -> [f, args...]
        lua_rotate(state, 2, -1);
        lua_settop(state, -2);
        (*state).lua_callk(nargs, MULTIPLE_RESULTS, 0, None);
        (*state).get_top()
    }
}

/// arg 1: light userdata holding a C string name; arg 2: light userdata
/// holding a `*mut Function`. Wraps the closure and assigns the global.
unsafe fn protected_register(state: *mut State) -> i32 {
    unsafe {
        let name = (*state).to_pointer(1) as *const i8;
        let raw = (*state).to_pointer(2) as *mut Function;
        let slot = crate::user::User::lua_newuserdatauv(state, size_of::<*mut Function>(), 0) as *mut *mut Function;
        *slot = raw;
        if lual_newmetatable(state, RUST_FUNCTION_METATABLE.as_ptr()) != 0 {
            lual_setfuncs(
                state,
                RUST_FUNCTION_METATABLE_FUNCTIONS.as_ptr(),
                RUST_FUNCTION_METATABLE_FUNCTIONS.len(),
                0,
            );
        }
        lua_setmetatable(state, -2);
        lua_pushcclosure(state, Some(rust_function_trampoline as unsafe fn(*mut State) -> i32), 1);
        lua_setglobal(state, name);
        0
    }
}

const RUST_FUNCTION_METATABLE_FUNCTIONS: [RegisteredFunction; 1] = [RegisteredFunction {
    registeredfunction_name: c"__gc".as_ptr(),
    registeredfunction_function: Some(rust_function_gc as unsafe fn(*mut State) -> i32),
}];

/// `__gc` for the closure-owning userdata: drops the boxed closure.
unsafe fn rust_function_gc(state: *mut State) -> i32 {
    unsafe {
        let slot = (*state).to_pointer(1) as *mut *mut Function;
        if !slot.is_null() && !(*slot).is_null() {
            drop(Box::from_raw(*slot));
            *slot = null_mut();
        }
        0
    }
}

/// The C function behind every registered Rust closure. Upvalue 1 is the
/// userdata owning the closure.
unsafe fn rust_function_trampoline(state: *mut State) -> i32 {
    unsafe {
        let slot = (*state).to_pointer(UPVALUE1) as *mut *mut Function;
        let function: &Function = &**slot;
        let nargs = (*state).get_top();
        let mut args = Vec::with_capacity(nargs as usize);
        for index in 1..=nargs {
            match value_from_stack(state, index) {
                Ok(v) => args.push(v),
                Err(e) => return raise(state, e.get_message()),
            }
        }
        match function(&args) {
            Ok(results) => {
                if lua_checkstack(state, results.len() as i32) == 0 {
                    return raise(state, "too many results");
                }
                for value in &results {
                    if let Err(e) = push_value(state, value) {
                        return raise(state, e.get_message());
                    }
                }
                results.len() as i32
            }
            Err(message) => raise(state, &message),
        }
    }
}

/// Raises `message` as a Lua error. Never returns.
unsafe fn raise(state: *mut State, message: &str) -> i32 {
    unsafe {
        lua_pushlstring(state, message.as_ptr() as *const i8, message.len());
        lua_error(state)
    }
}

// ─── stack <-> Value ────────────────────────────────────────────────────────

fn c_string(s: &str) -> Result<CString> {
    CString::new(s).map_err(|_| Error::new(ErrorKind::Conversion, "string contains an interior NUL byte"))
}

/// Pops the error object on top of the stack into an `Error`.
unsafe fn pop_error(state: *mut State, status: Status) -> Error {
    unsafe {
        let mut length: usize = 0;
        let pointer = lua_tolstring(state, -1, &mut length);
        let message = if pointer.is_null() {
            let type_name = std::ffi::CStr::from_ptr(lua_typename(state, lua_type(state, -1)));
            format!("(error object is a {} value)", type_name.to_string_lossy())
        } else {
            String::from_utf8_lossy(std::slice::from_raw_parts(pointer as *const u8, length)).into_owned()
        };
        lua_settop(state, -2);
        Error::from_status(status, message)
    }
}

/// Copies the value at `index` out of the interpreter.
unsafe fn value_from_stack(state: *mut State, index: i32) -> Result<Value> {
    unsafe {
        let mut visited: Vec<*const c_void> = Vec::new();
        value_from_stack_inner(state, lua_absindex(state, index), &mut visited)
    }
}

unsafe fn value_from_stack_inner(state: *mut State, index: i32, visited: &mut Vec<*const c_void>) -> Result<Value> {
    unsafe {
        match lua_type(state, index) {
            None | Some(TagType::Nil) => Ok(Value::Nil),
            Some(TagType::Boolean) => Ok(Value::Boolean(lua_toboolean(state, index))),
            Some(TagType::Numeric) => {
                if lua_isinteger(state, index) {
                    Ok(Value::Integer(lua_tointegerx(state, index, null_mut())))
                } else {
                    Ok(Value::Number(lua_tonumberx(state, index, null_mut())))
                }
            }
            Some(TagType::String) => {
                let mut length: usize = 0;
                let pointer = lua_tolstring(state, index, &mut length);
                let bytes = std::slice::from_raw_parts(pointer as *const u8, length).to_vec();
                Ok(Value::from_bytes(bytes))
            }
            Some(TagType::Table) => {
                let identity = (*state).to_pointer(index) as *const c_void;
                if visited.contains(&identity) {
                    return Err(Error::new(ErrorKind::Conversion, "cannot convert a table that refers to itself"));
                }
                if lua_checkstack(state, 3) == 0 {
                    return Err(Error::new(ErrorKind::Conversion, "table is nested too deeply"));
                }
                visited.push(identity);
                let mut pairs = Vec::new();
                (*state).push_nil();
                while lua_next(state, index) != 0 {
                    // Keys are never converted via tolstring on numerics, so
                    // the traversal key stays intact.
                    let key = value_from_stack_inner(state, lua_absindex(state, -2), visited);
                    let value = value_from_stack_inner(state, lua_absindex(state, -1), visited);
                    lua_settop(state, -2);
                    match (key, value) {
                        (Ok(k), Ok(v)) => pairs.push((k, v)),
                        (Err(e), _) | (_, Err(e)) => {
                            lua_settop(state, -2);
                            visited.pop();
                            return Err(e);
                        }
                    }
                }
                visited.pop();
                Ok(Value::Table(pairs))
            }
            Some(other) => {
                let type_name = std::ffi::CStr::from_ptr(lua_typename(state, Some(other)));
                Ok(Value::Opaque(type_name.to_string_lossy().into_owned()))
            }
        }
    }
}

/// Pushes `value` onto the stack. Fails, pushing nothing, if a table key is
/// `nil` or `NaN`, since those would raise when stored.
unsafe fn push_value(state: *mut State, value: &Value) -> Result<()> {
    unsafe {
        match value {
            Value::Nil | Value::Opaque(_) => (*state).push_nil(),
            Value::Boolean(b) => (*state).push_boolean(*b),
            Value::Integer(i) => (*state).push_integer(*i),
            Value::Number(n) => (*state).push_number(*n),
            Value::String(s) => {
                lua_pushlstring(state, s.as_ptr() as *const i8, s.len());
            }
            Value::Bytes(b) => {
                lua_pushlstring(state, b.as_ptr() as *const i8, b.len());
            }
            Value::Table(pairs) => {
                check_table_keys(pairs)?;
                if lua_checkstack(state, 3) == 0 {
                    return Err(Error::new(ErrorKind::Conversion, "table is nested too deeply"));
                }
                (*state).lua_createtable();
                for (key, value) in pairs {
                    if let Err(e) = push_value(state, key) {
                        lua_settop(state, -2);
                        return Err(e);
                    }
                    if let Err(e) = push_value(state, value) {
                        lua_settop(state, -3);
                        return Err(e);
                    }
                    lua_rawset(state, -3);
                }
            }
        }
        Ok(())
    }
}

/// Validates every key in a table tree before anything is pushed.
fn check_table_keys(pairs: &[(Value, Value)]) -> Result<()> {
    for (key, value) in pairs {
        match key {
            Value::Nil | Value::Opaque(_) => {
                return Err(Error::new(ErrorKind::Conversion, "table key is nil"));
            }
            Value::Number(n) if n.is_nan() => {
                return Err(Error::new(ErrorKind::Conversion, "table key is NaN"));
            }
            Value::Table(inner) => check_table_keys(inner)?,
            _ => {}
        }
        if let Value::Table(inner) = value {
            check_table_keys(inner)?;
        }
    }
    Ok(())
}

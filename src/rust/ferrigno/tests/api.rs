use ferrigno::api::{Error, ErrorKind, Lua, Value};
use std::cell::Cell;
use std::rc::Rc;

fn lua() -> Lua {
    Lua::new().expect("cannot create interpreter")
}

// ═══════════════════════════════════════════════════════════════
// eval / exec
// ═══════════════════════════════════════════════════════════════

#[test]
fn eval_returns_values() {
    let mut lua = lua();
    let values = lua.eval("return 1 + 2, 'three', 4.5, true, nil").unwrap();
    assert_eq!(
        values,
        vec![
            Value::Integer(3),
            Value::String("three".into()),
            Value::Number(4.5),
            Value::Boolean(true),
            Value::Nil,
        ]
    );
}

#[test]
fn eval_without_return_yields_nothing() {
    let mut lua = lua();
    assert_eq!(lua.eval("local x = 1").unwrap(), Vec::<Value>::new());
}

#[test]
fn exec_sets_state_for_later_calls() {
    let mut lua = lua();
    lua.exec("counter = 10").unwrap();
    lua.exec("counter = counter + 1").unwrap();
    assert_eq!(lua.get_global("counter").unwrap(), Value::Integer(11));
}

#[test]
fn syntax_error_is_reported() {
    let mut lua = lua();
    let error = lua.exec("return +").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Syntax);
    assert!(error.get_message().contains("unexpected symbol"), "{}", error);
}

#[test]
fn runtime_error_carries_message_and_traceback() {
    let mut lua = lua();
    let error = lua.exec("local t = nil; return t.field").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Runtime);
    assert!(error.get_message().contains("attempt to index a nil value"), "{}", error);
    assert!(error.get_message().contains("stack traceback"), "{}", error);
}

#[test]
fn error_raised_with_a_non_string_value() {
    let mut lua = lua();
    let error = lua.exec("error({code = 7})").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Runtime);
    assert!(error.get_message().contains("error object is a table value"), "{}", error);
}

#[test]
fn interpreter_recovers_after_errors() {
    let mut lua = lua();
    assert!(lua.exec("error('boom')").is_err());
    assert!(lua.exec("return +").is_err());
    assert_eq!(lua.eval("return 'fine'").unwrap(), vec![Value::from("fine")]);
}

#[test]
fn standard_libraries_are_open() {
    let mut lua = lua();
    let values = lua.eval("return string.rep('ab', 2), math.max(3, 9), #table.pack(1, 2, 3)").unwrap();
    assert_eq!(values, vec![Value::from("abab"), Value::Integer(9), Value::Integer(3)]);
}

#[test]
fn ferrigno_libraries_are_open() {
    let mut lua = lua();
    let values = lua.eval("local json = require('json'); return json.encode({1, 2})").unwrap();
    assert_eq!(values, vec![Value::from("[1,2]")]);
}

// ═══════════════════════════════════════════════════════════════
// exec_file
// ═══════════════════════════════════════════════════════════════

#[test]
fn exec_file_runs_a_script() {
    let dir = std::env::temp_dir().join(format!("ferrigno-api-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("script.lua");
    std::fs::write(&path, "answer = 42\nreturn answer * 2").unwrap();

    let mut lua = lua();
    let values = lua.exec_file(path.to_str().unwrap()).unwrap();
    assert_eq!(values, vec![Value::Integer(84)]);
    assert_eq!(lua.get_global("answer").unwrap(), Value::Integer(42));

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn exec_file_missing_is_a_file_error() {
    let mut lua = lua();
    let error = lua.exec_file("/nonexistent/path/to/script.lua").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::File);
    assert!(error.get_message().contains("cannot open"), "{}", error);
}

// ═══════════════════════════════════════════════════════════════
// globals
// ═══════════════════════════════════════════════════════════════

#[test]
fn get_global_missing_is_nil() {
    let mut lua = lua();
    assert_eq!(lua.get_global("no_such_global").unwrap(), Value::Nil);
}

#[test]
fn set_and_get_scalar_globals() {
    let mut lua = lua();
    lua.set_global("i", 7i64).unwrap();
    lua.set_global("n", 2.5f64).unwrap();
    lua.set_global("b", false).unwrap();
    lua.set_global("s", "text").unwrap();
    lua.set_global("nothing", ()).unwrap();
    assert_eq!(lua.get_global("i").unwrap(), Value::Integer(7));
    assert_eq!(lua.get_global("n").unwrap(), Value::Number(2.5));
    assert_eq!(lua.get_global("b").unwrap(), Value::Boolean(false));
    assert_eq!(lua.get_global("s").unwrap(), Value::from("text"));
    assert_eq!(lua.get_global("nothing").unwrap(), Value::Nil);
    assert_eq!(lua.eval("return i + n").unwrap(), vec![Value::Number(9.5)]);
}

#[test]
fn set_global_is_visible_to_lua() {
    let mut lua = lua();
    lua.set_global("greeting", "hello").unwrap();
    assert_eq!(lua.eval("return greeting .. ' world'").unwrap(), vec![Value::from("hello world")]);
}

#[test]
fn table_round_trip() {
    let mut lua = lua();
    let table = Value::Table(vec![
        (Value::from("name"), Value::from("ferrigno")),
        (Value::Integer(1), Value::Integer(100)),
        (Value::from("nested"), Value::Table(vec![(Value::from("x"), Value::Number(1.5))])),
    ]);
    lua.set_global("t", table).unwrap();
    assert_eq!(
        lua.eval("return t.name, t[1], t.nested.x").unwrap(),
        vec![Value::from("ferrigno"), Value::Integer(100), Value::Number(1.5)]
    );
    let back = lua.get_global("t").unwrap();
    assert_eq!(back.get(&Value::from("name")), Some(&Value::from("ferrigno")));
    assert_eq!(back.get(&Value::Integer(1)), Some(&Value::Integer(100)));
    let nested = back.get(&Value::from("nested")).unwrap();
    assert_eq!(nested.get(&Value::from("x")), Some(&Value::Number(1.5)));
    assert_eq!(back.as_table().unwrap().len(), 3);
}

#[test]
fn sequence_from_vec() {
    let mut lua = lua();
    lua.set_global("seq", vec![10i64, 20, 30]).unwrap();
    assert_eq!(lua.eval("return #seq, seq[1], seq[3]").unwrap(), vec![Value::Integer(3), Value::Integer(10), Value::Integer(30)]);
}

#[test]
fn table_with_nil_key_is_rejected_before_pushing() {
    let mut lua = lua();
    let error = lua.set_global("t", Value::Table(vec![(Value::Nil, Value::Integer(1))])).unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Conversion);
    let error = lua.set_global("t", Value::Table(vec![(Value::Number(f64::NAN), Value::Integer(1))])).unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Conversion);
    // The interpreter is still healthy.
    assert_eq!(lua.eval("return 1").unwrap(), vec![Value::Integer(1)]);
}

#[test]
fn cyclic_table_is_a_conversion_error() {
    let mut lua = lua();
    lua.exec("cyc = {}; cyc.self = cyc").unwrap();
    let error = lua.get_global("cyc").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Conversion);
    assert_eq!(lua.eval("return 2").unwrap(), vec![Value::Integer(2)]);
}

#[test]
fn shared_but_acyclic_subtables_convert() {
    let mut lua = lua();
    lua.exec("local inner = {1}; shared = {a = inner, b = inner}").unwrap();
    let value = lua.get_global("shared").unwrap();
    assert_eq!(value.as_table().unwrap().len(), 2);
}

#[test]
fn functions_are_opaque() {
    let mut lua = lua();
    assert_eq!(lua.get_global("print").unwrap(), Value::Opaque("function".into()));
}

#[test]
fn non_utf8_strings_become_bytes() {
    let mut lua = lua();
    let values = lua.eval("return '\\255\\0\\1'").unwrap();
    assert_eq!(values, vec![Value::Bytes(vec![255, 0, 1])]);
    lua.set_global("raw", Value::Bytes(vec![0, 200, 0])).unwrap();
    assert_eq!(lua.eval("return #raw, raw:byte(2)").unwrap(), vec![Value::Integer(3), Value::Integer(200)]);
}

#[test]
fn names_with_interior_nul_are_rejected() {
    let mut lua = lua();
    let error = lua.get_global("bad\0name").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Conversion);
}

#[test]
fn metamethod_error_on_global_access_is_caught() {
    let mut lua = lua();
    lua.exec("setmetatable(_G, {__index = function(_, k) error('no global ' .. k) end})").unwrap();
    let error = lua.get_global("missing").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Runtime);
    assert!(error.get_message().contains("no global missing"), "{}", error);
}

// ═══════════════════════════════════════════════════════════════
// call
// ═══════════════════════════════════════════════════════════════

#[test]
fn call_global_function_with_arguments() {
    let mut lua = lua();
    lua.exec("function add(a, b) return a + b, a - b end").unwrap();
    let results = lua.call("add", &[Value::Integer(5), Value::Integer(3)]).unwrap();
    assert_eq!(results, vec![Value::Integer(8), Value::Integer(2)]);
}

#[test]
fn call_with_no_arguments_and_no_results() {
    let mut lua = lua();
    lua.exec("called = false; function mark() called = true end").unwrap();
    assert_eq!(lua.call("mark", &[]).unwrap(), Vec::<Value>::new());
    assert_eq!(lua.get_global("called").unwrap(), Value::Boolean(true));
}

#[test]
fn call_passes_tables() {
    let mut lua = lua();
    lua.exec("function sum(t) local s = 0; for _, v in ipairs(t) do s = s + v end; return s end").unwrap();
    let results = lua.call("sum", &[Value::from(vec![1i64, 2, 3, 4])]).unwrap();
    assert_eq!(results, vec![Value::Integer(10)]);
}

#[test]
fn call_of_non_function_is_a_runtime_error() {
    let mut lua = lua();
    let error = lua.call("undefined_function", &[]).unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Runtime);
    assert!(error.get_message().contains("attempt to call a nil value"), "{}", error);
}

#[test]
fn call_propagates_lua_errors() {
    let mut lua = lua();
    lua.exec("function fail() error('failed inside') end").unwrap();
    let error = lua.call("fail", &[]).unwrap_err();
    assert!(error.get_message().contains("failed inside"), "{}", error);
}

// ═══════════════════════════════════════════════════════════════
// register
// ═══════════════════════════════════════════════════════════════

#[test]
fn registered_function_is_callable_from_lua() {
    let mut lua = lua();
    lua.register("double", |args| {
        let n = args.first().and_then(Value::as_integer).ok_or("expected an integer")?;
        Ok(vec![Value::Integer(n * 2)])
    })
    .unwrap();
    assert_eq!(lua.eval("return double(21)").unwrap(), vec![Value::Integer(42)]);
}

#[test]
fn registered_function_receives_all_arguments_and_returns_many() {
    let mut lua = lua();
    lua.register("echo", |args| Ok(args.to_vec())).unwrap();
    let values = lua.eval("return echo(1, 'two', {3}, nil, true)").unwrap();
    assert_eq!(values.len(), 5);
    assert_eq!(values[0], Value::Integer(1));
    assert_eq!(values[1], Value::from("two"));
    assert_eq!(values[2].get(&Value::Integer(1)), Some(&Value::Integer(3)));
    assert_eq!(values[3], Value::Nil);
    assert_eq!(values[4], Value::Boolean(true));
}

#[test]
fn registered_function_error_becomes_lua_error() {
    let mut lua = lua();
    lua.register("explode", |_| Err("kaboom".to_string())).unwrap();
    let error = lua.exec("explode()").unwrap_err();
    assert_eq!(error.get_kind(), ErrorKind::Runtime);
    assert!(error.get_message().contains("kaboom"), "{}", error);
    // And it can be caught in Lua.
    let values = lua.eval("local ok, msg = pcall(explode); return ok, msg").unwrap();
    assert_eq!(values, vec![Value::Boolean(false), Value::from("kaboom")]);
}

#[test]
fn registered_closure_captures_state() {
    let mut lua = lua();
    let hits = Rc::new(Cell::new(0));
    let counter = Rc::clone(&hits);
    lua.register("hit", move |_| {
        counter.set(counter.get() + 1);
        Ok(vec![])
    })
    .unwrap();
    lua.exec("for i = 1, 5 do hit() end").unwrap();
    assert_eq!(hits.get(), 5);
}

#[test]
fn registered_closure_is_dropped_when_collected() {
    struct DropFlag(Rc<Cell<bool>>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    let dropped = Rc::new(Cell::new(false));
    let flag = DropFlag(Rc::clone(&dropped));
    let mut lua = lua();
    lua.register("holder", move |_| {
        let _keep = &flag;
        Ok(vec![])
    })
    .unwrap();
    lua.exec("holder = nil; collectgarbage(); collectgarbage()").unwrap();
    assert!(dropped.get(), "closure should be freed once unreachable");
}

#[test]
fn registered_closure_is_dropped_with_interpreter() {
    struct DropFlag(Rc<Cell<bool>>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    let dropped = Rc::new(Cell::new(false));
    let flag = DropFlag(Rc::clone(&dropped));
    {
        let mut lua = lua();
        lua.register("holder", move |_| {
            let _keep = &flag;
            Ok(vec![])
        })
        .unwrap();
        assert!(!dropped.get());
    }
    assert!(dropped.get(), "closure should be freed when the interpreter closes");
}

#[test]
fn registered_function_can_call_back_into_lua_values() {
    let mut lua = lua();
    lua.register("sum_table", |args| {
        let table = args.first().and_then(Value::as_table).ok_or("expected a table")?;
        let sum: i64 = table.iter().filter_map(|(_, v)| v.as_integer()).sum();
        Ok(vec![Value::Integer(sum)])
    })
    .unwrap();
    assert_eq!(lua.eval("return sum_table({4, 5, 6})").unwrap(), vec![Value::Integer(15)]);
}

#[test]
fn error_display_is_the_message() {
    let error = Error::new(ErrorKind::Other, "something");
    assert_eq!(error.to_string(), "something");
}

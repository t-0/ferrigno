use std::process::Command;

fn run_ferrigno(args: &[&str]) -> std::process::Output {
    let bin = std::env::var("CARGO_BIN_EXE_ferrigno").unwrap_or_else(|_| {
        let test_exe = std::env::current_exe().expect("cannot locate test binary");
        let deps_dir = test_exe.parent().expect("no parent dir");
        let profile_dir = deps_dir.parent().expect("no profile dir");
        profile_dir.join("ferrigno").to_string_lossy().into_owned()
    });
    Command::new(bin).args(args).output().expect("failed to run ferrigno")
}

fn run_ok(code: &str) -> String {
    let output = run_ferrigno(&["-e", code]);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        output.status.success(),
        "code failed: exit {:?}\ncode: {}\nstdout: {}\nstderr: {}",
        output.status.code(),
        code,
        stdout,
        String::from_utf8_lossy(&output.stderr)
    );
    stdout
}

// ═══════════════════════════════════════════════════════════════
// dis library
// ═══════════════════════════════════════════════════════════════

#[test]
fn dis_dis_lua_function() {
    let out = run_ok("local dis = require('dis'); local function f(x) return x end; dis.dis(f)");
    assert!(out.contains("RETURN"));
    assert!(out.contains("function"));
}

#[test]
fn dis_code_returns_string() {
    let out = run_ok(
        "local dis = require('dis'); local function f(x) return x*x end; print(type(dis.code(f)))",
    );
    assert!(out.trim() == "string");
}

#[test]
fn dis_info_fields() {
    let out = run_ok(
        "local dis = require('dis'); local function f(a, b) return a+b end; \
         local i = dis.info(f); print(i.params, i.instructions > 0, i.vararg)",
    );
    assert!(out.trim() == "2\ttrue\tfalse");
}

#[test]
fn dis_opcodes_count() {
    let out = run_ok(
        "local dis = require('dis'); local function f() return 1 end; \
         local ops = dis.opcodes(f); print(#ops == dis.info(f).instructions)",
    );
    assert!(out.trim() == "true");
}

#[test]
fn dis_c_closure() {
    let out = run_ok("local dis = require('dis'); print(dis.info(print).type)");
    assert!(out.trim() == "C");
}

// ═══════════════════════════════════════════════════════════════
// functools library
// ═══════════════════════════════════════════════════════════════

#[test]
fn functools_partial() {
    let out = run_ok(
        "local F = require('functools'); \
         local add = function(a,b) return a+b end; \
         print(F.partial(add, 5)(3))",
    );
    assert!(out.trim() == "8");
}

#[test]
fn functools_map() {
    let out = run_ok(
        "local F = require('functools'); \
         local r = F.map(function(x) return x*2 end, {1,2,3}); \
         print(r[1], r[2], r[3])",
    );
    assert!(out.trim() == "2\t4\t6");
}

#[test]
fn functools_filter() {
    let out = run_ok(
        "local F = require('functools'); \
         local r = F.filter(function(x) return x%2==0 end, {1,2,3,4,5,6}); \
         print(#r, r[1], r[2], r[3])",
    );
    assert!(out.trim() == "3\t2\t4\t6");
}

#[test]
fn functools_reduce() {
    let out = run_ok(
        "local F = require('functools'); \
         print(F.reduce(function(a,b) return a+b end, {1,2,3,4,5}))",
    );
    assert!(out.trim() == "15");
}

#[test]
fn functools_reduce_with_init() {
    let out = run_ok(
        "local F = require('functools'); \
         print(F.reduce(function(a,b) return a+b end, {1,2,3}, 100))",
    );
    assert!(out.trim() == "106");
}

#[test]
fn functools_compose() {
    let out = run_ok(
        "local F = require('functools'); \
         local inc = function(x) return x+1 end; \
         local dbl = function(x) return x*2 end; \
         print(F.compose(inc, dbl)(5))",
    );
    assert!(out.trim() == "11"); // inc(dbl(5)) = inc(10) = 11
}

#[test]
fn functools_memoize() {
    let out = run_ok(
        "local F = require('functools'); \
         local n = 0; \
         local f = F.memoize(function(x) n=n+1; return x*x end); \
         f(4); f(4); f(5); f(5); \
         print(n)",
    );
    assert!(out.trim() == "2");
}

#[test]
fn functools_any_all() {
    let out = run_ok(
        "local F = require('functools'); \
         print(F.any(function(x) return x>3 end, {1,2,3,4})); \
         print(F.all(function(x) return x>0 end, {1,2,3})); \
         print(F.all(function(x) return x>2 end, {1,2,3}))",
    );
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(lines[0] == "true");
    assert!(lines[1] == "true");
    assert!(lines[2] == "false");
}

#[test]
fn functools_identity() {
    let out = run_ok(
        "local F = require('functools'); print(F.identity(1, 'a', true))",
    );
    assert!(out.trim() == "1\ta\ttrue");
}

#[test]
fn functools_flip() {
    let out = run_ok(
        "local F = require('functools'); \
         local div = function(a,b) return a/b end; \
         print(F.flip(div)(2, 10))",
    );
    assert!(out.trim() == "5.0");
}

// ═══════════════════════════════════════════════════════════════
// fmath library
// ═══════════════════════════════════════════════════════════════

#[test]
fn fmath_constants() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.pi > 3.14 and M.pi < 3.15); \
         print(M.e > 2.71 and M.e < 2.72); \
         print(M.tau > 6.28 and M.tau < 6.29); \
         print(M.inf == 1/0); \
         print(M.nan ~= M.nan)",
    );
    for line in out.trim().lines() {
        assert!(line == "true", "expected true, got: {}", line);
    }
}

#[test]
fn fmath_hyperbolic() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.sinh(0) == 0, M.cosh(0) == 1, M.tanh(0) == 0); \
         print(M.asinh(0) == 0, M.acosh(1) == 0, M.atanh(0) == 0)",
    );
    for line in out.trim().lines() {
        assert!(line == "true\ttrue\ttrue", "got: {}", line);
    }
}

#[test]
fn fmath_new_functions() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.cbrt(27)); \
         print(M.hypot(3, 4)); \
         print(M.log2(1024)); \
         print(M.log10(1000)); \
         print(M.trunc(3.7)); \
         print(M.trunc(-3.7)); \
         print(M.round(3.5))",
    );
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(lines[0] == "3.0");
    assert!(lines[1] == "5.0");
    assert!(lines[2] == "10.0");
    assert!(lines[3] == "3.0");
    assert!(lines[4] == "3");
    assert!(lines[5] == "-3");
    assert!(lines[6] == "4");
}

#[test]
fn fmath_predicates() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.isnan(0/0)); \
         print(M.isnan(1)); \
         print(M.isinf(1/0)); \
         print(M.isinf(1)); \
         print(M.isfinite(42)); \
         print(M.isfinite(1/0))",
    );
    let expected = ["true", "false", "true", "false", "true", "false"];
    for (line, exp) in out.trim().lines().zip(expected.iter()) {
        assert!(line == *exp, "expected {}, got: {}", exp, line);
    }
}

#[test]
fn fmath_integer_math() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.gcd(12, 8)); \
         print(M.lcm(4, 6)); \
         print(M.factorial(10)); \
         print(M.comb(10, 3)); \
         print(M.perm(5, 3))",
    );
    let expected = ["4", "12", "3628800", "120", "60"];
    for (line, exp) in out.trim().lines().zip(expected.iter()) {
        assert!(line == *exp, "expected {}, got: {}", exp, line);
    }
}

#[test]
fn fmath_clamp() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.clamp(15, 0, 10)); \
         print(M.clamp(-5, 0, 10)); \
         print(M.clamp(5, 0, 10))",
    );
    let expected = ["10.0", "0.0", "5.0"];
    for (line, exp) in out.trim().lines().zip(expected.iter()) {
        assert!(line == *exp, "expected {}, got: {}", exp, line);
    }
}

#[test]
fn fmath_sign() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.sign(-5), M.sign(0), M.sign(3))",
    );
    assert!(out.trim() == "-1\t0\t1");
}

#[test]
fn fmath_aggregates() {
    let out = run_ok(
        "local M = require('fmath'); \
         print(M.sum({1,2,3,4})); \
         print(M.prod({1,2,3,4})); \
         print(M.dist({0,0}, {3,4}))",
    );
    let expected = ["10.0", "24.0", "5.0"];
    for (line, exp) in out.trim().lines().zip(expected.iter()) {
        assert!(line == *exp, "expected {}, got: {}", exp, line);
    }
}

// ═══════════════════════════════════════════════════════════════
// syntax extensions
// ═══════════════════════════════════════════════════════════════

#[test]
fn syntax_brace_function() {
    let out = run_ok("local function f(x) { return x * x } print(f(7))");
    assert!(out.trim() == "49");
}

#[test]
fn syntax_brace_nested() {
    let out = run_ok(
        "local function outer(x) { \
           local function inner(y) { return x + y } \
           return inner(10) \
         } print(outer(5))",
    );
    assert!(out.trim() == "15");
}

#[test]
fn syntax_brace_disabled() {
    let output = Command::new(
        std::env::var("CARGO_BIN_EXE_ferrigno").unwrap_or_else(|_| {
            let test_exe = std::env::current_exe().expect("cannot locate test binary");
            let deps_dir = test_exe.parent().expect("no parent dir");
            let profile_dir = deps_dir.parent().expect("no profile dir");
            profile_dir.join("ferrigno").to_string_lossy().into_owned()
        }),
    )
    .env("FERRIGNO_EXTENSION_BRACE", "false")
    .args(&["-e", "local function f(x) { return x }"])
    .output()
    .expect("failed to run");
    assert!(!output.status.success());
}

#[test]
fn syntax_fstring_basic() {
    let out = run_ok(r#"local x = "world"; print($"hello {x}")"#);
    assert!(out.trim() == "hello world");
}

#[test]
fn syntax_fstring_expressions() {
    let out = run_ok(r#"print($"{2 + 3}")"#);
    assert!(out.trim() == "5");
}

#[test]
fn syntax_fstring_escaped_braces() {
    let out = run_ok(r#"print($"{{literal}}")"#);
    assert!(out.trim() == "{literal}");
}

#[test]
fn syntax_fstring_empty() {
    let out = run_ok(r#"print($"" == "")"#);
    assert!(out.trim() == "true");
}

#[test]
fn syntax_fstring_long() {
    let out = run_ok(r#"local x = 42; print($[[val={x}]])"#);
    assert!(out.trim() == "val=42");
}

#[test]
fn syntax_fstring_nested_quotes() {
    let out = run_ok(r#"print($"{"hello"}")"#);
    assert!(out.trim() == "hello");
}

#[test]
fn syntax_fstring_disabled() {
    let output = Command::new(
        std::env::var("CARGO_BIN_EXE_ferrigno").unwrap_or_else(|_| {
            let test_exe = std::env::current_exe().expect("cannot locate test binary");
            let deps_dir = test_exe.parent().expect("no parent dir");
            let profile_dir = deps_dir.parent().expect("no profile dir");
            profile_dir.join("ferrigno").to_string_lossy().into_owned()
        }),
    )
    .env("FERRIGNO_EXTENSION_FSTRING", "false")
    .args(&["-e", r#"print($"hello")"#])
    .output()
    .expect("failed to run");
    assert!(!output.status.success());
}

#[test]
fn syntax_backtick_basic() {
    let out = run_ok("print(`printf hello`)");
    assert!(out.trim() == "hello");
}

#[test]
fn syntax_backtick_interpolation() {
    let out = run_ok(r#"local x = "world"; print(`printf {x}`)"#);
    assert!(out.trim() == "world");
}

#[test]
fn syntax_backtick_failure() {
    let out = run_ok(
        "local ok, msg, code = `exit 42`; \
         print(ok == nil, type(msg), code)",
    );
    assert!(out.trim() == "true\tstring\t42");
}

#[test]
fn syntax_backtick_disabled() {
    let output = Command::new(
        std::env::var("CARGO_BIN_EXE_ferrigno").unwrap_or_else(|_| {
            let test_exe = std::env::current_exe().expect("cannot locate test binary");
            let deps_dir = test_exe.parent().expect("no parent dir");
            let profile_dir = deps_dir.parent().expect("no profile dir");
            profile_dir.join("ferrigno").to_string_lossy().into_owned()
        }),
    )
    .env("FERRIGNO_EXTENSION_BACKTICK", "false")
    .args(&["-e", "`echo hello`"])
    .output()
    .expect("failed to run");
    assert!(!output.status.success());
}

// ═══════════════════════════════════════════════════════════════
// itertools library
// ═══════════════════════════════════════════════════════════════

#[test]
fn itertools_range() {
    let out = run_ok(
        "local I = require('itertools'); \
         local r = I.range(5); print(#r, r[1], r[5])",
    );
    assert!(out.trim() == "5\t1\t5");
}

#[test]
fn itertools_range_step() {
    let out = run_ok(
        "local I = require('itertools'); \
         local r = I.range(0, 10, 3); print(r[1], r[2], r[3], r[4])",
    );
    assert!(out.trim() == "0\t3\t6\t9");
}

#[test]
fn itertools_chain() {
    let out = run_ok(
        "local I = require('itertools'); \
         local c = I.chain({1,2}, {3,4}, {5}); print(#c, c[1], c[5])",
    );
    assert!(out.trim() == "5\t1\t5");
}

#[test]
fn itertools_zip() {
    let out = run_ok(
        "local I = require('itertools'); \
         local z = I.zip({1,2,3}, {'a','b','c'}); \
         print(#z, z[1][1], z[1][2], z[3][1], z[3][2])",
    );
    assert!(out.trim() == "3\t1\ta\t3\tc");
}

#[test]
fn itertools_zip_shortest() {
    let out = run_ok(
        "local I = require('itertools'); \
         print(#I.zip({1,2,3}, {'a','b'}))",
    );
    assert!(out.trim() == "2");
}

#[test]
fn itertools_zip_longest() {
    let out = run_ok(
        "local I = require('itertools'); \
         local z = I.zip_longest(0, {1,2,3}, {'a','b'}); \
         print(#z, z[3][2])",
    );
    assert!(out.trim() == "3\t0");
}

#[test]
fn itertools_enumerate() {
    let out = run_ok(
        "local I = require('itertools'); \
         local e = I.enumerate({'a','b','c'}); \
         print(e[1][1], e[1][2], e[3][1], e[3][2])",
    );
    assert!(out.trim() == "1\ta\t3\tc");
}

#[test]
fn itertools_enumerate_start() {
    let out = run_ok(
        "local I = require('itertools'); \
         local e = I.enumerate({'x','y'}, 0); print(e[1][1], e[2][1])",
    );
    assert!(out.trim() == "0\t1");
}

#[test]
fn itertools_slice() {
    let out = run_ok(
        "local I = require('itertools'); \
         local s = I.slice({10,20,30,40,50}, 2, 4); \
         print(#s, s[1], s[3])",
    );
    assert!(out.trim() == "3\t20\t40");
}

#[test]
fn itertools_slice_step() {
    let out = run_ok(
        "local I = require('itertools'); \
         local s = I.slice({10,20,30,40,50}, 1, 5, 2); \
         print(#s, s[1], s[2], s[3])",
    );
    assert!(out.trim() == "3\t10\t30\t50");
}

#[test]
fn itertools_takewhile() {
    let out = run_ok(
        "local I = require('itertools'); \
         local t = I.takewhile(function(x) return x<4 end, {1,2,3,4,5}); \
         print(#t, t[1], t[3])",
    );
    assert!(out.trim() == "3\t1\t3");
}

#[test]
fn itertools_dropwhile() {
    let out = run_ok(
        "local I = require('itertools'); \
         local d = I.dropwhile(function(x) return x<4 end, {1,2,3,4,5}); \
         print(#d, d[1], d[2])",
    );
    assert!(out.trim() == "2\t4\t5");
}

#[test]
fn itertools_compress() {
    let out = run_ok(
        "local I = require('itertools'); \
         local c = I.compress({1,2,3,4}, {true,false,true,false}); \
         print(#c, c[1], c[2])",
    );
    assert!(out.trim() == "2\t1\t3");
}

#[test]
fn itertools_accumulate() {
    let out = run_ok(
        "local I = require('itertools'); \
         local a = I.accumulate({1,2,3,4}, function(x,y) return x+y end); \
         print(a[1], a[2], a[3], a[4])",
    );
    assert!(out.trim() == "1\t3\t6\t10");
}

#[test]
fn itertools_accumulate_init() {
    let out = run_ok(
        "local I = require('itertools'); \
         local a = I.accumulate({1,2,3}, function(x,y) return x+y end, 100); \
         print(#a, a[1], a[4])",
    );
    assert!(out.trim() == "4\t100\t106");
}

#[test]
fn itertools_pairwise() {
    let out = run_ok(
        "local I = require('itertools'); \
         local p = I.pairwise({1,2,3,4}); \
         print(#p, p[1][1], p[1][2], p[3][1], p[3][2])",
    );
    assert!(out.trim() == "3\t1\t2\t3\t4");
}

#[test]
fn itertools_flatten() {
    let out = run_ok(
        "local I = require('itertools'); \
         local f = I.flatten({{1,2},{3},4,{5,6}}); \
         print(#f, f[1], f[6])",
    );
    assert!(out.trim() == "6\t1\t6");
}

#[test]
fn itertools_batched() {
    let out = run_ok(
        "local I = require('itertools'); \
         local b = I.batched({1,2,3,4,5}, 2); \
         print(#b, #b[1], #b[3])",
    );
    assert!(out.trim() == "3\t2\t1");
}

#[test]
fn itertools_reversed() {
    let out = run_ok(
        "local I = require('itertools'); \
         local r = I.reversed({1,2,3,4,5}); \
         print(r[1], r[2], r[5])",
    );
    assert!(out.trim() == "5\t4\t1");
}

#[test]
fn itertools_starmap() {
    let out = run_ok(
        "local I = require('itertools'); \
         local s = I.starmap(function(a,b) return a+b end, {{1,2},{3,4},{5,6}}); \
         print(s[1], s[2], s[3])",
    );
    assert!(out.trim() == "3\t7\t11");
}

#[test]
fn itertools_product() {
    let out = run_ok(
        "local I = require('itertools'); \
         local p = I.product({1,2}, {'a','b'}); \
         print(#p, p[1][1], p[1][2], p[4][1], p[4][2])",
    );
    assert!(out.trim() == "4\t1\ta\t2\tb");
}

#[test]
fn itertools_combinations() {
    let out = run_ok(
        "local I = require('itertools'); \
         local c = I.combinations({1,2,3,4}, 2); \
         print(#c, c[1][1], c[1][2], c[6][1], c[6][2])",
    );
    assert!(out.trim() == "6\t1\t2\t3\t4");
}

#[test]
fn itertools_permutations() {
    let out = run_ok(
        "local I = require('itertools'); \
         print(#I.permutations({1,2,3}))",
    );
    assert!(out.trim() == "6");
}

#[test]
fn itertools_permutations_r() {
    let out = run_ok(
        "local I = require('itertools'); \
         local p = I.permutations({1,2,3}, 2); \
         print(#p, #p[1])",
    );
    assert!(out.trim() == "6\t2");
}

#[test]
fn itertools_groupby() {
    let out = run_ok(
        "local I = require('itertools'); \
         local g = I.groupby({1,1,2,2,2,3}, function(x) return x end); \
         print(#g, g[1][1], #g[1][2], g[2][1], #g[2][2], g[3][1], #g[3][2])",
    );
    assert!(out.trim() == "3\t1\t2\t2\t3\t3\t1");
}

#[test]
fn itertools_unique() {
    let out = run_ok(
        "local I = require('itertools'); \
         local u = I.unique({1,2,2,3,1,4,3}); \
         print(#u, u[1], u[2], u[3], u[4])",
    );
    assert!(out.trim() == "4\t1\t2\t3\t4");
}

#[test]
fn itertools_rep() {
    let out = run_ok(
        "local I = require('itertools'); \
         local r = I.rep('x', 3); print(#r, r[1], r[3])",
    );
    assert!(out.trim() == "3\tx\tx");
}

#[test]
fn itertools_cycle() {
    let out = run_ok(
        "local I = require('itertools'); \
         local c = I.cycle({1,2}, 3); print(#c, c[1], c[6])",
    );
    assert!(out.trim() == "6\t1\t2");
}

-- tests for ferrigno syntax extensions: brace functions, f-strings, backticks
print("testing ferrigno syntax extensions")

-- ═══════════════════════════════════════════════════════════════
-- brace function syntax
-- ═══════════════════════════════════════════════════════════════

-- basic brace function
local function square(x) { return x * x }
assert(square(7) == 49)

-- traditional syntax still works
local function cube(x)
    return x * x * x
end
assert(cube(3) == 27)

-- nested brace functions
local function outer(x) {
    local function inner(y) { return x + y }
    return inner(10)
}
assert(outer(5) == 15)

-- anonymous brace function
local add = function(a, b) { return a + b }
assert(add(2, 3) == 5)

-- method syntax with braces
local t = {}
function t:get_self() { return self }
assert(t:get_self() == t)

-- multiline brace function
local function multi(x) {
    local a = x + 1
    local b = a * 2
    return b
}
assert(multi(5) == 12)

-- empty brace function
local function noop() { }
assert(noop() == nil)

-- brace function with if/for
local function sum_to(n) {
    local s = 0
    for i = 1, n do
        s = s + i
    end
    return s
}
assert(sum_to(10) == 55)

-- mixed: brace outer, end inner
local function mixed_outer(x) {
    local function inner(y)
        return y * 2
    end
    return inner(x)
}
assert(mixed_outer(5) == 10)

-- mixed: end outer, brace inner
local function mixed_outer2(x)
    local function inner(y) { return y * 2 }
    return inner(x)
end
assert(mixed_outer2(5) == 10)

-- ═══════════════════════════════════════════════════════════════
-- f-strings ($"..." syntax)
-- ═══════════════════════════════════════════════════════════════

-- basic interpolation
local name = "world"
assert($"hello {name}" == "hello world")

-- multiple expressions
local a, b = 10, 20
assert($"{a} + {b} = {a + b}" == "10 + 20 = 30")

-- no interpolation (plain string)
assert($"no interp" == "no interp")

-- empty string
assert($"" == "")

-- expression only (returns the value itself, no string conversion)
assert($"{42}" == 42)
assert($"{"hello"}" == "hello")

-- adjacent expressions (.. forces string conversion)
assert($"{1}{2}{3}" == "123")

-- function calls in expressions
assert($"upper: {string.upper("hello")}" == "upper: HELLO")

-- table access
local t2 = {x = 99}
assert($"val={t2.x}" == "val=99")

-- arithmetic (expression-only fstrings return the value, not a string)
assert($"{2 + 3}" == 5)
assert($"{10 * 10}" == 100)
-- with literal prefix, .. forces string conversion
assert($"result: {2 + 3}" == "result: 5")

-- nested string quotes (same delimiter works inside {})
assert($"{"quoted"}" == "quoted")

-- single-quote f-strings
assert($'hello {"world"}' == "hello world")

-- escaped braces
assert($"literal {{braces}}" == "literal {braces}")
assert($"open {{ close }}" == "open { close }")

-- operator precedence: expressions are parenthesized
local x = true
assert($'{x and "yes" or "no"}' == "yes")

-- long f-strings $[[...]]
assert($[[hello]] == "hello")
assert($[[]] == "")
local v = "LUA"
assert($[[lang={v}]] == "lang=LUA")

-- long f-string multiline
local result = $[[
line1
line2
]]
assert(string.find(result, "line1"))
assert(string.find(result, "line2"))

-- long f-string with nested quotes
assert($[[she said "hello" and 'goodbye']] == 'she said "hello" and \'goodbye\'')

-- long f-string with ]] inside via $[=[...]=]
assert($[=[has ]] inside and {1+2} works]=] == "has ]] inside and 3 works")

-- ═══════════════════════════════════════════════════════════════
-- backtick execution
-- ═══════════════════════════════════════════════════════════════

-- basic command
local result = `echo hello`
assert(result == "hello\n", "got: " .. tostring(result))

-- interpolation in backtick
local msg = "world"
local result2 = `echo {msg}`
assert(result2 == "world\n", "got: " .. tostring(result2))

-- multiple return values on failure
local ok, errmsg, code = `exit 42`
assert(ok == nil)
assert(type(errmsg) == "string")
assert(code == 42)

-- success returns stdout
local out = `printf test`
assert(out == "test", "got: " .. tostring(out))

-- expression interpolation
local n = 5
local out2 = `printf %s {n * 2}`
assert(out2 == "10", "got: " .. tostring(out2))

-- escaped braces in backtick
local out3 = `printf "{{literal}}"`
assert(out3 == "{literal}", "got: " .. tostring(out3))

print'+'

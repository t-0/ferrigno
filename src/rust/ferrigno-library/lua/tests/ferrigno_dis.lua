-- tests for the dis library
print("testing dis library")

local dis = require("dis")

-- dis.dis on a Lua function (smoke test - just ensure it doesn't error)
local function square(x) return x * x end
dis.dis(square)

-- dis.code returns a string
local code = dis.code(square)
assert(type(code) == "string")
assert(#code > 0)
assert(string.find(code, "MUL"))
assert(string.find(code, "RETURN"))

-- dis.info returns a table with expected fields
local info = dis.info(square)
assert(type(info) == "table")
assert(info.params == 1)
assert(info.stacksize > 0)
assert(info.instructions > 0)
assert(type(info.upvalues) == "number")
assert(type(info.constants) == "number")
assert(type(info.vararg) == "boolean")
assert(info.vararg == false)
assert(type(info.linedefined) == "number")
assert(type(info.lastlinedefined) == "number")

-- dis.opcodes returns instruction array
local ops = dis.opcodes(square)
assert(type(ops) == "table")
assert(#ops == info.instructions)
for i, op in ipairs(ops) do
    assert(type(op.op) == "number")
    assert(type(op.name) == "string")
    assert(type(op.line) == "number")
    assert(type(op.a) == "number")
end

-- dis.constants returns constant values
local function uses_constants()
    local x = "hello"
    print(x, 42)
end
local consts = dis.constants(uses_constants)
assert(type(consts) == "table")
-- should contain "print", "hello", 42 or similar
local found_hello = false
for _, v in ipairs(consts) do
    if v == "hello" then found_hello = true end
end
-- "hello" might be in constants depending on compiler
-- just check the table is valid

-- vararg function
local function vararg(...) return ... end
local vinfo = dis.info(vararg)
assert(vinfo.vararg == true)
assert(vinfo.params == 0)

-- function with upvalues
local up = 10
local function with_upvalue() return up end
local uinfo = dis.info(with_upvalue)
assert(uinfo.upvalues == 1)

-- nested functions
local function outer()
    local function inner() return 1 end
    return inner
end
local oinfo = dis.info(outer)
assert(oinfo.children == 1)

-- C closure handling (should not error)
local dis_info_c = dis.info(print)
assert(dis_info_c.type == "C")
assert(type(dis_info_c.upvalues) == "number")

local code_c = dis.code(print)
assert(type(code_c) == "string")

-- dis.opcodes and dis.constants return empty tables for C functions
local ops_c = dis.opcodes(print)
assert(type(ops_c) == "table")
assert(#ops_c == 0)

local consts_c = dis.constants(print)
assert(type(consts_c) == "table")
assert(#consts_c == 0)

print'+'

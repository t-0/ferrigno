-- tests for the itertools library
print("testing itertools library")

local I = require("itertools")

-- ── range ───────────────────────────────────────────────────────────────────
do
    local r = I.range(5)
    assert(#r == 5)
    assert(r[1] == 1 and r[5] == 5)

    local r2 = I.range(2, 5)
    assert(#r2 == 4)
    assert(r2[1] == 2 and r2[4] == 5)

    local r3 = I.range(10, 2, -3)
    assert(#r3 == 3)
    assert(r3[1] == 10 and r3[2] == 7 and r3[3] == 4)

    local r4 = I.range(1, 0)
    assert(#r4 == 0)

    local r5 = I.range(0, 10, 3)
    assert(r5[1] == 0 and r5[2] == 3 and r5[3] == 6 and r5[4] == 9)

    local ok, err = pcall(I.range, 1, 10, 0)
    assert(not ok)
end

-- ── rep ─────────────────────────────────────────────────────────────────────
do
    local r = I.rep("x", 3)
    assert(#r == 3 and r[1] == "x" and r[3] == "x")

    local r2 = I.rep(42, 0)
    assert(#r2 == 0)

    local r3 = I.rep(true, 2)
    assert(r3[1] == true and r3[2] == true)
end

-- ── cycle ───────────────────────────────────────────────────────────────────
do
    local c = I.cycle({1, 2}, 3)
    assert(#c == 6)
    assert(c[1] == 1 and c[2] == 2 and c[3] == 1 and c[6] == 2)

    local c2 = I.cycle({1}, 0)
    assert(#c2 == 0)
end

-- ── slice ───────────────────────────────────────────────────────────────────
do
    local t = {10, 20, 30, 40, 50}
    local s = I.slice(t, 2, 4)
    assert(#s == 3 and s[1] == 20 and s[2] == 30 and s[3] == 40)

    local s2 = I.slice(t, 1, 5, 2)
    assert(#s2 == 3 and s2[1] == 10 and s2[2] == 30 and s2[3] == 50)

    local s3 = I.slice(t, 5, 1, -1)
    assert(#s3 == 5 and s3[1] == 50 and s3[5] == 10)

    local s4 = I.slice(t, 3)
    assert(#s4 == 3 and s4[1] == 30)
end

-- ── takewhile ───────────────────────────────────────────────────────────────
do
    local tw = I.takewhile(function(x) return x < 4 end, {1, 2, 3, 4, 5})
    assert(#tw == 3 and tw[1] == 1 and tw[3] == 3)

    local tw2 = I.takewhile(function() return false end, {1, 2, 3})
    assert(#tw2 == 0)

    local tw3 = I.takewhile(function() return true end, {1, 2, 3})
    assert(#tw3 == 3)

    local tw4 = I.takewhile(function(x) return x < 10 end, {})
    assert(#tw4 == 0)
end

-- ── dropwhile ───────────────────────────────────────────────────────────────
do
    local dw = I.dropwhile(function(x) return x < 4 end, {1, 2, 3, 4, 5})
    assert(#dw == 2 and dw[1] == 4 and dw[2] == 5)

    local dw2 = I.dropwhile(function() return false end, {1, 2, 3})
    assert(#dw2 == 3)

    local dw3 = I.dropwhile(function() return true end, {1, 2, 3})
    assert(#dw3 == 0)
end

-- ── compress ────────────────────────────────────────────────────────────────
do
    local c = I.compress({1, 2, 3, 4}, {true, false, true, false})
    assert(#c == 2 and c[1] == 1 and c[2] == 3)

    local c2 = I.compress({1, 2, 3}, {})
    assert(#c2 == 0)

    -- note: in Lua, 0 is truthy; use false to exclude
    local c3 = I.compress({"a", "b", "c"}, {true, false, true})
    assert(#c3 == 2 and c3[1] == "a" and c3[2] == "c")
end

-- ── chain ───────────────────────────────────────────────────────────────────
do
    local c = I.chain({1, 2}, {3, 4}, {5})
    assert(#c == 5)
    for i = 1, 5 do assert(c[i] == i) end

    local c2 = I.chain({}, {1}, {})
    assert(#c2 == 1 and c2[1] == 1)

    local c3 = I.chain()
    assert(#c3 == 0)

    local c4 = I.chain({"a"}, {"b"})
    assert(c4[1] == "a" and c4[2] == "b")
end

-- ── zip ─────────────────────────────────────────────────────────────────────
do
    local z = I.zip({1, 2, 3}, {"a", "b", "c"})
    assert(#z == 3)
    assert(z[1][1] == 1 and z[1][2] == "a")
    assert(z[3][1] == 3 and z[3][2] == "c")

    -- truncates to shortest
    local z2 = I.zip({1, 2, 3}, {"a", "b"})
    assert(#z2 == 2)

    local z3 = I.zip()
    assert(#z3 == 0)

    -- three tables
    local z4 = I.zip({1, 2}, {"a", "b"}, {true, false})
    assert(#z4 == 2)
    assert(z4[1][3] == true and z4[2][3] == false)
end

-- ── zip_longest ─────────────────────────────────────────────────────────────
do
    local z = I.zip_longest(0, {1, 2, 3}, {"a", "b"})
    assert(#z == 3)
    assert(z[1][1] == 1 and z[1][2] == "a")
    assert(z[3][1] == 3 and z[3][2] == 0) -- filled

    local z2 = I.zip_longest("?", {1}, {"a", "b", "c"})
    assert(#z2 == 3)
    assert(z2[2][1] == "?" and z2[2][2] == "b")
end

-- ── enumerate ───────────────────────────────────────────────────────────────
do
    local e = I.enumerate({"a", "b", "c"})
    assert(#e == 3)
    assert(e[1][1] == 1 and e[1][2] == "a")
    assert(e[3][1] == 3 and e[3][2] == "c")

    -- custom start
    local e2 = I.enumerate({"x", "y"}, 0)
    assert(e2[1][1] == 0 and e2[2][1] == 1)

    local e3 = I.enumerate({})
    assert(#e3 == 0)
end

-- ── accumulate ──────────────────────────────────────────────────────────────
do
    local a = I.accumulate({1, 2, 3, 4}, function(x, y) return x + y end)
    assert(#a == 4)
    assert(a[1] == 1 and a[2] == 3 and a[3] == 6 and a[4] == 10)

    -- with initial value
    local a2 = I.accumulate({1, 2, 3}, function(x, y) return x + y end, 100)
    assert(#a2 == 4) -- init + 3 elements
    assert(a2[1] == 100 and a2[2] == 101 and a2[3] == 103 and a2[4] == 106)

    -- multiply
    local a3 = I.accumulate({1, 2, 3, 4}, function(x, y) return x * y end)
    assert(a3[1] == 1 and a3[2] == 2 and a3[3] == 6 and a3[4] == 24)

    local a4 = I.accumulate({}, function(x, y) return x + y end)
    assert(#a4 == 0)
end

-- ── pairwise ────────────────────────────────────────────────────────────────
do
    local p = I.pairwise({1, 2, 3, 4})
    assert(#p == 3)
    assert(p[1][1] == 1 and p[1][2] == 2)
    assert(p[2][1] == 2 and p[2][2] == 3)
    assert(p[3][1] == 3 and p[3][2] == 4)

    local p2 = I.pairwise({1})
    assert(#p2 == 0)

    local p3 = I.pairwise({})
    assert(#p3 == 0)
end

-- ── flatten ─────────────────────────────────────────────────────────────────
do
    local f = I.flatten({{1, 2}, {3}, 4, {5, 6}})
    assert(#f == 6)
    for i = 1, 6 do assert(f[i] == i) end

    local f2 = I.flatten({})
    assert(#f2 == 0)

    -- only one level deep
    local f3 = I.flatten({{1, {2, 3}}, {4}})
    assert(f3[1] == 1)
    assert(type(f3[2]) == "table") -- {2,3} not flattened further
    assert(f3[3] == 4)
end

-- ── batched ─────────────────────────────────────────────────────────────────
do
    local b = I.batched({1, 2, 3, 4, 5}, 2)
    assert(#b == 3)
    assert(#b[1] == 2 and b[1][1] == 1 and b[1][2] == 2)
    assert(#b[2] == 2 and b[2][1] == 3 and b[2][2] == 4)
    assert(#b[3] == 1 and b[3][1] == 5)

    local b2 = I.batched({1, 2, 3}, 3)
    assert(#b2 == 1 and #b2[1] == 3)

    local b3 = I.batched({1, 2, 3}, 5)
    assert(#b3 == 1 and #b3[1] == 3)

    local b4 = I.batched({}, 2)
    assert(#b4 == 0)

    local ok, err = pcall(I.batched, {1}, 0)
    assert(not ok)
end

-- ── reversed ────────────────────────────────────────────────────────────────
do
    local r = I.reversed({1, 2, 3, 4, 5})
    assert(#r == 5)
    assert(r[1] == 5 and r[2] == 4 and r[5] == 1)

    local r2 = I.reversed({})
    assert(#r2 == 0)

    local r3 = I.reversed({42})
    assert(#r3 == 1 and r3[1] == 42)
end

-- ── starmap ─────────────────────────────────────────────────────────────────
do
    local s = I.starmap(function(a, b) return a + b end, {{1, 2}, {3, 4}, {5, 6}})
    assert(#s == 3)
    assert(s[1] == 3 and s[2] == 7 and s[3] == 11)

    local s2 = I.starmap(function(a, b) return a .. b end, {{"hello", " "}, {"world", "!"}})
    assert(s2[1] == "hello " and s2[2] == "world!")
end

-- ── product ─────────────────────────────────────────────────────────────────
do
    local p = I.product({1, 2}, {"a", "b"})
    assert(#p == 4)
    assert(p[1][1] == 1 and p[1][2] == "a")
    assert(p[2][1] == 1 and p[2][2] == "b")
    assert(p[3][1] == 2 and p[3][2] == "a")
    assert(p[4][1] == 2 and p[4][2] == "b")

    local p2 = I.product({}, {1, 2})
    assert(#p2 == 0)

    local p3 = I.product({1}, {2})
    assert(#p3 == 1 and p3[1][1] == 1 and p3[1][2] == 2)
end

-- ── combinations ────────────────────────────────────────────────────────────
do
    local c = I.combinations({1, 2, 3, 4}, 2)
    assert(#c == 6) -- C(4,2) = 6
    assert(c[1][1] == 1 and c[1][2] == 2)
    assert(c[6][1] == 3 and c[6][2] == 4)

    local c2 = I.combinations({1, 2, 3}, 3)
    assert(#c2 == 1)
    assert(c2[1][1] == 1 and c2[1][2] == 2 and c2[1][3] == 3)

    local c3 = I.combinations({1, 2, 3}, 0)
    assert(#c3 == 1 and #c3[1] == 0) -- one empty combination

    local c4 = I.combinations({1, 2}, 3)
    assert(#c4 == 0) -- r > n

    local c5 = I.combinations({1, 2, 3}, 1)
    assert(#c5 == 3)
end

-- ── permutations ────────────────────────────────────────────────────────────
do
    local p = I.permutations({1, 2, 3})
    assert(#p == 6) -- 3! = 6

    -- check all permutations are distinct
    local seen = {}
    for _, perm in ipairs(p) do
        local key = table.concat(perm, ",")
        assert(not seen[key], "duplicate permutation: " .. key)
        seen[key] = true
    end

    local p2 = I.permutations({1, 2, 3}, 2)
    assert(#p2 == 6) -- P(3,2) = 6
    for _, perm in ipairs(p2) do
        assert(#perm == 2)
    end

    local p3 = I.permutations({1})
    assert(#p3 == 1 and p3[1][1] == 1)
end

-- ── groupby ─────────────────────────────────────────────────────────────────
do
    local g = I.groupby({1, 1, 2, 2, 2, 3}, function(x) return x end)
    assert(#g == 3)
    assert(g[1][1] == 1 and #g[1][2] == 2)
    assert(g[2][1] == 2 and #g[2][2] == 3)
    assert(g[3][1] == 3 and #g[3][2] == 1)

    -- group by even/odd
    local g2 = I.groupby({1, 3, 2, 4, 5}, function(x) return x % 2 end)
    assert(#g2 == 3) -- odd, even, odd (consecutive groups)
    assert(g2[1][1] == 1 and #g2[1][2] == 2) -- {1,3}
    assert(g2[2][1] == 0 and #g2[2][2] == 2) -- {2,4}
    assert(g2[3][1] == 1 and #g2[3][2] == 1) -- {5}

    local g3 = I.groupby({}, function(x) return x end)
    assert(#g3 == 0)

    -- string groupby
    local g4 = I.groupby({"a", "a", "b"}, function(x) return x end)
    assert(#g4 == 2)
    assert(g4[1][1] == "a" and #g4[1][2] == 2)
end

-- ── unique ──────────────────────────────────────────────────────────────────
do
    local u = I.unique({1, 2, 2, 3, 1, 4, 3})
    assert(#u == 4)
    assert(u[1] == 1 and u[2] == 2 and u[3] == 3 and u[4] == 4)

    local u2 = I.unique({})
    assert(#u2 == 0)

    local u3 = I.unique({5, 5, 5})
    assert(#u3 == 1 and u3[1] == 5)

    local u4 = I.unique({"a", "b", "a", "c"})
    assert(#u4 == 3 and u4[1] == "a" and u4[2] == "b" and u4[3] == "c")
end

print'+'

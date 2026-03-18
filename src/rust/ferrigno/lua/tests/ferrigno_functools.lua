-- tests for the functools library
print("testing functools library")

local F = require("functools")

-- partial
do
    local function add(a, b) return a + b end
    local add5 = F.partial(add, 5)
    assert(add5(3) == 8)
    assert(add5(0) == 5)
    assert(add5(-5) == 0)

    -- partial with multiple bound args
    local function add3(a, b, c) return a + b + c end
    local add10_20 = F.partial(add3, 10, 20)
    assert(add10_20(30) == 60)

    -- partial with no extra args
    local add5_10 = F.partial(add, 5, 10)
    assert(add5_10() == 15)

    -- chained partial
    local inc = F.partial(add, 1)
    assert(inc(99) == 100)
end

-- map
do
    local doubled = F.map(function(x) return x * 2 end, {1, 2, 3, 4})
    assert(#doubled == 4)
    assert(doubled[1] == 2)
    assert(doubled[2] == 4)
    assert(doubled[3] == 6)
    assert(doubled[4] == 8)

    -- empty table
    local empty = F.map(function(x) return x end, {})
    assert(#empty == 0)

    -- string transform
    local upper = F.map(string.upper, {"hello", "world"})
    assert(upper[1] == "HELLO")
    assert(upper[2] == "WORLD")
end

-- filter
do
    local evens = F.filter(function(x) return x % 2 == 0 end, {1, 2, 3, 4, 5, 6})
    assert(#evens == 3)
    assert(evens[1] == 2)
    assert(evens[2] == 4)
    assert(evens[3] == 6)

    -- filter all
    local all = F.filter(function() return true end, {1, 2, 3})
    assert(#all == 3)

    -- filter none
    local none = F.filter(function() return false end, {1, 2, 3})
    assert(#none == 0)

    -- empty input
    local empty = F.filter(function() return true end, {})
    assert(#empty == 0)
end

-- reduce
do
    local sum = F.reduce(function(a, b) return a + b end, {1, 2, 3, 4, 5})
    assert(sum == 15)

    -- with initial value
    local sum100 = F.reduce(function(a, b) return a + b end, {1, 2, 3}, 100)
    assert(sum100 == 106)

    -- single element no init
    local single = F.reduce(function(a, b) return a + b end, {42})
    assert(single == 42)

    -- single element with init
    local single_init = F.reduce(function(a, b) return a + b end, {1}, 10)
    assert(single_init == 11)

    -- string concat
    local joined = F.reduce(function(a, b) return a .. ", " .. b end, {"a", "b", "c"})
    assert(joined == "a, b, c")

    -- error on empty without init
    local ok, err = pcall(F.reduce, function(a, b) return a + b end, {})
    assert(not ok)
    assert(string.find(err, "empty"))
end

-- compose
do
    local double = function(x) return x * 2 end
    local inc = function(x) return x + 1 end

    -- compose(f, g)(x) = f(g(x))
    local double_then_inc = F.compose(inc, double)
    assert(double_then_inc(5) == 11) -- inc(double(5)) = inc(10) = 11

    local inc_then_double = F.compose(double, inc)
    assert(inc_then_double(5) == 12) -- double(inc(5)) = double(6) = 12

    -- three functions
    local negate = function(x) return -x end
    local pipeline = F.compose(negate, double, inc)
    assert(pipeline(5) == -12) -- negate(double(inc(5))) = negate(double(6)) = negate(12) = -12

    -- single function
    local just_double = F.compose(double)
    assert(just_double(5) == 10)
end

-- memoize
do
    local call_count = 0
    local function expensive(x)
        call_count = 0
        call_count = call_count + 1
        return x * x
    end

    call_count = 0
    local memo = F.memoize(expensive)
    assert(memo(4) == 16)
    assert(memo(4) == 16)  -- cached
    assert(memo(5) == 25)
    assert(memo(5) == 25)  -- cached

    -- nil key
    local function ret_nil(x) return x end
    local memo2 = F.memoize(ret_nil)
    assert(memo2("hello") == "hello")
    assert(memo2("hello") == "hello")
end

-- any
do
    assert(F.any(function(x) return x > 3 end, {1, 2, 3, 4, 5}) == true)
    assert(F.any(function(x) return x > 10 end, {1, 2, 3}) == false)
    assert(F.any(function(x) return x > 0 end, {}) == false)  -- empty
    assert(F.any(function(x) return x == 1 end, {1}) == true)
end

-- all
do
    assert(F.all(function(x) return x > 0 end, {1, 2, 3}) == true)
    assert(F.all(function(x) return x > 2 end, {1, 2, 3}) == false)
    assert(F.all(function(x) return x > 0 end, {}) == true)  -- vacuous truth
    assert(F.all(function(x) return x == 1 end, {1}) == true)
end

-- identity
do
    local a, b, c = F.identity(1, 2, 3)
    assert(a == 1 and b == 2 and c == 3)

    local x = F.identity("hello")
    assert(x == "hello")

    -- zero args
    local n = select("#", F.identity())
    assert(n == 0)
end

-- flip
do
    local function div(a, b) return a / b end
    local rdiv = F.flip(div)
    assert(rdiv(2, 10) == 5)  -- div(10, 2) = 5

    local function sub(a, b) return a - b end
    local rsub = F.flip(sub)
    assert(rsub(3, 10) == 7)  -- sub(10, 3) = 7
end

print'+'

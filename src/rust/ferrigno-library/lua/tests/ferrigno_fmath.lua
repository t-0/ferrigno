-- tests for the fmath library
print("testing fmath library")

local M = require("fmath")
local eps = 1e-10

local function approx(a, b, tol)
    tol = tol or eps
    return math.abs(a - b) < tol
end

-- constants
assert(type(M.pi) == "number")
assert(approx(M.pi, 3.141592653589793))
assert(approx(M.tau, 2 * M.pi))
assert(approx(M.e, 2.718281828459045))
assert(M.inf == math.huge)
assert(M.inf == 1/0)
assert(M.nan ~= M.nan) -- NaN is not equal to itself
assert(M.maxinteger == math.maxinteger)
assert(M.mininteger == math.mininteger)

-- trig (same as math.*)
assert(approx(M.sin(0), 0))
assert(approx(M.sin(M.pi / 2), 1))
assert(approx(M.cos(0), 1))
assert(approx(M.cos(M.pi), -1))
assert(approx(M.tan(0), 0))
assert(approx(M.asin(1), M.pi / 2))
assert(approx(M.acos(1), 0))
assert(approx(M.atan(1, 1), M.pi / 4))
assert(approx(M.atan(0), 0))

-- hyperbolic
assert(approx(M.sinh(0), 0))
assert(approx(M.cosh(0), 1))
assert(approx(M.tanh(0), 0))
assert(approx(M.asinh(0), 0))
assert(approx(M.acosh(1), 0))
assert(approx(M.atanh(0), 0))
assert(approx(M.sinh(1), 1.1752011936438014, 1e-8))
assert(approx(M.cosh(1), 1.5430806348152437, 1e-8))
assert(approx(M.tanh(1), 0.7615941559557649, 1e-8))

-- exponential / logarithmic
assert(approx(M.exp(0), 1))
assert(approx(M.exp(1), M.e))
assert(approx(M.expm1(0), 0))
assert(approx(M.log(1), 0))
assert(approx(M.log(M.e), 1))
assert(approx(M.log(100, 10), 2))
assert(approx(M.log(1024, 2), 10))
assert(approx(M.log2(1024), 10))
assert(approx(M.log10(1000), 3))
assert(approx(M.log1p(0), 0))
-- log1p precision for small x
assert(approx(M.log1p(1e-15), 1e-15, 1e-25))

assert(approx(M.pow(2, 10), 1024))
assert(approx(M.pow(3, 0), 1))
assert(approx(M.sqrt(144), 12))
assert(approx(M.cbrt(27), 3))
assert(approx(M.cbrt(-8), -2))
assert(approx(M.hypot(3, 4), 5))
assert(approx(M.hypot(0, 0), 0))

-- rounding
assert(M.floor(3.7) == 3)
assert(M.floor(-3.7) == -4)
assert(M.floor(5) == 5)
assert(M.ceil(3.2) == 4)
assert(M.ceil(-3.2) == -3)
assert(M.ceil(5) == 5)
assert(M.trunc(3.7) == 3)
assert(M.trunc(-3.7) == -3)
assert(M.trunc(5) == 5)
assert(M.round(3.5) == 4)
assert(M.round(3.4) == 3)
assert(M.round(-3.5) == -4)

-- abs
assert(M.abs(-5) == 5)
assert(M.abs(5) == 5)
assert(M.abs(0) == 0)
assert(M.abs(-3.14) == 3.14)

-- classification
assert(M.isnan(0/0) == true)
assert(M.isnan(1) == false)
assert(M.isnan(M.inf) == false)
assert(M.isnan("hello") == false)

assert(M.isinf(1/0) == true)
assert(M.isinf(-1/0) == true)
assert(M.isinf(1) == false)
assert(M.isinf(0/0) == false)

assert(M.isfinite(42) == true)
assert(M.isfinite(3.14) == true)
assert(M.isfinite(1/0) == false)
assert(M.isfinite(0/0) == false)

assert(M.copysign(1, -1) == -1)
assert(M.copysign(-1, 1) == 1)
assert(M.copysign(3, -0.0) == -3)

assert(M.sign(-5) == -1)
assert(M.sign(0) == 0)
assert(M.sign(3) == 1)
assert(M.sign(-3.14) == -1.0)
assert(M.sign(0.0) == 0.0)
assert(M.isnan(M.sign(0/0)))

-- remainders
assert(M.fmod(7, 3) == 1)
assert(M.fmod(10, 5) == 0)
assert(approx(M.remainder(7, 3), 1))
assert(approx(M.remainder(7, 2), -1)) -- IEEE 754: 7 - round(7/2)*2 = 7 - 4*2 = -1

local ip, fp = M.modf(3.75)
assert(ip == 3 and approx(fp, 0.75))
local ip2, fp2 = M.modf(-3.75)
assert(ip2 == -3 and approx(fp2, -0.75))
local ip3, fp3 = M.modf(5)
assert(ip3 == 5 and fp3 == 0.0)

local m, e = M.frexp(0.5)
assert(m == 0.5 and e == 0)
assert(approx(M.ldexp(0.5, 3), 4.0))

-- angular
assert(approx(M.deg(M.pi), 180))
assert(approx(M.rad(180), M.pi))
assert(approx(M.deg(M.rad(45)), 45))

-- min / max / clamp
assert(M.min(3, 1, 2) == 1)
assert(M.max(3, 1, 2) == 3)
assert(M.min(5) == 5)
assert(M.clamp(15, 0, 10) == 10)
assert(M.clamp(-5, 0, 10) == 0)
assert(M.clamp(5, 0, 10) == 5)

-- integer arithmetic
assert(M.gcd(12, 8) == 4)
assert(M.gcd(7, 13) == 1)
assert(M.gcd(0, 5) == 5)
assert(M.gcd(100, 0) == 100)
assert(M.gcd(-12, 8) == 4)

assert(M.lcm(4, 6) == 12)
assert(M.lcm(3, 7) == 21)
assert(M.lcm(0, 5) == 0)

assert(M.factorial(0) == 1)
assert(M.factorial(1) == 1)
assert(M.factorial(5) == 120)
assert(M.factorial(10) == 3628800)
assert(M.factorial(20) == 2432902008176640000)
local ok, err = pcall(M.factorial, -1)
assert(not ok)
local ok2, err2 = pcall(M.factorial, 21)
assert(not ok2)

assert(M.comb(10, 3) == 120)
assert(M.comb(5, 0) == 1)
assert(M.comb(5, 5) == 1)
assert(M.comb(5, 6) == 0)
assert(M.comb(0, 0) == 1)

assert(M.perm(5, 3) == 60)
assert(M.perm(5, 0) == 1)
assert(M.perm(5, 5) == 120)
assert(M.perm(5) == 120) -- default k=n

-- special functions
assert(M.erf(0) == 0)
assert(approx(M.erf(1), 0.8427, 1e-3))
assert(approx(M.erfc(0), 1))
assert(approx(M.erf(1) + M.erfc(1), 1, 1e-6))

assert(approx(M.gamma(5), 24, 1e-6))  -- 4! = 24
assert(approx(M.gamma(1), 1, 1e-6))
assert(approx(M.gamma(0.5), M.sqrt(M.pi), 1e-6))
assert(approx(M.lgamma(5), M.log(24), 1e-6))

-- float introspection
local na = M.nextafter(1.0, 2.0)
assert(na > 1.0)
assert(na < 1.0 + 1e-10)
assert(M.nextafter(1.0, 0.0) < 1.0)
assert(M.nextafter(1.0, 1.0) == 1.0)

local u = M.ulp(1.0)
assert(u > 0)
assert(u < 1e-10)

-- aggregate
assert(M.sum({1, 2, 3, 4}) == 10)
assert(M.sum({}) == 0)
assert(M.prod({1, 2, 3, 4}) == 24)
assert(M.prod({}) == 1)
assert(approx(M.dist({0, 0}, {3, 4}), 5))
assert(M.dist({1}, {1}) == 0)

-- Lua compat
assert(M.tointeger(5) == 5)
assert(M.tointeger(5.0) == 5)
assert(M.tointeger(5.5) == nil)
assert(M.type(1) == "integer")
assert(M.type(1.0) == "float")
assert(M.type("x") == nil)
assert(M.ult(1, 2) == true)
assert(M.ult(2, 1) == false)

-- random
assert(type(M.random()) == "number")
assert(M.random() >= 0 and M.random() < 1)
local r = M.random(10)
assert(r >= 1 and r <= 10 and r == math.floor(r))
local r2 = M.random(5, 10)
assert(r2 >= 5 and r2 <= 10 and r2 == math.floor(r2))

-- randomseed accepts values
M.randomseed(42)
local a = M.random()
M.randomseed(42)
local b = M.random()
assert(a == b) -- same seed, same result

print'+'

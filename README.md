# ferrigno

## Overview

This is a reworking of the wonderful lua from lua.org in Rust. It passes the
Lua 5.5 test suite on macOS, and includes some additional libraries and syntax.
Libraries that need native system libraries are opt-in cargo features, so the
default build depends on nothing beyond libc.

The goal was to provide a lua embedded environment for rust, as a sandbox for
learning and to make it easier for me to write/deploy DevOps tooling.

## Installation

You need a recent stable Rust toolchain. The crate uses the 2024 edition and
let-chains, so Rust 1.88 or newer is required.

```sh
git clone https://github.com/t-0/ferrigno
cd ferrigno
./build.sh                      # dev and release builds with every library
./run.sh                        # runs the release binary
```

Or install the binaries onto your `PATH`:

```sh
cargo install --path src/rust/ferrigno                   # core Lua plus the pure-Rust libraries
cargo install --path src/rust/ferrigno --features full   # everything
```

Two binaries are produced: `ferrigno`, the interpreter, and `ferrignoc`, the
bytecode compiler.

The optional libraries link native libraries that must be present at build
time. macOS ships curl, sqlite and CoreMIDI. On Debian or Ubuntu:

```sh
sudo apt-get install libcurl4-openssl-dev libsqlite3-dev libasound2-dev
```

## Running

```
usage: ferrigno [options] [script [args]]
  -e stat   execute string 'stat'
  -i        enter interactive mode after executing 'script'
  -l mod    require library 'mod' into global 'mod'
  -l g=mod  require library 'mod' into global 'g'
  -v        show version information
  -E        ignore environment variables
  -W        turn warnings on
  --        stop handling options
  --bare    standard Lua state (skip embedded app)
  -         stop handling options and execute stdin
```

With no script and a terminal attached, `ferrigno` runs the embedded
`init.lua` and then drops into the REPL. `--bare` skips `init.lua`.

A script path starting with `@` is read from the resources compiled into the
binary rather than the filesystem (see [Embedded scripts](#embedded-scripts)):

```sh
ferrigno @tests/ferrigno_syntax.lua
```

The compiler mirrors `luac`:

```
usage: ferrignoc [options] [filenames]
  -l       list (use -l -l for full listing)
  -o name  output to file 'name' (default is "ferrignoc.out")
  -p       parse only
  -s       strip debug information
  --       stop handling options
  -        stop handling options and process stdin
```

## Syntax extensions

All three extensions are on by default. Each can be switched off with an
environment variable set to `0`, `false`, `no` or `off`:
`FERRIGNO_EXTENSION_BRACE`, `FERRIGNO_EXTENSION_FSTRING` and
`FERRIGNO_EXTENSION_BACKTICK`.

### Brace function bodies

A function body may be written with `{ }` instead of `... end`. The two forms
mix freely, including nested one inside the other.

```lua
local function square(x) { return x * x }

local add = function(a, b) { return a + b }

function t:get_self() { return self }

local function sum_to(n) {
    local s = 0
    for i = 1, n do s = s + i end
    return s
}
```

### Interpolated strings

`$"..."` and `$'...'` evaluate any `{expression}` inside the string. Long
form `$[[...]]` (and `$[=[...]=]`) works too and may span lines.

```lua
local name = "world"
print($"hello {name}")                   --> hello world
print($"{a} + {b} = {a + b}")            --> 10 + 20 = 30
print($"upper: {string.upper("hi")}")    --> upper: HI
print($"literal {{braces}}")             --> literal {braces}
print($[[lang={v}]])                     --> lang=LUA
```

Pieces are joined with `..`, so a string that is only a single expression
returns that expression's value unchanged: `$"{42}"` is the integer `42`, while
`$"n={42}"` is the string `"n=42"`. Write `{{` and `}}` for literal braces.

### Backtick commands

A backtick literal runs its contents through `sh -c` and evaluates to the
command's standard output. `{expression}` interpolation and `{{ }}` escapes
work as in interpolated strings.

```lua
local who = `whoami`                     -- "scott\n"
local out = `printf %s {n * 2}`          -- "10"
local ok, err, code = `exit 42`          -- nil, "exited with code 42", 42
```

On a non-zero exit the literal yields `nil`, a message and the exit code, in
the usual Lua error-return style.

## Libraries

The standard Lua 5.5 libraries are all present. These extras are always
compiled in and available through `require`:

| Library | Purpose | Functions |
|---|---|---|
| `dis` | Bytecode disassembly | `dis`, `code`, `info`, `opcodes`, `constants` |
| `fmath` | Extended math | `sin` … `atanh`, `exp`, `expm1`, `log`, `log2`, `log10`, `log1p`, `pow`, `sqrt`, `cbrt`, `hypot`, `floor`, `ceil`, `trunc`, `round`, `abs`, `isnan`, `isinf`, `isfinite`, `copysign`, `sign`, `fmod`, `remainder`, `modf`, `frexp`, `ldexp`, `deg`, `rad`, `min`, `max`, `clamp`, `gcd`, `lcm`, `factorial`, `comb`, `perm`, `erf`, `erfc`, `gamma`, `lgamma`, `nextafter`, `ulp`, `sum`, `prod`, `dist`, `tointeger`, `type`, `ult`, `random`, `randomseed` |
| `functools` | Functional helpers | `partial`, `reduce`, `map`, `filter`, `compose`, `memoize`, `any`, `all`, `identity`, `flip` |
| `itertools` | Iteration helpers | `range`, `rep`, `cycle`, `slice`, `takewhile`, `dropwhile`, `compress`, `chain`, `zip`, `zip_longest`, `enumerate`, `accumulate`, `pairwise`, `flatten`, `batched`, `reversed`, `starmap`, `product`, `combinations`, `permutations`, `groupby`, `unique` |
| `json` | JSON | `encode`, `decode`, and the `null` sentinel |
| `sh` | Shell commands | `sh.<command>(args...)` runs `command` with each argument shell-quoted and returns its stdout, or `nil, message, code` |
| `toml` | TOML | `parse`, `stringify` |

```lua
local sh = require("sh")
print(sh.echo("hi there", 42))           --> hi there 42

local json = require("json")
print(json.encode({a = 1, b = {1, 2}}))  --> {"b":[1,2],"a":1}
print(json.decode("[1,null]")[2] == json.null)  --> true
```

These need native libraries and are enabled per cargo feature:

| Feature | Library | Links | Functions |
|---|---|---|---|
| `sqlite` | `sqlite` | libsqlite3 | `open(path)` returns a connection with `exec`, `query`, `rows`, `prepare`, `last_insert_rowid`, `changes`, `close`; prepared statements have `bind`, `bind_values`, `step`, `get_row`, `columns`, `reset`, `finalize` |
| `requests` | `requests` | libcurl | `request`, `get`, `post`, `put`, `delete`, `patch`, `head` |
| `urllib` | `urllib` | libcurl | `encode`, `decode`, `parse`, `get`, `post`, `request` |
| `midi` | `midi` | CoreMIDI on macOS, libasound on Linux | `sources`, `destinations`, `open_output`, `open_input`; outputs have `send`, `note_on`, `note_off`, `cc`, `program_change`, `pitch_bend`, `aftertouch`, `clock`, `close`; inputs have `recv`, `pending`, `flush`, `close` |
| `tui` | `tui` | termios | `size`, `clear`, `clear_line`, `flush`, `move`, `hide_cursor`, `show_cursor`, `print`, `print_at`, `color`, `reset`, `enter_alt`, `exit_alt`, `raw`, `cooked`, `read_key`, `bell`, `init`, `cleanup` |

`ferrigno -v` lists the libraries compiled into a given binary.

## Embedded scripts

Every `.lua` file under `src/rust/ferrigno/lua/` is compiled into the binary
by `build.rs`. A path prefixed with `@` loads from that store, and `require`
inside an embedded script resolves first relative to the script's own
directory, then against the whole store, then the filesystem. That lets a
multi-file Lua application ship as a single executable.

```sh
ferrigno @tests/all.lua               # the embedded Lua 5.5 test suite
ferrigno @apps/sequencer/main.lua     # the MIDI sequencer; needs --features full
```

## Building

The default build has no dependencies beyond libc:

```sh
cargo build --release
```

Enable the optional libraries individually, or all at once with `full`:

```sh
cargo build --release --features sqlite,urllib
cargo build --release --features full
```

The `build.sh`, `test.sh` and `run.sh` scripts use `full`.

## Embedding

The `ferrigno::api` module is the safe entry point for using ferrigno as a
library. Add the crate as a dependency, then:

```rust
use ferrigno::api::{Lua, Value};

fn main() -> Result<(), ferrigno::api::Error> {
    let mut lua = Lua::new()?;

    lua.exec("greeting = 'hello'")?;
    assert_eq!(lua.get_global("greeting")?, Value::from("hello"));

    lua.set_global("limit", 3i64)?;
    let values = lua.eval("local t = {} for i = 1, limit do t[i] = i * i end return t")?;
    println!("{:?}", values[0]); // Table([(Integer(1), Integer(1)), ...])

    lua.exec("function add(a, b) return a + b end")?;
    assert_eq!(lua.call("add", &[Value::Integer(2), Value::Integer(3)])?, vec![Value::Integer(5)]);

    lua.register("double", |args| {
        let n = args.first().and_then(Value::as_integer).ok_or("expected an integer")?;
        Ok(vec![Value::Integer(n * 2)])
    })?;
    assert_eq!(lua.eval("return double(21)")?, vec![Value::Integer(42)]);

    Ok(())
}
```

Errors carry a kind (`Runtime`, `Syntax`, `File`, ...) and the message, with a
traceback for runtime errors. Tables cross the boundary as owned key/value
pairs; functions and userdata are reported as `Value::Opaque`. Callers who
need the full low-level API can reach the raw state through `Lua::get_state`.

## Testing

```sh
./test.sh          # builds, then runs cargo test and the Lua suites in debug and release
cargo test         # Rust integration tests plus the embedded Lua 5.5 suite
```

The upstream Lua 5.5 test suite lives in `src/rust/ferrigno/lua/tests` and is
run both from the embedded copy and from disk with `_U=true`. The
`ferrigno_*.lua` files there cover the extensions.

## CI and releases

GitHub Actions builds and tests every push and pull request on macOS and
Ubuntu, checks that the default build links none of the optional native
libraries, and runs clippy. Pushing a tag of the form `vX.Y.Z` builds release
binaries for Apple silicon, Intel macOS and x86_64 Linux and attaches them to
a GitHub release.

## Development & AI

I'd started this about 4 years ago, with a c2rust conversion and making it more
idiomatic. Althought it was feature complete, it had a number of hairy bugs and
progress was somewhat slow, being one of a number of weekend projects.

More recently I've used AI in three key areas which have helped me to get to
this point:

- slog of debugging
- completing refactoring
- adding additional library functionality quickly

## Relationship with Lua and other upstreams

It's a downstream work, I plan to keep it up-to-date functionally with upstream
Lua, where I need to embed in rust.

## License

MIT. See [LICENSE.md](LICENSE.md).

## Can I contribute?

Possibly, and particularly if you are happy to do so for fun / open-source.
Reach out so we can discuss it.

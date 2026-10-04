# ferrigno

## Overview

This is a reworking of the wonderful lua from lua.org in Rust. It does pass
the Lua 5.5 test suite, on MacOS, and includes some additional libraries and
syntax. Libraries that need native system libraries are opt-in cargo features.

The goal was to provide a lua embedded environment for rust, as a sandbox for
learning and  to make it easier for me to write/deploy DevOps tooling.

## Additional Features

- Syntax
    - "@<filename>" syntax for embedded scripts, which resolves their imports
    - $"...{<expression>}" for interpolated -strings
    - `<command> ...` for shell invocation
    - supports { } for functions
- Libraries (always available)
    - dis
    - fmath
    - functools
    - itertools
    - json
    - sh
    - toml
- Libraries (optional cargo features, off by default)
    - `midi` — links CoreMIDI on macOS, libasound on Linux
    - `requests` — links libcurl
    - `sqlite` — links libsqlite3
    - `tui` — raw terminal control
    - `urllib` — links libcurl

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

## Can I contribute?

Possibly, and particularly if you are happy to do so for fun / open-source.
Reach out so we can discuss it.

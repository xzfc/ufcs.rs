# ufcs.rs

[![Crates.io](https://img.shields.io/crates/v/ufcs.svg)](https://crates.io/crates/ufcs)
![License](https://img.shields.io/crates/l/ufcs.svg)
[![Build Status](https://travis-ci.org/xzfc/ufcs.rs.svg?branch=master)](https://travis-ci.org/xzfc/ufcs.rs)

`ufcs::Pipe` — helper trait to call free functions using method call syntax.

Rust already allows calling methods using function call syntax, but not the other way around.
This crate fills the gap by providing a simple helper trait `Pipe`.

## Usage

Cargo.toml:
```toml
[dependencies]
ufcs = "0.1.0"
```

Rust code:
```rust
use ufcs::Pipe;
```

## Examples

```rust
// Write this
foo().pipe(bar).pipe(baz);

// Instead of this
baz(bar(foo()));
```

```rust
// Write this
let text: String = reqwest::get("http://httpbin.org/get")
    .await?
    .json::<serde_json::Value>()
    .await?
    .pipe(toml::Value::try_from)?
    .pipe(|x| toml::to_string(&x))?;

// Instead of this
let text: String = toml::to_string(&toml::Value::try_from(
    reqwest::get("http://httpbin.org/get")
        .await?
        .json::<serde_json::Value>()
        .await?,
)?)?;
```

See [tests](tests/lib.rs) for more examples.

## See also

Roughtly the same feature is either implemented or proposed in various languages.

### Rust

* [`pipeline.rs`](https://github.com/johannhof/pipeline.rs): implemented as an macro
* [RFC #289](https://github.com/rust-lang/rfcs/issues/289): Unified function / method call syntax
* [RFC #2049](https://github.com/rust-lang/rfcs/issues/2049): Piping data to functions
* [Method-cascading and pipe-forward operators proposal](https://internals.rust-lang.org/t/method-cascading-and-pipe-forward-operators-proposal/7384/59)
* [Rust #44874](https://github.com/rust-lang/rust/issues/44874): Tracking issue for `arbitrary_self_types`

### Other languages

* https://en.wikipedia.org/wiki/Uniform_Function_Call_Syntax
* [Nim], [DLang]: built-in UFCS
* F#, [Elixir]: the pipe operator `|>`
* C++: [proposed](https://brevzin.github.io/c++/2019/04/13/ufcs-history/)

[Nim]: https://nim-lang.org/docs/manual.html#procedures-method-call-syntax
[DLang]: https://tour.dlang.org/tour/en/gems/uniform-function-call-syntax-ufcs
[Elixir]: https://elixirschool.com/en/lessons/basics/pipe-operator/

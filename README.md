# easy-sgr

[![Build status](https://github.com/4lineclear/easy-sgr/actions/workflows/rust.yml/badge.svg)](https://github.com/4lineclear/easy-sgr/actions) [![Crates.io](https://img.shields.io/crates/v/easy-sgr)](https://crates.io/crates/easy-sgr) [![docs.rs](https://img.shields.io/docsrs/easy-sgr)](https://docs.rs/easy-sgr)[![License](https://img.shields.io/crates/l/easy-sgr)](https://github.com/4lineclear/easy-sgr/blob/main/LICENSE) [![Code Coverage](https://codecov.io/gh/4lineclear/easy-sgr/branch/main/graph/badge.svg?token=0Q30XAW0PV)](https://codecov.io/gh/4lineclear/easy-sgr)

An easy to use library for adding graphical ANSI codes or [`SGR`][SGR] escape sequences to your project.
Its main strengths are the multitude of methods that is provided, and the
lack of dependencies; compile times should be pretty good.

This library does not support usage of non [`SGR`][SGR] ANSI escape sequences

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
easy-sgr="0.0.0"
```

## Usage

### `Color` and `Style` enums

The simplest way to color text, using these two enums allows you to
work inline of a string literal when using a macro such as
`println!`, `writeln!` or `format!`:

```rust
use easy_sgr::{Color::*, Style::*};

println!("{Italic}{RedFg}This should be italic & red!{Reset}");
```

`Color` and `Style` are both enums that implement `Display`: when they
are printed a matching [SGR][SGR] code is written.

This method is the best when it comes to simplicity, but has drawbacks;
using it rewrites the Escape sequence `\x1b[` and the End sequence `m` repeatedly.
In this example this is what would be written:

```plain
\x1b[3m\x1b[31mThis should be italic & red!\x1b[0m
```

This would not be much of an issue for the vast majority of use cases.

### `EasySGR` trait

This is similar to method as above, but using the `EasySGR` trait.
This trait is implemented by anything that implements `Into<AnsiString>` including `Style` and `Color`.
It's main purpose is to provide functions for chaining [`SGR`][SGR] codes.

The example above can be achieved using it as such:

```rust
use easy_sgr::{ Color::*, EasySGR, Style::*};

let sgr = Italic.color(RedFg);

println!("{sgr}This should be italic & red!{Reset}");
```

Now the output would look something like this:

```plain
\x1b[31;3mThis should be italic & red!\x1b[0m
```

Now instead of a rewriting the entire sequence,
the separator character `;` is used instead.

Doing this avoids the issue of rewriting the Escape and End sequences,
though is more expensive to use as it allocates `SGRString`.

### `SGRString` struct

`SGRString` is the type returned by all `EasySGR` functions, it encapsulates all
possible SGR sequences. You can use it to reproduce the previous examples as such:

```rust
use easy_sgr::{Color::*, EasySGR, Style::*};

let text = "This should be italic & red!"
    .to_sgr()
    .style(Italic)
    .color(RedFg);
println!("{text}");
```

You can actually forgo `.to_sgr()`, as all functions in the `EasySGR`
work for anything that implements `Into<SGRString>`, so `.style(..)` and
`.color(..)` can be directly called on the string literal.

The method above still uses the `EasySGR` trait, you can go without it:

```rust
use easy_sgr::{ColorKind, SGRString, StyleKind};

let mut text = SGRString::from("This should be italic & red!");
text.italic = StyleKind::Place;
text.foreground = ColorKind::Red;

println!("{text}")
```

### `SGRWriter` trait

The writer can also be used directly, instead of a using the above methods:

```rust
use std::io::{stdout, Write};
use easy_sgr::{Color::*, EasySGR, SGRWriter, StandardWriter, Style::*};

let mut writer = StandardWriter::io(stdout());
writer.sgr(&Italic.color(RedFg)).unwrap();
writer.write_inner("This should be italic & red!").unwrap();
writer.sgr(&Reset).unwrap();
```

or, when writing to a String

```rust
use easy_sgr::{Color::*, EasySGR, SGRWriter, StandardWriter, Style::*};

let stylized_string = {
    let mut writer = StandardWriter::fmt(String::new());
    writer.sgr(&Italic.color(RedFg)).unwrap();
    writer.write_inner("This should be italic & red!").unwrap();
    writer.sgr(&Reset).unwrap();
    writer.writer.0
};
```

## Structure

easy-sgr is split into three modules:

- discrete
- graphics
- writing

Though no modules really need to be seen, as all the types they contain are exported.

[SGR]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR

## TODO goals to publish

- [ ] Docs
    - [x] [Crate](src/lib.rs) level docs
    - [x] Module level docs
    - [x] [graphics module](src/graphics/mod.rs) docs
    - [x] [writing module](src/writing.rs) docs
    - [x] Clean up all docs
    - [x] Write [Structure](#structure) section
    - [ ] Improve test coverage

## TODO goals past publishing

- [ ] Add examples to docs
- [ ] Implement `FromStr` for [`SGR`][SGR] types
- [ ] Parser (`deSGR`)
- [ ] Macros (`SGRise`)
- [ ] `EasySGR` implementation that doesn't allocate a `SGRString`
- [ ] (maybe) create smart clean system

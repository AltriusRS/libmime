# libmime

Auto-generated IANA media type constants for Rust.

This crate provides every media type registered in the
[IANA Media Types Registry](https://www.iana.org/assignments/media-types/media-types.xhtml)
as compile-time constants. It is updated automatically on a weekly schedule
via CI, with no human intervention required.

## Usage

Add the dependency:

```bash
cargo add libmime
```

Use a constant directly:

```rust
use libmime::{APPLICATION_JSON, TEXT_HTML};

let content_type = APPLICATION_JSON;
println!("{}", content_type); // application/json
```

Look up a type from a string:

```rust
use libmime::lookup;

if let Some(mime) = lookup("image/png") {
    println!("{}", mime.subtype()); // png
}
```

Lookup is case-insensitive. It returns `None` for types not present
in the IANA registry.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std`   | Yes     | Enables `phf`-backed O(1) lookup. |

Without `std`, lookup falls back to a binary search over a sorted
static table. The core `Mime` type is always available in `no_std`
environments with no allocator requirement.

```toml
[dependencies]
libmime = { version = "0.1", default-features = false }
```

## The `Mime` Type

`Mime` is `Copy`, `Clone`, `Eq`, `Hash`, and consists entirely of
`&'static str` references and a fieldless enum. There are no heap
allocations.

```rust
pub struct Mime {
    pub top: TopLevel,
    pub sub: &'static str,
    pub suffix: Option<&'static str>,
}
```

`TopLevel` covers the ten IANA top-level types:

- `Application`
- `Audio`
- `Font`
- `Haptics`
- `Image`
- `Message`
- `Model`
- `Multipart`
- `Text`
- `Video`

## How Updates Work

A scheduled GitHub Actions workflow runs weekly and executes the
code generator (internally referred to as the "Mime-inator"). It
fetches the current CSV data from the IANA registry, regenerates
the constant definitions and lookup tables, and compares them
against the committed versions. If nothing has changed, the run
exits early. If the generated output differs, it runs the full
test and lint suite, bumps the patch version, commits, tags, and
publishes to crates.io.

The generator source lives in `generator/`.

## Scope

This crate does one thing: it gives you typed constants for known
IANA media types and a way to look them up by string. It does not
parse arbitrary media type strings, handle parameters like
`charset` or `boundary`, or deal with content negotiation. If you
need that, this is not the right crate.

## Minimum Supported Rust Version

Due to our dependency on the `phf` crate for `std` features, our MSRV remains in line with theirs. Currently that is an MSRV of 1.66, if `phf` changes theirs, we will follow.

## License

MIT

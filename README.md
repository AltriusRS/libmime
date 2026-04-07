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

## Versioning

This crate uses the `major.minor.IANA_UPDATED(-patch)` pattern, where patch is optional, and IANA_UPDATED is the date provided in the page [Media Types.xhtml](https://www.iana.org/assignments/media-types/media-types.xhtml) under the `Last Updated` section, minus the dashes.

The date is formatted as YYYYMMDD

## Minimum Supported Rust Version

Due to our dependency on the `phf` crate for `std` features, our MSRV remains in line with theirs. Currently that is an MSRV of 1.66, if `phf` changes theirs, we will follow.

## License

MIT

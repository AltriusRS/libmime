use crate::Mime;

#[cfg(feature = "std")]
include!("./generated_phf.rs");
// Provides: static PHF_MAP: phf::Map<&'static str, Mime> = ...;

#[cfg(not(feature = "std"))]
include!("./generated_map.rs");
// Provides: static MIME_MAP: &[(&str, Mime)] = &[ ... ];

#[cfg(not(feature = "std"))]
fn cmp_ignore_ascii_case(a: &str, b: &str) -> core::cmp::Ordering {
    a.bytes()
        .map(|b| b.to_ascii_lowercase())
        .cmp(b.bytes().map(|b| b.to_ascii_lowercase()))
}

/// Look up a known IANA MIME type by its essence string.
///
/// Case-insensitive. Returns `None` if the type is not in
/// the IANA registry.
///
/// With the `std` feature (default), this is an O(1) perfect
/// hash lookup. Without it, this is an O(log n) binary search.
pub fn lookup(essence: &str) -> Option<Mime> {
    #[cfg(feature = "std")]
    {
        let lower = essence.to_ascii_lowercase();
        PHF_MAP.get(lower.as_str()).copied()
    }

    #[cfg(not(feature = "std"))]
    {
        MIME_MAP
            .binary_search_by(|(key, _)| cmp_ignore_ascii_case(key, essence))
            .ok()
            .map(|i| MIME_MAP[i].1)
    }
}

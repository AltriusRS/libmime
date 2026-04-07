use anyhow::Result;
use std::collections::BTreeMap;

const IANA_BASE: &str = "https://www.iana.org/assignments/media-types";

const CATEGORIES: &[&str] = &[
    "application",
    "audio",
    "font",
    "haptics",
    "image",
    "message",
    "model",
    "multipart",
    "text",
    "video",
];

pub(crate) struct Entry {
    pub(crate) const_name: String,
    pub(crate) top_variant: String,
    pub(crate) sub: String,
    pub(crate) suffix: Option<String>,
    pub(crate) essence: String,
}

/// Turns e.g. ("application", "vnd.api+json") into "APPLICATION_VND_API_JSON"
fn const_name(category: &str, subtype: &str) -> String {
    format!("{}_{}", category, subtype)
        .to_ascii_uppercase()
        .replace(['-', '.', '+', ' '], "_")
}

/// Turns e.g. "application" into "Application"
fn top_variant(category: &str) -> String {
    let mut chars = category.chars();
    match chars.next() {
        Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

/// Splits e.g. "vnd.api+json" into ("vnd.api", Some("json"))
fn parse_subtype(raw: &str) -> (String, Option<String>) {
    match raw.rfind('+') {
        Some(pos) => {
            let sub = raw[..pos].to_lowercase();
            let suffix = raw[pos + 1..].to_lowercase();
            (sub, Some(suffix))
        }
        None => (raw.to_lowercase(), None),
    }
}

fn build_essence(category: &str, sub: &str, suffix: &Option<String>) -> String {
    match suffix {
        Some(s) => format!("{}/{}+{}", category, sub, s),
        None => format!("{}/{}", category, sub),
    }
}

pub(crate) async fn fetch_entries(client: &reqwest::Client) -> Result<BTreeMap<String, Entry>> {
    let mut entries = BTreeMap::new();

    for &category in CATEGORIES {
        let url = format!("{}/{}.csv", IANA_BASE, category);
        println!("  Fetching {}", url);

        let body = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("failed to fetch {}", url))?
            .text()
            .await
            .with_context(|| format!("failed to read body from {}", url))?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(body.as_bytes());

        for record in rdr.records().flatten() {
            let name = record.get(0).unwrap_or("").trim();

            // Skip empty, malformed, or DEPRECATED/OBSOLETED entries
            if name.is_empty() || name.contains(' ') {
                continue;
            }

            let (sub, suffix) = parse_subtype(name);
            let essence = build_essence(category, &sub, &suffix);
            let entry = Entry {
                const_name: const_name(category, name),
                top_variant: top_variant(category),
                sub,
                suffix,
                essence,
            };

            entries.insert(entry.const_name.clone(), entry);
        }
    }

    Ok(entries)
}

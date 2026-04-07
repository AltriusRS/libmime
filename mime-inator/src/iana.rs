use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use xml::{reader::XmlEvent, EventReader};

const IANA_BASE: &str = "https://www.iana.org/assignments/media-types";
const IANA_INDEX: &str = "https://www.iana.org/assignments/media-types/media-types.xml";

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

fn const_name(category: &str, subtype: &str) -> String {
    format!("{}_{}", category, subtype)
        .to_ascii_uppercase()
        .replace(['-', '.', '+', ' '], "_")
}

fn top_variant(category: &str) -> String {
    let mut chars = category.chars();
    match chars.next() {
        Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

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

fn fetch_iana_date(xml: &str) -> Result<String> {
    let parser = EventReader::from_str(xml);
    let mut in_updated = false;

    for event in parser {
        match event.context("failed to parse IANA XML")? {
            XmlEvent::StartElement { name, .. } if name.local_name == "updated" => {
                in_updated = true;
            }
            XmlEvent::Characters(text) | XmlEvent::CData(text) => {
                if in_updated {
                    let date = text.trim();
                    if !date.is_empty() {
                        return Ok(date.to_string());
                    }
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "updated" {
                    in_updated = false;
                }
            }
            _ => {}
        }
    }

    bail!("could not find <updated>...</updated> in IANA XML index")
}

pub(crate) async fn fetch_entries() -> Result<(BTreeMap<String, Entry>, String)> {
    let client = reqwest::Client::builder()
        .user_agent("mime-inator/0.1.0")
        .build()?;

    // Fetch the last-updated date from the index page
    info!("  Fetching {}", IANA_INDEX);
    let index_html = client
        .get(IANA_INDEX)
        .send()
        .await
        .context("failed to fetch IANA index page")?
        .text()
        .await
        .context("failed to read IANA index page")?;

    let iana_date = fetch_iana_date(&index_html)?;

    // Fetch each category CSV
    let mut entries = BTreeMap::new();

    for &category in CATEGORIES {
        let url = format!("{}/{}.csv", IANA_BASE, category);
        info!("  Fetching {}", url);

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

    Ok((entries, iana_date))
}

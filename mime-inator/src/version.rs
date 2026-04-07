use anyhow::{Context, Result};
use std::fs;

const CARGO_TOML: &str = "libmime/Cargo.toml";

pub(crate) fn update(iana_date: &str) -> Result<(String, String)> {
    let content = fs::read_to_string(CARGO_TOML).context("failed to read libmime/Cargo.toml")?;

    let mut doc = content
        .parse::<toml_edit::DocumentMut>()
        .context("failed to parse libmime/Cargo.toml")?;

    let current = doc["package"]["version"]
        .as_str()
        .context("missing version in Cargo.toml")?
        .to_string();

    // Parse current major.minor
    let parts: Vec<&str> = current.splitn(3, '.').collect();
    let major = parts.first().context("missing major version")?;
    let minor = parts.get(1).context("missing minor version")?;

    // Convert "2026-04-01" to "20260401"
    let date_part = iana_date.replace('-', "");

    let new = format!("{}.{}.{}", major, minor, date_part);

    if current == new {
        anyhow::bail!(
            "version {} already matches IANA date, \
             no automatic bump possible",
            current
        );
    }

    doc["package"]["version"] = toml_edit::value(new.clone());

    fs::write(CARGO_TOML, doc.to_string()).context("failed to write libmime/Cargo.toml")?;

    Ok((current, new))
}

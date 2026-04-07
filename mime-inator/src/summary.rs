use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;

const SUMMARY_PATH: &str = "mime-inator-summary.json";

#[derive(Serialize)]
pub(crate) struct Summary {
    pub(crate) changed: bool,
    pub(crate) iana_date: String,
    pub(crate) entry_count: usize,
    pub(crate) version_current: Option<String>,
    pub(crate) version_new: Option<String>,
    pub(crate) added: Vec<String>,
    pub(crate) removed: Vec<String>,
}

impl Summary {
    pub(crate) fn write(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("failed to serialize summary")?;

        fs::write(SUMMARY_PATH, &json)
            .context("failed to write summary")?;

        info!("  Wrote {}", SUMMARY_PATH);
        Ok(())
    }
}
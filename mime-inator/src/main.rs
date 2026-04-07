use anyhow::Result;
use log::LevelFilter;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub(crate) mod generator;
pub(crate) mod iana;
pub(crate) mod summary;
pub(crate) mod version;

use crate::summary::Summary;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    info!("Mime-inator activated!");

    info!("Fetching IANA registry...");
    let (entries, iana_date) = crate::iana::fetch_entries().await?;
    info!("Found {} MIME types", entries.len());
    info!("IANA last updated: {}", iana_date);

    info!("Generating code...");
    let diff = crate::generator::generate(&entries)?;

    if !diff.changed {
        info!("No changes detected. Nothing to do.");
        let summary = Summary {
            changed: false,
            iana_date,
            entry_count: entries.len(),
            version_current: None,
            version_new: None,
            added: vec![],
            removed: vec![],
        };
        summary.write()?;
        return Ok(());
    }

    info!("{} added, {} removed", diff.added.len(), diff.removed.len());

    info!("Updating version...");
    let (current, new) = crate::version::update(&iana_date)?;
    info!("Version: {} -> {}", current, new);

    let summary = Summary {
        changed: true,
        iana_date,
        entry_count: entries.len(),
        version_current: Some(current),
        version_new: Some(new),
        added: diff.added,
        removed: diff.removed,
    };

    summary.write()?;

    info!(
        "Mime-inator complete! \
         Behold, {} MIME types of the entire tri-state area!",
        entries.len()
    );

    Ok(())
}

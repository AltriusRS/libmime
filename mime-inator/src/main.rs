use anyhow::Result;

pub(crate) mod generator;
pub(crate) mod iana;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Mime-inator activated!");
    println!();

    println!("Fetching IANA registry...");
    let client = reqwest::Client::builder()
        .user_agent("mime-inator/0.1.0")
        .build()?;

    let entries = crate::iana::fetch_entries(&client).await?;
    println!();
    println!("Found {} MIME types", entries.len());
    println!();

    println!("Generating code...");
    crate::generator::generate(&entries)?;

    println!();
    println!(
        "Mime-inator complete! \
         Behold, {} MIME types of the entire tri-state area!",
        entries.len()
    );

    Ok(())
}

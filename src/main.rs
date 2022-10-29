use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use keebgen::{Definition, InlineDefinition, Keyboard};

#[derive(Debug, Parser)]
struct Args {
    /// The path to the keyboard definition. If omitted, the definition
    /// will be read from stdin instead.
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let keyboard: Keyboard = if let Some(file) = args.file {
        let reader = std::fs::OpenOptions::new()
            .read(true)
            .open(file)
            .context("Failed to read definition file")?;
        serde_json::from_reader(reader).context("Failed to parse definition file")?
    } else {
        let reader = std::io::stdin().lock();
        serde_json::from_reader(reader).context("Failed to parse definition from stdin")?
    };

    let definitions = retrieve_definitions(&keyboard.definitions[..]).await?;

    println!("Keyboard:\n{keyboard:?}\n\nDefinitions:\n{definitions:?}");

    Ok(())
}

/// Retrueve remove definitions from external git repositories
async fn retrieve_definitions(
    _definitions: &[Definition],
) -> anyhow::Result<Vec<InlineDefinition>> {
    Ok(vec![])
}

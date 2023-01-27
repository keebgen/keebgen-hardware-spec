use std::{io::Read, path::PathBuf};

use clap::Parser;
use keebgen::{Definition, InlineDefinition, KeyboardSpec};
use miette::{IntoDiagnostic, WrapErr};

#[derive(Debug, Parser)]
struct Args {
    /// The path to the keyboard definition. If omitted, the definition
    /// will be read from stdin instead.
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();
    let content = read_input(&args.file)?;
    let specs = match parse_specs(args.file, &content) {
        Ok(specs) => specs,
        Err(error) => {
            println!("{:?}", miette::Report::new(error));
            std::process::exit(1);
        }
    };

    println!("{specs:?}");

    Ok(())
}

fn read_input(file: &Option<PathBuf>) -> miette::Result<String> {
    if let Some(file) = &file {
        std::fs::read_to_string(file)
            .into_diagnostic()
            .wrap_err("Failed to read file content")
    } else {
        let mut content = String::new();
        std::io::stdin()
            .read_to_string(&mut content)
            .into_diagnostic()
            .wrap_err("Failed to read from stdin")?;

        Ok(content)
    }
}

fn parse_specs(file: Option<PathBuf>, content: &str) -> miette::Result<Vec<KeyboardSpec>> {
    if let Some(file) = file {
        knuffel::parse(&file.as_os_str().to_string_lossy(), content).into_diagnostic()
    } else {
        knuffel::parse("stdin", content).into_diagnostic()
    }
}

/// Retrueve remove definitions from external git repositories
async fn retrieve_definitions(
    _definitions: &[Definition],
) -> miette::Result<Vec<InlineDefinition>> {
    Ok(vec![])
}

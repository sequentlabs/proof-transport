use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name="tesl")]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    /// placeholder: prints version
    Version,
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Version => println!("proof-transport v0.1.0"),
    }
    Ok(())
}

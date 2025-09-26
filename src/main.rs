use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::from_reader;
use std::fs::File;

use proof_transport::{ast::Proof, frag::fragility_score, validator::validate_local_wf};

#[derive(Parser)]
#[command(name = "proof-transport", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Fragility { path: String },
    Validate { path: String },
}

fn load(path: &str) -> Result<Proof> {
    Ok(from_reader(File::open(path)?)?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Fragility { path } => {
            let p = load(&path)?;
            println!("{}", fragility_score(&p));
        }
        Cmd::Validate { path } => {
            let p = load(&path)?;
            validate_local_wf(&p)?;
            println!("ok");
        }
    }
    Ok(())
}

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::from_reader;
use std::fs::File;

use proof_transport::{ast::Proof, frag::fragility_score, validate_local_wf};

#[derive(Parser)]
#[command(name = "proof-transport", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print fragility score of a proof JSON file
    Fragility { file: String },
    /// Validate local well-formedness of a proof JSON file
    Validate { file: String },
}

fn read_proof(path: &str) -> Result<Proof> {
    Ok(from_reader(File::open(path)?)?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Fragility { file } => {
            let p = read_proof(&file)?;
            println!("{}", fragility_score(&p));
        }
        Cmd::Validate { file } => {
            let p = read_proof(&file)?;
            validate_local_wf(&p)?;
            println!("OK");
        }
    }
    Ok(())
}

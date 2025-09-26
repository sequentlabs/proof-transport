use clap::{Parser, Subcommand};
use serde_json::from_reader;
use std::fs::File;
use anyhow::Result;

use proof_transport::{
    ast::Proof,
    validator::validate_local_wf,
    frag::fragility_score
};

#[derive(Parser)]
#[command(name="proof-transport", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Validate { file: String },
    Frag { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Validate { file } => {
            let p: Proof = from_reader(File::open(file)?)?;
            validate_local_wf(&p)?;
            println!("valid");
        }
        Cmd::Frag { file } => {
            let p: Proof = from_reader(File::open(file)?)?;
            println!("{}", fragility_score(&p));
        }
    }
    Ok(())
}

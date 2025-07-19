// src/main.rs
/*
 * Main executable for DigitalpaymentsLabs
 */

use clap::Parser;
use digitalpaymentslabs::{Result, run};

#[derive(Parser)]
#[command(version, about = DigitalpaymentsLabs - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

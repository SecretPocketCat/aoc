use std::env;

use clap::Parser;

use aoc_client::get_input;
use day_17::solution::{part_a, part_b};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run part b
    #[arg(short)]
    b: bool,
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let input = get_input(env::current_dir()?, 17).await?;
    let output = if cli.b {
        part_b(&input)?
    } else {
        part_a(&input)?
    };
    println!("{output}");
    Ok(())
}

mod cli;
mod runner;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    runner::run(cli.command)
}

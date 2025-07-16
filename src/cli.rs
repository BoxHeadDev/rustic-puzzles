use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RusticPuzzles")]
#[command(about = "Run coding puzzles by category", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Aoc {
        #[arg(help = "Year (e.g. 2023)")]
        year: u16,
        #[arg(help = "Day (e.g. 1, 2...)")]
        day: u8,
    },
    Euler {
        #[arg(help = "Problem number (e.g. 1)")]
        number: u16,
    },
    Csec {
        #[arg(help = "Problem ID (e.g. a, b, etc.)")]
        id: String,
    },
}

use crate::cli::Commands;
use anyhow::{Result, bail};

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Aoc { year, day } => match (year, day) {
            (2022, 1) => {
                let input = include_str!("../inputs/aoc_2022_day01.txt");
                let result = crate::aoc::year2022::day01::part1(input);
                println!("Result: {result}");
            }
            _ => bail!("AoC problem for year {} day {} not implemented", year, day),
        },

        Commands::Euler { number } => match number {
            1 => {
                let result = crate::euler::problem001::solve();
                println!("Result: {result}");
            }
            _ => bail!("Euler problem {} not implemented", number),
        },

        Commands::Csec { id } => match id.as_str() {
            "a" => {
                let result = crate::csec::problem_a::solve();
                println!("Result: {result}");
            }
            _ => bail!("CSEC problem {} not implemented", id),
        },
    }

    Ok(())
}

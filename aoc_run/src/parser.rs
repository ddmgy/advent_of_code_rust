use std::ops::RangeInclusive;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aoc", author, about, long_about = None)]
/// An Advent of Code tool
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Create solution
    Create(CreateArgs),
    /// Run solution
    Run(RunArgs),
}

#[derive(Args)]
pub(crate) struct CreateArgs {
    #[arg(short, long, default_value_t = 2023, value_parser = year_in_range)]
    /// Number in range [2015, 2023]
    pub(crate) year: usize,

    #[arg(short, long, value_parser = day_in_range)]
    /// Number in range [1, 25]
    pub(crate) day: usize,

    #[arg(short = 'V')]
    pub(crate) show_version: bool,
}

#[derive(Args)]
pub(crate) struct RunArgs {
    #[arg(short, long, default_value_t = 2023, value_parser = year_in_range)]
    /// Number in range [2015, 2023]
    pub(crate) year: usize,

    #[arg(short, long, value_parser = day_in_range)]
    /// Number in range [1, 25]
    pub(crate) day: usize,

    #[arg(short = 'a')]
    /// Run solution or tests for part A
    pub(crate) run_part_a: bool,

    #[arg(short = 'b')]
    /// Run solution or tests for part B
    pub(crate) run_part_b: bool,

    #[arg(short = 'v', long)]
    /// Alternate version of given solution to run
    pub(crate) version: Option<String>,

    #[arg(short = 'V')]
    pub(crate) show_version: bool,
}

/// Valid years.
///
/// Will need to update upper bound for each new Advent of Code event.
const YEAR_RANGE: RangeInclusive<usize> = 2015..=2023;

/// Valid days.
const DAY_RANGE: RangeInclusive<usize> = 1..=25;

fn year_in_range(s: &str) -> Result<usize, String> {
    let year: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid year"))?;

    if YEAR_RANGE.contains(&year) {
        Ok(year)
    } else {
        Err(format!(
            "year not in range {}-{}",
            YEAR_RANGE.start(),
            YEAR_RANGE.end(),
        ))
    }
}

fn day_in_range(s: &str) -> Result<usize, String> {
    let day: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid day"))?;

    if DAY_RANGE.contains(&day) {
        Ok(day)
    } else {
        Err(format!(
            "day not in range {}-{}",
            DAY_RANGE.start(),
            DAY_RANGE.end(),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;

        use super::*;

        Cli::command().debug_assert();
    }
}

use std::fs;
use std::path::Path;

use askama::Template;
use chrono::Duration;
use clap::{crate_version, Parser};
use itertools::Itertools;

mod parser;
use parser::*;
mod template;
use template::AocTemplate;

fn pretty_duration(duration: Duration) -> String {
    if duration < Duration::microseconds(1) {
        format!("{} ns", duration.num_nanoseconds().unwrap())
    } else if duration < Duration::milliseconds(1) {
        format!(
            "{:.2} µs",
            duration.num_nanoseconds().unwrap() as f32 / 1000.0,
        )
    } else if duration < Duration::seconds(1) {
        format!(
            "{:.2} ms",
            duration.num_microseconds().unwrap() as f32 / 1000.0,
        )
    } else {
        format!(
            "{:.2} s",
            duration.num_milliseconds() as f32 / 1000.0,
        )
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(args) => {
            let CreateArgs {
                year,
                day,
                input_type,
                show_version,
            } = *args;

            if show_version {
                println!("aoc v{}", crate_version!());
                return Ok(());
            }

            let current_dir = std::env::current_dir().unwrap();
            let input_dir_name = format!(
                "{}/data/input/y{year}",
                current_dir.display(),
            );
            let input_dir = Path::new(&input_dir_name);

            if !input_dir.exists() {
                fs::create_dir_all(input_dir)?;
            }

            let input_file = input_dir.join(format!(
                "{}/day{day:02}.txt",
                input_dir.display(),
            ));

            if !input_file.exists() {
                fs::write(input_file, "")?;
            }

            let solution_dir_name = format!(
                "{}/aoc/src/solutions/y{year}",
                current_dir.display(),
            );
            let solution_dir = Path::new(&solution_dir_name);

            if !solution_dir.exists() {
                fs::create_dir_all(solution_dir)?;
            }

            let solution_file = solution_dir.join(format!(
                "{}/day{day:02}.rs",
                solution_dir.display(),
            ));

            if !solution_file.exists() {
                let (input_name, input_type) =
                    (input_type.name_as_string(), input_type.type_as_string());

                let temp = AocTemplate::new(year, day, input_name, input_type);
                fs::write(solution_file, temp.render()?)?;
            }

            Ok(())
        },
        Commands::Run(args) => {
            let RunArgs {
                year,
                day,
                run_part_a,
                run_part_b,
                version: _,
                show_version,
            } = *args;
            let version = args.version.clone();

            if show_version {
                println!("aoc v{}", crate_version!());
                return Ok(());
            }

            aoc::register_runners();

            if run_part_a {
                run(year, day, aoc_common::Part::A, version.clone());
            }
            if run_part_b {
                run(year, day, aoc_common::Part::B, version.clone());
            }

            Ok(())
        },
    }

}

fn run(year: usize, day: usize, part: aoc_common::Part, version: Option<String>) {
    match aoc::get_runner(year, day, part, version.clone()) {
        Ok(runner) => {
            let before = chrono::Utc::now();
            let result = runner();
            let after = chrono::Utc::now();
            let elapsed = format!(" ({})", pretty_duration(after - before));
            let header = format!(
                r#"Solution for {year}/{day:02} part {part}{}:"#,
                version.clone().map(|v| format!(r#" (version "{v}")"#)).unwrap_or_default(),
            );
            let sep = format!("\n{}", " ".repeat(header.len()));

            match result {
                Ok(v) => {
                    let result = v.lines().join(&sep);
                    println!("{header} {result}{elapsed}");
                },
                Err(e) => {
                    eprintln!("<error: {}>", e);
                    for cause in e.chain() {
                        eprintln!("  {}", cause);
                    }
                    eprintln!("  source: {:?}", e.source());
                    eprintln!("  root: {:?}", e.root_cause());
                },
            }
        },
        Err(e) => {
            eprintln!("<error: {}>", e);
        },
    }
}

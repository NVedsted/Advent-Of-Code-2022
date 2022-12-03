use std::{fs, io};
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use colored::Colorize;

use crate::day::{DayReport, PartReport};

mod day;
mod days;

#[derive(Parser)]
#[command(author, about)]
struct Args {
    #[arg(help = "Specifies the day to solve. Otherwise, all days are run. (1-25)")]
    day: Option<usize>,

    #[arg(help = "Read from standard input.", long)]
    stdin: bool,

    #[arg(help = "The file to read input from. If not provided, input/day<#>.in will be used.", long, short)]
    file: Option<String>,

    #[arg(help = "Attempts to validate outputs.", long, short)]
    validate: bool,

    #[arg(help = "Show timings.", long, short)]
    timings: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(day) = args.day {
        if run_day(day, &args).is_none() {
            println!("{}", format!("No solver for day {}", day).bright_yellow());
        }
    } else {
        let days = (1..=25)
            .map(|i| run_day(i, &args))
            .collect::<Vec<_>>();

        if args.timings {
            let total = days
                .into_iter()
                .filter_map(|r| r.and_then(|r| r.total_timing))
                .sum::<Duration>();

            println!("\nTotal time: {}ms", total.as_millis());
        }
    }
}

fn run_day(day: usize, args: &Args) -> Option<DayReport> {
    if day < 1 || day > 25 {
        println!("{}", "Day out of bounds".red());
        return None;
    }

    let Some(solver) = &days::DAYS[day - 1] else {
        return None;
    };

    let input = if args.stdin {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        input
    } else {
        let filename = match &args.file {
            Some(f) => f.to_owned(),
            None => format!("input/day{}.in", day),
        };
        let path = Path::new(&filename);

        if !path.exists() {
            println!("{}", format!("Day {} has no input", day).red());
            return None;
        }

        fs::read_to_string(filename).unwrap()
    }.replace('\r', "");

    let report = solver.solve(&input);

    print!("Day {}", day);

    if let Some(total_timing) = report.total_timing {
        if args.timings {
            print!(" ({}ms)", total_timing.as_millis());
        }
    }

    println!();

    present_part(day, 1, args.validate, args.timings, &report.part1);
    present_part(day, 2, args.validate, args.timings, &report.part2);

    Some(report)
}

fn present_part(day: usize, part: usize, validate: bool, timings: bool, report: &Option<PartReport>) {
    print!("\tPart {}: ", part);
    if let Some(report) = report {
        print!("{}", report.result.blue());

        if let Some(timing) = report.timing {
            if timings {
                print!(" ({}ms)", timing.as_millis());
            }
        }

        println!();

        if validate {
            validate_part(day, part, &report.result);
        }
    } else {
        println!("{}", "Not implemented".bright_yellow());
    }
}

fn validate_part(day: usize, part: usize, actual: &str) {
    let filename = format!("input/day{}.{}.out", day, part);
    let path = Path::new(&filename);

    if !path.exists() {
        println!("\t{}", "No validation file".bright_yellow());
    } else {
        let expected = fs::read_to_string(path).unwrap();

        if actual == expected {
            println!("\t{}", "Validated".green());
        } else {
            println!("{}", format!("Day {} part {} failed validation!\nExpected: {}\nActual: {}", day, part, expected, actual).red());
        }
    }
}

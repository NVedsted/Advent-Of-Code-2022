use std::{fs, io};
use std::io::Read;
use std::path::Path;

use clap::Parser;
use colored::Colorize;

use crate::day::DaySolver;

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
}

fn main() {
    let args = Args::parse();

    if let Some(day) = args.day {
        run_day(day, &args);
    } else {
        (1..=25)
            .for_each(|i| run_day(i, &args));
    }
}

fn run_day(day: usize, args: &Args) {
    if day < 1 || day > 25 {
        panic!("Day out of bounds");
    }

    println!("Day {}", day);

    let Some(solver) = &days::DAYS[day - 1] else {
        println!("\t{}", "No solver".bright_yellow());
        return;
    };

    let input = if args.stdin {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to get input");

        input
    } else {
        let filename = match &args.file {
            Some(f) => f.to_owned(),
            None => format!("input/day{}.in", day),
        };
        let path = Path::new(&filename);

        if !path.exists() {
            println!("\t{}", "No input".red());
            return;
        }

        fs::read_to_string(filename).unwrap()
    }.replace('\r', "");

    let (part1, part2) = solve_day(solver, &input);
    present_part(day, 1, args.validate, &part1);
    present_part(day, 2, args.validate, &part2);
}

fn present_part(day: usize, part: usize, validate: bool, result: &Option<String>) {
    print!("\tPart {}: ", part);
    if let Some(result) = result {
        println!("{}", result.blue());

        if validate {
            validate_part(day, part, &result);
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
            panic!("Day {} part {} failed validation!\nExpected: {}\nActual: {}", day, part, expected, actual);
        }
    }
}

fn solve_day(solver: &DaySolver, input: &str) -> (Option<String>, Option<String>) {
    match solver {
        DaySolver::Standard { part1, part2 } =>
            (part1.map(|s1| s1(input)), part2.map(|s2| s2(input))),
        DaySolver::Double(solver) => {
            let (s1, s2) = solver(input);
            (Some(s1), Some(s2))
        }
    }
}

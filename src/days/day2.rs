use Move::*;
use Outcome::*;

use crate::day::DaySolver;

pub const DAY_2: DaySolver = DaySolver::Standard { part1: Some(part_1), part2: Some(part_2) };

#[derive(Eq, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(self) -> Move {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn beaten_by(self) -> Move {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn score(self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Invalid"),
        }
    }
}

struct Round<T> {
    observation: Move,
    action: T,
}

impl<'a, T: From<&'a str>> From<&'a str> for Round<T> {
    fn from(s: &'a str) -> Self {
        let (opponent, play) = s.trim()
            .split_once(' ').unwrap();

        Round {
            observation: opponent.into(),
            action: play.into(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Invalid"),
        }
    }
}

fn outcome(play: Move, opponent: Move) -> Outcome {
    if play == opponent {
        Draw
    } else if play == opponent.beaten_by() {
        Win
    } else {
        Loss
    }
}

fn move_from_outcome(opponent: Move, outcome: Outcome) -> Move {
    match outcome {
        Draw => opponent,
        Win => opponent.beaten_by(),
        Loss => opponent.beats(),
    }
}

fn solver<'a, T: From<&'a str>, F: FnMut(Round<T>) -> i32>(input: &'a str, mapper: F) -> String {
    input.lines().map(Round::<T>::from).map(mapper).sum::<i32>().to_string()
}

fn part_1(input: &str) -> String {
    solver(input, |r: Round<Move>| outcome(r.action, r.observation).score() + r.action.score())
}

fn part_2(input: &str) -> String {
    solver(input, |r: Round<Outcome>| r.action.score() + move_from_outcome(r.observation, r.action).score())
}
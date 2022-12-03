use std::time::{Duration, Instant};

pub type PartSolver = fn(&str) -> String;

pub type DoubleSolver = fn(&str) -> (String, String);

pub enum DaySolver {
    Standard { part1: Option<PartSolver>, part2: Option<PartSolver> },
    Double(DoubleSolver),
}

pub struct PartReport {
    pub result: String,
    pub timing: Option<Duration>,
}

pub struct DayReport {
    pub part1: Option<PartReport>,
    pub part2: Option<PartReport>,
    pub total_timing: Option<Duration>,
}

fn solve_part(solver: &Option<PartSolver>, input: &str) -> Option<PartReport> {
    let Some(solver) = solver else {
        return None;
    };

    let start = Instant::now();
    let result = solver(input);
    let time = start.elapsed();
    Some(PartReport {
        result,
        timing: Some(time),
    })
}

impl DaySolver {
    pub fn solve(&self, input: &str) -> DayReport {
        match self {
            DaySolver::Standard { part1, part2 } => {
                let part1 = solve_part(part1, input);
                let part2 = solve_part(part2, input);
                let total_timing = match (&part1, &part2) {
                    | (Some(PartReport { timing: Some(t1), .. }), Some(PartReport { timing: Some(t2), .. })) => Some(*t1 + *t2),
                    _ => None,
                };

                DayReport { part1, part2, total_timing }
            }
            DaySolver::Double(solver) => {
                let start = Instant::now();
                let (s1, s2) = solver(input);
                let total_timing = start.elapsed();

                DayReport {
                    part1: Some(PartReport { result: s1, timing: None }),
                    part2: Some(PartReport { result: s2, timing: None }),
                    total_timing: Some(total_timing),
                }
            }
        }
    }
}
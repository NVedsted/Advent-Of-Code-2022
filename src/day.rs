pub type PartSolver = fn(&str) -> String;

pub type DoubleSolver = fn(&str) -> (String, String);

#[allow(dead_code)]
pub enum DaySolver {
    Standard { part1: Option<PartSolver>, part2: Option<PartSolver> },
    Double(DoubleSolver),
}
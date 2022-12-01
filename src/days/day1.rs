use crate::day::DaySolver;

pub const DAY_1: DaySolver = DaySolver::Double(solver);

fn solver(input: &str) -> (String, String) {
    let mut sums = input
        .split("\n\n")
        .map(|e| e.lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<_>>();
    sums.sort_by(|a, b| b.cmp(a));
    let part1 = sums.first().unwrap().to_string();
    let part2 = sums.into_iter().take(3).sum::<i32>().to_string();

    (part1, part2)
}
use std::collections::HashSet;
use crate::day::DaySolver;

pub const DAY_6: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

fn solver(input: &str, window: usize) -> String {
    let characters = input.chars().collect::<Vec<_>>();

    (characters.windows(window).enumerate()
        .filter(|(_, w)| (HashSet::<char>::from_iter((w).iter().cloned()).len()) == (w.len()))
        .next().unwrap().0 + window).to_string()
}

fn part1(input: &str) -> String {
    solver(input, 4)
}

fn part2(input: &str) -> String {
    solver(input, 14)
}

#[cfg(test)]
mod tests {
    use crate::days::day6::*;

    #[test]
    fn test_part1() {
        assert_eq!("7", part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!("5", part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!("6", part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!("10", part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!("11", part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part2() {
        assert_eq!("19", part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!("23", part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!("23", part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!("29", part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!("26", part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
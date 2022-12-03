use std::collections::HashSet;
use crate::day::DaySolver;

pub const DAY_3: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

fn priority(x: char) -> i32 {
    if x.is_ascii_lowercase() {
        x as i32 - 'a' as i32 + 1
    } else {
        x as i32 - 'A' as i32 + 27
    }
}

fn intersection(sets: &[HashSet<char>]) -> Vec<char> {
    let Some((first, rest)) = sets.split_first() else {
        return vec![];
    };
    let mut result = first.iter().cloned().collect::<Vec<_>>();
    rest.iter().for_each(|s| result.retain(|e| s.contains(e)));
    result
}

fn part1(input: &str) -> String {
    input
        .lines()
        .flat_map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            let first = first.chars().collect::<HashSet<_>>();
            let second = second.chars().collect::<HashSet<_>>();
            intersection(&[first, second])
        })
        .map(priority).sum::<i32>().to_string()
}

fn part2(input: &str) -> String {
    let rucksacks = input
        .lines()
        .map(|l| l.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    rucksacks.chunks_exact(3)
        .flat_map(intersection)
        .map(priority).sum::<i32>().to_string()
}


#[cfg(test)]
mod tests {
    use crate::days::day3::*;

    const EXAMPLE_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!("157", part1(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("70", part2(EXAMPLE_INPUT));
    }
}
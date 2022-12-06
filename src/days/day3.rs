use crate::day::DaySolver;

pub const DAY_3: DaySolver = DaySolver::Double(solver);

fn priority(x: u8) -> usize {
    if x.is_ascii_lowercase() {
        x as usize - 'a' as usize + 1
    } else {
        x as usize - 'A' as usize + 27
    }
}

fn intersection(sets: &[&[u8]]) -> usize {
    let mut seen = [0; 53];

    sets.iter()
        .enumerate()
        .for_each(|(i, l)| l.iter().cloned().for_each(|e| {
            let p = priority(e);
            if seen[p] == i {
                seen[p] = i + 1;
            }
        }));

    seen.into_iter().enumerate().filter(|&(_, c)| c == sets.len()).next().unwrap().0
}

fn solver(input: &str) -> (String, String) {
    let lines = input.lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>();

    let part1 = lines.iter()
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            intersection(&[first, second])
        })
        .sum::<usize>().to_string();

    let part2 = lines.chunks_exact(3)
        .map(|w| intersection(&[&w[0], &w[1], &w[2]]))
        .sum::<usize>().to_string();

    (part1, part2)
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
    fn test() {
        let (part1, part2) = solver(EXAMPLE_INPUT);
        assert_eq!("157", part1);
        assert_eq!("70", part2);
    }
}
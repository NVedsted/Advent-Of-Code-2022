use crate::day::DaySolver;

pub const DAY_6: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

type CharacterCount = [usize; char_to_index('z') + 1];

const fn char_to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn no_duplicates(counts: &CharacterCount) -> bool {
    counts.iter().cloned().all(|c| c < 2)
}

fn solver(input: &str, window_size: usize) -> String {
    let characters = input.chars().collect::<Vec<_>>();

    let mut counts: CharacterCount = Default::default();
    characters.iter()
        .take(window_size).cloned()
        .map(char_to_index)
        .for_each(|i| counts[i] += 1);

    if no_duplicates(&counts) {
        return window_size.to_string();
    }

    for i in window_size..characters.len() {
        counts[char_to_index(characters[i - window_size])] -= 1;
        counts[char_to_index(characters[i])] += 1;

        if no_duplicates(&counts) {
            return (i + 1).to_string();
        }
    }
    panic!("No solution");
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
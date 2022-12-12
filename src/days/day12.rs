use std::collections::VecDeque;
use crate::day::DaySolver;

pub const DAY_12: DaySolver = DaySolver::Double(solver);

fn solver(input: &str) -> (String, String) {
    let mut map = input.lines()
        .map(|l| l.as_bytes().iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut starts = vec![];
    for r in 0..height {
        for c in 0..width {
            if map[r][c] == b'S' {
                start = (r, c);
                map[r][c] = b'a';
            } else if map[r][c] == b'E' {
                end = (r, c);
                map[r][c] = b'z';
            }

            if map[r][c] == b'a' {
                starts.push((r, c));
            }
        }
    }

    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    let mut queue = VecDeque::from([(end.0, end.1, 0)]);
    while let Some((r, c, d)) = queue.pop_front() {
        if distances[r][c].is_some() {
            continue;
        }
        distances[r][c] = Some(d);

        if r > 0 && map[r - 1][c] >= map[r][c] - 1 {
            queue.push_back((r - 1, c, d + 1));
        }

        if r < height - 1 && map[r + 1][c] >= map[r][c] - 1 {
            queue.push_back((r + 1, c, d + 1));
        }

        if c > 0 && map[r][c - 1] >= map[r][c] - 1 {
            queue.push_back((r, c - 1, d + 1));
        }

        if c < width - 1 && map[r][c + 1] >= map[r][c] - 1 {
            queue.push_back((r, c + 1, d + 1));
        }
    }

    let part1 = distances[start.0][start.1].unwrap().to_string();
    let part2 = starts.into_iter()
        .filter_map(|s| distances[s.0][s.1])
        .min().unwrap().to_string();
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use crate::days::day12::*;

    const EXAMPLE_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test() {
        assert_eq!(("31".to_owned(), "29".to_owned()), solver(EXAMPLE_INPUT));
    }
}
use crate::day::DaySolver;

pub const DAY_4: DaySolver = DaySolver::Double(solver);

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn contains(&self, point: i32) -> bool {
        point >= self.from && point <= self.to
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.fully_contains(other) || other.contains(self.from) || other.contains(self.to)
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (from, to) = s.trim().split_once('-').unwrap();
        Self {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

struct Pair {
    first: Range,
    second: Range,
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (first, second) = s.trim().split_once(',').unwrap();
        Self {
            first: first.into(),
            second: second.into(),
        }
    }
}

fn solver(input: &str) -> (String, String) {
    let pairs = input
        .lines()
        .map(Pair::from)
        .collect::<Vec<_>>();

    let part1 = pairs.iter()
        .filter(|p| p.first.fully_contains(&p.second) || p.second.fully_contains(&p.first))
        .count();

    let part2 = pairs.into_iter()
        .filter(|p| p.first.overlaps(&p.second))
        .count();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use crate::days::day4::*;

    const EXAMPLE_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_solver() {
        let (part1, part2) = solver(EXAMPLE_INPUT);
        assert_eq!("2", part1);
        assert_eq!("4", part2);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(false, Range { from: 2, to: 4 }.overlaps(&Range { from: 6, to: 8 }));
        assert_eq!(false, Range { from: 6, to: 8 }.overlaps(&Range { from: 2, to: 4 }));
        assert_eq!(true, Range { from: 2, to: 4 }.overlaps(&Range { from: 3, to: 8 }));
        assert_eq!(true, Range { from: 3, to: 8 }.overlaps(&Range { from: 2, to: 4 }));
        assert_eq!(true, Range { from: 2, to: 8 }.overlaps(&Range { from: 4, to: 6 }));
        assert_eq!(true, Range { from: 4, to: 6 }.overlaps(&Range { from: 2, to: 8 }));
    }
}
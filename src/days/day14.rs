use std::cmp::{max, min};
use State::*;
use crate::day::DaySolver;

pub const DAY_14: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

struct Coordinate {
    x: usize,
    y: usize,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Self { x: x.parse().unwrap(), y: y.parse().unwrap() }
    }
}

struct Path {
    path: Vec<Coordinate>,
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Self { path: s.split(" -> ").map(Coordinate::from).collect() }
    }
}

impl Path {
    fn apply_to_map(&self, map: &mut Vec<Vec<State>>) {
        let (mut previous, rest) = self.path.split_first().unwrap();
        for next in rest {
            if previous.x == next.x {
                for y in min(previous.y, next.y)..=max(previous.y, next.y) {
                    map[previous.x][y] = Wall;
                }
            } else {
                for x in min(previous.x, next.x)..=max(previous.x, next.x) {
                    map[x][previous.y] = Wall;
                }
            }
            previous = next;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Air,
    Wall,
    Sand,
}

fn put_sand(map: &mut Vec<Vec<State>>, width: usize, height: usize, bottom_void: bool) -> Option<Coordinate> {
    let (mut x, mut y) = (500, 0);
    loop {
        if y == height - 1 {
            return if bottom_void {
                None
            } else {
                map[x][y] = Sand;
                Some(Coordinate { x, y })
            };
        }

        if map[x][y + 1] == Air {
            y += 1;
        } else if x == 0 {
            return if bottom_void { None } else { todo!("Solid-bottom overflow not implemented") };
        } else if x > 0 && map[x - 1][y + 1] == Air {
            y += 1;
            x -= 1;
        } else if x == width - 1 {
            return if bottom_void { None } else { todo!("Solid-bottom overflow not implemented") };
        } else if x < width - 1 && map[x + 1][y + 1] == Air {
            y += 1;
            x += 1;
        } else {
            map[x][y] = Sand;
            return Some(Coordinate { x, y });
        }
    }
}

fn part1(input: &str) -> String {
    let paths = input.lines()
        .map(Path::from)
        .collect::<Vec<_>>();

    let width = paths
        .iter()
        .map(|p| p.path.iter().map(|p| p.x).max().unwrap())
        .max().unwrap() + 1;
    let height = paths
        .iter()
        .map(|p| p.path.iter().map(|p| p.y).max().unwrap())
        .max().unwrap() + 1;

    let mut map = vec![vec![Air; height]; width];
    paths.iter().for_each(|p| p.apply_to_map(&mut map));

    let mut sand = 0;
    while put_sand(&mut map, width, height, true).is_some() {
        sand += 1;
    }

    sand.to_string()
}

fn part2(input: &str) -> String {
    let paths = input.lines()
        .map(Path::from)
        .collect::<Vec<_>>();

    // TODO: intelligently handle flow to sides without the need to guess the needed width.
    let width = paths
        .iter()
        .map(|p| p.path.iter().map(|p| p.x).max().unwrap())
        .max().unwrap() * 2 + 1;
    let height = paths
        .iter()
        .map(|p| p.path.iter().map(|p| p.y).max().unwrap())
        .max().unwrap() + 2;

    let mut map = vec![vec![Air; height]; width];
    paths.iter().for_each(|p| p.apply_to_map(&mut map));

    let mut sand = 0;
    while let Some(position) = put_sand(&mut map, width, height, false) {
        sand += 1;

        if position.x == 500 && position.y == 0 {
            break;
        }
    }
    sand.to_string()
}

#[cfg(test)]
mod tests {
    use crate::days::day14::*;

    const EXAMPLE_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!("24", part1(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("93", part2(EXAMPLE_INPUT));
    }
}
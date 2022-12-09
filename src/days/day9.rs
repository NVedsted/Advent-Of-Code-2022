use std::collections::HashSet;

use crate::day::DaySolver;
use crate::days::day9::Direction::*;

pub const DAY_9: DaySolver = DaySolver::Double(solver);

#[derive(Copy, Clone)]
enum Direction { Right, Left, Up, Down }

impl Direction {
    fn direction(self) -> (i32, i32) {
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, 1),
            Down => (0, -1),
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => panic!(),
        }
    }
}

struct Action {
    direction: Direction,
    distance: i32,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let (direction, distance) = s.split_once(' ').unwrap();
        Self {
            direction: direction.into(),
            distance: distance.parse().unwrap(),
        }
    }
}

fn update_tail(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    let horizontal_disjoint = (head.0 - tail.0).abs() > 1;
    let vertical_disjoint = (head.1 - tail.1).abs() > 1;

    if horizontal_disjoint && vertical_disjoint {
        tail.0 += (head.0 - tail.0).signum();
        tail.1 += (head.1 - tail.1).signum();
    } else if horizontal_disjoint {
        tail.0 += (head.0 - tail.0).signum();
        tail.1 = head.1;
    } else if vertical_disjoint {
        tail.1 += (head.1 - tail.1).signum();
        tail.0 = head.0;
    }
    tail
}

fn solver(input: &str) -> (String, String) {
    let actions = input.lines().map(Action::from).collect::<Vec<_>>();
    let part1 = count_tail_unique_visits(&actions, 2).to_string();
    let part2 = count_tail_unique_visits(&actions, 10).to_string();
    (part1, part2)
}

fn count_tail_unique_visits(actions: &[Action], length: usize) -> usize {
    let mut snake = vec![(0, 0); length];

    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));

    for action in actions {
        let (dx, dy) = action.direction.direction();
        for _ in 0..action.distance {
            snake[0].0 += dx;
            snake[0].1 += dy;

            for i in 0..snake.len() - 1 {
                snake[i + 1] = update_tail(snake[i], snake[i + 1]);
            }

            tail_positions.insert(snake.last().unwrap().clone());
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use crate::days::day9::*;

    const EXAMPLE_INPUT_1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_INPUT_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_1() {
        assert_eq!(("13".to_owned(), "1".to_owned()), solver(EXAMPLE_INPUT_1));
    }


    #[test]
    fn test_2() {
        assert_eq!("36", solver(EXAMPLE_INPUT_2).1);
    }
}
use std::collections::VecDeque;

use crate::day::DaySolver;
use crate::days::day11::Operand::*;
use crate::days::day11::OperationType::*;

pub const DAY_11: DaySolver = DaySolver::Double(solver);

#[derive(Clone)]
enum Operand {
    Old,
    Value(usize),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        if s == "old" {
            Old
        } else {
            Value(s.parse().unwrap())
        }
    }
}

#[derive(Clone)]
enum OperationType {
    Plus,
    Multiply,
}

impl From<&str> for OperationType {
    fn from(s: &str) -> Self {
        match s {
            "*" => Multiply,
            "+" => Plus,
            _ => panic!("Unsupported operation type"),
        }
    }
}

#[derive(Clone)]
struct Operation {
    left: Operand,
    operation: OperationType,
    right: Operand,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let mut split = s.split(' ');

        let left = split.next().unwrap().into();
        let operation = split.next().unwrap().into();
        let right = split.next().unwrap().into();

        Operation { left, operation, right }
    }
}

impl Operation {
    fn execute(&self, old: usize) -> usize {
        let left = match self.left {
            Old => old,
            Value(v) => v,
        };
        let right = match self.right {
            Old => old,
            Value(v) => v,
        };
        match self.operation {
            Plus => left + right,
            Multiply => left * right,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    starting_items: VecDeque<usize>,
    operation: Operation,
    test_divisible_by: usize,
    test_true_throws_to: usize,
    test_false_throws_to: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1);

        let starting_items = lines.next().unwrap()[18..].split(", ")
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>();

        let operation = lines.next().unwrap()[19..].into();

        let test_divisible_by = lines.next().unwrap()[21..].parse().unwrap();
        let test_true_throws_to = lines.next().unwrap()[29..].parse().unwrap();
        let test_false_throws_to = lines.next().unwrap()[30..].parse().unwrap();

        Self {
            starting_items,
            operation,
            test_divisible_by,
            test_true_throws_to,
            test_false_throws_to,
        }
    }
}

fn solver(input: &str) -> (String, String) {
    let monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    let part1 = process_rounds(monkeys.clone(), 20, true);
    let part2 = process_rounds(monkeys, 10000, false);
    (part1, part2)
}

fn process_rounds(mut monkeys: Vec<Monkey>, rounds: usize, with_worry_reduction: bool) -> String {
    let crt_mod = monkeys.iter().map(|m| m.test_divisible_by).product::<usize>();
    let mut inspection_count = vec![0usize; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut worry) = monkeys[i].starting_items.pop_front() {
                inspection_count[i] += 1;
                worry = monkeys[i].operation.execute(worry);
                if with_worry_reduction {
                    worry /= 3;
                }
                worry %= crt_mod;
                let throw_to = if worry % monkeys[i].test_divisible_by == 0 {
                    monkeys[i].test_true_throws_to
                } else {
                    monkeys[i].test_false_throws_to
                };
                monkeys[throw_to].starting_items.push_back(worry);
            }
        }
    }

    inspection_count.sort();
    inspection_count.into_iter().rev().take(2).product::<usize>().to_string()
}


#[cfg(test)]
mod tests {
    use crate::days::day11::*;

    const EXAMPLE_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test() {
        assert_eq!(("10605".to_owned(), "2713310158".to_owned()), solver(EXAMPLE_INPUT));
    }
}
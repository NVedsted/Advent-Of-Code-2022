use std::cmp::Ordering;
use crate::day::DaySolver;
use crate::days::day13::Value::{Integer, List};

pub const DAY_13: DaySolver = DaySolver::Double(solver);

#[derive(Clone, Eq, PartialEq)]
enum Value { Integer(usize), List(Vec<Value>) }

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (&Integer(i), l @ List(_)) => List(vec![Integer(i)]).partial_cmp(l).unwrap(),
            (l @ List(_), &Integer(i)) => l.partial_cmp(&List(vec![Integer(i)])).unwrap(),
            (List(l1), List(l2)) => l1.iter()
                .zip(l2)
                .map(|(l, r)| l.partial_cmp(r).unwrap())
                .filter(|r| r != &Ordering::Equal)
                .next().unwrap_or(l1.len().cmp(&l2.len())),
        })
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let mut stack = vec![];
        let mut current_integer = None;
        for c in s.as_bytes().iter().cloned() {
            if c == b'[' {
                stack.push(List(vec![]));
            } else if c.is_ascii_digit() {
                current_integer = Some(current_integer.unwrap_or(0) * 10 + (c - b'0') as usize);
            } else if current_integer.is_some() && (c == b',' || c == b']') {
                let List(list) = stack.last_mut().unwrap() else {
                    panic!("tried adding integer to non-list");
                };
                list.push(Integer(current_integer.unwrap()));
                current_integer = None;
            }

            if c == b']' {
                let done = stack.pop().unwrap();
                let Some(List(list)) = stack.last_mut() else {
                    return done;
                };
                list.push(done);
            }
        }

        panic!("outer list was never closed")
    }
}

fn solver(input: &str) -> (String, String) {
    let mut values = input.replace("\n\n", "\n").lines()
        .map(Value::from)
        .collect::<Vec<_>>();

    let part1 = values.chunks_exact(2).enumerate()
        .filter(|(_, l)| l[0] < l[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>().to_string();

    let marker1 = List(vec![List(vec![Integer(2)])]);
    let marker2 = List(vec![List(vec![Integer(6)])]);
    values.push(marker1.clone());
    values.push(marker2.clone());
    values.sort();
    let index1 = values.binary_search(&marker1).unwrap() + 1;
    let index2 = values.binary_search(&marker2).unwrap() + 1;
    let part2 = (index1 * index2).to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use crate::days::day13::*;

    const EXAMPLE_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test() {
        assert_eq!(("13".to_owned(), "140".to_owned()), solver(EXAMPLE_INPUT));
    }
}
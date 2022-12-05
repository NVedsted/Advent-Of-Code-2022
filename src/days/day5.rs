use crate::day::DaySolver;

pub const DAY_5: DaySolver = DaySolver::Double(solver);

type Stack = Vec<char>;

struct Action {
    quantity: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let (quantity, rest) = s.trim()[5..].split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();

        Self {
            quantity: quantity.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        }
    }
}

fn build_stacks(cargo_drawing: &str) -> Vec<Stack> {
    let cargo_lines = cargo_drawing.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .map(|l| l.chunks(4).enumerate()
            .filter(|(_, c)| !c[0].is_ascii_whitespace())
            .map(|(i, c)| (i, c[1]))
            .collect::<Vec<_>>())
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut stacks = vec![Stack::new(); cargo_lines.last().unwrap().len()];
    cargo_lines.into_iter().rev()
        .for_each(|l| l.into_iter().for_each(|(i, c)| stacks[i].push(c)));
    stacks
}

fn solver(input: &str) -> (String, String) {
    let (cargo_drawing, actions) = input.split_once("\n\n").unwrap();
    let initial_stacks = build_stacks(cargo_drawing);
    let actions = actions.lines().map(Action::from).collect::<Vec<_>>();

    let part1 = solve_part(initial_stacks.clone(), &actions, |stacks, action| {
        let start = stacks[action.from].len() - action.quantity;
        let values = stacks[action.from].drain(start..).rev().collect::<Vec<_>>();
        stacks[action.to].extend_from_slice(&values);
    });

    let part2 = solve_part(initial_stacks.clone(), &actions, |stacks, action| {
        let start = stacks[action.from].len() - action.quantity;
        let values = stacks[action.from].drain(start..).collect::<Vec<_>>();
        stacks[action.to].extend_from_slice(&values);
    });

    (part1, part2)
}

fn solve_part(mut stacks: Vec<Stack>, actions: &[Action], execute_action: fn(&mut Vec<Stack>, &Action)) -> String {
    actions.iter().for_each(|a| execute_action(&mut stacks, a));

    stacks.into_iter()
        .filter_map(|l| l.last().cloned())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::days::day5::*;

    const EXAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test() {
        let (part1, part2) = solver(&EXAMPLE_INPUT);
        assert_eq!("CMZ", part1);
        assert_eq!("MCD", part2);
    }
}
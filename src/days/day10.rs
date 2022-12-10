use crate::day::DaySolver;
use crate::days::day10::Command::*;

pub const DAY_10: DaySolver = DaySolver::Double(solver);

#[derive(Copy, Clone, Debug)]
enum Command {
    Noop,
    AddX(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match &s[..4] {
            "noop" => Noop,
            "addx" => AddX(s[5..].parse().unwrap()),
            _ => panic!("Invalid command"),
        }
    }
}

impl Command {
    fn cycles(self) -> usize {
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }
}

fn solver(input: &str) -> (String, String) {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let mut x = 1i32;
    let mut cycles = 1;
    let mut next_interesting = 20;
    let mut signal_strengths = 0;
    let mut image = [[false; WIDTH]; HEIGHT];
    for command in input.lines().map(Command::from) {
        for cycle in cycles..cycles + command.cycles() {
            if cycle == next_interesting {
                signal_strengths += next_interesting * x as usize;
                next_interesting += 40;
            }

            let col = (cycle - 1) % WIDTH;
            if x - 1 <= col as i32 && col as i32 <= x + 1 {
                image[(cycle - 1) / WIDTH][col] = true;
            }
        }

        match command {
            Noop => {}
            AddX(v) => { x += v }
        }
        cycles += command.cycles();
    }

    let part2 = image
        .map(|r| r.map(|c| if c { "#" } else { "." }).join(""))
        .join("\n");

    (signal_strengths.to_string(), format!("\n{}", part2))
}

#[cfg(test)]
mod tests {
    use crate::days::day10::*;

    const EXAMPLE_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8
addx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1
addx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop
noop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop
addx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop
addx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop
noop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9
addx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19
addx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9
addx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1
addx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10
noop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    const EXAMPLE_OUTPUT_2: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test1() {
        let (part1, part2) = solver(EXAMPLE_INPUT);
        assert_eq!("13140", part1);
        assert_eq!(EXAMPLE_OUTPUT_2, part2);
    }
}
use crate::day::DaySolver;

pub const DAY_7: DaySolver = DaySolver::Double(solver);

// Based on the observation that the input does an in-order tree-walk.
fn solver(input: &str) -> (String, String) {
    let mut directories = Vec::<usize>::new();
    let mut current_size = vec![];
    for line in input.lines().map(str::trim) {
        if line == "$ cd .." {
            directories.push(current_size.pop().unwrap());
        } else if line == "$ ls" {
            current_size.push(0);
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let (size, _) = line.split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            current_size.iter_mut().for_each(|e| *e += size);
        }
    }

    let total_used_space = current_size.first().unwrap().clone();
    directories.append(&mut current_size);

    let part1 = directories.iter().cloned()
        .filter(|&e| e <= 100000)
        .sum::<usize>().to_string();

    let unused_space = 70000000 - total_used_space;
    let to_be_freed = 30000000 - unused_space;
    let part2 = directories.into_iter()
        .filter(|&e| e >= to_be_freed)
        .min().unwrap().to_string();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use crate::days::day7::*;

    const EXAMPLE_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test() {
        assert_eq!(("95437".into(), "24933642".into()), solver(EXAMPLE_INPUT));
    }
}
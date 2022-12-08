use crate::day::DaySolver;

pub const DAY_8: DaySolver = DaySolver::Double(solver);

type Map = Vec<Vec<u8>>;

fn solver(input: &str) -> (String, String) {
    let map = input.lines()
        .map(|l| l.as_bytes().iter().map(|b| b - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    (part1(&map, height, width), part2(&map, height, width))
}

fn compute_horizontal_visibility<R: Iterator<Item=usize>>(map: &Map, seen: &mut Vec<Vec<bool>>, r: usize, mut cs: R) {
    let first_c = cs.next().unwrap();
    let mut tallest = map[r][first_c];
    seen[r][first_c] = true;
    for c in cs {
        if map[r][c] > tallest {
            tallest = map[r][c];
            seen[r][c] = true;
        }
    }
}

fn compute_vertical_visibility<R: Iterator<Item=usize>>(map: &Map, seen: &mut Vec<Vec<bool>>, mut rs: R, c: usize) {
    let first_r = rs.next().unwrap();
    let mut tallest = map[first_r][c];
    seen[first_r][c] = true;
    for r in rs {
        if map[r][c] > tallest {
            tallest = map[r][c];
            seen[r][c] = true;
        }
    }
}

fn part1(map: &Map, height: usize, width: usize) -> String {
    let mut seen = vec![vec![false; width]; height];

    for r in 0..height {
        compute_horizontal_visibility(&map, &mut seen, r, 0..width);
        compute_horizontal_visibility(&map, &mut seen, r, (0..width).rev());
    }

    for c in 0..width {
        compute_vertical_visibility(&map, &mut seen, 0..height, c);
        compute_vertical_visibility(&map, &mut seen, (0..height).rev(), c);
    }

    seen.into_iter()
        .map(|l| l.into_iter().filter(|&b| b == true).count())
        .sum::<usize>()
        .to_string()
}

fn horizontal_count<R: Iterator<Item=usize>>(map: &Map, value: u8, r: usize, cs: R) -> usize {
    let mut count = 0;
    for dc in cs {
        count += 1;
        if map[r][dc] >= value { break; }
    }
    count
}

fn vertical_count<R: Iterator<Item=usize>>(map: &Map, value: u8, rs: R, c: usize) -> usize {
    let mut count = 0;
    for dr in rs {
        count += 1;
        if map[dr][c] >= value { break; }
    }
    count
}

fn part2(map: &Map, height: usize, width: usize) -> String {
    (0..height).map(|r| (0..width).map(|c| {
        let down_count = vertical_count(&map, map[r][c], r + 1..height, c);
        let up_count = vertical_count(&map, map[r][c], (0..r).rev(), c);
        let right_count = horizontal_count(&map, map[r][c], r, c + 1..width);
        let left_count = horizontal_count(&map, map[r][c], r, (0..c).rev());
        down_count * up_count * right_count * left_count
    }).max().unwrap()).max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::days::day8::*;

    const EXAMPLE_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_() {
        assert_eq!(("21".to_owned(), "8".to_owned()), solver(EXAMPLE_INPUT));
    }
}
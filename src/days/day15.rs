use crate::day::DaySolver;

pub const DAY_15: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

struct Coordinate {
    x: i32,
    y: i32,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(", ").unwrap();
        Self { x: x[2..].parse().unwrap(), y: y[2..].parse().unwrap() }
    }
}

struct Sensor {
    position: Coordinate,
    closest_beacon: Coordinate,
    distance: i32,
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let (sensor, beacon) = s[10..].split_once(": closest beacon is at ").unwrap();
        let position: Coordinate = sensor.into();
        let closest_beacon: Coordinate = beacon.into();
        let distance_to_beacon = (position.x - closest_beacon.x).abs() + (position.y - closest_beacon.y).abs();
        Self { position, closest_beacon, distance: distance_to_beacon }
    }
}

fn split_range(range: (i32, i32), at: i32) -> ((i32, i32), Option<(i32, i32)>) {
    if range.0 == range.1 {
        panic!("cannot split 1-width range");
    }

    if range.0 == at {
        ((at + 1, range.1), None)
    } else if range.1 == at {
        ((range.0, at - 1), None)
    } else {
        ((range.0, at - 1), Some((at + 1, range.1)))
    }
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_by(|l, r| l.0.cmp(&r.0).then(l.1.cmp(&r.1)));
    let mut non_overlapped_ranges = vec![];
    let mut current = ranges[0];
    for range in ranges.into_iter().skip(1) {
        if range.0 <= current.1 + 1 {
            if range.1 > current.1 {
                current.1 = range.1;
            }
        } else {
            non_overlapped_ranges.push(current);
            current = range;
        }
    }
    non_overlapped_ranges.push(current);
    non_overlapped_ranges
}

fn part1(input: &str) -> String {
    part1_internal(input, 2000000)
}

fn part1_internal(input: &str, y: i32) -> String {
    let sensors = input.lines().map(Sensor::from).collect::<Vec<_>>();
    let mut ranges = vec![];
    for sensor in sensors {
        let perp_distance = (sensor.position.y - y).abs();
        if perp_distance > sensor.distance {
            continue;
        }

        let remaining_distance = sensor.distance - perp_distance;
        let mut start = sensor.position.x - remaining_distance;
        let mut end = sensor.position.x + remaining_distance;
        if sensor.closest_beacon.y == y {
            if start == end {
                continue;
            }
            if sensor.closest_beacon.x == start {
                start += 1;
            } else if sensor.closest_beacon.x == end {
                end -= 1;
            }
        }

        if sensor.position.y == y {
            let (first, second) = split_range((start, end), sensor.position.x);
            ranges.push(first);
            if let Some(second) = second {
                ranges.push(second);
            }
        } else {
            ranges.push((start, end));
        }
    }

    merge_ranges(ranges).into_iter()
        .map(|r| r.1 - r.0 + 1)
        .sum::<i32>().to_string()
}

fn part2(input: &str) -> String {
    part2_internal(input, 4000000)
}

fn has_gap_in_window(sorted_non_overlapping: &[(i32, i32)], window: i32) -> Option<i32> {
    let start_index = sorted_non_overlapping.iter().enumerate()
        .filter(|(_, r)| r.1 > 0)
        .next().unwrap().0;
    let end_index = sorted_non_overlapping.iter().enumerate().rev()
        .filter(|(_, r)| r.0 <= window)
        .next().unwrap().0;

    if start_index == end_index {
        None
    } else {
        Some(sorted_non_overlapping[start_index].1 + 1)
    }
}

fn part2_internal(input: &str, window: i32) -> String {
    let sensors = input.lines().map(Sensor::from).collect::<Vec<_>>();
    for y in 0..=window {
        let mut ranges = vec![];
        for sensor in &sensors {
            let perp_distance = (sensor.position.y - y).abs();
            if perp_distance > sensor.distance {
                continue;
            }

            let remaining_distance = sensor.distance - perp_distance;
            let start = sensor.position.x - remaining_distance;
            let end = sensor.position.x + remaining_distance;
            ranges.push((start, end));
        }

        let non_overlapping_occupied_ranges = merge_ranges(ranges);
        if let Some(x) = has_gap_in_window(&non_overlapping_occupied_ranges, window) {
            return (x as usize * 4000000 + y as usize).to_string();
        }
    }
    panic!("no solution");
}

#[cfg(test)]
mod tests {
    use crate::days::day15::*;

    const EXAMPLE_INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!("26", part1_internal(EXAMPLE_INPUT, 10));
    }

    #[test]
    fn test_part1_1() {
        assert_eq!("25", part1_internal(EXAMPLE_INPUT, 9));
    }

    #[test]
    fn test_part1_2() {
        assert_eq!("27", part1_internal(EXAMPLE_INPUT, 11));
    }

    #[test]
    fn test_part2() {
        assert_eq!("56000011", part2_internal(EXAMPLE_INPUT, 20));
    }
}
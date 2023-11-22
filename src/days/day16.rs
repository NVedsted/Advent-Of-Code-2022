use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::day::DaySolver;

pub const DAY_16: DaySolver = DaySolver::Standard { part1: Some(part1), part2: Some(part2) };

#[derive(Debug)]
struct Valve {
    tunnels: Vec<usize>,
    flow_rate: i32,
}

struct Map {
    start: usize,
    distances: Vec<Vec<i32>>,
    valves: Vec<Valve>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name_to_index = HashMap::new();

        let raw_valves = s.lines()
            .enumerate()
            .map(|(i, l)| {
                let name = l[6..8].to_owned();
                name_to_index.insert(name.to_owned(), i);
                let semicolon_index = l.find(';').unwrap();
                let flow_rate = l[23..semicolon_index].parse::<i32>().unwrap();

                let valves_index = if l.contains("tunnels") {
                    semicolon_index + 25
                } else {
                    semicolon_index + 24
                };

                let tunnels = l[valves_index..].split(", ").collect::<Vec<_>>();
                (flow_rate, tunnels)
            })
            .collect::<Vec<_>>();

        let mut valves = raw_valves.iter()
            .map(|(flow_rate, _)| Valve { tunnels: vec![], flow_rate: *flow_rate })
            .collect::<Vec<_>>();

        for (i, (_, raw_tunnels)) in raw_valves.into_iter().enumerate() {
            let mut tunnel_indexes = raw_tunnels.into_iter().map(|t| name_to_index[t]).collect();
            valves[i].tunnels.append(&mut tunnel_indexes);
        }

        let mut distances = vec![vec![i32::MAX; valves.len()]; valves.len()];

        for (i, valve) in valves.iter().enumerate() {
            valve.tunnels.iter().for_each(|&j| distances[i][j] = 1);
        }

        for (i, _) in valves.iter().enumerate() {
            distances[i][i] = 0;
        }

        for k in 0..valves.len() {
            for i in 0..valves.len() {
                for j in 0..valves.len() {
                    let distance = distances[i][k].saturating_add(distances[k][j]);
                    if distances[i][j] > distance {
                        distances[i][j] = distance;
                    }
                }
            }
        }

        Ok(Map { valves, start: name_to_index["AA"], distances })
    }
}

#[derive(Clone, Copy, Debug)]
enum Person {
    Moving {
        position: usize,
        time_remaining: i32,
    },
    Awaiting {
        position: usize,
    },
    Idle,
}

impl Person {
    #[inline]
    fn time_remaining(&self) -> Option<i32> {
        match self {
            Person::Moving { time_remaining, .. } => Some(*time_remaining),
            _ => None,
        }
    }

    #[inline]
    fn target(&self) -> Option<usize> {
        match self {
            Person::Moving { position, .. } => Some(*position),
            _ => None,
        }
    }

    #[inline]
    fn is_awaiting(&self) -> bool {
        match self {
            Person::Awaiting { .. } => true,
            _ => false,
        }
    }

    #[inline]
    fn is_idle(&self) -> bool {
        match self {
            Person::Idle => true,
            _ => false,
        }
    }

    #[inline]
    fn awaiting_in_position(&self) -> Option<usize> {
        match self {
            Person::Awaiting { position } => Some(*position),
            _ => None,
        }
    }

    #[inline]
    fn has_arrived(&self) -> bool {
        match self {
            Person::Moving { time_remaining, .. } => *time_remaining == 0,
            _ => false,
        }
    }

    #[inline]
    fn pass_time(&mut self, time: i32) {
        if let Self::Moving { time_remaining, .. } = self {
            *time_remaining -= time;
            assert!(*time_remaining >= 0);
        }
    }

    #[inline]
    fn new(start_position: usize) -> Self {
        Self::Awaiting { position: start_position }
    }
}

fn permute_people<const N: usize>(map: &Map, mut people: [Person; N], person_index: Option<usize>, time_remaining: i32, shut_valves: &mut HashSet<usize>, release_per_minute: i32, current_released: i32) -> i32 {
    let Some(person_index) = person_index else {
        return max_pressure(map, people, time_remaining, shut_valves, release_per_minute, current_released);
    };

    let next_person = people.iter().enumerate().skip(person_index + 1).filter(|(_, p)| p.is_awaiting()).map(|(i, _)| i).next();
    let current_position = people[person_index].awaiting_in_position().unwrap();

    let considerable_valves = shut_valves.iter()
        .cloned()
        .filter(|&v| map.distances[current_position][v] + 2 < time_remaining)
        .collect::<Vec<_>>();

    if considerable_valves.is_empty() {
        people[person_index] = Person::Idle;
        return permute_people(map, people, next_person, time_remaining, shut_valves, release_per_minute, current_released);
    }

    let mut max_pressure_value = 0;

    for target_valve in considerable_valves {
        let mut new_people = people.clone();

        new_people[person_index] = Person::Moving {
            position: target_valve,
            time_remaining: map.distances[current_position][target_valve] + 1,
        };
        shut_valves.remove(&target_valve);

        max_pressure_value = max(
            max_pressure_value,
            permute_people(map, new_people, next_person, time_remaining, shut_valves, release_per_minute, current_released));
        shut_valves.insert(target_valve);
    }

    people[person_index] = Person::Idle;
    max_pressure_value = max(
        max_pressure_value,
        permute_people(map, people, next_person, time_remaining, shut_valves, release_per_minute, current_released));

    max_pressure_value
}

fn max_pressure<const N: usize>(map: &Map, mut people: [Person; N], mut time_remaining: i32, shut_valves: &mut HashSet<usize>, mut release_per_minute: i32, mut current_released: i32) -> i32 {
    let time_to_pass = people.iter().filter_map(|p| p.time_remaining()).min().unwrap_or(0);

    if time_remaining == 0 || time_remaining - time_to_pass <= 0 || people.iter().all(|p| p.is_idle()) {
        return current_released + time_remaining * release_per_minute;
    }

    time_remaining -= time_to_pass;
    current_released += time_to_pass * release_per_minute;
    people.iter_mut().for_each(|p| p.pass_time(time_to_pass));
    people.iter_mut().filter(|p| p.has_arrived()).for_each(|p| {
        let position = p.target().unwrap();
        release_per_minute += map.valves[position].flow_rate;
        *p = Person::Awaiting { position };
    });

    let first_person_index = people.iter().enumerate().filter(|(_, p)| p.is_awaiting()).next().unwrap().0;
    permute_people(map, people, Some(first_person_index), time_remaining, shut_valves, release_per_minute, current_released)
}

fn find_max_pressure<const N: usize>(input: &str, time: i32) -> i32 {
    let map: Map = input.parse().unwrap();
    let mut shut_valves = map.valves.iter().enumerate().filter(|(_, v)| v.flow_rate > 0).map(|(i, _)| i).collect();
    max_pressure(&map, [Person::new(map.start); N], time, &mut shut_valves, 0, 0)
}

fn part1(input: &str) -> String {
    find_max_pressure::<1>(input, 30).to_string()
}

fn part2(input: &str) -> String {
    find_max_pressure::<2>(input, 26).to_string()
}


#[cfg(test)]
mod tests {
    use crate::days::day16::*;

    #[test]
    fn test1() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!("1651", part1(input));
    }

    #[test]
    fn test2() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!("1707", part2(input));
    }
}
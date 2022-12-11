mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use crate::day::DaySolver;


pub const DAYS: [Option<DaySolver>; 25] = [
    Some(day1::DAY_1),
    Some(day2::DAY_2),
    Some(day3::DAY_3),
    Some(day4::DAY_4),
    Some(day5::DAY_5),
    Some(day6::DAY_6),
    Some(day7::DAY_7),
    Some(day8::DAY_8),
    Some(day9::DAY_9),
    Some(day10::DAY_10),
    Some(day11::DAY_11),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
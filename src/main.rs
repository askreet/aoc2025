mod shared;

use shared::*;
use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(Error::new("usage: aoc2025 <day#> <part#>"));
    }

    let day = args[1].parse::<u32>()?;
    let part = args[2].parse::<u32>()?;

    let mut days: HashMap<u32, Box<dyn Solution>> = HashMap::new();
    days.insert(01, Box::new(day01::Day01));
    days.insert(02, Box::new(day02::Day02));
    days.insert(03, Box::new(day03::Day03));
    days.insert(04, Box::new(day04::Day04));
    days.insert(05, Box::new(day05::Day05));
    days.insert(06, Box::new(day06::Day06));
    days.insert(07, Box::new(day07::Day07));
    days.insert(08, Box::new(day08::Day08));
    days.insert(09, Box::new(day09::Day09));
    days.insert(10, Box::new(day10::Day10));
    days.insert(11, Box::new(day11::Day11));
    days.insert(12, Box::new(day12::Day12));

    if let Some(solution) = days.get(&day) {
        let result = if part == 1 {
            solution.part1()
        } else if part == 2 {
            solution.part2()
        } else {
            return Err(Error::new("invalid part number"));
        };

        match result {
            Ok(v) => println!("result: {}", v),
            Err(e) => println!("error: {}", e),
        }
    } else {
        return Err(Error::new(&format!("day {day} not found")));
    }

    Ok(())
}

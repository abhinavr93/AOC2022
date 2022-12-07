use aoc2022_lib::import_solution_modules;
use std::io::Error;
use std::{env, fs};

mod common;
import_solution_modules!();

fn main() -> std::io::Result<()> {
    let (use_example_input, day_num) = parse_arguments()?;
    let (ans1, ans2) = solution_runner(day_num, use_example_input);
    println!("Day {day_num} Part1 answer: {ans1}");
    println!("Day {day_num} Part2 answer: {ans2}");
    Ok(())
}

fn parse_arguments() -> Result<(bool, u32), Error> {
    let mut use_example_input = false;
    let mut day_num = 0;
    let args = env::args().skip(1);
    for arg in args {
        if arg == "-e" {
            use_example_input = true;
        } else {
            day_num = common::parse_day_num(&arg)?;
        }
    }
    Ok((use_example_input, day_num))
}

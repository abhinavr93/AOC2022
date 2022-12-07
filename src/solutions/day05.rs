use std::collections::VecDeque;

use regex::Regex;

pub fn part1(input: &str) -> String {
    let strs = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = parse_stacks(&strs[0]);

    for line in strs[1].lines() {
        let command_nums = parse_command_nums(&line);

        for _ in 0..command_nums[0] {
            let elem = stacks[command_nums[1] - 1]
                .pop_back()
                .expect("Invalid stack element");
            stacks[command_nums[2] - 1].push_back(elem);
        }
    }
    stacks
        .iter()
        .map(|s| s.back().expect("Invalid stack elem"))
        .collect::<String>()
}

pub fn part2(input: &str) -> String {
    let strs = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = parse_stacks(&strs[0]);

    for line in strs[1].lines() {
        let command_nums = parse_command_nums(&line);
        let mut elems : Vec<char> = Vec::new();
        for _ in 0..command_nums[0] {
            elems.push(stacks[command_nums[1] - 1]
                .pop_back()
                .expect("Invalid stack element"));   
        }
        for _ in 0..command_nums[0] {
            stacks[command_nums[2] - 1].push_back(elems.pop().expect("Invalid elems element"));
        }
    }
    stacks
        .iter()
        .map(|s| s.back().expect("Invalid stack elem"))
        .collect::<String>()
}

fn parse_stacks(input_str: &str) -> Vec<VecDeque<char>> {
    let n = (input_str.lines().next().expect("first line invalid").len() + 2)/4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); n];
    let re = Regex::new(r"\[(\w)\]").expect("Invalid regex");
    for line in input_str.lines() {
        let matches = re.find_iter(line).collect::<Vec<_>>();
        for mat in matches {
            let i = (mat.start() + mat.end()) / 2;
            stacks[(i - 1) / 4].push_front(line.chars().nth(i).expect("char not found"));
        }
    }
    stacks
}

fn parse_command_nums(input_line: &str) -> Vec<usize> {
    let captures = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)$")
        .unwrap()
        .captures(input_line)
        .expect("Unable to capture numbers.");
    let command_nums = captures
        .iter()
        .skip(1)
        .map(|x| {
            x.expect("some number")
                .as_str()
                .parse::<usize>()
                .expect("Invalid num")
        })
        .collect::<Vec<_>>();
    command_nums
}

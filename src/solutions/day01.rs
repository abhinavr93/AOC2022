use itertools::Itertools;

pub fn part1(input: &str) -> String {    
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let sums = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .collect::<Vec<usize>>();

    sums[0..3].iter().sum::<usize>().to_string()
}
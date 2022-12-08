use itertools::{max, Itertools};

pub fn part1(input: &str) -> String {
    let grid = input.lines().map(|x| x.chars().map(|c| c.to_digit(10).expect("Not valid digit")).collect::<Vec<_>>()).collect::<Vec<_>>();
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut visible_count = 2*(nrows + ncols) - 4;
    for r in 1..nrows-1 {
        for c in 1..ncols-1 {
            visible_count += if check_visibility(r,c, &grid) {1} else {0};
        }
    }
    visible_count.to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input.lines().map(|x| x.chars().map(|c| c.to_digit(10).expect("Not valid digit")).collect::<Vec<_>>()).collect::<Vec<_>>();
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut max_ss = 0;
    for r in 1..nrows-1 {
        for c in 1..ncols-1 {
            let ss = scenic_score(r, c, &grid);
            if ss > max_ss {
                max_ss = ss;}
        }
    }
    max_ss.to_string()
}

fn check_visibility(r: usize, c: usize, grid: &Vec<Vec<u32>>) -> bool {  
    if max(grid[r][0..c].iter()).unwrap() < &grid[r][c] {return true;}
    if max(grid[r][c+1..grid[r].len()].iter()).unwrap() < &grid[r][c] {return true;}
    if max(grid[0..r].iter().map(|v| v[c])).unwrap() < grid[r][c] {return true;}
    if max(grid[r+1..grid.len()].iter().map(|v| v[c])).unwrap() < grid[r][c] {return true;}   
    false
}

fn scenic_score(r: usize, c: usize, grid: &Vec<Vec<u32>>) -> usize {
    let vd1 = if let Some(x) = grid[r][0..c].iter().map(|&v| v).rev().find_position(|&v| v >= grid[r][c]) {x.0 + 1} else {c};
    let vd2 = if let Some(x) = grid[r][c+1..grid[r].len()].iter().map(|&v| v).find_position(|&v| v >= grid[r][c]) {x.0 + 1} else {grid[r].len()-c-1};
    let vd3 = if let Some(x) = grid[0..r].iter().map(|v| v[c]).rev().find_position(|&v| v >= grid[r][c]) {x.0 + 1} else {r};
    let vd4 = if let Some(x) = grid[r+1..grid.len()].iter().map(|v| v[c]).find_position(|&v| v >= grid[r][c]) {x.0 + 1} else {grid.len()-r-1};

    vd1*vd2*vd3*vd4
}


pub fn part1(input: &str) -> String {    
    const SCORE_MATRIX: [[usize; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
    find_total_score(input, &SCORE_MATRIX).to_string()
}

pub fn part2(input: &str) -> String {
    const SCORE_MATRIX: [[usize; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
    find_total_score(input, &SCORE_MATRIX).to_string()
}

fn find_total_score(input: &str, &score_matrix: &[[usize; 3]; 3]) -> usize {
    let mut total_score = 0;
    for line in input.lines() {
        let pair = line.trim().split(' ').collect::<Vec<&str>>();
        let i = pair[0].parse::<char>().unwrap() as usize - 65;
        let j = pair[1].parse::<char>().unwrap() as usize - 88;

        total_score += score_matrix[i][j];
    }
    total_score   
}
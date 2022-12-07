
pub fn part1(input: &str) -> String {    
    let mut total_pairs = 0;

    for line in input.lines() {
        let nums = line.trim().split(|c| c == ',' || c=='-').map(|x| x.parse::<usize>().expect("needs to be integer")).collect::<Vec<usize>>();
        total_pairs += check_if_pairs_overlap(&nums) as usize;
    }
    total_pairs.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total_pairs = 0;

    for line in input.lines() {
        let nums = line.trim().split(|c| c == ',' || c=='-').map(|x| x.parse::<usize>().expect("needs to be integer")).collect::<Vec<usize>>();
        total_pairs += check_if_any_overlap(&nums) as usize;
    }
    total_pairs.to_string()
}

fn check_if_pairs_overlap(nums: &Vec<usize>) -> bool {
    (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[2] <= nums[0] && nums[3] >= nums[1])
}

fn check_if_any_overlap(nums: &Vec<usize>) -> bool {
    !(nums[0] > nums[3] || nums[1] < nums[2]) 
}
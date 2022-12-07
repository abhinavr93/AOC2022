use std::collections::HashSet; 

pub fn part1(input: &str) -> String {    
   solve(input, 4).to_string()
}

pub fn part2(input: &str) -> String {
    solve(input, 14).to_string()
}

fn solve(input: &str, window_size: usize) -> usize {
    let s = input.chars().collect::<Vec<char>>();
   let windows = s[0..s.len()].windows(window_size);

   let mut loc = window_size;
   for group in windows {
        let set: HashSet<&char> = HashSet::from_iter(group.iter());
        if set.len() == window_size {
            return loc;
        }
        loc += 1;
   }  
   0
}
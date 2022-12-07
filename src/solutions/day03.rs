use std::collections::HashSet;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {    
    let mut priority_sum = 0;
    for line in input.lines() {
        let l = line.trim();
        let n = l.len();
        let mut char_iter = l.chars();
        let mut char_hash_set = HashSet::new();
        for i in 0..n {
            let c = char_iter.next().expect("Expected valid char");
            if i < n / 2 {
                char_hash_set.insert(c);
            } else if char_hash_set.contains(&c) {
                priority_sum += priority(c);
                break;
            }
        }
    }
    priority_sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut priority_sum = 0;
    let mut line_num = 0;
    let mut group_letter_count_map = HashMap::new();
    for line in input.lines() {
        let mut char_hash_set = HashSet::new();
        for c in line.trim().chars() {
            if !char_hash_set.contains(&c) {
                char_hash_set.insert(c);
                if group_letter_count_map.contains_key(&c) {
                    let count = group_letter_count_map[&c];
                    group_letter_count_map.insert(c, count + 1);
                    if group_letter_count_map[&c] == 3 {
                        priority_sum += priority(c);
                        break;
                    }
                } else {
                    group_letter_count_map.insert(c, 1);
                }
            }
        }

        if line_num % 3 == 2 {
            group_letter_count_map.clear();
        }

        line_num += 1;
    }
    priority_sum.to_string()
}

fn priority(c: char) -> usize {
    if c.is_ascii_uppercase() {
        c as usize - 38
    } else {
        c as usize - 96
    }
}
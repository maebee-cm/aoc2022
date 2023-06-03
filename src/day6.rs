const INPUT: &str = include_str!("../inputs/day6.txt");

pub fn solve() {
    let input = INPUT.chars().collect();
    part_one(&input);
    part_two(&input);
}

fn find_n_diff_chars(input: &Vec<char>, window_size: usize) -> Option<usize> {
    // we're not 0 indexed
    let mut last_idx = window_size;
    for window in input.windows(window_size) {
        let mut found_same_char = false;
        for i in 0..(window.len()-1) {
            let current_char = window[i];
            if window[(i+1)..].iter().any(|&e| e == current_char) {
                found_same_char = true;
                break;
            }
        }
        if !found_same_char {
            // +1 because not 0 indexed
            return Some(last_idx);
        }
        last_idx += 1;
    }

    None
}

fn part_one(input: &Vec<char>) {
    let result = find_n_diff_chars(input, 4).unwrap();
    println!("First start of packet marker: {}", result)
}

fn part_two(input: &Vec<char>) {
    let result = find_n_diff_chars(input, 14).unwrap();
    println!("First start of message marker: {}", result)
}
const INPUT: &str = include_str!("../inputs/day3.txt");

pub fn solve() {
    part_one();
    part_two();
}

fn char_to_priority(letter: char) -> u32 {
    if letter.is_uppercase() {
        return letter as u32 - 'A' as u32 + 27
    }
    else {
        return letter as u32 - 'a' as u32 + 1
    }
}

// takes a line from puzzle input, parses it as two separate compartments
// then returns the code for the item the two compartments share
fn line_to_priority(line: &str) -> u32 {
    // we know that he input has to be ascii and is exclusively a-z, and A-Z
    let (compartment_one, compartment_two) = line.split_at(line.len()/2);
    for letter in compartment_one.chars() {
        if compartment_two.contains(letter) {
            return char_to_priority(letter);
        }
    }

    0
}

fn part_one() {
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += line_to_priority(line);
    }

    println!("Sum of all shared items priority: {}", sum);
}

fn part_two() {
    let mut sum = 0;
    let lines: Vec<_> = INPUT.lines().collect();

    for idx in (0..lines.len()).step_by(3) {
        let first_line = lines[idx];
        let second_line = lines[idx+1];
        let third_line = lines[idx+2];

        for letter in first_line.chars() {
            if second_line.contains(letter) && third_line.contains(letter) {
                sum += char_to_priority(letter);
                break;
            }
        }
    }

    println!("Sum of all group badge priorities: {}", sum);
}
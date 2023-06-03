const INPUT: &str = include_str!("../inputs/day5.txt");

pub fn solve() {
    part_one();
    part_two();
}

// takes input and parses only the start of it returning a 2d matrix of crates
fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let crate_line_len = input.lines().next().unwrap().len();
    let crate_count = (crate_line_len-(crate_line_len/4))/3;
    let mut crates = Vec::with_capacity(crate_count);
    for _ in 0..crate_count {
        crates.push(Vec::new());
    }

    for line in input.lines() {
        // we reached the end of the crates
        if !line.contains('[') {
            break;
        }

        for i in 0..crate_count {
            let name = line.chars().nth(i*4+1).unwrap();
            if name != ' ' {
                crates[i].push(name);
            }
        }
    }

    for column in crates.iter_mut() {
        column.reverse();
    }

    crates
}

fn part_one() {
    let mut crates = parse_crates(INPUT);

    // skip the first lines so that we don't see the crate input
    let lines_to_skip = crates.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len()+2;
    for line in INPUT.lines().skip(lines_to_skip) {
        let line: Vec<_> = line.split(' ').collect();
        let (move_num_crates, from_column, to_column) = (line[1], line[3], line[5]);
        let move_num_crates: usize = move_num_crates.parse().unwrap();
        let from_column = from_column.parse::<usize>().unwrap()-1;
        let to_column = to_column.parse::<usize>().unwrap()-1;

        let start_idx = crates[from_column].len()-move_num_crates;
        let end_idx = crates[from_column].len();
        for i in (start_idx..end_idx).rev() {
            let value = crates[from_column][i];
            crates[to_column].push(value);
        }
        let new_from_column_size = crates[from_column].len()-move_num_crates;
        crates[from_column].truncate(new_from_column_size);
    }

    for column in crates {
        print!("{}", column.last().unwrap());
    }
    println!();
}

fn part_two() {
    let mut crates = parse_crates(INPUT);

    // skip the first lines so that we don't see the crate input
    let lines_to_skip = crates.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len()+2;
    for line in INPUT.lines().skip(lines_to_skip) {
        let line: Vec<_> = line.split(' ').collect();
        let (move_num_crates, from_column, to_column) = (line[1], line[3], line[5]);
        let move_num_crates: usize = move_num_crates.parse().unwrap();
        let from_column = from_column.parse::<usize>().unwrap()-1;
        let to_column = to_column.parse::<usize>().unwrap()-1;

        let start_idx = crates[from_column].len()-move_num_crates;
        let end_idx = crates[from_column].len();
        for i in start_idx..end_idx {
            let value = crates[from_column][i];
            crates[to_column].push(value);
        }
        let new_from_column_size = crates[from_column].len()-move_num_crates;
        crates[from_column].truncate(new_from_column_size);
    }

    for column in crates {
        print!("{}", column.last().unwrap());
    }
    println!();
}
const INPUT: &str = include_str!("../inputs/day4.txt");

pub fn solve() {
    part_one();
    part_two();
}

fn parse_line(line: &str) -> (std::ops::RangeInclusive<usize>, std::ops::RangeInclusive<usize>) {
    let get_range_from_text = |input: &str| -> std::ops::RangeInclusive::<usize> {
        let separator_idx = input.find('-').unwrap();
        let (first_num, second_num) = input.split_at(separator_idx);
        // to get rid of the dash
        let second_num = &second_num[1..];
        let first_num = first_num.parse().unwrap();
        let second_num: usize = second_num.parse().unwrap();

        first_num..=(second_num)
    };

    let separator_idx = line.find(',').unwrap();
    let (first_range, second_range) = line.split_at(separator_idx);
    // to get rid of the comma
    let second_range = &second_range[1..];

    let first_range = get_range_from_text(first_range);
    let second_range = get_range_from_text(second_range);

    (first_range, second_range)
}

fn part_one() {
    // check if the first range fully contains the second range
    let range_contains = |first: &std::ops::RangeInclusive<usize>, second: &std::ops::RangeInclusive<usize>| -> bool {
        first.start() <= second.start() && second.end() <= first.end()
    };

    let mut sum = 0;

    for line in INPUT.lines() {
        let (first_range, second_range) = parse_line(line);

        if range_contains(&first_range, &second_range) || range_contains(&second_range, &first_range) {
            sum += 1;
        }
    }

    println!("completely overlapping range count: {}", sum);
}

fn part_two() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let (first_range, second_range) = parse_line(line);

        // check if the ranges overlap
        if first_range.contains(&second_range.start()) || first_range.contains(&second_range.end()) ||
        second_range.contains(&first_range.start()) || second_range.contains(&first_range.end()) {
            sum += 1;
        }
    }

    println!("partially overlapping range count: {}", sum);
}
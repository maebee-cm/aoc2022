const INPUT: &str = include_str!("../inputs/day1.txt");

pub fn solve() {
    let mut parsed_input = parse_input();
    part_one(&parsed_input);
    part_two(&mut parsed_input);
}

fn parse_input() -> Vec<usize> {
    let mut ret = vec![0];

    for line in INPUT.lines() {
        if line.is_empty() {
            ret.push(0);
        }
        else {
            let index = ret.len() - 1;
            ret[index] += line.parse::<usize>().unwrap();
        }
    }

    ret
}

fn part_one(parsed_input: &Vec<usize>) {
    let max = parsed_input.iter().max().unwrap();

    println!("the elf with the most calories has: {} calories", max);
}

fn part_two(parsed_input: &Vec<usize>) {
    let mut largest = 0;
    let mut second_largest = 0;
    let mut third_largest = 0;

    for &val in parsed_input {
        if val > largest {
            third_largest = second_largest;
            second_largest = largest;
            largest = val
        }
        else if val > second_largest {
            third_largest = second_largest;
            second_largest = val;
        }
        else if val > third_largest {
            third_largest = val;
        }
    }

    println!("Total calories carried by the top 3 elves: {}", largest+second_largest+third_largest);
}
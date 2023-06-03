use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day9.txt");

pub fn solve() {
    let input = parse_input(INPUT);
    part_one(&input);
    part_two(&input);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32);

struct Rope {
    knots: Vec<Point>
}

impl Rope {
    pub fn new(knots: usize) -> Rope {
        Rope {
            knots: vec![Point(0, 0); knots]
        }
    }

    pub fn move_head(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.knots[0].1 += 1,
            Movement::Down => self.knots[0].1 -= 1,
            Movement::Left => self.knots[0].0 -= 1,
            Movement::Right => self.knots[0].0 +=1,
        }

        for i in 0..(self.knots.len()-1) {
            let distance = self.distance_between_knots(i, i+1);
            let abs_distance = (i32::abs(distance.0), i32::abs(distance.1));

            if abs_distance.0 > 1 || abs_distance.1 > 1 {
                if abs_distance.0 != 0 {
                    self.knots[i+1].0 += distance.0/abs_distance.0;
                }
                if abs_distance.1 != 0 {
                    self.knots[i+1].1 += distance.1/abs_distance.1;
                }
            }
        }
    }

    fn distance_between_knots(&self, first: usize, second: usize) -> (i32, i32) {
        (self.knots[first].0-self.knots[second].0, self.knots[first].1-self.knots[second].1)
    }
}

#[derive(Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn parse_input(input: &str) -> Vec<Movement> {
    let mut parsed = Vec::with_capacity(input.lines().count());

    // format is "<Direction letter><space><number>"
    for line in input.lines() {
        let direction = match &line[0..1] {
            "U" => Movement::Up,
            "D" => Movement::Down,
            "L" => Movement::Left,
            "R" => Movement::Right,
            _ => unreachable!()
        };
        let number = *&line[2..].parse().unwrap();

        for _ in 0..number {
            parsed.push(direction);
        }
    }

    parsed
}

fn part_one(input: &Vec<Movement>) {
    let mut unique_points = HashSet::with_capacity(input.len());
    let mut rope = Rope::new(2);

    for &movement in input {
        rope.move_head(movement);
        unique_points.insert(rope.knots[1]);
    }

    println!("Unique positions visited by 2 knotted rope {}", unique_points.len())
}

fn part_two(input: &Vec<Movement>) {
    let mut unique_points = HashSet::with_capacity(input.len());
    let mut rope = Rope::new(10);

    for &movement in input {
        rope.move_head(movement);
        unique_points.insert(rope.knots[9]);
    }

    println!("Unique positions visited by 10 knotted rope {}", unique_points.len())
}
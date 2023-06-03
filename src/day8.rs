const INPUT: &str = include_str!("../inputs/day8.txt");

pub fn solve() {
    let grid = parse_input(INPUT);
    part_one(&grid);
    part_two(&grid);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let height = input.lines().count();
    let width = input.lines().next()
        .expect("Input has no lines").len();

    let mut grid = Vec::with_capacity(height);
    for _ in 0..height {
        grid.push(Vec::with_capacity(width))
    }

    for (y, line) in input.lines().enumerate() {
        for ch in line.chars() {
            grid[y].push(ch.to_digit(10).unwrap() as u8);
        }
    }

    grid
}

fn part_one(grid: &Vec<Vec<u8>>) {
    let width = grid[0].len();
    let height = grid.len();

    let check_is_tree_visible = |tree_x: usize, tree_y: usize| -> bool {
        let mut hidden_neg_y = false;
        for y in (tree_y+1)..height {
            if grid[y][tree_x] >= grid[tree_y][tree_x] {
                hidden_neg_y = true;
                break;
            }
        }

        let mut hidden_pos_y = false;
        for y in 0..tree_y {
            if grid[y][tree_x] >= grid[tree_y][tree_x] {
                hidden_pos_y = true;
                break;
            }
        }

        let mut hidden_neg_x = false;
        for x in 0..tree_x {
            if grid[tree_y][x] >= grid[tree_y][tree_x] {
                hidden_neg_x = true;
                break;
            }
        }

        let mut hidden_pos_x = false;
        for x in (tree_x+1)..width {
            if grid[tree_y][x] >= grid[tree_y][tree_x] {
                hidden_pos_x = true;
                break;
            }
        }

        !hidden_neg_y || !hidden_pos_y || !hidden_neg_x || !hidden_pos_x
    };

    let mut visible_tree_count = width*2+height*2-4;
    for y in 1..(height-1) {
        for x in 1..(width-1) {
            if check_is_tree_visible(x, y) {
                visible_tree_count += 1;
            }
        }
    }

    println!("Number of visible trees: {}", visible_tree_count);
}

fn part_two(grid: &Vec<Vec<u8>>) {
    let width = grid[0].len();
    let height = grid.len();

    let get_scenic_score = |tree_x: usize, tree_y: usize| -> usize {
        let mut neg_y_score = 0;
        for y in (tree_y+1)..height {
            neg_y_score += 1;
            if grid[y][tree_x] >= grid[tree_y][tree_x] {
                break;
            }
        }

        let mut pos_y_score = 0;
        for y in (0..tree_y).rev() {
            pos_y_score += 1;
            if grid[y][tree_x] >= grid[tree_y][tree_x] {
                break;
            }
        }

        let mut neg_x_score = 0;
        for x in (0..tree_x).rev() {
            neg_x_score += 1;
            if grid[tree_y][x] >= grid[tree_y][tree_x] {
                break;
            }
        }

        let mut pos_x_score = 0;
        for x in (tree_x+1)..width {
            pos_x_score += 1;
            if grid[tree_y][x] >= grid[tree_y][tree_x] {
                break;
            }
        }

        neg_y_score * pos_y_score * neg_x_score * pos_x_score
    };

    let mut highest_score = 0;
    for y in 0..height {
        for x in 0..width {
            let score = get_scenic_score(x, y);
            if highest_score < score {
                highest_score = score;
                get_scenic_score(x, y);
            }
        }
    }

    println!("Highest possible scenic score: {}", highest_score);
}
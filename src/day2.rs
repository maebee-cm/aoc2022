const INPUT: &str = include_str!("../inputs/day2.txt");

pub fn solve() {
    part_one();
    part_two();
}

enum RockPaperScissor {
    Rock,
    Paper,
    Scissor
}

impl RockPaperScissor {
    pub fn get_points(&self) -> u32 {
        const ROCK_POINTS: u32 = 1;
        const PAPER_POINTS: u32 = 2;
        const SCISSOR_POINTS: u32 = 3;

        match self {
            RockPaperScissor::Rock => ROCK_POINTS,
            RockPaperScissor::Paper => PAPER_POINTS,
            RockPaperScissor::Scissor => SCISSOR_POINTS,
        }
    }

    // returns what would cause self to win
    pub fn get_win(&self) -> RockPaperScissor {
        match self {
            RockPaperScissor::Rock => RockPaperScissor::Scissor,
            RockPaperScissor::Paper => RockPaperScissor::Rock,
            RockPaperScissor::Scissor => RockPaperScissor::Paper
        }
    }

    // returns what would cause self to lose
    pub fn get_lose(&self) -> RockPaperScissor {
        match self {
            RockPaperScissor::Rock => RockPaperScissor::Paper,
            RockPaperScissor::Paper => RockPaperScissor::Scissor,
            RockPaperScissor::Scissor => RockPaperScissor::Rock
        }
    }

    // returns a copy of self
    pub fn get_tie(&self) -> RockPaperScissor {
        match self {
            RockPaperScissor::Rock => RockPaperScissor::Rock,
            RockPaperScissor::Paper => RockPaperScissor::Paper,
            RockPaperScissor::Scissor => RockPaperScissor::Scissor
        }
    }
}

fn calculate_score(opponent_choice: RockPaperScissor, our_choice: RockPaperScissor) -> u32 {
    const TIE_POINTS: u32 = 3;
    const WIN_POINTS: u32 = 6;

    match our_choice {
        RockPaperScissor::Rock => {
            match opponent_choice {
                RockPaperScissor::Rock => our_choice.get_points() + TIE_POINTS,
                RockPaperScissor::Paper => our_choice.get_points(),
                RockPaperScissor::Scissor => our_choice.get_points() + WIN_POINTS,
            }
        }
        RockPaperScissor::Paper => {
            match opponent_choice {
                RockPaperScissor::Rock => our_choice.get_points() + WIN_POINTS,
                RockPaperScissor::Paper => our_choice.get_points() + TIE_POINTS,
                RockPaperScissor::Scissor => our_choice.get_points(),
            }
        },
        RockPaperScissor::Scissor => {
            match opponent_choice {
                RockPaperScissor::Rock => our_choice.get_points(),
                RockPaperScissor::Paper => our_choice.get_points() + WIN_POINTS,
                RockPaperScissor::Scissor => our_choice.get_points() + TIE_POINTS,
            }
        },
    }
}

fn part_one() {
    let char_to_rps = |value: char| -> Result<RockPaperScissor, &'static str> {
        Ok(match value {
            'A' | 'X' => RockPaperScissor::Rock,
            'B' | 'Y' => RockPaperScissor::Paper,
            'C' | 'Z' => RockPaperScissor::Scissor,
            _ => return Err("Invalid character passed to char_to_rps")
        })
    };

    let mut sum = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();
        // insert a _ to get rid of the space inbetween
        let (opponent_choice, _, our_choice) = (chars.next(), chars.next(), chars.next());
        let opponent_choice = char_to_rps(opponent_choice.unwrap()).unwrap();
        let our_choice = char_to_rps(our_choice.unwrap()).unwrap();

        sum += calculate_score(opponent_choice, our_choice)
    }

    println!("Sum of score: {}", sum);
}

fn part_two() {
    // Takes a char and returns the equivalent RPS value
    let char_to_rps_value = |value: char| -> Result<RockPaperScissor, &'static str> {
        Ok(match value {
            'A' => RockPaperScissor::Rock,
            'B' => RockPaperScissor::Paper,
            'C' => RockPaperScissor::Scissor,
            _ => return Err("Invalid character passed to char_to_rps_value")
        })
    };

    // Takes a char code and returns a win/lose/tie method
    let char_to_rps_func = |value: char| -> Result<fn(&RockPaperScissor) -> RockPaperScissor, &'static str> {
        Ok(match value{
            'X' => RockPaperScissor::get_win,
            'Y' => RockPaperScissor::get_tie,
            'Z' => RockPaperScissor::get_lose,
            _ => return Err("Invalid character passed to char_to_rps_func")
        })
    };

    let mut sum = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();
        // insert a _ to get rid of the space inbetween
        let (opponent_choice, _, our_choice) = (chars.next(), chars.next(), chars.next());
        let opponent_choice = char_to_rps_value(opponent_choice.unwrap()).unwrap();
        let our_choice = (char_to_rps_func(our_choice.unwrap()).unwrap())(&opponent_choice);

        sum += calculate_score(opponent_choice, our_choice);
    }

    println!("Sum of score: {}", sum);
}
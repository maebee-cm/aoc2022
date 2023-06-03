const INPUT: &str = include_str!("../inputs/day10.txt");

pub fn solve() {
    let parsed = parse_input(INPUT);
    part_one(&parsed);
    part_two(&parsed);
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut parsed = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        parsed.push(match &line[0..4] {
            "addx" => Instruction::Addx(*&line[5..].parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!()
        });
    }

    parsed
}

#[derive(Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop
}

struct Cpu {
    x: i32,
    cycle: i32
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0
        }
    }
}

fn part_one(input: &Vec<Instruction>) {
    let mut cpu = Cpu::new();
    let mut interesting_cycle = 20;
    let mut interesting_cycle_sum = 0;
    let mut interesting_cycle_increment = |cpu: &Cpu| {
        if cpu.cycle >= interesting_cycle && interesting_cycle <= 220 {
            interesting_cycle_sum += interesting_cycle * cpu.x;
            interesting_cycle += 40;
        }
    };

    for &instruction in input {
        match instruction {
            Instruction::Addx(value) => {
                cpu.cycle += 2;
                interesting_cycle_increment(&cpu);
                cpu.x += value;
            },
            Instruction::Noop => {
                cpu.cycle += 1;
                interesting_cycle_increment(&cpu);
            },
        };
    }

    println!("Interesting cycle sum: {}", interesting_cycle_sum);
}

fn part_two(input: &Vec<Instruction>) {
    const BLANK_LINE: [char; 40] = ['.'; 40];
    let mut screen: Vec<String> = Vec::with_capacity(6);
    for _ in 0..6 {
        screen.push(BLANK_LINE.iter().collect());
    }

    let mut cpu = Cpu::new();

    for &instruction in input {
        let (cycles, val_to_add) = match instruction {
            Instruction::Addx(value) => (2, value),
            Instruction::Noop => (1, 0),
        };

        for _ in 0..cycles {
            unsafe {
                let vertical_pos = (cpu.cycle/40) as usize;
                let horizontal_pos = cpu.cycle%40;

                if i32::abs(horizontal_pos-cpu.x) <= 1 {
                    screen[vertical_pos].as_bytes_mut()[horizontal_pos as usize] = b'#';
                }
            }
            cpu.cycle += 1;
        }

        cpu.x += val_to_add;
    }

    for line in screen {
        println!("{}", line);
    }
}
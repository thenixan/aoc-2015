use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Register {
    A,B
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

impl Register {

    fn from_input(s: &str) -> Register {
        if s.chars().nth(4).unwrap() == 'a' {
            Register::A
        } else {
            Register::B
        }
    }
}

impl FromStr for Instruction {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("hlf ") {
            Ok(Instruction::Half(Register::from_input(s)))
        } else if s.starts_with("tpl ") {
            Ok(Instruction::Triple(Register::from_input(s)))
        } else if s.starts_with("inc ") {
            Ok(Instruction::Increment(Register::from_input(s)))
        } else if s.starts_with("jmp ") {
            Ok(Instruction::Jump(s.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()))
        } else if s.starts_with("jie ") {
            Ok(Instruction::JumpIfEven(Register::from_input(s), s.split_whitespace().nth(2).unwrap().parse::<i32>().unwrap()))
        } else if s.starts_with("jio ") {
            Ok(Instruction::JumpIfOne(Register::from_input(s), s.split_whitespace().nth(2).unwrap().parse::<i32>().unwrap()))
        } else {
            Err(())
        }
    }

}

pub fn run() {
    let input = File::open("inputs/task_23").unwrap();
    let input = BufReader::new(input);

    let instructions = input.lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<Instruction>())
        .filter_map(|s| s.ok())
        .collect::<Vec<Instruction>>();

    let (_, b) = evaluate(&instructions, 0, 0, 0);
    println!("Result: {}", b);
}

pub fn run_e() {
    let input = File::open("inputs/task_23").unwrap();
    let input = BufReader::new(input);

    let instructions = input.lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<Instruction>())
        .filter_map(|s| s.ok())
        .collect::<Vec<Instruction>>();

    let (_, b) = evaluate(&instructions, 0, 1, 0);
    println!("Result: {}", b);
}

fn evaluate(instructions: &Vec<Instruction>, position: i32, a: u32, b: u32) -> (u32, u32) {
    if position < 0 || instructions.len() <= position as usize {
        return (a, b)
    }
    let i = instructions.get(position as usize).unwrap();
    match i {
        Instruction::Half(Register::A) => evaluate(instructions, position + 1, a / 2, b),
        Instruction::Half(Register::B) => evaluate(instructions, position + 1, a, b / 2),
        Instruction::Triple(Register::A) => evaluate(instructions, position + 1, a * 3, b),
        Instruction::Triple(Register::B) => evaluate(instructions, position + 1, a, b * 3),
        Instruction::Increment(Register::A) => evaluate(instructions, position + 1, a + 1, b),
        Instruction::Increment(Register::B) => evaluate(instructions, position + 1, a, b + 1),
        Instruction::Jump(j) => evaluate(instructions, position + j, a, b),
        Instruction::JumpIfEven(Register::A, j) => {
            if a % 2 == 0 {
                evaluate(instructions, position + j, a, b)
            } else {
                evaluate(instructions, position + 1, a, b)
            }
        },
        Instruction::JumpIfEven(Register::B, j) => {
            if b % 2 == 0 {
                evaluate(instructions, position + j, a, b)
            } else {
                evaluate(instructions, position + 1, a, b)
            }
        },
        Instruction::JumpIfOne(Register::A, j) => {
            if a == 1 {
                evaluate(instructions, position + j, a, b)
            } else {
                evaluate(instructions, position + 1, a, b)
            }
        },
        Instruction::JumpIfOne(Register::B, j) => {
            if b == 1 {
                evaluate(instructions, position + j, a, b)
            } else {
                evaluate(instructions, position + 1, a, b)
            }
        },
    }
}
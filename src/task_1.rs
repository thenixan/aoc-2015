use std::fs::File;
use std::io::{BufReader, Read};

pub fn run() {
    let input = File::open("inputs/task_1").unwrap();
    let input = BufReader::new(input);
    let mut result = 0;
    for b in input.bytes() {
        let b = b.unwrap();
        match b as char {
            ')' => result -= 1,
            '(' => result += 1,
            _ => {}
        }
    }
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("inputs/task_1").unwrap();
    let input = BufReader::new(input);
    let mut sum = 0;
    let mut result = 0;
    for b in input.bytes() {
        let b = b.unwrap();
        match b as char {
            ')' => sum -= 1,
            '(' => sum += 1,
            _ => {}
        }
        result += 1;
        if sum < 0 {
            break;
        }
    }
    println!("Result: {}", result);
}
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check1(input: &str) -> bool {
    let input = input.as_bytes();
    for i in 2..input.len() {
        if input[i - 2] < input[i - 1] && input[i - 1] < input[i] && input[i - 1] - input[i - 2] == 1 && input[i] - input[i - 1] == 1 {
            return true;
        }
    }
    return false;
}

fn check2(input: &str) -> bool {
    let input = input.as_bytes();
    for i in input {
        if *i == 'i' as u8 || *i == 'o' as u8 || *i == 'l' as u8 {
            return true;
        }
    }
    return true;
}

fn check3(input: &str) -> bool {
    let mut got_first = None;
    let input = input.as_bytes();

    let mut i = 1;
    while i < input.len() {
        if input[i - 1] == input[i] {
            if got_first.is_none() {
                got_first = Some(input[i]);
                i += 1;
            } else if got_first.unwrap() != input[i] {
                return true;
            }
        }
        i += 1;
    }
    return false;
}

fn next_password(input: &String) -> String {
    let input = input.as_bytes();
    let mut result: Vec<u8> = vec![];
    let mut left: u8 = 1;

    let mut i = input.len();
    while i > 0 {
        let c = input[i - 1] + left;
        if c > 'z' as u8 {
            result.push('a' as u8);
            left = c - 'z' as u8;
        } else {
            result.push(c);
            left = 0;
        }
        i -= 1;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn run() {
    let input = File::open("inputs/task_11").unwrap();
    let input = BufReader::new(input);
    let input = input.lines().filter_map(|l| l.ok()).next().unwrap();

    let mut result = input;
    loop {
        if check1(&result) && check2(&result) && check3(&result) {
            break;
        }
        result = next_password(&result);
    }
    println!("Result: {}", result)
}

pub fn run_e() {
    let input = File::open("inputs/task_11").unwrap();
    let input = BufReader::new(input);
    let input = input.lines().filter_map(|l| l.ok()).next().unwrap();

    let mut result = input;
    let mut first = false;
    loop {
        if check1(&result) && check2(&result) && check3(&result) {
            if !first {
                first = true;
            } else {
                break;
            }
        }
        result = next_password(&result);
    }
    println!("Result: {}", result)
}
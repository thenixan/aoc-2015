use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Counter {
    symbols: usize,
    values: usize,
}

impl Counter {
    fn new() -> Self { Counter { symbols: 0, values: 0 } }
}

pub fn run() {
    let input = File::open("inputs/task_8").unwrap();
    let input = BufReader::new(input);

    let result = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .fold(Counter::new(), |mut acc, l| {
            let mut i = 0;
            acc.symbols += l.len();
            let l = l.as_bytes();
            while i < l.len() {
                if l[i] == '"' as u8 && (i == 0 || i == l.len() - 1) {
                    // do nothing
                } else if l[i] == '\\' as u8 {
                    i += 1;
                    if l[i] == 'x' as u8 {
                        i += 2;
                    }
                    acc.values += 1;
                } else {
                    acc.values += 1;
                }
                i += 1;
            }
            acc
        });
    println!("Result: {}", result.symbols - result.values)
}

pub fn run_e() {
    let input = File::open("inputs/task_8").unwrap();
    let input = BufReader::new(input);

    let result = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .fold(0_usize, |mut acc, l| {
            let mut i = 0;
            let l = l.as_bytes();
            while i < l.len() {
                if i == 0 || i == l.len() - 1 {
                    acc += 1;
                }
                if l[i] == '"' as u8 {
                    acc += 1;
                }
                if l[i] == '\\' as u8 {
                    acc += 1;
                }
                i += 1;
            }
            acc
        });
    println!("Result: {}", result)
}
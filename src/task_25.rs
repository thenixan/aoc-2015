use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq)]
struct Coords {
    col: usize,
    row: usize,
}

impl Coords {
    fn new() -> Self {
        Coords { col: 1, row: 1 }
    }

    fn next(&mut self) {
        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.col += 1;
            self.row -= 1;
        }
    }
}

pub fn run() {
    let input = File::open("inputs/task_25").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();

    let parameters = buffer
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| i == &15 || i == &17)
        .map(|(_, s)| s.to_string())
        .map(|mut s| {
            s.truncate(s.len() - 1);
            s
        })
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let target = Coords {
        row: parameters[0],
        col: parameters[1],
    };
    let result = evaluate(20151125, &target);

    println!("Result: {}", result);
}

fn evaluate(value: u128, target: &Coords) -> u128 {
    let mut position = Coords::new();

    let mut value = value;
    while &position != target {
        value = (value * 252533_u128) % 33554393_u128;
        position.next();
    }
    value
}

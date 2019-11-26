use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let input = File::open("inputs/task_10").unwrap();
    let input = BufReader::new(input);

    let input = input.lines().into_iter().filter_map(|l| l.ok()).next().unwrap();

    let input = evaluate(input, 40);
    println!("Result: {}", input.len());
}

fn evaluate(input: String, iteration: usize) -> String {
    if iteration == 0 {
        return input;
    } else {
        let modified_input: String = input
            .as_bytes()
            .into_iter()
            .fold(vec![], |mut acc, i| {
                let last = acc.pop();
                if last.is_none() {
                    acc.push(vec![i]);
                } else {
                    let mut last = last.unwrap();
                    if last[0] == i {
                        last.push(i);
                        acc.push(last);
                    } else {
                        acc.push(last);
                        acc.push(vec![i]);
                    }
                }
                acc
            })
            .into_iter()
            .map(|v| {
                format!("{}{}", v.len(), *v[0] as char)
            })
            .collect();
        return evaluate(modified_input, iteration - 1);
    }
}


pub fn run_e() {
    let input = File::open("inputs/task_10").unwrap();
    let input = BufReader::new(input);

    let input = input.lines().into_iter().filter_map(|l| l.ok()).next().unwrap();

    let input = evaluate(input, 50);
    println!("Result: {}", input.len());
}
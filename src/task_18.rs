use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let input = File::open("inputs/task_18").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Vec<bool>> = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.bytes().into_iter().map(|c| c == '#' as u8).collect())
        .collect();

    let mut result = input;
    for _ in 0..100 {
        result = next_step(result);
    }
    let result = count_enabled(&result);
    println!("Result: {}", result);
}


fn next_step(input: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; input[0].len()]; input.len()];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !input[i][j] {
                let count = area_count((i, j), &input);
                result[i][j] = count == 3;
            } else {
                let count = area_count((i, j), &input);
                result[i][j] = count == 2 || count == 3;
            }
        }
    }
    return result;
}

fn area_count(coords: (usize, usize), input: &Vec<Vec<bool>>) -> usize {
    let x = coords.0;
    let y = coords.1;
    let result = input.into_iter()
        .enumerate()
        .filter(|(i, _)| i32::max(0, x as i32 - 1) as usize <= *i && *i <= x + 1)
        .flat_map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .filter(|(j, v)| {
                    i32::max(0, y as i32 - 1) as usize <= *j && *j <= y + 1 && !(*j == y && i == x) && **v
                })
                .map(|(j, _)| { (i, j) })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    return result.iter().count();
}

fn count_enabled(input: &Vec<Vec<bool>>) -> usize {
    input.iter().flat_map(|l| l).filter(|i| **i).count()
}


fn next_step_e(input: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; input[0].len()]; input.len()];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if i == 0 && j == 0 {
                result[i][j] = true;
            } else if i == 0 && j == input[i].len() - 1 {
                result[i][j] = true;
            } else if i == input.len() - 1 && j == input[i].len() - 1 {
                result[i][j] = true;
            } else if i == input.len() - 1 && j == 0 {
                result[i][j] = true;
            } else if !input[i][j] {
                let count = area_count((i, j), &input);
                result[i][j] = count == 3;
            } else {
                let count = area_count((i, j), &input);
                result[i][j] = count == 2 || count == 3;
            }
        }
    }
    return result;
}

pub fn run_e() {
    let input = File::open("inputs/task_18").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Vec<bool>> = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.bytes().into_iter().map(|c| c == '#' as u8).collect())
        .collect();

    let mut result = input;
    let lx = 0;
    let ly = 0;
    result[lx][ly] = true;
    let lx = 0;
    let ly = result[0].len() - 1;
    result[lx][ly] = true;
    let lx = result.len() - 1;
    let ly = 0;
    result[lx][ly] = true;
    let lx = result.len() - 1;
    let ly = result[lx].len() - 1;
    result[lx][ly] = true;
    for _ in 0..100 {
        result = next_step_e(result);
    }
    let result = count_enabled(&result);
    println!("Result: {}", result);
}
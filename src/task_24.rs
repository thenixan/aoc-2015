use std::fs::File;
use std::io::{BufRead, BufReader};

fn e_q(weights: &Vec<i32>) -> u128 {
    weights.iter().fold(1u128, |s, i| s * *i as u128)
}

pub fn run() {
    let input = File::open("inputs/task_24").unwrap();
    let input = BufReader::new(input);

    let weights = input
        .lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    let balance = find_balance(&weights, 3);

    let fills = fill(&weights, balance, 0, 3);
    let legs = fills[0].clone();
    println!("Result: {:?} - {}", legs, e_q(&legs));
}

pub fn run_e() {
    let input = File::open("inputs/task_24").unwrap();
    let input = BufReader::new(input);

    let weights = input
        .lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    let balance = find_balance(&weights, 4);

    let fills = fill(&weights, balance, 0, 4);
    let legs = fills[0].clone();
    println!("Result: {:?} - {}", legs, e_q(&legs));
}

fn find_balance(weights: &Vec<i32>, iterations: i8) -> i32 {
    weights.iter().sum::<i32>() / iterations as i32
}

fn fill(weights: &Vec<i32>, target: i32, count: usize, iterations: i8) -> Vec<Vec<i32>> {
    if count == 0 {
        for c in 1..weights.len() {
            let found_in_legs = &mut fill(weights, target, c, iterations);
            found_in_legs.sort_by(|a, b| e_q(&a).cmp(&e_q(b)));
            for legs_variant in found_in_legs {
                let mut result = vec![legs_variant.clone()];
                for it in 1..iterations {
                    let flattened_result =
                        result.clone().into_iter().flatten().collect::<Vec<i32>>();
                    let filtered_weights: Vec<i32> = weights
                        .clone()
                        .into_iter()
                        .filter_map(|e| {
                            if flattened_result.contains(&e) {
                                None
                            } else {
                                Some(e)
                            }
                        })
                        .collect();

                    let mut c2 = 1;
                    while c2 < filtered_weights.len() && it as usize == result.len() {
                        let next_found = fill(&filtered_weights, target, c2, iterations);
                        if !next_found.is_empty() {
                            result.push(next_found.first().unwrap().clone());
                        }
                        c2 += 1;
                    }
                }
                return result;
            }
        }
        return vec![];
    } else if count == 1 {
        let mut result = vec![];
        for i in 0..weights.len() {
            if weights[i] == target {
                result.push(vec![target]);
            }
        }
        return result;
    } else {
        let mut result = vec![];
        let mut filtered_weights = weights.clone();
        while let Some(next) = filtered_weights.pop() {
            let filled = fill(
                &filtered_weights.clone(),
                target - next,
                count - 1,
                iterations,
            );
            for mut f in filled {
                f.push(next);
                result.push(f);
            }
        }
        return result;
    }
}

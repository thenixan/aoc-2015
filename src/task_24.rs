use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Clone)]
struct Layout {
    weights: Vec<Package>,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Placement {
    Legs,
    Trunk1,
    Trunk2,
}

impl Default for Placement {
    fn default() -> Self {
        Placement::Legs
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Package {
    weight: i32,
    placement: Placement,
}

impl Package {
    fn new(weight: i32, placement: Placement) -> Self {
        Package { weight, placement }
    }
}

impl Layout {
    fn raw_e_q(weights: &Vec<i32>) -> u128 {
        weights.iter().fold(1u128, |s, i| s * *i as u128)
    }
    fn e_q(&self) -> u128 {
        self.weights
            .iter()
            .filter_map(|p| {
                if p.placement == Placement::Legs {
                    Some(p.weight)
                } else {
                    None
                }
            })
            .fold(1u128, |s, i| s * i as u128)
    }

    fn weight(&self, placement: Placement) -> i32 {
        self.weights
            .iter()
            .filter_map(|p| {
                if p.placement == placement {
                    Some(p.weight)
                } else {
                    None
                }
            })
            .sum()
    }

    fn is_balanced(&self) -> bool {
        self.weight(Placement::Legs) == self.weight(Placement::Trunk1)
            && self.weight(Placement::Legs) == self.weight(Placement::Trunk2)
    }

    fn count(&self, placement: Placement) -> usize {
        self.weights
            .iter()
            .filter(|p| p.placement == placement)
            .count()
    }

    fn len(&self) -> usize {
        self.weights.len()
    }
}

impl Ord for Layout {
    fn cmp(&self, other: &Self) -> Ordering {
        match self
            .count(Placement::Legs)
            .cmp(&other.count(Placement::Legs))
        {
            Ordering::Equal => self.e_q().cmp(&other.e_q()),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Layout {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run() {
    let input = File::open("inputs/task_24").unwrap();
    let input = BufReader::new(input);

    let mut weights = input
        .lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    weights.sort();
    weights.reverse();
    // Result: [1, 89, 97, 103, 109, 113] - 10952264083
    // Result: [113, 109, 107, 103, 79, 1] - 10723906903

    let balance = find_balance(&weights);

    let fills = fill(&weights, balance, 0, 3);
    let legs = fills[0].clone();
    println!("Result: {:?} - {}", legs, Layout::raw_e_q(&legs));
}

pub fn run_e() {
    let input = File::open("inputs/task_24").unwrap();
    let input = BufReader::new(input);

    let mut weights = input
        .lines()
        .filter_map(|s| s.ok())
        .map(|l| l.parse::<i32>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i32>>();

    let balance = find_balance(&weights);

    let fills = fill(&weights, balance, 0, 4);
    let legs = fills[0].clone();
    println!("Result: {:?} - {}", legs, Layout::raw_e_q(&legs));
}

fn find_balance(weights: &Vec<i32>) -> i32 {
    weights.iter().sum::<i32>() / 3
}

fn fill(weights: &Vec<i32>, target: i32, count: usize, iterations: i8) -> Vec<Vec<i32>> {
    if count == 0 {
        for c in 1..weights.len() {
            let found_in_legs = &mut fill(weights, target, c, iterations);
            found_in_legs.sort_by(|a, b| Layout::raw_e_q(&a).cmp(&Layout::raw_e_q(b)));
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
        let mut i = 0;
        let mut result = vec![];
        let mut filtered_weights = weights.clone();
        while i < weights.len() && weights[i] < target {
            filtered_weights = filtered_weights
                .into_iter()
                .filter_map(|e| if e == weights[i] { None } else { Some(e) })
                .collect();
            let filled = fill(
                &filtered_weights,
                target - weights[i],
                count - 1,
                iterations,
            );
            for mut f in filled {
                f.push(weights[i]);
                result.push(f);
            }
            i += 1;
        }
        return result;
    }
}

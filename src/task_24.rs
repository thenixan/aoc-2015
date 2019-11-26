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

    let balance = find_balance(&weights);

    let fills = fill(&weights, balance, 0);
    println!("Result: {}", fills.len());
}

pub fn run_e() {
    let input = File::open("inputs/task_24").unwrap();
    let input = BufReader::new(input);
}

fn find_balance(weights: &Vec<i32>) -> i32 {
    weights.iter().sum::<i32>() / 3
}

fn fill(weights: &Vec<i32>, target: i32, count: usize) -> Vec<Vec<i32>> {
    if count == 0 {
        for c in 1..weights.len() - 2 {
            let r = fill(weights, target, c);
            println!("Found: {:?}", r);
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
        while i < weights.len() && weights[i] < target {
            let filled = fill(
                &weights
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(n, e)| if n == i { None } else { Some(e) })
                    .collect(),
                target - weights[i],
                count - 1,
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

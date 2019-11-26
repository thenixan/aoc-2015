use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Properties {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Properties {
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Properties { name: name.clone().to_string(), capacity, durability, flavor, texture, calories }
    }
}

impl FromStr for Properties {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(": ").collect();
        let name = s[0];
        let p: Vec<&str> = s[1].split(", ").collect();

        let capacity = p[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let durability = p[1].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let flavor = p[2].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let texture = p[3].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let calories = p[4].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        Ok(Properties::new(name, capacity, durability, flavor, texture, calories))
    }
}

pub fn run() {
    let input = File::open("inputs/task_15").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Properties> = input
        .lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Properties>().ok())
        .collect();

    let result = evaluate(&input, &vec![]);

    println!("Result: {}", result)
}

fn evaluate(properties: &Vec<Properties>, mtlpl: &Vec<i32>) -> i32 {
    if mtlpl.len() < properties.len() - 1 {
        let mut max = 0;
        for x in 0..=100 {
            let mut x_vec = mtlpl.clone();
            x_vec.push(x);
            let m = evaluate(properties, &x_vec);
            max = i32::max(max, m);
        }
        return max;
    } else {
        let x = mtlpl.into_iter().fold(100, |acc, x| acc - *x);
        let mut x_vec = mtlpl.clone();
        x_vec.push(x);
        let result: (i32, i32, i32, i32) = properties.into_iter().zip(x_vec.iter()).fold((0, 0, 0, 0), |mut acc, (p, x)| {
            acc.0 += p.capacity * *x;
            acc.1 += p.durability * *x;
            acc.2 += p.flavor * *x;
            acc.3 += p.texture * *x;
            acc
        });
        return if result.0 <= 0 || result.1 <= 0 || result.2 <= 0 || result.3 <= 0 { 0 } else { result.0 * result.1 * result.2 * result.3 };
    }
}


pub fn run_e() {
    let input = File::open("inputs/task_15").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Properties> = input
        .lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Properties>().ok())
        .collect();

    let result = evaluate_with_calories(&input, &vec![]);

    println!("Result: {}", result)
}

fn evaluate_with_calories(properties: &Vec<Properties>, mtlpl: &Vec<i32>) -> i32 {
    if mtlpl.len() < properties.len() - 1 {
        let mut max = 0;
        for x in 0..=100 {
            let mut x_vec = mtlpl.clone();
            x_vec.push(x);
            let m = evaluate_with_calories(properties, &x_vec);
            max = i32::max(max, m);
        }
        return max;
    } else {
        let x = mtlpl.into_iter().fold(100, |acc, x| acc - *x);
        let mut x_vec = mtlpl.clone();
        x_vec.push(x);
        let result: (i32, i32, i32, i32, i32) = properties.into_iter().zip(x_vec.iter()).fold((0, 0, 0, 0, 0), |mut acc, (p, x)| {
            acc.0 += p.capacity * *x;
            acc.1 += p.durability * *x;
            acc.2 += p.flavor * *x;
            acc.3 += p.texture * *x;
            acc.4 += p.calories * *x;
            acc
        });
        return if result.0 <= 0 || result.1 <= 0 || result.2 <= 0 || result.3 <= 0 || result.4 != 500 { 0 } else { result.0 * result.1 * result.2 * result.3 };
    }
}
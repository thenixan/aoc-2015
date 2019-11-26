use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

struct Terms {
    name: String,
    speed: u32,
    active: u32,
    rest: u32,
}

impl Terms {
    fn new(name: String, speed: u32, active: u32, rest: u32) -> Self { Terms { name, speed, active, rest } }
    fn position(&self, time: u32) -> u32 {
        let cycle_length = self.active + self.rest;
        let cycles = time / cycle_length;
        let cycles_distance = cycles * self.speed * self.active;
        let non_full_cycles = time % cycle_length;
        let non_full_active = u32::min(non_full_cycles, self.active);
        let non_full_distance = non_full_active * self.speed;
        return non_full_distance + cycles_distance;
    }
}

pub fn run() {
    let input = File::open("inputs/task_14").unwrap();
    let input = BufReader::new(input);
    let input: Vec<Terms> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l.split(" ").map(|s| s.to_string()).collect())
        .map(|l: Vec<String>| {
            Terms::new(
                l[0].clone(),
                l[3].parse::<u32>().unwrap(),
                l[6].parse::<u32>().unwrap(),
                l[13].parse::<u32>().unwrap(),
            )
        })
        .collect();

    let target = 2503;
    let result = input
        .into_iter()
        .map(|t| t.position(target))
        .max()
        .unwrap();
    println!("Result: {}", result);
}


pub fn run_e() {
    let input = File::open("inputs/task_14").unwrap();
    let input = BufReader::new(input);
    let input: Vec<Terms> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l.split(" ").map(|s| s.to_string()).collect())
        .map(|l: Vec<String>| {
            Terms::new(
                l[0].clone(),
                l[3].parse::<u32>().unwrap(),
                l[6].parse::<u32>().unwrap(),
                l[13].parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut points = HashMap::new();
    let target = 2503;

    for i in 1..target {
        let result = input
            .iter()
            .map(|t| t.position(i))
            .max()
            .unwrap();
        input.iter()
            .for_each(|t| if t.position(i) == result { *points.entry(&t.name).or_insert(0u32) += 1; });
    }

    let result = points.values().max().unwrap();

    println!("Result: {}", result);
}
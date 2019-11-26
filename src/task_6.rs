use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::RangeInclusive;

struct Plane {
    lights: [[bool; 1000]; 1000],
}

impl Plane {
    fn new() -> Plane { Plane { lights: [[false; 1000]; 1000] } }

    fn on(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.lights[x][y] = true;
            }
        }
    }

    fn off(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.lights[x][y] = false;
            }
        }
    }

    fn toggle(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.lights[x][y] = !self.lights[x][y];
            }
        }
    }

    fn count_enabled(&self) -> usize {
        let mut result: usize = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.lights[x][y] { result += 1; }
            }
        }
        return result;
    }
}

struct BrightnessControlledPlane {
    lights: [[u16; 1000]; 1000],
}

impl BrightnessControlledPlane {
    fn new() -> BrightnessControlledPlane { BrightnessControlledPlane { lights: [[0u16; 1000]; 1000] } }

    fn on(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.lights[x][y] += 1;
            }
        }
    }

    fn off(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                if self.lights[x][y] != 0 {
                    self.lights[x][y] -= 1;
                }
            }
        }
    }

    fn toggle(&mut self, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.lights[x][y] += 2;
            }
        }
    }

    fn count_brightness(&self) -> u32 {
        let mut result: u32 = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                result += self.lights[x][y] as u32;
            }
        }
        return result;
    }
}

pub fn run() {
    let input = File::open("inputs/task_6").unwrap();
    let input = BufReader::new(input);

    let plane = &mut Plane::new();

    input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            if l.starts_with("turn ") {
                l.as_str()[5..].to_string()
            } else {
                l
            }
        })
        .map(|l| {
            l.split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|l| {
            let from = l[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let to = l[3].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            match l[0].as_str() {
                "on" => plane.on(from[0]..=to[0], from[1]..=to[1]),
                "off" => plane.off(from[0]..=to[0], from[1]..=to[1]),
                "toggle" => plane.toggle(from[0]..=to[0], from[1]..=to[1]),
                _ => { panic!("Unknown directive") }
            }
        });

    println!("Result: {}", plane.count_enabled());
}

pub fn run_e() {
    let input = File::open("inputs/task_6").unwrap();
    let input = BufReader::new(input);

    let plane = &mut BrightnessControlledPlane::new();

    input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            if l.starts_with("turn ") {
                l.as_str()[5..].to_string()
            } else {
                l
            }
        })
        .map(|l| {
            l.split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|l| {
            let from = l[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let to = l[3].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            match l[0].as_str() {
                "on" => plane.on(from[0]..=to[0], from[1]..=to[1]),
                "off" => plane.off(from[0]..=to[0], from[1]..=to[1]),
                "toggle" => plane.toggle(from[0]..=to[0], from[1]..=to[1]),
                _ => { panic!("Unknown directive") }
            }
        });

    println!("Result: {}", plane.count_brightness());
}
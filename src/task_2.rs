use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

struct Dimensions {
    x: i32,
    y: i32,
    z: i32,
}

impl Dimensions {
    fn new(x: i32, y: i32, z: i32) -> Dimensions { Dimensions { x, y, z } }

    fn side1(&self) -> i32 {
        return self.x * self.y;
    }

    fn side2(&self) -> i32 {
        return self.y * self.z;
    }

    fn side3(&self) -> i32 {
        return self.z * self.x;
    }

    fn box_area(&self) -> i32 {
        return 2 * self.side1() + 2 * self.side2() + 2 * self.side3();
    }

    fn slack_area(&self) -> i32 {
        return self.side1().min(self.side2()).min(self.side3());
    }

    fn ribbon_length(&self) -> i32 {
        return self.x * self.y * self.z;
    }

    fn wrap_length(&self) -> i32 {
        let side1 = 2 * self.x;
        let side2 = 2 * self.y;
        let side3 = 2 * self.z;
        let max = side1.max(side2).max(side3);
        return side1 + side2 + side3 - max;
    }
}

impl FromStr for Dimensions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s.split("x").collect::<Vec<&str>>();
        return Ok(Dimensions::new(a[0].parse::<i32>().unwrap(), a[1].parse::<i32>().unwrap(), a[2].parse::<i32>().unwrap()));
    }
}

pub fn run() {
    let input = File::open("inputs/task_2").unwrap();
    let input = BufReader::new(input);
    let result: i32 = input.lines()
        .into_iter()
        .map(|l| { l.unwrap().parse::<Dimensions>().unwrap() })
        .map(|d| { d.box_area() + d.slack_area() })
        .sum();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("inputs/task_2").unwrap();
    let input = BufReader::new(input);
    let result: i32 = input.lines()
        .into_iter()
        .map(|l| { l.unwrap().parse::<Dimensions>().unwrap() })
        .map(|d| { d.ribbon_length() + d.wrap_length() })
        .sum();
    println!("Result: {}", result);
}
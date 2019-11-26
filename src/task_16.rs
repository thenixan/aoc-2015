use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Property {
    Children(i32),
    Cats(i32),
    Samoyeds(i32),
    Pomeranians(i32),
    Akitas(i32),
    Vizslas(i32),
    Goldfish(i32),
    Trees(i32),
    Cars(i32),
    Perfumes(i32),
}

#[derive(Debug)]
struct Prediction {
    number: i32,
    properties: Vec<Property>,
}

impl Property {
    fn matches(&self, known: &Vec<Self>) -> bool { known.contains(self) }
    fn matches_e(&self, known: &Vec<Self>) -> bool {
        match self {
            Property::Cats(c) | Property::Trees(c) => known.iter().all(|p| {
                match p {
                    Property::Cats(pc) | Property::Trees(pc) => c > pc,
                    _ => true
                }
            }),
            Property::Pomeranians(c) | Property::Goldfish(c) => known.iter().all(|p| {
                match p {
                    Property::Pomeranians(pc) | Property::Goldfish(pc) => c < pc,
                    _ => true
                }
            }),
            _ => self.matches(known)
        }
    }
}

impl FromStr for Property {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(": ").collect();
        let count = input[1].parse::<i32>();
        match count {
            Result::Err(e) => Result::Err(e.to_string()),
            Result::Ok(c) => {
                match input[0] {
                    "children" => Result::Ok(Property::Children(c)),
                    "cats" => Result::Ok(Property::Cats(c)),
                    "samoyeds" => Result::Ok(Property::Samoyeds(c)),
                    "pomeranians" => Result::Ok(Property::Pomeranians(c)),
                    "akitas" => Result::Ok(Property::Akitas(c)),
                    "vizslas" => Result::Ok(Property::Vizslas(c)),
                    "goldfish" => Result::Ok(Property::Goldfish(c)),
                    "trees" => Result::Ok(Property::Trees(c)),
                    "cars" => Result::Ok(Property::Cars(c)),
                    "perfumes" => Result::Ok(Property::Perfumes(c)),
                    _ => Result::Err("Unknown property".to_string())
                }
            }
        }
    }
}

impl FromStr for Prediction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.splitn(2, ": ").collect();
        let number = input[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>();
        match number {
            Result::Err(e) => Result::Err(e.to_string()),
            Result::Ok(n) => {
                let properties: Vec<&str> = input[1].split(", ").collect();
                let properties: Vec<Property> = properties.into_iter()
                    .map(|l| l.parse::<Property>())
                    .filter_map(|l| l.ok())
                    .collect();
                Result::Ok(Prediction { number: n, properties })
            }
        }
    }
}

impl Prediction {
    fn matches(&self, known: &Vec<Property>) -> bool { self.properties.iter().all(|p| p.matches(known)) }
    fn matches_e(&self, known: &Vec<Property>) -> bool { self.properties.iter().all(|p| p.matches_e(known)) }
}

pub fn run() {
    let input = File::open("inputs/task_16").unwrap();
    let input = BufReader::new(input);

    let known = vec![
        Property::Children(3),
        Property::Cats(7),
        Property::Samoyeds(2),
        Property::Pomeranians(3),
        Property::Akitas(0),
        Property::Vizslas(0),
        Property::Goldfish(5),
        Property::Trees(3),
        Property::Cars(2),
        Property::Perfumes(1)];

    let input: Vec<Prediction> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<Prediction>())
        .filter_map(|p| p.ok())
        .collect();

    let result = input.iter()
        .find(|p| p.matches(&known));

    println!("Result: {}", result.unwrap().number)
}


pub fn run_e() {
    let input = File::open("inputs/task_16").unwrap();
    let input = BufReader::new(input);

    let known = vec![
        Property::Children(3),
        Property::Cats(7),
        Property::Samoyeds(2),
        Property::Pomeranians(3),
        Property::Akitas(0),
        Property::Vizslas(0),
        Property::Goldfish(5),
        Property::Trees(3),
        Property::Cars(2),
        Property::Perfumes(1)];

    let input: Vec<Prediction> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<Prediction>())
        .filter_map(|p| p.ok())
        .collect();

    let result = input.iter()
        .find(|p| p.matches_e(&known));

    println!("Result: {}", result.unwrap().number)
}
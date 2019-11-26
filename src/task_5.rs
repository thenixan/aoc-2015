use std::fs::File;
use std::io::{BufReader, BufRead};

struct VowelsCheck {
    count: usize,
}

struct TwiceCheck {
    prev: Option<char>,
    triggered: bool,
}

struct SubstringCheck {
    prev: Option<char>,
    triggered: bool,
}

#[derive(Debug)]
struct TwoLettersTwice {
    first: Option<char>,
    tuples: Vec<(char, char)>,
    triggered: bool,
}

struct TwoLettersBetween {
    target: Option<char>,
    middle: Option<char>,
    triggered: bool,
}

impl VowelsCheck {
    fn new() -> VowelsCheck { VowelsCheck { count: 0 } }
}

impl TwiceCheck {
    fn new() -> TwiceCheck { TwiceCheck { prev: None, triggered: false } }
}

impl SubstringCheck {
    fn new() -> SubstringCheck { SubstringCheck { prev: None, triggered: false } }
}

impl TwoLettersTwice {
    fn new() -> TwoLettersTwice { TwoLettersTwice { first: None, tuples: vec![], triggered: false } }
}

impl TwoLettersBetween {
    fn new() -> TwoLettersBetween { TwoLettersBetween { target: None, middle: None, triggered: false } }
}


trait Check {
    fn check(&mut self, c: char);
    fn is_valid(&self) -> bool;
}

impl Check for VowelsCheck {
    fn check(&mut self, c: char) {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => self.count += 1,
            _ => {}
        }
    }
    fn is_valid(&self) -> bool { self.count >= 3 }
}

impl Check for TwiceCheck {
    fn check(&mut self, c: char) {
        if self.prev.is_none() {
            self.prev = Some(c);
        } else if !self.triggered && self.prev.is_some() {
            if self.prev.unwrap() == c {
                self.triggered = true;
            } else {
                self.prev = Some(c);
            }
        }
    }

    fn is_valid(&self) -> bool { self.triggered }
}

impl Check for SubstringCheck {
    fn check(&mut self, c: char) {
        if !self.triggered {
            if self.prev.is_none() {
                self.prev = Some(c);
            } else {
                let p = self.prev.unwrap();
                match (p, c) {
                    ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => self.triggered = true,
                    _ => self.prev = Some(c)
                }
            }
        }
    }

    fn is_valid(&self) -> bool { !self.triggered }
}

impl Check for TwoLettersTwice {
    fn check(&mut self, c: char) {
        if !self.triggered {
            if self.first.is_none() && self.tuples.is_empty() {
                self.first = Some(c)
            } else if self.first.is_some() {
                let p = self.first.unwrap();
                self.first = None;
                let result = (p, c);
                if self.tuples.contains(&result) {
                    self.triggered = true;
                } else {
                    self.tuples.push(result);
                }
            } else if !self.tuples.is_empty() {
                let last = self.tuples.last().unwrap().1;
                let result = (last, c);
                let size = self.tuples.len();
                if self.tuples[..size - 1].contains(&result) {
                    self.triggered = true;
                } else {
                    self.tuples.push(result)
                }
            }
        }
    }

    fn is_valid(&self) -> bool { self.triggered }
}

impl Check for TwoLettersBetween {
    fn check(&mut self, c: char) {
        if self.target.is_some() && self.target.unwrap() == c {
            self.triggered = true;
        } else {
            self.target = self.middle;
            self.middle = Some(c);
        }
    }

    fn is_valid(&self) -> bool { self.triggered }
}

pub fn run() {
    let input = File::open("inputs/task_5").unwrap();
    let input = BufReader::new(input);
    let result = input.lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            let check1 = &mut VowelsCheck::new();
            let check2 = &mut TwiceCheck::new();
            let check3 = &mut SubstringCheck::new();
            l.chars().into_iter().for_each(|c| {
                check1.check(c);
                check2.check(c);
                check3.check(c)
            });
            check1.is_valid() && check2.is_valid() && check3.is_valid()
        })
        .count();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("inputs/task_5").unwrap();
    let input = BufReader::new(input);
    let result = input.lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            let check1 = &mut TwoLettersTwice::new();
            let check2 = &mut TwoLettersBetween::new();
            l.chars().into_iter().for_each(|c| {
                check1.check(c);
                check2.check(c)
            });
//            println!("{}: {},{}", l, check1.is_valid(), check2.is_valid());
            check1.is_valid() && check2.is_valid()
        })
//        .map(|s| println!("{}", s))
        .count();
    println!("Result: {}", result);
}
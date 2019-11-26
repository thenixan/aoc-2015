use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use regex::Regex;
use std::ops::{Index, Deref};
use std::slice::SliceIndex;
use std::collections::{BTreeSet, HashSet};
use std::fmt::{Display, Formatter};
use core::fmt::Write;
use core::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
struct Element {
    name: String
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.name, f)
    }
}

impl Element {
    fn new(name: String) -> Self { Element { name } }
}

//impl ToString for Element {
//    fn to_string(&self) -> String { self.name.clone() }
//}

impl FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Element::new(s.to_string())) }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
struct Chain {
    value: Vec<Element>
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char('[').expect("Cannot write debug");
        for i in 0..self.value.len() {
            Display::fmt(&self.value[i], f).expect("Cannot write debug");
            f.write_char(' ').expect("Cannot write debug");
        }
        f.write_char(']')
    }
}

impl IntoIterator for Chain {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl Deref for Chain {
    type Target = [Element];

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([A-Z][a-z]?)").unwrap();
        let mut result: Vec<Element> = vec![];
        for cap in re.captures_iter(s) {
            result.push(cap[0].parse().unwrap())
        }
        Ok(Chain { value: result })
    }
}

impl<I: SliceIndex<[Element]>> Index<I> for Chain {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.value.index(index)
    }
}

impl Chain {
    fn len(&self) -> usize { self.value.len() }
    fn replace(mut self, pos: usize, len: usize, with: &Chain) -> Self {
        for _ in 0..len {
            self.value.remove(pos);
        }
        for i in 0..with.len() {
            self.value.insert(pos + i, with[i].clone());
        }
        self
    }
    fn replace_item(mut self, pos: usize, len: usize, with: &Element) -> Self {
        for _ in 0..len {
            self.value.remove(pos);
        }
        self.value.insert(pos, with.clone());
        self
    }
    fn contains(&self, other: &Element) -> bool {
        self.value.contains(other)
    }

    fn indexes_of_subchain(&self, other: &Chain) -> Vec<usize> {
        let mut result = vec![];
        if self.len() < other.len() {
            return result;
        }
        for i in 0..self.len() - other.len() {
            if self[i] == other[0] {
                let mut j = 0;
                while j < other.len() && self[i + j] == other[j] {
                    j += 1;
                }
                if j == other.len() {
                    result.push(i);
                }
            }
        }
        return result;
    }
}

//impl Into<Chain> for Element {
//    fn into(self) -> Chain {
//        Chain { value: vec![self] }
//    }
//}

impl From<Element> for Chain {
    fn from(e: Element) -> Self {
        Chain { value: vec![e] }
    }
}

//impl ToString for Chain {
//    fn to_string(&self) -> String {
//        self.value.iter().map(|v| v.to_string()).join("")
//    }
//}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Replacement {
    from: Element,
    to: Chain,
}

impl FromStr for Replacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" => ").collect();
        let from: Element = s[0].parse().unwrap();
        let to: Chain = s[1].parse().unwrap();
        Result::Ok(Replacement { from, to })
    }
}

pub fn run() {
    let input = File::open("inputs/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;
    let target: Chain = input.pop().unwrap().clone().parse().unwrap();

    let input: Vec<Replacement> = input.iter().filter_map(|l| l.parse::<Replacement>().ok()).collect();

    let mut result: BTreeSet<String> = BTreeSet::new();

    for i in 0..target.len() {
        for j in 0..input.len() {
            if target[i] == input[j].from {
                result.insert(target.clone().replace(i, 1, &input[j].to).to_string());
            }
        }
    }

    println!("Result: {}", result.len());
}

pub fn run_e() {
    let input = File::open("inputs/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;

    let target: Chain = input.pop().unwrap().clone().parse().unwrap();

    let mut input: Vec<Replacement> = input.iter()
        .filter_map(|l| l.parse::<Replacement>().ok())
        .collect();

    input = input
        .into_iter()
        .map(|r| {
            let from = Element::new("X".to_string());
            let to = Chain {
                value: r.to.value
                    .into_iter()
                    .map(|e| {
                        if e.name != "Rn" && e.name != "Y" && e.name != "Ar" {
                            Element::new("X".to_string())
                        } else {
                            e
                        }
                    })
                    .collect()
            };
            Replacement { from, to }
        })
        .collect();

    input.sort();

    input.dedup();

    let target = Chain {
        value: target
            .value
            .into_iter()
            .map(|e| {
                if e.name != "Rn" && e.name != "Y" && e.name != "Ar" {
                    Element::new("X".to_string())
                } else {
                    e
                }
            })
            .collect()
    };

    let elements = elements(&input);
    let terminal_elements = find_terminal_elements(&elements, &input);

    let terminal_mutations = find_mutations_with_elements(&terminal_elements, &input);
    let non_terminal_mutations: Vec<Replacement> = input.into_iter().filter(|m| !terminal_mutations.contains(&m)).collect();

    let mut history = HashSet::new();
//    let result = iterate(target.as_str(), &input, 0);

    let result = iterate(&mut history, &terminal_mutations, &non_terminal_mutations, &target, 0).unwrap();
    println!("Result: {}", result);
}

fn iterate(history: &mut HashSet<Chain>, terminal_mutations: &Vec<Replacement>, non_terminal_mutations: &Vec<Replacement>, target: &Chain, step: usize) -> Option<usize> {
    let mut result = None;

    if target.len() == 1 && &target[0].name == "X" {
        return Some(step);
    }

    let mut terminal_mutations_count = 0;

    let mut target = target.clone();
    for i in 0..terminal_mutations.len() {
        let indexes = target.indexes_of_subchain(&terminal_mutations[i].to);
        for j in 0..indexes.len() {
            let ind = indexes[j] - (j * terminal_mutations[i].to.len()) + j;
            target = target.replace_item(ind, terminal_mutations[i].to.len(), &terminal_mutations[i].from);
            terminal_mutations_count += 1;
        }
    }

    let mut non_terminal_mutations_count = 0;

    for i in 0..non_terminal_mutations.len() {
        let mutation = &non_terminal_mutations[i];
        let mut ind = target.indexes_of_subchain(&mutation.to);
        if ind.len() == 0 && target == mutation.to {
            ind = vec![0]
        }
        while ind.len() != 0 {
            let index = ind.first().unwrap();
            target = target.replace_item(*index, mutation.to.len(), &mutation.from);
            ind = target.indexes_of_subchain(&mutation.to);
            non_terminal_mutations_count += 1;
        }
    }

    let r = iterate(history, terminal_mutations, non_terminal_mutations, &target, step + non_terminal_mutations_count + terminal_mutations_count);
    if r.is_some() && (result.is_none() || result.unwrap() > r.unwrap()) {
        result = r;
    }

    return result;
}

fn find_mutations_with_elements(elements: &Vec<Element>, mutations: &Vec<Replacement>) -> Vec<Replacement> {
    mutations.clone().into_iter().filter(|p| {
        elements.iter().any(|e| p.to.contains(e))
    }).collect()
}

fn find_terminal_elements(elements: &Vec<Element>, mutations: &Vec<Replacement>) -> Vec<Element> {
    let mut result = vec![];
    for e in elements {
        if mutations.iter().any(|p| p.to.contains(e)) && !mutations.iter().any(|p| &p.from == e) {
            result.push(e.clone())
        }
    }
    return result;
}

fn elements(mutations: &Vec<Replacement>) -> Vec<Element> {
    let mut result: HashSet<Element> = HashSet::new();
    for i in 0..mutations.len() {
        result.insert(mutations[i].from.clone());
        for j in 0..mutations[i].to.len() {
            result.insert(mutations[i].to[j].clone());
        }
    }
    return result.into_iter().collect();
}
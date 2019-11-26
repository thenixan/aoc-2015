use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Variable {
    Named(String),
    Valued(u16),
}

impl From<String> for Variable {
    fn from(s: String) -> Self {
        match s.parse::<u16>() {
            Ok(v) => Variable::Valued(v),
            Err(_) => Variable::Named(s)
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    Input { value: u16, output: String },
    Not { input: Variable, output: String },
    Link { input: Variable, output: String },
    RShift { value: u16, input: Variable, output: String },
    LShift { value: u16, input: Variable, output: String },
    Or { left: Variable, right: Variable, output: String },
    And { left: Variable, right: Variable, output: String },
}

impl Command {
    fn inputs(&self) -> Vec<&str> {
        match self {
            Command::Input { .. } => Vec::new(),
            Command::Link { input, .. } => {
                match input {
                    Variable::Named(s) => vec![s],
                    Variable::Valued(_) => vec![]
                }
            }
            Command::Not { input, .. } => {
                match input {
                    Variable::Named(s) => vec![s],
                    Variable::Valued(_) => vec![]
                }
            }
            Command::RShift { input, .. } => {
                match input {
                    Variable::Named(s) => vec![s],
                    Variable::Valued(_) => vec![]
                }
            }
            Command::LShift { input, .. } => {
                match input {
                    Variable::Named(s) => vec![s],
                    Variable::Valued(_) => vec![]
                }
            }
            Command::Or { left, right, .. } => {
                match (left, right) {
                    (Variable::Named(l), Variable::Named(r)) => vec![l, r],
                    (Variable::Named(l), Variable::Valued(_)) => vec![l],
                    (Variable::Valued(_), Variable::Named(r)) => vec![r],
                    _ => vec![]
                }
            }
            Command::And { left, right, .. } => {
                match (left, right) {
                    (Variable::Named(l), Variable::Named(r)) => vec![l, r],
                    (Variable::Named(l), Variable::Valued(_)) => vec![l],
                    (Variable::Valued(_), Variable::Named(r)) => vec![r],
                    _ => vec![]
                }
            }
        }
    }

    fn output(&self) -> &str {
        match self {
            Command::Input { output, .. } => output,
            Command::Link { output, .. } => output,
            Command::Not { output, .. } => output,
            Command::RShift { output, .. } => output,
            Command::LShift { output, .. } => output,
            Command::Or { output, .. } => output,
            Command::And { output, .. } => output,
        }
    }

    fn run(&self, values: Vec<u16>) -> u16 {
        match self {
            Command::Input { value, .. } => *value,
            Command::Not { input, .. } => {
                match input {
                    Variable::Valued(v) => !*v,
                    _ => !values[0]
                }
            }
            Command::Link { input, .. } => {
                match input {
                    Variable::Valued(v) => *v,
                    _ => values[0]
                }
            }
            Command::RShift { input, value, .. } => {
                match input {
                    Variable::Valued(v) => *v >> *value,
                    _ => values[0] >> *value,
                }
            }
            Command::LShift { input, value, .. } => {
                match input {
                    Variable::Valued(v) => *v << *value,
                    _ => values[0] << *value,
                }
            }
            Command::Or { left, right, .. } => {
                match (left, right) {
                    (Variable::Valued(l), Variable::Valued(r)) => *l | *r,
                    (Variable::Valued(l), _) => *l | values[0],
                    (_, Variable::Valued(r)) => values[0] | *r,
                    _ => values[0] | values[1],
                }
            }
            Command::And { left, right, .. } => {
                match (left, right) {
                    (Variable::Valued(l), Variable::Valued(r)) => *l & *r,
                    (Variable::Valued(l), _) => *l & values[0],
                    (_, Variable::Valued(r)) => values[0] & *r,
                    _ => values[0] & values[1],
                }
            }
        }
    }
}

trait Identified {
    fn identificators(&self) -> Vec<&str>;
    fn values(&self, v: &Vec<String>) -> Vec<u16>;
}

fn evaluate(commands: &Vec<Command>) -> u16 {
    let mut values = HashMap::new();
    for s in commands.iter().filter(|c| c.inputs().is_empty()) {
        values.insert(s.output().to_string(), s.run(vec![]));
    }
    let result = loop {
        if values.contains_key("a") {
            break values.get("a").unwrap();
        }
        let mut loop_values = HashMap::new();
        for s in commands.iter().filter(|c| { c.inputs().iter().all(|f| values.contains_key(&f.to_string())) }) {
            let args: Vec<u16> = s.inputs().into_iter().map(|k| *values.get(k).unwrap()).collect();
            loop_values.insert(s.output().to_string(), s.run(args));
        }
        values.extend(loop_values);
    };
    return *result;
}


pub fn run() {
    let input = File::open("inputs/task_7").unwrap();
    let input = BufReader::new(input);

    let commands = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let splitted: Vec<&str> = l.split(" -> ").collect();
            let splitted = (splitted[0].to_string(), splitted[1].to_string());
            splitted
        })
        .map(|(i, o)| {
            let is_number = i.parse::<u16>();
            if is_number.is_ok() {
                Command::Input { value: is_number.unwrap(), output: o }
            } else {
                let splitted: Vec<&str> = i.split(" ").collect();
                if splitted.len() == 1 {
                    Command::Link { input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[0] == "NOT" {
                    Command::Not { input: Variable::from(splitted[1].to_string()), output: o }
                } else if splitted[1] == "RSHIFT" {
                    Command::RShift { value: splitted[2].parse().unwrap(), input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[1] == "LSHIFT" {
                    Command::LShift { value: splitted[2].parse().unwrap(), input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[1] == "OR" {
                    Command::Or { left: Variable::from(splitted[0].to_string()), right: Variable::from(splitted[2].to_string()), output: o }
                } else if splitted[1] == "AND" {
                    Command::And { left: Variable::from(splitted[0].to_string()), right: Variable::from(splitted[2].to_string()), output: o }
                } else {
                    unimplemented!("{:?}", splitted);
                }
            }
        })
        .collect::<Vec<Command>>();
    let result = evaluate(&commands);
    println!("Result: {}", result);
}


pub fn run_e() {
    let input = File::open("inputs/task_7").unwrap();
    let input = BufReader::new(input);

    let mut commands = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let splitted: Vec<&str> = l.split(" -> ").collect();
            let splitted = (splitted[0].to_string(), splitted[1].to_string());
            splitted
        })
        .map(|(i, o)| {
            let is_number = i.parse::<u16>();
            if is_number.is_ok() {
                Command::Input { value: is_number.unwrap(), output: o }
            } else {
                let splitted: Vec<&str> = i.split(" ").collect();
                if splitted.len() == 1 {
                    Command::Link { input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[0] == "NOT" {
                    Command::Not { input: Variable::from(splitted[1].to_string()), output: o }
                } else if splitted[1] == "RSHIFT" {
                    Command::RShift { value: splitted[2].parse().unwrap(), input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[1] == "LSHIFT" {
                    Command::LShift { value: splitted[2].parse().unwrap(), input: Variable::from(splitted[0].to_string()), output: o }
                } else if splitted[1] == "OR" {
                    Command::Or { left: Variable::from(splitted[0].to_string()), right: Variable::from(splitted[2].to_string()), output: o }
                } else if splitted[1] == "AND" {
                    Command::And { left: Variable::from(splitted[0].to_string()), right: Variable::from(splitted[2].to_string()), output: o }
                } else {
                    unimplemented!("{:?}", splitted);
                }
            }
        })
        .collect::<Vec<Command>>();
    let first_result = evaluate(&commands);

    let input = commands
        .iter()
        .enumerate()
        .find_map(|(s, c)| {
            match c {
                Command::Input { output, .. } => if output == "b" { Some(s) } else { None },
                _ => None
            }
        })
        .unwrap();

    commands.remove(input);
    commands.push(Command::Input { value: first_result, output: "b".to_string() });

    let result = evaluate(&commands);

    println!("Result: {}", result);
}

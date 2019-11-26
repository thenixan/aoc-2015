use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Terms {
    subject: String,
    score: i32,
    target: String,
}

#[derive(Debug, Clone)]
struct ComputedTerms {
    subjects: Vec<String>,
    score: i32,
}

pub fn run() {
    let input = File::open("inputs/task_13").unwrap();
    let input = BufReader::new(input);
    let mut input: Vec<ComputedTerms> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l[..l.len() - 1].to_string())
        .map(|l| l.split(" ").map(|l| l.to_string()).collect())
        .map(|l: Vec<String>| {
            let m = if l[2] == "gain" { 1 } else { -1 };
            Terms { subject: l[0].clone(), score: l[3].parse::<i32>().unwrap() * m, target: l[10].clone() }
        })
        .fold(vec![], |acc, t| {
            let mut contains = false;
            let mut acc: Vec<ComputedTerms> = acc
                .into_iter()
                .map(|mut c| {
                    if c.subjects.contains(&t.subject) && c.subjects.contains(&t.target) {
                        c.score += t.score;
                        contains = true;
                    }
                    c
                })
                .collect();
            if !contains {
                acc.push(ComputedTerms { subjects: vec![t.subject, t.target], score: t.score });
            }
            acc
        });
    input.sort_by(|a, b| b.score.cmp(&a.score));

    let subjects = input.iter().fold(vec![], |mut acc, i| {
        if !acc.contains(&i.subjects[0]) {
            acc.push(i.subjects[0].clone());
        }
        if !acc.contains(&i.subjects[1]) {
            acc.push(i.subjects[1].clone());
        }
        acc
    });

    let first = input.first().unwrap().clone();
    let result = vec![first.clone()];

    let result = iterate(&input, result, subjects.into_iter().filter(|l| !first.subjects.contains(l)).collect());

    let result: i32 = result.iter().map(|c| c.score).sum();

    println!("{:?}", result);
}

fn calc_score(scores: &Vec<ComputedTerms>) -> i32 {
    return scores.iter().map(|c| c.score).sum();
}

fn iterate(terms: &Vec<ComputedTerms>, mut result: Vec<ComputedTerms>, subjects: Vec<String>) -> Vec<ComputedTerms> {
//    if result.len() == subjects.len() - 1 {
//        return result;
//    }
    let last = if result.len() == 1 {
        result.last().unwrap().subjects[1].clone()
    } else {
        let s1 = result[result.len() - 2].subjects.clone();
        let s2 = result[result.len() - 1].subjects.clone();
        if s1.contains(&s2[0]) {
            s2[1].clone()
        } else {
            s2[0].clone()
        }
    };

    let mut max = None;
    let mut i = 0;
    while i < subjects.len() {
        let last = terms.into_iter().find(|p| p.subjects.contains(&last) && p.subjects.contains(&subjects[i]));
        match last {
            Some(l) => {
                let mut n = result.clone();
                n.push(l.clone());
                let n = iterate(terms, n, subjects.clone().into_iter().filter(|s| !l.subjects.contains(s)).collect());
                if max.is_none() {
                    max = Some(n);
                } else {
                    let nm = calc_score(&n);
                    if nm > calc_score(&max.clone().unwrap()) {
                        max = Some(n.clone());
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    if max.is_none() {
        let last = {
            let s1 = result[result.len() - 2].subjects.clone();
            let s2 = result[result.len() - 1].subjects.clone();
            if s1.contains(&s2[0]) {
                s2[1].clone()
            } else {
                s2[0].clone()
            }
        };
        let first = {
            let s1 = result[0].subjects.clone();
            let s2 = result[1].subjects.clone();
            if s2.contains(&s1[0]) {
                s1[1].clone()
            } else {
                s1[0].clone()
            }
        };
        result.push(terms.into_iter().find(|p| p.subjects.contains(&last) && p.subjects.contains(&first)).unwrap().clone());
        result
    } else {
        max.unwrap()
    }
}


pub fn run_e() {
    let input = File::open("inputs/task_13").unwrap();
    let input = BufReader::new(input);
    let mut input: Vec<ComputedTerms> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| l[..l.len() - 1].to_string())
        .map(|l| l.split(" ").map(|l| l.to_string()).collect())
        .map(|l: Vec<String>| {
            let m = if l[2] == "gain" { 1 } else { -1 };
            Terms { subject: l[0].clone(), score: l[3].parse::<i32>().unwrap() * m, target: l[10].clone() }
        })
        .fold(vec![], |acc, t| {
            let mut contains = false;
            let mut acc: Vec<ComputedTerms> = acc
                .into_iter()
                .map(|mut c| {
                    if c.subjects.contains(&t.subject) && c.subjects.contains(&t.target) {
                        c.score += t.score;
                        contains = true;
                    }
                    c
                })
                .collect();
            if !contains {
                acc.push(ComputedTerms { subjects: vec![t.subject, t.target], score: t.score });
            }
            acc
        });
    input.sort_by(|a, b| b.score.cmp(&a.score));

    let mut subjects = input.iter().fold(vec![], |mut acc, i| {
        if !acc.contains(&i.subjects[0]) {
            acc.push(i.subjects[0].clone());
        }
        if !acc.contains(&i.subjects[1]) {
            acc.push(i.subjects[1].clone());
        }
        acc
    });

    for s in &subjects {
        input.push(ComputedTerms { subjects: vec![s.clone(), "Me".to_string()], score: 0 });
    }

    subjects.push("Me".to_string());

    let first = input.first().unwrap().clone();
    let result = vec![first.clone()];

    let result = iterate(&input, result, subjects.into_iter().filter(|l| !first.subjects.contains(l)).collect());

    let result: i32 = result.iter().map(|c| c.score).sum();

    println!("{:?}", result);
}
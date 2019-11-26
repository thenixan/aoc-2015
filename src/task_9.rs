use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Route {
    from: String,
    to: String,
    distance: usize,
}

impl Route {
    fn new(from: String, to: String, distance: usize) -> Self { Route { from, to, distance } }
}

pub fn run() {
    let input = File::open("inputs/task_9").unwrap();
    let input = BufReader::new(input);
    let routes: Vec<Route> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            let route_and_distance: Vec<&str> = l.split(" = ").collect();
            let route_part: String = route_and_distance[0].to_string();
            let distance = route_and_distance[1].parse::<usize>().unwrap();
            let route: Vec<&str> = route_part.split(" to ").collect();
            Route::new(route[0].to_string(), route[1].to_string(), distance)
        })
        .collect();


    let counts = routes
        .iter()
        .fold(vec![], |mut acc, r| {
            if !acc.contains(&r.to) {
                acc.push(r.to.clone());
            }
            if !acc.contains(&r.from) {
                acc.push(r.from.clone());
            }
            acc
        });

    let mut result = 0;
    for i in 0..counts.len() {
        let from = &counts[i];
        let new_destinations: Vec<String> = counts.iter().filter(|r| r != &from).map(|s| s.clone()).collect();
        let found_length = find_route(&routes, &new_destinations, &from, true);
        if found_length.is_some() {
            let found_length = found_length.unwrap();
            if result == 0 || result > found_length {
                result = found_length;
            }
        }
    }
    println!("{:?}", result)
}

fn find_route(routes: &Vec<Route>, destinations: &Vec<String>, from: &String, shortest: bool) -> Option<usize> {
    let mut length = 0;
    if destinations.is_empty() {
        return Some(0);
    }
    for i in 0..destinations.len() {
        let to = &destinations[i];
        let route = &routes.into_iter().find(|r| {
            ((&r.from == from && &r.to == to) || (&r.from == to && &r.to == from)) && destinations.contains(to)
        });
        if route.is_some() {
            let route_length = route.unwrap().distance;
            let new_destinations: Vec<String> = destinations.iter().filter(|r| r != &to).map(|s| s.clone()).collect();
            let found_length = find_route(routes, &new_destinations, &to, shortest).map(|l| l + route_length);
            if found_length.is_some() {
                let found_length = found_length.unwrap();
                if shortest {
                    if length == 0 || length > found_length {
                        length = found_length;
                    }
                } else {
                    if length == 0 || length < found_length {
                        length = found_length;
                    }
                }
            }
        }
    }
    if length != 0 {
        return Some(length);
    } else {
        return None;
    }
}

pub fn run_e() {
    let input = File::open("inputs/task_9").unwrap();
    let input = BufReader::new(input);
    let routes: Vec<Route> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            let route_and_distance: Vec<&str> = l.split(" = ").collect();
            let route_part: String = route_and_distance[0].to_string();
            let distance = route_and_distance[1].parse::<usize>().unwrap();
            let route: Vec<&str> = route_part.split(" to ").collect();
            Route::new(route[0].to_string(), route[1].to_string(), distance)
        })
        .collect();


    let counts = routes
        .iter()
        .fold(vec![], |mut acc, r| {
            if !acc.contains(&r.to) {
                acc.push(r.to.clone());
            }
            if !acc.contains(&r.from) {
                acc.push(r.from.clone());
            }
            acc
        });

    let mut result = 0;
    for i in 0..counts.len() {
        let from = &counts[i];
        let new_destinations: Vec<String> = counts.iter().filter(|r| r != &from).map(|s| s.clone()).collect();
        let found_length = find_route(&routes, &new_destinations, &from, false);
        if found_length.is_some() {
            let found_length = found_length.unwrap();
            if result == 0 || result < found_length {
                result = found_length;
            }
        }
    }
    println!("{:?}", result)
}
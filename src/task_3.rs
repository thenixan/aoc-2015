use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn up(&mut self) {
        self.0 += 1;
    }
    fn down(&mut self) {
        self.0 -= 1;
    }
    fn left(&mut self) {
        self.1 -= 1;
    }
    fn right(&mut self) {
        self.1 += 1;
    }
}

struct Path {
    history: Vec<Point>
}

impl Path {
    fn new() -> Path { Path { history: vec![] } }

    fn put(&mut self, p: Point) {
        if !self.history.contains(&p) {
            self.history.push(p);
        }
    }

    fn size(&self) -> usize { self.history.len() }
}

pub fn run() {
    let input = File::open("inputs/task_3").unwrap();
    let input = BufReader::new(input);
    let path = &mut Path::new();

    let initial_point = &mut Point::new(0, 0);
    path.put(initial_point.clone());

    input.bytes().into_iter()
        .filter_map(|c| c.ok())
        .fold(initial_point, |acc, i| {
            match i as char {
                '>' => acc.right(),
                '<' => acc.left(),
                '^' => acc.up(),
                'v' => acc.down(),
                x => { panic!("Unknown char: {}", x) }
            };
            path.put(acc.clone());
            acc
        });

    println!("Result: {}", path.size());
}

pub fn run_e() {
    let input = File::open("inputs/task_3").unwrap();
    let input = BufReader::new(input);
    let path = &mut Path::new();

    let first_point = &mut Point::new(0, 0);
    let second_point = &mut Point::new(0, 0);

    path.put(first_point.clone());
    path.put(second_point.clone());

    input.bytes().into_iter()
        .filter_map(|c| c.ok())
        .fold((first_point, second_point), |acc, i| {
            match i as char {
                '>' => acc.0.right(),
                '<' => acc.0.left(),
                '^' => acc.0.up(),
                'v' => acc.0.down(),
                x => { panic!("Unknown char: {}", x) }
            };
            path.put(acc.0.clone());
            (acc.1, acc.0)
        });

    println!("Result: {}", path.size());
}
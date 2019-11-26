extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fs::File;
use std::io::{BufReader, BufRead};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn run() {
    let input = File::open("inputs/task_4").unwrap();
    let input = BufReader::new(input);

    let line = input.lines().next().unwrap().unwrap();

    let result = calculate(line, 5, num_cpus::get());

    println!("Result: {}", result);
}

pub fn calculate(s: String, count: usize, num_threads: usize) -> usize {
    let pool = ThreadPool::new(num_threads);

    let (tx, rx) = channel();

    let please_stop = Arc::new(AtomicBool::new(false));

    for j in 0..num_threads {
        let tx = tx.clone();
        let multiplier = j.clone();
        let pattern = s.clone();
        let should_i_stop = please_stop.clone();
        pool.execute(move || {
            let mut hasher = Md5::new();
            let mut output = [0; 16];
            let mut i = multiplier;
            loop {
                hasher.input(pattern.as_bytes());
                hasher.input(i.to_string().as_bytes());
                hasher.result(&mut output);
                if output.iter().flat_map(|o| vec![o >> 4, o << 4]).take(count).all(|o| o == 0u8) {
                    tx.send(i).unwrap();
                    break;
                }
                if should_i_stop.load(Ordering::SeqCst) {
                    break;
                }
                hasher.reset();
                i += num_threads;
            }
        })
    }

    let result = rx.recv().unwrap();
    please_stop.store(true, Ordering::SeqCst);
    pool.join();
    return result;
}

pub fn run_e() {
    let input = File::open("inputs/task_4").unwrap();
    let input = BufReader::new(input);

    let line = input.lines().next().unwrap().unwrap();

    let result = calculate(line, 6, num_cpus::get());

    println!("Result: {}", result);
}
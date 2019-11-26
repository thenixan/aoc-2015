use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

use regex::Regex;

pub fn run() {
    let input = File::open("inputs/task_12").unwrap();
    let input = BufReader::new(input);
    let input = input.lines().filter_map(|l| l.ok()).next().unwrap();
    let re = Regex::new(r#"(-?\d+)"#).unwrap();
    let sum: i32 = re.captures_iter(&input).map(|cap| cap[1].parse::<i32>()).filter_map(|r| r.ok()).sum();
    println!("Result: {}", sum);
}

pub fn run_e() {
    let input = File::open("inputs/task_12").unwrap();
    let input = BufReader::new(input);
    let mut iter = input.bytes().into_iter();

    let first_char = iter.next().unwrap().unwrap();

    let result = if first_char == '[' as u8 {
        read_array(&mut iter)
    } else {
        read_object(&mut iter)
    };

    println!("Result: {}", result);
}

fn read_array(iter: &mut dyn Iterator<Item=Result<u8, Error>>) -> i32 {
    let mut result = 0;

    loop {
        let r = iter.next().unwrap().unwrap();
        if r == ']' as u8 {
            return result;
        } else if r == '[' as u8 {
            result += read_array(iter);
        } else if r == '{' as u8 {
            result += read_object(iter);
        } else if r == '"' as u8 {
            loop {
                let inner = iter.next().unwrap().unwrap();
                if inner == '"' as u8 {
                    break;
                }
            }
        } else if r == ',' as u8 {
            // do nothing
        } else {
            let mut s = (r as char).to_string();
            loop {
                let inner = iter.next().unwrap().unwrap();
                if inner == ',' as u8 {
                    result += s.parse::<i32>().unwrap();
                    break;
                } else if inner == ']' as u8 {
                    result += s.parse::<i32>().unwrap();
                    return result;
                } else {
                    s = format!("{}{}", s, inner as char);
                }
            }
        }
    }
}

fn read_object(iter: &mut dyn Iterator<Item=Result<u8, Error>>) -> i32 {
    let mut result = 0;

    let mut has_red = false;

    loop {
        let r = iter.next().unwrap().unwrap();
        if r == '}' as u8 {
            return if has_red { 0 } else { result };
        } else if r == '[' as u8 {
            result += read_array(iter);
        } else if r == '{' as u8 {
            result += read_object(iter);
        } else if r == '"' as u8 {
            loop {
                let inner = iter.next().unwrap().unwrap();
                if inner == '"' as u8 {
                    break;
                }
            }
        } else if r == ',' as u8 {
            // do nothing
        } else if r == ':' as u8 {
            let n = iter.next().unwrap().unwrap();
            if n == '"' as u8 {
                let mut c1 = false;
                let mut c2 = false;
                let mut c3 = false;
                let mut count = 0;
                loop {
                    let inner = iter.next().unwrap().unwrap();
                    if inner == '"' as u8 {
                        break;
                    } else if count == 0 && inner == 'r' as u8 {
                        c1 = true;
                    } else if count == 1 && inner == 'e' as u8 {
                        c2 = true;
                    } else if count == 2 && inner == 'd' as u8 {
                        c3 = true;
                    }
                    count += 1;
                }
                if c1 && c2 && c3 && count == 3 {
                    has_red = true;
                }
            } else if n == '[' as u8 {
                result += read_array(iter);
            } else if n == '{' as u8 {
                result += read_object(iter);
            } else {
                let mut s = (n as char).to_string();
                loop {
                    let inner = iter.next().unwrap().unwrap();
                    if inner == ',' as u8 {
                        result += s.parse::<i32>().unwrap();
                        break;
                    } else if inner == '}' as u8 {
                        result += s.parse::<i32>().unwrap();
                        return if has_red { 0 } else { result };
                    } else {
                        s = format!("{}{}", s, inner as char);
                    }
                }
            }
        } else {
            println!("obj {}", r as char);
        }
    }
}
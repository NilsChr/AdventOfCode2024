use regex::Regex;

use crate::util::file_reader;
use crate::util::regex_helpers;
use std::fs::read_to_string;

pub fn start() {
    println!("Running day 3");
    let file_path: &str = "./input/day3/input.txt";

    let data = match file_reader::parse_file_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut task1 = 0;
    let mut task2 = 0;
    let regexv2 = r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)";
    let mut matches = regex_helpers::extract_all_matches(&regexv2, &data);
    let mut enabled = true;
    for m in matches.iter_mut() {
        if m == "do()" {
            enabled = true
        } else if m == "don't()" {
            enabled = false;
        } else {
            let re = Regex::new(r"\d+").unwrap();
            let numbers: Vec<i32> = re
                .find_iter(m)
                .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                .collect();
            let sum = numbers[0] * numbers[1];
            task1 += sum;
            if enabled == true {
                task2 += sum;
            }
        }
    }

    println!("Task1: {}", task1);
    println!("Task2: {}", task2);
}


pub fn start_GPT() {
    println!("Running day 3");
    let file_path = "./input/day3/input.txt";

    let data = match read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut task1 = 0;
    let mut task2 = 0;
    let mut enabled = true;

    let mut current_index = 0;
    let bytes = data.as_bytes();

    while current_index < bytes.len() {
        if bytes[current_index..].starts_with(b"do()") {
            enabled = true;
            current_index += 4; // Skip "do()"
        } else if bytes[current_index..].starts_with(b"don't()") {
            enabled = false;
            current_index += 7; // Skip "don't()"
        } else if let Some((a, b, new_index)) = parse_mul(&data, current_index) {
            let product = a * b;
            task1 += product;
            if enabled {
                task2 += product;
            }
            current_index = new_index;
        } else {
            current_index += 1; // Skip unrecognized characters
        }
    }

    println!("Task1: {}", task1);
    println!("Task2: {}", task2);
}

/// Parses a valid "mul(x,y)" construct starting from `index`
/// Returns the parsed numbers (x, y) and the new index after "mul(x,y)"
fn parse_mul(input: &str, index: usize) -> Option<(i32, i32, usize)> {
    if input[index..].starts_with("mul(") {
        let start = index + 4; // Skip "mul("
        if let Some(end) = input[start..].find(')') {
            let args = &input[start..start + end];
            let mut split = args.split(',');
            if let (Some(a), Some(b)) = (split.next(), split.next()) {
                if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                    return Some((a, b, start + end + 1));
                }
            }
        }
    }
    None
}
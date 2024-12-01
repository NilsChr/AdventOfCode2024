use crate::util::file_reader;
use std::collections::HashMap;

pub fn start() {
    println!("Running day 1");
    let file_path: &str = "./input/day1/input.txt";

    let parsed_rows = match file_reader::parse_file_to_rows(file_path) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for (_index, row) in parsed_rows.iter().enumerate() {
        let parts: Vec<&str> = row.split_whitespace().collect();
        if let Some(first) = parts.get(0) {
            if let Ok(num) = first.parse::<i32>() {
                array1.push(num);
            }
        }

        if let Some(second) = parts.get(1) {
            if let Ok(num) = second.parse::<i32>() {
                array2.push(num);
            }
        }
    }

    array1.sort();
    array2.sort();

    let mut task1 = 0;
    let mut task2 = 0;

    for i in 0..array1.len() {
        let dist = (array1[i] - array2[i]).abs();
        task1 += dist;

        let occurance = array2.iter().filter(|&n| *n == array1[i]).count() as i32;
        task2 += array1[i] * occurance;
    }

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}


/*
    Optimized with chatGPT
*/
pub fn start_improved() {
    println!("Running day 1");
    let file_path: &str = "./input/day1/input.txt";

    let parsed_rows = match file_reader::parse_file_to_rows(file_path) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for row in parsed_rows {
        let parts: Vec<&str> = row.split_whitespace().collect();
        if let Some(first) = parts.get(0).and_then(|x| x.parse::<i32>().ok()) {
            array1.push(first);
        }
        if let Some(second) = parts.get(1).and_then(|x| x.parse::<i32>().ok()) {
            array2.push(second);
        }
    }

    // Sort arrays
    array1.sort();
    array2.sort();

    // Precompute counts for array2
    let mut counts = HashMap::new();
    for &num in &array2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Compute task1 and task2
    let task1: i32 = array1
        .iter()
        .zip(&array2)
        .map(|(a, b)| (a - b).abs())
        .sum();

    let task2: i32 = array1
        .iter()
        .map(|a| a * counts.get(a).unwrap_or(&0))
        .sum();

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}


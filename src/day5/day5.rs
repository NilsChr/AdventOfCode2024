use std::collections::{HashMap, HashSet};

use crate::util::file_reader;

pub fn start() {
    println!("Running day 5");
    let file_path: &str = "./input/day5/input.txt";

    let lines = match file_reader::parse_file_to_rows(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut rules: Vec<[i32; 2]> = Vec::new();
    let mut page_numbers: Vec<Vec<i32>> = Vec::new();
    let mut build_rules = true;
    for line in lines {
        if line.is_empty() {
            build_rules = false;
            continue;
        }
        if build_rules == true {
            if let Some((key_str, value_str)) = line.split_once('|') {
                let key: i32 = key_str.parse().expect("Failed to parse key");
                let value: i32 = value_str.parse().expect("Failed to parse value");
                rules.push([key, value]);
            } else {
                println!("Input string is not in the correct format");
            }
        } else {
            let numbers: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse().expect("Failed to parse number"))
                .collect();

            page_numbers.push(numbers);
        }
    }

    let mut task1_score = 0;
    let mut task2_score = 0;

    for (_i, page) in page_numbers.iter_mut().enumerate() {
        if is_valid_page(&rules, &page) {
            task1_score += page[page.len() / 2];
        } else {
            loop {
                if reorder_page(&rules, page) == false {
                    break;
                }
            }
            task2_score += page[page.len() / 2];
        }
    }

    println!("Task1: {}", task1_score);
    println!("Task2: {}", task2_score);
}

fn is_valid_page_(rules: &Vec<[i32; 2]>, page: &Vec<i32>) -> bool {
    for (i, digit) in page.iter().enumerate() {
        let filtered_rules: Vec<&[i32; 2]> = rules
            .iter()
            .filter(|r| r[0] == *digit && page.contains(&r[1]))
            .collect();
        for rule in &filtered_rules {
            if let Some(index) = page.iter().position(|&x| x == rule[1]) {
                if index < i {
                    return false;
                }
            }
        }
    }
    true
}

fn is_valid_page(rules: &Vec<[i32; 2]>, page: &Vec<i32>) -> bool {
    let page_set: HashSet<i32> = page.iter().cloned().collect();
    for (i, digit) in page.iter().enumerate() {
        for rule in rules
            .iter()
            .filter(|r| r[0] == *digit && page_set.contains(&r[1]))
        {
            if let Some(index) = page.iter().position(|&x| x == rule[1]) {
                if index < i {
                    return false;
                }
            }
        }
    }
    true
}

fn reorder_page_(rules: &Vec<[i32; 2]>, page: &mut Vec<i32>) -> bool {
    let mut i = 0;
    let mut swapped = false;
    while i < page.len() {
        let digit = page[i];
        let filtered_rules: Vec<&[i32; 2]> = rules
            .iter()
            .filter(|r| r[0] == digit && page.contains(&r[1]))
            .collect();

        for rule in &filtered_rules {
            if let Some(index) = page
                .iter()
                .position(|&x| page[i] == rule[0] && x == rule[1])
            {
                if index < i {
                    page.swap(i, index);
                    swapped = true;
                }
            }
        }
        i += 1;
    }
    swapped
}

fn reorder_page(rules: &Vec<[i32; 2]>, page: &mut Vec<i32>) -> bool {
    let mut i = 0;
    let mut swapped = false;

    while i < page.len() {
        let digit = page[i];
        let filtered_rules: Vec<&[i32; 2]> = rules
            .iter()
            .filter(|r| r[0] == digit && page.contains(&r[1]))
            .collect();

        for rule in &filtered_rules {
            if let Some(index) = page
                .iter()
                .position(|&x| page[i] == rule[0] && x == rule[1])
            {
                if index < i {
                    page.swap(i, index);
                    swapped = true;
                }
            }
        }
        i += 1;
    }

    swapped
}

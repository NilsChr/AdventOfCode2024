use regex::Regex;

use crate::util::file_reader;
use crate::util::regex_helpers;
use std::fs::read_to_string;

pub fn start() {
    println!("Running day 4");
    let file_path: &str = "./input/day4/input.txt";

    let grid = match file_reader::parse_file_to_grid(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut task1_score = tast1(&grid);
    let mut task2 = 0;

    println!("Task1: {}", task1_score);
    println!("Task2: {}", task2);
}

fn tast1(grid: &Vec<Vec<char>>) -> i32 {

    println!("Checking coords 2,2");
    match get_char_at(grid, 2, 2) {
        Some(c) => println!("Char is {}", c),
        None => println!("Char not found at coords 2,2"),
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 'X' {
                println!("FOUND X");
            }
            //println!("Value at ({}, {}): {}", y, x, value);
        }
    }

    0
}

fn get_char_at(grid: &[Vec<char>], x: usize, y: usize) -> Option<char> {
    grid.get(y)?.get(x).cloned()
}

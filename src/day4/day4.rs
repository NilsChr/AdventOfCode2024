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

    let task1_score = task1(&grid);
    let task2_score = task2(&grid);

    println!("Task1: {}", task1_score);
    println!("Task2: {}", task2_score);
}

fn task1(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 'X' {
                count += search_word(grid, "XMAS", x, y, -1, -1);
                count += search_word(grid, "XMAS", x, y, 0, -1);
                count += search_word(grid, "XMAS", x, y, 1, -1);
                count += search_word(grid, "XMAS", x, y, -1, 0);
                count += search_word(grid, "XMAS", x, y, 1, 0);
                count += search_word(grid, "XMAS", x, y, -1, 1);
                count += search_word(grid, "XMAS", x, y, 0, 1);
                count += search_word(grid, "XMAS", x, y, 1, 1);
            }
        }
    }

    return count;
}

fn task2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if y == 0 || y == grid.len() || x == 0 || x == row.len() {
                continue;
            }
            if value == 'A' {
                let from_x = x - 1;
                let mut score_a = 0;
                score_a += search_word(grid, "MAS", from_x, y - 1, 1, 1);
                score_a += search_word(grid, "SAM", from_x, y - 1, 1, 1);

                let mut score_b = 0;
                score_b += search_word(grid, "MAS", from_x, y + 1, 1, -1);
                score_b += search_word(grid, "SAM", from_x, y + 1, 1, -1);

                if score_a > 0 && score_b > 0 {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn search_word(
    grid: &[Vec<char>],
    word: &str,
    from_x: usize,
    from_y: usize,
    dir_x: isize,
    dir_y: isize,
) -> i32 {
    let word_chars = word.as_bytes();
    let mut x = from_x as isize;
    let mut y = from_y as isize;

    for &letter in word_chars {
        if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[y as usize].len() {
            return 0;
        }

        if grid[y as usize][x as usize] as u8 != letter {
            return 0;
        }

        x += dir_x;
        y += dir_y;
    }
    1
}

fn search_word_(
    grid: &[Vec<char>],
    word: &str,
    from_x: usize,
    from_y: usize,
    dir_x: isize,
    dir_y: isize,
) -> i32 {
    let word_chars: Vec<char> = word.chars().collect();

    for (i, &letter) in word_chars.iter().enumerate() {
        let x = from_x as isize + i as isize * dir_x;
        let y = from_y as isize + i as isize * dir_y;

        if x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[y as usize].len() as isize {
            return 0;
        }

        if grid[y as usize][x as usize] != letter {
            return 0;
        }
    }
    1
}
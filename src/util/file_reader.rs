use std::fs::{self, File};
use std::io::{self, BufRead};

pub fn parse_file_to_string(file_path: &str) -> io::Result<String> {
    return fs::read_to_string(file_path)
}

pub fn parse_file_to_rows(file_path: &str) -> io::Result<Vec<String>> {
    let mut rows = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        rows.push(line);
    }

    Ok(rows)
}

pub fn parse_file_to_ints(file_path: &str) -> io::Result<Vec<i32>> {
    let mut numbers = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        match line.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse a line as i32")),
        }
    }

    Ok(numbers)
}

pub fn parse_file_to_grid(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect::<Result<_, _>>()?;
    Ok(grid)
}
use crate::util::file_reader;

pub fn start() {
    println!("Running day 2");
    let file_path: &str = "./input/day2/input.txt";

    let parsed_rows = match file_reader::parse_file_to_rows(file_path) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut score1: i32 = 0;
    let mut score2: i32 = 0;

    for row in parsed_rows {
        let parts: Vec<i32> = row
            .split_whitespace()
            .filter_map(|i| i.parse::<i32>().ok())
            .collect();

        let score = process_row(&parts);
        score1 += score;
        score2 += score;

        if score == 0 {
            for (index, _) in parts.iter().enumerate() {
                let filtered: Vec<_> = parts
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != index)
                    .map(|(_, &v)| v)
                    .collect();
                let score = process_row(&filtered);

                if score == 1 {
                    score2 += 1;
                    break;
                }
            }
        }
    }

    println!("Task1: {}", score1);
    println!("Task2: {}", score2);
}

fn process_row(parts: &[i32]) -> i32 {
    let increase = parts[1] >= parts[0];

    for window in parts.windows(2) {
        let a = window[0];
        let b = window[1];
        let dist = (a - b).abs();

        let out_of_bounds = dist < 1 || dist > 3;
        let wrong_order = (increase && a > b) || (!increase && b > a);

        if out_of_bounds || wrong_order || a == b {
            return 0;
        }
    }
    return 1;
}

pub fn start_gpt() {
    println!("Running day 2 - GPT");
    let file_path: &str = "./input/day2/input.txt";

    let parsed_rows = match file_reader::parse_file_to_rows(file_path) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut score1: i32 = 0;
    let mut score2: i32 = 0;

    for row in parsed_rows {
        let parts: Vec<i32> = row
            .split_whitespace()
            .filter_map(|i| i.parse::<i32>().ok())
            .collect();

        let score = process_row_gpt(&parts);
        score1 += score;
        score2 += score;

        if score == 0 {
            for index in 0..parts.len() {
                let filtered = &parts[0..index]
                    .iter()
                    .chain(&parts[index + 1..])
                    .copied()
                    .collect::<Vec<_>>();
                if process_row_gpt(filtered) == 1 {
                    score2 += 1;
                    break;
                }
            }
        }
    }

    println!("Task1: {}", score1);
    println!("Task2: {}", score2);
}

fn process_row_gpt(parts: &[i32]) -> i32 {
    if parts.len() < 2 {
        return 1; // A single-element row is trivially valid.
    }

    let mut iter = parts.windows(2);
    let increase = match iter.next() {
        Some(&[a, b]) => b > a,
        _ => true, // Default to increasing if no valid window exists.
    };

    for window in parts.windows(2) {
        let [a, b] = [window[0], window[1]];
        if (b - a).abs() < 1 || (b - a).abs() > 3 || a == b {
            return 0;
        }

        if (increase && a > b) || (!increase && b > a) {
            return 0;
        }
    }
    1
}

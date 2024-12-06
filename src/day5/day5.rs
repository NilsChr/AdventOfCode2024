use std::collections::HashMap;

use crate::util::file_reader;


pub fn start() {
    println!("Running day 5");
    let file_path: &str = "./input/day5/test.txt";

    let lines = match file_reader::parse_file_to_rows(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    //let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut map: Vec<[i32; 2]> = Vec::new();
    let mut page_numbers: Vec<Vec<i32>> = Vec::new();
    let mut build_map = true;
    for line in lines {
        if line.is_empty() {
            build_map = false;
            continue;
        }
        if build_map == true {
            if let Some((key_str, value_str)) = line.split_once('|') {
                // Parse key and value
                let key: i32 = key_str.parse().expect("Failed to parse key");
                let value: i32 = value_str.parse().expect("Failed to parse value");

                // Ensure the key exists in the map, or insert an empty Vec
                //map.entry(key).or_insert_with(Vec::new).push(value);
                map.push([key, value]);
            } else {
                println!("Input string is not in the correct format");
            }
        } else {
            // Parse comma-separated values into integers
            let numbers: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse().expect("Failed to parse number"))
                .collect();

            // Push into pageNumbers
            page_numbers.push(numbers);
        }

    }

    /* 
    for (key, values) in &map {
        let values_str = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
        println!("Key {}: {}", key, values_str);
    }
    */
    for pair in &map {
        println!("Key {}: {}", pair[0], pair[1]);
    }

    // Print the page numbers size and contents
    for (i, numbers) in page_numbers.iter().enumerate() {
        let numbers_str = numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
        println!("Page {}: {}", i + 1, numbers_str);
    }

    for (i, page) in page_numbers.iter().enumerate() {
        let score = validateLines(&map, &page_numbers);
    }


    let task1_score = 0;//task1(&grid);
    let task2_score = 0;//task2(&grid);

    println!("Task1: {}", task1_score);
    println!("Task2: {}", task2_score);
}

fn validateLines(map: &Vec<[i32; 2]>, pages: &Vec<Vec<i32>> ) -> i32 {
    let mut out = 0;

    for (i, page) in pages.iter().enumerate() {

        for (j, p) in page.iter().enumerate() {
            if i == j {
                break
            }

            if let Some(pair) = map.iter().find(|&m| m[0] as usize == i && m[1] as usize == j) {
                println!("Found pair: {:?} for i: {}, j: {}", pair, i, j);

                // Example logic to use the pair
                out += 1; // Increment `out` as an example
            }        
        }
    }

    out
}
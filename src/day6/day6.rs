use crate::util::file_reader;
use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn start() {
    println!("Running day 6");
    let file_path: &str = "./input/day6/input.txt";

    let mut grid = match file_reader::parse_file_to_grid_fast(file_path, 130, 130) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    
    let start = find_start(&grid);
    println!("Start {},{}", start.0, start.1);

    
    
    let scores = process(&mut grid, &start, false);
    // print_grid(&grid);

    println!("Task1: {}", scores.0);
    println!("Task2: {}", scores.1);
    

    //let is_loop = check_loop(&mut grid, &start, [0,-1]);
    //println!("LOOP {}", is_loop);
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == '^' {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    //println!();

    thread::sleep(Duration::from_millis(50));

    print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear the screen
    io::stdout().flush().unwrap();
    for (_y, row) in grid.iter().enumerate() {
        for (_x, &value) in row.iter().enumerate() {
            print!("{}", value);
        }
        println!();
    }
}

fn process(grid: &mut Vec<Vec<char>>, start: &(usize, usize), skip_2: bool) -> (usize, usize) {
    let mut dir: [isize; 2] = [0, -1];
    let mut pos = (start.0 as isize, start.1 as isize);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut loops = 0;

    loop {
        if pos.0 < 0 || pos.0 >= grid[0].len() as isize || pos.1 < 0 || pos.1 >= grid.len() as isize
        {
            break;
        }

        let next_pos = (pos.0 + dir[0], pos.1 + dir[1]);
        if visited.contains(&(pos.0 as usize, pos.1 as usize)) == false && skip_2 == false  {
            let mut grid2 = grid.clone();
            grid2[pos.1 as usize][pos.0 as usize] = '#';
            // println!("CHECKING MAP!");
            if check_loop(&mut grid2, &(start.0 as usize, start.1 as usize), [0, -1]) {
                loops += 1;
               // println!("FOUND A LOOP!");
                // thread::sleep(Duration::from_secs(5));
            }

        }

        // grid[pos.1 as usize][pos.0 as usize] = 'X';
        visited.insert((pos.0 as usize, pos.1 as usize));

        // println!("DONE");
        // print_grid(grid);
        if next_pos.0 < 0
            || next_pos.0 >= grid[0].len() as isize
            || next_pos.1 < 0
            || next_pos.1 >= grid.len() as isize
        {
            break;
        }

        if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = [-dir[1], dir[0]];
        }
        pos = (pos.0 + dir[0], pos.1 + dir[1]);
    }

    (visited.len(),loops)
}

fn check_loop(grid: &mut Vec<Vec<char>>, start: &(usize, usize), dir: [isize; 2]) -> bool {
    let mut dir = [dir[0], dir[1]];
    let mut pos = (start.0 as isize, start.1 as isize);
    let mut visited: HashSet<(usize, usize, [isize; 2])> = HashSet::new();
    let mut iteration = 0;
    loop {
        if pos.0 < 0 || pos.0 >= grid[0].len() as isize || pos.1 < 0 || pos.1 >= grid.len() as isize {
            return false;
        }

        if visited.contains(&(pos.0 as usize, pos.1 as usize, dir)) {
            return true;
        }

        grid[pos.1 as usize][pos.0 as usize] = 'o';
        if iteration > 0 {
            visited.insert((pos.0 as usize, pos.1 as usize, dir));
        }
        iteration += 1;

        let mut next_pos = (pos.0 + dir[0], pos.1 + dir[1]);

        if next_pos.0 < 0
            || next_pos.0 >= grid[0].len() as isize
            || next_pos.1 < 0
            || next_pos.1 >= grid.len() as isize
        {
            return false;
        }

        let mut col = false;
        if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = [-dir[1], dir[0]]; 
            next_pos = (pos.0 + dir[0], pos.1 + dir[1]);

            if next_pos.0 < 0
                || next_pos.0 >= grid[0].len() as isize
                || next_pos.1 < 0
                || next_pos.1 >= grid.len() as isize
                || grid[next_pos.1 as usize][next_pos.0 as usize] == '#'
            {
                col = true;
            }
        }

        if col == false {

            pos = next_pos;
        }
        //print_grid(grid);
    }
}

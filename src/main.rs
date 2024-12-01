#![allow(dead_code)]

mod day1;
mod util {
    pub mod file_reader; // Declare the fileReader module
}

use std::time::Instant;


fn main() {
    let start_time = Instant::now(); // Start the timer


    day1::start();

    // Measure and display execution time
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:.2?}", elapsed_time);
}

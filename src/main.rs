#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod util {
    pub mod file_reader; 
}

use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    day2::start();
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:.2?}", elapsed_time);
}

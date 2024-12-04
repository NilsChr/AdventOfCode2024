#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod day3;
mod day4;
pub mod util;

use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    day4::start();
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:.2?}", elapsed_time);
}

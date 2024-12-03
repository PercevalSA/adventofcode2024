// #![feature(test)]

// include function one from one file and call it in main function
use adventofcode2024::{day_1, day_2, day_3};

fn main() {
    println!("Starting advent of code");
    println!("day one");
    day_1::solve_one();
    day_1::solve_two();

    println!("day two");
    day_2::solve_one();
    day_2::solve_two();

    println!("day three");
    day_3::solve_one();
}

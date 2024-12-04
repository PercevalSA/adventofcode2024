// #![feature(test)]

// include function one from one file and call it in main function
use adventofcode2024::{day1, day2, day3};

fn main() {
    println!("Starting advent of code");
    println!("day one");
    day1::part1();
    day1::part2();

    println!("day two");
    day2::solve_part1();
    day2::solve_part1();

    println!("day three");
    day3::solve_part1();
    day3::solve_part2();
}

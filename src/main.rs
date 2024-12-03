// include function one from one file and call it in main function
mod day_1;
mod day_2;

fn main() {
    println!("Starting advent of code");
    day_1::solve_one();
    day_1::solve_two();

    day_2::solve_one();
    day_2::solve_two();
}

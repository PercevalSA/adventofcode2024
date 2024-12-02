// include function one from one file and call it in main function
mod one;

fn main() {
    println!("Starting advent of code");
    one::solve_one();
    one::solve_two();
}

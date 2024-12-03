// include function one from one file and call it in main function
mod one;
mod two;

fn main() {
    println!("Starting advent of code");
    one::solve_one();
    one::solve_two();

    two::solve_one();
    two::solve_two();
}

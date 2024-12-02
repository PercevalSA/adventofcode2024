// include function one from one file and call it in main function
mod one;

fn main() {
    println!("Hello, world!");
    one::solve();
}

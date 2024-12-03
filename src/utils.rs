use std::fs::File;
use std::io::Read;

pub fn read_file(filename: &str) -> String {
    let file_path = format!("/workspaces/adventofcode2024/resources/{}", filename);
    println!("reading file {}", file_path);
    let mut file = File::open(file_path).expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    content
}

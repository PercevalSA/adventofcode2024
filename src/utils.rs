use std::fs;

pub fn read_file(filename: &str) -> String {
    let path = format!("./resources/{}.txt", filename);
    fs::read_to_string(path).unwrap()
}

pub fn get_lines(content: &str) -> Vec<String> {
    content.split("\n").map(|s| s.to_string()).collect()
}

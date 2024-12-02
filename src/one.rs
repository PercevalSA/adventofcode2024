pub fn solve() {
    println!("Starting advent of code");
    let input_file = "one.txt";

    // open the file and get the content
    let content: String =
        std::fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let mut col_a: Vec<u32> = vec![];
    let mut col_b: Vec<u32> = vec![];

    for line in content.lines() {
        let mut iterator = line.split_whitespace();
        let first_number: u32 = iterator
            .next()
            .expect("first number not found")
            .parse()
            .unwrap();
        let second_number: u32 = iterator
            .next()
            .expect("second number not found")
            .parse()
            .unwrap();

        col_a.push(first_number);
        col_b.push(second_number);
    }

    col_a.sort();
    col_b.sort();

    let mut b_iter = col_b.iter();
    let all_distances: Vec<u32> = col_a
        .iter()
        .map(|x| x.abs_diff(*b_iter.next().expect("prout")))
        .collect();
    let total_distance: u32 = all_distances.iter().sum();
    println!("The sum of the elements is {}.", total_distance);
}

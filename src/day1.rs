fn get_data() -> (Vec<u32>, Vec<u32>) {
    let file_name: &str = "resources/1.txt";

    // open the file and get the content
    let content: String =
        std::fs::read_to_string(file_name).expect("Something went wrong reading the file");

    println!("reading file {}", file_name);

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
    (col_a, col_b)
}

pub fn part1() {
    let mut col_a: Vec<u32>;
    let mut col_b: Vec<u32>;

    (col_a, col_b) = get_data();

    col_a.sort();
    col_b.sort();

    let mut b_iter = col_b.iter();
    let all_distances: Vec<u32> = col_a
        .iter()
        .map(|x| x.abs_diff(*b_iter.next().expect("prout")))
        .collect();
    let total_distance: u32 = all_distances.iter().sum();
    println!("The sum of the elements is {}", total_distance);
}

pub fn part2() {
    let col_a: Vec<u32>;
    let col_b: Vec<u32>;
    (col_a, col_b) = get_data();

    let mut similarity_score: u32 = 0;

    for element in col_a {
        let occurence: usize = col_b.iter().filter(|&n| *n == element).count();
        similarity_score += element * occurence as u32;
    }

    println!("The similarity score is {}", similarity_score)
}

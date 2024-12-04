#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<&str> {
    // generates all lines present in data so in gain time in search
    // transform everything as byte to be faster?
    let lines: Vec<&str> = input.split("\n").collect();
    let line_size = lines[0].len();
    let input_size = input.len();
    let mut cols: String = String::from("");

    for i in 0..line_size {
        let mut new_col: String = "";
        for line in lines {
            new_col.extend(line);
        }
        cols.push_str(line[i]);

    }

    // println!(lines);
    // println!(cols);

    let all_data: Vec<&str> = vec![];
    all_data.extend_from_slice(&lines);
    all_data.extend_from_slice(&cols);

    all_data
}

#[aoc(day4, part1)]
pub fn part1(data: &Vec<$str>) -> u32 {
    println!("{}", data);
    0
}

// #[aoc(day4, part2)]
// pub fn part2(data: &str) -> u32 {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_file;

    #[test]
    fn test_part1() {
        let input = parse_input(read_file("day4_example").as_str());
        let result: u32 = 18;

        assert_eq!(part1(input), result);
    }

    // #[test]
    // fn test_part2() {}
}

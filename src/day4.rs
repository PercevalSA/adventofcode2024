use std::collections::HashMap;
use std::io::BufRead;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<String> {
    // generates all lines present in data so in gain time in search
    // transform everything as byte to be faster

    let input_bytes: &[u8] = input.as_bytes();
    let mut lines: Vec<String> = input_bytes.lines().collect::<Result<_, _>>().unwrap();

    let line_size = input_bytes.lines().next().expect("plop").unwrap().len();
    let nb_lines = input_bytes.lines().count();
    // we need that again

    let nb_diags = nb_lines + line_size;
    let mut cols: Vec<String> = vec![];
    let mut diags: HashMap<usize, String> = HashMap::new();
    let mut back_diags: HashMap<usize, String> = HashMap::new();

    for i in 0..line_size {
        let mut new_col: String = String::from("");

        // must be improved
        let lines_bytes = input_bytes.lines();

        for iter in lines_bytes.enumerate() {
            let (j, line) = iter;
            let new_char = line.unwrap().as_bytes()[i] as char;

            new_col.push(new_char as char);

            let id: usize = i + j;
            if !diags.contains_key(&id) {
                diags.insert(id, String::from(new_char));
            } else {
                diags.get_mut(&id).expect("not in hashmap").push(new_char);
            }

            let bid: usize = nb_diags - j + i ;
            if !back_diags.contains_key(&bid) {
                back_diags.insert(bid, String::from(new_char));
            } else {
                back_diags
                    .get_mut(&bid)
                    .expect("not in hashmap")
                    .push(new_char);
            }
        }
        cols.push(new_col);
    }

    let mut all_data: Vec<String> = vec![];
    let mut diagonals = diags.values().cloned().collect::<Vec<String>>();
    let mut back_diagonals = back_diags.values().cloned().collect::<Vec<String>>();

    all_data.append(&mut lines);
    all_data.append(&mut cols);
    all_data.append(&mut diagonals);
    all_data.append(&mut back_diagonals);

    all_data
}

#[aoc(day4, part1)]
pub fn part1(data: &Vec<String>) -> usize {
    let mut result: usize = 0;
    for line in data {
        result += line.match_indices("XMAS").count();
        result += line.match_indices("SAMX").count();
    }
    result
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
        let result: usize = 18;
        assert_eq!(part1(&input), result);
    }

    // #[test]
    // fn test_part2() {}
}

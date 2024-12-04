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

    // let num_string = num.to_string();
    // let b: u8  = num_string.as_bytes()[i];
    // let c: char = b as char;  // if you need to get the character as a unicode code point

    // If you do need to index code points, you have to use the chars() iterator:
    // num_string.chars().nth(i).unwrap()

    // we want to generate diag at the same time
    // there are nblines+ nbcol number of diags
    // a diag is indexed as nblines+nbcols-iterline-itercol
    // that mind need -1 adjustment because of index

    for i in 0..line_size {
        let mut new_col: String = String::from("");

        // must be improved
        let lines_bytes = input_bytes.lines();

        for iter in lines_bytes.enumerate() {
            let (j, line) = iter;
            let new_char = line.unwrap().as_bytes()[i] as char;

            new_col.push(new_char as char);

            let id = nb_diags - i - j;
            if !diags.contains_key(&id) {
                diags.insert(id, String::from(new_char));
            } else {
                diags.get_mut(&id).expect("not in hashmap").push(new_char);
            }
        }
        cols.push(new_col);
    }

    let mut all_data: Vec<String> = vec![];
    let mut diagonals = diags.values().cloned().collect::<Vec<String>>();

    all_data.append(&mut lines);
    all_data.append(&mut cols);
    all_data.append(&mut diagonals);

    all_data
}

#[aoc(day4, part1)]
pub fn part1(data: &Vec<String>) -> usize {
    let mut result: usize = 0;
    println!("{:?}", data);
    for line in data {
        let test = line.match_indices("XMAS").count();
        let tist = line.match_indices("SAMX").count();
        println!("found XMAS {} times and found SAMX {} times", test, tist);
        result += test;
        result += tist;
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

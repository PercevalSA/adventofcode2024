#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<&str> {
    // generates all lines present in data so in gain time in search
    // transform everything as byte to be faster?
    let lines: Vec<&str> = input.split("\n").collect();
    let line_size = lines[0].len();
    let nb_lines = lines.len();
    let input_size = input.len();
    let nb_diags = nb_lines + line_size;
    let mut cols: String = String::from("");
    let mut diags: [Vec<char>; nb_diags] = [];
    
    // let num_string = num.to_string();
    // let b: u8 = num_string.as_bytes()[i];
    // let c: char = b as char;  // if you need to get the character as a unicode code point

    // If you do need to index code points, you have to use the chars() iterator:
    // num_string.chars().nth(i).unwrap()
    
    // we want to generate diag at the same time
    // there are nblines+ nbcol number of diags
    // a diag is indexed as nblines+nbcols-iterline-itercol 
    // that mind need -1 adjustment because of index
    for i in 0..line_size {
        let mut new_col: String = "";
        for (j, line) in enumerate(lines) {
            new_col.extend(line[i]);
            diags[nb_diags - i - j].push(line[i];
        }
        cols.push_str(line[i]);

    }

    // println!(lines);
    // println!(cols);

    let all_data: Vec<&str> = vec![];
    all_data.extend_from_slice(&lines);
    all_data.extend_from_slice(&cols);
    all_data.extend_from_slice(&diags);

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

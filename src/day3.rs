use crate::utils::read_file;
use std::fmt::Display;

fn _solve_part_1(data: &str) -> u32 {
    // search for something like mul(u32, u32)
    let mut result: Vec<u32> = vec![];

    let potential_muls = data.split("mul(");

    for mul in potential_muls {
        let mut args = mul.split(")");
        // did not split so no ) in that part, so not valid
        if args.clone().count() == 1 {
            continue;
        }

        let numbers = args.next().unwrap();
        let mut num_str = numbers.split(",");

        let count = num_str.clone().count();
        if count < 2 || count > 2 {
            continue;
        }

        let n = num_str.next().unwrap().parse::<u32>();

        let num1 = match n {
            Ok(num) => num,
            Err(_error) => continue,
        };

        let m = num_str.next().unwrap().parse::<u32>();
        let num2 = match m {
            Ok(num) => num,
            Err(_error) => continue,
        };

        result.push(num1 * num2);
    }

    let sum: u32 = result.iter().sum();
    sum
}

pub fn part1(data: &str) -> impl Display {
    _solve_part_1(data)
}

pub fn solve_part_one() {
    let data_string: String = read_file("3.txt");
    let data: &str = data_string.as_str();
    println!("Amount given by valid operations: {}", part1(data));
}
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn search_valid_mul() {
        let data_str: String = read_file("3_example.txt");
        let data: &str = data_str.as_str();
        let result = _solve_part_1(data);
        assert_eq!(result, 161);
    }
}

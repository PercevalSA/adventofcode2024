use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(data: &str) -> u32 {
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

#[aoc(day3, part2)]
pub fn part2(data: &str) -> u32 {
    let mut final_list: String = String::from("");

    let do_mul_s = data.split("do()");
    for do_mul in do_mul_s {
        // get the first item after the split of any "don't"
        // that drops everything else
        final_list.push_str(do_mul.split("don't()").next().expect("plop"));
    }

    part1(final_list.as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_file;

    #[test]
    fn search_valid_mul() {
        let data_str: String = read_file("day3_example");
        let data: &str = data_str.as_str();
        let result = part1(data);
        assert_eq!(result, 161);
    }

    #[test]
    fn search_do_mul() {
        let data_str: String = read_file("day3_example_2");
        let data: &str = data_str.as_str();
        let result = part2(data);
        assert_eq!(result, 48);
    }
}

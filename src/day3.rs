use crate::utils::read_file;

fn search_valid_mul(data: String) -> Vec<u32> {
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
    // println!("Result: {:?}", result);
    result
}

pub fn part1() {
    let valids_operations = search_valid_mul(read_file("3.txt"));

    let sum: u32 = valids_operations.iter().sum();
    println!("Amount given by valid operations: {:?}", sum);
}

#[cfg(test)]

mod test {
    #[test]
    fn search_valid_mul() {
        println!("test not implemented")
    }
}

#[path ="utils.rs"]
mod utils;

// fn search_valid_mul(data: String) -> Vec<u32> {
//     // search for something like mul(u32, u32)
//     let result: Vec<u32> = vec![];

//     let splitted_data = data.split("mul(");
    
//     for mul in splitted_data {
//         let args = mul.split(")");
//         if args.count() == 1 {
//             continue;
//         }
//         let numbers = args.next().unwrap(); 
//         let (num1s, num2s): (&str, &str) = numbers.split(",");

//         let num1: u32 = num1s.parse::<u32>().expect("prout");
//         let num2: u32 = num2s.parse::<u32>().expect("plop");

//         result.push(num1 * num2);
//     }
//     result
// }

pub fn solve_one() {
    // let valids_operations = search_valid_mul(utils::read_file("3.txt"));

    println!("Solution: NONE");
}




#[cfg(test)]

mod test {
    #[test]
    fn search_valid_mul() {
        println!("test not implemented")
    }
    
}
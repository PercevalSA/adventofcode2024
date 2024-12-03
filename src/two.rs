use std::fs::File;
use std::io::Read;

fn get_data(filename: &str) -> String {
    let mut file = File::open(filename).expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    content
}

fn parse_data(content: String) -> Vec<Vec<u8>> {
    // each line is a report
    // each number in line is a level
    let mut data: Vec<Vec<u8>> = vec![];

    let lines = content.split('\n');

    for line in lines {
        let temp = line
            .split_whitespace()
            .map(|i| i.parse::<u8>().unwrap())
            .collect();

        data.push(temp);
    }

    data
}

fn is_valid_increase(report: &Vec<u8>) -> bool {
    let iter_window = report.windows(2);

    // increase
    for window in iter_window {
        if window[0] < window[1] && window[0].abs_diff(window[1]) < 4 {
        } else {
            return false;
        }
    }
    true
}

fn is_valid_decrease(report: &Vec<u8>) -> bool {
    let iter_window = report.windows(2);

    // increase
    for window in iter_window {
        if window[0] > window[1] && window[0].abs_diff(window[1]) < 4 {
        } else {
            return false;
        }
    }
    true
}

fn is_valid(report: &Vec<u8>) -> bool {
    println!("testing report {:?}", report);
    is_valid_increase(&report) || is_valid_decrease(&report)
}

fn count_valid_reports(data: Vec<Vec<u8>>) -> u32 {
    let mut valid_reports_amount: u32 = 0;

    for report in data {
        if is_valid(&report) {
            valid_reports_amount += 1;
        }
    }

    valid_reports_amount
}

pub fn solve_one() {
    // level should increase only or decrease only
    // difference between consecutives levels should be between 1 and 3
    let data: Vec<Vec<u8>> = parse_data(get_data("two.txt"));

    println!("Amount of valid reports {}", count_valid_reports(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let data = parse_data(get_data("/workspaces/adventofcode2024/src/two_example.txt"));
        println!("Les données de test sont {:?}", data);

        assert!(is_valid_decrease(&data[0]));
        assert!(!is_valid_increase(&data[0]));

        assert!(!is_valid_decrease(&data[1]));
        assert!(!is_valid_increase(&data[1]));

        assert!(!is_valid_decrease(&data[2]));
        assert!(!is_valid_increase(&data[2]));

        assert!(!is_valid_decrease(&data[3]));
        assert!(!is_valid_increase(&data[3]));

        assert!(!is_valid_decrease(&data[4]));
        assert!(!is_valid_increase(&data[4]));

        assert!(!is_valid_decrease(&data[5]));
        assert!(is_valid_increase(&data[5]));

        assert!(is_valid(&data[0]));
        assert!(is_valid(&data[5]));

        assert!(!is_valid(&data[1]));
        assert!(!is_valid(&data[2]));
        assert!(!is_valid(&data[3]));
        assert!(!is_valid(&data[4]));
    }

    #[test]
    fn test_count() {
        let data = parse_data(get_data("/workspaces/adventofcode2024/src/two_example.txt"));

        assert_eq!(count_valid_reports(data), 2);
    }
}
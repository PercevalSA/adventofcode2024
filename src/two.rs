use std::fs::File;
use std::io::Read;

fn get_data() -> String {
    let mut file = File::open("two.txt").expect("can't read file");
    let mut content = String::new();
    file.read_to_string(&mut content);

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

fn is_report_valid(report: Vec<u8>) -> bool {
    let mut valid: bool = false;

    if report.0 < report.1 {
        // increasing
        let iter_window = report.windows(2);
        for window in iter_window {
            if window.0 < window.1 && window.0.diff(window.1) < 4 {
                valid = true;
            }
        }
    }
    valid
}

pub fn solve_one() {
    // level should increase only or decrease only
    // difference between consecutives levels should be between 1 and 3
    let data: Vec<Vec<u8>> = parse_data(get_data());

    let mut valid_reports_amount: u32 = 0;
    for report in data {
        if is_report_valid(report) {
            valid_reports_amount += 1;
        }
    }

    println!("Amount of valid reports {}", valid_reports_amount);
}

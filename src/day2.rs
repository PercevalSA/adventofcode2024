type Report = Vec<u8>;

#[aoc_generator(day2)]
pub fn parse_data(input: &str) -> Vec<Report> {
    // each line is a report
    // each number in line is a level
    let mut data: Vec<Report> = vec![];

    let lines = input.split('\n');

    for line in lines {
        let temp = line
            .split_whitespace()
            .map(|i| i.parse::<u8>().unwrap())
            .collect();

        data.push(temp);
    }

    data
}

fn is_valid_increase(report: &Report) -> bool {
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

fn is_valid_decrease(report: &Report) -> bool {
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

fn count_valid_reports(data: &Vec<Report>) -> u32 {
    let mut valid_reports_amount: u32 = 0;

    // level should increase only or decrease only
    // difference between consecutives levels should be between 1 and 3
    for report in data {
        if is_valid_increase(&report) || is_valid_decrease(&report) {
            valid_reports_amount += 1;
        }
    }

    valid_reports_amount
}

#[aoc(day2, part1)]
pub fn part1(reports: &Vec<Report>) -> u32 {
    count_valid_reports(reports)
}

fn is_valid_with_removal(report: &Report) -> bool {
    for permutation in 0..report.len() {
        let mut temp_vec = report.clone();
        temp_vec.remove(permutation);

        if is_valid_increase(&temp_vec) || is_valid_decrease(&temp_vec) {
            return true;
        }
    }
    false
}

fn count_valid_reports_with_permutations(reports: &Vec<Report>) -> u32 {
    let mut valid_reports_amount: u32 = 0;

    for report in reports {
        if is_valid_increase(&report)
            || is_valid_decrease(&report)
            || is_valid_with_removal(&report)
        {
            valid_reports_amount += 1;
        }
    }

    valid_reports_amount
}

#[aoc(day2, part2)]
pub fn part2(reports: &Vec<Report>) -> u32 {
    count_valid_reports_with_permutations(reports)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_file;
    #[test]
    fn test_is_valid() {
        let data = parse_data(read_file("day2_example").as_str());
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
    }

    #[test]
    fn test_count() {
        let data = parse_data(read_file("day2_example").as_str());

        assert_eq!(count_valid_reports(&data), 2);
    }

    #[test]
    fn test_removal() {
        let data = parse_data(read_file("day2_example").as_str());
        assert!(is_valid_with_removal(&data[3]));
        assert!(is_valid_with_removal(&data[4]));

        assert_eq!(count_valid_reports_with_permutations(&data), 4);
    }
}

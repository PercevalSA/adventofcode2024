use crate::utils::read_file;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    // split on double line break: then split on pipe or split on comma
    let mut splitted_input = input.split("\n\n");
    let rules = splitted_input.next().expect("rules").lines();
    let updates = splitted_input.next().expect("pages number").lines();

    // rules can be view as an oriented graph
    // then the page are the path and we need to know if the path exist or not
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    for rule in rules {
        let mut nodes = rule.split("|");
        let from: u8 = nodes.next().unwrap().parse().unwrap();
        let to: u8 = nodes.next().unwrap().parse().unwrap();

        if graph.contains_key(&from) {
            graph.get_mut(&from).expect("from mut").push(to);
        } else {
            graph.insert(from, vec![to]);
        }
    }

    println!("This is my graph {:?}", graph);

    // let pages: Vec<Vec<u8>> = updates.iter().split(",").collect();
    let mut pages: Vec<Vec<u8>> = vec![];
    for update in updates {
        let pages_suite = update.split(",");
        let pages_numbers = pages_suite.map(|p| p.parse::<u8>().unwrap()).collect();
        pages.push(pages_numbers);
    }

    println!("Those are updates: {:?}", pages);

    (graph, pages)
}

#[aoc(day5, part1)]
fn part1(input: &(HashMap<u8, Vec<u8>>, Vec<Vec<u8>>)) -> u32 {
    let (graph, pages) = input;
    let result = 0;

    for update in pages {
        let mut is_valid = false;

        let result = update
            .windows(2)
            .inspect(|win| println!("a: {}, b: {}", win[0], win[1]))
            .collect::<Vec<_>>();

        println!("result {:?}", result);
    }
    result
}

// #[aoc(day5, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse(read_file("day5_example").as_str());
        let result: u32 = 143;
        assert_eq!(part1(input), result);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}

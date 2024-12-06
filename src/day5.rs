use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn parse(input: &str) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    // split on double line break: then split on pipe or split on comma
    let mut splitted_input = input.split("\n\n");
    let rules = splitted_input.next().expect("rules").lines();
    let updates = splitted_input.next().expect("pages number").lines();

    // rules can be view as an oriented graph
    // then the page are the path and we need to know if the path exist or not
    let mut graph: HashSet<(u8, u8)> = HashSet::new();
    for rule in rules {
        let mut nodes = rule.split("|");
        let from: u8 = nodes.next().unwrap().parse().unwrap();
        let to: u8 = nodes.next().unwrap().parse().unwrap();

        // this generates an oriented graph
        // let mut graph: HashSet<u8, Vec<u8>> = HashSet::new();
        // if graph.contains_key(&from) {
        //     graph.get_mut(&from).expect("from mut").push(to);
        // } else {
        //     graph.insert(from, vec![to]);
        // }

        // as all solutions are in rules we can have just a hash map of pairs
        graph.insert((from, to));
    }

    // println!("This is my graph {:?}", graph);

    // let pages: Vec<Vec<u8>> = updates.iter().split(",").collect();
    let mut pages: Vec<Vec<u8>> = vec![];
    for update in updates {
        let pages_suite = update.split(",");
        let pages_numbers = pages_suite.map(|p| p.parse::<u8>().unwrap()).collect();
        pages.push(pages_numbers);
    }

    // println!("Those are updates: {:?}", pages);

    (graph, pages)
}

#[aoc(day5, part1)]
pub fn part1(input: &(HashSet<(u8, u8)>, Vec<Vec<u8>>)) -> usize {
    let (graph, all_updates) = input;
    let mut result: usize = 0;

    // println!("all update: {:?}", all_updates);

    'outer: for update in all_updates {
        // println!("Update: {:?}", update);
        for window in update.windows(2) {
            if let [previous_page, next_page] = window {
                if !graph.contains(&(*previous_page, *next_page)) {
                    // println!("Invalid update {:?}", page_order);
                    continue 'outer;
                }
            }
        }

        // println!("Update is valid: {:?}", update);

        // always odd number of pages
        let middle = (update.len() - 1) / 2;
        result += update[middle] as usize;
    }
    result
}

#[aoc(day5, part2)]
pub fn part2(input: &(HashSet<(u8, u8)>, Vec<Vec<u8>>)) -> usize {
    let (graph, all_updates) = input;
    let mut result: usize = 0;

    // println!("all update: {:?}", all_updates);

    for update in all_updates {
        // println!("Update: {:?}", update);
        let mut update_mut = update.to_vec();
        let mut was_incorrect: bool = false;
        let mut is_correct: bool = false;

        'outer: while !is_correct {
            // println!("in while {:?}", update_mut);

            for window in update_mut.windows(2) {
                if let [previous_page, next_page] = window {
                    if !graph.contains(&(*previous_page, *next_page)) {
                        // here we swap the two numbers and re test the whole update by breaking the loop
                        // println!("Invalid update, swaping {:?}", page_order);
                        let pp_index = update_mut
                            .iter()
                            .position(|p| *p == *previous_page)
                            .unwrap();
                        let np_index = update_mut.iter().position(|p| *p == *next_page).unwrap();

                        update_mut.swap(pp_index, np_index);
                        // println!("new stuff: {:?}", update_mut);
                        was_incorrect = true;
                        continue 'outer;
                    }
                }
            }
            is_correct = true
        }

        // println!("Update is valid: {:?}", update);

        // always odd number of pages
        if was_incorrect {
            let middle = (update_mut.len() - 1) / 2;
            result += update_mut[middle] as usize;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_file;

    #[test]
    fn part1_example() {
        let input = parse(read_file("day5_example").as_str());
        let result: usize = 143;
        assert_eq!(part1(&input), result);
    }

    #[test]
    fn part2_example() {
        let input = parse(read_file("day5_example").as_str());
        let result: usize = 123;
        assert_eq!(part2(&input), result);
    }

    #[test]
    fn day5_part1() {
        let input = parse(read_file("day5").as_str());
        let result: usize = 5108;
        assert_eq!(part1(&input), result);
    }

    #[test]
    fn day5_part2() {
        let input = parse(read_file("day5").as_str());
        let result: usize = 7380;
        assert_eq!(part2(&input), result);
    }
}

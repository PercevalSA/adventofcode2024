use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    // split on double line break
    // then split on pipe
    // split on comma
    todo!()

    // rules can be view as an oriented graph
    // then the page are the path and we need to know if the path exist or not 
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
use std::collections::{HashMap, HashSet};

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut total_questions = 0;
    for group in input.split("\n\n") {
        let mut questions = HashSet::new();
        for person in group.lines() {
            for answer in person.chars() {
                questions.insert(answer);
            }
        }
        total_questions += questions.len();
    }

    total_questions
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let mut total_questions = 0;
    for group in input.split("\n\n") {
        let mut questions: HashMap<char, usize> = HashMap::new();
        let persons = group.lines().count();
        for person in group.lines() {
            for answer in person.chars() {
                *questions.entry(answer).or_insert(0) += 1;
            }
        }
        for entry in questions.values() {
            if entry == &persons {
                total_questions += 1;
            }
        }
    }

    total_questions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day6.txt");
        assert_eq!(part1(input), 6504);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day6.txt");
        assert_eq!(part2(input), 3351);
    }
}

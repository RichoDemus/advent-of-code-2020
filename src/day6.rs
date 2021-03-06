use std::collections::{HashMap, HashSet};

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut total_questions = 0;
    let mut questions = HashSet::new();
    for group in input.split("\n\n") {
        for person in group.lines() {
            for answer in person.chars() {
                questions.insert(answer);
            }
        }
        total_questions += questions.len();
        questions.clear();
    }

    total_questions
}

#[aoc(day6, part1, fp)]
fn part1_fp(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(str::chars)
                .fold(HashSet::new(), |mut acc, char| {
                    acc.insert(char);
                    acc
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part1, bit)]
fn part1_bit(input: &str) -> u32 {
    let mut total_questions = 0;
    let mut bits: u32 = 0;
    for group in input.split("\n\n") {
        for person in group.lines() {
            for answer in person.chars() {
                let ascii = answer as u128 - 97;
                let bit = 1_u32 << ascii;
                bits |= bit;
            }
        }
        // get the hamming weight (count 1s in a binary number)
        while bits > 0 {
            if bits & 1 == 1 {
                total_questions += 1;
            }
            bits >>= 1;
        }
        bits = 0;
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
        assert_eq!(part1_fp(input), 6504);
        assert_eq!(part1_bit(input), 6504);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day6.txt");
        assert_eq!(part2(input), 3351);
    }
}

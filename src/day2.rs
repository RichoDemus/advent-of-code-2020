use regex::Regex;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input.lines().into_iter().filter(password_valid).count()
}

#[aoc(day2, part1, perf)]
fn part1_perf(input: &str) -> usize {
    let mut valid_passwords = 0;

    for line in input.lines() {
        let mut split = line.split(':');
        let policy = split.next().unwrap();
        let password = split.next().unwrap();

        let mut ranges_and_char = policy.split(' ');
        let ranges = ranges_and_char.next().unwrap();
        let char = ranges_and_char.next().unwrap();

        let mut ranges = ranges.split('-');

        let lower_bounds: usize = ranges.next().unwrap().parse().unwrap();
        let upper_bounds: usize = ranges.next().unwrap().parse().unwrap();

        let characters = password.matches(char).count();
        if characters <= upper_bounds && characters >= lower_bounds {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .filter(password_valid_part2)
        .count()
}

fn password_valid(input: &&str) -> bool {
    let split = input.split(':').collect::<Vec<_>>();
    let policy = split.get(0).unwrap();
    let password = split.get(1).unwrap();
    let password = password.trim();

    let ranges_and_char = policy.split(' ').collect::<Vec<_>>();
    let char = ranges_and_char.get(1).unwrap();
    let ranges = ranges_and_char.get(0).unwrap();

    let ranges = ranges.split('-').collect::<Vec<_>>();

    let lower_bounds = ranges.get(0).unwrap();
    let lower_bounds: usize = lower_bounds.parse().unwrap();
    let upper_bounds = ranges.get(1).unwrap();
    let upper_bounds: usize = upper_bounds.parse().unwrap();

    let characters = password.matches(char).count();

    if characters > upper_bounds {
        return false;
    }
    if characters < lower_bounds {
        return false;
    }
    true
}

fn password_valid_part2(input: &&str) -> bool {
    let split = input.split(':').collect::<Vec<_>>();
    let policy = split.get(0).unwrap();
    let password = split.get(1).unwrap();
    let password = password.trim();

    let ranges_and_char = policy.split(' ').collect::<Vec<_>>();
    let char = ranges_and_char.get(1).unwrap().chars().next().unwrap();
    let ranges = ranges_and_char.get(0).unwrap();

    let ranges = ranges.split('-').collect::<Vec<_>>();

    let lower_bounds = ranges.get(0).unwrap();
    let first_index: usize = lower_bounds.parse().unwrap();
    let upper_bounds = ranges.get(1).unwrap();
    let second_index: usize = upper_bounds.parse().unwrap();

    let first_character = password.chars().nth(first_index - 1).unwrap();
    let second_character = password.chars().nth(second_index - 1).unwrap();

    if first_character == second_character {
        false
    } else {
        first_character == char || second_character == char
    }
}

#[aoc(day2, part1, regex)]
fn part1_regex(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|(lower, higher, char, password)| {
            let characters = password.matches(*char).count();
            characters <= *higher && characters >= *lower
        })
        .count()
}

#[aoc(day2, part1, regex_no_vec)]
fn part1_regex_no_vec(input: &str) -> usize {
    let re = Regex::new(r"(?m)^(\d+)-(\d+)\s(\w):\s(.*)$").unwrap();
    let mut valid_passwords = 0;
    for capture in re.captures_iter(input) {
        let first_number: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let second_number: usize = capture.get(2).unwrap().as_str().parse().unwrap();
        let character: char = capture.get(3).unwrap().as_str().parse().unwrap();
        let password: &str = capture.get(4).unwrap().as_str();

        let characters = password.matches(character).count();
        if characters <= second_number && characters >= first_number {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn parse(input: &str) -> Vec<(usize, usize, char, &str)> {
    let re = Regex::new(r"(?m)^(\w+)-(\w+)\s(\w):\s(.*)$").unwrap();

    re.captures_iter(input)
        .map(|capture| {
            let first_number: usize = capture.get(1).unwrap().as_str().parse().unwrap();
            let second_number: usize = capture.get(2).unwrap().as_str().parse().unwrap();
            let character: char = capture.get(3).unwrap().as_str().parse().unwrap();
            let password: &str = capture.get(4).unwrap().as_str();
            (first_number, second_number, character, password)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part1(input);

        assert_eq!(result, 396);
    }

    #[test]
    fn test_part1_regex() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part1_regex(input);

        assert_eq!(result, 396);
    }

    #[test]
    fn test_part1_regex_no_vec() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part1_regex_no_vec(input);

        assert_eq!(result, 396);
    }

    #[test]
    fn test_part1_perf() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part1_perf(input);

        assert_eq!(result, 396);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part2(input);

        assert_eq!(result, 428);
    }
}

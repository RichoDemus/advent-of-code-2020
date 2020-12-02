use regex::Regex;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().into_iter().filter(password_valid).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
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
pub fn part1_regex(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|(lower, higher, char, password)| {
            let characters = password.matches(*char).count();
            characters <= *higher && characters >= *lower
        })
        .count()
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
    fn test_part2() {
        let input = include_str!("../input/2020/day2.txt");

        let result = part2(input);

        assert_eq!(result, 428);
    }
}

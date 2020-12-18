use itertools::Itertools;
use regex::Regex;

#[aoc(day18, part1)]
fn part1_homework(input: &str) -> u64 {
    input
        .lines()
        .map(part1_calculate_line)
        .map(|(count, _)| count)
        .sum()
}

#[aoc(day18, part2)]
fn part2_homework(input: &str) -> u128 {
    input.lines().map(part2_calculate_line).sum()
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    None,
}

fn part1_calculate_line(line: &str) -> (u64, usize) {
    let mut sum = 0;
    let mut operation = Operation::Addition;
    let mut parenthesis_skip = 0;
    let mut skip_to_index = 0;

    for (index, char) in line.chars().enumerate() {
        if index < skip_to_index {
            continue;
        }
        match char {
            '+' => operation = Operation::Addition,
            '*' => operation = Operation::Multiplication,
            '(' => {
                // lets recursively call this for the thing right of the (
                let right_side = &line[index + 1..];
                let (digit, new_index) = part1_calculate_line(right_side);
                skip_to_index = index + new_index + 1;
                match operation {
                    Operation::Addition => sum += digit as u64,
                    Operation::Multiplication => sum *= digit as u64,
                    Operation::None => panic!("Don't know which is the current operation"),
                }
                parenthesis_skip += 1;
            }
            ')' => {
                // exit this
                parenthesis_skip -= 1;
                if parenthesis_skip < 0 {
                    return (sum, index);
                }
            }
            ' ' => {}
            digit => {
                //we have a digit, lets operate it together with the last
                let digit = digit.to_digit(10).expect("This should be a number");
                match operation {
                    Operation::Addition => sum += u64::from(digit),
                    Operation::Multiplication => sum *= u64::from(digit),
                    Operation::None => panic!("Don't know which is the current operation"),
                }
                operation = Operation::None;
            }
        }
    }

    (sum, line.len())
}

pub fn part2_calculate_line(line: &str) -> u128 {
    let original = line;
    let mut line = original.to_string();

    // find the "smallest" parenthesis, resolve and replace it, repeat until no more parens
    while line.contains('(') {
        let mut left_parenthesis = 0;
        let mut right_parenthesis = 0;
        for (index, char) in line.chars().enumerate() {
            match char {
                '(' => left_parenthesis = index,
                ')' => {
                    right_parenthesis = index;
                    break;
                }
                _ => (),
            }
        }
        let expression = &line[left_parenthesis..=right_parenthesis];
        let inside_parenthesis = &expression[1..];
        let inside_parenthesis = &inside_parenthesis[0..inside_parenthesis.len() - 1];

        let value = evaluate_paranthesisless_expression(inside_parenthesis);
        let value_str = value.to_string();
        let value_str = value_str.as_str();
        let new_line = line.replace(expression, value_str);
        line = new_line;
    }

    let result = evaluate_paranthesisless_expression(line.as_str());
    result
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(?P<left>\d+) \+ (?P<right>\d+)").unwrap();
}

fn evaluate_paranthesisless_expression(line: &str) -> u128 {
    assert!(!line.contains('('));
    assert!(!line.contains(')'));

    let original_line = line;
    let mut line = original_line.to_string();
    while line.contains('+') {
        let mut iter = REGEX.captures_iter(line.as_str());
        let captures = iter.next().unwrap();
        let left_str = &captures["left"];
        let left = left_str.parse::<u64>().unwrap();
        let right_str = &captures["right"];
        let right = right_str.parse::<u64>().unwrap();

        let result = left + right;
        let expression = left_str.to_owned() + " + " + right_str;

        line = line.replacen(expression.as_str(), result.to_string().as_str(), 1);
    }
    // now there should be just numbers and multiplication signs

    let result = line
        .split('*')
        .map(|digit| digit.trim().parse::<u128>().unwrap())
        .fold1(|left, right| left * right)
        .unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day18.txt");
        assert_eq!(part1_homework(input), 21347713555555);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day18.txt");
        assert_eq!(part2_homework(input), 275011754427339);
    }

    #[test]
    fn test_no_parenthesis() {
        assert_eq!(part1_calculate_line("2 * 3").0, 6);
        assert_eq!(part1_calculate_line("2 + 3").0, 5);
        assert_eq!(part1_calculate_line("2 + 3 * 2").0, 10);
    }

    #[test]
    fn test_provided_examples() {
        assert_eq!(part1_calculate_line("2 * 3 + (4 * 5)").0, 26, "First");
        assert_eq!(
            part1_calculate_line("5 + (8 * 3 + 9 + 3 * 4 * 3)").0,
            437,
            "Second"
        );
        assert_eq!(
            part1_calculate_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").0,
            12240,
            "Third"
        );
        assert_eq!(
            part1_calculate_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").0,
            13632,
            "Fourth"
        );
    }

    #[test]
    fn test_no_parenthesis_part2() {
        assert_eq!(part2_calculate_line("10 + 100"), 110);
        assert_eq!(part2_calculate_line("(4)"), 4);
        assert_eq!(part2_calculate_line("((4 + 1))"), 5);
        assert_eq!(part2_calculate_line("((4 + 1) + 1) + (3 + 7)"), 16);
        assert_eq!(part2_calculate_line("2 * 3"), 6);
        assert_eq!(part2_calculate_line("2 + 3"), 5);
        assert_eq!(part2_calculate_line("2 + 3 * 2"), 10);
        assert_eq!(part2_calculate_line("2 * 2 + 3"), 10);
        assert_eq!(part2_calculate_line("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2_calculate_line("1 + 2 * 3 + 4 * 5 + 6"), 231);
    }

    #[test]
    fn test_part2_provided() {
        assert_eq!(part2_calculate_line("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2_calculate_line("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2_calculate_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            part2_calculate_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            part2_calculate_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }

    #[test]
    fn more_tests() {
        assert_eq!(part2_calculate_line("((2 * 4 * 5 * 2 * 3) + 7 + 6 * 4 + 6) + 4 * (3 + (6 + 3 + 6 * 5 * 6 * 2) * 5 + (3 * 4 * 2 * 9 + 5 * 9) + (3 * 6 * 6 + 4) * 2) + 4 * 7"), 102799834004);
        assert_eq!(part2_calculate_line("5 + (9 + 9 * (4 + 7 + 2 + 9 * 7 * 6) + 3 * 9) + (9 * 4 + 3 * (4 * 9 + 4 * 3 * 4 + 2)) * (9 + 7 + 7 * 8 * 4) + 9 + ((7 + 2 * 5 + 5 + 8) * 2 * (6 + 9 * 3 + 9))"), 12353267555);
        assert_eq!(part2_calculate_line("((8 * 5 * 6 + 4 * 9) + 6 + 2 + 5) * 7 + (4 * 8 * 6 * 7 + (7 * 3 * 3 * 8 + 6) * 7) * 5 + 9 + (3 * 5 * 9 + 2 + (5 + 9 * 6) + (4 + 3 * 8 + 6 * 3 + 3))"), 44287030924241);
        assert_eq!(
            part2_calculate_line(
                "8 + 3 * ((4 * 8 * 5 * 5) + (8 * 9 * 8 + 7 * 9)) + (4 * 7) + 3 * 4"
            ),
            464244
        );
    }
}

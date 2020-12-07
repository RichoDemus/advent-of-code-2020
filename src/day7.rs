use crate::util::str_split;
use regex::Regex;
use std::collections::HashMap;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let rules = input
        .lines()
        .map(line_to_bag_rule)
        .map(|rule| (rule.color, rule.contents))
        .collect::<HashMap<String, Vec<Content>>>();

    rules
        .keys()
        .map(|bag| can_contain_golden_bag_rec(&rules, bag))
        .filter(|can_contain_golden_bag| *can_contain_golden_bag)
        .count()
}

fn can_contain_golden_bag_rec(bags: &HashMap<String, Vec<Content>>, current_bag: &str) -> bool {
    match bags.get(current_bag) {
        None => false,
        Some(bags_inside) if bags_inside.is_empty() => false,
        Some(bags_inside) if bags_inside.iter().any(|bag| bag.color == *"shiny gold") => true,
        Some(bags_inside) => bags_inside
            .iter()
            .map(|bag| can_contain_golden_bag_rec(bags, &bag.color))
            .any(|has_golden_bag| has_golden_bag),
    }
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let rules = input
        .lines()
        .map(line_to_bag_rule)
        .map(|rule| (rule.color, rule.contents))
        .collect::<HashMap<String, Vec<Content>>>();

    get_bags_inside_rec(&rules, &"shiny gold".to_string()) - 1
}

fn get_bags_inside_rec(bags: &HashMap<String, Vec<Content>>, current_bag: &str) -> usize {
    let empty_vec = vec![];
    let bags_inside = bags.get(current_bag).unwrap_or(&empty_vec);

    let inside: usize = bags_inside
        .iter()
        .map(|content| get_bags_inside_rec(bags, &content.color) * content.amount)
        .sum();
    1 + inside
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    color: String,
    contents: Vec<Content>,
}
#[derive(Debug, Eq, PartialEq)]
struct Content {
    amount: usize,
    color: String,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^\s*(\d+)\s(.*)\sbag.*$").unwrap();
}

fn line_to_bag_rule(line: &str) -> Rule {
    let (color, contents): (String, String) = str_split(line, "contain").unwrap();

    let (color, _): (String, String) = str_split(&color, "bag").unwrap();
    let color = color.trim();

    if contents.contains("no other bag") {
        return Rule {
            color: color.to_string(),
            contents: vec![],
        };
    }

    let contents = contents
        .split(',')
        .map(|content| {
            let capture = REGEX.captures_iter(content).next().unwrap();
            let amount = capture.get(1).unwrap().as_str().parse().unwrap();
            let color = capture.get(2).unwrap().as_str();

            Content {
                amount,
                color: color.to_string(),
            }
        })
        .collect::<Vec<_>>();

    Rule {
        color: color.to_string(),
        contents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day7.txt");
        assert_eq!(part1(input), 335);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day7.txt");
        assert_eq!(part2(input), 2431);
    }

    #[test]
    fn test_parse_single_line_no_contents() {
        let result = line_to_bag_rule("faded blue bags contain no other bags.");

        assert_eq!(
            result,
            Rule {
                color: "faded blue".to_string(),
                contents: vec![]
            }
        );
    }

    #[test]
    fn test_parse_single_line_one_contents() {
        let result = line_to_bag_rule("bright white bags contain 1 shiny gold bag.");

        assert_eq!(
            result,
            Rule {
                color: "bright white".to_string(),
                contents: vec![Content {
                    amount: 1,
                    color: "shiny gold".to_string()
                },]
            }
        );
    }

    #[test]
    fn test_parse_single_line_multiple_contents() {
        let result =
            line_to_bag_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.");

        assert_eq!(
            result,
            Rule {
                color: "light red".to_string(),
                contents: vec![
                    Content {
                        amount: 1,
                        color: "bright white".to_string()
                    },
                    Content {
                        amount: 2,
                        color: "muted yellow".to_string()
                    },
                ]
            }
        );
    }

    #[test]
    fn provided_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let result = part1(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn provided_example2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let result = part2(input);

        assert_eq!(result, 126);
    }
}

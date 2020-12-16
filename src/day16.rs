use crate::util;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc(day16, part1)]
fn part1_error_scanning_rate(input: &str) -> u64 {
    let parsed = parse(input);

    let mut error_scanning_rate = 0;
    for ticket in &parsed.nearby_tickets {
        let invalid_fields = get_invalid_fields(ticket, &parsed.rules);
        error_scanning_rate += invalid_fields.iter().sum::<u64>();
    }

    error_scanning_rate
}

#[aoc(day16, part2)]
fn part2_multiply_departures(input: &str) -> u64 {
    let parsed = parse(input);

    let fields = figure_out_which_field_is_which(&parsed.nearby_tickets, &parsed.rules);

    let departures = fields
        .into_iter()
        .filter_map(|(field_index, class)| {
            if class.starts_with("departure") {
                Some(parsed.my_ticket.fields.get(field_index).copied().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(departures.len(), 6);

    departures
        .into_iter()
        .fold1(|left, right| left * right)
        .unwrap()
}

#[derive(Debug, Eq, PartialEq)]
struct TicketInfo {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    class: String,
    ranges: Vec<(u64, u64)>,
}

impl Rule {
    fn invalid(&self, field: u64) -> bool {
        for (lower, upper) in &self.ranges {
            if lower <= &field && &field <= upper {
                return false;
            }
        }
        true
    }
}

// todo try out asref
impl From<(String, u64, u64, u64, u64)> for Rule {
    fn from((class, r1, r2, r3, r4): (String, u64, u64, u64, u64)) -> Self {
        Self {
            class,
            ranges: vec![(r1, r2), (r3, r4)],
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket {
    fields: Vec<u64>,
}

impl From<Vec<u64>> for Ticket {
    fn from(ticket: Vec<u64>) -> Self {
        Self { fields: ticket }
    }
}

fn parse(input: &str) -> TicketInfo {
    let (rules, my_ticket, nearby_tickets): (&str, &str, &str) =
        input.split("\n\n").collect_tuple().unwrap();

    TicketInfo {
        rules: parse_rules(rules),
        my_ticket: parse_tickets(my_ticket)
            .into_iter()
            .next()
            .expect("should be a ticket here"),
        nearby_tickets: parse_tickets(nearby_tickets),
    }
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules.lines().map(parse_rule).collect()
}

fn parse_rule(rule: &str) -> Rule {
    let (class, rules): (String, String) = util::str_split(rule, ":").unwrap();
    let (first_rule, second_rule): (String, String) =
        util::str_split(rules.as_str(), "or").unwrap();

    let (first_first_rule, first_second_rule): (u64, u64) =
        util::str_split(first_rule.as_str(), "-").unwrap();
    let (second_first_rule, second_second_rule): (u64, u64) =
        util::str_split(second_rule.as_str(), "-").unwrap();

    Rule::from((
        class,
        first_first_rule,
        first_second_rule,
        second_first_rule,
        second_second_rule,
    ))
}

fn parse_ticket(ticket: &str) -> Ticket {
    let numbers = ticket
        .split(',')
        .map(|n| {
            n.parse()
                .unwrap_or_else(|_| panic!("Couldn't parse {:?}", n))
        })
        .collect::<Vec<_>>();
    Ticket::from(numbers)
}

fn parse_tickets(rules: &str) -> Vec<Ticket> {
    rules
        .lines()
        .filter_map(|line| {
            if line.contains("your ticket") || line.contains("nearby tickets") {
                None
            } else {
                Some(parse_ticket(line))
            }
        })
        .collect()
}

fn get_invalid_fields(ticket: &Ticket, rules: &[Rule]) -> Vec<u64> {
    let mut invalid_fields = vec![];

    'field: for field in &ticket.fields {
        for rule in rules {
            if !rule.invalid(*field) {
                continue 'field;
            }
        }
        invalid_fields.push(*field);
    }

    invalid_fields
}

fn figure_out_which_field_is_which(tickets: &[Ticket], rules: &[Rule]) -> HashMap<usize, String> {
    let valid_tickets = tickets
        .iter()
        .filter(|ticket| get_invalid_fields(ticket, rules).is_empty())
        .collect::<Vec<_>>();

    let mut field_to_valid_rules_mapping: HashMap<usize, HashSet<String>> = HashMap::new();

    for rule in rules {
        'field: for field_index in 0..rules.len() {
            for ticket in &valid_tickets {
                let field = ticket
                    .fields
                    .get(field_index)
                    .expect("should be a field here");
                if rule.invalid(*field) {
                    // check if its valid for next field
                    continue 'field;
                }
            }
            // this means that this rule things all of these fields valid
            field_to_valid_rules_mapping
                .entry(field_index)
                .or_insert_with(HashSet::new)
                .insert(rule.class.clone());
        }
    }

    let mut fields_and_rules = field_to_valid_rules_mapping.into_iter().collect::<Vec<_>>();

    let mut result: HashMap<usize, String> = HashMap::new();
    while !fields_and_rules.is_empty() {
        let (index_with_least_candidates, class) = {
            let (index_with_least_candidates, candidates) = fields_and_rules
                .iter()
                .min_by_key(|(_, rules)| rules.len())
                .unwrap();
            assert_eq!(
                candidates.len(),
                1,
                "expected to have a candidate with just 1 valid fields: {:?}",
                fields_and_rules
            );

            let class = candidates.iter().next().cloned().unwrap();
            (*index_with_least_candidates, class)
        };

        fields_and_rules = fields_and_rules
            .into_iter()
            .filter_map(|(index, mut classes)| {
                classes.remove(class.as_str());
                if classes.is_empty() {
                    None
                } else {
                    Some((index, classes))
                }
            })
            .collect();
        result.insert(index_with_least_candidates, class);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day16.txt");
        assert_eq!(part1_error_scanning_rate(input), 22000);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day16.txt");
        assert_eq!(part2_multiply_departures(input), 410460648673);
    }

    #[test]
    fn test_my_own_example() {
        let rules = vec![
            Rule::from((String::from("class"), 1, 2, 4, 5)),
            Rule::from((String::from("row"), 7, 8, 10, 11)),
        ];

        let result = get_invalid_fields(&Ticket::from(vec![0, 0]), &rules);
        assert_eq!(result, vec![0, 0]);

        let result = get_invalid_fields(&Ticket::from(vec![0, 1]), &rules);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_part2_provided() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let parsed = parse(input);

        let fields = figure_out_which_field_is_which(&parsed.nearby_tickets, &parsed.rules);

        assert_eq!(fields[&0], "row");
        assert_eq!(fields[&1], "class");
        assert_eq!(fields[&2], "seat");
    }
}

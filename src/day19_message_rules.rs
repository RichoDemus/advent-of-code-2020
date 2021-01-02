use regex::Regex;
use crate::util;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug,Eq, PartialEq, Clone)]
enum Rule {
    Char(char),
    Sequence(Vec<usize>),
    Or(Vec<usize>,Vec<usize>),
}

lazy_static! {
    // static ref RULE_REGEX: Regex = Regex::new(r"(?P<left>\d+) \+ (?P<right>\d+)").unwrap();
    static ref RULE_REGEX: Regex = Regex::new("(?P<index>\\d+): \"?(P?<value>[\\d\\w]+)\"?").unwrap();
}

fn parse(input: &str) -> (Vec<Rule>, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap();

    let rules = rules.lines().map(|line|{
        let (index, data):(usize, String) = util::str_split(line, ":").unwrap();

        if data.contains("\"") {
            //it's  i: "c"
            let value = data.replace('"', "");
            let value:char = value.parse().unwrap_or_else(|_|panic!("Couldn't parse {:?}", value));
            (index,Rule::Char(value))
        } else if data.contains('|') {
            // it's i: x y | z h
            let (left, right): (Vec<usize>, Vec<usize>) = data.split('|')
                .map(|digits|digits.split_ascii_whitespace()
                    .map(|digit|digit.parse().unwrap())
                    .collect::<Vec<_>>())
                .collect_tuple().unwrap_or_else(||panic!("Couldn't parse pair of vec for {:?}", data));

            (index,Rule::Or(left, right))
        } else {
            // it's i: x y z
            let values = data.split_ascii_whitespace()
                .map(|digit|digit.parse().unwrap())
                .collect::<Vec<_>>();
            (index,Rule::Sequence(values))
        }
    }).collect::<Vec<_>>();

    let mut result_rules = vec![];
    let max = rules.iter().map(|(index,_)|index).max().unwrap();
    result_rules.resize_with(*max + 1, ||None);
    for (i,rule) in rules {
        result_rules[i] = Some(rule);
    }

    assert_eq!(result_rules.iter().filter(|rule|rule.is_none()).count(), 0);
    let result_rules = result_rules.into_iter().map(|rule|rule.unwrap()).collect();


    let messages = iter.next().unwrap();
    let messages = messages.lines().collect();

     (result_rules, messages)
}

fn get_valid_messages<'a>(messages: &'a [&str], rules: &[Rule]) -> Vec<&'a str> {
    messages.into_iter().filter(|msg|is_valid(msg, rules)).cloned().collect()
}

fn is_valid(message: &str, rules: &[Rule]) -> bool {
    println!("Does {:?} match {:?}", message, rules);
    let mut rules_stack: VecDeque<Rule> = VecDeque::new();
    rules_stack.push_front(rules.first().cloned().unwrap());
    let mut index_stack = VecDeque::new();

    let mut current_rule = rules.first().unwrap();
    let mut next_rule_index = 1;

    let mut i = 0;
    loop {
        let char = match &message.chars().nth(i) {
            None => {
                // I think this means we're done
                return true;
            }
            Some(c) => *c,
        };

        println!("\tchecking {:?}({}), rules: {:?}", char, i, rules_stack);
        let current_rule = rules_stack.pop_front().expect("should be a rule here");
        match &current_rule {
            Rule::Char(c) => {
                if char != *c {
                    // not a match
                    if index_stack.is_empty() {
                        println!("\t\tchar rule invalid: {:?} != {:?}. but index stack: {:?}", char, c, index_stack);
                        return false;
                    } else {
                        // this is wrong but we have alternate paths to follow
                        println!("\t\tchar rule invalid: {:?} != {:?}. but index stack: {:?}", char, c, index_stack);
                    }

                } else {
                    println!("\t\tChar rule valid!");
                    i += 1;
                }
            }
            Rule::Sequence(seq) => {
                let rules = seq.iter().map(|index|&rules[*index]).rev().collect::<Vec<_>>();
                println!("\t\t{:?} -> {:?}", current_rule, rules);
                for rule in rules {
                    rules_stack.push_front(rule.clone());
                }
            }
            Rule::Or(left_rules, right_rules) => {
                // new approach, we kinda clone the stacks or something
                //


                // if left_rules.is_empty() {
                //     // we've reached an or for the second time, peek to see if next is ok or we if change leg
                //     let next_rule = rules_stack.get(1).expect("Should be a rule here I think");
                //      // fuck, do I just
                //
                //
                //     i = index_stack.pop_front().expect("There should be an index here...");
                //     let rules = right_rules.iter().map(|index|&rules[*index]).rev().collect::<Vec<_>>();
                //     for rule in rules {
                //         rules_stack.push_front(rule.clone());
                //     }
                //
                // } else {
                //     // first time seeing this or, process left leg
                //     let rules = left_rules.iter().map(|index|&rules[*index]).rev().collect::<Vec<_>>();
                //     println!("\t\t{:?} -> {:?}", current_rule, rules);
                //     let replacement_of_current_rule = Rule::Or(vec![], right_rules.clone());
                //     rules_stack.push_front(replacement_of_current_rule);
                //     index_stack.push_front(i);
                //     for rule in rules {
                //         rules_stack.push_front(rule.clone());
                //     }
                //
                // }

            }
        }

    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2020/day18.txt");
    //     assert_eq!(part1_homework(input), 21347713555555);
    // }
    //
    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2020/day18.txt");
    //     assert_eq!(part2_homework(input), 275011754427339);
    // }

    #[test]
    fn test_parse() {
        let input = include_str!("../input/2020/day19.txt");

        let parsed = parse(input);

        // we just don't wanna panic

        // also make sure we're not order dependant


        let ordered_input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let scrambled_input = r#"0: 4 1 5
5: "b"
1: 2 3 | 3 2
3: 4 5 | 5 4
2: 4 4 | 5 5
4: "a"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let (ordered_rules, _) = parse(ordered_input);
        let (scrambled_rules, _) = parse(scrambled_input);

        assert_eq!(ordered_rules, scrambled_rules);
    }

    #[test]
    fn simple_example(){
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

abbbab"#;

        let (rules, messages) = parse(input);

        let result = get_valid_messages(&messages, &rules);
        assert_eq!(result, vec![
            "abbbab"
        ]);

    }

    #[test]
    fn test_provided_example() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let (rules, messages) = parse(input);


        println!("{:?}", messages);

        assert_eq!(rules, vec![
            Rule::Sequence(vec![4,1,5]),
            Rule::Or(vec![2,3], vec![3,2]),
            Rule::Or(vec![4,4], vec![5,5]),
            Rule::Or(vec![4,5], vec![5,4]),
            Rule::Char('a'),
            Rule::Char('b'),
        ]);

        assert_eq!(messages, vec![
"ababbb",
"bababa",
"abbbab",
"aaabbb",
"aaaabbb",
        ]);

        let result = get_valid_messages(&messages, &rules);
        assert_eq!(result, vec![
            "ababbb", "abbbab"
        ]);

    }
}
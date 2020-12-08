use std::collections::HashSet;

use crate::util::{add, str_split};

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let instructions = input.lines().collect::<Vec<_>>();
    let mut visitied_instructions = HashSet::new();
    let mut instruction_pointer: usize = 0;
    let mut accumulator = 0;
    loop {
        if visitied_instructions.contains(&instruction_pointer) {
            return accumulator;
        }
        let instruction = instructions
            .get(instruction_pointer)
            .expect("should be an instruction here");
        let (instruction, value): (String, i32) = str_split(instruction, " ").unwrap();
        match instruction.as_str() {
            "acc" => {
                accumulator += value;
            }
            "jmp" => {
                instruction_pointer = add(instruction_pointer, value - 1).unwrap();
                // -1 to offset the normal jump ^^
            }
            "nop" => (),
            inst => panic!("inst: {:?} val: {}", inst, value),
        }
        visitied_instructions.insert(instruction_pointer);
        instruction_pointer += 1;
    }
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let code = input
        .lines()
        .map(|line| {
            let (instruction, value): (String, i32) = str_split(line, " ").unwrap();
            (instruction, value)
        })
        .collect::<Vec<_>>();

    match part2_inner(&*code) {
        Ok(code) => code,
        Err(msg) => panic!("Not like this: {:?}", msg),
    }
}

fn part2_inner(code: &[(String, i32)]) -> Result<i32, String> {
    let mutations = generate_mutations(code);

    for mutation in mutations {
        let exit_code = get_exit_code(&mutation);
        if let Ok(code) = exit_code {
            return Ok(code);
        }
    }
    Err(String::from("No solution found"))
}

fn generate_mutations(code: &[(String, i32)]) -> Vec<Vec<(String, i32)>> {
    let mut mutations = vec![];
    for (i, (instruction, _)) in code.iter().enumerate() {
        if instruction.as_str() == "nop" {
            let mut mutation = code.to_owned();
            let _ = std::mem::replace(&mut mutation[i].0, String::from("jmp"));
            mutations.push(mutation);
        } else if instruction.as_str() == "jmp" {
            let mut mutation = code.to_owned();
            let _ = std::mem::replace(&mut mutation[i].0, String::from("nop"));
            mutations.push(mutation);
        }
    }

    mutations
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InfiniteLoopError;

fn get_exit_code(code: &[(String, i32)]) -> Result<i32, InfiniteLoopError> {
    let mut visitied_instructions = HashSet::new();
    let mut instruction_pointer: usize = 0;
    let mut accumulator = 0;
    loop {
        if visitied_instructions.contains(&instruction_pointer) {
            return Err(InfiniteLoopError);
        }
        let (instruction, value) = match code.get(instruction_pointer as usize) {
            None => return Ok(accumulator),
            Some(v) => v,
        };
        visitied_instructions.insert(instruction_pointer);
        match instruction.as_str() {
            "acc" => {
                accumulator += value;
                instruction_pointer += 1;
            }
            "jmp" => {
                if value == &0 {
                    //jmp 0 -_-
                    instruction_pointer += 1;
                } else {
                    instruction_pointer = add(instruction_pointer, *value).unwrap();
                }
            }
            "nop" => {
                instruction_pointer += 1;
            }
            inst => panic!("inst: {:?} val: {}", inst, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day8.txt");
        assert_eq!(part1(input), 1137);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day8.txt");
        assert_eq!(part2(input), 1125);
    }

    #[test]
    fn test_detect_infinite_loop() {
        let infinite_loop = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ]
        .into_iter()
        .map(|(s, i)| (String::from(s), i))
        .collect::<Vec<_>>();

        assert_eq!(get_exit_code(&infinite_loop), Err(InfiniteLoopError));
    }

    #[test]
    fn test_detect_correct_exit_value() {
        let code = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("nop", -4),
            ("acc", 6),
        ]
        .into_iter()
        .map(|(s, i)| (String::from(s), i))
        .collect::<Vec<_>>();

        assert_eq!(get_exit_code(&code), Ok(8));
    }

    #[test]
    fn test_generate_mutations() {
        let input = vec![("nop", 1), ("acc", 2), ("jmp", 3)]
            .into_iter()
            .map(|(s, i)| (String::from(s), i))
            .collect::<Vec<_>>();

        let expected = vec![
            vec![("jmp", 1), ("acc", 2), ("jmp", 3)]
                .into_iter()
                .map(|(s, i)| (String::from(s), i))
                .collect::<Vec<_>>(),
            vec![("nop", 1), ("acc", 2), ("nop", 3)]
                .into_iter()
                .map(|(s, i)| (String::from(s), i))
                .collect::<Vec<_>>(),
        ];

        assert_eq!(generate_mutations(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = vec![("nop", 1), ("acc", 2), ("jmp", -1)]
            .into_iter()
            .map(|(s, i)| (String::from(s), i))
            .collect::<Vec<_>>();

        assert_eq!(part2_inner(&input), Ok(2));

        let input = vec![("nop", 1), ("acc", 2), ("nop", 3), ("jmp", -1), ("jmp", -1)]
            .into_iter()
            .map(|(s, i)| (String::from(s), i))
            .collect::<Vec<_>>();

        assert_eq!(part2_inner(&input), Ok(2));
    }
    #[test]
    fn test_part22() {
        let input = vec![
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("nop", -4),
            ("acc", 6),
        ]
        .into_iter()
        .map(|(s, i)| (String::from(s), i))
        .collect::<Vec<_>>();

        assert_eq!(part2_inner(&input), Ok(8));
    }
}

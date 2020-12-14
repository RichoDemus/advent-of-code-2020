use crate::util;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    let mut computer = DockingComputer::new();

    for op in input {
        computer.process(op);
    }
    computer.memory.iter().sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = String::new();
    for op in input {
        match op {
            Operation::Bitmask(m) => {
                mask = m;
            }
            Operation::MemSet(index, value) => {
                let index_str = format!("{:036b}", index);
                let mut new_address = String::new();
                for (mask_bit, address_bit) in mask.chars().zip(index_str.chars()) {
                    let bit = match (mask_bit, address_bit) {
                        ('0', a) => a,
                        ('1', _) => '1',
                        ('X', _) => 'X',
                        (m, a) => panic!("Unhandled combination. mask: {:?} addr: {:?}", m, a),
                    };
                    new_address.push(bit);
                }
                let addresses = handle_floatings(new_address.as_str());
                for address in addresses {
                    memory.insert(address, value);
                }
            }
        }
    }

    memory.values().sum()
}

fn handle_floatings(addr: &str) -> HashSet<usize> {
    let addr = addr.chars().collect::<Vec<_>>();

    handle_floatings_rec(addr, 0)
}

fn handle_floatings_rec(mut addr: Vec<char>, index: usize) -> HashSet<usize> {
    if index >= addr.len() {
        let str: String = addr.iter().collect();
        let binary = usize::from_str_radix(str.as_str(), 2).unwrap();
        return HashSet::from_iter(vec![binary]);
    }

    let char = addr.get(index).unwrap();
    if *char == 'X' {
        let mut clone = addr.clone();
        {
            let mem = clone.get_mut(index).unwrap();
            *mem = '1';
        }

        {
            let mem = addr.get_mut(index).unwrap();
            *mem = '0';
        }

        let result = HashSet::new();
        let result: HashSet<usize> = result
            .union(&handle_floatings_rec(clone, index + 1))
            .copied()
            .collect();
        let result: HashSet<usize> = result
            .union(&handle_floatings_rec(addr, index + 1))
            .copied()
            .collect();

        result
    } else {
        handle_floatings_rec(addr, index + 1)
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(parse_operation).collect()
}

fn parse_operation(str: &str) -> Operation {
    if str.contains("mask") {
        let (_, mask): (String, String) = util::str_split(str, "=").unwrap();
        Operation::Bitmask(mask)
    } else {
        let (mem, value): (String, u64) = util::str_split(str, "=").unwrap();
        let (_, mem): (String, String) = util::str_split(mem.as_str(), "[").unwrap();
        let (mem, _): (usize, String) = util::str_split(mem.as_str(), "]").unwrap();

        Operation::MemSet(mem, value)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct DockingComputer {
    memory: Vec<u64>,
    bitmask: String,
    mem_size: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Bitmask(String),
    MemSet(usize, u64),
}

impl DockingComputer {
    fn new() -> Self {
        let mem_size = 65021;
        Self {
            memory: vec![0; mem_size],
            bitmask: String::new(),
            mem_size: mem_size - 1, // weird I know, but makes the setting operations easier
        }
    }

    fn process(&mut self, op: Operation) {
        match op {
            Operation::Bitmask(mask) => self.bitmask = mask,
            Operation::MemSet(pos, value) => {
                assert!(
                    pos <= self.mem_size,
                    "pos {} larger than size {}",
                    pos,
                    self.mem_size
                );
                let new_pos = self.mem_size - pos;
                let mem = self.memory.get_mut(new_pos).unwrap();
                let value = apply_bitmask(value, self.bitmask.as_str());
                *mem = value;
            }
        }
    }
}

fn apply_bitmask(mut value: u64, bitmask: &str) -> u64 {
    for (i, char) in bitmask.chars().rev().enumerate() {
        match char {
            '1' => {
                let mask = 1_u64 << i;
                value |= mask;
            }
            '0' => {
                let mask = 1_u64 << i;
                value &= !mask;
            }
            _ => (),
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day14.txt");
        assert_eq!(part1(input), 6386593869035);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day14.txt");
        assert_eq!(part2(input), 4288986482164);
    }

    #[test]
    fn test_bitmask_in_isolation() {
        assert_eq!(
            apply_bitmask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            73
        );
        assert_eq!(
            apply_bitmask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            101
        );
        assert_eq!(apply_bitmask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 64);
    }

    #[test]
    fn test_parse() {
        let input = "mask = 00X10101X110010011XX0X011X100000X010\nmem[13197] = 47579321";

        let result = parse(input);

        assert_eq!(
            result,
            vec![
                Operation::Bitmask(String::from("00X10101X110010011XX0X011X100000X010")),
                Operation::MemSet(13197, 47579321)
            ]
        );
    }

    #[test]
    fn test_provided_example() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let result = part1(input);

        assert_eq!(result, 165);
    }

    #[test]
    fn test_provided_example_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let result = part2(input);

        assert_eq!(result, 208);
    }

    #[test]
    fn test_handle_floatings() {
        let result = handle_floatings("000000000000000000000000000000X1101X");

        assert_eq!(result, HashSet::from_iter(vec![26, 27, 58, 59]));
    }
}

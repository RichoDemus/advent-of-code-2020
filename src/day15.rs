use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[aoc(day15, part1)]
fn part1_2020th_number(input: &str) -> u32 {
    let starting_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut last_spokens: HashMap<u32, u32, RandomState> = HashMap::new();
    let mut next_number = 0;

    for turn in 1..2020 as u32 {
        let maybe_precomputed_number = starting_numbers.get((turn - 1) as usize);
        if let Some(precomp) = maybe_precomputed_number {
            let when_was_it_last_said = last_spokens.get(precomp).copied();
            last_spokens.insert(*precomp, turn as u32);
            // last_number = *precomp;

            // check if we have a precomputed number next
            if let Some(precomp) = starting_numbers.get(turn as usize) {
                next_number = *precomp;
                continue;
            }

            // we don't have a precomp next number, so now we need to start calculating it
            next_number = match when_was_it_last_said {
                None => {
                    // this number hasn't been said before
                    0
                }
                Some(last_turn) => {
                    // this was last said on turn `turn`
                    turn - last_turn
                }
            };
        } else {
            let this_turns_number = next_number;
            let when_was_it_last_said = last_spokens.get(&next_number).copied();
            last_spokens.insert(this_turns_number, turn as u32);

            next_number = match when_was_it_last_said {
                None => {
                    // this number hasn't been said before
                    0
                }
                Some(last_turn) => {
                    // this was last said on turn `turn`
                    turn - last_turn
                }
            };
        }
    }

    next_number
}

#[aoc(day15, part2)]
fn part2_30000000th_number(input: &str) -> u32 {
    let starting_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut last_spokens: HashMap<u32, u32, RandomState> = HashMap::new();
    let mut next_number = 0;

    for turn in 1..30000000 as u32 {
        let maybe_precomputed_number = starting_numbers.get((turn - 1) as usize);
        if let Some(precomp) = maybe_precomputed_number {
            let when_was_it_last_said = last_spokens.get(precomp).copied();
            last_spokens.insert(*precomp, turn as u32);
            // last_number = *precomp;

            // check if we have a precomputed number next
            if let Some(precomp) = starting_numbers.get(turn as usize) {
                next_number = *precomp;
                continue;
            }

            // we don't have a precomp next number, so now we need to start calculating it
            next_number = match when_was_it_last_said {
                None => {
                    // this number hasn't been said before
                    0
                }
                Some(last_turn) => {
                    // this was last said on turn `turn`
                    turn - last_turn
                }
            };
        } else {
            let this_turns_number = next_number;
            let when_was_it_last_said = last_spokens.get(&next_number).copied();
            last_spokens.insert(this_turns_number, turn as u32);

            next_number = match when_was_it_last_said {
                None => {
                    // this number hasn't been said before
                    0
                }
                Some(last_turn) => {
                    // this was last said on turn `turn`
                    turn - last_turn
                }
            };
        }
    }

    next_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day15.txt");
        assert_eq!(part1_2020th_number(input), 610);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2020/day14.txt");
    //     assert_eq!(part2(input), 4288986482164);
    // }

    #[test]
    fn test_provided_examples() {
        assert_eq!(part1_2020th_number("0,3,6"), 436);
        assert_eq!(part1_2020th_number("1,3,2"), 1);
        assert_eq!(part1_2020th_number("2,1,3"), 10);
        assert_eq!(part1_2020th_number("1,2,3"), 27);
        assert_eq!(part1_2020th_number("2,3,1"), 78);
        assert_eq!(part1_2020th_number("3,2,1"), 438);
        assert_eq!(part1_2020th_number("3,1,2"), 1836);
    }

    #[test]
    fn test_30000000th() {
        assert_eq!(part2_30000000th_number("0,3,6"), 175594);
    }
}

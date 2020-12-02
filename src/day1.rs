use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day1)]
pub fn lines_of_ints_to_int_array(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input
        .iter()
        .permutations(2)
        // map from vector of 2 elements to a tuple2
        .map(|vec: Vec<&i32>| (**vec.get(0).unwrap(), **vec.get(1).unwrap()))
        .find_map(|(left, right)| {
            if left + right == 2020 {
                Some(left * right)
            } else {
                None
            }
        })
        // unwrap option
        .expect("should be a number here")
}

#[aoc(day1, part1, Set)]
pub fn part1_set(input: &[i32]) -> i32 {
    let set = HashSet::<&i32>::from_iter(input);
    for x in &set {
        let right = 2020 - *x;
        if set.contains(&right) {
            return *x * right;
        }
    }
    panic!("no solution")
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .permutations(3)
        // map from vector of 3 elements to a tuple3
        .map(|vec| {
            (
                **vec.get(0).unwrap(),
                **vec.get(1).unwrap(),
                **vec.get(2).unwrap(),
            )
        })
        .find_map(|(left, middle, right)| {
            if left + middle + right == 2020 {
                Some(left * middle * right)
            } else {
                None
            }
        })
        // .find(|(left, middle, right)| left + middle + right == 2020)
        // .map(|(left, middle, right)| left * middle * right)
        .expect("should be a number here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/2020/day1.txt");
        let input = lines_of_ints_to_int_array(input);

        let result = part1(input.as_slice());

        assert_eq!(result, 485739);
    }

    #[test]
    fn test_part1_set() {
        let input = include_str!("../input/2020/day1.txt");
        let input = lines_of_ints_to_int_array(input);

        let result = part1_set(input.as_slice());

        assert_eq!(result, 485739);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/2020/day1.txt");
        let input = lines_of_ints_to_int_array(input);

        let result = part2(input.as_slice());

        assert_eq!(result, 161109702);
    }
}

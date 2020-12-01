use itertools::Itertools;

#[aoc_generator(day1)]
pub fn lines_of_ints_to_int_array(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input
        .into_iter()
        .permutations(2)
        .find(|pair| *pair.get(0).unwrap() + *pair.get(1).unwrap() == 2020)
        .map(|pair| *pair.get(0).unwrap() * *pair.get(1).unwrap())
        .expect("should be a number here")
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .into_iter()
        .permutations(3)
        .find(|pair| *pair.get(0).unwrap() + *pair.get(1).unwrap() + *pair.get(2).unwrap() == 2020)
        .map(|pair| *pair.get(0).unwrap() * *pair.get(1).unwrap() * *pair.get(2).unwrap())
        .expect("should be a number here")
}

#[cfg(test)]
mod tests {}

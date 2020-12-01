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
        // map from vector of 2 elements to a tuple2
        .map(|vec: Vec<&i32>| (**vec.get(0).unwrap(), **vec.get(1).unwrap()))
        // takes one element matching predicate, returns option
        .find(|(left, right): (&i32, &i32)| left + right == 2020)
        .map(|(left, right): (i32, i32)| left * right)
        // unwrap option
        .expect("should be a number here")
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .into_iter()
        .permutations(3)
        // map from vector of 3 elements to a tuple3
        .map(|vec| {
            (
                **vec.get(0).unwrap(),
                **vec.get(1).unwrap(),
                **vec.get(2).unwrap(),
            )
        })
        .find(|(left, middle, right)| left + middle + right == 2020)
        .map(|(left, middle, right)| left * middle * right)
        .expect("should be a number here")
}

#[cfg(test)]
mod tests {}

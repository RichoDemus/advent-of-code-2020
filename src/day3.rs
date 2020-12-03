#[aoc_generator(day3, part1, map)]
fn matrix_of_symbols_to_boolean_matrix(input: &str) -> Map {
    let asd = input
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect())
        .collect::<Vec<Vec<bool>>>();
    let width = asd.get(0).unwrap().len();
    Map {
        coordinates: asd,
        width,
    }
}

#[aoc_generator(day3, part2, map)]
fn matrix_of_symbols_to_boolean_matrix2(input: &str) -> Map {
    matrix_of_symbols_to_boolean_matrix(input)
}

#[derive(Debug)]
struct Map {
    coordinates: Vec<Vec<bool>>,
    width: usize,
}

impl Map {
    fn tree_at(&self, row: usize, mut column: usize) -> bool {
        while column >= self.width {
            column -= self.width;
        }
        *self.coordinates.get(row).unwrap().get(column).unwrap()
    }
}

#[aoc(day3, part1, map)]
fn part1(map: &Map) -> i32 {
    let right_step = 3;
    let down_step = 1;

    let mut column = 0;
    let mut row = 0;
    let mut trees = 0;

    while row < map.coordinates.len() {
        if map.tree_at(row, column) {
            trees += 1;
        }
        row += down_step;
        column += right_step;
    }

    trees
}

#[aoc(day3, part1, perf)]
fn part1_perf(input: &str) -> i32 {
    let right_step = 3;

    let mut column = 0;
    let mut trees = 0;
    for line in input.lines() {
        if line.chars().nth(column % line.len()).unwrap() == '#' {
            trees += 1;
        }

        column += right_step;
    }
    trees
}

#[aoc(day3, part1, perf_enumerate)]
fn part1_perf_enumerate(input: &str) -> i32 {
    let right_step = 3;
    let mut trees = 0;
    for (i, line) in input.lines().enumerate() {
        if line.chars().nth(i * right_step % line.len()).unwrap() == '#' {
            trees += 1;
        }
    }
    trees
}

#[aoc(day3, part1, perf_bytes)]
fn part1_perf_bytes(input: &[u8]) -> i32 {
    let mut desired_column = 0;
    let mut current_column = 0;
    let mut width = 0;
    let mut trees = 0;
    for byte in input {
        match byte {
            35 => {
                // # is 35
                if desired_column == current_column {
                    trees += 1;
                }
                current_column += 1;
            }
            46 => {
                // . is 46
                current_column += 1;
            }
            10 => {
                // 10 is \n
                if width == 0 {
                    width = current_column;
                }
                current_column = 0;
                desired_column += 3;
                if desired_column >= width {
                    desired_column -= width;
                }
            }
            c => panic!("Unrecognized char: {}", c),
        };
    }
    trees
}

#[aoc(day3, part1, perf_bytes_split)]
fn part1_perf_bytes_split(input: &[u8]) -> i32 {
    let right_step = 3;

    let mut column = 0;
    let mut trees = 0;
    for line in input.split(|c| c == &10) {
        if line.is_empty() {
            continue;
        }
        if line.get(column % line.len()).unwrap() == &35 {
            trees += 1;
        }
        column += right_step;
    }
    trees
}

#[aoc(day3, part1, perf_bytes_modulus)]
fn part1_perf_bytes_modulus(input: &[u8]) -> i32 {
    let mut desired_column = 0;
    let mut current_column = 0;
    let mut width = 1;
    let mut trees = 0;
    for byte in input {
        match byte {
            35 => {
                // # is 35
                if desired_column % width == current_column {
                    trees += 1;
                }
                current_column += 1;
            }
            46 => {
                // . is 46
                current_column += 1;
            }
            10 => {
                // 10 is \n
                if width == 1 {
                    width = current_column;
                }
                current_column = 0;
                desired_column += 3;
            }
            c => panic!("Unrecognized char: {}", c),
        };
    }
    trees
}

#[aoc(day3, part2, map)]
fn part2(map: &Map) -> usize {
    fn get_trees_for_slope(map: &Map, right_step: usize, down_step: usize) -> usize {
        let mut column = 0;
        let mut row = 0;
        let mut trees = 0;

        while row < map.coordinates.len() {
            if map.tree_at(row, column) {
                trees += 1;
            }
            row += down_step;
            column += right_step;
        }

        trees
    }

    get_trees_for_slope(map, 1, 1)
        * get_trees_for_slope(map, 3, 1)
        * get_trees_for_slope(map, 5, 1)
        * get_trees_for_slope(map, 7, 1)
        * get_trees_for_slope(map, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_at() {
        let map = Map {
            coordinates: vec![vec![true, false, false], vec![false, false, true]],
            width: 3,
        };

        assert_eq!(map.tree_at(0, 0), true, "0,0");
        assert_eq!(map.tree_at(0, 1), false, "0,1");
        assert_eq!(map.tree_at(0, 2), false, "0,2");
        assert_eq!(map.tree_at(0, 3), true, "0,3");
        assert_eq!(map.tree_at(1, 0), false, "1,0");
        assert_eq!(map.tree_at(1, 1), false, "1,1");
        assert_eq!(map.tree_at(1, 2), true, "1,2");
        assert_eq!(map.tree_at(1, 3), false, "1,3");
    }

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day3.txt");
        let input = matrix_of_symbols_to_boolean_matrix(input);

        let result = part1(&input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part1_perf() {
        let input = include_str!("../input/2020/day3.txt");

        let result = part1_perf(input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part1_perf_enumerate() {
        let input = include_str!("../input/2020/day3.txt");

        let result = part1_perf_enumerate(input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part1_perf_bytes() {
        let input = include_bytes!("../input/2020/day3.txt");

        let result = part1_perf_bytes(input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part1_perf_bytes_split() {
        let input = include_bytes!("../input/2020/day3.txt");

        let result = part1_perf_bytes_split(input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part1_perf_bytes_modulus() {
        let input = include_bytes!("../input/2020/day3.txt");

        let result = part1_perf_bytes_modulus(input);

        assert_eq!(result, 171)
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day3.txt");
        let input = matrix_of_symbols_to_boolean_matrix(input);

        let result = part2(&input);

        assert_eq!(result, 1206576000)
    }
}

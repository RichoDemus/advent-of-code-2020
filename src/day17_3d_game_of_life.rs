use std::cmp;
use std::collections::HashSet;

#[aoc(day17, part1)]
fn part1_3d_game_of_life(input: &str) -> usize {
    let mut space = parse(input);

    // print(&space);

    for _cycle in 0..6 {
        space = tick(&space);
        // println!("\nAfter cycle {}\n", cycle + 1);
        // print(&space);
    }

    space.len()
}

#[aoc(day17, part2)]
fn part2_4d_game_of_life(input: &str) -> usize {
    let mut space = hyper_parse(input);

    // hyper_print(&space);

    for _cycle in 0..6 {
        space = hyper_tick(&space);
        // println!("\nAfter cycle {}\n", cycle + 1);
        // hyper_print(&space);
    }

    space.len()
}

fn tick(space: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut next_generation = HashSet::new();
    let (z_min, z_max, y_min, y_max, x_min, x_max) = calc_boundary(space);

    // loop outside the boundaries
    for z in z_min - 1..=z_max + 1 {
        for y in y_min - 1..=y_max + 1 {
            for x in x_min - 1..=x_max + 1 {
                let coordinate = Coordinate { z, y, x };
                let neighbours = count_neighbours(&coordinate, space);
                let active = space.contains(&coordinate);
                let new_state = if active {
                    neighbours == 2 || neighbours == 3
                } else {
                    neighbours == 3
                };
                if new_state {
                    next_generation.insert(coordinate);
                }
            }
        }
    }

    next_generation
}

fn hyper_tick(space: &HashSet<HyperCoordinate>) -> HashSet<HyperCoordinate> {
    let mut next_generation = HashSet::new();
    let (w_min, w_max, z_min, z_max, y_min, y_max, x_min, x_max) = calc_hyper_boundary(space);

    // loop outside the boundaries
    for w in w_min - 1..=w_max + 1 {
        for z in z_min - 1..=z_max + 1 {
            for y in y_min - 1..=y_max + 1 {
                for x in x_min - 1..=x_max + 1 {
                    let coordinate = HyperCoordinate { w, z, y, x };
                    let neighbours = count_hyper_neighbours(&coordinate, space);
                    let active = space.contains(&coordinate);
                    let new_state = if active {
                        neighbours == 2 || neighbours == 3
                    } else {
                        neighbours == 3
                    };
                    if new_state {
                        next_generation.insert(coordinate);
                    }
                }
            }
        }
    }
    next_generation
}

fn count_neighbours(coordinate: &Coordinate, space: &HashSet<Coordinate>) -> u8 {
    let mut neighbours = 0;
    // println!("Coord : {} {} {}", coordinate.z,coordinate.y,coordinate.x);
    for z in coordinate.z - 1..=coordinate.z + 1 {
        for y in coordinate.y - 1..=coordinate.y + 1 {
            for x in coordinate.x - 1..=coordinate.x + 1 {
                if z == coordinate.z && y == coordinate.y && x == coordinate.x {
                    continue;
                }
                // println!("\tChecking: {} {} {}", z,y,x);
                if space.contains(&Coordinate { z, y, x }) {
                    neighbours += 1;
                }
            }
        }
    }
    neighbours
}

fn count_hyper_neighbours(coordinate: &HyperCoordinate, space: &HashSet<HyperCoordinate>) -> u8 {
    let mut neighbours = 0;
    // println!("Coord : {} {} {}", coordinate.z,coordinate.y,coordinate.x);
    for w in coordinate.w - 1..=coordinate.w + 1 {
        for z in coordinate.z - 1..=coordinate.z + 1 {
            for y in coordinate.y - 1..=coordinate.y + 1 {
                for x in coordinate.x - 1..=coordinate.x + 1 {
                    if z == coordinate.z
                        && y == coordinate.y
                        && x == coordinate.x
                        && w == coordinate.w
                    {
                        continue;
                    }
                    // println!("\tChecking: {} {} {}", z,y,x);
                    if space.contains(&HyperCoordinate { w, z, y, x }) {
                        neighbours += 1;
                    }
                }
            }
        }
    }
    neighbours
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    z: i64,
    y: i64,
    x: i64,
}
#[derive(Debug, Hash, Eq, PartialEq)]
struct HyperCoordinate {
    z: i64,
    y: i64,
    x: i64,
    w: i64,
}

fn calc_boundary(coordinates: &HashSet<Coordinate>) -> (i64, i64, i64, i64, i64, i64) {
    coordinates
        .iter()
        .fold(None, |left_option, right| match left_option {
            None => Some((right.z, right.z, right.y, right.y, right.x, right.x)),
            Some((z_min, z_max, y_min, y_max, x_min, x_max)) => Some((
                cmp::min(z_min, right.z),
                cmp::max(z_max, right.z),
                cmp::min(y_min, right.y),
                cmp::max(y_max, right.y),
                cmp::min(x_min, right.x),
                cmp::max(x_max, right.x),
            )),
        })
        .unwrap()
}

fn calc_hyper_boundary(
    coordinates: &HashSet<HyperCoordinate>,
) -> (i64, i64, i64, i64, i64, i64, i64, i64) {
    coordinates
        .iter()
        .fold(None, |left_option, right| match left_option {
            None => Some((
                right.w, right.w, right.z, right.z, right.y, right.y, right.x, right.x,
            )),
            Some((w_min, w_max, z_min, z_max, y_min, y_max, x_min, x_max)) => Some((
                cmp::min(w_min, right.w),
                cmp::max(w_max, right.w),
                cmp::min(z_min, right.z),
                cmp::max(z_max, right.z),
                cmp::min(y_min, right.y),
                cmp::max(y_max, right.y),
                cmp::min(x_min, right.x),
                cmp::max(x_max, right.x),
            )),
        })
        .unwrap()
}

#[allow(dead_code)]
fn print(coordinates: &HashSet<Coordinate>) {
    let (z_min, z_max, y_min, y_max, x_min, x_max) = calc_boundary(coordinates);

    for z in z_min..=z_max {
        println!("x={}", z);
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let cell = if coordinates.contains(&Coordinate { z, y, x }) {
                    '#'
                } else {
                    '.'
                };
                print!("{} ", cell);
            }
            println!()
        }
    }
}

#[allow(dead_code)]
fn hyper_print(coordinates: &HashSet<HyperCoordinate>) {
    let (w_min, w_max, z_min, z_max, y_min, y_max, x_min, x_max) = calc_hyper_boundary(coordinates);

    for w in w_min..=w_max {
        for z in z_min..=z_max {
            println!("x={}, w={}", z, w);
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let cell = if coordinates.contains(&HyperCoordinate { z, y, x, w }) {
                        '#'
                    } else {
                        '.'
                    };
                    print!("{} ", cell);
                }
                println!()
            }
        }
    }
}

fn parse(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, char)| (row, column, char))
        })
        .filter_map(|(row, column, char)| {
            if char == '#' {
                Some(Coordinate {
                    x: column as i64,
                    y: row as i64,
                    z: 0,
                })
            } else {
                None
            }
        })
        .collect::<HashSet<Coordinate>>()
}

fn hyper_parse(input: &str) -> HashSet<HyperCoordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, char)| (row, column, char))
        })
        .filter_map(|(row, column, char)| {
            if char == '#' {
                Some(HyperCoordinate {
                    x: column as i64,
                    y: row as i64,
                    z: 0,
                    w: 0,
                })
            } else {
                None
            }
        })
        .collect::<HashSet<HyperCoordinate>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day17.txt");
        assert_eq!(part1_3d_game_of_life(input), 286);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day17.txt");
        assert_eq!(part2_4d_game_of_life(input), 960);
    }

    #[test]
    fn test_part1_provided_example() {
        let input = ".#.
..#
###";

        let result = part1_3d_game_of_life(input);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part2_provided_example() {
        let input = ".#.
..#
###";

        let result = part2_4d_game_of_life(input);
        assert_eq!(result, 848);
    }
}

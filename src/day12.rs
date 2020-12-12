#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    part1_manhattan_distance(input).0
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    part2_manhattan_distance(input).distance
}

fn part1_manhattan_distance(input: &str) -> (i32, Direction) {
    let parsed = parse(input);
    let mut facing = Direction::East;
    let mut x = 0; // west <-> east
    let mut y = 0; // north <-> south

    for (direction, amount) in parsed {
        #[allow(clippy::match_same_arms)]
        match (direction, facing, amount) {
            (Direction::North, _, amount) => y -= amount,
            (Direction::South, _, amount) => y += amount,
            (Direction::East, _, amount) => x += amount,
            (Direction::West, _, amount) => x -= amount,
            (Direction::Forward, Direction::North, amount) => y -= amount,
            (Direction::Forward, Direction::South, amount) => y += amount,
            (Direction::Forward, Direction::East, amount) => x += amount,
            (Direction::Forward, Direction::West, amount) => x -= amount,
            (Direction::Right, curr_facing, degrees) => facing = turn(curr_facing, degrees),
            (Direction::Left, curr_facing, degrees) => facing = turn(curr_facing, 360 - degrees),

            (direction, facing, _) => panic!(
                "Unhandled command moving {:?} facing {:?}",
                direction, facing
            ),
        }
    }

    (x.abs() + y.abs(), facing)
}

fn turn(direction: Direction, degrees: i32) -> Direction {
    let current_direction_angle = match direction {
        Direction::North => 0,
        Direction::East => 90,
        Direction::South => 180,
        Direction::West => 270,
        other => panic!("weird direction: {:?}", other),
    };

    let mut new_direction = current_direction_angle + degrees;
    while new_direction >= 360 {
        new_direction -= 360;
    }

    match new_direction {
        0 => Direction::North,
        90 => Direction::East,
        180 => Direction::South,
        270 => Direction::West,
        other => panic!("weird direction: {:?}", other),
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}
fn parse(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_at(1);
            let direction = match direction {
                "N" => Direction::North,
                "S" => Direction::South,
                "E" => Direction::East,
                "W" => Direction::West,
                "L" => Direction::Left,
                "R" => Direction::Right,
                "F" => Direction::Forward,
                other => panic!("Unexpected Direction {:?}", other),
            };
            let amount = amount
                .parse()
                .unwrap_or_else(|_| panic!("Couldn't parse {:?}", amount));
            (direction, amount)
        })
        .collect()
}

#[allow(dead_code)]
struct PartTwoResponse {
    distance: i32,
    ship_x: i32,
    ship_y: i32,
    waypoint_x:i32,
    waypoint_y:i32,
}

fn part2_manhattan_distance(input: &str) -> PartTwoResponse {
    let parsed = parse(input);
    // let mut facing = Direction::East;
    let mut waypoint_x = 10; // west <-> east
    let mut waypoint_y = -1; // north <-> south
    let mut ship_x = 0;
    let mut ship_y = 0;

    for (direction, amount) in parsed {
        match direction {
            Direction::North => waypoint_y -= amount,
            Direction::South => waypoint_y += amount,
            Direction::East => waypoint_x += amount,
            Direction::West => waypoint_x -= amount,
            Direction::Left => {
                let (x, y) = rotate_left(waypoint_x, waypoint_y, amount);
                waypoint_y = y;
                waypoint_x = x;
            }
            Direction::Right => {
                let (x, y) = rotate_right(waypoint_x, waypoint_y, amount);
                waypoint_y = y;
                waypoint_x = x;
            }
            Direction::Forward => {
                ship_x += waypoint_x * amount;
                ship_y += waypoint_y * amount;
            }
        }
    }

    PartTwoResponse {
        distance: ship_x.abs() + ship_y.abs(),
        ship_x,
        ship_y,
        waypoint_x,
        waypoint_y,
    }
}

const fn rotate_right(mut x: i32, mut y: i32, mut degrees: i32) -> (i32, i32) {
    while degrees > 0 {
        let new_x = -y;
        let new_y = x;
        x = new_x;
        y = new_y;
        degrees -= 90;
    }
    (x, y)
}

const fn rotate_left(mut x: i32, mut y: i32, mut degrees: i32) -> (i32, i32) {
    while degrees > 0 {
        let new_x = y;
        let new_y = -x;
        x = new_x;
        y = new_y;
        degrees -= 90;
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day12.txt");
        assert_eq!(part1(input), 362);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day12.txt");
        assert_eq!(part2(input), 29895);
    }

    #[test]
    fn test_parse() {
        let input = "N10
S3
E7
W90
F11
L1
R2";
        let result = parse(input);

        assert_eq!(
            result,
            vec![
                (Direction::North, 10),
                (Direction::South, 3),
                (Direction::East, 7),
                (Direction::West, 90),
                (Direction::Forward, 11),
                (Direction::Left, 1),
                (Direction::Right, 2),
            ]
        );
    }

    #[test]
    fn test_with_given_example() {
        let input = "F10
N3
F7
R90
F11";

        let result = part1_manhattan_distance(input).0;

        assert_eq!(result, 25);
    }

    #[test]
    fn should_face_east() {
        let (_, direction) = part1_manhattan_distance("F1");
        assert_eq!(direction, Direction::East);
    }

    #[test]
    fn test_with_turn_left() {
        let (_, direction) = part1_manhattan_distance("L90");
        assert_eq!(direction, Direction::North);
    }

    #[test]
    fn test_with_turn_right() {
        let (_, direction) = part1_manhattan_distance("R90");
        assert_eq!(direction, Direction::South);
    }

    #[test]
    fn test_with_turn_180() {
        let (_, direction) = part1_manhattan_distance("R180");
        assert_eq!(direction, Direction::West, "turn Right 180");

        let (_, direction) = part1_manhattan_distance("L180");
        assert_eq!(direction, Direction::West, "turn Left 180");
    }

    #[test]
    fn test_with_given_example_part2() {
        let input = "F10
N3
F7
R90
F11";

        let result = part2_manhattan_distance(input).distance;

        assert_eq!(result, 286);
    }

    #[test]
    fn test_with_given_example_part2_modified() {
        let input = "F10
N3
F7
R90";

        let result =
            part2_manhattan_distance(input);

        assert_eq!(result.ship_x, 170);
        assert_eq!(result.ship_y, -38);

        assert_eq!(result.waypoint_x, 4, "check waypoint x");
        assert_eq!(result.waypoint_y, 10, "check waypoint y");
    }

    #[test]
    fn test_rotate_waypoint_left() {
        let result =
            part2_manhattan_distance("L90");

        assert_eq!(result.waypoint_x, -1, "check waypoint x");
        assert_eq!(result.waypoint_y, -10, "check waypoint y");

        let result =
            part2_manhattan_distance("L90\nL90");

        assert_eq!(result.waypoint_x, -10, "check waypoint x");
        assert_eq!(result.waypoint_y, 1, "check waypoint y");
    }
}

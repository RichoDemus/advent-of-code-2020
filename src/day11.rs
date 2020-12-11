use std::collections::HashMap;
use std::convert::TryFrom;

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let input = parse(input);
    part1_calc_final_occupied_seats(input)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let input = parse(input);
    part2_calc_final_occupied_seats(input)
}

fn part1_calc_final_occupied_seats(seats: HashMap<(i32, i32), Seat>) -> usize {
    let mut last_gen_seats = seats;

    loop {
        let new_generation =
            calculate_next_generation(&last_gen_seats, 4, &calculate_num_occupied_adjacent_seats);
        if new_generation == last_gen_seats {
            //done
            return new_generation
                .values()
                .into_iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count();
        }
        last_gen_seats = new_generation;
    }
}

fn calculate_next_generation(
    seats: &HashMap<(i32, i32), Seat>,
    seats_required_to_make_empty: usize,
    fn_adjacent_seats: &dyn Fn(&HashMap<(i32, i32), Seat>, i32, i32) -> usize,
) -> HashMap<(i32, i32), Seat> {
    let mut next_generation = HashMap::new();
    for ((row, column), seat) in seats {
        let num_adjacent_seats = fn_adjacent_seats(seats, *row, *column);

        let new_seat_status = if *seat == Seat::Empty && num_adjacent_seats == 0 {
            Seat::Occupied
        } else if *seat == Seat::Occupied && num_adjacent_seats >= seats_required_to_make_empty {
            Seat::Empty
        } else {
            *seat
        };
        next_generation.insert((*row, *column), new_seat_status);
    }
    next_generation
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

fn parse(input: &str) -> HashMap<(i32, i32), Seat> {
    let mut seats = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            let seat = match char {
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                '.' => Seat::Floor,
                other => panic!("Unexpcted char: {:?}", other),
            };
            seats.insert(
                (i32::try_from(row).unwrap(), i32::try_from(column).unwrap()),
                seat,
            );
        }
    }
    seats
}

fn calculate_num_occupied_adjacent_seats(
    seats: &HashMap<(i32, i32), Seat>,
    row: i32,
    column: i32,
) -> usize {
    let adjacents_offsets = vec![
        (row - 1, column - 1),
        (row - 1, column),
        (row - 1, column + 1),
        (row, column - 1),
        (row, column + 1),
        (row + 1, column - 1),
        (row + 1, column),
        (row + 1, column + 1),
    ];

    adjacents_offsets
        .into_iter()
        .filter(|(row, column)| {
            if let Some(seat) = seats.get(&(*row, *column)) {
                if let Seat::Occupied = seat {
                    return true;
                }
            }
            false
        })
        .count()
}

fn calculate_num_occupied_adjacent_seats_part2(
    seats: &HashMap<(i32, i32), Seat>,
    row: i32,
    column: i32,
) -> usize {
    let adjacents_offsets = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut occupied_seats_seen = 0;

    'direction: for (row_offset, column_offset) in adjacents_offsets {
        let mut r = row;
        let mut c = column;
        loop {
            r += row_offset;
            c += column_offset;

            let maybe_seat = seats.get(&(r, c));
            match maybe_seat {
                Some(Seat::Occupied) => {
                    // We saw a seat
                    occupied_seats_seen += 1;
                    continue 'direction;
                }
                Some(Seat::Empty) | None => {
                    // We saw something that wasn't an occupied seat or we reached Out of bounds
                    continue 'direction;
                }
                _ => {
                    // we saw something that wasn't an occupied seat
                }
            }
        }
    }
    occupied_seats_seen
}

#[allow(dead_code)]
fn print(seats: &HashMap<(i32, i32), Seat>) {
    let mut row = 0;
    let mut column;
    // loop rows
    loop {
        column = 0;
        // loop columns
        'column: loop {
            let cell = seats.get(&(row, column));
            match cell {
                None => {
                    if column == 0 {
                        // we just switched row so we're done
                        return;
                    } else {
                        // we're probably just at the end of the column
                        row += 1;
                        println!();
                        break 'column;
                    }
                }
                Some(seat) => {
                    print!(
                        "{} ",
                        match seat {
                            Seat::Empty => 'L',
                            Seat::Occupied => '#',
                            Seat::Floor => '.',
                        }
                    )
                }
            }
            column += 1;
        }
    }
}

fn part2_calc_final_occupied_seats(seats: HashMap<(i32, i32), Seat>) -> usize {
    let mut last_gen_seats = seats;

    loop {
        let new_generation = calculate_next_generation(
            &last_gen_seats,
            5,
            &calculate_num_occupied_adjacent_seats_part2,
        );
        if new_generation == last_gen_seats {
            //done
            return new_generation
                .values()
                .into_iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count();
        }
        last_gen_seats = new_generation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day11.txt");
        assert_eq!(part1(input), 2441);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day11.txt");
        assert_eq!(part2(input), 2190);
    }

    #[test]
    fn test_provided_example() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let parsed = parse(input);

        let occupied_seats = part1_calc_final_occupied_seats(parsed);

        assert_eq!(occupied_seats, 37);
    }

    #[test]
    fn test_provided_example_part2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let parsed = parse(input);

        let occupied_seats = part2_calc_final_occupied_seats(parsed);

        assert_eq!(occupied_seats, 26)
    }
}

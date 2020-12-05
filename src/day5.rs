use itertools::Itertools;

#[aoc(day5, part1)]
fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|boarding_pass| get_seat_id(boarding_pass))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u16 {
    let mut ids = input
        .lines()
        .map(|boarding_pass| get_seat_id(boarding_pass))
        .sorted();

    let mut prev = ids.next().unwrap();

    for id in ids {
        if id - 1 != prev {
            return prev + 1;
        }
        prev = id;
    }

    panic!("this shouldn't happen")
}

fn calc_middle(strip: &str, seats_or_rows: u16) -> u16 {
    let mut low = 0;
    let mut high = seats_or_rows - 1;

    for char in strip.chars() {
        let length_left = high - low;
        match char {
            'F' | 'L' => {
                high -= (length_left + 1) / 2;
            }
            'B' | 'R' => {
                low += (length_left + 1) / 2;
            }
            other => panic!("Unrecognized strip char: {:?}", other),
        }
        if low == high {
            return high;
        }
    }

    panic!("this shouldn't happen")
}

fn get_seat_id(boarding_pass: &str) -> u16 {
    let (row_strip, seat_strip) = boarding_pass.split_at(7);
    let row = calc_middle(row_strip, 128);
    let column = calc_middle(seat_strip, 8);
    row * 8 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_given_input() {
        let strip = "FBFBBFFRLR";
        let (row_strip, seat_strip) = strip.split_at(7);
        let result = calc_middle(row_strip, 128);
        assert_eq!(result, 44);

        println!();
        let result = calc_middle(seat_strip, 8);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_some_passes() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day5.txt");
        assert_eq!(part1(input), 906);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day5.txt");
        assert_eq!(part2(input), 519);
    }
}

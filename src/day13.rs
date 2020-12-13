use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::string::ParseError;

#[aoc(day13, part1)]
fn part1(input: &str) -> u32 {
    let (buss, minutes) = get_id_and_minutes(input);
    buss * minutes
}

#[aoc(day13, part2)]
fn part2(input: &str) -> i128 {
    part2_calc_buss_offsets_using_chinese_remainder_theorem(input)
}

fn get_id_and_minutes(input: &str) -> (u32, u32) {
    let (my_time, busses): (u32, Vec<Buss>) = parse(input);

    let multiplied_until_after_my_time = busses.into_iter().filter_map(|buss| match buss {
        Buss::Numeric(b) => Some({
            let mut new_buss = 0;
            while new_buss < my_time {
                new_buss += b.id;
            }
            (b.id, new_buss)
        }),
        Buss::X => None,
    });

    let (buss, ealiest_time) = multiplied_until_after_my_time
        .fold1(
            |(left_buss, left_earliest_arrival), (right_buss, right_earliest_arrival)| {
                if left_earliest_arrival < right_earliest_arrival {
                    (left_buss, left_earliest_arrival)
                } else {
                    (right_buss, right_earliest_arrival)
                }
            },
        )
        .unwrap();

    let minutes_to_wait = ealiest_time - my_time;
    (buss, minutes_to_wait)
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Buss {
    Numeric(NumericBuss),
    X,
}

impl fmt::Debug for Buss {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Self::Numeric(b) => b.departure_time.to_string(),
                Self::X => String::from("x"),
            }
        ))
    }
}

impl Buss {
    const fn from_id(id: u32) -> Self {
        Self::Numeric(NumericBuss {
            id,
            departure_time: id,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct NumericBuss {
    id: u32,
    departure_time: u32,
}

impl FromStr for Buss {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            other => {
                let as_int: u32 = other.parse().expect("asd");
                Ok(Self::from_id(as_int))
            }
        }
    }
}

fn parse(input: &str) -> (u32, Vec<Buss>) {
    let mut iter = input.lines();
    let my_time: u32 = iter.next().unwrap().parse().unwrap();

    let busses = iter.next().unwrap();
    let busses = busses
        .split(',')
        .map(|buss| buss.parse::<Buss>().unwrap())
        .collect::<Vec<_>>();

    (my_time, busses)
}

fn part2_calc_buss_offsets_using_chinese_remainder_theorem(input: &str) -> i128 {
    let _ = input.lines().next();
    let ids: Vec<(usize, i128)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, v)| {
            if v == "x" {
                None
            } else {
                Some((i, v.parse::<i128>().unwrap()))
            }
        })
        .collect();

    let modulii: Vec<i128> = ids.iter().map(|a| a.1).collect();
    let residues: Vec<i128> = ids.iter().map(|a| a.0 as i128).collect();

    let product = modulii.iter().product::<i128>();
    let remainder = chinese_remainder(&residues, &modulii).unwrap();

    product - remainder
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[allow(clippy::clippy::many_single_char_names)]
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day13.txt");
        assert_eq!(part1(input), 2845);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day13.txt");
        assert_eq!(part2(input), 487905974205117);
    }

    #[test]
    fn test_parse() {
        let input = "939
7,13,x,x,59,x,31,19";

        let (my_time, busses): (u32, Vec<Buss>) = parse(input);

        assert_eq!(my_time, 939);
        assert_eq!(
            busses,
            vec![
                Buss::from_id(7),
                Buss::from_id(13),
                Buss::X,
                Buss::X,
                Buss::from_id(59),
                Buss::X,
                Buss::from_id(31),
                Buss::from_id(19),
            ]
        );
    }

    #[test]
    fn test_parse_buss_id() {
        assert_eq!("x".parse::<Buss>().unwrap(), Buss::X);
        assert_eq!("13".parse::<Buss>().unwrap(), Buss::from_id(13));
    }

    #[test]
    fn test_with_provided_data() {
        let input = "939
7,13,x,x,59,x,31,19";

        let (bus_id, minutes) = get_id_and_minutes(input);

        assert_eq!(bus_id, 59);
        assert_eq!(minutes, 5);
    }

    #[test]
    fn day13_part2_provided_data() {
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n17,x,13,19"),
            3417
        );
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n7,13,x,x,59,x,31,19"),
            1068781
        );
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n67,7,59,61"),
            754018
        );
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n67,x,7,59,61"),
            779210
        );
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n67,7,x,59,61"),
            1261476
        );
        assert_eq!(
            part2_calc_buss_offsets_using_chinese_remainder_theorem("0\n1789,37,47,1889"),
            1202161486
        );
    }

    #[test]
    fn test_chinese_remainder() {
        let modulii = [3, 5, 7];
        let residues = [2, 3, 2];

        let result = chinese_remainder(&residues, &modulii);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 23);
    }
}

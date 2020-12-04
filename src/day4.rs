#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let mut valid_passports = 0;
    for passport in input.split("\n\n") {
        if !has_all_fields(passport) {
            continue;
        }
        valid_passports += 1;
    }
    valid_passports
}

fn has_all_fields(passport: &str) -> bool {
    passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid")
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let mut valids = 0;

    let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let valid_hair_color_digits = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c", "d", "e", "f",
    ];

    'passport: for passport in input.split("\n\n") {
        if passport.is_empty() {
            continue 'passport;
        }
        if !has_all_fields(passport) {
            continue 'passport;
        }
        let pairs = passport.split_ascii_whitespace();

        for pair in pairs {
            let mut split = pair.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            match key {
                "byr" => {
                    let year: i32 = value.parse().unwrap_or(-1);
                    if year < 1920 || year > 2002 || value.len() != 4 {
                        continue 'passport;
                    }
                }
                "iyr" => {
                    let year: i32 = value.parse().unwrap_or(-1);
                    if year < 2010 || year > 2020 || value.len() != 4 {
                        continue 'passport;
                    }
                }
                "eyr" => {
                    let year: i32 = value.parse().unwrap_or(-1);
                    if year < 2020 || year > 2030 || value.len() != 4 {
                        continue 'passport;
                    }
                }
                "hgt" => {
                    if value.ends_with("cm") {
                        let height_str = value.split("cm").next().unwrap();
                        let height: i32 = height_str.parse().unwrap();
                        if height < 150 || height > 193 || height_str.len() != 3 {
                            continue 'passport;
                        }
                    } else if value.ends_with("in") {
                        let height_str = value.split("in").next().unwrap();
                        let height: i32 = height_str.parse().unwrap();
                        if height < 59 || height > 193 || height_str.len() != 2 {
                            continue 'passport;
                        }
                    } else {
                        continue 'passport;
                    }
                }
                "hcl" => {
                    let mut iter = value.chars();
                    let first = iter.next().unwrap();
                    if first != '#' {
                        continue 'passport;
                    }
                    for digit in iter {
                        if !valid_hair_color_digits.contains(&digit.to_string().as_str()) {
                            continue 'passport;
                        }
                    }
                }
                "ecl" => {
                    if !(valid_eye_colors.contains(&value.trim())) {
                        continue 'passport;
                    }
                }
                "pid" => {
                    let number: i32 = value.parse().unwrap_or(-1);
                    if value.len() != 9 || number == -1 {
                        continue 'passport;
                    }
                }
                "cid" => {}
                key => panic!(format!("Unhandled key {:?}", key)),
            }
        }
        valids += 1;
    }

    valids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = include_str!("../input/2020/day4.txt");

        let result = part1(input);
        assert_eq!(result, 239)
    }

    #[test]
    fn validate_part2() {
        let input = include_str!("../input/2020/day4.txt");

        let result = part2(input);
        assert_eq!(result, 188)
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let mut input = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let my_device_voltage = input.iter().max().unwrap() + 3;
    let (one_differences, three_differences) =
        calculate_differences(&mut input, my_device_voltage).unwrap();
    one_differences * three_differences
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    solve_part2(input)
}

fn solve_part2(mut numbers: Vec<u32>) -> u64 {
    numbers.push(0);
    numbers.sort_unstable();

    let mut paths = vec![0; numbers.len()];

    for (i, number) in numbers.iter().enumerate() {
        if i == 0 {
            paths[i] = 1;
            continue;
        }

        let mut current_paths = 0;
        if let Some(result) = i.checked_sub(1) {
            if let Some(one) = numbers.get(result) {
                if number - one < 4 {
                    current_paths += paths[i - 1];
                }
            }
        }
        if let Some(result) = i.checked_sub(2) {
            if let Some(two) = numbers.get(result) {
                if number - two < 4 {
                    current_paths += paths[i - 2];
                }
            }
        }
        if let Some(result) = i.checked_sub(3) {
            if let Some(three) = numbers.get(result) {
                if number - three < 4 {
                    current_paths += paths[i - 3];
                }
            }
        }
        paths[i] = current_paths as u64;
    }

    *paths.last().unwrap()
}

fn calculate_differences(input: &mut [u32], my_device_voltage: u32) -> Option<(u32, u32)> {
    input.sort_unstable();

    let mut one_differences = 0;
    let mut three_differences = 0;

    let copy = input.to_owned();
    let mut previous = copy
        .iter()
        .min()
        .unwrap_or_else(|| panic!("can't get min of {:?}", copy));

    for voltage in input {
        match *voltage - previous {
            0 => (),
            1 => {
                one_differences += 1;
            }
            2 => {}
            3 => {
                three_differences += 1;
            }
            other => {
                println!("\tDifference: {} - {} = {}", voltage, previous, other);
                return None;
            }
        }
        previous = voltage;
    }

    match my_device_voltage - previous {
        1 => {
            one_differences += 1;
        }
        2 => {}
        3 => {
            three_differences += 1;
        }
        other => {
            println!(
                "\tDifference: {} - {} = {}",
                my_device_voltage, previous, other
            );
            return None;
        }
    }

    Some((one_differences, three_differences))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2020/day10.txt");
        assert_eq!(part1(input), 1953);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2020/day10.txt");
        assert_eq!(part2(input), 3543369523456);
    }
}

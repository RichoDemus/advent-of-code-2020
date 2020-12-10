use std::cmp;
use std::collections::VecDeque;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();
    find_first_outlier(&input, 25)
}

fn find_first_outlier(input: &[usize], preamble_size: usize) -> usize {
    let mut buffer = VecDeque::with_capacity(3);
    for (i, xmas) in input.iter().enumerate() {
        assert!(xmas >= &0);
        if i < preamble_size {
            buffer.push_front(*xmas);
            buffer.truncate(preamble_size);
        } else {
            if !check_sum(&buffer, xmas) {
                return *xmas;
            }
            buffer.push_front(*xmas);
            buffer.truncate(preamble_size);
        }
    }

    // println!("buffer: {:?}", buffer);

    panic!()
}

fn check_sum(buffer: &VecDeque<usize>, xmas: &usize) -> bool {
    for buff in buffer {
        if buff > xmas {
            // wont be able to find a sum when operands > sum
            continue;
        }
        if let Some(diff) = xmas.checked_sub(*buff) {
            if buffer.contains(&diff) {
                // println!("{} + {} = {}", buff, diff,xmas );
                return true;
            } else {
                // println!("buffer {:?} does not contain {}, needed for {} + x = {}", buffer, diff, buff,xmas);
                // return *xmas;
            }
        } else {
            // could subtract
            // panic!("Subtraction failed: {} - {}", xmas, buff);
        }
    }
    false
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let xmas = 22477624;
    let input = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();
    find_smallest_and_largest_from_cont_set(&input, xmas)
}

fn find_smallest_and_largest_from_cont_set(buffer: &[usize], sum: usize) -> usize {
    let mut current_sum: usize;
    let mut smallest: usize;
    let mut largest: usize;
    for (i, start) in buffer.iter().enumerate() {
        current_sum = *start;
        smallest = current_sum;
        largest = current_sum;
        assert_ne!(current_sum, sum);
        for j in i + 1..buffer.len() {
            let next = buffer.get(j).unwrap();
            smallest = cmp::min(smallest, *next);
            largest = cmp::max(largest, *next);
            current_sum += next;
            if current_sum == sum {
                return smallest + largest;
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn it_works() {
        let mut preamble = VecDeque::with_capacity(3);
        for i in 1..10 {
            preamble.push_front(i);
            preamble.truncate(3);
            println!("numbers: {:?}", preamble);
        }
    }

    #[test]
    fn test_with_given_data() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let result = find_first_outlier(&input, 5);
        assert_eq!(result, 127);
    }

    #[test]
    fn part2_with_given_data() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let result = find_smallest_and_largest_from_cont_set(&input, 127);
        assert_eq!(result, 62);
    }
}

use std::fmt::Debug;
use std::str::FromStr;

///
/// Helper method to split and parse a string based on a delimiter
/// example
/// ```
/// # use advent_of_code_2020::util::str_split;
///
/// let (left, right):(u32,String) = str_split("10::hello","::").unwrap();
/// assert_eq!(left, 10);
/// assert_eq!(right,"hello");
/// ```
pub fn str_split<L: FromStr, R: FromStr>(input: &str, delimiter: &str) -> Option<(L, R)>
where
    <L as FromStr>::Err: Debug,
    <R as FromStr>::Err: Debug,
{
    let mut iter = input.split(delimiter);
    let left = iter.next();
    let right = iter.next();

    match (left, right) {
        (Some(left), Some(right)) => {
            let left: L = left.parse().expect("couldn't unwrap left");
            let right: R = right.parse().expect("couldn't unwrap right");
            Some((left, right))
        }
        _ => None,
    }
}

///
/// Helper method to add a signed integer to a usize
/// example
/// ```
/// # use advent_of_code_2020::util::add;
/// assert_eq!(add(10_usize, -2_i32), Some(8));
/// assert_eq!(add(10_usize, 2_i32), Some(12));
/// ```
#[allow(clippy::cast_sign_loss)]
pub const fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_split() {
        let (left, right): (String, String) = str_split("left:right", ":").unwrap();
        assert_eq!(left, "left");
        assert_eq!(right, "right");
    }

    #[test]
    fn test_str_split_generics() {
        let (left, right): (i32, char) = str_split("10:A", ":").unwrap();
        assert_eq!(left, 10);
        assert_eq!(right, 'A');
    }
}

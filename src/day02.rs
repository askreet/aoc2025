use std::ops::RangeInclusive;

use crate::shared::*;

pub struct Day02;

impl Solution for Day02 {
    fn part1(&self) -> Result<String> {
        let input = input(2)?;

        let result = solve(parse_ranges(&input)?, is_invalid_id_p1);

        return Ok(result.to_string());
    }

    fn part2(&self) -> Result<String> {
        let input = input(2)?;

        let result = solve(parse_ranges(&input)?, is_invalid_id_p2);

        return Ok(result.to_string());
    }
}

fn solve(ranges: Vec<RangeInclusive<usize>>, invalid_fn: fn(usize) -> bool) -> usize {
    let mut sum = 0;

    for range in ranges {
        for value in range {
            if invalid_fn(value) {
                sum += value;
            }
        }
    }

    return sum;
}

fn is_invalid_id_p1(n: usize) -> bool {
    let digits = ndigits(n);

    if digits % 2 == 1 {
        return false;
    }

    let power = 10usize.pow((digits / 2) as u32 as u32);
    let lower_half = n % power;
    let upper_half = (n - lower_half) / power;

    return upper_half == lower_half;
}

fn is_invalid_id_p2(n: usize) -> bool {
    let digits = Digits::of(n);
    'next_window: for window_size in 1..=(digits.len / 2) {
        // Only look at evenly distributed windows (i.e., where window size is a valid denominator
        // of the number of digits.)
        if digits.len % window_size != 0 {
            continue;
        }

        let n_windows = (digits.len / window_size) as usize;
        let window_length = digits.len as usize / n_windows;

        for digit_idx in 0..window_length {
            let digit = digits.ds[digit_idx];

            for window in 1..n_windows {
                let comparison_digit = digits.ds[(window * window_length) + digit_idx];

                if digit != comparison_digit {
                    continue 'next_window;
                }
            }
        }

        return true;
    }

    return false;
}

#[test]
fn test_is_invalid_id_p2() {
    let invalid = [
        11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
        2121212121,
    ];
    for n in invalid {
        assert_eq!(true, is_invalid_id_p2(n), "{n} should be an invalid id");
    }

    let valid = [0, 9, 981278309, 82173, 781297, 123, 43210, 100000000001];
    for n in valid {
        assert_eq!(false, is_invalid_id_p2(n), "{n} should be a valid id");
    }
}

fn parse_ranges(s: &str) -> Result<Vec<RangeInclusive<usize>>> {
    let mut result = Vec::new();

    for range in s.split(",") {
        let midpoint = range.find("-");
        if midpoint.is_none() {
            return err(&format!("invalid range: {range}"));
        }

        let (l, r) = range.split_at(midpoint.unwrap());
        let left_value: usize = match l.parse() {
            Err(_) => return err(&format!("invalid left hand value in range '{range}'")),
            Ok(v) => v,
        };
        let right_value: usize = match r[1..].parse() {
            Err(_) => return err(&format!("invalid right hand value in range '{range}'")),
            Ok(v) => v,
        };
        result.push(left_value..=right_value);
    }

    return Ok(result);
}

#[test]
fn test_parse_ranges() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(
        parse_ranges(input).expect("should not return error"),
        vec![
            11..=22,
            95..=115,
            998..=1012,
            1188511880..=1188511890,
            222220..=222224,
            1698522..=1698528,
            446443..=446449,
            38593856..=38593862,
            565653..=565659,
            824824821..=824824827,
            2121212118..=2121212124
        ]
    )
}

#[test]
fn test_example_part1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(
        1227775554,
        solve(parse_ranges(input).unwrap(), is_invalid_id_p1)
    );
}

#[test]
fn test_example_part2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(
        4174379265,
        solve(parse_ranges(input).unwrap(), is_invalid_id_p2)
    );
}

use crate::shared::*;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self) -> Result<String> {
        let mut sum = 0;

        for line in input_lines(3)? {
            let b = Bank::of(line?.trim())?;
            sum += best_joltage_rating(&b, 2);
        }

        return Ok(sum.to_string());
    }

    fn part2(&self) -> Result<String> {
        let mut sum = 0;

        for line in input_lines(3)? {
            let b = Bank::of(line?.trim())?;
            sum += best_joltage_rating(&b, 12);
        }

        return Ok(sum.to_string());
    }
}

struct Bank {
    values: Vec<u8>,
}

impl Bank {
    fn of(s: &str) -> Result<Bank> {
        let mut values = Vec::with_capacity(s.len());

        for c in s.bytes() {
            if (c & 0b00110000 != 0b00110000) || (c & 0b00001111 > 9) {
                return err(&format!("invalid character '{c}'"));
            }
            values.push(c & 0b00001111)
        }

        return Ok(Bank { values });
    }
}

#[test]
fn test_bank_of() {
    let b = Bank::of("987654321111111").expect("should not error");
    assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], b.values);
}

fn best_joltage_rating(b: &Bank, digits: usize) -> usize {
    assert!(digits > 0);
    assert!(digits <= b.values.len());

    // What is the earliest digit we can currently select?
    let mut start_idx = 0usize;

    // And the latest?
    let mut end_idx = b.values.len() - digits;

    let mut joltage = 0;

    for i in 0..digits {
        let selected_idx = start_idx + highest_leftmost_position(&b.values[start_idx..=end_idx]);
        start_idx = selected_idx + 1;

        let power = 10usize.pow(digits as u32 - i as u32 - 1);
        joltage += b.values[selected_idx] as usize * power;

        end_idx += 1;
    }

    return joltage;
}

fn highest_leftmost_position(values: &[u8]) -> usize {
    let mut best = (0, 0); /* idx, value */

    for i in 0..values.len() {
        if values[i] > best.1 {
            best = (i, values[i]);
        }
    }

    return best.0;
}

#[test]
fn test_best_joltage_rating() {
    let test_cases = [
        ("987654321111111", 98),
        ("811111111111119", 89),
        ("234234234234278", 78),
        ("818181911112111", 92),
    ];

    for (bank, expected_joltage) in test_cases {
        let b = Bank::of(bank).unwrap();
        assert_eq!(expected_joltage, best_joltage_rating(&b, 2))
    }
}

#[test]
fn test_best_joltage_rating_p2() {
    let test_cases = [
        ("987654321111111", 987654321111),
        ("811111111111119", 811111111119),
        ("234234234234278", 434234234278),
        ("818181911112111", 888911112111),
    ];

    for (bank, expected_joltage) in test_cases {
        let b = Bank::of(bank).unwrap();
        assert_eq!(expected_joltage, best_joltage_rating(&b, 12))
    }
}

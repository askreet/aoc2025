use crate::shared::*;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self) -> Result<String> {
        let mut dial = Dial::new();

        for line in input_lines(1)? {
            _ = dial.adjust(line?.trim())?
        }

        return Ok(dial.final_zeroes.to_string());
    }

    fn part2(&self) -> Result<String> {
        let mut dial = Dial::new();

        for line in input_lines(1)? {
            _ = dial.adjust(line?.trim())?
        }

        return Ok(dial.all_zeroes.to_string());
    }
}

struct Dial {
    value: i16,
    final_zeroes: u16,
    all_zeroes: u16,
}

impl Dial {
    fn new() -> Dial {
        Dial {
            value: 50,
            final_zeroes: 0,
            all_zeroes: 0,
        }
    }

    fn adjust(&mut self, v: &str) -> Result<i16> {
        let step = match v.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => return err("invalid direction"),
        };

        let count = v[1..].parse::<i16>()?;

        for _ in 0..count {
            self.value += step;
            if self.value == -1 {
                self.value = 99;
            }
            if self.value == 100 {
                self.value = 0;
            }
            if self.value == 0 {
                self.all_zeroes += 1;
            }
        }

        if self.value == 0 {
            self.final_zeroes += 1;
        }

        Ok(self.value)
    }
}

#[test]
fn test_example() {
    let mut dial = Dial::new();

    #[rustfmt::skip]
    let tests = [
        /* adjustment,  new value,  total final zeroes,  total all zeroes */
        ("L68",         82,         0,                   1),
        ("L30",         52,         0,                   1),
        ("R48",         0,          1,                   2),
        ("L5",          95,         1,                   2),
        ("R60",         55,         1,                   3),
        ("L55",         0,          2,                   4),
        ("L1",          99,         2,                   4),
        ("L99",         0,          3,                   5),
        ("R14",         14,         3,                   5),
        ("L82",         32,         3,                   6),
    ];

    assert_eq!(50, dial.value);
    for (adjustment, new_value, final_zeroes, all_zeroes) in tests {
        let result = dial.adjust(adjustment);
        assert_eq!(Ok(new_value), result);
        assert_eq!(
            final_zeroes, dial.final_zeroes,
            "after adjustment {adjustment} final zeroes should be {final_zeroes}, but was {}",
            dial.final_zeroes
        );
        assert_eq!(
            all_zeroes, dial.all_zeroes,
            "after adjustment {adjustment} all zeroes should be {all_zeroes}, but was {}",
            dial.all_zeroes
        );
    }
}

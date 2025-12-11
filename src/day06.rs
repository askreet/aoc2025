use crate::shared::*;

pub struct Day06;

impl Solution for Day06 {
    fn part1(&self) -> Result<String> {
        let input = input(6)?;

        Ok(part1(&input)?.to_string())
    }

    fn part2(&self) -> Result<String> {
        let input = input(6)?;

        Ok(part2(&input)?.to_string())
    }
}

fn part1(input: &str) -> Result<u64> {
    let lines = input.lines();

    let mut sum = 0;

    let mut operhand_iterators: Vec<_> = lines.map(|l| l.split_whitespace()).collect();

    let mut operators = operhand_iterators.pop().unwrap();

    'out: loop {
        let mut values: Vec<u64> = Vec::new();
        for i in &mut operhand_iterators {
            match i.next() {
                Some(v) => values.push(v.parse()?),
                None => break 'out,
            }
        }

        let this_result = match operators.next().unwrap() {
            "+" => values.into_iter().reduce(std::ops::Add::add).unwrap(),
            "-" => values.into_iter().reduce(std::ops::Sub::sub).unwrap(),
            "*" => values.into_iter().reduce(std::ops::Mul::mul).unwrap(),
            "/" => values.into_iter().reduce(std::ops::Div::div).unwrap(),
            v => return err(&format!("invalid operator: {v}")),
        };

        sum += this_result;
    }

    return Ok(sum);
}

#[test]
fn test_part1() {
    let result = part1(
        " 123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
",
    );

    assert_eq!(4277556, result.unwrap());
}

const SPACE: u8 = ' ' as u8;
const PLUS: u8 = '+' as u8;
const MULT: u8 = '*' as u8;

fn part2(input: &str) -> Result<u64> {
    let cg = CharGrid::from_str(input)?;

    let mut this_sum = 0;
    let mut this_product = 1;

    let mut sum: u64 = 0;

    for x in (0..=cg.x_max()).rev() {
        let mut this_operand: u64 = 0;

        for y in 0..=cg.y_max() - 1 {
            let c = cg.at(x, y) as u8;
            match c {
                0b0011_0000..=0b0011_1001 => {
                    this_operand *= 10;
                    this_operand += (c & 0b0100_1111) as u64;
                }
                SPACE => {}
                _ => return err(&format!("unexpected operand character '{}'", cg.at(x, y))),
            }
        }

        if this_operand > 0 {
            this_sum += this_operand;
            this_product *= this_operand;
            this_operand = 0;
        }

        match cg.at(x, cg.y_max()) {
            ' ' => {}
            '+' => {
                sum += this_sum;
                this_sum = 0;
                this_product = 1;
            }
            '*' => {
                sum += this_product;
                this_sum = 0;
                this_product = 1;
            }
            _ => {
                return err(&format!(
                    "unexpected operator character '{}'",
                    cg.at(x, cg.y_max())
                ));
            }
        }
    }

    return Ok(sum);
}

#[test]
fn test_part2() {
    let result = part2(concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    ));

    assert_eq!(3263827, result.unwrap());
}

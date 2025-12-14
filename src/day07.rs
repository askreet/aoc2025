use crate::shared::*;

pub struct Day07;

impl Solution for Day07 {
    fn part1(&self) -> Result<String> {
        let input = input(7)?;

        Ok(part1(&input)?.to_string())
    }

    fn part2(&self) -> Result<String> {
        let input = input(7)?;

        Ok(part2(&input)?.to_string())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut mg: MetaGrid<usize> = MetaGrid::from_str(input)?;

    for y in 0..mg.height() {
        for x in 0..mg.width() {
            let here = Position::at(x, y);

            if let Some((c, _)) = mg.at_checked(here + UP) {
                if c == 'S' || c == '|' {
                    match mg.at(here) {
                        ('.', _) => mg.set(here, '|'),
                        ('^', _) => {
                            mg.set_checked(here + LEFT, '|');
                            mg.set_checked(here + RIGHT, '|');
                            mg.set_meta(here, 1);
                        }
                        ('|', _) => {}
                        (c, _) => return err(&format!("unexpected character in part1 walk '{c}'")),
                    }
                }
            }
        }
    }

    return Ok(mg.sum_meta());
}

#[test]
fn test_part1() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    assert_eq!(21, part1(input).unwrap());
}

fn part2(input: &str) -> Result<usize> {
    let mut mg: MetaGrid<usize> = MetaGrid::from_str(input)?;

    mg.set_meta(mg.find_one('S')?, 1);

    for y in 1..mg.height() {
        for x in 0..mg.width() {
            let here = Position::at(x, y);

            let (_, timelines) = mg.at_owned(here + UP);
            if timelines > 0 {
                match mg.at(here) {
                    ('.', _) => mg.inc_meta(here, timelines),
                    ('^', _) => {
                        mg.inc_meta(here + LEFT, timelines);
                        mg.inc_meta(here + RIGHT, timelines);
                    }
                    ('|', _) => {}
                    (c, _) => return err(&format!("unexpected character in part1 walk '{c}'")),
                }
            }
        }
    }

    let mut total = 0;
    for x in 0..mg.width() {
        total += *mg.at(Position::at(x, mg.y_max())).1;
    }

    return Ok(total);
}

#[test]
fn test_part2() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    assert_eq!(40, part2(input).unwrap(),);
}

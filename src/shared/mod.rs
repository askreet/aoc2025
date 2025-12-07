#![allow(unused)]
mod char_grid;
mod meta_grid;

mod readers;
pub use readers::*;

use std::fmt::Formatter;
use std::num::ParseIntError;
use std::ops::Add;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(string: &str) -> Error {
        Error {
            msg: string.to_owned(),
        }
    }
}

pub fn err<T>(s: &str) -> Result<T> {
    Err(Error::new(s))
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            msg: format!("failed to parse int: {}", value.to_string()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            msg: format!("i/o error: {}", value.to_string()),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error {
            msg: format!("regex error: {}", value.to_string()),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error { msg: value }
    }
}

pub trait Solution {
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Direction(i8, i8);

pub const LEFT: Direction = Direction(-1, 0);
pub const RIGHT: Direction = Direction(1, 0);
pub const UP: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(0, 1);

impl Direction {
    pub fn of(x: i8, y: i8) -> Direction {
        Direction(x, y)
    }

    pub fn clockwise(&self) -> Direction {
        match self {
            Direction(0, -1) => RIGHT,
            Direction(1, 0) => DOWN,
            Direction(0, 1) => LEFT,
            Direction(-1, 0) => UP,
            _ => panic!("cannot rotate non-cardinal Direction"),
        }
    }

    pub fn anticlockwise(&self) -> Direction {
        match self {
            Direction(0, -1) => LEFT,
            Direction(-1, 0) => DOWN,
            Direction(0, 1) => RIGHT,
            Direction(1, 0) => UP,
            _ => panic!("cannot rotate non-cardinal Direction"),
        }
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(rhs.0 + self.0, rhs.1 + self.1)
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0 as i32,
            y: self.y + rhs.1 as i32,
        }
    }
}

impl Add<Direction> for &Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        *self + rhs
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Position> for &Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        *self + rhs
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Dimensions {
    pub w: i32,
    pub h: i32,
}

impl Dimensions {
    pub fn of(w: i32, h: i32) -> Dimensions {
        Dimensions { w, h }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn at(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn add(&mut self, dir: Direction) -> Position {
        Position {
            x: self.x + dir.0 as i32,
            y: self.y + dir.1 as i32,
        }
    }

    pub fn add_pos(&self, pos: Position) -> Position {
        Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
    pub fn sub_pos(&self, pos: Position) -> Position {
        Position {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }

    pub fn delta(&self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn wrapping_add_direction(
        &self,
        rhs: Direction,
        bounds: (Position, Dimensions),
    ) -> Position {
        let mut pos = self + rhs;

        while pos.x < bounds.0.x {
            pos.x += bounds.1.w;
        }
        while pos.y < bounds.0.y {
            pos.y += bounds.1.h;
        }

        while pos.x >= bounds.0.x + bounds.1.w {
            pos.x -= bounds.1.w;
        }
        while pos.y >= bounds.0.y + bounds.1.h {
            pos.y -= bounds.1.h;
        }

        pos
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_wrapping_add_direction() {
    let bounds = (Position::at(0, 0), Dimensions::of(10, 10));

    let mut pos = Position::at(0, 0);
    pos = pos.wrapping_add_direction(Direction::of(-1, -1), bounds);

    assert_eq!(Position::at(9, 9), pos);

    let mut pos = Position::at(5, 5);
    pos = pos.wrapping_add_direction(Direction::of(-100, 100), bounds);

    assert_eq!(Position::at(5, 5), pos);

    let bounds = (Position::at(0, 0), Dimensions::of(11, 7));
    let mut pos = Position::at(10, 3);
    pos = pos.wrapping_add_direction(Direction::of(1, 2), bounds);
    assert_eq!(Position::at(0, 5), pos);

    pos = pos.wrapping_add_direction(Direction::of(1, 2), bounds);
    assert_eq!(Position::at(1, 0), pos);

    pos = pos.wrapping_add_direction(Direction::of(1, 2), bounds);
    assert_eq!(Position::at(2, 2), pos);
}

pub fn permutations<T: Clone>(n: usize, items: &[T]) -> Vec<Vec<T>> {
    fn append<T: Clone>(v: &Vec<T>, op: T) -> Vec<T> {
        let mut v = v.clone();
        v.push(op);
        v
    }

    match n {
        0 => panic!("invalid size for permutations"),
        1 => items.iter().map(|value| vec![value.clone()]).collect(),
        n => permutations(n - 1, items)
            .into_iter()
            .flat_map(|vs| {
                items
                    .iter()
                    .map(|value| append(&vs, value.clone()))
                    .collect::<Vec<Vec<T>>>()
            })
            .collect(),
    }
}
#[test]
fn test_permuatations() {
    assert_eq!(
        vec![
            vec![1, 1, 1],
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![1, 2, 2],
            vec![2, 1, 1],
            vec![2, 1, 2],
            vec![2, 2, 1],
            vec![2, 2, 2],
        ],
        permutations(3, &[1, 2])
    );
}

pub fn combinations<T: Clone>(n: usize, items: &[T]) -> Vec<Vec<T>> {
    debug_assert!(n <= items.len());

    if n == 1 {
        items.iter().map(|v| vec![v.clone()]).collect()
    } else {
        let mut result = Vec::new();
        for head in 0..=items.len() - n {
            for tail in combinations(n - 1, &items[head + 1..]) {
                let mut combination = vec![items[head].clone()];
                tail.iter().for_each(|v| combination.push(v.clone()));
                result.push(combination);
            }
        }
        result
    }
}

#[test]
fn test_combinations() {
    assert_eq!(
        vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3]
        ],
        combinations(2, &[0, 1, 2, 3])
    );

    assert_eq!(
        vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 5],
            vec![1, 2, 4, 5],
            vec![1, 3, 4, 5],
            vec![2, 3, 4, 5]
        ],
        combinations(4, &[1, 2, 3, 4, 5])
    );
}

#[rustfmt::skip]
pub fn ndigits(n: usize) -> usize {
                                  if
                                n < 10                     { 1  }
    else if                    n  < 100                    { 2  }
    else if                   n   < 1000                   { 3  }
    else if                  n    < 10000                  { 4  }
    else if                 n     < 100000                 { 5  }
    else if                n      < 1000000                { 6  }
    else if               n       < 10000000               { 7  }
    else if              n        < 100000000              { 8  }
    else if             n         < 1000000000             { 9  }
    else if            n          < 10000000000            { 10 }
    else if           n           < 100000000000           { 11 }
    else if          n            < 1000000000000          { 12 }
    else if         n             < 10000000000000         { 13 }
    else if        n              < 100000000000000        { 14 }
    else if       n               < 1000000000000000       { 15 }
    else if      n                < 10000000000000000      { 16 }
    else if     n                 < 100000000000000000     { 17 }
    else if    n                  < 1000000000000000000    { 18 }
    else if   n                   < 10000000000000000000   { 19 }
    else                        { 20 }
}

// Digits is a stack-allocatable structure to capture the base-10 digits of
// any usize number.
pub struct Digits {
    pub ds: [u8; 20],
    pub len: u8,
}

impl Digits {
    fn new() -> Self {
        Digits {
            ds: [0; 20],
            len: 0,
        }
    }

    pub fn of(mut n: usize) -> Self {
        let mut d = Digits::new();

        d.len = ndigits(n) as u8;

        for pos in (0..d.len).rev() {
            d.ds[pos as usize] = (n % 10) as u8;
            n = n / 10;
        }

        return d;
    }
}

impl PartialEq for Digits {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.len {
            // values outside of len are not valid.
            if self.ds[i as usize] != other.ds[i as usize] {
                return false;
            }
        }

        return true;
    }
}

#[test]
fn test_digits_of() {
    #[rustfmt::skip]
    let test_cases = [
        (1,         vec![1]),
        (12,        vec![1, 2]),
        (123,       vec![1, 2, 3]),
        (1234,      vec![1, 2, 3, 4]),
        (12345,     vec![1, 2, 3, 4, 5]),
        (123456,    vec![1, 2, 3, 4, 5, 6]),
        (1234567,   vec![1, 2, 3, 4, 5, 6, 7]),
        (12345678,  vec![1, 2, 3, 4, 5, 6, 7, 8]),
        (123456789, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ];

    for (n, ref expected) in test_cases {
        let d = Digits::of(n);
        for i in 0..expected.len() {
            assert_eq!(
                expected[i], d.ds[i] as usize,
                "digit at index {i} should be {}, but was {}",
                expected[i], d.ds[i]
            );
        }
    }
}

pub fn split_docs(s: String) -> Vec<String> {
    let mut buf = String::new();
    let mut docs = Vec::new();

    for line in s.lines() {
        if line != "" {
            buf.push_str(line);
            buf.push('\n');
        } else {
            docs.push(buf.clone());
            buf.clear();
        }
    }
    if buf != "" {
        docs.push(buf);
    }

    docs
}

use crate::shared::*;
use std::collections::HashSet;
use std::fs::read_to_string;

// A 2d grid of chars which can be loaded from a text file.
#[derive(Debug, Clone)]
pub struct CharGrid {
    width: i32,
    chars: Vec<char>,
}

impl CharGrid {
    pub fn new(width: i32, height: i32) -> CharGrid {
        debug_assert!(width >= 1);
        debug_assert!(height >= 1);

        let chars = vec![' '; (width * height) as usize];

        CharGrid { width, chars }
    }

    pub fn from_file(path: &str) -> Result<CharGrid> {
        let contents = read_to_string(path)?;

        Self::from_str(&contents)
    }

    pub fn from_str(str: &str) -> Result<CharGrid> {
        let mut width: i32 = 0;
        let mut chars = Vec::new();

        for line in str.lines() {
            let trimmed = line.trim_end_matches("\n");
            if width == 0 {
                width = trimmed.len() as i32
            } else if width != trimmed.len() as i32 {
                return Err(Error::new("lines do not have equal length"));
            }

            for c in trimmed.chars() {
                chars.push(c);
            }
        }

        Ok(CharGrid { width, chars })
    }

    pub fn fill(&mut self, c: char) {
        self.chars.fill(c);
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn x_max(&self) -> i32 {
        self.width - 1
    }

    pub fn height(&self) -> i32 {
        self.chars.len() as i32 / self.width()
    }

    pub fn y_max(&self) -> i32 {
        self.height() - 1
    }

    pub fn set(&mut self, x: i32, y: i32, c: char) {
        self.chars[((y * self.width) + x) as usize] = c
    }

    pub fn set_pos(&mut self, pos: Position, c: char) {
        debug_assert!(
            self.in_bounds(pos),
            "set_pos called with out of bounds position {}",
            pos
        );
        self.set(pos.x, pos.y, c)
    }

    pub fn at(&self, x: i32, y: i32) -> char {
        self.chars[((y * self.width) + x) as usize]
    }

    pub fn at_pos(&self, pos: Position) -> char {
        self.at(pos.x, pos.y)
    }

    pub fn find_one(&self, c: char) -> Result<(i32, i32)> {
        let item = self.chars.iter().enumerate().find(|(idx, e)| **e == c);

        match item {
            Some((idx, _)) => Ok((idx as i32 % self.width, idx as i32 / self.width)),
            None => Err(Error::new(&format!("char '{}' not found in CharGrid", c))),
        }
    }

    pub fn find_one_pos(&self, c: char) -> Result<Position> {
        self.find_one(c).map(|v| Position::at(v.0, v.1))
    }

    pub fn find_all_pos(&self, c: char) -> Vec<Position> {
        self.chars
            .iter()
            .enumerate()
            .filter(|(idx, e)| **e == c)
            .map(|(idx, _)| Position {
                x: idx as i32 % self.width,
                y: idx as i32 / self.width,
            })
            .collect()
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        p.x >= 0 && p.y >= 0 && p.x <= self.x_max() && p.y <= self.y_max()
    }

    pub fn count(&self, c: char) -> usize {
        self.chars.iter().filter(|v| **v == c).count()
    }

    pub fn uniq_chars(&self) -> Vec<char> {
        let mut chars = HashSet::new();

        for c in &self.chars {
            chars.insert(*c);
        }

        chars.into_iter().collect()
    }

    pub fn line_direction(&self, mut x: i32, mut y: i32, dir: Direction) -> Vec<char> {
        debug_assert!(x >= 0 && x <= self.x_max());
        debug_assert!(y >= 0 && y <= self.y_max());

        let mut vec = Vec::new();

        loop {
            vec.push(self.at(x, y));

            x += dir.0 as i32;
            y += dir.1 as i32;

            if x < 0 || y < 0 || x > self.x_max() || y > self.y_max() {
                return vec;
            }
        }
    }

    pub fn windows(&self, width: i32, height: i32) -> Windows<'_> {
        Windows {
            cg: self,
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn draw(&self) -> String {
        let mut str = String::new();

        for y in 0..self.height() {
            for x in 0..self.width {
                str.push(self.at(x, y))
            }
            str.push('\n')
        }
        str.push('\n');

        str
    }
}

#[test]
fn test_basics() {
    let cg = CharGrid::from_str("AAAA\nBBBB\nCCCC\nDDDD").unwrap();

    assert_eq!(4, cg.width());
    assert_eq!(4, cg.height());
    assert_eq!(3, cg.y_max());
    assert_eq!(3, cg.x_max());
}

#[test]
fn test_line_direction() {
    let cg = CharGrid::from_str("AAAA\nBBBB\nCCCC\nDDDD").unwrap();

    assert_eq!(vec!['A', 'A', 'A', 'A'], cg.line_direction(0, 0, RIGHT));
    assert_eq!(vec!['B', 'B', 'B', 'B'], cg.line_direction(0, 1, RIGHT));
    assert_eq!(vec!['B', 'C', 'D'], cg.line_direction(0, 1, RIGHT + DOWN));
    assert_eq!(vec!['D', 'C', 'B', 'A'], cg.line_direction(3, 3, UP + LEFT));
}

#[test]
fn test_windows() {
    let cg = CharGrid::from_str("AAAA\nBBBB\nCCCC\nDDDD").unwrap();

    let views: Vec<_> = cg.windows(2, 2).collect();
    assert_eq!(9, views.len());
    assert_eq!(views[0].chars(), vec!['A', 'A', 'B', 'B']);
    assert_eq!(views[8].chars(), vec!['C', 'C', 'D', 'D']);
}

#[test]
fn test_find_one() {
    let cg = CharGrid::from_str("AAA\nBBB\nXYZ").unwrap();

    assert_eq!(Ok((0, 0)), cg.find_one('A'));
    assert_eq!(Ok((0, 1)), cg.find_one('B'));
    assert_eq!(Ok((2, 2)), cg.find_one('Z'));
    assert!(cg.find_one('^').is_err());
}

// Windows is an iterator over a CharGrid that yields every permutation of the given size as a
// read-only view.
pub struct Windows<'a> {
    cg: &'a CharGrid,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl<'a> Iterator for Windows<'a> {
    type Item = CharGridView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.cg.width() - self.width || self.y > self.cg.height() - self.height {
            return None;
        }

        let result = Some(CharGridView {
            cg: self.cg,
            x_offset: self.x,
            y_offset: self.y,
            width: self.width,
            height: self.height,
        });

        self.x += 1;
        if self.x > self.cg.width() - self.width {
            self.x = 0;
            self.y += 1;
        }

        result
    }
}

#[derive(Debug)]
pub struct CharGridView<'a> {
    cg: &'a CharGrid,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
}

impl CharGridView<'_> {
    pub fn at(&self, x: i32, y: i32) -> char {
        self.cg.at(x + self.x_offset, y + self.y_offset)
    }

    pub fn chars(&self) -> Vec<char> {
        let mut v = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                v.push(self.at(x, y))
            }
        }

        v
    }
}

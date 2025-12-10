use crate::shared::*;
use std::collections::HashSet;
use std::fs::read_to_string;

pub trait Metadata: Default + Clone {}
impl<T: Default + Clone> Metadata for T {}

// A 2d grid of chars which can be loaded from a text file,
// and carries metadata about each cell.
#[derive(Debug, Clone)]
pub struct MetaGrid<M: Metadata> {
    width: i32,
    chars: Vec<char>,
    meta: Vec<M>,
}

impl<M: Metadata> MetaGrid<M> {
    pub fn new(width: i32, height: i32) -> MetaGrid<M> {
        debug_assert!(width >= 1);
        debug_assert!(height >= 1);

        let chars = vec![' '; (width * height) as usize];
        let meta = vec![Default::default(); (width * height) as usize];

        MetaGrid { width, chars, meta }
    }

    pub fn from_file(path: &str) -> Result<MetaGrid<M>> {
        let contents = read_to_string(path)?;

        Self::from_str(&contents)
    }

    pub fn from_str(str: &str) -> Result<MetaGrid<M>> {
        let mut width: i32 = 0;
        let mut chars = Vec::new();

        for line in str.lines() {
            let trimmed = line.trim();
            if width == 0 {
                width = trimmed.len() as i32
            } else if width != trimmed.len() as i32 {
                return Err(Error::new("lines do not have equal length"));
            }

            for c in trimmed.chars() {
                chars.push(c);
            }
        }

        let meta = vec![Default::default(); chars.len()];
        let new = MetaGrid { width, chars, meta };
        debug_assert!(new.chars.len() == new.meta.len());

        Ok(new)
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

    pub fn set(&mut self, pos: Position, c: char) {
        debug_assert!(
            self.in_bounds(pos),
            "set called with out of bounds position {}",
            pos
        );

        self.chars[((pos.y * self.width) + pos.x) as usize] = c
    }

    pub fn at(&self, pos: Position) -> (char, &M) {
        let idx = ((pos.y * self.width) + pos.x) as usize;
        (self.chars[idx], &self.meta[idx])
    }

    pub fn at_owned(&self, pos: Position) -> (char, M) {
        let idx = ((pos.y * self.width) + pos.x) as usize;
        (self.chars[idx], self.meta[idx].clone())
    }

    pub fn meta(&self, pos: Position) -> &M {
        let idx = ((pos.y * self.width) + pos.x) as usize;
        &self.meta[idx]
    }

    pub fn set_meta(&mut self, pos: Position, v: M) {
        let idx = ((pos.y * self.width) + pos.x) as usize;
        self.meta[idx] = v;
    }

    pub fn find_one(&self, c: char) -> Result<Position> {
        let item = self.chars.iter().enumerate().find(|(idx, e)| **e == c);

        match item {
            Some((idx, _)) => Ok(Position::at(
                idx as i32 % self.width,
                idx as i32 / self.width,
            )),
            None => Err(Error::new(&format!("char '{}' not found in MetaGrid", c))),
        }
    }

    pub fn find_all(&self, c: char) -> Vec<Position> {
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

    pub fn adjacent(&self, pos: Position) -> Vec<(Position, char, M)> {
        let mut items = Vec::new();

        let positions = [
            pos + UP + LEFT,
            pos + UP,
            pos + UP + RIGHT,
            pos + LEFT,
            // self
            pos + RIGHT,
            pos + DOWN + LEFT,
            pos + DOWN,
            pos + DOWN + RIGHT,
        ];

        for p in positions {
            if self.in_bounds(p) {
                let at = self.at(p);
                items.push((p, at.0, at.1.clone()));
            }
        }

        return items;
    }

    pub fn adjacent_orthogonal(&self, pos: Position) -> Vec<(Position, char, M)> {
        let mut items = Vec::new();

        for p in [pos + UP, pos + LEFT, pos + RIGHT, pos + DOWN] {
            if self.in_bounds(p) {
                let at = self.at(p);
                items.push((p, at.0, at.1.clone()));
            }
        }

        return items;
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

    pub fn windows(&self, width: i32, height: i32) -> Windows<'_, M> {
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
                str.push(self.at(Position::at(x, y)).0)
            }
            str.push('\n')
        }
        str.push('\n');

        str
    }
}

#[test]
fn test_basics() {
    let mg: MetaGrid<usize> = MetaGrid::from_str("AAAA\nBBBB\nCCCC\nDDDD").unwrap();

    assert_eq!(4, mg.width());
    assert_eq!(4, mg.height());
    assert_eq!(3, mg.y_max());
    assert_eq!(3, mg.x_max());
}

#[test]
fn test_windows() {
    let mg: MetaGrid<usize> = MetaGrid::from_str("AAAA\nBBBB\nCCCC\nDDDD").unwrap();

    let views: Vec<_> = mg.windows(2, 2).collect();
    assert_eq!(9, views.len());
    assert_eq!(views[0].chars(), vec!['A', 'A', 'B', 'B']);
    assert_eq!(views[8].chars(), vec!['C', 'C', 'D', 'D']);
}

#[test]
fn test_find_one() {
    let mg: MetaGrid<usize> = MetaGrid::from_str("AAA\nBBB\nXYZ").unwrap();

    assert_eq!(Ok(Position::at(0, 0)), mg.find_one('A'));
    assert_eq!(Ok(Position::at(0, 1)), mg.find_one('B'));
    assert_eq!(Ok(Position::at(2, 2)), mg.find_one('Z'));
    assert!(mg.find_one('^').is_err());
}

// Windows is an iterator over a MetaGrid that yields every permutation of the given size as a
// read-only view.
pub struct Windows<'a, M: Metadata> {
    cg: &'a MetaGrid<M>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl<'a, M: Metadata> Iterator for Windows<'a, M> {
    type Item = MetaGridView<'a, M>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.cg.width() - self.width || self.y > self.cg.height() - self.height {
            return None;
        }

        let result = Some(MetaGridView {
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
pub struct MetaGridView<'a, M: Metadata> {
    cg: &'a MetaGrid<M>,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
}

impl<M: Metadata> MetaGridView<'_, M> {
    pub fn at(&self, pos: Position) -> (char, &M) {
        self.cg.at(pos + Position::at(self.x_offset, self.y_offset))
    }

    pub fn chars(&self) -> Vec<char> {
        let mut v = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                v.push(self.at(Position::at(x, y)).0)
            }
        }

        v
    }
}

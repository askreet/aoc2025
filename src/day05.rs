use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use crate::shared::*;

pub struct Day05;

impl Solution for Day05 {
    fn part1(&self) -> Result<String> {
        let input = input(5)?;

        return Ok(part1(&input)?.to_string());
    }

    fn part2(&self) -> Result<String> {
        let input = input(5)?;

        return Ok(part2(&input)?.to_string());
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    enum Mode {
        RANGES,
        INGREDIENTS,
    }
    let mut mode = Mode::RANGES;
    let mut fresh = 0;
    'lines: for line in input.lines() {
        if line == "" {
            mode = Mode::INGREDIENTS;
            continue;
        }

        match mode {
            Mode::RANGES => {
                let (l, r) = match line.split_once("-") {
                    Some((l, r)) => (l, r),
                    None => return err(&format!("invalid range '{line}'")),
                };
                ranges.push(RangeInclusive::new(l.parse()?, r.parse()?));
            }
            Mode::INGREDIENTS => {
                let id: u64 = line.parse()?;

                for range in &ranges {
                    if range.contains(&id) {
                        fresh += 1;
                        continue 'lines;
                    }
                }
            }
        }
    }

    return Ok(fresh);
}

#[test]
fn test_part1() {
    let fresh = part1(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    );

    assert_eq!(3, fresh.unwrap());
}

fn part2(input: &str) -> Result<u64> {
    let mut set = MergingRangeInclusiveSet::new();

    for line in input.lines() {
        if line == "" {
            break;
        }

        match line.split_once("-") {
            Some((start, end)) => set.insert(start.parse()?, end.parse()?),
            None => return err(&format!("invalid range '{line}'")),
        };
    }

    return Ok(set.sum());
}

#[test]
fn test_part2() {
    let total_ids = part2(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    );

    assert_eq!(14, total_ids.unwrap());
}

struct MergingRangeInclusiveSet {
    vals: Vec<u64>,
}

impl MergingRangeInclusiveSet {
    fn new() -> Self {
        Self { vals: Vec::new() }
    }
    fn insert(&mut self, start: u64, end: u64) {
        for idx in 0..self.vals.len() {
            if idx == 0 {
                // check for prepend
                if end < self.vals[idx] {
                    self.vals.insert(0, start);
                    self.vals.insert(1, end);
                    return;
                }
            }

            if idx % 2 == 0 {
                // check for overlap
                if let Some((start, end)) =
                    Self::join(self.vals[idx], self.vals[idx + 1], start, end)
                {
                    self.vals[idx] = start;
                    self.vals[idx + 1] = end;

                    self.compact();

                    return;
                }
            } else {
                if idx != self.vals.len() - 1 {
                    if start > self.vals[idx] && end < self.vals[idx + 1] {
                        self.vals.insert(idx + 1, start);
                        self.vals.insert(idx + 2, end);
                        return;
                    }
                }
            }
        }

        // append to end of list
        self.vals.push(start);
        self.vals.push(end);
    }

    // merge any ranges that have overlapped during an insert operation
    fn compact(&mut self) {
        let mut idx = 0;
        loop {
            if idx >= self.vals.len() - 2 {
                break;
            }

            let (start1, end1) = (self.vals[idx], self.vals[idx + 1]);
            let (start2, end2) = (self.vals[idx + 2], self.vals[idx + 3]);

            if let Some((start, end)) = Self::join(start1, end1, start2, end2) {
                self.vals[idx] = start;
                self.vals[idx + 1] = end;
                self.vals.remove(idx + 2);
                self.vals.remove(idx + 2);
            } else {
                // Only increment if we don't merge, in case we need to merge again.
                idx += 2;
            }
        }
    }

    fn join(start1: u64, end1: u64, start2: u64, end2: u64) -> Option<(u64, u64)> {
        let overlaps = (start1 >= start2 && start1 <= end2) // partial overlap
            || (end1 >= start2 && end1 <= end2)
            || (start1 < start2 && end1 > end2) // total subset
            || (start2 < start1 && end2 > end1);

        if overlaps {
            return Some((min::<u64>(start1, start2), max::<u64>(end1, end2)));
        }

        return None;
    }

    fn sum(&self) -> u64 {
        self.vals.chunks_exact(2).map(|v| v[1] - v[0] + 1).sum()
    }
}

#[cfg(test)]
mod merging_range_inclusive_set {
    use super::*;

    #[test]
    fn test_join() {
        assert_eq!(Some((1, 5)), MergingRangeInclusiveSet::join(1, 3, 3, 5));
        assert_eq!(Some((1, 5)), MergingRangeInclusiveSet::join(1, 5, 1, 5));
        assert_eq!(Some((1, 5)), MergingRangeInclusiveSet::join(1, 3, 2, 5));
        assert_eq!(Some((1, 5)), MergingRangeInclusiveSet::join(2, 5, 1, 3));
        assert_eq!(Some((1, 5)), MergingRangeInclusiveSet::join(1, 5, 1, 5));

        assert_eq!(Some((1, 30)), MergingRangeInclusiveSet::join(1, 30, 2, 29));

        assert_eq!(None, MergingRangeInclusiveSet::join(1, 3, 4, 7));
        assert_eq!(None, MergingRangeInclusiveSet::join(4, 7, 1, 3));
    }

    #[test]
    fn extend_one_range_rightward() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15],
        };
        set.insert(3, 7);
        assert_eq!(vec![1, 7, 10, 15], set.vals);
    }

    #[test]
    fn extend_one_range_leftward() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15],
        };
        set.insert(8, 13);
        assert_eq!(vec![1, 5, 8, 15], set.vals);
    }

    #[test]
    fn extend_one_range_bothward() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15],
        };
        set.insert(8, 17);
        assert_eq!(vec![1, 5, 8, 17], set.vals);
    }

    #[test]
    fn bridge_two_ranges() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15],
        };
        set.insert(4, 10);
        assert_eq!(vec![1, 15], set.vals);
    }

    #[test]
    fn bridge_three_ranges_with_extension() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15, 20, 25],
        };
        set.insert(3, 29);
        assert_eq!(vec![1, 29], set.vals);
    }

    #[test]
    fn insert_between_two_ranges() {
        let mut set = MergingRangeInclusiveSet {
            vals: vec![1, 5, 10, 15],
        };
        set.insert(6, 9);
        assert_eq!(vec![1, 5, 6, 9, 10, 15], set.vals);
    }
}

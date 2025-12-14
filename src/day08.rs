use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::shared::*;

pub struct Day08;

impl Solution for Day08 {
    fn part1(&self) -> Result<String> {
        let input = input(8)?;

        return part1(&input, 1000, 3).map(|v| v.to_string());
    }

    fn part2(&self) -> Result<String> {
        let input = input(8)?;

        return part2(&input).map(|v| v.to_string());
    }
}

#[derive(Debug, PartialEq)]
struct Distance(f32);
// This is a lie, but it's a believable lie. And those are the best kind.
impl Eq for Distance {}
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // reverse sort order for distance in binary heap
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse sort order for distance in binary heap
        other.0.total_cmp(&self.0)
    }
}

impl Distance {
    fn between(a: &JunctionBox, b: &JunctionBox) -> Self {
        Self(f32::sqrt(
            (a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2),
        ))
    }
}

#[derive(Debug)]
struct JunctionBox {
    x: f32,
    y: f32,
    z: f32,
    circuit_id: usize,
}

#[derive(Debug)]
struct Space {
    nodes: Vec<JunctionBox>,
}

impl Space {
    fn from_str(input: &str) -> Result<Self> {
        let mut space = Self { nodes: Vec::new() };

        for line in input.lines() {
            let values: Vec<_> = line.split(",").collect();
            if values.len() != 3 {
                return err(&format!("malformed line: {line}"));
            }
            let next_id = space.nodes.len();
            space.nodes.push(JunctionBox {
                x: values[0].parse()?,
                y: values[1].parse()?,
                z: values[2].parse()?,
                circuit_id: next_id,
            });
        }

        return Ok(space);
    }

    fn closest_boxes(&self) -> BinaryHeap<(Distance, (usize, usize))> {
        let mut heap: BinaryHeap<(Distance, (usize, usize))> = BinaryHeap::new();

        let ids: Vec<usize> = (0..self.nodes.len()).into_iter().collect();
        for ids in combinations(2, &ids) {
            let a = &self.nodes[ids[0]];
            let b = &self.nodes[ids[1]];

            heap.push((Distance::between(a, b), (ids[0], ids[1])));
        }

        return heap;
    }

    fn connect_closest(&mut self, n: usize) -> Result<()> {
        if n > self.nodes.len() {
            return err("not enough nodes to connect");
        }

        let mut heap = self.closest_boxes();

        for _ in 0..n {
            let (_, (a, b)) = heap.pop().unwrap();
            if !self.is_connected(a, b) {
                self.connect(a, b);
            }
        }

        return Ok(());
    }

    fn circuits(&self) -> Vec<HashSet<usize>> {
        let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (id, node) in self.nodes.iter().enumerate() {
            circuits
                .entry(node.circuit_id)
                .and_modify(|hs| {
                    hs.insert(id);
                })
                .or_insert_with(|| {
                    let mut hs = HashSet::new();
                    hs.insert(id);
                    return hs;
                });
        }

        return circuits.into_iter().map(|(k, v)| v).collect();
    }

    fn is_connected(&self, a: usize, b: usize) -> bool {
        self.nodes[a].circuit_id == self.nodes[b].circuit_id
    }

    fn connect(&mut self, a: usize, b: usize) {
        let source_id = max(self.nodes[a].circuit_id, self.nodes[b].circuit_id);
        let target_id = min(self.nodes[a].circuit_id, self.nodes[b].circuit_id);

        for node in &mut self.nodes {
            if node.circuit_id == source_id {
                node.circuit_id = target_id;
            }
        }
    }

    fn all_connected(&self) -> bool {
        let target = self.nodes[0].circuit_id;

        for node in &self.nodes {
            if node.circuit_id != target {
                return false;
            }
        }

        return true;
    }
}

fn part1(input: &str, conns: usize, top: usize) -> Result<usize> {
    let mut space = Space::from_str(input)?;

    space.connect_closest(conns)?;

    let mut circuits = space.circuits();
    circuits.sort_by_key(HashSet::len);
    let top: Vec<_> = circuits.into_iter().rev().take(top).collect();

    Ok(top
        .iter()
        .map(HashSet::len)
        .reduce(std::ops::Mul::mul)
        .unwrap())
}

#[test]
fn test_part1() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(40, part1(input, 10, 3).unwrap());
}

fn part2(input: &str) -> Result<usize> {
    let mut space = Space::from_str(input)?;

    let mut closest = space.closest_boxes();
    loop {
        if let Some((d, (a, b))) = closest.pop() {
            println!(
                "connecting {a}(x={}, circuit={}) to {b}(x={}, circuit={}) at a distance of {}",
                space.nodes[a].x,
                space.nodes[a].circuit_id,
                space.nodes[b].x,
                space.nodes[b].circuit_id,
                d.0
            );
            space.connect(a, b);
            if space.all_connected() {
                dbg!(closest.pop());
                return Ok((space.nodes[a].x as usize * space.nodes[b].x as usize) as usize);
            }
        } else {
            return err("ran out of boxes to connect");
        }
    }
}

#[test]
fn test_part2() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(25272, part2(input).unwrap());
}

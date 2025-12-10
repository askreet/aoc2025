use crate::shared::*;

pub struct Day04;

type Map = MetaGrid<usize>;

impl Solution for Day04 {
    fn part1(&self) -> Result<String> {
        let mut map = MetaGrid::from_file("inputs/4.txt")?;

        count_adj_rolls(&mut map);

        return Ok(n_accessible_rolls(&map).to_string());
    }

    fn part2(&self) -> Result<String> {
        let mut map = MetaGrid::from_file("inputs/4.txt")?;

        let mut total_removed = 0;
        while let Some(n_removed) = remove_rolls(&mut map) {
            total_removed += n_removed;
        }

        return Ok(total_removed.to_string());
    }
}

fn count_adj_rolls(map: &mut Map) {
    for pos in map.find_all('@') {
        let nearby_rolls = map
            .adjacent(pos)
            .iter()
            .filter(|(_, c, _)| *c == '@')
            .count();

        map.set_meta(pos, nearby_rolls);
    }
}

fn n_accessible_rolls(map: &Map) -> usize {
    map.find_all('@')
        .into_iter()
        .filter(|p| *map.meta(*p) < 4)
        .count()
}

fn remove_rolls(map: &mut Map) -> Option<usize> {
    count_adj_rolls(map);

    let removable: Vec<Position> = map
        .find_all('@')
        .into_iter()
        .filter(|p| *map.meta(*p) < 4)
        .collect();

    for pos in &removable {
        map.set(*pos, '.');
    }

    if removable.len() > 0 {
        return Some(removable.len());
    }
    return None;
}

#[test]
fn test_part1() {
    let mut map = MetaGrid::from_str(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    )
    .expect("should not error");

    count_adj_rolls(&mut map);

    assert_eq!(13, n_accessible_rolls(&map));
}

#[test]
fn test_part2() {
    let mut map = MetaGrid::from_str(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    )
    .expect("should not error");

    let mut total_removed = 0;
    while let Some(n_removed) = remove_rolls(&mut map) {
        total_removed += n_removed;
    }

    assert_eq!(43, total_removed);
}

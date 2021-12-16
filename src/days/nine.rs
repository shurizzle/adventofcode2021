use std::{
    collections::{BTreeSet, VecDeque},
    vec::IntoIter,
};

use crate::utils::matrix::{
    cardinal_coords, enum_iter, enum_navigate, navigate, Coord, IndexesIterator, Matrix,
};

const INPUT: &str = include_str!("../../inputs/9");

fn parse(text: &str) -> Matrix<u8> {
    text.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[inline]
fn adiacents<'a, T>(matrix: &'a Matrix<T>, pos: Coord) -> IndexesIterator<'a, T, IntoIter<Coord>> {
    enum_navigate(matrix, &pos, cardinal_coords)
}

fn low_points<'a, T: PartialOrd>(
    matrix: &'a Matrix<T>,
) -> impl Iterator<Item = (Coord, &'a T)> + 'a {
    enum_iter(matrix).filter(|&(pos, v)| adiacents(matrix, pos).find(|&(_, v2)| v >= v2).is_none())
}

fn basin<'a, T: PartialOrd>(
    matrix: &'a Matrix<T>,
    pos: Coord,
    max: &T,
    taken: &BTreeSet<Coord>,
) -> BTreeSet<Coord> {
    let mut state = BTreeSet::new();
    state.insert(pos);

    navigate(
        matrix,
        cardinal_coords,
        &pos,
        (state, taken),
        |(mut res, taken), (_, prev_value), (pos, value)| {
            (
                if prev_value < value && value < max && !taken.contains(&pos) {
                    res.insert(pos);
                    true
                } else {
                    false
                },
                (res, taken),
            )
        },
    )
    .0
}

fn basins<'a, T: PartialOrd>(
    matrix: &'a Matrix<T>,
    max: T,
) -> impl Iterator<Item = BTreeSet<Coord>> + 'a {
    let mut taken = BTreeSet::new();
    let lp = low_points(matrix).collect::<VecDeque<_>>();
    lp.into_iter().map(move |(pos, _)| {
        let b = basin(matrix, pos, &max, &taken);
        b.iter().for_each(|&p| {
            taken.insert(p);
        });
        b
    })
}

pub(crate) fn solution1(text: &str) -> usize {
    low_points(&parse(text))
        .map(|(_, &x)| 1 + (x as usize))
        .sum::<usize>()
}

pub(crate) fn solution2(text: &str) -> usize {
    let mut basins_sizes = basins(&parse(text), 9)
        .map(|ps| ps.len())
        .collect::<Vec<_>>();
    basins_sizes.sort_by(|a, b| b.cmp(a));

    basins_sizes.iter().take(3).fold(1, |acc, v| acc * v)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod nine_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 1134);
    }
}

use std::hash::Hash;

use pathfinding::num_traits::Zero;

use crate::utils::{
    inc::IncAssign,
    matrix::{dijkstra, Coord, Matrix},
};

const INPUT: &str = include_str!("../../inputs/15");

fn parse(text: &str) -> Matrix<usize> {
    text.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn solve<T>(matrix: &Matrix<T>) -> Option<(Vec<Coord>, T)>
where
    T: Eq + Hash + Copy + Clone + Ord + Zero,
{
    dijkstra(
        matrix,
        (0, 0),
        (matrix.len() - 1, matrix[matrix.len() - 1].len() - 1),
    )
}

fn inc_all<T: Clone + Ord + IncAssign>(matrix: &mut Matrix<T>, min: &T, max: &T) {
    for line in matrix.iter_mut() {
        for cell in line.iter_mut() {
            cell.inc_assign();
            if PartialOrd::gt(cell, max) {
                *cell = Clone::clone(min);
            }
        }
    }
}

fn mul<T: Clone + Ord + IncAssign>(matrix: Matrix<T>, times: usize, min: &T, max: &T) -> Matrix<T> {
    let max_times = times * 2 - 1;
    let mut i_times = 1;
    let mut i_range = min.clone().inc();
    let mut squares: Vec<Matrix<T>> = Vec::new();
    squares.push(matrix);

    while i_times < max_times && i_range.le(max) {
        squares.push(squares[squares.len() - 1].clone());
        let last_idx = squares.len() - 1;
        inc_all(&mut squares[last_idx], min, max);
        i_times.inc_assign();
        i_range.inc_assign();
    }

    let mut res: Matrix<T> = Vec::new();
    for i in 0..times {
        let mut line = squares[i % squares.len()].clone();
        for j in 1..times {
            for (il, l) in squares[(i + j) % squares.len()].iter().enumerate() {
                for c in l.iter() {
                    line[il].push(c.clone());
                }
            }
        }
        res.append(&mut line);
    }

    res
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(&parse(text)).unwrap().1
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(&mul(parse(text), 5, &1, &9)).unwrap().1
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod fifteen_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 40);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 315);
    }
}

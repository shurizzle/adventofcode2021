use fifo_set::FIFOSet;

use crate::utils::inc::IncAssign;

const INPUT: &str = include_str!("../../inputs/11");

fn neighbors(p: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
    fn triplet(x: usize, max: usize) -> Vec<usize> {
        let mut res = Vec::new();
        if x > 0 {
            res.push(x - 1);
        }
        res.push(x);
        let nx = x + 1;
        if nx < max {
            res.push(nx);
        }
        res
    }

    triplet(p.0, size.0)
        .into_iter()
        .flat_map(|x| {
            triplet(p.1, size.1)
                .into_iter()
                .filter_map(|y| if (x, y) == p { None } else { Some((x, y)) })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<(usize, usize)>>()
}

fn parse(text: &str) -> Vec<Vec<u8>> {
    text.trim()
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn inc_all<T: IncAssign + Ord>(matrix: &mut Vec<Vec<T>>, max: &T) -> FIFOSet<(usize, usize)> {
    let mut flashing = FIFOSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            matrix[i][j].inc_assign();

            if matrix[i][j].gt(max) {
                flashing.push((i, j));
            }
        }
    }

    flashing
}

fn flash<T: IncAssign + PartialOrd>(
    matrix: &mut Vec<Vec<T>>,
    max: &T,
    pos: (usize, usize),
    stack: &mut FIFOSet<(usize, usize)>,
) {
    for (i, j) in neighbors(pos, (matrix.len(), matrix[pos.0].len())) {
        if matrix[i][j].le(max) {
            matrix[i][j].inc_assign();
            if matrix[i][j].gt(max) {
                stack.push((i, j));
            }
        }
    }
}

fn flash_all<T: IncAssign + Ord>(
    matrix: &mut Vec<Vec<T>>,
    max: &T,
    mut stack: FIFOSet<(usize, usize)>,
) {
    while let Some(pos) = stack.pop() {
        if matrix[pos.0][pos.1].gt(max) {
            flash(matrix, max, pos, &mut stack);
        }
    }
}

fn count_and_reset<T: Clone + Ord>(matrix: &mut Vec<Vec<T>>, min: &T, max: &T) -> usize {
    let mut count: usize = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].gt(max) {
                count += 1;
                matrix[i][j] = min.clone();
            }
        }
    }

    count
}

fn evolve<T: IncAssign + Ord + Clone>(matrix: &mut Vec<Vec<T>>, min: &T, max: &T) -> usize {
    let flashing = inc_all(matrix, max);
    flash_all(matrix, max, flashing);
    count_and_reset(matrix, min, max)
}

pub(crate) fn solution1(text: &str) -> usize {
    let mut matrix = parse(text);
    (0..100).fold(0usize, |acc, _| acc + evolve(&mut matrix, &0, &9))
}

pub(crate) fn solution2(text: &str) -> usize {
    let mut matrix = parse(text);
    let count = matrix.iter().map(|line| line.len()).sum();
    let mut step = 1;
    while evolve(&mut matrix, &0, &9) != count {
        step += 1;
    }
    step
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 1: {}", solution2(INPUT));
}

#[cfg(test)]
mod eleven_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 1656);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 195);
    }
}

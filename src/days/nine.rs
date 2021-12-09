use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/9");

fn parse(text: &str) -> Vec<Vec<u8>> {
    text.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct IndexesIterator<'a, T> {
    matrix: &'a Vec<Vec<T>>,
    indexes: Vec<(usize, usize)>,
}

impl<'a, T> IndexesIterator<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>, indexes: Vec<(usize, usize)>) -> Self {
        Self { matrix, indexes }
    }
}

impl<'a, T> Iterator for IndexesIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some((i, j)) = self.indexes.pop() {
            if let Some(v) = self.matrix.get(i).and_then(|v| v.get(j)) {
                return Some(((i, j), v));
            }
        }

        None
    }
}

#[derive(Debug)]
struct MatrixEnumeratedIterator<'a, T> {
    matrix: &'a Vec<Vec<T>>,
    pos: (usize, usize),
}

impl<'a, T> MatrixEnumeratedIterator<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>) -> Self {
        Self {
            matrix,
            pos: (0, 0),
        }
    }
}

impl<'a, T> Iterator for MatrixEnumeratedIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while self.pos.0 < self.matrix.len() {
            if self.pos.1 < self.matrix[self.pos.0].len() {
                let res = (self.pos, &self.matrix[self.pos.0][self.pos.1]);

                self.pos.1 += 1;
                if self.pos.1 >= self.matrix[self.pos.0].len() {
                    self.pos.0 += 1;
                    self.pos.1 = 0;
                }

                return Some(res);
            }
            todo!()
        }

        None
    }
}

fn adiacent_indexes<T>(matrix: &Vec<Vec<T>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut idxs = Vec::new();

    if matrix.get(pos.0).and_then(|v| v.get(pos.1)).is_none() {
        return idxs;
    }

    if pos.0 > 0 && pos.1 < matrix[pos.0 - 1].len() {
        idxs.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 {
        idxs.push((pos.0, pos.1 - 1));
    }

    if pos.1 + 1 < matrix[pos.0].len() {
        idxs.push((pos.0, pos.1 + 1));
    }

    if pos.0 + 1 < matrix.len() && pos.1 < matrix[pos.0 + 1].len() {
        idxs.push((pos.0 + 1, pos.1));
    }

    idxs
}

fn adiacents<'a, T>(matrix: &'a Vec<Vec<T>>, pos: (usize, usize)) -> IndexesIterator<'a, T> {
    IndexesIterator::new(matrix, adiacent_indexes(matrix, pos))
}

#[derive(Debug)]
struct LowPointsIterator<'a, T: PartialOrd> {
    matrix: &'a Vec<Vec<T>>,
    it: MatrixEnumeratedIterator<'a, T>,
}

impl<'a, T: PartialOrd> LowPointsIterator<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>) -> Self {
        Self {
            matrix,
            it: MatrixEnumeratedIterator::new(matrix),
        }
    }
}

impl<'a, T: PartialOrd> Iterator for LowPointsIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some((pos, v)) = self.it.next() {
            if adiacents(self.matrix, pos)
                .find(|&(_, v2)| v >= v2)
                .is_none()
            {
                return Some((pos, v));
            }
        }

        None
    }
}

fn low_points<'a, T: PartialOrd>(matrix: &'a Vec<Vec<T>>) -> LowPointsIterator<'a, T> {
    LowPointsIterator::new(matrix)
}

fn basin<'a, T: PartialOrd>(
    matrix: &'a Vec<Vec<T>>,
    pos: (usize, usize),
    max: &T,
    taken: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    let mut stack = Vec::new();
    res.insert(pos);
    stack.push(pos);

    while let Some(pos) = stack.pop() {
        adiacents(matrix, pos)
            .filter_map::<(usize, usize), _>(|(p2, v2)| {
                if matrix.get(pos.0).and_then(|v| v.get(pos.1)).unwrap() < v2
                    && v2 < max
                    && !taken.contains(&p2)
                {
                    Some(p2)
                } else {
                    None
                }
            })
            .for_each(|pos| {
                res.insert(pos);
                stack.push(pos);
            });
    }

    res
}

#[derive(Debug)]
struct BasinsIterator<'a, T: PartialOrd> {
    matrix: &'a Vec<Vec<T>>,
    low_points: VecDeque<(usize, usize)>,
    taken: HashSet<(usize, usize)>,
    max: T,
}

impl<'a, T: PartialOrd> BasinsIterator<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>, max: T) -> Self {
        let lp = low_points(matrix).fold(VecDeque::new(), |mut acc, (p, _)| {
            acc.push_back(p);
            acc
        });
        let mut taken = HashSet::new();
        lp.iter().for_each(|p| {
            taken.insert(*p);
        });
        Self {
            matrix,
            low_points: lp,
            taken,
            max,
        }
    }
}

impl<'a, T: PartialOrd> Iterator for BasinsIterator<'a, T> {
    type Item = HashSet<(usize, usize)>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(p) = self.low_points.pop_front() {
            let b = basin(self.matrix, p, &self.max, &self.taken);
            b.iter().for_each(|&p| {
                self.taken.insert(p);
            });

            Some(b)
        } else {
            None
        }
    }
}

fn basins<'a, T: PartialOrd>(matrix: &'a Vec<Vec<T>>, max: T) -> BasinsIterator<'a, T> {
    BasinsIterator::new(matrix, max)
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

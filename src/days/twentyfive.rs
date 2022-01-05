use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../../inputs/25");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    EAST,
    SOUTH,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::EAST, Direction::SOUTH].into_iter()
    }

    pub fn next_pos(&self, current: (usize, usize), size: (usize, usize)) -> (usize, usize) {
        match self {
            Self::EAST => {
                let mut x = current.0 + 1;
                if x >= size.0 {
                    x = 0;
                }
                (x, current.1)
            }
            Self::SOUTH => {
                let mut y = current.1 + 1;
                if y >= size.1 {
                    y = 0;
                }
                (current.0, y)
            }
        }
    }
}

pub struct Matrix {
    width: usize,
    height: usize,
    positions: BTreeMap<Direction, BTreeSet<(usize, usize)>>,
}

impl Matrix {
    pub fn parse(text: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut positions = BTreeMap::new();

        for dir in Direction::iter() {
            positions.insert(dir, BTreeSet::new());
        }

        for (y, line) in text.lines().enumerate() {
            height = height.max(y);

            for (x, c) in line.chars().enumerate() {
                width = width.max(x);

                match c {
                    '.' => (),
                    'v' => {
                        positions.get_mut(&Direction::SOUTH).unwrap().insert((x, y));
                    }
                    '>' => {
                        positions.get_mut(&Direction::EAST).unwrap().insert((x, y));
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            width: width + 1,
            height: height + 1,
            positions,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn step(&mut self) -> usize {
        let mut count = 0;

        for dir in Direction::iter() {
            count += self.step_direction(dir);
        }

        count
    }

    fn step_direction(&mut self, dir: Direction) -> usize {
        let mut count = 0;
        let mut new_set = BTreeSet::new();

        for &pos in &self.positions[&dir] {
            let next_pos = dir.next_pos(pos, self.size());
            if self.is_free(&next_pos) {
                count += 1;
                new_set.insert(next_pos);
            } else {
                new_set.insert(pos);
            }
        }

        self.positions.insert(dir, new_set);

        count
    }

    fn is_free(&self, pos: &(usize, usize)) -> bool {
        for set in self.positions.values() {
            if set.contains(pos) {
                return false;
            }
        }

        true
    }
}

pub fn solution1(text: &str) -> usize {
    let mut matrix = Matrix::parse(text);
    let mut count = 1;
    while matrix.step() != 0 {
        count += 1;
    }
    count
}

pub fn solution2(_text: &str) -> usize {
    unimplemented!()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod twentyfive_tests {
    use super::{Direction, Matrix};

    #[test]
    fn test1() {
        let mut matrix = Matrix::parse("...>>>>>...");
        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(3, 0), (4, 0), (5, 0), (6, 0), (7, 0)]
                .into_iter()
                .collect()
        );
        assert_eq!(matrix.step(), 1);
        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(3, 0), (4, 0), (5, 0), (6, 0), (8, 0)]
                .into_iter()
                .collect()
        );

        assert_eq!(matrix.step(), 2);
        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(3, 0), (4, 0), (5, 0), (7, 0), (9, 0)]
                .into_iter()
                .collect()
        );

        assert_eq!(matrix.step(), 3);
        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(3, 0), (4, 0), (6, 0), (8, 0), (10, 0)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn test2() {
        let mut matrix = Matrix::parse(
            "..........
.>v....v..
.......>..
..........",
        );

        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(1, 1), (7, 2)].into_iter().collect()
        );
        assert_eq!(
            &matrix.positions[&Direction::SOUTH],
            &[(2, 1), (7, 1)].into_iter().collect()
        );

        matrix.step();
        assert_eq!(
            &matrix.positions[&Direction::EAST],
            &[(1, 1), (8, 2)].into_iter().collect()
        );
        assert_eq!(
            &matrix.positions[&Direction::SOUTH],
            &[(2, 2), (7, 2)].into_iter().collect()
        );
    }

    #[test]
    fn test3() {
        let mut matrix = Matrix::parse(
            "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
        );
        let mut count = 1;
        while matrix.step() != 0 {
            count += 1;
        }

        assert_eq!(count, 58)
    }
}

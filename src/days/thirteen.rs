use std::{collections::BTreeSet, mem::take};

const INPUT: &str = include_str!("../../inputs/13");

#[derive(Copy, Clone, Debug)]
enum Direction {
    X,
    Y,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Fold(Direction, usize),
}

#[derive(Clone, Debug)]
struct Matrix {
    points: BTreeSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            points: BTreeSet::new(),
            height: 0,
            width: 0,
        }
    }

    pub fn add(&mut self, point: (usize, usize)) {
        self.points.insert(point);
        self.width = self.width.max(point.0 + 1);
        self.height = self.height.max(point.1 + 1);
    }

    pub fn execute(&mut self, inst: Instruction) {
        match inst {
            Instruction::Fold(Direction::X, x) => self.fold_x(x),
            Instruction::Fold(Direction::Y, y) => self.fold_y(y),
        }
    }

    pub fn fold_x(&mut self, x: usize) {
        if x == 0 || x >= self.width {
            return;
        }

        let right_len = self.width - x - 1;
        let shift = if right_len > x { right_len - x } else { 0 };
        let cut_x = x;
        self.width = x.max(right_len);
        self.points = take(&mut self.points)
            .into_iter()
            .filter_map(|(x, y)| {
                let new_x = x + shift;
                let new_cut_x = cut_x + shift;

                if x > cut_x {
                    Some((new_cut_x - (new_x - new_cut_x), y))
                } else if x < cut_x {
                    Some((new_x, y))
                } else {
                    None
                }
            })
            .collect();
    }

    pub fn fold_y(&mut self, y: usize) {
        if y == 0 || y >= self.height {
            return;
        }

        let bottom_len = self.height - y - 1;
        let shift = if bottom_len > y { bottom_len - y } else { 0 };
        let cut_y = y;
        self.height = y.max(bottom_len);
        self.points = take(&mut self.points)
            .into_iter()
            .filter_map(|(x, y)| {
                let new_y = y + shift;
                let new_cut_y = cut_y + shift;

                if y > cut_y {
                    Some((x, new_cut_y - (new_y - new_cut_y)))
                } else if y < cut_y {
                    Some((x, new_y))
                } else {
                    None
                }
            })
            .collect();
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut prev: Option<(usize, usize)> = None;

        let mut points: Vec<(usize, usize)> = self.points.iter().map(|p| *p).collect();
        points.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            otherwise => otherwise,
        });

        for p in points.into_iter() {
            if prev.is_none() {
                prev = Some((0, 0));
                if p == (0, 0) {
                    write!(f, "#")?;
                    continue;
                } else {
                    write!(f, " ")?;
                }
            }
            if let Some(prev) = prev {
                let x = if prev.1 == p.1 {
                    prev.0 + 1
                } else {
                    for _ in prev.1..p.1 {
                        write!(f, "\n")?;
                    }
                    0
                };

                for _ in x..p.0 {
                    write!(f, " ")?;
                }
                write!(f, "#")?;
            }
            prev = Some(p);
        }
        Ok(())
    }
}

fn parse_point(text: &str) -> (usize, usize) {
    let mut ps = text.trim().splitn(2, ",");

    (
        ps.next().unwrap().parse().unwrap(),
        ps.next().unwrap().parse().unwrap(),
    )
}

fn parse_instruction(text: &str) -> Instruction {
    if text.trim().starts_with("fold along ") {
        let mut ps = text.trim()[11..].splitn(2, "=");

        let dir = match ps.next().unwrap() {
            "x" | "X" => Direction::X,
            "y" | "Y" => Direction::Y,
            _ => unreachable!(),
        };
        let amount = ps.next().unwrap().parse().unwrap();

        Instruction::Fold(dir, amount)
    } else {
        unreachable!()
    }
}

fn parse(text: &str) -> (Matrix, Vec<Instruction>) {
    let (_, m, c) = text.trim().lines().fold(
        (true, Matrix::new(), Vec::new()),
        |(mut f, mut m, mut c), line| {
            if line.trim().is_empty() {
                f = !f;
            } else {
                if f {
                    m.add(parse_point(line));
                } else {
                    c.push(parse_instruction(line));
                }
            }

            (f, m, c)
        },
    );

    (m, c)
}

pub(crate) fn solution1(text: &str) -> usize {
    let (mut matrix, instructions) = parse(text);
    instructions
        .into_iter()
        .take(1)
        .for_each(|i| matrix.execute(i));
    matrix.len()
}

pub(crate) fn solution2(text: &str) -> String {
    let (mut matrix, instructions) = parse(text);
    instructions.into_iter().for_each(|i| matrix.execute(i));
    matrix.to_string()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2:\n{}", solution2(INPUT));
}

#[cfg(test)]
mod thirteen_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 17);
    }

    #[test]
    fn test2() {
        assert_eq!(
            solution2(INPUT),
            "#####
#   #
#   #
#   #
#####"
                .to_owned()
        );
    }
}

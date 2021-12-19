use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../../inputs/19");

type N = i64;
type Coord = (N, N, N);

const ROTATIONS: &'static [(Coord, Coord, Coord)] = &[
    ((-1, 0, 0), (0, -1, 0), (0, 0, 1)),
    ((-1, 0, 0), (0, 0, -1), (0, -1, 0)),
    ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
    ((-1, 0, 0), (0, 1, 0), (0, 0, -1)),
    ((0, -1, 0), (-1, 0, 0), (0, 0, -1)),
    ((0, -1, 0), (0, 0, -1), (1, 0, 0)),
    ((0, -1, 0), (0, 0, 1), (-1, 0, 0)),
    ((0, -1, 0), (1, 0, 0), (0, 0, 1)),
    ((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
    ((0, 0, -1), (0, -1, 0), (-1, 0, 0)),
    ((0, 0, -1), (0, 1, 0), (1, 0, 0)),
    ((0, 0, -1), (1, 0, 0), (0, -1, 0)),
    ((0, 0, 1), (-1, 0, 0), (0, -1, 0)),
    ((0, 0, 1), (0, -1, 0), (1, 0, 0)),
    ((0, 0, 1), (0, 1, 0), (-1, 0, 0)),
    ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
    ((0, 1, 0), (-1, 0, 0), (0, 0, 1)),
    ((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
    ((0, 1, 0), (0, 0, 1), (1, 0, 0)),
    ((0, 1, 0), (1, 0, 0), (0, 0, -1)),
    ((1, 0, 0), (0, -1, 0), (0, 0, -1)),
    ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
    ((1, 0, 0), (0, 0, 1), (0, -1, 0)),
    ((1, 0, 0), (0, 1, 0), (0, 0, 1)),
];

#[derive(Clone, Debug)]
struct Scanner {
    num: usize,
    coords: BTreeSet<Coord>,
}

#[derive(Clone, Debug)]
struct ScannerAltOrientationsIterator<'a> {
    scanner: &'a Scanner,
    rotation: usize,
}

impl Scanner {
    pub fn new(num: usize, coords: BTreeSet<Coord>) -> Self {
        Self { num, coords }
    }

    pub fn num(&self) -> usize {
        self.num
    }

    pub fn coords(&self) -> &BTreeSet<Coord> {
        &self.coords
    }

    pub fn alt_orientations<'a>(&'a self) -> ScannerAltOrientationsIterator<'a> {
        ScannerAltOrientationsIterator {
            scanner: &self,
            rotation: 0,
        }
    }
}

impl<'a> Iterator for ScannerAltOrientationsIterator<'a> {
    type Item = Scanner;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.rotation >= ROTATIONS.len() {
            return None;
        }

        fn mulsum(a: &Coord, b: &Coord) -> N {
            a.0 * b.0 + a.1 * b.1 + a.2 * b.2
        }

        let (xrot, yrot, zrot) = &ROTATIONS[self.rotation];
        let mut coords = BTreeSet::new();
        for c in self.scanner.coords() {
            coords.insert((mulsum(c, xrot), mulsum(c, yrot), mulsum(c, zrot)));
        }
        self.rotation += 1;

        Some(Scanner::new(self.scanner.num(), coords))
    }
}

fn parse_scanner(text: &str) -> Scanner {
    let text = text.trim();
    let mut lines = text.lines();
    let first = &lines.next().unwrap()[12..];
    let num: usize = first[..(first.find(' ').unwrap())].parse().unwrap();
    let coords = lines
        .map(|line| {
            let mut p = line.split(',').map(|n| n.parse::<N>().unwrap());
            (p.next().unwrap(), p.next().unwrap(), p.next().unwrap())
        })
        .collect();

    Scanner::new(num, coords)
}

fn parse(text: &str) -> BTreeMap<usize, Scanner> {
    text.split("\n\n")
        .map(parse_scanner)
        .fold(BTreeMap::new(), |mut acc, scanner| {
            acc.insert(scanner.num(), scanner);
            acc
        })
}

fn match_scanner(known: &BTreeSet<Coord>, scanners: &BTreeMap<usize, Scanner>) -> (Scanner, Coord) {
    for s in scanners.values() {
        for o in s.alt_orientations() {
            for base_c in known {
                for (idx2, test_c) in o.coords().iter().enumerate() {
                    if idx2 + 11 > o.coords().len() {
                        break;
                    }

                    let off = (
                        base_c.0 - test_c.0,
                        base_c.1 - test_c.1,
                        base_c.2 - test_c.2,
                    );
                    let mut matches: usize = 1;
                    for &c in o.coords().iter().skip(idx2 + 1) {
                        if known.contains(&(c.0 + off.0, c.1 + off.1, c.2 + off.2)) {
                            matches += 1;
                        }
                    }
                    if matches >= 12 {
                        let coords = o
                            .coords()
                            .iter()
                            .map(|&c| (c.0 + off.0, c.1 + off.1, c.2 + off.2))
                            .collect::<BTreeSet<_>>();

                        return (Scanner::new(o.num(), coords), off);
                    }
                }
            }
        }
    }

    unreachable!()
}

fn solve(text: &str) -> (BTreeSet<Coord>, BTreeSet<Coord>) {
    let mut scanners = parse(text);
    let Scanner {
        coords: mut known, ..
    } = scanners.remove(&0).unwrap();
    let mut offsets = BTreeSet::new();
    offsets.insert((0, 0, 0));

    while !scanners.is_empty() {
        let (Scanner { mut coords, num }, offset) = match_scanner(&known, &scanners);
        scanners.remove(&num);
        known.append(&mut coords);
        offsets.insert(offset);
    }

    (known, offsets)
}

fn manhattan_distance(a: &Coord, b: &Coord) -> u64 {
    ((a.0 - b.0).abs() as u64) + ((a.1 - b.1).abs() as u64) + ((a.2 - b.2).abs() as u64)
}

fn result1(solution: &(BTreeSet<Coord>, BTreeSet<Coord>)) -> usize {
    solution.0.len()
}

fn result2(solution: &(BTreeSet<Coord>, BTreeSet<Coord>)) -> u64 {
    let offsets = &solution.1;
    let mut max = 0;

    for o0 in offsets.iter() {
        for o1 in offsets.iter() {
            max = max.max(manhattan_distance(o0, o1));
        }
    }

    max
}

pub fn solution() {
    let res = solve(INPUT);

    println!("Solution 1: {}", result1(&res));
    println!("Solution 2: {}", result2(&res));
}

#[cfg(test)]
mod nineteen_tests {
    // No tests for you this time
}

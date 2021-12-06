use bresenham::Bresenham;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use crate::utils::{is_eof, read_line};

const INPUT: &str = include_str!("../../inputs/five.txt");

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct LineIterator {
    bresenham: Bresenham,
    last: Option<(u32, u32)>,
}

impl LineIterator {
    pub fn new(line: &Line) -> Self {
        Self {
            bresenham: Bresenham::new(
                (line.p1.x as isize, line.p1.y as isize),
                (line.p2.x as isize, line.p2.y as isize),
            ),
            last: Some((line.p2.x, line.p2.y)),
        }
    }
}

impl Iterator for LineIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if let Some((x, y)) = self.bresenham.next() {
            Some((x as u32, y as u32))
        } else {
            let res = self.last;
            self.last = None;
            res
        }
    }
}

#[derive(Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line({:?}, {:?})", self.p1, self.p2)
    }
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn is_90deg(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    pub fn is_diagonal(&self) -> bool {
        #[inline]
        fn diff(a: u32, b: u32) -> u32 {
            if b > a {
                b - a
            } else {
                a - b
            }
        }

        diff(self.p1.x, self.p2.x) == diff(self.p1.y, self.p2.y)
    }

    pub fn points(&self) -> LineIterator {
        LineIterator::new(self)
    }
}

#[derive(Clone, Debug)]
struct Canvas {
    points: HashMap<(u32, u32), u32>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            points: HashMap::new(),
        }
    }

    fn write_point(&mut self, point: (u32, u32)) {
        self.points
            .insert(point, self.points.get(&point).map(|&v| v).unwrap_or(0) + 1);
    }

    pub fn write<W: Writeable>(&mut self, object: W) {
        object.write(self)
    }

    pub fn points(&self) -> CanvasPoints {
        CanvasPoints::new(self)
    }
}

struct CanvasPoints<'a> {
    iter: std::collections::hash_map::Iter<'a, (u32, u32), u32>,
}

impl<'a> CanvasPoints<'a> {
    pub fn new(canvas: &'a Canvas) -> Self {
        Self {
            iter: canvas.points.iter(),
        }
    }
}

impl<'a> Iterator for CanvasPoints<'a> {
    type Item = ((u32, u32), u32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some((&(x, y), &val)) = self.iter.next() {
            Some(((x, y), val))
        } else {
            None
        }
    }
}

trait Writeable {
    fn write(&self, canvas: &mut Canvas);
}

impl Writeable for Point {
    fn write(&self, canvas: &mut Canvas) {
        canvas.write((self.x, self.y))
    }
}

impl Writeable for Line {
    fn write(&self, canvas: &mut Canvas) {
        for p in self.points() {
            canvas.write(p);
        }
    }
}

impl Writeable for (u32, u32) {
    fn write(&self, canvas: &mut Canvas) {
        canvas.write_point(*self)
    }
}

fn parse_point(text: &str) -> Point {
    let c: Vec<&str> = text.split(',').collect();

    if c.len() != 2 {
        panic!("Invalid point")
    }

    Point {
        x: c[0].trim().parse::<u32>().unwrap(),
        y: c[1].trim().parse::<u32>().unwrap(),
    }
}

fn parse_line(line: &str) -> Line {
    let points: Vec<&str> = line.split(" -> ").collect();

    if points.len() != 2 {
        panic!("Invalid line")
    }

    Line {
        p1: parse_point(points[0]),
        p2: parse_point(points[1]),
    }
}

fn parse(text: &str) -> Vec<Line> {
    let mut text = BufReader::new(text.as_bytes());
    let mut lines = Vec::new();
    while !is_eof(&mut text).unwrap() {
        lines.push(parse_line(&read_line(&mut text).unwrap()));
    }
    lines
}

fn solve(text: &str, filter: fn(&Line) -> bool) -> usize {
    let mut canvas = Canvas::new();
    parse(text)
        .into_iter()
        .filter(filter)
        .for_each(|line| canvas.write(line));
    canvas.points().filter(|(_, x)| *x > 1).count()
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(text, Line::is_90deg)
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(text, |line| line.is_90deg() || line.is_diagonal())
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod five_tests {
    use crate::days::five::{solution1, solution2};

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 12);
    }
}

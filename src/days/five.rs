use std::io::{BufRead, BufReader};

const INPUT: &str = include_str!("../../inputs/five.txt");

#[derive(Clone, Copy, Debug)]
pub(crate) struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn is_straight(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
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
        if self.is_vertical() {
            (self.p1.y..=self.p2.y).for_each(|y| canvas.write((self.p1.x, y)));
        } else if self.is_horizontal() {
            (self.p1.x..=self.p2.x).for_each(|x| canvas.write((x, self.p1.y)));
        }
    }
}

impl Writeable for (u32, u32) {
    fn write(&self, canvas: &mut Canvas) {
        canvas.write_point(*self)
    }
}

#[derive(Clone, Debug)]
struct Canvas {
    points: Vec<Vec<i32>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let points = vec![vec![0; width as usize]; height as usize];
        Canvas { points }
    }

    fn write_point(&mut self, point: (u32, u32)) {
        self.points[point.0 as usize][point.1 as usize] += 1;
    }

    pub fn write<W: Writeable>(&mut self, object: W) {
        object.write(self)
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

fn is_eof<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<bool> {
    text.fill_buf().map(|b| b.is_empty())
}

fn read_line<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<String> {
    let mut line = String::new();
    text.read_line(&mut line)?;
    line.truncate(line.trim_end_matches('\n').len());
    line.truncate(line.trim_end_matches('\r').len());
    Ok(line)
}

pub(crate) fn parse(text: &str) -> Vec<Line> {
    let mut text = BufReader::new(text.as_bytes());
    let mut lines = Vec::new();
    while !is_eof(&mut text).unwrap() {
        lines.push(parse_line(&read_line(&mut text).unwrap()));
    }
    lines
}

pub(crate) fn solution1(text: &str) -> usize {
    let mut canvas = Canvas::new(10, 10);
    parse(text).into_iter().for_each(|line| canvas.write(line));
    println!("{:?}", canvas);
    5
}

pub fn solution() {}

#[cfg(test)]
mod five_tests {
    use crate::days::five::{parse, solution1};

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
}

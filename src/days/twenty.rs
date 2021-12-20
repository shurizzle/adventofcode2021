use std::{
    cmp::Ordering,
    collections::BTreeSet,
    mem::{swap, take},
};

use num::FromPrimitive;

const INPUT: &str = include_str!("../../inputs/20");

fn matrix_iter(width: usize, height: usize) -> impl Iterator<Item = Coord> {
    (0..height).flat_map(move |y| (0..(width)).map(move |x| (x, y).into()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(ord) = PartialOrd::partial_cmp(&self.y, &other.y) {
            match ord {
                Ordering::Equal => PartialOrd::partial_cmp(&self.x, &other.x),
                o => Some(o),
            }
        } else {
            None
        }
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match Ord::cmp(&self.y, &other.y) {
            Ordering::Equal => Ord::cmp(&self.x, &other.x),
            o => o,
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x, y }
    }
}

impl Into<(usize, usize)> for Coord {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Clone, Debug)]
struct Image {
    pixels: BTreeSet<Coord>,
    width: usize,
    height: usize,
}

impl Image {
    #[inline]
    pub fn new(width: usize, height: usize, pixels: BTreeSet<Coord>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, coord: &Coord) -> Pixel {
        if self.pixels.contains(coord) {
            Pixel::Light
        } else {
            Pixel::Dark
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Pixel> + 'a {
        matrix_iter(self.width(), self.height()).map(|c| self.get(&c))
    }

    pub fn enhance(&self, algo: &Algorithm, times: usize) -> Image {
        algo.enhance(&self, times)
    }

    pub fn lit_len(&self) -> usize {
        self.pixels.len()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut count = 0usize;
        for pixel in self.iter() {
            count += 1;
            if count > self.width() {
                count = 1;
                write!(f, "\n")?;
            }
            write!(
                f,
                "{}",
                match pixel {
                    Pixel::Light => '#',
                    Pixel::Dark => ' ',
                }
            )?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, num_derive::FromPrimitive)]
enum SquarePosition {
    TopLeft = 0,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Copy, Clone, Debug)]
struct SquarePositionIterator(Option<SquarePosition>);

impl Iterator for SquarePositionIterator {
    type Item = SquarePosition;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(pos) = take(&mut self.0) {
            self.0 = pos.next();
            Some(pos)
        } else {
            None
        }
    }
}

impl SquarePosition {
    pub fn coord(&self, center: &Coord, width: usize, height: usize) -> Option<Coord> {
        match self {
            Self::TopLeft => {
                if center.x > 0 && center.y > 0 {
                    Some((center.x - 1, center.y - 1).into())
                } else {
                    None
                }
            }
            Self::Top => {
                if center.y > 0 {
                    Some((center.x, center.y - 1).into())
                } else {
                    None
                }
            }
            Self::TopRight => {
                let new_x = center.x + 1;
                if new_x < width && center.y > 0 {
                    Some((new_x, center.y - 1).into())
                } else {
                    None
                }
            }
            Self::Left => {
                if center.x > 0 {
                    Some((center.x - 1, center.y).into())
                } else {
                    None
                }
            }
            Self::Center => Some(*center),
            Self::Right => {
                let new_x = center.x + 1;
                if new_x < width {
                    Some((new_x, center.y).into())
                } else {
                    None
                }
            }
            Self::BottomLeft => {
                let new_y = center.y + 1;
                if center.x > 0 && new_y < height {
                    Some((center.x - 1, new_y).into())
                } else {
                    None
                }
            }
            Self::Bottom => {
                let new_y = center.y + 1;
                if new_y < height {
                    Some((center.x, new_y).into())
                } else {
                    None
                }
            }
            Self::BottomRight => {
                let new_x = center.x + 1;
                let new_y = center.y + 1;
                if new_x < width && new_y < height {
                    Some((new_x, new_y).into())
                } else {
                    None
                }
            }
        }
    }

    pub fn next(&self) -> Option<Self> {
        FromPrimitive::from_isize((*self as isize) + 1)
    }

    pub fn iter() -> SquarePositionIterator {
        SquarePositionIterator(Some(<Self as Default>::default()))
    }
}

impl Default for SquarePosition {
    fn default() -> Self {
        Self::TopLeft
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Dark
    }
}

impl Into<usize> for Pixel {
    fn into(self) -> usize {
        match self {
            Self::Light => 1,
            Self::Dark => 0,
        }
    }
}

#[derive(Debug)]
struct Algorithm(BTreeSet<usize>);

impl Algorithm {
    #[inline]
    pub fn new(pixels: BTreeSet<usize>) -> Algorithm {
        Self(pixels)
    }

    pub fn _enhance(&self, image: &Image, def: usize) -> Image {
        let new_width = image.width() + 2;
        let new_height = image.height() + 2;
        let x_range = 1..=image.width();
        let y_range = 1..=image.height();

        let pixels = matrix_iter(new_width, new_height)
            .filter_map(|c| {
                let idx = SquarePosition::iter().fold(0usize, |acc, pos| {
                    (acc << 1)
                        | if let Some(c) = pos.coord(&c, new_width, new_height) {
                            if x_range.contains(&c.x) && y_range.contains(&c.y) {
                                image.get(&(c.x - 1, c.y - 1).into()).into()
                            } else {
                                def
                            }
                        } else {
                            def
                        }
                });

                if self.0.contains(&idx) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<Coord>>();

        Image::new(new_width, new_height, pixels)
    }

    pub fn enhance(&self, image: &Image, mut times: usize) -> Image {
        let mut def0: usize = 0;
        let mut def1: usize = if self.0.contains(&0) { 1 } else { 0 };

        let mut image = if times < 1 {
            return image.clone();
        } else {
            times -= 1;
            swap(&mut def0, &mut def1);
            self._enhance(image, def1)
        };

        while times > 0 {
            image = self._enhance(&image, def0);
            swap(&mut def0, &mut def1);
            times -= 1;
        }

        image
    }
}

fn parse_algo(text: &str) -> Algorithm {
    Algorithm::new(
        text.trim()
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
            .collect(),
    )
}

fn parse_image(text: &str) -> Image {
    let mut pixels = BTreeSet::new();
    let mut width = 0;
    let mut height = 0;

    for (y, l) in text.trim().lines().enumerate() {
        height = height.max(y);
        for (x, c) in l.chars().enumerate() {
            width = width.max(x);
            match c {
                '#' => {
                    pixels.insert((x, y).into());
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    Image::new(width + 1, height + 1, pixels)
}

fn parse(text: &str) -> (Algorithm, Image) {
    let mut p = text.trim().splitn(2, "\n\n");

    (
        parse_algo(p.next().unwrap()),
        parse_image(p.next().unwrap()),
    )
}

fn solve(text: &str, times: usize) -> usize {
    let (algo, image) = parse(text);
    image.enhance(&algo, times).lit_len()
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(text, 2)
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(text, 50)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod twenty_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 35);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 3351);
    }
}

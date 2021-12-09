#![allow(dead_code)]

use std::io::{BufRead, BufReader};

use crate::utils::{is_eof, read_line};

const INPUT: &str = include_str!("../../inputs/4");

#[derive(Copy, Clone, Debug)]
struct Board {
    numbers: [u8; 25],
    markeds: [bool; 25],
    bingo: Option<[u8; 5]>,
}

impl Board {
    pub fn new(numbers: &Vec<u8>) -> Option<Self> {
        if numbers.len() != 25 {
            None
        } else {
            let mut res = Self {
                numbers: [0; 25],
                markeds: [false; 25],
                bingo: None,
            };
            for i in 0..25 {
                res.numbers[i] = numbers[i];
            }
            Some(res)
        }
    }

    pub fn has_bingo(&self) -> bool {
        self.bingo.is_some()
    }

    pub fn bingo(&self) -> Option<&[u8; 5]> {
        self.bingo.as_ref()
    }

    pub fn mark(&mut self, number: u8) {
        if let Some(pos) = self
            .numbers
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == number)
            .map(|(i, _)| i)
            .next()
        {
            self.markeds[pos] = true;
            self.check_bingo();
        }
    }

    pub fn unmarkeds(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for i in 0..25 {
            if !self.markeds[i] {
                res.push(self.numbers[i]);
            }
        }
        res
    }

    fn check_bingo(&mut self) {
        if !self.has_bingo() {
            self.check_horizontal_bingo();
        }

        if !self.has_bingo() {
            self.check_vertical_bingo();
        }
    }

    fn check_horizontal_bingo(&mut self) {
        for i in 0..5usize {
            if (0..5usize)
                .into_iter()
                .map(|j| self.markeds[5 * i + j])
                .find(|&val| !val)
                .is_none()
            {
                let mut bingo: [u8; 5] = [0; 5];
                for j in 0..5usize {
                    bingo[j] = self.numbers[5 * i + j];
                }
                self.bingo = Some(bingo);
                return;
            }
        }
    }

    fn check_vertical_bingo(&mut self) {
        for i in 0..5usize {
            if (0..5usize)
                .into_iter()
                .map(|j| self.markeds[i + 5 * j])
                .find(|&val| !val)
                .is_none()
            {
                let mut bingo: [u8; 5] = [0; 5];
                for j in 0..5usize {
                    bingo[j] = self.numbers[i + 5 * j];
                }
                self.bingo = Some(bingo);
                return;
            }
        }
    }
}

fn skip_line<R: std::io::Read>(text: &mut BufReader<R>) {
    if !is_eof(text).unwrap() {
        let mut _discard = Vec::new();
        text.read_until(b'\n', &mut _discard).unwrap();
    }
}

fn parse_extractions<R: std::io::Read>(text: &mut BufReader<R>) -> Vec<u8> {
    let line = read_line(text).unwrap();
    line.split(',').map(|n| n.parse::<u8>().unwrap()).collect()
}

fn parse_board<R: std::io::Read>(text: &mut BufReader<R>) -> Board {
    let mut numbers = Vec::new();

    for _ in 0..5 {
        let line = read_line(text).unwrap();
        let mut pieces = line
            .split_ascii_whitespace()
            .map(|p| p.parse::<u8>().unwrap());

        for _ in 0..5 {
            numbers.push(pieces.next().unwrap());
        }

        assert_eq!(pieces.next(), None);
    }

    Board::new(&numbers).unwrap()
}

fn parse(text: &str) -> (Vec<u8>, Vec<Board>) {
    let mut text = BufReader::new(text.as_bytes());
    let mut boards = Vec::new();
    let extractions = parse_extractions(&mut text);
    while !is_eof(&mut text).unwrap() {
        skip_line(&mut text);
        if !is_eof(&mut text).unwrap() {
            boards.push(parse_board(&mut text));
        }
    }

    (extractions, boards)
}

pub(crate) fn solution1(text: &str) -> u32 {
    let (extractions, mut boards) = parse(text);

    for number in extractions {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.has_bingo() {
                return board.unmarkeds().into_iter().map(|x| x as u32).sum::<u32>()
                    * (number as u32);
            }
        }
    }

    panic!("no bingo");
}

pub(crate) fn solution2(text: &str) -> u32 {
    let (extractions, mut boards) = parse(text);
    let mut last = None;
    let mut last_number = None;

    for number in extractions {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.has_bingo() {
                last = Some(board.clone());
                last_number = Some(number as u32);
            }
        }

        boards = boards
            .into_iter()
            .filter(|board| !board.has_bingo())
            .collect::<Vec<Board>>();
    }

    last.unwrap()
        .unmarkeds()
        .into_iter()
        .map(|x| x as u32)
        .sum::<u32>()
        * last_number.unwrap()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod four_tests {
    use crate::days::four::{solution1, solution2};

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 4512);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 1924);
    }
}

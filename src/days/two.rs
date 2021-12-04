#![allow(dead_code)]

use std::{fmt::Debug, str::FromStr};

const INPUT: &str = include_str!("../../inputs/two.txt");
const INVALID: &str = "Invalid command";

#[derive(Copy, Clone, Default, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Default, Debug)]
struct State {
    coord: Coord,
    aim: isize,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn result(&self) -> isize {
        self.coord.x * self.coord.y
    }
}

#[derive(Copy, Clone, Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

#[derive(Debug, Clone)]
pub struct InvalidCommand;

impl std::fmt::Display for InvalidCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Invalid command")
    }
}

impl FromStr for Command {
    type Err = InvalidCommand;

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        if line.starts_with("up ") {
            Ok(Command::Up(line[3..].parse().or(Err(InvalidCommand))?))
        } else if line.starts_with("down ") {
            Ok(Command::Down(line[5..].parse().or(Err(InvalidCommand))?))
        } else if line.starts_with("forward ") {
            Ok(Command::Forward(line[8..].parse().or(Err(InvalidCommand))?))
        } else {
            Err(InvalidCommand)
        }
    }
}

fn solve(text: &str, apply_fn: fn(State, Command) -> State) -> isize {
    text.lines()
        .map(|line| <Command as FromStr>::from_str(line).unwrap())
        .fold(State::new(), apply_fn)
        .result()
}

fn apply1(mut state: State, cmd: Command) -> State {
    match cmd {
        Command::Forward(x) => {
            state.coord.x += x as isize;
        }
        Command::Down(y) => {
            state.coord.y += y as isize;
        }
        Command::Up(y) => {
            state.coord.y -= y as isize;
        }
    }

    state
}

fn apply2(mut state: State, cmd: Command) -> State {
    match cmd {
        Command::Forward(x) => {
            state.coord.x += x as isize;
            state.coord.y += state.aim * (x as isize);
        }
        Command::Down(aim) => {
            state.aim += aim as isize;
        }
        Command::Up(aim) => {
            state.aim -= aim as isize;
        }
    }

    state
}

pub(crate) fn solution1(text: &str) -> isize {
    solve(text, apply1)
}

pub(crate) fn solution2(text: &str) -> isize {
    solve(text, apply2)
}

pub fn solution() -> () {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod two_tests {
    use crate::days::two::{solution1, solution2};

    const TEST: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test1() -> () {
        assert_eq!(solution1(TEST), 150);
    }

    #[test]
    fn test2() -> () {
        assert_eq!(solution2(TEST), 900);
    }
}

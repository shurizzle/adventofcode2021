use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/10");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BracketDirection {
    Open,
    Close,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Copy, Clone)]
struct Bracket(pub BracketDirection, pub BracketType);

impl Bracket {
    pub fn get_direction(&self) -> BracketDirection {
        self.0
    }

    pub fn get_type(&self) -> BracketType {
        self.1
    }
}

impl std::fmt::Debug for Bracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Bracket(BracketDirection::Open, BracketType::Round) => "(",
                Bracket(BracketDirection::Close, BracketType::Round) => ")",
                Bracket(BracketDirection::Open, BracketType::Square) => "[",
                Bracket(BracketDirection::Close, BracketType::Square) => "]",
                Bracket(BracketDirection::Open, BracketType::Curly) => "{",
                Bracket(BracketDirection::Close, BracketType::Curly) => "}",
                Bracket(BracketDirection::Open, BracketType::Angle) => "<",
                Bracket(BracketDirection::Close, BracketType::Angle) => ">",
            }
        )
    }
}

impl FromStr for Bracket {
    type Err = ();

    fn from_str(text: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        if text.len() == 1 {
            text.chars()
                .next()
                .ok_or_else(|| ())
                .and_then(|c: char| <Bracket as TryFrom<char>>::try_from(c))
        } else {
            Err(())
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(
        c: char,
    ) -> std::result::Result<Self, <Self as std::convert::TryFrom<char>>::Error> {
        match c {
            '(' => Ok(Bracket(BracketDirection::Open, BracketType::Round)),
            ')' => Ok(Bracket(BracketDirection::Close, BracketType::Round)),
            '[' => Ok(Bracket(BracketDirection::Open, BracketType::Square)),
            ']' => Ok(Bracket(BracketDirection::Close, BracketType::Square)),
            '{' => Ok(Bracket(BracketDirection::Open, BracketType::Curly)),
            '}' => Ok(Bracket(BracketDirection::Close, BracketType::Curly)),
            '<' => Ok(Bracket(BracketDirection::Open, BracketType::Angle)),
            '>' => Ok(Bracket(BracketDirection::Close, BracketType::Angle)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
enum Error {
    Corrupted(BracketType),
    Incomplete(Vec<BracketType>),
}

impl Error {
    pub fn is_corrupted(&self) -> bool {
        match self {
            Self::Corrupted(_) => true,
            _ => false,
        }
    }

    pub fn is_incomplete(&self) -> bool {
        match self {
            Self::Incomplete(_) => true,
            _ => false,
        }
    }
}

fn parse(text: &str) -> Vec<Vec<Bracket>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Bracket>>()
        })
        .collect::<Vec<_>>()
}

fn check_errors(line: &Vec<Bracket>) -> Option<Error> {
    let mut stack = Vec::new();
    for b in line {
        match b.get_direction() {
            BracketDirection::Open => {
                stack.push(b.get_type());
            }
            BracketDirection::Close => {
                if let Some(ty) = stack.pop() {
                    if ty != b.get_type() {
                        return Some(Error::Corrupted(b.get_type()));
                    }
                } else {
                    unreachable!();
                }
            }
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        Some(Error::Incomplete(stack))
    } else {
        None
    }
}

fn error_score(error: Error) -> usize {
    match error {
        Error::Corrupted(ty) => match ty {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        },
        Error::Incomplete(missing) => missing
            .into_iter()
            .map(|ty| match ty {
                BracketType::Round => 1,
                BracketType::Square => 2,
                BracketType::Curly => 3,
                BracketType::Angle => 4,
            })
            .fold(0, |acc, v| acc * 5 + v),
    }
}

pub(crate) fn solution1(text: &str) -> usize {
    parse(text)
        .into_iter()
        .filter_map(|line| check_errors(&line))
        .filter(Error::is_corrupted)
        .map(error_score)
        .sum()
}

pub(crate) fn solution2(text: &str) -> usize {
    let mut scores = parse(text)
        .into_iter()
        .filter_map(|line| check_errors(&line))
        .filter(Error::is_incomplete)
        .map(error_score)
        .collect::<Vec<_>>();
    scores.sort();

    scores[scores.len() / 2]
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod ten_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 26397);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 288957);
    }
}

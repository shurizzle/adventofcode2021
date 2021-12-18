use std::{
    iter::Sum,
    mem::take,
    ops::{Add, AddAssign},
};

use num::integer::Integer;

const INPUT: &str = include_str!("../../inputs/18");

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Pair(NumberElement, NumberElement);

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Pair({:?},{:?})", self.0, self.1)
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum NumberElement {
    Pair(Box<Pair>),
    Single(u8),
}

impl std::fmt::Debug for NumberElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "NumberElement::")?;

        match self {
            Self::Pair(pair) => {
                write!(f, "Pair({:?},{:?})", pair.as_ref().0, pair.as_ref().1)
            }
            Self::Single(v) => write!(f, "Single({})", v),
        }
    }
}

impl std::fmt::Display for NumberElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Pair(pair) => write!(f, "{}", pair.as_ref()),
            Self::Single(v) => write!(f, "{}", v),
        }
    }
}

impl Default for NumberElement {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplodeResult {
    Explode,
    AddRight(u8),
    AddLeft(u8),
    Restart,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SplitResult {
    Split,
    Restart,
    None,
}

impl NumberElement {
    pub fn is_single(&self) -> bool {
        match self {
            Self::Single(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_pair(&self) -> bool {
        !self.is_single()
    }

    pub fn explode(&mut self, level: usize) -> ExplodeResult {
        match self {
            Self::Pair(pair) => pair.explode(level),
            _ => ExplodeResult::None,
        }
    }

    pub fn split(&mut self) -> SplitResult {
        match self {
            Self::Single(ref i) => {
                if *i > 9 {
                    SplitResult::Split
                } else {
                    SplitResult::None
                }
            }
            Self::Pair(pair) => pair.split(),
        }
    }

    fn add_left(&mut self, amount: u8) {
        match self {
            Self::Single(ref mut i) => *i += amount,
            Self::Pair(pair) => pair.add_left(amount),
        }
    }

    fn add_right(&mut self, amount: u8) {
        match self {
            Self::Single(ref mut i) => *i += amount,
            Self::Pair(pair) => pair.add_right(amount),
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            NumberElement::Pair(pair) => pair.magnitude(),
            NumberElement::Single(n) => *n as usize,
        }
    }
}

impl Pair {
    pub fn explode(&mut self, level: usize) -> ExplodeResult {
        let (res, to_add) = match self.0.explode(level + 1) {
            ExplodeResult::Explode => {
                if let NumberElement::Pair(pair) = take(&mut self.0) {
                    match *pair {
                        Pair(NumberElement::Single(left), NumberElement::Single(right)) => {
                            (ExplodeResult::AddLeft(left), Some(right))
                        }
                        _ => (ExplodeResult::None, None),
                    }
                } else {
                    (ExplodeResult::None, None)
                }
            }
            ExplodeResult::AddRight(to_add) => (ExplodeResult::Restart, Some(to_add)),
            other => (other, None),
        };

        if let Some(to_add) = to_add {
            self.1.add_left(to_add);
        }

        if !matches!(res, ExplodeResult::None) {
            return res;
        }

        let (res, to_add) = match self.1.explode(level + 1) {
            ExplodeResult::Explode => {
                if let NumberElement::Pair(pair) = take(&mut self.1) {
                    match *pair {
                        Pair(NumberElement::Single(left), NumberElement::Single(right)) => {
                            (ExplodeResult::AddRight(right), Some(left))
                        }
                        _ => (ExplodeResult::None, None),
                    }
                } else {
                    (ExplodeResult::None, None)
                }
            }
            ExplodeResult::AddLeft(to_add) => (ExplodeResult::Restart, Some(to_add)),
            other => (other, None),
        };

        if let Some(to_add) = to_add {
            self.0.add_right(to_add);
        }

        if !matches!(res, ExplodeResult::None) {
            return res;
        }

        if level > 3 {
            match self {
                Pair(NumberElement::Single(_), NumberElement::Single(_)) => {
                    return ExplodeResult::Explode;
                }
                _ => (),
            }
        }

        ExplodeResult::None
    }

    pub fn split(&mut self) -> SplitResult {
        let res = match self.0.split() {
            SplitResult::Split => {
                if let NumberElement::Single(i) = take(&mut self.0) {
                    self.0 = NumberElement::Pair(Box::new(Pair(
                        NumberElement::Single(i / 2),
                        NumberElement::Single(i.div_ceil(&2)),
                    )));
                    SplitResult::Restart
                } else {
                    SplitResult::None
                }
            }
            res => res,
        };

        if !matches!(res, SplitResult::None) {
            return res;
        }

        match self.1.split() {
            SplitResult::Split => {
                if let NumberElement::Single(i) = take(&mut self.1) {
                    self.1 = NumberElement::Pair(Box::new(Pair(
                        NumberElement::Single(i / 2),
                        NumberElement::Single(i.div_ceil(&2)),
                    )));
                    SplitResult::Restart
                } else {
                    SplitResult::None
                }
            }
            res => res,
        }
    }

    fn add_left(&mut self, amount: u8) {
        self.0.add_left(amount)
    }

    fn add_right(&mut self, amount: u8) {
        self.1.add_right(amount)
    }

    pub fn magnitude(&self) -> usize {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Number(Pair);

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Into<NumberElement> for Number {
    fn into(self) -> NumberElement {
        NumberElement::Pair(Box::new(self.0))
    }
}

impl Number {
    pub fn concat(self, other: Self) -> Self {
        Number(Pair(
            NumberElement::Pair(Box::new(self.0)),
            NumberElement::Pair(Box::new(other.0)),
        ))
    }

    pub fn concat_assign(&mut self, other: Self) {
        self.0 = Pair(
            NumberElement::Pair(Box::new(take(&mut self.0))),
            NumberElement::Pair(Box::new(other.0)),
        )
    }

    pub fn reduce(&mut self) {
        while {
            !(matches!(self.0.explode(0), ExplodeResult::None)
                && matches!(self.0.split(), SplitResult::None))
        } {}
    }

    pub fn magnitude(&self) -> usize {
        self.0.magnitude()
    }
}

impl Add<Self> for Number {
    type Output = Self;

    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        let mut res = self.concat(other);
        res.reduce();
        res
    }
}

impl AddAssign<Self> for Number {
    fn add_assign(&mut self, other: Self) {
        self.concat_assign(other);
        self.reduce();
    }
}

impl Sum for Number {
    fn sum<I>(it: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut acc = None;
        for n in it {
            match acc {
                None => {
                    acc = Some(n);
                }
                Some(ref mut acc) => {
                    *acc += n;
                }
            }
        }

        acc.unwrap_or_else(|| Number(Pair(NumberElement::Single(0), NumberElement::Single(0))))
    }
}

mod parser {
    pub type ParseResult<'a, T> = (&'a str, T);
    pub type Result<'a, T, E> = std::result::Result<ParseResult<'a, T>, E>;

    pub trait Parse: Sized {
        type Error;

        fn parse<'a>(text: &'a str) -> Result<'a, Self, Self::Error>;

        fn parse_all<'a>(
            text: &'a str,
        ) -> std::result::Result<Option<Self>, <Self as Parse>::Error> {
            match <Self as Parse>::parse(text) {
                Ok((text, res)) => {
                    if text.is_empty() {
                        Ok(Some(res))
                    } else {
                        Ok(None)
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    pub fn parse<'a, T: Parse>(text: &'a str) -> Result<'a, T, <T as Parse>::Error> {
        <T as Parse>::parse(text)
    }

    pub fn parse_all<'a, T: Parse>(
        text: &'a str,
    ) -> std::result::Result<Option<T>, <T as Parse>::Error> {
        <T as Parse>::parse_all(text)
    }

    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn to_digit(c: char) -> u8 {
        (c as u8) - ('0' as u8)
    }

    #[inline]
    pub fn next_char<'a>(text: &'a str) -> Option<(&'a str, char)> {
        if let Some(c) = text.chars().nth(0) {
            Some((&text[1..], c))
        } else {
            None
        }
    }

    impl Parse for super::Pair {
        type Error = ();

        fn parse<'a>(
            text: &'a str,
        ) -> std::result::Result<(&'a str, Self), <Self as Parse>::Error> {
            if let Some((text, '[')) = next_char(text) {
                let (text, left) = parse(text)?;
                if let Some((text, ',')) = next_char(text) {
                    let (text, right) = parse(text)?;

                    if let Some((text, ']')) = next_char(text) {
                        return Ok((text, super::Pair(left, right)));
                    }
                }
            }

            Err(())
        }
    }

    impl Parse for super::NumberElement {
        type Error = ();

        fn parse<'a>(text: &'a str) -> Result<'a, Self, <Self as Parse>::Error> {
            if let Some((text, c)) = next_char(text) {
                if is_digit(c) {
                    return Ok((text, super::NumberElement::Single(to_digit(c))));
                }
            }

            match parse(text) {
                Ok((text, pair)) => Ok((text, super::NumberElement::Pair(Box::new(pair)))),
                Err(_) => Err(()),
            }
        }
    }

    impl Parse for super::Number {
        type Error = ();

        fn parse<'a>(text: &'a str) -> Result<'a, Self, <Self as Parse>::Error> {
            match parse(text) {
                Ok((text, pair)) => Ok((text, super::Number(pair))),
                Err(_) => Err(()),
            }
        }
    }
}

fn parse_line(text: &str) -> Option<Number> {
    match self::parser::parse_all(text.trim()) {
        Err(_) => None,
        Ok(res) => res.map(|mut n: Number| {
            n.reduce();
            n
        }),
    }
}

fn parse(text: &str) -> Vec<Number> {
    text.trim()
        .lines()
        .map(|line| parse_line(line).unwrap())
        .collect()
}

pub(crate) fn solution1(text: &str) -> usize {
    parse(text).into_iter().sum::<Number>().magnitude()
}

pub(crate) fn solution2(text: &str) -> usize {
    let numbers = parse(text);
    let mut max = None;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let magnitude = (numbers[i].clone() + numbers[j].clone()).magnitude();
                max = max.map_or_else(|| Some(magnitude), |m: usize| Some(m.max(magnitude)));
            }
        }
    }

    max.unwrap()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod eighteen_tests {
    use super::{parse, parse_line, solution2, Number, NumberElement, Pair};

    #[inline]
    fn eq(text: &str) {
        let n = parse_line(text);
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(text, n.to_string());
    }

    #[test]
    fn test1() {
        eq("[1,2]");
    }

    #[test]
    fn test2() {
        eq("[[1,2],3]");
    }

    #[test]
    fn test3() {
        eq("[9,[8,7]]");
    }

    #[test]
    fn test4() {
        eq("[[1,9],[8,5]]");
    }

    #[test]
    fn test5() {
        let n1 = parse_line("[1,2]");
        assert!(n1.is_some());
        let n1 = n1.unwrap();
        let n2 = parse_line("[[3,4],5]");
        assert!(n2.is_some());
        let n2 = n2.unwrap();
        let n3 = n1.concat(n2);
        let result = parse_line("[[1,2],[[3,4],5]]");
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result, n3);
    }

    #[test]
    fn test6() {
        let n1 = parse_line("[1,2]");
        assert!(n1.is_some());
        let mut n1 = n1.unwrap();
        let n2 = parse_line("[[3,4],5]");
        assert!(n2.is_some());
        let n2 = n2.unwrap();
        n1.concat_assign(n2);
        let result = parse_line("[[1,2],[[3,4],5]]");
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result, n1);
    }

    #[test]
    fn test7() {
        let mut n = Number(Pair(NumberElement::Single(11), NumberElement::Single(1)));
        n.reduce();
        assert_eq!(n.to_string(), "[[5,6],1]");
    }

    #[test]
    fn test8() {
        assert_eq!(
            parse_line("[[[[[9,8],1],2],3],4]").map(|n| n.to_string()),
            Some("[[[[0,9],2],3],4]".to_owned())
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            parse_line("[7,[6,[5,[4,[3,2]]]]]").map(|n| n.to_string()),
            Some("[7,[6,[5,[7,0]]]]".to_owned())
        );
    }

    #[test]
    fn test10() {
        assert_eq!(
            parse_line("[[6,[5,[4,[3,2]]]],1]").map(|n| n.to_string()),
            Some("[[6,[5,[7,0]]],3]".to_owned())
        );
    }

    #[test]
    fn test11() {
        assert_eq!(
            parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").map(|n| n.to_string()),
            Some("[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_owned())
        );
    }

    #[test]
    fn test12() {
        assert_eq!(
            parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").map(|n| n.to_string()),
            Some("[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_owned())
        );
    }

    fn test_add(text: &str, result: &str) -> Number {
        let numbers = parse(text);
        let sum: Number = numbers.into_iter().sum();
        let result = parse_line(result);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(sum.to_string(), result.to_string());
        result
    }

    #[test]
    fn test13() {
        test_add(
            "[1,1]
[2,2]
[3,3]
[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
    }

    #[test]
    fn test14() {
        test_add(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
    }

    #[test]
    fn test15() {
        test_add(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
    }

    #[test]
    fn test16() {
        test_add(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    fn test17() {
        assert_eq!(
            test_add(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            )
            .magnitude(),
            4140
        );
    }

    #[test]
    fn test18() {
        assert_eq!(
            solution2(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            3993
        );
    }
}

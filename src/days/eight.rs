use std::{borrow::Borrow, collections::HashMap};

const INPUT: &str = include_str!("../../inputs/eight.txt");

fn raw_parse(text: &str) -> Vec<([String; 10], Vec<String>)> {
    text.lines()
        .into_iter()
        .map(|line| {
            let mut it = line.split('|').map(str::trim);

            let wires = it
                .next()
                .unwrap()
                .split(' ')
                .map(|x| x.trim().to_owned())
                .collect::<Vec<String>>()
                .try_into()
                .unwrap_or_else(|v: Vec<String>| {
                    panic!("Expected a Vec of length 10 but it was {}", v.len())
                });
            let nums = it
                .next()
                .unwrap()
                .split(' ')
                .map(|w| w.trim().to_owned())
                .collect::<Vec<String>>();

            assert_eq!(it.next(), None);

            (wires, nums)
        })
        .collect::<Vec<([String; 10], Vec<String>)>>()
}

fn split_values(
    values: [String; 10],
) -> (
    [char; 2],
    [char; 4],
    [char; 3],
    [[char; 5]; 3],
    [[char; 6]; 3],
) {
    let mut one = None;
    let mut four = None;
    let mut seven = None;
    let mut seg_5 = Vec::new();
    let mut seg_6 = Vec::new();

    for v in values.into_iter().map(|v| {
        let mut v = v.chars().collect::<Vec<char>>();
        v.sort();
        v
    }) {
        match v.len() {
            2 => {
                one = Some(v.try_into().unwrap());
            }
            3 => {
                seven = Some(v.try_into().unwrap());
            }
            4 => {
                four = Some(v.try_into().unwrap());
            }
            5 => {
                seg_5.push(v.try_into().unwrap());
            }
            6 => {
                seg_6.push(v.try_into().unwrap());
            }
            7 => (),
            _ => unreachable!(),
        }
    }

    (
        one.unwrap(),
        four.unwrap(),
        seven.unwrap(),
        seg_5.try_into().unwrap(),
        seg_6.try_into().unwrap(),
    )
}

pub fn make_bitmap<C: Borrow<char>, I: Iterator<Item = C>>(i: I) -> u8 {
    i.map(|c| (*c.borrow() as u8) - 'a' as u8)
        .fold(0, |acc, c| acc | (1 << c))
}

pub(crate) fn make_map(values: [String; 10]) -> HashMap<u8, usize> {
    let mut map = HashMap::new();

    let (one, four, seven, seg_5, seg_6) = split_values(values);

    let a = *seven.iter().find(|&c| !one.contains(c)).unwrap();
    let three = seg_5
        .iter()
        .find(|s5| s5.iter().filter(|&c| one.contains(c)).count() == 2)
        .unwrap();
    let diff = four
        .iter()
        .filter(|&c| !one.contains(c))
        .map(|&x| x)
        .collect::<Vec<char>>();
    let two = seg_5
        .iter()
        .filter(|&s5| s5 != three)
        .find(|&s5| diff.iter().find(|&c| !s5.contains(c)).is_some())
        .unwrap();
    let five = seg_5.iter().find(|&s5| s5 != three && s5 != two).unwrap();
    let b = *five.iter().find(|&c| !three.contains(c)).unwrap();
    let e = *two.iter().find(|&c| !three.contains(c)).unwrap();
    let c = *two.iter().find(|&c| !five.contains(c) && c != &e).unwrap();
    let zero = seg_6
        .iter()
        .find(|s6| s6.contains(&c) && s6.contains(&e))
        .unwrap();
    let d = ('a'..='g').into_iter().find(|c| !zero.contains(c)).unwrap();
    let f = *one.iter().find(|&v| v != &c).unwrap();
    let g = ('a'..='g')
        .into_iter()
        .find(|v| ![a, b, c, d, e, f].contains(v))
        .unwrap();

    map.insert(make_bitmap([a, b, c, e, f, g].iter()), 0);
    map.insert(make_bitmap([c, f].iter()), 1);
    map.insert(make_bitmap([a, c, d, e, g].iter()), 2);
    map.insert(make_bitmap([a, c, d, f, g].iter()), 3);
    map.insert(make_bitmap([b, c, d, f].iter()), 4);
    map.insert(make_bitmap([a, b, d, f, g].iter()), 5);
    map.insert(make_bitmap([a, b, d, e, f, g].iter()), 6);
    map.insert(make_bitmap([a, c, f].iter()), 7);
    map.insert(make_bitmap([a, b, c, d, e, f, g].iter()), 8);
    map.insert(make_bitmap([a, b, c, d, f, g].iter()), 9);

    map
}

fn parse(text: &str) -> Vec<Vec<usize>> {
    raw_parse(text)
        .into_iter()
        .map(|(wires, nums)| {
            let map = make_map(wires);
            nums.into_iter()
                .map(|n| map[&make_bitmap(n.chars())])
                .collect()
        })
        .collect()
}

pub(crate) fn solution1(text: &str) -> usize {
    raw_parse(text)
        .into_iter()
        .map(|(_, nums)| {
            nums.into_iter()
                .filter(|x| [2, 4, 3, 7].contains(&x.len()))
                .count()
        })
        .sum()
}

pub(crate) fn solution2(text: &str) -> usize {
    parse(text)
        .into_iter()
        .map(|nums| nums.into_iter().fold(0, |acc, n| acc * 10 + n))
        .sum()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod eight_tests {
    use super::{solution1, solution2};

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 61229);
    }
}

use std::{borrow::Borrow, collections::HashMap};

const INPUT: &str = include_str!("../../inputs/eight.txt");

fn raw_parse(text: &str) -> Vec<([u8; 10], Vec<u8>)> {
    text.lines()
        .into_iter()
        .map(|line| {
            let mut it = line.split('|').map(str::trim);

            let wires: [u8; 10] = it
                .next()
                .unwrap()
                .split(' ')
                .map(|x| make_bitmap(x.trim().chars()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|v: Vec<u8>| {
                    panic!("Expected a Vec of length 10 but it was {}", v.len())
                });
            let nums = it
                .next()
                .unwrap()
                .split(' ')
                .map(|w| make_bitmap(w.trim().chars()))
                .collect::<Vec<_>>();

            assert_eq!(it.next(), None);

            (wires, nums)
        })
        .collect::<Vec<_>>()
}

fn split_values(values: [u8; 10]) -> (u8, u8, u8, u8, [u8; 3], [u8; 3]) {
    let mut one = None;
    let mut four = None;
    let mut seven = None;
    let mut eight = None;
    let mut seg_5 = Vec::new();
    let mut seg_6 = Vec::new();

    for v in values {
        match v.count_ones() {
            2 => {
                one = Some(v);
            }
            3 => {
                seven = Some(v);
            }
            4 => {
                four = Some(v);
            }
            5 => {
                seg_5.push(v);
            }
            6 => {
                seg_6.push(v);
            }
            7 => {
                eight = Some(v);
            }
            _ => unreachable!(),
        }
    }

    (
        one.unwrap(),
        four.unwrap(),
        seven.unwrap(),
        eight.unwrap(),
        seg_5.try_into().unwrap(),
        seg_6.try_into().unwrap(),
    )
}

fn make_bitmap<C: Borrow<char>, I: Iterator<Item = C>>(i: I) -> u8 {
    i.map(|c| (*c.borrow() as u8) - 'a' as u8)
        .fold(0, |acc, c| acc | (1 << c))
}

fn contains(a: u8, b: u8) -> bool {
    a & b == b
}

fn extract(haystack: &mut Vec<u8>, find: impl Fn(u8) -> bool) -> u8 {
    let idx = haystack
        .iter()
        .enumerate()
        .find_map(|(i, &n)| if find(n) { Some(i) } else { None })
        .unwrap();
    haystack.remove(idx)
}

fn extract_contains(haystack: &mut Vec<u8>, search: u8) -> u8 {
    extract(haystack, |n| contains(n, search))
}

fn make_map(values: [u8; 10]) -> HashMap<u8, usize> {
    let mut map = HashMap::new();
    let (one, four, seven, eight, seg_5, seg_6) = split_values(values);
    let mut seg_5 = seg_5.to_vec();
    let mut seg_6 = seg_6.to_vec();

    map.insert(one, 1);
    map.insert(four, 4);
    map.insert(seven, 7);
    map.insert(eight, 8);
    map.insert(extract_contains(&mut seg_5, one), 3);
    map.insert(extract_contains(&mut seg_6, four), 9);
    map.insert(extract_contains(&mut seg_6, one), 0);
    let six = seg_6.remove(0);
    map.insert(six, 6);
    map.insert(extract(&mut seg_5, |n| contains(six, n)), 5);
    map.insert(seg_5.remove(0), 2);

    assert_eq!(seg_5.len(), 0);
    assert_eq!(seg_6.len(), 0);

    map
}

fn parse(text: &str) -> Vec<Vec<usize>> {
    raw_parse(text)
        .into_iter()
        .map(|(wires, nums)| {
            let map = make_map(wires);
            nums.into_iter().map(|n| map[&n]).collect()
        })
        .collect()
}

pub(crate) fn solution1(text: &str) -> usize {
    raw_parse(text)
        .into_iter()
        .map(|(_, nums)| {
            nums.into_iter()
                .filter(|x| [2, 4, 3, 7].contains(&x.count_ones()))
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

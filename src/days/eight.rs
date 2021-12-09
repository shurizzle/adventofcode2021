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
    [char; 7],
    [[char; 5]; 3],
    [[char; 6]; 3],
) {
    let mut one = None;
    let mut four = None;
    let mut seven = None;
    let mut eight = None;
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
            7 => {
                eight = Some(v.try_into().unwrap());
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

fn contains<C: PartialEq>(a: &[C], b: &[C]) -> bool {
    b.iter().find(|&c| !a.contains(c)).is_none()
}

fn extract<C: PartialEq, A: Borrow<[C]>>(haystack: &mut Vec<A>, find: impl Fn(&[C]) -> bool) -> A {
    let idx = haystack
        .iter()
        .enumerate()
        .find_map(|(i, n)| if find(n.borrow()) { Some(i) } else { None })
        .unwrap();
    haystack.remove(idx)
}

fn extract_contains<C: PartialEq, A: Borrow<[C]>>(haystack: &mut Vec<A>, search: &[C]) -> A {
    extract(haystack, |n| contains(n, search))
}

fn make_map(values: [String; 10]) -> HashMap<u8, usize> {
    let mut map = HashMap::new();

    let (one, four, seven, eight, seg_5, seg_6) = split_values(values);
    let mut seg_5 = seg_5.to_vec();
    let mut seg_6 = seg_6.to_vec();
    let three = extract_contains(&mut seg_5, &one);
    let nine = extract_contains(&mut seg_6, &four);
    let zero = extract_contains(&mut seg_6, &one);
    let six = seg_6.remove(0);
    let five = extract(&mut seg_5, |n| contains(&six, n));
    let two = seg_5.remove(0);

    assert_eq!(seg_5.len(), 0);
    assert_eq!(seg_6.len(), 0);

    map.insert(make_bitmap(zero.iter()), 0);
    map.insert(make_bitmap(one.iter()), 1);
    map.insert(make_bitmap(two.iter()), 2);
    map.insert(make_bitmap(three.iter()), 3);
    map.insert(make_bitmap(four.iter()), 4);
    map.insert(make_bitmap(five.iter()), 5);
    map.insert(make_bitmap(six.iter()), 6);
    map.insert(make_bitmap(seven.iter()), 7);
    map.insert(make_bitmap(eight.iter()), 8);
    map.insert(make_bitmap(nine.iter()), 9);

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

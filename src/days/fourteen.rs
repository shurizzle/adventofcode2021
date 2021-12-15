use std::{borrow::Borrow, cmp::Ordering, collections::BTreeMap, mem::take};

const INPUT: &str = include_str!("../../inputs/14");

#[derive(Debug)]
struct Polymer {
    units: BTreeMap<(char, char), usize>,
    rules: BTreeMap<(char, char), char>,
    first: Option<char>,
    last: Option<char>,
}

impl Polymer {
    pub fn new(template: &str, rules: BTreeMap<(char, char), char>) -> Self {
        let template = template.chars().collect::<Vec<_>>();
        let (first, last) = match template.len() {
            0 => (None, None),
            1 => (Some(template[0]), None),
            len => (Some(template[0]), Some(template[len - 1])),
        };

        let mut keys = Vec::new();
        for i in 1..template.len() {
            keys.push((template[i - 1], template[i]));
        }

        let mut units = BTreeMap::new();
        for key in keys {
            *units.entry(key).or_insert(0) += 1;
        }

        Self {
            units,
            rules,
            first,
            last,
        }
    }

    pub fn evolve(&mut self) {
        for (key, count) in take(&mut self.units) {
            if let Some(&ins) = self.rules.get(&key) {
                *self.units.entry((key.0, ins)).or_insert(0) += count;
                *self.units.entry((ins, key.1)).or_insert(0) += count;
            }
        }
    }

    pub fn get_counts(&self) -> BTreeMap<char, usize> {
        let mut counts = BTreeMap::new();
        for (&(a, b), &count) in self.units.iter() {
            *counts.entry(a).or_insert(0) += count;
            *counts.entry(b).or_insert(0) += count;
        }
        if let Some(first) = self.first {
            *counts.entry(first).or_insert(0) += 1;
        }
        if let Some(last) = self.last {
            *counts.entry(last).or_insert(0) += 1;
        }

        counts.iter_mut().for_each(|(_, c)| *c /= 2);

        counts
    }

    pub fn min_max(&self) -> (Option<usize>, Option<usize>) {
        let counts = self.get_counts();
        let (min, max) = min_max(counts.values());

        (min.map(|x| *x), max.map(|x| *x))
    }

    pub fn result(&self) -> usize {
        let (min, max) = self.min_max();

        max.unwrap_or(0) - min.unwrap_or(0)
    }
}

fn parse_rule(text: &str) -> ((char, char), char) {
    let mut it = text.splitn(2, " -> ");
    let pattern = it.next().unwrap();
    let insertion = it.next().unwrap();

    let pattern = pattern.chars().collect::<Vec<_>>();
    let insertion = insertion.chars().collect::<Vec<_>>();

    assert_eq!(it.next(), None);
    assert_eq!(pattern.len(), 2);
    assert_eq!(insertion.len(), 1);

    ((pattern[0], pattern[1]), insertion[0])
}

fn parse(text: &str) -> Polymer {
    let mut lines = text.lines().map(str::trim);

    let template = lines.next().unwrap();

    assert_eq!(lines.next(), Some(""));

    let rules = lines
        .map(|line| parse_rule(line))
        .fold(BTreeMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });

    Polymer::new(template, rules)
}

fn min_max_by<T, I, F>(it: I, cmp: F) -> (Option<T>, Option<T>)
where
    T: Copy,
    I: Iterator<Item = T>,
    F: Fn(T, T) -> Ordering,
{
    let mut min = None;
    let mut max = None;

    for value in it {
        min = min.map_or(Some(value), |min| {
            Some(match cmp(value, min) {
                Ordering::Less => value,
                _ => min,
            })
        });

        max = max.map_or(Some(value), |max| {
            Some(match cmp(value, max) {
                Ordering::Greater => value,
                _ => max,
            })
        });
    }

    (min, max)
}

fn min_max<T, I>(it: I) -> (Option<T>, Option<T>)
where
    T: Copy + Borrow<T> + Ord,
    I: Iterator<Item = T>,
{
    min_max_by(it, |a, b| Ord::cmp(a.borrow(), b.borrow()))
}

fn run(text: &str, steps: usize) -> usize {
    let mut polymer = parse(text);
    for _ in 0..steps {
        polymer.evolve();
    }
    polymer.result()
}

pub(crate) fn solution1(text: &str) -> usize {
    run(text, 10)
}

pub(crate) fn solution2(text: &str) -> usize {
    run(text, 40)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod fourteen_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 1588);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 2188189693529);
    }
}

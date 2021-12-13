use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

use crate::graph::Graph;

const INPUT: &str = include_str!("../../inputs/12");

fn raw_parse(text: &str) -> Vec<(String, String)> {
    text.lines()
        .map(|line| {
            let mut ps = line.splitn(2, '-').map(ToString::to_string);
            (ps.next().unwrap(), ps.next().unwrap())
        })
        .collect()
}

fn parse(text: &str) -> Graph<String> {
    Graph::from_edges(raw_parse(text))
}

#[derive(Clone)]
struct State<'a, T> {
    path: Cow<'a, Vec<&'a T>>,
    non_repeatables: Cow<'a, BTreeMap<&'a T, usize>>,
    is_small: Arc<Box<dyn Fn(&'a T) -> bool + 'a>>,
    valid_occurrences: Arc<Box<dyn Fn(&BTreeMap<&'a T, usize>, &'a T) -> bool + 'a>>,
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for State<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.path)
    }
}

impl<'a, T: Ord> State<'a, T> {
    pub fn new<F1, F2>(is_small: F1, valid_occurrences: F2) -> Self
    where
        F1: Fn(&'a T) -> bool + 'a,
        F2: Fn(&BTreeMap<&'a T, usize>, &'a T) -> bool + 'a,
    {
        Self {
            path: Cow::Owned(Vec::new()),
            non_repeatables: Cow::Owned(BTreeMap::new()),
            is_small: Arc::new(Box::new(is_small)),
            valid_occurrences: Arc::new(Box::new(valid_occurrences)),
        }
    }

    pub fn is_small(&self, element: &'a T) -> bool {
        (self.is_small)(element)
    }

    pub fn get_occurrences(&self, element: &'a T) -> Option<usize> {
        if self.is_small(element) {
            return Some(self.non_repeatables.get(element).map(|x| *x).unwrap_or(0));
        }

        None
    }

    pub fn can_push(&self, element: &'a T) -> bool {
        if self.is_small(element) {
            (self.valid_occurrences)(self.non_repeatables.as_ref(), element)
        } else {
            true
        }
    }

    pub fn push(&mut self, element: &'a T) -> bool {
        if self.is_small(element) {
            if (self.valid_occurrences)(self.non_repeatables.as_ref(), element) {
                let count = self.get_occurrences(element).unwrap_or(0) + 1;
                self.non_repeatables.to_mut().insert(element, count);
            } else {
                return false;
            }
        }

        self.path.to_mut().push(element);
        true
    }
}

fn all_paths<'a, 'b: 'a, 'c: 'a, T, F1, F2>(
    graph: &'a Graph<T>,
    start: &'b T,
    end: &'c T,
    is_small: F1,
    valid_occurrences: F2,
) -> Vec<Vec<&'a T>>
where
    T: Ord + Clone,
    F1: Fn(&'a T) -> bool + 'a,
    F2: Fn(&BTreeMap<&'a T, usize>, &'a T) -> bool + 'a,
{
    let mut paths = Vec::new();
    let mut stack: VecDeque<State<'a, T>> = VecDeque::new();

    {
        let mut state = State::new(is_small, valid_occurrences);
        state.push(graph.nodes().find(|&n| n == start).unwrap());
        stack.push_back(state);
    }

    while let Some(state) = stack.pop_front() {
        for neighbor in graph.neighbors(*state.path.last().unwrap()) {
            if neighbor == end {
                let mut path: Vec<&'a T> = state.path.clone().into_owned();
                path.push(neighbor);
                paths.push(path);
            } else if neighbor != start && state.can_push(neighbor) {
                let mut state = state.clone();
                if state.push(neighbor) {
                    stack.push_back(state);
                }
            }
        }
    }

    paths
}

fn is_small_cave(cave: &String) -> bool {
    cave.chars().find(|c| c.is_uppercase()).is_none()
}

pub(crate) fn solution1(text: &str) -> usize {
    let graph = parse(text);
    let start = "start".to_owned();
    let end = "end".to_owned();
    let paths = all_paths(&graph, &start, &end, is_small_cave, |occurrences, key| {
        occurrences.get(key).map(|x| *x).unwrap_or(0) < 1
    });
    paths.len()
}

pub(crate) fn solution2(text: &str) -> usize {
    let graph = parse(text);
    let start = "start".to_owned();
    let end = "end".to_owned();
    let paths = all_paths(&graph, &start, &end, is_small_cave, |occurrences, key| {
        let max = if occurrences.values().map(|x| *x).max().unwrap_or(0) == 2 {
            1
        } else {
            2
        };
        occurrences.get(key).map(|x| *x).unwrap_or(0) < max
    });
    paths.len()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod twelve_tests {
    use super::{solution1, solution2};

    const INPUT1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT1), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(solution1(INPUT2), 19);
    }

    #[test]
    fn test3() {
        assert_eq!(solution1(INPUT3), 226);
    }

    #[test]
    fn test4() {
        assert_eq!(solution2(INPUT1), 36);
    }

    #[test]
    fn test5() {
        assert_eq!(solution2(INPUT2), 103);
    }

    #[test]
    fn test6() {
        assert_eq!(solution2(INPUT3), 3509);
    }
}

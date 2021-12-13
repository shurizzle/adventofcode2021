#![allow(dead_code)]

use bimap::BiBTreeMap;

pub struct Graph<T: Ord> {
    idx: usize,
    map: BiBTreeMap<T, usize>,
    graph: petgraph::graphmap::UnGraphMap<usize, ()>,
}

pub struct Neighbors<'a, T: Ord> {
    it: Option<petgraph::graphmap::Neighbors<'a, usize, petgraph::Undirected>>,
    graph: &'a Graph<T>,
}

impl<'a, T: Ord> Iterator for Neighbors<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.it {
            Some(ref mut it) => match it.next() {
                Some(v) => self.graph.map.get_by_right(&v),
                None => None,
            },
            None => None,
        }
    }
}

pub struct Nodes<'a, T: Ord> {
    it: petgraph::graphmap::Nodes<'a, usize>,
    graph: &'a Graph<T>,
}

impl<'a, T: Ord> Iterator for Nodes<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.it.next() {
            Some(v) => self.graph.map.get_by_right(&v),
            None => None,
        }
    }
}

impl<T: Ord> Graph<T> {
    pub fn new() -> Self {
        Self {
            idx: 0,
            map: BiBTreeMap::new(),
            graph: petgraph::graphmap::UnGraphMap::new(),
        }
    }

    fn get_or_insert_index(&mut self, v: T) -> usize {
        if let Some(i) = self.map.get_by_left(&v) {
            *i
        } else {
            let i = self.idx;
            self.idx += 1;
            self.map.insert(v, i);
            i
        }
    }

    pub fn add_node(&mut self, v: T) {
        let idx = self.get_or_insert_index(v);
        self.graph.add_node(idx);
    }

    pub fn remove_node(&mut self, v: &T) -> bool {
        if let Some(i) = self.map.get_by_left(v) {
            self.graph.remove_node(*i)
        } else {
            false
        }
    }

    pub fn contains_node(&mut self, v: &T) -> bool {
        if let Some(i) = self.map.get_by_left(v) {
            self.graph.contains_node(*i)
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.idx = 0;
        self.graph.clear();
    }

    pub fn add_edge(&mut self, a: T, b: T) {
        let a = self.get_or_insert_index(a);
        let b = self.get_or_insert_index(b);
        self.graph.add_edge(a, b, ());
    }

    pub fn remove_edge(&mut self, a: &T, b: &T) {
        if let Some(a) = self.map.get_by_left(a) {
            if let Some(b) = self.map.get_by_left(b) {
                self.graph.remove_edge(*a, *b);
            }
        }
    }

    pub fn contains_edge(&self, a: &T, b: &T) -> bool {
        if let Some(a) = self.map.get_by_left(a) {
            if let Some(b) = self.map.get_by_left(b) {
                return self.graph.contains_edge(*a, *b);
            }
        }

        false
    }

    pub fn neighbors<'a>(&'a self, a: &T) -> Neighbors<'a, T> {
        let it = if let Some(a) = self.map.get_by_left(a) {
            Some(self.graph.neighbors(*a))
        } else {
            None
        };

        Neighbors { it, graph: self }
    }

    pub fn nodes<'a>(&'a self) -> Nodes<'a, T> {
        Nodes {
            it: self.graph.nodes(),
            graph: self,
        }
    }

    pub fn from_edges<I: IntoIterator<Item = (T, T)>>(iterable: I) -> Self {
        let mut res = Self::new();

        for (a, b) in iterable.into_iter() {
            res.add_edge(a, b);
        }

        res
    }
}

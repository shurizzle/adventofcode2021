use std::{
    borrow::{Borrow, BorrowMut},
    vec::IntoIter,
};

pub type Matrix<T> = Vec<Vec<T>>;
pub type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
pub struct MatrixEnumeratedIterator<'a, T> {
    matrix: &'a Vec<Vec<T>>,
    pos: Coord,
}

impl<'a, T> MatrixEnumeratedIterator<'a, T> {
    pub fn new(matrix: &'a Vec<Vec<T>>) -> Self {
        Self {
            matrix,
            pos: (0, 0),
        }
    }
}

impl<'a, T> Iterator for MatrixEnumeratedIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while self.pos.0 < self.matrix.len() {
            if self.pos.1 < self.matrix[self.pos.0].len() {
                let res = (self.pos, &self.matrix[self.pos.0][self.pos.1]);

                self.pos.1 += 1;
                if self.pos.1 >= self.matrix[self.pos.0].len() {
                    self.pos.0 += 1;
                    self.pos.1 = 0;
                }

                return Some(res);
            }
            todo!()
        }

        None
    }
}

pub fn enum_iter<'a, T>(matrix: &'a Matrix<T>) -> MatrixEnumeratedIterator<'a, T> {
    MatrixEnumeratedIterator::new(matrix)
}

pub trait Navigator<T, I: Iterator<Item = Coord>> {
    fn navigate(&mut self, matrix: &Matrix<T>, coord: &Coord) -> I;
}

impl<I, T, F> Navigator<T, I> for F
where
    I: Iterator<Item = Coord>,
    F: Fn(&Matrix<T>, &Coord) -> I,
{
    fn navigate(&mut self, matrix: &Matrix<T>, coord: &Coord) -> I {
        self(matrix, coord)
    }
}

#[derive(Debug)]
pub struct IndexesIterator<'a, T, I: Iterator<Item = Coord>> {
    matrix: &'a Vec<Vec<T>>,
    indexes: I,
}

impl<'a, T, I: Iterator<Item = Coord>> IndexesIterator<'a, T, I> {
    pub fn new(matrix: &'a Vec<Vec<T>>, indexes: I) -> Self {
        Self { matrix, indexes }
    }
}

impl<'a, T, I: Iterator<Item = Coord>> Iterator for IndexesIterator<'a, T, I> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some((i, j)) = self.indexes.next() {
            if let Some(v) = self.matrix.get(i).and_then(|v| v.get(j)) {
                return Some(((i, j), v));
            }
        }

        None
    }
}

pub fn enum_navigate<'a, T, I, N, B>(
    matrix: &'a Matrix<T>,
    coord: &Coord,
    mut navigator: B,
) -> IndexesIterator<'a, T, I>
where
    I: Iterator<Item = Coord>,
    N: Navigator<T, I>,
    B: BorrowMut<N>,
{
    IndexesIterator::new(matrix, navigator.borrow_mut().navigate(matrix, coord))
}

pub fn cardinal_coords<'a, T>(matrix: &'a Matrix<T>, pos: &Coord) -> IntoIter<Coord> {
    let mut idxs = Vec::new();

    if matrix.get(pos.0).and_then(|v| v.get(pos.1)).is_none() {
        return idxs.into_iter();
    }

    if pos.0 > 0 && pos.1 < matrix[pos.0 - 1].len() {
        idxs.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 {
        idxs.push((pos.0, pos.1 - 1));
    }

    if pos.1 + 1 < matrix[pos.0].len() {
        idxs.push((pos.0, pos.1 + 1));
    }

    if pos.0 + 1 < matrix.len() && pos.1 < matrix[pos.0 + 1].len() {
        idxs.push((pos.0 + 1, pos.1));
    }

    idxs.into_iter()
}

pub fn navigate<'a, T, I, N, B, F, S, C>(
    matrix: &'a Matrix<T>,
    mut navigator: B,
    pos: C,
    mut state: S,
    mut reducer: F,
) -> S
where
    I: Iterator<Item = Coord>,
    N: Navigator<T, I>,
    B: BorrowMut<N>,
    F: FnMut(S, (Coord, &'a T), (Coord, &'a T)) -> (bool, S),
    C: Borrow<Coord>,
{
    let mut stack: Vec<(Coord, &'a T)> = Vec::new();
    let pos = (pos.borrow().0, pos.borrow().1);
    if let Some(value) = matrix.get(pos.0).and_then(|line| line.get(pos.1)) {
        stack.push((pos, value));
    }

    while let Some((prev_pos, prev_value)) = stack.pop() {
        for (pos, value) in enum_navigate::<_, I, N, _>(matrix, &prev_pos, navigator.borrow_mut()) {
            let (ok, s) = reducer(state, (prev_pos, prev_value), (pos, value));
            state = s;
            if ok {
                stack.push((pos, value));
            }
        }
    }

    state
}

pub fn dijkstra<T, C1, C2>(matrix: &Matrix<T>, start: C1, end: C2) -> Option<(Vec<Coord>, T)>
where
    T: Eq + std::hash::Hash + Copy + Clone + Ord + pathfinding::num_traits::Zero,
    C1: Borrow<Coord>,
    C2: Borrow<Coord>,
{
    let start = start.borrow();
    let end = end.borrow();

    pathfinding::directed::dijkstra::dijkstra(
        start,
        |pos| {
            cardinal_coords(matrix, pos)
                .map(|pos| (pos, matrix[pos.0][pos.1].clone()))
                .clone()
        },
        |pos| end == pos,
    )
}

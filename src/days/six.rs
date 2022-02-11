const INPUT: &str = include_str!("../../inputs/6");

pub struct FixedRingBuffer<T> {
    buffer: Vec<T>,
    head: usize,
}

impl<T> FixedRingBuffer<T> {
    pub fn new_with_buffer(buffer: Vec<T>) -> Self {
        Self { buffer, head: 0 }
    }

    fn calc_index(&self, idx: usize) -> Option<usize> {
        if idx < self.buffer.len() {
            Some((idx + self.head) % self.len())
        } else {
            None
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.calc_index(idx)
            .map(|index| unsafe { self.buffer.get_unchecked(index) })
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.calc_index(idx)
            .map(|index| unsafe { self.buffer.get_unchecked_mut(index) })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn back(&mut self) {
        self.head = if self.head == 0 {
            self.len()
        } else {
            self.head
        } - 1;
    }

    pub fn forth(&mut self) {
        self.head = (self.head + 1) % self.len();
    }

    pub fn into_iter(self) -> <Vec<T> as IntoIterator>::IntoIter {
        self.buffer.into_iter()
    }
}

impl<T: Default> FixedRingBuffer<T> {
    pub fn new(len: usize) -> Self {
        Self::new_with_buffer((0..len).map(|_| Default::default()).collect())
    }
}

impl<T> std::ops::Index<usize> for FixedRingBuffer<T> {
    type Output = <Vec<T> as std::ops::Index<usize>>::Output;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(val) = self.get(index) {
            val
        } else {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len(),
                index,
            )
        }
    }
}

impl<T> std::ops::IndexMut<usize> for FixedRingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.len();
        if let Some(val) = self.get_mut(index) {
            val
        } else {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                len, index,
            )
        }
    }
}

fn parse(text: &str) -> FixedRingBuffer<usize> {
    let mut fishes = FixedRingBuffer::new(9);
    text.trim()
        .split(',')
        .for_each(|x| fishes[x.parse::<usize>().unwrap()] += 1);
    fishes
}

fn evolve(fishes: &mut FixedRingBuffer<usize>) {
    fishes.forth();
    fishes[6] += fishes[8];
}

fn solve(text: &str, days: usize) -> usize {
    let mut fishes = parse(text);
    (0..days).for_each(|_| evolve(&mut fishes));
    fishes.into_iter().sum()
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(text, 80)
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(text, 256)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod six_tests {
    use crate::days::six::solution2;

    use super::solution1;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    pub fn test1() {
        assert_eq!(solution1(INPUT), 5934);
    }

    #[test]
    pub fn test2() {
        assert_eq!(solution2(INPUT), 26984457539);
    }
}

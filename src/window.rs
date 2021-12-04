#![allow(dead_code)]

pub struct Window<T> {
    window: Vec<T>,
    size: usize,
}

impl<T> Window<T> {
    pub fn new(size: usize) -> Self {
        Window {
            window: Vec::new(),
            size,
        }
    }

    pub fn is_full(&self) -> bool {
        self.window.len() == self.size
    }

    pub fn push(&mut self, value: T) {
        if self.is_full() {
            self.window.remove(0);
        }

        self.window.push(value);
    }

    pub fn window(&self) -> &Vec<T> {
        &self.window
    }

    pub fn len(&self) -> usize {
        self.window.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

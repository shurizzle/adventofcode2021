pub mod dec;
pub mod inc;
pub mod matrix;

use std::{
    borrow::Borrow,
    cmp::Ordering,
    io::{BufRead, BufReader},
};

pub fn is_eof<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<bool> {
    text.fill_buf().map(|b| b.is_empty())
}

pub fn read_line<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<String> {
    let mut line = String::new();
    text.read_line(&mut line)?;
    line.truncate(line.trim_end_matches('\n').len());
    line.truncate(line.trim_end_matches('\r').len());
    Ok(line)
}

pub fn min_max_by<T, I, F>(it: I, cmp: F) -> (Option<T>, Option<T>)
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

pub fn min_max<T, I>(it: I) -> (Option<T>, Option<T>)
where
    T: Copy + Borrow<T> + Ord,
    I: Iterator<Item = T>,
{
    min_max_by(it, |a, b| Ord::cmp(a.borrow(), b.borrow()))
}

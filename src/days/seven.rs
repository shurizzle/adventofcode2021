const INPUT: &str = include_str!("../../inputs/7");

fn parse(text: &str) -> Vec<usize> {
    text.trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[inline]
fn diff(a: usize, b: usize) -> usize {
    if b > a {
        b - a
    } else {
        a - b
    }
}

fn linear_calculate_fuel(positions: &Vec<usize>, position: usize) -> usize {
    let mut fuel: usize = 0;
    for &pos in positions {
        fuel += diff(position, pos);
    }

    fuel
}

fn sum(distance: usize) -> usize {
    distance * (distance + 1) / 2
}

fn sum_calculate_fuel(positions: &Vec<usize>, position: usize) -> usize {
    let mut fuel: usize = 0;
    for &pos in positions {
        fuel += sum(diff(position, pos));
    }

    fuel
}

fn solve(text: &str, calculate_fuel: fn(&Vec<usize>, usize) -> usize) -> usize {
    let positions = parse(text);
    let max = *positions.iter().max().unwrap();
    (0..max)
        .into_iter()
        .map(|i| calculate_fuel(&positions, i))
        .min()
        .unwrap()
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(text, linear_calculate_fuel)
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(text, sum_calculate_fuel)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod seven_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 37);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 168);
    }
}

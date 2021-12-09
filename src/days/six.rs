const INPUT: &str = include_str!("../../inputs/6");

fn parse(text: &str) -> [usize; 9] {
    let mut fishes = [0usize; 9];
    text.trim()
        .split(',')
        .for_each(|x| fishes[x.parse::<usize>().unwrap()] += 1);
    fishes
}

fn evolve(fishes: &mut [usize; 9]) {
    let first = fishes[0];
    for i in 0..8 {
        fishes[i] = fishes[i + 1];
    }
    fishes[8] = first;
    fishes[6] += first;
}

fn solve(text: &str, days: usize) -> usize {
    let mut fishes = parse(text);
    for _ in 0..days {
        evolve(&mut fishes);
    }
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

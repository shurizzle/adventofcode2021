const INPUT: &str = include_str!("../../inputs/17");

type N = i64;
type Coord = (N, N);
type Square = (Coord, Coord);
type Velocity = (N, N);

fn parse_range(text: &str) -> Option<(N, N)> {
    if let Some(dots) = text.find("..") {
        let start = text[0..dots].trim().parse::<N>().ok()?;
        let stop = text[(dots + 2)..].trim().parse::<N>().ok()?;

        if start > stop {
            Some((stop, start))
        } else {
            Some((start, stop))
        }
    } else {
        None
    }
}

fn parse(text: &str) -> Square {
    let text = text.trim();

    if text.starts_with("target area: ") {
        let text = text[13..].trim();

        if let Some(comma) = text.find(',') {
            let first = text[0..comma].trim();
            let second = text[(comma + 1)..].trim();

            let ((x1, x2), (y1, y2)) = if first.starts_with("x=") && second.starts_with("y=") {
                (
                    parse_range(&first[2..]).unwrap(),
                    parse_range(&second[2..]).unwrap(),
                )
            } else if first.starts_with("y=") && second.starts_with("x=") {
                (
                    parse_range(&second[2..]).unwrap(),
                    parse_range(&first[2..]).unwrap(),
                )
            } else {
                unreachable!("Invalid text")
            };

            return ((x1, y2), (x2, y1));
        }
    }

    unreachable!("Invalid text")
}

fn get_min_x(square: &Square) -> Option<N> {
    for x in 1..(square.0 .0) {
        let n = x * (x + 1) / 2;
        if square.0 .0 <= n && n <= square.1 .0 {
            return Some(x);
        }
    }
    None
}

fn hits_target(square: &Square, mut velocity: Velocity) -> Option<N> {
    let mut max: Option<N> = None;

    let (mut x, mut y) = velocity;

    while x <= square.1 .0 && y >= square.1 .1 {
        max = max.map_or_else(|| Some(y), |m| Some(m.max(y)));

        if square.0 .0 <= x && x <= square.1 .0 && square.0 .1 >= y && y >= square.1 .1 {
            return max;
        }

        velocity.1 -= 1;
        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        x += velocity.0;
        y += velocity.1;
    }

    None
}

fn get_max_y(square: &Square, x_velocity: N) -> (N, N) {
    let mut velocity = (x_velocity, square.1 .1 * -1);

    loop {
        let res = hits_target(square, velocity);
        if let Some(max) = res {
            return (velocity.1, max);
        } else {
            velocity.1 -= 1;
        }
    }
}

pub(crate) fn solution1(text: &str) -> N {
    let square = parse(text);
    let (_, max_y) = get_max_y(&square, get_min_x(&square).unwrap());
    max_y
}

pub(crate) fn solution2(text: &str) -> usize {
    let square = parse(text);
    let min_x = get_min_x(&square).unwrap();
    let mut count = 0;
    for x in min_x..=(square.1 .0) {
        for y in (square.1 .1)..=(get_max_y(&square, min_x).0) {
            if hits_target(&square, (x, y)).is_some() {
                count += 1;
            }
        }
    }
    count
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod seventeen_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 45);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 112);
    }
}

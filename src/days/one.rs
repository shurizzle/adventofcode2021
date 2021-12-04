use crate::window::Window;

const INPUT: &str = include_str!("../../inputs/one.txt");

pub(crate) fn window_cmp(text: &str, window_size: usize) -> usize {
    let mut window: Window<u32> = Window::new(window_size);
    let mut prev = None;
    let mut count: usize = 0;

    for line in text.lines() {
        window.push(line.parse::<u32>().unwrap());

        if window.is_full() {
            let sum: u32 = window.window().iter().sum();

            if prev.is_some() {
                if sum > prev.unwrap() {
                    count += 1;
                }
            }

            prev = Some(sum);
        }
    }

    count
}

pub(crate) fn solution1(text: &str) -> usize {
    window_cmp(text, 1)
}

pub(crate) fn solution2(text: &str) -> usize {
    window_cmp(text, 3)
}

pub fn solution() -> () {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod one_tests {
    use crate::days::one::{solution1, solution2};

    const TEST: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test1() -> () {
        assert_eq!(solution1(TEST), 7);
    }

    #[test]
    fn test2() -> () {
        assert_eq!(solution2(TEST), 5);
    }
}

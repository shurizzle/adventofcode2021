const INPUT: &str = include_str!("../../inputs/3");

fn split(text: &str) -> Vec<Vec<u8>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}

fn transpose(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut cols = None;
    let mut res = Vec::new();

    for v in matrix.into_iter() {
        if cols.is_none() {
            cols = Some(v.len());
        } else if cols.unwrap() != v.len() {
            panic!("Invalid number of cols");
        }

        for (i, val) in v.into_iter().enumerate() {
            if res.len() <= i {
                res.push(Vec::new());
            }

            res[i].push(val);
        }
    }

    res
}

#[derive(Copy, Clone, Debug)]
struct Count {
    population: usize,
    size: usize,
}

fn popcnt(v: &Vec<u8>) -> usize {
    v.iter().fold(0, |count, &val| count + (val as usize))
}

fn count(v: &Vec<u8>) -> Count {
    Count {
        population: popcnt(v),
        size: v.len(),
    }
}

fn gamma_rate(count: &Count) -> u8 {
    if count.population > count.size / 2 {
        1
    } else {
        0
    }
}

fn epsilon_rate(count: &Count) -> u8 {
    if count.population > count.size / 2 {
        0
    } else {
        1
    }
}

fn rates(count: Count) -> (u8, u8) {
    (gamma_rate(&count), epsilon_rate(&count))
}

pub(crate) fn solution1(text: &str) -> usize {
    let (gamma, epsilon) = transpose(split(text)).iter().map(count).map(rates).fold(
        (String::new(), String::new()),
        |(gamma, epsilon), (g1, e1)| (gamma + &g1.to_string(), epsilon + &e1.to_string()),
    );

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn filter(mut matrix: Vec<Vec<u8>>, bit_criteria: fn(&Count) -> u8) -> Vec<u8> {
    if matrix.is_empty() {
        return Vec::new();
    }

    for i in 0..matrix[0].len() {
        let count = Count {
            population: matrix.iter().map(|line| line[i] as usize).sum(),
            size: matrix.len(),
        };

        let bit = bit_criteria(&count);

        matrix = matrix.into_iter().filter(|line| line[i] == bit).collect();

        if matrix.len() == 1 {
            return matrix[0].clone();
        }
    }

    matrix[0].clone()
}

fn oxygen_bit_criteria(count: &Count) -> u8 {
    if count.population >= count.size - count.population {
        1
    } else {
        0
    }
}

fn co2_bit_criteria(count: &Count) -> u8 {
    if count.population >= count.size - count.population {
        0
    } else {
        1
    }
}

fn filter_and_cast(matrix: Vec<Vec<u8>>, bit_criteria: fn(&Count) -> u8) -> usize {
    usize::from_str_radix(
        &filter(matrix.clone(), bit_criteria)
            .into_iter()
            .fold(String::new(), |rate, val| rate + &val.to_string()),
        2,
    )
    .unwrap()
}

pub(crate) fn solution2(text: &str) -> usize {
    let matrix = split(text);
    let oxygen_generator_rating = filter_and_cast(matrix.clone(), oxygen_bit_criteria);
    let co2_scrubber_rating = filter_and_cast(matrix, co2_bit_criteria);

    oxygen_generator_rating * co2_scrubber_rating
}

pub fn solution() -> () {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod three_tests {
    use crate::days::three::{solution1, solution2};

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn test1() {
        assert_eq!(solution1(INPUT), 198);
    }

    #[test]
    pub fn test2() {
        assert_eq!(solution2(INPUT), 230);
    }
}

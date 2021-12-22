use std::{
    collections::{BTreeMap, HashMap},
    mem::take,
};

const INPUT: &str = include_str!("../../inputs/21");

trait ParsePlayer: Sized {
    fn create(name: &str) -> Option<Self>;
}

impl ParsePlayer for usize {
    fn create(id: &str) -> Option<Self> {
        Some(id.parse().ok()?)
    }
}

fn parse_player<P: ParsePlayer>(text: &str) -> Option<(P, usize)> {
    let text = text.trim();
    if text.starts_with("Player ") {
        let text = &text[7..];
        if let Some(pos) = text.find(' ') {
            let player = <P as ParsePlayer>::create(&text[..pos])?;
            let text = &text[pos..];

            if text.starts_with(" starting position: ") {
                let position = text[20..].parse::<usize>().ok()?;

                return Some((player, position));
            }
        }
    }

    None
}

fn parse<P: ParsePlayer + Ord>(text: &str) -> BTreeMap<P, usize> {
    let mut players = BTreeMap::new();
    for line in text.trim().lines() {
        let (player, position) = parse_player(line).unwrap();
        players.insert(player, position);
    }
    players
}

trait Die {
    fn roll(&mut self) -> usize;

    fn rolln(&mut self, times: usize) -> usize {
        let mut sum = 0;

        for _ in 0..times {
            sum += self.roll();
        }

        sum
    }
}

#[derive(Copy, Clone, Debug)]
struct DeterministicDie {
    position: usize,
    max: usize,
}

impl DeterministicDie {
    pub fn new(max: usize) -> Self {
        Self { position: 1, max }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let current = self.position;

        self.position += 1;
        if self.position > self.max {
            self.position = 1;
        }

        current
    }

    fn rolln(&mut self, times: usize) -> usize {
        let mut sum = 0;

        for _ in 0..times {
            sum += self.roll();
        }

        sum
    }
}

#[derive(Clone, Debug)]
struct GameState<P> {
    pub rolled: usize,
    pub players: BTreeMap<P, (usize, usize)>,
}

impl<P> GameState<P>
where
    P: Ord,
{
    pub fn new(players: BTreeMap<P, usize>) -> Self {
        Self {
            rolled: 0,
            players: players.into_iter().map(|(p, x)| (p, (x, 0))).collect(),
        }
    }

    fn _rolln<D: Die>(rolled: &mut usize, die: &mut D, times: usize) -> usize {
        let res = die.rolln(times);
        *rolled += times;
        res
    }

    pub fn step<D: Die, F: Field, Fun>(&mut self, die: &mut D, field: &mut F, cond: Fun) -> bool
    where
        D: Die,
        F: Field,
        Fun: Fn(&P, usize) -> bool,
    {
        for (p, v) in self.players.iter_mut() {
            let amount = Self::_rolln(&mut self.rolled, die, 3);
            field.step(&mut v.0, amount);
            v.1 += v.0;

            if cond(&p, v.1) {
                return true;
            }
        }
        false
    }
}

trait Field {
    fn step(&mut self, current: &mut usize, amount: usize);
}

#[derive(Copy, Clone, Debug)]
struct MaxValueField {
    max: usize,
}

impl MaxValueField {
    pub fn new(max: usize) -> Self {
        Self { max: max + 1 }
    }
}

impl Field for MaxValueField {
    fn step(&mut self, current: &mut usize, mut amount: usize) {
        amount += *current;
        if amount >= self.max {
            let range = self.max - 1;
            let q = 1 + (amount - self.max) / range;
            amount -= q * range;
        }
        *current = amount;
    }
}

#[derive(Clone, Debug)]
struct Game<F, D, P> {
    state: GameState<P>,
    die: D,
    field: F,
}

impl<F, D, P> Game<F, D, P>
where
    F: Field,
    D: Die,
    P: Ord,
{
    pub fn new(players: BTreeMap<P, usize>, die: D, field: F) -> Self {
        Self {
            state: GameState::new(players),
            die,
            field,
        }
    }

    pub fn rolled(&self) -> usize {
        self.state.rolled
    }

    pub fn step_while<'a, Fun>(&'a mut self, cond: Fun) -> bool
    where
        Fun: Fn(&P, usize) -> bool,
    {
        self.state.step(&mut self.die, &mut self.field, cond)
    }

    pub fn players<'a>(&'a self) -> &'a BTreeMap<P, (usize, usize)> {
        &self.state.players
    }
}

fn generate_frequencies(min: usize, max: usize, times: usize) -> Vec<(usize, usize)> {
    let mut sums = vec![0usize];
    let mut freqs: BTreeMap<usize, usize> = BTreeMap::new();

    for _ in 0..times {
        for sum in take(&mut sums) {
            for v in min..=max {
                sums.push(sum + v);
            }
        }
    }

    for sum in sums {
        *freqs.entry(sum).or_insert(0) += 1;
    }

    freqs.into_iter().collect()
}

fn modulo(mut value: usize, min: usize, max: usize) -> usize {
    if value >= max {
        let range = max - min;
        let q = 1 + (value - max) / range;
        value -= q * range;
    }

    return value;
}

fn solve_recursive(
    pos_1: usize,
    pos_2: usize,
    score_1: usize,
    score_2: usize,
    cache: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
    freqs: &Vec<(usize, usize)>,
) -> (usize, usize) {
    let key = (pos_1, pos_2, score_1, score_2);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    if score_1 >= 21 {
        return (1, 0);
    }
    if score_2 >= 21 {
        return (0, 1);
    }

    let mut total_p1_wins = 0;
    let mut total_p2_wins = 0;

    for &(roll, freq) in freqs {
        let new_position = modulo(pos_1 + roll, 1, 11);
        let new_score = score_1 + new_position;

        let (p2_wins, p1_wins) =
            solve_recursive(pos_2, new_position, score_2, new_score, cache, freqs);

        total_p1_wins += freq * p1_wins;
        total_p2_wins += freq * p2_wins;
    }

    cache.insert(
        (pos_1, pos_2, score_1, score_2),
        (total_p1_wins, total_p2_wins),
    );

    return (total_p1_wins, total_p2_wins);
}

pub(crate) fn solution1(text: &str) -> usize {
    let players: BTreeMap<usize, usize> = parse(text);
    let mut game = Game::new(players, DeterministicDie::new(100), MaxValueField::new(10));

    while !game.step_while(|_, i| i >= 1000) {}
    let min = game.players().iter().map(|(_, i)| i.1).min();
    min.unwrap() * game.rolled()
}

pub(crate) fn solution2(text: &str) -> usize {
    let players: BTreeMap<usize, usize> = parse(text);
    let mut it = players.into_iter();
    let (wins_p1, wins_p2) = solve_recursive(
        it.next().unwrap().1,
        it.next().unwrap().1,
        0,
        0,
        &mut HashMap::new(),
        &generate_frequencies(1, 3, 3),
    );
    wins_p1.max(wins_p2)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod twentyone_tests {
    use super::{solution1, solution2};

    const INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT), 739785);
    }

    #[test]
    fn test2() {
        assert_eq!(solution2(INPUT), 444356092776315);
    }
}

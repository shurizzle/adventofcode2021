pub mod days;
pub(crate) mod graph;
pub(crate) mod utils;
pub(crate) mod window;

const DAYS: [fn() -> (); 12] = [
    days::one::solution,
    days::two::solution,
    days::three::solution,
    days::four::solution,
    days::five::solution,
    days::six::solution,
    days::seven::solution,
    days::eight::solution,
    days::nine::solution,
    days::ten::solution,
    days::eleven::solution,
    days::twelve::solution,
];

fn main() {
    match std::env::args().len() {
        1 => {
            for day in 1..=DAYS.len() {
                run(day);
            }
        }
        2 => {
            let day_string = std::env::args().nth(1).unwrap();
            if let Ok(day) = day_string.parse::<usize>() {
                run(day);
            } else {
                println!("'{}' is not a valid day", day_string);
            }
        }
        _ => {
            println!("Invalid arguments");
        }
    }
}

fn run(day: usize) {
    let idx = day - 1;
    if !(0..DAYS.len()).contains(&idx) {
        println!("Day {} does not exist", day);
        return;
    }

    println!("Day {}:", day);
    DAYS[idx]();
}

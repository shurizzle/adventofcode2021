pub mod days;
pub(crate) mod utils;
pub(crate) mod window;

const DAYS: [fn() -> (); 8] = [
    days::one::solution,
    days::two::solution,
    days::three::solution,
    days::four::solution,
    days::five::solution,
    days::six::solution,
    days::seven::solution,
    days::eight::solution,
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
    if day == 0 || day > DAYS.len() {
        println!("Day {} does not exist", day);
    }

    println!("Day {}:", day);
    DAYS[day - 1]();
}

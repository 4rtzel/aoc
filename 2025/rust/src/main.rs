use std::env;
use std::fs;
use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn str_to_day(day: &String) -> Option<u32> {
    day[1..].parse::<u32>().ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Too few arguments");
        return;
    }

    let file = fs::File::open(&args[2]).expect("Unable to open a file");
    let reader = io::BufReader::new(file);
    match str_to_day(&args[1]) {
        Some(day) => {
            match day {
                1  => day1::run(reader),
                2  => day2::run(reader),
                3  => day3::run(reader),
                4  => day4::run(reader),
                5  => day5::run(reader),
                _ => println!("Day {} is not implemented yet", day),
            }
        },
        None => println!("Error parsing input"),
    }
}

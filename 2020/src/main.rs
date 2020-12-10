use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn str_to_day(day: &String) -> Option<u32> {
    day[1..].parse::<u32>().ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Too few arguments");
        return;
    }

    match str_to_day(&args[1]) {
        Some(day) => {
            match day {
                1 => day1::run(&args[2]),
                2 => day2::run(&args[2]),
                3 => day3::run(&args[2]),
                4 => day4::run(&args[2]),
                5 => day5::run(&args[2]),
                6 => day6::run(&args[2]),
                7 => day7::run(&args[2]),
                8 => day8::run(&args[2]),
                9 => day9::run(&args[2]),
                _ => println!("Day {} is not implemented yet", day),
            }
        },
        None => println!("Error parsing input"),
    }
}

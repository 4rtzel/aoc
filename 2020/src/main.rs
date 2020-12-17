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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

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
                1  => day1::run(&args[2]),
                2  => day2::run(&args[2]),
                3  => day3::run(&args[2]),
                4  => day4::run(&args[2]),
                5  => day5::run(&args[2]),
                6  => day6::run(&args[2]),
                7  => day7::run(&args[2]),
                8  => day8::run(&args[2]),
                9  => day9::run(&args[2]),
                10 => day10::run(&args[2]),
                11 => day11::run(&args[2]),
                12 => day12::run(&args[2]),
                13 => day13::run(&args[2]),
                14 => day14::run(&args[2]),
                15 => day15::run(&args[2]),
                16 => day16::run(&args[2]),
                17 => day17::run(&args[2]),
                _ => println!("Day {} is not implemented yet", day),
            }
        },
        None => println!("Error parsing input"),
    }
}

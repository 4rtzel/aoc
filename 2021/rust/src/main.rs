use std::env;
use std::fs;
use std::io;

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
mod day18;
mod day19;

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
                6  => day6::run(reader),
                7  => day7::run(reader),
                8  => day8::run(reader),
                9  => day9::run(reader),
                10  => day10::run(reader),
                11  => day11::run(reader),
                12  => day12::run(reader),
                13  => day13::run(reader),
                14  => day14::run(reader),
                15  => day15::run(reader),
                16  => day16::run(reader),
                17  => day17::run(reader),
                18  => day18::run(reader),
                19  => day19::run(reader),
                _ => println!("Day {} is not implemented yet", day),
            }
        },
        None => println!("Error parsing input"),
    }
}

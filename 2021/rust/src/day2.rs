use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(commands: &Vec<(String, u32)>) {
    let mut position = 0;
    let mut depth = 0;
    for command in commands {
        match command.0.as_str() {
            "forward" => position += command.1,
            "down" => depth += command.1,
            "up" => depth -= command.1,
            _ => panic!("Command is unrecognized"),
        }
    }
    println!("{}", position * depth);
}

fn part_two(commands: &Vec<(String, u32)>) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command.0.as_str() {
            "forward" => {
                position += command.1;
                depth += aim * command.1;
            },
            "down" => aim += command.1,
            "up" => aim -= command.1,
            _ => panic!("Command is unrecognized"),
        }
    }
    println!("{}", position * depth);
}

pub fn run(reader: io::BufReader<File>) {
    let mut commands: Vec<(String, u32)> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(' ');
        let direction: String = split.next().unwrap().to_string();
        let value: u32 = split.next().unwrap().parse().unwrap();
        commands.push((direction, value));
    }
    part_one(&commands);
    part_two(&commands);
}

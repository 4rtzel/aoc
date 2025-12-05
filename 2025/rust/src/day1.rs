use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(spins: &Vec<(i32, u32)>) {
    let mut dial: i32 = 50;
    let mut zeros: u32 = 0;
    for (direction, spin) in spins {
        dial = (dial + if *direction == -1 { 100 - *spin as i32 } else { *spin as i32 }) % 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    println!("{}", zeros);
}

fn part_two(spins: &Vec<(i32, u32)>) {
    let mut dial: i32 = 50;
    let mut zeros: i32 = 0;
    for (direction, spin) in spins {
        let mut starts_at_zero = dial == 0;
        dial += direction * *spin as i32;
        while dial < 0 {
            dial += 100;
            if starts_at_zero {
                starts_at_zero = false;
            } else {
                zeros += 1;
            }
        }
        if dial == 0 {
            zeros += 1;
        }
        zeros += dial / 100;
        dial %= 100;
    }
    println!("{}", zeros);
}

pub fn run(reader: io::BufReader<File>) {
    let spins: Vec<(i32, u32)> = reader.lines().map(|line| {
        let line = line.unwrap();
        let direction = if line.chars().next() == Some('L') { -1 } else { 1 };
        (direction, line.get(1..).unwrap().parse::<u32>().unwrap())
    }).collect();
    part_one(&spins);
    part_two(&spins);
}

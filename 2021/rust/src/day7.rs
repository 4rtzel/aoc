use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run(reader: io::BufReader<File>) {
    let mut numbers: Vec<u32> = reader
        .lines()
        .map(|l| l.unwrap())
        .take(1)
        .collect::<String>()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    numbers.sort();

    let mut min_sum_1: u64 = std::u64::MAX;
    let mut min_sum_2: u64 = std::u64::MAX;
    for i in numbers[0]..=*numbers.last().unwrap() {
        let sum_1: u64 = numbers.iter().map(|n| (*n as i32 - i as i32).abs()).fold(0, |acc, n| acc + n as u64);
        let fuel_needed = |n| (1 + n) * n / 2;
        let sum_2: u64 = numbers.iter().map(|n| fuel_needed((*n as i32 - i as i32).abs())).fold(0, |acc, n| acc + n as u64);
        if sum_1 < min_sum_1 {
            min_sum_1 = sum_1;
        } else if sum_2 < min_sum_2 {
            min_sum_2 = sum_2;
        } else {
            break;
        }
    }
    println!("{}\n{}", min_sum_1, min_sum_2);
}

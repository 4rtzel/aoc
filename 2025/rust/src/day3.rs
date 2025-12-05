use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(banks: &Vec<Vec<u64>>) {
    println!("{}", both(banks, 2));
}

fn part_two(banks: &Vec<Vec<u64>>) {
    println!("{}", both(banks, 12));
}

fn both(banks: &Vec<Vec<u64>>, length: usize) -> u64 {
    let mut sum: u64 = 0;
    for bank in banks {
        let mut from: usize = 0;
        let mut value: u64 = 0;
        for period in (0..length).rev() {
            let (index, digit) = find_max(&bank, from, period);
            value += digit * 10u64.pow(period as u32);
            from = index + 1;
        }
        sum += value;
    }
    return sum;
}

fn find_max(bank: &Vec<u64>, from: usize, period: usize) -> (usize, u64) {
    bank[from..bank.len()]
        .iter()
        .enumerate()
        .rev()
        .skip(period)
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, c)| (i + from, *c))
        .unwrap()
}

pub fn run(reader: io::BufReader<File>) {
    let banks: Vec<Vec<u64>> = reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();
    part_one(&banks);
    part_two(&banks);
}

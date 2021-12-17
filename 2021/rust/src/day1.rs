use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(numbers: &Vec<u32>) {
    let mut total = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] < numbers[i] {
            total += 1;
        }
    }
    println!("{}", total);
}

fn part_two(numbers: &Vec<u32>) {
    let mut total = 0;
    for i in 3..numbers.len() {
        if (numbers[i - 3] + numbers[i - 2] + numbers[i - 1]) < (numbers[i - 2] + numbers[i - 1] + numbers[i]) {
            total += 1;
        }
    }
    println!("{}", total);
}

pub fn run(reader: io::BufReader<File>) {
    let numbers: Vec<u32> = reader.lines().map(|x| x.unwrap().parse::<u32>().unwrap()).collect();
    part_one(&numbers);
    part_two(&numbers);
}

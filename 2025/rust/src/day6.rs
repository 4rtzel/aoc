use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(column_numbers: &Vec<Vec<String>>) {
    let mut sum: u64 = 0;
    for column in column_numbers {
        let op: u64 = if column.last().unwrap().contains("*") { 1 } else { 0 };
        let numbers = column.iter().take(column.len() - 1).map(|v| v.trim().parse::<u64>().unwrap());
        sum += if op == 1 { numbers.product::<u64>() } else { numbers.sum() };
    }
    println!("{sum}");
}

fn part_two(column_numbers: &Vec<Vec<String>>) {
    let mut sum: u64 = 0;
    for column in column_numbers {
        let op: u64 = if column.last().unwrap().contains("*") { 1 } else { 0 };
        let mut result: u64 = op;
        for i in 0..column[0].len() {
            let number = column
                .iter()
                .take(column.len() - 1)
                .map(|v| v.chars().nth(i).unwrap())
                .collect::<String>()
                .trim()
                .parse::<u64>().unwrap();
            result = if op == 1 { result * number } else { result + number };
        }
        sum += result;
    }
    println!("{sum}");
}

pub fn run(reader: io::BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut column_numbers: Vec<Vec<String>> = vec![];
    let mut column_from = 0usize;
    for column in 0..lines[0].len() {
        if lines.iter().all(|v| v.chars().nth(column) == Some(' ')) {
            let numbers = lines.iter().map(|v| v[column_from..column].to_string()).collect();
            column_numbers.push(numbers);
            column_from = column + 1;
        }
    }
    let numbers = lines.iter().map(|v| v[column_from..].to_string()).collect();
    column_numbers.push(numbers);
    part_one(&column_numbers);
    part_two(&column_numbers);
}

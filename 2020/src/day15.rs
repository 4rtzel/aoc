use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let numbers: Vec<usize> = reader
        .lines()
        .map(|l| l.unwrap())
        .nth(0)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut last_seen: Vec<usize> = vec![0; 30_000_000];
    for (i, v) in numbers.iter().enumerate() {
        last_seen[*v] = i + 1;
    }

    let mut last: usize = 0;
    for i in numbers.len()..last_seen.len() - 1 {
        if i == 2020 - 1 {
            println!("{}", last);
        }
        let next = match last_seen[last] {
            0 => 0,
            v => i - v + 1,
        };
        last_seen[last] = i + 1;
        last = next;
    }
    println!("{}", last);
}

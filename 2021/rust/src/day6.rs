use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

pub fn run(reader: io::BufReader<File>) {
    let numbers: Vec<u32> = reader
        .lines()
        .map(|l| l.unwrap())
        .take(1)
        .collect::<String>()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut fishs: HashMap<u32, u64> = HashMap::new();
    for number in numbers.iter() {
        *fishs.entry(*number).or_insert(0) += 1;
    }

    for day in 0..256 {
        let mut fishs_temp = HashMap::new();
        for (key, val) in fishs.iter() {
            if *key == 0 {
                *fishs_temp.entry(8).or_insert(0) += val;
                *fishs_temp.entry(6).or_insert(0) += val;
            } else {
                *fishs_temp.entry(key - 1).or_insert(0) += val;
            }
        }
        fishs = fishs_temp;
        if day == 79 {
            println!("{}", fishs.values().fold(0, |acc, v| acc + v));
        }
    }

    println!("{}", fishs.values().fold(0, |acc, v| acc + v));
}

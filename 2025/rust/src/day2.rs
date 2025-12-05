use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(ranges: &Vec<Vec<u64>>) {
    let mut doubles: u64 = 0;
    for range in ranges {
        for value in range[0]..=range[1] {
            let v = value.to_string();
            if v.len() % 2 == 0 && v[0..v.len() / 2] == v[v.len() / 2..] {
                doubles += value;
            }
        }
    }
    println!("{doubles}");
}

fn part_two(ranges: &Vec<Vec<u64>>) {
    let mut doubles: u64 = 0;
    for range in ranges {
        for value in range[0]..=range[1] {
            let v = value.to_string();
            for repeated_part_size in 1..=v.len() / 2 {
                if v.len() % repeated_part_size != 0 {
                    continue;
                }
                let repeats = v.len() / repeated_part_size;
                if v[0..repeated_part_size].repeat(repeats) == v {
                    doubles += value;
                    break;
                }
            }
        }
    }
    println!("{doubles}");
}

pub fn run(reader: io::BufReader<File>) {
    let line: String = reader.lines().nth(0).unwrap().unwrap();
    let ranges: Vec<Vec<u64>> = line
        .split(',')
        .map(|r| { r.split('-').take(2).map(|v| v.parse::<u64>().unwrap()).collect() })
        .collect();
    part_one(&ranges);
    part_two(&ranges);
}

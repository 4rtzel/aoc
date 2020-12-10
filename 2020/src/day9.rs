use std::fs;
use std::io;
use std::io::BufRead;

fn has_sum(numbers: &Vec<u64>, from: usize, current: usize) -> bool {
    for i in from..current {
        for j in from..current {
            if numbers[i] == numbers[j] {
                continue;
            }
            if numbers[i] + numbers[j] == numbers[current] {
                return true;
            }
        }
    }
    false
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    const PREAMBLE_SIZE: usize = 25;
    let numbers: Vec<u64> = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let invalid = numbers.iter()
                         .skip(PREAMBLE_SIZE)
                         .enumerate()
                         .filter(|(i, _)| !has_sum(&numbers, *i, i + PREAMBLE_SIZE))
                         .nth(0)
                         .unwrap()
                         .1;
    println!("{}", invalid);
    for i in 0..numbers.len() {
        let mut acc = 0;
        let mut j = i;
        while acc < *invalid {
            acc += numbers[j];
            j += 1;
        }
        if acc == *invalid {
            println!("{}", numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap());
            break;
        }
    }
}

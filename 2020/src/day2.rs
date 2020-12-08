use std::fs;
use std::io;
use std::io::BufRead;

use regex;

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);
    let re = regex::Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut valid_count = 0;
    let mut valid_count2 = 0;

    for line in reader.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            let min = cap[1].parse::<usize>().unwrap();
            let max = cap[2].parse::<usize>().unwrap();
            let count = cap[4].matches(&cap[3]).count();

            // part one
            if count >= min && count <= max {
                valid_count += 1;
            }

            // part two
            let ch = cap[3].as_bytes()[0];
            if ((cap[4].as_bytes()[min - 1] == ch) as u8 ^ (cap[4].as_bytes()[max - 1] == ch) as u8) == 1 {
                valid_count2 += 1;
            }

        }
    }
    println!("{}", valid_count);
    println!("{}", valid_count2);
}

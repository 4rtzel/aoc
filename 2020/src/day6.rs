use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut answers_any: HashSet<char> = HashSet::new();
    let mut answers_all: HashSet<char> = HashSet::new();
    let mut total_any = 0;
    let mut total_all = 0;
    let mut first = true;
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {
            total_any += answers_any.len();
            total_all += answers_all.len();
            answers_any.clear();
            answers_all.clear();
            first = true;
        } else {
            answers_any.extend(line.chars());
            answers_all = if first {
                first = false;
                line.chars().collect()
            } else {
                answers_all.intersection(&line.chars().collect()).copied().collect()
            };
        }
    }
    println!("{}", total_any);
    println!("{}", total_all);
}

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn print_max_min_diff(pairs: &HashMap<(char, char), u64>) {
    let mut occurences: HashMap<char, u64> = HashMap::new();
    for (pair, count) in pairs.iter() {
        *occurences.entry(pair.1).or_insert(0) += count;
    }
    let min_elem = occurences.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
    let max_elem = occurences.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("{}", max_elem.1 - min_elem.1);
}

pub fn run(reader: io::BufReader<File>) {
    let mut lines = reader.lines().map(|l| l.unwrap()).filter(|l| !l.is_empty());
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    for i in 1..template.len() {
        *pairs.entry((template[i - 1], template[i])).or_insert(0) += 1;
    }
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let pair: (char, char) = (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap());
        let elem: char = line.chars().nth(6).unwrap();
        rules.insert(pair, elem);
    }
    for i in 0..40 {
        let p: Vec<((char, char), u64)> = pairs.iter().map(|(&p, &k)| (p, k)).collect();
        for (pair, count) in p.iter() {
            *pairs.entry(*pair).or_insert(0) -= count;
            let middle = *rules.get(pair).unwrap();
            *pairs.entry((pair.0, middle)).or_insert(0) += count;
            *pairs.entry((middle, pair.1)).or_insert(0) += count;
        }
        if i == 9 {
            print_max_min_diff(&pairs);
        }
    }
    print_max_min_diff(&pairs);
}

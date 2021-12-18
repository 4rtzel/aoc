use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

pub fn run(reader: io::BufReader<File>) {
    let symbol_map: HashMap<char, (char, u32, u64)> = HashMap::from([
        (')', ('(', 3, 1)),
        (']', ('[', 57, 2)),
        ('}', ('{', 1197, 3)),
        ('>', ('<', 25137, 4)),
    ]);

    let mut score1 = 0;
    let mut scores: Vec<u64> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut stack = Vec::new();
        let mut discarded = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let s = symbol_map.get(&c).unwrap();
                    if *stack.last().unwrap() != s.0 {
                        score1 += s.1;
                        discarded = true;
                        break;
                    }
                    stack.pop();
                }
                _ => panic!("Unexpected input"),
            }
        }
        if !discarded && stack.len() > 0 {
            let mut score: u64 = 0;
            for c in stack.iter().rev() {
                score = score * 5 + match c {
                    '(' => symbol_map.get(&')').unwrap().2,
                    '[' => symbol_map.get(&']').unwrap().2,
                    '{' => symbol_map.get(&'}').unwrap().2,
                    '<' => symbol_map.get(&'>').unwrap().2,
                    _ => panic!("Unexpected input"),
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}\n{}", score1, scores[scores.len() / 2]);
}

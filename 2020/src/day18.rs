use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Op {
    Number(i64),
    Plus,
    Mult,
    OpenPar,
    ClosePar,
}

fn find_highest_prio_1(expr: &Vec<Op>) -> usize {
    let mut prio = 0;
    let mut max_prio = 0;
    let mut from = 0;
    for (i, op) in expr.iter().enumerate() {
        if let Op::OpenPar = op {
            prio += 1;
        } else if let Op::ClosePar = op {
            prio -= 1;
        }
        if prio > max_prio {
            max_prio = prio;
            from = i;
        }
    }
    from
}

fn find_highest_prio_2(expr: &Vec<Op>) -> usize {
    let mut prio = 0;
    let mut max_prio = 0;
    let mut from = 0;
    for i in 0..expr.len() {
        let mut plus_prio = 0;
        if let Op::OpenPar = expr[i] {
            prio += 2;
        } else if let Op::ClosePar = expr[i] {
            prio -= 2;
        } else if let Op::Number(_) = expr[i] {
            if i < expr.len() - 1 {
                if let Op::Plus = expr[i + 1] {
                    plus_prio = 1;
                }
            }
        }
        if prio + plus_prio > max_prio {
            max_prio = prio + plus_prio;
            from = i;
        }
    }
    from
}

fn solve(mut expr: Vec<Op>, find_highest_prio: fn(&Vec<Op>) -> usize) -> i64 {
    while expr.len() != 1 {
        let mut from = find_highest_prio(&expr);
        if let Op::OpenPar = expr[from] {
            if let Op::ClosePar = expr[from + 2] {
                expr[from] = expr[from + 1];
                expr.drain(from + 1..=from + 2);
                continue;
            }
            from += 1;
        }
        expr[from] = Op::Number(
            match (expr[from], expr[from + 1], expr[from + 2]) {
                (Op::Number(v1), Op::Plus, Op::Number(v2)) => v1 + v2,
                (Op::Number(v1), Op::Mult, Op::Number(v2)) => v1 * v2,
                (_, _, _) => unreachable!(),
            }
        );
        expr.drain(from + 1..=from + 2);
    }
    if let Op::Number(v) = expr[0] { v } else { 0 }
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut expressions: Vec<Vec<Op>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap().replace("(", "( ").replace(")", " )")) {
        let mut expr: Vec<Op> = Vec::new();
        println!("{}", line);
        for token in line.split(' ') {
            expr.push(
                match token {
                    "+" => Op::Plus,
                    "*" => Op::Mult,
                    "(" => Op::OpenPar,
                    ")" => Op::ClosePar,
                    _   => Op::Number(token.parse().unwrap()),
                }
            )
        }
        expressions.push(expr);
    }
    let mut total1 = 0;
    let mut total2 = 0;
    for expr in expressions.iter() {
        total1 += solve(expr.clone(), find_highest_prio_1);
        total2 += solve(expr.clone(), find_highest_prio_2);
    }
    println!("{}", total1);
    println!("{}", total2);
}

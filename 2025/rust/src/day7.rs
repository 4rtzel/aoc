use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy)]
enum Type {
    Empty,
    Splitter,
    Beam(u64),
}

fn both(mut manifold: Vec<Vec<Type>>) {
    let mut splits = 0u64;
    for i in 1..manifold.len() {
        for j in 0..manifold[i].len() {
            match (manifold[i - 1][j], manifold[i][j]) {
                (Type::Beam(x), Type::Empty) => { manifold[i][j] = Type::Beam(x) },
                (Type::Beam(x), Type::Splitter) => {
                    splits += 1;
                    if j > 0 {
                        manifold[i][j - 1] = get_beam_value_for(manifold[i][j - 1], x);
                    }
                    if j + 1 < manifold[i].len() {
                        manifold[i][j + 1] = get_beam_value_for(manifold[i][j + 1], x);
                    }
                },
                (Type::Beam(x), Type::Beam(y)) => { manifold[i][j] = Type::Beam(x + y) },
                _ => {},
            }
        }
    }
    println!("{splits}");
    println!("{}", manifold.last().unwrap().iter().map(|v| if let Type::Beam(x) = v { *x } else { 0u64 }).sum::<u64>());
}

fn get_beam_value_for(t: Type, new_x: u64) -> Type {
    if let Type::Beam(prev_x) = t {
        Type::Beam(prev_x + new_x)
    } else {
        Type::Beam(new_x)
    }
}

pub fn run(reader: io::BufReader<File>) {
    let manifold: Vec<Vec<Type>> = reader
        .lines()
        .map(|l|
            l.unwrap().chars().map(|c|
                match c {
                    '.' => Type::Empty,
                    '^' => Type::Splitter,
                    _ => Type::Beam(1),
                }
            ).collect()
        )
        .collect();
    both(manifold);
}

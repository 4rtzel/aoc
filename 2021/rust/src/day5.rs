use std::fs::File;
use std::io;
use std::io::BufRead;
use std::cmp::{min, max};

pub fn run(reader: io::BufReader<File>) {
    let mut field1: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut field2: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    for line in reader.lines().map(|l| l.unwrap()) {
        let coords: Vec<usize> = line
            .replace(|c: char| !c.is_digit(10), " ")
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let x_min = min(coords[0], coords[2]);
        let x_max = max(coords[0], coords[2]);
        let y_min = min(coords[1], coords[3]);
        let y_max = max(coords[1], coords[3]);
        if coords[0] == coords[2] || coords[1] == coords[3] {
            for i in x_min..=x_max {
                for j in y_min..=y_max {
                    field1[i][j] += 1;
                    field2[i][j] += 1;
                }
            }
        } else {
            for i in 0..=(x_max - x_min) {
                let x = coords[0] as i32 + i as i32 * if coords[0] < coords[2] { 1 } else { -1 };
                let y = coords[1] as i32 + i as i32 * if coords[1] < coords[3] { 1 } else { -1 };
                field2[x as usize][y as usize] += 1;
            }
        }
    }

    let mut overlaps = 0;
    for row in field1.iter() {
        overlaps += row.iter().filter(|&n| *n > 1).count();
    }
    println!("{}", overlaps);
    overlaps = 0;
    for row in field2.iter() {
        overlaps += row.iter().filter(|&n| *n > 1).count();
    }
    println!("{}", overlaps);
}

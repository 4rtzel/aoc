use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

pub fn run(reader: io::BufReader<File>) {
    let mut points: HashSet<(u32, u32)> = HashSet::new();
    let mut folds: Vec<(char, u32)> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()).filter(|l| !l.is_empty()) {
        if line.chars().nth(0).unwrap().is_digit(10) {
            let mut coords = line.split(',').map(|n| n.parse().unwrap());
            points.insert((coords.next().unwrap(), coords.next().unwrap()));
        } else {
            let mut fold = line[11..].split('=');
            folds.push((fold.next().unwrap().chars().nth(0).unwrap(), fold.next().unwrap().parse().unwrap()));
        }
    }
    for (i, (direction, fold)) in folds.iter().enumerate() {
        let mut points_temp = HashSet::new();
        for (x, y) in points.iter() {
            let new_x = if *direction == 'x' && x > fold { fold - (x - fold) } else { *x };
            let new_y = if *direction == 'y' && y > fold { fold - (y - fold) } else { *y };
            points_temp.insert((new_x, new_y));
        }
        points = points_temp;
        if i == 0 {
            println!("{}", points.len());
        }
    }
    for i in 0..6 {
        for j in 0..40 {
            if points.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

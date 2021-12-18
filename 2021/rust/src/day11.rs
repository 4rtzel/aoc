use std::fs::File;
use std::io;
use std::io::BufRead;

fn level_up(map: &mut Vec<Vec<(u32, bool)>>, i: i32, j: i32, number_of_flashes: &mut u32) {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        return;
    }
    let mut v = &mut map[i as usize][j as usize];
    if v.1 {
        return;
    }
    v.0 += 1;
    if v.0 == 10 {
        *number_of_flashes += 1;
        v.1 = true;
        v.0 = 0;
        level_up(map, i - 1, j, number_of_flashes);
        level_up(map, i + 1, j, number_of_flashes);
        level_up(map, i, j - 1, number_of_flashes);
        level_up(map, i, j + 1, number_of_flashes);
        level_up(map, i - 1, j - 1, number_of_flashes);
        level_up(map, i - 1, j + 1, number_of_flashes);
        level_up(map, i + 1, j - 1, number_of_flashes);
        level_up(map, i + 1, j + 1, number_of_flashes);
    }
}

pub fn run(reader: io::BufReader<File>) {
    let mut map: Vec<Vec<(u32, bool)>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let row: Vec<(u32, bool)> = line.chars().map(|c| (c.to_digit(10).unwrap(), false)).collect();
        map.push(row);
    }

    let mut number_of_flashes = 0;
    let mut counter = 0;
    loop {
        for i in 0..map.len() as i32 {
            for j in 0..map[0].len() as i32 {
                level_up(&mut map, i, j, &mut number_of_flashes);
            }
        }
        let mut all_flashed = true;
        for i in 0..map.len() as i32 {
            for j in 0..map[0].len() as i32 {
                if !map[i as usize][j as usize].1 {
                    all_flashed = false;
                }
                map[i as usize][j as usize].1 = false;
            }
        }
        if counter == 99 {
            println!("{}", number_of_flashes);
        }
        counter += 1;
        if all_flashed {
            break;
        }
    }
    println!("{}", counter);
}

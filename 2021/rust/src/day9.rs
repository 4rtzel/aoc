use std::fs::File;
use std::io;
use std::io::BufRead;

fn get_value_or(map: &Vec<Vec<u32>>, i: i32, j: i32, or: u32) -> u32 {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        or
    } else {
        map[i as usize][j as usize]
    }
}

fn find_basin_size_and_mark_it(map: &mut Vec<Vec<u32>>, i: i32, j: i32) -> u32 {
    if get_value_or(map, i, j, 9) == 9 {
        return 0;
    }

    map[i as usize][j as usize] = 9;
    return 1 +
        find_basin_size_and_mark_it(map, i - 1, j) +
        find_basin_size_and_mark_it(map, i + 1, j) +
        find_basin_size_and_mark_it(map, i, j - 1) +
        find_basin_size_and_mark_it(map, i, j + 1);
}

pub fn run(reader: io::BufReader<File>) {
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(row);
    }

    let mut sum_of_risk_levels = 0;
    for i in 0..map.len() as i32 {
        for j in 0..map[0].len() as i32 {
            let current = get_value_or(&map, i, j, 0);
            if current < get_value_or(&map, i - 1, j, 10) &&
               current < get_value_or(&map, i + 1, j, 10) &&
               current < get_value_or(&map, i, j - 1, 10) &&
               current < get_value_or(&map, i, j + 1, 10) {
                sum_of_risk_levels += 1 + current;
            }

        }
    }
    println!("{}", sum_of_risk_levels);
    let mut basin_sizes: Vec<u32> = Vec::new();
    for i in 0..map.len() as i32 {
        for j in 0..map[0].len() as i32 {
            let current = get_value_or(&map, i, j, 0);
            if current != 9 {
                let basin_size = find_basin_size_and_mark_it(&mut map, i, j);
                basin_sizes.push(basin_size);
            }
        }
    }
    basin_sizes.sort();
    println!("{}", basin_sizes.iter().rev().take(3).fold(1, |acc, v| acc * v));
}

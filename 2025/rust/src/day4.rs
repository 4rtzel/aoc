use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(space: &Vec<Vec<bool>>) {
    let mut accessible_cells = 0;
    let rows = space.len() as i32;
    for row in 0i32..rows {
        let columns = space[row as usize].len() as i32;
        for column in 0i32..columns {
            if !space[row as usize][column as usize] {
                continue;
            }
            let mut adjacent_papers: u32 = 0;
            for i in (row - 1).max(0)..=(row + 1).min(rows - 1) {
                for j in (column - 1).max(0)..=(column + 1).min(columns - 1) {
                    if !(i == row && j == column) && space[i as usize][j as usize] {
                        adjacent_papers += 1;
                    }
                }
            }

            if adjacent_papers < 4 {
                accessible_cells += 1;
            }
        }
    }
    println!("{accessible_cells}");
}

fn part_two(space: &mut Vec<Vec<bool>>) {
    let mut removed = true;
    let mut total_removed = 0;
    while removed {
        removed = false;
        let rows = space.len() as i32;
        for row in 0i32..rows {
            let columns = space[row as usize].len() as i32;
            for column in 0i32..columns {
                if !space[row as usize][column as usize] {
                    continue;
                }
                let mut adjacent_papers: u32 = 0;
                for i in (row - 1).max(0)..=(row + 1).min(rows - 1) {
                    for j in (column - 1).max(0)..=(column + 1).min(columns - 1) {
                        if !(i == row && j == column) && space[i as usize][j as usize] {
                            adjacent_papers += 1;
                        }
                    }
                }

                if adjacent_papers < 4 {
                    total_removed += 1;
                    space[row as usize][column as usize] = false;
                    removed = true;
                }
            }
        }
    }
    println!("{total_removed}");
}

pub fn run(reader: io::BufReader<File>) {
    let mut space: Vec<Vec<bool>> = reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c == '@').collect())
        .collect();
    part_one(&space);
    part_two(&mut space);
}

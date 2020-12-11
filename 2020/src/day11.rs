use std::fs;
use std::io;
use std::io::BufRead;
use std::cmp::min;

#[derive(Clone, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn calculate_adjacent_seats(grid: &Vec<Vec<Position>>, i: usize, j: usize) -> u32 {
    let mut num_occupied = 0;
    for k in i.saturating_sub(1)..=min(i + 1, grid.len() - 1) {
        for l in j.saturating_sub(1)..=min(j + 1, grid[0].len() - 1) {
            if grid[k][l] == Position::OccupiedSeat && !(i == k && j == l) {
                num_occupied += 1;
            }
        }
    }
    num_occupied
}

fn calculate_in_sight_seats(grid: &Vec<Vec<Position>>, i: usize, j: usize) -> u32 {
    let mut num_occupied = 0;
    let inc: fn(i64) -> i64 = |x| x + 1;
    let dec: fn(i64) -> i64 = |x| x - 1;
    let nop: fn(i64) -> i64 = |x| x;
    for (fni, fnj) in [(dec, nop), (inc, nop), (nop, dec), (nop, inc), (dec, dec), (dec, inc), (inc, dec), (inc, inc)].iter() {
        let mut k = fni(i as i64);
        let mut l = fnj(j as i64);
        while (k >= 0 && k < grid.len() as i64) && (l >= 0 && l < grid[0].len() as i64) {
            if grid[k as usize][l as usize] != Position::Floor {
                if grid[k as usize][l as usize] == Position::OccupiedSeat {
                    num_occupied += 1;
                }
                break;
            }
            k = fni(k);
            l = fnj(l);
        }
    }
    num_occupied
}

fn calculate_number_of_occupied_seats_after_equlibrium(mut grid: Vec<Vec<Position>>,
                                                       threshold: u32,
                                                       calculate_seats: fn(&Vec<Vec<Position>>, usize, usize) -> u32) -> u32 {
    let mut changed = true;
    let mut grid2 = grid.clone();
    while changed {
        changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Position::Floor {
                    continue;
                }
                let num_occupied = calculate_seats(&grid, i, j);
                if grid[i][j] == Position::EmptySeat && num_occupied == 0 {
                    grid2[i][j] = Position::OccupiedSeat;
                    changed = true;
                } else if grid[i][j] == Position::OccupiedSeat && num_occupied >= threshold {
                    grid2[i][j] = Position::EmptySeat;
                    changed = true;
                }
            }
        }
        grid = grid2.clone();
    }
    grid.iter().map(|v| v.iter().fold(0, |acc, s| acc + (*s == Position::OccupiedSeat) as u32)).sum::<u32>()
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<Position>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        grid.push(line.chars().map(|c| match c {
                '.' => Position::Floor,
                'L' => Position::EmptySeat,
                _   => panic!("Unexpected position"),
            }).collect());
    }
    println!("{}", calculate_number_of_occupied_seats_after_equlibrium(grid.clone(), 4, calculate_adjacent_seats));
    println!("{}", calculate_number_of_occupied_seats_after_equlibrium(grid.clone(), 5, calculate_in_sight_seats));
}

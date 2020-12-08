use std::fs;
use std::io;
use std::io::BufRead;

fn part_one(map: &Vec<Vec<char>>) {
    const RIGHT: usize = 3;
    const DOWN: usize = 1;

    let mut i: usize = RIGHT;
    let mut j: usize = DOWN;
    let mut count = 0;
    while j < map.len() {
        if map[j][i] == '#' {
            count += 1;
        }

        i = (i + RIGHT) % map[0].len();
        j += DOWN;
    }
    println!("{}", count);
}

fn part_two(map: &Vec<Vec<char>>) {
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut total = 1;

    for (right, down) in slopes.iter() {
        let mut i: usize = *right;
        let mut j: usize = *down;
        let mut count = 0;
        while j < map.len() {
            if map[j][i] == '#' {
                count += 1;
            }

            i = (i + right) % map[0].len();
            j += down;
        }
        total *= count;
    }
    println!("{}", total);
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let map: Vec<Vec<char>> = reader.lines().map(|l| l.unwrap().chars().collect()).collect();
    part_one(&map);
    part_two(&map);
}

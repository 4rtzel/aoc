use std::fs;
use std::io;
use std::io::BufRead;
use std::cmp::max;

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    const MAX_ROW: usize = 127;
    const MAX_COLUMN: usize = 7;

    let mut max_sid = 0;
    let mut seats: [[bool; MAX_COLUMN + 1]; MAX_ROW + 1] = [[false; MAX_COLUMN + 1]; MAX_ROW + 1];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut row_from = 0;
        let mut row_to = MAX_ROW;
        let mut column_from = 0;
        let mut column_to = MAX_COLUMN;
        for ch in line.chars() {
            match ch {
                'F' => row_to = row_from + (row_to - row_from) / 2,
                'B' => row_from = row_from + (row_to - row_from) / 2 + 1,
                'L' => column_to = column_from + (column_to - column_from) / 2,
                'R' => column_from = column_from + (column_to - column_from) / 2 + 1,
                _   => panic!("Unexpected value"),
            }
            max_sid = max(max_sid, row_from * 8 + column_from);
        }
        seats[row_from][column_from] = true;
    }
    println!("{}", max_sid);

    // part two
    'outer: for (row, rows) in seats.iter().enumerate() {
        for (column, seat) in rows.iter().enumerate() {
            if !seat && column > 0 && seats[row][column - 1] && column < MAX_COLUMN && seats[row][column + 1] {
                println!("{}", row * 8 + column);
                break 'outer;
            }
        }
    }
}

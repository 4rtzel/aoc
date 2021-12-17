use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn run(reader: io::BufReader<File>) {
    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).filter(|s| !s.is_empty()).collect();
    let numbers: Vec<u32> = input[0].split(',').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut boards: Vec<(Vec<Vec<(u32, bool)>>, bool)> = Vec::new();
    for i in (1..input.len()).step_by(5) {
        let mut board: Vec<Vec<(u32, bool)>> = Vec::new();
        for j in 0..5 {
            let row: Vec<(u32, bool)> = input[i + j].split_whitespace().map(|n| (n.parse::<u32>().unwrap(), false)).collect();
            board.push(row);
        }
        boards.push((board, false));
    }
    let mut winners = 0;
    let mut winner_board_number = 0;
    let mut winner_number = 0;
    let mut loser_board_number = 0;
    let mut loser_number = 0;
    let boards_num: u32 = boards.len() as u32;
    for rnumber in numbers.iter() {
        for (board_num, (board, board_won)) in boards.iter_mut().enumerate().filter(|(_, b)| !b.1) {
            for row in board.iter_mut() {
                for number in row.iter_mut() {
                    if number.0 == *rnumber {
                        number.1 = true;
                    }
                }
            }

            for i in 0..5 {
                let mut row_won = true;
                let mut col_won = true;
                for j in 0..5 {
                    if !board[i][j].1 {
                        row_won = false;
                    }
                    if !board[j][i].1 {
                        col_won = false;
                    }
                }
                if row_won || col_won {
                    *board_won = true;
                    winners += 1;
                    if winners == 1 {
                        winner_board_number = board_num;
                        winner_number = *rnumber;
                    } else if winners == boards_num {
                        loser_board_number = board_num;
                        loser_number = *rnumber;
                    }
                    break;
                }
            }
        }
    }

    let mut total_unmarked = 0;
    for row in boards[winner_board_number].0.iter() {
        total_unmarked += row.iter().filter(|x| !x.1).fold(0, |acc, x| acc + x.0);
    }
    println!("{}", total_unmarked * winner_number);
    total_unmarked = 0;
    for row in boards[loser_board_number].0.iter() {
        total_unmarked += row.iter().filter(|x| !x.1).fold(0, |acc, x| acc + x.0);
    }
    println!("{}", total_unmarked * loser_number);
}

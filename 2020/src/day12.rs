use std::fs;
use std::io;
use std::io::BufRead;

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut inss: Vec<Instruction> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let (ins, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();
        inss.push(
            match ins {
                "N" => Instruction::North(num),
                "S" => Instruction::South(num),
                "E" => Instruction::East(num),
                "W" => Instruction::West(num),
                "L" => Instruction::Left(num),
                "R" => Instruction::Right(num),
                "F" => Instruction::Forward(num),
                _   => panic!("Unexpected value"),
            }
        )
    }

    // part one
    let mut direction: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for ins in inss.iter() {
        match ins {
            Instruction::North(v) => y += v,
            Instruction::South(v) => y -= v,
            Instruction::East(v) => x += v,
            Instruction::West(v) => x -= v,
            Instruction::Left(v) => direction += v,
            Instruction::Right(v) => direction -= v,
            Instruction::Forward(v) => {
                match direction % 360 {
                    90 | -270 => y += v,
                    -90 | 270 => y -= v,
                    0 => x += v,
                    180 | -180 => x -= v,
                    _ => panic!("Impossible!")
                }
            }
        }
    }
    println!("{}", x.abs() + y.abs());

    // part two
    x = 0;
    y = 0;
    let mut wx: i32 = 10;
    let mut wy: i32 = 1;
    for ins in inss.iter() {
        match ins {
            Instruction::North(v) => wy += v,
            Instruction::South(v) => wy -= v,
            Instruction::East(v) => wx += v,
            Instruction::West(v) => wx -= v,
            Instruction::Left(v) => {
                for _ in 0..(v / 90) {
                    let temp_wx = wx;
                    wx = -wy;
                    wy = temp_wx;
                }
            },
            Instruction::Right(v) => {
                for _ in 0..(v / 90) {
                    let temp_wx = wx;
                    wx = wy;
                    wy = -temp_wx;
                }
            },
            Instruction::Forward(v) => {
                x += wx * v;
                y += wy * v;
            },
        }
    }
    println!("{}", x.abs() + y.abs());
}

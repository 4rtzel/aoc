use std::fs;
use std::io;
use std::io::BufRead;

fn part_one(numbers: &Vec<u32>) {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            if num1 + num2 == 2020 {
                println!("{}", num1 * num2);
                return;
            }
        }
    }
}

fn part_two(numbers: &Vec<u32>) {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            for num3 in numbers.iter() {
                if num1 + num2 + num3== 2020 {
                    println!("{}", num1 * num2 * num3);
                    return;
                }
            }
        }
    }
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);
    let numbers: Vec<u32> = reader.lines().map(|x| x.unwrap().parse::<u32>().unwrap()).collect();
    part_one(&numbers);
    part_two(&numbers);
}

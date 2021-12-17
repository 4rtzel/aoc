use std::fs::File;
use std::io;
use std::io::BufRead;

const VALUE_LEN: usize = 12;

fn part_one(bits: &Vec<String>) {
    let mut bits_sum: [u32; VALUE_LEN] = [0; VALUE_LEN];
    let mut size: u32 = 0;
    for line in bits.iter() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                bits_sum[i] += 1;
            }
        }
        size += 1;
    }
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    for (i, bit) in bits_sum.iter().enumerate() {
        if bit > &(size / 2) {
            gamma_rate = gamma_rate | (1 << (VALUE_LEN - 1 - i));
        } else {
            epsilon_rate = epsilon_rate | (1 << (VALUE_LEN - 1 - i));
        }
    }
    println!("{}", gamma_rate * epsilon_rate);
}

fn part_two(bits: &Vec<String>) {
    let mut bit_position = 0;
    let mut oxygen_bits = bits.clone();
    let mut co2_bits = bits.clone();
    while oxygen_bits.len() != 1 {
        let mut bit_sum = 0;
        for line in oxygen_bits.iter() {
            if line.chars().nth(bit_position).unwrap() == '1' {
                bit_sum += 1;
            }
        }
        let most_common = if bit_sum * 2 >= oxygen_bits.len() { '1' } else { '0' };
        oxygen_bits.retain(|b| b.chars().nth(bit_position).unwrap() == most_common);
        println!("{:?}", oxygen_bits);
        bit_position += 1;
    }
    bit_position = 0;
    while co2_bits.len() != 1 {
        let mut bit_sum = 0;
        for line in co2_bits.iter() {
            if line.chars().nth(bit_position).unwrap() == '1' {
                bit_sum += 1;
            }
        }
        let least_common = if bit_sum * 2 >= co2_bits.len() { '0' } else { '1' };
        co2_bits.retain(|b| b.chars().nth(bit_position).unwrap() == least_common);
        println!("{:?}", co2_bits);
        bit_position += 1;
    }
    println!("{}", u32::from_str_radix(&oxygen_bits[0], 2).unwrap() * u32::from_str_radix(&co2_bits[0], 2).unwrap());
}

pub fn run(reader: io::BufReader<File>) {
    let bits: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    part_one(&bits);
    part_two(&bits);
}

use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct FloatingBits {
    origin: u64,            // original value (e.g. 42).
    max_value: u64,         // floating mask with removed zeros (e.g. 001001 -> 000011).
                            // We use it to iterate through all possible combinations
                            // by simple incrementing current_value and comparing it to max_value.
    current_value: u64,     // current value that < max_value.
    bits_mapping: Vec<u64>, // mapping between current_value's bits and original's bits.
                            // vec[0, 5] would mean that floating mask had 2 bits set -- 0 and 5.
}

impl FloatingBits {
    fn new(value: u64, mask_floating: u64) -> FloatingBits {
        let mut i: u64 = 0;
        let mut max_value: u64 = 0;
        let mut bits_mapping: Vec<u64> = Vec::new();
        for b in 0..64 {
            if mask_floating & (1 << b) != 0 {
                max_value |= 1 << i;
                i += 1;
                bits_mapping.push(b);
            }
        }
        FloatingBits { origin: value, bits_mapping: bits_mapping, current_value: 0, max_value: max_value }
    }
}

impl Iterator for FloatingBits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value > self.max_value {
            return None;
        }
        let mut value: u64 = self.origin;
        self.current_value += 1; // that's how we iterate through all combinations.
        for b in 0..self.bits_mapping.len() {
            // map the bits from current_value back to the original value accoriding
            // to the mask_floating (aka bits_mapping).
            if self.current_value & (1 << b) != 0 {
                value |= 1 << self.bits_mapping[b];
            } else {
                value &= !(1 << self.bits_mapping[b]);
            }
        }
        Some(value)
    }
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let re = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    const MASK_LEN: u64 = 36;
    let mut mask_set: u64 = 0;
    let mut mask_clear: u64 = std::u64::MAX;
    let mut mask_floating: u64 = 0;
    let mut mem1: HashMap<u64, u64> = HashMap::new();
    let mut mem2: HashMap<u64, u64> = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("mask") {
            mask_set = 0;
            mask_clear = std::u64::MAX;
            mask_floating = 0;
            for (i, ch) in line.split(' ').nth(2).unwrap().chars().enumerate() {
                match ch {
                    'X' => mask_floating |= 1u64 << (MASK_LEN - i as u64 - 1),
                    '1' => mask_set |= 1u64 << (MASK_LEN - i as u64 - 1),
                    '0' => mask_clear &= !(1u64 << (MASK_LEN - i as u64 - 1)),
                    _   => panic!("Unexpected maks"),
                }
            }
        } else {
            let caps = re.captures(&line).unwrap();
            let address = caps[1].parse::<u64>().unwrap();
            let value = caps[2].parse::<u64>().unwrap();
            mem1.insert(address, (value | mask_set) & mask_clear);
            for address in FloatingBits::new(address | mask_set, mask_floating) {
                mem2.insert(address, value);
            }
        }
    }
    println!("{}", mem1.iter().fold(0, |acc, (_, v)| acc + v));
    println!("{}", mem2.iter().fold(0, |acc, (_, v)| acc + v));
}

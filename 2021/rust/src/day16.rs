use std::fs::File;
use std::io;
use std::io::BufRead;

fn bits_to_number(bits: &[u8]) -> u64 {
    let mut shift = bits.len();
    let mut result: u64 = 0;
    for b in bits.iter() {
        shift -= 1;
        result = result | ((*b as u64) << shift);
    }
    return result;
}

fn process_packet(bits: &[u8], total_version: &mut u64) -> (u64, u64) {
    let version =  bits_to_number(&bits[0..3]);
    *total_version += version;
    let id = bits_to_number(&bits[3..6]);
    if id == 4 {
        let mut i = 6;
        let mut groups: Vec<u8> = Vec::new();
        loop {
            groups.extend(bits[i + 1..i + 5].iter());
            if bits[i] == 0 {
                break;
            }
            i += 5;
        }
        return (5 + i as u64, bits_to_number(&groups));
    } else {
        let mut values = Vec::new();
        let processed = if bits[6] == 0 {
            let total_length = bits_to_number(&bits[7..7 + 15]);
            let mut i = 7 + 15;
            let mut length_left = total_length;
            while length_left > 0 {
                let p = process_packet(&bits[i..], total_version);
                length_left -= p.0;
                values.push(p.1);
                i = (7 + 15 + (total_length - length_left)) as usize;
            }
            22 + total_length
        } else {
            let num_of_sub_packets = bits_to_number(&bits[7..7 + 11]);
            let mut i = 7 + 11;
            for _ in 0..num_of_sub_packets {
                let p = process_packet(&bits[i..], total_version);
                i += p.0 as usize;
                values.push(p.1);
            }
            i as u64
        };
        return (processed, match id {
            0 => values.iter().fold(0, |a, v| a + v),
            1 => values.iter().fold(1, |a, v| a * v),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => if values[0] > values[1] { 1 } else { 0 },
            6 => if values[0] < values[1] { 1 } else { 0 },
            7 => if values[0] == values[1] { 1 } else { 0 },
            _ => panic!("Unexpected id"),
        });
    }
}

pub fn run(reader: io::BufReader<File>) {
    let mut bits: Vec<u8> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        for b in line.as_bytes().iter().map(|c| format!("{:04b}", c - if *c < 65 { 48 } else { 55 })) {
            bits.extend(b.chars().map(|b| if b == '1' { 1 } else { 0 }));
        }
    }
    let mut total_version = 0;
    let results = process_packet(&bits, &mut total_version);
    println!("{}\n{}", total_version, results.1);
}

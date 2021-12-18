use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn retain_values_only_for(possible_values: &mut HashMap<char, Vec<char>>, retain_for: &[char], values: &str, remove_from_others: bool) {
    for (key, val) in possible_values.iter_mut() {
        if retain_for.contains(key) {
            val.retain(|v| values.contains(*v));
        } else if remove_from_others {
            val.retain(|v| !values.contains(*v));
        }
    }

    let singles: Vec<(char, char)>  = possible_values.iter().filter(|(_, v)| v.len() == 1).map(|(k, v)| (*k, v[0])).collect();
    for (single_key, single_val) in singles.iter() {
        for (key, val) in possible_values.iter_mut() {
            if key != single_key {
                val.retain(|v| v != single_val);
            }
        }
    }
}

pub fn run(reader: io::BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut number_of_1_4_7_8 = 0;
    let mut sum_of_outputs = 0;
    for line in lines.iter() {
        let input: Vec<&str> = line.splitn(2, '|').collect();
        let signal_digits: Vec<&str> = input[0].split_whitespace().collect();
        let output_digits: Vec<&str> = input[1].split_whitespace().collect();
        number_of_1_4_7_8 += output_digits.iter().filter(|s| [2, 4, 3, 7].contains(&s.len())).count();

        const SEGMENTS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let mut possible_values: HashMap<char, Vec<char>> = SEGMENTS.iter().map(|c| (*c, SEGMENTS.to_vec())).collect();
        for signal in signal_digits.iter().cycle() {
            if signal.len() == 2 {
                retain_values_only_for(&mut possible_values, &['c', 'f'], signal, true);
            } else if signal.len() == 4 {
                retain_values_only_for(&mut possible_values, &['b', 'c', 'd', 'f'], signal, true);
            } else if signal.len() == 3 {
                retain_values_only_for(&mut possible_values, &['a', 'c', 'f'], signal, true);
            } else if signal.len() == 5 {
                retain_values_only_for(&mut possible_values, &['a', 'd', 'g'], signal, false);
            } else if signal.len() == 6 {
                retain_values_only_for(&mut possible_values, &['a', 'b', 'f', 'g'], signal, false);
            }

            if possible_values.values().all(|v| v.len() == 1) {
                break;
            }
        }
        let possible_values: HashMap<char, char> = possible_values.iter().map(|(k, v)| (v[0], *k)).collect();
        let mut full_digit = String::new();
        for od in output_digits.iter() {
            let mut real_segments: Vec<char> = od.chars().map(|c| *possible_values.get(&c).unwrap()).collect();
            real_segments.sort();
            let digit = match real_segments.iter().collect::<String>().as_str() {
                "abcefg" => '0',
                "cf" => '1',
                "acdeg" => '2',
                "acdfg" => '3',
                "bcdf" => '4',
                "abdfg" => '5',
                "abdefg" => '6',
                "acf" => '7',
                "abcdefg" => '8',
                "abcdfg" => '9',
                _ => panic!("Unexpected pattern"),
            };
            full_digit.push(digit);
        }
        sum_of_outputs += full_digit.parse::<u32>().unwrap();
    }
    println!("{}\n{}", number_of_1_4_7_8, sum_of_outputs);
}

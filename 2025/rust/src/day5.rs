use std::fs::File;
use std::io;
use std::io::BufRead;

fn part_one(ranges: &Vec<Vec<u64>>, ids: &Vec<u64>) {
    let mut fresh_number = 0;
    for id in ids {
        for range in ranges {
            if (range[0]..=range[1]).contains(id) {
                fresh_number += 1;
                break;
            }
        }
    }
    println!("{fresh_number}");
}

fn part_two(ranges: &Vec<Vec<u64>>) {
    let mut merged_ranges: Vec<Vec<u64>> = vec![];
    for range in ranges {
        let mut merged = false;
        for i in 0..merged_ranges.len() {
            if (merged_ranges[i][0]..=merged_ranges[i][1]).contains(&range[0]) {
                merged_ranges[i][1] = merged_ranges[i][1].max(range[1]);
                merged = true;
                break;
            }
        }
        if !merged {
            merged_ranges.push(range.to_vec());
        }
    }
    let mut total: u64 = 0;
    for merged_range in merged_ranges {
        total += merged_range[1] - merged_range[0] + 1;
    }
    println!("{total}");
}

pub fn run(reader: io::BufReader<File>) {
    let mut ranges: Vec<Vec<u64>> = vec![];
    let mut ids: Vec<u64> = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let range = line.split('-').take(2).map(|v| v.parse::<u64>().unwrap()).collect();
            ranges.push(range);
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    }
    ranges.sort_by_key(|v| v[0]);
    part_one(&ranges, &ids);
    part_two(&ranges);
}

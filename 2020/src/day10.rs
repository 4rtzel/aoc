use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn part_two(adapters: &Vec<u32>, mut current_jolt: u32, from: usize, cache: &mut HashMap<(u32, usize), u64>) -> u64 {
    let mut valid: u64 = 0;
    let prev_current_jolt = current_jolt;
    let entry = cache.entry((current_jolt, from)).or_insert(0);
    if *entry != 0 {
        return *entry;
    }
    for i in (from + 1)..adapters.len() - 1 {
        if adapters[i + 1] - current_jolt <= 3 {
            valid += 1 + part_two(adapters, current_jolt, i, cache);
        }
        current_jolt = adapters[i];
    }
    *cache.get_mut(&(prev_current_jolt, from)).unwrap() = valid;
    valid
}
pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut adapters: Vec<u32> = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    println!("{}", adapters.iter()
                           .scan(0, |a, &j| { let r = j - *a; *a = j; Some(r) })
                           .fold([0, 0], |[l, r], v| [l + (v == 1) as u32, r + (v == 3) as u32])
                           .iter()
                           .product::<u32>());

    let mut cache: HashMap<(u32, usize), u64> = HashMap::new();
    println!("{}", part_two(&adapters, 0, 0, &mut cache) + 1);
}

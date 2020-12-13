use std::fs;
use std::io;
use std::io::BufRead;

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().map(|l| l.unwrap());
    let estimate = lines.next().unwrap().parse::<u32>().unwrap();
    let ids_str = lines.next().unwrap();

    let ids: Vec<u32> = ids_str.split(',')
                               .filter_map(|l| l.parse::<u32>().ok())
                               .collect();

    // part one
    println!("{}", ids.iter()
                      .map(|i| [*i, ((estimate / i) * i + i) - estimate]) // ignore estimate == (estimate / i) * i
                      .min_by_key(|[_, n]| *n)
                      .unwrap()
                      .iter()
                      .product::<u32>());

    // part two
    let ids: Vec<(u64, u64)> = ids_str.split(',')
                                      .enumerate()
                                      .filter_map(|(i, s)| if let Some(v) = s.parse::<u64>().ok() { Some((i as u64, v)) } else { None })
                                      .collect();

    // Example with "7,13,x,x,59,x,31,19" as input.
    // Linear equations will look like:
    //   timestamp = a1 * 7 - 0;
    //   timestamp = a2 * 13 - 1;
    //   timestamp = a3 * 59 - 4;
    //   ...
    // Find a first 'timestamp' that will give us an interger result
    //   while (timestamp + 0) % 7 != 0 {
    //   while (timestamp + 1) % 13 != 0 {
    //   while (timestamp + 4) % 59 != 0 {
    //   ...
    // Because the input contains only prime numbers, we could calculate the
    // number to advance, by simple multiplication.
    //   advance_by *= 7;
    //   advance_by *= 13;
    //   advance_by *= 59;
    let mut timestamp: u64 = ids[0].1;
    let mut advance_by: u64 = 1;
    for (offset, id) in ids.iter() {
        while (timestamp + offset) % id != 0 {
            timestamp += advance_by;
        }
        advance_by *= id;
    }
    println!("{}", timestamp);
}

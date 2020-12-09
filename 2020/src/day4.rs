use std::fs;
use std::io;
use std::io::BufRead;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_HGT: regex::Regex = regex::Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    static ref RE_HCL: regex::Regex = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: regex::Regex = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: regex::Regex = regex::Regex::new(r"^[0-9]{9}$").unwrap();
}

pub fn is_valid(key: &String, val: &String) -> bool {
    match key.as_ref() {
        "byr" => (1920..=2002).contains(&val.parse::<u32>().unwrap()),
        "iyr" => (2010..=2020).contains(&val.parse::<u32>().unwrap()),
        "eyr" => (2020..=2030).contains(&val.parse::<u32>().unwrap()),
        "hgt" => {
            let caps = RE_HGT.captures(val);
            match caps {
                Some(c) => {
                    let height = c[1].parse::<u32>().unwrap();
                    match c[2].as_ref() {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _    => false
                    }
                },
                None => false,
            }
        },
        "hcl" => RE_HCL.is_match(val),
        "ecl" => RE_ECL.is_match(val),
        "pid" => RE_PID.is_match(val),
        "cid" => true,
        _     => false,
    }
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let re = regex::Regex::new(r"([a-z][a-z][a-z]):([#a-z0-9]+)").unwrap();

    const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_count = 0;
    let mut valid_count2 = 0;
    let mut passport: Vec<(String, String)> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {
            // part one
            if FIELDS.iter().all(|f| passport.iter().any(|(key, _)| key == f)) {
                valid_count += 1;
            }

            // part two
            if FIELDS.iter().all(|f| passport.iter().any(|(key, _)| key == f)) {
                if passport.iter().all(|(key, value)| is_valid(key, value)) {
                    valid_count2 += 1;
                }
            }
            passport.clear();
        }
        for cap in re.captures_iter(&line) {
            passport.push((cap[1].to_string(), cap[2].to_string()));
        }
    }
    println!("{}", valid_count);
    println!("{}", valid_count2);
}

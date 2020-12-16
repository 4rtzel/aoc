use std::fs;
use std::io;
use std::io::BufRead;

struct Field {
    name: String,
    r1: std::ops::RangeInclusive<u32>,
    r2: std::ops::RangeInclusive<u32>,
    possible_locations: Vec<u32>,
}

impl Field {
    fn can_contain(&self, number: u32) -> bool {
        self.r1.contains(&number) || self.r2.contains(&number)
    }
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let field_re = regex::Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();

    let mut fields: Vec<Field> = Vec::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()).filter(|l| l.chars().last().unwrap_or(':') != ':') {
        if let Some(caps) = field_re.captures(&line) {
            fields.push(Field {
                name: caps[1].to_string(),
                r1: std::ops::RangeInclusive::new(caps[2].parse().unwrap(), caps[3].parse().unwrap()),
                r2: std::ops::RangeInclusive::new(caps[4].parse().unwrap(), caps[5].parse().unwrap()),
                possible_locations: Vec::new(),
            });
        } else {
            let tickets: Vec<u32> = line.split(',').map(|n| n.parse().unwrap()).collect();
            if my_ticket.is_empty() {
                my_ticket = tickets;
            } else {
                nearby_tickets.push(tickets);
            }
        }
    }

    // part one
    println!("{}", nearby_tickets
        .iter()
        .fold(0,
              |acc, t| acc + t.iter()
                              .find(|n| fields.iter().all(|r| !r.can_contain(**n)))
                              .unwrap_or(&0))
    );
    // part two
    nearby_tickets.retain(|t| t.iter().all(|n| fields.iter().any(|r| r.can_contain(*n))));
    for i in 0..fields.len() {
        for j in 0..nearby_tickets[0].len() {
            if nearby_tickets.iter().all(|t| fields[i].can_contain(t[j])) {
                // find all possible locations for each field
                fields[i].possible_locations.push(j as u32);
            }
        }
    }

    let mut departure_mult: u64 = 1;
    while !fields.iter().all(|r| r.possible_locations.len() == 0) {
        let to_remove: u32 = {
            // find a certain location for any field
            let certain_field = fields.iter().find(|r| r.possible_locations.len() == 1).unwrap();
            if certain_field.name.starts_with("departure") {
                departure_mult *= my_ticket[certain_field.possible_locations[0] as usize] as u64;
            }
            certain_field.possible_locations[0]
        };
        for field in fields.iter_mut() {
            // remove the location from all fields
            field.possible_locations.retain(|&n| n != to_remove);
        }
    }
    println!("{}", departure_mult);
}

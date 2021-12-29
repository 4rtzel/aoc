use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

type Point = (i32, i32, i32);
type Orientation = (i32, i32, i32);

fn get_all_relative_distances_from_node(beacons: &Vec<Point>, index: usize) -> Vec<(i32, Point)> {
    let mut distances = Vec::new();
    let (x, y, z) = beacons[index];
    for (_, beacon) in beacons.iter().enumerate() {
        distances.push(((beacon.0 - x).pow(2) + (beacon.1 - y).pow(2) + (beacon.2 - z).pow(2), *beacon));
    }
    distances.sort();
    return distances;
}

fn calculate_overlapping_scanner_position(scanners: &Vec<Vec<Point>>, scanner_index: usize, overlapping_scanner_index: usize) -> Option<(Point, Orientation)> {
    let mut base = Vec::new();
    let mut overlap = Vec::new();
    let mut matches = 0;
    'done: for beacon_index in 0..scanners[scanner_index].len() {
        base = get_all_relative_distances_from_node(&scanners[scanner_index], beacon_index);
        for overlapping_beacon_index in 0..scanners[overlapping_scanner_index].len() {
            overlap = get_all_relative_distances_from_node(&scanners[overlapping_scanner_index], overlapping_beacon_index);
            matches = calculate_number_of_matches(&base, &overlap);
            if matches >= 12 {
                break 'done;
            }
        }
    }
    if matches >= 12 {
        for orient in get_all_orientations().iter() {
            let mut i = 0;
            let mut j = 0;
            let mut numbers = HashSet::new();
            while i < base.len() && j < overlap.len() {
                if base[i].0 == overlap[j].0 {
                    let (x1, y1, z1) = base[i].1;
                    let (x2, y2, z2) = map_point(overlap[j].1, *orient);
                    numbers.insert((x1 + x2, y1 + y2, z1 + z2));
                    i += 1;
                    j += 1;
                } else if base[i].0 < overlap[j].0 {
                    i += 1;
                } else {
                    j += 1;
                }
            }
            if numbers.len() == 1 {
                return Some((*numbers.iter().nth(0).unwrap(), *orient));
            }
        }
    }
    None
}

fn calculate_number_of_matches(base: &Vec<(i32, Point)>, overlap: &Vec<(i32, Point)>) -> u32 {
    let mut matches = 0;
    for (n, _) in base.iter() {
        if overlap.binary_search_by(|v| v.0.cmp(n)).is_ok() {
            matches += 1;
        }
    }
    return matches;
}

fn get_all_orientations() -> Vec<Orientation> {
    let mut results: Vec<Orientation> = Vec::new();
    const MASKS: [(i32, i32, i32); 8] = [(1, 1, 1), (1, 1, -1), (1, -1, 1), (1, -1, -1), (-1, 1, 1), (-1, 1, -1), (-1, -1, 1), (-1, -1, -1)];
    for mask in MASKS.iter() {
        results.push((1 * mask.0, 2 * mask.1, 3 * mask.2));
        results.push((2 * mask.0, 3 * mask.1, 1 * mask.2));
        results.push((3 * mask.0, 1 * mask.1, 2 * mask.2));

        results.push((1 * mask.0, 3 * mask.1, 2 * mask.2));
        results.push((2 * mask.0, 1 * mask.1, 3 * mask.2));
        results.push((3 * mask.0, 2 * mask.1, 1 * mask.2));
    }
    return results;
}

fn map_point(p: Point, o: Orientation) -> Point {
    fn map(p: Point, v: i32) -> i32 {
        match v.abs() {
            1 => p.0 * v.signum(),
            2 => p.1 * v.signum(),
            3 => p.2 * v.signum(),
            _ => panic!("Unexpected number"),
        }
    }
    (map(p, o.0), map(p, o.1), map(p, o.2))
}

pub fn run(reader: io::BufReader<File>) {
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    let mut index: i32 = -1;
    for line in reader.lines().map(|l| l.unwrap()).filter(|l| !l.is_empty()) {
        if line.chars().nth(1).unwrap() == '-' {
            index += 1;
            scanners.push(Vec::new());
        } else {
            let coords: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
            scanners[index as usize].push((coords[0], coords[1], coords[2]));
        }
    }
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut scanners_positions: HashSet<Point> = HashSet::new();
    for beacon in scanners[0].iter() {
        beacons.insert(*beacon);
    }
    let mut normalized = vec![false; scanners.len()];
    normalized[0] = true;
    while !normalized.iter().all(|n| *n) {
        for i in 0..scanners.len() {
            for j in 0..scanners.len() {
                if i == j {
                    continue;
                }
                if normalized[i] && !normalized[j] {
                    if let Some(v) = calculate_overlapping_scanner_position(&scanners, i, j) {
                        for beacon_index in 0..scanners[j].len() {
                            let m = map_point(scanners[j][beacon_index], v.1);
                            scanners_positions.insert(v.0);
                            scanners[j][beacon_index] = (v.0.0 - m.0, v.0.1 - m.1, v.0.2 - m.2);
                            beacons.insert(scanners[j][beacon_index]);
                        }
                        normalized[j] = true;
                    }
                }
            }
        }
    }
    println!("{}", beacons.len());
    let scanners_positions: Vec<Point> = scanners_positions.iter().copied().collect();
    let mut max_distance = 0;
    for i in 0..scanners_positions.len() {
        for j in i + 1..scanners_positions.len() {
            let (x1, y1, z1) = scanners_positions[i];
            let (x2, y2, z2) = scanners_positions[j];
            max_distance = std::cmp::max(max_distance, (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs());
        }
    }
    println!("{}", max_distance);
}

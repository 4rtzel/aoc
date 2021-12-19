use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn find_dijkstra(map: &mut Vec<Vec<(u32, u32)>>) {
    map[0][0].1 = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut nodes: Vec<(i32, i32, u32)> = Vec::new();
    nodes.push((0, 0, 0));

    while !nodes.is_empty() {
        let (x, y, _) = nodes.remove(0);
        if visited.contains(&(x, y)) {
            continue;
        }
        for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if x + i < 0 || x + i >= map.len() as i32 || y + j < 0 || y + j >= map[0].len() as i32 {
                continue;
            }
            let k = (x + i) as usize;
            let l = (y + j) as usize;
            map[k][l].1 = std::cmp::min(map[k][l].1, map[x as usize][y as usize].1 + map[k][l].0);
            nodes.push((x + i, y + j, map[k][l].1));
        }
        visited.insert((x, y));
        nodes.sort_by(|(_, _, l), (_, _, r)| l.cmp(r));
    }
    println!("{}", map.last().unwrap().last().unwrap().1);
}

pub fn run(reader: io::BufReader<File>) {
    let mut map: Vec<Vec<(u32, u32)>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let row = line.chars().map(|c| (c.to_digit(10).unwrap(), u32::MAX)).collect();
        map.push(row);
    }
    find_dijkstra(&mut map);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            map[i][j].1 = u32::MAX;
        }
        let mut copy = map[i].clone();
        for _ in 0..4 {
            for k in 0..copy.len() {
                copy[k].0 = if copy[k].0 == 9 { 1 } else { copy[k].0 + 1 };
            }
            map[i].extend(copy.iter());
        }
    }
    let mut copy: Vec<Vec<(u32, u32)>> = map.clone();
    for _ in 0..4 {
        for i in 0..copy.len() {
            for j in 0..copy[i].len() {
                copy[i][j].0 = if copy[i][j].0 == 9 { 1 } else { copy[i][j].0 + 1 };
            }
        }
        for v in copy.iter() {
            map.push(v.to_vec());
        }
    }
    find_dijkstra(&mut map);
}

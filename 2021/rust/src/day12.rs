use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn traverse_paths<'a>(graph: &'a HashMap<String, Vec<String>>, part_two: bool, current: &'a str, path: &mut Vec<&'a str>, total_paths: &mut u32) {
    if part_two {
        if current.chars().nth(0).unwrap().is_lowercase() {
            if path.len() > 0 && current == "start" {
                return;
            }
            if path.contains(&current) {
                let mut occurences: HashSet<&str> = HashSet::new();
                for cave in path.iter().filter(|c| c.chars().nth(0).unwrap().is_lowercase()) {
                    match occurences.get(cave) {
                        Some(_) => return,
                        None => occurences.insert(&cave),
                    };
                }
            }
        }
    } else {
        if current.chars().nth(0).unwrap().is_lowercase() && path.contains(&current) {
            return;
        }
    }
    if current == "end" {
        *total_paths += 1;
        return;
    }
    path.push(current);
    for cave in graph.get(current).unwrap().iter() {
        traverse_paths(graph, part_two, cave, path, total_paths);
    }
    path.pop();
}

pub fn run(reader: io::BufReader<File>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let nodes: Vec<&str> = line.split('-').collect();
        graph.entry(nodes[0].to_string()).or_insert(Vec::new()).push(nodes[1].to_string());
        graph.entry(nodes[1].to_string()).or_insert(Vec::new()).push(nodes[0].to_string());
    }
    let mut path: Vec<&str> = Vec::new();
    let mut total_paths = 0;
    traverse_paths(&graph, false, "start", &mut path, &mut total_paths);
    println!("{}", total_paths);
    total_paths = 0;
    traverse_paths(&graph, true, "start", &mut path, &mut total_paths);
    println!("{}", total_paths);
}

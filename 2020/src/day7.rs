use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    color: String,
    number: u32,
}

type Bags = Vec<Bag>;

fn can_contain_shiny_gold(color: &String, graph: &HashMap<String, Bags>) -> bool {
    let links = graph.get(color).unwrap();
    if links.iter().any(|link| link.color == "shiny gold") {
        true
    } else {
        links.iter().any(|link| can_contain_shiny_gold(&link.color, graph))
    }
}

fn count_nested_bags(color: &String, graph: &HashMap<String, Bags>) -> u32 {
    graph.get(color).unwrap().iter().fold(0, |a, l| a + l.number + l.number * count_nested_bags(&l.color, graph))
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut graph: HashMap<String, Bags> = HashMap::new();
    let re = regex::Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bag").unwrap();

    for line in reader.lines().map(|l| l.unwrap()) {
        let color: String = line.split(' ').take(2).collect::<Vec<_>>().join(" ");
        graph.insert(color.clone(), Bags::new());
        for cap in re.captures_iter(&line) {
            let links = graph.entry(color.clone()).or_insert(Bags::new());
            links.push( Bag { color: cap[2].to_string(), number: cap[1].parse::<u32>().unwrap() } );
        }
    }

    println!("{}", graph.iter().filter(|(color, _)| can_contain_shiny_gold(color, &graph)).count());
    println!("{}", count_nested_bags(&"shiny gold".to_string(), &graph));
}

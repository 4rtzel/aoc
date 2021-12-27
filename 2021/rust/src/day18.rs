use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone)]
enum Node {
    Number(u32),
    Children(Vec<Node>),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(n) => write!(f, "N({})", n),
            Node::Children(c) => write!(f, "C{:?}", c),
        }
    }
}

impl Node {
    fn get_number(&self) -> Option<u32> {
        match self {
            Node::Number(n) => Some(*n),
            Node::Children(_) => None,
        }
    }
}

fn parse_snailfish_number(input: &Vec<char>) -> Node {
    let mut from = 0;
    parse_snailfish_number_impl(input, &mut from)
}

fn parse_snailfish_number_impl(input: &Vec<char>, from: &mut usize) -> Node {
    *from += 1;
    let mut children: Vec<Node> = Vec::new();
    while *from < input.len() {
        if input[*from] == '[' {
            children.push(parse_snailfish_number_impl(input, from));
        } else if input[*from].is_digit(10) {
            children.push(Node::Number(input[*from].to_digit(10).unwrap()));
            *from = *from + 1;
        } else if input[*from] == ',' {
            *from += 1;
            continue;
        } else {
            *from += 1;
            return Node::Children(children);
        }
    }
    unreachable!();
}

fn reduce_snailfish_number(root: &mut Node) {
    while explode_snailfish_number(root) || split_snailfish_number(root) { }
}

fn explode_snailfish_number(node: &mut Node) -> bool {
    let mut state = ExplodeState::new(false, false, false, 0, 0);
    explode_snailfish_number_impl(node, 0, &mut state);
    state.pair_found
}

struct ExplodeState {
    pair_found: bool,
    left_added: bool,
    right_added: bool,
    left_value: u32,
    right_value: u32,
}

impl ExplodeState {
    fn new(pair_found: bool, left_added: bool, right_added: bool, left_value: u32, right_value: u32) -> Self {
        Self {
            pair_found: pair_found,
            left_added: left_added,
            right_added: right_added,
            left_value: left_value,
            right_value: right_value,
        }
    }
}

fn explode_snailfish_number_impl(node: &mut Node, mut number_of_pairs: u32, state: &mut ExplodeState) -> bool {
    match node {
        Node::Number(_) => return false,
        Node::Children(children) => {
            if children.len() == 2 {
                let lhs = children[0].get_number();
                let rhs = children[1].get_number();
                    number_of_pairs += 1;

                if number_of_pairs > 4 && lhs.is_some() && rhs.is_some() {
                    state.pair_found = true;
                    state.left_value = lhs.unwrap();
                    state.right_value = rhs.unwrap();
                    return true;
                }
            } else {
                number_of_pairs = 0;
            }
            for i in 0..children.len() {
                let pair_exploded = explode_snailfish_number_impl(&mut children[i], number_of_pairs, state);
                if pair_exploded {
                    children[i] = Node::Number(0);
                }
                if state.pair_found {
                    if !state.left_added {
                        state.left_added = add_to_rightmost_number(&mut children[0..i], state.left_value);
                    }
                    if !state.right_added {
                        state.right_added = add_to_leftmost_number(&mut children[i + 1..], state.right_value);
                    }
                    break;
                }
            }
        },
    }
    return false;
}

fn add_to_rightmost_number(nodes: &mut [Node], value: u32) -> bool {
    for node in nodes.iter_mut().rev() {
        match node {
            Node::Number(n) => {
                *n += value;
                return true;
            },
            Node::Children(children) => {
                return add_to_rightmost_number(children, value);
            },
        }
    }
    return false;
}

fn add_to_leftmost_number(nodes: &mut [Node], value: u32) -> bool {
    for node in nodes.iter_mut() {
        match node {
            Node::Number(n) => {
                *n += value;
                return true;
            },
            Node::Children(children) => {
                return add_to_leftmost_number(children, value);
            },
        }
    }
    return false;
}

fn split_snailfish_number(node: &mut Node) -> bool {
    match node {
        Node::Number(n) => {
            if *n > 9 {
                *node = Node::Children(vec![Node::Number(*n / 2), Node::Number(*n - *n / 2)]);
                return true;
            }
        },
        Node::Children(children) => {
            for c in children.iter_mut() {
                if split_snailfish_number(c) {
                    return true;
                }
            }
        },
    }
    return false;
}

fn calculate_magnitude(node: &Node) -> u32 {
    match node {
        Node::Children(children) => calculate_magnitude(&children[0]) * 3 + calculate_magnitude(&children[1]) * 2,
        Node::Number(n) => *n,
    }
}

pub fn run(reader: io::BufReader<File>) {
    let mut numbers: Vec<Vec<char>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        numbers.push(line.chars().collect());
    }
    let mut sum = Node::Children(vec![parse_snailfish_number(&numbers[0])]);
    for number in numbers.iter().skip(1) {
        let root = parse_snailfish_number(&number);
        if let Node::Children(ref mut c) = sum {
            c.push(root);
        }
        reduce_snailfish_number(&mut sum);
        sum = Node::Children(vec![sum]);
    }
    if let Node::Children(children) = sum {
        println!("{}", calculate_magnitude(&children[0]));
    }

    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            sum = Node::Children(vec![parse_snailfish_number(&numbers[i]), parse_snailfish_number(&numbers[j])]);
            reduce_snailfish_number(&mut sum);
            max_magnitude = std::cmp::max(max_magnitude, calculate_magnitude(&sum));
        }
    }
    println!("{}", max_magnitude);
}


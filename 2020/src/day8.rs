use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Instruction {
    name: String,
    value: i32,
    executed: bool,
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut inss: Vec<Instruction> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        inss.push( Instruction {
            name: line.split(' ').nth(0).unwrap().to_string(),
            value: line.split(' ').nth(1).unwrap().parse().unwrap(),
            executed: false,
        });
    }
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    // part one
    loop {
        if inss[i as usize].executed {
            break;
        }
        inss[i as usize].executed = true;
        match inss[i as usize].name.as_ref() {
            "nop" => i += 1,
            "acc" => { acc += inss[i as usize].value; i += 1 },
            "jmp" => i += inss[i as usize].value,
            _     => panic!("Unexpected instruction"),
        }
    }
    println!("{}", acc);

    // part two
    'outer: for j in 0..inss.len() {
        inss[j].name = match inss[j].name.as_ref() {
            "nop" => "jmp".to_string(),
            "jmp" => "nop".to_string(),
            _     => continue,
        };
        inss.iter_mut().for_each(|ins| ins.executed = false);
        i = 0;
        acc = 0;
        loop {
            if i as usize == inss.len() {
                break 'outer;
            }
            if inss[i as usize].executed {
                break;
            }
            inss[i as usize].executed = true;
            match inss[i as usize].name.as_ref() {
                "nop" => i += 1,
                "acc" => { acc += inss[i as usize].value; i += 1 },
                "jmp" => i += inss[i as usize].value,
                _     => panic!("Unexpected instruction"),
            }
        }
        inss[j].name = match inss[j].name.as_ref() {
            "nop" => "jmp".to_string(),
            "jmp" => "nop".to_string(),
            _     => panic!("That shouldn't happen"),
        }
    }
    println!("{}", acc);
}

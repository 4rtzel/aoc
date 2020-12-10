use std::fs;
use std::io;
use std::io::BufRead;

enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

struct Code {
    ins: Instruction,
    executed: bool,
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut inss: Vec<Code> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let value = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        inss.push( Code {
            ins: match line.split(' ').nth(0).unwrap() {
                "nop" => Instruction::Nop(value),
                "acc" => Instruction::Acc(value),
                "jmp" => Instruction::Jmp(value),
                _     => panic!("Unexpected value"),
            },
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
        match inss[i as usize].ins {
            Instruction::Nop(_) => i += 1,
            Instruction::Acc(v) => { acc += v; i += 1 },
            Instruction::Jmp(v) => i += v,
        }
    }
    println!("{}", acc);

    // part two
    'outer: for j in 0..inss.len() {
        inss[j].ins = match inss[j].ins {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _                   => continue,
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
            match inss[i as usize].ins {
                Instruction::Nop(_) => i += 1,
                Instruction::Acc(v) => { acc += v; i += 1 },
                Instruction::Jmp(v) => i += v,
            }
        }
        inss[j].ins = match inss[j].ins {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _                   => panic!("That shouldn't happen"),
        }
    }
    println!("{}", acc);
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let mut lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");

    let (_, acc) = run_till_infinite_or_end(&lines_vec);

    println!("acc on first repeated instruction is: {}", acc);

    let mut modified_acc = 0;
    let instruction_count = lines_vec.len();
    for i in 0..lines_vec.len() {
        let new_line;
        match &lines_vec[i][..3] {
            "nop" => {
                new_line = lines_vec[i].replace("nop","jmp");
            },
            "jmp" => {
                new_line = lines_vec[i].replace("jmp","nop");
            },
            "acc" => continue,
            u => panic!("Encountered unknown instruction code: {}", u)
        }
        
        let old_line = std::mem::replace(&mut lines_vec[i], new_line);
        let (ip, acc) = run_till_infinite_or_end(&lines_vec);
        if ip == instruction_count {
            modified_acc = acc;
            break;
        }
        lines_vec[i] = old_line;
    }
    println!("acc value when program exists with one modified instruction: {}", modified_acc)
}

fn run_till_infinite_or_end(instruction_lines: &Vec<String>) -> (usize, i32) {
    let instruction_count = instruction_lines.len();
    let mut run_instructions: HashSet<usize> = HashSet::new();
    let mut ip = 0;
    let mut acc = 0;

    while !run_instructions.contains(&ip) {
        run_instructions.insert(ip);
        let (op, param) = instruction_lines[ip].split_at(4);
        match op {
            "jmp " => {
                let param_num: i32 = match param.parse() {
                    Ok(x) => x,
                    Err(e) => {panic!("Couldn't parse parameter for op at {} ({}{}); Err: {}", ip, op, param, e)}
                };
                if param_num < 0 {
                    ip -= param_num.abs() as usize;
                } else {
                    ip += param_num as usize;
                }
            },
            "acc " => {
                let param_num: i32 = match param.parse() {
                    Ok(x) => x,
                    Err(e) => {panic!("Couldn't parse parameter for op at {} ({}{}); Err: {}", ip, op, param, e)}
                };
                acc += param_num;
                ip += 1;
            },
            "nop " => {
                ip += 1;
            },
            _ => {
                panic!("Unknown instruction: {} {}!", op, param);
            }
        }

        if ip >= instruction_count {
            ip = instruction_count;
            break;
        }
    }

    (ip, acc)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
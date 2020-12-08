use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");

    let mut run_instructions: HashSet<usize> = HashSet::new();
    let mut ip = 0;
    let mut acc = 0;
    while !run_instructions.contains(&ip) {
        run_instructions.insert(ip);
        let (op, param) = lines_vec[ip].split_at(4);
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
    }

    println!("acc on first repeated instruction is: {}", acc);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
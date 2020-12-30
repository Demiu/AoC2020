use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)
        .expect("Couldn't read input to string");

    let numbers: Vec<u32> = file_str
        .lines()
        .next()
        .expect("Couldn't extract first line from file")
        .split(',')
        .map(move |s| s.parse())
        .collect::<Result<_, _>>()
        .expect("Cannot parse all numbers to a vec");
    
    // last starting number is treated like non-starting
    let mut number_memory = HashMap::new();
    for i in 0..(numbers.len() - 1) {
        number_memory.insert(numbers[i], (i+1) as u32);
    }

    let starting_turn = numbers.len();
    let mut last_number = *numbers.last().unwrap();
    for turn in starting_turn..2020 {
        match number_memory.insert(last_number, turn as u32) {
            Some(last_seen) => {
                last_number = turn as u32 - last_seen;
            },
            None => {
                last_number = 0;
            }
        }
    }

    println!("Number on turn 2020: {}", last_number);
}
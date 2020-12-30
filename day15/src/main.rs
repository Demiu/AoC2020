use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_str = read_to_string("./input.txt")
        .expect("Couldn't read input to string");

    let mut numbers: Vec<u32> = file_str
        .lines() // the input.txt file has a trailing newline
        .next()
        .expect("Couldn't extract first line from file")
        .split(',')
        .map(move |s| s.parse())
        .collect::<Result<_, _>>()
        .expect("Cannot parse all numbers to a vec");
    
    // last starting number is treated like non-starting
    let mut number_memory = HashMap::new();
    let starting_turn = numbers.len();
    let mut next_number = numbers.pop().unwrap();
    for (i, num) in numbers.into_iter().enumerate() {
        number_memory.insert(num, (i + 1) as u32);
    }

    for turn in starting_turn..2020 {
        match number_memory.insert(next_number, turn as u32) {
            Some(last_seen) => {
                next_number = turn as u32 - last_seen;
            },
            None => {
                next_number = 0;
            }
        }
    }

    println!("Number on turn 2020: {}", next_number);

    for turn in 2020..30000000 {
        match number_memory.insert(next_number, turn as u32) {
            Some(last_seen) => {
                next_number = turn as u32 - last_seen;
            },
            None => {
                next_number = 0;
            }
        }
    }
    
    println!("Number on turn 30000000: {}", next_number);
}
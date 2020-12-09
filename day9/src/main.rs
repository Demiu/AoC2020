use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");
    let numbers: Vec<u64> = lines_vec
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut last_numbers = VecDeque::<u64>::new();
    for num in numbers.iter().take(25) {
        last_numbers.push_back(*num);
    }

    let mut p1_answer = 0;
    for num in numbers.iter().skip(25) {
        if !can_two_elems_sum(&last_numbers, *num) {
            p1_answer = *num;
            break;
        }
        last_numbers.push_back(*num);
        last_numbers.pop_front();
    }

    println!("Answer to part 1: {}", p1_answer);

    last_numbers.clear();
    let mut it = numbers.iter();
    let mut sum = 0;
    while let Some(num) = it.next() {
        sum += num;
        last_numbers.push_back(*num);
        while sum > p1_answer {
            if let Some(popped) = last_numbers.pop_front() {
                sum -= popped;
            } else {
                break;
            }
        }
        if sum == p1_answer {
            break;
        }
    }

    let p2_answer = last_numbers.iter().max().unwrap() + last_numbers.iter().min().unwrap();
    println!("Answer to part 2: {}", p2_answer);
}

fn can_two_elems_sum(deque: &VecDeque<u64>, sum: u64) -> bool {
    for i in 0..deque.len() {
        for j in 0..deque.len() {
            if deque[i] + deque[j] == sum { return true; } 
        }
    }
    return false;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
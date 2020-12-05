use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::{min, max};

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");
    let seat_ids = lines_vec.iter().fold((u32::MAX, 0, 0), |a, s| day5_seats_fold(a, s));
    let (min_seat, max_seat, seat_sum) = seat_ids;

    println!("Max seat ID found {}", max_seat);

    let full_seat_count = max_seat - min_seat + 1;
    let full_seat_sum = (full_seat_count * (min_seat + max_seat)) / 2;
    let my_seat = full_seat_sum - seat_sum;

    println!("My seat ID is {}", my_seat);
}

fn day5_seats_fold(accu: (u32, u32, u32), s: &String) -> (u32, u32, u32) {
    let seat_value = s.chars().fold(0, |acc, c| day5_char_to_bin(acc, c));
    return (min(accu.0, seat_value), 
            max(accu.1, seat_value), 
            accu.2 + seat_value);
}

fn day5_char_to_bin(accu: u32, c: char) -> u32 {
    let c_val = match c {
        'B' | 'R' => 1,
        'F' | 'L' => 0,
        _ => 0
    };
    return accu*2 + c_val;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
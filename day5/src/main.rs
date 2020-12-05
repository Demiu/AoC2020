use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");
    let max_seat_id = lines_vec.iter().map(|s| s.chars().fold(
            0, |acc, c| day5_char_to_bin(acc, c))
        ).max().unwrap();

    println!("Max seat ID found {}", max_seat_id);
}

fn day5_char_to_bin(accu: u32, c: char) -> u32 {
    let c_val = if c=='B' || c=='R' {1} else {0};
    return accu*2 + c_val;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
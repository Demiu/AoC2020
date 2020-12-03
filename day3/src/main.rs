use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(mut lines) = read_lines("./input.txt") {
        let mut trees_hit: u32 = 0;
        let mut horizontal_pos: usize = 0;
        let horizontal_change: usize = 3;
        let line_width = lines.next().unwrap().unwrap().chars().count();

        while let Some(Ok(line)) = lines.next() {
            horizontal_pos = horizontal_pos + horizontal_change;
            horizontal_pos -= if horizontal_pos >= line_width { line_width } else { 0 };

            if line.chars().nth(horizontal_pos).unwrap() == '#' {
                trees_hit += 1;
            }
        }

        println!("Hit {} trees on the path.", trees_hit);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
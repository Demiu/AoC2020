use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut valid_count: u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                let parts: Vec<&str> = line_str.split(' ').collect();
                let bounds: Vec<&str> = parts[0].split('-').collect();
                let lower_bound = bounds[0].parse::<u32>().unwrap();
                let upper_bound = bounds[1].parse::<u32>().unwrap();
                let character: char = parts[1].chars().next().unwrap();
                let password = parts[2];
                let char_count = password.matches(character).count() as u32;
                if char_count >= lower_bound && char_count <= upper_bound {
                    valid_count += 1;
                }
            }
        }

        println!("Found {} valid passwords.", valid_count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
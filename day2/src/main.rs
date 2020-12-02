use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut valid_count1: u32 = 0;
        let mut valid_count2: u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                // part 1
                let parts: Vec<&str> = line_str.split(' ').collect();
                let bounds: Vec<&str> = parts[0].split('-').collect();
                let lower_bound = bounds[0].parse::<u32>().unwrap();
                let upper_bound = bounds[1].parse::<u32>().unwrap();
                let character: char = parts[1].chars().next().unwrap();
                let password = parts[2];
                let char_count = password.matches(character).count() as u32;
                if char_count >= lower_bound && char_count <= upper_bound {
                    valid_count1 += 1;
                }

                // part 2
                let lower_matches = password.chars().nth(lower_bound as usize - 1).unwrap() == character;
                let upper_matches = password.chars().nth(upper_bound as usize - 1).unwrap() == character;
                if lower_matches != upper_matches {
                    valid_count2 += 1;
                }
            }
        }

        println!("Found {} valid passwords for part 1\nFound {} valid passwords for part 2"
                , valid_count1, valid_count2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
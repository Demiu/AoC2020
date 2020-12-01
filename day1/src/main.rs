use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
     if let Ok(lines) = read_lines("./input.txt") {
        let mut numbers = vec![];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                if let Ok(number) = line_str.parse::<u32>() {
                    numbers.push(number);
                }
            }
        }

        'search_2: for i in 0 .. numbers.len() {
            for j in (i+1) .. numbers.len() {
                let sum = numbers[i] + numbers[j];
                if sum == 2020 {
                    let mul = numbers[i] * numbers[j];
                    println!("{}", mul);
                    break 'search_2;
                }
            }
        }

        'search_3: for i in 0 .. numbers.len() {
            for j in (i+1) .. numbers.len() {
                for k in (j+1) .. numbers.len() {
                    let sum = numbers[i] + numbers[j] + numbers[k];
                    if sum == 2020 {
                        let mul = numbers[i] * numbers[j] * numbers[k];
                        println!("{}", mul);
                        break 'search_3;
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
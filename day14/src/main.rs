use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines
        .collect::<Result<_, _>>()
        .expect("Couldn't collect lines into a vec");
    
    // simplified function; A is override, B is override value, C is input:
    // out = (A and B) or (!A and C)
    let mut mask_override: u64 = 0; // if bit positive mask_override overrides value
    let mut mask_override_val: u64 = 0;
    let mut mem = HashMap::new();
    for line in lines_vec {
        let line_segments: Vec<_> = line.split(' ').collect();
        if line_segments[0].starts_with("mas") {
            // no deconstructing assignments :(
            let new_masks = parse_mask(line_segments[2]); 
            mask_override = new_masks.0;
            mask_override_val = new_masks.1;
        } else { // starts_with("mem")
            let address_str_bracket = line_segments[0].split_at(4).1;
            let address: u32 = address_str_bracket[0..(address_str_bracket.len() - 1)]
                .parse().expect("Couldn't parse memory address");
            let val: u64 = line_segments[2]
                .parse().expect("Couldn't parse memory value");
            let masked_val = ( mask_override & mask_override_val) | 
                             (!mask_override & val);
            mem.insert(address, masked_val);
        }
    }

    let mut sum = 0;
    for (_, v) in mem {
        sum += v;
    }

    println!("Part 1 answer: {}", sum);
}

// returns override and value masks
fn parse_mask(mask_string: &str) -> (u64, u64) {
    let mut or = 0;
    let mut val = 0;
    for c in mask_string.chars() {
        or <<= 1;
        val <<= 1;
        match c {
            '0' => {
                or += 1; // val is already 0
            },
            '1' => {
                or += 1;
                val += 1;
            }
            'X' => (), // both are already 0
            _ => panic!("Cannot parse character in mask: {}", c)
        }
    }
    return (or, val);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
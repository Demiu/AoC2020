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
    for line in lines_vec.iter() {
        let line_segments: Vec<_> = line.split(' ').collect();
        if line_segments[0].starts_with("mas") {
            // no deconstructing assignments :(
            let new_masks = parse_mask(line_segments[2]); 
            mask_override = new_masks.0;
            mask_override_val = new_masks.1;
        } else { // starts_with("mem")
            let address_str_bracket = line_segments[0].split_at(4).1;
            let address: u64 = address_str_bracket[0..(address_str_bracket.len() - 1)]
                .parse().expect("Couldn't parse memory address");
            let val: u64 = line_segments[2]
                .parse().expect("Couldn't parse memory value");
            let masked_val = ( mask_override & mask_override_val) | 
                             (!mask_override & val);
            mem.insert(address, masked_val);
        }
    }

    let mut sum = 0;
    for (_, v) in mem.iter() {
        sum += v;
    }

    println!("Part 1 answer: {}", sum);

    // values that when added to address flip one floating bit
    mask_override = 0;
    let mut override_values = vec![];
    mem.clear();
    for line in lines_vec.iter() {
        let line_segments: Vec<_> = line.split(' ').collect();
        if line_segments[0].starts_with("mas") {
            // no deconstructing assignments :(
            let new_masks_floating = parse_mask_floating(line_segments[2]); 
            mask_override = new_masks_floating.0;
            override_values = new_masks_floating.1;
        } else { // starts_with("mem")
            let address_str_bracket = line_segments[0].split_at(4).1;
            let address: u64 = address_str_bracket[0..(address_str_bracket.len() - 1)]
                .parse().expect("Couldn't parse memory address");
            let val: u64 = line_segments[2]
                .parse().expect("Couldn't parse memory value");

            for ov in override_values.iter() {
                let masked_address = (mask_override & *ov) | 
                                     (!mask_override & address);
                mem.insert(masked_address, val);
            }
        }
    }

    let mut sum = 0;
    for (_, v) in mem.iter() {
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

// returns override mask and value masks
fn parse_mask_floating(mask_string: &str) -> (u64, Vec<u64>) {
    let mut or = 0;
    let mut vals = vec![0];
    for c in mask_string.chars() {
        or <<= 1;
        for v in vals.iter_mut() {
            *v <<= 1;
        }
        match c {
            '0' => (), // in part 2 '0' means unchanged bit
            '1' => {
                or += 1;
                for v in vals.iter_mut() {
                    *v += 1;
                }
            }
            'X' => { 
                or += 1;
                // values for override with value 0 already exist
                // new values need to be added for override with 1
                // precache len as we'll grow vals
                let old_vals_len = vals.len();
                for i in 0..old_vals_len {
                    vals.push(vals[i] + 1);
                }
            }, 
            _ => panic!("Cannot parse character in mask: {}", c)
        }
    }
    return (or, vals);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Couldn't read input to string");

    let part1_regex_str = r"(?m)(^\s^|^)((((byr|iyr|eyr|hgt|hcl|ecl|pid|cid):.*?)($| |$\s^)){8}|(((byr|iyr|eyr|hgt|hcl|ecl|pid):.*?)($| |$\s^)){7})";

    let part1_regex = Regex::new(part1_regex_str).expect("Failed creating regex");
    let matches = part1_regex.find_iter(&file_str).count();

    println!("Found {} matches", matches);
}
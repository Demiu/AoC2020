use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Couldn't read input to string");

    let part1_regex_str = concat!(
        r"(?m)(^\s^|^)(",
        r"(((byr|iyr|eyr|hgt|hcl|ecl|pid|cid):.*?)($| |$\s^)){8}|",
        r"(((byr|iyr|eyr|hgt|hcl|ecl|pid):.*?)($| |$\s^)){7})");
    let part1_regex = Regex::new(part1_regex_str).expect("Failed creating regex for part 1");
    let part1_matches = part1_regex.find_iter(&file_str).count();
    println!("Found {} matches for part 1", part1_matches);

    let part2_regex_str = concat!(
        r"(?m)(^\s^|^)",
        r"((((",
            r"byr:(19[2-9][0-9]|200[012])|iyr:20(1[0-9]|20)|eyr:20(2[0-9]|30)|",
            r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)|hcl:#([a-f]|[0-9]){6}|",
            r"ecl:(amb|blu|brn|gry|grn|hzl|oth)|pid:[0-9]{9}|cid.*?))($| |$\s^)){8}",
        r"|(((",
            r"byr:(19[2-9][0-9]|200[012])|iyr:20(1[0-9]|20)|eyr:20(2[0-9]|30)|",
            r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)|hcl:#([a-f]|[0-9]){6}|",
            r"ecl:(amb|blu|brn|gry|grn|hzl|oth)|pid:[0-9]{9}))($| |$\s^)){7})"
    );
    let part2_regex = Regex::new(part2_regex_str).expect("Failed creating regex for part 2");
    let part2_matches = part2_regex.find_iter(&file_str).count();
    println!("Found {} matches for part 2", part2_matches);
}
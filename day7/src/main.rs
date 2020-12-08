use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");

    let mut rules: HashMap<_,_> = HashMap::new();
    for line in lines_vec.iter() {
        let mut words = line.split_whitespace();

        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();
        let parent = format!("{} {}", first_word, second_word);

        // skip to beginning of 1st child
        words.nth(1);
        let children = match words.next().unwrap() {
            "no" => vec![],
            num => {
                let mut ret = vec![];

                let count: u32 = num.parse().unwrap();
                let child_first_word = words.next().unwrap();
                let child_second_word = words.next().unwrap();
                ret.push((format!("{} {}", child_first_word, child_second_word), count));

                while words.next().unwrap().chars().last().unwrap() == ',' {
                    let count: u32 = words.next().unwrap().parse().unwrap();
                    let child_first_word = words.next().unwrap();
                    let child_second_word = words.next().unwrap();
                    ret.push((format!("{} {}", child_first_word, child_second_word), count));
                }
                ret
            }
        };

        rules.insert(parent, children);
    }

    let my_bag = "shiny gold".to_owned();

    let mut found = 0;
    for parent in rules.keys() {
        if bag_can_be_in(&my_bag, parent, &rules) {
            found += 1;
        }
    }
    println!("Part 1 answer: {}", found);

    let bags_in_mine = total_bags_in_bag(&my_bag, &rules) - 1;
    println!("Part 2 answer: {}", bags_in_mine);
}

fn bag_can_be_in(inner: &String, outer: &String, rules: &HashMap<String, Vec<(String, u32)>>) -> bool{
    for (bag, _) in &rules[outer] {
        if bag == inner {
            return true;
        }
    }
    for (bag, _) in &rules[outer] {
        if bag_can_be_in(inner, bag, rules) {
            return true;
        }
    }

    return false;
}

// includes the "outer" bag
fn total_bags_in_bag(bag: &String, rules: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut total = 1;
    for (subbag, count) in &rules[bag] {
        total += count * total_bags_in_bag(subbag, &rules);
    }
    return total;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
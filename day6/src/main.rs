use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Couldn't read input to string");
    let groups: Vec<_> = file_str.split("\n\n").collect();

    let answer = groups.iter().fold(0, 
        |sum, s| sum + s.chars().fold(HashMap::new(), 
            |mut map, c| if c != '\n' 
                {
                    map.insert(c, 1);
                    printMap(&map);
                    map
                } else {map}
            ).keys().len());
    
    println!("Part 1 answer: {}", answer);
}

fn printMap(map: &HashMap<char, i32>) {
    for (k, v) in map {
        println!("Key: {} Value: {}", k, v);
    }
    println!("\n");
}
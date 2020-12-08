use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Couldn't read input to string");
    let groups: Vec<_> = file_str.split("\n\n").collect();

    let answer1 = groups.iter().fold(0, 
        |sum, s| sum + s.chars().fold(HashMap::new(), 
            |mut map, c| if c != '\n' 
                {
                    map.insert(c, 1);
                    map
                } else {map}
            ).keys().len());
    
    println!("Part 1 answer: {}", answer1);

    let answer2 = groups.iter().fold(0, 
        |sum, s| sum + 
            {
                let mut chars_hashmap = s.chars().fold(HashMap::new(), 
                    |mut map, c| 
                        {
                            match map.insert(c, 1) {
                                Some(x) => map.insert(c, x+1),
                                None    => None
                            };
                            map
                        }
                    );
                // sadly, last line in input has a trailing newline that isn't split off
                // so it has to be accounted for here
                if s.chars().last().unwrap() == '\n' {
                    *chars_hashmap.get_mut(&'\n').unwrap() -= 1;
                }
                let people_in_group = match chars_hashmap.get(&'\n') {
                    Some(x) => x+1,
                    None    => 1,
                };
                chars_hashmap.iter().fold(0, |sum, kvp| sum + if (*kvp.1) == people_in_group {1} else {0})
            });
    
    println!("Part 2 answer: {}", answer2);
}
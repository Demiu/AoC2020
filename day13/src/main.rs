use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines
        .collect::<Result<_, _>>()
        .expect("Couldn't collect lines into a vec");
    
    let delay: u32 = lines_vec[0].parse().expect("Couldn't parse earliest timestamp");
    let bus_ids: Vec<Option<u32>> = lines_vec[1].split(',').map(|n| n.parse().ok()).collect();

    let mut earliest_id = 0;
    let mut earliest_wait = u32::MAX;
    for bus_time_opt in bus_ids {
        let bus_time = match bus_time_opt {
            Some(b) => b,
            None => continue,
        };
        let count = delay / bus_time;
        if count == 0 { // perfect alignment
            earliest_id = bus_time;
            break;
        }
        let wait_time = (bus_time * (count + 1)) - delay;
        if wait_time < earliest_wait {
            earliest_id = bus_time;
            earliest_wait = wait_time;
        }
    }

    println!("P1 answer: {}", earliest_id * earliest_wait);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
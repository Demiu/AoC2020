use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().expect("Couldn't collect lines into a vec");
    let mut numbers: Vec<u32> = lines_vec
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();
    numbers.sort_unstable();

    let mut prev_jolt = 0;
    let mut step_ups1 = 0;
    let mut step_ups3 = 1; // the final step up to the device
    for n in numbers.iter().copied() {
        match n - prev_jolt {
            1 => step_ups1 += 1,
            2 => (),
            3 => step_ups3 += 1,
            u => panic!("Unexpected step-up of {}", u)
        }
        prev_jolt = n;
    }

    println!("Part 1 answer: {}", step_ups1 * step_ups3);

    let mut prev_jolt = 0;
    let mut possible_connections: Vec<u64> = vec![0, 0, 1]; // three zeroes to "preseed"
    for n in numbers {
        let jolt_gap = n-prev_jolt;
        for _ in 1..jolt_gap { 
            // pad the joltages for missing values
            possible_connections.push(0);
        }
        prev_jolt = n;

        let connections_for_n = possible_connections[(n-1) as usize] + 
                                possible_connections[ n    as usize] + 
                                possible_connections[(n+1) as usize];
        possible_connections.push(connections_for_n);
    }

    println!("Part 2 answer: {}", possible_connections.last().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines
        .collect::<Result<_, _>>()
        .expect("Couldn't collect lines into a vec");
    
    let delay: u32 = lines_vec[0]
        .parse()
        .expect("Couldn't parse earliest timestamp");
    let bus_ids: Vec<Option<u32>> = lines_vec[1]
        .split(',')
        .map(|n| n.parse().ok())
        .collect();

    let mut earliest_id = 0;
    let mut earliest_wait = u32::MAX;
    for bus_time_opt in bus_ids.iter() {
        let bus_time = match bus_time_opt {
            Some(b) => b,
            None => continue,
        };
        let count = delay / bus_time;
        if count == 0 { // perfect alignment
            earliest_id = *bus_time;
            break;
        }
        let wait_time = (bus_time * (count + 1)) - delay;
        if wait_time < earliest_wait {
            earliest_id = *bus_time;
            earliest_wait = wait_time;
        }
    }

    println!("P1 answer: {}", earliest_id * earliest_wait);

    let mut big_n: u64 = 1;
    let mut schedule = vec![];
    for i in 0..bus_ids.len() {
        match bus_ids[i] {
            Some(time) => {
                schedule.push((time, i as u32));
                big_n *= time as u64;
            },
            None => ()
        }
    }

    // for chinese remainder theorem: .0 is the remainder, .1 is the divisor
    let mut crt_data = vec![];
    for departure in schedule {
        let divisor = departure.0;
        let offset = departure.1;
        // grow divisor to prevent underflow from substraction 
        // all extra will be modulo'd out anyway
        let mut divisor_grown = divisor;
        while offset > divisor_grown { 
            divisor_grown += departure.0;
        }
        let remainder = (divisor_grown - offset) % divisor;
        crt_data.push((remainder, divisor));
    }

    let mut t_sum: u64 = 0;
    for (remainder, divisor) in crt_data {
        let big_n_i = big_n / divisor as u64;
        let mut big_n_i_inverse: u64 = 1;
        for i in 0..divisor {
            if (big_n_i * i as u64) % divisor as u64 == 1 {
                big_n_i_inverse = i as u64;
                break;
            }
        }
        t_sum += remainder as u64 * big_n_i * big_n_i_inverse;
    }
    let t = t_sum % big_n;

    println!("Part 2 answer: {}", t);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
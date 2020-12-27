use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn main() {
    let mut file = File::open("./input.txt").expect("Couldn't find input file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)
        .expect("Couldn't read input to string");

    let mut layout: Vec<Vec<Seat>> = vec![vec![]];
    for c in file_str.chars() {
        let layout1_last_index = layout.len() - 1;
        match c {
            '.' => layout[layout1_last_index].push(Seat::Floor),
            'L' => layout[layout1_last_index].push(Seat::Empty),
            '#' => layout[layout1_last_index].push(Seat::Occupied),
            '\n' => layout.push(vec![]),
            u => panic!("Unhandled seat type: {}", u),
        }
    }
    if layout.last().unwrap().len() == 0 { // clear the last empty line
        layout.pop();
    }

    let mut buffer1 = layout.clone();
    let mut buffer2 = layout;
    while step(&buffer1, &mut buffer2) != 0 { 
        let swap = buffer1;
        buffer1 = buffer2;
        buffer2 = swap;
    }

    println!("Part 1 answer: {}", count_occupied(&buffer2));
}

#[allow(dead_code)]
fn print_layout(l: &Vec<Vec<Seat>>) -> () {
    for line in l {
        for s in line {
            let c = match s {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#'
            };
            print!("{}", c);
        }
        print!("\n");
    }
}

fn step(i: &Vec<Vec<Seat>>, o: &mut Vec<Vec<Seat>>) -> u32 {
    let mut changed = 0;
    for y in 0..i.len() {
        for x in 0..i[y].len() {
            match i[y][x] {
                Seat::Floor => (),
                Seat::Empty => {
                    if get_neighbouring_occupied_count(i, x, y) == 0 {
                        o[y][x] = Seat::Occupied;
                        changed += 1;
                    } else {
                        o[y][x] = Seat::Empty;
                    }
                },
                Seat::Occupied => {
                    if get_neighbouring_occupied_count(i, x, y) > 3 {
                        o[y][x] = Seat::Empty;
                        changed += 1;
                    } else {
                        o[y][x] = Seat::Occupied;
                    }
                }
            }
        }
    }
    return changed;
}

fn get_neighbouring_occupied_count(l: &Vec<Vec<Seat>>, x: usize, y: usize) -> u32 {
    let mut occupied = 0;
    let left = x > 0;
    let right = x + 1 < l[y].len();
    let top = y > 0;
    let bottom = y + 1 < l.len();
    if left {
        occupied += if let Seat::Occupied = l[y][x-1] {1} else {0};
        if top {
            occupied += if let Seat::Occupied = l[y-1][x-1] {1} else {0};
        }
        if bottom {
            occupied += if let Seat::Occupied = l[y+1][x-1] {1} else {0};
        }
    }
    if right {
        occupied += if let Seat::Occupied = l[y][x+1] {1} else {0};
        if top {
            occupied += if let Seat::Occupied = l[y-1][x+1] {1} else {0};
        }
        if bottom {
            occupied += if let Seat::Occupied = l[y+1][x+1] {1} else {0};
        }
    }
    if top {
        occupied += if let Seat::Occupied = l[y-1][x] {1} else {0};
    }
    if bottom {
        occupied += if let Seat::Occupied = l[y+1][x] {1} else {0};
    }
    return occupied;
}

fn count_occupied(l: &Vec<Vec<Seat>>) -> u32 {
    let mut occupied = 0;
    for line in l {
        for seat in line {
            if let Seat::Occupied = seat {
                occupied += 1;
            }
        }       
    }
    return occupied;
}
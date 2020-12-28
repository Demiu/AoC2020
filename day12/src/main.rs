use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

enum Direction {
    North,
    South,
    East,
    West
}

struct Point {
    x: i32,
    y: i32
}

impl Action {
    fn to_direction(&self) -> Option<Direction> {
        match self {
            Action::North => Some(Direction::North),
            Action::South => Some(Direction::South),
            Action::East => Some(Direction::East),
            Action::West => Some(Direction::West),
            _ => None,
        }
    }
}

impl Direction {
    fn transform_direction(&mut self, action: Action, value: i32) -> () {
        match action {
            Action::Left => {
                *self = match (&self,value) {
                    (Direction::South, 180) | 
                    (Direction::East, 90) |
                    (Direction::West, 270) 
                        => Direction::North,
                    (Direction::North, 180) | 
                    (Direction::East, 270) |
                    (Direction::West, 90) 
                        => Direction::South,
                    (Direction::North, 270) | 
                    (Direction::South, 90) |
                    (Direction::West, 180) 
                        => Direction::East,
                    (Direction::North, 90) | 
                    (Direction::South, 270) |
                    (Direction::East, 180) 
                        => Direction::West,
                    _ => panic!("Unhandled rotation value: {}", value),
                }
            },
            Action::Right => {
                *self = match (&self, value) {
                    (Direction::South, 180) |
                    (Direction::East, 270) |
                    (Direction::West, 90) 
                        => Direction::North,
                    (Direction::North, 180) |
                    (Direction::East, 90) |
                    (Direction::West, 270)
                        => Direction::South,
                    (Direction::North, 90) |
                    (Direction::South, 270) |
                    (Direction::West, 180) 
                        => Direction::East,
                    (Direction::North, 270) |
                    (Direction::South, 90) |
                    (Direction::East, 180) 
                        => Direction::West,
                    _ => panic!("Unhandled rotation value: {}", value),
                }
            },
            _ => (panic!("Unhandled action in transform_direction"))
        };
    }
}

impl Point {
    fn move_in_direction(&mut self, direction: &Direction, value: i32) -> () {
        match direction {
            Direction::North => self.y += value,
            Direction::South => self.y -= value,
            Direction::East => self.x += value,
            Direction::West => self.x -= value,
        }
    }

    fn rotate_around_origin(&mut self, action: Action, value: i32) -> () {
        match (action, value) {
            (Action::Right, 90) | (Action::Left, 270) 
                => *self = Point {x: self.y, y: -self.x},
            (Action::Right, 270) | (Action::Left, 90) 
                => *self = Point {x: -self.y, y: self.x},
            (Action::Right, 180) | (Action::Left, 180) 
                => *self = Point {x: -self.x, y: -self.y},
            _ => panic!("Unhandled action in rotate_around_origin"),
        }
    }
}

fn main() {
    let lines = read_lines("./input.txt").expect("Couldn't read input.txt");
    let lines_vec: Vec<_> = lines
        .collect::<Result<_, _>>()
        .expect("Couldn't collect lines into a vec");

    let instructions: Vec<_> = lines_vec
        .iter()
        .map(|line| {
            let (action_str, value_str) = line.split_at(1);
            let action = match action_str {
                "N" => Action::North,
                "S" => Action::South,
                "E" => Action::East,
                "W" => Action::West,
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => panic!("Unable to parse action \"{}\"", action_str)
            };
            let value: i32 = value_str.parse().unwrap();
            (action, value)
        })
        .collect();
    
    let mut pos = Point{x: 0, y: 0};
    let mut dir = Direction::East;
    for i in instructions.iter() {
        execute_instruction_p1(*i, &mut pos, &mut dir);
    }

    println!("Final position for part 1: [{},{}]; Manhattan distance: {}", 
        pos.x, pos.y, pos.x.abs() + pos.y.abs());

    let mut bpos = Point{x: 0, y: 0};
    let mut wpos = Point{x: 10, y: 1};
    for i in instructions.iter() {
        execute_instruction_p2(*i, &mut bpos, &mut wpos);
    }
    
    println!("Final position for part 2: [{},{}]; Manhattan distance: {}", 
        bpos.x, bpos.y, bpos.x.abs() + bpos.y.abs());
}

fn execute_instruction_p1(instruction: (Action, i32), position: &mut Point, 
                          direction: &mut Direction) -> () 
{
    match instruction.0 {
        Action::Left | Action::Right => {
            direction.transform_direction(instruction.0, instruction.1);
        },
        Action::North | Action::South | Action::East | Action::West => {
            match instruction.0.to_direction() {
                Some(d) => position.move_in_direction(&d, instruction.1),
                None => panic!("Wrong action entered direction move branch"),
            }
        },
        Action::Forward => {
            position.move_in_direction(direction, instruction.1);
        },
    }
}

fn execute_instruction_p2(instruction: (Action, i32), boat_position: &mut Point, 
                          waypoint_position: &mut Point) -> () 
{
    match instruction.0 {
        Action::Left | Action::Right => {
            waypoint_position.rotate_around_origin(instruction.0, instruction.1);
        },
        Action::North | Action::South | Action::East | Action::West => {
            match instruction.0.to_direction() {
                Some(d) => waypoint_position.move_in_direction(&d, instruction.1),
                None => panic!("Wrong action entered waypoint move branch"),
            }
        },
        Action::Forward => {
            *boat_position = Point{
                x: boat_position.x + (waypoint_position.x * instruction.1), 
                y: boat_position.y + (waypoint_position.y * instruction.1)};
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
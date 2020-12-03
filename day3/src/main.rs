use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let lines_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
        let line_width = lines_vec[0].len();

        // part 1
        let hits_3_1 = slope_trees(&lines_vec, line_width, 3, 1);
        println!("Hit {} trees on the 3 right, 1 down path.", hits_3_1);

        // part 2
        let hits_1_1 = slope_trees(&lines_vec, line_width, 1, 1);
        let hits_5_1 = slope_trees(&lines_vec, line_width, 5, 1);
        let hits_7_1 = slope_trees(&lines_vec, line_width, 7, 1);
        let hits_1_2 = slope_trees(&lines_vec, line_width, 1, 2);
        let hits = hits_1_1 * hits_3_1 * hits_5_1 * hits_7_1 * hits_1_2;
        println!("The product of tree hits for all tested slopes is {}", hits);
    }
}

fn slope_trees(lines: &Vec<String>, line_width: usize, right: usize, down: usize) -> u32 {
    let mut trees_hit: u32 = 0;
    let mut horizontal_pos: usize = right;

    for line in lines.iter().skip(down).step_by(down) {
        if line.chars().nth(horizontal_pos).unwrap() == '#' {
            trees_hit += 1;
        }

        horizontal_pos = horizontal_pos + right;
        horizontal_pos -= if horizontal_pos >= line_width { line_width } else { 0 };
    }
    return trees_hit;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
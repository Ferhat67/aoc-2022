/********************************************
 * Day 4: Camp Cleanup
 * https://adventofcode.com/2022/day/4
 ********************************************/

use std::{path::Path, fs::File, io::Read, ops::Range};

const _DEBUG: bool = false;

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} pairs. Parsing ranges and calculating overlaps...", input_string.lines().count());
    let mut fully_contained = 0;
    let mut overlaps = 0;
    for line in input_string.lines() {
        let (first_range, second_range) = parse_ranges(&line);
        if is_fully_contained_in(&first_range, &second_range) || is_fully_contained_in(&second_range, &first_range) {
            fully_contained += 1;
        }
        if is_overlapping_with(&first_range, &second_range) {
            overlaps +=1;
        }
    }
    println!("Found {} pairs where one fully contains the other!", fully_contained);
    println!("Found {} pairs where there is an overlap!", overlaps);

    /***********************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 1000 pairs. Parsing ranges and calculating overlaps...
     * Found 466 pairs where one fully contains the other!
     * Found 865 pairs where there is an overlap!
     ***********************************************/
}

fn is_fully_contained_in(first: &Range<i32>, second: &Range<i32>) -> bool {
    first.clone().all(|section| second.contains(&section))
}

fn is_overlapping_with(first: &Range<i32>, second: &Range<i32>) -> bool {
    for section in first.clone() {
        if second.contains(&section) {
            return true;
        }
    }
    false
}

fn parse_ranges(line: &str) -> (Range<i32>,Range<i32>) {
    let pair: Vec<&str> = line.split(",").collect();
    let first: Vec<i32> = pair[0].split("-").flat_map(|a| a.parse::<i32>()).collect(); 
    let second: Vec<i32> = pair[1].split("-").flat_map(|a| a.parse::<i32>()).collect();
    let first_range = first[0]..first[1]+1;     // [start,end+1)
    let second_range = second[0]..second[1]+1;  // [start,end+1)
    (first_range, second_range)
}

fn read_input() -> String {
    let input_path = Path::new("./src/input.txt");
    let mut input_file = match File::open(&input_path) {
        Err(error) => panic!("Failed to open {} - error: {}", input_path.display(), error),
        Ok(file) => file,
    };
    let mut input_string = String::new();
    match input_file.read_to_string(&mut input_string) {
        Err(error) => panic!("Failed to read content of {} - error: {}", input_path.display(), error),
        Ok(_bytelength) => println!("Successfully read {}", input_path.display()),
    };
    input_string
}

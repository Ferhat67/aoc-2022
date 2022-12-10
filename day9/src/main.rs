/********************************************
 * Day 9: Rope Bridge
 * https://adventofcode.com/2022/day/9
 ********************************************/

use std::{collections::HashSet, fs::File, io::Read, path::Path, u8};

use regex::Regex;

const _DEBUG: bool = true;

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: char,
    steps: i32,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Knot {
    position: Position,
    trail: Vec<Position>,
}

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} lines", input_string.lines().count());

    let motions = parse_head_motions(&input_string);

    let knots = simulate_rope(2, &motions);
    let tail_positions = calc_number_of_unique_positions(&knots[1].trail);
    println!("Tail of the two-knotted rope visits {} positions!", tail_positions);
    assert_eq!(6486, tail_positions);

    let knots = simulate_rope(10, &motions);
    let tail_positions = calc_number_of_unique_positions(&knots[9].trail);
    println!("Tail of the ten-knotted rope visits {} positions!", tail_positions);
    assert_eq!(2678, tail_positions);

    /***********************************************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 2000 lines
     * Tail of the two-knotted rope visits 6486 positions!
     * Tail of the ten-knotted rope visits 2678 positions!
     ***********************************************************************************/
}

fn calc_number_of_unique_positions(trail: &Vec<Position>) -> usize {
    trail
    .iter()
        // Position struct needs eq/hash method implemented to rule out duplicates
        // but time is points ... so lets compare strings instead
        .map(|p| format!("{}/{}", p.x, p.y))
        .collect::<HashSet<String>>()
        .len()
}

fn build_rope(rope_length: u8) -> Vec<Knot> {
    const S: Position = Position { x: 0, y: 0 };
    (0..rope_length).map(|_| Knot { position: S, trail: vec![S] }).collect::<Vec<Knot>>()
}

fn simulate_rope(rope_length: u8, motions: &[Motion]) -> Vec<Knot> {
    let mut knots = build_rope(rope_length); 
    for motion in motions {
        for _ in 1..=motion.steps {
            // first move the ropes' head
            let head = &mut knots[0];
            match motion.direction {
                'L' => head.position.x -= 1,
                'R' => head.position.x += 1,
                'U' => head.position.y += 1,
                'D' => head.position.y -= 1,
                _ => panic!("Invalid Direction"),
            };
            head.trail.push(head.position);

            // move the remaining knots by iterating pairwise
            for i in 1..rope_length as usize {
                // We cant have immutable and mutable (or two mutable) borrows/refs of same Array ...
                // ... so we "split" into two separate Slices over same values (just Rust problems)
                let (left, right) = knots.split_at_mut(i);
                let (previous, current) = (left.last().unwrap(), &mut right[0]);
                // move the current knot (local tail) according to the previous one's position (local head)
                move_tail_following_head(previous, current);
                current.trail.push(current.position);
            }
        }
    }
    knots
}

fn move_tail_following_head(head: &Knot, tail: &mut Knot) {
    let dx = (head.position.x - tail.position.x).abs();
    let dy = (head.position.y - tail.position.y).abs();
    if dy > 1 || dx > 1 {
        if head.position.x > tail.position.x {
            tail.position.x += 1;
        } else if head.position.x < tail.position.x {
            tail.position.x -= 1;
        }
        if head.position.y > tail.position.y {
            tail.position.y += 1;
        } else if head.position.y < tail.position.y {
            tail.position.y -= 1;
        }
    }
}

fn parse_head_motions(input_string: &String) -> Vec<Motion> {
    let mut motions = vec![];
    let motion_pattern = Regex::new(r"^([LRUD]{1}) (\d+)$").unwrap();
    for line in input_string.lines() {
        let captures = motion_pattern.captures(line).unwrap();
        let direction = captures[1].to_ascii_uppercase().pop().unwrap() as char;
        let steps = captures[2].parse::<i32>().unwrap();
        motions.push(Motion { direction, steps });
    }
    motions
}

fn read_input() -> String {
    let input_path = Path::new("./src/input.txt");
    let mut input_file = match File::open(&input_path) {
        Err(error) => panic!("Failed to open {} - error: {}", input_path.display(), error),
        Ok(file) => file,
    };
    let mut input_string = String::new();
    match input_file.read_to_string(&mut input_string) {
        Err(error) => panic!(
            "Failed to read content of {} - error: {}",
            input_path.display(),
            error
        ),
        Ok(_bytelength) => println!("Successfully read {}", input_path.display()),
    };
    input_string
}

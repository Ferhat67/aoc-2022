/********************************************
 * Day 5: Supply Stacks
 * https://adventofcode.com/2022/day/5
 ********************************************/

use std::{fs::File, io::Read, path::Path, vec};

use regex::Regex;

const _DEBUG: bool = false;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize
}

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} lines.", input_string.lines().count());

    let (stacks_input, moves_input) = parse_input_sections(&input_string);
    let mut stacks = parse_initial_stacks(stacks_input);
    let moves = parse_moves(moves_input);

    println!("Rearrange crates with CrateMover 9000...");
    rearrange_with_crate_mover_9000(&mut stacks, &moves);
    print_stack_tops(&mut stacks);

    println!("Rearrange crates with CrateMover 9001...");
    stacks = parse_initial_stacks(stacks_input);
    rearrange_with_crate_mover_9001(&mut stacks, &moves);
    print_stack_tops(&mut stacks);

    /********************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 514 lines.
     * Rearrange crates with CrateMover 9000...
     * [S][B][P][Q][R][S][C][D][F]
     * Rearrange crates with CrateMover 9001...
     * [R][G][L][V][R][C][Q][S][B]
     ********************************************/
}

fn rearrange_with_crate_mover_9000(stacks: &mut Vec<Vec<String>>, moves: &Vec<Move>) {
    for (_i, mov) in moves.iter().enumerate() {
        for _j in 1..mov.count + 1 {
            let from_stack = &mut stacks[mov.from];
            let c = &from_stack.pop().unwrap();
            let _ = &stacks[mov.to].push(c.to_string());
        }
    }
}

fn rearrange_with_crate_mover_9001(stacks: &mut Vec<Vec<String>>, moves: &Vec<Move>) {
    for (_i, mov) in moves.iter().enumerate() {
        let mut temp = vec![];
        for _j in 1..mov.count + 1 {
            let from_stack = &mut stacks[mov.from];
            let c = &from_stack.pop().unwrap();
            let _ = &temp.push(c.to_string());
        }
        // push temp slice to restore correct order
        for _j in 0..temp.len() {
            let c = &temp.pop().unwrap();
            let _ = &stacks[mov.to].push(c.to_string());
        }
    }
}

fn print_stack_tops(stacks: &mut Vec<Vec<String>>) {
    // beware that this is mutating the stacks. only print at the end of process
    for i in 1..stacks.len() {
        print!("{}", stacks[i].pop().unwrap().as_str());
    }
    println!();
}

fn parse_moves(moves_input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in moves_input.lines() {
        let move_data: Vec<usize> = line.split(" ").flat_map(|a| a.parse::<usize>()).collect();
        moves.push(Move { from: move_data[1], to: move_data[2], count: move_data[0] });
    }
    if _DEBUG {
        println!("{:#?}", moves);
    }
    moves
}

fn parse_input_sections(input_string: &String) -> (&str, &str) {
    let sections: Vec<&str> = input_string.split("\n\n").collect();
    (sections[0], sections[1])
}

fn parse_initial_stacks(stacks_string: &str) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = vec![vec![]];
    let mut stacks_horizontal: Vec<Vec<String>> = Vec::new();
    let stack_line_pattern = Regex::new(r"(\[[A-Z]{1}\])*").unwrap();
    for line in stacks_string.lines() {
        if !line.starts_with("[") {
            break; // we reached end of stacks block
        }
        // line:            "[V]-----[B]---------------------[F]"
        let trimmed_line = line.replace("    ", " "); // reduce 4 spaces to 1 for regex to work
        let a = trimmed_line.as_str();
        // trimmed_line:    "[V]--[B]------[F]"
        let row = stack_line_pattern
            .find_iter(a)
            .map(|a| a.as_str().to_string())
            .collect::<Vec<String>>();
        // row:             ["[V]", "", "[B]", "", "", "", "", "", "[F]"]
        stacks_horizontal.push(row);
    }
    // transform into vertical stacks...
    for (_i, row) in stacks_horizontal.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if stacks.get(j + 1).is_some() {
                if !cell.is_empty() {
                    stacks[j + 1].push(cell.clone());
                }
            } else {
                if !cell.is_empty() {
                    stacks.push(vec![cell.clone()]);
                } else {
                    stacks.push(vec![]);
                }
            }
        }
    }
    // reverse stacks in correct order (bottom-up)
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }
    if _DEBUG {
        println!("{:#?}", stacks);
    }
    stacks
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

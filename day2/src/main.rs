/********************************************
 * Day 2: Rock Paper Scissors
 * https://adventofcode.com/2022/day/2
 ********************************************/

use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Reading input file...");
    let input_string = read_input();

    let mut total_game_score = 0;
    println!("Calculating total score...");
    for line in input_string.lines() {
        total_game_score += evaluate_score_by_strategy1(line);
        print_round_debug_info(false, line);
    }
    println!("Total score for guessed strategy is {}", total_game_score);

    println!("Recalculating total score with the actual strategy...");
    total_game_score = 0;
    for line in input_string.lines() {
        total_game_score += evaluate_score_by_strategy2(line);
        print_round_debug_info(false, line);
    }
    println!("Real total score is {}", total_game_score);

    /*******************************************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Calculating total score...
     * Total score for guessed strategy is 13682
     * Recalculating total score with the actual strategy...
     * Real total score is 12881
     *******************************************************************************/
}

fn evaluate_score_by_strategy1(round_outcome: &str) -> i32 {
    let outcome_score: i32 = match round_outcome {
        "A X" => 3,     // Rock vs Rock => DRAW
        "B X" => 0,     // Paper vs Rock => LOOSE
        "C X" => 6,     // Scissors vs Rock => WIN
        "A Y" => 6,     // Rock vs Paper => WIN
        "B Y" => 3,     // Paper vs Paper => DRAW
        "C Y" => 0,     // Scissors vs Paper => LOOSE
        "A Z" => 0,     // Rock vs Scissors => LOOSE
        "B Z" => 6,     // Paper vs Scissors => WIN
        "C Z" => 3,     // Scissors vs Scissors => DRAW
        _ => panic!("Unexpected round outcome: {}", round_outcome)
    };
    let shape = round_outcome.chars().nth(2).unwrap();
    let shape_score: i32 = match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unexpected shape: {}", shape)
    };
    outcome_score + shape_score
}

fn evaluate_score_by_strategy2(round_outcome: &str) -> i32 {
    match round_outcome {
        "A X" => 3,     // Loose vs Rock => Scissors (0+3)
        "B X" => 1,     // Loose vs Paper => Rock (0+1)
        "C X" => 2,     // Loose vs Scissors => Paper (0+2)
        "A Y" => 4,     // Draw vs Rock => Rock (3+1)
        "B Y" => 5,     // Draw vs Paper => Paper (3+2)
        "C Y" => 6,     // Draw vs Scissors => Scissors (3+3)
        "A Z" => 8,     // Win vs Rock => Paper (6+2)
        "B Z" => 9,     // Win vs Paper => Scissors (6+3)
        "C Z" => 7,     // Win vs Scissors => Rock (6+1)
        _ => panic!("Unexpected round outcome: {}", round_outcome)
    }
}

fn decode_shape(shape: char) -> &'static str {
    match shape {
        'A' => "ROCK",
        'B' => "PAPER",
        'C' => "SCISSORS",
        'X' => "ROCK",
        'Y' => "PAPER",
        'Z' => "SCISSORS",
        _ => panic!("Unexpected shape: {}", shape)
    }
}

fn print_round_debug_info(print: bool, line: &str) {
    if print {
        let opponent_shape = line.chars().nth(0).unwrap();
        let my_shape = line.chars().nth(2).unwrap();
        println!("{} vs {} => {}", decode_shape(opponent_shape), decode_shape(my_shape), evaluate_score_by_strategy1(line));
    }
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
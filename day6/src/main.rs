/********************************************
 * Day 6: Tuning Trouble
 * https://adventofcode.com/2022/day/6
 ********************************************/

use std::{fs::File, io::Read, path::Path, collections::HashSet};

const _DEBUG: bool = false;

fn main() {
    println!("Reading input file...");
    let input_string = read_input();

    assert_eq!(5, find_start_of_packet(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(6, find_start_of_packet(&String::from("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(10, find_start_of_packet(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(11, find_start_of_packet(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    println!("End of first start-of-packet marker at index:\t {}", find_start_of_packet(&input_string));

    assert_eq!(19, find_start_of_message(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(23, find_start_of_message(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(23, find_start_of_message(&String::from("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(29, find_start_of_message(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(26, find_start_of_message(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    println!("End of first start-of-message marker at index:\t {}", find_start_of_message(&input_string));

    /******************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * End of first start-of-packet marker at index:  1702
     * End of first start-of-message marker at index: 3559
     ******************************************************/
}

fn find_start_of_message(input_string: &String) -> usize {
    for i in 0..input_string.len()-4 {
        let start_marker = &input_string[i..i+4];
        let start_marker_unique_chars = start_marker.chars().collect::<HashSet<char>>();
        if start_marker_unique_chars.len() == 4 {
            let message_marker = &input_string[i..i+14];
            let message_marker_unique_chars = message_marker.chars().collect::<HashSet<char>>().len();
            if message_marker_unique_chars == 14 {
                return i+14;
            }
        }
    }
    panic!("No message found...");
}

fn find_start_of_packet(input_string: &String) -> usize {
    for i in 0..input_string.len()-4 {
        let start_marker = &input_string[i..i+4];
        let unique_chars = start_marker.chars().collect::<HashSet<char>>();
        if unique_chars.len() == 4 {
            return i+4;
        }
    }
    panic!("No packet found...");
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
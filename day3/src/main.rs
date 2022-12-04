/********************************************
 * Day 3: Rucksack Reorganization
 * https://adventofcode.com/2022/day/3
 ********************************************/

use std::{path::Path, fs::File, io::Read, collections::HashSet};

const _DEBUG: bool = false;

fn main() {
    println!("Reading input file...");
    let input_string = read_input();

    if _DEBUG {
        println!("Priority of {} is {}", 'p', get_item_priority('p'));
        println!("Priority of {} is {}", 'L', get_item_priority('L'));
        println!("Priority of {} is {}", 'P', get_item_priority('P'));
        println!("Priority of {} is {}", 'v', get_item_priority('v'));
        println!("Priority of {} is {}", 't', get_item_priority('t'));
        println!("Priority of {} is {}", 's', get_item_priority('s'));
    }

    println!("Comparing both compartments of each rucksack for misplaced items and calculating sum of priorities...");
    let total_misplaced_item_priority: i32 = input_string.lines().map(|line| calc_misplaced_item_priority(line)).sum();
    println!("Sum of priorities of all item types appearing in both compartments of a rucksack: {}", total_misplaced_item_priority);

    println!("Parsing groups of three...");
    let groups_of_three = parse_groups_of_three(input_string.as_str());
    println!("Finding badges and calculating sum of badge item priorities...");
    let total_badge_priority: i32 = groups_of_three.iter().map(|group| calc_badge_priority(group)).sum();
    println!("Sum of all badge item priorities: {}", total_badge_priority);

    /********************************************************************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Comparing both compartments of each rucksack for misplaced items and calculating sum of priorities...
     * Sum of priorities of all item types appearing in both compartments of a rucksack: 7716
     * Parsing groups of three...
     * Finding badges and calculating sum of badge item priorities...
     * Sum of all badge item priorities: 2973
     ********************************************************************************************************/
}

fn calc_misplaced_item_priority(rucksack_line: &str) -> i32 {
    let compartments = parse_rucksack_compartments(rucksack_line);
    let item_types = get_item_types_appearing_in_both_compartments(compartments);
    let priorities: i32 = item_types.iter().map(|i| get_item_priority(i)).sum();
    return priorities;
}

fn parse_rucksack_compartments(rucksack_line: &str) -> (&str, &str) {
    if rucksack_line.len() % 2 != 0 {
        panic!("Invalid input! Uneven length for rucksack {}", rucksack_line);
    }
    let mid = rucksack_line.len() / 2;
    rucksack_line.split_at(mid)
}

fn get_item_priority(item_type: &char) -> i32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priority = letters.find(*item_type).unwrap_or_else(|| panic!("Invalid item type {}", item_type));
    (priority + 1) as i32
}

fn get_item_types_appearing_in_both_compartments(compartments: (&str, &str)) -> HashSet<char> {
    let compartment1 = compartments.0;
    let compartment2 = compartments.1;
    let appearing_in_both: HashSet<char> = compartment1.chars().filter(|item_type| compartment2.contains(*item_type)).collect();
    if _DEBUG {
        println!("{:?} with item types appearing in both: {:?}", compartments, appearing_in_both);
    }
    appearing_in_both
}

fn parse_groups_of_three(input_string: &str) -> Vec<Vec<String>> {
    let mut groups_of_three: Vec<Vec<String>> = Vec::new();
    for (line_index, rucksack) in input_string.lines().enumerate() {
        // start a new group every 3rd line
        if line_index % 3 == 0 {
            let rucksacks = vec![rucksack.to_string()];
            groups_of_three.push(rucksacks);
        }
        else {
            let mut rucksacks = groups_of_three.pop().unwrap();
            rucksacks.push(rucksack.to_string());
            groups_of_three.push(rucksacks);
        }
    }
    groups_of_three
}

fn calc_badge_priority(group_of_three: &Vec<String>) -> i32 {
    let first_rucksack = &group_of_three[0];
    let second_rucksack = &group_of_three[1];
    let third_rucksack = &group_of_three[2];
    let badge: HashSet<char> = first_rucksack.chars().filter(|i| second_rucksack.contains(*i) && third_rucksack.contains(*i)).collect();
    if badge.is_empty() {
        panic!("Invalid input! Could not detect a badge in a group of three rucksacks!");
    }
    if badge.len() > 1 {
        panic!("Invalid input! Found more than one badge in a group of three rucksacks! {:?}", badge);
    }
    let badge_priority = get_item_priority(badge.into_iter().collect::<Vec<char>>()[0]);
    badge_priority
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
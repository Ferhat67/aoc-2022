/********************************************
 * Day 1: Calorie Counting
 * https://adventofcode.com/2022/day/1
 ********************************************/

use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} lines. Parsing inventories...", input_string.lines().count());
    let inventories = parse_inventories(&input_string);
    println!("Found {} inventories. Calculating calory totals...", inventories.iter().count());
    let inventory_totals = calc_inventory_totals(&inventories);
    println!("Determining inventory with most calories...");
    let most_calories = inventory_totals.iter().max().unwrap();
    println!("Elf carrying most calories, carries {} calories in total!", most_calories);
    let top_three_total_calories = calc_top_three_total(&inventory_totals);
    println!("The three elves carrying the most calories, carry a total of {} calories!", top_three_total_calories);

    /*******************************************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 2255 lines. Parsing inventories...
     * Found 247 inventories. Calculating calory totals...
     * Determining inventory with most calories...
     * Elf carrying most calories, carries 66719 calories in total!
     * The three elves carrying the most calories, carry a total of 198551 calories!
     *******************************************************************************/
}

fn parse_inventories(inventory_content: &String) -> Vec<String> {
    inventory_content.split("\n\n").map(|a| a.to_string()).collect()
}

fn calc_inventory_totals(inventories: &Vec<String>) -> Vec<i32> {
    inventories.iter().map(calc_inventory_total).collect()
}

fn calc_inventory_total(inventory_string: &String) -> i32 {
    inventory_string.split("\n").flat_map(|a| a.parse::<i32>()).reduce(|a, b| a + b).unwrap()
}

fn calc_top_three_total(inventory_totals: &Vec<i32>) -> i32 {
    let mut sorted_totals = inventory_totals.to_owned();
    sorted_totals.sort();
    sorted_totals.reverse();
    let top_three = (sorted_totals[0..3]).to_vec();
    top_three.iter().map(|a| *a).reduce(|a,b| a + b).unwrap()
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

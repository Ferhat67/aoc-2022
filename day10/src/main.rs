/********************************************
 * Day 10: Cathode-Ray Tube
 * https://adventofcode.com/2022/day/10
 ********************************************/

use std::{path::Path, fs::File, io::Read};

const _DEBUG: bool = true;

#[derive(Debug, Clone)]
struct Instruction {
    name: String,
    param: i32,
}

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} lines", input_string.lines().count());

    let instructions = parse_instructions(&input_string);

    let value_per_cycle = calc_register_value_per_cycle(&instructions);
    let total_signal_strength = calc_total_signal_strength(&value_per_cycle);
    println!("Sum of signal strengths is: {}", total_signal_strength);
    assert_eq!(11720, total_signal_strength);

    let crt_output = render_crt_output(&value_per_cycle);
    print!("CRT displays:\n\n{}", crt_output);

    /**********************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 146 lines
     * Sum of signal strengths is: 11720
     * CRT displays:
     *
     * ####.###...##..###..####.###...##....##.
     * #....#..#.#..#.#..#.#....#..#.#..#....#.
     * ###..#..#.#....#..#.###..#..#.#.......#.
     * #....###..#....###..#....###..#.......#.
     * #....#.#..#..#.#.#..#....#....#..#.#..#.
     * ####.#..#..##..#..#.####.#.....##...##..
     **********************************************/
}

fn calc_total_signal_strength(value_per_cycle: &Vec<i32>) -> i32 {
    [20, 60, 100, 140, 180, 220].map(|cycle| value_per_cycle[cycle - 1] * cycle as i32).iter().sum()
}

fn render_crt_output(value_per_cycle: &Vec<i32>) -> String {
    let mut output = "".to_string();
    for (cycle, register_value) in value_per_cycle.iter().enumerate() {
        let current_pixel_x = (cycle as i32) % 40;
        let sprite_positions = [*register_value, register_value - 1, register_value + 1];
        // draw # when current pixel position is in sprites' position range
        if sprite_positions.contains(&current_pixel_x) {
            output.push('#');
        } else {
            output.push('.');
        }
        // start next row of pixels when reaching max width
        if (cycle + 1) % 40 == 0 {
            output.push_str("\n");
        }
    }
    output
}

fn calc_register_value_per_cycle(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut value_per_cycle: Vec<i32> = vec![];
    let mut register_value = 1;

    for instruction in instructions {
        let cycles = if instruction.name.eq("addx") { 2 } else { 1 }; // 2 cycles for addx; 1 for noop
        // collect register value for each instruction cycle
        for cycle in 0..cycles {
            value_per_cycle.push(register_value);
            // at end of last addx cycle, add the param value to the register
            if cycle == cycles - 1 {
                register_value += instruction.param;
            }
        }
    }
    value_per_cycle
}

fn parse_instructions(input_string: &String) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input_string.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let name = parts[0].to_string();
        if name.eq("addx") {
            let param: i32 = parts[1].parse().unwrap();
            instructions.push(Instruction { name, param });
        } else {
            instructions.push(Instruction { name: "noop".to_string(), param: 0 });
        }
    }
    instructions
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

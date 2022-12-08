/********************************************
 * Day 8: Treetop Tree House
 * https://adventofcode.com/2022/day/8
 ********************************************/

use std::{fs::File, io::Read, path::Path};

const _DEBUG: bool = false;

enum Direction {
    Vertical,
    VerticalReverse,
    Horizontal,
    HorizontalReverse,
}

fn main() {
    println!("Reading input file...");
    let input_string = read_input();
    println!("Read {} lines", input_string.lines().count());

    let tree_grid = read_tree_grid(&input_string);

    let visible_trees = find_trees_visible_from_outside(&tree_grid);
    println!("Trees that are visible from outside the grid: {}", visible_trees.len());
    assert_eq!(1763, visible_trees.len());

    let scenic_scores = calc_scenic_scores(&tree_grid);
    let max_score = scenic_scores.iter().map(|v| v.2).max().unwrap();
    println!("Highest scenic score of a tree: {}", max_score);
    assert_eq!(671160, max_score);

    /******************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Read 99 lines
     * Trees that are visible from outside the grid: 1763
     * Highest scenic score of a tree: 671160
     ******************************************************/
}

fn calc_scenic_scores(rows: &Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
    let mut scores = vec![];
    for (y, row) in rows.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if y == 0 || x == 0 || y == rows.len() - 1 || x == row.len() - 1 {
                // no need to calc scenic scores for trees at the edges since they are always multiples of 0
                scores.push((x, y, 0));
            } else {
                let (mut west, mut east, mut north, mut south) = (0,0,0,0);
                east += count_trees_visible_from(x, y, *tree, rows, Direction::Horizontal, false);
                west += count_trees_visible_from(x, y, *tree, rows, Direction::HorizontalReverse, false);
                south += count_trees_visible_from(x, y, *tree, rows, Direction::Vertical, false);
                north += count_trees_visible_from(x, y, *tree, rows, Direction::VerticalReverse, false);
                scores.push((x,y,west * east * north * south));
            }
        }
    }
    scores
}

fn count_trees_visible_from(x: usize, y: usize, tree_height: i32, grid: &Vec<Vec<i32>>, direction: Direction, smaller_only: bool) -> i32 {
    let mut visible_trees = 0;
    let range: Box<dyn Iterator<Item=usize>> = calc_iteration_range(x, y, &grid, &direction);
    for i in range {
        let other_tree_height = match direction {
            Direction::Horizontal | Direction::HorizontalReverse => grid[y][i],
            Direction::Vertical | Direction::VerticalReverse => grid[i][x]
        };
        if other_tree_height >= tree_height {
            visible_trees += if smaller_only {0} else {1}; // Part 1 requires us to only include smaller trees
            break;
        }
        else {
            visible_trees += 1;
        }
    }
    visible_trees
}

fn find_trees_visible_from_outside(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
    let mut visible_trees = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if y == 0 || x == 0 || y == grid.len() - 1 || x == row.len() - 1 {
                visible_trees.push((x, y, *tree));
            } else {
                // delta of trees until the edge is reached
                let max_west = x as i32;
                let max_north = y as i32;
                let max_east = (row.len() - 1 - x) as i32;
                let max_south = (grid.len() - 1 - y) as i32;
                // is tree is visible from any of the four sides? => all remaining trees in direction need to be visible
                let west = count_trees_visible_from(x, y,*tree, grid, Direction::HorizontalReverse, true) == max_west;
                let east = count_trees_visible_from(x, y,*tree, grid, Direction::Horizontal, true) == max_east;
                let north = count_trees_visible_from(x, y,*tree, grid, Direction::VerticalReverse, true) == max_north;
                let south = count_trees_visible_from(x, y,*tree, grid, Direction::Vertical, true) == max_south;
                // must be visible from at least one side
                if west || east || north || south {
                    visible_trees.push((x, y, *tree));
                }
            }
        }
    }
    visible_trees
}

fn calc_iteration_range(x: usize, y: usize, grid: &Vec<Vec<i32>>, direction: &Direction) -> Box<dyn Iterator<Item = usize>> {
    let range: Box<dyn Iterator<Item=usize>> = match direction {
        Direction::Horizontal => Box::new((x + 1)..(grid[y].len())),
        Direction::HorizontalReverse => Box::new((0..x).rev()),
        Direction::Vertical => Box::new((y + 1)..(grid.len())),
        Direction::VerticalReverse => Box::new((0..y).rev()),
    };
    range
}

fn read_tree_grid(input_string: &String) -> Vec<Vec<i32>> {
    let mut rows = vec![];
    for line in input_string.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let d = c.to_digit(10).unwrap();
            row.push(d as i32);
        }
        rows.push(row);
    }
    rows
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

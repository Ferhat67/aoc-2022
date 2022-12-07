/********************************************
 * Day 7: No Space Left On Device
 * https://adventofcode.com/2022/day/7
 ********************************************/

use std::{collections::HashMap, fs::File, io::Read, path::Path};

use regex::Regex;

const _DEBUG: bool = false;

#[derive(Debug)]
struct FileInfo {
    absolute_path: String,
    size: i32,
}

fn main() {
    println!("Reading input file...");
    let input_string = read_input();

    let file_paths = parse_absolute_paths_and_sizes_per_file(&input_string);
    let filesize_by_prefix = group_and_sum_filesizes_by_directory_prefix(&file_paths);

    let sum_of_directories_smaller_than_100k: i32 = filesize_by_prefix
        .iter()
        .filter(|(_k, v)| **v <= 100000)
        .map(|(_k, v)| v)
        .sum();

    println!("Sum over sizes of directories each smaller than 100000: {:?}", sum_of_directories_smaller_than_100k);
    assert_eq!(1348005, sum_of_directories_smaller_than_100k);

    const TOTAL_DISK_SPACE: i32 = 70000000;
    const REQUIRED_SPACE: i32 = 30000000;
    let total_used_space: &i32 = filesize_by_prefix.values().max().unwrap();
    let total_unused_space: i32 = TOTAL_DISK_SPACE - total_used_space;
    let to_be_deleted_space = REQUIRED_SPACE - total_unused_space;

    let size_of_smallest_directory_to_be_deleted = filesize_by_prefix
        .values()
        .filter(|v| **v >= to_be_deleted_space)
        .min()
        .unwrap();

    println!("Size of smallest directory that can be deleted to free up enough space: {:?}", size_of_smallest_directory_to_be_deleted);
    assert_eq!(12785886, *size_of_smallest_directory_to_be_deleted);

    /***********************************************************************************
     * Reading input file...
     * Successfully read ./src/input.txt
     * Sum over sizes of directories each smaller than 100000: 1348005
     * Size of smallest directory that can be deleted to free up enough space: 12785886
     ***********************************************************************************/
}

fn group_and_sum_filesizes_by_directory_prefix(file_infos: &Vec<FileInfo>) -> HashMap<String, i32> {
    let mut filesize_by_prefix: HashMap<String, i32> = HashMap::new();
    for file_info in file_infos {
        let path_segments = file_info.absolute_path.split("/").collect::<Vec<&str>>();
        for i in 1..path_segments.len() {
            let prefix = format!("{}/", &path_segments[0..i].join("/"));
            if filesize_by_prefix.contains_key(&prefix) {
                let prefix_dir_size = filesize_by_prefix.get(&prefix).unwrap();
                filesize_by_prefix.insert(prefix, prefix_dir_size + file_info.size);
            } else {
                filesize_by_prefix.insert(prefix, file_info.size);
            }
        }
    }
    if _DEBUG {
        println!("{:#?}", filesize_by_prefix);
    }
    filesize_by_prefix
}

fn parse_absolute_paths_and_sizes_per_file(input_string: &String) -> Vec<FileInfo> {
    let mut file_infos: Vec<FileInfo> = vec![];
    let mut path_segments = vec![];
    let ls_output_pattern = Regex::new(r"^(?P<filesize>\d+) (?P<filename>[a-zA-Z0-9\.]+)$").unwrap();

    for line in input_string.lines() {
        if line.starts_with("$ cd") {
            let cd_path = &line[5..];
            if cd_path.eq("/") {
                path_segments = vec!["".to_string()];
            } else if cd_path.starts_with("..") {
                // remove the last path segment to step into parent directory
                path_segments.pop();
            } else {
                // append the new directory
                path_segments.push(cd_path.to_string());
            }
        } else if ls_output_pattern.is_match(line) {
            // construct absolute path to file based on current path segments
            let captures = ls_output_pattern.captures(line).unwrap();
            let file_abs_path = format!("{}/{}", path_segments.join("/"), &captures["filesize"]);
            let filesize: i32 = captures["filesize"].parse().unwrap();
            let file_info = FileInfo {
                absolute_path: file_abs_path,
                size: filesize,
            };
            file_infos.push(file_info);
        }
    }
    if _DEBUG {
        println!("{:#?}", file_infos);
    }
    file_infos
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

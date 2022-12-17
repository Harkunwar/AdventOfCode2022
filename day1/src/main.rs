use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    return data;
}

fn main() {
    let input_string = get_input("/Users/harkunwar/advent_of_code/day1/src/input.txt");
    let input_array = input_string.split('\n');
    let mut current_elf = 0;
    let mut max_elf = 0;
    for input in input_array {
        if input.eq("") {
            current_elf = 0;
        } else {
            let current_value: i32 = input.parse().unwrap();
            current_elf += current_value;
            max_elf = max(max_elf, current_elf);
        }
    }
    println!("{}", max_elf);
}

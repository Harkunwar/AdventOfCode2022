use crate::common::*;
use std::cmp::max;

pub fn day_1() {
    let mut input_string = get_input("/Users/harkunwar/advent_of_code/assets/input_1.txt");
    input_string.push_str("\n");
    let input_array = input_string.split('\n');
    let mut max_elfs = [0, 0, 0];
    let mut current_elf = 0;
    for input in input_array {
        if input.eq("") {
            let mut min_idx = 0;
            for i in 0..3 {
                if max_elfs[min_idx] > max_elfs[i] {
                    min_idx = i;
                }
            }
            max_elfs[min_idx] = max(max_elfs[min_idx], current_elf);
            current_elf = 0;
        } else {
            let current_value: i32 = input.parse().unwrap();
            current_elf += current_value;
        }
    }
    let total_calories = max_elfs[0] + max_elfs[1] + max_elfs[2];

    let max_calories = max(max(max_elfs[0], max_elfs[1]), max_elfs[2]);

    println!("Day 1, Part 1: {}", max_calories);
    println!("Day 1, Part 2: {}", total_calories);
}

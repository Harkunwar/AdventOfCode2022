use std::collections::HashSet;

use crate::common::get_input;

fn get_priority(c: &char) -> i32 {
    if c.is_uppercase() {
        return (*c as i32) - ('A' as i32) + 27;
    }
    return (*c as i32) - ('a' as i32) + 1;
}

fn part_1(input_string: &String) {
    let input_array = input_string.split('\n');
    let mut total_priority = 0;

    for input in input_array {
        let len = input.len();
        let mut chars_input = input.chars();
        let mut compartment_a_set: HashSet<char> = HashSet::new();
        for _ in 0..(len / 2) {
            let c = chars_input.next().unwrap();
            compartment_a_set.insert(c);
        }
        for _ in 0..(len / 2) {
            let c = chars_input.next().unwrap();
            if compartment_a_set.contains(&c) {
                total_priority += get_priority(&c);
                break;
            }
        }
    }
    println!("Day 3, Part 1: {}", total_priority);
}

fn fill_set(set: &mut HashSet<char>, string: &str) {
    let chars_input = string.chars();
    for c in chars_input {
        set.insert(c);
    }
}

fn part_2(input_string: &String) {
    let input_array = input_string.split('\n');
    let mut total_priority = 0;
    let mut i = 0;
    let set_array: [&mut HashSet<char>; 3] = [
        &mut HashSet::new(),
        &mut HashSet::new(),
        &mut HashSet::new(),
    ];
    for input in input_array {
        set_array[i].clear();
        fill_set(set_array[i], input);
        i += 1;
        if i == 3 {
            i = 0;
            let result = set_array[0]
                .iter()
                .filter(|c| set_array[1].contains(c))
                .filter(|c| set_array[2].contains(c))
                .next()
                .unwrap();
            total_priority += get_priority(result);
        }
    }
    println!("Day 3, Part 2: {}", total_priority);
}

pub fn day_3() {
    let input_string = get_input("/Users/harkunwar/advent_of_code/assets/input_3.txt");
    part_1(&input_string);
    part_2(&input_string);
}

use crate::common::*;
use std::collections::HashMap;

fn part_1_choose_score(us: &str) -> i32 {
    if us == "X" {
        return 1;
    } else if us == "Y" {
        return 2;
    }
    return 3;
}

fn part_1_score(them: &str, us: &str) -> i32 {
    if (them == "A" && us == "X") || (them == "B" && us == "Y") || (them == "C" && us == "Z") {
        return 3 + part_1_choose_score(us);
    } else if (them == "A" && us == "Y") || (them == "B" && us == "Z") || (them == "C" && us == "X")
    {
        return 6 + part_1_choose_score(us);
    } else {
        return 0 + part_1_choose_score(us);
    }
}

fn part_1() {
    let input_string = get_input("/Users/harkunwar/advent_of_code/assets/input_2.txt");
    let input_array = input_string.split('\n');
    let mut our_score = 0;
    for input in input_array {
        let mut plays = input.split(' ');
        let their_play = plays.next().unwrap();
        let our_play = plays.next().unwrap();
        let score = part_1_score(their_play, our_play);
        our_score += score;
    }
    println!("Day 2, Part 1: {}", our_score);
}

fn part_2_choose_score(us: &str) -> i32 {
    if us == "X" {
        return 0;
    } else if us == "Y" {
        return 3;
    }
    return 6;
}

fn part_2_score(them: &str, us: &str) -> i32 {
    let mut a_dict: HashMap<&str, i32> = HashMap::new();
    a_dict.insert("X", 3);
    a_dict.insert("Y", 1);
    a_dict.insert("Z", 2);

    let mut b_dict: HashMap<&str, i32> = HashMap::new();
    b_dict.insert("X", 1);
    b_dict.insert("Y", 2);
    b_dict.insert("Z", 3);

    let mut c_dict: HashMap<&str, i32> = HashMap::new();
    c_dict.insert("X", 2);
    c_dict.insert("Y", 3);
    c_dict.insert("Z", 1);

    let mut them_dict: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    them_dict.insert("A", a_dict);
    them_dict.insert("B", b_dict);
    them_dict.insert("C", c_dict);

    let score = *them_dict.get(them).unwrap().get(us).unwrap();
    return score + part_2_choose_score(us);
}

fn part_2() {
    let input_string = get_input("/Users/harkunwar/advent_of_code/assets/input_2.txt");
    let input_array = input_string.split('\n');
    let mut our_score = 0;
    for input in input_array {
        let mut plays = input.split(' ');
        let their_play = plays.next().unwrap();
        let our_play = plays.next().unwrap();
        let score = part_2_score(their_play, our_play);
        our_score += score;
    }
    println!("Day 2, Part 2: {}", our_score);
}

pub fn day_2() {
    part_1();
    part_2();
}

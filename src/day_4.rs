use crate::common::get_input;

fn is_sorted(array: &[i32]) -> bool {
    if array.len() > 0 {
        let mut prev = array[0];
        for item in array {
            if prev > *item {
                return false;
            }
            prev = *item;
        }
    }
    return true;
}

pub fn day_4() {
    let input_string = get_input("/Users/harkunwar/advent_of_code/assets/input_4.txt");
    let input_array = input_string.split('\n');
    let mut complete_overlap_count = 0;
    let mut overlap_count = 0;
    for input in input_array {
        let mut elf_works_raw = input.split(',');
        let mut first_elf_raw = elf_works_raw.next().unwrap().split('-');
        let mut second_elf_raw = elf_works_raw.next().unwrap().split('-');
        let first_elf_work: [i32; 2] = [
            first_elf_raw.next().unwrap().parse().unwrap(),
            first_elf_raw.next().unwrap().parse().unwrap(),
        ];
        let second_elf_work: [i32; 2] = [
            second_elf_raw.next().unwrap().parse().unwrap(),
            second_elf_raw.next().unwrap().parse().unwrap(),
        ];
        if (first_elf_work[0] <= second_elf_work[0] && first_elf_work[1] >= second_elf_work[1])
            || (first_elf_work[0] >= second_elf_work[0] && first_elf_work[1] <= second_elf_work[1])
        {
            complete_overlap_count += 1;
            overlap_count += 1;
            continue;
        }
        let overlap_order_1 = [
            first_elf_work[0],
            second_elf_work[0],
            first_elf_work[1],
            second_elf_work[1],
        ];
        let overlap_order_2 = [
            second_elf_work[0],
            first_elf_work[0],
            second_elf_work[1],
            first_elf_work[1],
        ];
        if is_sorted(&overlap_order_1) || is_sorted(&overlap_order_2) {
            overlap_count += 1;
        }
    }
    println!("Day 4, Part 1: {}", complete_overlap_count);
    println!("Day 4, Part 2: {}", overlap_count);
}

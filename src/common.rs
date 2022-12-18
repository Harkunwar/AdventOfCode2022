use std::fs::File;
use std::io::prelude::*;

pub fn get_input(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    return data;
}

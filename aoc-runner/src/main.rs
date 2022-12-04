// System Imports
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

//Custom imports
use day_4;

const INPUT_FILE_NAME: &str = "./data/input.txt";

fn read_input_from_file(file_name: &str) -> String {
    //Filename should be relative to the place from where you are running
    //keep that in mind lol
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    buffer
}

fn main() {
    let file_data = read_input_from_file(INPUT_FILE_NAME);
    println!("Part one output: {}", day_4::part_one(&file_data));
    println!("Part two output: {}", day_4::part_two(&file_data));
}

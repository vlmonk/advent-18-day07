#[macro_use]
extern crate lazy_static;
extern crate regex;

mod input_record;

use crate::input_record::InputRecord;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filtename = "input.txt";
    let mut file = File::open(filtename).expect("Can't open file");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Can't read file");

    let input: Vec<InputRecord> = content
        .lines()
        .map(|line| InputRecord::parse(line))
        .collect();
}

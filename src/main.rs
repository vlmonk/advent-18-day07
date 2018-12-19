#![feature(duration_as_u128)]

mod input_record;
mod solver;
mod tick_solver;

use crate::input_record::InputRecord;
use crate::solver::Solver;
use crate::tick_solver::TickSolver;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let filtename = "input.txt";
    let mut file = File::open(filtename).expect("Can't open file");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Can't read file");

    let input: Vec<InputRecord> = content
        .lines()
        .map(|line| InputRecord::parse(line))
        .collect();

    let input2 = input.clone();

    let solver = Solver::new(input);
    let result = solver.result();
    println!("Task 1: {}", result);

    let mut solver2 = TickSolver::new(input2, 5, 60);
    let result = solver2.result();
    println!("Task 2: {}", result);

    let result = now.elapsed().as_micros();
    println!("Total time: {} µs", result)
}

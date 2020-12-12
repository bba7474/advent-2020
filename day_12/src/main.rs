use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::ferry::{Instruction, new_ferry};

mod ferry;

fn main() {
    let instructions = read_input();

    let mut ferry = new_ferry(&String::from("E"));

    for instr in instructions {
        ferry.travel(instr);
    }

    let manhattan_dist = ferry.calc_manhattan_distance();
    announce_answer(format!("The ferry is at a Manhattan Distance of {}", manhattan_dist));
}

fn read_input() -> Vec<Instruction> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

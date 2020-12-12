use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::ferry::{Instruction1, Instruction2, new_ferry};

mod ferry;

fn main() {
    let instructions_1 = read_input_1();
    let mut ferry_1 = new_ferry(&String::from("E"));
    for instr in instructions_1 {
        ferry_1.travel_1(instr);
    }

    let manhattan_dist_1 = ferry_1.calc_manhattan_distance();
    announce_answer(format!("The ferry is at a Manhattan Distance of {}", manhattan_dist_1));

    let instructions_2 = read_input_2();
    let mut ferry_2 = new_ferry(&String::from("E"));
    for instr in instructions_2 {
        ferry_2.travel_2(instr);
    }

    let manhattan_dist_2 = ferry_2.calc_manhattan_distance();
    announce_answer(format!("The ferry is at a Manhattan Distance of {}", manhattan_dist_2));
}

fn read_input_1() -> Vec<Instruction1> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_input_2() -> Vec<Instruction2> {
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

use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::plane_seat::{get_plane_seat_id};

mod plane_seat;

fn main() {
    let answer = read_input().iter()
        .map(|s| get_plane_seat_id(s))
        .max().unwrap();

    announce_answer(answer.to_string());
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn announce_answer(answer: String) {
    let message = format!("The answer is: {}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

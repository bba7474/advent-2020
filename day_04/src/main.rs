use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::passport::{is_valid_passport_1, is_valid_passport_2};

mod passport;

fn main() {
    let input = read_input();

    let count_valid_passports_1 = input.iter()
        .filter(|passport| is_valid_passport_1(passport))
        .count();

    announce_answer(count_valid_passports_1.to_string());

    let count_valid_passports_2 = input.iter()
        .filter(|passport| is_valid_passport_2(passport))
        .count();

    announce_answer(count_valid_passports_2.to_string());
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split("\r\n\r\n")
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

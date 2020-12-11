use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::seating::Seating;

mod seating;

fn main() {
    let mut seating = read_input();
    let mut occ_seats_count = 0;
    loop {
        seating.apply_rules();
        let count = seating.count_occupied();
        if count == occ_seats_count {
            break;
        }
        occ_seats_count = count;
    }

    announce_answer(format!("There are {} occupied seats when the rules stabilise", occ_seats_count));
}

fn read_input() -> Seating {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .parse().unwrap()
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

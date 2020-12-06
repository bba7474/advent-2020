use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::plane_seat::{get_plane_seat_id};
use std::cmp::min;

mod plane_seat;

fn main() {
    let mut seat_ids = read_input().iter()
        .map(|s| get_plane_seat_id(s))
        .collect::<Vec<i32>>();

    seat_ids.sort();

    let pairs = seat_ids.windows(2);

    let mut seat_id = 0;

    for pair in pairs.into_iter() {
        if (pair[0] - pair[1]).abs() > 1 {
            seat_id = min(pair[0], pair[1]) + 1;
            break;
        }
    }

    announce_answer(seat_id.to_string());
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

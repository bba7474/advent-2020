use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

use crate::seating::Seating;

mod seating;

fn main() {
    let seating_plan = read_input();

    let occ_seats_count_1 = get_occupied_seats_1(&mut seating_plan.clone());
    announce_answer(format!("There are {} occupied seats when the adjacency rules stabilise", occ_seats_count_1));

    let occ_seats_count_2 = get_occupied_seats_2(&mut seating_plan.clone());
    announce_answer(format!("There are {} occupied seats when the visible rules stabilise", occ_seats_count_2));
}

fn get_occupied_seats_1(seating: &mut Seating) -> i32 {
    let mut count_occupied_seats = 0;
    loop {
        seating.apply_rules_1();
        let count = seating.count_occupied();
        if count == count_occupied_seats {
            break;
        }
        count_occupied_seats = count;
    }
    count_occupied_seats
}

fn get_occupied_seats_2(seating: &mut Seating) -> i32 {
    let mut count_occupied_seats = 0;
    loop {
        seating.apply_rules_2();
        let count = seating.count_occupied();
        if count == count_occupied_seats {
            break;
        }
        count_occupied_seats = count;
    }
    count_occupied_seats
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
